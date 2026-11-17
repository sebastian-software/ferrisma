// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2019, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/ipred16.S` (16 bpc; one kernel for 10 and 12 bit) with NEON
//! intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062), in the order of the
//! assembly file. The entry points are the `pub unsafe extern "C" fn
//! dav1d_*_16bpc_neon`; the DSP slots and the C wrappers around the
//! directional fill kernels are in `simd/arm/ipred.rs`.
//!
//! Conventions of the whole file: general registers keep their assembly
//! names (`x0`, `w7`, ...); vector registers are typed locals named after
//! the registers (`v0`, `v16`), not the register array of `util.rs`: the
//! kernels work on bytes, 16-bit and 32-bit lanes in turn, and a typed
//! local per value reads closer to the assembly line than a reinterpreted
//! array element. A jump table (`jumptable` + `br`) is a `match` on the
//! table index, with the labels as comments. Pointers that the assembly
//! post-increments past a buffer after the last access use wrapping
//! arithmetic. Each section states its deviations from the assembly and,
//! per entry point, the memory it reads and writes.
//!
//! # Part A: the DC, V, H, Paeth and smooth predictors
//!
//! From `ipred16.S` lines 36–1426: `ipred_dc_128`, `ipred_v`, `ipred_h`,
//! `ipred_dc_top`, `ipred_dc_left`, `ipred_dc`, `ipred_paeth`, `ipred_smooth`,
//! `ipred_smooth_v`, `ipred_smooth_h` are the entry points
//! `dav1d_ipred_*_16bpc_neon` (one kernel for 10 and 12 bit; only
//! `ipred_dc_128` reads `bitdepth_max`), with the labels of their jump tables
//! as `match` arms and, for `ipred_dc_left`/`ipred_dc`, a `match` on the height
//! (`L(ipred_dc*_h*)`) followed by one on the width (`L(ipred_dc*_w*)`). The
//! store loops the DC predictors and `ipred_v` repeat are
//! `ipred_dc_128_w{4,8,16,32}` (inlined) and the functions `ipred_dc_128_w64`
//! and `ipred_v_w64`. `ipred_h_160` holds the loops of `ipred_h` for 16, 32 and
//! 64 columns. The 16+ column loops of the smooth predictors are
//! `ipred_smooth_160::<W>`, `ipred_smooth_v_160::<W>`,
//! `ipred_smooth_h_160::<W>`, the assembly's loop nest instantiated for each
//! width. `pa_*` are instruction helpers: `pa_tbl_idx`, `pa_subs_gt`,
//! `pa_ld1r_2s`, `pa_ld1r_2d`, `pa_st1_2`, `pa_s`, `pa_sabd`, `pa_cmge`,
//! `pa_sqrdmulh`, `pa_ushll8`, `pa_smlal`, `pa_smlal2`, `pa_rshrn9`,
//! `pa_rshrn9_2`, `pa_addv_4h`, `pa_addv_8h`, `pa_uaddlv_4h`, `pa_uaddlv_8h`,
//! `pa_add_uaddlv`, `pa_urshr_dup`, `pa_rshrn_dup`, `pa_mul_ushr17`. Pointers
//! are byte pointers (the stride is in bytes); vector registers are typed
//! locals named after the assembly's registers; the deviations are in the
//! comments at the functions.
//!
//! # Part B: the edge helpers and the z1 fill kernels
//!
//! From `ipred16.S` lines 1427–2061, the 16 bpc forms of part B:
//! `padding_mask_buf`/`padding_mask` (`PADDING_MASK_BUF`, `padding_mask()`;
//! `ipred_z1_bit` is the `bit` instruction with its byte mask), `edge_filter`
//! (`EDGE_FILTER`), `dav1d_ipred_z1_upsample_edge_16bpc_neon`,
//! `dav1d_ipred_z2_upsample_edge_16bpc_neon` (both clamp to `bitdepth_max`),
//! `dav1d_ipred_z1_filter_edge_16bpc_neon` (`ipred_z1_filter_edge_fivetap`,
//! `ipred_z1_filter_edge_tap3`/`_tap5`), `dav1d_ipred_pixel_set_16bpc_neon`,
//! `dav1d_ipred_reverse_16bpc_neon`, `dav1d_ipred_z1_fill1_16bpc_neon` (jump
//! table as `match`, the block `160:/320:/640:` as
//! `ipred_z1_fill1_160_320_640::<W>`) and `dav1d_ipred_z1_fill2_16bpc_neon`.
//! Typed locals named after the registers; pixel pointers are `*u16`, the
//! stride is in bytes. Two oddities of the assembly are kept: `ipred_z1_fill2`
//! replicates one byte as its padding value (rows the wrapper cannot reach),
//! and the three-tap `ipred_z1_filter_edge` stores 8 pixels per 16 counted in
//! its final padding loop (never more than 8 are asked for). The helpers work
//! in blocks of 8 or 16 pixels: they read up to 22 pixels above `in[end]` and,
//! at filter strength 3, `in[-1]`.
//!
//! # Part C: the z2 fill kernels
//!
//! Part C — the z2 fill kernels (`ipred16.S` lines 2062–3450): `increments` is
//! `INCREMENTS` (eight entries here); `ipred_z2_fill1_16bpc_neon` is
//! `dav1d_ipred_z2_fill1_16bpc_neon` with its jump table as a `match` on the
//! width and its labels `40:`, `80:` and `160:`/`320:`/`640:` as the functions
//! `ipred_z2_fill1_w4`, `_w8` and `_w16` (inner labels `4:`/`49:`, `8:`/`89:`,
//! `1:`–`4:`, `21:`–`23:`, `169:` with `31:`–`34:` are commented loops and
//! blocks inside them); `ipred_z2_fill2_16bpc_neon` and
//! `ipred_z2_fill3_16bpc_neon` have their labels `40:` and `80:` as
//! `ipred_z2_fill2_w4`/`_w8` and `ipred_z2_fill3_w4`/`_w8`. Registers are typed
//! locals named after the assembly's: pixels, positions and fractions in 16-bit
//! lanes, table indices in bytes (`z2_8h`/`z2_16b` change the view), `v21s` for
//! the 32-bit products of a `umull`/`umlal` pair. Where the left edge is taller
//! than the 32 pixels a `tbx` reaches, the assembly loads single pixels per
//! column (`ld3`/`ld1`/`ld4`/`ld2` by lane); these are the `vldNq_lane_u16`
//! calls. The products with the per-row fractions of `ipred_z2_fill1_w16` are
//! written by element (see the comment there). Caller: `ipred_z2_neon` of
//! `simd/arm/ipred.rs`, as at 8 bpc.
//!
//! # Part E: the z3 fill kernels
//!
//! From `ipred16.S` lines 3451–3901, the z3 fill kernels, called by
//! `ipred_z3_neon` of `simd/arm/ipred.rs`. At 16 bpc both kernels fill two
//! columns at a time from the edge at each column's base, and
//! `ipred_z3_fill1_tbl` is indexed with the height:
//! `dav1d_ipred_z3_fill1_16bpc_neon` dispatches to `ipred_z3_fill1_h4` (`40:`),
//! `_h8` (`80:`) and `ipred_z3_fill1_h16::<H>` (`160:/320:/640:`, one instance
//! per height 16, 32, 64); `dav1d_ipred_z3_fill2_16bpc_neon` (upsampled edge)
//! is `ipred_z3_fill2_h4`/`_h8`. Columns whose base is past the edge are filled
//! by `ipred_z3_fill_padding_neon` (same name; its table's labels are
//! `ipred_z3_fill_padding_cols::<N>`, `ipred_z3_fill_padding_wide` is inlined
//! into it), reached by a tail branch as in the assembly. The repeated
//! `umull`/`umlal`/`rshrn` and `ushll`/`smlal`/`rshrn` sequences are the
//! helpers `ipred_z3_umull_umlal_rshrn` and `ipred_z3_ushll_smlal_rshrn`. Typed
//! locals named after the registers; pointers are byte pointers. Deviations
//! that change the instruction count: the 2-pixel row stores are `st2` lane
//! stores of the two result registers instead of `zip` + `st1`; the tall path's
//! row loop is unrolled per height; the column pointers are kept instead of
//! recomputed; the wide padding loop counts blocks. `fill2` replicates the byte
//! at `left + max_base_y` as padding, as the assembly does (not reached from
//! the wrapper).
//!
//! # Part D: the filter predictor, the palette predictor and chroma from luma
//!
//! > From `ipred16.S` lines 3902–5711: the macro `filter_fn bpc` as the generic
//! `filter_fn::<BPC>` with its instances `ipred_filter_10bpc_neon` and
//! `ipred_filter_12bpc_neon` (blocks `40:`, `80:` inline, `160:`/`320:` in
//! `ipred_filter_w16_neon::<BPC>`; the arithmetic and clamp of one 4x2 block is
//! the macro `pd_filter_fn_block!`, 16-bit `mul`/`mla` at 10 bit, widening
//! `smull`/`smlal` at 12 bit) and the exported `dav1d_ipred_filter_16bpc_neon`,
//! which chooses between them by `bitdepth_max`; `dav1d_pal_pred_16bpc_neon`
//! (`pal_pred_tbl`: `match` on `pd_tbl_idx`; `pal_pred_idx32` and
//! `pal_pred_tbl2` are the unpacking of the indices and the table lookups of
//! one register); `dav1d_ipred_cfl_128_16bpc_neon` with the splat blocks
//! `L(ipred_cfl_splat_w4)`, `_w8`, `_w16` as
//! `ipred_cfl_splat_w{4,8,16}_16bpc_neon` (per-register arithmetic
//! `ipred_cfl_splat_px`), `dav1d_ipred_cfl_top_16bpc_neon`,
//! `dav1d_ipred_cfl_left_16bpc_neon` and `dav1d_ipred_cfl_16bpc_neon`, which
//! compute the dc (`ipred_cfl_dc_mul` is the division step of the four
//! `L(ipred_cfl_w*)` blocks) and branch into the splat blocks (tail calls); and
//! `dav1d_ipred_cfl_ac_{420,422,444}_16bpc_neon` (prologue `ipred_cfl_ac_head`,
//! nested `match`es for the width and `w_pad` tables, `ipred_cfl_ac_uaddw` for
//! a `uaddw`/`uaddw2` pair, `ipred_cfl_ac_420_addp` for the 4:2:0 subsampling
//! sum) with the shared tails `L(ipred_cfl_ac_420_w4_hpad)`,
//! `L(ipred_cfl_ac_420_w8_hpad)`, `L(ipred_cfl_ac_420_w16_hpad)` and
//! `L(ipred_cfl_ac_420_w4_calc_subtract_dc)` as functions of those names
//! (suffix `_16bpc_neon`), each entered by a tail call;
//! `L(ipred_cfl_ac_444_w32_hpad)` is inline. Vector registers are typed locals
//! named after the assembly's registers; `pd_*` are single instructions without
//! an intrinsic of their own.
use core::arch::aarch64::*;
use core::ffi::c_int;

// ============================================================================
// Part A (ipred16.S lines 36–1426): the DC, V, H, Paeth and smooth predictors.
// ============================================================================
// Conventions of this part: general registers keep their assembly names (`x0`, `w4`, ...) and pointers are byte
// pointers, as the assembly's address arithmetic (the stride is in bytes); vector registers are typed locals
// named after the registers (`v0`, `v16`), since the kernels are too small for a register array. A jump table
// (`br x5`) is a `match` on `clz(width) - 25`, with the labels as comments. A post-incremented address is the
// access followed by `wrapping_offset`/`wrapping_add`, because the last increment leaves the block. The helpers
// `pa_*` are single instructions (or pairs) that have no intrinsic of their own.
//
// Where the rows of a loop are stored through two pointers that the assembly post-increments by a register
// (`st1 {v0.8h}, [x0], x1; st1 {v0.8h}, [x6], x1`, twice), the second pair of rows is addressed as `[x0, x1]`,
// `[x6, x1]` and both pointers advance once per iteration: a plain store has no post-increment by a register,
// and with this form the compiler needs one add per iteration instead of four. The stores keep their order,
// and the comments name the assembly's instruction.

/// `clz w3, w3; sub w3, w3, #25`: the index into a width (or height) jump table, 0 for 64 down to 4 for 4.
#[inline(always)]
fn pa_tbl_idx(w: c_int) -> u32 {
    (w as u32).leading_zeros().wrapping_sub(25)
}

/// `subs w4, w4, #n` together with the `b.gt` (or negated `b.le`) that follows it: counts a block dimension down
/// and is true while something remains. The dimensions are multiples of `n`, so the condition is written as
/// `!= 0`, the form the compiler turns into `subs` and a branch.
#[inline(always)]
fn pa_subs_gt(w: &mut c_int, n: c_int) -> bool {
    *w = w.wrapping_sub(n);
    *w != 0
}

/// `ld1r {v0.2s}, [p]`
#[inline(always)]
unsafe fn pa_ld1r_2s(p: *const u8) -> uint8x8_t {
    vreinterpret_u8_u32(vdup_n_u32(p.cast::<u32>().read_unaligned()))
}

/// `ld1r {v0.2d}, [p]`
#[inline(always)]
unsafe fn pa_ld1r_2d(p: *const u8) -> uint16x8_t {
    vreinterpretq_u16_u64(vdupq_n_u64(p.cast::<u64>().read_unaligned()))
}

/// `st1 {v0.8h, v1.8h}, [p]`, written as two plain stores: the compiler pairs them and keeps the post-increment
/// (`stp q0, q1, [x0], #32`), where it computes the address of a two-register `st1` with an add per row and
/// iteration.
#[inline(always)]
unsafe fn pa_st1_2(p: *mut u8, a: uint16x8_t, b: uint16x8_t) {
    vst1q_u16(p.cast(), a);
    vst1q_u16(p.add(16).cast(), b);
}

#[inline(always)]
unsafe fn pa_s(v: uint16x8_t) -> int16x8_t {
    vreinterpretq_s16_u16(v)
}

/// `sabd vd.8h, vn.8h, vm.8h`
#[inline(always)]
unsafe fn pa_sabd(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    vreinterpretq_u16_s16(vabdq_s16(pa_s(a), pa_s(b)))
}

/// `cmge vd.8h, vn.8h, vm.8h`
#[inline(always)]
unsafe fn pa_cmge(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    vcgeq_s16(pa_s(a), pa_s(b))
}

/// `sqrdmulh vd.8h, vn.8h, vm.8h`
#[inline(always)]
unsafe fn pa_sqrdmulh(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    vreinterpretq_u16_s16(vqrdmulhq_s16(pa_s(a), pa_s(b)))
}

/// `ushll vd.4s, vn.4h, #8`
#[inline(always)]
unsafe fn pa_ushll8(a: uint16x4_t) -> int32x4_t {
    vreinterpretq_s32_u32(vshll_n_u16::<8>(a))
}

/// `smlal vd.4s, vn.4h, vm.4h`
#[inline(always)]
unsafe fn pa_smlal(acc: int32x4_t, a: uint16x8_t, b: uint16x8_t) -> int32x4_t {
    vmlal_s16(acc, vget_low_s16(pa_s(a)), vget_low_s16(pa_s(b)))
}

/// `smlal2 vd.4s, vn.8h, vm.8h`
#[inline(always)]
unsafe fn pa_smlal2(acc: int32x4_t, a: uint16x8_t, b: uint16x8_t) -> int32x4_t {
    vmlal_high_s16(acc, pa_s(a), pa_s(b))
}

/// `rshrn vd.4h, vn.4s, #9`
#[inline(always)]
unsafe fn pa_rshrn9(a: int32x4_t) -> uint16x4_t {
    vreinterpret_u16_s16(vrshrn_n_s32::<9>(a))
}

/// `rshrn vd.4h, vn.4s, #9; rshrn2 vd.8h, vm.4s, #9`
#[inline(always)]
unsafe fn pa_rshrn9_2(a: int32x4_t, b: int32x4_t) -> uint16x8_t {
    vreinterpretq_u16_s16(vrshrn_high_n_s32::<9>(vrshrn_n_s32::<9>(a), b))
}

// The sums of the DC predictors. The assembly computes them in lane 0 of a register (`addv h0`, `uaddlv s0`,
// `add v0.2s, ...`) and replicates the result at the end (`dup v0.8h, v0.h[0]`). Here a sum stands for "the
// value in lane 0" with the other lanes unspecified (the scalar replicated), and the instructions work on the
// whole register (`.8h`, `.4s`), of which only lane 0 is read. In this form the compiler takes the result of
// the `addv`/`uaddlv` instruction as it is; a cleared register with the sum inserted costs it two more
// instructions per sum.

/// `addv h0, v0.4h`: the sum in lane 0.
#[inline(always)]
unsafe fn pa_addv_4h(v: uint16x4_t) -> uint16x8_t {
    vdupq_n_u16(vaddv_u16(v))
}

/// `addv h0, v0.8h`: the sum in lane 0.
#[inline(always)]
unsafe fn pa_addv_8h(v: uint16x8_t) -> uint16x8_t {
    vdupq_n_u16(vaddvq_u16(v))
}

/// `uaddlv s0, v0.4h`: the sum in lane 0.
#[inline(always)]
unsafe fn pa_uaddlv_4h(v: uint16x4_t) -> uint32x4_t {
    vdupq_n_u32(vaddlv_u16(v))
}

/// `uaddlv s0, v0.8h`: the sum in lane 0.
#[inline(always)]
unsafe fn pa_uaddlv_8h(v: uint16x8_t) -> uint32x4_t {
    vdupq_n_u32(vaddlvq_u16(v))
}

/// `add v0.2s, v0.2s, v1.2s` with a fresh `uaddlv` result as an operand, written as the saturating `uqadd`: the
/// same value (the sums of at most 128 pixels and the rounding term are far below 2^32), and the form the
/// compiler keeps in the vector unit (it moves a plain add of such scalars into general registers, two or
/// three more instructions).
#[inline(always)]
unsafe fn pa_add_uaddlv(a: uint32x4_t, b: uint32x4_t) -> uint32x4_t {
    vqaddq_u32(a, b)
}

/// `urshr v0.4h, v0.4h, #N; dup v0.8h, v0.h[0]` of a replicated sum: the rounding shift of the whole register
/// already is the replicated result.
#[inline(always)]
unsafe fn pa_urshr_dup<const N: i32>(v: uint16x8_t) -> uint16x8_t {
    vrshrq_n_u16::<N>(v)
}

/// `rshrn v4.4h, v0.4s, #N; dup v0.8h, v4.h[0]`
#[inline(always)]
unsafe fn pa_rshrn_dup<const N: i32>(v: uint32x4_t) -> uint16x8_t {
    vdupq_lane_u16::<0>(vrshrn_n_u32::<N>(v))
}

/// `dup v16.2s, w16; mul v0.2s, v0.2s, v16.2s; ushr v0.2s, v0.2s, #17`
#[inline(always)]
unsafe fn pa_mul_ushr17(v: uint32x4_t, w16: u32) -> uint32x4_t {
    vshrq_n_u32::<17>(vmulq_u32(v, vdupq_n_u32(w16)))
}

// The store loops of the DC predictors. `ipred_dc_128`, `ipred_dc_top`, `ipred_dc_left` (`L(ipred_dc_left_w*)`),
// `ipred_dc` (`L(ipred_dc_w*)`, label `2:`) and `ipred_v` each end in the same five loops; here they are one set
// of functions, inlined into each entry point as the assembly repeats them.
//
// The loops for 4 and 8 columns (one register per row, a plain store) address the second pair of rows as
// `[x0, x1]`, `[x6, x1]`, see the head of the file. The loops for 16 and 32 columns keep the assembly's
// post-incremented `st1` of several registers, on one pointer that visits the rows in the order of the
// assembly's two (`x0`, `x6` = `x0` + stride; here `x1` is the stride, not twice the stride). The loops for 64
// columns are functions of their own, see there.

/// `4:` of `ipred_dc_128` (`40:`), `ipred_dc_top`, `ipred_v`, `L(ipred_dc_left_w4)`, `L(ipred_dc_w4)`.
#[inline(always)]
unsafe fn ipred_dc_128_w4(mut x0: *mut u8, mut x6: *mut u8, x1: isize, mut w4: c_int, v0: uint16x4_t) {
    loop {
        vst1_u16(x0.cast(), v0); // st1 {v0.4h}, [x0], x1
        vst1_u16(x6.cast(), v0); // st1 {v0.4h}, [x6], x1
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        vst1_u16(x0.wrapping_offset(x1).cast(), v0); // st1 {v0.4h}, [x0], x1
        vst1_u16(x6.wrapping_offset(x1).cast(), v0); // st1 {v0.4h}, [x6], x1
        x0 = x0.wrapping_offset(x1 << 1);
        x6 = x6.wrapping_offset(x1 << 1);
        if !gt {
            break; // b.gt 4b
        }
    }
}

/// `8:` of `ipred_dc_128` (`80:`), `ipred_dc_top`, `ipred_v`, `L(ipred_dc_left_w8)`, `L(ipred_dc_w8)`.
#[inline(always)]
unsafe fn ipred_dc_128_w8(mut x0: *mut u8, mut x6: *mut u8, x1: isize, mut w4: c_int, v0: uint16x8_t) {
    loop {
        vst1q_u16(x0.cast(), v0); // st1 {v0.8h}, [x0], x1
        vst1q_u16(x6.cast(), v0); // st1 {v0.8h}, [x6], x1
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        vst1q_u16(x0.wrapping_offset(x1).cast(), v0); // st1 {v0.8h}, [x0], x1
        vst1q_u16(x6.wrapping_offset(x1).cast(), v0); // st1 {v0.8h}, [x6], x1
        x0 = x0.wrapping_offset(x1 << 1);
        x6 = x6.wrapping_offset(x1 << 1);
        if !gt {
            break; // b.gt 8b
        }
    }
}

/// `16:` of `ipred_dc_128` (`160:`), `ipred_dc_top`, `ipred_v`, `L(ipred_dc_left_w16)`, `L(ipred_dc_w16)`.
#[inline(always)]
unsafe fn ipred_dc_128_w16(mut x0: *mut u8, x1: isize, mut w4: c_int, v01: uint16x8x2_t) {
    loop {
        vst1q_u16_x2(x0.cast(), v01); // st1 {v0.8h, v1.8h}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        vst1q_u16_x2(x0.cast(), v01); // st1 {v0.8h, v1.8h}, [x6], x1
        x0 = x0.wrapping_offset(x1);
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        vst1q_u16_x2(x0.cast(), v01); // st1 {v0.8h, v1.8h}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        vst1q_u16_x2(x0.cast(), v01); // st1 {v0.8h, v1.8h}, [x6], x1
        x0 = x0.wrapping_offset(x1);
        if !gt {
            break; // b.gt 16b
        }
    }
}

/// `32:` of `ipred_dc_128` (`320:`), `ipred_dc_top`, `ipred_v`, `L(ipred_dc_left_w32)`, `L(ipred_dc_w32)`.
#[inline(always)]
unsafe fn ipred_dc_128_w32(mut x0: *mut u8, x1: isize, mut w4: c_int, v0123: uint16x8x4_t) {
    loop {
        vst1q_u16_x4(x0.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        vst1q_u16_x4(x0.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        x0 = x0.wrapping_offset(x1);
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        vst1q_u16_x4(x0.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        vst1q_u16_x4(x0.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        x0 = x0.wrapping_offset(x1);
        if !gt {
            break; // b.gt 32b
        }
    }
}

/// `64:` of `ipred_dc_128` (`640:`), `ipred_dc_top`, `L(ipred_dc_left_w64)`, `L(ipred_dc_w64)`, with the `sub x1,
/// x1, #64` that precedes it.
///
/// The assembly's two row pointers `x0`, `x6` each store the two halves of their row and advance by two rows.
/// Here `x0` stores the first halves of all rows and `x0h` (= `x0` + 64) the second halves, each advancing by
/// the stride, in the assembly's order of stores. The stride is passed twice (`x1`, `x1h`), and the loop is a
/// function of its own that the entry points branch to: so the compiler keeps two pointers with a
/// post-incremented `st1` each (ten instructions per four rows, as the assembly); with one stride, or inlined,
/// it forms every second address with an add (thirteen or fourteen).
///
/// # Safety
/// Writes 64 bytes at `x0`, `x0 + x1`, ... and at `x0h`, `x0h + x1h`, ... for `w4` rows.
#[inline(never)]
#[allow(improper_ctypes_definitions)] // NEON register arguments (v0-v7 under the C ABI), not a foreign interface
unsafe extern "C" fn ipred_dc_128_w64(
    mut x0: *mut u8,
    x1: isize,
    mut x0h: *mut u8,
    x1h: isize,
    mut w4: c_int,
    v0123: uint16x8x4_t,
) {
    loop {
        vst1q_u16_x4(x0.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        x0 = x0.wrapping_offset(x1);
        vst1q_u16_x4(x0.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        x0 = x0.wrapping_offset(x1);
        vst1q_u16_x4(x0h.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        x0h = x0h.wrapping_offset(x1h);
        vst1q_u16_x4(x0h.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        x0h = x0h.wrapping_offset(x1h);
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        vst1q_u16_x4(x0.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        x0 = x0.wrapping_offset(x1);
        vst1q_u16_x4(x0.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        x0 = x0.wrapping_offset(x1);
        vst1q_u16_x4(x0h.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
        x0h = x0h.wrapping_offset(x1h);
        vst1q_u16_x4(x0h.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], x1
        x0h = x0h.wrapping_offset(x1h);
        if !gt {
            break; // b.gt 64b
        }
    }
}

/// `64:` of `ipred_v` (`640:`), with the `sub x1, x1, #64` that precedes it; as [`ipred_dc_128_w64`], with
/// `v0`-`v3` for the first and `v4`-`v7` for the second half of a row.
///
/// # Safety
/// Writes 64 bytes at `x0`, `x0 + x1`, ... and at `x0h`, `x0h + x1h`, ... for `w4` rows.
#[inline(never)]
#[allow(improper_ctypes_definitions)] // NEON register arguments (v0-v7 under the C ABI), not a foreign interface
unsafe extern "C" fn ipred_v_w64(
    mut x0: *mut u8,
    x1: isize,
    mut x0h: *mut u8,
    x1h: isize,
    mut w4: c_int,
    v0123: uint16x8x4_t,
    v4567: uint16x8x4_t,
) {
    loop {
        vst1q_u16_x4(x0.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        x0 = x0.wrapping_offset(x1);
        vst1q_u16_x4(x0.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        x0 = x0.wrapping_offset(x1);
        vst1q_u16_x4(x0h.cast(), v4567); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], x1
        x0h = x0h.wrapping_offset(x1h);
        vst1q_u16_x4(x0h.cast(), v4567); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], x1
        x0h = x0h.wrapping_offset(x1h);
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        vst1q_u16_x4(x0.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        x0 = x0.wrapping_offset(x1);
        vst1q_u16_x4(x0.cast(), v0123); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x6], #64
        x0 = x0.wrapping_offset(x1);
        vst1q_u16_x4(x0h.cast(), v4567); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], x1
        x0h = x0h.wrapping_offset(x1h);
        vst1q_u16_x4(x0h.cast(), v4567); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x6], x1
        x0h = x0h.wrapping_offset(x1h);
        if !gt {
            break; // b.gt 64b
        }
    }
}

/// `ipred_dc_128_16bpc_neon`
///
/// # Safety
/// Writes `dst` rows `0..height`, columns `0..width` (16-bit pixels, `stride` in bytes); reads no memory.
/// `width` is 4, 8, 16, 32 or 64 and `height` a multiple of 4.
pub unsafe extern "C" fn dav1d_ipred_dc_128_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    _topleft: *const u16,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
) {
    let (x0, x1, w4) = (dst.cast::<u8>(), stride, height);
    let w8 = bitdepth_max; // ldr w8, [sp]
    let v0 = vdupq_n_u16(w8 as u16); // dup v0.8h, w8
    let x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    let v0 = vrshrq_n_u16::<1>(v0); // urshr v0.8h, v0.8h, #1
    match pa_tbl_idx(width) {
        4 => ipred_dc_128_w4(x0, x6, x1, w4, vget_low_u16(v0)), // 40:
        3 => ipred_dc_128_w8(x0, x6, x1, w4, v0),               // 80:
        2 => ipred_dc_128_w16(x0, stride, w4, uint16x8x2_t(v0, v0)), // 160: mov v1.16b, v0.16b
        1 => ipred_dc_128_w32(x0, stride, w4, uint16x8x4_t(v0, v0, v0, v0)), // 320: mov v1/v2/v3.16b, v0.16b
        0 => {
            // 640: mov v1/v2/v3.16b, v0.16b; sub x1, x1, #64
            ipred_dc_128_w64(
                x0,
                stride,
                x0.wrapping_add(64),
                stride,
                w4,
                uint16x8x4_t(v0, v0, v0, v0),
            )
        }
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `ipred_v_16bpc_neon`
///
/// # Safety
/// Reads `topleft[1..=width]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_v_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let (x0, x1, w4) = (dst.cast::<u8>(), stride, height);
    let x2 = topleft.cast::<u8>().add(2); // add x2, x2, #2
    let x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    match pa_tbl_idx(width) {
        4 => ipred_dc_128_w4(x0, x6, x1, w4, vld1_u16(x2.cast())), // 40: ld1 {v0.4h}, [x2]; 4:
        3 => ipred_dc_128_w8(x0, x6, x1, w4, vld1q_u16(x2.cast())), // 80: ld1 {v0.8h}, [x2]; 8:
        2 => ipred_dc_128_w16(x0, stride, w4, vld1q_u16_x2(x2.cast())), // 160: ld1 {v0.8h, v1.8h}, [x2]; 16:
        1 => ipred_dc_128_w32(x0, stride, w4, vld1q_u16_x4(x2.cast())), // 320: ld1 {v0.8h, ..., v3.8h}, [x2]; 32:
        0 => {
            // 640:
            let v0123 = vld1q_u16_x4(x2.cast()); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], #64
            let v4567 = vld1q_u16_x4(x2.add(64).cast()); // ld1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x2]
            ipred_v_w64(x0, stride, x0.wrapping_add(64), stride, w4, v0123, v4567)
            // sub x1, x1, #64; 64:
        }
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `160:`, `320:`, `640:` of `ipred_h_16bpc_neon`. A function of its own that the entry point branches to (with
/// the arguments where they are): with all five loops in one function the compiler steps through the rows of
/// the loops for 4 and 8 columns with an add each (three more instructions per four rows).
///
/// The loops use one row pointer that advances by the stride and store a whole row at a time (the assembly's
/// `x0` and `x6` = `x0` + stride advance by twice the stride, and it stores the upper parts of two rows before
/// their first 16 bytes).
///
/// # Safety
/// As the entry point, for `width` >= 16.
#[inline(never)]
unsafe extern "C" fn ipred_h_160(dst: *mut u16, stride: isize, topleft: *const u16, width: c_int, height: c_int) {
    let (mut x0, mut w4) = (dst.cast::<u8>(), height);
    let mut x2 = topleft.cast::<u8>().sub(8); // sub x2, x2, #8; mov x7, #-8
    match pa_tbl_idx(width) {
        2 => loop {
            // 160: 16:
            let uint16x8x4_t(v0, v1, v2, v3) = vld4q_dup_u16(x2.cast()); // ld4r {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], x7
            x2 = x2.wrapping_sub(8);
            vst1q_u16(x0.add(16).cast(), v3); // str q3, [x0, #16]
            vst1q_u16(x0.cast(), v3); // st1 {v3.8h}, [x0], x1
            x0 = x0.wrapping_offset(stride);
            vst1q_u16(x0.add(16).cast(), v2); // str q2, [x6, #16]
            vst1q_u16(x0.cast(), v2); // st1 {v2.8h}, [x6], x1
            x0 = x0.wrapping_offset(stride);
            let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
            vst1q_u16(x0.add(16).cast(), v1); // str q1, [x0, #16]
            vst1q_u16(x0.cast(), v1); // st1 {v1.8h}, [x0], x1
            x0 = x0.wrapping_offset(stride);
            vst1q_u16(x0.add(16).cast(), v0); // str q0, [x6, #16]
            vst1q_u16(x0.cast(), v0); // st1 {v0.8h}, [x6], x1
            x0 = x0.wrapping_offset(stride);
            if !gt {
                break; // b.gt 16b
            }
        },
        1 => loop {
            // 320: 32:
            let uint16x8x4_t(v0, v1, v2, v3) = vld4q_dup_u16(x2.cast()); // ld4r {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], x7
            x2 = x2.wrapping_sub(8);
            vst1q_u16(x0.add(16).cast(), v3); // str q3, [x0, #16]
            vst1q_u16(x0.add(32).cast(), v3); // stp q3, q3, [x0, #32]
            vst1q_u16(x0.add(48).cast(), v3);
            vst1q_u16(x0.cast(), v3); // st1 {v3.8h}, [x0], x1
            x0 = x0.wrapping_offset(stride);
            vst1q_u16(x0.add(16).cast(), v2); // str q2, [x6, #16]
            vst1q_u16(x0.add(32).cast(), v2); // stp q2, q2, [x6, #32]
            vst1q_u16(x0.add(48).cast(), v2);
            vst1q_u16(x0.cast(), v2); // st1 {v2.8h}, [x6], x1
            x0 = x0.wrapping_offset(stride);
            let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
            vst1q_u16(x0.add(16).cast(), v1); // str q1, [x0, #16]
            vst1q_u16(x0.add(32).cast(), v1); // stp q1, q1, [x0, #32]
            vst1q_u16(x0.add(48).cast(), v1);
            vst1q_u16(x0.cast(), v1); // st1 {v1.8h}, [x0], x1
            x0 = x0.wrapping_offset(stride);
            vst1q_u16(x0.add(16).cast(), v0); // str q0, [x6, #16]
            vst1q_u16(x0.add(32).cast(), v0); // stp q0, q0, [x6, #32]
            vst1q_u16(x0.add(48).cast(), v0);
            vst1q_u16(x0.cast(), v0); // st1 {v0.8h}, [x6], x1
            x0 = x0.wrapping_offset(stride);
            if !gt {
                break; // b.gt 32b
            }
        },
        0 => loop {
            // 640: 64:
            let uint16x8x4_t(v0, v1, v2, v3) = vld4q_dup_u16(x2.cast()); // ld4r {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], x7
            x2 = x2.wrapping_sub(8);
            vst1q_u16(x0.add(16).cast(), v3); // str q3, [x0, #16]
            vst1q_u16(x0.add(32).cast(), v3); // stp q3, q3, [x0, #32]
            vst1q_u16(x0.add(48).cast(), v3);
            vst1q_u16(x0.add(64).cast(), v3); // stp q3, q3, [x0, #64]
            vst1q_u16(x0.add(80).cast(), v3);
            vst1q_u16(x0.add(96).cast(), v3); // stp q3, q3, [x0, #96]
            vst1q_u16(x0.add(112).cast(), v3);
            vst1q_u16(x0.cast(), v3); // st1 {v3.8h}, [x0], x1
            x0 = x0.wrapping_offset(stride);
            vst1q_u16(x0.add(16).cast(), v2); // str q2, [x6, #16]
            vst1q_u16(x0.add(32).cast(), v2); // stp q2, q2, [x6, #32]
            vst1q_u16(x0.add(48).cast(), v2);
            vst1q_u16(x0.add(64).cast(), v2); // stp q2, q2, [x6, #64]
            vst1q_u16(x0.add(80).cast(), v2);
            vst1q_u16(x0.add(96).cast(), v2); // stp q2, q2, [x6, #96]
            vst1q_u16(x0.add(112).cast(), v2);
            vst1q_u16(x0.cast(), v2); // st1 {v2.8h}, [x6], x1
            x0 = x0.wrapping_offset(stride);
            let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
            vst1q_u16(x0.add(16).cast(), v1); // str q1, [x0, #16]
            vst1q_u16(x0.add(32).cast(), v1); // stp q1, q1, [x0, #32]
            vst1q_u16(x0.add(48).cast(), v1);
            vst1q_u16(x0.add(64).cast(), v1); // stp q1, q1, [x0, #64]
            vst1q_u16(x0.add(80).cast(), v1);
            vst1q_u16(x0.add(96).cast(), v1); // stp q1, q1, [x0, #96]
            vst1q_u16(x0.add(112).cast(), v1);
            vst1q_u16(x0.cast(), v1); // st1 {v1.8h}, [x0], x1
            x0 = x0.wrapping_offset(stride);
            vst1q_u16(x0.add(16).cast(), v0); // str q0, [x6, #16]
            vst1q_u16(x0.add(32).cast(), v0); // stp q0, q0, [x6, #32]
            vst1q_u16(x0.add(48).cast(), v0);
            vst1q_u16(x0.add(64).cast(), v0); // stp q0, q0, [x6, #64]
            vst1q_u16(x0.add(80).cast(), v0);
            vst1q_u16(x0.add(96).cast(), v0); // stp q0, q0, [x6, #96]
            vst1q_u16(x0.add(112).cast(), v0);
            vst1q_u16(x0.cast(), v0); // st1 {v0.8h}, [x6], x1
            x0 = x0.wrapping_offset(stride);
            if !gt {
                break; // b.gt 64b
            }
        },
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `ipred_h_16bpc_neon`
///
/// # Safety
/// Reads `topleft[-height..0]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_h_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let (mut x0, x1, mut w4) = (dst.cast::<u8>(), stride, height);
    let mut x2 = topleft.cast::<u8>().sub(8); // sub x2, x2, #8; mov x7, #-8
    let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    match pa_tbl_idx(width) {
        4 => loop {
            // 40: 4:
            let uint16x8x4_t(v0, v1, v2, v3) = vld4q_dup_u16(x2.cast()); // ld4r {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], x7
            x2 = x2.wrapping_sub(8);
            vst1_u16(x0.cast(), vget_low_u16(v3)); // st1 {v3.4h}, [x0], x1
            vst1_u16(x6.cast(), vget_low_u16(v2)); // st1 {v2.4h}, [x6], x1
            let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
            vst1_u16(x0.wrapping_offset(x1).cast(), vget_low_u16(v1)); // st1 {v1.4h}, [x0], x1
            vst1_u16(x6.wrapping_offset(x1).cast(), vget_low_u16(v0)); // st1 {v0.4h}, [x6], x1
            x0 = x0.wrapping_offset(x1 << 1);
            x6 = x6.wrapping_offset(x1 << 1);
            if !gt {
                break; // b.gt 4b
            }
        },
        3 => loop {
            // 80: 8:
            let uint16x8x4_t(v0, v1, v2, v3) = vld4q_dup_u16(x2.cast()); // ld4r {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], x7
            x2 = x2.wrapping_sub(8);
            vst1q_u16(x0.cast(), v3); // st1 {v3.8h}, [x0], x1
            vst1q_u16(x6.cast(), v2); // st1 {v2.8h}, [x6], x1
            let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
            vst1q_u16(x0.wrapping_offset(x1).cast(), v1); // st1 {v1.8h}, [x0], x1
            vst1q_u16(x6.wrapping_offset(x1).cast(), v0); // st1 {v0.8h}, [x6], x1
            x0 = x0.wrapping_offset(x1 << 1);
            x6 = x6.wrapping_offset(x1 << 1);
            if !gt {
                break; // b.gt 8b
            }
        },
        0..=2 => ipred_h_160(dst, stride, topleft, width, height), // 160: 320: 640:
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `ipred_dc_top_16bpc_neon`
///
/// # Safety
/// Reads `topleft[1..=width]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_dc_top_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let (x0, x1, w4) = (dst.cast::<u8>(), stride, height);
    let x2 = topleft.cast::<u8>().add(2); // add x2, x2, #2
    let x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    match pa_tbl_idx(width) {
        4 => {
            // 40:
            let v0 = vld1_u16(x2.cast()); // ld1 {v0.4h}, [x2]
            let v0 = pa_addv_4h(v0); // addv h0, v0.4h
            let v0 = pa_urshr_dup::<2>(v0); // urshr v0.4h, v0.4h, #2; dup v0.4h, v0.h[0]
            ipred_dc_128_w4(x0, x6, x1, w4, vget_low_u16(v0)) // 4:
        }
        3 => {
            // 80:
            let v0 = vld1q_u16(x2.cast()); // ld1 {v0.8h}, [x2]
            let v0 = pa_addv_8h(v0); // addv h0, v0.8h
            let v0 = pa_urshr_dup::<3>(v0); // urshr v0.4h, v0.4h, #3; dup v0.8h, v0.h[0]
            ipred_dc_128_w8(x0, x6, x1, w4, v0) // 8:
        }
        2 => {
            // 160:
            let uint16x8x2_t(v0, v1) = vld1q_u16_x2(x2.cast()); // ld1 {v0.8h, v1.8h}, [x2]
            let v0 = vpaddq_u16(v0, v1); // addp v0.8h, v0.8h, v1.8h
            let v0 = pa_addv_8h(v0); // addv h0, v0.8h
            let v0 = pa_urshr_dup::<4>(v0); // urshr v2.4h, v0.4h, #4; dup v0.8h, v2.h[0]; dup v1.8h, v2.h[0]
            ipred_dc_128_w16(x0, stride, w4, uint16x8x2_t(v0, v0)) // 16:
        }
        1 => {
            // 320:
            let uint16x8x4_t(v0, v1, v2, v3) = vld1q_u16_x4(x2.cast()); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x2]
            let v0 = vpaddq_u16(v0, v1); // addp v0.8h, v0.8h, v1.8h
            let v2 = vpaddq_u16(v2, v3); // addp v2.8h, v2.8h, v3.8h
            let v0 = vpaddq_u16(v0, v2); // addp v0.8h, v0.8h, v2.8h
            let v0 = pa_uaddlv_8h(v0); // uaddlv s0, v0.8h
            let v0 = pa_rshrn_dup::<5>(v0); // rshrn v4.4h, v0.4s, #5; dup v0.8h, v4.h[0] (and v1, v2, v3)
            ipred_dc_128_w32(x0, stride, w4, uint16x8x4_t(v0, v0, v0, v0)) // 32:
        }
        0 => {
            // 640:
            let uint16x8x4_t(v0, v1, v2, v3) = vld1q_u16_x4(x2.cast()); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], #64
            let v0 = vpaddq_u16(v0, v1); // addp v0.8h, v0.8h, v1.8h
            let uint16x8x4_t(v4, v5, v6, v7) = vld1q_u16_x4(x2.add(64).cast()); // ld1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x2]
            let v2 = vpaddq_u16(v2, v3); // addp v2.8h, v2.8h, v3.8h
            let v4 = vpaddq_u16(v4, v5); // addp v4.8h, v4.8h, v5.8h
            let v6 = vpaddq_u16(v6, v7); // addp v6.8h, v6.8h, v7.8h
            let v0 = vpaddq_u16(v0, v2); // addp v0.8h, v0.8h, v2.8h
            let v4 = vpaddq_u16(v4, v6); // addp v4.8h, v4.8h, v6.8h
            let v0 = vpaddq_u16(v0, v4); // addp v0.8h, v0.8h, v4.8h
            let v0 = pa_uaddlv_8h(v0); // uaddlv s0, v0.8h
            let v0 = pa_rshrn_dup::<6>(v0); // rshrn v4.4h, v0.4s, #6; dup v0.8h, v4.h[0] (and v1, v2, v3)
            ipred_dc_128_w64(
                x0,
                stride,
                x0.wrapping_add(64),
                stride,
                w4,
                uint16x8x4_t(v0, v0, v0, v0),
            ) // sub x1, x1, #64; 64:
        }
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `ipred_dc_left_16bpc_neon`
///
/// # Safety
/// Reads `topleft[-height..0]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_dc_left_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let (x0, x1, w3, w4) = (dst.cast::<u8>(), stride, width, height);
    let x2 = topleft.cast::<u8>().sub((w4 as u32 as usize) << 1); // sub x2, x2, w4, uxtw #1
    let x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
                      // br x5: the height
    let v0: uint16x8_t = match pa_tbl_idx(w4) {
        4 => {
            // L(ipred_dc_left_h4):
            let v0 = vld1_u16(x2.cast()); // ld1 {v0.4h}, [x2]
            let v0 = pa_addv_4h(v0); // addv h0, v0.4h
            let v0 = pa_urshr_dup::<2>(v0); // urshr v0.4h, v0.4h, #2; dup v0.8h, v0.h[0]
            v0
        }
        3 => {
            // L(ipred_dc_left_h8):
            let v0 = vld1q_u16(x2.cast()); // ld1 {v0.8h}, [x2]
            let v0 = pa_addv_8h(v0); // addv h0, v0.8h
            let v0 = pa_urshr_dup::<3>(v0); // urshr v0.4h, v0.4h, #3; dup v0.8h, v0.h[0]
            v0
        }
        2 => {
            // L(ipred_dc_left_h16):
            let uint16x8x2_t(v0, v1) = vld1q_u16_x2(x2.cast()); // ld1 {v0.8h, v1.8h}, [x2]
            let v0 = vpaddq_u16(v0, v1); // addp v0.8h, v0.8h, v1.8h
            let v0 = pa_addv_8h(v0); // addv h0, v0.8h
            let v0 = pa_urshr_dup::<4>(v0); // urshr v2.4h, v0.4h, #4; dup v0.8h, v2.h[0]; dup v1.8h, v2.h[0]
            v0
        }
        1 => {
            // L(ipred_dc_left_h32):
            let uint16x8x4_t(v0, v1, v2, v3) = vld1q_u16_x4(x2.cast()); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x2]
            let v0 = vpaddq_u16(v0, v1); // addp v0.8h, v0.8h, v1.8h
            let v2 = vpaddq_u16(v2, v3); // addp v2.8h, v2.8h, v3.8h
            let v0 = vpaddq_u16(v0, v2); // addp v0.8h, v0.8h, v2.8h
            let v0 = vpaddlq_u16(v0); // uaddlp v0.4s, v0.8h
            let v0 = vdupq_n_u32(vaddvq_u32(v0)); // addv s0, v0.4s
            let v0 = pa_rshrn_dup::<5>(v0); // rshrn v4.4h, v0.4s, #5; dup v0.8h, v4.h[0]
            v0
        }
        0 => {
            // L(ipred_dc_left_h64):
            let uint16x8x4_t(v0, v1, v2, v3) = vld1q_u16_x4(x2.cast()); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], #64
            let v0 = vpaddq_u16(v0, v1); // addp v0.8h, v0.8h, v1.8h
            let uint16x8x4_t(v4, v5, v6, v7) = vld1q_u16_x4(x2.add(64).cast()); // ld1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x2]
            let v2 = vpaddq_u16(v2, v3); // addp v2.8h, v2.8h, v3.8h
            let v4 = vpaddq_u16(v4, v5); // addp v4.8h, v4.8h, v5.8h
            let v6 = vpaddq_u16(v6, v7); // addp v6.8h, v6.8h, v7.8h
            let v0 = vpaddq_u16(v0, v2); // addp v0.8h, v0.8h, v2.8h
            let v4 = vpaddq_u16(v4, v6); // addp v4.8h, v4.8h, v6.8h
            let v0 = vpaddq_u16(v0, v4); // addp v0.8h, v0.8h, v4.8h
            let v0 = pa_uaddlv_8h(v0); // uaddlv s0, v0.8h
            let v0 = pa_rshrn_dup::<6>(v0); // rshrn v4.4h, v0.4s, #6; dup v0.8h, v4.h[0]
            v0
        }
        _ => core::hint::unreachable_unchecked(),
    };
    // br x3: the width
    match pa_tbl_idx(w3) {
        4 => ipred_dc_128_w4(x0, x6, x1, w4, vget_low_u16(v0)), // L(ipred_dc_left_w4):
        3 => ipred_dc_128_w8(x0, x6, x1, w4, v0),               // L(ipred_dc_left_w8):
        2 => ipred_dc_128_w16(x0, stride, w4, uint16x8x2_t(v0, v0)), // L(ipred_dc_left_w16): mov v1.16b, v0.16b
        1 => ipred_dc_128_w32(x0, stride, w4, uint16x8x4_t(v0, v0, v0, v0)), // L(ipred_dc_left_w32): mov v1/v2/v3
        0 => {
            // L(ipred_dc_left_w64): mov v1/v2/v3.16b, v0.16b; sub x1, x1, #64
            ipred_dc_128_w64(
                x0,
                stride,
                x0.wrapping_add(64),
                stride,
                w4,
                uint16x8x4_t(v0, v0, v0, v0),
            )
        }
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `ipred_dc_16bpc_neon`
///
/// # Safety
/// Reads `topleft[-height..0]` and `topleft[1..=width]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_dc_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let (x0, x1, w3, w4) = (dst.cast::<u8>(), stride, width, height);
    let x2 = topleft.cast::<u8>().sub((w4 as u32 as usize) << 1); // sub x2, x2, w4, uxtw #1
    let w7 = (w3 + w4) as u32; // add w7, w3, w4 (width + height)
    let v16 = vdupq_n_u32(w7); // dup v16.4s, w7
    let w7 = w7.reverse_bits().leading_zeros(); // rbit w7, w7; clz w7, w7 (ctz(width + height))
    let w7 = (w7 as i32).wrapping_neg(); // neg w7, w7
    let v16 = vshrq_n_u32::<1>(v16); // ushr v16.4s, v16.4s, #1 ((width + height) >> 1)
    let v17 = vdupq_n_s32(w7); // dup v17.4s, w7 (-ctz(width + height))
    let x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
                      // br x5: the height; each label leaves the left sum in s0 and x2 at topleft + 1 pixel
    let (v0, x2): (uint32x4_t, *const u8) = match pa_tbl_idx(w4) {
        4 => {
            // L(ipred_dc_h4):
            let v0 = vld1_u16(x2.cast()); // ld1 {v0.4h}, [x2], #8
            let v0 = pa_uaddlv_4h(v0); // uaddlv s0, v0.4h
            (v0, x2.add(8 + 2)) // add x2, x2, #2
        }
        3 => {
            // L(ipred_dc_h8):
            let v0 = vld1q_u16(x2.cast()); // ld1 {v0.8h}, [x2], #16
            let v0 = pa_uaddlv_8h(v0); // uaddlv s0, v0.8h
            (v0, x2.add(16 + 2)) // add x2, x2, #2
        }
        2 => {
            // L(ipred_dc_h16):
            let uint16x8x2_t(v0, v1) = vld1q_u16_x2(x2.cast()); // ld1 {v0.8h, v1.8h}, [x2], #32
            let v0 = vpaddq_u16(v0, v1); // addp v0.8h, v0.8h, v1.8h
            let v0 = pa_uaddlv_8h(v0); // uaddlv s0, v0.8h
            (v0, x2.add(32 + 2)) // add x2, x2, #2
        }
        1 => {
            // L(ipred_dc_h32):
            let uint16x8x4_t(v0, v1, v2, v3) = vld1q_u16_x4(x2.cast()); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], #64
            let v0 = vpaddq_u16(v0, v1); // addp v0.8h, v0.8h, v1.8h
            let v2 = vpaddq_u16(v2, v3); // addp v2.8h, v2.8h, v3.8h
            let v0 = vpaddq_u16(v0, v2); // addp v0.8h, v0.8h, v2.8h
            let v0 = pa_uaddlv_8h(v0); // uaddlv s0, v0.8h
            (v0, x2.add(64 + 2)) // add x2, x2, #2
        }
        0 => {
            // L(ipred_dc_h64):
            let uint16x8x4_t(v0, v1, v2, v3) = vld1q_u16_x4(x2.cast()); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], #64
            let v0 = vpaddq_u16(v0, v1); // addp v0.8h, v0.8h, v1.8h
            let uint16x8x4_t(v4, v5, v6, v7) = vld1q_u16_x4(x2.add(64).cast()); // ld1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x2], #64
            let v2 = vpaddq_u16(v2, v3); // addp v2.8h, v2.8h, v3.8h
            let v4 = vpaddq_u16(v4, v5); // addp v4.8h, v4.8h, v5.8h
            let v6 = vpaddq_u16(v6, v7); // addp v6.8h, v6.8h, v7.8h
            let v0 = vpaddq_u16(v0, v2); // addp v0.8h, v0.8h, v2.8h
            let v4 = vpaddq_u16(v4, v6); // addp v4.8h, v4.8h, v6.8h
            let v0 = vpaddq_u16(v0, v4); // addp v0.8h, v0.8h, v4.8h
            let v0 = pa_uaddlv_8h(v0); // uaddlv s0, v0.8h
            (v0, x2.add(128 + 2)) // add x2, x2, #2
        }
        _ => core::hint::unreachable_unchecked(),
    };
    // br x3: the width. The code from `1:` on stands on both sides of `b.eq 1f`: where the paths join again, the
    // compiler computes the multiplication on both and selects (several instructions more for the square blocks).
    match pa_tbl_idx(w3) {
        4 => {
            // L(ipred_dc_w4):
            let v1 = vld1_u16(x2.cast()); // ld1 {v1.4h}, [x2]
            let v0 = pa_add_uaddlv(v0, v16); // add v0.2s, v0.2s, v16.2s
            let v1 = pa_uaddlv_4h(v1); // uaddlv s1, v1.4h
            let v0 = pa_add_uaddlv(v0, v1); // add v0.2s, v0.2s, v1.2s
            let v0 = vshlq_u32(v0, v17); // ushl v0.2s, v0.2s, v17.2s
            if w4 == 4 {
                // cmp w4, #4; b.eq 1f
                let v0 = vdupq_laneq_u16::<0>(vreinterpretq_u16_u32(v0)); // 1:
                return ipred_dc_128_w4(x0, x6, x1, w4, vget_low_u16(v0)); // 2:
            }
            // h = 8/16: cmp w4, #16
            let w16: u32 = if w4 == 16 { 0x6667 } else { 0xAAAB }; // mov w16, #0x6667; mov w17, #0xAAAB; csel w16, w16, w17, eq
            let v0 = pa_mul_ushr17(v0, w16); // dup v16.2s, w16; mul v0.2s, v0.2s, v16.2s; ushr v0.2s, v0.2s, #17
            let v0 = vdupq_laneq_u16::<0>(vreinterpretq_u16_u32(v0)); // 1: dup v0.4h, v0.h[0]
            ipred_dc_128_w4(x0, x6, x1, w4, vget_low_u16(v0)) // 2:
        }
        3 => {
            // L(ipred_dc_w8):
            let v1 = vld1q_u16(x2.cast()); // ld1 {v1.8h}, [x2]
            let v0 = pa_add_uaddlv(v0, v16); // add v0.2s, v0.2s, v16.2s
            let v1 = pa_uaddlv_8h(v1); // uaddlv s1, v1.8h
            let v0 = pa_add_uaddlv(v0, v1); // add v0.2s, v0.2s, v1.2s
            let v0 = vshlq_u32(v0, v17); // ushl v0.2s, v0.2s, v17.2s
            if w4 == 8 {
                // cmp w4, #8; b.eq 1f
                let v0 = vdupq_laneq_u16::<0>(vreinterpretq_u16_u32(v0)); // 1:
                return ipred_dc_128_w8(x0, x6, x1, w4, v0); // 2:
            }
            // h = 4/16/32: cmp w4, #32
            let w16: u32 = if w4 == 32 { 0x6667 } else { 0xAAAB }; // mov w16, #0x6667; mov w17, #0xAAAB; csel w16, w16, w17, eq
            let v0 = pa_mul_ushr17(v0, w16); // dup v16.2s, w16; mul v0.2s, v0.2s, v16.2s; ushr v0.2s, v0.2s, #17
            let v0 = vdupq_laneq_u16::<0>(vreinterpretq_u16_u32(v0)); // 1: dup v0.8h, v0.h[0]
            ipred_dc_128_w8(x0, x6, x1, w4, v0) // 2:
        }
        2 => {
            // L(ipred_dc_w16):
            let uint16x8x2_t(v1, v2) = vld1q_u16_x2(x2.cast()); // ld1 {v1.8h, v2.8h}, [x2]
            let v0 = pa_add_uaddlv(v0, v16); // add v0.2s, v0.2s, v16.2s
            let v1 = vpaddq_u16(v1, v2); // addp v1.8h, v1.8h, v2.8h
            let v1 = pa_uaddlv_8h(v1); // uaddlv s1, v1.8h
            let v0 = pa_add_uaddlv(v0, v1); // add v0.2s, v0.2s, v1.2s
            let v4 = vshlq_u32(v0, v17); // ushl v4.2s, v0.2s, v17.2s
            if w4 == 16 {
                // cmp w4, #16; b.eq 1f
                let v0 = vdupq_laneq_u16::<0>(vreinterpretq_u16_u32(v4)); // 1:
                return ipred_dc_128_w16(x0, stride, w4, uint16x8x2_t(v0, v0)); // 2:
            }
            // h = 4/8/32/64: tst w4, #(32+16+8)
            let w16: u32 = if w4 & (32 + 16 + 8) == 0 { 0x6667 } else { 0xAAAB }; // mov w16, #0x6667; mov w17, #0xAAAB; csel w16, w16, w17, eq
            let v4 = pa_mul_ushr17(v4, w16); // dup v16.2s, w16; mul v4.2s, v4.2s, v16.2s; ushr v4.2s, v4.2s, #17
            let v0 = vdupq_laneq_u16::<0>(vreinterpretq_u16_u32(v4)); // 1: dup v0.8h, v4.h[0]; dup v1.8h, v4.h[0]
            ipred_dc_128_w16(x0, stride, w4, uint16x8x2_t(v0, v0)) // 2:
        }
        1 => {
            // L(ipred_dc_w32):
            let uint16x8x4_t(v1, v2, v3, v4) = vld1q_u16_x4(x2.cast()); // ld1 {v1.8h, v2.8h, v3.8h, v4.8h}, [x2]
            let v0 = pa_add_uaddlv(v0, v16); // add v0.2s, v0.2s, v16.2s
            let v1 = vpaddq_u16(v1, v2); // addp v1.8h, v1.8h, v2.8h
            let v3 = vpaddq_u16(v3, v4); // addp v3.8h, v3.8h, v4.8h
            let v1 = vpaddq_u16(v1, v3); // addp v1.8h, v1.8h, v3.8h
            let v1 = pa_uaddlv_8h(v1); // uaddlv s1, v1.8h
            let v0 = pa_add_uaddlv(v0, v1); // add v0.2s, v0.2s, v1.2s
            let v4 = vshlq_u32(v0, v17); // ushl v4.2s, v0.2s, v17.2s
            if w4 == 32 {
                // cmp w4, #32; b.eq 1f
                let v0 = vdupq_laneq_u16::<0>(vreinterpretq_u16_u32(v4)); // 1:
                return ipred_dc_128_w32(x0, stride, w4, uint16x8x4_t(v0, v0, v0, v0));
                // 2:
            }
            // h = 8/16/64: cmp w4, #8
            let w16: u32 = if w4 == 8 { 0x6667 } else { 0xAAAB }; // mov w16, #0x6667; mov w17, #0xAAAB; csel w16, w16, w17, eq
            let v4 = pa_mul_ushr17(v4, w16); // dup v16.2s, w16; mul v4.2s, v4.2s, v16.2s; ushr v4.2s, v4.2s, #17
            let v0 = vdupq_laneq_u16::<0>(vreinterpretq_u16_u32(v4)); // 1: dup v0.8h, v4.h[0] (and v1, v2, v3)
            ipred_dc_128_w32(x0, stride, w4, uint16x8x4_t(v0, v0, v0, v0)) // 2:
        }
        0 => {
            // L(ipred_dc_w64):
            let uint16x8x4_t(v1, v2, v3, v4) = vld1q_u16_x4(x2.cast()); // ld1 {v1.8h, v2.8h, v3.8h, v4.8h}, [x2], #64
            let v0 = pa_add_uaddlv(v0, v16); // add v0.2s, v0.2s, v16.2s
            let v1 = vpaddq_u16(v1, v2); // addp v1.8h, v1.8h, v2.8h
            let uint16x8x4_t(v20, v21, v22, v23) = vld1q_u16_x4(x2.add(64).cast()); // ld1 {v20.8h, v21.8h, v22.8h, v23.8h}, [x2]
            let v3 = vpaddq_u16(v3, v4); // addp v3.8h, v3.8h, v4.8h
            let v20 = vpaddq_u16(v20, v21); // addp v20.8h, v20.8h, v21.8h
            let v22 = vpaddq_u16(v22, v23); // addp v22.8h, v22.8h, v23.8h
            let v1 = vpaddq_u16(v1, v3); // addp v1.8h, v1.8h, v3.8h
            let v20 = vpaddq_u16(v20, v22); // addp v20.8h, v20.8h, v22.8h
            let v1 = vpaddq_u16(v1, v20); // addp v1.8h, v1.8h, v20.8h
            let v1 = pa_uaddlv_8h(v1); // uaddlv s1, v1.8h
            let v0 = pa_add_uaddlv(v0, v1); // add v0.2s, v0.2s, v1.2s
            let v4 = vshlq_u32(v0, v17); // ushl v4.2s, v0.2s, v17.2s
            if w4 == 64 {
                // cmp w4, #64; b.eq 1f
                let v0 = vdupq_laneq_u16::<0>(vreinterpretq_u16_u32(v4)); // 1:
                return ipred_dc_128_w64(
                    x0,
                    stride,
                    x0.wrapping_add(64),
                    stride,
                    w4,
                    uint16x8x4_t(v0, v0, v0, v0),
                ); // 2:
            }
            // h = 16/32: cmp w4, #16
            let w16: u32 = if w4 == 16 { 0x6667 } else { 0xAAAB }; // mov w16, #0x6667; mov w17, #0xAAAB; csel w16, w16, w17, eq
            let v4 = pa_mul_ushr17(v4, w16); // dup v16.2s, w16; mul v4.2s, v4.2s, v16.2s; ushr v4.2s, v4.2s, #17
            let v0 = vdupq_laneq_u16::<0>(vreinterpretq_u16_u32(v4)); // 1: sub x1, x1, #64; dup v0.8h, v4.h[0] (and v1, v2, v3)
            ipred_dc_128_w64(
                x0,
                stride,
                x0.wrapping_add(64),
                stride,
                w4,
                uint16x8x4_t(v0, v0, v0, v0),
            ) // 2:
        }
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `ipred_paeth_16bpc_neon`
///
/// # Safety
/// Reads `topleft[-height..=width]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_paeth_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let (mut x0, x1, mut w3, mut w4) = (dst.cast::<u8>(), stride, width, height);
    let x2 = topleft.cast::<u8>();
    let v4 = vld1q_dup_u16(x2.cast()); // ld1r {v4.8h}, [x2]
    let mut x8 = x2.add(2); // add x8, x2, #2
    let mut x2 = x2.sub(8); // sub x2, x2, #8; mov x7, #-8
    let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let mut x1 = x1 << 1; // lsl x1, x1, #1
    match pa_tbl_idx(width) {
        4 => {
            // 40:
            let v5 = pa_ld1r_2d(x8); // ld1r {v5.2d}, [x8]
            let v6 = vsubq_u16(v5, v4); // sub v6.8h, v5.8h, v4.8h (top - topleft)
            loop {
                // 4:
                let uint16x4x4_t(v0, v1, v2, v3) = vld4_dup_u16(x2.cast()); // ld4r {v0.4h, v1.4h, v2.4h, v3.4h}, [x2], x7
                x2 = x2.wrapping_sub(8);
                let v0 = vcombine_u16(v0, v1); // zip1 v0.2d, v0.2d, v1.2d
                let v2 = vcombine_u16(v2, v3); // zip1 v2.2d, v2.2d, v3.2d
                let v16 = vaddq_u16(v6, v0); // add v16.8h, v6.8h, v0.8h (base)
                let v17 = vaddq_u16(v6, v2); // add v17.8h, v6.8h, v2.8h
                let v20 = pa_sabd(v5, v16); // sabd v20.8h, v5.8h, v16.8h (tdiff)
                let v21 = pa_sabd(v5, v17); // sabd v21.8h, v5.8h, v17.8h
                let v22 = pa_sabd(v4, v16); // sabd v22.8h, v4.8h, v16.8h (tldiff)
                let v23 = pa_sabd(v4, v17); // sabd v23.8h, v4.8h, v17.8h
                let v16 = pa_sabd(v0, v16); // sabd v16.8h, v0.8h, v16.8h (ldiff)
                let v17 = pa_sabd(v2, v17); // sabd v17.8h, v2.8h, v17.8h
                let v18 = vminq_u16(v20, v22); // umin v18.8h, v20.8h, v22.8h (min(tdiff, tldiff))
                let v19 = vminq_u16(v21, v23); // umin v19.8h, v21.8h, v23.8h
                let v20 = pa_cmge(v22, v20); // cmge v20.8h, v22.8h, v20.8h (tldiff >= tdiff)
                let v21 = pa_cmge(v23, v21); // cmge v21.8h, v23.8h, v21.8h
                let v16 = pa_cmge(v18, v16); // cmge v16.8h, v18.8h, v16.8h (min(tdiff, tldiff) >= ldiff)
                let v17 = pa_cmge(v19, v17); // cmge v17.8h, v19.8h, v17.8h
                let v21 = vbslq_u16(v21, v5, v4); // bsl v21.16b, v5.16b, v4.16b (tdiff <= tldiff ? top : topleft)
                let v20 = vbslq_u16(v20, v5, v4); // bsl v20.16b, v5.16b, v4.16b
                let v21 = vbslq_u16(v17, v2, v21); // bit v21.16b, v2.16b, v17.16b (ldiff <= min ? left : ...)
                let v20 = vbslq_u16(v16, v0, v20); // bit v20.16b, v0.16b, v16.16b
                vst1_u16(x0.cast(), vget_high_u16(v21)); // st1 {v21.d}[1], [x0], x1
                vst1_u16(x6.cast(), vget_low_u16(v21)); // st1 {v21.d}[0], [x6], x1
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                vst1_u16(x0.wrapping_offset(x1).cast(), vget_high_u16(v20)); // st1 {v20.d}[1], [x0], x1
                vst1_u16(x6.wrapping_offset(x1).cast(), vget_low_u16(v20)); // st1 {v20.d}[0], [x6], x1
                x0 = x0.wrapping_offset(x1 << 1);
                x6 = x6.wrapping_offset(x1 << 1);
                if !gt {
                    break; // b.gt 4b
                }
            }
        }
        0..=3 => {
            // 80: 160: 320: 640:
            let mut v5 = vld1q_u16(x8.cast()); // ld1 {v5.8h}, [x8], #16
            x8 = x8.wrapping_add(16);
            let w9 = w3; // mov w9, w3
                         // Set up pointers for four rows in parallel; x0, x6, x5, x10
            let mut x5 = x0.wrapping_offset(x1); // add x5, x0, x1
            let mut x10 = x6.wrapping_offset(x1); // add x10, x6, x1
            x1 <<= 1; // lsl x1, x1, #1
            x1 -= (w3 as u32 as isize) << 1; // sub x1, x1, w3, uxtw #1
            loop {
                // 1:
                let uint16x8x4_t(v0, v1, v2, v3) = vld4q_dup_u16(x2.cast()); // ld4r {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], x7
                x2 = x2.wrapping_sub(8);
                loop {
                    // 2:
                    let v6 = vsubq_u16(v5, v4); // sub v6.8h, v5.8h, v4.8h (top - topleft)
                    let v16 = vaddq_u16(v6, v0); // add v16.8h, v6.8h, v0.8h (base)
                    let v17 = vaddq_u16(v6, v1); // add v17.8h, v6.8h, v1.8h
                    let v18 = vaddq_u16(v6, v2); // add v18.8h, v6.8h, v2.8h
                    let v19 = vaddq_u16(v6, v3); // add v19.8h, v6.8h, v3.8h
                    let v20 = pa_sabd(v5, v16); // sabd v20.8h, v5.8h, v16.8h (tdiff)
                    let v21 = pa_sabd(v5, v17); // sabd v21.8h, v5.8h, v17.8h
                    let v22 = pa_sabd(v5, v18); // sabd v22.8h, v5.8h, v18.8h
                    let v23 = pa_sabd(v5, v19); // sabd v23.8h, v5.8h, v19.8h
                    let v24 = pa_sabd(v4, v16); // sabd v24.8h, v4.8h, v16.8h (tldiff)
                    let v25 = pa_sabd(v4, v17); // sabd v25.8h, v4.8h, v17.8h
                    let v26 = pa_sabd(v4, v18); // sabd v26.8h, v4.8h, v18.8h
                    let v27 = pa_sabd(v4, v19); // sabd v27.8h, v4.8h, v19.8h
                    let v16 = pa_sabd(v0, v16); // sabd v16.8h, v0.8h, v16.8h (ldiff)
                    let v17 = pa_sabd(v1, v17); // sabd v17.8h, v1.8h, v17.8h
                    let v18 = pa_sabd(v2, v18); // sabd v18.8h, v2.8h, v18.8h
                    let v19 = pa_sabd(v3, v19); // sabd v19.8h, v3.8h, v19.8h
                    let v28 = vminq_u16(v20, v24); // umin v28.8h, v20.8h, v24.8h (min(tdiff, tldiff))
                    let v29 = vminq_u16(v21, v25); // umin v29.8h, v21.8h, v25.8h
                    let v30 = vminq_u16(v22, v26); // umin v30.8h, v22.8h, v26.8h
                    let v31 = vminq_u16(v23, v27); // umin v31.8h, v23.8h, v27.8h
                    let v20 = pa_cmge(v24, v20); // cmge v20.8h, v24.8h, v20.8h (tldiff >= tdiff)
                    let v21 = pa_cmge(v25, v21); // cmge v21.8h, v25.8h, v21.8h
                    let v22 = pa_cmge(v26, v22); // cmge v22.8h, v26.8h, v22.8h
                    let v23 = pa_cmge(v27, v23); // cmge v23.8h, v27.8h, v23.8h
                    let v16 = pa_cmge(v28, v16); // cmge v16.8h, v28.8h, v16.8h (min(tdiff, tldiff) >= ldiff)
                    let v17 = pa_cmge(v29, v17); // cmge v17.8h, v29.8h, v17.8h
                    let v18 = pa_cmge(v30, v18); // cmge v18.8h, v30.8h, v18.8h
                    let v19 = pa_cmge(v31, v19); // cmge v19.8h, v31.8h, v19.8h
                    let v23 = vbslq_u16(v23, v5, v4); // bsl v23.16b, v5.16b, v4.16b (tdiff <= tldiff ? top : topleft)
                    let v22 = vbslq_u16(v22, v5, v4); // bsl v22.16b, v5.16b, v4.16b
                    let v21 = vbslq_u16(v21, v5, v4); // bsl v21.16b, v5.16b, v4.16b
                    let v20 = vbslq_u16(v20, v5, v4); // bsl v20.16b, v5.16b, v4.16b
                    let v23 = vbslq_u16(v19, v3, v23); // bit v23.16b, v3.16b, v19.16b (ldiff <= min ? left : ...)
                    let v22 = vbslq_u16(v18, v2, v22); // bit v22.16b, v2.16b, v18.16b
                    let v21 = vbslq_u16(v17, v1, v21); // bit v21.16b, v1.16b, v17.16b
                    let v20 = vbslq_u16(v16, v0, v20); // bit v20.16b, v0.16b, v16.16b
                    vst1q_u16(x0.cast(), v23); // st1 {v23.8h}, [x0], #16
                    x0 = x0.wrapping_add(16);
                    vst1q_u16(x6.cast(), v22); // st1 {v22.8h}, [x6], #16
                    x6 = x6.wrapping_add(16);
                    let gt = pa_subs_gt(&mut w3, 8); // subs w3, w3, #8
                    vst1q_u16(x5.cast(), v21); // st1 {v21.8h}, [x5], #16
                    x5 = x5.wrapping_add(16);
                    vst1q_u16(x10.cast(), v20); // st1 {v20.8h}, [x10], #16
                    x10 = x10.wrapping_add(16);
                    if !gt {
                        break; // b.le 8f
                    }
                    v5 = vld1q_u16(x8.cast()); // ld1 {v5.8h}, [x8], #16
                    x8 = x8.wrapping_add(16);
                    // b 2b
                }
                // 8:
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                if !gt {
                    break; // b.le 9f
                }
                // End of horizontal loop, move pointers to next four rows
                x8 = x8.wrapping_sub((w9 as u32 as usize) << 1); // sub x8, x8, w9, uxtw #1
                x0 = x0.wrapping_offset(x1); // add x0, x0, x1
                x6 = x6.wrapping_offset(x1); // add x6, x6, x1
                                             // Load the top row as early as possible
                v5 = vld1q_u16(x8.cast()); // ld1 {v5.8h}, [x8], #16
                x8 = x8.wrapping_add(16);
                x5 = x5.wrapping_offset(x1); // add x5, x5, x1
                x10 = x10.wrapping_offset(x1); // add x10, x10, x1
                w3 = w9; // mov w3, w9
                         // b 1b
            }
            // 9: ret
        }
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `160:`/`320:`/`640:` of `ipred_smooth_16bpc_neon`, instantiated for each width `W`: one loop in the
/// assembly, three instances here, in which the compiler unrolls the column loop `2:` (its count is a
/// constant). As a loop it costs more: the compiler addresses the rows of a column loop from one offset, an
/// add per row and iteration, and it widens the factors that do not change in the column loop outside of
/// it, which takes the widening multiply-accumulates apart.
///
/// # Safety
/// As the entry point; the arguments are the registers of the same names at the dispatch.
#[inline(always)]
unsafe fn ipred_smooth_160<const W: c_int>(
    mut x0: *mut u8,
    mut x1: isize,
    mut x2: *const u8,
    mut x6: *mut u8,
    mut x8: *const u8,
    mut x10: *const u8,
    mut x11: *const u8,
    mut w4: c_int,
    v4: uint16x8_t,
) {
    let mut w3: c_int = W;
    let x12 = x2.add((w3 as u32 as usize) << 1); // add x12, x2, w3, uxtw #1
    x1 -= (w3 as u32 as isize) << 1; // sub x1, x1, w3, uxtw #1
    let v5 = vld1q_dup_u16(x12.cast()); // ld1r {v5.8h}, [x12] (right)
    x2 = x2.sub(4); // sub x2, x2, #4; mov x7, #-4
    let w9 = w3; // mov w9, w3
    let v31 = vadd_u16(vget_low_u16(v4), vget_low_u16(v5)); // add v31.4h, v4.4h, v5.4h (bottom+right)
    loop {
        // 1:
        let uint16x8x2_t(v0, v1) = vld2q_dup_u16(x2.cast()); // ld2r {v0.8h, v1.8h}, [x2], x7 (left)
        x2 = x2.wrapping_sub(4);
        let uint8x8x2_t(v16, v17) = vld2_dup_u8(x11); // ld2r {v16.8b, v17.8b}, [x11], #2 (weights_ver)
        x11 = x11.wrapping_add(2);
        let v0 = vsubq_u16(v0, v5); // sub v0.8h, v0.8h, v5.8h (left-right)
        let v1 = vsubq_u16(v1, v5); // sub v1.8h, v1.8h, v5.8h
        let v16 = vmovl_u8(v16); // uxtl v16.8h, v16.8b (weights_ver)
        let v17 = vmovl_u8(v17); // uxtl v17.8h, v17.8b
        loop {
            // 2:
            let v7 = vld1q_u8(x10); // ld1 {v7.16b}, [x10], #16 (weights_hor)
            x10 = x10.wrapping_add(16);
            let uint16x8x2_t(v2, v3) = vld1q_u16_x2(x8.cast()); // ld1 {v2.8h, v3.8h}, [x8], #32 (top)
            x8 = x8.wrapping_add(32);
            let v20 = pa_ushll8(v31); // ushll v20.4s, v31.4h, #8 ((bottom+right)*256)
            let v21 = pa_ushll8(v31); // ushll v21.4s, v31.4h, #8
            let v22 = pa_ushll8(v31); // ushll v22.4s, v31.4h, #8
            let v23 = pa_ushll8(v31); // ushll v23.4s, v31.4h, #8
            let v24 = pa_ushll8(v31); // ushll v24.4s, v31.4h, #8
            let v25 = pa_ushll8(v31); // ushll v25.4s, v31.4h, #8
            let v26 = pa_ushll8(v31); // ushll v26.4s, v31.4h, #8
            let v27 = pa_ushll8(v31); // ushll v27.4s, v31.4h, #8
            let v6 = vmovl_u8(vget_low_u8(v7)); // uxtl v6.8h, v7.8b (weights_hor)
            let v7 = vmovl_high_u8(v7); // uxtl2 v7.8h, v7.16b
            let v2 = vsubq_u16(v2, v4); // sub v2.8h, v2.8h, v4.8h (top-bottom)
            let v3 = vsubq_u16(v3, v4); // sub v3.8h, v3.8h, v4.8h
            let v20 = pa_smlal(v20, v1, v6); // smlal v20.4s, v1.4h, v6.4h (+= (left-right)*weights_hor)
            let v21 = pa_smlal2(v21, v1, v6); // smlal2 v21.4s, v1.8h, v6.8h (left flipped)
            let v22 = pa_smlal(v22, v1, v7); // smlal v22.4s, v1.4h, v7.4h
            let v23 = pa_smlal2(v23, v1, v7); // smlal2 v23.4s, v1.8h, v7.8h
            let v24 = pa_smlal(v24, v0, v6); // smlal v24.4s, v0.4h, v6.4h
            let v25 = pa_smlal2(v25, v0, v6); // smlal2 v25.4s, v0.8h, v6.8h
            let v26 = pa_smlal(v26, v0, v7); // smlal v26.4s, v0.4h, v7.4h
            let v27 = pa_smlal2(v27, v0, v7); // smlal2 v27.4s, v0.8h, v7.8h
            let v20 = pa_smlal(v20, v2, v16); // smlal v20.4s, v2.4h, v16.4h (+= (top-bottom)*weights_ver)
            let v21 = pa_smlal2(v21, v2, v16); // smlal2 v21.4s, v2.8h, v16.8h
            let v22 = pa_smlal(v22, v3, v16); // smlal v22.4s, v3.4h, v16.4h
            let v23 = pa_smlal2(v23, v3, v16); // smlal2 v23.4s, v3.8h, v16.8h
            let v24 = pa_smlal(v24, v2, v17); // smlal v24.4s, v2.4h, v17.4h
            let v25 = pa_smlal2(v25, v2, v17); // smlal2 v25.4s, v2.8h, v17.8h
            let v26 = pa_smlal(v26, v3, v17); // smlal v26.4s, v3.4h, v17.4h
            let v27 = pa_smlal2(v27, v3, v17); // smlal2 v27.4s, v3.8h, v17.8h
            let v20 = pa_rshrn9_2(v20, v21); // rshrn v20.4h, v20.4s, #9; rshrn2 v20.8h, v21.4s, #9
            let v21 = pa_rshrn9_2(v22, v23); // rshrn v21.4h, v22.4s, #9; rshrn2 v21.8h, v23.4s, #9
            let v22 = pa_rshrn9_2(v24, v25); // rshrn v22.4h, v24.4s, #9; rshrn2 v22.8h, v25.4s, #9
            let v23 = pa_rshrn9_2(v26, v27); // rshrn v23.4h, v26.4s, #9; rshrn2 v23.8h, v27.4s, #9
            let gt = pa_subs_gt(&mut w3, 16); // subs w3, w3, #16
            pa_st1_2(x0, v20, v21); // st1 {v20.8h, v21.8h}, [x0], #32
            x0 = x0.wrapping_add(32);
            pa_st1_2(x6, v22, v23); // st1 {v22.8h, v23.8h}, [x6], #32
            x6 = x6.wrapping_add(32);
            if !gt {
                break; // b.gt 2b
            }
        }
        let gt = pa_subs_gt(&mut w4, 2); // subs w4, w4, #2
        if !gt {
            break; // b.le 9f
        }
        x8 = x8.wrapping_sub((w9 as u32 as usize) << 1); // sub x8, x8, w9, uxtw #1
        x10 = x10.wrapping_sub(w9 as u32 as usize); // sub x10, x10, w9, uxtw
        x0 = x0.wrapping_offset(x1); // add x0, x0, x1
        x6 = x6.wrapping_offset(x1); // add x6, x6, x1
        w3 = w9; // mov w3, w9
                 // b 1b
    }
    // 9: ret
}

/// `ipred_smooth_16bpc_neon`
///
/// # Safety
/// Reads `topleft[-height..0]`, `topleft[1..=width]` and `dav1d_sm_weights[width..2 * width]`,
/// `[height..2 * height]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_smooth_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let (mut x0, x1, mut x2, mut w3, mut w4) = (dst.cast::<u8>(), stride, topleft.cast::<u8>(), width, height);
    let x10 = core::ptr::addr_of!(crate::common::tables::dav1d_sm_weights).cast::<u8>(); // movrel x10, X(sm_weights)
    let mut x11 = x10.add(w4 as u32 as usize); // add x11, x10, w4, uxtw
    let mut x10 = x10.add(w3 as u32 as usize); // add x10, x10, w3, uxtw
    let x12 = x2.sub((w4 as u32 as usize) << 1); // sub x12, x2, w4, uxtw #1
    let v4 = vld1q_dup_u16(x12.cast()); // ld1r {v4.8h}, [x12] (bottom)
    let mut x8 = x2.add(2); // add x8, x2, #2
    let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let mut x1 = x1 << 1; // lsl x1, x1, #1
    match pa_tbl_idx(width) {
        4 => {
            // 40:
            let v6 = pa_ld1r_2d(x8); // ld1r {v6.2d}, [x8] (top)
            let v7 = pa_ld1r_2s(x10); // ld1r {v7.2s}, [x10] (weights_hor)
            x2 = x2.sub(8); // sub x2, x2, #8; mov x7, #-8
            let v5 = vdupq_laneq_u16::<3>(v6); // dup v5.8h, v6.h[3] (right)
            let v6 = vsubq_u16(v6, v4); // sub v6.8h, v6.8h, v4.8h (top-bottom)
            let v7 = vmovl_u8(v7); // uxtl v7.8h, v7.8b (weights_hor)
            let v31 = vadd_u16(vget_low_u16(v4), vget_low_u16(v5)); // add v31.4h, v4.4h, v5.4h (bottom+right)
            loop {
                // 4:
                let uint16x4x4_t(v0, v1, v2, v3) = vld4_dup_u16(x2.cast()); // ld4r {v0.4h, v1.4h, v2.4h, v3.4h}, [x2], x7 (left)
                x2 = x2.wrapping_sub(8);
                let uint8x8x4_t(v16, v17, v18, v19) = vld4_dup_u8(x11); // ld4r {v16.8b, v17.8b, v18.8b, v19.8b}, [x11], #4 (weights_ver)
                x11 = x11.wrapping_add(4);
                let v20 = pa_ushll8(v31); // ushll v20.4s, v31.4h, #8 ((bottom+right)*256)
                let v21 = pa_ushll8(v31); // ushll v21.4s, v31.4h, #8
                let v22 = pa_ushll8(v31); // ushll v22.4s, v31.4h, #8
                let v23 = pa_ushll8(v31); // ushll v23.4s, v31.4h, #8
                let v1 = vcombine_u16(v1, v0); // zip1 v1.2d, v1.2d, v0.2d (left, flipped)
                let v0 = vcombine_u16(v3, v2); // zip1 v0.2d, v3.2d, v2.2d
                let v16 = vreinterpret_u8_u32(vzip1_u32(vreinterpret_u32_u8(v16), vreinterpret_u32_u8(v17))); // zip1 v16.2s, v16.2s, v17.2s (weights_ver)
                let v18 = vreinterpret_u8_u32(vzip1_u32(vreinterpret_u32_u8(v18), vreinterpret_u32_u8(v19))); // zip1 v18.2s, v18.2s, v19.2s
                let v0 = vsubq_u16(v0, v5); // sub v0.8h, v0.8h, v5.8h (left-right)
                let v1 = vsubq_u16(v1, v5); // sub v1.8h, v1.8h, v5.8h
                let v16 = vmovl_u8(v16); // uxtl v16.8h, v16.8b (weights_ver)
                let v18 = vmovl_u8(v18); // uxtl v18.8h, v18.8b
                let v20 = pa_smlal(v20, v0, v7); // smlal v20.4s, v0.4h, v7.4h (+= (left-right)*weights_hor)
                let v21 = pa_smlal2(v21, v0, v7); // smlal2 v21.4s, v0.8h, v7.8h
                let v22 = pa_smlal(v22, v1, v7); // smlal v22.4s, v1.4h, v7.4h
                let v23 = pa_smlal2(v23, v1, v7); // smlal2 v23.4s, v1.8h, v7.8h
                let v20 = pa_smlal(v20, v6, v16); // smlal v20.4s, v6.4h, v16.4h (+= (top-bottom)*weights_ver)
                let v21 = pa_smlal2(v21, v6, v16); // smlal2 v21.4s, v6.8h, v16.8h
                let v22 = pa_smlal(v22, v6, v18); // smlal v22.4s, v6.4h, v18.4h
                let v23 = pa_smlal2(v23, v6, v18); // smlal2 v23.4s, v6.8h, v18.8h
                let v20 = pa_rshrn9(v20); // rshrn v20.4h, v20.4s, #9
                let v21 = pa_rshrn9(v21); // rshrn v21.4h, v21.4s, #9
                let v22 = pa_rshrn9(v22); // rshrn v22.4h, v22.4s, #9
                let v23 = pa_rshrn9(v23); // rshrn v23.4h, v23.4s, #9
                vst1_u16(x0.cast(), v20); // st1 {v20.4h}, [x0], x1
                vst1_u16(x6.cast(), v21); // st1 {v21.4h}, [x6], x1
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                vst1_u16(x0.wrapping_offset(x1).cast(), v22); // st1 {v22.4h}, [x0], x1
                vst1_u16(x6.wrapping_offset(x1).cast(), v23); // st1 {v23.4h}, [x6], x1
                x0 = x0.wrapping_offset(x1 << 1);
                x6 = x6.wrapping_offset(x1 << 1);
                if !gt {
                    break; // b.gt 4b
                }
            }
        }
        3 => {
            // 80:
            let v6 = vld1q_u16(x8.cast()); // ld1 {v6.8h}, [x8] (top)
            let v7 = vld1_u8(x10); // ld1 {v7.8b}, [x10] (weights_hor)
            x2 = x2.sub(8); // sub x2, x2, #8; mov x7, #-8
            let v5 = vdupq_laneq_u16::<7>(v6); // dup v5.8h, v6.h[7] (right)
            let v6 = vsubq_u16(v6, v4); // sub v6.8h, v6.8h, v4.8h (top-bottom)
            let v7 = vmovl_u8(v7); // uxtl v7.8h, v7.8b (weights_hor)
            let v31 = vadd_u16(vget_low_u16(v4), vget_low_u16(v5)); // add v31.4h, v4.4h, v5.4h (bottom+right)
            loop {
                // 8:
                let uint16x8x4_t(v0, v1, v2, v3) = vld4q_dup_u16(x2.cast()); // ld4r {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], x7 (left)
                x2 = x2.wrapping_sub(8);
                let uint8x8x4_t(v16, v17, v18, v19) = vld4_dup_u8(x11); // ld4r {v16.8b, v17.8b, v18.8b, v19.8b}, [x11], #4 (weights_ver)
                x11 = x11.wrapping_add(4);
                let v20 = pa_ushll8(v31); // ushll v20.4s, v31.4h, #8 ((bottom+right)*256)
                let v21 = pa_ushll8(v31); // ushll v21.4s, v31.4h, #8
                let v22 = pa_ushll8(v31); // ushll v22.4s, v31.4h, #8
                let v23 = pa_ushll8(v31); // ushll v23.4s, v31.4h, #8
                let v24 = pa_ushll8(v31); // ushll v24.4s, v31.4h, #8
                let v25 = pa_ushll8(v31); // ushll v25.4s, v31.4h, #8
                let v26 = pa_ushll8(v31); // ushll v26.4s, v31.4h, #8
                let v27 = pa_ushll8(v31); // ushll v27.4s, v31.4h, #8
                let v0 = vsubq_u16(v0, v5); // sub v0.8h, v0.8h, v5.8h (left-right)
                let v1 = vsubq_u16(v1, v5); // sub v1.8h, v1.8h, v5.8h
                let v2 = vsubq_u16(v2, v5); // sub v2.8h, v2.8h, v5.8h
                let v3 = vsubq_u16(v3, v5); // sub v3.8h, v3.8h, v5.8h
                let v16 = vmovl_u8(v16); // uxtl v16.8h, v16.8b (weights_ver)
                let v17 = vmovl_u8(v17); // uxtl v17.8h, v17.8b
                let v18 = vmovl_u8(v18); // uxtl v18.8h, v18.8b
                let v19 = vmovl_u8(v19); // uxtl v19.8h, v19.8b
                let v20 = pa_smlal(v20, v3, v7); // smlal v20.4s, v3.4h, v7.4h (+= (left-right)*weights_hor)
                let v21 = pa_smlal2(v21, v3, v7); // smlal2 v21.4s, v3.8h, v7.8h (left flipped)
                let v22 = pa_smlal(v22, v2, v7); // smlal v22.4s, v2.4h, v7.4h
                let v23 = pa_smlal2(v23, v2, v7); // smlal2 v23.4s, v2.8h, v7.8h
                let v24 = pa_smlal(v24, v1, v7); // smlal v24.4s, v1.4h, v7.4h
                let v25 = pa_smlal2(v25, v1, v7); // smlal2 v25.4s, v1.8h, v7.8h
                let v26 = pa_smlal(v26, v0, v7); // smlal v26.4s, v0.4h, v7.4h
                let v27 = pa_smlal2(v27, v0, v7); // smlal2 v27.4s, v0.8h, v7.8h
                let v20 = pa_smlal(v20, v6, v16); // smlal v20.4s, v6.4h, v16.4h (+= (top-bottom)*weights_ver)
                let v21 = pa_smlal2(v21, v6, v16); // smlal2 v21.4s, v6.8h, v16.8h
                let v22 = pa_smlal(v22, v6, v17); // smlal v22.4s, v6.4h, v17.4h
                let v23 = pa_smlal2(v23, v6, v17); // smlal2 v23.4s, v6.8h, v17.8h
                let v24 = pa_smlal(v24, v6, v18); // smlal v24.4s, v6.4h, v18.4h
                let v25 = pa_smlal2(v25, v6, v18); // smlal2 v25.4s, v6.8h, v18.8h
                let v26 = pa_smlal(v26, v6, v19); // smlal v26.4s, v6.4h, v19.4h
                let v27 = pa_smlal2(v27, v6, v19); // smlal2 v27.4s, v6.8h, v19.8h
                let v20 = pa_rshrn9_2(v20, v21); // rshrn v20.4h, v20.4s, #9; rshrn2 v20.8h, v21.4s, #9
                let v21 = pa_rshrn9_2(v22, v23); // rshrn v21.4h, v22.4s, #9; rshrn2 v21.8h, v23.4s, #9
                let v22 = pa_rshrn9_2(v24, v25); // rshrn v22.4h, v24.4s, #9; rshrn2 v22.8h, v25.4s, #9
                let v23 = pa_rshrn9_2(v26, v27); // rshrn v23.4h, v26.4s, #9; rshrn2 v23.8h, v27.4s, #9
                vst1q_u16(x0.cast(), v20); // st1 {v20.8h}, [x0], x1
                vst1q_u16(x6.cast(), v21); // st1 {v21.8h}, [x6], x1
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                vst1q_u16(x0.wrapping_offset(x1).cast(), v22); // st1 {v22.8h}, [x0], x1
                vst1q_u16(x6.wrapping_offset(x1).cast(), v23); // st1 {v23.8h}, [x6], x1
                x0 = x0.wrapping_offset(x1 << 1);
                x6 = x6.wrapping_offset(x1 << 1);
                if !gt {
                    break; // b.gt 8b
                }
            }
        }
        2 => ipred_smooth_160::<16>(x0, x1, x2, x6, x8, x10, x11, w4, v4), // 160:
        1 => ipred_smooth_160::<32>(x0, x1, x2, x6, x8, x10, x11, w4, v4), // 320:
        0 => ipred_smooth_160::<64>(x0, x1, x2, x6, x8, x10, x11, w4, v4), // 640:
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `160:`/`320:`/`640:` of `ipred_smooth_v_16bpc_neon`, instantiated for each width `W`: one loop in the
/// assembly, three instances here, in which the compiler unrolls the column loop `2:` (its count is a
/// constant). As a loop it costs more: the compiler addresses the rows of a column loop from one offset, an
/// add per row and iteration.
///
/// # Safety
/// As the entry point; the arguments are the registers of the same names at the dispatch.
#[inline(always)]
unsafe fn ipred_smooth_v_160<const W: c_int>(
    mut x0: *mut u8,
    mut x1: isize,
    mut x2: *const u8,
    mut x6: *mut u8,
    mut x7: *const u8,
    mut w4: c_int,
    v4: uint16x8_t,
) {
    let mut w3: c_int = W;
    // Set up pointers for four rows in parallel; x0, x6, x5, x8
    let mut x5 = x0.wrapping_offset(x1); // add x5, x0, x1
    let mut x8 = x6.wrapping_offset(x1); // add x8, x6, x1
    x1 <<= 1; // lsl x1, x1, #1
    x1 -= (w3 as u32 as isize) << 1; // sub x1, x1, w3, uxtw #1
    let w9 = w3; // mov w9, w3
    loop {
        // 1:
        let uint8x8x4_t(v16, v17, v18, v19) = vld4_dup_u8(x7); // ld4r {v16.8b, v17.8b, v18.8b, v19.8b}, [x7], #4 (weights_ver)
        x7 = x7.wrapping_add(4);
        let v16 = vshll_n_u8::<7>(v16); // ushll v16.8h, v16.8b, #7 (weights_ver << 7)
        let v17 = vshll_n_u8::<7>(v17); // ushll v17.8h, v17.8b, #7
        let v18 = vshll_n_u8::<7>(v18); // ushll v18.8h, v18.8b, #7
        let v19 = vshll_n_u8::<7>(v19); // ushll v19.8h, v19.8b, #7
        loop {
            // 2:
            let uint16x8x2_t(v2, v3) = vld1q_u16_x2(x2.cast()); // ld1 {v2.8h, v3.8h}, [x2], #32 (top)
            x2 = x2.wrapping_add(32);
            let v2 = vsubq_u16(v2, v4); // sub v2.8h, v2.8h, v4.8h (top-bottom)
            let v3 = vsubq_u16(v3, v4); // sub v3.8h, v3.8h, v4.8h
            let v20 = pa_sqrdmulh(v2, v16); // sqrdmulh v20.8h, v2.8h, v16.8h (((top-bottom)*weights_ver + 128) >> 8)
            let v21 = pa_sqrdmulh(v3, v16); // sqrdmulh v21.8h, v3.8h, v16.8h
            let v22 = pa_sqrdmulh(v2, v17); // sqrdmulh v22.8h, v2.8h, v17.8h
            let v23 = pa_sqrdmulh(v3, v17); // sqrdmulh v23.8h, v3.8h, v17.8h
            let v24 = pa_sqrdmulh(v2, v18); // sqrdmulh v24.8h, v2.8h, v18.8h
            let v25 = pa_sqrdmulh(v3, v18); // sqrdmulh v25.8h, v3.8h, v18.8h
            let v26 = pa_sqrdmulh(v2, v19); // sqrdmulh v26.8h, v2.8h, v19.8h
            let v27 = pa_sqrdmulh(v3, v19); // sqrdmulh v27.8h, v3.8h, v19.8h
            let v20 = vaddq_u16(v20, v4); // add v20.8h, v20.8h, v4.8h
            let v21 = vaddq_u16(v21, v4); // add v21.8h, v21.8h, v4.8h
            let v22 = vaddq_u16(v22, v4); // add v22.8h, v22.8h, v4.8h
            let v23 = vaddq_u16(v23, v4); // add v23.8h, v23.8h, v4.8h
            let v24 = vaddq_u16(v24, v4); // add v24.8h, v24.8h, v4.8h
            let v25 = vaddq_u16(v25, v4); // add v25.8h, v25.8h, v4.8h
            let v26 = vaddq_u16(v26, v4); // add v26.8h, v26.8h, v4.8h
            let v27 = vaddq_u16(v27, v4); // add v27.8h, v27.8h, v4.8h
            let gt = pa_subs_gt(&mut w3, 16); // subs w3, w3, #16
            pa_st1_2(x0, v20, v21); // st1 {v20.8h, v21.8h}, [x0], #32
            x0 = x0.wrapping_add(32);
            pa_st1_2(x6, v22, v23); // st1 {v22.8h, v23.8h}, [x6], #32
            x6 = x6.wrapping_add(32);
            pa_st1_2(x5, v24, v25); // st1 {v24.8h, v25.8h}, [x5], #32
            x5 = x5.wrapping_add(32);
            pa_st1_2(x8, v26, v27); // st1 {v26.8h, v27.8h}, [x8], #32
            x8 = x8.wrapping_add(32);
            if !gt {
                break; // b.gt 2b
            }
        }
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        if !gt {
            break; // b.le 9f
        }
        x2 = x2.wrapping_sub((w9 as u32 as usize) << 1); // sub x2, x2, w9, uxtw #1
        x0 = x0.wrapping_offset(x1); // add x0, x0, x1
        x6 = x6.wrapping_offset(x1); // add x6, x6, x1
        x5 = x5.wrapping_offset(x1); // add x5, x5, x1
        x8 = x8.wrapping_offset(x1); // add x8, x8, x1
        w3 = w9; // mov w3, w9
                 // b 1b
    }
    // 9: ret
}

/// `ipred_smooth_v_16bpc_neon`
///
/// # Safety
/// Reads `topleft[-height]`, `topleft[1..=width]` and `dav1d_sm_weights[height..2 * height]`; writes `dst` rows
/// `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_smooth_v_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let (mut x0, x1, mut w4) = (dst.cast::<u8>(), stride, height);
    let x2 = topleft.cast::<u8>();
    let x7 = core::ptr::addr_of!(crate::common::tables::dav1d_sm_weights).cast::<u8>(); // movrel x7, X(sm_weights)
    let mut x7 = x7.add(w4 as u32 as usize); // add x7, x7, w4, uxtw
    let x8 = x2.sub((w4 as u32 as usize) << 1); // sub x8, x2, w4, uxtw #1
    let v4 = vld1q_dup_u16(x8.cast()); // ld1r {v4.8h}, [x8] (bottom)
    let mut x2 = x2.add(2); // add x2, x2, #2
    let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let mut x1 = x1 << 1; // lsl x1, x1, #1
    match pa_tbl_idx(width) {
        4 => {
            // 40:
            let v6 = pa_ld1r_2d(x2); // ld1r {v6.2d}, [x2] (top)
            let v6 = vsubq_u16(v6, v4); // sub v6.8h, v6.8h, v4.8h (top-bottom)
            loop {
                // 4:
                let uint8x8x4_t(v16, v17, v18, v19) = vld4_dup_u8(x7); // ld4r {v16.8b, v17.8b, v18.8b, v19.8b}, [x7], #4 (weights_ver)
                x7 = x7.wrapping_add(4);
                let v16 = vreinterpret_u8_u32(vzip1_u32(vreinterpret_u32_u8(v16), vreinterpret_u32_u8(v17))); // zip1 v16.2s, v16.2s, v17.2s (weights_ver)
                let v18 = vreinterpret_u8_u32(vzip1_u32(vreinterpret_u32_u8(v18), vreinterpret_u32_u8(v19))); // zip1 v18.2s, v18.2s, v19.2s
                let v16 = vshll_n_u8::<7>(v16); // ushll v16.8h, v16.8b, #7 (weights_ver << 7)
                let v18 = vshll_n_u8::<7>(v18); // ushll v18.8h, v18.8b, #7
                let v20 = pa_sqrdmulh(v6, v16); // sqrdmulh v20.8h, v6.8h, v16.8h (((top-bottom)*weights_ver + 128) >> 8)
                let v21 = pa_sqrdmulh(v6, v18); // sqrdmulh v21.8h, v6.8h, v18.8h
                let v20 = vaddq_u16(v20, v4); // add v20.8h, v20.8h, v4.8h
                let v21 = vaddq_u16(v21, v4); // add v21.8h, v21.8h, v4.8h
                vst1_u16(x0.cast(), vget_low_u16(v20)); // st1 {v20.d}[0], [x0], x1
                vst1_u16(x6.cast(), vget_high_u16(v20)); // st1 {v20.d}[1], [x6], x1
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                vst1_u16(x0.wrapping_offset(x1).cast(), vget_low_u16(v21)); // st1 {v21.d}[0], [x0], x1
                vst1_u16(x6.wrapping_offset(x1).cast(), vget_high_u16(v21)); // st1 {v21.d}[1], [x6], x1
                x0 = x0.wrapping_offset(x1 << 1);
                x6 = x6.wrapping_offset(x1 << 1);
                if !gt {
                    break; // b.gt 4b
                }
            }
        }
        3 => {
            // 80:
            let v6 = vld1q_u16(x2.cast()); // ld1 {v6.8h}, [x2] (top)
            let v6 = vsubq_u16(v6, v4); // sub v6.8h, v6.8h, v4.8h (top-bottom)
            loop {
                // 8:
                let uint8x8x4_t(v16, v17, v18, v19) = vld4_dup_u8(x7); // ld4r {v16.8b, v17.8b, v18.8b, v19.8b}, [x7], #4 (weights_ver)
                x7 = x7.wrapping_add(4);
                let v16 = vshll_n_u8::<7>(v16); // ushll v16.8h, v16.8b, #7 (weights_ver << 7)
                let v17 = vshll_n_u8::<7>(v17); // ushll v17.8h, v17.8b, #7
                let v18 = vshll_n_u8::<7>(v18); // ushll v18.8h, v18.8b, #7
                let v19 = vshll_n_u8::<7>(v19); // ushll v19.8h, v19.8b, #7
                let v20 = pa_sqrdmulh(v6, v16); // sqrdmulh v20.8h, v6.8h, v16.8h (((top-bottom)*weights_ver + 128) >> 8)
                let v21 = pa_sqrdmulh(v6, v17); // sqrdmulh v21.8h, v6.8h, v17.8h
                let v22 = pa_sqrdmulh(v6, v18); // sqrdmulh v22.8h, v6.8h, v18.8h
                let v23 = pa_sqrdmulh(v6, v19); // sqrdmulh v23.8h, v6.8h, v19.8h
                let v20 = vaddq_u16(v20, v4); // add v20.8h, v20.8h, v4.8h
                let v21 = vaddq_u16(v21, v4); // add v21.8h, v21.8h, v4.8h
                let v22 = vaddq_u16(v22, v4); // add v22.8h, v22.8h, v4.8h
                let v23 = vaddq_u16(v23, v4); // add v23.8h, v23.8h, v4.8h
                vst1q_u16(x0.cast(), v20); // st1 {v20.8h}, [x0], x1
                vst1q_u16(x6.cast(), v21); // st1 {v21.8h}, [x6], x1
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                vst1q_u16(x0.wrapping_offset(x1).cast(), v22); // st1 {v22.8h}, [x0], x1
                vst1q_u16(x6.wrapping_offset(x1).cast(), v23); // st1 {v23.8h}, [x6], x1
                x0 = x0.wrapping_offset(x1 << 1);
                x6 = x6.wrapping_offset(x1 << 1);
                if !gt {
                    break; // b.gt 8b
                }
            }
        }
        2 => ipred_smooth_v_160::<16>(x0, x1, x2, x6, x7, w4, v4), // 160:
        1 => ipred_smooth_v_160::<32>(x0, x1, x2, x6, x7, w4, v4), // 320:
        0 => ipred_smooth_v_160::<64>(x0, x1, x2, x6, x7, w4, v4), // 640:
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `160:`/`320:`/`640:` of `ipred_smooth_h_16bpc_neon`, instantiated for each width `W`: one loop in the
/// assembly, three instances here, in which the compiler unrolls the column loop `2:` (its count is a
/// constant). As a loop it costs more: the compiler addresses the rows of a column loop from one offset, an
/// add per row and iteration.
///
/// # Safety
/// As the entry point; the arguments are the registers of the same names at the dispatch.
#[inline(always)]
unsafe fn ipred_smooth_h_160<const W: c_int>(
    mut x0: *mut u8,
    mut x1: isize,
    mut x2: *const u8,
    mut x6: *mut u8,
    mut x8: *const u8,
    mut w4: c_int,
    v5: uint16x8_t,
) {
    let mut w3: c_int = W;
    x2 = x2.sub(8); // sub x2, x2, #8; mov x7, #-8
                    // Set up pointers for four rows in parallel; x0, x6, x5, x10
    let mut x5 = x0.wrapping_offset(x1); // add x5, x0, x1
    let mut x10 = x6.wrapping_offset(x1); // add x10, x6, x1
    x1 <<= 1; // lsl x1, x1, #1
    x1 -= (w3 as u32 as isize) << 1; // sub x1, x1, w3, uxtw #1
    let w9 = w3; // mov w9, w3
    loop {
        // 1:
        let uint16x8x4_t(v0, v1, v2, v3) = vld4q_dup_u16(x2.cast()); // ld4r {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], x7 (left)
        x2 = x2.wrapping_sub(8);
        let v0 = vsubq_u16(v0, v5); // sub v0.8h, v0.8h, v5.8h (left-right)
        let v1 = vsubq_u16(v1, v5); // sub v1.8h, v1.8h, v5.8h
        let v2 = vsubq_u16(v2, v5); // sub v2.8h, v2.8h, v5.8h
        let v3 = vsubq_u16(v3, v5); // sub v3.8h, v3.8h, v5.8h
        loop {
            // 2:
            let v7 = vld1q_u8(x8); // ld1 {v7.16b}, [x8], #16 (weights_hor)
            x8 = x8.wrapping_add(16);
            let v6 = vshll_n_u8::<7>(vget_low_u8(v7)); // ushll v6.8h, v7.8b, #7 (weights_hor << 7)
            let v7 = vshll_high_n_u8::<7>(v7); // ushll2 v7.8h, v7.16b, #7
            let v20 = pa_sqrdmulh(v3, v6); // sqrdmulh v20.8h, v3.8h, v6.8h (((left-right)*weights_hor + 128) >> 8)
            let v21 = pa_sqrdmulh(v3, v7); // sqrdmulh v21.8h, v3.8h, v7.8h (left flipped)
            let v22 = pa_sqrdmulh(v2, v6); // sqrdmulh v22.8h, v2.8h, v6.8h
            let v23 = pa_sqrdmulh(v2, v7); // sqrdmulh v23.8h, v2.8h, v7.8h
            let v24 = pa_sqrdmulh(v1, v6); // sqrdmulh v24.8h, v1.8h, v6.8h
            let v25 = pa_sqrdmulh(v1, v7); // sqrdmulh v25.8h, v1.8h, v7.8h
            let v26 = pa_sqrdmulh(v0, v6); // sqrdmulh v26.8h, v0.8h, v6.8h
            let v27 = pa_sqrdmulh(v0, v7); // sqrdmulh v27.8h, v0.8h, v7.8h
            let v20 = vaddq_u16(v20, v5); // add v20.8h, v20.8h, v5.8h
            let v21 = vaddq_u16(v21, v5); // add v21.8h, v21.8h, v5.8h
            let v22 = vaddq_u16(v22, v5); // add v22.8h, v22.8h, v5.8h
            let v23 = vaddq_u16(v23, v5); // add v23.8h, v23.8h, v5.8h
            let v24 = vaddq_u16(v24, v5); // add v24.8h, v24.8h, v5.8h
            let v25 = vaddq_u16(v25, v5); // add v25.8h, v25.8h, v5.8h
            let v26 = vaddq_u16(v26, v5); // add v26.8h, v26.8h, v5.8h
            let v27 = vaddq_u16(v27, v5); // add v27.8h, v27.8h, v5.8h
            let gt = pa_subs_gt(&mut w3, 16); // subs w3, w3, #16
            pa_st1_2(x0, v20, v21); // st1 {v20.8h, v21.8h}, [x0], #32
            x0 = x0.wrapping_add(32);
            pa_st1_2(x6, v22, v23); // st1 {v22.8h, v23.8h}, [x6], #32
            x6 = x6.wrapping_add(32);
            pa_st1_2(x5, v24, v25); // st1 {v24.8h, v25.8h}, [x5], #32
            x5 = x5.wrapping_add(32);
            pa_st1_2(x10, v26, v27); // st1 {v26.8h, v27.8h}, [x10], #32
            x10 = x10.wrapping_add(32);
            if !gt {
                break; // b.gt 2b
            }
        }
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        if !gt {
            break; // b.le 9f
        }
        x8 = x8.wrapping_sub(w9 as u32 as usize); // sub x8, x8, w9, uxtw
        x0 = x0.wrapping_offset(x1); // add x0, x0, x1
        x6 = x6.wrapping_offset(x1); // add x6, x6, x1
        x5 = x5.wrapping_offset(x1); // add x5, x5, x1
        x10 = x10.wrapping_offset(x1); // add x10, x10, x1
        w3 = w9; // mov w3, w9
                 // b 1b
    }
    // 9: ret
}

/// `ipred_smooth_h_16bpc_neon`
///
/// # Safety
/// Reads `topleft[-height..0]`, `topleft[width]` and `dav1d_sm_weights[width..2 * width]`; writes `dst` rows
/// `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_smooth_h_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let (mut x0, x1, mut w3, mut w4) = (dst.cast::<u8>(), stride, width, height);
    let mut x2 = topleft.cast::<u8>();
    let x8 = core::ptr::addr_of!(crate::common::tables::dav1d_sm_weights).cast::<u8>(); // movrel x8, X(sm_weights)
    let mut x8 = x8.add(w3 as u32 as usize); // add x8, x8, w3, uxtw
    let x12 = x2.add((w3 as u32 as usize) << 1); // add x12, x2, w3, uxtw #1
    let v5 = vld1q_dup_u16(x12.cast()); // ld1r {v5.8h}, [x12] (right)
    let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let mut x1 = x1 << 1; // lsl x1, x1, #1
    match pa_tbl_idx(width) {
        4 => {
            // 40:
            let v7 = pa_ld1r_2s(x8); // ld1r {v7.2s}, [x8] (weights_hor)
            x2 = x2.sub(8); // sub x2, x2, #8; mov x7, #-8
            let v7 = vshll_n_u8::<7>(v7); // ushll v7.8h, v7.8b, #7 (weights_hor << 7)
            loop {
                // 4:
                let uint16x4x4_t(v0, v1, v2, v3) = vld4_dup_u16(x2.cast()); // ld4r {v0.4h, v1.4h, v2.4h, v3.4h}, [x2], x7 (left)
                x2 = x2.wrapping_sub(8);
                let v1 = vcombine_u16(v1, v0); // zip1 v1.2d, v1.2d, v0.2d (left, flipped)
                let v0 = vcombine_u16(v3, v2); // zip1 v0.2d, v3.2d, v2.2d
                let v0 = vsubq_u16(v0, v5); // sub v0.8h, v0.8h, v5.8h (left-right)
                let v1 = vsubq_u16(v1, v5); // sub v1.8h, v1.8h, v5.8h
                let v20 = pa_sqrdmulh(v0, v7); // sqrdmulh v20.8h, v0.8h, v7.8h (((left-right)*weights_hor + 128) >> 8)
                let v21 = pa_sqrdmulh(v1, v7); // sqrdmulh v21.8h, v1.8h, v7.8h
                let v20 = vaddq_u16(v20, v5); // add v20.8h, v20.8h, v5.8h
                let v21 = vaddq_u16(v21, v5); // add v21.8h, v21.8h, v5.8h
                vst1_u16(x0.cast(), vget_low_u16(v20)); // st1 {v20.d}[0], [x0], x1
                vst1_u16(x6.cast(), vget_high_u16(v20)); // st1 {v20.d}[1], [x6], x1
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                vst1_u16(x0.wrapping_offset(x1).cast(), vget_low_u16(v21)); // st1 {v21.d}[0], [x0], x1
                vst1_u16(x6.wrapping_offset(x1).cast(), vget_high_u16(v21)); // st1 {v21.d}[1], [x6], x1
                x0 = x0.wrapping_offset(x1 << 1);
                x6 = x6.wrapping_offset(x1 << 1);
                if !gt {
                    break; // b.gt 4b
                }
            }
        }
        3 => {
            // 80:
            let v7 = vld1_u8(x8); // ld1 {v7.8b}, [x8] (weights_hor)
            x2 = x2.sub(8); // sub x2, x2, #8; mov x7, #-8
            let v7 = vshll_n_u8::<7>(v7); // ushll v7.8h, v7.8b, #7 (weights_hor << 7)
            loop {
                // 8:
                let uint16x8x4_t(v0, v1, v2, v3) = vld4q_dup_u16(x2.cast()); // ld4r {v0.8h, v1.8h, v2.8h, v3.8h}, [x2], x7 (left)
                x2 = x2.wrapping_sub(8);
                let v3 = vsubq_u16(v3, v5); // sub v3.8h, v3.8h, v5.8h (left-right)
                let v2 = vsubq_u16(v2, v5); // sub v2.8h, v2.8h, v5.8h
                let v1 = vsubq_u16(v1, v5); // sub v1.8h, v1.8h, v5.8h
                let v0 = vsubq_u16(v0, v5); // sub v0.8h, v0.8h, v5.8h
                let v20 = pa_sqrdmulh(v3, v7); // sqrdmulh v20.8h, v3.8h, v7.8h (((left-right)*weights_hor + 128) >> 8)
                let v21 = pa_sqrdmulh(v2, v7); // sqrdmulh v21.8h, v2.8h, v7.8h (left flipped)
                let v22 = pa_sqrdmulh(v1, v7); // sqrdmulh v22.8h, v1.8h, v7.8h
                let v23 = pa_sqrdmulh(v0, v7); // sqrdmulh v23.8h, v0.8h, v7.8h
                let v20 = vaddq_u16(v20, v5); // add v20.8h, v20.8h, v5.8h
                let v21 = vaddq_u16(v21, v5); // add v21.8h, v21.8h, v5.8h
                let v22 = vaddq_u16(v22, v5); // add v22.8h, v22.8h, v5.8h
                let v23 = vaddq_u16(v23, v5); // add v23.8h, v23.8h, v5.8h
                vst1q_u16(x0.cast(), v20); // st1 {v20.8h}, [x0], x1
                vst1q_u16(x6.cast(), v21); // st1 {v21.8h}, [x6], x1
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                vst1q_u16(x0.wrapping_offset(x1).cast(), v22); // st1 {v22.8h}, [x0], x1
                vst1q_u16(x6.wrapping_offset(x1).cast(), v23); // st1 {v23.8h}, [x6], x1
                x0 = x0.wrapping_offset(x1 << 1);
                x6 = x6.wrapping_offset(x1 << 1);
                if !gt {
                    break; // b.gt 8b
                }
            }
        }
        2 => ipred_smooth_h_160::<16>(x0, x1, x2, x6, x8, w4, v5), // 160:
        1 => ipred_smooth_h_160::<32>(x0, x1, x2, x6, x8, w4, v5), // 320:
        0 => ipred_smooth_h_160::<64>(x0, x1, x2, x6, x8, w4, v5), // 640:
        _ => core::hint::unreachable_unchecked(),
    }
}

// ============================================================================
// Part B (ipred16.S lines 1427–2061): the edge helpers and the z1 fill kernels.
// ============================================================================
// Register convention of this part: typed locals named after the assembly's
// registers (`v0`, `v31`, `x0`, `w7`); a `.4h`/`.2s` register is a 64-bit
// vector type, a `.8h`/`.4s` register a 128-bit one. Pixel pointers are
// `*const u16`/`*mut u16`, so an offset of `n` pixels is the assembly's
// `uxtw #1` or byte offset `2*n`; the stride is in bytes
// (`wrapping_byte_offset`). Post-incremented addresses are advanced with
// `wrapping_add` after the access, so that no pointer past the last access
// is formed with `add`.

/// `padding_mask_buf`: 48 bytes of 0x00 followed by the 48 bytes of 0xff at
/// the inner label `padding_mask`.
static PADDING_MASK_BUF: [u8; 96] = {
    let mut buf = [0u8; 96];
    let mut i = 48;
    while i < 96 {
        buf[i] = 0xff;
        i += 1;
    }
    buf
};

/// `movrel xN, padding_mask`: the address of the 0xff half of
/// `padding_mask_buf`. A load at `padding_mask() - 2*n` gives a mask that
/// keeps the first `n` pixels and replaces the rest.
#[inline(always)]
fn padding_mask() -> *const u8 {
    // SAFETY: 48 is inside the 96-byte static.
    unsafe { PADDING_MASK_BUF.as_ptr().add(48) }
}

/// `bit vd.16b, vn.16b, vm.16b` with `vd`, `vn` typed as `.8h` registers:
/// `vn` where the mask `vm` (bytes of `padding_mask_buf`) is set, else
/// `vd`.
#[inline(always)]
unsafe fn ipred_z1_bit(vd: uint16x8_t, vn: uint16x8_t, vm: uint8x16_t) -> uint16x8_t {
    vreinterpretq_u16_u8(vbslq_u8(vm, vreinterpretq_u8_u16(vn), vreinterpretq_u8_u16(vd)))
}

/// `dav1d_ipred_z1_upsample_edge_16bpc_neon(pixel *out, const int hsz, const pixel *const in, const int end,
/// const int bitdepth_max)`
///
/// # Safety
/// Reads `in[0..16]` (whatever `end` is; the pixels above `in[end]` are
/// replaced through the padding mask) and `in[end]`, with 0 <= `end` <= 24
/// (the mask load is `padding_mask - 2*end`, 32 bytes); writes `out[0..32]`.
pub unsafe extern "C" fn dav1d_ipred_z1_upsample_edge_16bpc_neon(
    out: *mut u16,
    hsz: c_int,
    in_: *const u16,
    end: c_int,
    bitdepth_max: c_int,
) {
    let _ = hsz;
    let (x0, x2, w3) = (out, in_, end as u32 as usize);
    let v30 = vdupq_n_s16(bitdepth_max as i16); // dup v30.8h, w4 (bitdepth_max)
    let x4 = padding_mask(); // movrel x4, padding_mask
    let uint16x8x2_t(v0, v1) = vld1q_u16_x2(x2); // ld1 {v0.8h, v1.8h}, [x2] (in[])
    let x5 = x2.add(w3); // add x5, x2, w3, uxtw #1 (in[end])
    let x4 = x4.sub(w3 << 1); // sub x4, x4, w3, uxtw #1

    let v2 = vld1q_dup_u16(x5); // ld1r {v2.8h}, [x5] (padding)
    let uint8x16x2_t(v3, v4) = vld1q_u8_x2(x4); // ld1 {v3.8h, v4.8h}, [x4] (padding_mask)

    let v31 = vdupq_n_u16(9); // movi v31.8h, #9

    let v0 = ipred_z1_bit(v0, v2, v3); // bit v0.16b, v2.16b, v3.16b (padded in[])
    let v1 = ipred_z1_bit(v1, v2, v4); // bit v1.16b, v2.16b, v4.16b

    let v4 = vextq_u16::<1>(v0, v1); // ext v4.16b, v0.16b, v1.16b, #2
    let v5 = vextq_u16::<1>(v1, v2); // ext v5.16b, v1.16b, v2.16b, #2
    let v6 = vextq_u16::<2>(v0, v1); // ext v6.16b, v0.16b, v1.16b, #4
    let v7 = vextq_u16::<2>(v1, v2); // ext v7.16b, v1.16b, v2.16b, #4
    let v16 = vextq_u16::<3>(v0, v1); // ext v16.16b, v0.16b, v1.16b, #6
    let v17 = vextq_u16::<3>(v1, v2); // ext v17.16b, v1.16b, v2.16b, #6

    let v18 = vaddq_u16(v4, v6); // add v18.8h, v4.8h, v6.8h (in[i+1] + in[i+2])
    let v19 = vaddq_u16(v5, v7); // add v19.8h, v5.8h, v7.8h
    let v20 = vaddq_u16(v0, v16); // add v20.8h, v0.8h, v16.8h
    let v21 = vaddq_u16(v1, v17); // add v21.8h, v1.8h, v17.8h
    let v22 = vmull_u16(vget_low_u16(v18), vget_low_u16(v31)); // umull v22.4s, v18.4h, v31.4h (9*(in[i+1] + in[i+2]))
    let v23 = vmull_high_u16(v18, v31); // umull2 v23.4s, v18.8h, v31.8h
    let v24 = vmull_u16(vget_low_u16(v19), vget_low_u16(v31)); // umull v24.4s, v19.4h, v31.4h
    let v25 = vmull_high_u16(v19, v31); // umull2 v25.4s, v19.8h, v31.8h
    let v22 = vsubw_u16(v22, vget_low_u16(v20)); // usubw v22.4s, v22.4s, v20.4h
    let v23 = vsubw_high_u16(v23, v20); // usubw2 v23.4s, v23.4s, v20.8h
    let v24 = vsubw_u16(v24, vget_low_u16(v21)); // usubw v24.4s, v24.4s, v21.4h
    let v25 = vsubw_high_u16(v25, v21); // usubw2 v25.4s, v25.4s, v21.8h

    let v16 = vqrshrun_n_s32::<4>(vreinterpretq_s32_u32(v22)); // sqrshrun v16.4h, v22.4s, #4
    let v16 = vqrshrun_high_n_s32::<4>(v16, vreinterpretq_s32_u32(v23)); // sqrshrun2 v16.8h, v23.4s, #4
    let v17 = vqrshrun_n_s32::<4>(vreinterpretq_s32_u32(v24)); // sqrshrun v17.4h, v24.4s, #4
    let v17 = vqrshrun_high_n_s32::<4>(v17, vreinterpretq_s32_u32(v25)); // sqrshrun2 v17.8h, v25.4s, #4

    let v16 = vreinterpretq_u16_s16(vminq_s16(vreinterpretq_s16_u16(v16), v30)); // smin v16.8h, v16.8h, v30.8h
    let v17 = vreinterpretq_u16_s16(vminq_s16(vreinterpretq_s16_u16(v17), v30)); // smin v17.8h, v17.8h, v30.8h

    let v0 = vzip1q_u16(v4, v16); // zip1 v0.8h, v4.8h, v16.8h
    let v1 = vzip2q_u16(v4, v16); // zip2 v1.8h, v4.8h, v16.8h
    let v2 = vzip1q_u16(v5, v17); // zip1 v2.8h, v5.8h, v17.8h
    let v3 = vzip2q_u16(v5, v17); // zip2 v3.8h, v5.8h, v17.8h

    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0]
}

/// `dav1d_ipred_z2_upsample_edge_16bpc_neon(pixel *out, const int sz, const pixel *const in,
/// const int bitdepth_max)`
///
/// Here, sz is 4 or 8, and we produce 2*sz+1 output elements.
///
/// # Safety
/// Reads `in[0..16]` (the pixels above `in[sz]` are replaced through the
/// padding mask) and `in[sz]`; writes `out[0..17]`.
pub unsafe extern "C" fn dav1d_ipred_z2_upsample_edge_16bpc_neon(
    out: *mut u16,
    sz: c_int,
    in_: *const u16,
    bitdepth_max: c_int,
) {
    let (x0, w1, x2) = (out, sz as u32 as usize, in_);
    let v30 = vdupq_n_s16(bitdepth_max as i16); // dup v30.8h, w3 (bitdepth_max)
    let x4 = padding_mask(); // movrel x4, padding_mask
    let uint16x8x2_t(v0, v1) = vld1q_u16_x2(x2); // ld1 {v0.8h, v1.8h}, [x2] (in[])
    let x5 = x2.add(w1); // add x5, x2, w1, uxtw #1 (in[sz])
    let x4 = x4.sub(w1 << 1); // sub x4, x4, w1, uxtw #1

    let v3 = vld1q_dup_u16(x2); // ld1r {v3.8h}, [x2] (in[0] for padding)
    let v2 = vld1q_dup_u16(x5); // ld1r {v2.8h}, [x5] (padding)
    let uint8x16x2_t(v4, v5) = vld1q_u8_x2(x4); // ld1 {v4.8h, v5.8h}, [x4] (padding_mask)

    let v31 = vdupq_n_u16(9); // movi v31.8h, #9

    let v0 = ipred_z1_bit(v0, v2, v4); // bit v0.16b, v2.16b, v4.16b (padded in[])
    let v1 = ipred_z1_bit(v1, v2, v5); // bit v1.16b, v2.16b, v5.16b

    let v4 = vextq_u16::<7>(v3, v0); // ext v4.16b, v3.16b, v0.16b, #14
    let v5 = vextq_u16::<1>(v0, v1); // ext v5.16b, v0.16b, v1.16b, #2
    let v6 = vextq_u16::<2>(v0, v1); // ext v6.16b, v0.16b, v1.16b, #4

    let v16 = vaddq_u16(v0, v5); // add v16.8h, v0.8h, v5.8h (in[i+0] + in[i+1])
    let v17 = vaddq_u16(v4, v6); // add v17.8h, v4.8h, v6.8h (in[i-1] + in[i+2])
    let v18 = vmull_u16(vget_low_u16(v16), vget_low_u16(v31)); // umull v18.4s, v16.4h, v31.4h (9*(in[i+1] + in[i+2]))
    let v19 = vmull_high_u16(v16, v31); // umull2 v19.4s, v16.8h, v31.8h
    let v18 = vsubw_u16(v18, vget_low_u16(v17)); // usubw v18.4s, v18.4s, v17.4h
    let v19 = vsubw_high_u16(v19, v17); // usubw2 v19.4s, v19.4s, v17.8h

    let v16 = vqrshrun_n_s32::<4>(vreinterpretq_s32_u32(v18)); // sqrshrun v16.4h, v18.4s, #4
    let v16 = vqrshrun_high_n_s32::<4>(v16, vreinterpretq_s32_u32(v19)); // sqrshrun2 v16.8h, v19.4s, #4

    let x5 = x0.add(16); // add x5, x0, #2*16

    let v16 = vreinterpretq_u16_s16(vminq_s16(vreinterpretq_s16_u16(v16), v30)); // smin v16.8h, v16.8h, v30.8h

    let v4 = vzip1q_u16(v0, v16); // zip1 v4.8h, v0.8h, v16.8h
    let v5 = vzip2q_u16(v0, v16); // zip2 v5.8h, v0.8h, v16.8h

    vst1q_lane_u16::<0>(x5, v2); // st1 {v2.h}[0], [x5]
                                 // In case sz=8, output one single pixel in out[16].
    vst1q_u16_x2(x0, uint16x8x2_t(v4, v5)); // st1 {v4.8h, v5.8h}, [x0]
}

/// `edge_filter`
static EDGE_FILTER: [u16; 8] = [
    0, 4, 8, 0, //
    0, 5, 6, 0,
    // Leaving out the coeffs for strength=3
    //  2, 4, 4, 0
];

/// "Filter one block" of the three-tap path of `ipred_z1_filter_edge`
/// without its `urshr`/`st1`: `v30` holds kernel[1], `v31` kernel[2]; the
/// result is the assembly's `v16`, `v17` after `urshr`.
#[inline(always)]
unsafe fn ipred_z1_filter_edge_tap3(
    v0: uint16x8_t,
    v1: uint16x8_t,
    v2: uint16x8_t,
    v30: uint16x8_t,
    v31: uint16x8_t,
) -> uint16x8x2_t {
    let v3 = vextq_u16::<1>(v0, v1); // ext v3.16b, v0.16b, v1.16b, #2
    let v4 = vextq_u16::<1>(v1, v2); // ext v4.16b, v1.16b, v2.16b, #2
    let v5 = vextq_u16::<2>(v0, v1); // ext v5.16b, v0.16b, v1.16b, #4
    let v6 = vextq_u16::<2>(v1, v2); // ext v6.16b, v1.16b, v2.16b, #4
    let v16 = vmulq_u16(v0, v30); // mul v16.8h, v0.8h, v30.8h
    let v16 = vmlaq_u16(v16, v3, v31); // mla v16.8h, v3.8h, v31.8h
    let v16 = vmlaq_u16(v16, v5, v30); // mla v16.8h, v5.8h, v30.8h
    let v17 = vmulq_u16(v1, v30); // mul v17.8h, v1.8h, v30.8h
    let v17 = vmlaq_u16(v17, v4, v31); // mla v17.8h, v4.8h, v31.8h
    let v17 = vmlaq_u16(v17, v6, v30); // mla v17.8h, v6.8h, v30.8h
    let v16 = vrshrq_n_u16::<4>(v16); // urshr v16.8h, v16.8h, #4
    let v17 = vrshrq_n_u16::<4>(v17); // urshr v17.8h, v17.8h, #4
    uint16x8x2_t(v16, v17)
}

/// "Filter one block" of `L(fivetap)` of `ipred_z1_filter_edge`: `v29` = 2,
/// `v30` = `v31` = 4; the result is the assembly's `v20`, `v21` after
/// `urshr`.
#[inline(always)]
unsafe fn ipred_z1_filter_edge_tap5(
    v0: uint16x8_t,
    v1: uint16x8_t,
    v2: uint16x8_t,
    v29: uint16x8_t,
    v30: uint16x8_t,
    v31: uint16x8_t,
) -> uint16x8x2_t {
    let v3 = vextq_u16::<1>(v0, v1); // ext v3.16b, v0.16b, v1.16b, #2
    let v4 = vextq_u16::<1>(v1, v2); // ext v4.16b, v1.16b, v2.16b, #2
    let v5 = vextq_u16::<2>(v0, v1); // ext v5.16b, v0.16b, v1.16b, #4
    let v6 = vextq_u16::<2>(v1, v2); // ext v6.16b, v1.16b, v2.16b, #4
    let v16 = vextq_u16::<3>(v0, v1); // ext v16.16b, v0.16b, v1.16b, #6
    let v17 = vextq_u16::<3>(v1, v2); // ext v17.16b, v1.16b, v2.16b, #6
    let v18 = vextq_u16::<4>(v0, v1); // ext v18.16b, v0.16b, v1.16b, #8
    let v19 = vextq_u16::<4>(v1, v2); // ext v19.16b, v1.16b, v2.16b, #8
    let v20 = vmulq_u16(v0, v29); // mul v20.8h, v0.8h, v29.8h
    let v20 = vmlaq_u16(v20, v3, v30); // mla v20.8h, v3.8h, v30.8h
    let v20 = vmlaq_u16(v20, v5, v31); // mla v20.8h, v5.8h, v31.8h
    let v20 = vmlaq_u16(v20, v16, v30); // mla v20.8h, v16.8h, v30.8h
    let v20 = vmlaq_u16(v20, v18, v29); // mla v20.8h, v18.8h, v29.8h
    let v21 = vmulq_u16(v1, v29); // mul v21.8h, v1.8h, v29.8h
    let v21 = vmlaq_u16(v21, v4, v30); // mla v21.8h, v4.8h, v30.8h
    let v21 = vmlaq_u16(v21, v6, v31); // mla v21.8h, v6.8h, v31.8h
    let v21 = vmlaq_u16(v21, v17, v30); // mla v21.8h, v17.8h, v30.8h
    let v21 = vmlaq_u16(v21, v19, v29); // mla v21.8h, v19.8h, v29.8h
    let v20 = vrshrq_n_u16::<4>(v20); // urshr v20.8h, v20.8h, #4
    let v21 = vrshrq_n_u16::<4>(v21); // urshr v21.8h, v21.8h, #4
    uint16x8x2_t(v20, v21)
}

/// `dav1d_ipred_z1_filter_edge_16bpc_neon(pixel *out, const int sz, const pixel *const in, const int end,
/// const int strength)`
///
/// # Safety
/// `strength` is 1, 2 or 3 and `end` >= 0 (the callers pass at least 4).
/// Block `j` (16 output pixels) loads 16 input pixels more before it
/// decides between filtering and padding, and the padding starts at the
/// first block with `end - 16*j < 17` (`< 18` at strength 3). With `j` the
/// last block that loads, the smaller of that block and the last output
/// block:
/// - strength 1, 2: reads `in[0..16*j+24]`, and `in[end]` when the padding
///   starts; writes `out[0..16*(j+1)]` and, when the padding started and
///   `sz` is not reached, 8 pixels more per further 16 of `sz` (the
///   assembly's loop `5:` counts 16 and stores 8; the wrappers never ask for
///   more than 8 pixels there);
/// - strength 3: reads `in[-1..16*j+23]` (`in[-1]` is loaded and replaced
///   by `in[0]`), and `in[end]` when the padding starts; writes `out[0..n]`
///   with `n` = `sz` rounded up to 16 while blocks are filtered, then to 8.
/// The loads reach up to 22 pixels above `in[end]` (three taps) resp. 20
/// (five taps); those values are replaced through the padding mask or not
/// used.
pub unsafe extern "C" fn dav1d_ipred_z1_filter_edge_16bpc_neon(
    out: *mut u16,
    sz: c_int,
    in_: *const u16,
    end: c_int,
    strength: c_int,
) {
    let (mut x0, mut w1, mut x2, mut w3, w4) = (out, sz, in_, end, strength);
    // cmp w4, #3; b.eq L(fivetap) (if (strength == 3) goto fivetap)
    if w4 == 3 {
        return ipred_z1_filter_edge_fivetap(x0, w1, x2, w3);
    }

    // movrel x5, edge_filter, -6; add x5, x5, w4, uxtw #3 (edge_filter + 2*((strength - 1)*4 + 1))
    let x5 = (EDGE_FILTER.as_ptr() as *const u8).wrapping_add(((w4 as u32 as usize) << 3).wrapping_sub(6));

    // ld1 {v31.s}[0], [x5] (kernel[1-2])
    let v31 = vreinterpretq_u16_u32(vsetq_lane_u32::<0>(
        core::ptr::read_unaligned(x5 as *const u32),
        vdupq_n_u32(0),
    ));

    let mut v0 = vld1q_u16(x2); // ld1 {v0.8h}, [x2], #16
    x2 = x2.wrapping_add(8);

    let v30 = vdupq_laneq_u16::<0>(v31); // dup v30.8h, v31.h[0]
    let v31 = vdupq_laneq_u16::<1>(v31); // dup v31.8h, v31.h[1]
    let mut v1;
    loop {
        // 1:
        // in[end], is the last valid pixel. We produce 16 pixels out by
        // using 18 pixels in - the last pixel used is [17] of the ones
        // read/buffered.
        let v2;
        uint16x8x2_t(v1, v2) = vld1q_u16_x2(x2); // cmp w3, #17; ld1 {v1.8h, v2.8h}, [x2], #32
        x2 = x2.wrapping_add(16);
        if w3 < 17 {
            break; // b.lt 2f
        }
        let v16_17 = ipred_z1_filter_edge_tap3(v0, v1, v2, v30, v31);
        w1 -= 16; // subs w1, w1, #16
        v0 = v2; // mov v0.16b, v2.16b
        w3 -= 16; // sub w3, w3, #16
        vst1q_u16_x2(x0, v16_17); // st1 {v16.8h, v17.8h}, [x0], #32
        x0 = x0.wrapping_add(16);
        if w1 <= 0 {
            return; // b.gt 1b; ret
        }
    }
    // 2:
    // Right padding

    // x2[w3-24] is the padding pixel (x2 points 24 pixels ahead)
    let x5 = padding_mask(); // movrel x5, padding_mask
    let w6 = w3 - 24; // sub w6, w3, #24
    let x5 = x5.sub((w3 as u32 as usize) << 1); // sub x5, x5, w3, uxtw #1
    let x6 = x2.wrapping_offset(w6 as isize); // add x6, x2, w6, sxtw #1

    let uint8x16x2_t(v3, v4) = vld1q_u8_x2(x5); // ld1 {v3.8h, v4.8h}, [x5] (padding_mask)

    let v2 = vld1q_dup_u16(x6); // ld1r {v2.8h}, [x6]
    let v0 = ipred_z1_bit(v0, v2, v3); // bit v0.16b, v2.16b, v3.16b (Pad v0-v1)
    let v1 = ipred_z1_bit(v1, v2, v4); // bit v1.16b, v2.16b, v4.16b

    // Filter one block
    let v16_17 = ipred_z1_filter_edge_tap3(v0, v1, v2, v30, v31);
    w1 -= 16; // subs w1, w1, #16
    vst1q_u16_x2(x0, v16_17); // st1 {v16.8h, v17.8h}, [x0], #32
    x0 = x0.wrapping_add(16);
    if w1 <= 0 {
        return; // b.le 9f
    }
    loop {
        // 5:
        // After one block, any remaining output would only be filtering
        // padding - thus just store the padding.
        w1 -= 16; // subs w1, w1, #16
        vst1q_u16(x0, v2); // st1 {v2.16b}, [x0], #16
        x0 = x0.wrapping_add(8);
        if w1 <= 0 {
            return; // b.gt 5b; 9: ret
        }
    }
}

/// `L(fivetap)` of `ipred_z1_filter_edge_16bpc_neon` (strength 3), with the
/// function's registers `x0` (out), `w1` (sz), `x2` (in), `w3` (end).
#[inline(always)]
unsafe fn ipred_z1_filter_edge_fivetap(mut x0: *mut u16, mut w1: c_int, x2: *const u16, mut w3: c_int) {
    let mut x2 = x2.sub(1); // sub x2, x2, #2 (topleft -= 1 pixel)
    let v29 = vdupq_n_u16(2); // movi v29.8h, #2
    let mut v0 = vld1q_u16(x2); // ld1 {v0.8h}, [x2], #16
    x2 = x2.wrapping_add(8);
    let v30 = vdupq_n_u16(4); // movi v30.8h, #4
    let v31 = vdupq_n_u16(4); // movi v31.8h, #4
    v0 = vsetq_lane_u16::<0>(vgetq_lane_u16::<1>(v0), v0); // ins v0.h[0], v0.h[1]
    let (mut v1, mut v2);
    loop {
        // 1:
        // in[end+1], is the last valid pixel. We produce 16 pixels out by
        // using 20 pixels in - the last pixel used is [19] of the ones
        // read/buffered.
        uint16x8x2_t(v1, v2) = vld1q_u16_x2(x2); // cmp w3, #18; ld1 {v1.8h, v2.8h}, [x2], #32
        x2 = x2.wrapping_add(16);
        if w3 < 18 {
            break; // b.lt 2f (if (end + 1 < 19))
        }
        let v20_21 = ipred_z1_filter_edge_tap5(v0, v1, v2, v29, v30, v31);
        w1 -= 16; // subs w1, w1, #16
        v0 = v2; // mov v0.16b, v2.16b
        w3 -= 16; // sub w3, w3, #16
        vst1q_u16_x2(x0, v20_21); // st1 {v20.8h, v21.8h}, [x0], #32
        x0 = x0.wrapping_add(16);
        if w1 <= 0 {
            return; // b.gt 1b; ret
        }
    }
    // 2:
    // Right padding

    // x2[w3+1-24] is the padding pixel (x2 points 24 pixels ahead)
    let x5 = padding_mask().sub(2); // movrel x5, padding_mask, -2
    let w6 = w3 - 23; // sub w6, w3, #23
    let x5 = x5.sub((w3 as u32 as usize) << 1); // sub x5, x5, w3, uxtw #1
    let x6 = x2.wrapping_offset(w6 as isize); // add x6, x2, w6, sxtw #1

    let uint8x16x3_t(v3, v4, v5) = vld1q_u8_x3(x5); // ld1 {v3.8h, v4.8h, v5.8h}, [x5] (padding_mask)

    let v28 = vld1q_dup_u16(x6); // ld1r {v28.8h}, [x6]
    v0 = ipred_z1_bit(v0, v28, v3); // bit v0.16b, v28.16b, v3.16b (Pad v0-v2)
    v1 = ipred_z1_bit(v1, v28, v4); // bit v1.16b, v28.16b, v4.16b
    v2 = ipred_z1_bit(v2, v28, v5); // bit v2.16b, v28.16b, v5.16b
    loop {
        // 4:
        // Filter one block
        let v20_21 = ipred_z1_filter_edge_tap5(v0, v1, v2, v29, v30, v31);
        w1 -= 16; // subs w1, w1, #16
        v0 = v2; // mov v0.16b, v2.16b
        v1 = v28; // mov v1.16b, v28.16b
        v2 = v28; // mov v2.16b, v28.16b
        w3 -= 16; // sub w3, w3, #16
        vst1q_u16_x2(x0, v20_21); // st1 {v20.8h, v21.8h}, [x0], #32
        x0 = x0.wrapping_add(16);
        if w1 <= 0 {
            return; // b.le 9f
        }
        // v0-v1[w3+1] is the last valid pixel; if (w3 + 1 > 0) we need to
        // filter properly once more - aka (w3 >= 0).
        if w3 < 0 {
            break; // cmp w3, #0; b.ge 4b
        }
    }
    loop {
        // 5:
        // When w3 <= 0, all remaining pixels in v0-v1 are equal to the
        // last valid pixel - thus just output that without filtering.
        w1 -= 8; // subs w1, w1, #8
        vst1q_u16(x0, v28); // st1 {v28.8h}, [x0], #16
        x0 = x0.wrapping_add(8);
        if w1 <= 0 {
            return; // b.gt 5b; 9: ret
        }
    }
}

/// `dav1d_ipred_pixel_set_16bpc_neon(pixel *out, const pixel px, const int n)`
///
/// # Safety
/// Writes `out[0..m]` with `m` = `n` rounded up to 8 (8 for `n` <= 0);
/// reads no memory.
pub unsafe extern "C" fn dav1d_ipred_pixel_set_16bpc_neon(out: *mut u16, px: u16, n: c_int) {
    let (mut x0, mut w2) = (out, n);
    let v0 = vdupq_n_u16(px); // dup v0.8h, w1
    loop {
        // 1:
        w2 -= 8; // subs w2, w2, #8
        vst1q_u16(x0, v0); // st1 {v0.8h}, [x0], #16
        x0 = x0.wrapping_add(8);
        if w2 <= 0 {
            return; // b.gt 1b; ret
        }
    }
}

/// `dav1d_ipred_z1_fill1_16bpc_neon(pixel *dst, const ptrdiff_t stride, const pixel *const top, const int width,
/// const int height, const int dx, const int max_base_x)`
///
/// The jump table `ipred_z1_fill1_tbl` (indexed with `clz(width) - 25`) is
/// the `match` on the width. `w5` (dx) and `w7` (xpos) are 64-bit here: both
/// are non-negative and `xpos` stays far below 2^32 (at most `height * dx`,
/// `dx` < 2048), so `lsr w8, w7, #6` and the `uxtw` of the addressing give
/// the same values. A pair of `st1 ..., [x0], x1` is a store at `x0`, one at
/// `x0 + x1` and one step of two rows.
///
/// # Safety
/// `width` is 4, 8, 16, 32 or 64, `height` even and positive. Writes `dst`
/// rows `0..height`, columns `0..width` (`stride` in bytes). Reads
/// `top[max_base_x]` and, for each pair of rows whose first row has `base`
/// < `max_base_x`, at the `base` of either row: `top[base..base+8]`
/// (width 4), `top[base..base+9]` (width 8), `top[base..base+width+8]`
/// (width >= 16); the second row's `base` is not compared and can reach
/// `max_base_x + 15`.
pub unsafe extern "C" fn dav1d_ipred_z1_fill1_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    top: *const u16,
    width: c_int,
    height: c_int,
    dx: c_int,
    max_base_x: c_int,
) {
    let (mut x0, x1, x2, w3, mut w4, w5, w6) = (dst, stride, top, width, height, dx as u32 as usize, max_base_x);
    // clz w9, w3; movrel x8, ipred_z1_fill1_tbl; sub w9, w9, #25; ldrsw x9, [x8, w9, uxtw #2]
    let x10 = x2.add(w6 as u32 as usize); // add x10, x2, w6, uxtw #1 (top[max_base_x])
    let v31 = vld1q_dup_u16(x10); // ld1r {v31.8h}, [x10] (padding)
    let mut w7 = w5; // mov w7, w5
    let w15: u32 = 64; // mov w15, #64
    match w3 {
        // br x8
        4 => {
            // 40:
            loop {
                // 4:
                let w8 = w7 >> 6; // lsr w8, w7, #6 (base)
                let w9 = (w7 & 0x3e) as u32; // and w9, w7, #0x3e (frac)
                let w7b = w7.wrapping_add(w5); // add w7, w7, w5 (xpos += dx)
                let w10 = w7b >> 6; // lsr w10, w7, #6 (base)
                let w11 = (w7b & 0x3e) as u32; // and w11, w7, #0x3e (frac)
                if w8 as i32 >= w6 {
                    break; // cmp w8, w6 (base >= max_base_x); b.ge 49f
                }
                // lsl w8, w8, #1; lsl w10, w10, #1
                let v0 = vld1q_u16(x2.add(w8)); // ldr q0, [x2, w8, uxtw] (top[base])
                let v2 = vld1q_u16(x2.add(w10)); // ldr q2, [x2, w10, uxtw]
                let v4 = vdup_n_s16(w9 as i16); // dup v4.4h, w9 (frac)
                let v5 = vdup_n_s16(w11 as i16); // dup v5.4h, w11
                let v1 = vextq_u16::<1>(v0, v0); // ext v1.16b, v0.16b, v0.16b, #2 (top[base+1])
                let v3 = vextq_u16::<1>(v2, v2); // ext v3.16b, v2.16b, v2.16b, #2
                let v6 = vsub_u16(vget_low_u16(v1), vget_low_u16(v0)); // sub v6.4h, v1.4h, v0.4h
                let v7 = vsub_u16(vget_low_u16(v3), vget_low_u16(v2)); // sub v7.4h, v3.4h, v2.4h
                let v16 = vshll_n_u16::<6>(vget_low_u16(v0)); // ushll v16.4s, v0.4h, #6 (top[base]*64)
                let v17 = vshll_n_u16::<6>(vget_low_u16(v2)); // ushll v17.4s, v2.4h, #6
                                                              // smlal v16.4s, v6.4h, v4.4h (+ top[base+1]*frac)
                let v16 = vmlal_s16(vreinterpretq_s32_u32(v16), vreinterpret_s16_u16(v6), v4);
                // smlal v17.4s, v7.4h, v5.4h
                let v17 = vmlal_s16(vreinterpretq_s32_u32(v17), vreinterpret_s16_u16(v7), v5);
                let v16 = vrshrn_n_u32::<6>(vreinterpretq_u32_s32(v16)); // rshrn v16.4h, v16.4s, #6
                let v17 = vrshrn_n_u32::<6>(vreinterpretq_u32_s32(v17)); // rshrn v17.4h, v17.4s, #6
                vst1_u16(x0, v16); // st1 {v16.4h}, [x0], x1
                w7 = w7.wrapping_add(w5 << 1); // add w7, w7, w5 (xpos += dx; with the one above)
                w4 -= 2; // subs w4, w4, #2
                vst1_u16(x0.wrapping_byte_offset(x1), v17); // st1 {v17.4h}, [x0], x1
                x0 = x0.wrapping_byte_offset(x1 << 1);
                if w4 <= 0 {
                    return; // b.gt 4b; ret
                }
            }
            loop {
                // 49:
                vst1_u16(x0, vget_low_u16(v31)); // st1 {v31.4h}, [x0], x1
                w4 -= 2; // subs w4, w4, #2
                vst1_u16(x0.wrapping_byte_offset(x1), vget_low_u16(v31)); // st1 {v31.4h}, [x0], x1
                x0 = x0.wrapping_byte_offset(x1 << 1);
                if w4 <= 0 {
                    return; // b.gt 49b; ret
                }
            }
        }
        8 => {
            // 80:
            loop {
                // 8:
                let w8 = w7 >> 6; // lsr w8, w7, #6 (base)
                let w9 = (w7 & 0x3e) as u32; // and w9, w7, #0x3e (frac)
                let w7b = w7.wrapping_add(w5); // add w7, w7, w5 (xpos += dx)
                let w10 = w7b >> 6; // lsr w10, w7, #6 (base)
                let w11 = (w7b & 0x3e) as u32; // and w11, w7, #0x3e (frac)
                if w8 as i32 >= w6 {
                    break; // cmp w8, w6 (base >= max_base_x); b.ge 89f
                }
                let x8 = x2.add(w8); // add x8, x2, w8, uxtw #1
                let x10 = x2.add(w10); // add x10, x2, w10, uxtw #1
                let v4 = vdupq_n_u16(w9 as u16); // dup v4.8h, w9 (frac)
                let v5 = vdupq_n_u16(w11 as u16); // dup v5.8h, w11
                let v0 = vld1q_u16(x8); // ld1 {v0.8h}, [x8] (top[base])
                let v2 = vld1q_u16(x10); // ld1 {v2.8h}, [x10]
                let w9 = w15 - w9; // sub w9, w15, w9 (64 - frac)
                let w11 = w15 - w11; // sub w11, w15, w11
                let v1 = vsetq_lane_u16::<0>(*x8.add(8), vdupq_n_u16(0)); // ldr h1, [x8, #16]
                let v3 = vsetq_lane_u16::<0>(*x10.add(8), vdupq_n_u16(0)); // ldr h3, [x10, #16]
                let v6 = vdupq_n_u16(w9 as u16); // dup v6.8h, w9 (64 - frac)
                let v7 = vdupq_n_u16(w11 as u16); // dup v7.8h, w11
                let v1 = vextq_u16::<1>(v0, v1); // ext v1.16b, v0.16b, v1.16b, #2 (top[base+1])
                let v3 = vextq_u16::<1>(v2, v3); // ext v3.16b, v2.16b, v3.16b, #2
                let v16 = vmull_u16(vget_low_u16(v0), vget_low_u16(v6)); // umull v16.4s, v0.4h, v6.4h
                let v16 = vmlal_u16(v16, vget_low_u16(v1), vget_low_u16(v4)); // umlal v16.4s, v1.4h, v4.4h
                let v17 = vmull_high_u16(v0, v6); // umull2 v17.4s, v0.8h, v6.8h
                let v17 = vmlal_high_u16(v17, v1, v4); // umlal2 v17.4s, v1.8h, v4.8h
                let v18 = vmull_u16(vget_low_u16(v2), vget_low_u16(v7)); // umull v18.4s, v2.4h, v7.4h
                let v18 = vmlal_u16(v18, vget_low_u16(v3), vget_low_u16(v5)); // umlal v18.4s, v3.4h, v5.4h
                let v19 = vmull_high_u16(v2, v7); // umull2 v19.4s, v2.8h, v7.8h
                let v19 = vmlal_high_u16(v19, v3, v5); // umlal2 v19.4s, v3.8h, v5.8h
                let v16 = vrshrn_n_u32::<6>(v16); // rshrn v16.4h, v16.4s, #6
                let v16 = vrshrn_high_n_u32::<6>(v16, v17); // rshrn2 v16.8h, v17.4s, #6
                let v17 = vrshrn_n_u32::<6>(v18); // rshrn v17.4h, v18.4s, #6
                let v17 = vrshrn_high_n_u32::<6>(v17, v19); // rshrn2 v17.8h, v19.4s, #6
                vst1q_u16(x0, v16); // st1 {v16.8h}, [x0], x1
                w7 = w7.wrapping_add(w5 << 1); // add w7, w7, w5 (xpos += dx; with the one above)
                w4 -= 2; // subs w4, w4, #2
                vst1q_u16(x0.wrapping_byte_offset(x1), v17); // st1 {v17.8h}, [x0], x1
                x0 = x0.wrapping_byte_offset(x1 << 1);
                if w4 <= 0 {
                    return; // b.gt 8b; ret
                }
            }
            loop {
                // 89:
                vst1q_u16(x0, v31); // st1 {v31.8h}, [x0], x1
                w4 -= 2; // subs w4, w4, #2
                vst1q_u16(x0.wrapping_byte_offset(x1), v31); // st1 {v31.8h}, [x0], x1
                x0 = x0.wrapping_byte_offset(x1 << 1);
                if w4 <= 0 {
                    return; // b.gt 89b; ret
                }
            }
        }
        16 => ipred_z1_fill1_160_320_640::<16>(x0, x1, x2, w4, w5, w6, v31),
        32 => ipred_z1_fill1_160_320_640::<32>(x0, x1, x2, w4, w5, w6, v31),
        _ => ipred_z1_fill1_160_320_640::<64>(x0, x1, x2, w4, w5, w6, v31),
    }
}

/// The labels `160:`, `320:`, `640:` of `ipred_z1_fill1_16bpc_neon` (one
/// shared block in the assembly), with the function's registers; `W` is the
/// width (`w3`), a constant of each instance so that the compiler sees the
/// number of 16-pixel columns.
#[inline(always)]
unsafe fn ipred_z1_fill1_160_320_640<const W: i32>(
    mut x0: *mut u16,
    x1: isize,
    x2: *const u16,
    mut w4: c_int,
    w5: usize,
    w6: c_int,
    v31: uint16x8_t,
) {
    let mut w3 = W;
    let mut w7 = w5; // mov w7, w5
    let w15: u32 = 64; // mov w15, #64
    let w12 = w3; // mov w12, w3

    let mut x13 = x0.wrapping_byte_offset(x1); // add x13, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    let x1 = x1 - ((w3 as u32 as isize) << 1); // sub x1, x1, w3, uxtw #1
    loop {
        // 1:
        let w8 = w7 >> 6; // lsr w8, w7, #6 (base)
        let w9 = (w7 & 0x3e) as u32; // and w9, w7, #0x3e (frac)
        w7 = w7.wrapping_add(w5); // add w7, w7, w5 (xpos += dx)
        let w10 = w7 >> 6; // lsr w10, w7, #6 (base)
        let w11 = (w7 & 0x3e) as u32; // and w11, w7, #0x3e (frac)
        if w8 as i32 >= w6 {
            break; // cmp w8, w6 (base >= max_base_x); b.ge 169f
        }
        let mut x8 = x2.add(w8); // add x8, x2, w8, uxtw #1
        let mut x10 = x2.add(w10); // add x10, x2, w10, uxtw #1
        let v6 = vdupq_n_u16(w9 as u16); // dup v6.8h, w9 (frac)
        let v7 = vdupq_n_u16(w11 as u16); // dup v7.8h, w11
        let uint16x8x3_t(mut v0, mut v1, mut v2) = vld1q_u16_x3(x8); // ld1 {v0.8h, v1.8h, v2.8h}, [x8], #48 (top[base])
        x8 = x8.wrapping_add(24);
        let uint16x8x3_t(mut v3, mut v4, mut v5) = vld1q_u16_x3(x10); // ld1 {v3.8h, v4.8h, v5.8h}, [x10], #48
        x10 = x10.wrapping_add(24);
        let w9 = w15 - w9; // sub w9, w15, w9 (64 - frac)
        let w11 = w15 - w11; // sub w11, w15, w11
        let v16 = vdupq_n_u16(w9 as u16); // dup v16.8h, w9 (64 - frac)
        let v17 = vdupq_n_u16(w11 as u16); // dup v17.8h, w11
        w7 = w7.wrapping_add(w5); // add w7, w7, w5 (xpos += dx)
        loop {
            // 2:
            let v18 = vextq_u16::<1>(v0, v1); // ext v18.16b, v0.16b, v1.16b, #2 (top[base+1])
            let v19 = vextq_u16::<1>(v1, v2); // ext v19.16b, v1.16b, v2.16b, #2
            let v20 = vextq_u16::<1>(v3, v4); // ext v20.16b, v3.16b, v4.16b, #2
            let v21 = vextq_u16::<1>(v4, v5); // ext v21.16b, v4.16b, v5.16b, #2
            w3 -= 16; // subs w3, w3, #16
            let v22 = vmull_u16(vget_low_u16(v0), vget_low_u16(v16)); // umull v22.4s, v0.4h, v16.4h
            let v22 = vmlal_u16(v22, vget_low_u16(v18), vget_low_u16(v6)); // umlal v22.4s, v18.4h, v6.4h
            let v23 = vmull_high_u16(v0, v16); // umull2 v23.4s, v0.8h, v16.8h
            let v23 = vmlal_high_u16(v23, v18, v6); // umlal2 v23.4s, v18.8h, v6.8h
            let v24 = vmull_u16(vget_low_u16(v1), vget_low_u16(v16)); // umull v24.4s, v1.4h, v16.4h
            let v24 = vmlal_u16(v24, vget_low_u16(v19), vget_low_u16(v6)); // umlal v24.4s, v19.4h, v6.4h
            let v25 = vmull_high_u16(v1, v16); // umull2 v25.4s, v1.8h, v16.8h
            let v25 = vmlal_high_u16(v25, v19, v6); // umlal2 v25.4s, v19.8h, v6.8h
            let v26 = vmull_u16(vget_low_u16(v3), vget_low_u16(v17)); // umull v26.4s, v3.4h, v17.4h
            let v26 = vmlal_u16(v26, vget_low_u16(v20), vget_low_u16(v7)); // umlal v26.4s, v20.4h, v7.4h
            let v27 = vmull_high_u16(v3, v17); // umull2 v27.4s, v3.8h, v17.8h
            let v27 = vmlal_high_u16(v27, v20, v7); // umlal2 v27.4s, v20.8h, v7.8h
            let v28 = vmull_u16(vget_low_u16(v4), vget_low_u16(v17)); // umull v28.4s, v4.4h, v17.4h
            let v28 = vmlal_u16(v28, vget_low_u16(v21), vget_low_u16(v7)); // umlal v28.4s, v21.4h, v7.4h
            let v29 = vmull_high_u16(v4, v17); // umull2 v29.4s, v4.8h, v17.8h
            let v29 = vmlal_high_u16(v29, v21, v7); // umlal2 v29.4s, v21.8h, v7.8h
            let v22 = vrshrn_n_u32::<6>(v22); // rshrn v22.4h, v22.4s, #6
            let v22 = vrshrn_high_n_u32::<6>(v22, v23); // rshrn2 v22.8h, v23.4s, #6
            let v23 = vrshrn_n_u32::<6>(v24); // rshrn v23.4h, v24.4s, #6
            let v23 = vrshrn_high_n_u32::<6>(v23, v25); // rshrn2 v23.8h, v25.4s, #6
            let v24 = vrshrn_n_u32::<6>(v26); // rshrn v24.4h, v26.4s, #6
            let v24 = vrshrn_high_n_u32::<6>(v24, v27); // rshrn2 v24.8h, v27.4s, #6
            let v25 = vrshrn_n_u32::<6>(v28); // rshrn v25.4h, v28.4s, #6
            let v25 = vrshrn_high_n_u32::<6>(v25, v29); // rshrn2 v25.8h, v29.4s, #6
            vst1q_u16_x2(x0, uint16x8x2_t(v22, v23)); // st1 {v22.8h, v23.8h}, [x0], #32
            x0 = x0.wrapping_add(16);
            vst1q_u16_x2(x13, uint16x8x2_t(v24, v25)); // st1 {v24.8h, v25.8h}, [x13], #32
            x13 = x13.wrapping_add(16);
            if w3 <= 0 {
                break; // b.le 3f
            }
            v0 = v2; // mov v0.16b, v2.16b
            uint16x8x2_t(v1, v2) = vld1q_u16_x2(x8); // ld1 {v1.8h, v2.8h}, [x8], #32 (top[base])
            x8 = x8.wrapping_add(16);
            v3 = v5; // mov v3.16b, v5.16b
            uint16x8x2_t(v4, v5) = vld1q_u16_x2(x10); // ld1 {v4.8h, v5.8h}, [x10], #32
            x10 = x10.wrapping_add(16);
            // b 2b
        }
        // 3:
        w4 -= 2; // subs w4, w4, #2
        if w4 <= 0 {
            return; // b.le 9f; 9: ret
        }
        x0 = x0.wrapping_byte_offset(x1); // add x0, x0, x1
        x13 = x13.wrapping_byte_offset(x1); // add x13, x13, x1
        w3 = w12; // mov w3, w12
                  // b 1b
    }
    loop {
        // 169:
        vst1q_u16(x0, v31); // st1 {v31.8h}, [x0], #16
        x0 = x0.wrapping_add(8);
        w3 -= 8; // subs w3, w3, #8
        vst1q_u16(x13, v31); // st1 {v31.8h}, [x13], #16
        x13 = x13.wrapping_add(8);
        if w3 > 0 {
            continue; // b.gt 169b
        }
        w4 -= 2; // subs w4, w4, #2
        if w4 <= 0 {
            return; // b.le 9b
        }
        x0 = x0.wrapping_byte_offset(x1); // add x0, x0, x1
        x13 = x13.wrapping_byte_offset(x1); // add x13, x13, x1
        w3 = w12; // mov w3, w12
                  // b 169b
    }
}

/// `dav1d_ipred_z1_fill2_16bpc_neon` (same parameters as `fill1`; `top` is
/// the upsampled edge, `dx` and `max_base_x` are in its units). `w5`, `w7`
/// and the pairs of stores as in `fill1`.
///
/// The padding value is taken as the assembly takes it: `add x10, x2, w6,
/// uxtw` (no `#1`) and `ld1r {v31.16b}` load the single byte at byte offset
/// `max_base_x` of `top` and replicate it to both bytes of every pixel. The
/// wrapper never reaches the rows that use it (`49:`, `89:`): with an
/// upsampled edge `base` stays below `max_base_x`.
///
/// # Safety
/// `width` is 4 or 8, `height` even and positive. Writes `dst` rows
/// `0..height`, columns `0..width` (`stride` in bytes). Reads the byte at
/// byte offset `max_base_x` of `top` and, for each pair of rows whose first
/// row has `base` < `max_base_x`, at the `base` of either row:
/// `top[base..base+8]` (width 4) or `top[base..base+16]` (width 8); the
/// second row's `base` is not compared and can reach `max_base_x + 31`.
pub unsafe extern "C" fn dav1d_ipred_z1_fill2_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    top: *const u16,
    width: c_int,
    height: c_int,
    dx: c_int,
    max_base_x: c_int,
) {
    let (mut x0, x1, x2, w3, mut w4, w5, w6) = (dst, stride, top, width, height, dx as u32 as usize, max_base_x);
    let x10 = (x2 as *const u8).add(w6 as u32 as usize); // add x10, x2, w6, uxtw (top[max_base_x])
    let v31 = vreinterpretq_u16_u8(vld1q_dup_u8(x10)); // ld1r {v31.16b}, [x10] (padding)
    let mut w7 = w5; // mov w7, w5
    let w15: u32 = 64; // mov w15, #64
    if w3 != 8 {
        // cmp w3, #8; b.eq 8f
        loop {
            // 4: (w == 4)
            let w8 = w7 >> 6; // lsr w8, w7, #6 (base)
            let w9 = (w7 & 0x3e) as u32; // and w9, w7, #0x3e (frac)
            let w7b = w7.wrapping_add(w5); // add w7, w7, w5 (xpos += dx)
            let w10 = w7b >> 6; // lsr w10, w7, #6 (base)
            let w11 = (w7b & 0x3e) as u32; // and w11, w7, #0x3e (frac)
            if w8 as i32 >= w6 {
                break; // cmp w8, w6 (base >= max_base_x); b.ge 49f
            }
            // lsl w8, w8, #1; lsl w10, w10, #1
            let v0 = vld1q_u16(x2.add(w8)); // ldr q0, [x2, w8, uxtw] (top[base])
            let v2 = vld1q_u16(x2.add(w10)); // ldr q2, [x2, w10, uxtw]
            let v4 = vdup_n_s16(w9 as i16); // dup v4.4h, w9 (frac)
            let v5 = vdup_n_s16(w11 as i16); // dup v5.4h, w11
            let v1 = vuzp2q_u16(v0, v0); // uzp2 v1.8h, v0.8h, v0.8h (top[base+1])
            let v0 = vuzp1q_u16(v0, v0); // uzp1 v0.8h, v0.8h, v0.8h (top[base])
            let v3 = vuzp2q_u16(v2, v2); // uzp2 v3.8h, v2.8h, v2.8h
            let v2 = vuzp1q_u16(v2, v2); // uzp1 v2.8h, v2.8h, v2.8h
            let v6 = vsub_u16(vget_low_u16(v1), vget_low_u16(v0)); // sub v6.4h, v1.4h, v0.4h (top[base+1]-top[base])
            let v7 = vsub_u16(vget_low_u16(v3), vget_low_u16(v2)); // sub v7.4h, v3.4h, v2.4h
            let v16 = vshll_n_u16::<6>(vget_low_u16(v0)); // ushll v16.4s, v0.4h, #6 (top[base]*64)
            let v17 = vshll_n_u16::<6>(vget_low_u16(v2)); // ushll v17.4s, v2.4h, #6
                                                          // smlal v16.4s, v6.4h, v4.4h (+ top[base+1]*frac)
            let v16 = vmlal_s16(vreinterpretq_s32_u32(v16), vreinterpret_s16_u16(v6), v4);
            // smlal v17.4s, v7.4h, v5.4h
            let v17 = vmlal_s16(vreinterpretq_s32_u32(v17), vreinterpret_s16_u16(v7), v5);
            let v16 = vrshrn_n_u32::<6>(vreinterpretq_u32_s32(v16)); // rshrn v16.4h, v16.4s, #6
            let v17 = vrshrn_n_u32::<6>(vreinterpretq_u32_s32(v17)); // rshrn v17.4h, v17.4s, #6
            vst1_u16(x0, v16); // st1 {v16.4h}, [x0], x1
            w7 = w7.wrapping_add(w5 << 1); // add w7, w7, w5 (xpos += dx; with the one above)
            w4 -= 2; // subs w4, w4, #2
            vst1_u16(x0.wrapping_byte_offset(x1), v17); // st1 {v17.4h}, [x0], x1
            x0 = x0.wrapping_byte_offset(x1 << 1);
            if w4 <= 0 {
                return; // b.gt 4b; ret
            }
        }
        loop {
            // 49:
            vst1_u16(x0, vget_low_u16(v31)); // st1 {v31.4h}, [x0], x1
            w4 -= 2; // subs w4, w4, #2
            vst1_u16(x0.wrapping_byte_offset(x1), vget_low_u16(v31)); // st1 {v31.4h}, [x0], x1
            x0 = x0.wrapping_byte_offset(x1 << 1);
            if w4 <= 0 {
                return; // b.gt 49b; ret
            }
        }
    }

    loop {
        // 8: (w == 8)
        let w8 = w7 >> 6; // lsr w8, w7, #6 (base)
        let w9 = (w7 & 0x3e) as u32; // and w9, w7, #0x3e (frac)
        let w7b = w7.wrapping_add(w5); // add w7, w7, w5 (xpos += dx)
        let w10 = w7b >> 6; // lsr w10, w7, #6 (base)
        let w11 = (w7b & 0x3e) as u32; // and w11, w7, #0x3e (frac)
        if w8 as i32 >= w6 {
            break; // cmp w8, w6 (base >= max_base_x); b.ge 89f
        }
        let x8 = x2.add(w8); // add x8, x2, w8, uxtw #1
        let x10 = x2.add(w10); // add x10, x2, w10, uxtw #1
        let v4 = vdupq_n_u16(w9 as u16); // dup v4.8h, w9 (frac)
        let v5 = vdupq_n_u16(w11 as u16); // dup v5.8h, w11
        let uint16x8x2_t(v0, v1) = vld1q_u16_x2(x8); // ld1 {v0.8h, v1.8h}, [x8] (top[base])
        let uint16x8x2_t(v2, v3) = vld1q_u16_x2(x10); // ld1 {v2.8h, v3.8h}, [x10]
        let w9 = w15 - w9; // sub w9, w15, w9 (64 - frac)
        let w11 = w15 - w11; // sub w11, w15, w11
        let v6 = vdupq_n_u16(w9 as u16); // dup v6.8h, w9 (64 - frac)
        let v7 = vdupq_n_u16(w11 as u16); // dup v7.8h, w11
        let v20 = vuzp2q_u16(v0, v1); // uzp2 v20.8h, v0.8h, v1.8h (top[base+1])
        let v0 = vuzp1q_u16(v0, v1); // uzp1 v0.8h, v0.8h, v1.8h (top[base])
        let v21 = vuzp2q_u16(v2, v3); // uzp2 v21.8h, v2.8h, v3.8h
        let v2 = vuzp1q_u16(v2, v3); // uzp1 v2.8h, v2.8h, v3.8h
        let v16 = vmull_u16(vget_low_u16(v0), vget_low_u16(v6)); // umull v16.4s, v0.4h, v6.4h (top[base]*(64-frac))
        let v16 = vmlal_u16(v16, vget_low_u16(v20), vget_low_u16(v4)); // umlal v16.4s, v20.4h, v4.4h
        let v17 = vmull_high_u16(v0, v6); // umull2 v17.4s, v0.8h, v6.8h
        let v17 = vmlal_high_u16(v17, v20, v4); // umlal2 v17.4s, v20.8h, v4.8h
        let v18 = vmull_u16(vget_low_u16(v2), vget_low_u16(v7)); // umull v18.4s, v2.4h, v7.4h
        let v18 = vmlal_u16(v18, vget_low_u16(v21), vget_low_u16(v5)); // umlal v18.4s, v21.4h, v5.4h
        let v19 = vmull_high_u16(v2, v7); // umull2 v19.4s, v2.8h, v7.8h
        let v19 = vmlal_high_u16(v19, v21, v5); // umlal2 v19.4s, v21.8h, v5.8h
        let v16 = vrshrn_n_u32::<6>(v16); // rshrn v16.4h, v16.4s, #6
        let v16 = vrshrn_high_n_u32::<6>(v16, v17); // rshrn2 v16.8h, v17.4s, #6
        let v17 = vrshrn_n_u32::<6>(v18); // rshrn v17.4h, v18.4s, #6
        let v17 = vrshrn_high_n_u32::<6>(v17, v19); // rshrn2 v17.8h, v19.4s, #6
        vst1q_u16(x0, v16); // st1 {v16.8h}, [x0], x1
        w7 = w7.wrapping_add(w5 << 1); // add w7, w7, w5 (xpos += dx; with the one above)
        w4 -= 2; // subs w4, w4, #2
        vst1q_u16(x0.wrapping_byte_offset(x1), v17); // st1 {v17.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1 << 1);
        if w4 <= 0 {
            return; // b.gt 8b; ret
        }
    }
    loop {
        // 89:
        vst1q_u16(x0, v31); // st1 {v31.8h}, [x0], x1
        w4 -= 2; // subs w4, w4, #2
        vst1q_u16(x0.wrapping_byte_offset(x1), v31); // st1 {v31.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1 << 1);
        if w4 <= 0 {
            return; // b.gt 89b; ret
        }
    }
}

/// `dav1d_ipred_reverse_16bpc_neon(pixel *dst, const pixel *const src, const int n)`
///
/// # Safety
/// With `m` = `n` rounded up to 8 (8 for `n` <= 0): reads `src[-m..0]` and
/// writes `dst[0..m]`, `dst[i] = src[-1 - i]`.
pub unsafe extern "C" fn dav1d_ipred_reverse_16bpc_neon(dst: *mut u16, src: *const u16, n: c_int) {
    let (mut x0, mut x1, mut w2) = (dst, src, n);
    x1 = x1.wrapping_sub(8); // sub x1, x1, #16
    let mut x3 = x0.wrapping_add(4); // add x3, x0, #8
                                     // mov x4, #16
    loop {
        // 1:
        let v0 = vld1q_u16(x1); // ld1 {v0.8h}, [x1]
        w2 -= 8; // subs w2, w2, #8
        let v0 = vreinterpretq_u64_u16(vrev64q_u16(v0)); // rev64 v0.8h, v0.8h
        x1 = x1.wrapping_sub(8); // sub x1, x1, #16
        core::ptr::write_unaligned(x0 as *mut u64, vgetq_lane_u64::<1>(v0)); // st1 {v0.d}[1], [x0], x4
        x0 = x0.wrapping_add(8);
        core::ptr::write_unaligned(x3 as *mut u64, vgetq_lane_u64::<0>(v0)); // st1 {v0.d}[0], [x3], x4
        x3 = x3.wrapping_add(8);
        if w2 <= 0 {
            return; // b.gt 1b; ret
        }
    }
}

// ============================================================================
// Part C (ipred16.S lines 2062–3450): the z2 fill kernels.
// ============================================================================
// The vector registers are typed locals named after the assembly's
// registers (`v16`, `v29`, ...): pixels, positions and fractions are 16-bit
// lanes, the table indices bytes; a register that holds the 32-bit products
// of a `umull`/`umlal` pair is the local with the suffix `s` (`v21s`), and a
// multi-register table (`{v0.16b, v1.16b}`) is the local named after its
// registers (`v0v1`). General registers keep their names (`x0`, `w8`, ...);
// `x0`, `x2` and `x3` are pixel pointers that the assembly's byte offsets
// move with `wrapping_byte_offset`.

/// `const increments, align=4`
static INCREMENTS: [u16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

/// A register's `.8h` view.
#[inline(always)]
unsafe fn z2_8h(a: uint8x16_t) -> uint16x8_t {
    vreinterpretq_u16_u8(a)
}

/// A register's `.16b` view.
#[inline(always)]
unsafe fn z2_16b(a: uint16x8_t) -> uint8x16_t {
    vreinterpretq_u8_u16(a)
}

/// `trn1 vd.2d, vn.2d, vm.2d`
#[inline(always)]
unsafe fn z2_trn1_2d(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    vreinterpretq_u16_u64(vtrn1q_u64(vreinterpretq_u64_u16(a), vreinterpretq_u64_u16(b)))
}

/// `smov w, v.b[LANE]` as the sign-extended offset of an address.
#[inline(always)]
unsafe fn z2_smov_b<const LANE: i32>(v: uint8x16_t) -> isize {
    vgetq_lane_s8::<LANE>(vreinterpretq_s8_u8(v)) as isize
}

/// `ipred_z2_fill1_16bpc_neon`, label `40:` (width 4).
///
/// # Safety
/// As for `dav1d_ipred_z2_fill1_16bpc_neon` with `width == 4`; `w8` is the
/// entry's `xpos`, `w7` is `-dy`, `v31` is `increments`.
#[inline(never)]
unsafe fn ipred_z2_fill1_w4(
    mut x0: *mut u16,
    x1: isize,
    x2: *const u16,
    x3: *const u16,
    mut w8: c_int,
    mut w5: c_int,
    w6: c_int,
    w7: c_int,
    v31: uint16x8_t,
) {
    let v30 = vdup_n_u16(w7 as u16); // dup v30.4h, w7  (-dy)
    let v17 = vdup_n_u8(1); // movi v17.8b, #1

    let v16 = vmul_u16(vget_low_u16(v31), v30); // mul v16.4h, v31.4h, v30.4h  ({0,1,2,3} * -dy)
    let v25 = vdupq_n_u16(0x3e); // movi v25.8h, #0x3e
    let v30 = vadd_u16(v16, v30); // add v30.4h, v16.4h, v30.4h  (-= dy)

    // Worst case height for w=4 is 16, but we need at least h+1 elements
    let v0v1v2 = vld1q_u8_x3(x3 as *const u8); // ld1 {v0.8h, v1.8h, v2.8h}, [x3]  (left[])
    let v0v1 = uint8x16x2_t(v0v1v2.0, v0v1v2.1);

    let v26 = vdupq_n_u16(64); // movi v26.8h, #64
    let v19 = vdup_n_u8(4); // movi v19.16b, #4

    let v29 = vshrn_n_u16::<6>(vcombine_u16(v30, vdup_n_u16(0))); // shrn v29.8b, v30.8h, #6  (ypos >> 6)
    let v27 = vand_u16(v30, vget_low_u16(v25)); // and v27.8b, v30.8b, v25.8b  (frac_y)

    let v29 = vadd_u8(v29, v17); // add v29.8b, v29.8b, v17.8b  (base_y = (ypos >> 6) + 1)

    let v23 = vreinterpret_u8_u16(vdup_n_u16(1 << 8)); // movi v23.4h, #1, lsl #8
    let v29 = vshl_n_u8::<1>(v29); // shl v29.8b, v29.8b, #1  (2*base_y)
    let v29 = vzip1_u8(v29, v29); // zip1 v29.8b, v29.8b, v29.8b  (duplicate elements)
    let v17 = vdup_n_u8(2); // movi v17.8b, #2
    let v29 = vadd_u8(v29, v23); // add v29.8b, v29.8b, v23.8b  (2*base, 2*base+1, ...)

    let v30 = vadd_u8(v29, v17); // add v30.8b, v29.8b, v17.8b  (base_y + 1 (*2))
    let v28 = vadd_u8(v29, v19); // add v28.8b, v29.8b, v19.8b  (base_y + 2 (*2))

    let v18 = vqtbl1_u8(v0v1v2.0, v29); // tbl v18.8b, {v0.16b}, v29.8b  (left[base_y])
    let mut v18 = z2_8h(vcombine_u8(v18, vdup_n_u8(0)));

    let mut v30 = vcombine_u8(v30, v28); // trn1 v30.2d, v30.2d, v28.2d  (base_y + 1, base_y + 2)

    let v28 = vsub_u16(vget_low_u16(v26), v27); // sub v28.4h, v26.4h, v27.4h  (64 - frac_y)

    let v31 = z2_trn1_2d(v31, v31); // trn1 v31.2d, v31.2d, v31.2d  ({0,1,2,3,0,1,2,3})

    let v27 = vcombine_u16(v27, v27); // trn1 v27.2d, v27.2d, v27.2d  (frac_y)
    let v28 = vcombine_u16(v28, v28); // trn1 v28.2d, v28.2d, v28.2d  (64 - frac_y)

    let v29 = vdupq_n_u8(4); // movi v29.16b, #4
    loop {
        // 4:
        let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
        let v16 = vdup_n_u16(w8 as u16); // dup v16.4h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
        if w9 <= -4 {
            break; // cmp w9, #-4; b.le 49f  (base_x <= -4)
        }

        let w9 = w9 << 1; // lsl w9, w9, #1
        let w11 = w11 << 1; // lsl w11, w11, #1

        let v17 = vdup_n_u16(w8 as u16); // dup v17.4h, w8  (xpos)

        let v4 = vld1q_u16(x2.wrapping_byte_offset(w9 as isize)); // ldr q4, [x2, w9, sxtw]  (top[base_x])
        let v6 = vld1q_u16(x2.wrapping_byte_offset(w11 as isize)); // ldr q6, [x2, w11, sxtw]

        let v16 = vcombine_u16(v16, v17); // trn1 v16.2d, v16.2d, v17.2d  (xpos)

        // Cut corners here; only doing tbl over v0-v1 here; we only
        // seem to need the last pixel, from v2, after skipping to the
        // left-only codepath below.
        let v19 = z2_8h(vqtbl2q_u8(v0v1, v30)); // tbl v19.16b, {v0.16b, v1.16b}, v30.16b  (left[base_y+1], left[base_y+2])

        let v20 = vshrq_n_s16::<6>(vreinterpretq_s16_u16(v16)); // sshr v20.8h, v16.8h, #6  (first base_x for each row)

        let v5 = vextq_u16::<1>(v4, v4); // ext v5.16b, v4.16b, v4.16b, #2  (top[base_x+1])
        let v7 = vextq_u16::<1>(v6, v6); // ext v7.16b, v6.16b, v6.16b, #2

        let v16 = vandq_u16(v16, v25); // and v16.16b, v16.16b, v25.16b  (frac_x)

        v18 = z2_trn1_2d(v18, v19); // trn1 v18.2d, v18.2d, v19.2d  (left[base_y], left[base_y+1])

        let v4 = z2_trn1_2d(v4, v6); // trn1 v4.2d, v4.2d, v6.2d  (top[base_x])
        let v5 = z2_trn1_2d(v5, v7); // trn1 v5.2d, v5.2d, v7.2d  (top[base_x+1])

        let v17 = vsubq_u16(v26, v16); // sub v17.8h, v26.8h, v16.8h  (64 - frac_x)

        let v20 = vaddq_s16(v20, vreinterpretq_s16_u16(v31)); // add v20.8h, v20.8h, v31.8h  (actual base_x)

        let v21s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v21.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
        let v21s = vmlal_u16(v21s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v21.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
        let v22s = vmull_high_u16(v18, v28); // umull2 v22.4s, v18.8h, v28.8h
        let v22s = vmlal_high_u16(v22s, v19, v27); // umlal2 v22.4s, v19.8h, v27.8h

        let v23s = vmull_u16(vget_low_u16(v4), vget_low_u16(v17)); // umull v23.4s, v4.4h, v17.4h  (top[base_x]*(64-frac_x))
        let v23s = vmlal_u16(v23s, vget_low_u16(v5), vget_low_u16(v16)); // umlal v23.4s, v5.4h, v16.4h  (+ top[base_x+1]*frac_x)
        let v24s = vmull_high_u16(v4, v17); // umull2 v24.4s, v4.8h, v17.8h
        let v24s = vmlal_high_u16(v24s, v5, v16); // umlal2 v24.4s, v5.8h, v16.8h

        let v20 = vcgezq_s16(v20); // cmge v20.8h, v20.8h, #0

        let v21 = vrshrn_n_u32::<6>(v21s); // rshrn v21.4h, v21.4s, #6
        let v21 = vrshrn_high_n_u32::<6>(v21, v22s); // rshrn2 v21.8h, v22.4s, #6
        let v22 = vrshrn_n_u32::<6>(v23s); // rshrn v22.4h, v23.4s, #6
        let v22 = vrshrn_high_n_u32::<6>(v22, v24s); // rshrn2 v22.8h, v24.4s, #6

        let v21 = vbslq_u16(v20, v22, v21); // bit v21.16b, v22.16b, v20.16b

        vst1_u16(x0, vget_low_u16(v21)); // st1 {v21.d}[0], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        w5 -= 2; // subs w5, w5, #2
        vst1_u16(x0, vget_high_u16(v21)); // st1 {v21.d}[1], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = vextq_u16::<4>(v19, v19); // ext v18.16b, v19.16b, v19.16b, #8
        v30 = vaddq_u8(v30, v29); // add v30.16b, v30.16b, v29.16b  (base_y += 2 (*2))
    }

    loop {
        // 49:
        let v19 = z2_8h(vqtbl3q_u8(v0v1v2, v30)); // tbl v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b  (left[base_y+1], left[base_y+2])

        v18 = z2_trn1_2d(v18, v19); // trn1 v18.2d, v18.2d, v19.2d  (left[base_y], left[base_y+1])

        let v20s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v20.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
        let v20s = vmlal_u16(v20s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v20.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
        let v21s = vmull_high_u16(v18, v28); // umull2 v21.4s, v18.8h, v28.8h
        let v21s = vmlal_high_u16(v21s, v19, v27); // umlal2 v21.4s, v19.8h, v27.8h

        let v20 = vrshrn_n_u32::<6>(v20s); // rshrn v20.4h, v20.4s, #6
        let v20 = vrshrn_high_n_u32::<6>(v20, v21s); // rshrn2 v20.8h, v21.4s, #6

        vst1_u16(x0, vget_low_u16(v20)); // st1 {v20.d}[0], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        vst1_u16(x0, vget_high_u16(v20)); // st1 {v20.d}[1], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = vextq_u16::<4>(v19, v19); // ext v18.16b, v19.16b, v19.16b, #8
        v30 = vaddq_u8(v30, v29); // add v30.16b, v30.16b, v29.16b  (base_y += 2 (*2))
    }
}

/// `ipred_z2_fill1_16bpc_neon`, label `80:` (width 8). The saves of
/// `d8`-`d15` are the compiler's.
///
/// # Safety
/// As for `dav1d_ipred_z2_fill1_16bpc_neon` with `width == 8`; `w8` is the
/// entry's `xpos`, `w7` is `-dy`, `v31` is `increments`.
#[inline(never)]
unsafe fn ipred_z2_fill1_w8(
    mut x0: *mut u16,
    x1: isize,
    x2: *const u16,
    x3: *const u16,
    mut w8: c_int,
    mut w5: c_int,
    w6: c_int,
    w7: c_int,
    v31: uint16x8_t,
) {
    let v18 = vdupq_n_u16(w7 as u16); // dup v18.8h, w7  (-dy)
    let x3 = x3.add(1); // add x3, x3, #2  (Skip past left[0])

    let v16 = vmulq_u16(v31, v18); // mul v16.8h, v31.8h, v18.8h  ({0,1,2,3,4,5,6,7} * -dy)
    let v25 = vdupq_n_u16(0x3e); // movi v25.8h, #0x3e
    let v16 = vaddq_u16(v16, v18); // add v16.8h, v16.8h, v18.8h  (-= dy)

    // Worst case height for w=8 is 32.
    let v0v1v2v3 = vld1q_u8_x4(x3 as *const u8); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x3]  (left[])
    let v15 = vld1q_dup_u16(x2); // ld1r {v15.8h}, [x2]  (left[0] == top[0])

    let v26 = vdupq_n_u16(64); // movi v26.8h, #64
    let v19 = vdupq_n_u8(4); // movi v19.16b, #4

    let v29 = vshrn_n_u16::<6>(v16); // shrn v29.8b, v16.8h, #6  (ypos >> 6)
    let v27 = vandq_u16(v16, v25); // and v27.16b, v16.16b, v25.16b  (frac_y)

    let v23 = z2_16b(vdupq_n_u16(1 << 8)); // movi v23.8h, #1, lsl #8
    let v29 = vshl_n_u8::<1>(v29); // shl v29.8b, v29.8b, #1  (2*base_y)
    let v18 = v15; // mov v18.16b, v15.16b  (left[0])
    let v29 = vcombine_u8(v29, vdup_n_u8(0));
    let v29 = vzip1q_u8(v29, v29); // zip1 v29.16b, v29.16b, v29.16b  (duplicate elements)
    let v17 = vdupq_n_u8(2); // movi v17.16b, #2
    let v29 = vaddq_u8(v29, v23); // add v29.16b, v29.16b, v23.16b  (2*base, 2*base+1, ...)

    // Cut corners here; for the first row we don't expect to need to
    // read outside of v0.
    let mut v18 = z2_8h(vqtbx1q_u8(z2_16b(v18), v0v1v2v3.0, v29)); // tbx v18.16b, {v0.16b}, v29.16b  (left[base_y])

    let mut v30 = vaddq_u8(v29, v19); // add v30.16b, v29.16b, v19.16b  (base_y + 2 (*2))
    let mut v29 = vaddq_u8(v29, v17); // add v29.16b, v29.16b, v17.16b  (base_y + 1 (*2))

    let v28 = vsubq_u16(v26, v27); // sub v28.8h, v26.8h, v27.8h  (64 - frac_y)

    let v24 = vdupq_n_u8(4); // movi v24.16b, #4
    loop {
        // 8:
        let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
        let v16 = vdupq_n_u16(w8 as u16); // dup v16.8h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
        if w9 <= -16 {
            break; // cmp w9, #-16; b.le 89f  (base_x <= -16)
        }

        let v17 = vdupq_n_u16(w8 as u16); // dup v17.8h, w8  (xpos)

        let x9 = x2.wrapping_offset(w9 as isize); // add x9, x2, w9, sxtw #1
        let x11 = x2.wrapping_offset(w11 as isize); // add x11, x2, w11, sxtw #1

        let v4v5 = vld1q_u16_x2(x9); // ld1 {v4.8h, v5.8h}, [x9]  (top[base_x])
        let v19 = v15; // mov v19.16b, v15.16b  (left[0])
        let v6v7 = vld1q_u16_x2(x11); // ld1 {v6.8h, v7.8h}, [x11]
        let (v4, v6) = (v4v5.0, v6v7.0);

        let v19 = z2_8h(vqtbx4q_u8(z2_16b(v19), v0v1v2v3, v29)); // tbx v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y+1])

        let v20 = v15; // mov v20.16b, v15.16b  (left[0])

        let v21 = vshrq_n_s16::<6>(vreinterpretq_s16_u16(v16)); // sshr v21.8h, v16.8h, #6  (first base_x)
        let v22 = vshrq_n_s16::<6>(vreinterpretq_s16_u16(v17)); // sshr v22.8h, v17.8h, #6

        let v20 = z2_8h(vqtbx4q_u8(z2_16b(v20), v0v1v2v3, v30)); // tbx v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v30.16b  (left[base_y+2])

        let v5 = vextq_u16::<1>(v4, v4v5.1); // ext v5.16b, v4.16b, v5.16b, #2  (top[base_x+1])
        let v7 = vextq_u16::<1>(v6, v6v7.1); // ext v7.16b, v6.16b, v7.16b, #2

        let v16 = vandq_u16(v16, v25); // and v16.16b, v16.16b, v25.16b  (frac_x)
        let v17 = vandq_u16(v17, v25); // and v17.16b, v17.16b, v25.16b

        let v10s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v10.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
        let v10s = vmlal_u16(v10s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v10.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)

        let v8 = vsubq_u16(v26, v16); // sub v8.8h, v26.8h, v16.8h  (64 - frac_x)
        let v9 = vsubq_u16(v26, v17); // sub v9.8h, v26.8h, v17.8h

        let v11s = vmull_high_u16(v18, v28); // umull2 v11.4s, v18.8h, v28.8h
        let v11s = vmlal_high_u16(v11s, v19, v27); // umlal2 v11.4s, v19.8h, v27.8h

        let v21 = vaddq_s16(v21, vreinterpretq_s16_u16(v31)); // add v21.8h, v21.8h, v31.8h  (actual base_x)
        let v22 = vaddq_s16(v22, vreinterpretq_s16_u16(v31)); // add v22.8h, v22.8h, v31.8h

        let v12s = vmull_u16(vget_low_u16(v19), vget_low_u16(v28)); // umull v12.4s, v19.4h, v28.4h
        let v12s = vmlal_u16(v12s, vget_low_u16(v20), vget_low_u16(v27)); // umlal v12.4s, v20.4h, v27.4h
        let v13s = vmull_high_u16(v19, v28); // umull2 v13.4s, v19.8h, v28.8h
        let v13s = vmlal_high_u16(v13s, v20, v27); // umlal2 v13.4s, v20.8h, v27.8h

        let v10 = vrshrn_n_u32::<6>(v10s); // rshrn v10.4h, v10.4s, #6
        let v10 = vrshrn_high_n_u32::<6>(v10, v11s); // rshrn2 v10.8h, v11.4s, #6
        let v11 = vrshrn_n_u32::<6>(v12s); // rshrn v11.4h, v12.4s, #6
        let v11 = vrshrn_high_n_u32::<6>(v11, v13s); // rshrn2 v11.8h, v13.4s, #6

        let v12s = vmull_u16(vget_low_u16(v4), vget_low_u16(v8)); // umull v12.4s, v4.4h, v8.4h  (top[base_x]*(64-frac_x))
        let v12s = vmlal_u16(v12s, vget_low_u16(v5), vget_low_u16(v16)); // umlal v12.4s, v5.4h, v16.4h  (+ top[base_x+1]*frac_x)
        let v13s = vmull_high_u16(v4, v8); // umull2 v13.4s, v4.8h, v8.8h
        let v13s = vmlal_high_u16(v13s, v5, v16); // umlal2 v13.4s, v5.8h, v16.8h
        let v14s = vmull_u16(vget_low_u16(v6), vget_low_u16(v9)); // umull v14.4s, v6.4h, v9.4h
        let v14s = vmlal_u16(v14s, vget_low_u16(v7), vget_low_u16(v17)); // umlal v14.4s, v7.4h, v17.4h
        let v18s = vmull_high_u16(v6, v9); // umull2 v18.4s, v6.8h, v9.8h
        let v18s = vmlal_high_u16(v18s, v7, v17); // umlal2 v18.4s, v7.8h, v17.8h

        let v21 = vcgezq_s16(v21); // cmge v21.8h, v21.8h, #0
        let v22 = vcgezq_s16(v22); // cmge v22.8h, v22.8h, #0

        let v12 = vrshrn_n_u32::<6>(v12s); // rshrn v12.4h, v12.4s, #6
        let v12 = vrshrn_high_n_u32::<6>(v12, v13s); // rshrn2 v12.8h, v13.4s, #6
        let v13 = vrshrn_n_u32::<6>(v14s); // rshrn v13.4h, v14.4s, #6
        let v13 = vrshrn_high_n_u32::<6>(v13, v18s); // rshrn2 v13.8h, v18.4s, #6

        let v10 = vbslq_u16(v21, v12, v10); // bit v10.16b, v12.16b, v21.16b
        let v11 = vbslq_u16(v22, v13, v11); // bit v11.16b, v13.16b, v22.16b

        vst1q_u16(x0, v10); // st1 {v10.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        vst1q_u16(x0, v11); // st1 {v11.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = v20; // mov v18.16b, v20.16b
        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 2 (*2))
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b  (base_y += 2 (*2))
    }

    loop {
        // 89:
        let v19 = v15; // mov v19.16b, v15.16b
        let v20 = v15; // mov v20.16b, v15.16b
        let v19 = z2_8h(vqtbx4q_u8(z2_16b(v19), v0v1v2v3, v29)); // tbx v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y+1])
        let v20 = z2_8h(vqtbx4q_u8(z2_16b(v20), v0v1v2v3, v30)); // tbx v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v30.16b  (left[base_y+2])

        let v4s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v4.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
        let v4s = vmlal_u16(v4s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v4.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
        let v5s = vmull_high_u16(v18, v28); // umull2 v5.4s, v18.8h, v28.8h
        let v5s = vmlal_high_u16(v5s, v19, v27); // umlal2 v5.4s, v19.8h, v27.8h
        let v6s = vmull_u16(vget_low_u16(v19), vget_low_u16(v28)); // umull v6.4s, v19.4h, v28.4h
        let v6s = vmlal_u16(v6s, vget_low_u16(v20), vget_low_u16(v27)); // umlal v6.4s, v20.4h, v27.4h
        let v7s = vmull_high_u16(v19, v28); // umull2 v7.4s, v19.8h, v28.8h
        let v7s = vmlal_high_u16(v7s, v20, v27); // umlal2 v7.4s, v20.8h, v27.8h

        let v4 = vrshrn_n_u32::<6>(v4s); // rshrn v4.4h, v4.4s, #6
        let v4 = vrshrn_high_n_u32::<6>(v4, v5s); // rshrn2 v4.8h, v5.4s, #6
        let v5 = vrshrn_n_u32::<6>(v6s); // rshrn v5.4h, v6.4s, #6
        let v5 = vrshrn_high_n_u32::<6>(v5, v7s); // rshrn2 v5.8h, v7.4s, #6

        vst1q_u16(x0, v4); // st1 {v4.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        vst1q_u16(x0, v5); // st1 {v5.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = v20; // mov v18.16b, v20.16b
        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 2 (*2))
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b  (base_y += 2 (*2))
    }
}

/// `ipred_z2_fill1_16bpc_neon`, labels `160:`/`320:`/`640:` (widths 16, 32
/// and 64). The saves of `d8`-`d15` are the compiler's.
///
/// # Safety
/// As for `dav1d_ipred_z2_fill1_16bpc_neon` with `width` 16, 32 or 64; `w7`
/// is `-dy`, `v31` is `increments`.
#[inline(never)]
unsafe fn ipred_z2_fill1_w16(
    mut x0: *mut u16,
    mut x1: isize,
    x2: *const u16,
    x3: *const u16,
    mut w4: c_int,
    mut w5: c_int,
    w6: c_int,
    w7: c_int,
    v31: uint16x8_t,
) {
    let mut w8 = (1 << 6) - w6; // mov w8, #(1 << 6); sub w8, w8, w6  (xpos = (1 << 6) - dx)

    let v25 = vdupq_n_u16(w7 as u16); // dup v25.8h, w7  (-dy)
    let x3 = x3.add(1); // add x3, x3, #2  (Skip past left[0])

    let mut x13 = x0.wrapping_byte_offset(x1); // add x13, x0, x1  (alternating row)
    x1 <<= 1; // lsl x1, x1, #1  (stride *= 2)
    x1 = x1.wrapping_sub((w4 as u32 as isize) << 1); // sub x1, x1, w4, uxtw #1  (stride -= width)

    let v11 = vdupq_n_u16(8); // movi v11.8h, #8
    let v26 = vmulq_u16(v31, v25); // mul v26.8h, v31.8h, v25.8h  ({0,1,2,3,4,5,6,7} * -dy)
    let mut v26 = vaddq_u16(v26, v25); // add v26.8h, v26.8h, v25.8h  (-= dy)
    let v25 = vmulq_u16(v25, v11); // mul v25.8h, v25.8h, v11.8h  (-8*dy)

    // Worst case height is 64, but we can only fit 32 pixels into
    // v0-v3 usable within one tbx instruction. As long as base_y is
    // up to 32, we use tbx.
    let v0v1v2v3 = vld1q_u8_x4(x3 as *const u8); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x3]  (left[])
    let v15 = vld1q_dup_u16(x2); // ld1r {v15.8h}, [x2]  (left[0] == top[0])

    let mut w12 = w4; // mov w12, w4  (orig w)
    let w14 = -w4; // neg w14, w4  (-w)

    let mut v23;
    loop {
        // 1:
        v23 = v26; // mov v23.16b, v26.16b  (reset ypos)

        let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
        let v16 = vdupq_n_u16(w8 as u16); // dup v16.8h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
        if w9 <= w14 {
            break; // cmp w9, w14; b.le 169f  (base_x <= -w)
        }

        let v17 = vdupq_n_u16(w8 as u16); // dup v17.8h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)

        let mut x9 = x2.wrapping_offset(w9 as isize); // add x9, x2, w9, sxtw #1
        let mut x11 = x2.wrapping_offset(w11 as isize); // add x11, x2, w11, sxtw #1

        let v21 = vshrq_n_s16::<6>(vreinterpretq_s16_u16(v16)); // sshr v21.8h, v16.8h, #6  (first base_x)
        let v22 = vshrq_n_s16::<6>(vreinterpretq_s16_u16(v17)); // sshr v22.8h, v17.8h, #6

        let mut v4 = vld1q_u16(x9); // ld1 {v4.8h}, [x9], #16  (top[base_x])
        x9 = x9.wrapping_add(8);
        let mut v6 = vld1q_u16(x11); // ld1 {v6.8h}, [x11], #16
        x11 = x11.wrapping_add(8);

        let v10 = vdupq_n_u16(0x3e); // movi v10.8h, #0x3e
        let v11 = vdupq_n_u16(64); // movi v11.8h, #64

        let v16 = vandq_u16(v16, v10); // and v16.16b, v16.16b, v10.16b  (frac_x)
        let v17 = vandq_u16(v17, v10); // and v17.16b, v17.16b, v10.16b

        let v8 = vsubq_u16(v11, v16); // sub v8.8h, v11.8h, v16.8h  (64 - frac_x)
        let v9 = vsubq_u16(v11, v17); // sub v9.8h, v11.8h, v17.8h

        // (v16, v17, v8 and v9 hold one value in every lane. The products with
        // them below are written by element, `vN.h[0]` at `2:` and `vN.h[1]`
        // at `4:`: the same values and one instruction each, as the
        // assembly's by-vector form. Written by vector, the compiler widens
        // the four vectors once for both loops and multiplies with `ushll` +
        // `mul`/`mla`, with spills.)

        let mut v21 = vaddq_s16(v21, vreinterpretq_s16_u16(v31)); // add v21.8h, v21.8h, v31.8h  (actual base_x)
        let mut v22 = vaddq_s16(v22, vreinterpretq_s16_u16(v31)); // add v22.8h, v22.8h, v31.8h

        'l2: loop {
            // 2:
            let w10 = vgetq_lane_s16::<0>(v22) as c_int; // smov w10, v22.h[0]

            let v29 = vcombine_u8(vshrn_n_u16::<6>(v23), vdup_n_u8(0)); // shrn v29.8b, v23.8h, #6  (ypos >> 6)
            let v12 = vdupq_n_u16(64); // movi v12.8h, #64
            let x10 = z2_smov_b::<0>(v29); // smov w10, v29.b[0]  (base_y[0])
            let v10 = vdupq_n_u16(0x3e); // movi v10.8h, #0x3e

            if w10 >= 0 {
                // cmp w10, #0; b.ge 4f  (base_x (bottom left) >= 0)
                loop {
                    // 4: The rest of the row only predicted from top[]
                    let v5 = vld1q_u16(x9); // ld1 {v5.8h}, [x9], #16  (top[base_x])
                    x9 = x9.wrapping_add(8);
                    let v7 = vld1q_u16(x11); // ld1 {v7.8h}, [x11], #16
                    x11 = x11.wrapping_add(8);

                    let v18 = vextq_u16::<1>(v4, v5); // ext v18.16b, v4.16b, v5.16b, #2  (top[base_x+1])
                    let v19 = vextq_u16::<1>(v6, v7); // ext v19.16b, v6.16b, v7.16b, #2

                    let v12s = vmull_laneq_u16::<1>(vget_low_u16(v4), v8); // umull v12.4s, v4.4h, v8.4h  (top[base_x]*(64-frac_x))
                    let v12s = vmlal_laneq_u16::<1>(v12s, vget_low_u16(v18), v16); // umlal v12.4s, v18.4h, v16.4h  (+ top[base_x+1]*frac_x)
                    let v13s = vmull_high_laneq_u16::<1>(v4, v8); // umull2 v13.4s, v4.8h, v8.8h
                    let v13s = vmlal_high_laneq_u16::<1>(v13s, v18, v16); // umlal2 v13.4s, v18.8h, v16.8h
                    let v14s = vmull_laneq_u16::<1>(vget_low_u16(v6), v9); // umull v14.4s, v6.4h, v9.4h
                    let v14s = vmlal_laneq_u16::<1>(v14s, vget_low_u16(v19), v17); // umlal v14.4s, v19.4h, v17.4h
                    let v20s = vmull_high_laneq_u16::<1>(v6, v9); // umull2 v20.4s, v6.8h, v9.8h
                    let v20s = vmlal_high_laneq_u16::<1>(v20s, v19, v17); // umlal2 v20.4s, v19.8h, v17.8h

                    let v12 = vrshrn_n_u32::<6>(v12s); // rshrn v12.4h, v12.4s, #6
                    let v12 = vrshrn_high_n_u32::<6>(v12, v13s); // rshrn2 v12.8h, v13.4s, #6
                    let v13 = vrshrn_n_u32::<6>(v14s); // rshrn v13.4h, v14.4s, #6
                    let v13 = vrshrn_high_n_u32::<6>(v13, v20s); // rshrn2 v13.8h, v20.4s, #6

                    vst1q_u16(x0, v12); // st1 {v12.8h}, [x0], #16
                    x0 = x0.wrapping_add(8);
                    w4 -= 8; // subs w4, w4, #8
                    vst1q_u16(x13, v13); // st1 {v13.8h}, [x13], #16
                    x13 = x13.wrapping_add(8);
                    if w4 <= 0 {
                        break 'l2; // b.le 3b
                    }

                    v4 = v5; // mov v4.16b, v5.16b
                    v6 = v7; // mov v6.16b, v7.16b
                }
            }
            let v27 = vandq_u16(v23, v10); // and v27.16b, v23.16b, v10.16b  (frac_y)

            let v18 = v15; // mov v18.16b, v15.16b  (left[0])
            let v28 = vsubq_u16(v12, v27); // sub v28.8h, v12.8h, v27.8h  (64 - frac_y)

            let v18v19v20 = if x10 <= 32 - 3 {
                // cmp w10, #(32-3); b.gt 22f
                // 21:
                // base_y < 32, using tbx
                let v29 = vshl_n_u8::<1>(vget_low_u8(v29)); // shl v29.8b, v29.8b, #1  (2*base_y)
                let v11 = z2_16b(vdupq_n_u16(1 << 8)); // movi v11.8h, #1, lsl #8
                let v29 = vcombine_u8(v29, vdup_n_u8(0));
                let v29 = vzip1q_u8(v29, v29); // zip1 v29.16b, v29.16b, v29.16b  (duplicate elements)
                let v29 = vaddq_u8(v29, v11); // add v29.16b, v29.16b, v11.16b  (2*base, 2*base+1, ...)

                let v13 = vdupq_n_u8(2); // movi v13.16b, #2

                let v18 = z2_8h(vqtbx4q_u8(z2_16b(v18), v0v1v2v3, v29)); // tbx v18.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y])

                let v29 = vaddq_u8(v29, v13); // add v29.16b, v29.16b, v13.16b  (base_y + 1 (*2))
                let v19 = v15; // mov v19.16b, v15.16b  (left[0])

                let v19 = z2_8h(vqtbx4q_u8(z2_16b(v19), v0v1v2v3, v29)); // tbx v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y+1])

                let v29 = vaddq_u8(v29, v13); // add v29.16b, v29.16b, v13.16b  (base_y + 2 (*2))
                let v20 = v15; // mov v20.16b, v15.16b  (left[0])

                let v20 = z2_8h(vqtbx4q_u8(z2_16b(v20), v0v1v2v3, v29)); // tbx v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y+2])

                uint16x8x3_t(v18, v19, v20) // b 23f
            } else {
                // 22:
                // base_y >= 32, using separate loads.
                // (Every lane of v19 and v20 is loaded; what they hold before
                // has no counterpart in the assembly, which loads into the
                // registers as they are.)
                let t = uint16x8x3_t(v18, v15, v15);
                let t = vld3q_lane_u16::<0>(x3.wrapping_offset(x10), t); // add x10, x3, w10, sxtw #1; ld3 {v18.h, v19.h, v20.h}[0], [x10]
                let x15 = x3.wrapping_offset(z2_smov_b::<1>(v29)); // smov w15, v29.b[1]; add x15, x3, w15, sxtw #1
                let t = vld3q_lane_u16::<1>(x15, t); // ld3 {v18.h, v19.h, v20.h}[1], [x15]
                let x16 = x3.wrapping_offset(z2_smov_b::<2>(v29)); // smov w16, v29.b[2]; add x16, x3, w16, sxtw #1
                let t = vld3q_lane_u16::<2>(x16, t); // ld3 {v18.h, v19.h, v20.h}[2], [x16]
                let x17 = x3.wrapping_offset(z2_smov_b::<3>(v29)); // smov w17, v29.b[3]; add x17, x3, w17, sxtw #1
                let t = vld3q_lane_u16::<3>(x17, t); // ld3 {v18.h, v19.h, v20.h}[3], [x17]
                let x10 = x3.wrapping_offset(z2_smov_b::<4>(v29)); // smov w10, v29.b[4]; add x10, x3, w10, sxtw #1
                let t = vld3q_lane_u16::<4>(x10, t); // ld3 {v18.h, v19.h, v20.h}[4], [x10]
                let x15 = x3.wrapping_offset(z2_smov_b::<5>(v29)); // smov w15, v29.b[5]; add x15, x3, w15, sxtw #1
                let t = vld3q_lane_u16::<5>(x15, t); // ld3 {v18.h, v19.h, v20.h}[5], [x15]
                let x16 = x3.wrapping_offset(z2_smov_b::<6>(v29)); // smov w16, v29.b[6]; add x16, x3, w16, sxtw #1
                let t = vld3q_lane_u16::<6>(x16, t); // ld3 {v18.h, v19.h, v20.h}[6], [x16]
                let x17 = x3.wrapping_offset(z2_smov_b::<7>(v29)); // smov w17, v29.b[7]; add x17, x3, w17, sxtw #1
                vld3q_lane_u16::<7>(x17, t) // ld3 {v18.h, v19.h, v20.h}[7], [x17]
            };
            let (v18, v19, v20) = (v18v19v20.0, v18v19v20.1, v18v19v20.2);

            // 23:
            let v5 = vld1q_u16(x9); // ld1 {v5.8h}, [x9], #16  (top[base_x])
            x9 = x9.wrapping_add(8);
            let v7 = vld1q_u16(x11); // ld1 {v7.8h}, [x11], #16
            x11 = x11.wrapping_add(8);

            v23 = vaddq_u16(v23, v25); // add v23.8h, v23.8h, v25.8h  (ypos -= 8*dy)

            let v10s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v10.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
            let v10s = vmlal_u16(v10s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v10.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
            let v11s = vmull_high_u16(v18, v28); // umull2 v11.4s, v18.8h, v28.8h
            let v11s = vmlal_high_u16(v11s, v19, v27); // umlal2 v11.4s, v19.8h, v27.8h
            let v12s = vmull_u16(vget_low_u16(v19), vget_low_u16(v28)); // umull v12.4s, v19.4h, v28.4h
            let v12s = vmlal_u16(v12s, vget_low_u16(v20), vget_low_u16(v27)); // umlal v12.4s, v20.4h, v27.4h
            let v13s = vmull_high_u16(v19, v28); // umull2 v13.4s, v19.8h, v28.8h
            let v13s = vmlal_high_u16(v13s, v20, v27); // umlal2 v13.4s, v20.8h, v27.8h

            let v18 = vextq_u16::<1>(v4, v5); // ext v18.16b, v4.16b, v5.16b, #2  (top[base_x+1])
            let v19 = vextq_u16::<1>(v6, v7); // ext v19.16b, v6.16b, v7.16b, #2

            let v10 = vrshrn_n_u32::<6>(v10s); // rshrn v10.4h, v10.4s, #6
            let v10 = vrshrn_high_n_u32::<6>(v10, v11s); // rshrn2 v10.8h, v11.4s, #6
            let v11 = vrshrn_n_u32::<6>(v12s); // rshrn v11.4h, v12.4s, #6
            let v11 = vrshrn_high_n_u32::<6>(v11, v13s); // rshrn2 v11.8h, v13.4s, #6

            let v12s = vmull_laneq_u16::<0>(vget_low_u16(v4), v8); // umull v12.4s, v4.4h, v8.4h  (top[base_x]*(64-frac_x))
            let v12s = vmlal_laneq_u16::<0>(v12s, vget_low_u16(v18), v16); // umlal v12.4s, v18.4h, v16.4h  (+ top[base_x+1]*frac_x)
            let v13s = vmull_high_laneq_u16::<0>(v4, v8); // umull2 v13.4s, v4.8h, v8.8h
            let v13s = vmlal_high_laneq_u16::<0>(v13s, v18, v16); // umlal2 v13.4s, v18.8h, v16.8h
            let v14s = vmull_laneq_u16::<0>(vget_low_u16(v6), v9); // umull v14.4s, v6.4h, v9.4h
            let v14s = vmlal_laneq_u16::<0>(v14s, vget_low_u16(v19), v17); // umlal v14.4s, v19.4h, v17.4h
            let v20s = vmull_high_laneq_u16::<0>(v6, v9); // umull2 v20.4s, v6.8h, v9.8h
            let v20s = vmlal_high_laneq_u16::<0>(v20s, v19, v17); // umlal2 v20.4s, v19.8h, v17.8h

            let v18 = vcgezq_s16(v21); // cmge v18.8h, v21.8h, #0
            let v19 = vcgezq_s16(v22); // cmge v19.8h, v22.8h, #0

            let v12 = vrshrn_n_u32::<6>(v12s); // rshrn v12.4h, v12.4s, #6
            let v12 = vrshrn_high_n_u32::<6>(v12, v13s); // rshrn2 v12.8h, v13.4s, #6
            let v13 = vrshrn_n_u32::<6>(v14s); // rshrn v13.4h, v14.4s, #6
            let v13 = vrshrn_high_n_u32::<6>(v13, v20s); // rshrn2 v13.8h, v20.4s, #6

            let v10 = vbslq_u16(v18, v12, v10); // bit v10.16b, v12.16b, v18.16b
            let v11 = vbslq_u16(v19, v13, v11); // bit v11.16b, v13.16b, v19.16b

            vst1q_u16(x0, v10); // st1 {v10.8h}, [x0], #16
            x0 = x0.wrapping_add(8);
            w4 -= 8; // subs w4, w4, #8
            vst1q_u16(x13, v11); // st1 {v11.8h}, [x13], #16
            x13 = x13.wrapping_add(8);
            if w4 <= 0 {
                break; // b.le 3f
            }

            let v10 = vdupq_n_s16(8); // movi v10.8h, #8
            v4 = v5; // mov v4.16b, v5.16b
            v6 = v7; // mov v6.16b, v7.16b
            v21 = vaddq_s16(v21, v10); // add v21.8h, v21.8h, v10.8h  (base_x += 8)
            v22 = vaddq_s16(v22, v10); // add v22.8h, v22.8h, v10.8h
        }

        // 3:
        w5 -= 2; // subs w5, w5, #2
        if w5 <= 0 {
            return; // b.le 9f
        }
        let v10 = vdupq_n_u16(128); // movi v10.8h, #128
        x0 = x0.wrapping_byte_offset(x1); // add x0, x0, x1
        x13 = x13.wrapping_byte_offset(x1); // add x13, x13, x1
        w4 = w12; // mov w4, w12  (reset w)
        v26 = vaddq_u16(v26, v10); // add v26.8h, v26.8h, v10.8h  (ypos += 2*(1<<6))
    }

    // 169: The rest of the block only predicted from left[]
    x1 = x1.wrapping_add((w4 as u32 as isize) << 1); // add x1, x1, w4, uxtw #1  (restore stride)
    w12 = w5; // mov w12, w5  (orig remaining h)
    loop {
        // 1:
        let v12 = vdupq_n_u16(64); // movi v12.8h, #64
        let v10 = vdupq_n_u16(0x3e); // movi v10.8h, #0x3e

        let v29 = vcombine_u8(vshrn_n_u16::<6>(v23), vdup_n_u8(0)); // shrn v29.8b, v23.8h, #6  (ypos >> 6)
        let v27 = vandq_u16(v23, v10); // and v27.16b, v23.16b, v10.16b  (frac_y)

        let w10 = z2_smov_b::<0>(v29); // smov w10, v29.b[0]  (base_y[0])

        let v29 = vshl_n_u8::<1>(vget_low_u8(v29)); // shl v29.8b, v29.8b, #1  (2*base_y)
        let v11 = z2_16b(vdupq_n_u16(1 << 8)); // movi v11.8h, #1, lsl #8
        let v29 = vcombine_u8(v29, vdup_n_u8(0));
        let v29 = vzip1q_u8(v29, v29); // zip1 v29.16b, v29.16b, v29.16b  (duplicate elements)
        v23 = vaddq_u16(v23, v25); // add v23.8h, v23.8h, v25.8h  (ypos -= 8*dy)
        let mut v29 = vaddq_u8(v29, v11); // add v29.16b, v29.16b, v11.16b  (2*base, 2*base+1, ...)

        let mut v18 = v15; // mov v18.16b, v15.16b  (left[0])
        let v21 = vdupq_n_u8(2); // movi v21.16b, #2

        let v28 = vsubq_u16(v12, v27); // sub v28.8h, v12.8h, v27.8h  (64 - frac_y)

        'l4: {
            if w10 <= 32 - 1 {
                // cmp w10, #(32-1); b.gt 31f
                v18 = z2_8h(vqtbx4q_u8(z2_16b(v18), v0v1v2v3, v29)); // tbx v18.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y])
                v29 = vaddq_u8(v29, v21); // add v29.16b, v29.16b, v21.16b  (base_y + 1 (*2))

                loop {
                    // 2:
                    // base_y < 32, using tbx.
                    let w10 = z2_smov_b::<0>(v29); // smov w10, v29.b[0]  (base_y[0])
                    let v19 = v15; // mov v19.16b, v15.16b  (left[0])
                    if w10 > 64 - 4 {
                        break; // cmp w10, #(64-4); b.gt 32f
                    }
                    let v19 = z2_8h(vqtbx4q_u8(z2_16b(v19), v0v1v2v3, v29)); // tbx v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y+1])
                    v29 = vaddq_u8(v29, v21); // add v29.16b, v29.16b, v21.16b  (base_y + 2 (*2))
                    let v20 = v15; // mov v20.16b, v15.16b  (left[0])
                    let v20 = z2_8h(vqtbx4q_u8(z2_16b(v20), v0v1v2v3, v29)); // tbx v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y+2])
                    v29 = vaddq_u8(v29, v21); // add v29.16b, v29.16b, v21.16b  (next base_y)

                    let v10s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v10.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
                    let v10s = vmlal_u16(v10s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v10.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
                    let v11s = vmull_high_u16(v18, v28); // umull2 v11.4s, v18.8h, v28.8h
                    let v11s = vmlal_high_u16(v11s, v19, v27); // umlal2 v11.4s, v19.8h, v27.8h
                    let v12s = vmull_u16(vget_low_u16(v19), vget_low_u16(v28)); // umull v12.4s, v19.4h, v28.4h
                    let v12s = vmlal_u16(v12s, vget_low_u16(v20), vget_low_u16(v27)); // umlal v12.4s, v20.4h, v27.4h
                    let v13s = vmull_high_u16(v19, v28); // umull2 v13.4s, v19.8h, v28.8h
                    let v13s = vmlal_high_u16(v13s, v20, v27); // umlal2 v13.4s, v20.8h, v27.8h

                    let v10 = vrshrn_n_u32::<6>(v10s); // rshrn v10.4h, v10.4s, #6
                    let v10 = vrshrn_high_n_u32::<6>(v10, v11s); // rshrn2 v10.8h, v11.4s, #6
                    let v11 = vrshrn_n_u32::<6>(v12s); // rshrn v11.4h, v12.4s, #6
                    let v11 = vrshrn_high_n_u32::<6>(v11, v13s); // rshrn2 v11.8h, v13.4s, #6

                    vst1q_u16(x0, v10); // st1 {v10.8h}, [x0], x1
                    x0 = x0.wrapping_byte_offset(x1);
                    w5 -= 2; // subs w5, w5, #2
                    vst1q_u16(x13, v11); // st1 {v11.8h}, [x13], x1
                    x13 = x13.wrapping_byte_offset(x1);
                    if w5 <= 0 {
                        break 'l4; // b.le 4f
                    }
                    v18 = v20; // mov v18.16b, v20.16b
                }
            } else {
                // 31: base_y >= 32, using separate loads, loading v18 if we had to bail
                // in the prologue.
                let x10 = x3.wrapping_byte_offset(z2_smov_b::<0>(v29)); // smov w10, v29.b[0]; add x10, x3, w10, sxtw
                v18 = vld1q_lane_u16::<0>(x10, v18); // ld1 {v18.h}[0], [x10]
                let x15 = x3.wrapping_byte_offset(z2_smov_b::<2>(v29)); // smov w15, v29.b[2]; add x15, x3, w15, sxtw
                v18 = vld1q_lane_u16::<1>(x15, v18); // ld1 {v18.h}[1], [x15]
                let x16 = x3.wrapping_byte_offset(z2_smov_b::<4>(v29)); // smov w16, v29.b[4]; add x16, x3, w16, sxtw
                v18 = vld1q_lane_u16::<2>(x16, v18); // ld1 {v18.h}[2], [x16]
                let x17 = x3.wrapping_byte_offset(z2_smov_b::<6>(v29)); // smov w17, v29.b[6]; add x17, x3, w17, sxtw
                v18 = vld1q_lane_u16::<3>(x17, v18); // ld1 {v18.h}[3], [x17]
                let x10 = x3.wrapping_byte_offset(z2_smov_b::<8>(v29)); // smov w10, v29.b[8]; add x10, x3, w10, sxtw
                v18 = vld1q_lane_u16::<4>(x10, v18); // ld1 {v18.h}[4], [x10]
                let x15 = x3.wrapping_byte_offset(z2_smov_b::<10>(v29)); // smov w15, v29.b[10]; add x15, x3, w15, sxtw
                v18 = vld1q_lane_u16::<5>(x15, v18); // ld1 {v18.h}[5], [x15]
                let x16 = x3.wrapping_byte_offset(z2_smov_b::<12>(v29)); // smov w16, v29.b[12]; add x16, x3, w16, sxtw
                let x17 = x3.wrapping_byte_offset(z2_smov_b::<14>(v29)); // smov w17, v29.b[14]; add x17, x3, w17, sxtw

                // (movi v21.16b, #2: v21 still holds it here)
                v29 = vaddq_u8(v29, v21); // add v29.16b, v29.16b, v21.16b  (next base_y)
                v18 = vld1q_lane_u16::<6>(x16, v18); // ld1 {v18.h}[6], [x16]
                v18 = vld1q_lane_u16::<7>(x17, v18); // ld1 {v18.h}[7], [x17]
            }

            // 32: base_y >= 32, using separate loads.
            if w5 >= 4 {
                // cmp w5, #4; b.lt 34f
                loop {
                    // 33: h >= 4, preserving v18 from the previous round, loading v19-v22.
                    // (Every lane of v19-v22 is loaded; what they hold before
                    // has no counterpart in the assembly.)
                    let t = uint16x8x4_t(v15, v15, v15, v15);
                    let x10 = x3.wrapping_byte_offset(z2_smov_b::<0>(v29)); // smov w10, v29.b[0]; add x10, x3, w10, sxtw
                    w5 -= 4; // subs w5, w5, #4
                    let t = vld4q_lane_u16::<0>(x10, t); // ld4 {v19.h, v20.h, v21.h, v22.h}[0], [x10]
                    let x15 = x3.wrapping_byte_offset(z2_smov_b::<2>(v29)); // smov w15, v29.b[2]; add x15, x3, w15, sxtw
                    let t = vld4q_lane_u16::<1>(x15, t); // ld4 {v19.h, v20.h, v21.h, v22.h}[1], [x15]
                    let x16 = x3.wrapping_byte_offset(z2_smov_b::<4>(v29)); // smov w16, v29.b[4]; add x16, x3, w16, sxtw
                    let t = vld4q_lane_u16::<2>(x16, t); // ld4 {v19.h, v20.h, v21.h, v22.h}[2], [x16]
                    let x17 = x3.wrapping_byte_offset(z2_smov_b::<6>(v29)); // smov w17, v29.b[6]; add x17, x3, w17, sxtw
                    let t = vld4q_lane_u16::<3>(x17, t); // ld4 {v19.h, v20.h, v21.h, v22.h}[3], [x17]
                    let x10 = x3.wrapping_byte_offset(z2_smov_b::<8>(v29)); // smov w10, v29.b[8]; add x10, x3, w10, sxtw
                    let t = vld4q_lane_u16::<4>(x10, t); // ld4 {v19.h, v20.h, v21.h, v22.h}[4], [x10]
                    let x15 = x3.wrapping_byte_offset(z2_smov_b::<10>(v29)); // smov w15, v29.b[10]; add x15, x3, w15, sxtw
                    let t = vld4q_lane_u16::<5>(x15, t); // ld4 {v19.h, v20.h, v21.h, v22.h}[5], [x15]
                    let x16 = x3.wrapping_byte_offset(z2_smov_b::<12>(v29)); // smov w16, v29.b[12]; add x16, x3, w16, sxtw
                    let t = vld4q_lane_u16::<6>(x16, t); // ld4 {v19.h, v20.h, v21.h, v22.h}[6], [x16]
                    let x17 = x3.wrapping_byte_offset(z2_smov_b::<14>(v29)); // smov w17, v29.b[14]; add x17, x3, w17, sxtw
                    let v10 = vdupq_n_u8(8); // movi v10.16b, #8
                    v29 = vaddq_u8(v29, v10); // add v29.16b, v29.16b, v10.16b  (next base_y)
                    let t = vld4q_lane_u16::<7>(x17, t); // ld4 {v19.h, v20.h, v21.h, v22.h}[7], [x17]
                    let (v19, v20, v21, v22) = (t.0, t.1, t.2, t.3);

                    let v10s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v10.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
                    let v10s = vmlal_u16(v10s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v10.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
                    let v11s = vmull_high_u16(v18, v28); // umull2 v11.4s, v18.8h, v28.8h
                    let v11s = vmlal_high_u16(v11s, v19, v27); // umlal2 v11.4s, v19.8h, v27.8h
                    let v12s = vmull_u16(vget_low_u16(v19), vget_low_u16(v28)); // umull v12.4s, v19.4h, v28.4h
                    let v12s = vmlal_u16(v12s, vget_low_u16(v20), vget_low_u16(v27)); // umlal v12.4s, v20.4h, v27.4h
                    let v13s = vmull_high_u16(v19, v28); // umull2 v13.4s, v19.8h, v28.8h
                    let v13s = vmlal_high_u16(v13s, v20, v27); // umlal2 v13.4s, v20.8h, v27.8h

                    let v10 = vrshrn_n_u32::<6>(v10s); // rshrn v10.4h, v10.4s, #6
                    let v10 = vrshrn_high_n_u32::<6>(v10, v11s); // rshrn2 v10.8h, v11.4s, #6
                    let v11 = vrshrn_n_u32::<6>(v12s); // rshrn v11.4h, v12.4s, #6
                    let v11 = vrshrn_high_n_u32::<6>(v11, v13s); // rshrn2 v11.8h, v13.4s, #6

                    let v12s = vmull_u16(vget_low_u16(v20), vget_low_u16(v28)); // umull v12.4s, v20.4h, v28.4h  (left[base_y]*(64-frac_y))
                    let v12s = vmlal_u16(v12s, vget_low_u16(v21), vget_low_u16(v27)); // umlal v12.4s, v21.4h, v27.4h  (+ left[base_y+1]*frac_y)
                    let v13s = vmull_high_u16(v20, v28); // umull2 v13.4s, v20.8h, v28.8h
                    let v13s = vmlal_high_u16(v13s, v21, v27); // umlal2 v13.4s, v21.8h, v27.8h
                    let v14s = vmull_u16(vget_low_u16(v21), vget_low_u16(v28)); // umull v14.4s, v21.4h, v28.4h
                    let v14s = vmlal_u16(v14s, vget_low_u16(v22), vget_low_u16(v27)); // umlal v14.4s, v22.4h, v27.4h
                    let v18s = vmull_high_u16(v21, v28); // umull2 v18.4s, v21.8h, v28.8h
                    let v18s = vmlal_high_u16(v18s, v22, v27); // umlal2 v18.4s, v22.8h, v27.8h

                    let v12 = vrshrn_n_u32::<6>(v12s); // rshrn v12.4h, v12.4s, #6
                    let v12 = vrshrn_high_n_u32::<6>(v12, v13s); // rshrn2 v12.8h, v13.4s, #6
                    let v13 = vrshrn_n_u32::<6>(v14s); // rshrn v13.4h, v14.4s, #6
                    let v13 = vrshrn_high_n_u32::<6>(v13, v18s); // rshrn2 v13.8h, v18.4s, #6

                    vst1q_u16(x0, v10); // st1 {v10.8h}, [x0], x1
                    x0 = x0.wrapping_byte_offset(x1);
                    vst1q_u16(x13, v11); // st1 {v11.8h}, [x13], x1
                    x13 = x13.wrapping_byte_offset(x1);
                    vst1q_u16(x0, v12); // st1 {v12.8h}, [x0], x1
                    x0 = x0.wrapping_byte_offset(x1);
                    vst1q_u16(x13, v13); // st1 {v13.8h}, [x13], x1
                    x13 = x13.wrapping_byte_offset(x1);
                    if w5 < 2 {
                        break 'l4; // cmp w5, #2; b.lt 4f
                    }
                    v18 = v22; // mov v18.16b, v22.16b
                    if w5 <= 2 {
                        break; // b.gt 33b
                    }
                }
            }

            // 34: h == 2, preserving v18 from the previous round, loading v19-v20.
            // (Every lane of v19 and v20 is loaded; what they hold before has
            // no counterpart in the assembly.)
            let t = uint16x8x2_t(v15, v15);
            let x10 = x3.wrapping_byte_offset(z2_smov_b::<0>(v29)); // smov w10, v29.b[0]; add x10, x3, w10, sxtw
            let t = vld2q_lane_u16::<0>(x10, t); // ld2 {v19.h, v20.h}[0], [x10]
            let x15 = x3.wrapping_byte_offset(z2_smov_b::<2>(v29)); // smov w15, v29.b[2]; add x15, x3, w15, sxtw
            let t = vld2q_lane_u16::<1>(x15, t); // ld2 {v19.h, v20.h}[1], [x15]
            let x16 = x3.wrapping_byte_offset(z2_smov_b::<4>(v29)); // smov w16, v29.b[4]; add x16, x3, w16, sxtw
            let t = vld2q_lane_u16::<2>(x16, t); // ld2 {v19.h, v20.h}[2], [x16]
            let x17 = x3.wrapping_byte_offset(z2_smov_b::<6>(v29)); // smov w17, v29.b[6]; add x17, x3, w17, sxtw
            let t = vld2q_lane_u16::<3>(x17, t); // ld2 {v19.h, v20.h}[3], [x17]
            let x10 = x3.wrapping_byte_offset(z2_smov_b::<8>(v29)); // smov w10, v29.b[8]; add x10, x3, w10, sxtw
            let t = vld2q_lane_u16::<4>(x10, t); // ld2 {v19.h, v20.h}[4], [x10]
            let x15 = x3.wrapping_byte_offset(z2_smov_b::<10>(v29)); // smov w15, v29.b[10]; add x15, x3, w15, sxtw
            let t = vld2q_lane_u16::<5>(x15, t); // ld2 {v19.h, v20.h}[5], [x15]
            let x16 = x3.wrapping_byte_offset(z2_smov_b::<12>(v29)); // smov w16, v29.b[12]; add x16, x3, w16, sxtw
            let t = vld2q_lane_u16::<6>(x16, t); // ld2 {v19.h, v20.h}[6], [x16]
            let x17 = x3.wrapping_byte_offset(z2_smov_b::<14>(v29)); // smov w17, v29.b[14]; add x17, x3, w17, sxtw

            // (movi v21.16b, #4; add v29.16b, v29.16b, v21.16b (next base_y)
            // have no counterpart: v29 is not read again before `1:` writes it)
            let t = vld2q_lane_u16::<7>(x17, t); // ld2 {v19.h, v20.h}[7], [x17]
            let (v19, v20) = (t.0, t.1);

            let v10s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v10.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
            let v10s = vmlal_u16(v10s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v10.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
            let v11s = vmull_high_u16(v18, v28); // umull2 v11.4s, v18.8h, v28.8h
            let v11s = vmlal_high_u16(v11s, v19, v27); // umlal2 v11.4s, v19.8h, v27.8h
            let v12s = vmull_u16(vget_low_u16(v19), vget_low_u16(v28)); // umull v12.4s, v19.4h, v28.4h
            let v12s = vmlal_u16(v12s, vget_low_u16(v20), vget_low_u16(v27)); // umlal v12.4s, v20.4h, v27.4h
            let v13s = vmull_high_u16(v19, v28); // umull2 v13.4s, v19.8h, v28.8h
            let v13s = vmlal_high_u16(v13s, v20, v27); // umlal2 v13.4s, v20.8h, v27.8h

            let v10 = vrshrn_n_u32::<6>(v10s); // rshrn v10.4h, v10.4s, #6
            let v10 = vrshrn_high_n_u32::<6>(v10, v11s); // rshrn2 v10.8h, v11.4s, #6
            let v11 = vrshrn_n_u32::<6>(v12s); // rshrn v11.4h, v12.4s, #6
            let v11 = vrshrn_high_n_u32::<6>(v11, v13s); // rshrn2 v11.8h, v13.4s, #6

            vst1q_u16(x0, v10); // st1 {v10.8h}, [x0], x1
            x0 = x0.wrapping_byte_offset(x1);
            vst1q_u16(x13, v11); // st1 {v11.8h}, [x13], x1
            x13 = x13.wrapping_byte_offset(x1);
            // The h==2 case only happens once at the end, if at all.
        }

        // 4:
        w4 -= 8; // subs w4, w4, #8
        if w4 <= 0 {
            return; // b.le 9f
        }

        x1 = ((x1 as usize) >> 1) as isize; // lsr x1, x1, #1
        let x12 = w12 as u32 as isize; // (mov w12, w5 cleared the upper half of x12)
        x0 = x0.wrapping_byte_offset(x1.wrapping_mul(x12).wrapping_neg()); // msub x0, x1, x12, x0  (ptr -= h * stride)
        x13 = x13.wrapping_byte_offset(x1.wrapping_mul(x12).wrapping_neg()); // msub x13, x1, x12, x13
        x1 <<= 1; // lsl x1, x1, #1
        x0 = x0.wrapping_add(8); // add x0, x0, #16
        x13 = x13.wrapping_add(8); // add x13, x13, #16
        w5 = w12; // mov w5, w12  (reset h)
    }
}

/// `void ipred_z2_fill1_16bpc_neon(pixel *dst, const ptrdiff_t stride,
/// const pixel *const top, const pixel *const left, const int width,
/// const int height, const int dx, const int dy);`
///
/// The entry point computes `xpos`, loads `increments`, negates `dy` and
/// dispatches on the width (the assembly's jump table `ipred_z2_fill1_tbl`).
/// The widths 4 and 8 get `xpos` in the place of the width; the code of the
/// widths 16, 32 and 64 needs the width and computes `xpos` itself.
///
/// # Safety
/// `width` is 4, 8, 16, 32 or 64 and `height` is even and at most 64 (16
/// for width 4, 32 for width 8); `stride` is in bytes. Writes `dst` rows
/// `0..height`, columns `0..width`. Reads `left[0..24]` (width 4) or
/// `left[1..33]` and `top[0]` (width 8 and more). The widths 16 and more
/// also load single pixels at `left[1 + base_y..]` (three, four or two in a
/// row, at `22:` and `31:`-`34:`) where the first column's `base_y` is
/// beyond what `tbx` reaches: from `left[-81]` (a column that is predicted
/// from `top`; the value is not used) to `left[64]`. For each pair of rows
/// that is not predicted from `left` alone it reads `top[base_x..base_x +
/// 8]` (width 4), `top[base_x..base_x + 16]` (width 8) or
/// `top[base_x..base_x + width + 8]` (width 16 and more), with `base_x =
/// ((1 << 6) - (y + 1) * dx) >> 6` of the pair's two rows `y`. A pair is
/// read from `top` while its first row's `base_x` is above -4 (width 4),
/// -16 (width 8) or `-width`; the second row's is up to `dx >> 6` lower.
/// Over the block sizes and the derivatives of `dav1d_dr_intra_derivative`
/// that is `top[-6..8]` (width 4), `top[-31..16]` (8), `top[-31..24]` (16),
/// `top[-34..40]` (32) and `top[-69..72]` (64). `ipred_z2_neon` passes
/// `top` and `left` inside one zero-initialized buffer that covers all of
/// it.
pub unsafe extern "C" fn dav1d_ipred_z2_fill1_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    top: *const u16,
    left: *const u16,
    width: c_int,
    height: c_int,
    dx: c_int,
    dy: c_int,
) {
    let w8 = (1 << 6) - dx; // mov w8, #(1 << 6); sub w8, w8, w6  (xpos = (1 << 6) - dx)
    let v31 = vld1q_u16(INCREMENTS.as_ptr()); // ldrel q31, x11, increments
    let w7 = dy.wrapping_neg(); // neg w7, w7  (-dy)

    // clz w10, w4; sub w10, w10, #25; ldrsw x10, [x9, w10, uxtw #2]; br x9
    match width {
        4 => ipred_z2_fill1_w4(dst, stride, top, left, w8, height, dx, w7, v31), // 40:
        8 => ipred_z2_fill1_w8(dst, stride, top, left, w8, height, dx, w7, v31), // 80:
        _ => ipred_z2_fill1_w16(dst, stride, top, left, width, height, dx, w7, v31), // 160:, 320:, 640:
    }
}

/// `ipred_z2_fill2_16bpc_neon`, label `40:` (width 4).
///
/// # Safety
/// As for `dav1d_ipred_z2_fill2_16bpc_neon` with `width == 4`; `w8` is the
/// entry's `xpos`, `w7` is `-dy`, `v31` is `increments`.
#[inline(always)]
unsafe fn ipred_z2_fill2_w4(
    mut x0: *mut u16,
    x1: isize,
    x2: *const u16,
    x3: *const u16,
    mut w8: c_int,
    mut w5: c_int,
    w6: c_int,
    w7: c_int,
    v31: uint16x8_t,
) {
    let v30 = vdup_n_u16(w7 as u16); // dup v30.4h, w7  (-dy)
    let v17 = vdup_n_u8(1); // movi v17.8b, #1

    let v16 = vmul_u16(vget_low_u16(v31), v30); // mul v16.4h, v31.4h, v30.4h  ({0,1,2,3} * -dy)
    let v25 = vdupq_n_u16(0x3e); // movi v25.8h, #0x3e
    let v30 = vadd_u16(v16, v30); // add v30.4h, v16.4h, v30.4h  (-= dy)

    // For upsample_top, w <= 8 and h <= 8; we may need up to h+1 elements
    // from left.
    let v0v1 = vld1q_u8_x2(x3 as *const u8); // ld1 {v0.8h, v1.8h}, [x3]  (left[])

    let v26 = vdupq_n_u16(64); // movi v26.8h, #64
    let v19 = vdup_n_u8(4); // movi v19.16b, #4

    let v29 = vshrn_n_u16::<6>(vcombine_u16(v30, vdup_n_u16(0))); // shrn v29.8b, v30.8h, #6  (ypos >> 6)
    let v27 = vand_u16(v30, vget_low_u16(v25)); // and v27.8b, v30.8b, v25.8b  (frac_y)

    let v29 = vadd_u8(v29, v17); // add v29.8b, v29.8b, v17.8b  (base_y = (ypos >> 6) + 1)

    let v23 = vreinterpret_u8_u16(vdup_n_u16(1 << 8)); // movi v23.4h, #1, lsl #8
    let v29 = vshl_n_u8::<1>(v29); // shl v29.8b, v29.8b, #1  (2*base_y)
    let v29 = vzip1_u8(v29, v29); // zip1 v29.8b, v29.8b, v29.8b  (duplicate elements)
    let v17 = vdup_n_u8(2); // movi v17.8b, #2
    let v29 = vadd_u8(v29, v23); // add v29.8b, v29.8b, v23.8b  (2*base, 2*base+1, ...)

    let v30 = vadd_u8(v29, v17); // add v30.8b, v29.8b, v17.8b  (base_y + 1 (*2))
    let v28 = vadd_u8(v29, v19); // add v28.8b, v29.8b, v19.8b  (base_y + 2 (*2))

    let v18 = vqtbl1_u8(v0v1.0, v29); // tbl v18.8b, {v0.16b}, v29.8b  (left[base_y])
    let mut v18 = z2_8h(vcombine_u8(v18, vdup_n_u8(0)));

    let mut v30 = vcombine_u8(v30, v28); // trn1 v30.2d, v30.2d, v28.2d  (base_y + 1, base_y + 2)

    let v28 = vsub_u16(vget_low_u16(v26), v27); // sub v28.4h, v26.4h, v27.4h  (64 - frac_y)

    let v31 = z2_trn1_2d(v31, v31); // trn1 v31.2d, v31.2d, v31.2d  ({0,1,2,3,0,1,2,3})

    let v27 = vcombine_u16(v27, v27); // trn1 v27.2d, v27.2d, v27.2d  (frac_y)
    let v28 = vcombine_u16(v28, v28); // trn1 v28.2d, v28.2d, v28.2d  (64 - frac_y)

    let v29 = vdupq_n_u8(4); // movi v29.16b, #4
    let v31 = vaddq_u16(v31, v31); // add v31.8h, v31.8h, v31.8h  ({0,2,4,6,0,2,4,6})
    loop {
        // 4:
        let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
        let v16 = vdup_n_u16(w8 as u16); // dup v16.4h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
        if w9 <= -8 {
            break; // cmp w9, #-8; b.le 49f  (base_x <= -8)
        }

        let w9 = w9 << 1; // lsl w9, w9, #1
        let w11 = w11 << 1; // lsl w11, w11, #1

        let v17 = vdup_n_u16(w8 as u16); // dup v17.4h, w8  (xpos)

        let v4 = vld1q_u16(x2.wrapping_byte_offset(w9 as isize)); // ldr q4, [x2, w9, sxtw]  (top[base_x])
        let v6 = vld1q_u16(x2.wrapping_byte_offset(w11 as isize)); // ldr q6, [x2, w11, sxtw]

        let v16 = vcombine_u16(v16, v17); // trn1 v16.2d, v16.2d, v17.2d  (xpos)

        let v19 = z2_8h(vqtbl2q_u8(v0v1, v30)); // tbl v19.16b, {v0.16b, v1.16b}, v30.16b  (left[base_y+1], left[base_y+2])

        let v20 = vshrq_n_s16::<6>(vreinterpretq_s16_u16(v16)); // sshr v20.8h, v16.8h, #6  (first base_x for each row)

        let v5 = vuzp2q_u16(v4, v6); // uzp2 v5.8h, v4.8h, v6.8h  (top[base_x+1])
        let v4 = vuzp1q_u16(v4, v6); // uzp1 v4.8h, v4.8h, v6.8h  (top[base_x])

        let v16 = vandq_u16(v16, v25); // and v16.16b, v16.16b, v25.16b  (frac_x)

        v18 = z2_trn1_2d(v18, v19); // trn1 v18.2d, v18.2d, v19.2d  (left[base_y], left[base_y+1])

        let v17 = vsubq_u16(v26, v16); // sub v17.8h, v26.8h, v16.8h  (64 - frac_x)

        let v20 = vaddq_s16(v20, vreinterpretq_s16_u16(v31)); // add v20.8h, v20.8h, v31.8h  (actual base_x)

        let v21s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v21.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
        let v21s = vmlal_u16(v21s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v21.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
        let v22s = vmull_high_u16(v18, v28); // umull2 v22.4s, v18.8h, v28.8h
        let v22s = vmlal_high_u16(v22s, v19, v27); // umlal2 v22.4s, v19.8h, v27.8h

        let v23s = vmull_u16(vget_low_u16(v4), vget_low_u16(v17)); // umull v23.4s, v4.4h, v17.4h  (top[base_x]*(64-frac_x))
        let v23s = vmlal_u16(v23s, vget_low_u16(v5), vget_low_u16(v16)); // umlal v23.4s, v5.4h, v16.4h  (+ top[base_x+1]*frac_x)
        let v24s = vmull_high_u16(v4, v17); // umull2 v24.4s, v4.8h, v17.8h
        let v24s = vmlal_high_u16(v24s, v5, v16); // umlal2 v24.4s, v5.8h, v16.8h

        let v20 = vcgezq_s16(v20); // cmge v20.8h, v20.8h, #0

        let v21 = vrshrn_n_u32::<6>(v21s); // rshrn v21.4h, v21.4s, #6
        let v21 = vrshrn_high_n_u32::<6>(v21, v22s); // rshrn2 v21.8h, v22.4s, #6
        let v22 = vrshrn_n_u32::<6>(v23s); // rshrn v22.4h, v23.4s, #6
        let v22 = vrshrn_high_n_u32::<6>(v22, v24s); // rshrn2 v22.8h, v24.4s, #6

        let v21 = vbslq_u16(v20, v22, v21); // bit v21.16b, v22.16b, v20.16b

        vst1_u16(x0, vget_low_u16(v21)); // st1 {v21.d}[0], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        w5 -= 2; // subs w5, w5, #2
        vst1_u16(x0, vget_high_u16(v21)); // st1 {v21.d}[1], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = vextq_u16::<4>(v19, v19); // ext v18.16b, v19.16b, v19.16b, #8
        v30 = vaddq_u8(v30, v29); // add v30.16b, v30.16b, v29.16b  (base_y += 2 (*2))
    }

    loop {
        // 49:
        let v19 = z2_8h(vqtbl2q_u8(v0v1, v30)); // tbl v19.16b, {v0.16b, v1.16b}, v30.16b  (left[base_y+1], left[base_y+2])

        v18 = z2_trn1_2d(v18, v19); // trn1 v18.2d, v18.2d, v19.2d  (left[base_y], left[base_y+1])

        let v20s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v20.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
        let v20s = vmlal_u16(v20s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v20.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
        let v21s = vmull_high_u16(v18, v28); // umull2 v21.4s, v18.8h, v28.8h
        let v21s = vmlal_high_u16(v21s, v19, v27); // umlal2 v21.4s, v19.8h, v27.8h

        let v20 = vrshrn_n_u32::<6>(v20s); // rshrn v20.4h, v20.4s, #6
        let v20 = vrshrn_high_n_u32::<6>(v20, v21s); // rshrn2 v20.8h, v21.4s, #6

        vst1_u16(x0, vget_low_u16(v20)); // st1 {v20.d}[0], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        vst1_u16(x0, vget_high_u16(v20)); // st1 {v20.d}[1], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = vextq_u16::<4>(v19, v19); // ext v18.16b, v19.16b, v19.16b, #8
        v30 = vaddq_u8(v30, v29); // add v30.16b, v30.16b, v29.16b  (base_y += 2 (*2))
    }
}

/// `ipred_z2_fill2_16bpc_neon`, label `80:` (width 8). The saves of
/// `d8`-`d15` are the compiler's.
///
/// # Safety
/// As for `dav1d_ipred_z2_fill2_16bpc_neon` with `width == 8`; `w8` is the
/// entry's `xpos`, `w7` is `-dy`, `v31` is `increments`.
#[inline(never)]
unsafe fn ipred_z2_fill2_w8(
    mut x0: *mut u16,
    x1: isize,
    x2: *const u16,
    x3: *const u16,
    mut w8: c_int,
    mut w5: c_int,
    w6: c_int,
    w7: c_int,
    v31: uint16x8_t,
) {
    let v18 = vdupq_n_u16(w7 as u16); // dup v18.8h, w7  (-dy)
    let v17 = vdup_n_u8(1); // movi v17.8b, #1

    let v16 = vmulq_u16(v31, v18); // mul v16.8h, v31.8h, v18.8h  ({0,1,2,3,4,5,6,7} * -dy)
    let v25 = vdupq_n_u16(0x3e); // movi v25.8h, #0x3e
    let v16 = vaddq_u16(v16, v18); // add v16.8h, v16.8h, v18.8h  (-= dy)

    // For upsample_top, w <= 8 and h <= 8; we may need up to h+1 elements
    // from left.
    let v0v1 = vld1q_u8_x2(x3 as *const u8); // ld1 {v0.8h, v1.8h}, [x3]  (left[])

    let v26 = vdupq_n_u16(64); // movi v26.8h, #64
    let v19 = vdupq_n_u8(4); // movi v19.16b, #4

    let v29 = vshrn_n_u16::<6>(v16); // shrn v29.8b, v16.8h, #6  (ypos >> 6)
    let v27 = vandq_u16(v16, v25); // and v27.16b, v16.16b, v25.16b  (frac_y)

    let v29 = vadd_u8(v29, v17); // add v29.8b, v29.8b, v17.8b  (base_y = (ypos >> 6) + 1)

    let v23 = z2_16b(vdupq_n_u16(1 << 8)); // movi v23.8h, #1, lsl #8
    let v29 = vshl_n_u8::<1>(v29); // shl v29.8b, v29.8b, #1  (2*base_y)
    let v29 = vcombine_u8(v29, vdup_n_u8(0));
    let v29 = vzip1q_u8(v29, v29); // zip1 v29.16b, v29.16b, v29.16b  (duplicate elements)
    let v17 = vdupq_n_u8(2); // movi v17.16b, #2
    let v29 = vaddq_u8(v29, v23); // add v29.16b, v29.16b, v23.16b  (2*base, 2*base+1, ...)

    // Cut corners here; for the first row we don't expect to need to
    // read outside of v0.
    let mut v18 = z2_8h(vqtbl1q_u8(v0v1.0, v29)); // tbl v18.16b, {v0.16b}, v29.16b  (left[base_y])

    let mut v30 = vaddq_u8(v29, v19); // add v30.16b, v29.16b, v19.16b  (base_y + 2 (*2))
    let mut v29 = vaddq_u8(v29, v17); // add v29.16b, v29.16b, v17.16b  (base_y + 1 (*2))

    let v28 = vsubq_u16(v26, v27); // sub v28.8h, v26.8h, v27.8h  (64 - frac_y)

    let v24 = vdupq_n_u8(4); // movi v24.16b, #4
    let v31 = z2_8h(vaddq_u8(z2_16b(v31), z2_16b(v31))); // add v31.16b, v31.16b, v31.16b  ({0,2,4,6,8,10,12,14})
    loop {
        // 8:
        let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
        let v16 = vdupq_n_u16(w8 as u16); // dup v16.8h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
        if w9 <= -16 {
            break; // cmp w9, #-16; b.le 89f  (base_x <= -16)
        }

        let v17 = vdupq_n_u16(w8 as u16); // dup v17.8h, w8  (xpos)

        let x9 = x2.wrapping_offset(w9 as isize); // add x9, x2, w9, sxtw #1
        let x11 = x2.wrapping_offset(w11 as isize); // add x11, x2, w11, sxtw #1

        let v4v5 = vld1q_u16_x2(x9); // ld1 {v4.8h, v5.8h}, [x9]  (top[base_x])
        let v6v7 = vld1q_u16_x2(x11); // ld1 {v6.8h, v7.8h}, [x11]

        let v19 = z2_8h(vqtbl2q_u8(v0v1, v29)); // tbl v19.16b, {v0.16b, v1.16b}, v29.16b  (left[base_y+1])

        let v21 = vshrq_n_s16::<6>(vreinterpretq_s16_u16(v16)); // sshr v21.8h, v16.8h, #6  (first base_x)
        let v22 = vshrq_n_s16::<6>(vreinterpretq_s16_u16(v17)); // sshr v22.8h, v17.8h, #6

        let v20 = z2_8h(vqtbl2q_u8(v0v1, v30)); // tbl v20.16b, {v0.16b, v1.16b}, v30.16b  (left[base_y+2])

        let v2 = vuzp2q_u16(v4v5.0, v4v5.1); // uzp2 v2.8h, v4.8h, v5.8h  (top[base_x+1])
        let v4 = vuzp1q_u16(v4v5.0, v4v5.1); // uzp1 v4.8h, v4.8h, v5.8h  (top[base_x])
        let v3 = vuzp2q_u16(v6v7.0, v6v7.1); // uzp2 v3.8h, v6.8h, v7.8h
        let v6 = vuzp1q_u16(v6v7.0, v6v7.1); // uzp1 v6.8h, v6.8h, v7.8h
        let v5 = v2; // mov v5.16b, v2.16b
        let v7 = v3; // mov v7.16b, v3.16b

        let v16 = vandq_u16(v16, v25); // and v16.16b, v16.16b, v25.16b  (frac_x)
        let v17 = vandq_u16(v17, v25); // and v17.16b, v17.16b, v25.16b

        let v10s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v10.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
        let v10s = vmlal_u16(v10s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v10.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)

        let v8 = vsubq_u16(v26, v16); // sub v8.8h, v26.8h, v16.8h  (64 - frac_x)
        let v9 = vsubq_u16(v26, v17); // sub v9.8h, v26.8h, v17.8h

        let v11s = vmull_high_u16(v18, v28); // umull2 v11.4s, v18.8h, v28.8h
        let v11s = vmlal_high_u16(v11s, v19, v27); // umlal2 v11.4s, v19.8h, v27.8h

        let v21 = vaddq_s16(v21, vreinterpretq_s16_u16(v31)); // add v21.8h, v21.8h, v31.8h  (actual base_x)
        let v22 = vaddq_s16(v22, vreinterpretq_s16_u16(v31)); // add v22.8h, v22.8h, v31.8h

        let v12s = vmull_u16(vget_low_u16(v19), vget_low_u16(v28)); // umull v12.4s, v19.4h, v28.4h
        let v12s = vmlal_u16(v12s, vget_low_u16(v20), vget_low_u16(v27)); // umlal v12.4s, v20.4h, v27.4h
        let v13s = vmull_high_u16(v19, v28); // umull2 v13.4s, v19.8h, v28.8h
        let v13s = vmlal_high_u16(v13s, v20, v27); // umlal2 v13.4s, v20.8h, v27.8h

        let v10 = vrshrn_n_u32::<6>(v10s); // rshrn v10.4h, v10.4s, #6
        let v10 = vrshrn_high_n_u32::<6>(v10, v11s); // rshrn2 v10.8h, v11.4s, #6
        let v11 = vrshrn_n_u32::<6>(v12s); // rshrn v11.4h, v12.4s, #6
        let v11 = vrshrn_high_n_u32::<6>(v11, v13s); // rshrn2 v11.8h, v13.4s, #6

        let v12s = vmull_u16(vget_low_u16(v4), vget_low_u16(v8)); // umull v12.4s, v4.4h, v8.4h  (top[base_x]*(64-frac_x))
        let v12s = vmlal_u16(v12s, vget_low_u16(v5), vget_low_u16(v16)); // umlal v12.4s, v5.4h, v16.4h  (+ top[base_x+1]*frac_x)
        let v13s = vmull_high_u16(v4, v8); // umull2 v13.4s, v4.8h, v8.8h
        let v13s = vmlal_high_u16(v13s, v5, v16); // umlal2 v13.4s, v5.8h, v16.8h
        let v14s = vmull_u16(vget_low_u16(v6), vget_low_u16(v9)); // umull v14.4s, v6.4h, v9.4h
        let v14s = vmlal_u16(v14s, vget_low_u16(v7), vget_low_u16(v17)); // umlal v14.4s, v7.4h, v17.4h
        let v18s = vmull_high_u16(v6, v9); // umull2 v18.4s, v6.8h, v9.8h
        let v18s = vmlal_high_u16(v18s, v7, v17); // umlal2 v18.4s, v7.8h, v17.8h

        let v21 = vcgezq_s16(v21); // cmge v21.8h, v21.8h, #0
        let v22 = vcgezq_s16(v22); // cmge v22.8h, v22.8h, #0

        let v12 = vrshrn_n_u32::<6>(v12s); // rshrn v12.4h, v12.4s, #6
        let v12 = vrshrn_high_n_u32::<6>(v12, v13s); // rshrn2 v12.8h, v13.4s, #6
        let v13 = vrshrn_n_u32::<6>(v14s); // rshrn v13.4h, v14.4s, #6
        let v13 = vrshrn_high_n_u32::<6>(v13, v18s); // rshrn2 v13.8h, v18.4s, #6

        let v10 = vbslq_u16(v21, v12, v10); // bit v10.16b, v12.16b, v21.16b
        let v11 = vbslq_u16(v22, v13, v11); // bit v11.16b, v13.16b, v22.16b

        vst1q_u16(x0, v10); // st1 {v10.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        vst1q_u16(x0, v11); // st1 {v11.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = v20; // mov v18.16b, v20.16b
        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 2 (*2))
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b  (base_y += 2 (*2))
    }

    loop {
        // 89:
        let v19 = z2_8h(vqtbl2q_u8(v0v1, v29)); // tbl v19.16b, {v0.16b, v1.16b}, v29.16b  (left[base_y+1])
        let v20 = z2_8h(vqtbl2q_u8(v0v1, v30)); // tbl v20.16b, {v0.16b, v1.16b}, v30.16b  (left[base_y+2])

        let v4s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v4.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
        let v4s = vmlal_u16(v4s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v4.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
        let v5s = vmull_high_u16(v18, v28); // umull2 v5.4s, v18.8h, v28.8h
        let v5s = vmlal_high_u16(v5s, v19, v27); // umlal2 v5.4s, v19.8h, v27.8h
        let v6s = vmull_u16(vget_low_u16(v19), vget_low_u16(v28)); // umull v6.4s, v19.4h, v28.4h
        let v6s = vmlal_u16(v6s, vget_low_u16(v20), vget_low_u16(v27)); // umlal v6.4s, v20.4h, v27.4h
        let v7s = vmull_high_u16(v19, v28); // umull2 v7.4s, v19.8h, v28.8h
        let v7s = vmlal_high_u16(v7s, v20, v27); // umlal2 v7.4s, v20.8h, v27.8h

        let v4 = vrshrn_n_u32::<6>(v4s); // rshrn v4.4h, v4.4s, #6
        let v4 = vrshrn_high_n_u32::<6>(v4, v5s); // rshrn2 v4.8h, v5.4s, #6
        let v5 = vrshrn_n_u32::<6>(v6s); // rshrn v5.4h, v6.4s, #6
        let v5 = vrshrn_high_n_u32::<6>(v5, v7s); // rshrn2 v5.8h, v7.4s, #6

        vst1q_u16(x0, v4); // st1 {v4.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        vst1q_u16(x0, v5); // st1 {v5.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = v20; // mov v18.16b, v20.16b
        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 2 (*2))
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b  (base_y += 2 (*2))
    }
}

/// `ipred_z2_fill2_16bpc_neon` (same parameters as `ipred_z2_fill1`): the
/// variant for an upsampled `top` (`top[2 * x]`, `dx` doubled by the caller).
///
/// # Safety
/// `width` and `height` are 4 or 8 (the caller upsamples only for `width +
/// height <= 16`); `stride` is in bytes. Writes `dst` rows `0..height`,
/// columns `0..width`. Reads `left[0..16]`, and for each pair of rows that
/// is not predicted from `left` alone `top[base_x..base_x + 8]` (width 4) or
/// `top[base_x..base_x + 16]` (width 8) with `base_x = ((2 << 6) - (y + 1)
/// * dx) >> 6` of the pair's two rows `y`.
pub unsafe extern "C" fn dav1d_ipred_z2_fill2_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    top: *const u16,
    left: *const u16,
    width: c_int,
    height: c_int,
    dx: c_int,
    dy: c_int,
) {
    let w8 = (2 << 6) - dx; // mov w8, #(2 << 6); sub w8, w8, w6  (xpos = (2 << 6) - dx)

    let v31 = vld1q_u16(INCREMENTS.as_ptr()); // ldrel q31, x11, increments
    let w7 = dy.wrapping_neg(); // neg w7, w7  (-dy)
    if width == 8 {
        // cmp w4, #8; b.eq 80f
        ipred_z2_fill2_w8(dst, stride, top, left, w8, height, dx, w7, v31) // 80:
    } else {
        ipred_z2_fill2_w4(dst, stride, top, left, w8, height, dx, w7, v31) // 40:
    }
}

/// `ipred_z2_fill3_16bpc_neon`, label `40:` (width 4).
///
/// # Safety
/// As for `dav1d_ipred_z2_fill3_16bpc_neon` with `width == 4`; `w8` is the
/// entry's `xpos`, `w7` is `-dy`, `v31` is `increments`.
#[inline(always)]
unsafe fn ipred_z2_fill3_w4(
    mut x0: *mut u16,
    x1: isize,
    x2: *const u16,
    x3: *const u16,
    mut w8: c_int,
    mut w5: c_int,
    w6: c_int,
    w7: c_int,
    v31: uint16x8_t,
) {
    let v30 = vdup_n_u16(w7 as u16); // dup v30.4h, w7  (-dy)

    // (movi v17.8b, #1: v17 is written again before it is read)

    let v16 = vmul_u16(vget_low_u16(v31), v30); // mul v16.4h, v31.4h, v30.4h  ({0,1,2,3} * -dy)
    let v25 = vdupq_n_u16(0x3e); // movi v25.8h, #0x3e
    let v30 = vadd_u16(v16, v30); // add v30.4h, v16.4h, v30.4h  (-= dy)

    // For upsample_left, w <= 8 and h <= 8; we may need up to 2*h+1 elements.
    let v0v1v2 = vld1q_u8_x3(x3 as *const u8); // ld1 {v0.8h, v1.8h, v2.8h}, [x3]  (left[])

    let v26 = vdupq_n_u16(64); // movi v26.8h, #64
    let v19 = vdup_n_u8(2); // movi v19.16b, #2

    let v29 = vshrn_n_u16::<6>(vcombine_u16(v30, vdup_n_u16(0))); // shrn v29.8b, v30.8h, #6  (ypos >> 6)
    let v27 = vand_u16(v30, vget_low_u16(v25)); // and v27.8b, v30.8b, v25.8b  (frac_y)

    let v29 = vadd_u8(v29, v19); // add v29.8b, v29.8b, v19.8b  (base_y = (ypos >> 6) + 2)

    let v23 = vreinterpret_u8_u16(vdup_n_u16(1 << 8)); // movi v23.4h, #1, lsl #8
    let v29 = vshl_n_u8::<1>(v29); // shl v29.8b, v29.8b, #1  (2*base_y)
    let v19 = vdup_n_u8(4); // movi v19.16b, #4
    let v29 = vzip1_u8(v29, v29); // zip1 v29.8b, v29.8b, v29.8b  (duplicate elements)
    let v17 = vdup_n_u8(2); // movi v17.8b, #2
    let v29 = vadd_u8(v29, v23); // add v29.8b, v29.8b, v23.8b  (2*base, 2*base+1, ...)

    let v30 = vadd_u8(v29, v17); // add v30.8b, v29.8b, v17.8b  (base_y + 1 (*2))
    let v28 = vadd_u8(v29, v19); // add v28.8b, v29.8b, v19.8b  (base_y + 2 (*2))

    let v31 = z2_trn1_2d(v31, v31); // trn1 v31.2d, v31.2d, v31.2d  ({0,1,2,3,0,1,2,3})

    let v24 = vadd_u8(v30, v19); // add v24.8b, v30.8b, v19.8b  (base_y + 3 (*2))

    let mut v29 = vcombine_u8(v29, v28); // trn1 v29.2d, v29.2d, v28.2d  (base_y + 0, base_y + 2)
    let mut v30 = vcombine_u8(v30, v24); // trn1 v30.2d, v30.2d, v24.2d  (base_y + 1, base_y + 3)

    let v28 = vsub_u16(vget_low_u16(v26), v27); // sub v28.4h, v26.4h, v27.4h  (64 - frac_y)

    let v27 = vcombine_u16(v27, v27); // trn1 v27.2d, v27.2d, v27.2d  (frac_y)
    let v28 = vcombine_u16(v28, v28); // trn1 v28.2d, v28.2d, v28.2d  (64 - frac_y)

    let v24 = vdupq_n_u8(8); // movi v24.16b, #8
    loop {
        // 4:
        let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
        let v16 = vdup_n_u16(w8 as u16); // dup v16.4h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
        if w9 <= -4 {
            break; // cmp w9, #-4; b.le 49f  (base_x <= -4)
        }

        let w9 = w9 << 1; // lsl w9, w9, #1
        let w11 = w11 << 1; // lsl w11, w11, #1

        let v17 = vdup_n_u16(w8 as u16); // dup v17.4h, w8  (xpos)

        let v4 = vld1q_u16(x2.wrapping_byte_offset(w9 as isize)); // ldr q4, [x2, w9, sxtw]  (top[base_x])
        let v6 = vld1q_u16(x2.wrapping_byte_offset(w11 as isize)); // ldr q6, [x2, w11, sxtw]

        let v16 = vcombine_u16(v16, v17); // trn1 v16.2d, v16.2d, v17.2d  (xpos)

        let v18 = z2_8h(vqtbl3q_u8(v0v1v2, v29)); // tbl v18.16b, {v0.16b, v1.16b, v2.16b}, v29.16b  (left[base_y+0], left[base_y+2])
        let v19 = z2_8h(vqtbl3q_u8(v0v1v2, v30)); // tbl v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b  (left[base_y+1], left[base_y+3])

        let v20 = vshrq_n_s16::<6>(vreinterpretq_s16_u16(v16)); // sshr v20.8h, v16.8h, #6  (first base_x for each row)

        let v5 = vextq_u16::<1>(v4, v4); // ext v5.16b, v4.16b, v4.16b, #2  (top[base_x+1])
        let v7 = vextq_u16::<1>(v6, v6); // ext v7.16b, v6.16b, v6.16b, #2

        let v16 = vandq_u16(v16, v25); // and v16.16b, v16.16b, v25.16b  (frac_x)

        let v4 = z2_trn1_2d(v4, v6); // trn1 v4.2d, v4.2d, v6.2d  (top[base_x])
        let v5 = z2_trn1_2d(v5, v7); // trn1 v5.2d, v5.2d, v7.2d  (top[base_x+1])

        let v17 = vsubq_u16(v26, v16); // sub v17.8h, v26.8h, v16.8h  (64 - frac_x)

        let v20 = vaddq_s16(v20, vreinterpretq_s16_u16(v31)); // add v20.8h, v20.8h, v31.8h  (actual base_x)

        let v21s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v21.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
        let v21s = vmlal_u16(v21s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v21.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
        let v22s = vmull_high_u16(v18, v28); // umull2 v22.4s, v18.8h, v28.8h
        let v22s = vmlal_high_u16(v22s, v19, v27); // umlal2 v22.4s, v19.8h, v27.8h

        let v23s = vmull_u16(vget_low_u16(v4), vget_low_u16(v17)); // umull v23.4s, v4.4h, v17.4h  (top[base_x]*(64-frac_x))
        let v23s = vmlal_u16(v23s, vget_low_u16(v5), vget_low_u16(v16)); // umlal v23.4s, v5.4h, v16.4h  (+ top[base_x+1]*frac_x)
        let v24s = vmull_high_u16(v4, v17); // umull2 v24.4s, v4.8h, v17.8h
        let v24s = vmlal_high_u16(v24s, v5, v16); // umlal2 v24.4s, v5.8h, v16.8h

        let v20 = vcgezq_s16(v20); // cmge v20.8h, v20.8h, #0

        let v21 = vrshrn_n_u32::<6>(v21s); // rshrn v21.4h, v21.4s, #6
        let v21 = vrshrn_high_n_u32::<6>(v21, v22s); // rshrn2 v21.8h, v22.4s, #6
        let v22 = vrshrn_n_u32::<6>(v23s); // rshrn v22.4h, v23.4s, #6
        let v22 = vrshrn_high_n_u32::<6>(v22, v24s); // rshrn2 v22.8h, v24.4s, #6

        // (movi v24.16b, #8: the local v24 still holds it; the assembly's
        // register was used for the products)

        let v21 = vbslq_u16(v20, v22, v21); // bit v21.16b, v22.16b, v20.16b

        vst1_u16(x0, vget_low_u16(v21)); // st1 {v21.d}[0], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        w5 -= 2; // subs w5, w5, #2
        vst1_u16(x0, vget_high_u16(v21)); // st1 {v21.d}[1], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 4 (*2))
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b  (base_y += 4 (*2))
    }

    loop {
        // 49:
        let v18 = z2_8h(vqtbl3q_u8(v0v1v2, v29)); // tbl v18.16b, {v0.16b, v1.16b, v2.16b}, v29.16b  (left[base_y+0], left[base_y+2])
        let v19 = z2_8h(vqtbl3q_u8(v0v1v2, v30)); // tbl v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b  (left[base_y+1], left[base_y+3])

        let v20s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v20.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
        let v20s = vmlal_u16(v20s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v20.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
        let v21s = vmull_high_u16(v18, v28); // umull2 v21.4s, v18.8h, v28.8h
        let v21s = vmlal_high_u16(v21s, v19, v27); // umlal2 v21.4s, v19.8h, v27.8h

        let v20 = vrshrn_n_u32::<6>(v20s); // rshrn v20.4h, v20.4s, #6
        let v20 = vrshrn_high_n_u32::<6>(v20, v21s); // rshrn2 v20.8h, v21.4s, #6

        vst1_u16(x0, vget_low_u16(v20)); // st1 {v20.d}[0], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        vst1_u16(x0, vget_high_u16(v20)); // st1 {v20.d}[1], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 4 (*2))
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b  (base_y += 4 (*2))
    }
}

/// `ipred_z2_fill3_16bpc_neon`, label `80:` (width 8). The saves of
/// `d8`-`d15` are the compiler's.
///
/// # Safety
/// As for `dav1d_ipred_z2_fill3_16bpc_neon` with `width == 8`; `w8` is the
/// entry's `xpos`, `w7` is `-dy`, `v31` is `increments`.
#[inline(never)]
unsafe fn ipred_z2_fill3_w8(
    mut x0: *mut u16,
    x1: isize,
    x2: *const u16,
    x3: *const u16,
    mut w8: c_int,
    mut w5: c_int,
    w6: c_int,
    w7: c_int,
    v31: uint16x8_t,
) {
    let v18 = vdupq_n_u16(w7 as u16); // dup v18.8h, w7  (-dy)
    let v17 = vdupq_n_u8(2); // movi v17.16b, #2

    let v16 = vmulq_u16(v31, v18); // mul v16.8h, v31.8h, v18.8h  ({0,1,2,3,4,5,6,7} * -dy)
    let v25 = vdupq_n_u16(0x3e); // movi v25.8h, #0x3e
    let v16 = vaddq_u16(v16, v18); // add v16.8h, v16.8h, v18.8h  (-= dy)

    // For upsample_left, w <= 8 and h <= 8; we may need up to 2*h+1 elements.
    let v0v1v2 = vld1q_u8_x3(x3 as *const u8); // ld1 {v0.8h, v1.8h, v2.8h}, [x3]  (left[])

    let v26 = vdupq_n_u16(64); // movi v26.8h, #64

    // (movi v19.16b, #4: v19 is written again before it is read)

    let v29 = vshrn_n_u16::<6>(v16); // shrn v29.8b, v16.8h, #6  (ypos >> 6)
    let v27 = vandq_u16(v16, v25); // and v27.16b, v16.16b, v25.16b  (frac_y)

    let v29 = vadd_u8(v29, vget_low_u8(v17)); // add v29.8b, v29.8b, v17.8b  (base_y = (ypos >> 6) + 2)

    let v23 = z2_16b(vdupq_n_u16(1 << 8)); // movi v23.8h, #1, lsl #8
    let v29 = vshl_n_u8::<1>(v29); // shl v29.8b, v29.8b, #1  (2*base_y)

    // (mov v18.16b, v15.16b: v15 is not set in this function and v18 is
    // written again before it is read)
    let v29 = vcombine_u8(v29, vdup_n_u8(0));
    let v29 = vzip1q_u8(v29, v29); // zip1 v29.16b, v29.16b, v29.16b  (duplicate elements)
    let mut v29 = vaddq_u8(v29, v23); // add v29.16b, v29.16b, v23.16b  (2*base, 2*base+1, ...)

    let mut v30 = vaddq_u8(v29, v17); // add v30.16b, v29.16b, v17.16b  (base_y + 1 (*2))

    let v28 = vsubq_u16(v26, v27); // sub v28.8h, v26.8h, v27.8h  (64 - frac_y)

    let v24 = vdupq_n_u8(4); // movi v24.16b, #4
    loop {
        // 8:
        let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
        let v16 = vdupq_n_u16(w8 as u16); // dup v16.8h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
        if w9 <= -16 {
            break; // cmp w9, #-16; b.le 89f  (base_x <= -16)
        }

        let v17 = vdupq_n_u16(w8 as u16); // dup v17.8h, w8  (xpos)

        let x9 = x2.wrapping_offset(w9 as isize); // add x9, x2, w9, sxtw #1
        let x11 = x2.wrapping_offset(w11 as isize); // add x11, x2, w11, sxtw #1

        let v4v5 = vld1q_u16_x2(x9); // ld1 {v4.8h, v5.8h}, [x9]  (top[base_x])
        let v6v7 = vld1q_u16_x2(x11); // ld1 {v6.8h, v7.8h}, [x11]
        let (v4, v6) = (v4v5.0, v6v7.0);

        let v18 = z2_8h(vqtbl3q_u8(v0v1v2, v29)); // tbl v18.16b, {v0.16b, v1.16b, v2.16b}, v29.16b  (left[base_y+0])
        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 2 (*2))
        let v19 = z2_8h(vqtbl3q_u8(v0v1v2, v30)); // tbl v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b  (left[base_y+1])
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b
        let v22 = vshrq_n_s16::<6>(vreinterpretq_s16_u16(v16)); // sshr v22.8h, v16.8h, #6  (first base_x)
        let v20 = z2_8h(vqtbl3q_u8(v0v1v2, v29)); // tbl v20.16b, {v0.16b, v1.16b, v2.16b}, v29.16b  (left[base_y+2])
        let v23 = vshrq_n_s16::<6>(vreinterpretq_s16_u16(v17)); // sshr v23.8h, v17.8h, #6
        let v21 = z2_8h(vqtbl3q_u8(v0v1v2, v30)); // tbl v21.16b, {v0.16b, v1.16b, v2.16b}, v30.16b  (left[base_y+3])

        let v5 = vextq_u16::<1>(v4, v4v5.1); // ext v5.16b, v4.16b, v5.16b, #2  (top[base_x+1])
        let v7 = vextq_u16::<1>(v6, v6v7.1); // ext v7.16b, v6.16b, v7.16b, #2

        let v16 = vandq_u16(v16, v25); // and v16.16b, v16.16b, v25.16b  (frac_x)
        let v17 = vandq_u16(v17, v25); // and v17.16b, v17.16b, v25.16b

        let v10s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v10.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
        let v10s = vmlal_u16(v10s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v10.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)

        let v8 = vsubq_u16(v26, v16); // sub v8.8h, v26.8h, v16.8h  (64 - frac_x)
        let v9 = vsubq_u16(v26, v17); // sub v9.8h, v26.8h, v17.8h

        let v11s = vmull_high_u16(v18, v28); // umull2 v11.4s, v18.8h, v28.8h
        let v11s = vmlal_high_u16(v11s, v19, v27); // umlal2 v11.4s, v19.8h, v27.8h

        let v22 = vaddq_s16(v22, vreinterpretq_s16_u16(v31)); // add v22.8h, v22.8h, v31.8h  (actual base_x)
        let v23 = vaddq_s16(v23, vreinterpretq_s16_u16(v31)); // add v23.8h, v23.8h, v31.8h

        let v12s = vmull_u16(vget_low_u16(v20), vget_low_u16(v28)); // umull v12.4s, v20.4h, v28.4h
        let v12s = vmlal_u16(v12s, vget_low_u16(v21), vget_low_u16(v27)); // umlal v12.4s, v21.4h, v27.4h
        let v13s = vmull_high_u16(v20, v28); // umull2 v13.4s, v20.8h, v28.8h
        let v13s = vmlal_high_u16(v13s, v21, v27); // umlal2 v13.4s, v21.8h, v27.8h

        let v10 = vrshrn_n_u32::<6>(v10s); // rshrn v10.4h, v10.4s, #6
        let v10 = vrshrn_high_n_u32::<6>(v10, v11s); // rshrn2 v10.8h, v11.4s, #6
        let v11 = vrshrn_n_u32::<6>(v12s); // rshrn v11.4h, v12.4s, #6
        let v11 = vrshrn_high_n_u32::<6>(v11, v13s); // rshrn2 v11.8h, v13.4s, #6

        let v12s = vmull_u16(vget_low_u16(v4), vget_low_u16(v8)); // umull v12.4s, v4.4h, v8.4h  (top[base_x]*(64-frac_x))
        let v12s = vmlal_u16(v12s, vget_low_u16(v5), vget_low_u16(v16)); // umlal v12.4s, v5.4h, v16.4h  (+ top[base_x+1]*frac_x)
        let v13s = vmull_high_u16(v4, v8); // umull2 v13.4s, v4.8h, v8.8h
        let v13s = vmlal_high_u16(v13s, v5, v16); // umlal2 v13.4s, v5.8h, v16.8h
        let v14s = vmull_u16(vget_low_u16(v6), vget_low_u16(v9)); // umull v14.4s, v6.4h, v9.4h
        let v14s = vmlal_u16(v14s, vget_low_u16(v7), vget_low_u16(v17)); // umlal v14.4s, v7.4h, v17.4h
        let v18s = vmull_high_u16(v6, v9); // umull2 v18.4s, v6.8h, v9.8h
        let v18s = vmlal_high_u16(v18s, v7, v17); // umlal2 v18.4s, v7.8h, v17.8h

        let v22 = vcgezq_s16(v22); // cmge v22.8h, v22.8h, #0
        let v23 = vcgezq_s16(v23); // cmge v23.8h, v23.8h, #0

        let v12 = vrshrn_n_u32::<6>(v12s); // rshrn v12.4h, v12.4s, #6
        let v12 = vrshrn_high_n_u32::<6>(v12, v13s); // rshrn2 v12.8h, v13.4s, #6
        let v13 = vrshrn_n_u32::<6>(v14s); // rshrn v13.4h, v14.4s, #6
        let v13 = vrshrn_high_n_u32::<6>(v13, v18s); // rshrn2 v13.8h, v18.4s, #6

        let v10 = vbslq_u16(v22, v12, v10); // bit v10.16b, v12.16b, v22.16b
        let v11 = vbslq_u16(v23, v13, v11); // bit v11.16b, v13.16b, v23.16b

        vst1q_u16(x0, v10); // st1 {v10.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        vst1q_u16(x0, v11); // st1 {v11.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 2 (*2))
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b
    }

    loop {
        // 89:
        let v18 = z2_8h(vqtbl3q_u8(v0v1v2, v29)); // tbl v18.16b, {v0.16b, v1.16b, v2.16b}, v29.16b  (left[base_y+0])
        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 2 (*2))
        let v19 = z2_8h(vqtbl3q_u8(v0v1v2, v30)); // tbl v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b  (left[base_y+1])
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b
        let v20 = z2_8h(vqtbl3q_u8(v0v1v2, v29)); // tbl v20.16b, {v0.16b, v1.16b, v2.16b}, v29.16b  (left[base_y+2])
        let v21 = z2_8h(vqtbl3q_u8(v0v1v2, v30)); // tbl v21.16b, {v0.16b, v1.16b, v2.16b}, v30.16b  (left[base_y+3])

        let v4s = vmull_u16(vget_low_u16(v18), vget_low_u16(v28)); // umull v4.4s, v18.4h, v28.4h  (left[base_y]*(64-frac_y))
        let v4s = vmlal_u16(v4s, vget_low_u16(v19), vget_low_u16(v27)); // umlal v4.4s, v19.4h, v27.4h  (+ left[base_y+1]*frac_y)
        let v5s = vmull_high_u16(v18, v28); // umull2 v5.4s, v18.8h, v28.8h
        let v5s = vmlal_high_u16(v5s, v19, v27); // umlal2 v5.4s, v19.8h, v27.8h
        let v6s = vmull_u16(vget_low_u16(v20), vget_low_u16(v28)); // umull v6.4s, v20.4h, v28.4h
        let v6s = vmlal_u16(v6s, vget_low_u16(v21), vget_low_u16(v27)); // umlal v6.4s, v21.4h, v27.4h
        let v7s = vmull_high_u16(v20, v28); // umull2 v7.4s, v20.8h, v28.8h
        let v7s = vmlal_high_u16(v7s, v21, v27); // umlal2 v7.4s, v21.8h, v27.8h

        let v4 = vrshrn_n_u32::<6>(v4s); // rshrn v4.4h, v4.4s, #6
        let v4 = vrshrn_high_n_u32::<6>(v4, v5s); // rshrn2 v4.8h, v5.4s, #6
        let v5 = vrshrn_n_u32::<6>(v6s); // rshrn v5.4h, v6.4s, #6
        let v5 = vrshrn_high_n_u32::<6>(v5, v7s); // rshrn2 v5.8h, v7.4s, #6

        vst1q_u16(x0, v4); // st1 {v4.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        vst1q_u16(x0, v5); // st1 {v5.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 2 (*2))
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b
    }
}

/// `ipred_z2_fill3_16bpc_neon` (same parameters as `ipred_z2_fill1`): the
/// variant for an upsampled `left` (`left[2 * y]`, `dy` doubled by the
/// caller).
///
/// # Safety
/// `width` and `height` are 4 or 8 (the caller upsamples only for `width +
/// height <= 16`); `stride` is in bytes. Writes `dst` rows `0..height`,
/// columns `0..width`. Reads `left[0..24]`, and for each pair of rows that
/// is not predicted from `left` alone `top[base_x..base_x + 8]` (width 4) or
/// `top[base_x..base_x + 16]` (width 8) with `base_x = ((1 << 6) - (y + 1)
/// * dx) >> 6` of the pair's two rows `y`.
pub unsafe extern "C" fn dav1d_ipred_z2_fill3_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    top: *const u16,
    left: *const u16,
    width: c_int,
    height: c_int,
    dx: c_int,
    dy: c_int,
) {
    let w8 = (1 << 6) - dx; // mov w8, #(1 << 6); sub w8, w8, w6  (xpos = (1 << 6) - dx)

    let v31 = vld1q_u16(INCREMENTS.as_ptr()); // ldrel q31, x11, increments
    let w7 = dy.wrapping_neg(); // neg w7, w7  (-dy)
    if width == 8 {
        // cmp w4, #8; b.eq 80f
        ipred_z2_fill3_w8(dst, stride, top, left, w8, height, dx, w7, v31) // 80:
    } else {
        ipred_z2_fill3_w4(dst, stride, top, left, w8, height, dx, w7, v31) // 40:
    }
}

// ============================================================================
// Part E (ipred16.S lines 3451–3901): the z3 fill kernels.
// ============================================================================
// Registers are typed locals named after the assembly's registers (`v4`,
// `v18`, `x13`, `w7`); a 64-bit arrangement is a 64-bit vector type.
// Pointers are byte pointers, as the assembly's address arithmetic; the
// post-incremented addresses are advanced with `wrapping_add`/
// `wrapping_offset` after the access, as the assembly's `[x0], x1`.

/// `zip1`/`zip2 v18.8h, v16.8h, v17.8h` followed by `st1 {v18.s}[L], [p]`:
/// the pixels `v16.h[L]`, `v17.h[L]` at `p`, as one `st2` of the lane of
/// the two registers.
#[inline(always)]
unsafe fn ipred_z3_st2_h<const L: i32>(p: *mut u8, v16v17: uint16x8x2_t) {
    vst2q_lane_u16::<L>(p as *mut u16, v16v17);
}

/// The same for two registers whose `.4h` arrangement was written last
/// (`zip1 v18.8h, v16.8h, v17.8h`, lanes 0 to 3).
#[inline(always)]
unsafe fn ipred_z3_st2_4h<const L: i32>(p: *mut u8, v16v17: uint16x4x2_t) {
    vst2_lane_u16::<L>(p as *mut u16, v16v17);
}

/// `ushll v.4s, a.4h, #6` / `smlal v.4s, d.4h, frac.4h` / `rshrn v.4h,
/// v.4s, #6`: `(a * 64 + d * frac + 32) >> 6` with the signed difference
/// `d`.
#[inline(always)]
unsafe fn ipred_z3_ushll_smlal_rshrn(a: uint16x4_t, d: uint16x4_t, frac: uint16x4_t) -> uint16x4_t {
    let v16 = vreinterpretq_s32_u32(vshll_n_u16::<6>(a)); // ushll v16.4s, v0.4h, #6
    let v16 = vmlal_s16(v16, vreinterpret_s16_u16(d), vreinterpret_s16_u16(frac)); // smlal v16.4s, v6.4h, v4.4h
    vreinterpret_u16_s16(vrshrn_n_s32::<6>(v16)) // rshrn v16.4h, v16.4s, #6
}

/// `umull v.4s, a.4h, c.4h` / `umlal v.4s, b.4h, f.4h` / `umull2` /
/// `umlal2` / `rshrn v.4h, v.4s, #6` / `rshrn2 v.8h, v.4s, #6`:
/// `(a * c + b * f + 32) >> 6` per lane.
#[inline(always)]
unsafe fn ipred_z3_umull_umlal_rshrn(a: uint16x8_t, c: uint16x8_t, b: uint16x8_t, f: uint16x8_t) -> uint16x8_t {
    let lo = vmull_u16(vget_low_u16(a), vget_low_u16(c)); // umull v16.4s, v0.4h, v6.4h
    let lo = vmlal_u16(lo, vget_low_u16(b), vget_low_u16(f)); // umlal v16.4s, v1.4h, v4.4h
    let hi = vmull_high_u16(a, c); // umull2 v17.4s, v0.8h, v6.8h
    let hi = vmlal_high_u16(hi, b, f); // umlal2 v17.4s, v1.8h, v4.8h
    vrshrn_high_n_u32::<6>(vrshrn_n_u32::<6>(lo), hi) // rshrn v16.4h, v16.4s, #6; rshrn2 v16.8h, v17.4s, #6
}

/// `ipred_z3_fill1_16bpc_neon`:
/// `void ipred_z3_fill1_16bpc_neon(pixel *dst, const ptrdiff_t stride,
/// const pixel *const left, const int width, const int height, const int dy,
/// const int max_base_y);`
///
/// # Safety
/// Called by `ipred_z3_neon` (`simd/arm/ipred.rs`) only. Writes `dst` rows
/// `0..height`, columns `0..width`. Reads `left[max_base_y]` and, for the
/// columns' `base < max_base_y + 16`, `left[base..base + 8]` (height 4),
/// `left[base..base + 9]` (height 8) or `left[base..base + height + 8]`
/// (height 16 to 64), all inside the wrapper's `left_out`
/// (`64 + 64 + (64 + 15) * 2` pixels).
pub unsafe extern "C" fn dav1d_ipred_z3_fill1_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    width: c_int,
    height: c_int,
    dy: c_int,
    max_base_y: c_int,
) {
    let x0 = dst as *mut u8;
    let x2 = left as *const u8;
    let x10 = x2.add((max_base_y as u32 as usize) << 1); // add x10, x2, w6, uxtw #1: left[max_base_y]
    let v31 = vld1q_dup_u16(x10 as *const u16); // ld1r {v31.8h}, [x10]: padding
    let w7 = dy as u32; // mov w7, w5
    let x13 = x0.wrapping_offset(stride); // add x13, x0, x1
    let x1 = stride.wrapping_shl(1); // lsl x1, x1, #1

    // clz w9, w4; sub w9, w9, #25; ipred_z3_fill1_tbl; br x8
    match (height as u32).leading_zeros().wrapping_sub(25) {
        4 => ipred_z3_fill1_h4(x0, x1, x2, width, height, dy, max_base_y, w7, x13, v31),
        3 => ipred_z3_fill1_h8(x0, x1, x2, width, height, dy, max_base_y, w7, x13, v31),
        2 => ipred_z3_fill1_h16::<16>(x0, x1, x2, width, dy, max_base_y, w7, x13, v31),
        1 => ipred_z3_fill1_h16::<32>(x0, x1, x2, width, dy, max_base_y, w7, x13, v31),
        0 => ipred_z3_fill1_h16::<64>(x0, x1, x2, width, dy, max_base_y, w7, x13, v31),
        // SAFETY: the height is one of 4, 8, 16, 32, 64.
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `40:` of `ipred_z3_fill1_16bpc_neon` (the table is indexed with the
/// height: four rows per column pair)
#[inline(always)]
unsafe fn ipred_z3_fill1_h4(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w3: c_int,
    w4: c_int,
    w5: c_int,
    w6: c_int,
    mut w7: u32,
    mut x13: *mut u8,
    v31: uint16x8_t,
) {
    loop {
        // 4:
        let w8 = w7 >> 6; // lsr w8, w7, #6: base
        let w9 = w7 & 0x3e; // and w9, w7, #0x3e: frac
        w7 = w7.wrapping_add(w5 as u32); // add w7, w7, w5: xpos += dx
        let w10 = w7 >> 6; // lsr w10, w7, #6: base
        let w11 = w7 & 0x3e; // and w11, w7, #0x3e: frac
        if w8 as c_int >= w6 {
            // cmp w8, w6: base >= max_base_x; b.ge ipred_z3_fill_padding_neon
            return ipred_z3_fill_padding_neon(x0, x1, x13, w3, w4, v31);
        }
        let w8 = w8 << 1; // lsl w8, w8, #1
        let w10 = w10 << 1; // lsl w10, w10, #1
        let v0 = vld1q_u16(x2.add(w8 as usize) as *const u16); // ldr q0, [x2, w8, uxtw]: left[base]
        let v2 = vld1q_u16(x2.add(w10 as usize) as *const u16); // ldr q2, [x2, w10, uxtw]
        let v4 = vdupq_n_u16(w9 as u16); // dup v4.8h, w9: frac
        let v5 = vdupq_n_u16(w11 as u16); // dup v5.8h, w11
        let v1 = vextq_u16::<1>(v0, v0); // ext v1.16b, v0.16b, v0.16b, #2: left[base+1]
        let v3 = vextq_u16::<1>(v2, v2); // ext v3.16b, v2.16b, v2.16b, #2
        let v6 = vsub_u16(vget_low_u16(v1), vget_low_u16(v0)); // sub v6.4h, v1.4h, v0.4h: top[base+1]-top[base]
        let v7 = vsub_u16(vget_low_u16(v3), vget_low_u16(v2)); // sub v7.4h, v3.4h, v2.4h

        // ushll v16.4s, v0.4h, #6: top[base]*64; smlal v16.4s, v6.4h, v4.4h: + top[base+1]*frac;
        // rshrn v16.4h, v16.4s, #6
        let v16 = ipred_z3_ushll_smlal_rshrn(vget_low_u16(v0), v6, vget_low_u16(v4));
        // ushll v17.4s, v2.4h, #6; smlal v17.4s, v7.4h, v5.4h; rshrn v17.4h, v17.4s, #6
        let v17 = ipred_z3_ushll_smlal_rshrn(vget_low_u16(v2), v7, vget_low_u16(v5));
        w3 -= 2; // subs w3, w3, #2

        // zip1 v18.8h, v16.8h, v17.8h: the stores take the pairs from the
        // two registers
        let v16v17 = uint16x4x2_t(v16, v17);
        ipred_z3_st2_4h::<0>(x0, v16v17); // st1 {v18.s}[0], [x0], x1
        ipred_z3_st2_4h::<1>(x13, v16v17); // st1 {v18.s}[1], [x13], x1
        w7 = w7.wrapping_add(w5 as u32); // add w7, w7, w5: xpos += dx
        ipred_z3_st2_4h::<2>(x0.wrapping_offset(x1), v16v17); // st1 {v18.s}[2], [x0]
        ipred_z3_st2_4h::<3>(x13.wrapping_offset(x1), v16v17); // st1 {v18.s}[3], [x13]
        if w3 <= 0 {
            return; // b.le 9f
        }
        // sub x0, x0, x1: ptr -= 4 * (2*stride); sub x13, x13, x1: back at
        // the top of the column pair (`x0`, `x13`)
        x0 = x0.wrapping_add(4); // add x0, x0, #4
        x13 = x13.wrapping_add(4); // add x13, x13, #4
    }
}

/// `80:` of `ipred_z3_fill1_16bpc_neon` (height 8)
#[inline(always)]
unsafe fn ipred_z3_fill1_h8(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w3: c_int,
    w4: c_int,
    w5: c_int,
    w6: c_int,
    mut w7: u32,
    mut x13: *mut u8,
    v31: uint16x8_t,
) {
    let w15 = 64; // mov w15, #64
    loop {
        // 8:
        let w8 = w7 >> 6; // lsr w8, w7, #6: base
        let w9 = w7 & 0x3e; // and w9, w7, #0x3e: frac
        w7 = w7.wrapping_add(w5 as u32); // add w7, w7, w5: xpos += dx
        let w10 = w7 >> 6; // lsr w10, w7, #6: base
        let w11 = w7 & 0x3e; // and w11, w7, #0x3e: frac
        if w8 as c_int >= w6 {
            // cmp w8, w6: base >= max_base_x; b.ge ipred_z3_fill_padding_neon
            return ipred_z3_fill_padding_neon(x0, x1, x13, w3, w4, v31);
        }
        let x8 = x2.add((w8 as usize) << 1); // add x8, x2, w8, uxtw #1
        let x10 = x2.add((w10 as usize) << 1); // add x10, x2, w10, uxtw #1
        let v4 = vdupq_n_u16(w9 as u16); // dup v4.8h, w9: frac
        let v5 = vdupq_n_u16(w11 as u16); // dup v5.8h, w11
        let v0 = vld1q_u16(x8 as *const u16); // ld1 {v0.8h}, [x8]: left[base]
        let v2 = vld1q_u16(x10 as *const u16); // ld1 {v2.8h}, [x10]
        let w9 = w15 - w9; // sub w9, w15, w9: 64 - frac
        let w11 = w15 - w11; // sub w11, w15, w11
        let v1 = vsetq_lane_u16::<0>((x8.add(16) as *const u16).read_unaligned(), vdupq_n_u16(0)); // ldr h1, [x8, #16]
        let v3 = vsetq_lane_u16::<0>((x10.add(16) as *const u16).read_unaligned(), vdupq_n_u16(0)); // ldr h3, [x10, #16]
        let v6 = vdupq_n_u16(w9 as u16); // dup v6.8h, w9: 64 - frac
        let v7 = vdupq_n_u16(w11 as u16); // dup v7.8h, w11
        let v1 = vextq_u16::<1>(v0, v1); // ext v1.16b, v0.16b, v1.16b, #2: left[base+1]
        let v3 = vextq_u16::<1>(v2, v3); // ext v3.16b, v2.16b, v3.16b, #2

        // umull v16.4s, v0.4h, v6.4h: left[base]*(64-frac); umlal v16.4s, v1.4h, v4.4h: + left[base+1]*frac;
        // umull2 v17.4s, v0.8h, v6.8h; umlal2 v17.4s, v1.8h, v4.8h;
        // rshrn v16.4h, v16.4s, #6; rshrn2 v16.8h, v17.4s, #6
        let v16 = ipred_z3_umull_umlal_rshrn(v0, v6, v1, v4);
        // umull v18.4s, v2.4h, v7.4h; umlal v18.4s, v3.4h, v5.4h; umull2 v19.4s, v2.8h, v7.8h;
        // umlal2 v19.4s, v3.8h, v5.8h; rshrn v17.4h, v18.4s, #6; rshrn2 v17.8h, v19.4s, #6
        let v17 = ipred_z3_umull_umlal_rshrn(v2, v7, v3, v5);
        w3 -= 2; // subs w3, w3, #2

        // zip1 v18.8h, v16.8h, v17.8h; zip2 v19.8h, v16.8h, v17.8h: the
        // stores take the pairs from the two registers
        let v16v17 = uint16x8x2_t(v16, v17);
        w7 = w7.wrapping_add(w5 as u32); // add w7, w7, w5: xpos += dx

        // The row pointers of this column pair; `x0`/`x13` stay at its top.
        let (mut p0, mut p13) = (x0, x13);
        ipred_z3_st2_h::<0>(p0, v16v17); // st1 {v18.s}[0], [x0], x1
        p0 = p0.wrapping_offset(x1);
        ipred_z3_st2_h::<1>(p13, v16v17); // st1 {v18.s}[1], [x13], x1
        p13 = p13.wrapping_offset(x1);
        ipred_z3_st2_h::<2>(p0, v16v17); // st1 {v18.s}[2], [x0], x1
        p0 = p0.wrapping_offset(x1);
        ipred_z3_st2_h::<3>(p13, v16v17); // st1 {v18.s}[3], [x13], x1
        p13 = p13.wrapping_offset(x1);
        ipred_z3_st2_h::<4>(p0, v16v17); // st1 {v19.s}[0], [x0], x1
        p0 = p0.wrapping_offset(x1);
        ipred_z3_st2_h::<5>(p13, v16v17); // st1 {v19.s}[1], [x13], x1
        p13 = p13.wrapping_offset(x1);
        ipred_z3_st2_h::<6>(p0, v16v17); // st1 {v19.s}[2], [x0], x1
        ipred_z3_st2_h::<7>(p13, v16v17); // st1 {v19.s}[3], [x13], x1
        if w3 <= 0 {
            return; // b.le 9f
        }
        // sub x0, x0, x1, lsl #2: ptr -= 4 * (2*stride); sub x13, x13, x1, lsl #2:
        // back at the top of the column pair (`x0`, `x13`)
        x0 = x0.wrapping_add(4); // add x0, x0, #4
        x13 = x13.wrapping_add(4); // add x13, x13, #4
    }
}

/// `160:`/`320:`/`640:` of `ipred_z3_fill1_16bpc_neon` (height 16, 32, 64):
/// one block in the assembly; here one instance per height `H`, so that
/// the compiler unrolls the loop `2:` over the 16 rows and the entry point
/// keeps no frame for it.
#[inline(never)]
unsafe fn ipred_z3_fill1_h16<const H: c_int>(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w3: c_int,
    w5: c_int,
    w6: c_int,
    mut w7: u32,
    mut x13: *mut u8,
    v31: uint16x8_t,
) {
    let mut w4 = H;
    let w15 = 64; // mov w15, #64
    let w12 = w4; // mov w12, w4
    loop {
        // 1:
        let w8 = w7 >> 6; // lsr w8, w7, #6: base
        let w9 = w7 & 0x3e; // and w9, w7, #0x3e: frac
        w7 = w7.wrapping_add(w5 as u32); // add w7, w7, w5: ypos += dy
        let w10 = w7 >> 6; // lsr w10, w7, #6: base
        let w11 = w7 & 0x3e; // and w11, w7, #0x3e: frac
        if w8 as c_int >= w6 {
            // cmp w8, w6: base >= max_base_y; b.ge ipred_z3_fill_padding_neon
            return ipred_z3_fill_padding_neon(x0, x1, x13, w3, w4, v31);
        }
        let mut x8 = x2.add((w8 as usize) << 1); // add x8, x2, w8, uxtw #1
        let mut x10 = x2.add((w10 as usize) << 1); // add x10, x2, w10, uxtw #1
        let v6 = vdupq_n_u16(w9 as u16); // dup v6.8h, w9: frac
        let v7 = vdupq_n_u16(w11 as u16); // dup v7.8h, w11
        let v0v2 = vld1q_u16_x3(x8 as *const u16); // ld1 {v0.8h, v1.8h, v2.8h}, [x8], #48: left[base]
        x8 = x8.wrapping_add(48);
        let v3v5 = vld1q_u16_x3(x10 as *const u16); // ld1 {v3.8h, v4.8h, v5.8h}, [x10], #48
        x10 = x10.wrapping_add(48);
        let (mut v0, mut v1, mut v2) = (v0v2.0, v0v2.1, v0v2.2);
        let (mut v3, mut v4, mut v5) = (v3v5.0, v3v5.1, v3v5.2);
        let w9 = w15 - w9; // sub w9, w15, w9: 64 - frac
        let w11 = w15 - w11; // sub w11, w15, w11
        let v16 = vdupq_n_u16(w9 as u16); // dup v16.8h, w9: 64 - frac
        let v17 = vdupq_n_u16(w11 as u16); // dup v17.8h, w11
        w7 = w7.wrapping_add(w5 as u32); // add w7, w7, w5: ypos += dy

        // The row pointers of this column pair; `x0`/`x13` stay at its top
        // (see `3:`).
        let (mut p0, mut p13) = (x0, x13);
        loop {
            // 2:
            let v18 = vextq_u16::<1>(v0, v1); // ext v18.16b, v0.16b, v1.16b, #2: left[base+1]
            let v19 = vextq_u16::<1>(v1, v2); // ext v19.16b, v1.16b, v2.16b, #2
            let v20 = vextq_u16::<1>(v3, v4); // ext v20.16b, v3.16b, v4.16b, #2
            let v21 = vextq_u16::<1>(v4, v5); // ext v21.16b, v4.16b, v5.16b, #2
            w4 -= 16; // subs w4, w4, #16

            // umull v22.4s, v0.4h, v16.4h: left[base]*(64-frac); umlal v22.4s, v18.4h, v6.4h: + left[base+1]*frac;
            // umull2 v23.4s, v0.8h, v16.8h; umlal2 v23.4s, v18.8h, v6.8h;
            // rshrn v22.4h, v22.4s, #6; rshrn2 v22.8h, v23.4s, #6
            let v22 = ipred_z3_umull_umlal_rshrn(v0, v16, v18, v6);
            // umull v24.4s, v1.4h, v16.4h; umlal v24.4s, v19.4h, v6.4h; umull2 v25.4s, v1.8h, v16.8h;
            // umlal2 v25.4s, v19.8h, v6.8h; rshrn v23.4h, v24.4s, #6; rshrn2 v23.8h, v25.4s, #6
            let v23 = ipred_z3_umull_umlal_rshrn(v1, v16, v19, v6);
            // umull v26.4s, v3.4h, v17.4h; umlal v26.4s, v20.4h, v7.4h; umull2 v27.4s, v3.8h, v17.8h;
            // umlal2 v27.4s, v20.8h, v7.8h; rshrn v24.4h, v26.4s, #6; rshrn2 v24.8h, v27.4s, #6
            let v24 = ipred_z3_umull_umlal_rshrn(v3, v17, v20, v7);
            // umull v28.4s, v4.4h, v17.4h; umlal v28.4s, v21.4h, v7.4h; umull2 v29.4s, v4.8h, v17.8h;
            // umlal2 v29.4s, v21.8h, v7.8h; rshrn v25.4h, v28.4s, #6; rshrn2 v25.8h, v29.4s, #6
            let v25 = ipred_z3_umull_umlal_rshrn(v4, v17, v21, v7);
            // zip1 v18.8h, v22.8h, v24.8h; zip2 v19.8h, v22.8h, v24.8h;
            // zip1 v20.8h, v23.8h, v25.8h; zip2 v21.8h, v23.8h, v25.8h: the
            // stores take the pairs from the two registers
            let v22v24 = uint16x8x2_t(v22, v24);
            let v23v25 = uint16x8x2_t(v23, v25);
            ipred_z3_st2_h::<0>(p0, v22v24); // st1 {v18.s}[0], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_h::<1>(p13, v22v24); // st1 {v18.s}[1], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_h::<2>(p0, v22v24); // st1 {v18.s}[2], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_h::<3>(p13, v22v24); // st1 {v18.s}[3], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_h::<4>(p0, v22v24); // st1 {v19.s}[0], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_h::<5>(p13, v22v24); // st1 {v19.s}[1], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_h::<6>(p0, v22v24); // st1 {v19.s}[2], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_h::<7>(p13, v22v24); // st1 {v19.s}[3], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_h::<0>(p0, v23v25); // st1 {v20.s}[0], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_h::<1>(p13, v23v25); // st1 {v20.s}[1], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_h::<2>(p0, v23v25); // st1 {v20.s}[2], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_h::<3>(p13, v23v25); // st1 {v20.s}[3], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_h::<4>(p0, v23v25); // st1 {v21.s}[0], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_h::<5>(p13, v23v25); // st1 {v21.s}[1], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_h::<6>(p0, v23v25); // st1 {v21.s}[2], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_h::<7>(p13, v23v25); // st1 {v21.s}[3], [x13], x1
            p13 = p13.wrapping_offset(x1);
            if w4 <= 0 {
                break; // b.le 3f
            }
            v0 = v2; // mov v0.16b, v2.16b
            let v1v2 = vld1q_u16_x2(x8 as *const u16); // ld1 {v1.8h, v2.8h}, [x8], #32: left[base]
            x8 = x8.wrapping_add(32);
            (v1, v2) = (v1v2.0, v1v2.1);
            v3 = v5; // mov v3.16b, v5.16b
            let v4v5 = vld1q_u16_x2(x10 as *const u16); // ld1 {v4.8h, v5.8h}, [x10], #32
            x10 = x10.wrapping_add(32);
            (v4, v5) = (v4v5.0, v4v5.1);
        }

        // 3:
        w3 -= 2; // subs w3, w3, #2
        if w3 <= 0 {
            return; // b.le 9f
        }
        // lsr x1, x1, #1; msub x0, x1, x12, x0: ptr -= h * stride; msub x13, x1, x12, x13;
        // lsl x1, x1, #1: back at the top of the column pair (`x0`, `x13`)
        x0 = x0.wrapping_add(4); // add x0, x0, #4
        x13 = x13.wrapping_add(4); // add x13, x13, #4
        w4 = w12; // mov w4, w12
    }
}

/// One label (`20:`, `40:`, `80:`) of `ipred_z3_fill_padding_neon`: fills
/// `N` columns of the constant height with padding and steps to the next
/// columns. Returns true at `b.le 9f`.
#[inline(always)]
unsafe fn ipred_z3_fill_padding_cols<const N: usize>(
    x0: &mut *mut u8,
    x1: isize,
    x13: &mut *mut u8,
    w3: &mut c_int,
    w4: &mut c_int,
    w12: c_int,
    v31: uint16x8_t,
) -> bool {
    // st1 {v31.s}[0] / {v31.4h} / {v31.8h}, [p], x1
    #[inline(always)]
    unsafe fn st<const N: usize>(p: *mut u8, v31: uint16x8_t) {
        match N {
            2 => (p as *mut u32).write_unaligned(vgetq_lane_u32::<0>(vreinterpretq_u32_u16(v31))),
            4 => vst1_u16(p as *mut u16, vget_low_u16(v31)),
            _ => vst1q_u16(p as *mut u16, v31),
        }
    }
    // The row pointers of these columns; `x0`/`x13` stay at their top.
    let (mut p0, mut p13) = (*x0, *x13);
    loop {
        // 2: / 4: / 8:
        st::<N>(p0, v31); // st1 {v31}, [x0], x1
        p0 = p0.wrapping_offset(x1);
        *w4 -= 4; // subs w4, w4, #4
        st::<N>(p13, v31); // st1 {v31}, [x13], x1
        p13 = p13.wrapping_offset(x1);
        st::<N>(p0, v31); // st1 {v31}, [x0], x1
        p0 = p0.wrapping_offset(x1);
        st::<N>(p13, v31); // st1 {v31}, [x13], x1
        p13 = p13.wrapping_offset(x1);
        if *w4 <= 0 {
            break; // b.gt 2b
        }
    }
    *w3 -= N as c_int; // subs w3, w3, #N

    // lsr x1, x1, #1; msub x0, x1, x12, x0: ptr -= h * stride; msub x13, x1, x12, x13:
    // back at the top of the columns (`x0`, `x13`)
    if *w3 <= 0 {
        return true; // b.le 9f
    }
    // lsl x1, x1, #1
    *x0 = x0.wrapping_add(2 * N); // add x0, x0, #2*N
    *x13 = x13.wrapping_add(2 * N); // add x13, x13, #2*N
    *w4 = w12; // mov w4, w12
    false // b 1b
}

/// `ipred_z3_fill_padding_neon`: reached by a branch from the fill kernels
/// with `x0`/`x13` at the current column of rows 0 and 1, `x1` the doubled
/// stride, `w3` the remaining width, `w4` the constant height and `v31` the
/// padding.
///
/// # Safety
/// Writes the remaining `w3` columns of the `w4` rows of `dst`.
#[inline(never)]
unsafe fn ipred_z3_fill_padding_neon(
    mut x0: *mut u8,
    x1: isize,
    mut x13: *mut u8,
    mut w3: c_int,
    mut w4: c_int,
    v31: uint16x8_t,
) {
    if w3 > 8 {
        // cmp w3, #8; b.gt ipred_z3_fill_padding_wide
        return ipred_z3_fill_padding_wide(x0, x1, w3, w4, v31);
    }
    // w3 = remaining width, w4 = constant height
    let w12 = w4; // mov w12, w4

    loop {
        // 1:
        // Fill a WxH rectangle with padding. W can be any number;
        // this fills the exact width by filling in the largest
        // power of two in the remaining width, and repeating.
        // clz w9, w3; sub w9, w9, #25; ipred_z3_fill_padding_tbl; br x9
        let done = match (w3 as u32).leading_zeros().wrapping_sub(25) {
            5 => ipred_z3_fill_padding_cols::<2>(&mut x0, x1, &mut x13, &mut w3, &mut w4, w12, v31), // 20:
            4 => ipred_z3_fill_padding_cols::<4>(&mut x0, x1, &mut x13, &mut w3, &mut w4, w12, v31), // 40:
            0 | 1 | 2 | 3 => ipred_z3_fill_padding_cols::<8>(&mut x0, x1, &mut x13, &mut w3, &mut w4, w12, v31), // 80: 160: 320: 640:
            // SAFETY: the remaining width is even and in 2..=8 here.
            _ => core::hint::unreachable_unchecked(),
        };
        if done {
            return; // 9: ret
        }
    }
}

/// `ipred_z3_fill_padding_wide`: Fill a WxH rectangle with padding, with
/// W > 8.
#[inline(always)]
unsafe fn ipred_z3_fill_padding_wide(mut x0: *mut u8, x1: isize, mut w3: c_int, mut w4: c_int, v31: uint16x8_t) {
    let x1 = (x1 as usize) >> 1; // lsr x1, x1, #1
    let w12 = w3; // mov w12, w3
    let x1 = x1.wrapping_sub((w3 as u32 as usize) << 1); // sub x1, x1, w3, uxtw #1
    loop {
        // 1:
        let w5 = w3 & 7; // ands w5, w3, #7
        if w5 != 0 {
            // b.eq 2f
            // If the width isn't aligned to 8, first do one 8 pixel write
            // and align the start pointer.
            w3 -= w5; // sub w3, w3, w5
            vst1q_u16(x0 as *mut u16, v31); // st1 {v31.8h}, [x0]
            x0 = x0.wrapping_add((w5 as u32 as usize) << 1); // add x0, x0, w5, uxtw #1
        }
        // Fill the rest of the line with aligned 8 pixel writes.
        // (`w3` is a positive multiple of 8 here; the loop counts its
        // 8-pixel blocks down to zero, for `subs w3, w3, #8` / `b.gt 2b`.)
        let mut blocks = (w3 as u32) >> 3;
        loop {
            // 2:
            vst1q_u16(x0 as *mut u16, v31); // st1 {v31.8h}, [x0], #16
            x0 = x0.wrapping_add(16);
            blocks -= 1; // subs w3, w3, #8
            if blocks == 0 {
                break; // b.gt 2b
            }
        }
        w4 -= 1; // subs w4, w4, #1
        x0 = x0.wrapping_add(x1); // add x0, x0, x1
        if w4 <= 0 {
            return; // b.le 9f
        }
        w3 = w12; // mov w3, w12
    }
}

/// `ipred_z3_fill2_16bpc_neon`:
/// `void ipred_z3_fill2_16bpc_neon(pixel *dst, const ptrdiff_t stride,
/// const pixel *const left, const int width, const int height, const int dy,
/// const int max_base_y);` (the upsampled edge: `left[]` steps by two per
/// row).
///
/// The assembly takes the padding from the byte at `left + max_base_y`
/// (`add x10, x2, w6, uxtw` without the pixel size, `ld1r {v31.16b}`), not
/// from the pixel `left[max_base_y]`; this is kept. It only shows where a
/// column's base reaches `max_base_y`, which the wrapper's upsampled calls
/// (width + height <= 16, dy <= 2 * 51) never do.
///
/// # Safety
/// Called by `ipred_z3_neon` (`simd/arm/ipred.rs`) only, with width and
/// height 4 or 8. Writes `dst` rows `0..height`, columns `0..width`. Reads
/// the byte at `left + max_base_y` and, for the columns' `base <
/// max_base_y + 4`, `left[base..base + 8]` (height 4) or
/// `left[base..base + 16]` (height 8).
pub unsafe extern "C" fn dav1d_ipred_z3_fill2_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    left: *const u16,
    width: c_int,
    height: c_int,
    dy: c_int,
    max_base_y: c_int,
) {
    let x0 = dst as *mut u8;
    let x2 = left as *const u8;
    let x10 = x2.add(max_base_y as u32 as usize); // add x10, x2, w6, uxtw: left[max_base_y]
    let v31 = vreinterpretq_u16_u8(vld1q_dup_u8(x10)); // ld1r {v31.16b}, [x10]: padding
    let w7 = dy as u32; // mov w7, w5
    let x13 = x0.wrapping_offset(stride); // add x13, x0, x1
    let x1 = stride.wrapping_shl(1); // lsl x1, x1, #1
    if height == 8 {
        // cmp w4, #8; b.eq 8f
        ipred_z3_fill2_h8(x0, x1, x2, width, height, dy, max_base_y, w7, x13, v31)
    } else {
        ipred_z3_fill2_h4(x0, x1, x2, width, height, dy, max_base_y, w7, x13, v31)
    }
}

/// `4:` of `ipred_z3_fill2_16bpc_neon` (h == 4)
#[inline(always)]
unsafe fn ipred_z3_fill2_h4(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w3: c_int,
    w4: c_int,
    w5: c_int,
    w6: c_int,
    mut w7: u32,
    mut x13: *mut u8,
    v31: uint16x8_t,
) {
    loop {
        // 4:
        let w8 = w7 >> 6; // lsr w8, w7, #6: base
        let w9 = w7 & 0x3e; // and w9, w7, #0x3e: frac
        w7 = w7.wrapping_add(w5 as u32); // add w7, w7, w5: xpos += dx
        let w10 = w7 >> 6; // lsr w10, w7, #6: base
        let w11 = w7 & 0x3e; // and w11, w7, #0x3e: frac
        if w8 as c_int >= w6 {
            // cmp w8, w6: base >= max_base_x; b.ge ipred_z3_fill_padding_neon
            return ipred_z3_fill_padding_neon(x0, x1, x13, w3, w4, v31);
        }
        let w8 = w8 << 1; // lsl w8, w8, #1
        let w10 = w10 << 1; // lsl w10, w10, #1
        let v0 = vld1q_u16(x2.add(w8 as usize) as *const u16); // ldr q0, [x2, w8, uxtw]: top[base]
        let v2 = vld1q_u16(x2.add(w10 as usize) as *const u16); // ldr q2, [x2, w10, uxtw]
        let v4 = vdup_n_u16(w9 as u16); // dup v4.4h, w9: frac
        let v5 = vdup_n_u16(w11 as u16); // dup v5.4h, w11
        let v1 = vuzp2q_u16(v0, v0); // uzp2 v1.8h, v0.8h, v0.8h: top[base+1]
        let v0 = vuzp1q_u16(v0, v0); // uzp1 v0.8h, v0.8h, v0.8h: top[base]
        let v3 = vuzp2q_u16(v2, v2); // uzp2 v3.8h, v2.8h, v2.8h
        let v2 = vuzp1q_u16(v2, v2); // uzp1 v2.8h, v2.8h, v2.8h
        let v6 = vsub_u16(vget_low_u16(v1), vget_low_u16(v0)); // sub v6.4h, v1.4h, v0.4h: top[base+1]-top[base]
        let v7 = vsub_u16(vget_low_u16(v3), vget_low_u16(v2)); // sub v7.4h, v3.4h, v2.4h

        // ushll v16.4s, v0.4h, #6: top[base]*64; smlal v16.4s, v6.4h, v4.4h: + top[base+1]*frac;
        // rshrn v16.4h, v16.4s, #6
        let v16 = ipred_z3_ushll_smlal_rshrn(vget_low_u16(v0), v6, v4);
        // ushll v17.4s, v2.4h, #6; smlal v17.4s, v7.4h, v5.4h; rshrn v17.4h, v17.4s, #6
        let v17 = ipred_z3_ushll_smlal_rshrn(vget_low_u16(v2), v7, v5);
        w3 -= 2; // subs w3, w3, #2

        // zip1 v18.8h, v16.8h, v17.8h: the stores take the pairs from the
        // two registers
        let v16v17 = uint16x4x2_t(v16, v17);
        ipred_z3_st2_4h::<0>(x0, v16v17); // st1 {v18.s}[0], [x0], x1
        ipred_z3_st2_4h::<1>(x13, v16v17); // st1 {v18.s}[1], [x13], x1
        w7 = w7.wrapping_add(w5 as u32); // add w7, w7, w5: xpos += dx
        ipred_z3_st2_4h::<2>(x0.wrapping_offset(x1), v16v17); // st1 {v18.s}[2], [x0]
        ipred_z3_st2_4h::<3>(x13.wrapping_offset(x1), v16v17); // st1 {v18.s}[3], [x13]
        if w3 <= 0 {
            return; // b.le 9f
        }
        // sub x0, x0, x1: ptr -= 4 * (2*stride); sub x13, x13, x1: back at
        // the top of the column pair (`x0`, `x13`)
        x0 = x0.wrapping_add(4); // add x0, x0, #4
        x13 = x13.wrapping_add(4); // add x13, x13, #4
    }
}

/// `8:` of `ipred_z3_fill2_16bpc_neon` (h == 8)
#[inline(always)]
unsafe fn ipred_z3_fill2_h8(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w3: c_int,
    w4: c_int,
    w5: c_int,
    w6: c_int,
    mut w7: u32,
    mut x13: *mut u8,
    v31: uint16x8_t,
) {
    let w15 = 64; // mov w15, #64
    loop {
        // 8:
        let w8 = w7 >> 6; // lsr w8, w7, #6: base
        let w9 = w7 & 0x3e; // and w9, w7, #0x3e: frac
        w7 = w7.wrapping_add(w5 as u32); // add w7, w7, w5: xpos += dx
        let w10 = w7 >> 6; // lsr w10, w7, #6: base
        let w11 = w7 & 0x3e; // and w11, w7, #0x3e: frac
        if w8 as c_int >= w6 {
            // cmp w8, w6: base >= max_base_x; b.ge ipred_z3_fill_padding_neon
            return ipred_z3_fill_padding_neon(x0, x1, x13, w3, w4, v31);
        }
        let x8 = x2.add((w8 as usize) << 1); // add x8, x2, w8, uxtw #1
        let x10 = x2.add((w10 as usize) << 1); // add x10, x2, w10, uxtw #1
        let v4 = vdupq_n_u16(w9 as u16); // dup v4.8h, w9: frac
        let v5 = vdupq_n_u16(w11 as u16); // dup v5.8h, w11
        let v0v1 = vld1q_u16_x2(x8 as *const u16); // ld1 {v0.8h, v1.8h}, [x8]: top[base]
        let v2v3 = vld1q_u16_x2(x10 as *const u16); // ld1 {v2.8h, v3.8h}, [x10]
        let w9 = w15 - w9; // sub w9, w15, w9: 64 - frac
        let w11 = w15 - w11; // sub w11, w15, w11
        let v6 = vdupq_n_u16(w9 as u16); // dup v6.8h, w9: 64 - frac
        let v7 = vdupq_n_u16(w11 as u16); // dup v7.8h, w11
        let v20 = vuzp2q_u16(v0v1.0, v0v1.1); // uzp2 v20.8h, v0.8h, v1.8h: top[base+1]
        let v0 = vuzp1q_u16(v0v1.0, v0v1.1); // uzp1 v0.8h, v0.8h, v1.8h: top[base]
        let v21 = vuzp2q_u16(v2v3.0, v2v3.1); // uzp2 v21.8h, v2.8h, v3.8h
        let v2 = vuzp1q_u16(v2v3.0, v2v3.1); // uzp1 v2.8h, v2.8h, v3.8h

        // umull v16.4s, v0.4h, v6.4h: top[base]*(64-frac); umlal v16.4s, v20.4h, v4.4h: + top[base+1]*frac;
        // umull2 v17.4s, v0.8h, v6.8h; umlal2 v17.4s, v20.8h, v4.8h;
        // rshrn v16.4h, v16.4s, #6; rshrn2 v16.8h, v17.4s, #6
        let v16 = ipred_z3_umull_umlal_rshrn(v0, v6, v20, v4);
        // umull v18.4s, v2.4h, v7.4h; umlal v18.4s, v21.4h, v5.4h; umull2 v19.4s, v2.8h, v7.8h;
        // umlal2 v19.4s, v21.8h, v5.8h; rshrn v17.4h, v18.4s, #6; rshrn2 v17.8h, v19.4s, #6
        let v17 = ipred_z3_umull_umlal_rshrn(v2, v7, v21, v5);
        w3 -= 2; // subs w3, w3, #2

        // zip1 v18.8h, v16.8h, v17.8h; zip2 v19.8h, v16.8h, v17.8h: the
        // stores take the pairs from the two registers
        let v16v17 = uint16x8x2_t(v16, v17);
        w7 = w7.wrapping_add(w5 as u32); // add w7, w7, w5: xpos += dx

        // The row pointers of this column pair; `x0`/`x13` stay at its top.
        let (mut p0, mut p13) = (x0, x13);
        ipred_z3_st2_h::<0>(p0, v16v17); // st1 {v18.s}[0], [x0], x1
        p0 = p0.wrapping_offset(x1);
        ipred_z3_st2_h::<1>(p13, v16v17); // st1 {v18.s}[1], [x13], x1
        p13 = p13.wrapping_offset(x1);
        ipred_z3_st2_h::<2>(p0, v16v17); // st1 {v18.s}[2], [x0], x1
        p0 = p0.wrapping_offset(x1);
        ipred_z3_st2_h::<3>(p13, v16v17); // st1 {v18.s}[3], [x13], x1
        p13 = p13.wrapping_offset(x1);
        ipred_z3_st2_h::<4>(p0, v16v17); // st1 {v19.s}[0], [x0], x1
        p0 = p0.wrapping_offset(x1);
        ipred_z3_st2_h::<5>(p13, v16v17); // st1 {v19.s}[1], [x13], x1
        p13 = p13.wrapping_offset(x1);
        ipred_z3_st2_h::<6>(p0, v16v17); // st1 {v19.s}[2], [x0], x1
        ipred_z3_st2_h::<7>(p13, v16v17); // st1 {v19.s}[3], [x13], x1
        if w3 <= 0 {
            return; // b.le 9f
        }
        // sub x0, x0, x1, lsl #2: ptr -= 4 * (2*stride); sub x13, x13, x1, lsl #2:
        // back at the top of the column pair (`x0`, `x13`)
        x0 = x0.wrapping_add(4); // add x0, x0, #4
        x13 = x13.wrapping_add(4); // add x13, x13, #4
    }
}

// ============================================================================
// Part D (ipred16.S lines 3902–5711): the filter predictor, the palette predictor and chroma from luma.
// ============================================================================
// Correspondence with the assembly (this part): the vector registers are typed locals named after the
// registers (`v0`, `v16`, ...); a register that the assembly reuses for a value of another width is a new
// binding of the same name. General registers keep their assembly names (`x0`, `w4`, ...). Pixel and
// coefficient pointers are typed (`*mut u16`), the stride stays in bytes (`wrapping_byte_offset`). A
// post-incremented address is an explicit `wrapping_add`/`wrapping_byte_offset` after the access, so that the
// address after the last access is never formed with `add`. Each jump table is a `match`; the arms carry the
// label names. A branch into a block that several entry points share (`L(ipred_cfl_splat_w*)`,
// `L(ipred_cfl_ac_420_w*_hpad)`, `L(ipred_cfl_ac_420_w4_calc_subtract_dc)`) is a call in tail position of the
// function with the label's name.

// --- single instructions without an intrinsic of their own (`pd_*`) ---------------------------------------------

/// `clz w9, w4; sub w9, w9, #25`: the index into a jump table of five widths, 0 for 64 down to 4 for 4.
#[inline(always)]
fn pd_tbl_idx(w: c_int) -> u32 {
    (w as u32).leading_zeros().wrapping_sub(25)
}

/// `subs w, w, #n` together with the `b.gt` (or the negated `b.le`) that follows it: counts a block dimension
/// down and is true while something remains. The dimensions are positive multiples of `n`, so the condition is
/// written as `!= 0`, the form the compiler turns into `subs` and a branch.
#[inline(always)]
fn pd_subs_gt(w: &mut c_int, n: c_int) -> bool {
    *w = w.wrapping_sub(n);
    *w != 0
}

/// `ldr d, [p]` / `ld1 {v.4h}, [p]`: four pixels into the low half of a register, the high half cleared.
#[inline(always)]
unsafe fn pd_ld1_4h(p: *const u16) -> uint16x8_t {
    vcombine_u16(vld1_u16(p), vdup_n_u16(0))
}

/// `ld1 {v.4h}, [lo]` then `ld1 {v.d}[1], [hi]`: two rows of four pixels in one register.
#[inline(always)]
unsafe fn pd_ld1_d2(lo: *const u16, hi: *const u16) -> uint16x8_t {
    vcombine_u16(vld1_u16(lo), vld1_u16(hi))
}

/// `st1 {v.d}[LANE], [p]`
#[inline(always)]
unsafe fn pd_st1q_d<const LANE: i32>(p: *mut u16, a: uint16x8_t) {
    p.cast::<u64>()
        .write_unaligned(vgetq_lane_u64::<LANE>(vreinterpretq_u64_u16(a)));
}

/// `addv h, v.4h`: the sum as lane 0 of a `.4h` register (the other lanes, zero in the assembly, hold the sum
/// too; only lane 0 is used).
#[inline(always)]
unsafe fn pd_addv_4h(a: uint16x4_t) -> uint16x4_t {
    vdup_n_u16(vaddv_u16(a))
}

/// `addv h, v.8h`: as `pd_addv_4h`.
#[inline(always)]
unsafe fn pd_addv_8h(a: uint16x8_t) -> uint16x4_t {
    vdup_n_u16(vaddvq_u16(a))
}

/// `uaddlv s, v.4h`: the sum as lane 0 of a `.4s` register (the other lanes as `pd_addv_4h`).
#[inline(always)]
unsafe fn pd_uaddlv_4h(a: uint16x4_t) -> uint32x4_t {
    vdupq_n_u32(vaddlv_u16(a))
}

/// `uaddlv s, v.8h`: as `pd_uaddlv_4h`.
#[inline(always)]
unsafe fn pd_uaddlv_8h(a: uint16x8_t) -> uint32x4_t {
    vdupq_n_u32(vaddlvq_u16(a))
}

// --- ipred_filter ----------------------------------------------------------------------------------------------

/// The arithmetic of one 4x2 block of `filter_fn`, with its clamp: the first product, the six accumulated
/// products (`tap * register[lane]`, in the order the assembly issues them) and
/// - at 10 bpc `mul`, `mla` x6 on 16 bits, `srshr #4`, `smax` (`v30` = 0), `smin` (`v31` = bitdepth_max);
/// - at 12 bpc `smull`, `smlal` x6 on the low halves, `smull2`, `smlal2` x6 on the high halves, `sqrshrun #4`,
///   `sqrshrun2 #4`, `smin`.
/// The assembly interleaves the lines of consecutive blocks (the head of the next block before the rounding of
/// this one); here a block is one expression.
macro_rules! pd_filter_fn_block {
    ($bpc:expr, $v31:expr; $t0:ident * $r0:ident[$l0:literal] $(, $t:ident * $r:ident[$l:literal])+) => {{
        if $bpc == 10 {
            let mut a = vmulq_laneq_s16::<$l0>($t0, $r0); // mul
            $( a = vmlaq_laneq_s16::<$l>(a, $t, $r); )+ // mla
            let a = vrshrq_n_s16::<4>(a); // srshr #4
            let a = vmaxq_s16(a, vdupq_n_s16(0)); // smax (v30)
            vminq_s16(a, $v31) // smin
        } else {
            let mut a = vmull_laneq_s16::<$l0>(vget_low_s16($t0), $r0); // smull
            $( a = vmlal_laneq_s16::<$l>(a, vget_low_s16($t), $r); )+ // smlal
            let mut b = vmull_high_laneq_s16::<$l0>($t0, $r0); // smull2
            $( b = vmlal_high_laneq_s16::<$l>(b, $t, $r); )+ // smlal2
            let a = vqrshrun_n_s32::<4>(a); // sqrshrun #4
            let a = vqrshrun_high_n_s32::<4>(a, b); // sqrshrun2 #4
            vminq_s16(vreinterpretq_s16_u16(a), $v31) // smin
        }
    }};
}

/// The macro `filter_fn bpc` of ipred16.S: `ipred_filter_\bpc\()bpc_neon`, with `BPC` = 10 or 12 and
/// bitdepth_max in `w8`.
#[inline(always)]
unsafe fn filter_fn<const BPC: u32>(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    filt_idx: c_int,
    w8: c_int,
) {
    let (mut x0, mut x1, mut x2, w3, mut w4) = (dst, stride, topleft, width, height);
    let w5 = (filt_idx & 511) as usize; // and w5, w5, #511
    let x6 = core::ptr::addr_of!(crate::common::tables::dav1d_filter_intra_taps).cast::<i8>(); // movrel x6, X(filter_intra_taps)
    let x6 = x6.add(w5 << 6); // lsl w5, w5, #6; add x6, x6, w5, uxtw
    let t = vld1_s8_x4(x6); // ld1 {v16.8b, v17.8b, v18.8b, v19.8b}, [x6], #32
    let u = vld1_s8_x3(x6.add(32)); // ld1 {v20.8b, v21.8b, v22.8b}, [x6]
    let v16 = vmovl_s8(t.0); // sxtl v16.8h, v16.8b
    let v17 = vmovl_s8(t.1); // sxtl v17.8h, v17.8b
    let v18 = vmovl_s8(t.2); // sxtl v18.8h, v18.8b
    let v19 = vmovl_s8(t.3); // sxtl v19.8h, v19.8b
    let mut x6 = x0.wrapping_byte_offset(x1); // add x6, x0, x1
    x1 <<= 1; // lsl x1, x1, #1
    let v20 = vmovl_s8(u.0); // sxtl v20.8h, v20.8b
    let v21 = vmovl_s8(u.1); // sxtl v21.8h, v21.8b
    let v22 = vmovl_s8(u.2); // sxtl v22.8h, v22.8b
    let v31 = vdupq_n_s16(w8 as i16); // dup v31.8h, w8

    // .if \bpc == 10: movi v30.8h, #0 (in the block macro)

    // br x5 (ipred_filter\bpc\()_tbl, by the width)
    match width {
        4 => {
            // 40:
            let mut v0 = vreinterpretq_s16_u16(pd_ld1_4h(x2.add(1))); // ldur d0, [x2, #2]: top (0-3)
            x2 = x2.sub(2); // sub x2, x2, #4
            loop {
                // 4:
                let v1 = vreinterpretq_s16_u16(pd_ld1_4h(x2)); // ld1 {v1.4h}, [x2], x7: left (0-1) + topleft (2)
                x2 = x2.wrapping_sub(2); // (x7 = -4)

                // p1(top[0]) * filter(1), p2(top[1]) * filter(2), p3(top[2]) * filter(3), p4(top[3]) * filter(4),
                // p0(topleft) * filter(0), p5(left[0]) * filter(5), p6(left[1]) * filter(6)
                let v2 = pd_filter_fn_block!(BPC, v31;
                    v17 * v0[0], v18 * v0[1], v19 * v0[2], v20 * v0[3], v16 * v1[2], v21 * v1[1], v22 * v1[0]);
                let gt = pd_subs_gt(&mut w4, 2); // subs w4, w4, #2
                vst1_s16(x0.cast(), vget_low_s16(v2)); // st1 {v2.d}[0], [x0], x1
                x0 = x0.wrapping_byte_offset(x1);
                v0 = vextq_s16::<4>(v2, v2); // ext v0.16b, v2.16b, v2.16b, #8: move top from [4-7] to [0-3]
                vst1_s16(x6.cast(), vget_low_s16(v0)); // st1 {v2.d}[1], [x6], x1
                x6 = x6.wrapping_byte_offset(x1);
                if !gt {
                    break; // b.gt 4b
                }
            }
        }
        8 => {
            // 80:
            let mut v0 = vld1q_s16(x2.add(1).cast()); // ldur q0, [x2, #2]: top (0-7)
            x2 = x2.sub(2); // sub x2, x2, #4
            loop {
                // 8:
                let v1 = vreinterpretq_s16_u16(pd_ld1_4h(x2)); // ld1 {v1.4h}, [x2], x7: left (0-1) + topleft (2)
                x2 = x2.wrapping_sub(2); // (x7 = -4)
                let v2 = pd_filter_fn_block!(BPC, v31;
                    v17 * v0[0], v18 * v0[1], v19 * v0[2], v20 * v0[3], v16 * v1[2], v21 * v1[1], v22 * v1[0]);
                let v3 = pd_filter_fn_block!(BPC, v31;
                    v17 * v0[4], v18 * v0[5], v19 * v0[6], v20 * v0[7], v16 * v0[3], v21 * v2[3], v22 * v2[7]);
                let gt = pd_subs_gt(&mut w4, 2); // subs w4, w4, #2
                let v23 = uint64x2x2_t(vreinterpretq_u64_s16(v2), vreinterpretq_u64_s16(v3));
                vst2q_lane_u64::<0>(x0.cast(), v23); // st2 {v2.d, v3.d}[0], [x0], x1
                x0 = x0.wrapping_byte_offset(x1);
                v0 = vreinterpretq_s16_u64(vzip2q_u64(v23.0, v23.1)); // zip2 v0.2d, v2.2d, v3.2d
                vst2q_lane_u64::<1>(x6.cast(), v23); // st2 {v2.d, v3.d}[1], [x6], x1
                x6 = x6.wrapping_byte_offset(x1);
                if !gt {
                    break; // b.gt 8b
                }
            }
        }
        _ => ipred_filter_w16_neon::<BPC>(x0, x1, x2, w3, w4, x6, v16, v17, v18, v19, v20, v21, v22, v31), // 160: 320:
    }
}

/// The blocks `160:`/`320:` of `filter_fn bpc` (w 16 and 32), with the registers of `filter_fn`: `x1` twice the
/// stride, `x6` = `x0` + stride, `v16`-`v22` the taps, `v31` bitdepth_max. A function of its own so that the
/// registers this path keeps across its loops (callee-saved ones) are not saved on the small paths.
#[inline(never)]
unsafe fn ipred_filter_w16_neon<const BPC: u32>(
    mut x0: *mut u16,
    mut x1: isize,
    mut x2: *const u16,
    mut w3: c_int,
    mut w4: c_int,
    mut x6: *mut u16,
    v16: int16x8_t,
    v17: int16x8_t,
    v18: int16x8_t,
    v19: int16x8_t,
    v20: int16x8_t,
    v21: int16x8_t,
    v22: int16x8_t,
    v31: int16x8_t,
) {
    let mut x8 = x2.add(1); // add x8, x2, #2
    x2 = x2.sub(2); // sub x2, x2, #4
    x1 -= (w3 as isize) << 1; // sub x1, x1, w3, uxtw #1
    let w9 = w3; // mov w9, w3
    loop {
        // 1:
        let mut v0 = vreinterpretq_s16_u16(pd_ld1_4h(x2)); // ld1 {v0.4h}, [x2], x7: left (0-1) + topleft (2)
        x2 = x2.wrapping_sub(2); // (x7 = -4)
        loop {
            // 2:
            let v12 = vld1q_s16_x2(x8.cast()); // ld1 {v1.8h, v2.8h}, [x8], #32: top(0-15)
            x8 = x8.wrapping_add(16);
            let (v1, v2) = (v12.0, v12.1);
            // p0(topleft) * filter(0), p5(left[0]) * filter(5), p6(left[1]) * filter(6),
            // p1(top[0]) * filter(1), p2(top[1]) * filter(2), p3(top[2]) * filter(3), p4(top[3]) * filter(4)
            let v3 = pd_filter_fn_block!(BPC, v31;
                v16 * v0[2], v21 * v0[1], v22 * v0[0], v17 * v1[0], v18 * v1[1], v19 * v1[2], v20 * v1[3]);
            let v4 = pd_filter_fn_block!(BPC, v31;
                v17 * v1[4], v18 * v1[5], v19 * v1[6], v20 * v1[7], v16 * v1[3], v21 * v3[3], v22 * v3[7]);
            let v5 = pd_filter_fn_block!(BPC, v31;
                v17 * v2[0], v18 * v2[1], v19 * v2[2], v20 * v2[3], v16 * v1[7], v21 * v4[3], v22 * v4[7]);
            let v6 = pd_filter_fn_block!(BPC, v31;
                v17 * v2[4], v18 * v2[5], v19 * v2[6], v20 * v2[7], v16 * v2[3], v21 * v5[3], v22 * v5[7]);
            let gt = pd_subs_gt(&mut w3, 16); // subs w3, w3, #16

            v0 = vcopyq_laneq_s16::<2, 7>(v0, v2); // ins v0.h[2], v2.h[7]
            let v3456 = uint64x2x4_t(
                vreinterpretq_u64_s16(v3),
                vreinterpretq_u64_s16(v4),
                vreinterpretq_u64_s16(v5),
                vreinterpretq_u64_s16(v6),
            );
            vst4q_lane_u64::<0>(x0.cast(), v3456); // st4 {v3.d, v4.d, v5.d, v6.d}[0], [x0], #32
            x0 = x0.wrapping_add(16);
            v0 = vcopyq_laneq_s16::<0, 7>(v0, v6); // ins v0.h[0], v6.h[7]
            vst4q_lane_u64::<1>(x6.cast(), v3456); // st4 {v3.d, v4.d, v5.d, v6.d}[1], [x6], #32
            x6 = x6.wrapping_add(16);
            v0 = vcopyq_laneq_s16::<1, 3>(v0, v6); // ins v0.h[1], v6.h[3]
            if !gt {
                break; // b.gt 2b
            }
        }
        let gt = pd_subs_gt(&mut w4, 2); // subs w4, w4, #2
        if !gt {
            break; // b.le 9f
        }
        x8 = x6.sub(w9 as usize).cast_const(); // sub x8, x6, w9, uxtw #1
        x0 = x0.wrapping_byte_offset(x1); // add x0, x0, x1
        x6 = x6.wrapping_byte_offset(x1); // add x6, x6, x1
        w3 = w9; // mov w3, w9

        // b 1b
    }
    // 9:
}

/// `ipred_filter_10bpc_neon` (`filter_fn 10`).
#[inline(never)]
unsafe fn ipred_filter_10bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    filt_idx: c_int,
    w8: c_int,
) {
    filter_fn::<10>(dst, stride, topleft, width, height, filt_idx, w8)
}

/// `ipred_filter_12bpc_neon` (`filter_fn 12`).
#[inline(never)]
unsafe fn ipred_filter_12bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    filt_idx: c_int,
    w8: c_int,
) {
    filter_fn::<12>(dst, stride, topleft, width, height, filt_idx, w8)
}

/// `dav1d_ipred_filter_16bpc_neon`: the recursive filter predictor; branches to the 10 or the 12 bit instance
/// of `filter_fn` by `bitdepth_max`.
///
/// Safety: reads 56 bytes of `dav1d_filter_intra_taps[filt_idx & 511]` (`filt_idx & 511 < 5` by the caller's
/// contract), the edge pixels `topleft[-height .. width + 1]` (w 4 reads `topleft[1..5]` and four pixels at
/// `topleft - 2 - 2k` for each row pair `k`; w 8 the same with `topleft[1..9]`; w 16/32 `topleft[1 .. width + 1]`
/// in steps of 16 and the same four pixels per row pair) and, for w 16/32, the row `2k + 1` of `dst` it wrote
/// before; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_filter_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    filt_idx: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
) {
    let w8 = bitdepth_max; // ldr w8, [sp]
    if w8 <= 0x3ff {
        // cmp w8, 0x3ff; b.le ipred_filter_10bpc_neon
        ipred_filter_10bpc_neon(dst, stride, topleft, width, height, filt_idx, w8)
    } else {
        ipred_filter_12bpc_neon(dst, stride, topleft, width, height, filt_idx, w8)
        // b ipred_filter_12bpc_neon
    }
}

// --- pal_pred --------------------------------------------------------------------------------------------------

/// The index arithmetic of `pal_pred_16bpc_neon` for one register of interleaved indices `a, b, c, ...`
/// (`v`): `add v.16b, v.16b, v.16b` (the byte offsets `2*a, 2*b, ...`), `zip1`/`zip2` of the register with
/// itself (`2*a, 2*a, 2*b, 2*b, ...`), `add .8h` of `v31` = 0x0100 (`2*a, 2*a+1, ...`) and the two `tbl` on the
/// palette `v30`: sixteen pixels in two registers.
#[inline(always)]
unsafe fn pal_pred_tbl2(v: uint8x16_t, v30: uint8x16_t, v31: uint16x8_t) -> (uint16x8_t, uint16x8_t) {
    let v = vaddq_u8(v, v); // add v.16b, v.16b, v.16b
    let v0 = vreinterpretq_u16_u8(vzip1q_u8(v, v)); // zip1 v0.16b, v.16b, v.16b
    let v1 = vreinterpretq_u16_u8(vzip2q_u8(v, v)); // zip2 v1.16b, v.16b, v.16b
    let v0 = vaddq_u16(v0, v31); // add v0.8h, v0.8h, v31.8h
    let v1 = vaddq_u16(v1, v31); // add v1.8h, v1.8h, v31.8h
    let v0 = vqtbl1q_u8(v30, vreinterpretq_u8_u16(v0)); // tbl v0.16b, {v30.16b}, v0.16b
    let v1 = vqtbl1q_u8(v30, vreinterpretq_u8_u16(v1)); // tbl v1.16b, {v30.16b}, v1.16b
    (vreinterpretq_u16_u8(v0), vreinterpretq_u16_u8(v1))
}

/// The head of the blocks `16:`, `32:` and `64:` of `pal_pred_16bpc_neon`: 32 bytes of packed indices into
/// four registers of interleaved indices (`ushr #4`, `and` with `v29` = 7, `zip1`/`zip2`).
#[inline(always)]
unsafe fn pal_pred_idx32(x3: *const u8, v29: uint8x16_t) -> [uint8x16_t; 4] {
    let v45 = vld1q_u8_x2(x3); // ld1 {v4.16b, v5.16b}, [x3], #32
    let v7 = vshrq_n_u8::<4>(v45.0); // ushr v7.16b, v4.16b, #4
    let v6 = vandq_u8(v45.0, v29); // and v6.16b, v4.16b, v29.16b
    let v3 = vshrq_n_u8::<4>(v45.1); // ushr v3.16b, v5.16b, #4
    let v2 = vandq_u8(v45.1, v29); // and v2.16b, v5.16b, v29.16b
    let v4 = vzip1q_u8(v6, v7); // zip1 v4.16b, v6.16b, v7.16b
    let v5 = vzip2q_u8(v6, v7); // zip2 v5.16b, v6.16b, v7.16b
    let v6 = vzip1q_u8(v2, v3); // zip1 v6.16b, v2.16b, v3.16b
    let v7 = vzip2q_u8(v2, v3); // zip2 v7.16b, v2.16b, v3.16b
    [v4, v5, v6, v7]
}

/// `dav1d_pal_pred_16bpc_neon`: the palette predictor.
///
/// Safety: reads `pal[0..8]` (16 bytes) and exactly the `w * h / 2` bytes of the packed index buffer `idx` (two
/// 3-bit indices per byte, the low nibble first), in order and without alignment requirement: 8 (w 4) or 16
/// (w 8) or 32 (w 16) bytes per four rows, 32 bytes per two rows at w 32 and per row at w 64; writes `dst` rows
/// `0..h`, columns `0..w`.
pub unsafe extern "C" fn dav1d_pal_pred_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    pal: *const u16,
    idx: *const u8,
    w: c_int,
    h: c_int,
) {
    let (mut x0, mut x1, mut x3, mut w5) = (dst, stride, idx, h);
    let v30 = vreinterpretq_u8_u16(vld1q_u16(pal)); // ld1 {v30.8h}, [x2]
    let v29 = vdupq_n_u8(7); // movi v29.16b, #7
    let v31 = vdupq_n_u16(0x100); // movi v31.8h, #1, lsl #8

    // br x6 (pal_pred_tbl, by the width)
    match pd_tbl_idx(w) {
        4 => {
            // 40:
            let mut x2 = x0.wrapping_byte_offset(x1); // add x2, x0, x1
            x1 <<= 1; // lsl x1, x1, #1
            loop {
                // 4:
                let v1 = vld1_u8(x3); // ld1 {v1.8b}, [x3], #8
                x3 = x3.wrapping_add(8);
                let gt = pd_subs_gt(&mut w5, 4); // subs w5, w5, #4
                let v3 = vshr_n_u8::<4>(v1); // ushr v3.8b, v1.8b, #4
                let v2 = vand_u8(v1, vget_low_u8(v29)); // and v2.8b, v1.8b, v29.8b
                let v1 = vzip1q_u8(vcombine_u8(v2, vdup_n_u8(0)), vcombine_u8(v3, vdup_n_u8(0))); // zip1 v1.16b, v2.16b, v3.16b

                // Restructure v1 from a, b, c, ... into 2*a, 2*a+1, 2*b, 2*b+1, 2*c, 2*c+1, ...
                let (v0, v1) = pal_pred_tbl2(v1, v30, v31);
                pd_st1q_d::<0>(x0, v0); // st1 {v0.d}[0], [x0], x1
                x0 = x0.wrapping_byte_offset(x1);
                pd_st1q_d::<1>(x2, v0); // st1 {v0.d}[1], [x2], x1
                x2 = x2.wrapping_byte_offset(x1);
                pd_st1q_d::<0>(x0, v1); // st1 {v1.d}[0], [x0], x1
                x0 = x0.wrapping_byte_offset(x1);
                pd_st1q_d::<1>(x2, v1); // st1 {v1.d}[1], [x2], x1
                x2 = x2.wrapping_byte_offset(x1);
                if !gt {
                    break; // b.gt 4b
                }
            }
        }
        3 => {
            // 80:
            let mut x2 = x0.wrapping_byte_offset(x1); // add x2, x0, x1
            x1 <<= 1; // lsl x1, x1, #1
            loop {
                // 8:
                let v2 = vld1q_u8(x3); // ld1 {v2.16b}, [x3], #16
                x3 = x3.wrapping_add(16);
                let gt = pd_subs_gt(&mut w5, 4); // subs w5, w5, #4
                let v4 = vshrq_n_u8::<4>(v2); // ushr v4.16b, v2.16b, #4
                let v3 = vandq_u8(v2, v29); // and v3.16b, v2.16b, v29.16b
                let v2 = vzip1q_u8(v3, v4); // zip1 v2.16b, v3.16b, v4.16b
                let v3 = vzip2q_u8(v3, v4); // zip2 v3.16b, v3.16b, v4.16b
                let (v0, v1) = pal_pred_tbl2(v2, v30, v31);
                let (v2, v3) = pal_pred_tbl2(v3, v30, v31);
                vst1q_u16(x0, v0); // st1 {v0.8h}, [x0], x1
                x0 = x0.wrapping_byte_offset(x1);
                vst1q_u16(x2, v1); // st1 {v1.8h}, [x2], x1
                x2 = x2.wrapping_byte_offset(x1);
                vst1q_u16(x0, v2); // st1 {v2.8h}, [x0], x1
                x0 = x0.wrapping_byte_offset(x1);
                vst1q_u16(x2, v3); // st1 {v3.8h}, [x2], x1
                x2 = x2.wrapping_byte_offset(x1);
                if !gt {
                    break; // b.gt 8b
                }
            }
        }
        2 => {
            // 160:
            let mut x2 = x0.wrapping_byte_offset(x1); // add x2, x0, x1
            x1 <<= 1; // lsl x1, x1, #1
            loop {
                // 16:
                let [v4, v5, v6, v7] = pal_pred_idx32(x3, v29);
                x3 = x3.wrapping_add(32);
                let gt = pd_subs_gt(&mut w5, 4); // subs w5, w5, #4
                let (v0, v1) = pal_pred_tbl2(v4, v30, v31);
                let (v2, v3) = pal_pred_tbl2(v5, v30, v31);
                let (v4, v5) = pal_pred_tbl2(v6, v30, v31);
                let (v6, v7) = pal_pred_tbl2(v7, v30, v31);
                vst1q_u16_x2(x0, uint16x8x2_t(v0, v1)); // st1 {v0.8h, v1.8h}, [x0], x1
                x0 = x0.wrapping_byte_offset(x1);
                vst1q_u16_x2(x2, uint16x8x2_t(v2, v3)); // st1 {v2.8h, v3.8h}, [x2], x1
                x2 = x2.wrapping_byte_offset(x1);
                vst1q_u16_x2(x0, uint16x8x2_t(v4, v5)); // st1 {v4.8h, v5.8h}, [x0], x1
                x0 = x0.wrapping_byte_offset(x1);
                vst1q_u16_x2(x2, uint16x8x2_t(v6, v7)); // st1 {v6.8h, v7.8h}, [x2], x1
                x2 = x2.wrapping_byte_offset(x1);
                if !gt {
                    break; // b.gt 16b
                }
            }
        }
        1 => {
            // 320:
            let mut x2 = x0.wrapping_byte_offset(x1); // add x2, x0, x1
            x1 <<= 1; // lsl x1, x1, #1
            loop {
                // 32:
                let [v4, v5, v6, v7] = pal_pred_idx32(x3, v29);
                x3 = x3.wrapping_add(32);
                let gt = pd_subs_gt(&mut w5, 2); // subs w5, w5, #2
                let (v0, v1) = pal_pred_tbl2(v4, v30, v31);
                let (v2, v3) = pal_pred_tbl2(v5, v30, v31);
                let (v4, v5) = pal_pred_tbl2(v6, v30, v31);
                let (v6, v7) = pal_pred_tbl2(v7, v30, v31);
                vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
                x0 = x0.wrapping_byte_offset(x1);
                vst1q_u16_x4(x2, uint16x8x4_t(v4, v5, v6, v7)); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x2], x1
                x2 = x2.wrapping_byte_offset(x1);
                if !gt {
                    break; // b.gt 32b
                }
            }
        }
        0 => {
            // 640:
            let mut x2 = x0.wrapping_add(32); // add x2, x0, #64
            loop {
                // 64:
                let [v4, v5, v6, v7] = pal_pred_idx32(x3, v29);
                x3 = x3.wrapping_add(32);
                let gt = pd_subs_gt(&mut w5, 1); // subs w5, w5, #1
                let (v0, v1) = pal_pred_tbl2(v4, v30, v31);
                let (v2, v3) = pal_pred_tbl2(v5, v30, v31);
                let (v4, v5) = pal_pred_tbl2(v6, v30, v31);
                let (v6, v7) = pal_pred_tbl2(v7, v30, v31);
                vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], x1
                x0 = x0.wrapping_byte_offset(x1);
                vst1q_u16_x4(x2, uint16x8x4_t(v4, v5, v6, v7)); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x2], x1
                x2 = x2.wrapping_byte_offset(x1);
                if !gt {
                    break; // b.gt 64b
                }
            }
        }
        // SAFETY: the width is 4, 8, 16, 32 or 64 (the assembly indexes its five-entry table with it).
        _ => core::hint::unreachable_unchecked(),
    }
}

// --- ipred_cfl -------------------------------------------------------------------------------------------------

/// One register of the splat loops: `smull`, `smull2` (diff = ac * alpha), `cmlt #0` and `add` on both halves
/// (diff + sign), `rshrn #6`, `rshrn2 #6` ((diff + sign + 32) >> 6 = apply_sign()), `add` (dc), `smax` with
/// `v30` = 0 and `smin` with `v31` = bitdepth_max. The assembly issues these instruction by instruction over
/// its two or four registers; here a register is one expression.
#[inline(always)]
unsafe fn ipred_cfl_splat_px(ac: int16x8_t, v0: int16x8_t, v1: int16x8_t, v31: int16x8_t) -> uint16x8_t {
    let v2 = vmull_s16(vget_low_s16(ac), vget_low_s16(v1)); // smull v2.4s, v4.4h, v1.4h: diff = ac * alpha
    let v3 = vmull_high_s16(ac, v1); // smull2 v3.4s, v4.8h, v1.8h
    let v16 = vreinterpretq_s32_u32(vcltzq_s32(v2)); // cmlt v16.4s, v2.4s, #0: sign
    let v17 = vreinterpretq_s32_u32(vcltzq_s32(v3)); // cmlt v17.4s, v3.4s, #0
    let v2 = vaddq_s32(v2, v16); // add v2.4s, v2.4s, v16.4s: diff + sign
    let v3 = vaddq_s32(v3, v17); // add v3.4s, v3.4s, v17.4s
    let v2 = vrshrn_n_s32::<6>(v2); // rshrn v2.4h, v2.4s, #6: (diff + sign + 32) >> 6 = apply_sign()
    let v2 = vrshrn_high_n_s32::<6>(v2, v3); // rshrn2 v2.8h, v3.4s, #6
    let v2 = vaddq_s16(v2, v0); // add v2.8h, v2.8h, v0.8h: dc + apply_sign()
    let v2 = vmaxq_s16(v2, vdupq_n_s16(0)); // smax v2.8h, v2.8h, v30.8h
    let v2 = vminq_s16(v2, v31); // smin v2.8h, v2.8h, v31.8h
    vreinterpretq_u16_s16(v2)
}

/// `L(ipred_cfl_splat_w4)` of `ipred_cfl_128_16bpc_neon`; `v0` is the dc, `v1` alpha, `v31` bitdepth_max, `x6`
/// = `x0` + stride and `x1` twice the stride (`v30` = 0 is formed in `ipred_cfl_splat_px`).
#[inline(never)]
unsafe fn ipred_cfl_splat_w4_16bpc_neon(
    mut x0: *mut u16,
    x1: isize,
    mut x6: *mut u16,
    mut x5: *const i16,
    mut w4: c_int,
    v0: int16x8_t,
    v1: int16x8_t,
    v31: int16x8_t,
) {
    loop {
        // 1:
        let v45 = vld1q_s16_x2(x5); // ld1 {v4.8h, v5.8h}, [x5], #32
        x5 = x5.wrapping_add(16);
        let gt = pd_subs_gt(&mut w4, 4); // subs w4, w4, #4
        let v2 = ipred_cfl_splat_px(v45.0, v0, v1, v31);
        let v3 = ipred_cfl_splat_px(v45.1, v0, v1, v31);
        pd_st1q_d::<0>(x0, v2); // st1 {v2.d}[0], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        pd_st1q_d::<1>(x6, v2); // st1 {v2.d}[1], [x6], x1
        x6 = x6.wrapping_byte_offset(x1);
        pd_st1q_d::<0>(x0, v3); // st1 {v3.d}[0], [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        pd_st1q_d::<1>(x6, v3); // st1 {v3.d}[1], [x6], x1
        x6 = x6.wrapping_byte_offset(x1);
        if !gt {
            break; // b.gt 1b
        }
    }
}

/// `L(ipred_cfl_splat_w8)` of `ipred_cfl_128_16bpc_neon`.
#[inline(never)]
unsafe fn ipred_cfl_splat_w8_16bpc_neon(
    mut x0: *mut u16,
    x1: isize,
    mut x6: *mut u16,
    mut x5: *const i16,
    mut w4: c_int,
    v0: int16x8_t,
    v1: int16x8_t,
    v31: int16x8_t,
) {
    loop {
        // 1:
        let v45 = vld1q_s16_x2(x5); // ld1 {v4.8h, v5.8h}, [x5], #32
        x5 = x5.wrapping_add(16);
        let gt = pd_subs_gt(&mut w4, 2); // subs w4, w4, #2
        let v2 = ipred_cfl_splat_px(v45.0, v0, v1, v31);
        let v3 = ipred_cfl_splat_px(v45.1, v0, v1, v31);
        vst1q_u16(x0, v2); // st1 {v2.8h}, [x0], x1
        x0 = x0.wrapping_byte_offset(x1);
        vst1q_u16(x6, v3); // st1 {v3.8h}, [x6], x1
        x6 = x6.wrapping_byte_offset(x1);
        if !gt {
            break; // b.gt 1b
        }
    }
}

/// `L(ipred_cfl_splat_w16)` of `ipred_cfl_128_16bpc_neon` (w 16 and w 32).
#[inline(never)]
unsafe fn ipred_cfl_splat_w16_16bpc_neon(
    mut x0: *mut u16,
    mut x1: isize,
    mut x6: *mut u16,
    mut w3: c_int,
    mut w4: c_int,
    mut x5: *const i16,
    v0: int16x8_t,
    v1: int16x8_t,
    v31: int16x8_t,
) {
    let mut x7 = x5.add(w3 as usize); // add x7, x5, w3, uxtw #1
    x1 -= (w3 as isize) << 1; // sub x1, x1, w3, uxtw #1
    let w9 = w3; // mov w9, w3
    loop {
        // 1:
        let v23 = vld1q_s16_x2(x5); // ld1 {v2.8h, v3.8h}, [x5], #32
        x5 = x5.wrapping_add(16);
        let v45 = vld1q_s16_x2(x7); // ld1 {v4.8h, v5.8h}, [x7], #32
        x7 = x7.wrapping_add(16);
        let gt = pd_subs_gt(&mut w3, 16); // subs w3, w3, #16
        let v2 = ipred_cfl_splat_px(v23.0, v0, v1, v31);
        let v3 = ipred_cfl_splat_px(v23.1, v0, v1, v31);
        let v4 = ipred_cfl_splat_px(v45.0, v0, v1, v31);
        let v5 = ipred_cfl_splat_px(v45.1, v0, v1, v31);
        vst1q_u16_x2(x0, uint16x8x2_t(v2, v3)); // st1 {v2.8h, v3.8h}, [x0], #32
        x0 = x0.wrapping_add(16);
        vst1q_u16_x2(x6, uint16x8x2_t(v4, v5)); // st1 {v4.8h, v5.8h}, [x6], #32
        x6 = x6.wrapping_add(16);
        if gt {
            continue; // b.gt 1b
        }
        let gt = pd_subs_gt(&mut w4, 2); // subs w4, w4, #2
        x5 = x5.wrapping_add(w9 as usize); // add x5, x5, w9, uxtw #1
        x7 = x7.wrapping_add(w9 as usize); // add x7, x7, w9, uxtw #1
        x0 = x0.wrapping_byte_offset(x1); // add x0, x0, x1
        x6 = x6.wrapping_byte_offset(x1); // add x6, x6, x1
        w3 = w9; // mov w3, w9
        if !gt {
            break; // b.gt 1b
        }
    }
}

/// `br x9` with `ipred_cfl_splat_tbl` (`ipred_cfl_128_tbl`), by the width.
#[inline(always)]
unsafe fn ipred_cfl_splat_tbl_16bpc(
    x0: *mut u16,
    x1: isize,
    x6: *mut u16,
    w3: c_int,
    w4: c_int,
    x5: *const i16,
    v0: int16x8_t,
    v1: int16x8_t,
    v31: int16x8_t,
) {
    match w3 {
        4 => ipred_cfl_splat_w4_16bpc_neon(x0, x1, x6, x5, w4, v0, v1, v31),
        8 => ipred_cfl_splat_w8_16bpc_neon(x0, x1, x6, x5, w4, v0, v1, v31),
        _ => ipred_cfl_splat_w16_16bpc_neon(x0, x1, x6, w3, w4, x5, v0, v1, v31),
    }
}

/// `dav1d_ipred_cfl_128_16bpc_neon`: chroma from luma with the dc `(bitdepth_max + 1) >> 1`.
///
/// Safety (all four `ipred_cfl*` entry points): reads `ac[0 .. width * height]` and writes `dst` rows
/// `0..height`, columns `0..width`; this one reads nothing of the edge.
pub unsafe extern "C" fn dav1d_ipred_cfl_128_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    _topleft: *const u16,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    bitdepth_max: c_int,
) {
    let v31 = vdupq_n_s16(bitdepth_max as i16); // dup v31.8h, w7: bitdepth_max
    let v0 = vreinterpretq_s16_u16(vrshrq_n_u16::<1>(vreinterpretq_u16_s16(v31))); // urshr v0.8h, v31.8h, #1
    let v1 = vdupq_n_s16(alpha as i16); // dup v1.8h, w6: alpha
    let x6 = dst.wrapping_byte_offset(stride); // add x6, x0, x1
    let x1 = stride << 1; // lsl x1, x1, #1
    ipred_cfl_splat_tbl_16bpc(dst, x1, x6, width, height, ac, v0, v1, v31) // movi v30.8h, #0; br x7
}

/// `dav1d_ipred_cfl_top_16bpc_neon`: chroma from luma with the dc of the top edge.
///
/// Safety: as `dav1d_ipred_cfl_128_16bpc_neon`, and reads `topleft[1 .. width + 1]`.
pub unsafe extern "C" fn dav1d_ipred_cfl_top_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    bitdepth_max: c_int,
) {
    let v31 = vdupq_n_s16(bitdepth_max as i16); // dup v31.8h, w7: bitdepth_max
    let v1 = vdupq_n_s16(alpha as i16); // dup v1.8h, w6: alpha
    let x2 = topleft.add(1); // add x2, x2, #2
    let x6 = dst.wrapping_byte_offset(stride); // add x6, x0, x1
    let x1 = stride << 1; // lsl x1, x1, #1

    // movi v30.8h, #0; br x7 (ipred_cfl_top_tbl, by the width)
    match width {
        4 => {
            // 4:
            let v0 = vld1_u16(x2); // ld1 {v0.4h}, [x2]
            let v0 = pd_addv_4h(v0); // addv h0, v0.4h
            let v0 = vrshr_n_u16::<2>(v0); // urshr v0.4h, v0.4h, #2
            let v0 = vreinterpretq_s16_u16(vdupq_lane_u16::<0>(v0)); // dup v0.8h, v0.h[0]
            ipred_cfl_splat_w4_16bpc_neon(dst, x1, x6, ac, height, v0, v1, v31) // b L(ipred_cfl_splat_w4)
        }
        8 => {
            // 8:
            let v0 = vld1q_u16(x2); // ld1 {v0.8h}, [x2]
            let v0 = pd_addv_8h(v0); // addv h0, v0.8h
            let v0 = vrshr_n_u16::<3>(v0); // urshr v0.4h, v0.4h, #3
            let v0 = vreinterpretq_s16_u16(vdupq_lane_u16::<0>(v0)); // dup v0.8h, v0.h[0]
            ipred_cfl_splat_w8_16bpc_neon(dst, x1, x6, ac, height, v0, v1, v31) // b L(ipred_cfl_splat_w8)
        }
        16 => {
            // 16:
            let v23 = vld1q_u16_x2(x2); // ld1 {v2.8h, v3.8h}, [x2]
            let v0 = vpaddq_u16(v23.0, v23.1); // addp v0.8h, v2.8h, v3.8h
            let v0 = pd_addv_8h(v0); // addv h0, v0.8h
            let v0 = vrshr_n_u16::<4>(v0); // urshr v0.4h, v0.4h, #4
            let v0 = vreinterpretq_s16_u16(vdupq_lane_u16::<0>(v0)); // dup v0.8h, v0.h[0]
            ipred_cfl_splat_w16_16bpc_neon(dst, x1, x6, width, height, ac, v0, v1, v31)
            // b L(ipred_cfl_splat_w16)
        }
        _ => {
            // 32:
            let t = vld1q_u16_x4(x2); // ld1 {v2.8h, v3.8h, v4.8h, v5.8h}, [x2]
            let v2 = vpaddq_u16(t.0, t.1); // addp v2.8h, v2.8h, v3.8h
            let v4 = vpaddq_u16(t.2, t.3); // addp v4.8h, v4.8h, v5.8h
            let v0 = vpaddq_u16(v2, v4); // addp v0.8h, v2.8h, v4.8h
            let v0 = pd_uaddlv_8h(v0); // uaddlv s0, v0.8h
            let v0 = vrshrn_n_u32::<5>(v0); // rshrn v0.4h, v0.4s, #5
            let v0 = vreinterpretq_s16_u16(vdupq_lane_u16::<0>(v0)); // dup v0.8h, v0.h[0]
            ipred_cfl_splat_w16_16bpc_neon(dst, x1, x6, width, height, ac, v0, v1, v31)
            // b L(ipred_cfl_splat_w16)
        }
    }
}

/// `dav1d_ipred_cfl_left_16bpc_neon`: chroma from luma with the dc of the left edge.
///
/// Safety: as `dav1d_ipred_cfl_128_16bpc_neon`, and reads `topleft[-height .. 0]`.
pub unsafe extern "C" fn dav1d_ipred_cfl_left_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    bitdepth_max: c_int,
) {
    let v31 = vdupq_n_s16(bitdepth_max as i16); // dup v31.8h, w7: bitdepth_max
    let x2 = topleft.sub(height as usize); // sub x2, x2, w4, uxtw #1
    let v1 = vdupq_n_s16(alpha as i16); // dup v1.8h, w6: alpha
    let x6 = dst.wrapping_byte_offset(stride); // add x6, x0, x1
    let x1 = stride << 1; // lsl x1, x1, #1

    // movi v30.8h, #0; br x7 (ipred_cfl_left_tbl, by the height)
    let v0 = match height {
        4 => {
            // L(ipred_cfl_left_h4):
            let v0 = vld1_u16(x2); // ld1 {v0.4h}, [x2]
            let v0 = pd_addv_4h(v0); // addv h0, v0.4h
            vrshr_n_u16::<2>(v0) // urshr v0.4h, v0.4h, #2
        }
        8 => {
            // L(ipred_cfl_left_h8):
            let v0 = vld1q_u16(x2); // ld1 {v0.8h}, [x2]
            let v0 = pd_addv_8h(v0); // addv h0, v0.8h
            vrshr_n_u16::<3>(v0) // urshr v0.4h, v0.4h, #3
        }
        16 => {
            // L(ipred_cfl_left_h16):
            let v23 = vld1q_u16_x2(x2); // ld1 {v2.8h, v3.8h}, [x2]
            let v0 = vpaddq_u16(v23.0, v23.1); // addp v0.8h, v2.8h, v3.8h
            let v0 = pd_addv_8h(v0); // addv h0, v0.8h
            vrshr_n_u16::<4>(v0) // urshr v0.4h, v0.4h, #4
        }
        _ => {
            // L(ipred_cfl_left_h32):
            let t = vld1q_u16_x4(x2); // ld1 {v2.8h, v3.8h, v4.8h, v5.8h}, [x2]
            let v2 = vpaddq_u16(t.0, t.1); // addp v2.8h, v2.8h, v3.8h
            let v4 = vpaddq_u16(t.2, t.3); // addp v4.8h, v4.8h, v5.8h
            let v0 = vpaddq_u16(v2, v4); // addp v0.8h, v2.8h, v4.8h
            let v0 = pd_uaddlv_8h(v0); // uaddlv s0, v0.8h
            vrshrn_n_u32::<5>(v0) // rshrn v0.4h, v0.4s, #5
        }
    };
    let v0 = vreinterpretq_s16_u16(vdupq_lane_u16::<0>(v0)); // dup v0.8h, v0.h[0] (each block)
    ipred_cfl_splat_tbl_16bpc(dst, x1, x6, width, height, ac, v0, v1, v31) // br x9
}

/// The tail of the four `L(ipred_cfl_w*)` blocks after `ushl`: for a rectangular block the division by 3 or 5
/// (`mul` with 0xAAAB or 0x6667 (`five`), `ushr #17`), then `dup v0.8h, v0.h[0]`.
#[inline(always)]
unsafe fn ipred_cfl_dc_mul(v0: uint32x2_t, square: bool, five: bool) -> int16x8_t {
    let mut v0 = v0;
    if !square {
        // cmp w4, #w; b.eq 1f
        let w16: u32 = if five { 0x6667 } else { 0xAAAB }; // mov w16, #0x6667; mov w17, #0xAAAB; csel w16, w16, w17, eq
        let v16 = vdup_n_u32(w16); // dup v16.2s, w16
        v0 = vmul_u32(v0, v16); // mul v0.2s, v0.2s, v16.2s
        v0 = vshr_n_u32::<17>(v0); // ushr v0.2s, v0.2s, #17
    }
    // 1:
    vreinterpretq_s16_u16(vdupq_lane_u16::<0>(vreinterpret_u16_u32(v0))) // dup v0.8h, v0.h[0]
}

/// `dav1d_ipred_cfl_16bpc_neon`: chroma from luma with the dc of the left and top edges.
///
/// Safety: as `dav1d_ipred_cfl_128_16bpc_neon`, and reads `topleft[-height .. 0]` and
/// `topleft[1 .. width + 1]`.
pub unsafe extern "C" fn dav1d_ipred_cfl_16bpc_neon(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    bitdepth_max: c_int,
) {
    let (w3, w4) = (width, height);
    let v31 = vdupq_n_s16(bitdepth_max as i16); // dup v31.8h, w7: bitdepth_max
    let x2 = topleft.sub(w4 as usize); // sub x2, x2, w4, uxtw #1
    let w8 = (w3 + w4) as u32; // add w8, w3, w4: width + height
    let v1 = vdupq_n_s16(alpha as i16); // dup v1.8h, w6: alpha
    let v16 = vdup_n_u32(w8); // dup v16.4s, w8: width + height
    let w8 = w8.trailing_zeros() as i32; // rbit w8, w8; clz w8, w8: ctz(width + height)
    let w8 = -w8; // neg w8, w8: -ctz(width + height)
    let v16 = vshr_n_u32::<1>(v16); // ushr v16.4s, v16.4s, #1: (width + height) >> 1
    let v17 = vdup_n_s32(w8); // dup v17.4s, w8: -ctz(width + height)
    let x6 = dst.wrapping_byte_offset(stride); // add x6, x0, x1
    let x1 = stride << 1; // lsl x1, x1, #1

    // movi v30.8h, #0; br x7 (ipred_cfl_tbl, by the height)
    let v0 = match w4 {
        4 => {
            // L(ipred_cfl_h4):
            let v0 = vld1_u16(x2); // ld1 {v0.4h}, [x2], #8
            pd_uaddlv_4h(v0) // uaddlv s0, v0.4h
        }
        8 => {
            // L(ipred_cfl_h8):
            let v0 = vld1q_u16(x2); // ld1 {v0.8h}, [x2], #16
            pd_uaddlv_8h(v0) // uaddlv s0, v0.8h
        }
        16 => {
            // L(ipred_cfl_h16):
            let v23 = vld1q_u16_x2(x2); // ld1 {v2.8h, v3.8h}, [x2], #32
            let v0 = vpaddq_u16(v23.0, v23.1); // addp v0.8h, v2.8h, v3.8h
            pd_uaddlv_8h(v0) // uaddlv s0, v0.8h
        }
        _ => {
            // L(ipred_cfl_h32):
            let t = vld1q_u16_x4(x2); // ld1 {v2.8h, v3.8h, v4.8h, v5.8h}, [x2], #64
            let v2 = vpaddq_u16(t.0, t.1); // addp v2.8h, v2.8h, v3.8h
            let v4 = vpaddq_u16(t.2, t.3); // addp v4.8h, v4.8h, v5.8h
            let v0 = vpaddq_u16(v2, v4); // addp v0.8h, v2.8h, v4.8h
            pd_uaddlv_8h(v0) // uaddlv s0, v0.8h
        }
    };
    let v0 = vget_low_u32(v0);
    let x2 = topleft.add(1); // (x2 after the post-increment) add x2, x2, #2

    // br x9 (ipred_cfl_tbl + 4, by the width)
    match w3 {
        4 => {
            // L(ipred_cfl_w4):
            let v2 = vld1_u16(x2); // ld1 {v2.4h}, [x2]
            let v0 = vadd_u32(v0, v16); // add v0.2s, v0.2s, v16.2s
            let v2 = vget_low_u32(pd_uaddlv_4h(v2)); // uaddlv s2, v2.4h
            let v0 = vadd_u32(v0, v2); // add v0.2s, v0.2s, v2.2s
            let v0 = vshl_u32(v0, v17); // ushl v0.2s, v0.2s, v17.2s
            let v0 = ipred_cfl_dc_mul(v0, w4 == 4, w4 == 16); // cmp w4, #4; b.eq 1f; h = 8/16: cmp w4, #16
            ipred_cfl_splat_w4_16bpc_neon(dst, x1, x6, ac, w4, v0, v1, v31) // b L(ipred_cfl_splat_w4)
        }
        8 => {
            // L(ipred_cfl_w8):
            let v2 = vld1q_u16(x2); // ld1 {v2.8h}, [x2]
            let v0 = vadd_u32(v0, v16); // add v0.2s, v0.2s, v16.2s
            let v2 = vget_low_u32(pd_uaddlv_8h(v2)); // uaddlv s2, v2.8h
            let v0 = vadd_u32(v0, v2); // add v0.2s, v0.2s, v2.2s
            let v0 = vshl_u32(v0, v17); // ushl v0.2s, v0.2s, v17.2s
            let v0 = ipred_cfl_dc_mul(v0, w4 == 8, w4 == 32); // cmp w4, #8; b.eq 1f; h = 4/16/32: cmp w4, #32
            ipred_cfl_splat_w8_16bpc_neon(dst, x1, x6, ac, w4, v0, v1, v31) // b L(ipred_cfl_splat_w8)
        }
        16 => {
            // L(ipred_cfl_w16):
            let v23 = vld1q_u16_x2(x2); // ld1 {v2.8h, v3.8h}, [x2]
            let v0 = vadd_u32(v0, v16); // add v0.2s, v0.2s, v16.2s
            let v2 = vpaddq_u16(v23.0, v23.1); // addp v2.8h, v2.8h, v3.8h
            let v2 = vget_low_u32(pd_uaddlv_8h(v2)); // uaddlv s2, v2.8h
            let v0 = vadd_u32(v0, v2); // add v0.2s, v0.2s, v2.2s
            let v0 = vshl_u32(v0, v17); // ushl v0.2s, v0.2s, v17.2s

            // cmp w4, #16; b.eq 1f; h = 4/8/32: tst w4, #(32+16+8) (16 added to make a consecutive bitmask)
            let v0 = ipred_cfl_dc_mul(v0, w4 == 16, w4 & (32 + 16 + 8) == 0);
            ipred_cfl_splat_w16_16bpc_neon(dst, x1, x6, w3, w4, ac, v0, v1, v31)
            // b L(ipred_cfl_splat_w16)
        }
        _ => {
            // L(ipred_cfl_w32):
            let t = vld1q_u16_x4(x2); // ld1 {v2.8h, v3.8h, v4.8h, v5.8h}, [x2]
            let v0 = vadd_u32(v0, v16); // add v0.4s, v0.4s, v16.4s
            let v2 = vpaddq_u16(t.0, t.1); // addp v2.8h, v2.8h, v3.8h
            let v4 = vpaddq_u16(t.2, t.3); // addp v4.8h, v4.8h, v5.8h
            let v2 = vpaddq_u16(v2, v4); // addp v2.8h, v2.8h, v4.8h
            let v2 = vget_low_u32(pd_uaddlv_8h(v2)); // uaddlv s2, v2.8h
            let v0 = vadd_u32(v0, v2); // add v0.2s, v0.2s, v2.2s
            let v0 = vshl_u32(v0, v17); // ushl v0.2s, v0.2s, v17.2s
            let v0 = ipred_cfl_dc_mul(v0, w4 == 32, w4 == 8); // cmp w4, #32; b.eq 1f; h = 8/16: cmp w4, #8
            ipred_cfl_splat_w16_16bpc_neon(dst, x1, x6, w3, w4, ac, v0, v1, v31)
            // b L(ipred_cfl_splat_w16)
        }
    }
}

// --- ipred_cfl_ac ----------------------------------------------------------------------------------------------

/// `uaddw va.4s, va.4s, v.4h; uaddw2 vb.4s, vb.4s, v.8h`: adds one register of coefficients to a pair of sums.
#[inline(always)]
unsafe fn ipred_cfl_ac_uaddw(va: &mut uint32x4_t, vb: &mut uint32x4_t, v: uint16x8_t) {
    *va = vaddw_u16(*va, vget_low_u16(v));
    *vb = vaddw_high_u16(*vb, v);
}

/// The subsampling sum of 4:2:0: `addp va.8h, a0.8h, a1.8h; addp vb.8h, b0.8h, b1.8h; add va.8h, va.8h, vb.8h`
/// with the two rows `a` and `b` (the registers of `a` hold the pixels the result's low and high half sum up).
/// Written as `add` of the rows followed by one `addp` (the same sums, three instructions as well): the
/// compiler reassociates the sum of two `addp` into `uzp1` + `uzp2` + `addp` + two `add`.
#[inline(always)]
unsafe fn ipred_cfl_ac_420_addp(a0: uint16x8_t, b0: uint16x8_t, a1: uint16x8_t, b1: uint16x8_t) -> uint16x8_t {
    vpaddq_u16(vaddq_u16(a0, b0), vaddq_u16(a1, b1))
}

/// The common head of the three `ipred_cfl_ac_*` entry points after the table load: `w8` = height - h_pad
/// (in rows of `ac`) and `w9` = log2sz. (`dup v31.4s, w9; neg v31.4s, v31.4s` follow in
/// `ipred_cfl_ac_420_w4_calc_subtract_dc_16bpc_neon`.)
#[inline(always)]
fn ipred_cfl_ac_head(w4: c_int, w5: c_int, w6: c_int) -> (c_int, c_int) {
    let w8 = w6 - w4; // sub w8, w6, w4: height - h_pad
    let w9 = w5.trailing_zeros(); // rbit w9, w5; clz w9, w9: ctz(width)
    let w10 = w6.trailing_zeros(); // rbit w10, w6; clz w10, w10: ctz(height)
    let w9 = (w9 + w10) as c_int; // add w9, w9, w10: log2sz
    (w8, w9)
}

/// `L(ipred_cfl_ac_420_w4_calc_subtract_dc)`: aggregates the sums and subtracts the dc from `w6 / 4` groups of
/// 16 coefficients. `v31` = -log2sz is formed here (`dup v31.4s, w9; neg v31.4s, v31.4s` of the entry points),
/// so that the shared blocks get `w9` in a general register and at most eight vector registers. (The blocks
/// shared by the `ipred_cfl_ac_*` entry points take their registers in one order, `x0`, `w6`, `w9`, `w4`, the
/// sums `v24`-`v27`, then the last row, so that the branch from one into the next moves nothing.)
#[inline(never)]
unsafe fn ipred_cfl_ac_420_w4_calc_subtract_dc_16bpc_neon(
    x0: *mut u16,
    mut w6: c_int,
    w9: c_int,
    v24: uint32x4_t,
    v25: uint32x4_t,
    v26: uint32x4_t,
    v27: uint32x4_t,
) {
    // Aggregate the sums
    let v24 = vaddq_u32(v24, v25); // add v24.4s, v24.4s, v25.4s
    let v26 = vaddq_u32(v26, v27); // add v26.4s, v26.4s, v27.4s
    let v0 = vaddq_u32(v24, v26); // add v0.4s, v24.4s, v26.4s
    let s0 = vaddvq_u32(v0); // addv s0, v0.4s: sum
    let mut x0 = x0.sub((w6 as u32 as usize) << 2); // sub x0, x0, w6, uxtw #3
    let v31 = vdupq_n_s32(w9); // dup v31.4s, w9
    let v31 = vnegq_s32(v31); // neg v31.4s, v31.4s: -log2sz
    let v0 = vdup_n_u32(s0); // (the sum in lane 0; only lane 0 is used)
    let v4 = vrshl_u32(v0, vget_low_s32(v31)); // urshl v4.2s, v0.2s, v31.2s: (sum + (1 << (log2sz - 1))) >>= log2sz
    let v4 = vdupq_lane_u16::<0>(vreinterpret_u16_u32(v4)); // dup v4.8h, v4.h[0]
    loop {
        // 6: Subtract dc from ac
        let v01 = vld1q_u16_x2(x0); // ld1 {v0.8h, v1.8h}, [x0]
        let gt = pd_subs_gt(&mut w6, 4); // subs w6, w6, #4
        let v0 = vsubq_u16(v01.0, v4); // sub v0.8h, v0.8h, v4.8h
        let v1 = vsubq_u16(v01.1, v4); // sub v1.8h, v1.8h, v4.8h
        vst1q_u16_x2(x0, uint16x8x2_t(v0, v1)); // st1 {v0.8h, v1.8h}, [x0], #32
        x0 = x0.wrapping_add(16);
        if !gt {
            break; // b.gt 6b
        }
    }
}

/// `L(ipred_cfl_ac_420_w4_hpad)`: the vertical padding for w 4, then
/// `L(ipred_cfl_ac_420_w4_calc_subtract_dc)`; `v0`, `v1` hold the last row twice each.
#[inline(never)]
unsafe fn ipred_cfl_ac_420_w4_hpad_16bpc_neon(
    mut x0: *mut u16,
    w6: c_int,
    w9: c_int,
    mut w4: c_int,
    mut v24: uint32x4_t,
    mut v25: uint32x4_t,
    mut v26: uint32x4_t,
    mut v27: uint32x4_t,
    v0: uint16x8_t,
    v1: uint16x8_t,
) {
    if w4 != 0 {
        // cbz w4, 3f
        loop {
            // 2: Vertical padding (h_pad > 0)
            let gt = pd_subs_gt(&mut w4, 4); // subs w4, w4, #4
            vst1q_u16_x2(x0, uint16x8x2_t(v0, v1)); // st1 {v0.8h, v1.8h}, [x0], #32
            x0 = x0.add(16);
            ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
            ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
            if !gt {
                break; // b.gt 2b
            }
        }
    }
    // 3:
    ipred_cfl_ac_420_w4_calc_subtract_dc_16bpc_neon(x0, w6, w9, v24, v25, v26, v27)
    // L(ipred_cfl_ac_420_w4_calc_subtract_dc):
}

/// `L(ipred_cfl_ac_420_w8_hpad)`: the vertical padding for w 8, then
/// `L(ipred_cfl_ac_420_w4_calc_subtract_dc)` with twice the height; `v0`, `v1` hold the last row.
#[inline(never)]
unsafe fn ipred_cfl_ac_420_w8_hpad_16bpc_neon(
    mut x0: *mut u16,
    w6: c_int,
    w9: c_int,
    mut w4: c_int,
    mut v24: uint32x4_t,
    mut v25: uint32x4_t,
    mut v26: uint32x4_t,
    mut v27: uint32x4_t,
    v0: uint16x8_t,
    v1: uint16x8_t,
) {
    if w4 != 0 {
        // cbz w4, 3f
        loop {
            // 2: Vertical padding (h_pad > 0)
            let gt = pd_subs_gt(&mut w4, 4); // subs w4, w4, #4
            vst1q_u16_x2(x0, uint16x8x2_t(v0, v1)); // st1 {v0.8h, v1.8h}, [x0], #32
            x0 = x0.add(16);
            ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
            ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
            vst1q_u16_x2(x0, uint16x8x2_t(v0, v1)); // st1 {v0.8h, v1.8h}, [x0], #32
            x0 = x0.add(16);
            ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
            ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
            if !gt {
                break; // b.gt 2b
            }
        }
    }
    // 3:
    // Double the height and reuse the w4 summing/subtracting
    let w6 = w6 << 1; // lsl w6, w6, #1
    ipred_cfl_ac_420_w4_calc_subtract_dc_16bpc_neon(x0, w6, w9, v24, v25, v26, v27)
    // b L(ipred_cfl_ac_420_w4_calc_subtract_dc)
}

/// `L(ipred_cfl_ac_420_w16_hpad)`: the vertical padding for w 16, then
/// `L(ipred_cfl_ac_420_w4_calc_subtract_dc)` with four times the height; `v0`, `v1` and `v2`, `v3` hold the
/// last row.
#[inline(never)]
unsafe fn ipred_cfl_ac_420_w16_hpad_16bpc_neon(
    mut x0: *mut u16,
    w6: c_int,
    w9: c_int,
    mut w4: c_int,
    mut v24: uint32x4_t,
    mut v25: uint32x4_t,
    mut v26: uint32x4_t,
    mut v27: uint32x4_t,
    v0: uint16x8_t,
    v1: uint16x8_t,
    v2: uint16x8_t,
    v3: uint16x8_t,
) {
    if w4 != 0 {
        // cbz w4, 3f
        loop {
            // 2: Vertical padding (h_pad > 0)
            let gt = pd_subs_gt(&mut w4, 4); // subs w4, w4, #4
            vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
            x0 = x0.add(32);
            ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
            ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
            ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
            ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
            vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
            x0 = x0.add(32);
            ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
            ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
            ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
            ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
            if !gt {
                break; // b.gt 2b
            }
        }
    }
    // 3:
    // Quadruple the height and reuse the w4 summing/subtracting
    let w6 = w6 << 2; // lsl w6, w6, #2
    ipred_cfl_ac_420_w4_calc_subtract_dc_16bpc_neon(x0, w6, w9, v24, v25, v26, v27)
    // b L(ipred_cfl_ac_420_w4_calc_subtract_dc)
}

/// `dav1d_ipred_cfl_ac_420_16bpc_neon`: the luma of a 4:2:0 block, subsampled, times 8, less its average.
///
/// Safety: reads the luma rows `0 .. 2 * (ch - 4 * h_pad)` at the pixels `0 .. 2 * (cw - 4 * w_pad)` exactly
/// (cw 4: 8 pixels per row; cw 8: 16, or 8 with `w_pad` 1; cw 16: 32, 24, 16 or 8 for `w_pad` 0..3); writes
/// `ac[0 .. cw * ch]`, rows of `cw` coefficients, and reads them back for the subtraction.
pub unsafe extern "C" fn dav1d_ipred_cfl_ac_420_16bpc_neon(
    ac: *mut i16,
    ypx: *const u16,
    stride: isize,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
) {
    let (mut x0, mut x1, w3, w5, w6) = (ac.cast::<u16>(), ypx, w_pad, cw, ch);
    let w4 = h_pad << 2; // lsl w4, w4, #2
    let mut v24 = vdupq_n_u32(0); // movi v24.4s, #0
    let mut v25 = vdupq_n_u32(0); // movi v25.4s, #0
    let mut v26 = vdupq_n_u32(0); // movi v26.4s, #0
    let mut v27 = vdupq_n_u32(0); // movi v27.4s, #0
    let (mut w8, w9) = ipred_cfl_ac_head(w4, w5, w6);
    let mut x10 = x1.wrapping_byte_offset(stride); // add x10, x1, x2
    let x2 = stride << 1; // lsl x2, x2, #1

    // br x7 (ipred_cfl_ac_420_tbl, by the width)
    match w5 {
        4 => {
            // L(ipred_cfl_ac_420_w4):
            let mut v0;
            loop {
                // 1: Copy and subsample input
                let t0 = vld1q_u16(x1); // ld1 {v0.8h}, [x1], x2
                x1 = x1.wrapping_byte_offset(x2);
                let t1 = vld1q_u16(x10); // ld1 {v1.8h}, [x10], x2
                x10 = x10.wrapping_byte_offset(x2);
                let t2 = vld1q_u16(x1); // ld1 {v2.8h}, [x1], x2
                x1 = x1.wrapping_byte_offset(x2);
                let t3 = vld1q_u16(x10); // ld1 {v3.8h}, [x10], x2
                x10 = x10.wrapping_byte_offset(x2);
                // addp v0.8h, v0.8h, v2.8h; addp v1.8h, v1.8h, v3.8h; add v0.8h, v0.8h, v1.8h (the rows are added
                // first, see `ipred_cfl_ac_420_addp`)
                let t0 = ipred_cfl_ac_420_addp(t0, t1, t2, t3);
                v0 = vshlq_n_u16::<1>(t0); // shl v0.8h, v0.8h, #1
                let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                vst1q_u16(x0, v0); // st1 {v0.8h}, [x0], #16
                x0 = x0.add(8);
                ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                if !gt {
                    break; // b.gt 1b
                }
            }
            let v0q = vreinterpretq_u64_u16(v0);
            let v1 = vreinterpretq_u16_u64(vtrn2q_u64(v0q, v0q)); // trn2 v1.2d, v0.2d, v0.2d
            let v0 = v1; // trn2 v0.2d, v0.2d, v0.2d
            ipred_cfl_ac_420_w4_hpad_16bpc_neon(x0, w6, w9, w4, v24, v25, v26, v27, v0, v1)
            // L(ipred_cfl_ac_420_w4_hpad):
        }
        8 => {
            // L(ipred_cfl_ac_420_w8):
            let (v0, v1);
            if w3 == 0 {
                // cbnz w3, L(ipred_cfl_ac_420_w8_wpad)
                let mut r1;
                loop {
                    // 1: Copy and subsample input, without padding
                    let t01 = vld1q_u16_x2(x1); // ld1 {v0.8h, v1.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t23 = vld1q_u16_x2(x10); // ld1 {v2.8h, v3.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t45 = vld1q_u16_x2(x1); // ld1 {v4.8h, v5.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t67 = vld1q_u16_x2(x10); // ld1 {v6.8h, v7.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);

                    // addp v0.8h, v0.8h, v1.8h; addp v2.8h, v2.8h, v3.8h; add v0.8h, v0.8h, v2.8h
                    let t0 = ipred_cfl_ac_420_addp(t01.0, t23.0, t01.1, t23.1);

                    // addp v4.8h, v4.8h, v5.8h; addp v6.8h, v6.8h, v7.8h; add v4.8h, v4.8h, v6.8h
                    let t4 = ipred_cfl_ac_420_addp(t45.0, t67.0, t45.1, t67.1);
                    let r0 = vshlq_n_u16::<1>(t0); // shl v0.8h, v0.8h, #1
                    r1 = vshlq_n_u16::<1>(t4); // shl v1.8h, v4.8h, #1
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x2(x0, uint16x8x2_t(r0, r1)); // st1 {v0.8h, v1.8h}, [x0], #32
                    x0 = x0.add(16);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, r0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, r1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                }
                v0 = r1; // mov v0.16b, v1.16b
                v1 = r1;
                // b L(ipred_cfl_ac_420_w8_hpad)
            } else {
                // L(ipred_cfl_ac_420_w8_wpad):
                let (mut r2, mut r3);
                loop {
                    // 1: Copy and subsample input, padding 4
                    let t0 = vld1q_u16(x1); // ld1 {v0.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t1 = vld1q_u16(x10); // ld1 {v1.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t2 = vld1q_u16(x1); // ld1 {v2.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t3 = vld1q_u16(x10); // ld1 {v3.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    // addp v0.8h, v0.8h, v2.8h; addp v1.8h, v1.8h, v3.8h; add v0.8h, v0.8h, v1.8h
                    let t0 = ipred_cfl_ac_420_addp(t0, t1, t2, t3);
                    let t0 = vshlq_n_u16::<1>(t0); // shl v0.8h, v0.8h, #1
                    let r1 = vdup_laneq_u16::<3>(t0); // dup v1.4h, v0.h[3]
                    r3 = vdup_laneq_u16::<7>(t0); // dup v3.4h, v0.h[7]
                    r2 = vget_high_u16(t0); // trn2 v2.2d, v0.2d, v0.2d
                    let r0 = vget_low_u16(t0);
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1_u16_x4(x0, uint16x4x4_t(r0, r1, r2, r3)); // st1 {v0.4h, v1.4h, v2.4h, v3.4h}, [x0], #32
                    x0 = x0.add(16);
                    v24 = vaddw_u16(v24, r0); // uaddw v24.4s, v24.4s, v0.4h
                    v25 = vaddw_u16(v25, r1); // uaddw v25.4s, v25.4s, v1.4h
                    v26 = vaddw_u16(v26, r2); // uaddw v26.4s, v26.4s, v2.4h
                    v27 = vaddw_u16(v27, r3); // uaddw v27.4s, v27.4s, v3.4h
                    if !gt {
                        break; // b.gt 1b
                    }
                }
                v0 = vcombine_u16(r2, r3); // trn1 v0.2d, v2.2d, v3.2d
                v1 = v0; // trn1 v1.2d, v2.2d, v3.2d
            }
            ipred_cfl_ac_420_w8_hpad_16bpc_neon(x0, w6, w9, w4, v24, v25, v26, v27, v0, v1)
            // L(ipred_cfl_ac_420_w8_hpad):
        }
        _ => {
            // L(ipred_cfl_ac_420_w16):
            let (mut v2, mut v3);

            // br x7 (ipred_cfl_ac_420_w16_tbl, by w_pad)
            match w3 {
                0 => loop {
                    // L(ipred_cfl_ac_420_w16_wpad0):
                    // 1: Copy and subsample input, without padding
                    let a = vld1q_u16_x4(x1); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let b = vld1q_u16_x4(x10); // ld1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let c = vld1q_u16_x4(x1); // ld1 {v16.8h, v17.8h, v18.8h, v19.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let e = vld1q_u16_x4(x10); // ld1 {v20.8h, v21.8h, v22.8h, v23.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);

                    // addp v0.8h, v0.8h, v1.8h; addp v4.8h, v4.8h, v5.8h; add v0.8h, v0.8h, v4.8h
                    let t0 = ipred_cfl_ac_420_addp(a.0, b.0, a.1, b.1);

                    // addp v2.8h, v2.8h, v3.8h; addp v6.8h, v6.8h, v7.8h; add v2.8h, v2.8h, v6.8h
                    let t2 = ipred_cfl_ac_420_addp(a.2, b.2, a.3, b.3);

                    // addp v16.8h, v16.8h, v17.8h; addp v20.8h, v20.8h, v21.8h; add v16.8h, v16.8h, v20.8h
                    let t16 = ipred_cfl_ac_420_addp(c.0, e.0, c.1, e.1);

                    // addp v18.8h, v18.8h, v19.8h; addp v22.8h, v22.8h, v23.8h; add v18.8h, v18.8h, v22.8h
                    let t18 = ipred_cfl_ac_420_addp(c.2, e.2, c.3, e.3);
                    let v0 = vshlq_n_u16::<1>(t0); // shl v0.8h, v0.8h, #1
                    let v1 = vshlq_n_u16::<1>(t2); // shl v1.8h, v2.8h, #1
                    v2 = vshlq_n_u16::<1>(t16); // shl v2.8h, v16.8h, #1
                    v3 = vshlq_n_u16::<1>(t18); // shl v3.8h, v18.8h, #1
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                1 => loop {
                    // L(ipred_cfl_ac_420_w16_wpad1):
                    // 1: Copy and subsample input, padding 4
                    let t2 = vld1q_u16(x1.add(16)); // ldr q2, [x1, #32]
                    let t01 = vld1q_u16_x2(x1); // ld1 {v0.8h, v1.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t5 = vld1q_u16(x10.add(16)); // ldr q5, [x10, #32]
                    let t34 = vld1q_u16_x2(x10); // ld1 {v3.8h, v4.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t18 = vld1q_u16(x1.add(16)); // ldr q18, [x1, #32]
                    let t1617 = vld1q_u16_x2(x1); // ld1 {v16.8h, v17.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t21 = vld1q_u16(x10.add(16)); // ldr q21, [x10, #32]
                    let t1920 = vld1q_u16_x2(x10); // ld1 {v19.8h, v20.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);

                    // addp v2.8h, v2.8h, v2.8h; addp v5.8h, v5.8h, v5.8h; add v2.4h, v2.4h, v5.4h
                    let t2 = vget_low_u16(ipred_cfl_ac_420_addp(t2, t5, t2, t5));

                    // addp v0.8h, v0.8h, v1.8h; addp v3.8h, v3.8h, v4.8h; add v0.8h, v0.8h, v3.8h
                    let t0 = ipred_cfl_ac_420_addp(t01.0, t34.0, t01.1, t34.1);

                    // addp v18.8h, v18.8h, v18.8h; addp v21.8h, v21.8h, v21.8h; add v18.4h, v18.4h, v21.4h
                    let t18 = vget_low_u16(ipred_cfl_ac_420_addp(t18, t21, t18, t21));

                    // addp v16.8h, v16.8h, v17.8h; addp v19.8h, v19.8h, v20.8h; add v16.8h, v16.8h, v19.8h
                    let t16 = ipred_cfl_ac_420_addp(t1617.0, t1920.0, t1617.1, t1920.1);
                    let t1 = vshl_n_u16::<1>(t2); // shl v1.4h, v2.4h, #1
                    let v0 = vshlq_n_u16::<1>(t0); // shl v0.8h, v0.8h, #1
                    let t3 = vshl_n_u16::<1>(t18); // shl v3.4h, v18.4h, #1
                    v2 = vshlq_n_u16::<1>(t16); // shl v2.8h, v16.8h, #1
                    let t4 = vdup_lane_u16::<3>(t1); // dup v4.4h, v1.h[3]
                    let t5 = vdup_lane_u16::<3>(t3); // dup v5.4h, v3.h[3]
                    let v1 = vcombine_u16(t1, t4); // trn1 v1.2d, v1.2d, v4.2d
                    v3 = vcombine_u16(t3, t5); // trn1 v3.2d, v3.2d, v5.2d
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                2 => loop {
                    // L(ipred_cfl_ac_420_w16_wpad2):
                    // 1: Copy and subsample input, padding 8
                    let t01 = vld1q_u16_x2(x1); // ld1 {v0.8h, v1.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t23 = vld1q_u16_x2(x10); // ld1 {v2.8h, v3.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t45 = vld1q_u16_x2(x1); // ld1 {v4.8h, v5.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t67 = vld1q_u16_x2(x10); // ld1 {v6.8h, v7.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);

                    // addp v0.8h, v0.8h, v1.8h; addp v2.8h, v2.8h, v3.8h; add v0.8h, v0.8h, v2.8h
                    let t0 = ipred_cfl_ac_420_addp(t01.0, t23.0, t01.1, t23.1);

                    // addp v4.8h, v4.8h, v5.8h; addp v6.8h, v6.8h, v7.8h; add v4.8h, v4.8h, v6.8h
                    let t4 = ipred_cfl_ac_420_addp(t45.0, t67.0, t45.1, t67.1);
                    let v0 = vshlq_n_u16::<1>(t0); // shl v0.8h, v0.8h, #1
                    v2 = vshlq_n_u16::<1>(t4); // shl v2.8h, v4.8h, #1
                    let v1 = vdupq_laneq_u16::<7>(v0); // dup v1.8h, v0.h[7]
                    v3 = vdupq_laneq_u16::<7>(v2); // dup v3.8h, v2.h[7]
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                _ => loop {
                    // L(ipred_cfl_ac_420_w16_wpad3):
                    // 1: Copy and subsample input, padding 12
                    let t0 = vld1q_u16(x1); // ld1 {v0.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t2 = vld1q_u16(x10); // ld1 {v2.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t4 = vld1q_u16(x1); // ld1 {v4.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t6 = vld1q_u16(x10); // ld1 {v6.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    // addp v0.8h, v0.8h, v4.8h; addp v2.8h, v2.8h, v6.8h; add v0.8h, v0.8h, v2.8h
                    let t0 = ipred_cfl_ac_420_addp(t0, t2, t4, t6);
                    let t0 = vshlq_n_u16::<1>(t0); // shl v0.8h, v0.8h, #1
                    let v1 = vdupq_laneq_u16::<3>(t0); // dup v1.8h, v0.h[3]
                    v3 = vdupq_laneq_u16::<7>(t0); // dup v3.8h, v0.h[7]
                    v2 = vcombine_u16(vget_high_u16(t0), vget_high_u16(v3)); // trn2 v2.2d, v0.2d, v3.2d
                    let v0 = vcombine_u16(vget_low_u16(t0), vget_low_u16(v1)); // trn1 v0.2d, v0.2d, v1.2d
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
            }
            // mov v0.16b, v2.16b; mov v1.16b, v3.16b (the arguments)
            ipred_cfl_ac_420_w16_hpad_16bpc_neon(x0, w6, w9, w4, v24, v25, v26, v27, v2, v3, v2, v3)
            // b L(ipred_cfl_ac_420_w16_hpad)
        }
    }
}

/// `dav1d_ipred_cfl_ac_422_16bpc_neon`: the luma of a 4:2:2 block, subsampled, times 8, less its average.
///
/// Safety: reads the luma rows `0 .. ch - 4 * h_pad` at the pixels `0 .. 2 * (cw - 4 * w_pad)` exactly (cw 4:
/// 8 pixels per row; cw 8: 16, or 8 with `w_pad` 1; cw 16: 32, 24, 16 or 8 for `w_pad` 0..3); writes
/// `ac[0 .. cw * ch]`, rows of `cw` coefficients, and reads them back for the subtraction.
pub unsafe extern "C" fn dav1d_ipred_cfl_ac_422_16bpc_neon(
    ac: *mut i16,
    ypx: *const u16,
    stride: isize,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
) {
    let (mut x0, mut x1, w3, w5, w6) = (ac.cast::<u16>(), ypx, w_pad, cw, ch);
    let w4 = h_pad << 2; // lsl w4, w4, #2
    let mut v24 = vdupq_n_u32(0); // movi v24.4s, #0
    let mut v25 = vdupq_n_u32(0); // movi v25.4s, #0
    let mut v26 = vdupq_n_u32(0); // movi v26.4s, #0
    let mut v27 = vdupq_n_u32(0); // movi v27.4s, #0
    let (mut w8, w9) = ipred_cfl_ac_head(w4, w5, w6);
    let mut x10 = x1.wrapping_byte_offset(stride); // add x10, x1, x2
    let x2 = stride << 1; // lsl x2, x2, #1

    // br x7 (ipred_cfl_ac_422_tbl, by the width)
    match w5 {
        4 => {
            // L(ipred_cfl_ac_422_w4):
            let mut v1;
            loop {
                // 1: Copy and subsample input
                let t0 = vld1q_u16(x1); // ld1 {v0.8h}, [x1], x2
                x1 = x1.wrapping_byte_offset(x2);
                let t1 = vld1q_u16(x10); // ld1 {v1.8h}, [x10], x2
                x10 = x10.wrapping_byte_offset(x2);
                let t2 = vld1q_u16(x1); // ld1 {v2.8h}, [x1], x2
                x1 = x1.wrapping_byte_offset(x2);
                let t3 = vld1q_u16(x10); // ld1 {v3.8h}, [x10], x2
                x10 = x10.wrapping_byte_offset(x2);
                let t0 = vpaddq_u16(t0, t1); // addp v0.8h, v0.8h, v1.8h
                let t2 = vpaddq_u16(t2, t3); // addp v2.8h, v2.8h, v3.8h
                let v0 = vshlq_n_u16::<2>(t0); // shl v0.8h, v0.8h, #2
                v1 = vshlq_n_u16::<2>(t2); // shl v1.8h, v2.8h, #2
                let gt = pd_subs_gt(&mut w8, 4); // subs w8, w8, #4
                vst1q_u16_x2(x0, uint16x8x2_t(v0, v1)); // st1 {v0.8h, v1.8h}, [x0], #32
                x0 = x0.add(16);
                ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                if !gt {
                    break; // b.gt 1b
                }
            }
            let v1q = vreinterpretq_u64_u16(v1);
            let v0 = vreinterpretq_u16_u64(vtrn2q_u64(v1q, v1q)); // trn2 v0.2d, v1.2d, v1.2d
            let v1 = v0; // trn2 v1.2d, v1.2d, v1.2d
            ipred_cfl_ac_420_w4_hpad_16bpc_neon(x0, w6, w9, w4, v24, v25, v26, v27, v0, v1)
            // b L(ipred_cfl_ac_420_w4_hpad)
        }
        8 => {
            // L(ipred_cfl_ac_422_w8):
            let mut v3;
            if w3 == 0 {
                // cbnz w3, L(ipred_cfl_ac_422_w8_wpad)
                loop {
                    // 1: Copy and subsample input, without padding
                    let t01 = vld1q_u16_x2(x1); // ld1 {v0.8h, v1.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t23 = vld1q_u16_x2(x10); // ld1 {v2.8h, v3.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t45 = vld1q_u16_x2(x1); // ld1 {v4.8h, v5.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t0 = vpaddq_u16(t01.0, t01.1); // addp v0.8h, v0.8h, v1.8h
                    let t67 = vld1q_u16_x2(x10); // ld1 {v6.8h, v7.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t2 = vpaddq_u16(t23.0, t23.1); // addp v2.8h, v2.8h, v3.8h
                    let t4 = vpaddq_u16(t45.0, t45.1); // addp v4.8h, v4.8h, v5.8h
                    let t6 = vpaddq_u16(t67.0, t67.1); // addp v6.8h, v6.8h, v7.8h
                    let v0 = vshlq_n_u16::<2>(t0); // shl v0.8h, v0.8h, #2
                    let v1 = vshlq_n_u16::<2>(t2); // shl v1.8h, v2.8h, #2
                    let v2 = vshlq_n_u16::<2>(t4); // shl v2.8h, v4.8h, #2
                    v3 = vshlq_n_u16::<2>(t6); // shl v3.8h, v6.8h, #2
                    let gt = pd_subs_gt(&mut w8, 4); // subs w8, w8, #4
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                }
            } else {
                // L(ipred_cfl_ac_422_w8_wpad):
                loop {
                    // 1: Copy and subsample input, padding 4
                    let t0 = vld1q_u16(x1); // ld1 {v0.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t1 = vld1q_u16(x10); // ld1 {v1.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t2 = vld1q_u16(x1); // ld1 {v2.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t3 = vld1q_u16(x10); // ld1 {v3.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t0 = vpaddq_u16(t0, t1); // addp v0.8h, v0.8h, v1.8h
                    let t2 = vpaddq_u16(t2, t3); // addp v2.8h, v2.8h, v3.8h
                    let t0 = vshlq_n_u16::<2>(t0); // shl v0.8h, v0.8h, #2
                    let t2 = vshlq_n_u16::<2>(t2); // shl v2.8h, v2.8h, #2
                    let t4 = vdup_laneq_u16::<3>(t0); // dup v4.4h, v0.h[3]
                    let t5 = vdup_laneq_u16::<7>(t0); // dup v5.8h, v0.h[7]
                    let t6 = vdup_laneq_u16::<3>(t2); // dup v6.4h, v2.h[3]
                    let t7 = vdup_laneq_u16::<7>(t2); // dup v7.8h, v2.h[7]
                    let v1 = vcombine_u16(vget_high_u16(t0), t5); // trn2 v1.2d, v0.2d, v5.2d
                    let v0 = vcombine_u16(vget_low_u16(t0), t4); // trn1 v0.2d, v0.2d, v4.2d
                    v3 = vcombine_u16(vget_high_u16(t2), t7); // trn2 v3.2d, v2.2d, v7.2d
                    let v2 = vcombine_u16(vget_low_u16(t2), t6); // trn1 v2.2d, v2.2d, v6.2d
                    let gt = pd_subs_gt(&mut w8, 4); // subs w8, w8, #4
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                }
            }
            // mov v0.16b, v3.16b; mov v1.16b, v3.16b
            ipred_cfl_ac_420_w8_hpad_16bpc_neon(x0, w6, w9, w4, v24, v25, v26, v27, v3, v3)
            // b L(ipred_cfl_ac_420_w8_hpad)
        }
        _ => {
            // L(ipred_cfl_ac_422_w16):
            let (mut v2, mut v3);

            // br x7 (ipred_cfl_ac_422_w16_tbl, by w_pad)
            match w3 {
                0 => loop {
                    // L(ipred_cfl_ac_422_w16_wpad0):
                    // 1: Copy and subsample input, without padding
                    let a = vld1q_u16_x4(x1); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let b = vld1q_u16_x4(x10); // ld1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t0 = vpaddq_u16(a.0, a.1); // addp v0.8h, v0.8h, v1.8h
                    let t2 = vpaddq_u16(a.2, a.3); // addp v2.8h, v2.8h, v3.8h
                    let t4 = vpaddq_u16(b.0, b.1); // addp v4.8h, v4.8h, v5.8h
                    let t6 = vpaddq_u16(b.2, b.3); // addp v6.8h, v6.8h, v7.8h
                    let v0 = vshlq_n_u16::<2>(t0); // shl v0.8h, v0.8h, #2
                    let v1 = vshlq_n_u16::<2>(t2); // shl v1.8h, v2.8h, #2
                    v2 = vshlq_n_u16::<2>(t4); // shl v2.8h, v4.8h, #2
                    v3 = vshlq_n_u16::<2>(t6); // shl v3.8h, v6.8h, #2
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                1 => loop {
                    // L(ipred_cfl_ac_422_w16_wpad1):
                    // 1: Copy and subsample input, padding 4
                    let t2 = vld1q_u16(x1.add(16)); // ldr q2, [x1, #32]
                    let t01 = vld1q_u16_x2(x1); // ld1 {v0.8h, v1.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t6 = vld1q_u16(x10.add(16)); // ldr q6, [x10, #32]
                    let t45 = vld1q_u16_x2(x10); // ld1 {v4.8h, v5.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t2 = vpaddq_u16(t2, t2); // addp v2.8h, v2.8h, v2.8h
                    let t0 = vpaddq_u16(t01.0, t01.1); // addp v0.8h, v0.8h, v1.8h
                    let t6 = vpaddq_u16(t6, t6); // addp v6.8h, v6.8h, v6.8h
                    let t4 = vpaddq_u16(t45.0, t45.1); // addp v4.8h, v4.8h, v5.8h
                    let t1 = vshl_n_u16::<2>(vget_low_u16(t2)); // shl v1.4h, v2.4h, #2
                    let v0 = vshlq_n_u16::<2>(t0); // shl v0.8h, v0.8h, #2
                    let t3 = vshl_n_u16::<2>(vget_low_u16(t6)); // shl v3.4h, v6.4h, #2
                    v2 = vshlq_n_u16::<2>(t4); // shl v2.8h, v4.8h, #2
                    let t4 = vdup_lane_u16::<3>(t1); // dup v4.4h, v1.h[3]
                    let t5 = vdup_lane_u16::<3>(t3); // dup v5.4h, v3.h[3]
                    let v1 = vcombine_u16(t1, t4); // trn1 v1.2d, v1.2d, v4.2d
                    v3 = vcombine_u16(t3, t5); // trn1 v3.2d, v3.2d, v5.2d
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                2 => loop {
                    // L(ipred_cfl_ac_422_w16_wpad2):
                    // 1: Copy and subsample input, padding 8
                    let t01 = vld1q_u16_x2(x1); // ld1 {v0.8h, v1.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t23 = vld1q_u16_x2(x10); // ld1 {v2.8h, v3.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t0 = vpaddq_u16(t01.0, t01.1); // addp v0.8h, v0.8h, v1.8h
                    let t2 = vpaddq_u16(t23.0, t23.1); // addp v2.8h, v2.8h, v3.8h
                    let v0 = vshlq_n_u16::<2>(t0); // shl v0.8h, v0.8h, #2
                    v2 = vshlq_n_u16::<2>(t2); // shl v2.8h, v2.8h, #2
                    let v1 = vdupq_laneq_u16::<7>(v0); // dup v1.8h, v0.h[7]
                    v3 = vdupq_laneq_u16::<7>(v2); // dup v3.8h, v2.h[7]
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                _ => loop {
                    // L(ipred_cfl_ac_422_w16_wpad3):
                    // 1: Copy and subsample input, padding 12
                    let t0 = vld1q_u16(x1); // ld1 {v0.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t2 = vld1q_u16(x10); // ld1 {v2.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let t0 = vpaddq_u16(t0, t0); // addp v0.8h, v0.8h, v0.8h
                    let t2 = vpaddq_u16(t2, t2); // addp v2.8h, v2.8h, v2.8h
                    let t0 = vshl_n_u16::<2>(vget_low_u16(t0)); // shl v0.4h, v0.4h, #2
                    let t2 = vshl_n_u16::<2>(vget_low_u16(t2)); // shl v2.4h, v2.4h, #2
                    let v1 = vdupq_lane_u16::<3>(t0); // dup v1.8h, v0.h[3]
                    v3 = vdupq_lane_u16::<3>(t2); // dup v3.8h, v2.h[3]
                    let v0 = vcombine_u16(t0, vget_low_u16(v1)); // trn1 v0.2d, v0.2d, v1.2d
                    v2 = vcombine_u16(t2, vget_low_u16(v3)); // trn1 v2.2d, v2.2d, v3.2d
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
            }
            // mov v0.16b, v2.16b; mov v1.16b, v3.16b (the arguments)
            ipred_cfl_ac_420_w16_hpad_16bpc_neon(x0, w6, w9, w4, v24, v25, v26, v27, v2, v3, v2, v3)
            // b L(ipred_cfl_ac_420_w16_hpad)
        }
    }
}

/// `dav1d_ipred_cfl_ac_444_16bpc_neon`: the luma of a 4:4:4 block times 8, less its average.
///
/// Safety: reads the luma rows `0 .. ch - 4 * h_pad` at the pixels `0 .. cw - 4 * w_pad` exactly, for the
/// values of `w_pad` the layout has (cw 4 and 8: 0; cw 16: 0 or 2, any other value is read as 2; cw 32: 0,
/// 2, 4 or 6, an odd value is read as the even one below it); writes `ac[0 .. cw * ch]`, rows of `cw`
/// coefficients, and reads them back for the subtraction.
pub unsafe extern "C" fn dav1d_ipred_cfl_ac_444_16bpc_neon(
    ac: *mut i16,
    ypx: *const u16,
    stride: isize,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
) {
    let (mut x0, mut x1, w3, w5, w6) = (ac.cast::<u16>(), ypx, w_pad, cw, ch);
    let w4 = h_pad << 2; // lsl w4, w4, #2
    let mut v24 = vdupq_n_u32(0); // movi v24.4s, #0
    let mut v25 = vdupq_n_u32(0); // movi v25.4s, #0
    let mut v26 = vdupq_n_u32(0); // movi v26.4s, #0
    let mut v27 = vdupq_n_u32(0); // movi v27.4s, #0
    let (mut w8, w9) = ipred_cfl_ac_head(w4, w5, w6);
    let mut x10 = x1.wrapping_byte_offset(stride); // add x10, x1, x2
    let x2 = stride << 1; // lsl x2, x2, #1

    // br x7 (ipred_cfl_ac_444_tbl, by the width)
    match w5 {
        4 => {
            // L(ipred_cfl_ac_444_w4):
            let mut v1;
            loop {
                // 1: Copy and expand input
                let t0 = pd_ld1_d2(x1, x10); // ld1 {v0.4h}, [x1], x2; ld1 {v0.d}[1], [x10], x2
                x1 = x1.wrapping_byte_offset(x2);
                x10 = x10.wrapping_byte_offset(x2);
                let t1 = pd_ld1_d2(x1, x10); // ld1 {v1.4h}, [x1], x2; ld1 {v1.d}[1], [x10], x2
                x1 = x1.wrapping_byte_offset(x2);
                x10 = x10.wrapping_byte_offset(x2);
                let v0 = vshlq_n_u16::<3>(t0); // shl v0.8h, v0.8h, #3
                v1 = vshlq_n_u16::<3>(t1); // shl v1.8h, v1.8h, #3
                let gt = pd_subs_gt(&mut w8, 4); // subs w8, w8, #4
                vst1q_u16_x2(x0, uint16x8x2_t(v0, v1)); // st1 {v0.8h, v1.8h}, [x0], #32
                x0 = x0.add(16);
                ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                if !gt {
                    break; // b.gt 1b
                }
            }
            let v1q = vreinterpretq_u64_u16(v1);
            let v0 = vreinterpretq_u16_u64(vtrn2q_u64(v1q, v1q)); // trn2 v0.2d, v1.2d, v1.2d
            let v1 = v0; // trn2 v1.2d, v1.2d, v1.2d
            ipred_cfl_ac_420_w4_hpad_16bpc_neon(x0, w6, w9, w4, v24, v25, v26, v27, v0, v1)
            // b L(ipred_cfl_ac_420_w4_hpad)
        }
        8 => {
            // L(ipred_cfl_ac_444_w8):
            let mut v3;
            loop {
                // 1: Copy and expand input
                let t0 = vld1q_u16(x1); // ld1 {v0.8h}, [x1], x2
                x1 = x1.wrapping_byte_offset(x2);
                let t1 = vld1q_u16(x10); // ld1 {v1.8h}, [x10], x2
                x10 = x10.wrapping_byte_offset(x2);
                let t2 = vld1q_u16(x1); // ld1 {v2.8h}, [x1], x2
                x1 = x1.wrapping_byte_offset(x2);
                let v0 = vshlq_n_u16::<3>(t0); // shl v0.8h, v0.8h, #3
                let t3 = vld1q_u16(x10); // ld1 {v3.8h}, [x10], x2
                x10 = x10.wrapping_byte_offset(x2);
                let v1 = vshlq_n_u16::<3>(t1); // shl v1.8h, v1.8h, #3
                let v2 = vshlq_n_u16::<3>(t2); // shl v2.8h, v2.8h, #3
                v3 = vshlq_n_u16::<3>(t3); // shl v3.8h, v3.8h, #3
                let gt = pd_subs_gt(&mut w8, 4); // subs w8, w8, #4
                vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                x0 = x0.add(32);
                ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                if !gt {
                    break; // b.gt 1b
                }
            }
            // mov v0.16b, v3.16b; mov v1.16b, v3.16b
            ipred_cfl_ac_420_w8_hpad_16bpc_neon(x0, w6, w9, w4, v24, v25, v26, v27, v3, v3)
            // b L(ipred_cfl_ac_420_w8_hpad)
        }
        16 => {
            // L(ipred_cfl_ac_444_w16):
            let (mut v2, mut v3);
            if w3 == 0 {
                // cbnz w3, L(ipred_cfl_ac_444_w16_wpad)
                loop {
                    // 1: Copy and expand input, without padding
                    let t01 = vld1q_u16_x2(x1); // ld1 {v0.8h, v1.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t23 = vld1q_u16_x2(x10); // ld1 {v2.8h, v3.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let v0 = vshlq_n_u16::<3>(t01.0); // shl v0.8h, v0.8h, #3
                    let v1 = vshlq_n_u16::<3>(t01.1); // shl v1.8h, v1.8h, #3
                    v2 = vshlq_n_u16::<3>(t23.0); // shl v2.8h, v2.8h, #3
                    v3 = vshlq_n_u16::<3>(t23.1); // shl v3.8h, v3.8h, #3
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                }
            } else {
                // L(ipred_cfl_ac_444_w16_wpad):
                loop {
                    // 1: Copy and expand input, padding 8
                    let t0 = vld1q_u16(x1); // ld1 {v0.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    let t2 = vld1q_u16(x10); // ld1 {v2.8h}, [x10], x2
                    x10 = x10.wrapping_byte_offset(x2);
                    let v0 = vshlq_n_u16::<3>(t0); // shl v0.8h, v0.8h, #3
                    v2 = vshlq_n_u16::<3>(t2); // shl v2.8h, v2.8h, #3
                    let v1 = vdupq_laneq_u16::<7>(v0); // dup v1.8h, v0.h[7]
                    v3 = vdupq_laneq_u16::<7>(v2); // dup v3.8h, v2.h[7]
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                }
            }
            // mov v0.16b, v2.16b; mov v1.16b, v3.16b (the arguments)
            ipred_cfl_ac_420_w16_hpad_16bpc_neon(x0, w6, w9, w4, v24, v25, v26, v27, v2, v3, v2, v3)
            // b L(ipred_cfl_ac_420_w16_hpad)
        }
        _ => {
            // L(ipred_cfl_ac_444_w32):
            let (mut v0, mut v1, mut v2, mut v3);
            let x2 = x2 >> 1; // lsr x2, x2, #1: Restore the stride to one line increments

            // lsr w3, w3, #1; br x7 (ipred_cfl_ac_444_w32_tbl, by w_pad / 2)
            match w3 >> 1 {
                0 => loop {
                    // L(ipred_cfl_ac_444_w32_wpad0):
                    // 1: Copy and expand input, without padding
                    let t = vld1q_u16_x4(x1); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    v0 = vshlq_n_u16::<3>(t.0); // shl v0.8h, v0.8h, #3
                    v1 = vshlq_n_u16::<3>(t.1); // shl v1.8h, v1.8h, #3
                    v2 = vshlq_n_u16::<3>(t.2); // shl v2.8h, v2.8h, #3
                    v3 = vshlq_n_u16::<3>(t.3); // shl v3.8h, v3.8h, #3
                    let gt = pd_subs_gt(&mut w8, 1); // subs w8, w8, #1
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                1 => loop {
                    // L(ipred_cfl_ac_444_w32_wpad2):
                    // 1: Copy and expand input, padding 8
                    let t = vld1q_u16_x3(x1); // ld1 {v0.8h, v1.8h, v2.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    v2 = vshlq_n_u16::<3>(t.2); // shl v2.8h, v2.8h, #3
                    v0 = vshlq_n_u16::<3>(t.0); // shl v0.8h, v0.8h, #3
                    v1 = vshlq_n_u16::<3>(t.1); // shl v1.8h, v1.8h, #3
                    v3 = vdupq_laneq_u16::<7>(v2); // dup v3.8h, v2.h[7]
                    let gt = pd_subs_gt(&mut w8, 1); // subs w8, w8, #1
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                2 => loop {
                    // L(ipred_cfl_ac_444_w32_wpad4):
                    // 1: Copy and expand input, padding 16
                    let t = vld1q_u16_x2(x1); // ld1 {v0.8h, v1.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    v1 = vshlq_n_u16::<3>(t.1); // shl v1.8h, v1.8h, #3
                    v0 = vshlq_n_u16::<3>(t.0); // shl v0.8h, v0.8h, #3
                    v2 = vdupq_laneq_u16::<7>(v1); // dup v2.8h, v1.h[7]
                    v3 = v2; // dup v3.8h, v1.h[7]
                    let gt = pd_subs_gt(&mut w8, 1); // subs w8, w8, #1
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                _ => loop {
                    // L(ipred_cfl_ac_444_w32_wpad6):
                    // 1: Copy and expand input, padding 24
                    let t = vld1q_u16(x1); // ld1 {v0.8h}, [x1], x2
                    x1 = x1.wrapping_byte_offset(x2);
                    v0 = vshlq_n_u16::<3>(t); // shl v0.8h, v0.8h, #3
                    v1 = vdupq_laneq_u16::<7>(v0); // dup v1.8h, v0.h[7]
                    v2 = v1; // dup v2.8h, v0.h[7]
                    v3 = v1; // dup v3.8h, v0.h[7]
                    let gt = pd_subs_gt(&mut w8, 1); // subs w8, w8, #1
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
            }
            // L(ipred_cfl_ac_444_w32_hpad):
            let mut w4 = w4;
            if w4 != 0 {
                // cbz w4, 3f
                loop {
                    // 2: Vertical padding (h_pad > 0)
                    let gt = pd_subs_gt(&mut w4, 2); // subs w4, w4, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v0); // uaddw v24.4s, v24.4s, v0.4h; uaddw2 v25.4s, v25.4s, v0.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v1); // uaddw v26.4s, v26.4s, v1.4h; uaddw2 v27.4s, v27.4s, v1.8h
                    ipred_cfl_ac_uaddw(&mut v24, &mut v25, v2); // uaddw v24.4s, v24.4s, v2.4h; uaddw2 v25.4s, v25.4s, v2.8h
                    ipred_cfl_ac_uaddw(&mut v26, &mut v27, v3); // uaddw v26.4s, v26.4s, v3.4h; uaddw2 v27.4s, v27.4s, v3.8h
                    if !gt {
                        break; // b.gt 2b
                    }
                }
            }
            // 3:
            // Multiply the height by eight and reuse the w4 subtracting
            let w6 = w6 << 3; // lsl w6, w6, #3
            ipred_cfl_ac_420_w4_calc_subtract_dc_16bpc_neon(x0, w6, w9, v24, v25, v26, v27)
            // b L(ipred_cfl_ac_420_w4_calc_subtract_dc)
        }
    }
}
