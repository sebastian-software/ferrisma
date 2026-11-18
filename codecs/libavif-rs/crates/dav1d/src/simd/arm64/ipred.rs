// Copyright © 2018, VideoLAN and dav1d authors
// Copyright © 2019, Martin Storsjo
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/arm/64/ipred.S` (8 bpc) with NEON
//! intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062), in the order of the
//! assembly file. The entry points are the `pub unsafe extern "C" fn
//! dav1d_*_8bpc_neon`; the DSP slots and the C wrappers around the
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
//! From `ipred.S` lines 35–1296: `ipred_dc_128`, `ipred_v`, `ipred_h`,
//! `ipred_dc_top`, `ipred_dc_left`, `ipred_dc`, `ipred_paeth`, `ipred_smooth`,
//! `ipred_smooth_v`, `ipred_smooth_h` are the entry points
//! `dav1d_ipred_*_8bpc_neon`, one function each with the labels of its jump
//! table as `match` arms (`40:` … `640:`; for `ipred_dc_left`/`ipred_dc` a
//! first `match` on the height for `L(ipred_dc*_h*)` and a second on the width
//! for `L(ipred_dc*_w*)`, as the assembly's `br x5` and `br x3`). The store
//! loops that the DC predictors and `ipred_v` repeat
//! (`4:`/`8:`/`16:`/`32:`/`64:`) are the inlined functions
//! `ipred_dc_128_w{4,8,16,32,64}`. The 16+ column loops of `ipred_paeth`
//! (`160:`/`320:`/`640:`) and `ipred_smooth` (`160:`) are the functions
//! `ipred_paeth_160` and `ipred_smooth_160`, called by their entry points.
//! `pa_*` are instruction helpers: `pa_tbl_idx` (`clz`/`sub` of the dispatch),
//! `pa_subs_gt` (`subs` + `b.gt`), `pa_ldr_s`, `pa_ldr_s_ld1r`,
//! `pa_ldr_h_ld1r`, `pa_ld1r_2s`, `pa_uxtl_zip1`, `pa_st1_s`, `pa_st1_s_8b`,
//! `pa_uaddlv_8b`, `pa_uaddlv_16b`, `pa_add_uaddlv`, `pa_rshrn_dup`,
//! `pa_sqdmulh_4h`. Vector registers are typed locals named after the
//! assembly's registers; the deviations are in the comments at the functions (store addressing of
//! the four-row loops, the model of the `uaddlv` sums, `ld1r`/`zip1` for the
//! per-row `ldr`/`uxtl`, the duplicated tail of `ipred_dc`, the two outlined
//! loops).
//!
//! # Part B: the edge helpers and the z1 fill kernels
//!
//! From `ipred.S` lines 1297–1850: the constant `padding_mask_buf` with its
//! inner label `padding_mask` (`PADDING_MASK_BUF`, `padding_mask()`) and
//! `edge_filter` (`EDGE_FILTER`); the edge helpers
//! `dav1d_ipred_z1_upsample_edge_8bpc_neon`,
//! `dav1d_ipred_z2_upsample_edge_8bpc_neon`,
//! `dav1d_ipred_z1_filter_edge_8bpc_neon` (its `L(fivetap)` half is
//! `ipred_z1_filter_edge_fivetap`, the repeated "Filter one block" sequences
//! are `ipred_z1_filter_edge_tap3`/`_tap5`), `dav1d_ipred_pixel_set_8bpc_neon`
//! and `dav1d_ipred_reverse_8bpc_neon`; and the z1 fill kernels
//! `dav1d_ipred_z1_fill1_8bpc_neon` (the jump table `ipred_z1_fill1_tbl` is the
//! `match` on the width; the shared block `160:/320:/640:` is
//! `ipred_z1_fill1_160_320_640::<W>`, instantiated per width so that the
//! compiler sees the number of 16-pixel columns) and
//! `dav1d_ipred_z1_fill2_8bpc_neon`. These functions use typed locals named
//! after the assembly's registers instead of the register array. They are
//! called by the wrappers `ipred_z1_neon`, `ipred_z2_neon`, `ipred_z3_neon` of
//! `simd/arm/ipred.rs`. The helpers load and store whole 16-pixel blocks: they
//! read up to 30 pixels above `in[end]` and, at filter strength 3, `in[-1]`,
//! and write their output rounded up to 16 pixels; the wrappers' buffers are
//! sized and initialized for that (see `simd/arm/ipred.rs`), and the caller's
//! edge buffer must be the whole `edge[257]` array.
//!
//! # Part C: the z2 fill kernels
//!
//! Part C — the z2 fill kernels (directional prediction between 90 and 180
//! degrees; `ipred.S` lines 1851–3074): the constant `increments` is
//! `INCREMENTS`; `ipred_z2_fill1_8bpc_neon` is
//! `dav1d_ipred_z2_fill1_8bpc_neon`, whose jump table `ipred_z2_fill1_tbl` is a
//! `match` on the width and whose labels `40:`, `80:`, `160:` and `320:`/`640:`
//! are the functions `ipred_z2_fill1_w4`, `_w8`, `_w16` and `_w32` (the inner
//! labels `4:`/`49:`, `8:`/`89:`, `16:`/`169:`, `1:`–`4:`/`329:` are commented
//! loops inside them); `ipred_z2_fill2_8bpc_neon` (upsampled `top`) and
//! `ipred_z2_fill3_8bpc_neon` (upsampled `left`) are one function each with the
//! labels `40:` and `80:` as the two arms of the width test. The registers are
//! typed locals named after the assembly's (`v16`; `v16h` holds the 16-bit
//! products of a `umull`/`umlal` pair; `v0v1` is a `tbl` table). The saves of
//! `d8`–`d15` are the compiler's. Callers: `ipred_z2_neon` of
//! `simd/arm/ipred.rs` only, which passes `top` and `left` inside one zero-
//! initialized buffer with four elements in front; the kernels load whole
//! vectors around what it writes (see the entry points' safety comments).
//!
//! # Part E: the z3 fill kernels
//!
//! From `ipred.S` lines 3075–3685, the z3 fill kernels, called by
//! `ipred_z3_neon` of `simd/arm/ipred.rs` with the flipped, filtered or
//! upsampled left edge. `dav1d_ipred_z3_fill1_8bpc_neon`: the labels of
//! `ipred_z3_fill1_tbl` are `ipred_z3_fill1_w4` (`40:`), `_w8` (`80:`), `_w16`
//! (`160:`) and `_w32` (`320:/640:`), which look the edge up with `tbx` in the
//! first 32, 48 or 64 edge bytes (taken when `max_base_y <= 64`);
//! `L(ipred_z3_fill1_large_h16)` is `ipred_z3_fill1_large_h16::<H>`, one
//! instance per height 16, 32, 64, which fills two columns at a time from the
//! edge at each column's base and branches to `ipred_z3_fill_padding_neon`
//! (same name; its table's labels are `ipred_z3_fill_padding_cols::<N>`, and
//! `ipred_z3_fill_padding_wide` is inlined into it) for the columns whose base
//! is past the edge. `dav1d_ipred_z3_fill2_8bpc_neon` (upsampled edge) is
//! `ipred_z3_fill2_w4`/`_w8`. These functions use typed locals named after the
//! assembly's registers. Deviations that change the instruction count: the
//! 2-byte row stores of the column path are `st2` lane stores of the two result
//! registers instead of `zip` + `st1`; that path's row loop is unrolled per
//! height; the column pointers are kept instead of recomputed with `msub`;
//! `xtn` + `xtn2` is one `uzp1`. The column path loads `height + 16` edge bytes
//! per column, up to 15 past the range the wrapper pads; the wrapper zeroes 16
//! elements there.
//!
//! # Part D: the filter predictor, the palette predictor and chroma from luma
//!
//! > From `ipred.S` lines 3686–5232: the filter predictor
//! `dav1d_ipred_filter_8bpc_neon` (blocks `40:`, `80:` in the entry point,
//! `160:`/`320:` in `ipred_filter_w16_8bpc_neon`; `ipred_filter_tbl` is the
//! `match` on the width); the palette predictor `dav1d_pal_pred_8bpc_neon`
//! (`pal_pred_tbl`: `match` on `pd_tbl_idx`, one arm per width); chroma from
//! luma: `dav1d_ipred_cfl_128_8bpc_neon` with its three splat blocks
//! `L(ipred_cfl_splat_w4)`, `_w8`, `_w16` as
//! `ipred_cfl_splat_w{4,8,16}_8bpc_neon` (their per-register arithmetic is
//! `ipred_cfl_splat_px`; `ipred_cfl_splat_tbl`, used by `cfl_128` and
//! `cfl_left`, is `ipred_cfl_splat_tbl_8bpc`), `dav1d_ipred_cfl_top_8bpc_neon`,
//! `dav1d_ipred_cfl_left_8bpc_neon` and `dav1d_ipred_cfl_8bpc_neon`, which
//! compute the dc and branch into the splat blocks (tail calls); and
//! `dav1d_ipred_cfl_ac_{420,422,444}_8bpc_neon` (common prologue
//! `ipred_cfl_ac_head`; the width and `w_pad` tables are nested `match`es) with
//! the tails they share: `L(ipred_cfl_ac_420_w4_hpad)`,
//! `L(ipred_cfl_ac_420_w8_hpad)`, `L(ipred_cfl_ac_420_w16_hpad)`,
//! `L(ipred_cfl_ac_420_w8_calc_subtract_dc)`,
//! `L(ipred_cfl_ac_420_w8_subtract_dc)` as functions of those names (suffix
//! `_8bpc_neon`), each entered by a tail call; `L(ipred_cfl_ac_444_w32_hpad)`
//! is inline in the 4:4:4 entry point, its only user. The helpers `pd_*` are
//! single instructions without an intrinsic of their own (lane loads and stores
//! through unaligned scalar accesses, `uaddlv` into a vector lane, the
//! `subs`/`b.gt` loop counter) and the three forms of `uxtl` the filter needs
//! (see the deviations). Vector registers are typed locals named after the
//! assembly's registers.
use core::arch::aarch64::*;
use core::ffi::c_int;

// ============================================================================
// Part A (ipred.S lines 35–1296): the DC, V, H, Paeth and smooth predictors.
// ============================================================================
// Conventions of this part: general registers keep their assembly names (`x0`, `w4`, ...); vector registers are
// typed locals named after the registers (`v0`, `v16`), since the kernels are too small for a register array.
// A jump table (`br x5`) is a `match` on `clz(width) - 25`, with the labels as comments. A post-incremented
// address is the access followed by `wrapping_offset`/`wrapping_add`, because the last increment leaves the
// block. The helpers `pa_*` are single instructions (or pairs) that have no intrinsic of their own.
//
// Where the rows of a loop are stored through two pointers that the assembly post-increments by a register
// (`st1 {v0.8b}, [x0], x1; st1 {v0.8b}, [x6], x1`, twice), the second pair of rows is addressed as `[x0, x1]`,
// `[x6, x1]` and both pointers advance once per iteration (the form of `4:` in `ipred_v`): a plain store has no
// post-increment by a register, and with this form the compiler needs one add per iteration instead of four.
// The stores keep their order, and the comments name the assembly's instruction.

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

/// `ldr s0, [p]`: four bytes into a cleared register.
#[inline(always)]
unsafe fn pa_ldr_s(p: *const u8) -> uint8x16_t {
    vreinterpretq_u8_u32(vsetq_lane_u32::<0>(p.cast::<u32>().read_unaligned(), vdupq_n_u32(0)))
}

/// `ldr s0, [p]` inside a loop, for a register of which only the low four bytes are read afterwards: written as
/// the load-and-replicate `ld1r {v0.4s}, [p]`, which the compiler keeps in the vector unit (it loads a cleared
/// register through a general register, two more instructions per row).
#[inline(always)]
unsafe fn pa_ldr_s_ld1r(p: *const u8) -> uint8x16_t {
    vreinterpretq_u8_u32(vdupq_n_u32(p.cast::<u32>().read_unaligned()))
}

/// `ldr h0, [p]` inside a loop, for a register of which only the low two bytes are read afterwards: `ld1r
/// {v0.8h}, [p]`, as [`pa_ldr_s_ld1r`].
#[inline(always)]
unsafe fn pa_ldr_h_ld1r(p: *const u8) -> uint8x16_t {
    vreinterpretq_u8_u16(vdupq_n_u16(p.cast::<u16>().read_unaligned()))
}

/// `ld1r {v0.2s}, [p]`
#[inline(always)]
unsafe fn pa_ld1r_2s(p: *const u8) -> uint8x8_t {
    vreinterpret_u8_u32(vdup_n_u32(p.cast::<u32>().read_unaligned()))
}

/// `uxtl v1.8h, v1.8b` of a vector whose lanes are then used one by one (`v1.h[i]`): the bytes interleaved with
/// zero bytes (`zip1`), which is the same value. Through a widening cast the compiler replicates each byte lane
/// and widens it again instead of multiplying by the element, two more instructions per row.
#[inline(always)]
unsafe fn pa_uxtl_zip1(v: uint8x16_t) -> uint16x8_t {
    vreinterpretq_u16_u8(vzip1q_u8(v, vdupq_n_u8(0)))
}

/// `st1 {v0.s}[I], [p]` (also `str s0, [p]` with `I` = 0)
#[inline(always)]
unsafe fn pa_st1_s<const I: i32>(p: *mut u8, v: uint8x16_t) {
    p.cast::<u32>()
        .write_unaligned(vgetq_lane_u32::<I>(vreinterpretq_u32_u8(v)));
}

/// `st1 {v0.s}[I], [p]` of a 64-bit register (also `str s0, [p]` with `I` = 0)
#[inline(always)]
unsafe fn pa_st1_s_8b<const I: i32>(p: *mut u8, v: uint8x8_t) {
    p.cast::<u32>()
        .write_unaligned(vget_lane_u32::<I>(vreinterpret_u32_u8(v)));
}

// The sums of the DC predictors. The assembly computes them in lane 0 of 64-bit vectors (`add v0.4h, ...`);
// here the same instructions work on the whole register (`.8h`), of which only lane 0 is read, and a `uaddlv`
// result stands for "the sum in lane 0" with the other lanes unspecified (the scalar replicated). In this form
// the compiler takes the result of the `uaddlv` instruction as it is; a cleared 64-bit vector with the sum
// inserted costs it two more instructions per sum.

/// `uaddlv h0, v0.8b`: the sum in lane 0.
#[inline(always)]
unsafe fn pa_uaddlv_8b(v: uint8x8_t) -> uint16x8_t {
    vdupq_n_u16(vaddlv_u8(v))
}

/// `uaddlv h0, v0.16b`: the sum in lane 0.
#[inline(always)]
unsafe fn pa_uaddlv_16b(v: uint8x16_t) -> uint16x8_t {
    vdupq_n_u16(vaddlvq_u8(v))
}

/// `add v0.4h, v0.4h, v1.4h` with a fresh `uaddlv` result as an operand, written as the saturating `uqadd`: the
/// same value (the sums of at most 128 pixels and the rounding term are far below 0xffff), and the form the
/// compiler keeps in the vector unit (it moves a plain add of such scalars into general registers, two or
/// three more instructions).
#[inline(always)]
unsafe fn pa_add_uaddlv(a: uint16x8_t, b: uint16x8_t) -> uint16x8_t {
    vqaddq_u16(a, b)
}

/// `rshrn v0.8b, v0.8h, #N`, then `dup v0.8b, v0.b[0]`.
#[inline(always)]
unsafe fn pa_rshrn_dup<const N: i32>(v: uint16x8_t) -> uint8x8_t {
    vdup_lane_u8::<0>(vrshrn_n_u16::<N>(v))
}

/// `dup v16.4h, w16; sqdmulh v0.4h, v0.4h, v16.4h`
#[inline(always)]
unsafe fn pa_sqdmulh_4h(v: uint16x8_t, w16: i16) -> uint16x8_t {
    vreinterpretq_u16_s16(vqdmulhq_s16(vreinterpretq_s16_u16(v), vdupq_n_s16(w16)))
}

// The store loops of the DC predictors. `ipred_dc_128`, `ipred_dc_top`, `ipred_dc_left` (`L(ipred_dc_left_w*)`)
// and `ipred_dc` (`L(ipred_dc_w*)`, label `2:`) each end in the same five loops, and `ipred_v` in the last four;
// here they are one set of functions, inlined into each entry point as the assembly repeats them.
//
// The assembly stores every row with a post-increment by a register (`st1 {v0}, [x0], x1`), which exists for a
// plain store only with several registers (`st1 {v0, v1}`). The loops for 4, 8 and 16 columns therefore address
// the second pair of rows as `[x0, x1]`, `[x6, x1]` and advance both pointers once, as `4:` of `ipred_v` does.
// The loops for 32 and 64 columns keep the post-increments, on one pointer that visits the rows in the order of
// the assembly's two (`x0`, `x6` = `x0` + stride; here `x1` is the stride, not twice the stride).

/// `4:` of `ipred_dc_128` (`40:`), `ipred_dc_top`, `L(ipred_dc_left_w4)`, `L(ipred_dc_w4)`.
#[inline(always)]
unsafe fn ipred_dc_128_w4(mut x0: *mut u8, mut x6: *mut u8, x1: isize, mut w4: c_int, v0: uint8x8_t) {
    loop {
        pa_st1_s_8b::<0>(x0, v0); // st1 {v0.s}[0], [x0], x1
        pa_st1_s_8b::<0>(x6, v0); // st1 {v0.s}[0], [x6], x1
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        pa_st1_s_8b::<0>(x0.wrapping_offset(x1), v0); // st1 {v0.s}[0], [x0], x1
        pa_st1_s_8b::<0>(x6.wrapping_offset(x1), v0); // st1 {v0.s}[0], [x6], x1
        x0 = x0.wrapping_offset(x1 << 1);
        x6 = x6.wrapping_offset(x1 << 1);
        if !gt {
            break; // b.gt 4b
        }
    }
}

/// `8:` of `ipred_dc_128` (`80:`), `ipred_dc_top`, `ipred_v`, `L(ipred_dc_left_w8)`, `L(ipred_dc_w8)`.
#[inline(always)]
unsafe fn ipred_dc_128_w8(mut x0: *mut u8, mut x6: *mut u8, x1: isize, mut w4: c_int, v0: uint8x8_t) {
    loop {
        vst1_u8(x0, v0); // st1 {v0.8b}, [x0], x1
        vst1_u8(x6, v0); // st1 {v0.8b}, [x6], x1
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        vst1_u8(x0.wrapping_offset(x1), v0); // st1 {v0.8b}, [x0], x1
        vst1_u8(x6.wrapping_offset(x1), v0); // st1 {v0.8b}, [x6], x1
        x0 = x0.wrapping_offset(x1 << 1);
        x6 = x6.wrapping_offset(x1 << 1);
        if !gt {
            break; // b.gt 8b
        }
    }
}

/// `16:` of `ipred_dc_128` (`160:`), `ipred_dc_top`, `ipred_v`, `L(ipred_dc_left_w16)`, `L(ipred_dc_w16)`.
#[inline(always)]
unsafe fn ipred_dc_128_w16(mut x0: *mut u8, mut x6: *mut u8, x1: isize, mut w4: c_int, v0: uint8x16_t) {
    loop {
        vst1q_u8(x0, v0); // st1 {v0.16b}, [x0], x1
        vst1q_u8(x6, v0); // st1 {v0.16b}, [x6], x1
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        vst1q_u8(x0.wrapping_offset(x1), v0); // st1 {v0.16b}, [x0], x1
        vst1q_u8(x6.wrapping_offset(x1), v0); // st1 {v0.16b}, [x6], x1
        x0 = x0.wrapping_offset(x1 << 1);
        x6 = x6.wrapping_offset(x1 << 1);
        if !gt {
            break; // b.gt 16b
        }
    }
}

/// `32:` of `ipred_dc_128` (`320:`), `ipred_dc_top`, `ipred_v`, `L(ipred_dc_left_w32)`, `L(ipred_dc_w32)`.
#[inline(always)]
unsafe fn ipred_dc_128_w32(mut x0: *mut u8, x1: isize, mut w4: c_int, v01: uint8x16x2_t) {
    loop {
        vst1q_u8_x2(x0, v01); // st1 {v0.16b, v1.16b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        vst1q_u8_x2(x0, v01); // st1 {v0.16b, v1.16b}, [x6], x1
        x0 = x0.wrapping_offset(x1);
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        vst1q_u8_x2(x0, v01); // st1 {v0.16b, v1.16b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        vst1q_u8_x2(x0, v01); // st1 {v0.16b, v1.16b}, [x6], x1
        x0 = x0.wrapping_offset(x1);
        if !gt {
            break; // b.gt 32b
        }
    }
}

/// `64:` of `ipred_dc_128` (`640:`), `ipred_dc_top`, `ipred_v`, `L(ipred_dc_left_w64)`, `L(ipred_dc_w64)`.
#[inline(always)]
unsafe fn ipred_dc_128_w64(mut x0: *mut u8, x1: isize, mut w4: c_int, v0123: uint8x16x4_t) {
    loop {
        vst1q_u8_x4(x0, v0123); // st1 {v0.16b, v1.16b, v2.16b, v3.16b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        vst1q_u8_x4(x0, v0123); // st1 {v0.16b, v1.16b, v2.16b, v3.16b}, [x6], x1
        x0 = x0.wrapping_offset(x1);
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        vst1q_u8_x4(x0, v0123); // st1 {v0.16b, v1.16b, v2.16b, v3.16b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        vst1q_u8_x4(x0, v0123); // st1 {v0.16b, v1.16b, v2.16b, v3.16b}, [x6], x1
        x0 = x0.wrapping_offset(x1);
        if !gt {
            break; // b.gt 64b
        }
    }
}

/// `ipred_dc_128_8bpc_neon`
///
/// # Safety
/// Writes `dst` rows `0..height`, columns `0..width`; reads no memory. `width` is 4, 8, 16, 32 or 64 and
/// `height` a multiple of 4.
pub unsafe extern "C" fn dav1d_ipred_dc_128_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    _topleft: *const u8,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let (x0, x1, w4) = (dst, stride, height);
    let v0 = vdupq_n_u8(128); // movi v0.16b, #128
    let x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    match pa_tbl_idx(width) {
        4 => ipred_dc_128_w4(x0, x6, x1, w4, vget_low_u8(v0)),       // 40:
        3 => ipred_dc_128_w8(x0, x6, x1, w4, vget_low_u8(v0)),       // 80:
        2 => ipred_dc_128_w16(x0, x6, x1, w4, v0),                   // 160:
        1 => ipred_dc_128_w32(x0, stride, w4, uint8x16x2_t(v0, v0)), // 320: movi v1.16b, #128
        0 => ipred_dc_128_w64(x0, stride, w4, uint8x16x4_t(v0, v0, v0, v0)), // 640: movi v1/v2/v3.16b, #128
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `ipred_v_8bpc_neon`
///
/// # Safety
/// Reads `topleft[1..=width]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_v_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let (mut x0, x1, mut w4) = (dst, stride, height);
    let x2 = topleft.add(1); // add x2, x2, #1
    let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    match pa_tbl_idx(width) {
        4 => {
            // 40:
            let v0 = pa_ldr_s(x2); // ldr s0, [x2]
            loop {
                // 4:
                pa_st1_s::<0>(x0, v0); // str s0, [x0]
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                pa_st1_s::<0>(x6, v0); // str s0, [x6]
                pa_st1_s::<0>(x0.wrapping_offset(x1), v0); // str s0, [x0, x1]
                x0 = x0.wrapping_offset(x1 << 1); // add x0, x0, x1, lsl #1
                pa_st1_s::<0>(x6.wrapping_offset(x1), v0); // str s0, [x6, x1]
                x6 = x6.wrapping_offset(x1 << 1); // add x6, x6, x1, lsl #1
                if !gt {
                    break; // b.gt 4b
                }
            }
        }
        3 => ipred_dc_128_w8(x0, x6, x1, w4, vld1_u8(x2)), // 80: ld1 {v0.8b}, [x2]; 8:
        2 => ipred_dc_128_w16(x0, x6, x1, w4, vld1q_u8(x2)), // 160: ld1 {v0.16b}, [x2]; 16:
        1 => ipred_dc_128_w32(x0, stride, w4, vld1q_u8_x2(x2)), // 320: ld1 {v0.16b, v1.16b}, [x2]; 32:
        0 => ipred_dc_128_w64(x0, stride, w4, vld1q_u8_x4(x2)), // 640: ld1 {v0.16b, ..., v3.16b}, [x2]; 64:
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `ipred_h_8bpc_neon`
///
/// The loops for 4, 8 and 16 columns address the rows `[x0, x1]` and `[x0, x8]` from a second pointer `x0x1`
/// (= `x0` + stride) as `[x0x1]` and `[x0x1, x6]`: with one pointer the compiler steps from row to row with an
/// add each (three more instructions per four rows).
///
/// # Safety
/// Reads `topleft[-height..0]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_h_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let (mut x0, x1, mut w4) = (dst, stride, height);
    let mut x2 = topleft.sub(4); // sub x2, x2, #4; mov x7, #-4
    let x6 = x1 + x1; // add x6, x1, x1 (2 * stride, 4..16 blocks only); add x8, x1, x1, lsl #1: see above
    match pa_tbl_idx(width) {
        4 => {
            // 40:
            let mut x0x1 = x0.wrapping_offset(x1);
            loop {
                // 4:
                let uint8x8x4_t(v0, v1, v2, v3) = vld4_dup_u8(x2); // ld4r {v0.8b, v1.8b, v2.8b, v3.8b}, [x2], x7
                x2 = x2.wrapping_sub(4);
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                pa_st1_s_8b::<0>(x0, v3); // str s3, [x0]
                pa_st1_s_8b::<0>(x0x1, v2); // str s2, [x0, x1]
                pa_st1_s_8b::<0>(x0.wrapping_offset(x6), v1); // str s1, [x0, x6]
                pa_st1_s_8b::<0>(x0x1.wrapping_offset(x6), v0); // str s0, [x0, x8]
                x0 = x0.wrapping_offset(x1 << 2); // add x0, x0, x1, lsl #2
                x0x1 = x0x1.wrapping_offset(x1 << 2);
                if !gt {
                    break; // b.gt 4b
                }
            }
        }
        3 => {
            // 80:
            let mut x0x1 = x0.wrapping_offset(x1);
            loop {
                // 8:
                let uint8x8x4_t(v0, v1, v2, v3) = vld4_dup_u8(x2); // ld4r {v0.8b, v1.8b, v2.8b, v3.8b}, [x2], x7
                x2 = x2.wrapping_sub(4);
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                vst1_u8(x0, v3); // str d3, [x0]
                vst1_u8(x0x1, v2); // str d2, [x0, x1]
                vst1_u8(x0.wrapping_offset(x6), v1); // str d1, [x0, x6]
                vst1_u8(x0x1.wrapping_offset(x6), v0); // str d0, [x0, x8]
                x0 = x0.wrapping_offset(x1 << 2); // add x0, x0, x1, lsl #2
                x0x1 = x0x1.wrapping_offset(x1 << 2);
                if !gt {
                    break; // b.gt 8b
                }
            }
        }
        2 => {
            // 160:
            let mut x0x1 = x0.wrapping_offset(x1);
            loop {
                // 16:
                let uint8x16x4_t(v0, v1, v2, v3) = vld4q_dup_u8(x2); // ld4r {v0.16b, v1.16b, v2.16b, v3.16b}, [x2], x7
                x2 = x2.wrapping_sub(4);
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                vst1q_u8(x0, v3); // str q3, [x0]
                vst1q_u8(x0x1, v2); // str q2, [x0, x1]
                vst1q_u8(x0.wrapping_offset(x6), v1); // str q1, [x0, x6]
                vst1q_u8(x0x1.wrapping_offset(x6), v0); // str q0, [x0, x8]
                x0 = x0.wrapping_offset(x1 << 2); // add x0, x0, x1, lsl #2
                x0x1 = x0x1.wrapping_offset(x1 << 2);
                if !gt {
                    break; // b.gt 16b
                }
            }
        }
        1 => {
            // 320: add x6, x0, x1 (the two row pointers advance as one here: `x0`, by the stride)
            loop {
                // 32:
                let uint8x16x4_t(v0, v1, v2, v3) = vld4q_dup_u8(x2); // ld4r {v0.16b, v1.16b, v2.16b, v3.16b}, [x2], x7
                x2 = x2.wrapping_sub(4);
                vst1q_u8(x0, v3); // stp q3, q3, [x0]
                vst1q_u8(x0.add(16), v3);
                x0 = x0.wrapping_offset(x1); // add x0, x0, x1, lsl #1
                vst1q_u8(x0, v2); // stp q2, q2, [x6]
                vst1q_u8(x0.add(16), v2);
                x0 = x0.wrapping_offset(x1); // add x6, x6, x1, lsl #1
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                vst1q_u8(x0, v1); // stp q1, q1, [x0]
                vst1q_u8(x0.add(16), v1);
                x0 = x0.wrapping_offset(x1); // add x0, x0, x1, lsl #1
                vst1q_u8(x0, v0); // stp q0, q0, [x6]
                vst1q_u8(x0.add(16), v0);
                x0 = x0.wrapping_offset(x1); // add x6, x6, x1, lsl #1
                if !gt {
                    break; // b.gt 32b
                }
            }
        }
        0 => {
            // 640: add x6, x0, x1 (the two row pointers advance as one here: `x0`, by the stride)
            loop {
                // 64:
                let uint8x16x4_t(v0, v1, v2, v3) = vld4q_dup_u8(x2); // ld4r {v0.16b, v1.16b, v2.16b, v3.16b}, [x2], x7
                x2 = x2.wrapping_sub(4);
                vst1q_u8(x0, v3); // stp q3, q3, [x0]
                vst1q_u8(x0.add(16), v3);
                vst1q_u8(x0.add(32), v3); // stp q3, q3, [x0, #32]
                vst1q_u8(x0.add(48), v3);
                x0 = x0.wrapping_offset(x1); // add x0, x0, x1, lsl #1
                vst1q_u8(x0, v2); // stp q2, q2, [x6]
                vst1q_u8(x0.add(16), v2);
                vst1q_u8(x0.add(32), v2); // stp q2, q2, [x6, #32]
                vst1q_u8(x0.add(48), v2);
                x0 = x0.wrapping_offset(x1); // add x6, x6, x1, lsl #1
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                vst1q_u8(x0, v1); // stp q1, q1, [x0]
                vst1q_u8(x0.add(16), v1);
                vst1q_u8(x0.add(32), v1); // stp q1, q1, [x0, #32]
                vst1q_u8(x0.add(48), v1);
                x0 = x0.wrapping_offset(x1); // add x0, x0, x1, lsl #1
                vst1q_u8(x0, v0); // stp q0, q0, [x6]
                vst1q_u8(x0.add(16), v0);
                vst1q_u8(x0.add(32), v0); // stp q0, q0, [x6, #32]
                vst1q_u8(x0.add(48), v0);
                x0 = x0.wrapping_offset(x1); // add x6, x6, x1, lsl #1
                if !gt {
                    break; // b.gt 64b
                }
            }
        }
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `ipred_dc_top_8bpc_neon`
///
/// # Safety
/// Reads `topleft[1..=width]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_dc_top_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let (x0, x1, w4) = (dst, stride, height);
    let x2 = topleft.add(1); // add x2, x2, #1
    let x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    match pa_tbl_idx(width) {
        4 => {
            // 40:
            let v0 = pa_ld1r_2s(x2); // ld1r {v0.2s}, [x2]
            let v0 = pa_uaddlv_8b(v0); // uaddlv h0, v0.8b
            let v0 = pa_rshrn_dup::<3>(v0); // rshrn v0.8b, v0.8h, #3; dup v0.8b, v0.b[0]
            ipred_dc_128_w4(x0, x6, x1, w4, v0) // 4:
        }
        3 => {
            // 80:
            let v0 = vld1_u8(x2); // ld1 {v0.8b}, [x2]
            let v0 = pa_uaddlv_8b(v0); // uaddlv h0, v0.8b
            let v0 = pa_rshrn_dup::<3>(v0); // rshrn v0.8b, v0.8h, #3; dup v0.8b, v0.b[0]
            ipred_dc_128_w8(x0, x6, x1, w4, v0) // 8:
        }
        2 => {
            // 160:
            let v0 = vld1q_u8(x2); // ld1 {v0.16b}, [x2]
            let v0 = pa_uaddlv_16b(v0); // uaddlv h0, v0.16b
            let v0 = pa_rshrn_dup::<4>(v0); // rshrn v0.8b, v0.8h, #4
            let v0 = vcombine_u8(v0, v0); // dup v0.16b, v0.b[0]
            ipred_dc_128_w16(x0, x6, x1, w4, v0) // 16:
        }
        1 => {
            // 320:
            let uint8x16x2_t(v0, v1) = vld1q_u8_x2(x2); // ld1 {v0.16b, v1.16b}, [x2]
            let v0 = pa_uaddlv_16b(v0); // uaddlv h0, v0.16b
            let v1 = pa_uaddlv_16b(v1); // uaddlv h1, v1.16b
            let v2 = pa_add_uaddlv(v0, v1); // add v2.4h, v0.4h, v1.4h
            let v2 = pa_rshrn_dup::<5>(v2); // rshrn v2.8b, v2.8h, #5
            let v0 = vcombine_u8(v2, v2); // dup v0.16b, v2.b[0]; dup v1.16b, v2.b[0]
            ipred_dc_128_w32(x0, stride, w4, uint8x16x2_t(v0, v0))
            // 32:
        }
        0 => {
            // 640:
            let uint8x16x4_t(v0, v1, v2, v3) = vld1q_u8_x4(x2); // ld1 {v0.16b, v1.16b, v2.16b, v3.16b}, [x2]
            let v0 = pa_uaddlv_16b(v0); // uaddlv h0, v0.16b
            let v1 = pa_uaddlv_16b(v1); // uaddlv h1, v1.16b
            let v2 = pa_uaddlv_16b(v2); // uaddlv h2, v2.16b
            let v3 = pa_uaddlv_16b(v3); // uaddlv h3, v3.16b
            let v4 = pa_add_uaddlv(v0, v1); // add v4.4h, v0.4h, v1.4h
            let v5 = pa_add_uaddlv(v2, v3); // add v5.4h, v2.4h, v3.4h
            let v4 = vaddq_u16(v4, v5); // add v4.4h, v4.4h, v5.4h
            let v4 = pa_rshrn_dup::<6>(v4); // rshrn v4.8b, v4.8h, #6
            let v0 = vcombine_u8(v4, v4); // dup v0.16b, v4.b[0]; ...; dup v3.16b, v4.b[0]
            ipred_dc_128_w64(x0, stride, w4, uint8x16x4_t(v0, v0, v0, v0))
            // 64:
        }
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `ipred_dc_left_8bpc_neon`
///
/// # Safety
/// Reads `topleft[-height..0]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_dc_left_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let (x0, x1, w3, w4) = (dst, stride, width, height);
    let x2 = topleft.sub(w4 as u32 as usize); // sub x2, x2, w4, uxtw
    let x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
                      // br x5: the height
    let v0: uint8x8_t = match pa_tbl_idx(w4) {
        4 => {
            // L(ipred_dc_left_h4):
            let v0 = pa_ld1r_2s(x2); // ld1r {v0.2s}, [x2]
            let v0 = pa_uaddlv_8b(v0); // uaddlv h0, v0.8b
            pa_rshrn_dup::<3>(v0) // rshrn v0.8b, v0.8h, #3
        }
        3 => {
            // L(ipred_dc_left_h8):
            let v0 = vld1_u8(x2); // ld1 {v0.8b}, [x2]
            let v0 = pa_uaddlv_8b(v0); // uaddlv h0, v0.8b
            pa_rshrn_dup::<3>(v0) // rshrn v0.8b, v0.8h, #3
        }
        2 => {
            // L(ipred_dc_left_h16):
            let v0 = vld1q_u8(x2); // ld1 {v0.16b}, [x2]
            let v0 = pa_uaddlv_16b(v0); // uaddlv h0, v0.16b
            pa_rshrn_dup::<4>(v0) // rshrn v0.8b, v0.8h, #4
        }
        1 => {
            // L(ipred_dc_left_h32):
            let uint8x16x2_t(v0, v1) = vld1q_u8_x2(x2); // ld1 {v0.16b, v1.16b}, [x2]
            let v0 = pa_uaddlv_16b(v0); // uaddlv h0, v0.16b
            let v1 = pa_uaddlv_16b(v1); // uaddlv h1, v1.16b
            let v0 = pa_add_uaddlv(v0, v1); // add v0.4h, v0.4h, v1.4h
            pa_rshrn_dup::<5>(v0) // rshrn v0.8b, v0.8h, #5
        }
        0 => {
            // L(ipred_dc_left_h64):
            let uint8x16x4_t(v0, v1, v2, v3) = vld1q_u8_x4(x2); // ld1 {v0.16b, v1.16b, v2.16b, v3.16b}, [x2]
            let v0 = pa_uaddlv_16b(v0); // uaddlv h0, v0.16b
            let v1 = pa_uaddlv_16b(v1); // uaddlv h1, v1.16b
            let v2 = pa_uaddlv_16b(v2); // uaddlv h2, v2.16b
            let v3 = pa_uaddlv_16b(v3); // uaddlv h3, v3.16b
            let v0 = pa_add_uaddlv(v0, v1); // add v0.4h, v0.4h, v1.4h
            let v2 = pa_add_uaddlv(v2, v3); // add v2.4h, v2.4h, v3.4h
            let v0 = vaddq_u16(v0, v2); // add v0.4h, v0.4h, v2.4h
            pa_rshrn_dup::<6>(v0) // rshrn v0.8b, v0.8h, #6
        }
        _ => core::hint::unreachable_unchecked(),
    };
    let v0q = vcombine_u8(v0, v0); // dup v0.16b, v0.b[0]
                                   // br x3: the width
    match pa_tbl_idx(w3) {
        4 => ipred_dc_128_w4(x0, x6, x1, w4, v0),   // L(ipred_dc_left_w4):
        3 => ipred_dc_128_w8(x0, x6, x1, w4, v0),   // L(ipred_dc_left_w8):
        2 => ipred_dc_128_w16(x0, x6, x1, w4, v0q), // L(ipred_dc_left_w16):
        1 => ipred_dc_128_w32(x0, stride, w4, uint8x16x2_t(v0q, v0q)), // L(ipred_dc_left_w32): mov v1.16b, v0.16b
        0 => ipred_dc_128_w64(x0, stride, w4, uint8x16x4_t(v0q, v0q, v0q, v0q)), // L(ipred_dc_left_w64): mov v1/v2/v3
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `ipred_dc_8bpc_neon`
///
/// # Safety
/// Reads `topleft[-height..0]` and `topleft[1..=width]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_dc_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let (x0, x1, w3, w4) = (dst, stride, width, height);
    let x2 = topleft.sub(w4 as u32 as usize); // sub x2, x2, w4, uxtw
    let w7 = (w3 + w4) as u32; // add w7, w3, w4 (width + height)
    let v16 = vdupq_n_u16(w7 as u16); // dup v16.8h, w7
    let w7 = w7.reverse_bits().leading_zeros(); // rbit w7, w7; clz w7, w7 (ctz(width + height))
    let w7 = (w7 as i32).wrapping_neg(); // neg w7, w7
    let v16 = vshrq_n_u16::<1>(v16); // ushr v16.8h, v16.8h, #1 ((width + height) >> 1)
    let v17 = vdupq_n_s16(w7 as i16); // dup v17.8h, w7 (-ctz(width + height))
    let x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    const M3: i16 = 0x3334 / 2;
    const M5: i16 = 0x5556 / 2;
    // br x5: the height; each label leaves the left sum in h0 and x2 at topleft + 1
    let (v0, x2): (uint16x8_t, *const u8) = match pa_tbl_idx(w4) {
        4 => {
            // L(ipred_dc_h4):
            let v0 = vget_low_u8(pa_ldr_s(x2)); // ld1 {v0.s}[0], [x2], #4; ins v0.s[1], wzr
            let v0 = pa_uaddlv_8b(v0); // uaddlv h0, v0.8b
            (v0, x2.add(4 + 1)) // add x2, x2, #1
        }
        3 => {
            // L(ipred_dc_h8):
            let v0 = vld1_u8(x2); // ld1 {v0.8b}, [x2], #8
            let v0 = pa_uaddlv_8b(v0); // uaddlv h0, v0.8b
            (v0, x2.add(8 + 1)) // add x2, x2, #1
        }
        2 => {
            // L(ipred_dc_h16):
            let v0 = vld1q_u8(x2); // ld1 {v0.16b}, [x2], #16
            let v0 = pa_uaddlv_16b(v0); // uaddlv h0, v0.16b
            (v0, x2.add(16 + 1)) // add x2, x2, #1
        }
        1 => {
            // L(ipred_dc_h32):
            let uint8x16x2_t(v0, v1) = vld1q_u8_x2(x2); // ld1 {v0.16b, v1.16b}, [x2], #32
            let v0 = pa_uaddlv_16b(v0); // uaddlv h0, v0.16b
            let v1 = pa_uaddlv_16b(v1); // uaddlv h1, v1.16b
            let v0 = pa_add_uaddlv(v0, v1); // add v0.4h, v0.4h, v1.4h
            (v0, x2.add(32 + 1)) // add x2, x2, #1
        }
        0 => {
            // L(ipred_dc_h64):
            let uint8x16x4_t(v0, v1, v2, v3) = vld1q_u8_x4(x2); // ld1 {v0.16b, v1.16b, v2.16b, v3.16b}, [x2], #64
            let v0 = pa_uaddlv_16b(v0); // uaddlv h0, v0.16b
            let v1 = pa_uaddlv_16b(v1); // uaddlv h1, v1.16b
            let v2 = pa_uaddlv_16b(v2); // uaddlv h2, v2.16b
            let v3 = pa_uaddlv_16b(v3); // uaddlv h3, v3.16b
            let v0 = pa_add_uaddlv(v0, v1); // add v0.4h, v0.4h, v1.4h
            let v2 = pa_add_uaddlv(v2, v3); // add v2.4h, v2.4h, v3.4h
            let v0 = vaddq_u16(v0, v2); // add v0.4h, v0.4h, v2.4h
            (v0, x2.add(64 + 1)) // add x2, x2, #1
        }
        _ => core::hint::unreachable_unchecked(),
    };
    // br x3: the width. The code from `1:` on stands on both sides of `b.eq 1f`: where the paths join again, the
    // compiler computes the multiplication on both and selects (six instructions more for the square blocks).
    match pa_tbl_idx(w3) {
        4 => {
            // L(ipred_dc_w4):
            let v1 = vget_low_u8(pa_ldr_s(x2)); // ld1 {v1.s}[0], [x2]; ins v1.s[1], wzr
            let v0 = vaddq_u16(v0, v16); // add v0.4h, v0.4h, v16.4h
            let v1 = pa_uaddlv_8b(v1); // uaddlv h1, v1.8b
            let v0 = pa_add_uaddlv(v0, v1); // add v0.4h, v0.4h, v1.4h
            let v0 = vshlq_u16(v0, v17); // ushl v0.4h, v0.4h, v17.4h
            if w4 == 4 {
                // cmp w4, #4; b.eq 1f
                let v0 = vdup_laneq_u8::<0>(vreinterpretq_u8_u16(v0)); // 1:
                return ipred_dc_128_w4(x0, x6, x1, w4, v0); // 2:
            }
            // h = 8/16
            let w16 = ((M3 as u32) | (M5 as u32) << 16).wrapping_shr((w4 + w4) as u32) as i16; // mov w16, #(0x3334/2); movk w16, #(0x5556/2), lsl #16; add w17, w4, w4; lsr w16, w16, w17
            let v0 = pa_sqdmulh_4h(v0, w16); // dup v16.4h, w16; sqdmulh v0.4h, v0.4h, v16.4h
            let v0 = vdup_laneq_u8::<0>(vreinterpretq_u8_u16(v0)); // 1: dup v0.8b, v0.b[0]
            ipred_dc_128_w4(x0, x6, x1, w4, v0) // 2:
        }
        3 => {
            // L(ipred_dc_w8):
            let v1 = vld1_u8(x2); // ld1 {v1.8b}, [x2]
            let v0 = vaddq_u16(v0, v16); // add v0.4h, v0.4h, v16.4h
            let v1 = pa_uaddlv_8b(v1); // uaddlv h1, v1.8b
            let v0 = pa_add_uaddlv(v0, v1); // add v0.4h, v0.4h, v1.4h
            let v0 = vshlq_u16(v0, v17); // ushl v0.4h, v0.4h, v17.4h
            if w4 == 8 {
                // cmp w4, #8; b.eq 1f
                let v0 = vdup_laneq_u8::<0>(vreinterpretq_u8_u16(v0)); // 1:
                return ipred_dc_128_w8(x0, x6, x1, w4, v0); // 2:
            }
            // h = 4/16/32
            let w16 = if w4 == 32 { M3 } else { M5 }; // cmp w4, #32; mov w16, #(0x3334/2); mov w17, #(0x5556/2); csel w16, w16, w17, eq
            let v0 = pa_sqdmulh_4h(v0, w16); // dup v16.4h, w16; sqdmulh v0.4h, v0.4h, v16.4h
            let v0 = vdup_laneq_u8::<0>(vreinterpretq_u8_u16(v0)); // 1: dup v0.8b, v0.b[0]
            ipred_dc_128_w8(x0, x6, x1, w4, v0) // 2:
        }
        2 => {
            // L(ipred_dc_w16):
            let v1 = vld1q_u8(x2); // ld1 {v1.16b}, [x2]
            let v0 = vaddq_u16(v0, v16); // add v0.4h, v0.4h, v16.4h
            let v1 = pa_uaddlv_16b(v1); // uaddlv h1, v1.16b
            let v0 = pa_add_uaddlv(v0, v1); // add v0.4h, v0.4h, v1.4h
            let v0 = vshlq_u16(v0, v17); // ushl v0.4h, v0.4h, v17.4h
            if w4 == 16 {
                // cmp w4, #16; b.eq 1f
                let v0 = vdupq_laneq_u8::<0>(vreinterpretq_u8_u16(v0)); // 1:
                return ipred_dc_128_w16(x0, x6, x1, w4, v0); // 2:
            }
            // h = 4/8/32/64
            let w16 = if w4 & (32 + 16 + 8) == 0 { M3 } else { M5 }; // tst w4, #(32+16+8); mov w16, #(0x3334/2); mov w17, #(0x5556/2); csel w16, w16, w17, eq
            let v0 = pa_sqdmulh_4h(v0, w16); // dup v16.4h, w16; sqdmulh v0.4h, v0.4h, v16.4h
            let v0 = vdupq_laneq_u8::<0>(vreinterpretq_u8_u16(v0)); // 1: dup v0.16b, v0.b[0]
            ipred_dc_128_w16(x0, x6, x1, w4, v0) // 2:
        }
        1 => {
            // L(ipred_dc_w32):
            let uint8x16x2_t(v1, v2) = vld1q_u8_x2(x2); // ld1 {v1.16b, v2.16b}, [x2]
            let v0 = vaddq_u16(v0, v16); // add v0.4h, v0.4h, v16.4h
            let v1 = pa_uaddlv_16b(v1); // uaddlv h1, v1.16b
            let v2 = pa_uaddlv_16b(v2); // uaddlv h2, v2.16b
            let v0 = pa_add_uaddlv(v0, v1); // add v0.4h, v0.4h, v1.4h
            let v0 = pa_add_uaddlv(v0, v2); // add v0.4h, v0.4h, v2.4h
            let v4 = vshlq_u16(v0, v17); // ushl v4.4h, v0.4h, v17.4h
            if w4 == 32 {
                // cmp w4, #32; b.eq 1f
                let v0 = vdupq_laneq_u8::<0>(vreinterpretq_u8_u16(v4)); // 1:
                return ipred_dc_128_w32(x0, stride, w4, uint8x16x2_t(v0, v0)); // 2:
            }
            // h = 8/16/64
            let w16 = if w4 == 8 { M3 } else { M5 }; // cmp w4, #8; mov w16, #(0x3334/2); mov w17, #(0x5556/2); csel w16, w16, w17, eq
            let v4 = pa_sqdmulh_4h(v4, w16); // dup v16.4h, w16; sqdmulh v4.4h, v4.4h, v16.4h
            let v0 = vdupq_laneq_u8::<0>(vreinterpretq_u8_u16(v4)); // 1: dup v0.16b, v4.b[0] (and v1)
            ipred_dc_128_w32(x0, stride, w4, uint8x16x2_t(v0, v0)) // 2:
        }
        0 => {
            // L(ipred_dc_w64):
            let uint8x16x4_t(v1, v2, v3, v4) = vld1q_u8_x4(x2); // ld1 {v1.16b, v2.16b, v3.16b, v4.16b}, [x2]
            let v0 = vaddq_u16(v0, v16); // add v0.4h, v0.4h, v16.4h
            let v1 = pa_uaddlv_16b(v1); // uaddlv h1, v1.16b
            let v2 = pa_uaddlv_16b(v2); // uaddlv h2, v2.16b
            let v3 = pa_uaddlv_16b(v3); // uaddlv h3, v3.16b
            let v4 = pa_uaddlv_16b(v4); // uaddlv h4, v4.16b
            let v1 = pa_add_uaddlv(v1, v2); // add v1.4h, v1.4h, v2.4h
            let v3 = pa_add_uaddlv(v3, v4); // add v3.4h, v3.4h, v4.4h
            let v0 = vaddq_u16(v0, v1); // add v0.4h, v0.4h, v1.4h
            let v0 = vaddq_u16(v0, v3); // add v0.4h, v0.4h, v3.4h
            let v4 = vshlq_u16(v0, v17); // ushl v4.4h, v0.4h, v17.4h
            if w4 == 64 {
                // cmp w4, #64; b.eq 1f
                let v0 = vdupq_laneq_u8::<0>(vreinterpretq_u8_u16(v4)); // 1:
                return ipred_dc_128_w64(x0, stride, w4, uint8x16x4_t(v0, v0, v0, v0));
                // 2:
            }
            // h = 16/32
            let w16 = ((M5 as u32) | (M3 as u32) << 16).wrapping_shr(w4 as u32) as i16; // mov w16, #(0x5556/2); movk w16, #(0x3334/2), lsl #16; lsr w16, w16, w4
            let v4 = pa_sqdmulh_4h(v4, w16); // dup v16.4h, w16; sqdmulh v4.4h, v4.4h, v16.4h
            let v0 = vdupq_laneq_u8::<0>(vreinterpretq_u8_u16(v4)); // 1: dup v0.16b, v4.b[0] (and v1, v2, v3)
            ipred_dc_128_w64(x0, stride, w4, uint8x16x4_t(v0, v0, v0, v0)) // 2:
        }
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `160:`/`320:`/`640:` of `ipred_paeth_8bpc_neon`: the loop over four rows and 16 columns at a time.
///
/// # Safety
/// As the entry point; `x2` = `topleft - 4`, `x6` = `x0` + stride, `x1` = 2 * stride, `x8` = `topleft + 1`,
/// `v4` = the top left pixel in every byte.
unsafe fn ipred_paeth_160(
    mut x0: *mut u8,
    mut x1: isize,
    mut x2: *const u8,
    mut w3: c_int,
    mut w4: c_int,
    mut x6: *mut u8,
    mut x8: *const u8,
    v4: uint8x16_t,
) {
    let mut v5 = vld1q_u8(x8); // ld1 {v5.16b}, [x8], #16
    x8 = x8.wrapping_add(16);
    let w9 = w3; // mov w9, w3
                 // Set up pointers for four rows in parallel; x0, x6, x5, x10
    let mut x5 = x0.wrapping_offset(x1); // add x5, x0, x1
    let mut x10 = x6.wrapping_offset(x1); // add x10, x6, x1
    x1 <<= 1; // lsl x1, x1, #1
    x1 -= w3 as u32 as isize; // sub x1, x1, w3, uxtw
    loop {
        // 1:
        let uint8x16x4_t(v0, v1, v2, v3) = vld4q_dup_u8(x2); // ld4r {v0.16b, v1.16b, v2.16b, v3.16b}, [x2], x7
        x2 = x2.wrapping_sub(4);
        loop {
            // 2:
            let v6 = vsubl_u8(vget_low_u8(v5), vget_low_u8(v4)); // usubl v6.8h, v5.8b, v4.8b (top - topleft)
            let v7 = vsubl_high_u8(v5, v4); // usubl2 v7.8h, v5.16b, v4.16b
            let v24 = vaddw_u8(v6, vget_low_u8(v0)); // uaddw v24.8h, v6.8h, v0.8b
            let v25 = vaddw_u8(v7, vget_low_u8(v0)); // uaddw v25.8h, v7.8h, v0.8b
            let v26 = vaddw_u8(v6, vget_low_u8(v1)); // uaddw v26.8h, v6.8h, v1.8b
            let v27 = vaddw_u8(v7, vget_low_u8(v1)); // uaddw v27.8h, v7.8h, v1.8b
            let v28 = vaddw_u8(v6, vget_low_u8(v2)); // uaddw v28.8h, v6.8h, v2.8b
            let v29 = vaddw_u8(v7, vget_low_u8(v2)); // uaddw v29.8h, v7.8h, v2.8b
            let v30 = vaddw_u8(v6, vget_low_u8(v3)); // uaddw v30.8h, v6.8h, v3.8b
            let v31 = vaddw_u8(v7, vget_low_u8(v3)); // uaddw v31.8h, v7.8h, v3.8b
            let v17 = vqmovun_s16(vreinterpretq_s16_u16(v26)); // sqxtun v17.8b, v26.8h (base)
            let v17 = vqmovun_high_s16(v17, vreinterpretq_s16_u16(v27)); // sqxtun2 v17.16b, v27.8h
            let v16 = vqmovun_s16(vreinterpretq_s16_u16(v24)); // sqxtun v16.8b, v24.8h
            let v16 = vqmovun_high_s16(v16, vreinterpretq_s16_u16(v25)); // sqxtun2 v16.16b, v25.8h
            let v19 = vqmovun_s16(vreinterpretq_s16_u16(v30)); // sqxtun v19.8b, v30.8h
            let v19 = vqmovun_high_s16(v19, vreinterpretq_s16_u16(v31)); // sqxtun2 v19.16b, v31.8h
            let v18 = vqmovun_s16(vreinterpretq_s16_u16(v28)); // sqxtun v18.8b, v28.8h
            let v18 = vqmovun_high_s16(v18, vreinterpretq_s16_u16(v29)); // sqxtun2 v18.16b, v29.8h
            let v23 = vabdq_u8(v5, v19); // uabd v23.16b, v5.16b, v19.16b (tdiff)
            let v22 = vabdq_u8(v5, v18); // uabd v22.16b, v5.16b, v18.16b
            let v21 = vabdq_u8(v5, v17); // uabd v21.16b, v5.16b, v17.16b
            let v20 = vabdq_u8(v5, v16); // uabd v20.16b, v5.16b, v16.16b
            let v27 = vabdq_u8(v4, v19); // uabd v27.16b, v4.16b, v19.16b (tldiff)
            let v26 = vabdq_u8(v4, v18); // uabd v26.16b, v4.16b, v18.16b
            let v25 = vabdq_u8(v4, v17); // uabd v25.16b, v4.16b, v17.16b
            let v24 = vabdq_u8(v4, v16); // uabd v24.16b, v4.16b, v16.16b
            let v19 = vabdq_u8(v3, v19); // uabd v19.16b, v3.16b, v19.16b (ldiff)
            let v18 = vabdq_u8(v2, v18); // uabd v18.16b, v2.16b, v18.16b
            let v17 = vabdq_u8(v1, v17); // uabd v17.16b, v1.16b, v17.16b
            let v16 = vabdq_u8(v0, v16); // uabd v16.16b, v0.16b, v16.16b
            let v31 = vminq_u8(v23, v27); // umin v31.16b, v23.16b, v27.16b (min(tdiff, tldiff))
            let v30 = vminq_u8(v22, v26); // umin v30.16b, v22.16b, v26.16b
            let v29 = vminq_u8(v21, v25); // umin v29.16b, v21.16b, v25.16b
            let v28 = vminq_u8(v20, v24); // umin v28.16b, v20.16b, v24.16b
            let v23 = vcgeq_u8(v27, v23); // cmhs v23.16b, v27.16b, v23.16b (tldiff >= tdiff)
            let v22 = vcgeq_u8(v26, v22); // cmhs v22.16b, v26.16b, v22.16b
            let v21 = vcgeq_u8(v25, v21); // cmhs v21.16b, v25.16b, v21.16b
            let v20 = vcgeq_u8(v24, v20); // cmhs v20.16b, v24.16b, v20.16b
            let v19 = vcgeq_u8(v31, v19); // cmhs v19.16b, v31.16b, v19.16b (min(tdiff, tldiff) >= ldiff)
            let v18 = vcgeq_u8(v30, v18); // cmhs v18.16b, v30.16b, v18.16b
            let v17 = vcgeq_u8(v29, v17); // cmhs v17.16b, v29.16b, v17.16b
            let v16 = vcgeq_u8(v28, v16); // cmhs v16.16b, v28.16b, v16.16b
            let v23 = vbslq_u8(v23, v5, v4); // bsl v23.16b, v5.16b, v4.16b (tdiff <= tldiff ? top : topleft)
            let v22 = vbslq_u8(v22, v5, v4); // bsl v22.16b, v5.16b, v4.16b
            let v21 = vbslq_u8(v21, v5, v4); // bsl v21.16b, v5.16b, v4.16b
            let v20 = vbslq_u8(v20, v5, v4); // bsl v20.16b, v5.16b, v4.16b
            let v23 = vbslq_u8(v19, v3, v23); // bit v23.16b, v3.16b, v19.16b (ldiff <= min ? left : ...)
            let v22 = vbslq_u8(v18, v2, v22); // bit v22.16b, v2.16b, v18.16b
            let v21 = vbslq_u8(v17, v1, v21); // bit v21.16b, v1.16b, v17.16b
            let v20 = vbslq_u8(v16, v0, v20); // bit v20.16b, v0.16b, v16.16b
            let gt = pa_subs_gt(&mut w3, 16); // subs w3, w3, #16
            vst1q_u8(x0, v23); // st1 {v23.16b}, [x0], #16
            x0 = x0.wrapping_add(16);
            vst1q_u8(x6, v22); // st1 {v22.16b}, [x6], #16
            x6 = x6.wrapping_add(16);
            vst1q_u8(x5, v21); // st1 {v21.16b}, [x5], #16
            x5 = x5.wrapping_add(16);
            vst1q_u8(x10, v20); // st1 {v20.16b}, [x10], #16
            x10 = x10.wrapping_add(16);
            if !gt {
                break; // b.le 8f
            }
            v5 = vld1q_u8(x8); // ld1 {v5.16b}, [x8], #16
            x8 = x8.wrapping_add(16);
            // b 2b
        }
        // 8:
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        if !gt {
            break; // b.le 9f
        }
        // End of horizontal loop, move pointers to next four rows
        x8 = x8.wrapping_sub(w9 as u32 as usize); // sub x8, x8, w9, uxtw
        x0 = x0.wrapping_offset(x1); // add x0, x0, x1
        x6 = x6.wrapping_offset(x1); // add x6, x6, x1
                                     // Load the top row as early as possible
        v5 = vld1q_u8(x8); // ld1 {v5.16b}, [x8], #16
        x8 = x8.wrapping_add(16);
        x5 = x5.wrapping_offset(x1); // add x5, x5, x1
        x10 = x10.wrapping_offset(x1); // add x10, x10, x1
        w3 = w9; // mov w3, w9
                 // b 1b
    }
    // 9: ret
}

/// `ipred_paeth_8bpc_neon`
///
/// # Safety
/// Reads `topleft[-height..=width]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_paeth_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let (mut x0, x1, w3, mut w4) = (dst, stride, width, height);
    let v4 = vld1q_dup_u8(topleft); // ld1r {v4.16b}, [x2]
    let x8 = topleft.add(1); // add x8, x2, #1
    let mut x2 = topleft.sub(4); // sub x2, x2, #4; mov x7, #-4
    let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    match pa_tbl_idx(width) {
        4 => {
            // 40:
            let v5 = vreinterpretq_u8_u32(vdupq_n_u32(x8.cast::<u32>().read_unaligned())); // ld1r {v5.4s}, [x8]
            let v6 = vsubl_u8(vget_low_u8(v5), vget_low_u8(v4)); // usubl v6.8h, v5.8b, v4.8b (top - topleft)
            loop {
                // 4:
                let uint8x8x4_t(v0, v1, v2, v3) = vld4_dup_u8(x2); // ld4r {v0.8b, v1.8b, v2.8b, v3.8b}, [x2], x7
                x2 = x2.wrapping_sub(4);
                let v0 = vzip1_u32(vreinterpret_u32_u8(v0), vreinterpret_u32_u8(v1)); // zip1 v0.2s, v0.2s, v1.2s
                let v2 = vzip1_u32(vreinterpret_u32_u8(v2), vreinterpret_u32_u8(v3)); // zip1 v2.2s, v2.2s, v3.2s
                let (v0, v2) = (vreinterpret_u8_u32(v0), vreinterpret_u8_u32(v2));
                let v16 = vaddw_u8(v6, v0); // uaddw v16.8h, v6.8h, v0.8b
                let v17 = vaddw_u8(v6, v2); // uaddw v17.8h, v6.8h, v2.8b
                let v16 = vqmovun_s16(vreinterpretq_s16_u16(v16)); // sqxtun v16.8b, v16.8h (base)
                let v16 = vqmovun_high_s16(v16, vreinterpretq_s16_u16(v17)); // sqxtun2 v16.16b, v17.8h
                let v0 = vcombine_u8(v0, v2); // zip1 v0.2d, v0.2d, v2.2d
                let v20 = vabdq_u8(v5, v16); // uabd v20.16b, v5.16b, v16.16b (tdiff)
                let v22 = vabdq_u8(v4, v16); // uabd v22.16b, v4.16b, v16.16b (tldiff)
                let v16 = vabdq_u8(v0, v16); // uabd v16.16b, v0.16b, v16.16b (ldiff)
                let v18 = vminq_u8(v20, v22); // umin v18.16b, v20.16b, v22.16b (min(tdiff, tldiff))
                let v20 = vcgeq_u8(v22, v20); // cmhs v20.16b, v22.16b, v20.16b (tldiff >= tdiff)
                let v16 = vcgeq_u8(v18, v16); // cmhs v16.16b, v18.16b, v16.16b (min(tdiff, tldiff) >= ldiff)
                let v20 = vbslq_u8(v20, v5, v4); // bsl v20.16b, v5.16b, v4.16b (tdiff <= tldiff ? top : topleft)
                let v20 = vbslq_u8(v16, v0, v20); // bit v20.16b, v0.16b, v16.16b (ldiff <= min ? left : ...)
                pa_st1_s::<3>(x0, v20); // st1 {v20.s}[3], [x0], x1
                pa_st1_s::<2>(x6, v20); // st1 {v20.s}[2], [x6], x1
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                pa_st1_s::<1>(x0.wrapping_offset(x1), v20); // st1 {v20.s}[1], [x0], x1
                pa_st1_s::<0>(x6.wrapping_offset(x1), v20); // st1 {v20.s}[0], [x6], x1
                x0 = x0.wrapping_offset(x1 << 1);
                x6 = x6.wrapping_offset(x1 << 1);
                if !gt {
                    break; // b.gt 4b
                }
            }
        }
        3 => {
            // 80:
            let v5 = vreinterpretq_u8_u64(vdupq_n_u64(x8.cast::<u64>().read_unaligned())); // ld1r {v5.2d}, [x8]
            let v6 = vsubl_u8(vget_low_u8(v5), vget_low_u8(v4)); // usubl v6.8h, v5.8b, v4.8b (top - topleft)
            loop {
                // 8:
                let uint8x8x4_t(v0, v1, v2, v3) = vld4_dup_u8(x2); // ld4r {v0.8b, v1.8b, v2.8b, v3.8b}, [x2], x7
                x2 = x2.wrapping_sub(4);
                let v16 = vaddw_u8(v6, v0); // uaddw v16.8h, v6.8h, v0.8b
                let v17 = vaddw_u8(v6, v1); // uaddw v17.8h, v6.8h, v1.8b
                let v18 = vaddw_u8(v6, v2); // uaddw v18.8h, v6.8h, v2.8b
                let v19 = vaddw_u8(v6, v3); // uaddw v19.8h, v6.8h, v3.8b
                let v16 = vqmovun_s16(vreinterpretq_s16_u16(v16)); // sqxtun v16.8b, v16.8h (base)
                let v16 = vqmovun_high_s16(v16, vreinterpretq_s16_u16(v17)); // sqxtun2 v16.16b, v17.8h
                let v18 = vqmovun_s16(vreinterpretq_s16_u16(v18)); // sqxtun v18.8b, v18.8h
                let v18 = vqmovun_high_s16(v18, vreinterpretq_s16_u16(v19)); // sqxtun2 v18.16b, v19.8h
                let v2 = vcombine_u8(v2, v3); // zip1 v2.2d, v2.2d, v3.2d
                let v0 = vcombine_u8(v0, v1); // zip1 v0.2d, v0.2d, v1.2d
                let v21 = vabdq_u8(v5, v18); // uabd v21.16b, v5.16b, v18.16b (tdiff)
                let v20 = vabdq_u8(v5, v16); // uabd v20.16b, v5.16b, v16.16b
                let v23 = vabdq_u8(v4, v18); // uabd v23.16b, v4.16b, v18.16b (tldiff)
                let v22 = vabdq_u8(v4, v16); // uabd v22.16b, v4.16b, v16.16b
                let v17 = vabdq_u8(v2, v18); // uabd v17.16b, v2.16b, v18.16b (ldiff)
                let v16 = vabdq_u8(v0, v16); // uabd v16.16b, v0.16b, v16.16b
                let v19 = vminq_u8(v21, v23); // umin v19.16b, v21.16b, v23.16b (min(tdiff, tldiff))
                let v18 = vminq_u8(v20, v22); // umin v18.16b, v20.16b, v22.16b
                let v21 = vcgeq_u8(v23, v21); // cmhs v21.16b, v23.16b, v21.16b (tldiff >= tdiff)
                let v20 = vcgeq_u8(v22, v20); // cmhs v20.16b, v22.16b, v20.16b
                let v17 = vcgeq_u8(v19, v17); // cmhs v17.16b, v19.16b, v17.16b (min(tdiff, tldiff) >= ldiff)
                let v16 = vcgeq_u8(v18, v16); // cmhs v16.16b, v18.16b, v16.16b
                let v21 = vbslq_u8(v21, v5, v4); // bsl v21.16b, v5.16b, v4.16b (tdiff <= tldiff ? top : topleft)
                let v20 = vbslq_u8(v20, v5, v4); // bsl v20.16b, v5.16b, v4.16b
                let v21 = vbslq_u8(v17, v2, v21); // bit v21.16b, v2.16b, v17.16b (ldiff <= min ? left : ...)
                let v20 = vbslq_u8(v16, v0, v20); // bit v20.16b, v0.16b, v16.16b
                vst1_u8(x0, vget_high_u8(v21)); // st1 {v21.d}[1], [x0], x1
                vst1_u8(x6, vget_low_u8(v21)); // st1 {v21.d}[0], [x6], x1
                let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
                vst1_u8(x0.wrapping_offset(x1), vget_high_u8(v20)); // st1 {v20.d}[1], [x0], x1
                vst1_u8(x6.wrapping_offset(x1), vget_low_u8(v20)); // st1 {v20.d}[0], [x6], x1
                x0 = x0.wrapping_offset(x1 << 1);
                x6 = x6.wrapping_offset(x1 << 1);
                if !gt {
                    break; // b.gt 8b
                }
            }
        }
        0..=2 => ipred_paeth_160(x0, x1, x2, w3, w4, x6, x8, v4), // 160: 320: 640:
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `160:` of `ipred_smooth_8bpc_neon` (width >= 16).
///
/// # Safety
/// As the entry point; `x8` = `topleft + 1`, `x10` = `sm_weights + width`, `x11` = `sm_weights + height`, `v4` =
/// the bottom pixel in every byte.
unsafe fn ipred_smooth_160(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w3: c_int,
    w4: c_int,
    mut x8: *const u8,
    mut x10: *const u8,
    x11: *const u8,
    v4: uint8x16_t,
) {
    let x12 = x2.add(w3 as u32 as usize); // add x12, x2, w3, uxtw
    let v5 = vld1q_dup_u8(x12); // ld1r {v5.16b}, [x12] (right)
    let v31 = vcombine_u8(vdup_n_u8(128), vdup_n_u8(0)); // movi v31.8b, #128
    let v30 = vreinterpretq_u16_u8(vzip1q_u8(v31, v5)); // zip1 v30.16b, v31.16b, v5.16b (right*256 + rnd)
    let v31 = vreinterpretq_u16_u8(vzip1q_u8(v31, v4)); // zip1 v31.16b, v31.16b, v4.16b (bottom*256 + rnd)
    loop {
        // 161:
        let v7 = vld1q_u8(x10); // ldr q7, [x10], #16 (weights_hor)
        x10 = x10.wrapping_add(16);
        let v3 = vld1q_u8(x8); // ldr q3, [x8], #16 (top)
        x8 = x8.wrapping_add(16);
        let mut x14 = x2; // mov x14, x2
        let mut x13 = x11; // mov x13, x11
        let v6 = vmovl_u8(vget_low_u8(v7)); // uxtl v6.8h, v7.8b (weights_hor)
        let v2 = vsubl_u8(vget_low_u8(v3), vget_low_u8(v4)); // usubl v2.8h, v3.8b, v4.8b (top-bottom)
        let v7 = vmovl_high_u8(v7); // uxtl2 v7.8h, v7.16b
        let v3 = vsubl_high_u8(v3, v4); // usubl2 v3.8h, v3.16b, v4.16b
        let mut w9 = w4; // mov w9, w4
        let mut x5 = x0; // mov x5, x0
        loop {
            // 16:
            x14 = x14.sub(2);
            let v0 = pa_ldr_h_ld1r(x14); // ldr h0, [x14, #-2]! (left)
            let v1 = pa_ldr_h_ld1r(x13); // ldr h1, [x13], #2 (weights_ver)
            x13 = x13.wrapping_add(2);
            let v0 = vsubl_u8(vget_low_u8(v0), vget_low_u8(v5)); // usubl v0.8h, v0.8b, v5.8b (left-right)
            let v1 = pa_uxtl_zip1(v1); // uxtl v1.8h, v1.8b (weights_ver)
            let v16 = vmlaq_laneq_u16::<1>(v30, v6, v0); // mov v16.16b, v30.16b; mla v16.8h, v6.8h, v0.h[1]
            let v17 = vmlaq_laneq_u16::<0>(v31, v2, v1); // mov v17.16b, v31.16b; mla v17.8h, v2.8h, v1.h[0]
            let v18 = vmlaq_laneq_u16::<1>(v30, v7, v0); // mov v18.16b, v30.16b; mla v18.8h, v7.8h, v0.h[1]
            let v19 = vmlaq_laneq_u16::<0>(v31, v3, v1); // mov v19.16b, v31.16b; mla v19.8h, v3.8h, v1.h[0]
            let gt = pa_subs_gt(&mut w9, 2); // subs w9, w9, #2
            let v20 = vmlaq_laneq_u16::<0>(v30, v6, v0); // mov v20.16b, v30.16b; mla v20.8h, v6.8h, v0.h[0]
            let v21 = vmlaq_laneq_u16::<1>(v31, v2, v1); // mov v21.16b, v31.16b; mla v21.8h, v2.8h, v1.h[1]
            let v22 = vmlaq_laneq_u16::<0>(v30, v7, v0); // mov v22.16b, v30.16b; mla v22.8h, v7.8h, v0.h[0]
            let v23 = vmlaq_laneq_u16::<1>(v31, v3, v1); // mov v23.16b, v31.16b; mla v23.8h, v3.8h, v1.h[1]
            let v16 = vhaddq_u16(v16, v17); // uhadd v16.8h, v16.8h, v17.8h
            let v18 = vhaddq_u16(v18, v19); // uhadd v18.8h, v18.8h, v19.8h
            let v20 = vhaddq_u16(v20, v21); // uhadd v20.8h, v20.8h, v21.8h
            let v22 = vhaddq_u16(v22, v23); // uhadd v22.8h, v22.8h, v23.8h
            let v16 = vuzp2q_u8(vreinterpretq_u8_u16(v16), vreinterpretq_u8_u16(v18)); // uzp2 v16.16b, v16.16b, v18.16b
            let v20 = vuzp2q_u8(vreinterpretq_u8_u16(v20), vreinterpretq_u8_u16(v22)); // uzp2 v20.16b, v20.16b, v22.16b
            vst1q_u8(x5, v16); // str q16, [x5]
            vst1q_u8(x5.wrapping_offset(x1), v20); // str q20, [x5, x1]
            x5 = x5.wrapping_offset(x1 << 1); // add x5, x5, x1, lsl #1
            if !gt {
                break; // b.gt 16b
            }
        }
        w3 -= 16; // sub w3, w3, #16
        x0 = x0.wrapping_add(16); // add x0, x0, #16
        if w3 == 0 {
            break; // cbnz w3, 161b
        }
    }
    // ret
}

/// `ipred_smooth_8bpc_neon`
///
/// # Safety
/// Reads `topleft[-height..0]`, `topleft[1..=width]` and `dav1d_sm_weights[width..2 * width]`,
/// `[height..2 * height]`; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_smooth_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let (mut x0, x1, mut x2, w3, mut w4) = (dst, stride, topleft, width, height);
    let x10 = core::ptr::addr_of!(crate::common::tables::dav1d_sm_weights).cast::<u8>(); // movrel x10, X(sm_weights)
    let x12 = x2.sub(w4 as u32 as usize); // sub x12, x2, w4, uxtw
    let mut x11 = x10.add(w4 as u32 as usize); // add x11, x10, w4, uxtw
    let x10 = x10.add(w3 as u32 as usize); // add x10, x10, w3, uxtw
    let v4 = vld1q_dup_u8(x12); // ld1r {v4.16b}, [x12] (bottom)
    let x8 = x2.add(1); // add x8, x2, #1
    if w3 > 8 {
        // cmp w3, #8; b.gt 160f
        return ipred_smooth_160(x0, x1, x2, w3, w4, x8, x10, x11, v4);
    }
    if w3 == 8 {
        // b.eq 80f
        // 80: (width = 8)
        let v6 = vld1_u8(x8); // ld1 {v6.8b}, [x8] (top)
        let v7 = vld1_u8(x10); // ld1 {v7.8b}, [x10] (weights_hor)
        let v31 = vcombine_u8(vdup_n_u8(128), vdup_n_u8(0)); // movi v31.8b, #128
        let v5 = vdupq_lane_u8::<7>(v6); // dup v5.16b, v6.b[7] (right)
        let v7 = vmovl_u8(v7); // uxtl v7.8h, v7.8b (weights_hor)
        let v6 = vsubl_u8(v6, vget_low_u8(v4)); // usubl v6.8h, v6.8b, v4.8b (top-bottom)
        let v30 = vreinterpretq_u16_u8(vzip1q_u8(v31, v5)); // zip1 v30.16b, v31.16b, v5.16b (right*256 + rnd)
        let v31 = vreinterpretq_u16_u8(vzip1q_u8(v31, v4)); // zip1 v31.16b, v31.16b, v4.16b (bottom*256 + rnd)
        let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
        let x1 = x1 << 1; // lsl x1, x1, #1
        loop {
            // 8:
            x2 = x2.sub(4);
            let v0 = pa_ldr_s_ld1r(x2); // ldr s0, [x2, #-4]! (left)
            let v1 = pa_ldr_s_ld1r(x11); // ldr s1, [x11], #4 (weights_ver)
            x11 = x11.wrapping_add(4);
            let v0 = vsubl_u8(vget_low_u8(v0), vget_low_u8(v5)); // usubl v0.8h, v0.8b, v5.8b (left-right)
            let v1 = pa_uxtl_zip1(v1); // uxtl v1.8h, v1.8b (weights_ver)
            let v20 = vmlaq_laneq_u16::<3>(v30, v7, v0); // mov v20.16b, v30.16b; mla v20.8h, v7.8h, v0.h[3]
            let v21 = vmlaq_laneq_u16::<0>(v31, v6, v1); // mov v21.16b, v31.16b; mla v21.8h, v6.8h, v1.h[0]
            let v22 = vmlaq_laneq_u16::<2>(v30, v7, v0); // mov v22.16b, v30.16b; mla v22.8h, v7.8h, v0.h[2] (left flipped)
            let v23 = vmlaq_laneq_u16::<1>(v31, v6, v1); // mov v23.16b, v31.16b; mla v23.8h, v6.8h, v1.h[1]
            let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
            let v24 = vmlaq_laneq_u16::<1>(v30, v7, v0); // mov v24.16b, v30.16b; mla v24.8h, v7.8h, v0.h[1]
            let v25 = vmlaq_laneq_u16::<2>(v31, v6, v1); // mov v25.16b, v31.16b; mla v25.8h, v6.8h, v1.h[2]
            let v26 = vmlaq_laneq_u16::<0>(v30, v7, v0); // mov v26.16b, v30.16b; mla v26.8h, v7.8h, v0.h[0]
            let v27 = vmlaq_laneq_u16::<3>(v31, v6, v1); // mov v27.16b, v31.16b; mla v27.8h, v6.8h, v1.h[3]
            let v20 = vhaddq_u16(v20, v21); // uhadd v20.8h, v20.8h, v21.8h
            let v22 = vhaddq_u16(v22, v23); // uhadd v22.8h, v22.8h, v23.8h
            let v24 = vhaddq_u16(v24, v25); // uhadd v24.8h, v24.8h, v25.8h
            let v26 = vhaddq_u16(v26, v27); // uhadd v26.8h, v26.8h, v27.8h
            let v20 = vuzp2q_u8(vreinterpretq_u8_u16(v20), vreinterpretq_u8_u16(v24)); // uzp2 v20.16b, v20.16b, v24.16b
            let v22 = vuzp2q_u8(vreinterpretq_u8_u16(v22), vreinterpretq_u8_u16(v26)); // uzp2 v22.16b, v22.16b, v26.16b
            vst1_u8(x0, vget_low_u8(v20)); // st1 {v20.8b}, [x0], x1
            vst1_u8(x6, vget_low_u8(v22)); // st1 {v22.8b}, [x6], x1
            vst1_u8(x0.wrapping_offset(x1), vget_high_u8(v20)); // st1 {v20.d}[1], [x0], x1
            vst1_u8(x6.wrapping_offset(x1), vget_high_u8(v22)); // st1 {v22.d}[1], [x6], x1
            x0 = x0.wrapping_offset(x1 << 1);
            x6 = x6.wrapping_offset(x1 << 1);
            if !gt {
                break; // b.gt 8b
            }
        }
        return; // ret
    }
    // 40: (width = 4)
    let v6 = pa_ld1r_2s(x8); // ld1r {v6.2s}, [x8] (top)
    let v7 = pa_ld1r_2s(x10); // ld1r {v7.2s}, [x10] (weights_hor)
    let v28 = vcombine_u8(vdup_n_u8(1), vdup_n_u8(0)); // movi v28.8b, #1
    let v29 = vdupq_n_u8(2); // movi v29.16b, #2
    let v27 = vdupq_n_u8(3); // movi v27.16b, #3
    let v31 = vcombine_u8(vdup_n_u8(128), vdup_n_u8(0)); // movi v31.8b, #128
    let v29 = vaddq_u8(v29, v28); // add v29.16b, v29.16b, v28.16b
    let v5 = vdupq_lane_u8::<3>(v6); // dup v5.16b, v6.b[3] (right)
    let v6 = vsubl_u8(v6, vget_low_u8(v4)); // usubl v6.8h, v6.8b, v4.8b (top-bottom)
    let v29 = vreinterpretq_u8_u32(vuzp1q_u32(vreinterpretq_u32_u8(v29), vreinterpretq_u32_u8(v28))); // uzp1 v29.4s, v29.4s, v28.4s
    let v7 = vmovl_u8(v7); // uxtl v7.8h, v7.8b (weights_hor)
    let v27 = vsubq_u8(v27, v29); // sub v27.16b, v27.16b, v29.16b
    let v30 = vreinterpretq_u16_u8(vzip1q_u8(v31, v5)); // zip1 v30.16b, v31.16b, v5.16b (right*256 + rnd)
    let v31 = vreinterpretq_u16_u8(vzip1q_u8(v31, v4)); // zip1 v31.16b, v31.16b, v4.16b (bottom*256 + rnd)
    let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    loop {
        // 4:
        x2 = x2.sub(4);
        let v0 = pa_ldr_s_ld1r(x2); // ldr s0, [x2, #-4]!
        let v1 = pa_ldr_s_ld1r(x11); // ldr s1, [x11], #4 (weights_ver)
        x11 = x11.wrapping_add(4);
        let v0 = vqtbl1q_u8(v0, v29); // tbl v0.16b, {v0.16b}, v29.16b
        let v1 = vqtbl1q_u8(v1, v27); // tbl v1.16b, {v1.16b}, v27.16b
        let v17 = vsubl_u8(vget_low_u8(v0), vget_low_u8(v5)); // usubl v17.8h, v0.8b, v5.8b (left-right)
        let v16 = vmovl_u8(vget_low_u8(v1)); // uxtl v16.8h, v1.8b
        let v19 = vsubl_high_u8(v0, v5); // usubl2 v19.8h, v0.16b, v5.16b (left-right)
        let v18 = vmovl_high_u8(v1); // uxtl2 v18.8h, v1.16b
        let v20 = vmlaq_u16(v30, v7, v17); // mov v20.16b, v30.16b; mla v20.8h, v7.8h, v17.8h
        let v22 = vmlaq_u16(v31, v6, v16); // mov v22.16b, v31.16b; mla v22.8h, v6.8h, v16.8h
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        let v21 = vmlaq_u16(v30, v7, v19); // mov v21.16b, v30.16b; mla v21.8h, v7.8h, v19.8h
        let v23 = vmlaq_u16(v31, v6, v18); // mov v23.16b, v31.16b; mla v23.8h, v6.8h, v18.8h
        let v20 = vhaddq_u16(v20, v22); // uhadd v20.8h, v20.8h, v22.8h
        let v21 = vhaddq_u16(v21, v23); // uhadd v21.8h, v21.8h, v23.8h
        let v20 = vuzp2q_u8(vreinterpretq_u8_u16(v20), vreinterpretq_u8_u16(v21)); // uzp2 v20.16b, v20.16b, v21.16b
        pa_st1_s::<0>(x0, v20); // st1 {v20.s}[0], [x0], x1
        pa_st1_s::<1>(x6, v20); // st1 {v20.s}[1], [x6], x1
        pa_st1_s::<2>(x0.wrapping_offset(x1), v20); // st1 {v20.s}[2], [x0], x1
        pa_st1_s::<3>(x6.wrapping_offset(x1), v20); // st1 {v20.s}[3], [x6], x1
        x0 = x0.wrapping_offset(x1 << 1);
        x6 = x6.wrapping_offset(x1 << 1);
        if !gt {
            break; // b.gt 4b
        }
    }
}

/// `ipred_smooth_v_8bpc_neon`
///
/// # Safety
/// Reads `topleft[-height]`, `topleft[1..=width]` and `dav1d_sm_weights[height..2 * height]`; writes `dst` rows
/// `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_smooth_v_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let (mut x0, x1, mut w3, mut w4) = (dst, stride, width, height);
    let x8 = topleft.sub(w4 as u32 as usize); // sub x8, x2, w4, uxtw
    let x7 = core::ptr::addr_of!(crate::common::tables::dav1d_sm_weights).cast::<u8>(); // movrel x7, X(sm_weights)
    let v4 = vld1q_dup_u8(x8); // ld1r {v4.16b}, [x8] (bottom)
    let v31 = vcombine_u8(vdup_n_u8(128), vdup_n_u8(0)); // movi v31.8b, #128
    let mut x2 = topleft.add(1); // add x2, x2, #1
    let mut x7 = x7.add(w4 as u32 as usize); // add x7, x7, w4, uxtw
    let v31 = vreinterpretq_u16_u8(vzip1q_u8(v31, v4)); // zip1 v31.16b, v31.16b, v4.16b (bottom*256 + rnd)
    if w3 > 8 {
        // cmp w3, #8; b.gt 160f
        loop {
            // 160:
            let v7 = vld1q_u8(x2); // ldr q7, [x2], #16 (top)
            x2 = x2.wrapping_add(16);
            let mut x5 = x7; // mov x5, x7
            let mut w8 = w4; // mov w8, w4
            let v6 = vsubl_u8(vget_low_u8(v7), vget_low_u8(v4)); // usubl v6.8h, v7.8b, v4.8b (top-bottom lower)
            let v7 = vsubl_high_u8(v7, v4); // usubl2 v7.8h, v7.16b, v4.16b (top-bottom upper)
            let mut x9 = x0; // mov x9, x0
            let mut x10 = x0.wrapping_offset(x1 << 1); // add x10, x0, x1, lsl #1
            loop {
                // 16:
                let v5 = pa_ldr_s_ld1r(x5); // ldr s5, [x5], #4 (weights_ver)
                x5 = x5.wrapping_add(4);
                let v5 = pa_uxtl_zip1(v5); // uxtl v5.8h, v5.8b (weights_ver)
                let v16 = vmulq_laneq_u16::<0>(v6, v5); // mul v16.8h, v6.8h, v5.h[0] (bottom*256 + (top-bottom)*weights_ver)
                let v17 = vmulq_laneq_u16::<1>(v6, v5); // mul v17.8h, v6.8h, v5.h[1]
                let v18 = vmulq_laneq_u16::<0>(v7, v5); // mul v18.8h, v7.8h, v5.h[0]
                let v19 = vmulq_laneq_u16::<1>(v7, v5); // mul v19.8h, v7.8h, v5.h[1]
                let v16 = vaddhn_u16(v16, v31); // addhn v16.8b, v16.8h, v31.8h
                let v17 = vaddhn_u16(v17, v31); // addhn v17.8b, v17.8h, v31.8h
                let v16 = vaddhn_high_u16(v16, v18, v31); // addhn2 v16.16b, v18.8h, v31.8h
                let v17 = vaddhn_high_u16(v17, v19, v31); // addhn2 v17.16b, v19.8h, v31.8h
                let gt = pa_subs_gt(&mut w8, 4); // subs w8, w8, #4
                let v20 = vmulq_laneq_u16::<2>(v6, v5); // mul v20.8h, v6.8h, v5.h[2]
                let v21 = vmulq_laneq_u16::<3>(v6, v5); // mul v21.8h, v6.8h, v5.h[3]
                let v22 = vmulq_laneq_u16::<2>(v7, v5); // mul v22.8h, v7.8h, v5.h[2]
                let v23 = vmulq_laneq_u16::<3>(v7, v5); // mul v23.8h, v7.8h, v5.h[3]
                let v20 = vaddhn_u16(v20, v31); // addhn v20.8b, v20.8h, v31.8h
                let v21 = vaddhn_u16(v21, v31); // addhn v21.8b, v21.8h, v31.8h
                let v20 = vaddhn_high_u16(v20, v22, v31); // addhn2 v20.16b, v22.8h, v31.8h
                let v21 = vaddhn_high_u16(v21, v23, v31); // addhn2 v21.16b, v23.8h, v31.8h
                vst1q_u8(x9, v16); // str q16, [x9]
                vst1q_u8(x9.wrapping_offset(x1), v17); // str q17, [x9, x1]
                x9 = x9.wrapping_offset(x1 << 2); // add x9, x9, x1, lsl #2
                vst1q_u8(x10, v20); // str q20, [x10]
                vst1q_u8(x10.wrapping_offset(x1), v21); // str q21, [x10, x1]
                x10 = x10.wrapping_offset(x1 << 2); // add x10, x10, x1, lsl #2
                if !gt {
                    break; // b.gt 16b
                }
            }
            w3 -= 16; // sub w3, w3, #16
            x0 = x0.wrapping_add(16); // add x0, x0, #16
            if w3 == 0 {
                break; // cbnz w3, 160b
            }
        }
        return; // ret
    }
    if w3 == 8 {
        // b.eq 80f
        // 80:
        let v6 = vld1_u8(x2); // ld1 {v6.8b}, [x2] (top)
        let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
        let x1 = x1 << 1; // lsl x1, x1, #1
        let v6 = vsubl_u8(v6, vget_low_u8(v4)); // usubl v6.8h, v6.8b, v4.8b (top-bottom)
        loop {
            // 8:
            let v7 = pa_ldr_s_ld1r(x7); // ldr s7, [x7], #4 (weights_ver)
            x7 = x7.wrapping_add(4);
            let v7 = pa_uxtl_zip1(v7); // uxtl v7.8h, v7.8b (weights_ver)
            let v24 = vmulq_laneq_u16::<0>(v6, v7); // mul v24.8h, v6.8h, v7.h[0] (bottom*256 + (top-bottom)*weights_ver)
            let v25 = vmulq_laneq_u16::<1>(v6, v7); // mul v25.8h, v6.8h, v7.h[1]
            let v26 = vmulq_laneq_u16::<2>(v6, v7); // mul v26.8h, v6.8h, v7.h[2]
            let v27 = vmulq_laneq_u16::<3>(v6, v7); // mul v27.8h, v6.8h, v7.h[3]
            let v24 = vaddhn_u16(v24, v31); // addhn v24.8b, v24.8h, v31.8h
            let v25 = vaddhn_u16(v25, v31); // addhn v25.8b, v25.8h, v31.8h
            let v26 = vaddhn_u16(v26, v31); // addhn v26.8b, v26.8h, v31.8h
            let v27 = vaddhn_u16(v27, v31); // addhn v27.8b, v27.8h, v31.8h
            let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
            vst1_u8(x0, v24); // st1 {v24.8b}, [x0], x1
            vst1_u8(x6, v25); // st1 {v25.8b}, [x6], x1
            vst1_u8(x0.wrapping_offset(x1), v26); // st1 {v26.8b}, [x0], x1
            vst1_u8(x6.wrapping_offset(x1), v27); // st1 {v27.8b}, [x6], x1
            x0 = x0.wrapping_offset(x1 << 1);
            x6 = x6.wrapping_offset(x1 << 1);
            if !gt {
                break; // b.gt 8b
            }
        }
        return; // ret
    }
    // 40:
    let v6 = pa_ld1r_2s(x2); // ld1r {v6.2s}, [x2] (top)
    let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    let v6 = vsubl_u8(v6, vget_low_u8(v4)); // usubl v6.8h, v6.8b, v4.8b (top-bottom)
    loop {
        // 4:
        let v7 = pa_ldr_s_ld1r(x7); // ldr s7, [x7], #4 (weights_ver)
        x7 = x7.wrapping_add(4);
        let v7 = pa_uxtl_zip1(v7); // uxtl v7.8h, v7.8b (weights_ver)
        let v7 = vzip1q_u16(v7, v7); // zip1 v7.8h, v7.8h, v7.8h
        let v16 = vzip1q_u16(v7, v7); // zip1 v16.8h, v7.8h, v7.8h
        let v18 = vzip2q_u16(v7, v7); // zip2 v18.8h, v7.8h, v7.8h (splat weights_ver)
        let v22 = vmulq_u16(v6, v16); // mul v22.8h, v6.8h, v16.8h (bottom*256 + (top-bottom)*weights_ver)
        let v23 = vmulq_u16(v6, v18); // mul v23.8h, v6.8h, v18.8h
        let v22 = vaddhn_u16(v22, v31); // addhn v22.8b, v22.8h, v31.8h
        let v23 = vaddhn_u16(v23, v31); // addhn v23.8b, v23.8h, v31.8h
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        pa_st1_s_8b::<0>(x0, v22); // st1 {v22.s}[0], [x0], x1
        pa_st1_s_8b::<1>(x6, v22); // st1 {v22.s}[1], [x6], x1
        pa_st1_s_8b::<0>(x0.wrapping_offset(x1), v23); // st1 {v23.s}[0], [x0], x1
        pa_st1_s_8b::<1>(x6.wrapping_offset(x1), v23); // st1 {v23.s}[1], [x6], x1
        x0 = x0.wrapping_offset(x1 << 1);
        x6 = x6.wrapping_offset(x1 << 1);
        if !gt {
            break; // b.gt 4b
        }
    }
}

/// `ipred_smooth_h_8bpc_neon`
///
/// # Safety
/// Reads `topleft[-height..0]`, `topleft[width]` and `dav1d_sm_weights[width..2 * width]`; writes `dst` rows
/// `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_smooth_h_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _a: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let (mut x0, x1, mut x2, mut w3, mut w4) = (dst, stride, topleft, width, height);
    let x12 = x2.add(w3 as u32 as usize); // add x12, x2, w3, uxtw
    let x8 = core::ptr::addr_of!(crate::common::tables::dav1d_sm_weights).cast::<u8>(); // movrel x8, X(sm_weights)
    let v5 = vld1q_dup_u8(x12); // ld1r {v5.16b}, [x12] (right)
    let v31 = vcombine_u8(vdup_n_u8(128), vdup_n_u8(0)); // movi v31.8b, #128
    let mut x8 = x8.add(w3 as u32 as usize); // add x8, x8, w3, uxtw
    let v31 = vreinterpretq_u16_u8(vzip1q_u8(v31, v5)); // zip1 v31.16b, v31.16b, v5.16b (right*256 + rnd)
    if w3 > 8 {
        // cmp w3, #8; b.gt 160f
        loop {
            // 160:
            let v7 = vld1q_u8(x8); // ldr q7, [x8], #16 (weights_hor)
            x8 = x8.wrapping_add(16);
            let mut x12 = x2; // mov x12, x2
            let mut w9 = w4; // mov w9, w4
            let v6 = vmovl_u8(vget_low_u8(v7)); // uxtl v6.8h, v7.8b (weights_hor lower)
            let v7 = vmovl_high_u8(v7); // uxtl2 v7.8h, v7.16b (weights_hor upper)
            let mut x5 = x0; // mov x5, x0
            let mut x7 = x0.wrapping_offset(x1 << 1); // add x7, x0, x1, lsl #1
            loop {
                // 16:
                x12 = x12.sub(4);
                let v0 = pa_ldr_s_ld1r(x12); // ldr s0, [x12, #-4]! (left)
                let v0 = vsubl_u8(vget_low_u8(v0), vget_low_u8(v5)); // usubl v0.8h, v0.8b, v5.8b
                let v16 = vmulq_laneq_u16::<3>(v6, v0); // mul v16.8h, v6.8h, v0.h[3] (right*256 + (left-right)*weights_hor)
                let v18 = vmulq_laneq_u16::<2>(v6, v0); // mul v18.8h, v6.8h, v0.h[2] (left flipped)
                let v17 = vmulq_laneq_u16::<3>(v7, v0); // mul v17.8h, v7.8h, v0.h[3]
                let v19 = vmulq_laneq_u16::<2>(v7, v0); // mul v19.8h, v7.8h, v0.h[2]
                let v16 = vaddhn_u16(v16, v31); // addhn v16.8b, v16.8h, v31.8h
                let v18 = vaddhn_u16(v18, v31); // addhn v18.8b, v18.8h, v31.8h
                let v16 = vaddhn_high_u16(v16, v17, v31); // addhn2 v16.16b, v17.8h, v31.8h
                let v18 = vaddhn_high_u16(v18, v19, v31); // addhn2 v18.16b, v19.8h, v31.8h
                let gt = pa_subs_gt(&mut w9, 4); // subs w9, w9, #4
                let v20 = vmulq_laneq_u16::<1>(v6, v0); // mul v20.8h, v6.8h, v0.h[1]
                let v22 = vmulq_laneq_u16::<0>(v6, v0); // mul v22.8h, v6.8h, v0.h[0]
                let v21 = vmulq_laneq_u16::<1>(v7, v0); // mul v21.8h, v7.8h, v0.h[1]
                let v23 = vmulq_laneq_u16::<0>(v7, v0); // mul v23.8h, v7.8h, v0.h[0]
                let v20 = vaddhn_u16(v20, v31); // addhn v20.8b, v20.8h, v31.8h
                let v22 = vaddhn_u16(v22, v31); // addhn v22.8b, v22.8h, v31.8h
                let v20 = vaddhn_high_u16(v20, v21, v31); // addhn2 v20.16b, v21.8h, v31.8h
                let v22 = vaddhn_high_u16(v22, v23, v31); // addhn2 v22.16b, v23.8h, v31.8h
                vst1q_u8(x5, v16); // str q16, [x5]
                vst1q_u8(x5.wrapping_offset(x1), v18); // str q18, [x5, x1]
                x5 = x5.wrapping_offset(x1 << 2); // add x5, x5, x1, lsl #2
                vst1q_u8(x7, v20); // str q20, [x7]
                vst1q_u8(x7.wrapping_offset(x1), v22); // str q22, [x7, x1]
                x7 = x7.wrapping_offset(x1 << 2); // add x7, x7, x1, lsl #2
                if !gt {
                    break; // b.gt 16b
                }
            }
            w3 -= 16; // sub w3, w3, #16
            x0 = x0.wrapping_add(16); // add x0, x0, #16
            if w3 == 0 {
                break; // cbnz w3, 160b
            }
        }
        return; // ret
    }
    if w3 == 8 {
        // b.eq 80f
        // 80:
        let v7 = vld1_u8(x8); // ld1 {v7.8b}, [x8] (weights_hor)
        let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
        let x1 = x1 << 1; // lsl x1, x1, #1
        let v7 = vmovl_u8(v7); // uxtl v7.8h, v7.8b (weights_hor)
        loop {
            // 8:
            x2 = x2.sub(4);
            let v0 = pa_ldr_s_ld1r(x2); // ldr s0, [x2, #-4]! (left)
            let v0 = vsubl_u8(vget_low_u8(v0), vget_low_u8(v5)); // usubl v0.8h, v0.8b, v5.8b
            let v20 = vmulq_laneq_u16::<3>(v7, v0); // mul v20.8h, v7.8h, v0.h[3] (right*256 + (left-right)*weights_hor)
            let v21 = vmulq_laneq_u16::<2>(v7, v0); // mul v21.8h, v7.8h, v0.h[2] (left flipped)
            let v22 = vmulq_laneq_u16::<1>(v7, v0); // mul v22.8h, v7.8h, v0.h[1]
            let v23 = vmulq_laneq_u16::<0>(v7, v0); // mul v23.8h, v7.8h, v0.h[0]
            let v20 = vaddhn_u16(v20, v31); // addhn v20.8b, v20.8h, v31.8h
            let v21 = vaddhn_u16(v21, v31); // addhn v21.8b, v21.8h, v31.8h
            let v22 = vaddhn_u16(v22, v31); // addhn v22.8b, v22.8h, v31.8h
            let v23 = vaddhn_u16(v23, v31); // addhn v23.8b, v23.8h, v31.8h
            let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
            vst1_u8(x0, v20); // st1 {v20.8b}, [x0], x1
            vst1_u8(x6, v21); // st1 {v21.8b}, [x6], x1
            vst1_u8(x0.wrapping_offset(x1), v22); // st1 {v22.8b}, [x0], x1
            vst1_u8(x6.wrapping_offset(x1), v23); // st1 {v23.8b}, [x6], x1
            x0 = x0.wrapping_offset(x1 << 1);
            x6 = x6.wrapping_offset(x1 << 1);
            if !gt {
                break; // b.gt 8b
            }
        }
        return; // ret
    }
    // 40:
    let v7 = pa_ld1r_2s(x8); // ld1r {v7.2s}, [x8] (weights_hor)
    let mut x6 = x0.wrapping_offset(x1); // add x6, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    let v7 = vmovl_u8(v7); // uxtl v7.8h, v7.8b (weights_hor)
    loop {
        // 4:
        x2 = x2.sub(4);
        let v0 = vget_low_u8(pa_ldr_s_ld1r(x2)); // ldr s0, [x2, #-4]! (left)
        let v0 = vcombine_u8(vzip1_u8(v0, v0), vdup_n_u8(0)); // zip1 v0.8b, v0.8b, v0.8b
        let v0 = vzip1q_u8(v0, v0); // zip1 v0.16b, v0.16b, v0.16b (replicate left[1..4])
        let v20 = vsubl_high_u8(v0, v5); // usubl2 v20.8h, v0.16b, v5.16b (left-right, upper part, for flipped stores)
        let v21 = vsubl_u8(vget_low_u8(v0), vget_low_u8(v5)); // usubl v21.8h, v0.8b, v5.8b (left-right, lower part)
        let v20 = vmulq_u16(v20, v7); // mul v20.8h, v20.8h, v7.8h (right*256 + (left-right)*weights_hor)
        let v21 = vmulq_u16(v21, v7); // mul v21.8h, v21.8h, v7.8h
        let v20 = vaddhn_u16(v20, v31); // addhn v20.8b, v20.8h, v31.8h
        let v21 = vaddhn_u16(v21, v31); // addhn v21.8b, v21.8h, v31.8h
        let gt = pa_subs_gt(&mut w4, 4); // subs w4, w4, #4
        pa_st1_s_8b::<1>(x0, v20); // st1 {v20.s}[1], [x0], x1 (complete flip at store)
        pa_st1_s_8b::<0>(x6, v20); // st1 {v20.s}[0], [x6], x1
        pa_st1_s_8b::<1>(x0.wrapping_offset(x1), v21); // st1 {v21.s}[1], [x0], x1
        pa_st1_s_8b::<0>(x6.wrapping_offset(x1), v21); // st1 {v21.s}[0], [x6], x1
        x0 = x0.wrapping_offset(x1 << 1);
        x6 = x6.wrapping_offset(x1 << 1);
        if !gt {
            break; // b.gt 4b
        }
    }
}

// ============================================================================
// Part B (ipred.S lines 1297–1850): the edge helpers and the z1 fill kernels.
// ============================================================================
// Register convention of this part: typed locals named after the assembly's
// registers (`v0`, `v31`, `x0`, `w7`); a `.8b`/`.4h` register is a 64-bit
// vector type, a `.16b`/`.8h` register a 128-bit one. Post-incremented
// addresses are advanced with `wrapping_add` after the access, so that no
// pointer past the last access is formed with `add`.

/// `padding_mask_buf`: 32 bytes of 0x00 followed by the 32 bytes of 0xff at
/// the inner label `padding_mask`.
static PADDING_MASK_BUF: [u8; 64] = {
    let mut buf = [0u8; 64];
    let mut i = 32;
    while i < 64 {
        buf[i] = 0xff;
        i += 1;
    }
    buf
};

/// `movrel xN, padding_mask`: the address of the 0xff half of
/// `padding_mask_buf`. A 16-byte load at `padding_mask() - n` (0 <= n <= 32)
/// gives a mask that keeps the first `n` bytes and replaces the rest.
#[inline(always)]
fn padding_mask() -> *const u8 {
    // SAFETY: 32 is inside the 64-byte static.
    unsafe { PADDING_MASK_BUF.as_ptr().add(32) }
}

/// `st1 {v.s}[0], [x]`: the low four bytes of a `.8b` register, to an
/// address without alignment.
#[inline(always)]
unsafe fn ipred_z1_st1_s0(x: *mut u8, v: uint8x8_t) {
    core::ptr::write_unaligned(x as *mut u32, vget_lane_u32::<0>(vreinterpret_u32_u8(v)));
}

/// `dav1d_ipred_z1_upsample_edge_8bpc_neon(pixel *out, const int hsz, const pixel *const in, const int end)`
///
/// # Safety
/// Reads `in[0..16]` (whatever `end` is; the bytes above `in[end]` are
/// replaced through the padding mask) and `in[end]`, with 0 <= `end` <= 16
/// (the mask load is `padding_mask - end`); writes `out[0..32]`.
pub unsafe extern "C" fn dav1d_ipred_z1_upsample_edge_8bpc_neon(out: *mut u8, hsz: c_int, in_: *const u8, end: c_int) {
    let _ = hsz;
    let (x0, x2, w3) = (out, in_, end as u32 as usize);
    let x4 = padding_mask(); // movrel x4, padding_mask
    let v0 = vld1q_u8(x2); // ld1 {v0.16b}, [x2] (in[])
    let x5 = x2.add(w3); // add x5, x2, w3, uxtw (in[end])
    let x4 = x4.sub(w3); // sub x4, x4, w3, uxtw

    let v1 = vld1q_dup_u8(x5); // ld1r {v1.16b}, [x5] (padding)
    let v3 = vld1q_u8(x4); // ld1 {v3.16b}, [x4] (padding_mask)

    let v31 = vdupq_n_u16(9); // movi v31.8h, #9

    let v0 = vbslq_u8(v3, v1, v0); // bit v0.16b, v1.16b, v3.16b (padded in[])

    let v4 = vextq_u8::<1>(v0, v1); // ext v4.16b, v0.16b, v1.16b, #1
    let v5 = vextq_u8::<2>(v0, v1); // ext v5.16b, v0.16b, v1.16b, #2
    let v6 = vextq_u8::<3>(v0, v1); // ext v6.16b, v0.16b, v1.16b, #3

    let v16 = vaddl_u8(vget_low_u8(v4), vget_low_u8(v5)); // uaddl v16.8h, v4.8b, v5.8b (in[i+1] + in[i+2])
    let v17 = vaddl_high_u8(v4, v5); // uaddl2 v17.8h, v4.16b, v5.16b
    let v18 = vaddl_u8(vget_low_u8(v0), vget_low_u8(v6)); // uaddl v18.8h, v0.8b, v6.8b (in[i+0] + in[i+3])
    let v19 = vaddl_high_u8(v0, v6); // uaddl2 v19.8h, v0.16b, v6.16b
    let v16 = vmulq_u16(v16, v31); // mul v16.8h, v16.8h, v31.8h (9*(in[i+1] + in[i+2]))
    let v17 = vmulq_u16(v17, v31); // mul v17.8h, v17.8h, v31.8h
    let v16 = vsubq_u16(v16, v18); // sub v16.8h, v16.8h, v18.8h
    let v17 = vsubq_u16(v17, v19); // sub v17.8h, v17.8h, v19.8h

    let v16 = vqrshrun_n_s16::<4>(vreinterpretq_s16_u16(v16)); // sqrshrun v16.8b, v16.8h, #4
    let v16 = vqrshrun_high_n_s16::<4>(v16, vreinterpretq_s16_u16(v17)); // sqrshrun2 v16.16b, v17.8h, #4

    let v0 = vzip1q_u8(v4, v16); // zip1 v0.16b, v4.16b, v16.16b
    let v1 = vzip2q_u8(v4, v16); // zip2 v1.16b, v4.16b, v16.16b

    vst1q_u8_x2(x0, uint8x16x2_t(v0, v1)); // st1 {v0.16b, v1.16b}, [x0]
}

/// `dav1d_ipred_z2_upsample_edge_8bpc_neon(pixel *out, const int sz, const pixel *const in)`
///
/// Here, sz is 4 or 8, and we produce 2*sz+1 output elements.
///
/// # Safety
/// Reads `in[0..16]` (the bytes above `in[sz]` are replaced through the
/// padding mask) and `in[sz]`; writes `out[0..17]`.
pub unsafe extern "C" fn dav1d_ipred_z2_upsample_edge_8bpc_neon(out: *mut u8, sz: c_int, in_: *const u8) {
    let (x0, w1, x2) = (out, sz as u32 as usize, in_);
    let x4 = padding_mask(); // movrel x4, padding_mask
    let v0 = vld1q_u8(x2); // ld1 {v0.16b}, [x2] (in[])
    let x5 = x2.add(w1); // add x5, x2, w1, uxtw (in[sz])
    let x4 = x4.sub(w1); // sub x4, x4, w1, uxtw

    let v2 = vld1q_dup_u8(x2); // ld1r {v2.16b}, [x2] (in[0] for padding)
    let v1 = vld1q_dup_u8(x5); // ld1r {v1.16b}, [x5] (padding)
    let v3 = vld1q_u8(x4); // ld1 {v3.16b}, [x4] (padding_mask)

    let v31 = vdupq_n_u16(9); // movi v31.8h, #9

    let v0 = vbslq_u8(v3, v1, v0); // bit v0.16b, v1.16b, v3.16b (padded in[])

    let v4 = vextq_u8::<15>(v2, v0); // ext v4.16b, v2.16b, v0.16b, #15
    let v5 = vextq_u8::<1>(v0, v1); // ext v5.16b, v0.16b, v1.16b, #1
    let v6 = vextq_u8::<2>(v0, v1); // ext v6.16b, v0.16b, v1.16b, #2

    let v16 = vaddl_u8(vget_low_u8(v0), vget_low_u8(v5)); // uaddl v16.8h, v0.8b, v5.8b (in[i+0] + in[i+1])
    let v18 = vaddl_u8(vget_low_u8(v4), vget_low_u8(v6)); // uaddl v18.8h, v4.8b, v6.8b (in[i-1] + in[i+2])
    let v16 = vmulq_u16(v16, v31); // mul v16.8h, v16.8h, v31.8h (9*(in[i+1] + in[i+2]))
    let v16 = vsubq_u16(v16, v18); // sub v16.8h, v16.8h, v18.8h

    let v16 = vqrshrun_n_s16::<4>(vreinterpretq_s16_u16(v16)); // sqrshrun v16.8b, v16.8h, #4

    let x5 = x0.add(16); // add x5, x0, #16

    let v2 = vzip1q_u8(v0, vcombine_u8(v16, vdup_n_u8(0))); // zip1 v2.16b, v0.16b, v16.16b

    vst1q_lane_u8::<0>(x5, v1); // st1 {v1.b}[0], [x5]
                                // In case sz=8, output one single pixel in out[16].
    vst1q_u8(x0, v2); // st1 {v2.16b}, [x0]
}

/// `edge_filter`
static EDGE_FILTER: [u8; 8] = [
    0, 4, 8, 0, //
    0, 5, 6, 0,
    // Leaving out the coeffs for strength=3
    //  2, 4, 4, 0
];

/// "Filter one block" of the three-tap path of `ipred_z1_filter_edge`:
/// `v30` holds kernel[1], `v31` kernel[2]; the result is the assembly's `v4`.
#[inline(always)]
unsafe fn ipred_z1_filter_edge_tap3(v0: uint8x16_t, v1: uint8x16_t, v30: uint8x16_t, v31: uint8x16_t) -> uint8x16_t {
    let v2 = vextq_u8::<1>(v0, v1); // ext v2.16b, v0.16b, v1.16b, #1
    let v3 = vextq_u8::<2>(v0, v1); // ext v3.16b, v0.16b, v1.16b, #2
    let v4 = vmull_u8(vget_low_u8(v0), vget_low_u8(v30)); // umull v4.8h, v0.8b, v30.8b
    let v4 = vmlal_u8(v4, vget_low_u8(v2), vget_low_u8(v31)); // umlal v4.8h, v2.8b, v31.8b
    let v4 = vmlal_u8(v4, vget_low_u8(v3), vget_low_u8(v30)); // umlal v4.8h, v3.8b, v30.8b
    let v5 = vmull_high_u8(v0, v30); // umull2 v5.8h, v0.16b, v30.16b
    let v5 = vmlal_high_u8(v5, v2, v31); // umlal2 v5.8h, v2.16b, v31.16b
    let v5 = vmlal_high_u8(v5, v3, v30); // umlal2 v5.8h, v3.16b, v30.16b
    let v4 = vrshrn_n_u16::<4>(v4); // rshrn v4.8b, v4.8h, #4
    vrshrn_high_n_u16::<4>(v4, v5) // rshrn2 v4.16b, v5.8h, #4
}

/// "Filter one block" of `L(fivetap)` of `ipred_z1_filter_edge`: `v29` = 2,
/// `v30` = `v31` = 4; the result is the assembly's `v6`.
#[inline(always)]
unsafe fn ipred_z1_filter_edge_tap5(
    v0: uint8x16_t,
    v1: uint8x16_t,
    v29: uint8x16_t,
    v30: uint8x16_t,
    v31: uint8x16_t,
) -> uint8x16_t {
    let v2 = vextq_u8::<1>(v0, v1); // ext v2.16b, v0.16b, v1.16b, #1
    let v3 = vextq_u8::<2>(v0, v1); // ext v3.16b, v0.16b, v1.16b, #2
    let v4 = vextq_u8::<3>(v0, v1); // ext v4.16b, v0.16b, v1.16b, #3
    let v5 = vextq_u8::<4>(v0, v1); // ext v5.16b, v0.16b, v1.16b, #4
    let v6 = vmull_u8(vget_low_u8(v0), vget_low_u8(v29)); // umull v6.8h, v0.8b, v29.8b
    let v6 = vmlal_u8(v6, vget_low_u8(v2), vget_low_u8(v30)); // umlal v6.8h, v2.8b, v30.8b
    let v6 = vmlal_u8(v6, vget_low_u8(v3), vget_low_u8(v31)); // umlal v6.8h, v3.8b, v31.8b
    let v6 = vmlal_u8(v6, vget_low_u8(v4), vget_low_u8(v30)); // umlal v6.8h, v4.8b, v30.8b
    let v6 = vmlal_u8(v6, vget_low_u8(v5), vget_low_u8(v29)); // umlal v6.8h, v5.8b, v29.8b
    let v7 = vmull_high_u8(v0, v29); // umull2 v7.8h, v0.16b, v29.16b
    let v7 = vmlal_high_u8(v7, v2, v30); // umlal2 v7.8h, v2.16b, v30.16b
    let v7 = vmlal_high_u8(v7, v3, v31); // umlal2 v7.8h, v3.16b, v31.16b
    let v7 = vmlal_high_u8(v7, v4, v30); // umlal2 v7.8h, v4.16b, v30.16b
    let v7 = vmlal_high_u8(v7, v5, v29); // umlal2 v7.8h, v5.16b, v29.16b
    let v6 = vrshrn_n_u16::<4>(v6); // rshrn v6.8b, v6.8h, #4
    vrshrn_high_n_u16::<4>(v6, v7) // rshrn2 v6.16b, v7.8h, #4
}

/// `dav1d_ipred_z1_filter_edge_8bpc_neon(pixel *out, const int sz, const pixel *const in, const int end,
/// const int strength)`
///
/// # Safety
/// `strength` is 1, 2 or 3 and `end` >= 0 (the callers pass at least 4).
/// Writes `out[0..n]` with `n` = `sz` rounded up to 16 (16 for `sz` <= 0).
/// Block `j` (16 output pixels) loads 16 input pixels more before it
/// decides between filtering and padding, and the padding starts at the
/// first block with `end - 16*j < 17` (`< 18` at strength 3). With `j` the
/// last block that loads, the smaller of that block and the last output
/// block:
/// - strength 1, 2: reads `in[0..16*(j+2)]`, and `in[end]` when the padding
///   starts;
/// - strength 3: reads `in[-1..16*(j+2)-1]` (`in[-1]` is loaded and replaced
///   by `in[0]`), and `in[end]` when the padding starts.
/// This is up to 30 pixels above `in[end]` (three taps) and up to 28 (five
/// taps); those values are replaced through the padding mask or not used.
pub unsafe extern "C" fn dav1d_ipred_z1_filter_edge_8bpc_neon(
    out: *mut u8,
    sz: c_int,
    in_: *const u8,
    end: c_int,
    strength: c_int,
) {
    let (mut x0, mut w1, mut x2, mut w3, w4) = (out, sz, in_, end, strength);
    // cmp w4, #3; b.eq L(fivetap) (if (strength == 3) goto fivetap)
    if w4 == 3 {
        return ipred_z1_filter_edge_fivetap(x0, w1, x2, w3);
    }

    // movrel x5, edge_filter, -3; add x5, x5, w4, uxtw #2 (edge_filter + (strength - 1)*4 + 1)
    let x5 = EDGE_FILTER
        .as_ptr()
        .wrapping_add(((w4 as u32 as usize) << 2).wrapping_sub(3));

    // ld1 {v31.h}[0], [x5] (kernel[1-2])
    let v31 = vreinterpretq_u8_u16(vsetq_lane_u16::<0>(
        core::ptr::read_unaligned(x5 as *const u16),
        vdupq_n_u16(0),
    ));

    let mut v0 = vld1q_u8(x2); // ld1 {v0.16b}, [x2], #16
    x2 = x2.wrapping_add(16);

    let v30 = vdupq_laneq_u8::<0>(v31); // dup v30.16b, v31.b[0]
    let v31 = vdupq_laneq_u8::<1>(v31); // dup v31.16b, v31.b[1]
    loop {
        // 1:
        // in[end], is the last valid pixel. We produce 16 pixels out by
        // using 18 pixels in - the last pixel used is [17] of the ones
        // read/buffered.
        let v1 = vld1q_u8(x2); // cmp w3, #17; ld1 {v1.16b}, [x2], #16
        x2 = x2.wrapping_add(16);
        if w3 < 17 {
            break; // b.lt 2f
        }
        let v4 = ipred_z1_filter_edge_tap3(v0, v1, v30, v31);
        w1 -= 16; // subs w1, w1, #16
        v0 = v1; // mov v0.16b, v1.16b
        w3 -= 16; // sub w3, w3, #16
        vst1q_u8(x0, v4); // st1 {v4.16b}, [x0], #16
        x0 = x0.wrapping_add(16);
        if w1 <= 0 {
            return; // b.gt 1b; ret
        }
    }
    // 2:
    // Right padding

    // x2[w3-32] is the padding pixel (x2 points 32 bytes ahead)
    let x5 = padding_mask(); // movrel x5, padding_mask
    let w6 = w3 - 32; // sub w6, w3, #32
    let x5 = x5.sub(w3 as u32 as usize); // sub x5, x5, w3, uxtw
    let x6 = x2.wrapping_offset(w6 as isize); // add x6, x2, w6, sxtw

    let v2 = vld1q_u8(x5); // ld1 {v2.16b}, [x5] (padding_mask)

    let v1 = vld1q_dup_u8(x6); // ld1r {v1.16b}, [x6]
    let v0 = vbslq_u8(v2, v1, v0); // bit v0.16b, v1.16b, v2.16b (Pad v0-v1)

    // Filter one block
    let v4 = ipred_z1_filter_edge_tap3(v0, v1, v30, v31);
    w1 -= 16; // subs w1, w1, #16
    vst1q_u8(x0, v4); // st1 {v4.16b}, [x0], #16
    x0 = x0.wrapping_add(16);
    if w1 <= 0 {
        return; // b.le 9f
    }
    loop {
        // 5:
        // After one block, any remaining output would only be filtering
        // padding - thus just store the padding.
        w1 -= 16; // subs w1, w1, #16
        vst1q_u8(x0, v1); // st1 {v1.16b}, [x0], #16
        x0 = x0.wrapping_add(16);
        if w1 <= 0 {
            return; // b.gt 5b; 9: ret
        }
    }
}

/// `L(fivetap)` of `ipred_z1_filter_edge_8bpc_neon` (strength 3), with the
/// function's registers `x0` (out), `w1` (sz), `x2` (in), `w3` (end).
#[inline(always)]
unsafe fn ipred_z1_filter_edge_fivetap(mut x0: *mut u8, mut w1: c_int, x2: *const u8, mut w3: c_int) {
    let mut x2 = x2.sub(1); // sub x2, x2, #1 (topleft -= 1)
    let v29 = vdupq_n_u8(2); // movi v29.16b, #2
    let mut v0 = vld1q_u8(x2); // ld1 {v0.16b}, [x2], #16
    x2 = x2.wrapping_add(16);
    let v30 = vdupq_n_u8(4); // movi v30.16b, #4
    let v31 = vdupq_n_u8(4); // movi v31.16b, #4
    v0 = vsetq_lane_u8::<0>(vgetq_lane_u8::<1>(v0), v0); // ins v0.b[0], v0.b[1]
    let mut v1;
    loop {
        // 1:
        // in[end+1], is the last valid pixel. We produce 16 pixels out by
        // using 20 pixels in - the last pixel used is [19] of the ones
        // read/buffered.
        v1 = vld1q_u8(x2); // cmp w3, #18; ld1 {v1.16b}, [x2], #16
        x2 = x2.wrapping_add(16);
        if w3 < 18 {
            break; // b.lt 2f (if (end + 1 < 19))
        }
        let v6 = ipred_z1_filter_edge_tap5(v0, v1, v29, v30, v31);
        w1 -= 16; // subs w1, w1, #16
        v0 = v1; // mov v0.16b, v1.16b
        w3 -= 16; // sub w3, w3, #16
        vst1q_u8(x0, v6); // st1 {v6.16b}, [x0], #16
        x0 = x0.wrapping_add(16);
        if w1 <= 0 {
            return; // b.gt 1b; ret
        }
    }
    // 2:
    // Right padding

    // x2[w3+1-32] is the padding pixel (x2 points 32 bytes ahead)
    let x5 = padding_mask().sub(1); // movrel x5, padding_mask, -1
    let w6 = w3 - 31; // sub w6, w3, #31
    let x5 = x5.sub(w3 as u32 as usize); // sub x5, x5, w3, uxtw
    let x6 = x2.wrapping_offset(w6 as isize); // add x6, x2, w6, sxtw

    let uint8x16x2_t(v2, v3) = vld1q_u8_x2(x5); // ld1 {v2.16b, v3.16b}, [x5] (padding_mask)

    let v28 = vld1q_dup_u8(x6); // ld1r {v28.16b}, [x6]
    v0 = vbslq_u8(v2, v28, v0); // bit v0.16b, v28.16b, v2.16b (Pad v0-v1)
    v1 = vbslq_u8(v3, v28, v1); // bit v1.16b, v28.16b, v3.16b
    loop {
        // 4:
        // Filter one block
        let v6 = ipred_z1_filter_edge_tap5(v0, v1, v29, v30, v31);
        w1 -= 16; // subs w1, w1, #16
        v0 = v1; // mov v0.16b, v1.16b
        v1 = v28; // mov v1.16b, v28.16b
        w3 -= 16; // sub w3, w3, #16
        vst1q_u8(x0, v6); // st1 {v6.16b}, [x0], #16
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
        w1 -= 16; // subs w1, w1, #16
        vst1q_u8(x0, v1); // st1 {v1.16b}, [x0], #16
        x0 = x0.wrapping_add(16);
        if w1 <= 0 {
            return; // b.gt 5b; 9: ret
        }
    }
}

/// `dav1d_ipred_pixel_set_8bpc_neon(pixel *out, const pixel px, const int n)`
///
/// # Safety
/// Writes `out[0..m]` with `m` = `n` rounded up to 16 (16 for `n` <= 0);
/// reads no memory.
pub unsafe extern "C" fn dav1d_ipred_pixel_set_8bpc_neon(out: *mut u8, px: u8, n: c_int) {
    let (mut x0, mut w2) = (out, n);
    let v0 = vdupq_n_u8(px); // dup v0.16b, w1
    loop {
        // 1:
        w2 -= 16; // subs w2, w2, #16
        vst1q_u8(x0, v0); // st1 {v0.16b}, [x0], #16
        x0 = x0.wrapping_add(16);
        if w2 <= 0 {
            return; // b.gt 1b; ret
        }
    }
}

/// `dav1d_ipred_z1_fill1_8bpc_neon(pixel *dst, const ptrdiff_t stride, const pixel *const top, const int width,
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
/// rows `0..height`, columns `0..width`. Reads `top[max_base_x]` and, for
/// each pair of rows whose first row has `base` < `max_base_x`, at the
/// `base` of either row: `top[base..base+8]` (width 4),
/// `top[base..base+16]` (width 8), `top[base..base+width+16]` (width >= 16);
/// the second row's `base` is not compared and can reach
/// `max_base_x + 15`.
pub unsafe extern "C" fn dav1d_ipred_z1_fill1_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    top: *const u8,
    width: c_int,
    height: c_int,
    dx: c_int,
    max_base_x: c_int,
) {
    let (mut x0, x1, x2, w3, mut w4, w5, w6) = (dst, stride, top, width, height, dx as u32 as usize, max_base_x);
    // clz w9, w3; movrel x8, ipred_z1_fill1_tbl; sub w9, w9, #25; ldrsw x9, [x8, w9, uxtw #2]
    let x10 = x2.add(w6 as u32 as usize); // add x10, x2, w6, uxtw (top[max_base_x])
    let v31 = vld1q_dup_u8(x10); // ld1r {v31.16b}, [x10] (padding)
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
                let v0 = vld1_u8(x2.add(w8)); // ldr d0, [x2, w8, uxtw] (top[base])
                let v2 = vld1_u8(x2.add(w10)); // ldr d2, [x2, w10, uxtw]
                let v4 = vdup_n_u16(w9 as u16); // dup v4.4h, w9 (frac)
                let v5 = vdup_n_u16(w11 as u16); // dup v5.4h, w11
                let v1 = vext_u8::<1>(v0, v0); // ext v1.8b, v0.8b, v0.8b, #1 (top[base+1])
                let v3 = vext_u8::<1>(v2, v2); // ext v3.8b, v2.8b, v2.8b, #1
                let v6 = vsubl_u8(v1, v0); // usubl v6.8h, v1.8b, v0.8b (top[base+1]-top[base])
                let v7 = vsubl_u8(v3, v2); // usubl v7.8h, v3.8b, v2.8b
                let v16 = vshll_n_u8::<6>(v0); // ushll v16.8h, v0.8b, #6 (top[base]*64)
                let v17 = vshll_n_u8::<6>(v2); // ushll v17.8h, v2.8b, #6
                                               // mla v16.4h, v6.4h, v4.4h (+ top[base+1]*frac)
                let v16 = vmla_u16(vget_low_u16(v16), vget_low_u16(v6), v4);
                let v17 = vmla_u16(vget_low_u16(v17), vget_low_u16(v7), v5); // mla v17.4h, v7.4h, v5.4h
                let v16 = vrshrn_n_u16::<6>(vcombine_u16(v16, vdup_n_u16(0))); // rshrn v16.8b, v16.8h, #6
                let v17 = vrshrn_n_u16::<6>(vcombine_u16(v17, vdup_n_u16(0))); // rshrn v17.8b, v17.8h, #6
                ipred_z1_st1_s0(x0, v16); // st1 {v16.s}[0], [x0], x1
                w7 = w7.wrapping_add(w5 << 1); // add w7, w7, w5 (xpos += dx; with the one above)
                w4 -= 2; // subs w4, w4, #2
                ipred_z1_st1_s0(x0.wrapping_offset(x1), v17); // st1 {v17.s}[0], [x0], x1
                x0 = x0.wrapping_offset(x1 << 1);
                if w4 <= 0 {
                    return; // b.gt 4b; ret
                }
            }
            loop {
                // 49:
                ipred_z1_st1_s0(x0, vget_low_u8(v31)); // st1 {v31.s}[0], [x0], x1
                w4 -= 2; // subs w4, w4, #2
                ipred_z1_st1_s0(x0.wrapping_offset(x1), vget_low_u8(v31)); // st1 {v31.s}[0], [x0], x1
                x0 = x0.wrapping_offset(x1 << 1);
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
                let v0 = vld1q_u8(x2.add(w8)); // ldr q0, [x2, w8, uxtw] (top[base])
                let v2 = vld1q_u8(x2.add(w10)); // ldr q2, [x2, w10, uxtw]
                let v4 = vdup_n_u8(w9 as u8); // dup v4.8b, w9 (frac)
                let v5 = vdup_n_u8(w11 as u8); // dup v5.8b, w11
                let w9 = w15 - w9; // sub w9, w15, w9 (64 - frac)
                let w11 = w15 - w11; // sub w11, w15, w11
                let v6 = vdup_n_u8(w9 as u8); // dup v6.8b, w9 (64 - frac)
                let v7 = vdup_n_u8(w11 as u8); // dup v7.8b, w11
                let v1 = vextq_u8::<1>(v0, v0); // ext v1.16b, v0.16b, v0.16b, #1 (top[base+1])
                let v3 = vextq_u8::<1>(v2, v2); // ext v3.16b, v2.16b, v2.16b, #1
                let v16 = vmull_u8(vget_low_u8(v0), v6); // umull v16.8h, v0.8b, v6.8b (top[base]*(64-frac))
                let v16 = vmlal_u8(v16, vget_low_u8(v1), v4); // umlal v16.8h, v1.8b, v4.8b (+ top[base+1]*frac)
                let v17 = vmull_u8(vget_low_u8(v2), v7); // umull v17.8h, v2.8b, v7.8b
                let v17 = vmlal_u8(v17, vget_low_u8(v3), v5); // umlal v17.8h, v3.8b, v5.8b
                let v16 = vrshrn_n_u16::<6>(v16); // rshrn v16.8b, v16.8h, #6
                let v17 = vrshrn_n_u16::<6>(v17); // rshrn v17.8b, v17.8h, #6
                vst1_u8(x0, v16); // st1 {v16.8b}, [x0], x1
                w7 = w7.wrapping_add(w5 << 1); // add w7, w7, w5 (xpos += dx; with the one above)
                w4 -= 2; // subs w4, w4, #2
                vst1_u8(x0.wrapping_offset(x1), v17); // st1 {v17.8b}, [x0], x1
                x0 = x0.wrapping_offset(x1 << 1);
                if w4 <= 0 {
                    return; // b.gt 8b; ret
                }
            }
            loop {
                // 89:
                vst1_u8(x0, vget_low_u8(v31)); // st1 {v31.8b}, [x0], x1
                w4 -= 2; // subs w4, w4, #2
                vst1_u8(x0.wrapping_offset(x1), vget_low_u8(v31)); // st1 {v31.8b}, [x0], x1
                x0 = x0.wrapping_offset(x1 << 1);
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

/// The labels `160:`, `320:`, `640:` of `ipred_z1_fill1_8bpc_neon` (one
/// shared block in the assembly), with the function's registers; `W` is the
/// width (`w3`), a constant of each instance so that the compiler sees the
/// number of 16-pixel columns.
#[inline(always)]
unsafe fn ipred_z1_fill1_160_320_640<const W: i32>(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w4: c_int,
    w5: usize,
    w6: c_int,
    v31: uint8x16_t,
) {
    let mut w3 = W;
    let mut w7 = w5; // mov w7, w5
    let w15: u32 = 64; // mov w15, #64
    let w12 = w3; // mov w12, w3

    let mut x13 = x0.wrapping_offset(x1); // add x13, x0, x1
    let x1 = x1 << 1; // lsl x1, x1, #1
    let x1 = x1 - (w3 as u32 as isize); // sub x1, x1, w3, uxtw
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
        let mut x8 = x2.add(w8); // add x8, x2, w8, uxtw
        let mut x10 = x2.add(w10); // add x10, x2, w10, uxtw
        let v4 = vdupq_n_u8(w9 as u8); // dup v4.16b, w9 (frac)
        let v5 = vdupq_n_u8(w11 as u8); // dup v5.16b, w11
        let uint8x16x2_t(mut v0, mut v1) = vld1q_u8_x2(x8); // ld1 {v0.16b, v1.16b}, [x8], #32 (top[base])
        x8 = x8.wrapping_add(32);
        let uint8x16x2_t(mut v2, mut v3) = vld1q_u8_x2(x10); // ld1 {v2.16b, v3.16b}, [x10], #32
        x10 = x10.wrapping_add(32);
        let w9 = w15 - w9; // sub w9, w15, w9 (64 - frac)
        let w11 = w15 - w11; // sub w11, w15, w11
        let v6 = vdupq_n_u8(w9 as u8); // dup v6.16b, w9 (64 - frac)
        let v7 = vdupq_n_u8(w11 as u8); // dup v7.16b, w11
        w7 = w7.wrapping_add(w5); // add w7, w7, w5 (xpos += dx)
        loop {
            // 2:
            let v16 = vextq_u8::<1>(v0, v1); // ext v16.16b, v0.16b, v1.16b, #1 (top[base+1])
            let v17 = vextq_u8::<1>(v2, v3); // ext v17.16b, v2.16b, v3.16b, #1
            w3 -= 16; // subs w3, w3, #16
            let v18 = vmull_u8(vget_low_u8(v0), vget_low_u8(v6)); // umull v18.8h, v0.8b, v6.8b (top[base]*(64-frac))
            let v18 = vmlal_u8(v18, vget_low_u8(v16), vget_low_u8(v4)); // umlal v18.8h, v16.8b, v4.8b
            let v19 = vmull_high_u8(v0, v6); // umull2 v19.8h, v0.16b, v6.16b
            let v19 = vmlal_high_u8(v19, v16, v4); // umlal2 v19.8h, v16.16b, v4.16b
            let v20 = vmull_u8(vget_low_u8(v2), vget_low_u8(v7)); // umull v20.8h, v2.8b, v7.8b
            let v20 = vmlal_u8(v20, vget_low_u8(v17), vget_low_u8(v5)); // umlal v20.8h, v17.8b, v5.8b
            let v21 = vmull_high_u8(v2, v7); // umull2 v21.8h, v2.16b, v7.16b
            let v21 = vmlal_high_u8(v21, v17, v5); // umlal2 v21.8h, v17.16b, v5.16b
            let v16 = vrshrn_n_u16::<6>(v18); // rshrn v16.8b, v18.8h, #6
            let v16 = vrshrn_high_n_u16::<6>(v16, v19); // rshrn2 v16.16b, v19.8h, #6
            let v17 = vrshrn_n_u16::<6>(v20); // rshrn v17.8b, v20.8h, #6
            let v17 = vrshrn_high_n_u16::<6>(v17, v21); // rshrn2 v17.16b, v21.8h, #6
            vst1q_u8(x0, v16); // st1 {v16.16b}, [x0], #16
            x0 = x0.wrapping_add(16);
            vst1q_u8(x13, v17); // st1 {v17.16b}, [x13], #16
            x13 = x13.wrapping_add(16);
            if w3 <= 0 {
                break; // b.le 3f
            }
            v0 = v1; // mov v0.16b, v1.16b
            v1 = vld1q_u8(x8); // ld1 {v1.16b}, [x8], #16 (top[base])
            x8 = x8.wrapping_add(16);
            v2 = v3; // mov v2.16b, v3.16b
            v3 = vld1q_u8(x10); // ld1 {v3.16b}, [x10], #16
            x10 = x10.wrapping_add(16);
            // b 2b
        }
        // 3:
        w4 -= 2; // subs w4, w4, #2
        if w4 <= 0 {
            return; // b.le 9f; 9: ret
        }
        x0 = x0.wrapping_offset(x1); // add x0, x0, x1
        x13 = x13.wrapping_offset(x1); // add x13, x13, x1
        w3 = w12; // mov w3, w12
                  // b 1b
    }
    loop {
        // 169:
        vst1q_u8(x0, v31); // st1 {v31.16b}, [x0], #16
        x0 = x0.wrapping_add(16);
        w3 -= 16; // subs w3, w3, #16
        vst1q_u8(x13, v31); // st1 {v31.16b}, [x13], #16
        x13 = x13.wrapping_add(16);
        if w3 > 0 {
            continue; // b.gt 169b
        }
        w4 -= 2; // subs w4, w4, #2
        if w4 <= 0 {
            return; // b.le 9b
        }
        x0 = x0.wrapping_offset(x1); // add x0, x0, x1
        x13 = x13.wrapping_offset(x1); // add x13, x13, x1
        w3 = w12; // mov w3, w12
                  // b 169b
    }
}

/// `dav1d_ipred_z1_fill2_8bpc_neon` (same parameters as `fill1`; `top` is
/// the upsampled edge, `dx` and `max_base_x` are in its units). `w5`, `w7`
/// and the pairs of stores as in `fill1`.
///
/// # Safety
/// `width` is 4 or 8, `height` even and positive. Writes `dst` rows
/// `0..height`, columns `0..width`. Reads `top[max_base_x]` and, for each
/// pair of rows whose first row has `base` < `max_base_x`, at the `base` of
/// either row: `top[base..base+8]` (width 4) or `top[base..base+16]`
/// (width 8); the second row's `base` is not compared and can reach
/// `max_base_x + 31`.
pub unsafe extern "C" fn dav1d_ipred_z1_fill2_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    top: *const u8,
    width: c_int,
    height: c_int,
    dx: c_int,
    max_base_x: c_int,
) {
    let (mut x0, x1, x2, w3, mut w4, w5, w6) = (dst, stride, top, width, height, dx as u32 as usize, max_base_x);
    let x10 = x2.add(w6 as u32 as usize); // add x10, x2, w6, uxtw (top[max_base_x])
    let v31 = vld1q_dup_u8(x10); // ld1r {v31.16b}, [x10] (padding)
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
            let v0 = vld1_u8(x2.add(w8)); // ldr d0, [x2, w8, uxtw] (top[base])
            let v2 = vld1_u8(x2.add(w10)); // ldr d2, [x2, w10, uxtw]
            let v4 = vdup_n_u16(w9 as u16); // dup v4.4h, w9 (frac)
            let v5 = vdup_n_u16(w11 as u16); // dup v5.4h, w11
            let v1 = vuzp2_u8(v0, v0); // uzp2 v1.8b, v0.8b, v0.8b (top[base+1])
            let v0 = vuzp1_u8(v0, v0); // uzp1 v0.8b, v0.8b, v0.8b (top[base])
            let v3 = vuzp2_u8(v2, v2); // uzp2 v3.8b, v2.8b, v2.8b
            let v2 = vuzp1_u8(v2, v2); // uzp1 v2.8b, v2.8b, v2.8b
            let v6 = vsubl_u8(v1, v0); // usubl v6.8h, v1.8b, v0.8b (top[base+1]-top[base])
            let v7 = vsubl_u8(v3, v2); // usubl v7.8h, v3.8b, v2.8b
            let v16 = vshll_n_u8::<6>(v0); // ushll v16.8h, v0.8b, #6 (top[base]*64)
            let v17 = vshll_n_u8::<6>(v2); // ushll v17.8h, v2.8b, #6
                                           // mla v16.4h, v6.4h, v4.4h (+ top[base+1]*frac)
            let v16 = vmla_u16(vget_low_u16(v16), vget_low_u16(v6), v4);
            let v17 = vmla_u16(vget_low_u16(v17), vget_low_u16(v7), v5); // mla v17.4h, v7.4h, v5.4h
            let v16 = vrshrn_n_u16::<6>(vcombine_u16(v16, vdup_n_u16(0))); // rshrn v16.8b, v16.8h, #6
            let v17 = vrshrn_n_u16::<6>(vcombine_u16(v17, vdup_n_u16(0))); // rshrn v17.8b, v17.8h, #6
            ipred_z1_st1_s0(x0, v16); // st1 {v16.s}[0], [x0], x1
            w7 = w7.wrapping_add(w5 << 1); // add w7, w7, w5 (xpos += dx; with the one above)
            w4 -= 2; // subs w4, w4, #2
            ipred_z1_st1_s0(x0.wrapping_offset(x1), v17); // st1 {v17.s}[0], [x0], x1
            x0 = x0.wrapping_offset(x1 << 1);
            if w4 <= 0 {
                return; // b.gt 4b; ret
            }
        }
        loop {
            // 49:
            ipred_z1_st1_s0(x0, vget_low_u8(v31)); // st1 {v31.s}[0], [x0], x1
            w4 -= 2; // subs w4, w4, #2
            ipred_z1_st1_s0(x0.wrapping_offset(x1), vget_low_u8(v31)); // st1 {v31.s}[0], [x0], x1
            x0 = x0.wrapping_offset(x1 << 1);
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
        let v0 = vld1q_u8(x2.add(w8)); // ldr q0, [x2, w8, uxtw] (top[base])
        let v2 = vld1q_u8(x2.add(w10)); // ldr q2, [x2, w10, uxtw]
        let v4 = vdup_n_u8(w9 as u8); // dup v4.8b, w9 (frac)
        let v5 = vdup_n_u8(w11 as u8); // dup v5.8b, w11
        let w9 = w15 - w9; // sub w9, w15, w9 (64 - frac)
        let w11 = w15 - w11; // sub w11, w15, w11
        let v6 = vdup_n_u8(w9 as u8); // dup v6.8b, w9 (64 - frac)
        let v7 = vdup_n_u8(w11 as u8); // dup v7.8b, w11
        let v1 = vuzp2q_u8(v0, v0); // uzp2 v1.16b, v0.16b, v0.16b (top[base+1])
        let v0 = vuzp1q_u8(v0, v0); // uzp1 v0.16b, v0.16b, v0.16b (top[base])
        let v3 = vuzp2q_u8(v2, v2); // uzp2 v3.16b, v2.16b, v2.16b
        let v2 = vuzp1q_u8(v2, v2); // uzp1 v2.16b, v2.16b, v2.16b
        let v16 = vmull_u8(vget_low_u8(v1), v4); // umull v16.8h, v1.8b, v4.8b (top[base+1]*frac)
        let v16 = vmlal_u8(v16, vget_low_u8(v0), v6); // umlal v16.8h, v0.8b, v6.8b (+ top[base]*(64-frac))
        let v17 = vmull_u8(vget_low_u8(v3), v5); // umull v17.8h, v3.8b, v5.8b
        let v17 = vmlal_u8(v17, vget_low_u8(v2), v7); // umlal v17.8h, v2.8b, v7.8b
        let v16 = vrshrn_n_u16::<6>(v16); // rshrn v16.8b, v16.8h, #6
        let v17 = vrshrn_n_u16::<6>(v17); // rshrn v17.8b, v17.8h, #6
        vst1_u8(x0, v16); // st1 {v16.8b}, [x0], x1
        w7 = w7.wrapping_add(w5 << 1); // add w7, w7, w5 (xpos += dx; with the one above)
        w4 -= 2; // subs w4, w4, #2
        vst1_u8(x0.wrapping_offset(x1), v17); // st1 {v17.8b}, [x0], x1
        x0 = x0.wrapping_offset(x1 << 1);
        if w4 <= 0 {
            return; // b.gt 8b; ret
        }
    }
    loop {
        // 89:
        vst1_u8(x0, vget_low_u8(v31)); // st1 {v31.8b}, [x0], x1
        w4 -= 2; // subs w4, w4, #2
        vst1_u8(x0.wrapping_offset(x1), vget_low_u8(v31)); // st1 {v31.8b}, [x0], x1
        x0 = x0.wrapping_offset(x1 << 1);
        if w4 <= 0 {
            return; // b.gt 89b; ret
        }
    }
}

/// `dav1d_ipred_reverse_8bpc_neon(pixel *dst, const pixel *const src, const int n)`
///
/// # Safety
/// With `m` = `n` rounded up to 16 (16 for `n` <= 0): reads `src[-m..0]`
/// and writes `dst[0..m]`, `dst[i] = src[-1 - i]`.
pub unsafe extern "C" fn dav1d_ipred_reverse_8bpc_neon(dst: *mut u8, src: *const u8, n: c_int) {
    let (mut x0, mut x1, mut w2) = (dst, src, n);
    x1 = x1.wrapping_sub(16); // sub x1, x1, #16
    let mut x3 = x0.wrapping_add(8); // add x3, x0, #8
                                     // mov x4, #16
    loop {
        // 1:
        let v0 = vld1q_u8(x1); // ld1 {v0.16b}, [x1]
        w2 -= 16; // subs w2, w2, #16
        let v0 = vreinterpretq_u64_u8(vrev64q_u8(v0)); // rev64 v0.16b, v0.16b
        x1 = x1.wrapping_sub(16); // sub x1, x1, #16
        core::ptr::write_unaligned(x0 as *mut u64, vgetq_lane_u64::<1>(v0)); // st1 {v0.d}[1], [x0], x4
        x0 = x0.wrapping_add(16);
        core::ptr::write_unaligned(x3 as *mut u64, vgetq_lane_u64::<0>(v0)); // st1 {v0.d}[0], [x3], x4
        x3 = x3.wrapping_add(16);
        if w2 <= 0 {
            return; // b.gt 1b; ret
        }
    }
}

// ============================================================================
// Part C (ipred.S lines 1851–3074): the z2 fill kernels.
// ============================================================================
// The vector registers are typed locals named after the assembly's
// registers (`v16`, `v29`, ...); a register that holds the 16-bit products
// of a `umull`/`umlal` pair is the local with the suffix `h` (`v16h`), and
// a multi-register table (`{v0.16b, v1.16b}`) is the local named after its
// registers (`v0v1`). General registers keep their names (`x0`, `w8`, ...).

/// `const increments, align=4`
static INCREMENTS: [u16; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

/// `trn1 vd.2s, vn.2s, vm.2s` on byte vectors.
#[inline(always)]
unsafe fn z2_trn1_2s(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    vreinterpret_u8_u32(vtrn1_u32(vreinterpret_u32_u8(a), vreinterpret_u32_u8(b)))
}

/// `trn1 vd.2d, vn.2d, vm.2d` on byte vectors.
#[inline(always)]
unsafe fn z2_trn1_2d(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    vreinterpretq_u8_u64(vtrn1q_u64(vreinterpretq_u64_u8(a), vreinterpretq_u64_u8(b)))
}

/// `st1 {v.s}[LANE], [x0]`
#[inline(always)]
unsafe fn z2_st1_s<const LANE: i32>(x0: *mut u8, v: uint8x8_t) {
    (x0 as *mut u32).write_unaligned(vget_lane_u32::<LANE>(vreinterpret_u32_u8(v)));
}

/// `ipred_z2_fill1_8bpc_neon`, label `40:` (width 4).
///
/// # Safety
/// As for `dav1d_ipred_z2_fill1_8bpc_neon` with `width == 4`; `w8` is the
/// entry's `xpos`, `w7` is `-dy`, `v31` the first half of `increments`.
unsafe fn ipred_z2_fill1_w4(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    x3: *const u8,
    mut w8: c_int,
    mut w5: c_int,
    w6: c_int,
    w7: c_int,
    v31: uint16x8_t,
) {
    let v30 = vdup_n_u16(w7 as u16); // dup v30.4h, w7  (-dy)
    let v17 = vdup_n_u8(1); // movi v17.8b, #1

    let v16 = vmul_u16(vget_low_u16(v31), v30); // mul v16.4h, v31.4h, v30.4h  ({0,1,2,3} * -dy)
    let v25 = vdup_n_u8(0x3e); // movi v25.16b, #0x3e
    let v30 = vadd_u16(v16, v30); // add v30.4h, v16.4h, v30.4h  (-= dy)

    let v31 = vmovn_u16(v31); // xtn v31.8b, v31.8h  ({0,1,2,3})

    // Worst case height for w=4 is 16, but we need at least h+1 elements
    let v0v1 = vld1q_u8_x2(x3); // ld1 {v0.16b, v1.16b}, [x3]  (left[])
    let v0 = v0v1.0;

    let v26 = vdup_n_u8(64); // movi v26.16b, #64
    let v19 = vdup_n_u8(2); // movi v19.16b, #2

    let v30 = vcombine_u16(v30, vdup_n_u16(0)); // (the .4h results cleared the high half)
    let v27 = vmovn_u16(v30); // xtn v27.8b, v30.8h  ((uint8_t)ypos)
    let v29 = vshrn_n_u16::<6>(v30); // shrn v29.8b, v30.8h, #6  (ypos >> 6)
    let v27 = vand_u8(v27, v25); // and v27.8b, v27.8b, v25.8b  (frac_y)

    let v29 = vadd_u8(v29, v17); // add v29.8b, v29.8b, v17.8b  (base_y = (ypos >> 6) + 1)

    let v30 = vadd_u8(v29, v17); // add v30.8b, v29.8b, v17.8b  (base_y + 1)
    let v28 = vadd_u8(v29, v19); // add v28.8b, v29.8b, v19.8b  (base_y + 2)

    let mut v16 = vqtbl1_u8(v0, v29); // tbl v16.8b, {v0.16b}, v29.8b  (left[base_y])

    let mut v30 = z2_trn1_2s(v30, v28); // trn1 v30.2s, v30.2s, v28.2s  (base_y + 1, base_y + 2)

    let v28 = vsub_u8(v26, v27); // sub v28.8b, v26.8b, v27.8b  (64 - frac_y)

    let v31 = z2_trn1_2s(v31, v31); // trn1 v31.2s, v31.2s, v31.2s  ({0,1,2,3,0,1,2,3})

    let v27 = z2_trn1_2s(v27, v27); // trn1 v27.2s, v27.2s, v27.2s  (frac_y)
    let v28 = z2_trn1_2s(v28, v28); // trn1 v28.2s, v28.2s, v28.2s  (64 - frac_y)

    let v29 = vdup_n_u8(2); // movi v29.8b, #2
    loop {
        // 4:
        let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
        let v6 = vdup_n_u16(w8 as u16); // dup v6.4h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
        if w9 <= -4 {
            break; // cmp w9, #-4; b.le 49f  (base_x <= -4)
        }

        let v7 = vdup_n_u16(w8 as u16); // dup v7.4h, w8  (xpos)

        let v2 = vld1_u8(x2.wrapping_offset(w9 as isize)); // ldr d2, [x2, w9, sxtw]  (top[base_x])
        let v4 = vld1_u8(x2.wrapping_offset(w11 as isize)); // ldr d4, [x2, w11, sxtw]

        let v6 = vcombine_u16(v6, v7); // trn1 v6.2d, v6.2d, v7.2d  (xpos)

        // Cut corners here; only doing tbl over v0 here; we only
        // seem to need the last pixel, from v1, after skipping to the
        // left-only codepath below.
        let v17 = vqtbl1_u8(v0, v30); // tbl v17.8b, {v0.16b}, v30.8b  (left[base_y+1], left[base_y+2])

        let v20 = vshrn_n_u16::<6>(v6); // shrn v20.8b, v6.8h, #6  (first base_x for each row)
        let v6 = vmovn_u16(v6); // xtn v6.8b, v6.8h  ((uint8_t)xpos)

        let v3 = vext_u8::<1>(v2, v2); // ext v3.8b, v2.8b, v2.8b, #1  (top[base_x+1])
        let v5 = vext_u8::<1>(v4, v4); // ext v5.8b, v4.8b, v4.8b, #1

        let v6 = vand_u8(v6, v25); // and v6.8b, v6.8b, v25.8b  (frac_x)

        v16 = z2_trn1_2s(v16, v17); // trn1 v16.2s, v16.2s, v17.2s  (left[base_y], left[base_y+1])

        let v2 = z2_trn1_2s(v2, v4); // trn1 v2.2s, v2.2s, v4.2s  (top[base_x])
        let v3 = z2_trn1_2s(v3, v5); // trn1 v3.2s, v3.2s, v5.2s  (top[base_x+1])

        let v7 = vsub_u8(v26, v6); // sub v7.8b, v26.8b, v6.8b  (64 - frac_x)

        let v20 = vadd_u8(v20, v31); // add v20.8b, v20.8b, v31.8b  (actual base_x)

        let v16h = vmull_u8(v16, v28); // umull v16.8h, v16.8b, v28.8b  (left[base_y]*(64-frac_y))
        let v16h = vmlal_u8(v16h, v17, v27); // umlal v16.8h, v17.8b, v27.8b  (+ left[base_y+1]*frac_y)

        let v22h = vmull_u8(v2, v7); // umull v22.8h, v2.8b, v7.8b  (top[base_x]*(64-frac_x))
        let v22h = vmlal_u8(v22h, v3, v6); // umlal v22.8h, v3.8b, v6.8b  (+ top[base_x+1]*frac_x)

        let v20 = vcgez_s8(vreinterpret_s8_u8(v20)); // cmge v20.8b, v20.8b, #0

        v16 = vrshrn_n_u16::<6>(v16h); // rshrn v16.8b, v16.8h, #6
        let v22 = vrshrn_n_u16::<6>(v22h); // rshrn v22.8b, v22.8h, #6

        v16 = vbsl_u8(v20, v22, v16); // bit v16.8b, v22.8b, v20.8b

        z2_st1_s::<0>(x0, v16); // st1 {v16.s}[0], [x0], x1
        x0 = x0.wrapping_offset(x1);
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        w5 -= 2; // subs w5, w5, #2
        z2_st1_s::<1>(x0, v16); // st1 {v16.s}[1], [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v16 = vext_u8::<4>(v17, v17); // ext v16.8b, v17.8b, v17.8b, #4
        v30 = vadd_u8(v30, v29); // add v30.8b, v30.8b, v29.8b  (base_y += 2)
    }

    loop {
        // 49:
        let v17 = vqtbl2_u8(v0v1, v30); // tbl v17.8b, {v0.16b, v1.16b}, v30.8b  (left[base_y+1], left[base_y+2])

        v16 = z2_trn1_2s(v16, v17); // trn1 v16.2s, v16.2s, v17.2s  (left[base_y], left[base_y+1])

        let v18h = vmull_u8(v16, v28); // umull v18.8h, v16.8b, v28.8b  (left[base_y]*(64-frac_y))
        let v18h = vmlal_u8(v18h, v17, v27); // umlal v18.8h, v17.8b, v27.8b  (+ left[base_y+1]*frac_y)
        let v18 = vrshrn_n_u16::<6>(v18h); // rshrn v18.8b, v18.8h, #6

        z2_st1_s::<0>(x0, v18); // st1 {v18.s}[0], [x0], x1
        x0 = x0.wrapping_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        z2_st1_s::<1>(x0, v18); // st1 {v18.s}[1], [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v16 = vext_u8::<4>(v17, v17); // ext v16.8b, v17.8b, v17.8b, #4
        v30 = vadd_u8(v30, v29); // add v30.8b, v30.8b, v29.8b  (base_y += 2)
    }
}

/// `ipred_z2_fill1_8bpc_neon`, label `80:` (width 8).
///
/// # Safety
/// As for `dav1d_ipred_z2_fill1_8bpc_neon` with `width == 8`; `w8` is the
/// entry's `xpos`, `w7` is `-dy`, `v31` the first half of `increments`.
unsafe fn ipred_z2_fill1_w8(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    x3: *const u8,
    mut w8: c_int,
    mut w5: c_int,
    w6: c_int,
    w7: c_int,
    v31: uint16x8_t,
) {
    let v30 = vdupq_n_u16(w7 as u16); // dup v30.8h, w7  (-dy)
    let v17 = vdup_n_u8(1); // movi v17.8b, #1

    let v16 = vmulq_u16(v31, v30); // mul v16.8h, v31.8h, v30.8h  ({0,1,2,3,4,5,6,7} * -dy)
    let v25 = vdupq_n_u8(0x3e); // movi v25.16b, #0x3e
    let v30 = vaddq_u16(v16, v30); // add v30.8h, v16.8h, v30.8h  (-= dy)

    let v31 = vmovn_u16(v31); // xtn v31.8b, v31.8h  ({0,1,2,3,4,5,6,7})

    // Worst case height for w=8 is 32, but we need at least h+1 elements
    let v0v1v2 = vld1q_u8_x3(x3); // ld1 {v0.16b, v1.16b, v2.16b}, [x3]  (left[])
    let v0v1 = uint8x16x2_t(v0v1v2.0, v0v1v2.1);

    let v26 = vdupq_n_u8(64); // movi v26.16b, #64
    let v19 = vdup_n_u8(2); // movi v19.16b, #2

    let v27 = vmovn_u16(v30); // xtn v27.8b, v30.8h  ((uint8_t)ypos)
    let v29 = vshrn_n_u16::<6>(v30); // shrn v29.8b, v30.8h, #6  (ypos >> 6)
    let v27 = vand_u8(v27, vget_low_u8(v25)); // and v27.8b, v27.8b, v25.8b  (frac_y)

    let v29 = vadd_u8(v29, v17); // add v29.8b, v29.8b, v17.8b  (base_y = (ypos >> 6) + 1)

    // Cut corners here; for the first row we don't expect to need to
    // read outside of v0.
    let mut v18 = vqtbl1_u8(v0v1v2.0, v29); // tbl v18.8b, {v0.16b}, v29.8b  (left[base_y])

    let mut v30 = vadd_u8(v29, v19); // add v30.8b, v29.8b, v19.8b  (base_y + 2)
    let mut v29 = vadd_u8(v29, v17); // add v29.8b, v29.8b, v17.8b  (base_y + 1)

    let v28 = vsub_u8(vget_low_u8(v26), v27); // sub v28.8b, v26.8b, v27.8b  (64 - frac_y)

    let v31 = vcombine_u8(v31, v31); // trn1 v31.2d, v31.2d, v31.2d  ({0,1,2,3,4,5,6,7,0,1,2,3,4,5,6,7})

    let v24 = vdup_n_u8(2); // movi v24.8b, #2
    loop {
        // 8:
        let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
        let v16 = vdupq_n_u16(w8 as u16); // dup v16.8h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
        if w9 <= -8 {
            break; // cmp w9, #-8; b.le 89f  (base_x <= -8)
        }

        let v17 = vdupq_n_u16(w8 as u16); // dup v17.8h, w8  (xpos)

        let v4 = vld1q_u8(x2.wrapping_offset(w9 as isize)); // ldr q4, [x2, w9, sxtw]  (top[base_x])
        let v6 = vld1q_u8(x2.wrapping_offset(w11 as isize)); // ldr q6, [x2, w11, sxtw]

        // Cut corners here; only doing tbl over v0-v1 here; we only
        // seem to need the last pixel, from v2, after skipping to the
        // left-only codepath below.
        let v19 = vqtbl2_u8(v0v1, v29); // tbl v19.8b, {v0.16b, v1.16b}, v29.8b  (left[base_y+1])

        let v21 = vshrn_n_u16::<6>(v16); // shrn v21.8b, v16.8h, #6  (first base_x)
        let v21 = vshrn_high_n_u16::<6>(v21, v17); // shrn2 v21.16b, v17.8h, #6
        let v16 = vmovn_u16(v16); // xtn v16.8b, v16.8h  ((uint8_t)xpos)
        let v16 = vmovn_high_u16(v16, v17); // xtn2 v16.16b, v17.8h

        let v20 = vqtbl2_u8(v0v1, v30); // tbl v20.8b, {v0.16b, v1.16b}, v30.8b  (left[base_y+2])

        let v5 = vextq_u8::<1>(v4, v4); // ext v5.16b, v4.16b, v4.16b, #1  (top[base_x+1])
        let v7 = vextq_u8::<1>(v6, v6); // ext v7.16b, v6.16b, v6.16b, #1

        let v16 = vandq_u8(v16, v25); // and v16.16b, v16.16b, v25.16b  (frac_x)

        let v4 = z2_trn1_2d(v4, v6); // trn1 v4.2d, v4.2d, v6.2d  (top[base_x])
        let v5 = z2_trn1_2d(v5, v7); // trn1 v5.2d, v5.2d, v7.2d  (top[base_x+1])

        let v7 = vsubq_u8(v26, v16); // sub v7.16b, v26.16b, v16.16b  (64 - frac_x)

        let v21 = vaddq_u8(v21, v31); // add v21.16b, v21.16b, v31.16b  (actual base_x)

        let v6h = vmull_u8(v18, v28); // umull v6.8h, v18.8b, v28.8b  (left[base_y]*(64-frac_y))
        let v6h = vmlal_u8(v6h, v19, v27); // umlal v6.8h, v19.8b, v27.8b  (+ left[base_y+1]*frac_y)
        let v17h = vmull_u8(v19, v28); // umull v17.8h, v19.8b, v28.8b
        let v17h = vmlal_u8(v17h, v20, v27); // umlal v17.8h, v20.8b, v27.8b

        let v22h = vmull_u8(vget_low_u8(v4), vget_low_u8(v7)); // umull v22.8h, v4.8b, v7.8b  (top[base_x]*(64-frac_x))
        let v22h = vmlal_u8(v22h, vget_low_u8(v5), vget_low_u8(v16)); // umlal v22.8h, v5.8b, v16.8b  (+ top[base_x+1]*frac_x)
        let v23h = vmull_high_u8(v4, v7); // umull2 v23.8h, v4.16b, v7.16b
        let v23h = vmlal_high_u8(v23h, v5, v16); // umlal2 v23.8h, v5.16b, v16.16b

        let v21 = vcgezq_s8(vreinterpretq_s8_u8(v21)); // cmge v21.16b, v21.16b, #0

        let v6 = vrshrn_n_u16::<6>(v6h); // rshrn v6.8b, v6.8h, #6
        let v6 = vrshrn_high_n_u16::<6>(v6, v17h); // rshrn2 v6.16b, v17.8h, #6
        let v22 = vrshrn_n_u16::<6>(v22h); // rshrn v22.8b, v22.8h, #6
        let v22 = vrshrn_high_n_u16::<6>(v22, v23h); // rshrn2 v22.16b, v23.8h, #6

        let v6 = vbslq_u8(v21, v22, v6); // bit v6.16b, v22.16b, v21.16b

        vst1_u8(x0, vget_low_u8(v6)); // st1 {v6.d}[0], [x0], x1
        x0 = x0.wrapping_offset(x1);
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        w5 -= 2; // subs w5, w5, #2
        vst1_u8(x0, vget_high_u8(v6)); // st1 {v6.d}[1], [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = v20; // mov v18.8b, v20.8b
        v29 = vadd_u8(v29, v24); // add v29.8b, v29.8b, v24.8b  (base_y += 2)
        v30 = vadd_u8(v30, v24); // add v30.8b, v30.8b, v24.8b  (base_y += 2)
    }

    loop {
        // 89:
        let v19 = vqtbl3_u8(v0v1v2, v29); // tbl v19.8b, {v0.16b, v1.16b, v2.16b}, v29.8b  (left[base_y+1])
        let v20 = vqtbl3_u8(v0v1v2, v30); // tbl v20.8b, {v0.16b, v1.16b, v2.16b}, v30.8b  (left[base_y+2])

        let v6h = vmull_u8(v18, v28); // umull v6.8h, v18.8b, v28.8b  (left[base_y]*(64-frac_y))
        let v6h = vmlal_u8(v6h, v19, v27); // umlal v6.8h, v19.8b, v27.8b  (+ left[base_y+1]*frac_y)
        let v17h = vmull_u8(v19, v28); // umull v17.8h, v19.8b, v28.8b
        let v17h = vmlal_u8(v17h, v20, v27); // umlal v17.8h, v20.8b, v27.8b

        let v6 = vrshrn_n_u16::<6>(v6h); // rshrn v6.8b, v6.8h, #6
        let v6 = vrshrn_high_n_u16::<6>(v6, v17h); // rshrn2 v6.16b, v17.8h, #6

        vst1_u8(x0, vget_low_u8(v6)); // st1 {v6.d}[0], [x0], x1
        x0 = x0.wrapping_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        vst1_u8(x0, vget_high_u8(v6)); // st1 {v6.d}[1], [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = v20; // mov v18.8b, v20.8b
        v29 = vadd_u8(v29, v24); // add v29.8b, v29.8b, v24.8b  (base_y += 2)
        v30 = vadd_u8(v30, v24); // add v30.8b, v30.8b, v24.8b  (base_y += 2)
    }
}

/// `ipred_z2_fill1_8bpc_neon`, label `160:` (width 16). The saves of
/// `d8`-`d15` are the compiler's.
///
/// # Safety
/// As for `dav1d_ipred_z2_fill1_8bpc_neon` with `width == 16`; `w8` is the
/// entry's `xpos`, `w7` is `-dy`, `v31` the first half of `increments`.
unsafe fn ipred_z2_fill1_w16(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    x3: *const u8,
    mut w8: c_int,
    mut w5: c_int,
    w6: c_int,
    w7: c_int,
    v31: uint16x8_t,
) {
    let v18h = vdupq_n_u16(w7 as u16); // dup v18.8h, w7  (-dy)
    let v17 = vdupq_n_u8(1); // movi v17.16b, #1
    let x3 = x3.add(1); // add x3, x3, #1  (Skip past left[0])

    let v14 = vld1q_u16(INCREMENTS.as_ptr().add(8)); // ldr q14, [x11, #16]  ({8,9,10,11,12,13,14,15})

    let v16h = vmulq_u16(v31, v18h); // mul v16.8h, v31.8h, v18.8h  ({0,1,2,3,4,5,6,7} * -dy)
    let v19h = vmulq_u16(v14, v18h); // mul v19.8h, v14.8h, v18.8h  ({8,9,10,11,12,13,14,15} * -dy)
    let v25 = vdupq_n_u8(0x3e); // movi v25.16b, #0x3e
    let v16h = vaddq_u16(v16h, v18h); // add v16.8h, v16.8h, v18.8h  (-= dy)
    let v18h = vaddq_u16(v19h, v18h); // add v18.8h, v19.8h, v18.8h

    let v31 = vmovn_u16(v31); // xtn v31.8b, v31.8h  ({0,1,2,3,4,5,6,7})
    let v31 = vmovn_high_u16(v31, v14); // xtn2 v31.16b, v14.8h  ({8,9,10,11,12,13,14,15})

    // Worst case height is 64.
    let v0v1v2v3 = vld1q_u8_x4(x3); // ld1 {v0.16b, v1.16b, v2.16b, v3.16b}, [x3]  (left[])
    let v15 = vld1q_dup_u8(x2); // ld1r {v15.16b}, [x2]  (left[0] == top[0])

    let v26 = vdupq_n_u8(64); // movi v26.16b, #64
    let v19 = vdupq_n_u8(2); // movi v19.16b, #2

    let v27 = vmovn_u16(v16h); // xtn v27.8b, v16.8h  ((uint8_t)ypos)
    let v27 = vmovn_high_u16(v27, v18h); // xtn2 v27.16b, v18.8h
    let v29 = vshrn_n_u16::<6>(v16h); // shrn v29.8b, v16.8h, #6  (ypos >> 6)
    let v29 = vshrn_high_n_u16::<6>(v29, v18h); // shrn2 v29.16b, v18.8h, #6
    let v18 = v15; // mov v18.16b, v15.16b  (left[0])
    let v27 = vandq_u8(v27, v25); // and v27.16b, v27.16b, v25.16b  (frac_y)

    // Cut corners here; for the first row we don't expect to need to
    // read outside of v0.
    let mut v18 = vqtbx1q_u8(v18, v0v1v2v3.0, v29); // tbx v18.16b, {v0.16b}, v29.16b  (left[base_y])

    let mut v30 = vaddq_u8(v29, v19); // add v30.16b, v29.16b, v19.16b  (base_y + 2)
    let mut v29 = vaddq_u8(v29, v17); // add v29.16b, v29.16b, v17.16b  (base_y + 1)

    let v28 = vsubq_u8(v26, v27); // sub v28.16b, v26.16b, v27.16b  (64 - frac_y)

    let v24 = vdupq_n_u8(2); // movi v24.16b, #2
    loop {
        // 16:
        let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
        let v16h = vdupq_n_u16(w8 as u16); // dup v16.8h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
        if w9 <= -16 {
            break; // cmp w9, #-16; b.le 169f  (base_x <= -16)
        }

        let v17h = vdupq_n_u16(w8 as u16); // dup v17.8h, w8  (xpos)

        let x9 = x2.wrapping_offset(w9 as isize); // add x9, x2, w9, sxtw
        let x11 = x2.wrapping_offset(w11 as isize); // add x11, x2, w11, sxtw

        let v4v5 = vld1q_u8_x2(x9); // ld1 {v4.16b, v5.16b}, [x9]  (top[base_x])
        let v19 = v15; // mov v19.16b, v15.16b  (left[0])
        let v6v7 = vld1q_u8_x2(x11); // ld1 {v6.16b, v7.16b}, [x11]
        let (v4, v6) = (v4v5.0, v6v7.0);

        let v19 = vqtbx4q_u8(v19, v0v1v2v3, v29); // tbx v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y+1])

        let v20 = v15; // mov v20.16b, v15.16b  (left[0])

        let v21 = vshrn_n_u16::<6>(v16h); // shrn v21.8b, v16.8h, #6  (first base_x)
        let v22 = vshrn_n_u16::<6>(v17h); // shrn v22.8b, v17.8h, #6
        let v16 = vmovn_u16(v16h); // xtn v16.8b, v16.8h  ((uint8_t)xpos)
        let v17 = vmovn_u16(v17h); // xtn v17.8b, v17.8h

        let v20 = vqtbx4q_u8(v20, v0v1v2v3, v30); // tbx v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v30.16b  (left[base_y+2])

        let v21 = vcombine_u8(v21, v21); // trn1 v21.2d, v21.2d, v21.2d  (first base_x)
        let v22 = vcombine_u8(v22, v22); // trn1 v22.2d, v22.2d, v22.2d
        let v16 = vcombine_u8(v16, v16); // trn1 v16.2d, v16.2d, v16.2d  ((uint8_t)xpos)
        let v17 = vcombine_u8(v17, v17); // trn1 v17.2d, v17.2d, v17.2d

        let v5 = vextq_u8::<1>(v4, v4v5.1); // ext v5.16b, v4.16b, v5.16b, #1  (top[base_x+1])
        let v7 = vextq_u8::<1>(v6, v6v7.1); // ext v7.16b, v6.16b, v7.16b, #1

        let v16 = vandq_u8(v16, v25); // and v16.16b, v16.16b, v25.16b  (frac_x)
        let v17 = vandq_u8(v17, v25); // and v17.16b, v17.16b, v25.16b

        let v10h = vmull_u8(vget_low_u8(v18), vget_low_u8(v28)); // umull v10.8h, v18.8b, v28.8b  (left[base_y]*(64-frac_y))
        let v10h = vmlal_u8(v10h, vget_low_u8(v19), vget_low_u8(v27)); // umlal v10.8h, v19.8b, v27.8b  (+ left[base_y+1]*frac_y)

        let v8 = vsubq_u8(v26, v16); // sub v8.16b, v26.16b, v16.16b  (64 - frac_x)
        let v9 = vsubq_u8(v26, v17); // sub v9.16b, v26.16b, v17.16b

        let v11h = vmull_high_u8(v18, v28); // umull2 v11.8h, v18.16b, v28.16b
        let v11h = vmlal_high_u8(v11h, v19, v27); // umlal2 v11.8h, v19.16b, v27.16b

        let v21 = vaddq_u8(v21, v31); // add v21.16b, v21.16b, v31.16b  (actual base_x)
        let v22 = vaddq_u8(v22, v31); // add v22.16b, v22.16b, v31.16b

        let v12h = vmull_u8(vget_low_u8(v19), vget_low_u8(v28)); // umull v12.8h, v19.8b, v28.8b
        let v12h = vmlal_u8(v12h, vget_low_u8(v20), vget_low_u8(v27)); // umlal v12.8h, v20.8b, v27.8b
        let v13h = vmull_high_u8(v19, v28); // umull2 v13.8h, v19.16b, v28.16b
        let v13h = vmlal_high_u8(v13h, v20, v27); // umlal2 v13.8h, v20.16b, v27.16b

        let v10 = vrshrn_n_u16::<6>(v10h); // rshrn v10.8b, v10.8h, #6
        let v10 = vrshrn_high_n_u16::<6>(v10, v11h); // rshrn2 v10.16b, v11.8h, #6
        let v11 = vrshrn_n_u16::<6>(v12h); // rshrn v11.8b, v12.8h, #6
        let v11 = vrshrn_high_n_u16::<6>(v11, v13h); // rshrn2 v11.16b, v13.8h, #6

        let v12h = vmull_u8(vget_low_u8(v4), vget_low_u8(v8)); // umull v12.8h, v4.8b, v8.8b  (top[base_x]*(64-frac_x))
        let v12h = vmlal_u8(v12h, vget_low_u8(v5), vget_low_u8(v16)); // umlal v12.8h, v5.8b, v16.8b  (+ top[base_x+1]*frac_x)
        let v13h = vmull_high_u8(v4, v8); // umull2 v13.8h, v4.16b, v8.16b
        let v13h = vmlal_high_u8(v13h, v5, v16); // umlal2 v13.8h, v5.16b, v16.16b
        let v14h = vmull_u8(vget_low_u8(v6), vget_low_u8(v9)); // umull v14.8h, v6.8b, v9.8b
        let v14h = vmlal_u8(v14h, vget_low_u8(v7), vget_low_u8(v17)); // umlal v14.8h, v7.8b, v17.8b
        let v18h = vmull_high_u8(v6, v9); // umull2 v18.8h, v6.16b, v9.16b
        let v18h = vmlal_high_u8(v18h, v7, v17); // umlal2 v18.8h, v7.16b, v17.16b

        let v21 = vcgezq_s8(vreinterpretq_s8_u8(v21)); // cmge v21.16b, v21.16b, #0
        let v22 = vcgezq_s8(vreinterpretq_s8_u8(v22)); // cmge v22.16b, v22.16b, #0

        let v12 = vrshrn_n_u16::<6>(v12h); // rshrn v12.8b, v12.8h, #6
        let v12 = vrshrn_high_n_u16::<6>(v12, v13h); // rshrn2 v12.16b, v13.8h, #6
        let v13 = vrshrn_n_u16::<6>(v14h); // rshrn v13.8b, v14.8h, #6
        let v13 = vrshrn_high_n_u16::<6>(v13, v18h); // rshrn2 v13.16b, v18.8h, #6

        let v10 = vbslq_u8(v21, v12, v10); // bit v10.16b, v12.16b, v21.16b
        let v11 = vbslq_u8(v22, v13, v11); // bit v11.16b, v13.16b, v22.16b

        vst1q_u8(x0, v10); // st1 {v10.16b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        vst1q_u8(x0, v11); // st1 {v11.16b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = v20; // mov v18.16b, v20.16b
        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 2)
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b  (base_y += 2)
    }

    loop {
        // 169:
        let v19 = v15; // mov v19.16b, v15.16b
        let v20 = v15; // mov v20.16b, v15.16b
        let v19 = vqtbx4q_u8(v19, v0v1v2v3, v29); // tbx v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y+1])
        let v20 = vqtbx4q_u8(v20, v0v1v2v3, v30); // tbx v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v30.16b  (left[base_y+2])

        let v4h = vmull_u8(vget_low_u8(v18), vget_low_u8(v28)); // umull v4.8h, v18.8b, v28.8b  (left[base_y]*(64-frac_y))
        let v4h = vmlal_u8(v4h, vget_low_u8(v19), vget_low_u8(v27)); // umlal v4.8h, v19.8b, v27.8b  (+ left[base_y+1]*frac_y)
        let v5h = vmull_high_u8(v18, v28); // umull2 v5.8h, v18.16b, v28.16b
        let v5h = vmlal_high_u8(v5h, v19, v27); // umlal2 v5.8h, v19.16b, v27.16b
        let v6h = vmull_u8(vget_low_u8(v19), vget_low_u8(v28)); // umull v6.8h, v19.8b, v28.8b
        let v6h = vmlal_u8(v6h, vget_low_u8(v20), vget_low_u8(v27)); // umlal v6.8h, v20.8b, v27.8b
        let v7h = vmull_high_u8(v19, v28); // umull2 v7.8h, v19.16b, v28.16b
        let v7h = vmlal_high_u8(v7h, v20, v27); // umlal2 v7.8h, v20.16b, v27.16b

        let v4 = vrshrn_n_u16::<6>(v4h); // rshrn v4.8b, v4.8h, #6
        let v4 = vrshrn_high_n_u16::<6>(v4, v5h); // rshrn2 v4.16b, v5.8h, #6
        let v5 = vrshrn_n_u16::<6>(v6h); // rshrn v5.8b, v6.8h, #6
        let v5 = vrshrn_high_n_u16::<6>(v5, v7h); // rshrn2 v5.16b, v7.8h, #6

        vst1q_u8(x0, v4); // st1 {v4.16b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        vst1q_u8(x0, v5); // st1 {v5.16b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = v20; // mov v18.16b, v20.16b
        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 2)
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b  (base_y += 2)
    }
}

/// `ipred_z2_fill1_8bpc_neon`, labels `320:`/`640:` (widths 32 and 64).
/// The saves of `d8`-`d15` are the compiler's.
///
/// # Safety
/// As for `dav1d_ipred_z2_fill1_8bpc_neon` with `width` 32 or 64; `w7` is
/// `-dy`, `v31` the first half of `increments`.
unsafe fn ipred_z2_fill1_w32(
    mut x0: *mut u8,
    mut x1: isize,
    x2: *const u8,
    x3: *const u8,
    mut w4: c_int,
    mut w5: c_int,
    w6: c_int,
    w7: c_int,
    v31: uint16x8_t,
) {
    let mut w8 = (1 << 6) - w6; // mov w8, #(1 << 6); sub w8, w8, w6  (xpos = (1 << 6) - dx)

    let v25h = vdupq_n_u16(w7 as u16); // dup v25.8h, w7  (-dy)
    let x3 = x3.add(1); // add x3, x3, #1  (Skip past left[0])

    let v14 = vld1q_u16(INCREMENTS.as_ptr().add(8)); // ldr q14, [x11, #16]  ({8,9,10,11,12,13,14,15})

    let mut x13 = x0.wrapping_offset(x1); // add x13, x0, x1  (alternating row)
    x1 <<= 1; // lsl x1, x1, #1  (stride *= 2)
    x1 = x1.wrapping_sub(w4 as u32 as isize); // sub x1, x1, w4, uxtw  (stride -= width)

    let v11h = vdupq_n_u16(8); // movi v11.8h, #8
    let v26h = vmulq_u16(v31, v25h); // mul v26.8h, v31.8h, v25.8h  ({0,1,2,3,4,5,6,7} * -dy)
    let mut v26h = vaddq_u16(v26h, v25h); // add v26.8h, v26.8h, v25.8h  (-= dy)
    let v25h = vmulq_u16(v25h, v11h); // mul v25.8h, v25.8h, v11.8h  (-8*dy)

    let v31 = vmovn_u16(v31); // xtn v31.8b, v31.8h  ({0,1,2,3,4,5,6,7})
    let v31 = vmovn_high_u16(v31, v14); // xtn2 v31.16b, v14.8h  ({8,9,10,11,12,13,14,15})

    // Worst case height is 64.
    let v0v1v2v3 = vld1q_u8_x4(x3); // ld1 {v0.16b, v1.16b, v2.16b, v3.16b}, [x3]  (left[])
    let v15 = vld1q_dup_u8(x2); // ld1r {v15.16b}, [x2]  (left[0] == top[0])

    let mut w12 = w4; // mov w12, w4  (orig w)
    let w14 = -w4; // neg w14, w4  (-w)

    let mut v23h;
    loop {
        // 1:
        v23h = v26h; // mov v23.16b, v26.16b  (reset ypos)

        let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
        let v16h = vdupq_n_u16(w8 as u16); // dup v16.8h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
        if w9 <= w14 {
            break; // cmp w9, w14; b.le 329f  (base_x <= -w)
        }

        let v17h = vdupq_n_u16(w8 as u16); // dup v17.8h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)

        let mut x9 = x2.wrapping_offset(w9 as isize); // add x9, x2, w9, sxtw
        let mut x11 = x2.wrapping_offset(w11 as isize); // add x11, x2, w11, sxtw

        let v21 = vqshrn_n_s16::<6>(vreinterpretq_s16_u16(v16h)); // sqshrn v21.8b, v16.8h, #6  (first base_x)
        let v22 = vqshrn_n_s16::<6>(vreinterpretq_s16_u16(v17h)); // sqshrn v22.8b, v17.8h, #6
        let v16 = vmovn_u16(v16h); // xtn v16.8b, v16.8h  ((uint8_t)xpos)
        let v17 = vmovn_u16(v17h); // xtn v17.8b, v17.8h

        let mut v4 = vld1q_u8(x9); // ld1 {v4.16b}, [x9], #16  (top[base_x])
        x9 = x9.wrapping_add(16);
        let mut v6 = vld1q_u8(x11); // ld1 {v6.16b}, [x11], #16
        x11 = x11.wrapping_add(16);

        let v21 = vcombine_s8(v21, v21); // trn1 v21.2d, v21.2d, v21.2d  (first base_x)
        let v22 = vcombine_s8(v22, v22); // trn1 v22.2d, v22.2d, v22.2d
        let v16 = vcombine_u8(v16, v16); // trn1 v16.2d, v16.2d, v16.2d  ((uint8_t)xpos)
        let v17 = vcombine_u8(v17, v17); // trn1 v17.2d, v17.2d, v17.2d

        let v10 = vdupq_n_u8(0x3e); // movi v10.16b, #0x3e
        let v11 = vdupq_n_u8(64); // movi v11.16b, #64

        let v16 = vandq_u8(v16, v10); // and v16.16b, v16.16b, v10.16b  (frac_x)
        let v17 = vandq_u8(v17, v10); // and v17.16b, v17.16b, v10.16b

        let v8 = vsubq_u8(v11, v16); // sub v8.16b, v11.16b, v16.16b  (64 - frac_x)
        let v9 = vsubq_u8(v11, v17); // sub v9.16b, v11.16b, v17.16b

        let mut v21 = vaddq_s8(v21, vreinterpretq_s8_u8(v31)); // add v21.16b, v21.16b, v31.16b  (actual base_x)
        let mut v22 = vaddq_s8(v22, vreinterpretq_s8_u8(v31)); // add v22.16b, v22.16b, v31.16b

        'l2: loop {
            // 2:
            // (movi v20.16b, #2 has no counterpart: v20 is written before it is read)
            let v13h = vaddq_u16(v23h, v25h); // add v13.8h, v23.8h, v25.8h  (ypos -= 8*dy)
            let v12 = vdupq_n_u8(64); // movi v12.16b, #64
            let v10 = vdupq_n_u8(0x3e); // movi v10.16b, #0x3e

            let w10 = vgetq_lane_s8::<0>(v22) as c_int; // smov w10, v22.b[0]

            let v27 = vmovn_u16(v23h); // xtn v27.8b, v23.8h  ((uint8_t)ypos)
            let v27 = vmovn_high_u16(v27, v13h); // xtn2 v27.16b, v13.8h
            let v29 = vshrn_n_u16::<6>(v23h); // shrn v29.8b, v23.8h, #6  (ypos >> 6)
            let v29 = vshrn_high_n_u16::<6>(v29, v13h); // shrn2 v29.16b, v13.8h, #6
            let v27 = vandq_u8(v27, v10); // and v27.16b, v27.16b, v10.16b  (frac_y)

            let v18 = v15; // mov v18.16b, v15.16b  (left[0])

            if w10 >= 0 {
                // cmp w10, #0; b.ge 4f  (base_x (bottom left) >= 0)
                loop {
                    // 4: The rest of the row only predicted from top[]
                    let v5 = vld1q_u8(x9); // ld1 {v5.16b}, [x9], #16  (top[base_x])
                    x9 = x9.wrapping_add(16);
                    let v7 = vld1q_u8(x11); // ld1 {v7.16b}, [x11], #16
                    x11 = x11.wrapping_add(16);

                    let v18 = vextq_u8::<1>(v4, v5); // ext v18.16b, v4.16b, v5.16b, #1  (top[base_x+1])
                    let v19 = vextq_u8::<1>(v6, v7); // ext v19.16b, v6.16b, v7.16b, #1

                    let v12h = vmull_u8(vget_low_u8(v4), vget_low_u8(v8)); // umull v12.8h, v4.8b, v8.8b  (top[base_x]*(64-frac_x))
                    let v12h = vmlal_u8(v12h, vget_low_u8(v18), vget_low_u8(v16)); // umlal v12.8h, v18.8b, v16.8b  (+ top[base_x+1]*frac_x)
                    let v13h = vmull_high_u8(v4, v8); // umull2 v13.8h, v4.16b, v8.16b
                    let v13h = vmlal_high_u8(v13h, v18, v16); // umlal2 v13.8h, v18.16b, v16.16b
                    let v14h = vmull_u8(vget_low_u8(v6), vget_low_u8(v9)); // umull v14.8h, v6.8b, v9.8b
                    let v14h = vmlal_u8(v14h, vget_low_u8(v19), vget_low_u8(v17)); // umlal v14.8h, v19.8b, v17.8b
                    let v20h = vmull_high_u8(v6, v9); // umull2 v20.8h, v6.16b, v9.16b
                    let v20h = vmlal_high_u8(v20h, v19, v17); // umlal2 v20.8h, v19.16b, v17.16b

                    let v12 = vrshrn_n_u16::<6>(v12h); // rshrn v12.8b, v12.8h, #6
                    let v12 = vrshrn_high_n_u16::<6>(v12, v13h); // rshrn2 v12.16b, v13.8h, #6
                    let v13 = vrshrn_n_u16::<6>(v14h); // rshrn v13.8b, v14.8h, #6
                    let v13 = vrshrn_high_n_u16::<6>(v13, v20h); // rshrn2 v13.16b, v20.8h, #6

                    vst1q_u8(x0, v12); // st1 {v12.16b}, [x0], #16
                    x0 = x0.wrapping_add(16);
                    w4 -= 16; // subs w4, w4, #16
                    vst1q_u8(x13, v13); // st1 {v13.16b}, [x13], #16
                    x13 = x13.wrapping_add(16);
                    if w4 <= 0 {
                        break 'l2; // b.le 3b
                    }

                    v4 = v5; // mov v4.16b, v5.16b
                    v6 = v7; // mov v6.16b, v7.16b
                }
            }

            v23h = vaddq_u16(v13h, v25h); // add v23.8h, v13.8h, v25.8h  (ypos -= 8*dy)
            let v13 = vdupq_n_u8(1); // movi v13.16b, #1

            let v18 = vqtbx4q_u8(v18, v0v1v2v3, v29); // tbx v18.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y])
            let v29 = vaddq_u8(v29, v13); // add v29.16b, v29.16b, v13.16b  (base_y + 1)
            let v19 = v15; // mov v19.16b, v15.16b  (left[0])

            let v28 = vsubq_u8(v12, v27); // sub v28.16b, v12.16b, v27.16b  (64 - frac_y)

            let v5 = vld1q_u8(x9); // ld1 {v5.16b}, [x9], #16  (top[base_x])
            x9 = x9.wrapping_add(16);
            let v7 = vld1q_u8(x11); // ld1 {v7.16b}, [x11], #16
            x11 = x11.wrapping_add(16);

            let v19 = vqtbx4q_u8(v19, v0v1v2v3, v29); // tbx v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y+1])
            let v29 = vaddq_u8(v29, v13); // add v29.16b, v29.16b, v13.16b  (base_y + 2)

            let v20 = v15; // mov v20.16b, v15.16b  (left[0])
            let v20 = vqtbx4q_u8(v20, v0v1v2v3, v29); // tbx v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y+2])

            let v10h = vmull_u8(vget_low_u8(v18), vget_low_u8(v28)); // umull v10.8h, v18.8b, v28.8b  (left[base_y]*(64-frac_y))
            let v10h = vmlal_u8(v10h, vget_low_u8(v19), vget_low_u8(v27)); // umlal v10.8h, v19.8b, v27.8b  (+ left[base_y+1]*frac_y)
            let v11h = vmull_high_u8(v18, v28); // umull2 v11.8h, v18.16b, v28.16b
            let v11h = vmlal_high_u8(v11h, v19, v27); // umlal2 v11.8h, v19.16b, v27.16b
            let v12h = vmull_u8(vget_low_u8(v19), vget_low_u8(v28)); // umull v12.8h, v19.8b, v28.8b
            let v12h = vmlal_u8(v12h, vget_low_u8(v20), vget_low_u8(v27)); // umlal v12.8h, v20.8b, v27.8b
            let v13h = vmull_high_u8(v19, v28); // umull2 v13.8h, v19.16b, v28.16b
            let v13h = vmlal_high_u8(v13h, v20, v27); // umlal2 v13.8h, v20.16b, v27.16b

            let v18 = vextq_u8::<1>(v4, v5); // ext v18.16b, v4.16b, v5.16b, #1  (top[base_x+1])
            let v19 = vextq_u8::<1>(v6, v7); // ext v19.16b, v6.16b, v7.16b, #1

            let v10 = vrshrn_n_u16::<6>(v10h); // rshrn v10.8b, v10.8h, #6
            let v10 = vrshrn_high_n_u16::<6>(v10, v11h); // rshrn2 v10.16b, v11.8h, #6
            let v11 = vrshrn_n_u16::<6>(v12h); // rshrn v11.8b, v12.8h, #6
            let v11 = vrshrn_high_n_u16::<6>(v11, v13h); // rshrn2 v11.16b, v13.8h, #6

            let v12h = vmull_u8(vget_low_u8(v4), vget_low_u8(v8)); // umull v12.8h, v4.8b, v8.8b  (top[base_x]*(64-frac_x))
            let v12h = vmlal_u8(v12h, vget_low_u8(v18), vget_low_u8(v16)); // umlal v12.8h, v18.8b, v16.8b  (+ top[base_x+1]*frac_x)
            let v13h = vmull_high_u8(v4, v8); // umull2 v13.8h, v4.16b, v8.16b
            let v13h = vmlal_high_u8(v13h, v18, v16); // umlal2 v13.8h, v18.16b, v16.16b
            let v14h = vmull_u8(vget_low_u8(v6), vget_low_u8(v9)); // umull v14.8h, v6.8b, v9.8b
            let v14h = vmlal_u8(v14h, vget_low_u8(v19), vget_low_u8(v17)); // umlal v14.8h, v19.8b, v17.8b
            let v20h = vmull_high_u8(v6, v9); // umull2 v20.8h, v6.16b, v9.16b
            let v20h = vmlal_high_u8(v20h, v19, v17); // umlal2 v20.8h, v19.16b, v17.16b

            let v18 = vcgezq_s8(v21); // cmge v18.16b, v21.16b, #0
            let v19 = vcgezq_s8(v22); // cmge v19.16b, v22.16b, #0

            let v12 = vrshrn_n_u16::<6>(v12h); // rshrn v12.8b, v12.8h, #6
            let v12 = vrshrn_high_n_u16::<6>(v12, v13h); // rshrn2 v12.16b, v13.8h, #6
            let v13 = vrshrn_n_u16::<6>(v14h); // rshrn v13.8b, v14.8h, #6
            let v13 = vrshrn_high_n_u16::<6>(v13, v20h); // rshrn2 v13.16b, v20.8h, #6

            let v10 = vbslq_u8(v18, v12, v10); // bit v10.16b, v12.16b, v18.16b
            let v11 = vbslq_u8(v19, v13, v11); // bit v11.16b, v13.16b, v19.16b

            vst1q_u8(x0, v10); // st1 {v10.16b}, [x0], #16
            x0 = x0.wrapping_add(16);
            w4 -= 16; // subs w4, w4, #16
            vst1q_u8(x13, v11); // st1 {v11.16b}, [x13], #16
            x13 = x13.wrapping_add(16);
            if w4 <= 0 {
                break; // b.le 3f
            }

            let v10 = vdupq_n_s8(16); // movi v10.16b, #16
            v4 = v5; // mov v4.16b, v5.16b
            v6 = v7; // mov v6.16b, v7.16b
            v21 = vaddq_s8(v21, v10); // add v21.16b, v21.16b, v10.16b  (base_x += 16)
            v22 = vaddq_s8(v22, v10); // add v22.16b, v22.16b, v10.16b
        }

        // 3:
        w5 -= 2; // subs w5, w5, #2
        if w5 <= 0 {
            return; // b.le 9f
        }
        let v10h = vdupq_n_u16(128); // movi v10.8h, #128
        x0 = x0.wrapping_offset(x1); // add x0, x0, x1
        x13 = x13.wrapping_offset(x1); // add x13, x13, x1
        w4 = w12; // mov w4, w12  (reset w)
        v26h = vaddq_u16(v26h, v10h); // add v26.8h, v26.8h, v10.8h  (ypos += 2*(1<<6))
    }

    // 329: The rest of the block only predicted from left[]
    x1 = x1.wrapping_add(w4 as u32 as isize); // add x1, x1, w4, uxtw  (restore stride)
    w12 = w5; // mov w12, w5  (orig remaining h)
    loop {
        // 1:
        let v13h = vaddq_u16(v23h, v25h); // add v13.8h, v23.8h, v25.8h  (ypos -= 8*dy)
        let v12 = vdupq_n_u8(64); // movi v12.16b, #64
        let v10 = vdupq_n_u8(0x3e); // movi v10.16b, #0x3e

        let v27 = vmovn_u16(v23h); // xtn v27.8b, v23.8h  ((uint8_t)ypos)
        let v27 = vmovn_high_u16(v27, v13h); // xtn2 v27.16b, v13.8h
        let v29 = vshrn_n_u16::<6>(v23h); // shrn v29.8b, v23.8h, #6  (ypos >> 6)
        let v29 = vshrn_high_n_u16::<6>(v29, v13h); // shrn2 v29.16b, v13.8h, #6
        let v27 = vandq_u8(v27, v10); // and v27.16b, v27.16b, v10.16b  (frac_y)

        let v18 = v15; // mov v18.16b, v15.16b  (left[0])
        v23h = vaddq_u16(v13h, v25h); // add v23.8h, v13.8h, v25.8h  (ypos -= 8*dy)
        let v21 = vdupq_n_u8(1); // movi v21.16b, #1

        let mut v18 = vqtbx4q_u8(v18, v0v1v2v3, v29); // tbx v18.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y])
        let mut v29 = vaddq_u8(v29, v21); // add v29.16b, v29.16b, v21.16b  (base_y + 1)

        let v28 = vsubq_u8(v12, v27); // sub v28.16b, v12.16b, v27.16b  (64 - frac_y)
        loop {
            // 2:
            let v19 = v15; // mov v19.16b, v15.16b  (left[0])
            let v19 = vqtbx4q_u8(v19, v0v1v2v3, v29); // tbx v19.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y+1])
            v29 = vaddq_u8(v29, v21); // add v29.16b, v29.16b, v21.16b  (base_y + 2)
            let v20 = v15; // mov v20.16b, v15.16b  (left[0])
            let v20 = vqtbx4q_u8(v20, v0v1v2v3, v29); // tbx v20.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v29.16b  (left[base_y+2])
            v29 = vaddq_u8(v29, v21); // add v29.16b, v29.16b, v21.16b  (next base_y)

            let v10h = vmull_u8(vget_low_u8(v18), vget_low_u8(v28)); // umull v10.8h, v18.8b, v28.8b  (left[base_y]*(64-frac_y))
            let v10h = vmlal_u8(v10h, vget_low_u8(v19), vget_low_u8(v27)); // umlal v10.8h, v19.8b, v27.8b  (+ left[base_y+1]*frac_y)
            let v11h = vmull_high_u8(v18, v28); // umull2 v11.8h, v18.16b, v28.16b
            let v11h = vmlal_high_u8(v11h, v19, v27); // umlal2 v11.8h, v19.16b, v27.16b
            let v12h = vmull_u8(vget_low_u8(v19), vget_low_u8(v28)); // umull v12.8h, v19.8b, v28.8b
            let v12h = vmlal_u8(v12h, vget_low_u8(v20), vget_low_u8(v27)); // umlal v12.8h, v20.8b, v27.8b
            let v13h = vmull_high_u8(v19, v28); // umull2 v13.8h, v19.16b, v28.16b
            let v13h = vmlal_high_u8(v13h, v20, v27); // umlal2 v13.8h, v20.16b, v27.16b

            let v10 = vrshrn_n_u16::<6>(v10h); // rshrn v10.8b, v10.8h, #6
            let v10 = vrshrn_high_n_u16::<6>(v10, v11h); // rshrn2 v10.16b, v11.8h, #6
            let v11 = vrshrn_n_u16::<6>(v12h); // rshrn v11.8b, v12.8h, #6
            let v11 = vrshrn_high_n_u16::<6>(v11, v13h); // rshrn2 v11.16b, v13.8h, #6

            vst1q_u8(x0, v10); // st1 {v10.16b}, [x0], x1
            x0 = x0.wrapping_offset(x1);
            w5 -= 2; // subs w5, w5, #2
            vst1q_u8(x13, v11); // st1 {v11.16b}, [x13], x1
            x13 = x13.wrapping_offset(x1);
            if w5 <= 0 {
                break; // b.le 3f
            }
            v18 = v20; // mov v18.16b, v20.16b
        }

        // 3:
        w4 -= 16; // subs w4, w4, #16
        if w4 <= 0 {
            return; // b.le 9f
        }

        x1 = ((x1 as usize) >> 1) as isize; // lsr x1, x1, #1
        let x12 = w12 as u32 as isize; // (mov w12, w5 cleared the upper half of x12)
        x0 = x0.wrapping_offset(x1.wrapping_mul(x12).wrapping_neg()); // msub x0, x1, x12, x0  (ptr -= h * stride)
        x13 = x13.wrapping_offset(x1.wrapping_mul(x12).wrapping_neg()); // msub x13, x1, x12, x13
        x1 <<= 1; // lsl x1, x1, #1
        x0 = x0.wrapping_add(16); // add x0, x0, #16
        x13 = x13.wrapping_add(16); // add x13, x13, #16
        w5 = w12; // mov w5, w12  (reset h)
    }
}

/// `void ipred_z2_fill1_8bpc_neon(pixel *dst, const ptrdiff_t stride,
/// const pixel *const top, const pixel *const left, const int width,
/// const int height, const int dx, const int dy);`
///
/// The entry point computes `xpos`, loads `increments`, negates `dy` and
/// dispatches on the width (the assembly's jump table `ipred_z2_fill1_tbl`).
/// The widths 4, 8 and 16 get `xpos` in the place of the width; the code of
/// the widths 32 and 64 needs the width and computes `xpos` itself.
///
/// # Safety
/// `width` is 4, 8, 16, 32 or 64 and `height` is even and at most 64 (16
/// for width 4, 32 for width 8). Writes `dst` rows `0..height`, columns
/// `0..width` (`stride` bytes per row). Reads `left[0..32]` (width 4),
/// `left[0..48]` (width 8) or `left[1..65]` and `top[0]` (width 16 and
/// more), and for each pair of rows that is not predicted from `left`
/// alone `top[base_x..base_x + 8]` (width 4), `top[base_x..base_x + 16]`
/// (width 8), `top[base_x..base_x + 32]` (width 16) or
/// `top[base_x..base_x + width + 16]` (width 32 and 64), with `base_x =
/// ((1 << 6) - (y + 1) * dx) >> 6` of the pair's two rows `y`. A pair is
/// read from `top` while its first row's `base_x` is above `-width`; the
/// second row's is up to `dx >> 6` lower. Over the block sizes and the
/// derivatives of `dav1d_dr_intra_derivative` that is `top[-6..8]` (width
/// 4), `top[-11..16]` (8), `top[-31..32]` (16), `top[-34..48]` (32) and
/// `top[-69..80]` (64). `ipred_z2_neon` passes `top` and `left` inside one
/// zero-initialized buffer that covers all of it.
pub unsafe extern "C" fn dav1d_ipred_z2_fill1_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    top: *const u8,
    left: *const u8,
    width: c_int,
    height: c_int,
    dx: c_int,
    dy: c_int,
) {
    let w8 = (1 << 6) - dx; // mov w8, #(1 << 6); sub w8, w8, w6  (xpos = (1 << 6) - dx)
    let v31 = vld1q_u16(INCREMENTS.as_ptr()); // movrel x11, increments; ld1 {v31.8h}, [x11]
    let w7 = dy.wrapping_neg(); // neg w7, w7  (-dy)

    // clz w10, w4; sub w10, w10, #25; ldrsw x10, [x9, w10, uxtw #2]; br x9
    match width {
        4 => ipred_z2_fill1_w4(dst, stride, top, left, w8, height, dx, w7, v31), // 40:
        8 => ipred_z2_fill1_w8(dst, stride, top, left, w8, height, dx, w7, v31), // 80:
        16 => ipred_z2_fill1_w16(dst, stride, top, left, w8, height, dx, w7, v31), // 160:
        _ => ipred_z2_fill1_w32(dst, stride, top, left, width, height, dx, w7, v31), // 320:, 640:
    }
}

/// `ipred_z2_fill2_8bpc_neon` (same parameters as `ipred_z2_fill1`): the
/// variant for an upsampled `top` (`top[2 * x]`, `dx` doubled by the caller).
///
/// # Safety
/// `width` and `height` are 4 or 8 (the caller upsamples only for `width +
/// height <= 16`). Writes `dst` rows `0..height`, columns `0..width`. Reads
/// `left[0..16]`, and for each pair of rows that is not predicted from
/// `left` alone `top[base_x..base_x + 8]` (width 4) or `top[base_x..base_x
/// + 16]` (width 8) with `base_x = ((2 << 6) - (y + 1) * dx) >> 6` of the
/// pair's two rows `y`.
pub unsafe extern "C" fn dav1d_ipred_z2_fill2_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    top: *const u8,
    left: *const u8,
    width: c_int,
    height: c_int,
    dx: c_int,
    dy: c_int,
) {
    let (mut x0, x1, x2, x3, w4, mut w5, w6) = (dst, stride, top, left, width, height, dx);
    let mut w8 = (2 << 6) - w6; // mov w8, #(2 << 6); sub w8, w8, w6  (xpos = (2 << 6) - dx)

    let v31 = vld1q_u16(INCREMENTS.as_ptr()); // ldrel q31, x11, increments
    let w7 = dy.wrapping_neg(); // neg w7, w7  (-dy)
    if w4 != 8 {
        // cmp w4, #8; b.eq 80f
        // 40:
        let v30 = vdup_n_u16(w7 as u16); // dup v30.4h, w7  (-dy)
        let v17 = vdup_n_u8(1); // movi v17.8b, #1

        let v16 = vmul_u16(vget_low_u16(v31), v30); // mul v16.4h, v31.4h, v30.4h  ({0,1,2,3} * -dy)
        let v25 = vdup_n_u8(0x3e); // movi v25.16b, #0x3e
        let v30 = vadd_u16(v16, v30); // add v30.4h, v16.4h, v30.4h  (-= dy)

        let v31 = vmovn_u16(v31); // xtn v31.8b, v31.8h  ({0,1,2,3})

        // For upsample_top, w <= 8 and h <= 8; we may need up to h+1 elements
        // from left.
        let v0 = vld1q_u8(x3); // ld1 {v0.16b}, [x3]  (left[])

        let v26 = vdup_n_u8(64); // movi v26.16b, #64
        let v19 = vdup_n_u8(2); // movi v19.16b, #2

        let v30 = vcombine_u16(v30, vdup_n_u16(0)); // (the .4h results cleared the high half)
        let v27 = vmovn_u16(v30); // xtn v27.8b, v30.8h  ((uint8_t)ypos)
        let v29 = vshrn_n_u16::<6>(v30); // shrn v29.8b, v30.8h, #6  (ypos >> 6)
        let v27 = vand_u8(v27, v25); // and v27.8b, v27.8b, v25.8b  (frac_y)

        let v29 = vadd_u8(v29, v17); // add v29.8b, v29.8b, v17.8b  (base_y = (ypos >> 6) + 1)

        let v30 = vadd_u8(v29, v17); // add v30.8b, v29.8b, v17.8b  (base_y + 1)
        let v28 = vadd_u8(v29, v19); // add v28.8b, v29.8b, v19.8b  (base_y + 2)

        let mut v16 = vqtbl1_u8(v0, v29); // tbl v16.8b, {v0.16b}, v29.8b  (left[base_y])

        let mut v30 = z2_trn1_2s(v30, v28); // trn1 v30.2s, v30.2s, v28.2s  (base_y + 1, base_y + 2)

        let v28 = vsub_u8(v26, v27); // sub v28.8b, v26.8b, v27.8b  (64 - frac_y)

        let v31 = z2_trn1_2s(v31, v31); // trn1 v31.2s, v31.2s, v31.2s  ({0,1,2,3,0,1,2,3})

        let v27 = z2_trn1_2s(v27, v27); // trn1 v27.2s, v27.2s, v27.2s  (frac_y)
        let v28 = z2_trn1_2s(v28, v28); // trn1 v28.2s, v28.2s, v28.2s  (64 - frac_y)

        let v29 = vdup_n_u8(2); // movi v29.8b, #2
        let v31 = vadd_u8(v31, v31); // add v31.8b, v31.8b, v31.8b  ({0,2,4,6,0,2,4,6})
        loop {
            // 4:
            let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
            let v6 = vdup_n_u16(w8 as u16); // dup v6.4h, w8  (xpos)
            w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
            let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
            if w9 <= -8 {
                break; // cmp w9, #-8; b.le 49f  (base_x <= -8)
            }

            let v7 = vdup_n_u16(w8 as u16); // dup v7.4h, w8  (xpos)

            let v2 = vld1_u8(x2.wrapping_offset(w9 as isize)); // ldr d2, [x2, w9, sxtw]  (top[base_x])
            let v4 = vld1_u8(x2.wrapping_offset(w11 as isize)); // ldr d4, [x2, w11, sxtw]

            let v6 = vcombine_u16(v6, v7); // trn1 v6.2d, v6.2d, v7.2d  (xpos)

            let v17 = vqtbl1_u8(v0, v30); // tbl v17.8b, {v0.16b}, v30.8b  (left[base_y+1], left[base_y+2])

            let v20 = vshrn_n_u16::<6>(v6); // shrn v20.8b, v6.8h, #6  (first base_x for each row)
            let v6 = vmovn_u16(v6); // xtn v6.8b, v6.8h  ((uint8_t)xpos)

            let v3 = vuzp2_u8(v2, v4); // uzp2 v3.8b, v2.8b, v4.8b  (top[base_x+1])
            let v2 = vuzp1_u8(v2, v4); // uzp1 v2.8b, v2.8b, v4.8b  (top[base_x])

            let v6 = vand_u8(v6, v25); // and v6.8b, v6.8b, v25.8b  (frac_x)

            v16 = z2_trn1_2s(v16, v17); // trn1 v16.2s, v16.2s, v17.2s  (left[base_y], left[base_y+1])

            let v7 = vsub_u8(v26, v6); // sub v7.8b, v26.8b, v6.8b  (64 - frac_x)

            let v20 = vadd_u8(v20, v31); // add v20.8b, v20.8b, v31.8b  (actual base_x)

            let v16h = vmull_u8(v16, v28); // umull v16.8h, v16.8b, v28.8b  (left[base_y]*(64-frac_y))
            let v16h = vmlal_u8(v16h, v17, v27); // umlal v16.8h, v17.8b, v27.8b  (+ left[base_y+1]*frac_y)

            let v22h = vmull_u8(v2, v7); // umull v22.8h, v2.8b, v7.8b  (top[base_x]*(64-frac_x))
            let v22h = vmlal_u8(v22h, v3, v6); // umlal v22.8h, v3.8b, v6.8b  (+ top[base_x+1]*frac_x)

            let v20 = vcgez_s8(vreinterpret_s8_u8(v20)); // cmge v20.8b, v20.8b, #0

            v16 = vrshrn_n_u16::<6>(v16h); // rshrn v16.8b, v16.8h, #6
            let v22 = vrshrn_n_u16::<6>(v22h); // rshrn v22.8b, v22.8h, #6

            v16 = vbsl_u8(v20, v22, v16); // bit v16.8b, v22.8b, v20.8b

            z2_st1_s::<0>(x0, v16); // st1 {v16.s}[0], [x0], x1
            x0 = x0.wrapping_offset(x1);
            w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
            w5 -= 2; // subs w5, w5, #2
            z2_st1_s::<1>(x0, v16); // st1 {v16.s}[1], [x0], x1
            x0 = x0.wrapping_offset(x1);
            if w5 <= 0 {
                return; // b.le 9f
            }

            v16 = vext_u8::<4>(v17, v17); // ext v16.8b, v17.8b, v17.8b, #4
            v30 = vadd_u8(v30, v29); // add v30.8b, v30.8b, v29.8b  (base_y += 2)
        }

        loop {
            // 49:
            let v17 = vqtbl1_u8(v0, v30); // tbl v17.8b, {v0.16b}, v30.8b  (left[base_y+1], left[base_y+2])

            v16 = z2_trn1_2s(v16, v17); // trn1 v16.2s, v16.2s, v17.2s  (left[base_y], left[base_y+1])

            let v18h = vmull_u8(v16, v28); // umull v18.8h, v16.8b, v28.8b  (left[base_y]*(64-frac_y))
            let v18h = vmlal_u8(v18h, v17, v27); // umlal v18.8h, v17.8b, v27.8b  (+ left[base_y+1]*frac_y)
            let v18 = vrshrn_n_u16::<6>(v18h); // rshrn v18.8b, v18.8h, #6

            z2_st1_s::<0>(x0, v18); // st1 {v18.s}[0], [x0], x1
            x0 = x0.wrapping_offset(x1);
            w5 -= 2; // subs w5, w5, #2
            z2_st1_s::<1>(x0, v18); // st1 {v18.s}[1], [x0], x1
            x0 = x0.wrapping_offset(x1);
            if w5 <= 0 {
                return; // b.le 9f
            }

            v16 = vext_u8::<4>(v17, v17); // ext v16.8b, v17.8b, v17.8b, #4
            v30 = vadd_u8(v30, v29); // add v30.8b, v30.8b, v29.8b  (base_y += 2)
        }
    }

    // 80:
    let v30 = vdupq_n_u16(w7 as u16); // dup v30.8h, w7  (-dy)
    let v17 = vdup_n_u8(1); // movi v17.8b, #1

    let v16 = vmulq_u16(v31, v30); // mul v16.8h, v31.8h, v30.8h  ({0,1,2,3,4,5,6,7} * -dy)
    let v25 = vdupq_n_u8(0x3e); // movi v25.16b, #0x3e
    let v30 = vaddq_u16(v16, v30); // add v30.8h, v16.8h, v30.8h  (-= dy)

    let v31 = vmovn_u16(v31); // xtn v31.8b, v31.8h  ({0,1,2,3,4,5,6,7})

    // For upsample_top, w <= 8 and h <= 8; we may need up to h+1 elements
    // from left.
    let v0 = vld1q_u8(x3); // ld1 {v0.16b}, [x3]  (left[])

    let v26 = vdupq_n_u8(64); // movi v26.16b, #64
    let v19 = vdup_n_u8(2); // movi v19.16b, #2

    let v27 = vmovn_u16(v30); // xtn v27.8b, v30.8h  ((uint8_t)ypos)
    let v29 = vshrn_n_u16::<6>(v30); // shrn v29.8b, v30.8h, #6  (ypos >> 6)
    let v27 = vand_u8(v27, vget_low_u8(v25)); // and v27.8b, v27.8b, v25.8b  (frac_y)

    let v29 = vadd_u8(v29, v17); // add v29.8b, v29.8b, v17.8b  (base_y = (ypos >> 6) + 1)

    let mut v18 = vqtbl1_u8(v0, v29); // tbl v18.8b, {v0.16b}, v29.8b  (left[base_y])

    let mut v30 = vadd_u8(v29, v19); // add v30.8b, v29.8b, v19.8b  (base_y + 2)
    let mut v29 = vadd_u8(v29, v17); // add v29.8b, v29.8b, v17.8b  (base_y + 1)

    let v28 = vsub_u8(vget_low_u8(v26), v27); // sub v28.8b, v26.8b, v27.8b  (64 - frac_y)

    let v31 = vcombine_u8(v31, v31); // trn1 v31.2d, v31.2d, v31.2d  ({0,1,2,3,4,5,6,7,0,1,2,3,4,5,6,7})

    let v24 = vdup_n_u8(2); // movi v24.8b, #2
    let v31 = vaddq_u8(v31, v31); // add v31.16b, v31.16b, v31.16b  ({0,2,4,6,8,10,12,14,0,2,4,6,8,10,12,14})
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

        let v4 = vld1q_u8(x2.wrapping_offset(w9 as isize)); // ldr q4, [x2, w9, sxtw]  (top[base_x])
        let v6 = vld1q_u8(x2.wrapping_offset(w11 as isize)); // ldr q6, [x2, w11, sxtw]

        let v19 = vqtbl1_u8(v0, v29); // tbl v19.8b, {v0.16b}, v29.8b  (left[base_y+1])

        let v21 = vshrn_n_u16::<6>(v16); // shrn v21.8b, v16.8h, #6  (first base_x)
        let v21 = vshrn_high_n_u16::<6>(v21, v17); // shrn2 v21.16b, v17.8h, #6
        let v16 = vmovn_u16(v16); // xtn v16.8b, v16.8h  ((uint8_t)xpos)
        let v16 = vmovn_high_u16(v16, v17); // xtn2 v16.16b, v17.8h

        let v20 = vqtbl1_u8(v0, v30); // tbl v20.8b, {v0.16b}, v30.8b  (left[base_y+2])

        let v5 = vuzp2q_u8(v4, v6); // uzp2 v5.16b, v4.16b, v6.16b  (top[base_x+1])
        let v4 = vuzp1q_u8(v4, v6); // uzp1 v4.16b, v4.16b, v6.16b  (top[base_x])

        let v16 = vandq_u8(v16, v25); // and v16.16b, v16.16b, v25.16b  (frac_x)

        let v7 = vsubq_u8(v26, v16); // sub v7.16b, v26.16b, v16.16b  (64 - frac_x)

        let v21 = vaddq_u8(v21, v31); // add v21.16b, v21.16b, v31.16b  (actual base_x)

        let v6h = vmull_u8(v18, v28); // umull v6.8h, v18.8b, v28.8b  (left[base_y]*(64-frac_y))
        let v6h = vmlal_u8(v6h, v19, v27); // umlal v6.8h, v19.8b, v27.8b  (+ left[base_y+1]*frac_y)
        let v17h = vmull_u8(v19, v28); // umull v17.8h, v19.8b, v28.8b
        let v17h = vmlal_u8(v17h, v20, v27); // umlal v17.8h, v20.8b, v27.8b

        let v22h = vmull_u8(vget_low_u8(v4), vget_low_u8(v7)); // umull v22.8h, v4.8b, v7.8b  (top[base_x]*(64-frac_x))
        let v22h = vmlal_u8(v22h, vget_low_u8(v5), vget_low_u8(v16)); // umlal v22.8h, v5.8b, v16.8b  (+ top[base_x+1]*frac_x)
        let v23h = vmull_high_u8(v4, v7); // umull2 v23.8h, v4.16b, v7.16b
        let v23h = vmlal_high_u8(v23h, v5, v16); // umlal2 v23.8h, v5.16b, v16.16b

        let v21 = vcgezq_s8(vreinterpretq_s8_u8(v21)); // cmge v21.16b, v21.16b, #0

        let v6 = vrshrn_n_u16::<6>(v6h); // rshrn v6.8b, v6.8h, #6
        let v6 = vrshrn_high_n_u16::<6>(v6, v17h); // rshrn2 v6.16b, v17.8h, #6
        let v22 = vrshrn_n_u16::<6>(v22h); // rshrn v22.8b, v22.8h, #6
        let v22 = vrshrn_high_n_u16::<6>(v22, v23h); // rshrn2 v22.16b, v23.8h, #6

        let v6 = vbslq_u8(v21, v22, v6); // bit v6.16b, v22.16b, v21.16b

        vst1_u8(x0, vget_low_u8(v6)); // st1 {v6.d}[0], [x0], x1
        x0 = x0.wrapping_offset(x1);
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        w5 -= 2; // subs w5, w5, #2
        vst1_u8(x0, vget_high_u8(v6)); // st1 {v6.d}[1], [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = v20; // mov v18.8b, v20.8b
        v29 = vadd_u8(v29, v24); // add v29.8b, v29.8b, v24.8b  (base_y += 2)
        v30 = vadd_u8(v30, v24); // add v30.8b, v30.8b, v24.8b  (base_y += 2)
    }

    loop {
        // 89:
        let v19 = vqtbl1_u8(v0, v29); // tbl v19.8b, {v0.16b}, v29.8b  (left[base_y+1])
        let v20 = vqtbl1_u8(v0, v30); // tbl v20.8b, {v0.16b}, v30.8b  (left[base_y+2])

        let v6h = vmull_u8(v18, v28); // umull v6.8h, v18.8b, v28.8b  (left[base_y]*(64-frac_y))
        let v6h = vmlal_u8(v6h, v19, v27); // umlal v6.8h, v19.8b, v27.8b  (+ left[base_y+1]*frac_y)
        let v17h = vmull_u8(v19, v28); // umull v17.8h, v19.8b, v28.8b
        let v17h = vmlal_u8(v17h, v20, v27); // umlal v17.8h, v20.8b, v27.8b

        let v6 = vrshrn_n_u16::<6>(v6h); // rshrn v6.8b, v6.8h, #6
        let v6 = vrshrn_high_n_u16::<6>(v6, v17h); // rshrn2 v6.16b, v17.8h, #6

        vst1_u8(x0, vget_low_u8(v6)); // st1 {v6.d}[0], [x0], x1
        x0 = x0.wrapping_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        vst1_u8(x0, vget_high_u8(v6)); // st1 {v6.d}[1], [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v18 = v20; // mov v18.8b, v20.8b
        v29 = vadd_u8(v29, v24); // add v29.8b, v29.8b, v24.8b  (base_y += 2)
        v30 = vadd_u8(v30, v24); // add v30.8b, v30.8b, v24.8b  (base_y += 2)
    }
}

/// `ipred_z2_fill3_8bpc_neon` (same parameters as `ipred_z2_fill1`): the
/// variant for an upsampled `left` (`left[2 * y]`, `dy` doubled by the
/// caller).
///
/// # Safety
/// `width` and `height` are 4 or 8 (the caller upsamples only for `width +
/// height <= 16`). Writes `dst` rows `0..height`, columns `0..width`. Reads
/// `left[0..32]` (width 4) or `left[0..48]` (width 8), and for each pair of
/// rows that is not predicted from `left` alone `top[base_x..base_x + 8]`
/// (width 4) or `top[base_x..base_x + 16]` (width 8) with `base_x = ((1 <<
/// 6) - (y + 1) * dx) >> 6` of the pair's two rows `y`.
pub unsafe extern "C" fn dav1d_ipred_z2_fill3_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    top: *const u8,
    left: *const u8,
    width: c_int,
    height: c_int,
    dx: c_int,
    dy: c_int,
) {
    let (mut x0, x1, x2, x3, w4, mut w5, w6) = (dst, stride, top, left, width, height, dx);
    let mut w8 = (1 << 6) - w6; // mov w8, #(1 << 6); sub w8, w8, w6  (xpos = (1 << 6) - dx)

    let v31 = vld1q_u16(INCREMENTS.as_ptr()); // ldrel q31, x11, increments
    let w7 = dy.wrapping_neg(); // neg w7, w7  (-dy)
    if w4 != 8 {
        // cmp w4, #8; b.eq 80f
        // 40:
        let v30 = vdup_n_u16(w7 as u16); // dup v30.4h, w7  (-dy)
        let v17 = vdup_n_u8(1); // movi v17.8b, #1

        let v16 = vmul_u16(vget_low_u16(v31), v30); // mul v16.4h, v31.4h, v30.4h  ({0,1,2,3} * -dy)
        let v25 = vdup_n_u8(0x3e); // movi v25.16b, #0x3e
        let v30 = vadd_u16(v16, v30); // add v30.4h, v16.4h, v30.4h  (-= dy)

        let v31 = vmovn_u16(v31); // xtn v31.8b, v31.8h  ({0,1,2,3})

        // For upsample_left, w <= 8 and h <= 8; we may need up to 2*h+1 elements.
        let v0v1 = vld1q_u8_x2(x3); // ld1 {v0.16b, v1.16b}, [x3]  (left[])

        let v26 = vdup_n_u8(64); // movi v26.16b, #64
        let v19 = vdup_n_u8(2); // movi v19.16b, #2

        let v30 = vcombine_u16(v30, vdup_n_u16(0)); // (the .4h results cleared the high half)
        let v27 = vmovn_u16(v30); // xtn v27.8b, v30.8h  ((uint8_t)ypos)
        let v29 = vshrn_n_u16::<6>(v30); // shrn v29.8b, v30.8h, #6  (ypos >> 6)
        let v27 = vand_u8(v27, v25); // and v27.8b, v27.8b, v25.8b  (frac_y)

        let v29 = vadd_u8(v29, v19); // add v29.8b, v29.8b, v19.8b  (base_y = (ypos >> 6) + 2)

        let v30 = vadd_u8(v29, v17); // add v30.8b, v29.8b, v17.8b  (base_y + 1)
        let v28 = vadd_u8(v29, v19); // add v28.8b, v29.8b, v19.8b  (base_y + 2)

        let v31 = z2_trn1_2s(v31, v31); // trn1 v31.2s, v31.2s, v31.2s  ({0,1,2,3,0,1,2,3})

        let v24 = vadd_u8(v30, v19); // add v24.8b, v30.8b, v19.8b  (base_y + 3)

        let mut v29 = z2_trn1_2s(v29, v28); // trn1 v29.2s, v29.2s, v28.2s  (base_y + 0, base_y + 2)
        let mut v30 = z2_trn1_2s(v30, v24); // trn1 v30.2s, v30.2s, v24.2s  (base_y + 1, base_y + 3)

        let v28 = vsub_u8(v26, v27); // sub v28.8b, v26.8b, v27.8b  (64 - frac_y)

        let v27 = z2_trn1_2s(v27, v27); // trn1 v27.2s, v27.2s, v27.2s  (frac_y)
        let v28 = z2_trn1_2s(v28, v28); // trn1 v28.2s, v28.2s, v28.2s  (64 - frac_y)

        let v24 = vdup_n_u8(4); // movi v24.8b, #4
        loop {
            // 4:
            let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
            let v6 = vdup_n_u16(w8 as u16); // dup v6.4h, w8  (xpos)
            w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
            let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
            if w9 <= -4 {
                break; // cmp w9, #-4; b.le 49f  (base_x <= -4)
            }

            let v7 = vdup_n_u16(w8 as u16); // dup v7.4h, w8  (xpos)

            let v2 = vld1_u8(x2.wrapping_offset(w9 as isize)); // ldr d2, [x2, w9, sxtw]  (top[base_x])
            let v4 = vld1_u8(x2.wrapping_offset(w11 as isize)); // ldr d4, [x2, w11, sxtw]

            let v6 = vcombine_u16(v6, v7); // trn1 v6.2d, v6.2d, v7.2d  (xpos)

            let v16 = vqtbl2_u8(v0v1, v29); // tbl v16.8b, {v0.16b, v1.16b}, v29.8b  (left[base_y+0], left[base_y+2])
            let v17 = vqtbl2_u8(v0v1, v30); // tbl v17.8b, {v0.16b, v1.16b}, v30.8b  (left[base_y+1], left[base_y+3])

            let v20 = vshrn_n_u16::<6>(v6); // shrn v20.8b, v6.8h, #6  (first base_x for each row)
            let v6 = vmovn_u16(v6); // xtn v6.8b, v6.8h  ((uint8_t)xpos)

            let v3 = vext_u8::<1>(v2, v2); // ext v3.8b, v2.8b, v2.8b, #1  (top[base_x+1])
            let v5 = vext_u8::<1>(v4, v4); // ext v5.8b, v4.8b, v4.8b, #1

            let v6 = vand_u8(v6, v25); // and v6.8b, v6.8b, v25.8b  (frac_x)

            let v2 = z2_trn1_2s(v2, v4); // trn1 v2.2s, v2.2s, v4.2s  (top[base_x])
            let v3 = z2_trn1_2s(v3, v5); // trn1 v3.2s, v3.2s, v5.2s  (top[base_x+1])

            let v7 = vsub_u8(v26, v6); // sub v7.8b, v26.8b, v6.8b  (64 - frac_x)

            let v20 = vadd_u8(v20, v31); // add v20.8b, v20.8b, v31.8b  (actual base_x)

            let v16h = vmull_u8(v16, v28); // umull v16.8h, v16.8b, v28.8b  (left[base_y]*(64-frac_y))
            let v16h = vmlal_u8(v16h, v17, v27); // umlal v16.8h, v17.8b, v27.8b  (+ left[base_y+1]*frac_y)

            let v22h = vmull_u8(v2, v7); // umull v22.8h, v2.8b, v7.8b  (top[base_x]*(64-frac_x))
            let v22h = vmlal_u8(v22h, v3, v6); // umlal v22.8h, v3.8b, v6.8b  (+ top[base_x+1]*frac_x)

            let v20 = vcgez_s8(vreinterpret_s8_u8(v20)); // cmge v20.8b, v20.8b, #0

            let v16 = vrshrn_n_u16::<6>(v16h); // rshrn v16.8b, v16.8h, #6
            let v22 = vrshrn_n_u16::<6>(v22h); // rshrn v22.8b, v22.8h, #6

            let v16 = vbsl_u8(v20, v22, v16); // bit v16.8b, v22.8b, v20.8b

            z2_st1_s::<0>(x0, v16); // st1 {v16.s}[0], [x0], x1
            x0 = x0.wrapping_offset(x1);
            w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
            w5 -= 2; // subs w5, w5, #2
            z2_st1_s::<1>(x0, v16); // st1 {v16.s}[1], [x0], x1
            x0 = x0.wrapping_offset(x1);
            if w5 <= 0 {
                return; // b.le 9f
            }

            v29 = vadd_u8(v29, v24); // add v29.8b, v29.8b, v24.8b  (base_y += 4)
            v30 = vadd_u8(v30, v24); // add v30.8b, v30.8b, v24.8b  (base_y += 4)
        }

        loop {
            // 49:
            let v16 = vqtbl2_u8(v0v1, v29); // tbl v16.8b, {v0.16b, v1.16b}, v29.8b  (left[base_y+0], left[base_y+2])
            let v17 = vqtbl2_u8(v0v1, v30); // tbl v17.8b, {v0.16b, v1.16b}, v30.8b  (left[base_y+1], left[base_y+3])

            let v18h = vmull_u8(v16, v28); // umull v18.8h, v16.8b, v28.8b  (left[base_y]*(64-frac_y))
            let v18h = vmlal_u8(v18h, v17, v27); // umlal v18.8h, v17.8b, v27.8b  (+ left[base_y+1]*frac_y)
            let v18 = vrshrn_n_u16::<6>(v18h); // rshrn v18.8b, v18.8h, #6

            z2_st1_s::<0>(x0, v18); // st1 {v18.s}[0], [x0], x1
            x0 = x0.wrapping_offset(x1);
            w5 -= 2; // subs w5, w5, #2
            z2_st1_s::<1>(x0, v18); // st1 {v18.s}[1], [x0], x1
            x0 = x0.wrapping_offset(x1);
            if w5 <= 0 {
                return; // b.le 9f
            }

            v29 = vadd_u8(v29, v24); // add v29.8b, v29.8b, v24.8b  (base_y += 4)
            v30 = vadd_u8(v30, v24); // add v30.8b, v30.8b, v24.8b  (base_y += 4)
        }
    }

    // 80:
    let v30 = vdupq_n_u16(w7 as u16); // dup v30.8h, w7  (-dy)
    let v17 = vdup_n_u8(1); // movi v17.8b, #1

    let v16 = vmulq_u16(v31, v30); // mul v16.8h, v31.8h, v30.8h  ({0,1,2,3,4,5,6,7} * -dy)
    let v25 = vdupq_n_u8(0x3e); // movi v25.16b, #0x3e
    let v30 = vaddq_u16(v16, v30); // add v30.8h, v16.8h, v30.8h  (-= dy)

    let v31 = vmovn_u16(v31); // xtn v31.8b, v31.8h  ({0,1,2,3,4,5,6,7})

    // For upsample_left, w <= 8 and h <= 8; we may need up to 2*h+1 elements.
    let v0v1v2 = vld1q_u8_x3(x3); // ld1 {v0.16b, v1.16b, v2.16b}, [x3]  (left[])

    let v26 = vdupq_n_u8(64); // movi v26.16b, #64
    let v19 = vdup_n_u8(2); // movi v19.16b, #2

    let v27 = vmovn_u16(v30); // xtn v27.8b, v30.8h  ((uint8_t)ypos)
    let v29 = vshrn_n_u16::<6>(v30); // shrn v29.8b, v30.8h, #6  (ypos >> 6)
    let v27 = vand_u8(v27, vget_low_u8(v25)); // and v27.8b, v27.8b, v25.8b  (frac_y)

    let v29 = vadd_u8(v29, v19); // add v29.8b, v29.8b, v19.8b  (base_y = (ypos >> 6) + 2)

    let v28 = vadd_u8(v29, v17); // add v28.8b, v29.8b, v17.8b  (base_y + 1)
    let v30 = vadd_u8(v29, v19); // add v30.8b, v29.8b, v19.8b  (base_y + 2)

    let v31 = vcombine_u8(v31, v31); // trn1 v31.2d, v31.2d, v31.2d  ({0,1,2,3,4,5,6,7,0,1,2,3,4,5,6,7})
    let v24 = vadd_u8(v28, v19); // add v24.8b, v28.8b, v19.8b  (base_y + 3)

    let mut v29 = vcombine_u8(v29, v30); // trn1 v29.2d, v29.2d, v30.2d  (base_y + 0, base_y + 2)
    let mut v30 = vcombine_u8(v28, v24); // trn1 v30.2d, v28.2d, v24.2d  (base_y + 1, base_y + 3)

    let v28 = vsub_u8(vget_low_u8(v26), v27); // sub v28.8b, v26.8b, v27.8b  (64 - frac_y)

    let v24 = vdupq_n_u8(4); // movi v24.16b, #4

    let v27 = vcombine_u8(v27, v27); // trn1 v27.2d, v27.2d, v27.2d  (frac_y)
    let v28 = vcombine_u8(v28, v28); // trn1 v28.2d, v28.2d, v28.2d  (64 - frac_y)
    loop {
        // 8:
        let w9 = w8 >> 6; // asr w9, w8, #6  (base_x)
        let v16 = vdupq_n_u16(w8 as u16); // dup v16.8h, w8  (xpos)
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        let w11 = w8 >> 6; // asr w11, w8, #6  (base_x)
        if w9 <= -8 {
            break; // cmp w9, #-8; b.le 89f  (base_x <= -8)
        }

        let v17 = vdupq_n_u16(w8 as u16); // dup v17.8h, w8  (xpos)

        let v4 = vld1q_u8(x2.wrapping_offset(w9 as isize)); // ldr q4, [x2, w9, sxtw]  (top[base_x])
        let v6 = vld1q_u8(x2.wrapping_offset(w11 as isize)); // ldr q6, [x2, w11, sxtw]

        let v18 = vqtbl3q_u8(v0v1v2, v29); // tbl v18.16b, {v0.16b, v1.16b, v2.16b}, v29.16b  (left[base_y+0], left[base_y+2])
        let v19 = vqtbl3q_u8(v0v1v2, v30); // tbl v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b  (left[base_y+1], left[base_y+3])

        let v21 = vshrn_n_u16::<6>(v16); // shrn v21.8b, v16.8h, #6  (first base_x)
        let v21 = vshrn_high_n_u16::<6>(v21, v17); // shrn2 v21.16b, v17.8h, #6
        let v16 = vmovn_u16(v16); // xtn v16.8b, v16.8h  ((uint8_t)xpos)
        let v16 = vmovn_high_u16(v16, v17); // xtn2 v16.16b, v17.8h

        let v5 = vextq_u8::<1>(v4, v4); // ext v5.16b, v4.16b, v4.16b, #1  (top[base_x+1])
        let v7 = vextq_u8::<1>(v6, v6); // ext v7.16b, v6.16b, v6.16b, #1

        let v16 = vandq_u8(v16, v25); // and v16.16b, v16.16b, v25.16b  (frac_x)

        let v4 = z2_trn1_2d(v4, v6); // trn1 v4.2d, v4.2d, v6.2d  (top[base_x])
        let v5 = z2_trn1_2d(v5, v7); // trn1 v5.2d, v5.2d, v7.2d  (top[base_x+1])

        let v7 = vsubq_u8(v26, v16); // sub v7.16b, v26.16b, v16.16b  (64 - frac_x)

        let v21 = vaddq_u8(v21, v31); // add v21.16b, v21.16b, v31.16b  (actual base_x)

        let v6h = vmull_u8(vget_low_u8(v18), vget_low_u8(v28)); // umull v6.8h, v18.8b, v28.8b  (left[base_y]*(64-frac_y))
        let v6h = vmlal_u8(v6h, vget_low_u8(v19), vget_low_u8(v27)); // umlal v6.8h, v19.8b, v27.8b  (+ left[base_y+1]*frac_y)
        let v17h = vmull_high_u8(v18, v28); // umull2 v17.8h, v18.16b, v28.16b
        let v17h = vmlal_high_u8(v17h, v19, v27); // umlal2 v17.8h, v19.16b, v27.16b

        let v22h = vmull_u8(vget_low_u8(v4), vget_low_u8(v7)); // umull v22.8h, v4.8b, v7.8b  (top[base_x]*(64-frac_x))
        let v22h = vmlal_u8(v22h, vget_low_u8(v5), vget_low_u8(v16)); // umlal v22.8h, v5.8b, v16.8b  (+ top[base_x+1]*frac_x)
        let v23h = vmull_high_u8(v4, v7); // umull2 v23.8h, v4.16b, v7.16b
        let v23h = vmlal_high_u8(v23h, v5, v16); // umlal2 v23.8h, v5.16b, v16.16b

        let v21 = vcgezq_s8(vreinterpretq_s8_u8(v21)); // cmge v21.16b, v21.16b, #0

        let v6 = vrshrn_n_u16::<6>(v6h); // rshrn v6.8b, v6.8h, #6
        let v6 = vrshrn_high_n_u16::<6>(v6, v17h); // rshrn2 v6.16b, v17.8h, #6
        let v22 = vrshrn_n_u16::<6>(v22h); // rshrn v22.8b, v22.8h, #6
        let v22 = vrshrn_high_n_u16::<6>(v22, v23h); // rshrn2 v22.16b, v23.8h, #6

        let v6 = vbslq_u8(v21, v22, v6); // bit v6.16b, v22.16b, v21.16b

        vst1_u8(x0, vget_low_u8(v6)); // st1 {v6.d}[0], [x0], x1
        x0 = x0.wrapping_offset(x1);
        w8 -= w6; // sub w8, w8, w6  (xpos -= dx)
        w5 -= 2; // subs w5, w5, #2
        vst1_u8(x0, vget_high_u8(v6)); // st1 {v6.d}[1], [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 4)
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b  (base_y += 4)
    }

    loop {
        // 89:
        let v18 = vqtbl3q_u8(v0v1v2, v29); // tbl v18.16b, {v0.16b, v1.16b, v2.16b}, v29.16b  (left[base_y+0], left[base_y+2])
        let v19 = vqtbl3q_u8(v0v1v2, v30); // tbl v19.16b, {v0.16b, v1.16b, v2.16b}, v30.16b  (left[base_y+1], left[base_y+3])

        let v6h = vmull_u8(vget_low_u8(v18), vget_low_u8(v28)); // umull v6.8h, v18.8b, v28.8b  (left[base_y]*(64-frac_y))
        let v6h = vmlal_u8(v6h, vget_low_u8(v19), vget_low_u8(v27)); // umlal v6.8h, v19.8b, v27.8b  (+ left[base_y+1]*frac_y)
        let v17h = vmull_high_u8(v18, v28); // umull2 v17.8h, v18.16b, v28.16b
        let v17h = vmlal_high_u8(v17h, v19, v27); // umlal2 v17.8h, v19.16b, v27.16b

        let v6 = vrshrn_n_u16::<6>(v6h); // rshrn v6.8b, v6.8h, #6
        let v6 = vrshrn_high_n_u16::<6>(v6, v17h); // rshrn2 v6.16b, v17.8h, #6

        vst1_u8(x0, vget_low_u8(v6)); // st1 {v6.d}[0], [x0], x1
        x0 = x0.wrapping_offset(x1);
        w5 -= 2; // subs w5, w5, #2
        vst1_u8(x0, vget_high_u8(v6)); // st1 {v6.d}[1], [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w5 <= 0 {
            return; // b.le 9f
        }

        v29 = vaddq_u8(v29, v24); // add v29.16b, v29.16b, v24.16b  (base_y += 4)
        v30 = vaddq_u8(v30, v24); // add v30.16b, v30.16b, v24.16b  (base_y += 4)
    }
}

// ============================================================================
// Part E (ipred.S lines 3075–3685): the z3 fill kernels.
// ============================================================================
// Registers are typed locals named after the assembly's registers (`v4`,
// `v27`, `x13`, `w4`); a 64-bit arrangement is a 64-bit vector type. The
// post-incremented addresses are advanced with `wrapping_add`/
// `wrapping_offset` after the access, as the assembly's `[x0], x1`.

/// `const increments` (ipred.S, beside the z2 kernels): the eight words
/// that `ldrel q30, x11, increments` loads.
static IPRED_Z3_INCREMENTS: [u16; 8] = [0, 1, 2, 3, 4, 5, 6, 7];

/// `st1 {v.s}[L], [p]` of a 64-bit register.
#[inline(always)]
unsafe fn ipred_z3_st1_s<const L: i32>(p: *mut u8, v: uint8x8_t) {
    (p as *mut u32).write_unaligned(vget_lane_u32::<L>(vreinterpret_u32_u8(v)));
}

/// `zip1`/`zip2 v18.16b, v16.16b, v17.16b` followed by `st1 {v18.h}[L],
/// [p]`: the bytes `v16.b[L]`, `v17.b[L]` at `p`, as one `st2` of the lane
/// of the two registers.
#[inline(always)]
unsafe fn ipred_z3_st2_b<const L: i32>(p: *mut u8, v16v17: uint8x16x2_t) {
    vst2q_lane_u8::<L>(p, v16v17);
}

/// `trn1 vd.2s, vn.2s, vm.2s` on byte registers.
#[inline(always)]
unsafe fn ipred_z3_trn1_2s(a: uint8x8_t, b: uint8x8_t) -> uint8x8_t {
    vreinterpret_u8_u32(vtrn1_u32(vreinterpret_u32_u8(a), vreinterpret_u32_u8(b)))
}

/// `trn1 vd.2d, vn.2d, vm.2d` of two registers whose 64-bit arrangement
/// was written last (their high halves are clear).
#[inline(always)]
unsafe fn ipred_z3_trn1_2d(a: uint8x8_t, b: uint8x8_t) -> uint8x16_t {
    vcombine_u8(a, b)
}

/// `xtn vd.8b, va.8h` followed by `xtn2 vd.16b, vb.8h`, as the one `uzp1
/// vd.16b, va.16b, vb.16b` that gives the same register. (Written as the
/// two narrowing moves, the compiler computes the halves of the masked
/// value a second time for the widening multiplications.)
#[inline(always)]
unsafe fn ipred_z3_xtn_xtn2(a: uint16x8_t, b: uint16x8_t) -> uint8x16_t {
    vuzp1q_u8(vreinterpretq_u8_u16(a), vreinterpretq_u8_u16(b))
}

/// `ipred_z3_fill1_8bpc_neon`:
/// `void ipred_z3_fill1_8bpc_neon(pixel *dst, const ptrdiff_t stride,
/// const pixel *const left, const int width, const int height, const int dy,
/// const int max_base_y);`
///
/// # Safety
/// Called by `ipred_z3_neon` (`simd/arm/ipred.rs`) only. Writes `dst` rows
/// `0..height`, columns `0..width`. Reads `left[max_base_y]` and, with
/// `max_base_y <= 64`, `left[0..32]` (width 4), `left[0..48]` (width 8) or
/// `left[0..64]` (width 16 to 64); with `max_base_y > 64` it reads
/// `left[base..base + height + 16]` for the columns' `base <
/// max_base_y + 16`, all inside the wrapper's `left_out`
/// (`64 + 64 + (64 + 15) * 2` pixels).
pub unsafe extern "C" fn dav1d_ipred_z3_fill1_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    width: c_int,
    height: c_int,
    dy: c_int,
    max_base_y: c_int,
) {
    let x10 = left.add(max_base_y as u32 as usize); // add x10, x2, w6, uxtw: left[max_base_y]
    let v30 = vld1q_u16(IPRED_Z3_INCREMENTS.as_ptr()); // ldrel q30, x11, increments
    let v31 = vld1q_dup_u8(x10); // ld1r {v31.16b}, [x10]: padding
    if max_base_y > 64 {
        // cmp w6, #64; b.gt L(ipred_z3_fill1_large_h16)
        // (max_base_y > 64 needs width + height > 64: the height is 16, 32
        // or 64 here)
        return match height {
            16 => ipred_z3_fill1_large_h16::<16>(dst, stride, left, width, dy, max_base_y, v31),
            32 => ipred_z3_fill1_large_h16::<32>(dst, stride, left, width, dy, max_base_y, v31),
            _ => ipred_z3_fill1_large_h16::<64>(dst, stride, left, width, dy, max_base_y, v31),
        };
    }
    // clz w9, w3; sub w9, w9, #25; ipred_z3_fill1_tbl; br x8
    match (width as u32).leading_zeros().wrapping_sub(25) {
        4 => ipred_z3_fill1_w4(dst, stride, left, height, dy, v30, v31),
        3 => ipred_z3_fill1_w8(dst, stride, left, height, dy, v30, v31),
        2 => ipred_z3_fill1_w16(dst, stride, left, height, dy, v30, v31),
        0 | 1 => ipred_z3_fill1_w32(dst, stride, left, width, height, dy, max_base_y, v30, v31),
        // SAFETY: the width is one of 4, 8, 16, 32, 64.
        _ => core::hint::unreachable_unchecked(),
    }
}

/// `40:` of `ipred_z3_fill1_8bpc_neon`
#[inline(always)]
unsafe fn ipred_z3_fill1_w4(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w4: c_int,
    w5: c_int,
    v30: uint16x8_t,
    v31: uint8x16_t,
) {
    let v29 = vdup_n_u16(w5 as u16); // dup v29.4h, w5: dy

    let v30 = vmul_u16(vget_low_u16(v30), v29); // mul v30.4h, v30.4h, v29.4h: {0,1,2,3}*dy
    let v23 = vdup_n_u8(0x3e); // movi v23.16b, #0x3e

    // Worst case max_base_y is width+height-1, for w=4, h=16, <= 32
    let v0v1 = vld1q_u8_x2(x2); // ld1 {v0.16b, v1.16b}, [x2]: left[]
    let v30 = vadd_u16(v29, v30); // add v30.4h, v29.4h, v30.4h: ypos

    let v22 = vdup_n_u8(64); // movi v22.16b, #64
    let v20 = vdup_n_u8(1); // movi v20.16b, #1
    let v21 = vdup_n_u8(2); // movi v21.16b, #2

    let v30 = vcombine_u16(v30, vdup_n_u16(0)); // (the .4h results cleared the high half)
    let v24 = vmovn_u16(v30); // xtn v24.8b, v30.8h: (uint8_t)ypos
    let v26 = vqshrn_n_u16::<6>(v30); // uqshrn v26.8b, v30.8h, #6: base
    let v24 = vand_u8(v24, v23); // and v24.8b, v24.8b, v23.8b: frac

    let v4 = vget_low_u8(v31); // mov v4.8b, v31.8b
    let v27 = vqadd_u8(v26, v20); // uqadd v27.8b, v26.8b, v20.8b: base + 1
    let v28 = vqadd_u8(v26, v21); // uqadd v28.8b, v26.8b, v21.8b: base + 2
    let v25 = vsub_u8(v22, v24); // sub v25.8b, v22.8b, v24.8b: 64 - frac

    let mut v4 = vqtbx2_u8(v4, v0v1, v26); // tbx v4.8b, {v0.16b, v1.16b}, v26.8b: left[base]

    let mut v27 = ipred_z3_trn1_2s(v27, v28); // trn1 v27.2s, v27.2s, v28.2s: base + 1, base + 2
    let v24 = ipred_z3_trn1_2s(v24, v24); // trn1 v24.2s, v24.2s, v24.2s: frac
    let v25 = ipred_z3_trn1_2s(v25, v25); // trn1 v25.2s, v25.2s, v25.2s: 64 - frac
    loop {
        // 1:
        let v5 = vget_low_u8(v31); // mov v5.8b, v31.8b
        let v5 = vqtbx2_u8(v5, v0v1, v27); // tbx v5.8b, {v0.16b, v1.16b}, v27.8b: left[base+1], left[base+2]

        v4 = ipred_z3_trn1_2s(v4, v5); // trn1 v4.2s, v4.2s, v5.2s: left[base], left[base+1]

        let v16 = vmull_u8(v4, v25); // umull v16.8h, v4.8b, v25.8b: left[base]*(64-frac)
        let v16 = vmlal_u8(v16, v5, v24); // umlal v16.8h, v5.8b, v24.8b: + left[base+1]*frac
        let v16 = vrshrn_n_u16::<6>(v16); // rshrn v16.8b, v16.8h, #6
        ipred_z3_st1_s::<0>(x0, v16); // st1 {v16.s}[0], [x0], x1
        x0 = x0.wrapping_offset(x1);
        w4 -= 2; // subs w4, w4, #2
        ipred_z3_st1_s::<1>(x0, v16); // st1 {v16.s}[1], [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w4 <= 0 {
            break; // b.le 9f
        }

        v4 = vext_u8::<4>(v5, v5); // ext v4.8b, v5.8b, v5.8b, #4
        v27 = vqadd_u8(v27, v21); // uqadd v27.8b, v27.8b, v21.8b: base += 2
    }
}

/// `80:` of `ipred_z3_fill1_8bpc_neon`
#[inline(always)]
unsafe fn ipred_z3_fill1_w8(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w4: c_int,
    w5: c_int,
    v30: uint16x8_t,
    v31: uint8x16_t,
) {
    let v29 = vdupq_n_u16(w5 as u16); // dup v29.8h, w5: dy

    let v30 = vmulq_u16(v30, v29); // mul v30.8h, v30.8h, v29.8h: {0,1,2,3,4,5,6,7}*dy
    let v23 = vdup_n_u8(0x3e); // movi v23.16b, #0x3e

    // Worst case max_base_y is width+height-1, for w=8, h=32, <= 48
    let v0v2 = vld1q_u8_x3(x2); // ld1 {v0.16b, v1.16b, v2.16b}, [x2]: left[]
    let v30 = vaddq_u16(v29, v30); // add v30.8h, v29.8h, v30.8h: ypos

    let v22 = vdup_n_u8(64); // movi v22.16b, #64
    let v20 = vdup_n_u8(1); // movi v20.16b, #1
    let v21 = vdup_n_u8(2); // movi v21.16b, #2

    let v24 = vmovn_u16(v30); // xtn v24.8b, v30.8h: (uint8_t)ypos
    let v26 = vqshrn_n_u16::<6>(v30); // uqshrn v26.8b, v30.8h, #6: base
    let v24 = vand_u8(v24, v23); // and v24.8b, v24.8b, v23.8b: frac

    let v4 = vget_low_u8(v31); // mov v4.8b, v31.8b
    let mut v27 = vqadd_u8(v26, v20); // uqadd v27.8b, v26.8b, v20.8b: base + 1
    let mut v28 = vqadd_u8(v26, v21); // uqadd v28.8b, v26.8b, v21.8b: base + 2
    let v25 = vsub_u8(v22, v24); // sub v25.8b, v22.8b, v24.8b: 64 - frac

    let mut v4 = vqtbx3_u8(v4, v0v2, v26); // tbx v4.8b, {v0.16b, v1.16b, v2.16b}, v26.8b: left[base]
    loop {
        // 1:
        let v5 = vget_low_u8(v31); // mov v5.8b, v31.8b
        let v6 = vget_low_u8(v31); // mov v6.8b, v31.8b
        let v5 = vqtbx3_u8(v5, v0v2, v27); // tbx v5.8b, {v0.16b, v1.16b, v2.16b}, v27.8b: left[base+1]
        let v6 = vqtbx3_u8(v6, v0v2, v28); // tbx v6.8b, {v0.16b, v1.16b, v2.16b}, v28.8b: left[base+2]

        let v16 = vmull_u8(v4, v25); // umull v16.8h, v4.8b, v25.8b: left[base]*(64-frac)
        let v16 = vmlal_u8(v16, v5, v24); // umlal v16.8h, v5.8b, v24.8b: + left[base+1]*frac
        let v17 = vmull_u8(v5, v25); // umull v17.8h, v5.8b, v25.8b
        let v17 = vmlal_u8(v17, v6, v24); // umlal v17.8h, v6.8b, v24.8b
        let v16 = vrshrn_n_u16::<6>(v16); // rshrn v16.8b, v16.8h, #6
        let v17 = vrshrn_n_u16::<6>(v17); // rshrn v17.8b, v17.8h, #6
        vst1_u8(x0, v16); // st1 {v16.8b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        w4 -= 2; // subs w4, w4, #2
        vst1_u8(x0, v17); // st1 {v17.8b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w4 <= 0 {
            break; // b.le 9f
        }

        v4 = v6; // mov v4.8b, v6.8b
        v27 = vqadd_u8(v27, v21); // uqadd v27.8b, v27.8b, v21.8b: base += 2
        v28 = vqadd_u8(v28, v21); // uqadd v28.8b, v28.8b, v21.8b: base += 2
    }
}

/// `160:` of `ipred_z3_fill1_8bpc_neon`
#[inline(always)]
unsafe fn ipred_z3_fill1_w16(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w4: c_int,
    w5: c_int,
    v30: uint16x8_t,
    v31: uint8x16_t,
) {
    let v28 = vdupq_n_u16(w5 as u16); // dup v28.8h, w5: dy

    let v29 = vshlq_n_u16::<3>(v28); // shl v29.8h, v28.8h, #3: 8*dy
    let v30 = vmulq_u16(v30, v28); // mul v30.8h, v30.8h, v28.8h: {0,1,2,3,4,5,6,7}*dy
    let v23 = vdupq_n_u8(0x3e); // movi v23.16b, #0x3e

    // This is only executed if we've checked that max_base_y <= 64.
    let v0v3 = vld1q_u8_x4(x2); // ld1 {v0.16b, v1.16b, v2.16b, v3.16b}, [x2]: left[]
    let v28 = vaddq_u16(v28, v30); // add v28.8h, v28.8h, v30.8h: ypos

    let v22 = vdupq_n_u8(64); // movi v22.16b, #64
    let v20 = vdupq_n_u8(1); // movi v20.16b, #1
    let v21 = vdupq_n_u8(2); // movi v21.16b, #2

    let v29 = vaddq_u16(v28, v29); // add v29.8h, v28.8h, v29.8h: ypos + 8*dy

    let v24 = ipred_z3_xtn_xtn2(v28, v29); // xtn v24.8b, v28.8h; xtn2 v24.16b, v29.8h: (uint8_t)ypos
    let v26 = vqshrn_high_n_u16::<6>(vqshrn_n_u16::<6>(v28), v29); // uqshrn v26.8b, v28.8h, #6; uqshrn2 v26.16b, v29.8h, #6: base
    let v24 = vandq_u8(v24, v23); // and v24.16b, v24.16b, v23.16b: frac

    let v4 = v31; // mov v4.16b, v31.16b
    let mut v27 = vqaddq_u8(v26, v20); // uqadd v27.16b, v26.16b, v20.16b: base + 1
    let mut v28 = vqaddq_u8(v26, v21); // uqadd v28.16b, v26.16b, v21.16b: base + 2
    let v25 = vsubq_u8(v22, v24); // sub v25.16b, v22.16b, v24.16b: 64 - frac

    let mut v4 = vqtbx4q_u8(v4, v0v3, v26); // tbx v4.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v26.16b: left[base]
    loop {
        // 1:
        let v5 = v31; // mov v5.16b, v31.16b
        let v6 = v31; // mov v6.16b, v31.16b
        let v5 = vqtbx4q_u8(v5, v0v3, v27); // tbx v5.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v27.16b: left[base+1]
        let v6 = vqtbx4q_u8(v6, v0v3, v28); // tbx v6.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v28.16b: left[base+2]

        let v16 = vmull_u8(vget_low_u8(v4), vget_low_u8(v25)); // umull v16.8h, v4.8b, v25.8b: left[base]*(64-frac)
        let v16 = vmlal_u8(v16, vget_low_u8(v5), vget_low_u8(v24)); // umlal v16.8h, v5.8b, v24.8b: + left[base+1]*frac
        let v17 = vmull_high_u8(v4, v25); // umull2 v17.8h, v4.16b, v25.16b
        let v17 = vmlal_high_u8(v17, v5, v24); // umlal2 v17.8h, v5.16b, v24.16b
        let v18 = vmull_u8(vget_low_u8(v5), vget_low_u8(v25)); // umull v18.8h, v5.8b, v25.8b
        let v18 = vmlal_u8(v18, vget_low_u8(v6), vget_low_u8(v24)); // umlal v18.8h, v6.8b, v24.8b
        let v19 = vmull_high_u8(v5, v25); // umull2 v19.8h, v5.16b, v25.16b
        let v19 = vmlal_high_u8(v19, v6, v24); // umlal2 v19.8h, v6.16b, v24.16b
        let v16 = vrshrn_high_n_u16::<6>(vrshrn_n_u16::<6>(v16), v17); // rshrn v16.8b, v16.8h, #6; rshrn2 v16.16b, v17.8h, #6
        let v17 = vrshrn_high_n_u16::<6>(vrshrn_n_u16::<6>(v18), v19); // rshrn v17.8b, v18.8h, #6; rshrn2 v17.16b, v19.8h, #6
        vst1q_u8(x0, v16); // st1 {v16.16b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        w4 -= 2; // subs w4, w4, #2
        vst1q_u8(x0, v17); // st1 {v17.16b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w4 <= 0 {
            break; // b.le 9f
        }

        v4 = v6; // mov v4.16b, v6.16b
        v27 = vqaddq_u8(v27, v21); // uqadd v27.16b, v27.16b, v21.16b: base += 2
        v28 = vqaddq_u8(v28, v21); // uqadd v28.16b, v28.16b, v21.16b: base += 2
    }
}

/// `320:`/`640:` of `ipred_z3_fill1_8bpc_neon`
#[inline(always)]
unsafe fn ipred_z3_fill1_w32(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w3: c_int,
    mut w4: c_int,
    w5: c_int,
    w6: c_int,
    v30: uint16x8_t,
    v31: uint8x16_t,
) {
    let v28 = vdupq_n_u16(w5 as u16); // dup v28.8h, w5: dy
    let w12 = w3; // mov w12, w3

    let mut x13 = x0.wrapping_offset(x1); // add x13, x0, x1

    let v29 = vshlq_n_u16::<3>(v28); // shl v29.8h, v28.8h, #3: 8*dy
    let v30 = vmulq_u16(v30, v28); // mul v30.8h, v30.8h, v28.8h: {0,1,2,3,4,5,6,7}*dy
    let v23 = vdupq_n_u8(0x3e); // movi v23.16b, #0x3e

    let x1 = x1.wrapping_shl(1); // lsl x1, x1, #1
    let x1 = x1.wrapping_sub(w3 as u32 as isize); // sub x1, x1, w3, uxtw
    let mut v30 = vaddq_u16(v28, v30); // add v30.8h, v28.8h, v30.8h: ypos

    // This is only executed if we've checked that max_base_y <= 64.
    let v0v3 = vld1q_u8_x4(x2); // ld1 {v0.16b, v1.16b, v2.16b, v3.16b}, [x2]: left[]

    let v22 = vdupq_n_u8(64); // movi v22.16b, #64
    let v20 = vdupq_n_u8(1); // movi v20.16b, #1
    let v21 = vdupq_n_u8(2); // movi v21.16b, #2

    loop {
        // 1:
        let mut v26 = v30; // mov v26.16b, v30.16b: reset ypos

        loop {
            // 2:
            let v27 = vaddq_u16(v26, v29); // add v27.8h, v26.8h, v29.8h: ypos + 8*dy
            let v16 = vqshrn_high_n_u16::<6>(vqshrn_n_u16::<6>(v26), v27); // uqshrn v16.8b, v26.8h, #6; uqshrn2 v16.16b, v27.8h, #6: base
            let v24 = ipred_z3_xtn_xtn2(v26, v27); // xtn v24.8b, v26.8h; xtn2 v24.16b, v27.8h: (uint8_t)ypos
            let w14 = vgetq_lane_u8::<0>(v16) as c_int; // umov w14, v16.b[0]
            let v24 = vandq_u8(v24, v23); // and v24.16b, v24.16b, v23.16b: frac

            let v17 = vqaddq_u8(v16, v20); // uqadd v17.16b, v16.16b, v20.16b: base + 1
            let v18 = vqaddq_u8(v16, v21); // uqadd v18.16b, v16.16b, v21.16b: base + 2
            let v25 = vsubq_u8(v22, v24); // sub v25.16b, v22.16b, v24.16b: 64 - frac

            if w14 >= w6 {
                // cmp w14, w6: base >= max_base_y; b.ge 4f
                loop {
                    // 4:
                    w3 -= 16; // subs w3, w3, #16
                    vst1q_u8(x0, v31); // st1 {v31.16b}, [x0], #16
                    x0 = x0.wrapping_add(16);
                    vst1q_u8(x13, v31); // st1 {v31.16b}, [x13], #16
                    x13 = x13.wrapping_add(16);
                    if w3 <= 0 {
                        break; // b.gt 4b
                    }
                }
                break; // b 3b
            }

            let v4 = v31; // mov v4.16b, v31.16b
            let v5 = v31; // mov v5.16b, v31.16b
            let v6 = v31; // mov v6.16b, v31.16b
            let v4 = vqtbx4q_u8(v4, v0v3, v16); // tbx v4.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v16.16b: left[base]
            let v5 = vqtbx4q_u8(v5, v0v3, v17); // tbx v5.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v17.16b: left[base+1]
            let v6 = vqtbx4q_u8(v6, v0v3, v18); // tbx v6.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v18.16b: left[base+2]

            w3 -= 16; // subs w3, w3, #16
            let v16 = vmull_u8(vget_low_u8(v4), vget_low_u8(v25)); // umull v16.8h, v4.8b, v25.8b: left[base]*(64-frac)
            let v16 = vmlal_u8(v16, vget_low_u8(v5), vget_low_u8(v24)); // umlal v16.8h, v5.8b, v24.8b: + left[base+1]*frac
            let v17 = vmull_high_u8(v4, v25); // umull2 v17.8h, v4.16b, v25.16b
            let v17 = vmlal_high_u8(v17, v5, v24); // umlal2 v17.8h, v5.16b, v24.16b
            let v18 = vmull_u8(vget_low_u8(v5), vget_low_u8(v25)); // umull v18.8h, v5.8b, v25.8b
            let v18 = vmlal_u8(v18, vget_low_u8(v6), vget_low_u8(v24)); // umlal v18.8h, v6.8b, v24.8b
            let v19 = vmull_high_u8(v5, v25); // umull2 v19.8h, v5.16b, v25.16b
            let v19 = vmlal_high_u8(v19, v6, v24); // umlal2 v19.8h, v6.16b, v24.16b
            let v16 = vrshrn_high_n_u16::<6>(vrshrn_n_u16::<6>(v16), v17); // rshrn v16.8b, v16.8h, #6; rshrn2 v16.16b, v17.8h, #6
            let v17 = vrshrn_high_n_u16::<6>(vrshrn_n_u16::<6>(v18), v19); // rshrn v17.8b, v18.8h, #6; rshrn2 v17.16b, v19.8h, #6
            vst1q_u8(x0, v16); // st1 {v16.16b}, [x0], #16
            x0 = x0.wrapping_add(16);
            vst1q_u8(x13, v17); // st1 {v17.16b}, [x13], #16
            x13 = x13.wrapping_add(16);
            if w3 <= 0 {
                break; // b.le 3f
            }
            v26 = vaddq_u16(v27, v29); // add v26.8h, v27.8h, v29.8h: ypos += 16*dy
        }

        // 3:
        w4 -= 2; // subs w4, w4, #2
        if w4 <= 0 {
            return; // b.le 9f
        }
        let v16 = vdupq_n_u16(128); // movi v16.8h, #128
        x0 = x0.wrapping_offset(x1); // add x0, x0, x1
        x13 = x13.wrapping_offset(x1); // add x13, x13, x1
        v30 = vaddq_u16(v30, v16); // add v30.8h, v30.8h, v16.8h: ypos = dy + y*(1<<6)*2
        w3 = w12; // mov w3, w12
    }
}

/// `L(ipred_z3_fill1_large_h16)` of `ipred_z3_fill1_8bpc_neon`:
/// Fallback case for max_base_y > 64; similar to the z1 implementation.
/// This does the filtering vertically, filling out a 2x pixel column at a
/// time.
///
/// The height `H` (16, 32 or 64: max_base_y > 64 needs width + height > 64)
/// is a constant of each instance, so that the compiler unrolls the loop
/// `2:` over the 16 rows and has the `dup`s of the column pair beside the
/// multiplications (`umull2`/`umlal2`).
unsafe fn ipred_z3_fill1_large_h16<const H: c_int>(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w3: c_int,
    w5: c_int,
    w6: c_int,
    v31: uint8x16_t,
) {
    let mut w4 = H;
    let mut w7 = w5 as u32; // mov w7, w5
    let w15 = 64; // mov w15, #64
    let mut x13 = x0.wrapping_offset(x1); // add x13, x0, x1
    let x1 = x1.wrapping_shl(1); // lsl x1, x1, #1

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
        let mut x8 = x2.add(w8 as usize); // add x8, x2, w8, uxtw
        let mut x10 = x2.add(w10 as usize); // add x10, x2, w10, uxtw
        let v4 = vdupq_n_u8(w9 as u8); // dup v4.16b, w9: frac
        let v5 = vdupq_n_u8(w11 as u8); // dup v5.16b, w11
        let v0v1 = vld1q_u8_x2(x8); // ld1 {v0.16b, v1.16b}, [x8], #32: left[base]
        x8 = x8.wrapping_add(32);
        let v2v3 = vld1q_u8_x2(x10); // ld1 {v2.16b, v3.16b}, [x10], #32
        x10 = x10.wrapping_add(32);
        let (mut v0, mut v1, mut v2, mut v3) = (v0v1.0, v0v1.1, v2v3.0, v2v3.1);
        let w9 = w15 - w9; // sub w9, w15, w9: 64 - frac
        let w11 = w15 - w11; // sub w11, w15, w11
        let v6 = vdupq_n_u8(w9 as u8); // dup v6.16b, w9: 64 - frac
        let v7 = vdupq_n_u8(w11 as u8); // dup v7.16b, w11
        w7 = w7.wrapping_add(w5 as u32); // add w7, w7, w5: ypos += dy

        // The row pointers of this column pair; `x0`/`x13` stay at its top
        // (see `3:`).
        let (mut p0, mut p13) = (x0, x13);
        loop {
            // 2:
            let v16 = vextq_u8::<1>(v0, v1); // ext v16.16b, v0.16b, v1.16b, #1: left[base+1]
            let v17 = vextq_u8::<1>(v2, v3); // ext v17.16b, v2.16b, v3.16b, #1
            w4 -= 16; // subs w4, w4, #16
            let v18 = vmull_u8(vget_low_u8(v16), vget_low_u8(v4)); // umull v18.8h, v16.8b, v4.8b: left[base+1]*frac
            let v18 = vmlal_u8(v18, vget_low_u8(v0), vget_low_u8(v6)); // umlal v18.8h, v0.8b, v6.8b: + left[base]*(64-frac)
            let v19 = vmull_high_u8(v16, v4); // umull2 v19.8h, v16.16b, v4.16b
            let v19 = vmlal_high_u8(v19, v0, v6); // umlal2 v19.8h, v0.16b, v6.16b
            let v20 = vmull_u8(vget_low_u8(v17), vget_low_u8(v5)); // umull v20.8h, v17.8b, v5.8b
            let v20 = vmlal_u8(v20, vget_low_u8(v2), vget_low_u8(v7)); // umlal v20.8h, v2.8b, v7.8b
            let v21 = vmull_high_u8(v17, v5); // umull2 v21.8h, v17.16b, v5.16b
            let v21 = vmlal_high_u8(v21, v2, v7); // umlal2 v21.8h, v2.16b, v7.16b
            let v16 = vrshrn_high_n_u16::<6>(vrshrn_n_u16::<6>(v18), v19); // rshrn v16.8b, v18.8h, #6; rshrn2 v16.16b, v19.8h, #6
            let v17 = vrshrn_high_n_u16::<6>(vrshrn_n_u16::<6>(v20), v21); // rshrn v17.8b, v20.8h, #6; rshrn2 v17.16b, v21.8h, #6

            // zip1 v18.16b, v16.16b, v17.16b; zip2 v19.16b, v16.16b, v17.16b:
            // the stores take the pairs from the two registers
            let v16v17 = uint8x16x2_t(v16, v17);
            ipred_z3_st2_b::<0>(p0, v16v17); // st1 {v18.h}[0], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_b::<1>(p13, v16v17); // st1 {v18.h}[1], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_b::<2>(p0, v16v17); // st1 {v18.h}[2], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_b::<3>(p13, v16v17); // st1 {v18.h}[3], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_b::<4>(p0, v16v17); // st1 {v18.h}[4], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_b::<5>(p13, v16v17); // st1 {v18.h}[5], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_b::<6>(p0, v16v17); // st1 {v18.h}[6], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_b::<7>(p13, v16v17); // st1 {v18.h}[7], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_b::<8>(p0, v16v17); // st1 {v19.h}[0], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_b::<9>(p13, v16v17); // st1 {v19.h}[1], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_b::<10>(p0, v16v17); // st1 {v19.h}[2], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_b::<11>(p13, v16v17); // st1 {v19.h}[3], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_b::<12>(p0, v16v17); // st1 {v19.h}[4], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_b::<13>(p13, v16v17); // st1 {v19.h}[5], [x13], x1
            p13 = p13.wrapping_offset(x1);
            ipred_z3_st2_b::<14>(p0, v16v17); // st1 {v19.h}[6], [x0], x1
            p0 = p0.wrapping_offset(x1);
            ipred_z3_st2_b::<15>(p13, v16v17); // st1 {v19.h}[7], [x13], x1
            p13 = p13.wrapping_offset(x1);
            if w4 <= 0 {
                break; // b.le 3f
            }
            v0 = v1; // mov v0.16b, v1.16b
            v1 = vld1q_u8(x8); // ld1 {v1.16b}, [x8], #16: left[base]
            x8 = x8.wrapping_add(16);
            v2 = v3; // mov v2.16b, v3.16b
            v3 = vld1q_u8(x10); // ld1 {v3.16b}, [x10], #16
            x10 = x10.wrapping_add(16);
        }

        // 3:
        w3 -= 2; // subs w3, w3, #2
        if w3 <= 0 {
            return; // b.le 9f
        }
        // lsr x1, x1, #1; msub x0, x1, x12, x0: ptr -= h * stride; msub x13, x1, x12, x13;
        // lsl x1, x1, #1: back at the top of the column pair (`x0`, `x13`)
        x0 = x0.wrapping_add(2); // add x0, x0, #2
        x13 = x13.wrapping_add(2); // add x13, x13, #2
        w4 = w12; // mov w4, w12
    }
}

/// One label (`20:`, `40:`, `80:`, `160:`) of `ipred_z3_fill_padding_neon`:
/// fills `N` columns of the constant height with padding and steps to the
/// next columns. Returns true at `b.le 9f`.
#[inline(always)]
unsafe fn ipred_z3_fill_padding_cols<const N: usize>(
    x0: &mut *mut u8,
    x1: isize,
    x13: &mut *mut u8,
    w3: &mut c_int,
    w4: &mut c_int,
    w12: c_int,
    v31: uint8x16_t,
) -> bool {
    // st1 {v31.h}[0] / {v31.s}[0] / {v31.8b} / {v31.16b}, [p], x1
    #[inline(always)]
    unsafe fn st<const N: usize>(p: *mut u8, v31: uint8x16_t) {
        match N {
            2 => (p as *mut u16).write_unaligned(vgetq_lane_u16::<0>(vreinterpretq_u16_u8(v31))),
            4 => (p as *mut u32).write_unaligned(vgetq_lane_u32::<0>(vreinterpretq_u32_u8(v31))),
            8 => vst1_u8(p, vget_low_u8(v31)),
            _ => vst1q_u8(p, v31),
        }
    }
    // The row pointers of these columns; `x0`/`x13` stay at their top.
    let (mut p0, mut p13) = (*x0, *x13);
    loop {
        // 2: / 4: / 8: / 16:
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
    *x0 = x0.wrapping_add(N); // add x0, x0, #N
    *x13 = x13.wrapping_add(N); // add x13, x13, #N
    *w4 = w12; // mov w4, w12
    false // b 1b
}

/// `ipred_z3_fill_padding_neon`: reached by a branch from
/// `L(ipred_z3_fill1_large_h16)` with `x0`/`x13` at the current column of
/// rows 0 and 1, `x1` the doubled stride, `w3` the remaining width, `w4`
/// the constant height and `v31` the padding.
///
/// # Safety
/// Writes the remaining `w3` columns of the `w4` rows of `dst`.
unsafe fn ipred_z3_fill_padding_neon(
    mut x0: *mut u8,
    x1: isize,
    mut x13: *mut u8,
    mut w3: c_int,
    mut w4: c_int,
    v31: uint8x16_t,
) {
    if w3 > 16 {
        // cmp w3, #16; b.gt ipred_z3_fill_padding_wide
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
            3 => ipred_z3_fill_padding_cols::<8>(&mut x0, x1, &mut x13, &mut w3, &mut w4, w12, v31), // 80:
            0 | 1 | 2 => ipred_z3_fill_padding_cols::<16>(&mut x0, x1, &mut x13, &mut w3, &mut w4, w12, v31), // 160: 320: 640:
            // SAFETY: the remaining width is even and in 2..=16 here.
            _ => core::hint::unreachable_unchecked(),
        };
        if done {
            return; // 9: ret
        }
    }
}

/// `ipred_z3_fill_padding_wide`: Fill a WxH rectangle with padding, with
/// W > 16.
#[inline(always)]
unsafe fn ipred_z3_fill_padding_wide(mut x0: *mut u8, x1: isize, mut w3: c_int, mut w4: c_int, v31: uint8x16_t) {
    let x1 = (x1 as usize) >> 1; // lsr x1, x1, #1
    let w12 = w3; // mov w12, w3
    let x1 = x1.wrapping_sub(w3 as u32 as usize); // sub x1, x1, w3, uxtw
    loop {
        // 1:
        let w5 = w3 & 15; // ands w5, w3, #15
        if w5 != 0 {
            // b.eq 2f
            // If the width isn't aligned to 16, first do one 16 byte write
            // and align the start pointer.
            w3 -= w5; // sub w3, w3, w5
            vst1q_u8(x0, v31); // st1 {v31.16b}, [x0]
            x0 = x0.wrapping_add(w5 as u32 as usize); // add x0, x0, w5, uxtw
        }
        loop {
            // 2:
            // Fill the rest of the line with aligned 16 byte writes.
            w3 -= 16; // subs w3, w3, #16
            vst1q_u8(x0, v31); // st1 {v31.16b}, [x0], #16
            x0 = x0.wrapping_add(16);
            if w3 <= 0 {
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

/// `ipred_z3_fill2_8bpc_neon`:
/// `void ipred_z3_fill2_8bpc_neon(pixel *dst, const ptrdiff_t stride,
/// const pixel *const left, const int width, const int height, const int dy,
/// const int max_base_y);` (the upsampled edge: `left[]` steps by two per
/// row).
///
/// # Safety
/// Called by `ipred_z3_neon` (`simd/arm/ipred.rs`) only, with width and
/// height 4 or 8. Writes `dst` rows `0..height`, columns `0..width`. Reads
/// `left[max_base_y]` and `left[0..32]`.
pub unsafe extern "C" fn dav1d_ipred_z3_fill2_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    left: *const u8,
    width: c_int,
    height: c_int,
    dy: c_int,
    max_base_y: c_int,
) {
    let x10 = left.add(max_base_y as u32 as usize); // add x10, x2, w6, uxtw: left[max_base_y]
    let v30 = vld1q_u16(IPRED_Z3_INCREMENTS.as_ptr()); // ldrel q30, x11, increments
    let v31 = vld1q_dup_u8(x10); // ld1r {v31.16b}, [x10]: padding
    if width == 8 {
        // cmp w3, #8; b.eq 80f
        ipred_z3_fill2_w8(dst, stride, left, height, dy, v30, v31)
    } else {
        ipred_z3_fill2_w4(dst, stride, left, height, dy, v30, v31)
    }
}

/// `40:` of `ipred_z3_fill2_8bpc_neon` (w == 4)
#[inline(always)]
unsafe fn ipred_z3_fill2_w4(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w4: c_int,
    w5: c_int,
    v30: uint16x8_t,
    v31: uint8x16_t,
) {
    let v29 = vdup_n_u16(w5 as u16); // dup v29.4h, w5: dy

    let v30 = vmul_u16(vget_low_u16(v30), v29); // mul v30.4h, v30.4h, v29.4h: {0,1,2,3}*dy
    let v23 = vdup_n_u8(0x3e); // movi v23.16b, #0x3e

    // Worst case max_base_y is 2*(width+height)-2, but width+height <= 16,
    // so max_base_y <= 32.
    let v0v1 = vld1q_u8_x2(x2); // ld1 {v0.16b, v1.16b}, [x2]: left[]
    let v30 = vadd_u16(v29, v30); // add v30.4h, v29.4h, v30.4h: ypos

    let v22 = vdup_n_u8(64); // movi v22.16b, #64
    let v20 = vdup_n_u8(1); // movi v20.16b, #1
    let v21 = vdup_n_u8(2); // movi v21.16b, #2

    let v30 = vcombine_u16(v30, vdup_n_u16(0)); // (the .4h results cleared the high half)
    let v24 = vmovn_u16(v30); // xtn v24.8b, v30.8h: (uint8_t)ypos
    let v26 = vqshrn_n_u16::<6>(v30); // uqshrn v26.8b, v30.8h, #6: base
    let v24 = vand_u8(v24, v23); // and v24.8b, v24.8b, v23.8b: frac

    let v27 = vqadd_u8(v26, v20); // uqadd v27.8b, v26.8b, v20.8b: base + 1
    let v28 = vqadd_u8(v26, v21); // uqadd v28.8b, v26.8b, v21.8b: base + 2
    let v25 = vsub_u8(v22, v24); // sub v25.8b, v22.8b, v24.8b: 64 - frac
    let v29 = vqadd_u8(v27, v21); // uqadd v29.8b, v27.8b, v21.8b: base + 3

    let v24 = ipred_z3_trn1_2s(v24, v24); // trn1 v24.2s, v24.2s, v24.2s: frac
    let mut v26 = ipred_z3_trn1_2s(v26, v28); // trn1 v26.2s, v26.2s, v28.2s: base + 0, base + 2
    let mut v27 = ipred_z3_trn1_2s(v27, v29); // trn1 v27.2s, v27.2s, v29.2s: base + 1, base + 3
    let v25 = ipred_z3_trn1_2s(v25, v25); // trn1 v25.2s, v25.2s, v25.2s: 64 - frac

    let v21 = vdup_n_u8(4); // movi v21.16b, #4
    loop {
        // 1:
        let v4 = vget_low_u8(v31); // mov v4.8b, v31.8b
        let v5 = vget_low_u8(v31); // mov v5.8b, v31.8b
        let v4 = vqtbx2_u8(v4, v0v1, v26); // tbx v4.8b, {v0.16b, v1.16b}, v26.8b: left[base], left[base+2]
        let v5 = vqtbx2_u8(v5, v0v1, v27); // tbx v5.8b, {v0.16b, v1.16b}, v27.8b: left[base+1], left[base+3]

        let v16 = vmull_u8(v4, v25); // umull v16.8h, v4.8b, v25.8b: left[base]*(64-frac)
        let v16 = vmlal_u8(v16, v5, v24); // umlal v16.8h, v5.8b, v24.8b: + left[base+1]*frac
        let v16 = vrshrn_n_u16::<6>(v16); // rshrn v16.8b, v16.8h, #6
        ipred_z3_st1_s::<0>(x0, v16); // st1 {v16.s}[0], [x0], x1
        x0 = x0.wrapping_offset(x1);
        w4 -= 2; // subs w4, w4, #2
        ipred_z3_st1_s::<1>(x0, v16); // st1 {v16.s}[1], [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w4 <= 0 {
            break; // b.le 9f
        }

        v26 = vqadd_u8(v26, v21); // uqadd v26.8b, v26.8b, v21.8b: base += 4
        v27 = vqadd_u8(v27, v21); // uqadd v27.8b, v27.8b, v21.8b: base += 4
    }
}

/// `80:` of `ipred_z3_fill2_8bpc_neon` (w == 8). The assembly's `tbx` names
/// four table registers but loads only `v0` and `v1`; the indices stay
/// below 32 (the wrapper upsamples only with width + height <= 16 and
/// dy <= 2 * 51), so the table here is the two loaded registers.
#[inline(always)]
unsafe fn ipred_z3_fill2_w8(
    mut x0: *mut u8,
    x1: isize,
    x2: *const u8,
    mut w4: c_int,
    w5: c_int,
    v30: uint16x8_t,
    v31: uint8x16_t,
) {
    let v29 = vdupq_n_u16(w5 as u16); // dup v29.8h, w5: dy

    let v30 = vmulq_u16(v30, v29); // mul v30.8h, v30.8h, v29.8h: {0,1,2,3,4,5,6,7}*dy
    let v23 = vdup_n_u8(0x3e); // movi v23.16b, #0x3e

    // Worst case max_base_y is 2*(width+height)-2, but width+height <= 16,
    // so max_base_y <= 32.
    let v0v1 = vld1q_u8_x2(x2); // ld1 {v0.16b, v1.16b}, [x2]: left[]
    let v30 = vaddq_u16(v29, v30); // add v30.8h, v29.8h, v30.8h: ypos

    let v22 = vdup_n_u8(64); // movi v22.16b, #64
    let v20 = vdup_n_u8(1); // movi v20.16b, #1
    let v21 = vdup_n_u8(2); // movi v21.16b, #2

    let v24 = vmovn_u16(v30); // xtn v24.8b, v30.8h: (uint8_t)ypos
    let v26 = vqshrn_n_u16::<6>(v30); // uqshrn v26.8b, v30.8h, #6: base
    let v24 = vand_u8(v24, v23); // and v24.8b, v24.8b, v23.8b: frac

    let v27 = vqadd_u8(v26, v20); // uqadd v27.8b, v26.8b, v20.8b: base + 1
    let v28 = vqadd_u8(v26, v21); // uqadd v28.8b, v26.8b, v21.8b: base + 2
    let v25 = vsub_u8(v22, v24); // sub v25.8b, v22.8b, v24.8b: 64 - frac
    let v29 = vqadd_u8(v27, v21); // uqadd v29.8b, v27.8b, v21.8b: base + 3

    let v24 = ipred_z3_trn1_2d(v24, v24); // trn1 v24.2d, v24.2d, v24.2d: frac
    let mut v26 = ipred_z3_trn1_2d(v26, v28); // trn1 v26.2d, v26.2d, v28.2d: base + 0, base + 2
    let mut v27 = ipred_z3_trn1_2d(v27, v29); // trn1 v27.2d, v27.2d, v29.2d: base + 1, base + 3
    let v25 = ipred_z3_trn1_2d(v25, v25); // trn1 v25.2d, v25.2d, v25.2d: 64 - frac

    let v21 = vdupq_n_u8(4); // movi v21.16b, #4
    loop {
        // 1:
        let v4 = v31; // mov v4.16b, v31.16b
        let v5 = v31; // mov v5.16b, v31.16b
        let v4 = vqtbx2q_u8(v4, v0v1, v26); // tbx v4.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v26.16b: left[base], left[base+2]
        let v5 = vqtbx2q_u8(v5, v0v1, v27); // tbx v5.16b, {v0.16b, v1.16b, v2.16b, v3.16b}, v27.16b: left[base+1], left[base+3]

        let v16 = vmull_u8(vget_low_u8(v4), vget_low_u8(v25)); // umull v16.8h, v4.8b, v25.8b: left[base]*(64-frac)
        let v16 = vmlal_u8(v16, vget_low_u8(v5), vget_low_u8(v24)); // umlal v16.8h, v5.8b, v24.8b: + left[base+1]*frac
        let v17 = vmull_high_u8(v4, v25); // umull2 v17.8h, v4.16b, v25.16b
        let v17 = vmlal_high_u8(v17, v5, v24); // umlal2 v17.8h, v5.16b, v24.16b
        let v16 = vrshrn_n_u16::<6>(v16); // rshrn v16.8b, v16.8h, #6
        let v17 = vrshrn_n_u16::<6>(v17); // rshrn v17.8b, v17.8h, #6
        vst1_u8(x0, v16); // st1 {v16.8b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        w4 -= 2; // subs w4, w4, #2
        vst1_u8(x0, v17); // st1 {v17.8b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        if w4 <= 0 {
            break; // b.le 9f
        }

        v26 = vqaddq_u8(v26, v21); // uqadd v26.16b, v26.16b, v21.16b: base += 4
        v27 = vqaddq_u8(v27, v21); // uqadd v27.16b, v27.16b, v21.16b: base += 4
    }
}

// ============================================================================
// Part D (ipred.S lines 3686–5232): the filter predictor, the palette predictor and chroma from luma.
// ============================================================================
// Correspondence with the assembly (this part): the vector registers are typed locals named after the
// registers (`v0`, `v16`, ...); a register that the assembly reuses for a value of another width is a new
// binding of the same name. General registers keep their assembly names (`x0`, `w4`, ...). A post-incremented
// address is an explicit `wrapping_add`/`wrapping_offset` after the access, so that the address after the last
// access is never formed with `add`. Each jump table is a `match`; the arms carry the label names. A branch
// into a block that several entry points share (`L(ipred_cfl_splat_w*)`, `L(ipred_cfl_ac_420_w*_hpad)`,
// `L(ipred_cfl_ac_420_w8_calc_subtract_dc)`, `L(ipred_cfl_ac_420_w8_subtract_dc)`) is a call in tail position
// of the function with the label's name.

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

/// `ldur s, [p]` / `ld1 {v.s}[0], [p]`: four bytes into the low 32-bit lane, the rest of the register
/// cleared (where the assembly inserts into a stale register, only the bytes loaded here are used; where it
/// clears the other lane with `ins v.s[1], wzr`, this is that clearing).
#[inline(always)]
unsafe fn pd_ld1_s(p: *const u8) -> uint8x8_t {
    vreinterpret_u8_u32(vset_lane_u32::<0>(p.cast::<u32>().read_unaligned(), vdup_n_u32(0)))
}

/// `ld1 {v.s}[0], [lo]` then `ld1 {v.s}[1], [hi]`: two rows of four bytes in one register.
#[inline(always)]
unsafe fn pd_ld1_s2(lo: *const u8, hi: *const u8) -> uint8x8_t {
    let v = vset_lane_u32::<0>(lo.cast::<u32>().read_unaligned(), vdup_n_u32(0));
    vreinterpret_u8_u32(vset_lane_u32::<1>(hi.cast::<u32>().read_unaligned(), v))
}

/// `ld1 {v.8b}, [lo]` then `ld1 {v.d}[1], [hi]`: two rows of eight bytes in one register.
#[inline(always)]
unsafe fn pd_ld1_d2(lo: *const u8, hi: *const u8) -> uint8x16_t {
    let v = vreinterpretq_u64_u8(vcombine_u8(vld1_u8(lo), vdup_n_u8(0)));
    vreinterpretq_u8_u64(vsetq_lane_u64::<1>(hi.cast::<u64>().read_unaligned(), v))
}

/// `ld1r {v.2s}, [p]`
#[inline(always)]
unsafe fn pd_ld1r_2s(p: *const u8) -> uint8x8_t {
    vreinterpret_u8_u32(vdup_n_u32(p.cast::<u32>().read_unaligned()))
}

/// `st1 {v.s}[LANE], [p]` of a 64-bit register.
#[inline(always)]
unsafe fn pd_st1_s<const LANE: i32>(p: *mut u8, a: uint8x8_t) {
    p.cast::<u32>()
        .write_unaligned(vget_lane_u32::<LANE>(vreinterpret_u32_u8(a)));
}

/// `st1 {v.s}[LANE], [p]`
#[inline(always)]
unsafe fn pd_st1q_s<const LANE: i32>(p: *mut u8, a: uint8x16_t) {
    p.cast::<u32>()
        .write_unaligned(vgetq_lane_u32::<LANE>(vreinterpretq_u32_u8(a)));
}

/// `st1 {v.d}[LANE], [p]`
#[inline(always)]
unsafe fn pd_st1q_d<const LANE: i32>(p: *mut u8, a: uint8x16_t) {
    p.cast::<u64>()
        .write_unaligned(vgetq_lane_u64::<LANE>(vreinterpretq_u64_u8(a)));
}

/// `uxtl vd.8h, vn.8b`, typed for the signed multiplications of the filter and written as `zip1` with a zero
/// register: the compiler rewrites a multiplication by a lane of a register it knows to be a zero extension
/// (`mul vd.8h, vn.8h, vm.h[i]`) into `dup` + `ushll` + `mul`, three instructions for one, and does not when
/// the widening is an interleave with zeros (the same bytes on little endian).
#[inline(always)]
unsafe fn pd_uxtl_s16(a: uint8x8_t) -> int16x8_t {
    vreinterpretq_s16_u8(vzip1q_u8(vcombine_u8(a, vdup_n_u8(0)), vdupq_n_u8(0)))
}

/// `uxtl vd.8h, vn.8b` of a block the same loop iteration has just narrowed (`sqrshrun`), whose lanes then
/// multiply (`mla vd.8h, vn.8h, vm.h[3]`): written as a polynomial multiplication by one (`pmull vd.8h, vn.8b,
/// one.8b`, the identity, widened), one instruction as `uxtl`. The compiler sees through `pd_uxtl_s16` there
/// and replaces each such `mla` by `dup` + `ushll` + `mla`; it has no rule for `pmull`.
#[inline(always)]
unsafe fn pd_uxtl_pmull(a: uint8x8_t) -> int16x8_t {
    vreinterpretq_s16_p16(vmull_p8(vreinterpret_p8_u8(a), vdup_n_p8(1)))
}

/// `uxtl vd.8h, vn.8b` (`HI` false) or `uxtl2 vd.8h, vn.16b` (`HI` true) of a 128-bit register, as
/// `pd_uxtl_s16`.
#[inline(always)]
unsafe fn pd_uxtlq_s16<const HI: bool>(a: uint8x16_t) -> int16x8_t {
    let z = vdupq_n_u8(0);
    vreinterpretq_s16_u8(if HI { vzip2q_u8(a, z) } else { vzip1q_u8(a, z) })
}

/// `uaddlv h, v.8b`: the sum as lane 0 of a `.4h` register (the other lanes, zero in the assembly, hold the
/// sum too; only lane 0 is used).
#[inline(always)]
unsafe fn pd_uaddlv_8b(a: uint8x8_t) -> uint16x4_t {
    vdup_n_u16(vaddlv_u8(a))
}

/// `uaddlv h, v.16b`: as `pd_uaddlv_8b`.
#[inline(always)]
unsafe fn pd_uaddlv_16b(a: uint8x16_t) -> uint16x4_t {
    vdup_n_u16(vaddlvq_u8(a))
}

// --- ipred_filter ----------------------------------------------------------------------------------------------

/// The address of the left pixels of `ipred_filter`: the assembly reads four bytes at `x2` = `topleft - 2` and
/// post-decrements `x2` by 2 (`x7`) per pair of rows. Here the address is `topleft - 2 - height` plus the rows
/// that remain (`w4`, read before its `subs`): the same addresses without a second counter.
#[inline(always)]
fn pd_filter_left(topleft: *const u8, height: c_int) -> *const u8 {
    topleft.wrapping_sub(2).wrapping_sub(height as usize)
}

/// `dav1d_ipred_filter_8bpc_neon`: the recursive filter predictor.
///
/// Safety: reads 56 bytes of `dav1d_filter_intra_taps[filt_idx & 511]` (`filt_idx & 511 < 5` by the caller's
/// contract), the edge bytes `topleft[-height .. width + 1]` (w 4 reads `topleft[1..5]` and four bytes at
/// `topleft - 2 - 2k` for each row pair `k`; w 8 the same with `topleft[1..9]`; w 16/32 `topleft[1 .. width + 1]`
/// in steps of 16 and the same four bytes per row pair) and, for w 16/32, the row `2k + 1` of `dst` it wrote
/// before; writes `dst` rows `0..height`, columns `0..width`.
pub unsafe extern "C" fn dav1d_ipred_filter_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    filt_idx: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let (mut x0, mut x1, x2, w3, mut w4) = (dst, stride, topleft, width, height);
    let w5 = (filt_idx & 511) as usize; // and w5, w5, #511
    let x6 = core::ptr::addr_of!(crate::common::tables::dav1d_filter_intra_taps).cast::<i8>(); // movrel x6, X(filter_intra_taps)
    let x6 = x6.add(w5 << 6); // lsl w5, w5, #6; add x6, x6, w5, uxtw
    let t = vld1_s8_x4(x6); // ld1 {v16.8b, v17.8b, v18.8b, v19.8b}, [x6], #32
    let u = vld1_s8_x3(x6.add(32)); // ld1 {v20.8b, v21.8b, v22.8b}, [x6]
    let v16 = vmovl_s8(t.0); // sxtl v16.8h, v16.8b
    let v17 = vmovl_s8(t.1); // sxtl v17.8h, v17.8b
    let v18 = vmovl_s8(t.2); // sxtl v18.8h, v18.8b
    let v19 = vmovl_s8(t.3); // sxtl v19.8h, v19.8b
    let mut x6 = x0.offset(x1); // add x6, x0, x1
    x1 <<= 1; // lsl x1, x1, #1
    let v20 = vmovl_s8(u.0); // sxtl v20.8h, v20.8b
    let v21 = vmovl_s8(u.1); // sxtl v21.8h, v21.8b
    let v22 = vmovl_s8(u.2); // sxtl v22.8h, v22.8b

    // br x5 (ipred_filter_tbl, by the width)
    match width {
        4 => {
            // 40:
            let mut v0 = pd_uxtl_s16(pd_ld1_s(x2.add(1))); // ldur s0, [x2, #1]; uxtl v0.8h, v0.8b: top (0-3)
            let x2 = pd_filter_left(x2, w4); // sub x2, x2, #2; mov x7, #-2
            loop {
                // 4:
                let v1 = pd_ld1_s(x2.wrapping_add(w4 as usize)); // ld1 {v1.s}[0], [x2], x7: left (0-1) + topleft (2)
                let mut v2 = vmulq_laneq_s16::<0>(v17, v0); // mul v2.8h, v17.8h, v0.h[0]: p1(top[0]) * filter(1)
                v2 = vmlaq_laneq_s16::<1>(v2, v18, v0); // mla v2.8h, v18.8h, v0.h[1]: p2(top[1]) * filter(2)
                v2 = vmlaq_laneq_s16::<2>(v2, v19, v0); // mla v2.8h, v19.8h, v0.h[2]: p3(top[2]) * filter(3)
                let v1 = pd_uxtl_s16(v1); // uxtl v1.8h, v1.8b
                v2 = vmlaq_laneq_s16::<3>(v2, v20, v0); // mla v2.8h, v20.8h, v0.h[3]: p4(top[3]) * filter(4)
                v2 = vmlaq_laneq_s16::<2>(v2, v16, v1); // mla v2.8h, v16.8h, v1.h[2]: p0(topleft) * filter(0)
                v2 = vmlaq_laneq_s16::<1>(v2, v21, v1); // mla v2.8h, v21.8h, v1.h[1]: p5(left[0]) * filter(5)
                v2 = vmlaq_laneq_s16::<0>(v2, v22, v1); // mla v2.8h, v22.8h, v1.h[0]: p6(left[1]) * filter(6)
                let v2 = vqrshrun_n_s16::<4>(v2); // sqrshrun v2.8b, v2.8h, #4
                let gt = pd_subs_gt(&mut w4, 2); // subs w4, w4, #2
                pd_st1_s::<0>(x0, v2); // st1 {v2.s}[0], [x0], x1
                x0 = x0.wrapping_offset(x1);
                let v2 = vext_u8::<4>(v2, v2); // ext v0.16b, v0.16b, v0.16b, #8 (before the uxtl, on bytes): move top from [4-7] to [0-3]
                pd_st1_s::<0>(x6, v2); // st1 {v2.s}[1], [x6], x1
                x6 = x6.wrapping_offset(x1);
                v0 = pd_uxtl_s16(v2); // uxtl v0.8h, v2.8b
                if !gt {
                    break; // b.gt 4b
                }
            }
        }
        8 => {
            // 80:
            let mut v0 = pd_uxtl_s16(vld1_u8(x2.add(1))); // ldur d0, [x2, #1]; uxtl v0.8h, v0.8b: top (0-7)
            let x2 = pd_filter_left(x2, w4); // sub x2, x2, #2; mov x7, #-2
            loop {
                // 8:
                let v1 = pd_ld1_s(x2.wrapping_add(w4 as usize)); // ld1 {v1.s}[0], [x2], x7: left (0-1) + topleft (2)
                let mut v2 = vmulq_laneq_s16::<0>(v17, v0); // mul v2.8h, v17.8h, v0.h[0]: p1(top[0]) * filter(1)
                v2 = vmlaq_laneq_s16::<1>(v2, v18, v0); // mla v2.8h, v18.8h, v0.h[1]: p2(top[1]) * filter(2)
                v2 = vmlaq_laneq_s16::<2>(v2, v19, v0); // mla v2.8h, v19.8h, v0.h[2]: p3(top[2]) * filter(3)
                let v1 = pd_uxtl_s16(v1); // uxtl v1.8h, v1.8b
                v2 = vmlaq_laneq_s16::<3>(v2, v20, v0); // mla v2.8h, v20.8h, v0.h[3]: p4(top[3]) * filter(4)
                v2 = vmlaq_laneq_s16::<2>(v2, v16, v1); // mla v2.8h, v16.8h, v1.h[2]: p0(topleft) * filter(0)
                v2 = vmlaq_laneq_s16::<1>(v2, v21, v1); // mla v2.8h, v21.8h, v1.h[1]: p5(left[0]) * filter(5)
                v2 = vmlaq_laneq_s16::<0>(v2, v22, v1); // mla v2.8h, v22.8h, v1.h[0]: p6(left[1]) * filter(6)
                let mut v3 = vmulq_laneq_s16::<4>(v17, v0); // mul v3.8h, v17.8h, v0.h[4]: p1(top[0]) * filter(1)
                v3 = vmlaq_laneq_s16::<5>(v3, v18, v0); // mla v3.8h, v18.8h, v0.h[5]: p2(top[1]) * filter(2)
                v3 = vmlaq_laneq_s16::<6>(v3, v19, v0); // mla v3.8h, v19.8h, v0.h[6]: p3(top[2]) * filter(3)
                let v2 = vqrshrun_n_s16::<4>(v2); // sqrshrun v2.8b, v2.8h, #4
                let v1 = pd_uxtl_pmull(v2); // uxtl v1.8h, v2.8b: first block, in 16 bit
                v3 = vmlaq_laneq_s16::<7>(v3, v20, v0); // mla v3.8h, v20.8h, v0.h[7]: p4(top[3]) * filter(4)
                v3 = vmlaq_laneq_s16::<3>(v3, v16, v0); // mla v3.8h, v16.8h, v0.h[3]: p0(topleft) * filter(0)
                v3 = vmlaq_laneq_s16::<3>(v3, v21, v1); // mla v3.8h, v21.8h, v1.h[3]: p5(left[0]) * filter(5)
                v3 = vmlaq_laneq_s16::<7>(v3, v22, v1); // mla v3.8h, v22.8h, v1.h[7]: p6(left[1]) * filter(6)
                let v3 = vqrshrun_n_s16::<4>(v3); // sqrshrun v3.8b, v3.8h, #4
                let gt = pd_subs_gt(&mut w4, 2); // subs w4, w4, #2
                let v23 = uint32x2x2_t(vreinterpret_u32_u8(v2), vreinterpret_u32_u8(v3));
                vst2_lane_u32::<0>(x0.cast(), v23); // st2 {v2.s, v3.s}[0], [x0], x1
                x0 = x0.wrapping_offset(x1);
                let v0n = vzip2_u32(v23.0, v23.1); // zip2 v0.2s, v2.2s, v3.2s
                vst2_lane_u32::<1>(x6.cast(), v23); // st2 {v2.s, v3.s}[1], [x6], x1
                x6 = x6.wrapping_offset(x1);
                v0 = pd_uxtl_s16(vreinterpret_u8_u32(v0n)); // uxtl v0.8h, v0.8b
                if !gt {
                    break; // b.gt 8b
                }
            }
        }
        _ => ipred_filter_w16_8bpc_neon(x0, x1, x2, w3, w4, x6, v16, v17, v18, v19, v20, v21, v22), // 160: 320:
    }
}

/// The blocks `160:`/`320:` of `ipred_filter_8bpc_neon` (w 16 and 32), with the entry point's registers: `x1` twice
/// the stride, `x6` = `x0` + stride, `v16`-`v22` the taps.
unsafe fn ipred_filter_w16_8bpc_neon(
    mut x0: *mut u8,
    mut x1: isize,
    x2: *const u8,
    mut w3: c_int,
    mut w4: c_int,
    mut x6: *mut u8,
    v16: int16x8_t,
    v17: int16x8_t,
    v18: int16x8_t,
    v19: int16x8_t,
    v20: int16x8_t,
    v21: int16x8_t,
    v22: int16x8_t,
) {
    let mut x8 = x2.add(1); // add x8, x2, #1
    let x2 = pd_filter_left(x2, w4); // sub x2, x2, #2; mov x7, #-2
    x1 -= w3 as isize; // sub x1, x1, w3, uxtw
    let w9 = w3; // mov w9, w3
    loop {
        // 1:
        let mut v0 = pd_uxtl_s16(pd_ld1_s(x2.wrapping_add(w4 as usize))); // ld1 {v0.s}[0], [x2], x7; uxtl v0.8h, v0.8b: left (0-1) + topleft (2)
        loop {
            // 2:
            let v2 = vld1q_u8(x8); // ld1 {v2.16b}, [x8], #16: top(0-15)
            x8 = x8.wrapping_add(16);
            let mut v3 = vmulq_laneq_s16::<2>(v16, v0); // mul v3.8h, v16.8h, v0.h[2]: p0(topleft) * filter(0)
            v3 = vmlaq_laneq_s16::<1>(v3, v21, v0); // mla v3.8h, v21.8h, v0.h[1]: p5(left[0]) * filter(5)
            let v1 = pd_uxtlq_s16::<false>(v2); // uxtl v1.8h, v2.8b: top(0-7)
            let v2 = pd_uxtlq_s16::<true>(v2); // uxtl2 v2.8h, v2.16b: top(8-15)
            v3 = vmlaq_laneq_s16::<0>(v3, v22, v0); // mla v3.8h, v22.8h, v0.h[0]: p6(left[1]) * filter(6)
            v3 = vmlaq_laneq_s16::<0>(v3, v17, v1); // mla v3.8h, v17.8h, v1.h[0]: p1(top[0]) * filter(1)
            v3 = vmlaq_laneq_s16::<1>(v3, v18, v1); // mla v3.8h, v18.8h, v1.h[1]: p2(top[1]) * filter(2)
            v3 = vmlaq_laneq_s16::<2>(v3, v19, v1); // mla v3.8h, v19.8h, v1.h[2]: p3(top[2]) * filter(3)
            v3 = vmlaq_laneq_s16::<3>(v3, v20, v1); // mla v3.8h, v20.8h, v1.h[3]: p4(top[3]) * filter(4)

            let mut v4 = vmulq_laneq_s16::<4>(v17, v1); // mul v4.8h, v17.8h, v1.h[4]: p1(top[0]) * filter(1)
            v4 = vmlaq_laneq_s16::<5>(v4, v18, v1); // mla v4.8h, v18.8h, v1.h[5]: p2(top[1]) * filter(2)
            v4 = vmlaq_laneq_s16::<6>(v4, v19, v1); // mla v4.8h, v19.8h, v1.h[6]: p3(top[2]) * filter(3)
            let v3 = vqrshrun_n_s16::<4>(v3); // sqrshrun v3.8b, v3.8h, #4
            v0 = pd_uxtl_pmull(v3); // uxtl v0.8h, v3.8b: first block, in 16 bit
            v4 = vmlaq_laneq_s16::<7>(v4, v20, v1); // mla v4.8h, v20.8h, v1.h[7]: p4(top[3]) * filter(4)
            v4 = vmlaq_laneq_s16::<3>(v4, v16, v1); // mla v4.8h, v16.8h, v1.h[3]: p0(topleft) * filter(0)
            v4 = vmlaq_laneq_s16::<3>(v4, v21, v0); // mla v4.8h, v21.8h, v0.h[3]: p5(left[0]) * filter(5)
            v4 = vmlaq_laneq_s16::<7>(v4, v22, v0); // mla v4.8h, v22.8h, v0.h[7]: p6(left[1]) * filter(6)

            let mut v5 = vmulq_laneq_s16::<0>(v17, v2); // mul v5.8h, v17.8h, v2.h[0]: p1(top[0]) * filter(1)
            v5 = vmlaq_laneq_s16::<1>(v5, v18, v2); // mla v5.8h, v18.8h, v2.h[1]: p2(top[1]) * filter(2)
            v5 = vmlaq_laneq_s16::<2>(v5, v19, v2); // mla v5.8h, v19.8h, v2.h[2]: p3(top[2]) * filter(3)
            let v4 = vqrshrun_n_s16::<4>(v4); // sqrshrun v4.8b, v4.8h, #4
            v0 = pd_uxtl_pmull(v4); // uxtl v0.8h, v4.8b: second block, in 16 bit
            v5 = vmlaq_laneq_s16::<3>(v5, v20, v2); // mla v5.8h, v20.8h, v2.h[3]: p4(top[3]) * filter(4)
            v5 = vmlaq_laneq_s16::<7>(v5, v16, v1); // mla v5.8h, v16.8h, v1.h[7]: p0(topleft) * filter(0)
            v5 = vmlaq_laneq_s16::<3>(v5, v21, v0); // mla v5.8h, v21.8h, v0.h[3]: p5(left[0]) * filter(5)
            v5 = vmlaq_laneq_s16::<7>(v5, v22, v0); // mla v5.8h, v22.8h, v0.h[7]: p6(left[1]) * filter(6)

            let mut v6 = vmulq_laneq_s16::<4>(v17, v2); // mul v6.8h, v17.8h, v2.h[4]: p1(top[0]) * filter(1)
            v6 = vmlaq_laneq_s16::<5>(v6, v18, v2); // mla v6.8h, v18.8h, v2.h[5]: p2(top[1]) * filter(2)
            v6 = vmlaq_laneq_s16::<6>(v6, v19, v2); // mla v6.8h, v19.8h, v2.h[6]: p3(top[2]) * filter(3)
            let v5 = vqrshrun_n_s16::<4>(v5); // sqrshrun v5.8b, v5.8h, #4
            v0 = pd_uxtl_pmull(v5); // uxtl v0.8h, v5.8b: third block, in 16 bit
            v6 = vmlaq_laneq_s16::<7>(v6, v20, v2); // mla v6.8h, v20.8h, v2.h[7]: p4(top[3]) * filter(4)
            v6 = vmlaq_laneq_s16::<3>(v6, v16, v2); // mla v6.8h, v16.8h, v2.h[3]: p0(topleft) * filter(0)
            v6 = vmlaq_laneq_s16::<3>(v6, v21, v0); // mla v6.8h, v21.8h, v0.h[3]: p5(left[0]) * filter(5)
            v6 = vmlaq_laneq_s16::<7>(v6, v22, v0); // mla v6.8h, v22.8h, v0.h[7]: p6(left[1]) * filter(6)

            let gt = pd_subs_gt(&mut w3, 16); // subs w3, w3, #16
            let v6 = vqrshrun_n_s16::<4>(v6); // sqrshrun v6.8b, v6.8h, #4

            let v3456 = uint32x2x4_t(
                vreinterpret_u32_u8(v3),
                vreinterpret_u32_u8(v4),
                vreinterpret_u32_u8(v5),
                vreinterpret_u32_u8(v6),
            );
            vst4_lane_u32::<0>(x0.cast(), v3456); // st4 {v3.s, v4.s, v5.s, v6.s}[0], [x0], #16
            x0 = x0.wrapping_add(16);
            vst4_lane_u32::<1>(x6.cast(), v3456); // st4 {v3.s, v4.s, v5.s, v6.s}[1], [x6], #16
            x6 = x6.wrapping_add(16);
            if !gt {
                break; // b.le 8f
            }
            v0 = vcopyq_laneq_s16::<2, 7>(v0, v2); // ins v0.h[2], v2.h[7]
            let v0b = vreinterpretq_u8_s16(v0);
            let v0b = vcopyq_lane_u8::<0, 7>(v0b, v6); // ins v0.b[0], v6.b[7]
            let v0b = vcopyq_lane_u8::<2, 3>(v0b, v6); // ins v0.b[2], v6.b[3]
            v0 = vreinterpretq_s16_u8(v0b);
            // b 2b
        }
        // 8:
        let gt = pd_subs_gt(&mut w4, 2); // subs w4, w4, #2
        if !gt {
            break; // b.le 9f
        }
        x8 = x6.sub(w9 as usize); // sub x8, x6, w9, uxtw
        x0 = x0.offset(x1); // add x0, x0, x1
        x6 = x6.offset(x1); // add x6, x6, x1
        w3 = w9; // mov w3, w9

        // b 1b
    }
    // 9:
}

// --- pal_pred --------------------------------------------------------------------------------------------------

/// `dav1d_pal_pred_8bpc_neon`: the palette predictor.
///
/// Safety: reads `pal[0..8]` and exactly the `w * h / 2` bytes of the packed index buffer `idx` (two 3-bit
/// indices per byte, the low nibble first), in order and without alignment requirement: 8 (w 4), 16 (w 8),
/// 32 (w 16) or 64 (w 32) bytes per four rows, 64 bytes per two rows at w 64; writes `dst` rows `0..h`,
/// columns `0..w`.
pub unsafe extern "C" fn dav1d_pal_pred_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    pal: *const u8,
    idx: *const u8,
    w: c_int,
    h: c_int,
) {
    let (mut x0, mut x1, mut x3, mut w5) = (dst, stride, idx, h);
    let v0 = vcombine_u8(vld1_u8(pal), vdup_n_u8(0)); // ld1 {v0.8b}, [x2]
    let v31 = vdupq_n_u8(7); // movi v31.16b, #7
    let mut x2 = x0.offset(x1); // add x2, x0, x1
    x1 <<= 1; // lsl x1, x1, #1

    // br x6 (pal_pred_tbl, by the width)
    match pd_tbl_idx(w) {
        4 => loop {
            // 40: 4:
            let v1 = vld1_u8(x3); // ld1 {v1.8b}, [x3], #8
            x3 = x3.wrapping_add(8);
            let gt = pd_subs_gt(&mut w5, 4); // subs w5, w5, #4
            let v3 = vshr_n_u8::<4>(v1); // ushr v3.8b, v1.8b, #4
            let v2 = vand_u8(v1, vget_low_u8(v31)); // and v2.8b, v1.8b, v31.8b
            let v1 = vzip1q_u8(vcombine_u8(v2, vdup_n_u8(0)), vcombine_u8(v3, vdup_n_u8(0))); // zip1 v1.16b, v2.16b, v3.16b
            let v1 = vqtbl1q_u8(v0, v1); // tbl v1.16b, {v0.16b}, v1.16b
            pd_st1q_s::<0>(x0, v1); // st1 {v1.s}[0], [x0], x1
            x0 = x0.wrapping_offset(x1);
            pd_st1q_s::<1>(x2, v1); // st1 {v1.s}[1], [x2], x1
            x2 = x2.wrapping_offset(x1);
            pd_st1q_s::<2>(x0, v1); // st1 {v1.s}[2], [x0], x1
            x0 = x0.wrapping_offset(x1);
            pd_st1q_s::<3>(x2, v1); // st1 {v1.s}[3], [x2], x1
            x2 = x2.wrapping_offset(x1);
            if !gt {
                break; // b.gt 4b
            }
        },
        3 => loop {
            // 80: 8:
            let v1 = vld1q_u8(x3); // ld1 {v1.16b}, [x3], #16
            x3 = x3.wrapping_add(16);
            let gt = pd_subs_gt(&mut w5, 4); // subs w5, w5, #4
            let v4 = vshrq_n_u8::<4>(v1); // ushr v4.16b, v1.16b, #4
            let v3 = vandq_u8(v1, v31); // and v3.16b, v1.16b, v31.16b
            let v1 = vzip1q_u8(v3, v4); // zip1 v1.16b, v3.16b, v4.16b
            let v2 = vzip2q_u8(v3, v4); // zip2 v2.16b, v3.16b, v4.16b
            let v1 = vqtbl1q_u8(v0, v1); // tbl v1.16b, {v0.16b}, v1.16b
            pd_st1q_d::<0>(x0, v1); // st1 {v1.d}[0], [x0], x1
            x0 = x0.wrapping_offset(x1);
            let v2 = vqtbl1q_u8(v0, v2); // tbl v2.16b, {v0.16b}, v2.16b
            pd_st1q_d::<1>(x2, v1); // st1 {v1.d}[1], [x2], x1
            x2 = x2.wrapping_offset(x1);
            pd_st1q_d::<0>(x0, v2); // st1 {v2.d}[0], [x0], x1
            x0 = x0.wrapping_offset(x1);
            pd_st1q_d::<1>(x2, v2); // st1 {v2.d}[1], [x2], x1
            x2 = x2.wrapping_offset(x1);
            if !gt {
                break; // b.gt 8b
            }
        },
        2 => loop {
            // 160: 16:
            let v12 = vld1q_u8_x2(x3); // ld1 {v1.16b, v2.16b}, [x3], #32
            x3 = x3.wrapping_add(32);
            let gt = pd_subs_gt(&mut w5, 4); // subs w5, w5, #4
            let v5 = vshrq_n_u8::<4>(v12.0); // ushr v5.16b, v1.16b, #4
            let v4 = vandq_u8(v12.0, v31); // and v4.16b, v1.16b, v31.16b
            let v7 = vshrq_n_u8::<4>(v12.1); // ushr v7.16b, v2.16b, #4
            let v6 = vandq_u8(v12.1, v31); // and v6.16b, v2.16b, v31.16b
            let v1 = vzip1q_u8(v4, v5); // zip1 v1.16b, v4.16b, v5.16b
            let v2 = vzip2q_u8(v4, v5); // zip2 v2.16b, v4.16b, v5.16b
            let v3 = vzip1q_u8(v6, v7); // zip1 v3.16b, v6.16b, v7.16b
            let v1 = vqtbl1q_u8(v0, v1); // tbl v1.16b, {v0.16b}, v1.16b
            let v4 = vzip2q_u8(v6, v7); // zip2 v4.16b, v6.16b, v7.16b
            let v2 = vqtbl1q_u8(v0, v2); // tbl v2.16b, {v0.16b}, v2.16b
            vst1q_u8(x0, v1); // st1 {v1.16b}, [x0], x1
            x0 = x0.wrapping_offset(x1);
            let v3 = vqtbl1q_u8(v0, v3); // tbl v3.16b, {v0.16b}, v3.16b
            vst1q_u8(x2, v2); // st1 {v2.16b}, [x2], x1
            x2 = x2.wrapping_offset(x1);
            let v4 = vqtbl1q_u8(v0, v4); // tbl v4.16b, {v0.16b}, v4.16b
            vst1q_u8(x0, v3); // st1 {v3.16b}, [x0], x1
            x0 = x0.wrapping_offset(x1);
            vst1q_u8(x2, v4); // st1 {v4.16b}, [x2], x1
            x2 = x2.wrapping_offset(x1);
            if !gt {
                break; // b.gt 16b
            }
        },
        1 => loop {
            // 320: 32:
            let t = vld1q_u8_x4(x3); // ld1 {v16.16b, v17.16b, v18.16b, v19.16b}, [x3], #64
            x3 = x3.wrapping_add(64);
            let gt = pd_subs_gt(&mut w5, 4); // subs w5, w5, #4
            let v21 = vshrq_n_u8::<4>(t.0); // ushr v21.16b, v16.16b, #4
            let v20 = vandq_u8(t.0, v31); // and v20.16b, v16.16b, v31.16b
            let v23 = vshrq_n_u8::<4>(t.1); // ushr v23.16b, v17.16b, #4
            let v22 = vandq_u8(t.1, v31); // and v22.16b, v17.16b, v31.16b
            let v25 = vshrq_n_u8::<4>(t.2); // ushr v25.16b, v18.16b, #4
            let v24 = vandq_u8(t.2, v31); // and v24.16b, v18.16b, v31.16b
            let v27 = vshrq_n_u8::<4>(t.3); // ushr v27.16b, v19.16b, #4
            let v26 = vandq_u8(t.3, v31); // and v26.16b, v19.16b, v31.16b
            let v16 = vzip1q_u8(v20, v21); // zip1 v16.16b, v20.16b, v21.16b
            let v17 = vzip2q_u8(v20, v21); // zip2 v17.16b, v20.16b, v21.16b
            let v18 = vzip1q_u8(v22, v23); // zip1 v18.16b, v22.16b, v23.16b
            let v19 = vzip2q_u8(v22, v23); // zip2 v19.16b, v22.16b, v23.16b
            let v20 = vzip1q_u8(v24, v25); // zip1 v20.16b, v24.16b, v25.16b
            let v21 = vzip2q_u8(v24, v25); // zip2 v21.16b, v24.16b, v25.16b
            let v16 = vqtbl1q_u8(v0, v16); // tbl v16.16b, {v0.16b}, v16.16b
            let v22 = vzip1q_u8(v26, v27); // zip1 v22.16b, v26.16b, v27.16b
            let v17 = vqtbl1q_u8(v0, v17); // tbl v17.16b, {v0.16b}, v17.16b
            let v23 = vzip2q_u8(v26, v27); // zip2 v23.16b, v26.16b, v27.16b
            let v18 = vqtbl1q_u8(v0, v18); // tbl v18.16b, {v0.16b}, v18.16b
            let v19 = vqtbl1q_u8(v0, v19); // tbl v19.16b, {v0.16b}, v19.16b
            let v20 = vqtbl1q_u8(v0, v20); // tbl v20.16b, {v0.16b}, v20.16b
            vst1q_u8_x2(x0, uint8x16x2_t(v16, v17)); // st1 {v16.16b, v17.16b}, [x0], x1
            x0 = x0.wrapping_offset(x1);
            let v21 = vqtbl1q_u8(v0, v21); // tbl v21.16b, {v0.16b}, v21.16b
            vst1q_u8_x2(x2, uint8x16x2_t(v18, v19)); // st1 {v18.16b, v19.16b}, [x2], x1
            x2 = x2.wrapping_offset(x1);
            let v22 = vqtbl1q_u8(v0, v22); // tbl v22.16b, {v0.16b}, v22.16b
            vst1q_u8_x2(x0, uint8x16x2_t(v20, v21)); // st1 {v20.16b, v21.16b}, [x0], x1
            x0 = x0.wrapping_offset(x1);
            let v23 = vqtbl1q_u8(v0, v23); // tbl v23.16b, {v0.16b}, v23.16b
            vst1q_u8_x2(x2, uint8x16x2_t(v22, v23)); // st1 {v22.16b, v23.16b}, [x2], x1
            x2 = x2.wrapping_offset(x1);
            if !gt {
                break; // b.gt 32b
            }
        },
        0 => loop {
            // 640: 64:
            let t = vld1q_u8_x4(x3); // ld1 {v16.16b, v17.16b, v18.16b, v19.16b}, [x3], #64
            x3 = x3.wrapping_add(64);
            let gt = pd_subs_gt(&mut w5, 2); // subs w5, w5, #2
            let v21 = vshrq_n_u8::<4>(t.0); // ushr v21.16b, v16.16b, #4
            let v20 = vandq_u8(t.0, v31); // and v20.16b, v16.16b, v31.16b
            let v23 = vshrq_n_u8::<4>(t.1); // ushr v23.16b, v17.16b, #4
            let v22 = vandq_u8(t.1, v31); // and v22.16b, v17.16b, v31.16b
            let v25 = vshrq_n_u8::<4>(t.2); // ushr v25.16b, v18.16b, #4
            let v24 = vandq_u8(t.2, v31); // and v24.16b, v18.16b, v31.16b
            let v27 = vshrq_n_u8::<4>(t.3); // ushr v27.16b, v19.16b, #4
            let v26 = vandq_u8(t.3, v31); // and v26.16b, v19.16b, v31.16b
            let v16 = vzip1q_u8(v20, v21); // zip1 v16.16b, v20.16b, v21.16b
            let v17 = vzip2q_u8(v20, v21); // zip2 v17.16b, v20.16b, v21.16b
            let v18 = vzip1q_u8(v22, v23); // zip1 v18.16b, v22.16b, v23.16b
            let v19 = vzip2q_u8(v22, v23); // zip2 v19.16b, v22.16b, v23.16b
            let v20 = vzip1q_u8(v24, v25); // zip1 v20.16b, v24.16b, v25.16b
            let v21 = vzip2q_u8(v24, v25); // zip2 v21.16b, v24.16b, v25.16b
            let v16 = vqtbl1q_u8(v0, v16); // tbl v16.16b, {v0.16b}, v16.16b
            let v22 = vzip1q_u8(v26, v27); // zip1 v22.16b, v26.16b, v27.16b
            let v17 = vqtbl1q_u8(v0, v17); // tbl v17.16b, {v0.16b}, v17.16b
            let v23 = vzip2q_u8(v26, v27); // zip2 v23.16b, v26.16b, v27.16b
            let v18 = vqtbl1q_u8(v0, v18); // tbl v18.16b, {v0.16b}, v18.16b
            let v19 = vqtbl1q_u8(v0, v19); // tbl v19.16b, {v0.16b}, v19.16b
            vst1q_u8_x4(x0, uint8x16x4_t(v16, v17, v18, v19)); // st1 {v16.16b, v17.16b, v18.16b, v19.16b}, [x0], x1
            x0 = x0.wrapping_offset(x1);
            let v20 = vqtbl1q_u8(v0, v20); // tbl v20.16b, {v0.16b}, v20.16b
            let v21 = vqtbl1q_u8(v0, v21); // tbl v21.16b, {v0.16b}, v21.16b
            let v22 = vqtbl1q_u8(v0, v22); // tbl v22.16b, {v0.16b}, v22.16b
            let v23 = vqtbl1q_u8(v0, v23); // tbl v23.16b, {v0.16b}, v23.16b
            vst1q_u8_x4(x2, uint8x16x4_t(v20, v21, v22, v23)); // st1 {v20.16b, v21.16b, v22.16b, v23.16b}, [x2], x1
            x2 = x2.wrapping_offset(x1);
            if !gt {
                break; // b.gt 64b
            }
        },
        // SAFETY: the width is 4, 8, 16, 32 or 64 (the assembly indexes its five-entry table with it).
        _ => core::hint::unreachable_unchecked(),
    }
}

// --- ipred_cfl -------------------------------------------------------------------------------------------------

/// One register of the splat loops: `mul`, `cmlt #0`, `add`, `srshr #6`, `add` (dc), `sqxtun`, that is
/// `iclip_pixel(dc + apply_sign((abs(diff) + 32) >> 6, diff))` with `diff = ac * alpha`.
#[inline(always)]
unsafe fn ipred_cfl_splat_px(ac: int16x8_t, v0: int16x8_t, v1: int16x8_t) -> uint8x8_t {
    let v2 = vmulq_s16(ac, v1); // mul v2.8h, v2.8h, v1.8h: diff = ac * alpha
    let v4 = vreinterpretq_s16_u16(vcltzq_s16(v2)); // cmlt v4.8h, v2.8h, #0: sign
    let v2 = vaddq_s16(v2, v4); // add v2.8h, v2.8h, v4.8h: diff + sign
    let v2 = vrshrq_n_s16::<6>(v2); // srshr v2.8h, v2.8h, #6: (diff + sign + 32) >> 6 = apply_sign()
    let v2 = vaddq_s16(v2, v0); // add v2.8h, v2.8h, v0.8h: dc + apply_sign()
    vqmovun_s16(v2) // sqxtun v2.8b, v2.8h: iclip_pixel(dc + apply_sign())
}

/// `L(ipred_cfl_splat_w4)` of `ipred_cfl_128_8bpc_neon`; `v0` is the dc, `v1` alpha, `x6` = `x0` + stride and
/// `x1` twice the stride.
unsafe fn ipred_cfl_splat_w4_8bpc_neon(
    mut x0: *mut u8,
    x1: isize,
    mut x6: *mut u8,
    mut x5: *const i16,
    mut w4: c_int,
    v0: int16x8_t,
    v1: int16x8_t,
) {
    loop {
        // 1:
        let v23 = vld1q_s16_x2(x5); // ld1 {v2.8h, v3.8h}, [x5], #32
        x5 = x5.wrapping_add(16);
        let v2 = ipred_cfl_splat_px(v23.0, v0, v1);
        let v3 = ipred_cfl_splat_px(v23.1, v0, v1);
        pd_st1_s::<0>(x0, v2); // st1 {v2.s}[0], [x0], x1
        x0 = x0.wrapping_offset(x1);
        pd_st1_s::<1>(x6, v2); // st1 {v2.s}[1], [x6], x1
        x6 = x6.wrapping_offset(x1);
        let gt = pd_subs_gt(&mut w4, 4); // subs w4, w4, #4
        pd_st1_s::<0>(x0, v3); // st1 {v3.s}[0], [x0], x1
        x0 = x0.wrapping_offset(x1);
        pd_st1_s::<1>(x6, v3); // st1 {v3.s}[1], [x6], x1
        x6 = x6.wrapping_offset(x1);
        if !gt {
            break; // b.gt 1b
        }
    }
}

/// `L(ipred_cfl_splat_w8)` of `ipred_cfl_128_8bpc_neon`.
unsafe fn ipred_cfl_splat_w8_8bpc_neon(
    mut x0: *mut u8,
    x1: isize,
    mut x6: *mut u8,
    mut x5: *const i16,
    mut w4: c_int,
    v0: int16x8_t,
    v1: int16x8_t,
) {
    loop {
        // 1:
        let t = vld1q_s16_x4(x5); // ld1 {v2.8h, v3.8h, v4.8h, v5.8h}, [x5], #64
        x5 = x5.wrapping_add(32);
        let v2 = ipred_cfl_splat_px(t.0, v0, v1);
        let v3 = ipred_cfl_splat_px(t.1, v0, v1);
        let v4 = ipred_cfl_splat_px(t.2, v0, v1);
        let v5 = ipred_cfl_splat_px(t.3, v0, v1);
        vst1_u8(x0, v2); // st1 {v2.8b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        vst1_u8(x6, v3); // st1 {v3.8b}, [x6], x1
        x6 = x6.wrapping_offset(x1);
        let gt = pd_subs_gt(&mut w4, 4); // subs w4, w4, #4
        vst1_u8(x0, v4); // st1 {v4.8b}, [x0], x1
        x0 = x0.wrapping_offset(x1);
        vst1_u8(x6, v5); // st1 {v5.8b}, [x6], x1
        x6 = x6.wrapping_offset(x1);
        if !gt {
            break; // b.gt 1b
        }
    }
}

/// `L(ipred_cfl_splat_w16)` of `ipred_cfl_128_8bpc_neon` (w 16 and w 32).
unsafe fn ipred_cfl_splat_w16_8bpc_neon(
    mut x0: *mut u8,
    mut x1: isize,
    mut x6: *mut u8,
    mut w3: c_int,
    mut w4: c_int,
    mut x5: *const i16,
    v0: int16x8_t,
    v1: int16x8_t,
) {
    let mut x7 = x5.add(w3 as usize); // add x7, x5, w3, uxtw #1
    x1 -= w3 as isize; // sub x1, x1, w3, uxtw
    let w9 = w3; // mov w9, w3
    loop {
        // 1:
        let v23 = vld1q_s16_x2(x5); // ld1 {v2.8h, v3.8h}, [x5], #32
        x5 = x5.wrapping_add(16);
        let v45 = vld1q_s16_x2(x7); // ld1 {v4.8h, v5.8h}, [x7], #32
        x7 = x7.wrapping_add(16);
        let v2 = ipred_cfl_splat_px(v23.0, v0, v1);
        let v3 = ipred_cfl_splat_px(v23.1, v0, v1);
        let v4 = ipred_cfl_splat_px(v45.0, v0, v1);
        let v5 = ipred_cfl_splat_px(v45.1, v0, v1);
        let gt = pd_subs_gt(&mut w3, 16); // subs w3, w3, #16
        vst1_u8_x2(x0, uint8x8x2_t(v2, v3)); // st1 {v2.8b, v3.8b}, [x0], #16
        x0 = x0.wrapping_add(16);
        vst1_u8_x2(x6, uint8x8x2_t(v4, v5)); // st1 {v4.8b, v5.8b}, [x6], #16
        x6 = x6.wrapping_add(16);
        if gt {
            continue; // b.gt 1b
        }
        let gt = pd_subs_gt(&mut w4, 2); // subs w4, w4, #2
        x5 = x5.wrapping_add(w9 as usize); // add x5, x5, w9, uxtw #1
        x7 = x7.wrapping_add(w9 as usize); // add x7, x7, w9, uxtw #1
        x0 = x0.wrapping_offset(x1); // add x0, x0, x1
        x6 = x6.wrapping_offset(x1); // add x6, x6, x1
        w3 = w9; // mov w3, w9
        if !gt {
            break; // b.gt 1b
        }
    }
}

/// `br x9` with `ipred_cfl_splat_tbl` (`ipred_cfl_128_tbl`), by the width.
#[inline(always)]
unsafe fn ipred_cfl_splat_tbl_8bpc(
    x0: *mut u8,
    x1: isize,
    x6: *mut u8,
    w3: c_int,
    w4: c_int,
    x5: *const i16,
    v0: int16x8_t,
    v1: int16x8_t,
) {
    match w3 {
        4 => ipred_cfl_splat_w4_8bpc_neon(x0, x1, x6, x5, w4, v0, v1),
        8 => ipred_cfl_splat_w8_8bpc_neon(x0, x1, x6, x5, w4, v0, v1),
        _ => ipred_cfl_splat_w16_8bpc_neon(x0, x1, x6, w3, w4, x5, v0, v1),
    }
}

/// `dav1d_ipred_cfl_128_8bpc_neon`: chroma from luma with the dc 128.
///
/// Safety (all four `ipred_cfl*` entry points): reads `ac[0 .. width * height]` and writes `dst` rows
/// `0..height`, columns `0..width`; this one reads nothing of the edge.
pub unsafe extern "C" fn dav1d_ipred_cfl_128_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    _topleft: *const u8,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
) {
    let v0 = vdupq_n_s16(128); // movi v0.8h, #128: dc
    let v1 = vdupq_n_s16(alpha as i16); // dup v1.8h, w6: alpha
    let x6 = dst.offset(stride); // add x6, x0, x1
    let x1 = stride << 1; // lsl x1, x1, #1
    ipred_cfl_splat_tbl_8bpc(dst, x1, x6, width, height, ac, v0, v1) // br x7
}

/// `dav1d_ipred_cfl_top_8bpc_neon`: chroma from luma with the dc of the top edge.
///
/// Safety: as `dav1d_ipred_cfl_128_8bpc_neon`, and reads `topleft[1 .. width + 1]`.
pub unsafe extern "C" fn dav1d_ipred_cfl_top_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
) {
    let v1 = vdupq_n_s16(alpha as i16); // dup v1.8h, w6: alpha
    let x2 = topleft.add(1); // add x2, x2, #1
    let x6 = dst.offset(stride); // add x6, x0, x1
    let x1 = stride << 1; // lsl x1, x1, #1

    // br x7 (ipred_cfl_top_tbl, by the width)
    match width {
        4 => {
            // 4:
            let v0 = pd_ld1r_2s(x2); // ld1r {v0.2s}, [x2]
            let v0 = pd_uaddlv_8b(v0); // uaddlv h0, v0.8b
            let v0 = vrshr_n_u16::<3>(v0); // urshr v0.4h, v0.4h, #3
            let v0 = vreinterpretq_s16_u16(vdupq_lane_u16::<0>(v0)); // dup v0.8h, v0.h[0]
            ipred_cfl_splat_w4_8bpc_neon(dst, x1, x6, ac, height, v0, v1) // b L(ipred_cfl_splat_w4)
        }
        8 => {
            // 8:
            let v0 = vld1_u8(x2); // ld1 {v0.8b}, [x2]
            let v0 = pd_uaddlv_8b(v0); // uaddlv h0, v0.8b
            let v0 = vrshr_n_u16::<3>(v0); // urshr v0.4h, v0.4h, #3
            let v0 = vreinterpretq_s16_u16(vdupq_lane_u16::<0>(v0)); // dup v0.8h, v0.h[0]
            ipred_cfl_splat_w8_8bpc_neon(dst, x1, x6, ac, height, v0, v1) // b L(ipred_cfl_splat_w8)
        }
        16 => {
            // 16:
            let v0 = vld1q_u8(x2); // ld1 {v0.16b}, [x2]
            let v0 = pd_uaddlv_16b(v0); // uaddlv h0, v0.16b
            let v0 = vrshr_n_u16::<4>(v0); // urshr v0.4h, v0.4h, #4
            let v0 = vreinterpretq_s16_u16(vdupq_lane_u16::<0>(v0)); // dup v0.8h, v0.h[0]
            ipred_cfl_splat_w16_8bpc_neon(dst, x1, x6, width, height, ac, v0, v1)
            // b L(ipred_cfl_splat_w16)
        }
        _ => {
            // 32:
            let v23 = vld1q_u8_x2(x2); // ld1 {v2.16b, v3.16b}, [x2]
            let v2 = pd_uaddlv_16b(v23.0); // uaddlv h2, v2.16b
            let v3 = pd_uaddlv_16b(v23.1); // uaddlv h3, v3.16b
            let v2 = vadd_u16(v2, v3); // add v2.4h, v2.4h, v3.4h
            let v2 = vrshr_n_u16::<5>(v2); // urshr v2.4h, v2.4h, #5
            let v0 = vreinterpretq_s16_u16(vdupq_lane_u16::<0>(v2)); // dup v0.8h, v2.h[0]
            ipred_cfl_splat_w16_8bpc_neon(dst, x1, x6, width, height, ac, v0, v1)
            // b L(ipred_cfl_splat_w16)
        }
    }
}

/// `dav1d_ipred_cfl_left_8bpc_neon`: chroma from luma with the dc of the left edge.
///
/// Safety: as `dav1d_ipred_cfl_128_8bpc_neon`, and reads `topleft[-height .. 0]`.
pub unsafe extern "C" fn dav1d_ipred_cfl_left_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
) {
    let x2 = topleft.sub(height as usize); // sub x2, x2, w4, uxtw
    let v1 = vdupq_n_s16(alpha as i16); // dup v1.8h, w6: alpha
    let x6 = dst.offset(stride); // add x6, x0, x1
    let x1 = stride << 1; // lsl x1, x1, #1

    // br x7 (ipred_cfl_left_tbl, by the height)
    let v0 = match height {
        4 => {
            // L(ipred_cfl_left_h4):
            let v0 = pd_ld1r_2s(x2); // ld1r {v0.2s}, [x2]
            let v0 = pd_uaddlv_8b(v0); // uaddlv h0, v0.8b
            vrshr_n_u16::<3>(v0) // urshr v0.4h, v0.4h, #3
        }
        8 => {
            // L(ipred_cfl_left_h8):
            let v0 = vld1_u8(x2); // ld1 {v0.8b}, [x2]
            let v0 = pd_uaddlv_8b(v0); // uaddlv h0, v0.8b
            vrshr_n_u16::<3>(v0) // urshr v0.4h, v0.4h, #3
        }
        16 => {
            // L(ipred_cfl_left_h16):
            let v0 = vld1q_u8(x2); // ld1 {v0.16b}, [x2]
            let v0 = pd_uaddlv_16b(v0); // uaddlv h0, v0.16b
            vrshr_n_u16::<4>(v0) // urshr v0.4h, v0.4h, #4
        }
        _ => {
            // L(ipred_cfl_left_h32):
            let v23 = vld1q_u8_x2(x2); // ld1 {v2.16b, v3.16b}, [x2]
            let v2 = pd_uaddlv_16b(v23.0); // uaddlv h2, v2.16b
            let v3 = pd_uaddlv_16b(v23.1); // uaddlv h3, v3.16b
            let v2 = vadd_u16(v2, v3); // add v2.4h, v2.4h, v3.4h
            vrshr_n_u16::<5>(v2) // urshr v2.4h, v2.4h, #5
        }
    };
    let v0 = vreinterpretq_s16_u16(vdupq_lane_u16::<0>(v0)); // dup v0.8h, v0.h[0] (each block)
    ipred_cfl_splat_tbl_8bpc(dst, x1, x6, width, height, ac, v0, v1) // br x9
}

/// `dav1d_ipred_cfl_8bpc_neon`: chroma from luma with the dc of the left and top edges.
///
/// Safety: as `dav1d_ipred_cfl_128_8bpc_neon`, and reads `topleft[-height .. 0]` and
/// `topleft[1 .. width + 1]`.
pub unsafe extern "C" fn dav1d_ipred_cfl_8bpc_neon(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
) {
    let (w3, w4) = (width, height);
    let x2 = topleft.sub(w4 as usize); // sub x2, x2, w4, uxtw
    let w8 = (w3 + w4) as u32; // add w8, w3, w4: width + height
    let v1 = vdupq_n_s16(alpha as i16); // dup v1.8h, w6: alpha
    let v16 = vdup_n_u16(w8 as u16); // dup v16.8h, w8: width + height
    let w8 = w8.trailing_zeros() as i32; // rbit w8, w8; clz w8, w8: ctz(width + height)
    let w8 = -w8; // neg w8, w8: -ctz(width + height)
    let v16 = vshr_n_u16::<1>(v16); // ushr v16.8h, v16.8h, #1: (width + height) >> 1
    let v17 = vdup_n_s16(w8 as i16); // dup v17.8h, w8: -ctz(width + height)
    let x6 = dst.offset(stride); // add x6, x0, x1
    let x1 = stride << 1; // lsl x1, x1, #1

    // br x7 (ipred_cfl_tbl, by the height)
    let v0 = match w4 {
        4 => {
            // L(ipred_cfl_h4):
            let v0 = pd_ld1_s(x2); // ld1 {v0.s}[0], [x2], #4; ins v0.s[1], wzr
            pd_uaddlv_8b(v0) // uaddlv h0, v0.8b
        }
        8 => {
            // L(ipred_cfl_h8):
            let v0 = vld1_u8(x2); // ld1 {v0.8b}, [x2], #8
            pd_uaddlv_8b(v0) // uaddlv h0, v0.8b
        }
        16 => {
            // L(ipred_cfl_h16):
            let v0 = vld1q_u8(x2); // ld1 {v0.16b}, [x2], #16
            pd_uaddlv_16b(v0) // uaddlv h0, v0.16b
        }
        _ => {
            // L(ipred_cfl_h32):
            let v23 = vld1q_u8_x2(x2); // ld1 {v2.16b, v3.16b}, [x2], #32
            let v2 = pd_uaddlv_16b(v23.0); // uaddlv h2, v2.16b
            let v3 = pd_uaddlv_16b(v23.1); // uaddlv h3, v3.16b
            vadd_u16(v2, v3) // add v0.4h, v2.4h, v3.4h
        }
    };
    let x2 = topleft.add(1); // (x2 after the post-increment) add x2, x2, #1

    // br x9 (ipred_cfl_tbl + 4, by the width)
    match w3 {
        4 => {
            // L(ipred_cfl_w4):
            let v2 = pd_ld1_s(x2); // ld1 {v2.s}[0], [x2]; ins v2.s[1], wzr
            let v0 = vadd_u16(v0, v16); // add v0.4h, v0.4h, v16.4h
            let v2 = pd_uaddlv_8b(v2); // uaddlv h2, v2.8b
            let v0 = vadd_u16(v0, v2); // add v0.4h, v0.4h, v2.4h
            let mut v0 = vreinterpret_s16_u16(vshl_u16(v0, v17)); // ushl v0.4h, v0.4h, v17.4h
            if w4 != 4 {
                // cmp w4, #4; b.eq 1f
                // h = 8/16
                let w16 = 0x3334 / 2 | (0x5556 / 2) << 16; // mov w16, #(0x3334/2); movk w16, #(0x5556/2), lsl #16
                let w17 = (w4 + w4) as u32; // add w17, w4, w4: w17 = 2*h = 16 or 32
                let w16 = u32::wrapping_shr(w16, w17); // lsr w16, w16, w17
                let v16 = vdup_n_s16(w16 as i16); // dup v16.4h, w16
                v0 = vqdmulh_s16(v0, v16); // sqdmulh v0.4h, v0.4h, v16.4h
            }
            // 1:
            let v0 = vdupq_lane_s16::<0>(v0); // dup v0.8h, v0.h[0]
            ipred_cfl_splat_w4_8bpc_neon(dst, x1, x6, ac, w4, v0, v1) // b L(ipred_cfl_splat_w4)
        }
        8 => {
            // L(ipred_cfl_w8):
            let v2 = vld1_u8(x2); // ld1 {v2.8b}, [x2]
            let v0 = vadd_u16(v0, v16); // add v0.4h, v0.4h, v16.4h
            let v2 = pd_uaddlv_8b(v2); // uaddlv h2, v2.8b
            let v0 = vadd_u16(v0, v2); // add v0.4h, v0.4h, v2.4h
            let mut v0 = vreinterpret_s16_u16(vshl_u16(v0, v17)); // ushl v0.4h, v0.4h, v17.4h
            if w4 != 8 {
                // cmp w4, #8; b.eq 1f
                // h = 4/16/32
                let w16 = if w4 == 32 { 0x3334 / 2 } else { 0x5556 / 2 }; // cmp w4, #32; mov w16, #(0x3334/2); mov w17, #(0x5556/2); csel w16, w16, w17, eq
                let v16 = vdup_n_s16(w16); // dup v16.4h, w16
                v0 = vqdmulh_s16(v0, v16); // sqdmulh v0.4h, v0.4h, v16.4h
            }
            // 1:
            let v0 = vdupq_lane_s16::<0>(v0); // dup v0.8h, v0.h[0]
            ipred_cfl_splat_w8_8bpc_neon(dst, x1, x6, ac, w4, v0, v1) // b L(ipred_cfl_splat_w8)
        }
        16 => {
            // L(ipred_cfl_w16):
            let v2 = vld1q_u8(x2); // ld1 {v2.16b}, [x2]
            let v0 = vadd_u16(v0, v16); // add v0.4h, v0.4h, v16.4h
            let v2 = pd_uaddlv_16b(v2); // uaddlv h2, v2.16b
            let v0 = vadd_u16(v0, v2); // add v0.4h, v0.4h, v2.4h
            let mut v0 = vreinterpret_s16_u16(vshl_u16(v0, v17)); // ushl v0.4h, v0.4h, v17.4h
            if w4 != 16 {
                // cmp w4, #16; b.eq 1f
                // h = 4/8/32
                let w16 = if w4 == 4 { 0x3334 / 2 } else { 0x5556 / 2 }; // cmp w4, #4; mov w16, #(0x3334/2); mov w17, #(0x5556/2); csel w16, w16, w17, eq
                let v16 = vdup_n_s16(w16); // dup v16.4h, w16
                v0 = vqdmulh_s16(v0, v16); // sqdmulh v0.4h, v0.4h, v16.4h
            }
            // 1:
            let v0 = vdupq_lane_s16::<0>(v0); // dup v0.8h, v0.h[0]
            ipred_cfl_splat_w16_8bpc_neon(dst, x1, x6, w3, w4, ac, v0, v1) // b L(ipred_cfl_splat_w16)
        }
        _ => {
            // L(ipred_cfl_w32):
            let v23 = vld1q_u8_x2(x2); // ld1 {v2.16b, v3.16b}, [x2]
            let v0 = vadd_u16(v0, v16); // add v0.4h, v0.4h, v16.4h
            let v2 = pd_uaddlv_16b(v23.0); // uaddlv h2, v2.16b
            let v3 = pd_uaddlv_16b(v23.1); // uaddlv h3, v3.16b
            let v0 = vadd_u16(v0, v2); // add v0.4h, v0.4h, v2.4h
            let v0 = vadd_u16(v0, v3); // add v0.4h, v0.4h, v3.4h
            let mut v0 = vreinterpret_s16_u16(vshl_u16(v0, v17)); // ushl v0.4h, v0.4h, v17.4h
            if w4 != 32 {
                // cmp w4, #32; b.eq 1f
                // h = 8/16
                let w16 = 0x5556 / 2 | (0x3334 / 2) << 16; // mov w16, #(0x5556/2); movk w16, #(0x3334/2), lsl #16
                let w17 = (w4 + w4) as u32; // add w17, w4, w4: w17 = 2*h = 16 or 32
                let w16 = u32::wrapping_shr(w16, w17); // lsr w16, w16, w17
                let v16 = vdup_n_s16(w16 as i16); // dup v16.4h, w16
                v0 = vqdmulh_s16(v0, v16); // sqdmulh v0.4h, v0.4h, v16.4h
            }
            // 1:
            let v0 = vdupq_lane_s16::<0>(v0); // dup v0.8h, v0.h[0]
            ipred_cfl_splat_w16_8bpc_neon(dst, x1, x6, w3, w4, ac, v0, v1) // b L(ipred_cfl_splat_w16)
        }
    }
}

// --- ipred_cfl_ac ----------------------------------------------------------------------------------------------

/// The common head of the three `ipred_cfl_ac_*` entry points after the table load: `w8` = height - h_pad
/// (in rows of `ac`) and `w9` = log2sz. (`dup v31.4s, w9; neg v31.4s, v31.4s` follow in `ipred_cfl_ac_dc`.)
#[inline(always)]
fn ipred_cfl_ac_head(w4: c_int, w5: c_int, w6: c_int) -> (c_int, c_int) {
    let w8 = w6 - w4; // sub w8, w6, w4: height - h_pad
    let w9 = w5.trailing_zeros(); // rbit w9, w5; clz w9, w9: ctz(width)
    let w10 = w6.trailing_zeros(); // rbit w10, w6; clz w10, w10: ctz(height)
    let w9 = (w9 + w10) as c_int; // add w9, w9, w10: log2sz
    (w8, w9)
}

/// `L(ipred_cfl_ac_420_w8_subtract_dc)`: subtracts the dc `v4` from `w6 / 4` groups of 32 coefficients.
/// (The blocks shared by the `ipred_cfl_ac_*` entry points take their registers in one order, `x0`, `w6`, `w9`,
/// `w4`, the sums `v16`-`v19`, then the last row, so that the branch from one into the next moves nothing.)
unsafe fn ipred_cfl_ac_420_w8_subtract_dc_8bpc_neon(mut x0: *mut u16, mut w6: c_int, v4: uint16x8_t) {
    loop {
        // 6: Subtract dc from ac
        let t = vld1q_u16_x4(x0); // ld1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0]
        let gt = pd_subs_gt(&mut w6, 4); // subs w6, w6, #4
        let v0 = vsubq_u16(t.0, v4); // sub v0.8h, v0.8h, v4.8h
        let v1 = vsubq_u16(t.1, v4); // sub v1.8h, v1.8h, v4.8h
        let v2 = vsubq_u16(t.2, v4); // sub v2.8h, v2.8h, v4.8h
        let v3 = vsubq_u16(t.3, v4); // sub v3.8h, v3.8h, v4.8h
        vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
        x0 = x0.wrapping_add(32);
        if !gt {
            break; // b.gt 6b
        }
    }
}

/// `urshl v4.2s, v0.2s, v31.2s; dup v4.8h, v4.h[0]` on the sum in `s0`: the rounded dc in every lane. `v31`
/// = -log2sz is formed here (`dup v31.4s, w9; neg v31.4s, v31.4s` of the entry points), so that the shared
/// blocks get `w9` in a general register and at most eight vector registers.
#[inline(always)]
unsafe fn ipred_cfl_ac_dc(s0: u32, w9: c_int) -> uint16x8_t {
    let v31 = vdupq_n_s32(w9); // dup v31.4s, w9
    let v31 = vnegq_s32(v31); // neg v31.4s, v31.4s: -log2sz
    let v0 = vdup_n_u32(s0); // (the sum in lane 0; only lane 0 is used)
    let v4 = vrshl_u32(v0, vget_low_s32(v31)); // urshl v4.2s, v0.2s, v31.2s: (sum + (1 << (log2sz - 1))) >>= log2sz
    vdupq_lane_u16::<0>(vreinterpret_u16_u32(v4)) // dup v4.8h, v4.h[0]
}

/// `L(ipred_cfl_ac_420_w8_calc_subtract_dc)`: aggregates the sums, then subtracts the dc.
unsafe fn ipred_cfl_ac_420_w8_calc_subtract_dc_8bpc_neon(
    x0: *mut u16,
    w6: c_int,
    w9: c_int,
    v16: uint16x8_t,
    v17: uint16x8_t,
    v18: uint16x8_t,
    v19: uint16x8_t,
) {
    // Aggregate the sums
    let v0 = vaddq_u16(v16, v17); // add v0.8h, v16.8h, v17.8h
    let v2 = vaddq_u16(v18, v19); // add v2.8h, v18.8h, v19.8h
    let v0 = vpaddlq_u16(v0); // uaddlp v0.4s, v0.8h
    let v2 = vpaddlq_u16(v2); // uaddlp v2.4s, v2.8h
    let v0 = vaddq_u32(v0, v2); // add v0.4s, v0.4s, v2.4s
    let s0 = vaddvq_u32(v0); // addv s0, v0.4s: sum
    let x0 = x0.sub((w6 as u32 as usize) << 3); // sub x0, x0, w6, uxtw #4
    let v4 = ipred_cfl_ac_dc(s0, w9);
    ipred_cfl_ac_420_w8_subtract_dc_8bpc_neon(x0, w6, v4) // L(ipred_cfl_ac_420_w8_subtract_dc):
}

/// `L(ipred_cfl_ac_420_w4_hpad)`: the vertical padding, the dc and its subtraction for w 4; `v0`, `v1` hold
/// the last row twice each.
unsafe fn ipred_cfl_ac_420_w4_hpad_8bpc_neon(
    mut x0: *mut u16,
    mut w6: c_int,
    w9: c_int,
    mut w4: c_int,
    mut v16: uint16x8_t,
    mut v17: uint16x8_t,
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
            v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
            v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
            if !gt {
                break; // b.gt 2b
            }
        }
    }
    // 3:
    // Aggregate the sums
    let v0 = vaddq_u16(v16, v17); // add v0.8h, v16.8h, v17.8h
    let s0 = vaddlvq_u16(v0); // uaddlv s0, v0.8h: sum
    let mut x0 = x0.sub((w6 as u32 as usize) << 2); // sub x0, x0, w6, uxtw #3
    let v4 = ipred_cfl_ac_dc(s0, w9);
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

/// `L(ipred_cfl_ac_420_w8_hpad)`: the vertical padding for w 8, then
/// `L(ipred_cfl_ac_420_w8_calc_subtract_dc)`; `v0`, `v1` hold the last row.
unsafe fn ipred_cfl_ac_420_w8_hpad_8bpc_neon(
    mut x0: *mut u16,
    w6: c_int,
    w9: c_int,
    mut w4: c_int,
    mut v16: uint16x8_t,
    mut v17: uint16x8_t,
    mut v18: uint16x8_t,
    mut v19: uint16x8_t,
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
            v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
            v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
            vst1q_u16_x2(x0, uint16x8x2_t(v0, v1)); // st1 {v0.8h, v1.8h}, [x0], #32
            x0 = x0.add(16);
            v18 = vaddq_u16(v18, v0); // add v18.8h, v18.8h, v0.8h
            v19 = vaddq_u16(v19, v1); // add v19.8h, v19.8h, v1.8h
            if !gt {
                break; // b.gt 2b
            }
        }
    }
    // 3:
    ipred_cfl_ac_420_w8_calc_subtract_dc_8bpc_neon(x0, w6, w9, v16, v17, v18, v19)
    // L(ipred_cfl_ac_420_w8_calc_subtract_dc):
}

/// `L(ipred_cfl_ac_420_w16_hpad)`: the vertical padding for w 16, then
/// `L(ipred_cfl_ac_420_w8_calc_subtract_dc)` with twice the height; `v0`, `v1` and `v2`, `v3` hold the last
/// row.
unsafe fn ipred_cfl_ac_420_w16_hpad_8bpc_neon(
    mut x0: *mut u16,
    w6: c_int,
    w9: c_int,
    mut w4: c_int,
    mut v16: uint16x8_t,
    mut v17: uint16x8_t,
    mut v18: uint16x8_t,
    mut v19: uint16x8_t,
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
            v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
            v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
            v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
            v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
            vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
            x0 = x0.add(32);
            v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
            v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
            v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
            v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
            if !gt {
                break; // b.gt 2b
            }
        }
    }
    // 3:
    // Double the height and reuse the w8 summing/subtracting
    let w6 = w6 << 1; // lsl w6, w6, #1
    ipred_cfl_ac_420_w8_calc_subtract_dc_8bpc_neon(x0, w6, w9, v16, v17, v18, v19)
    // b L(ipred_cfl_ac_420_w8_calc_subtract_dc)
}

/// `dav1d_ipred_cfl_ac_420_8bpc_neon`: the luma of a 4:2:0 block, subsampled, times 8, less its average.
///
/// Safety: reads the luma rows `0 .. 2 * (ch - 4 * h_pad)` at the columns `0 .. 2 * (cw - 4 * w_pad)`
/// exactly (cw 4: 8 bytes per row; cw 8: 16, with `w_pad` 8; cw 16: 32, 24, 16 or 8 for `w_pad` 0..3);
/// writes `ac[0 .. cw * ch]`, rows of `cw` coefficients, and reads them back for the subtraction.
pub unsafe extern "C" fn dav1d_ipred_cfl_ac_420_8bpc_neon(
    ac: *mut i16,
    ypx: *const u8,
    stride: isize,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
) {
    let (mut x0, mut x1, w3, w5, w6) = (ac.cast::<u16>(), ypx, w_pad, cw, ch);
    let w4 = h_pad << 2; // lsl w4, w4, #2
    let mut v16 = vdupq_n_u16(0); // movi v16.8h, #0
    let mut v17 = vdupq_n_u16(0); // movi v17.8h, #0
    let mut v18 = vdupq_n_u16(0); // movi v18.8h, #0
    let mut v19 = vdupq_n_u16(0); // movi v19.8h, #0
    let (mut w8, w9) = ipred_cfl_ac_head(w4, w5, w6);
    let mut x10 = x1.offset(stride); // add x10, x1, x2
    let x2 = stride << 1; // lsl x2, x2, #1

    // br x7 (ipred_cfl_ac_420_tbl, by the width)
    match w5 {
        4 => {
            // L(ipred_cfl_ac_420_w4):
            let mut v0;
            loop {
                // 1: Copy and subsample input
                let a = x1; // ld1 {v0.8b}, [x1], x2
                let b = x10; // ld1 {v1.8b}, [x10], x2
                x1 = x1.wrapping_offset(x2);
                x10 = x10.wrapping_offset(x2);
                let t0 = pd_ld1_d2(a, x1); // ld1 {v0.d}[1], [x1], x2
                let t1 = pd_ld1_d2(b, x10); // ld1 {v1.d}[1], [x10], x2
                x1 = x1.wrapping_offset(x2);
                x10 = x10.wrapping_offset(x2);
                let t0 = vpaddlq_u8(t0); // uaddlp v0.8h, v0.16b
                let t1 = vpaddlq_u8(t1); // uaddlp v1.8h, v1.16b
                v0 = vaddq_u16(t0, t1); // add v0.8h, v0.8h, v1.8h
                v0 = vshlq_n_u16::<1>(v0); // shl v0.8h, v0.8h, #1
                let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                vst1q_u16(x0, v0); // st1 {v0.8h}, [x0], #16
                x0 = x0.add(8);
                v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                if !gt {
                    break; // b.gt 1b
                }
            }
            let v0q = vreinterpretq_u64_u16(v0);
            let v1 = vreinterpretq_u16_u64(vtrn2q_u64(v0q, v0q)); // trn2 v1.2d, v0.2d, v0.2d
            let v0 = v1; // trn2 v0.2d, v0.2d, v0.2d
            ipred_cfl_ac_420_w4_hpad_8bpc_neon(x0, w6, w9, w4, v16, v17, v0, v1)
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
                    let t0 = vld1q_u8(x1); // ld1 {v0.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t1 = vld1q_u8(x10); // ld1 {v1.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t2 = vld1q_u8(x1); // ld1 {v2.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t0 = vpaddlq_u8(t0); // uaddlp v0.8h, v0.16b
                    let t3 = vld1q_u8(x10); // ld1 {v3.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t1 = vpaddlq_u8(t1); // uaddlp v1.8h, v1.16b
                    let t2 = vpaddlq_u8(t2); // uaddlp v2.8h, v2.16b
                    let t3 = vpaddlq_u8(t3); // uaddlp v3.8h, v3.16b
                    let t0 = vaddq_u16(t0, t1); // add v0.8h, v0.8h, v1.8h
                    let t2 = vaddq_u16(t2, t3); // add v2.8h, v2.8h, v3.8h
                    let r0 = vshlq_n_u16::<1>(t0); // shl v0.8h, v0.8h, #1
                    r1 = vshlq_n_u16::<1>(t2); // shl v1.8h, v2.8h, #1
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x2(x0, uint16x8x2_t(r0, r1)); // st1 {v0.8h, v1.8h}, [x0], #32
                    x0 = x0.add(16);
                    v16 = vaddq_u16(v16, r0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, r1); // add v17.8h, v17.8h, v1.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                }
                v0 = r1; // mov v0.16b, v1.16b
                v1 = r1;
                // b L(ipred_cfl_ac_420_w8_hpad)
            } else {
                // L(ipred_cfl_ac_420_w8_wpad):
                let (mut a16, mut a17, mut a18, mut a19) = (vdup_n_u16(0), vdup_n_u16(0), vdup_n_u16(0), vdup_n_u16(0));
                let (mut r2, mut r3);
                loop {
                    // 1: Copy and subsample input, padding 4
                    let a = x1; // ld1 {v0.8b}, [x1], x2
                    let b = x10; // ld1 {v1.8b}, [x10], x2
                    x1 = x1.wrapping_offset(x2);
                    x10 = x10.wrapping_offset(x2);
                    let t0 = pd_ld1_d2(a, x1); // ld1 {v0.d}[1], [x1], x2
                    let t1 = pd_ld1_d2(b, x10); // ld1 {v1.d}[1], [x10], x2
                    x1 = x1.wrapping_offset(x2);
                    x10 = x10.wrapping_offset(x2);
                    let t0 = vpaddlq_u8(t0); // uaddlp v0.8h, v0.16b
                    let t1 = vpaddlq_u8(t1); // uaddlp v1.8h, v1.16b
                    let t0 = vaddq_u16(t0, t1); // add v0.8h, v0.8h, v1.8h
                    let t0 = vshlq_n_u16::<1>(t0); // shl v0.8h, v0.8h, #1
                    let r1 = vdup_laneq_u16::<3>(t0); // dup v1.4h, v0.h[3]
                    r3 = vdup_laneq_u16::<7>(t0); // dup v3.4h, v0.h[7]
                    r2 = vget_high_u16(t0); // trn2 v2.2d, v0.2d, v0.2d
                    let r0 = vget_low_u16(t0);
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1_u16_x4(x0, uint16x4x4_t(r0, r1, r2, r3)); // st1 {v0.4h, v1.4h, v2.4h, v3.4h}, [x0], #32
                    x0 = x0.add(16);
                    a16 = vadd_u16(a16, r0); // add v16.4h, v16.4h, v0.4h
                    a17 = vadd_u16(a17, r1); // add v17.4h, v17.4h, v1.4h
                    a18 = vadd_u16(a18, r2); // add v18.4h, v18.4h, v2.4h
                    a19 = vadd_u16(a19, r3); // add v19.4h, v19.4h, v3.4h
                    if !gt {
                        break; // b.gt 1b
                    }
                }
                v16 = vcombine_u16(a16, vdup_n_u16(0));
                v17 = vcombine_u16(a17, vdup_n_u16(0));
                v18 = vcombine_u16(a18, vdup_n_u16(0));
                v19 = vcombine_u16(a19, vdup_n_u16(0));
                v0 = vcombine_u16(r2, r3); // trn1 v0.2d, v2.2d, v3.2d
                v1 = v0; // trn1 v1.2d, v2.2d, v3.2d
            }
            ipred_cfl_ac_420_w8_hpad_8bpc_neon(x0, w6, w9, w4, v16, v17, v18, v19, v0, v1)
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
                    let t01 = vld1q_u8_x2(x1); // ld1 {v0.16b, v1.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t23 = vld1q_u8_x2(x10); // ld1 {v2.16b, v3.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t0 = vpaddlq_u8(t01.0); // uaddlp v0.8h, v0.16b
                    let t45 = vld1q_u8_x2(x1); // ld1 {v4.16b, v5.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t1 = vpaddlq_u8(t01.1); // uaddlp v1.8h, v1.16b
                    let t67 = vld1q_u8_x2(x10); // ld1 {v6.16b, v7.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t2 = vpaddlq_u8(t23.0); // uaddlp v2.8h, v2.16b
                    let t3 = vpaddlq_u8(t23.1); // uaddlp v3.8h, v3.16b
                    let t4 = vpaddlq_u8(t45.0); // uaddlp v4.8h, v4.16b
                    let t5 = vpaddlq_u8(t45.1); // uaddlp v5.8h, v5.16b
                    let t6 = vpaddlq_u8(t67.0); // uaddlp v6.8h, v6.16b
                    let t7 = vpaddlq_u8(t67.1); // uaddlp v7.8h, v7.16b
                    let t0 = vaddq_u16(t0, t2); // add v0.8h, v0.8h, v2.8h
                    let t1 = vaddq_u16(t1, t3); // add v1.8h, v1.8h, v3.8h
                    let t4 = vaddq_u16(t4, t6); // add v4.8h, v4.8h, v6.8h
                    let t5 = vaddq_u16(t5, t7); // add v5.8h, v5.8h, v7.8h
                    let r0 = vshlq_n_u16::<1>(t0); // shl v0.8h, v0.8h, #1
                    let r1 = vshlq_n_u16::<1>(t1); // shl v1.8h, v1.8h, #1
                    v2 = vshlq_n_u16::<1>(t4); // shl v2.8h, v4.8h, #1
                    v3 = vshlq_n_u16::<1>(t5); // shl v3.8h, v5.8h, #1
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(r0, r1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, r0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, r1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                1 => loop {
                    // L(ipred_cfl_ac_420_w16_wpad1):
                    // 1: Copy and subsample input, padding 4
                    let t1 = vld1_u8(x1.add(16)); // ldr d1, [x1, #16]
                    let t0 = vld1q_u8(x1); // ld1 {v0.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t3 = vld1_u8(x10.add(16)); // ldr d3, [x10, #16]
                    let t2 = vld1q_u8(x10); // ld1 {v2.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t1 = vpaddl_u8(t1); // uaddlp v1.4h, v1.8b
                    let t5 = vld1_u8(x1.add(16)); // ldr d5, [x1, #16]
                    let t0 = vpaddlq_u8(t0); // uaddlp v0.8h, v0.16b
                    let t4 = vld1q_u8(x1); // ld1 {v4.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t3 = vpaddl_u8(t3); // uaddlp v3.4h, v3.8b
                    let t7 = vld1_u8(x10.add(16)); // ldr d7, [x10, #16]
                    let t2 = vpaddlq_u8(t2); // uaddlp v2.8h, v2.16b
                    let t6 = vld1q_u8(x10); // ld1 {v6.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t5 = vpaddl_u8(t5); // uaddlp v5.4h, v5.8b
                    let t4 = vpaddlq_u8(t4); // uaddlp v4.8h, v4.16b
                    let t7 = vpaddl_u8(t7); // uaddlp v7.4h, v7.8b
                    let t6 = vpaddlq_u8(t6); // uaddlp v6.8h, v6.16b
                    let t1 = vadd_u16(t1, t3); // add v1.4h, v1.4h, v3.4h
                    let t0 = vaddq_u16(t0, t2); // add v0.8h, v0.8h, v2.8h
                    let t5 = vadd_u16(t5, t7); // add v5.4h, v5.4h, v7.4h
                    let t4 = vaddq_u16(t4, t6); // add v4.8h, v4.8h, v6.8h
                    let t1 = vshl_n_u16::<1>(t1); // shl v1.4h, v1.4h, #1
                    let r0 = vshlq_n_u16::<1>(t0); // shl v0.8h, v0.8h, #1
                    let t3 = vshl_n_u16::<1>(t5); // shl v3.4h, v5.4h, #1
                    v2 = vshlq_n_u16::<1>(t4); // shl v2.8h, v4.8h, #1
                    let t4 = vdup_lane_u16::<3>(t1); // dup v4.4h, v1.h[3]
                    let t5 = vdup_lane_u16::<3>(t3); // dup v5.4h, v3.h[3]
                    let r1 = vcombine_u16(t1, t4); // trn1 v1.2d, v1.2d, v4.2d
                    v3 = vcombine_u16(t3, t5); // trn1 v3.2d, v3.2d, v5.2d
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(r0, r1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, r0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, r1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                2 => loop {
                    // L(ipred_cfl_ac_420_w16_wpad2):
                    // 1: Copy and subsample input, padding 8
                    let t0 = vld1q_u8(x1); // ld1 {v0.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t2 = vld1q_u8(x10); // ld1 {v2.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t4 = vld1q_u8(x1); // ld1 {v4.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t0 = vpaddlq_u8(t0); // uaddlp v0.8h, v0.16b
                    let t6 = vld1q_u8(x10); // ld1 {v6.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t2 = vpaddlq_u8(t2); // uaddlp v2.8h, v2.16b
                    let t4 = vpaddlq_u8(t4); // uaddlp v4.8h, v4.16b
                    let t6 = vpaddlq_u8(t6); // uaddlp v6.8h, v6.16b
                    let t0 = vaddq_u16(t0, t2); // add v0.8h, v0.8h, v2.8h
                    let t4 = vaddq_u16(t4, t6); // add v4.8h, v4.8h, v6.8h
                    let r0 = vshlq_n_u16::<1>(t0); // shl v0.8h, v0.8h, #1
                    v2 = vshlq_n_u16::<1>(t4); // shl v2.8h, v4.8h, #1
                    let r1 = vdupq_laneq_u16::<7>(r0); // dup v1.8h, v0.h[7]
                    v3 = vdupq_laneq_u16::<7>(v2); // dup v3.8h, v2.h[7]
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(r0, r1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, r0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, r1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                _ => loop {
                    // L(ipred_cfl_ac_420_w16_wpad3):
                    // 1: Copy and subsample input, padding 12
                    let t0 = vld1_u8(x1); // ld1 {v0.8b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t2 = vld1_u8(x10); // ld1 {v2.8b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t4 = vld1_u8(x1); // ld1 {v4.8b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t0 = vpaddl_u8(t0); // uaddlp v0.4h, v0.8b
                    let t6 = vld1_u8(x10); // ld1 {v6.8b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t2 = vpaddl_u8(t2); // uaddlp v2.4h, v2.8b
                    let t4 = vpaddl_u8(t4); // uaddlp v4.4h, v4.8b
                    let t6 = vpaddl_u8(t6); // uaddlp v6.4h, v6.8b
                    let t0 = vadd_u16(t0, t2); // add v0.4h, v0.4h, v2.4h
                    let t4 = vadd_u16(t4, t6); // add v4.4h, v4.4h, v6.4h
                    let t0 = vshl_n_u16::<1>(t0); // shl v0.4h, v0.4h, #1
                    let t2 = vshl_n_u16::<1>(t4); // shl v2.4h, v4.4h, #1
                    let r1 = vdupq_lane_u16::<3>(t0); // dup v1.8h, v0.h[3]
                    v3 = vdupq_lane_u16::<3>(t2); // dup v3.8h, v2.h[3]
                    let r0 = vcombine_u16(t0, vget_low_u16(r1)); // trn1 v0.2d, v0.2d, v1.2d
                    v2 = vcombine_u16(t2, vget_low_u16(v3)); // trn1 v2.2d, v2.2d, v3.2d
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(r0, r1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, r0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, r1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
            }
            // mov v0.16b, v2.16b; mov v1.16b, v3.16b (the arguments)
            ipred_cfl_ac_420_w16_hpad_8bpc_neon(x0, w6, w9, w4, v16, v17, v18, v19, v2, v3, v2, v3)
            // b L(ipred_cfl_ac_420_w16_hpad)
        }
    }
}

/// `dav1d_ipred_cfl_ac_422_8bpc_neon`: the luma of a 4:2:2 block, subsampled, times 8, less its average.
///
/// Safety: reads the luma rows `0 .. ch - 4 * h_pad` at the columns `0 .. 2 * (cw - 4 * w_pad)` exactly
/// (cw 4: 8 bytes per row; cw 8: 16, with `w_pad` 8; cw 16: 32, 24, 16 or 8 for `w_pad` 0..3); writes
/// `ac[0 .. cw * ch]`, rows of `cw` coefficients, and reads them back for the subtraction.
pub unsafe extern "C" fn dav1d_ipred_cfl_ac_422_8bpc_neon(
    ac: *mut i16,
    ypx: *const u8,
    stride: isize,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
) {
    let (mut x0, mut x1, w3, w5, w6) = (ac.cast::<u16>(), ypx, w_pad, cw, ch);
    let w4 = h_pad << 2; // lsl w4, w4, #2
    let mut v16 = vdupq_n_u16(0); // movi v16.8h, #0
    let mut v17 = vdupq_n_u16(0); // movi v17.8h, #0
    let mut v18 = vdupq_n_u16(0); // movi v18.8h, #0
    let mut v19 = vdupq_n_u16(0); // movi v19.8h, #0
    let (mut w8, w9) = ipred_cfl_ac_head(w4, w5, w6);
    let mut x10 = x1.offset(stride); // add x10, x1, x2
    let x2 = stride << 1; // lsl x2, x2, #1

    // br x7 (ipred_cfl_ac_422_tbl, by the width)
    match w5 {
        4 => {
            // L(ipred_cfl_ac_422_w4):
            let mut v1;
            loop {
                // 1: Copy and subsample input
                let t0 = pd_ld1_d2(x1, x10); // ld1 {v0.8b}, [x1], x2; ld1 {v0.d}[1], [x10], x2
                x1 = x1.wrapping_offset(x2);
                x10 = x10.wrapping_offset(x2);
                let t1 = pd_ld1_d2(x1, x10); // ld1 {v1.8b}, [x1], x2; ld1 {v1.d}[1], [x10], x2
                x1 = x1.wrapping_offset(x2);
                x10 = x10.wrapping_offset(x2);
                let t0 = vpaddlq_u8(t0); // uaddlp v0.8h, v0.16b
                let t1 = vpaddlq_u8(t1); // uaddlp v1.8h, v1.16b
                let v0 = vshlq_n_u16::<2>(t0); // shl v0.8h, v0.8h, #2
                v1 = vshlq_n_u16::<2>(t1); // shl v1.8h, v1.8h, #2
                let gt = pd_subs_gt(&mut w8, 4); // subs w8, w8, #4
                v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                vst1q_u16_x2(x0, uint16x8x2_t(v0, v1)); // st1 {v0.8h, v1.8h}, [x0], #32
                x0 = x0.add(16);
                if !gt {
                    break; // b.gt 1b
                }
            }
            let v1q = vreinterpretq_u64_u16(v1);
            let v0 = vreinterpretq_u16_u64(vtrn2q_u64(v1q, v1q)); // trn2 v0.2d, v1.2d, v1.2d
            let v1 = v0; // trn2 v1.2d, v1.2d, v1.2d
            ipred_cfl_ac_420_w4_hpad_8bpc_neon(x0, w6, w9, w4, v16, v17, v0, v1)
            // b L(ipred_cfl_ac_420_w4_hpad)
        }
        8 => {
            // L(ipred_cfl_ac_422_w8):
            let mut v3;
            if w3 == 0 {
                // cbnz w3, L(ipred_cfl_ac_422_w8_wpad)
                loop {
                    // 1: Copy and subsample input, without padding
                    let t0 = vld1q_u8(x1); // ld1 {v0.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t1 = vld1q_u8(x10); // ld1 {v1.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t2 = vld1q_u8(x1); // ld1 {v2.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t0 = vpaddlq_u8(t0); // uaddlp v0.8h, v0.16b
                    let t3 = vld1q_u8(x10); // ld1 {v3.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t1 = vpaddlq_u8(t1); // uaddlp v1.8h, v1.16b
                    let t2 = vpaddlq_u8(t2); // uaddlp v2.8h, v2.16b
                    let t3 = vpaddlq_u8(t3); // uaddlp v3.8h, v3.16b
                    let v0 = vshlq_n_u16::<2>(t0); // shl v0.8h, v0.8h, #2
                    let v1 = vshlq_n_u16::<2>(t1); // shl v1.8h, v1.8h, #2
                    let v2 = vshlq_n_u16::<2>(t2); // shl v2.8h, v2.8h, #2
                    v3 = vshlq_n_u16::<2>(t3); // shl v3.8h, v3.8h, #2
                    let gt = pd_subs_gt(&mut w8, 4); // subs w8, w8, #4
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                }
            } else {
                // L(ipred_cfl_ac_422_w8_wpad):
                loop {
                    // 1: Copy and subsample input, padding 4
                    let t0 = pd_ld1_d2(x1, x10); // ld1 {v0.8b}, [x1], x2; ld1 {v0.d}[1], [x10], x2
                    x1 = x1.wrapping_offset(x2);
                    x10 = x10.wrapping_offset(x2);
                    let t2 = pd_ld1_d2(x1, x10); // ld1 {v2.8b}, [x1], x2; ld1 {v2.d}[1], [x10], x2
                    x1 = x1.wrapping_offset(x2);
                    x10 = x10.wrapping_offset(x2);
                    let t0 = vpaddlq_u8(t0); // uaddlp v0.8h, v0.16b
                    let t2 = vpaddlq_u8(t2); // uaddlp v2.8h, v2.16b
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
                    v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                }
            }
            // mov v0.16b, v3.16b; mov v1.16b, v3.16b
            ipred_cfl_ac_420_w8_hpad_8bpc_neon(x0, w6, w9, w4, v16, v17, v18, v19, v3, v3)
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
                    let t01 = vld1q_u8_x2(x1); // ld1 {v0.16b, v1.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t23 = vld1q_u8_x2(x10); // ld1 {v2.16b, v3.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t0 = vpaddlq_u8(t01.0); // uaddlp v0.8h, v0.16b
                    let t1 = vpaddlq_u8(t01.1); // uaddlp v1.8h, v1.16b
                    let t2 = vpaddlq_u8(t23.0); // uaddlp v2.8h, v2.16b
                    let t3 = vpaddlq_u8(t23.1); // uaddlp v3.8h, v3.16b
                    let v0 = vshlq_n_u16::<2>(t0); // shl v0.8h, v0.8h, #2
                    let v1 = vshlq_n_u16::<2>(t1); // shl v1.8h, v1.8h, #2
                    v2 = vshlq_n_u16::<2>(t2); // shl v2.8h, v2.8h, #2
                    v3 = vshlq_n_u16::<2>(t3); // shl v3.8h, v3.8h, #2
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                1 => loop {
                    // L(ipred_cfl_ac_422_w16_wpad1):
                    // 1: Copy and subsample input, padding 4
                    let t1 = vld1_u8(x1.add(16)); // ldr d1, [x1, #16]
                    let t0 = vld1q_u8(x1); // ld1 {v0.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t3 = vld1_u8(x10.add(16)); // ldr d3, [x10, #16]
                    let t2 = vld1q_u8(x10); // ld1 {v2.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t1 = vpaddl_u8(t1); // uaddlp v1.4h, v1.8b
                    let t0 = vpaddlq_u8(t0); // uaddlp v0.8h, v0.16b
                    let t3 = vpaddl_u8(t3); // uaddlp v3.4h, v3.8b
                    let t2 = vpaddlq_u8(t2); // uaddlp v2.8h, v2.16b
                    let t1 = vshl_n_u16::<2>(t1); // shl v1.4h, v1.4h, #2
                    let v0 = vshlq_n_u16::<2>(t0); // shl v0.8h, v0.8h, #2
                    let t3 = vshl_n_u16::<2>(t3); // shl v3.4h, v3.4h, #2
                    v2 = vshlq_n_u16::<2>(t2); // shl v2.8h, v2.8h, #2
                    let t4 = vdup_lane_u16::<3>(t1); // dup v4.4h, v1.h[3]
                    let t5 = vdup_lane_u16::<3>(t3); // dup v5.4h, v3.h[3]
                    let v1 = vcombine_u16(t1, t4); // trn1 v1.2d, v1.2d, v4.2d
                    v3 = vcombine_u16(t3, t5); // trn1 v3.2d, v3.2d, v5.2d
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                2 => loop {
                    // L(ipred_cfl_ac_422_w16_wpad2):
                    // 1: Copy and subsample input, padding 8
                    let t0 = vld1q_u8(x1); // ld1 {v0.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t2 = vld1q_u8(x10); // ld1 {v2.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t0 = vpaddlq_u8(t0); // uaddlp v0.8h, v0.16b
                    let t2 = vpaddlq_u8(t2); // uaddlp v2.8h, v2.16b
                    let v0 = vshlq_n_u16::<2>(t0); // shl v0.8h, v0.8h, #2
                    v2 = vshlq_n_u16::<2>(t2); // shl v2.8h, v2.8h, #2
                    let v1 = vdupq_laneq_u16::<7>(v0); // dup v1.8h, v0.h[7]
                    v3 = vdupq_laneq_u16::<7>(v2); // dup v3.8h, v2.h[7]
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                _ => loop {
                    // L(ipred_cfl_ac_422_w16_wpad3):
                    // 1: Copy and subsample input, padding 12
                    let t0 = vld1_u8(x1); // ld1 {v0.8b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t2 = vld1_u8(x10); // ld1 {v2.8b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t0 = vpaddl_u8(t0); // uaddlp v0.4h, v0.8b
                    let t2 = vpaddl_u8(t2); // uaddlp v2.4h, v2.8b
                    let t0 = vshl_n_u16::<2>(t0); // shl v0.4h, v0.4h, #2
                    let t2 = vshl_n_u16::<2>(t2); // shl v2.4h, v2.4h, #2
                    let v1 = vdupq_lane_u16::<3>(t0); // dup v1.8h, v0.h[3]
                    v3 = vdupq_lane_u16::<3>(t2); // dup v3.8h, v2.h[3]
                    let v0 = vcombine_u16(t0, vget_low_u16(v1)); // trn1 v0.2d, v0.2d, v1.2d
                    v2 = vcombine_u16(t2, vget_low_u16(v3)); // trn1 v2.2d, v2.2d, v3.2d
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
            }
            // mov v0.16b, v2.16b; mov v1.16b, v3.16b (the arguments)
            ipred_cfl_ac_420_w16_hpad_8bpc_neon(x0, w6, w9, w4, v16, v17, v18, v19, v2, v3, v2, v3)
            // b L(ipred_cfl_ac_420_w16_hpad)
        }
    }
}

/// `dav1d_ipred_cfl_ac_444_8bpc_neon`: the luma of a 4:4:4 block times 8, less its average.
///
/// Safety: reads the luma rows `0 .. ch - 4 * h_pad` at the columns `0 .. cw - 4 * w_pad` exactly, for the
/// values of `w_pad` the layout has (cw 4 and 8: 0; cw 16: 0 or 2, any other value is read as 2; cw 32: 0,
/// 2, 4 or 6, an odd value is read as the even one below it); writes `ac[0 .. cw * ch]`, rows of `cw`
/// coefficients, and reads them back for the subtraction.
pub unsafe extern "C" fn dav1d_ipred_cfl_ac_444_8bpc_neon(
    ac: *mut i16,
    ypx: *const u8,
    stride: isize,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
) {
    let (mut x0, mut x1, w3, w5, w6) = (ac.cast::<u16>(), ypx, w_pad, cw, ch);
    let mut w4 = h_pad << 2; // lsl w4, w4, #2
    let mut v16 = vdupq_n_u16(0); // movi v16.8h, #0
    let mut v17 = vdupq_n_u16(0); // movi v17.8h, #0
    let mut v18 = vdupq_n_u16(0); // movi v18.8h, #0
    let mut v19 = vdupq_n_u16(0); // movi v19.8h, #0
    let (mut w8, w9) = ipred_cfl_ac_head(w4, w5, w6);
    let mut x10 = x1.offset(stride); // add x10, x1, x2
    let x2 = stride << 1; // lsl x2, x2, #1

    // br x7 (ipred_cfl_ac_444_tbl, by the width)
    match w5 {
        4 => {
            // L(ipred_cfl_ac_444_w4):
            let mut v1;
            loop {
                // 1: Copy and expand input
                let t0 = pd_ld1_s2(x1, x10); // ld1 {v0.s}[0], [x1], x2; ld1 {v0.s}[1], [x10], x2
                x1 = x1.wrapping_offset(x2);
                x10 = x10.wrapping_offset(x2);
                let t1 = pd_ld1_s2(x1, x10); // ld1 {v1.s}[0], [x1], x2; ld1 {v1.s}[1], [x10], x2
                x1 = x1.wrapping_offset(x2);
                x10 = x10.wrapping_offset(x2);
                let v0 = vshll_n_u8::<3>(t0); // ushll v0.8h, v0.8b, #3
                v1 = vshll_n_u8::<3>(t1); // ushll v1.8h, v1.8b, #3
                let gt = pd_subs_gt(&mut w8, 4); // subs w8, w8, #4
                v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                vst1q_u16_x2(x0, uint16x8x2_t(v0, v1)); // st1 {v0.8h, v1.8h}, [x0], #32
                x0 = x0.add(16);
                if !gt {
                    break; // b.gt 1b
                }
            }
            let v1q = vreinterpretq_u64_u16(v1);
            let v0 = vreinterpretq_u16_u64(vtrn2q_u64(v1q, v1q)); // trn2 v0.2d, v1.2d, v1.2d
            let v1 = v0; // trn2 v1.2d, v1.2d, v1.2d
            ipred_cfl_ac_420_w4_hpad_8bpc_neon(x0, w6, w9, w4, v16, v17, v0, v1)
            // b L(ipred_cfl_ac_420_w4_hpad)
        }
        8 => {
            // L(ipred_cfl_ac_444_w8):
            let mut v3;
            loop {
                // 1: Copy and expand input
                let t0 = vld1_u8(x1); // ld1 {v0.8b}, [x1], x2
                x1 = x1.wrapping_offset(x2);
                let t1 = vld1_u8(x10); // ld1 {v1.8b}, [x10], x2
                x10 = x10.wrapping_offset(x2);
                let t2 = vld1_u8(x1); // ld1 {v2.8b}, [x1], x2
                x1 = x1.wrapping_offset(x2);
                let v0 = vshll_n_u8::<3>(t0); // ushll v0.8h, v0.8b, #3
                let t3 = vld1_u8(x10); // ld1 {v3.8b}, [x10], x2
                x10 = x10.wrapping_offset(x2);
                let v1 = vshll_n_u8::<3>(t1); // ushll v1.8h, v1.8b, #3
                let v2 = vshll_n_u8::<3>(t2); // ushll v2.8h, v2.8b, #3
                v3 = vshll_n_u8::<3>(t3); // ushll v3.8h, v3.8b, #3
                let gt = pd_subs_gt(&mut w8, 4); // subs w8, w8, #4
                vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                x0 = x0.add(32);
                v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                if !gt {
                    break; // b.gt 1b
                }
            }
            // mov v0.16b, v3.16b; mov v1.16b, v3.16b
            ipred_cfl_ac_420_w8_hpad_8bpc_neon(x0, w6, w9, w4, v16, v17, v18, v19, v3, v3)
            // b L(ipred_cfl_ac_420_w8_hpad)
        }
        16 => {
            // L(ipred_cfl_ac_444_w16):
            let (mut v6, mut v7);
            if w3 == 0 {
                // cbnz w3, L(ipred_cfl_ac_444_w16_wpad)
                loop {
                    // 1: Copy and expand input, without padding
                    let t0 = vld1q_u8(x1); // ld1 {v0.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t2 = vld1q_u8(x10); // ld1 {v2.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t4 = vld1q_u8(x1); // ld1 {v4.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let v1 = vshll_high_n_u8::<3>(t0); // ushll2 v1.8h, v0.16b, #3
                    let v0 = vshll_n_u8::<3>(vget_low_u8(t0)); // ushll v0.8h, v0.8b, #3
                    let t6 = vld1q_u8(x10); // ld1 {v6.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let v3 = vshll_high_n_u8::<3>(t2); // ushll2 v3.8h, v2.16b, #3
                    let v2 = vshll_n_u8::<3>(vget_low_u8(t2)); // ushll v2.8h, v2.8b, #3
                    let v5 = vshll_high_n_u8::<3>(t4); // ushll2 v5.8h, v4.16b, #3
                    let v4 = vshll_n_u8::<3>(vget_low_u8(t4)); // ushll v4.8h, v4.8b, #3
                    v7 = vshll_high_n_u8::<3>(t6); // ushll2 v7.8h, v6.16b, #3
                    v6 = vshll_n_u8::<3>(vget_low_u8(t6)); // ushll v6.8h, v6.8b, #3
                    let gt = pd_subs_gt(&mut w8, 4); // subs w8, w8, #4
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    vst1q_u16_x4(x0, uint16x8x4_t(v4, v5, v6, v7)); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v4); // add v16.8h, v16.8h, v4.8h
                    v17 = vaddq_u16(v17, v5); // add v17.8h, v17.8h, v5.8h
                    v18 = vaddq_u16(v18, v6); // add v18.8h, v18.8h, v6.8h
                    v19 = vaddq_u16(v19, v7); // add v19.8h, v19.8h, v7.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                }
            } else {
                // L(ipred_cfl_ac_444_w16_wpad):
                loop {
                    // 1: Copy and expand input, padding 8
                    let t0 = vld1_u8(x1); // ld1 {v0.8b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t2 = vld1_u8(x10); // ld1 {v2.8b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let t4 = vld1_u8(x1); // ld1 {v4.8b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t6 = vld1_u8(x10); // ld1 {v6.8b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let v0 = vshll_n_u8::<3>(t0); // ushll v0.8h, v0.8b, #3
                    let v2 = vshll_n_u8::<3>(t2); // ushll v2.8h, v2.8b, #3
                    let v4 = vshll_n_u8::<3>(t4); // ushll v4.8h, v4.8b, #3
                    v6 = vshll_n_u8::<3>(t6); // ushll v6.8h, v6.8b, #3
                    let v1 = vdupq_laneq_u16::<7>(v0); // dup v1.8h, v0.h[7]
                    let v3 = vdupq_laneq_u16::<7>(v2); // dup v3.8h, v2.h[7]
                    let v5 = vdupq_laneq_u16::<7>(v4); // dup v5.8h, v4.h[7]
                    v7 = vdupq_laneq_u16::<7>(v6); // dup v7.8h, v6.h[7]
                    let gt = pd_subs_gt(&mut w8, 4); // subs w8, w8, #4
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    vst1q_u16_x4(x0, uint16x8x4_t(v4, v5, v6, v7)); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v4); // add v16.8h, v16.8h, v4.8h
                    v17 = vaddq_u16(v17, v5); // add v17.8h, v17.8h, v5.8h
                    v18 = vaddq_u16(v18, v6); // add v18.8h, v18.8h, v6.8h
                    v19 = vaddq_u16(v19, v7); // add v19.8h, v19.8h, v7.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                }
            }
            // mov v0.16b, v6.16b; mov v1.16b, v7.16b; mov v2.16b, v6.16b; mov v3.16b, v7.16b
            ipred_cfl_ac_420_w16_hpad_8bpc_neon(x0, w6, w9, w4, v16, v17, v18, v19, v6, v7, v6, v7)
            // b L(ipred_cfl_ac_420_w16_hpad)
        }
        _ => {
            // L(ipred_cfl_ac_444_w32):
            let (mut v4, mut v5, mut v6, mut v7);
            // lsr w3, w3, #1; br x7 (ipred_cfl_ac_444_w32_tbl, by w_pad / 2)
            match w3 >> 1 {
                0 => loop {
                    // L(ipred_cfl_ac_444_w32_wpad0):
                    // 1: Copy and expand input, without padding
                    let t23 = vld1q_u8_x2(x1); // ld1 {v2.16b, v3.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t67 = vld1q_u8_x2(x10); // ld1 {v6.16b, v7.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let v0 = vshll_n_u8::<3>(vget_low_u8(t23.0)); // ushll v0.8h, v2.8b, #3
                    let v1 = vshll_high_n_u8::<3>(t23.0); // ushll2 v1.8h, v2.16b, #3
                    let v2 = vshll_n_u8::<3>(vget_low_u8(t23.1)); // ushll v2.8h, v3.8b, #3
                    let v3 = vshll_high_n_u8::<3>(t23.1); // ushll2 v3.8h, v3.16b, #3
                    v4 = vshll_n_u8::<3>(vget_low_u8(t67.0)); // ushll v4.8h, v6.8b, #3
                    v5 = vshll_high_n_u8::<3>(t67.0); // ushll2 v5.8h, v6.16b, #3
                    v6 = vshll_n_u8::<3>(vget_low_u8(t67.1)); // ushll v6.8h, v7.8b, #3
                    v7 = vshll_high_n_u8::<3>(t67.1); // ushll2 v7.8h, v7.16b, #3
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    vst1q_u16_x4(x0, uint16x8x4_t(v4, v5, v6, v7)); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v4); // add v16.8h, v16.8h, v4.8h
                    v17 = vaddq_u16(v17, v5); // add v17.8h, v17.8h, v5.8h
                    v18 = vaddq_u16(v18, v6); // add v18.8h, v18.8h, v6.8h
                    v19 = vaddq_u16(v19, v7); // add v19.8h, v19.8h, v7.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                1 => loop {
                    // L(ipred_cfl_ac_444_w32_wpad2):
                    // 1: Copy and expand input, padding 8
                    let t2 = vld1_u8(x1.add(16)); // ldr d2, [x1, #16]
                    let t1 = vld1q_u8(x1); // ld1 {v1.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t6 = vld1_u8(x10.add(16)); // ldr d6, [x10, #16]
                    let t5 = vld1q_u8(x10); // ld1 {v5.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let v2 = vshll_n_u8::<3>(t2); // ushll v2.8h, v2.8b, #3
                    let v0 = vshll_n_u8::<3>(vget_low_u8(t1)); // ushll v0.8h, v1.8b, #3
                    let v1 = vshll_high_n_u8::<3>(t1); // ushll2 v1.8h, v1.16b, #3
                    v6 = vshll_n_u8::<3>(t6); // ushll v6.8h, v6.8b, #3
                    v4 = vshll_n_u8::<3>(vget_low_u8(t5)); // ushll v4.8h, v5.8b, #3
                    v5 = vshll_high_n_u8::<3>(t5); // ushll2 v5.8h, v5.16b, #3
                    let v3 = vdupq_laneq_u16::<7>(v2); // dup v3.8h, v2.h[7]
                    v7 = vdupq_laneq_u16::<7>(v6); // dup v7.8h, v6.h[7]
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    vst1q_u16_x4(x0, uint16x8x4_t(v4, v5, v6, v7)); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v4); // add v16.8h, v16.8h, v4.8h
                    v17 = vaddq_u16(v17, v5); // add v17.8h, v17.8h, v5.8h
                    v18 = vaddq_u16(v18, v6); // add v18.8h, v18.8h, v6.8h
                    v19 = vaddq_u16(v19, v7); // add v19.8h, v19.8h, v7.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                2 => loop {
                    // L(ipred_cfl_ac_444_w32_wpad4):
                    // 1: Copy and expand input, padding 16
                    let t1 = vld1q_u8(x1); // ld1 {v1.16b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t5 = vld1q_u8(x10); // ld1 {v5.16b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let v0 = vshll_n_u8::<3>(vget_low_u8(t1)); // ushll v0.8h, v1.8b, #3
                    let v1 = vshll_high_n_u8::<3>(t1); // ushll2 v1.8h, v1.16b, #3
                    v4 = vshll_n_u8::<3>(vget_low_u8(t5)); // ushll v4.8h, v5.8b, #3
                    v5 = vshll_high_n_u8::<3>(t5); // ushll2 v5.8h, v5.16b, #3
                    let v2 = vdupq_laneq_u16::<7>(v1); // dup v2.8h, v1.h[7]
                    let v3 = v2; // dup v3.8h, v1.h[7]
                    v6 = vdupq_laneq_u16::<7>(v5); // dup v6.8h, v5.h[7]
                    v7 = v6; // dup v7.8h, v5.h[7]
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    vst1q_u16_x4(x0, uint16x8x4_t(v4, v5, v6, v7)); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v4); // add v16.8h, v16.8h, v4.8h
                    v17 = vaddq_u16(v17, v5); // add v17.8h, v17.8h, v5.8h
                    v18 = vaddq_u16(v18, v6); // add v18.8h, v18.8h, v6.8h
                    v19 = vaddq_u16(v19, v7); // add v19.8h, v19.8h, v7.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
                _ => loop {
                    // L(ipred_cfl_ac_444_w32_wpad6):
                    // 1: Copy and expand input, padding 24
                    let t0 = vld1_u8(x1); // ld1 {v0.8b}, [x1], x2
                    x1 = x1.wrapping_offset(x2);
                    let t4 = vld1_u8(x10); // ld1 {v4.8b}, [x10], x2
                    x10 = x10.wrapping_offset(x2);
                    let v0 = vshll_n_u8::<3>(t0); // ushll v0.8h, v0.8b, #3
                    v4 = vshll_n_u8::<3>(t4); // ushll v4.8h, v4.8b, #3
                    let v1 = vdupq_laneq_u16::<7>(v0); // dup v1.8h, v0.h[7]
                    let v2 = v1; // dup v2.8h, v0.h[7]
                    let v3 = v1; // dup v3.8h, v0.h[7]
                    v5 = vdupq_laneq_u16::<7>(v4); // dup v5.8h, v4.h[7]
                    v6 = v5; // dup v6.8h, v4.h[7]
                    v7 = v5; // dup v7.8h, v4.h[7]
                    let gt = pd_subs_gt(&mut w8, 2); // subs w8, w8, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v0, v1, v2, v3)); // st1 {v0.8h, v1.8h, v2.8h, v3.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v0); // add v16.8h, v16.8h, v0.8h
                    v17 = vaddq_u16(v17, v1); // add v17.8h, v17.8h, v1.8h
                    v18 = vaddq_u16(v18, v2); // add v18.8h, v18.8h, v2.8h
                    v19 = vaddq_u16(v19, v3); // add v19.8h, v19.8h, v3.8h
                    vst1q_u16_x4(x0, uint16x8x4_t(v4, v5, v6, v7)); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v4); // add v16.8h, v16.8h, v4.8h
                    v17 = vaddq_u16(v17, v5); // add v17.8h, v17.8h, v5.8h
                    v18 = vaddq_u16(v18, v6); // add v18.8h, v18.8h, v6.8h
                    v19 = vaddq_u16(v19, v7); // add v19.8h, v19.8h, v7.8h
                    if !gt {
                        break; // b.gt 1b
                    }
                },
            }
            // L(ipred_cfl_ac_444_w32_hpad):
            if w4 != 0 {
                // cbz w4, 3f
                loop {
                    // 2: Vertical padding (h_pad > 0)
                    let gt = pd_subs_gt(&mut w4, 2); // subs w4, w4, #2
                    vst1q_u16_x4(x0, uint16x8x4_t(v4, v5, v6, v7)); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v4); // add v16.8h, v16.8h, v4.8h
                    v17 = vaddq_u16(v17, v5); // add v17.8h, v17.8h, v5.8h
                    v18 = vaddq_u16(v18, v6); // add v18.8h, v18.8h, v6.8h
                    v19 = vaddq_u16(v19, v7); // add v19.8h, v19.8h, v7.8h
                    vst1q_u16_x4(x0, uint16x8x4_t(v4, v5, v6, v7)); // st1 {v4.8h, v5.8h, v6.8h, v7.8h}, [x0], #64
                    x0 = x0.add(32);
                    v16 = vaddq_u16(v16, v4); // add v16.8h, v16.8h, v4.8h
                    v17 = vaddq_u16(v17, v5); // add v17.8h, v17.8h, v5.8h
                    v18 = vaddq_u16(v18, v6); // add v18.8h, v18.8h, v6.8h
                    v19 = vaddq_u16(v19, v7); // add v19.8h, v19.8h, v7.8h
                    if !gt {
                        break; // b.gt 2b
                    }
                }
            }
            // 3:
            // Quadruple the height and reuse the w8 subtracting
            let w6 = w6 << 2; // lsl w6, w6, #2

            // Aggregate the sums, with wider intermediates earlier than in
            // ipred_cfl_ac_420_w8_calc_subtract_dc.
            let v0 = vpaddlq_u16(v16); // uaddlp v0.4s, v16.8h
            let v1 = vpaddlq_u16(v17); // uaddlp v1.4s, v17.8h
            let v2 = vpaddlq_u16(v18); // uaddlp v2.4s, v18.8h
            let v3 = vpaddlq_u16(v19); // uaddlp v3.4s, v19.8h
            let v0 = vaddq_u32(v0, v1); // add v0.4s, v0.4s, v1.4s
            let v2 = vaddq_u32(v2, v3); // add v2.4s, v2.4s, v3.4s
            let v0 = vaddq_u32(v0, v2); // add v0.4s, v0.4s, v2.4s
            let s0 = vaddvq_u32(v0); // addv s0, v0.4s: sum
            let x0 = x0.sub((w6 as u32 as usize) << 3); // sub x0, x0, w6, uxtw #4
            let v4 = ipred_cfl_ac_dc(s0, w9);
            ipred_cfl_ac_420_w8_subtract_dc_8bpc_neon(x0, w6, v4) // b L(ipred_cfl_ac_420_w8_subtract_dc)
        }
    }
}
