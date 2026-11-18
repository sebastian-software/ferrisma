// Copyright © 2018-2021, VideoLAN and dav1d authors
// Copyright © 2018-2021, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/x86/ipred16_avx2.asm` (16 bpc, used at 10
//! and 12 bit) with AVX2
//! intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! The 22 entry points the original assigns on an AVX2 machine:
//! `dav1d_ipred_{dc,dc_128,dc_top,dc_left,h,v,paeth,smooth,smooth_h,smooth_v,
//! z1,z2,z3,filter}_16bpc_avx2`, `dav1d_ipred_cfl{,_128,_top,_left}_16bpc_avx2`,
//! `dav1d_ipred_cfl_ac_{420,422,444}_16bpc_avx2` and `dav1d_pal_pred_16bpc_avx2`.
//! The directional predictors prepare (filter, upsample) their edge
//! themselves, as on x86 in the original; there is no C wrapper.
//!
//! Correspondence with the assembly (each section below starts with the
//! mapping of its labels and macros):
//! - Each `cglobal` is the `extern "C"` function of the same name. The
//!   vector registers are the register file `M` (`m[5]` is `m5`; an `xmN`
//!   operand is `lo(m[N])`, an `xmm` result is stored zero-extended with
//!   `zx`); the general-purpose registers are locals (or the fields of a
//!   small struct where they live across labels) named as `DEFINE_ARGS`
//!   names them. Every function but the exported ones is `#[inline(always)]`
//!   without `#[target_feature]`, so each entry point is one function of
//!   AVX2 code; the kernels contain no call.
//! - A jump table (`JMP_TABLE` and `movsxd`/`add`/`jmp`) is a `match` on the
//!   index in the table's order; a label that several places jump to or
//!   `call` is a function named `<cglobal>_<label>`; macros are functions
//!   of their name in lower case with register numbers as arguments.
//! - `SECTION_RODATA` up to the jump tables is one byte array, `RODATA`,
//!   with a constant per label (`ro(label, off)`), because the assembly reads
//!   across labels; its bytes are the original object's. `%define`d
//!   offsets into it (`pb_1`, `pw_8`, …) are offsets from their label.
//!   `dav1d_dr_intra_derivative` and `dav1d_filter_intra_taps` are the
//!   crate's tables.
//! - Helpers that are not assembly items carry the suffix of the section
//!   that defines them (`_q1` … `_q6`).
//!
//! Deviations from the assembly, and whether they change the count (the
//! sections list their own):
//! - `tzcnt`, `lzcnt`, `popcnt` are `trailing_zeros`, `leading_zeros`,
//!   `count_ones` and `shlx`/`shrx`/`sarx` plain shifts (masked where the
//!   assembly relies on the count's low five bits): the original selects
//!   these kernels on its AVX2 flag (AVX2, BMI1, BMI2), and the rebuild
//!   enables no feature beyond AVX2. `popcnt` (the edge filter strength of
//!   z1–z3) costs about 12 instructions more per use; `tzcnt` one.
//! - The jump tables' indices outside the contract (`w`, `h` powers of two
//!   from 4 to 64) go to the last arm (smooth_v: `unreachable!()`; the
//!   assembly's jump beyond its table is undefined).
//! - z3's frames are zeroed at the start of each path that uses them and
//!   16-byte aligned (all their accesses are unaligned moves).
//! - Stack frames (`ALLOC_STACK`) are locals zeroed whole, or written whole
//!   before the first load (said at each site). The z1 and z3 frames are
//!   larger than the assembly's: its main loops load lanes from beyond its
//!   frame and blend them away; here those bytes are zeros of the larger
//!   frame. A few 32-byte stores per call on the paths with a frame.
//! - Every vector access to caller memory is an unaligned move where the
//!   assembly uses `mova`; `movhps`/`pextr*`/`pinsr*`/`movd` to and from
//!   memory are unaligned scalar accesses of the same bytes. Same count.
//! - Instruction scheduling and register allocation are the compiler's.
//!
//! Reads beyond what the C kernels read (writes: exactly the block, and
//! for `cfl_ac` exactly the `cw*ch` values of `ac`), kept as the assembly
//! makes them; the values never reach the output. Derived from the code by
//! the sections, not checked with guard pages:
//! - the edge (`topleft`, in pixels): dc_top reads 16 pixels and v 64 at
//!   every width, dc_left 16 from `tl-h`, filter and cfl_top/cfl_left whole
//!   vectors (the filter from `tl[-(h+6)]` to `tl[34]`); z1 reads up to
//!   `topleft[191]`, z2 `topleft[-66..66]`, z3 down to `topleft[-191]`. In
//!   the decoder `topleft` is `edge_16bpc + 128` inside the task context's
//!   scratch structure, which has the bytes before and behind the array
//!   (`simd/x86/ipred.rs`, test `the_edge_array_has_readable_slack_around_it`);
//! - `cfl_ac` loads luma columns 8 to 15 where C reads 0 to 7 (420/422
//!   `.w16_wpad` with `wpad` 3, 444 `.w16` and `.w32_wpad`), inside the luma
//!   rows' stride.
//!
//! Safety (argued once for the family): the callers (`recon_tmpl`, the
//! test) pass a destination block of `width` x `height` pixels with its
//! stride in bytes, `topleft` as above, `width` and `height` powers of two
//! from 4 to 64 (the ones the original's tables have), the angle with its
//! flag bits, `ac` with `width * height` values and `alpha`, `bitdepth_max`
//! 1023 or 4095, `pal` with 8
//! entries and the packed indices, and for `cfl_ac` the luma rows with the
//! padding counts; the kernels read and write what the assembly reads and
//! writes (above).
#![allow(clippy::all, non_snake_case, non_upper_case_globals)]

use core::arch::x86_64::*;
use core::ffi::{c_int, c_uint};

// ---- registers, memory operands, read-only data ----

/// The vector registers `m0` … `m15` (`ymm`); `m[5]` is `m5`. An `xmN` operand is `lo(m[N])`; an instruction
/// with an `xmm` destination stores `zx(…)` (VEX clears the upper half).
pub(crate) type M = [__m256i; 16];

/// A fresh register file (all zero; the compiler drops the zeros of registers written before they are read).
#[inline(always)]
pub(crate) unsafe fn regs() -> M {
    [_mm256_setzero_si256(); 16]
}

/// `xmN`: the low 128 bits.
#[inline(always)]
pub(crate) unsafe fn lo(x: __m256i) -> __m128i {
    _mm256_castsi256_si128(x)
}

/// An `xmm` result written to `mN`: the upper half is zero.
#[inline(always)]
pub(crate) unsafe fn zx(x: __m128i) -> __m256i {
    _mm256_zextsi128_si256(x)
}

/// `p + off` bytes, wrapping (no claim that the address is in bounds until it is accessed).
#[inline(always)]
pub(crate) fn at<T>(p: *mut T, off: isize) -> *mut T {
    (p as *mut u8).wrapping_offset(off) as *mut T
}

/// `SECTION_RODATA` of `ipred16_avx2.asm` up to its jump tables, byte for byte (taken from the original's object
/// file): one array with a constant per label, because the assembly may address a table at offsets that reach
/// into the next one. The jump tables that follow (code addresses) are not part of it; they become `match`es.
#[repr(C, align(64))]
pub(crate) struct Rodata(pub(crate) [u8; 1244]);
pub(crate) static RODATA: Rodata = Rodata([
    0x00, 0x00, 0x00, 0x00, 0x80, 0x7f, 0x00, 0x40, 0x80, 0x7f, 0x80, 0x4a, 0x80, 0x2a, 0x00, 0x20, // 0x000
    0x80, 0x7f, 0x80, 0x62, 0x00, 0x49, 0x80, 0x34, 0x80, 0x24, 0x00, 0x19, 0x80, 0x12, 0x00, 0x10, // 0x010
    0x80, 0x7f, 0x80, 0x70, 0x00, 0x62, 0x00, 0x55, 0x80, 0x48, 0x80, 0x3d, 0x00, 0x33, 0x00, 0x2a, // 0x020
    0x00, 0x22, 0x00, 0x1b, 0x80, 0x15, 0x80, 0x10, 0x00, 0x0d, 0x00, 0x0a, 0x80, 0x08, 0x00, 0x08, // 0x030
    0x80, 0x7f, 0x00, 0x78, 0x80, 0x70, 0x00, 0x69, 0x00, 0x62, 0x00, 0x5b, 0x80, 0x54, 0x80, 0x4e, // 0x040
    0x80, 0x48, 0x80, 0x42, 0x00, 0x3d, 0x80, 0x37, 0x80, 0x32, 0x00, 0x2e, 0x80, 0x29, 0x00, 0x25, // 0x050
    0x00, 0x21, 0x80, 0x1d, 0x00, 0x1a, 0x80, 0x16, 0x80, 0x13, 0x00, 0x11, 0x80, 0x0e, 0x80, 0x0c, // 0x060
    0x80, 0x0a, 0x80, 0x08, 0x00, 0x07, 0x00, 0x06, 0x00, 0x05, 0x80, 0x04, 0x00, 0x04, 0x00, 0x04, // 0x070
    0x80, 0x7f, 0x00, 0x7c, 0x00, 0x78, 0x80, 0x74, 0x80, 0x70, 0x00, 0x6d, 0x00, 0x69, 0x80, 0x65, // 0x080
    0x00, 0x62, 0x80, 0x5e, 0x00, 0x5b, 0x00, 0x58, 0x80, 0x54, 0x80, 0x51, 0x00, 0x4e, 0x00, 0x4b, // 0x090
    0x00, 0x48, 0x00, 0x45, 0x80, 0x42, 0x80, 0x3f, 0x80, 0x3c, 0x00, 0x3a, 0x80, 0x37, 0x00, 0x35, // 0x0a0
    0x80, 0x32, 0x00, 0x30, 0x80, 0x2d, 0x00, 0x2b, 0x00, 0x29, 0x80, 0x26, 0x80, 0x24, 0x80, 0x22, // 0x0b0
    0x80, 0x20, 0x80, 0x1e, 0x80, 0x1c, 0x00, 0x1b, 0x00, 0x19, 0x80, 0x17, 0x00, 0x16, 0x80, 0x14, // 0x0c0
    0x00, 0x13, 0x80, 0x11, 0x00, 0x10, 0x80, 0x0e, 0x80, 0x0d, 0x80, 0x0c, 0x00, 0x0b, 0x00, 0x0a, // 0x0d0
    0x00, 0x09, 0x00, 0x08, 0x80, 0x07, 0x80, 0x06, 0x00, 0x06, 0x00, 0x05, 0x80, 0x04, 0x00, 0x04, // 0x0e0
    0x80, 0x03, 0x00, 0x03, 0x00, 0x03, 0x80, 0x02, 0x80, 0x02, 0x00, 0x02, 0x00, 0x02, 0x00, 0x02, // 0x0f0
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0xff, 0x00, 0x01, 0x00, 0x80, 0x00, 0x80, 0x00, // 0x100
    0xff, 0x00, 0x01, 0x00, 0x95, 0x00, 0x6b, 0x00, 0x55, 0x00, 0xab, 0x00, 0x40, 0x00, 0xc0, 0x00, // 0x110
    0xff, 0x00, 0x01, 0x00, 0xc5, 0x00, 0x3b, 0x00, 0x92, 0x00, 0x6e, 0x00, 0x69, 0x00, 0x97, 0x00, // 0x120
    0x49, 0x00, 0xb7, 0x00, 0x32, 0x00, 0xce, 0x00, 0x25, 0x00, 0xdb, 0x00, 0x20, 0x00, 0xe0, 0x00, // 0x130
    0xff, 0x00, 0x01, 0x00, 0xe1, 0x00, 0x1f, 0x00, 0xc4, 0x00, 0x3c, 0x00, 0xaa, 0x00, 0x56, 0x00, // 0x140
    0x91, 0x00, 0x6f, 0x00, 0x7b, 0x00, 0x85, 0x00, 0x66, 0x00, 0x9a, 0x00, 0x54, 0x00, 0xac, 0x00, // 0x150
    0x44, 0x00, 0xbc, 0x00, 0x36, 0x00, 0xca, 0x00, 0x2b, 0x00, 0xd5, 0x00, 0x21, 0x00, 0xdf, 0x00, // 0x160
    0x1a, 0x00, 0xe6, 0x00, 0x14, 0x00, 0xec, 0x00, 0x11, 0x00, 0xef, 0x00, 0x10, 0x00, 0xf0, 0x00, // 0x170
    0xff, 0x00, 0x01, 0x00, 0xf0, 0x00, 0x10, 0x00, 0xe1, 0x00, 0x1f, 0x00, 0xd2, 0x00, 0x2e, 0x00, // 0x180
    0xc4, 0x00, 0x3c, 0x00, 0xb6, 0x00, 0x4a, 0x00, 0xa9, 0x00, 0x57, 0x00, 0x9d, 0x00, 0x63, 0x00, // 0x190
    0x91, 0x00, 0x6f, 0x00, 0x85, 0x00, 0x7b, 0x00, 0x7a, 0x00, 0x86, 0x00, 0x6f, 0x00, 0x91, 0x00, // 0x1a0
    0x65, 0x00, 0x9b, 0x00, 0x5c, 0x00, 0xa4, 0x00, 0x53, 0x00, 0xad, 0x00, 0x4a, 0x00, 0xb6, 0x00, // 0x1b0
    0x42, 0x00, 0xbe, 0x00, 0x3b, 0x00, 0xc5, 0x00, 0x34, 0x00, 0xcc, 0x00, 0x2d, 0x00, 0xd3, 0x00, // 0x1c0
    0x27, 0x00, 0xd9, 0x00, 0x22, 0x00, 0xde, 0x00, 0x1d, 0x00, 0xe3, 0x00, 0x19, 0x00, 0xe7, 0x00, // 0x1d0
    0x15, 0x00, 0xeb, 0x00, 0x11, 0x00, 0xef, 0x00, 0x0e, 0x00, 0xf2, 0x00, 0x0c, 0x00, 0xf4, 0x00, // 0x1e0
    0x0a, 0x00, 0xf6, 0x00, 0x09, 0x00, 0xf7, 0x00, 0x08, 0x00, 0xf8, 0x00, 0x08, 0x00, 0xf8, 0x00, // 0x1f0
    0xff, 0x00, 0x01, 0x00, 0xf8, 0x00, 0x08, 0x00, 0xf0, 0x00, 0x10, 0x00, 0xe9, 0x00, 0x17, 0x00, // 0x200
    0xe1, 0x00, 0x1f, 0x00, 0xda, 0x00, 0x26, 0x00, 0xd2, 0x00, 0x2e, 0x00, 0xcb, 0x00, 0x35, 0x00, // 0x210
    0xc4, 0x00, 0x3c, 0x00, 0xbd, 0x00, 0x43, 0x00, 0xb6, 0x00, 0x4a, 0x00, 0xb0, 0x00, 0x50, 0x00, // 0x220
    0xa9, 0x00, 0x57, 0x00, 0xa3, 0x00, 0x5d, 0x00, 0x9c, 0x00, 0x64, 0x00, 0x96, 0x00, 0x6a, 0x00, // 0x230
    0x90, 0x00, 0x70, 0x00, 0x8a, 0x00, 0x76, 0x00, 0x85, 0x00, 0x7b, 0x00, 0x7f, 0x00, 0x81, 0x00, // 0x240
    0x79, 0x00, 0x87, 0x00, 0x74, 0x00, 0x8c, 0x00, 0x6f, 0x00, 0x91, 0x00, 0x6a, 0x00, 0x96, 0x00, // 0x250
    0x65, 0x00, 0x9b, 0x00, 0x60, 0x00, 0xa0, 0x00, 0x5b, 0x00, 0xa5, 0x00, 0x56, 0x00, 0xaa, 0x00, // 0x260
    0x52, 0x00, 0xae, 0x00, 0x4d, 0x00, 0xb3, 0x00, 0x49, 0x00, 0xb7, 0x00, 0x45, 0x00, 0xbb, 0x00, // 0x270
    0x41, 0x00, 0xbf, 0x00, 0x3d, 0x00, 0xc3, 0x00, 0x39, 0x00, 0xc7, 0x00, 0x36, 0x00, 0xca, 0x00, // 0x280
    0x32, 0x00, 0xce, 0x00, 0x2f, 0x00, 0xd1, 0x00, 0x2c, 0x00, 0xd4, 0x00, 0x29, 0x00, 0xd7, 0x00, // 0x290
    0x26, 0x00, 0xda, 0x00, 0x23, 0x00, 0xdd, 0x00, 0x20, 0x00, 0xe0, 0x00, 0x1d, 0x00, 0xe3, 0x00, // 0x2a0
    0x1b, 0x00, 0xe5, 0x00, 0x19, 0x00, 0xe7, 0x00, 0x16, 0x00, 0xea, 0x00, 0x14, 0x00, 0xec, 0x00, // 0x2b0
    0x12, 0x00, 0xee, 0x00, 0x10, 0x00, 0xf0, 0x00, 0x0f, 0x00, 0xf1, 0x00, 0x0d, 0x00, 0xf3, 0x00, // 0x2c0
    0x0c, 0x00, 0xf4, 0x00, 0x0a, 0x00, 0xf6, 0x00, 0x09, 0x00, 0xf7, 0x00, 0x08, 0x00, 0xf8, 0x00, // 0x2d0
    0x07, 0x00, 0xf9, 0x00, 0x06, 0x00, 0xfa, 0x00, 0x06, 0x00, 0xfa, 0x00, 0x05, 0x00, 0xfb, 0x00, // 0x2e0
    0x05, 0x00, 0xfb, 0x00, 0x04, 0x00, 0xfc, 0x00, 0x04, 0x00, 0xfc, 0x00, 0x04, 0x00, 0xfc, 0x00, // 0x2f0
    0x06, 0x07, 0x06, 0x07, 0x00, 0x01, 0x02, 0x03, 0x02, 0x03, 0x02, 0x03, 0x08, 0x09, 0x0a, 0x0b, // 0x300
    0x04, 0x05, 0x04, 0x05, 0x04, 0x05, 0x06, 0x07, 0x00, 0x01, 0x00, 0x01, 0x0c, 0x0d, 0x0e, 0x0f, // 0x310
    0x08, 0x09, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x0e, 0x0f, 0x0c, 0x0d, 0xff, 0xff, // 0x320
    0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x04, 0x05, 0x02, 0x03, 0xff, 0xff, // 0x330
    0x0c, 0x0d, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x0a, 0x0b, 0x08, 0x09, 0xff, 0xff, // 0x340
    0x00, 0x02, 0x04, 0x06, 0x08, 0x0a, 0x0c, 0x0e, 0x01, 0x03, 0x05, 0x07, 0x09, 0x0b, 0x0d, 0x0f, // 0x350
    0x00, 0x00, 0x40, 0x00, 0x80, 0x00, 0xc0, 0x00, 0x00, 0x01, 0x40, 0x01, 0x80, 0x01, 0xc0, 0x01, // 0x360
    0x00, 0x02, 0x40, 0x02, 0x80, 0x02, 0xc0, 0x02, 0x00, 0x03, 0x40, 0x03, 0x80, 0x03, 0xc0, 0x03, // 0x370
    0x37, 0x7f, 0x27, 0x7f, 0x27, 0x7f, 0x07, 0x0f, 0x1f, 0x07, 0x0f, 0x1f, 0x00, 0x03, 0x1f, 0x00, // 0x380
    0x27, 0x3f, 0x13, 0x2f, 0x13, 0x2f, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x00, 0x00, 0x00, 0x00, // 0x390
    0x07, 0x07, 0x0b, 0x0b, 0x0f, 0x0f, 0x13, 0x13, 0x13, 0x17, 0x17, 0x17, 0x1f, 0x1f, 0x1f, 0x27, // 0x3a0
    0x27, 0x27, 0x2f, 0x2f, 0x2f, 0x3f, 0x3f, 0x3f, 0x4f, 0x4f, 0x4f, 0xff, 0x00, 0xfc, 0x00, 0xfc, // 0x3b0
    0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00, // 0x3c0
    0x09, 0x00, 0x0a, 0x00, 0x0b, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x0e, 0x00, 0x0f, 0x00, 0x10, 0x00, // 0x3d0
    0x10, 0x00, 0x0f, 0x00, 0x0e, 0x00, 0x0d, 0x00, 0x0c, 0x00, 0x0b, 0x00, 0x0a, 0x00, 0x09, 0x00, // 0x3e0
    0x08, 0x00, 0x07, 0x00, 0x06, 0x00, 0x05, 0x00, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x00, // 0x3f0
    0x01, 0x00, 0x02, 0x00, 0x01, 0x00, 0x02, 0x00, 0x01, 0x00, 0x02, 0x00, 0x01, 0x00, 0x02, 0x00, // 0x400
    0x03, 0x00, 0x04, 0x00, 0x03, 0x00, 0x04, 0x00, 0x03, 0x00, 0x04, 0x00, 0x03, 0x00, 0x04, 0x00, // 0x410
    0x01, 0x00, 0x02, 0x00, 0x05, 0x00, 0x06, 0x00, 0x03, 0x00, 0x04, 0x00, 0x07, 0x00, 0x08, 0x00, // 0x420
    0x05, 0x00, 0x06, 0x00, 0x10, 0x00, 0x10, 0x00, 0x07, 0x00, 0x08, 0x00, 0x5a, 0x5a, 0x5a, 0x5a, // 0x430
    0x03, 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, // 0x440
    0x01, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, // 0x450
    0x00, 0x01, 0x04, 0x05, 0x08, 0x09, 0x0c, 0x0d, 0x02, 0x03, 0x06, 0x07, 0x0a, 0x0b, 0x0e, 0x0f, // 0x460
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, // 0x470
    0x06, 0x07, 0x0e, 0x0f, 0x04, 0x05, 0x0c, 0x0d, 0x04, 0x05, 0x0c, 0x0d, 0x02, 0x03, 0x0a, 0x0b, // 0x480
    0x06, 0x07, 0x0e, 0x0f, 0x02, 0x03, 0x0a, 0x0b, 0x04, 0x05, 0x0c, 0x0d, 0x00, 0x01, 0x08, 0x09, // 0x490
    0x04, 0x00, 0x04, 0x00, 0x05, 0x00, 0x05, 0x00, 0x04, 0x00, 0x04, 0x00, 0x08, 0x00, 0x08, 0x00, // 0x4a0
    0x06, 0x00, 0x06, 0x00, 0x04, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // 0x4b0
    0x02, 0x00, 0x02, 0x00, 0x01, 0x00, 0x01, 0x00, 0x03, 0x00, 0x03, 0x00, 0x3e, 0x00, 0x3e, 0x00, // 0x4c0
    0x00, 0x02, 0x00, 0x02, 0x00, 0x08, 0x00, 0x08, 0x08, 0x00, 0x00, 0x00, // 0x4d0
]);
pub(crate) const dav1d_smooth_weights_1d_16bpc: usize = 0x000;
pub(crate) const dav1d_smooth_weights_2d_16bpc: usize = 0x100;
pub(crate) const ipred_hv_shuf: usize = 0x300;
pub(crate) const filter_shuf1: usize = 0x320;
pub(crate) const filter_shuf2: usize = 0x330;
pub(crate) const filter_shuf3: usize = 0x340;
pub(crate) const pal_pred_shuf: usize = 0x350;
pub(crate) const z_base_inc: usize = 0x360;
pub(crate) const z_filter_t0: usize = 0x380;
pub(crate) const z_filter_t1: usize = 0x390;
pub(crate) const z_filter_wh: usize = 0x3a0;
pub(crate) const pw_m1024: usize = 0x3bc;
pub(crate) const pw_1to16: usize = 0x3c0;
pub(crate) const pw_16to1: usize = 0x3e0;
pub(crate) const z2_ymul: usize = 0x400;
pub(crate) const z2_ymul8: usize = 0x420;
pub(crate) const pb_90: usize = 0x43c;
pub(crate) const z2_y_shuf_h4: usize = 0x440;
pub(crate) const z_upsample: usize = 0x460;
pub(crate) const z2_x_shuf: usize = 0x470;
pub(crate) const z2_y_shuf: usize = 0x480;
pub(crate) const z2_y_shuf_us: usize = 0x490;
pub(crate) const z_filter_k: usize = 0x4a0;
pub(crate) const pw_1: usize = 0x4c4;
pub(crate) const pw_3: usize = 0x4c8;
pub(crate) const pw_62: usize = 0x4cc;
pub(crate) const pw_512: usize = 0x4d0;
pub(crate) const pw_2048: usize = 0x4d4;
pub(crate) const pd_8: usize = 0x4d8;

/// `[label+off]`: the address of a read-only operand. Every access through it must lie inside `RODATA`; say at
/// the use why it does when the offset is not a constant.
#[inline(always)]
pub(crate) fn ro(label: usize, off: isize) -> *const u8 {
    RODATA.0.as_ptr().wrapping_add(label).wrapping_offset(off)
}

// ---- part 1: `ipred16_avx2.asm` lines 138–649 (dc_top, dc_left, dc, dc_128, v, IPRED_H/h, PAETH/paeth) and 4870–4987 (pal_pred) ----

// Memory operands of part 1 (unaligned accesses of the bytes the assembly touches; `p` is a byte address).

/// `movd`-style 16-bit read for `vpbroadcastw m, [p]`.
#[inline(always)]
unsafe fn rd16_q1(p: *const u8) -> i16 {
    p.cast::<i16>().read_unaligned()
}

/// 32-bit read for `vpbroadcastd m, [p]`.
#[inline(always)]
unsafe fn rd32_q1(p: *const u8) -> i32 {
    p.cast::<i32>().read_unaligned()
}

/// 64-bit read for `vpbroadcastq m, [p]`.
#[inline(always)]
unsafe fn rd64_q1(p: *const u8) -> i64 {
    p.cast::<i64>().read_unaligned()
}

/// `movu`/`mova m, [p]` (32 bytes).
#[inline(always)]
unsafe fn ld32_q1(p: *const u8) -> __m256i {
    _mm256_loadu_si256(p.cast())
}

/// `movu`/`mova xm, [p]` (16 bytes).
#[inline(always)]
unsafe fn ld16_q1(p: *const u8) -> __m128i {
    _mm_loadu_si128(p.cast())
}

/// `movq xm, [p]` (8 bytes, upper part zero).
#[inline(always)]
unsafe fn ld8_q1(p: *const u8) -> __m128i {
    _mm_loadl_epi64(p.cast())
}

/// `mova [p], m` (32 bytes).
#[inline(always)]
unsafe fn st32_q1(p: *mut u8, x: __m256i) {
    _mm256_storeu_si256(p.cast(), x)
}

/// `mova [p], xm` / `vextracti128 [p], m, 1` (16 bytes).
#[inline(always)]
unsafe fn st16_q1(p: *mut u8, x: __m128i) {
    _mm_storeu_si128(p.cast(), x)
}

/// `movq [p], xm` (the low 8 bytes).
#[inline(always)]
unsafe fn st8_q1(p: *mut u8, x: __m128i) {
    _mm_storel_epi64(p.cast(), x)
}

/// `movhps [p], xm` (the high 8 bytes).
#[inline(always)]
unsafe fn st8h_q1(p: *mut u8, x: __m128i) {
    _mm_storeh_pd(p.cast(), _mm_castsi128_pd(x))
}

// ---- the splat labels of `ipred_dc_16bpc` (`ipred_dc_splat_16bpc_avx2_table`: `.s4` … `.s64`) ----
// They store m0 (`.s32`: m0, m1; `.s64`: m0 … m3) to every row; dc_top, dc_left, dc_128 and v jump into them.

/// `.s4`: four rows of four pixels per iteration.
#[inline(always)]
unsafe fn ipred_dc_16bpc_s4(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, mut h: i32) {
    loop {
        st8_q1(dst, lo(m[0])); //                                       movq   [dstq+strideq*0], xm0
        st8_q1(at(dst, stride), lo(m[0])); //                           movq   [dstq+strideq*1], xm0
        st8_q1(at(dst, stride * 2), lo(m[0])); //                       movq   [dstq+strideq*2], xm0
        st8_q1(at(dst, stride3), lo(m[0])); //                          movq   [dstq+stride3q ], xm0
        dst = at(dst, stride * 4); //                                   lea    dstq, [dstq+strideq*4]
        h -= 4; //                                                      sub    hd, 4
        if h <= 0 {
            break; //                                                   jg .s4
        }
    }
}

/// `.s8`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_s8(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, mut h: i32) {
    loop {
        st16_q1(dst, lo(m[0])); //                                      mova   [dstq+strideq*0], xm0
        st16_q1(at(dst, stride), lo(m[0])); //                          mova   [dstq+strideq*1], xm0
        st16_q1(at(dst, stride * 2), lo(m[0])); //                      mova   [dstq+strideq*2], xm0
        st16_q1(at(dst, stride3), lo(m[0])); //                         mova   [dstq+stride3q ], xm0
        dst = at(dst, stride * 4); //                                   lea    dstq, [dstq+strideq*4]
        h -= 4; //                                                      sub    hd, 4
        if h <= 0 {
            break; //                                                   jg .s8
        }
    }
}

/// `.s16`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_s16(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, mut h: i32) {
    loop {
        st32_q1(dst, m[0]); //                                          mova   [dstq+strideq*0], m0
        st32_q1(at(dst, stride), m[0]); //                              mova   [dstq+strideq*1], m0
        st32_q1(at(dst, stride * 2), m[0]); //                          mova   [dstq+strideq*2], m0
        st32_q1(at(dst, stride3), m[0]); //                             mova   [dstq+stride3q ], m0
        dst = at(dst, stride * 4); //                                   lea    dstq, [dstq+strideq*4]
        h -= 4; //                                                      sub    hd, 4
        if h <= 0 {
            break; //                                                   jg .s16
        }
    }
}

/// `.s32`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_s32(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, mut h: i32) {
    loop {
        st32_q1(dst, m[0]); //                                          mova [dstq+strideq*0+32*0], m0
        st32_q1(at(dst, 32), m[1]); //                                  mova [dstq+strideq*0+32*1], m1
        st32_q1(at(dst, stride), m[0]); //                              mova [dstq+strideq*1+32*0], m0
        st32_q1(at(dst, stride + 32), m[1]); //                         mova [dstq+strideq*1+32*1], m1
        st32_q1(at(dst, stride * 2), m[0]); //                          mova [dstq+strideq*2+32*0], m0
        st32_q1(at(dst, stride * 2 + 32), m[1]); //                     mova [dstq+strideq*2+32*1], m1
        st32_q1(at(dst, stride3), m[0]); //                             mova [dstq+stride3q +32*0], m0
        st32_q1(at(dst, stride3 + 32), m[1]); //                        mova [dstq+stride3q +32*1], m1
        dst = at(dst, stride * 4); //                                   lea    dstq, [dstq+strideq*4]
        h -= 4; //                                                      sub    hd, 4
        if h <= 0 {
            break; //                                                   jg .s32
        }
    }
}

/// `.s64`: two rows of 64 pixels per iteration.
#[inline(always)]
unsafe fn ipred_dc_16bpc_s64(m: &mut M, mut dst: *mut u8, stride: isize, mut h: i32) {
    loop {
        st32_q1(dst, m[0]); //                                          mova [dstq+strideq*0+32*0], m0
        st32_q1(at(dst, 32), m[1]); //                                  mova [dstq+strideq*0+32*1], m1
        st32_q1(at(dst, 64), m[2]); //                                  mova [dstq+strideq*0+32*2], m2
        st32_q1(at(dst, 96), m[3]); //                                  mova [dstq+strideq*0+32*3], m3
        st32_q1(at(dst, stride), m[0]); //                              mova [dstq+strideq*1+32*0], m0
        st32_q1(at(dst, stride + 32), m[1]); //                         mova [dstq+strideq*1+32*1], m1
        st32_q1(at(dst, stride + 64), m[2]); //                         mova [dstq+strideq*1+32*2], m2
        st32_q1(at(dst, stride + 96), m[3]); //                         mova [dstq+strideq*1+32*3], m3
        dst = at(dst, stride * 2); //                                   lea    dstq, [dstq+strideq*2]
        h -= 2; //                                                      sub    hd, 2
        if h <= 0 {
            break; //                                                   jg .s64
        }
    }
}

/// `ipred_dc_splat_16bpc_avx2_table`: `movsxd wq, [r5+wq*4]` / `add wq, r5` / `jmp wq`, indexed by `tzcnt(w)`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_splat(m: &mut M, dst: *mut u8, stride: isize, stride3: isize, wi: u32, h: i32) {
    match wi {
        2 => ipred_dc_16bpc_s4(m, dst, stride, stride3, h),
        3 => ipred_dc_16bpc_s8(m, dst, stride, stride3, h),
        4 => ipred_dc_16bpc_s16(m, dst, stride, stride3, h),
        5 => ipred_dc_16bpc_s32(m, dst, stride, stride3, h),
        _ => ipred_dc_16bpc_s64(m, dst, stride, h),
    }
}

// ---- ipred_dc_left_16bpc's labels `.h64` … `.h4` (a fall-through chain; dc_top enters it by `w`) ----

/// `.h4` (and the end of every longer chain): sums the low four words of `xm0`, rounds, shifts and broadcasts
/// the average to m0 … m3.
#[inline(always)]
unsafe fn ipred_dc_left_16bpc_h4(m: &mut M) {
    m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[3]))); //             punpcklwd xm0, xm3
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); //                     psrlq  xm1, xm0, 32
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[1] = zx(_mm_srli_si128::<8>(lo(m[0]))); //                      psrldq xm1, xm0, 8
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[4]))); //                  paddd  xm0, xm4
    m[0] = zx(_mm_srl_epi32(lo(m[0]), lo(m[5]))); //                  psrld  xm0, xm5
                                                  // lea stride3q, [strideq*3]: computed by the caller before the splat
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); //                       vpbroadcastw m0, xm0
    m[1] = m[0]; //                                                    mova   m1, m0
    m[2] = m[0]; //                                                    mova   m2, m0
    m[3] = m[0]; //                                                    mova   m3, m0
}

/// `.h8`.
#[inline(always)]
unsafe fn ipred_dc_left_16bpc_h8(m: &mut M) {
    m[1] = zx(_mm_srli_si128::<8>(lo(m[0]))); //                      psrldq xm1, xm0, 8
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //                  paddw  xm0, xm1
    ipred_dc_left_16bpc_h4(m)
}

/// `.h16`.
#[inline(always)]
unsafe fn ipred_dc_left_16bpc_h16(m: &mut M) {
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); //                vextracti128 xm1, m0, 1
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //                  paddw  xm0, xm1
    ipred_dc_left_16bpc_h8(m)
}

/// `.h32`.
#[inline(always)]
unsafe fn ipred_dc_left_16bpc_h32(m: &mut M, tl: *const u8) {
    m[0] = _mm256_add_epi16(m[0], ld32_q1(tl.wrapping_add(32))); //    paddw  m0, [tlq+32]
    ipred_dc_left_16bpc_h16(m)
}

/// `.h64`.
#[inline(always)]
unsafe fn ipred_dc_left_16bpc_h64(m: &mut M, tl: *const u8) {
    m[0] = _mm256_add_epi16(m[0], ld32_q1(tl.wrapping_add(96))); //    paddw  m0, [tlq+96]
    m[0] = _mm256_add_epi16(m[0], ld32_q1(tl.wrapping_add(64))); //    paddw  m0, [tlq+64]
    ipred_dc_left_16bpc_h32(m, tl)
}

/// `ipred_dc_left_16bpc_avx2_table`: `movsxd r6, [r5+…*4]` / `add r6, r5` / `jmp r6`, indexed by `tzcnt(h)`
/// (dc_left) or `tzcnt(w)` (dc_top).
#[inline(always)]
unsafe fn ipred_dc_left_16bpc_table(m: &mut M, tl: *const u8, i: u32) {
    match i {
        2 => ipred_dc_left_16bpc_h4(m),
        3 => ipred_dc_left_16bpc_h8(m),
        4 => ipred_dc_left_16bpc_h16(m),
        5 => ipred_dc_left_16bpc_h32(m, tl),
        _ => ipred_dc_left_16bpc_h64(m, tl),
    }
}

/// `ipred_dc_top_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_dc_top_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let mut m = regs();
    let dst = dst.cast::<u8>();
    let h = height; //                                                 movifnidn hd, hm
    let tl = topleft.cast::<u8>().wrapping_add(2); //                  add    tlq, 2
    m[4] = zx(_mm_cvtsi32_si128(width)); //                           movd   xm4, wd
    m[3] = zx(_mm_setzero_si128()); //                                pxor   xm3, xm3
    m[4] = zx(_mm_avg_epu16(lo(m[4]), lo(m[3]))); //                  pavgw  xm4, xm3
    let w = (width as u32).trailing_zeros(); //                        tzcnt  wd, wd (w is a power of two >= 4)
    m[5] = zx(_mm_cvtsi32_si128(w as i32)); //                         movd   xm5, wd
    m[0] = ld32_q1(tl); //                                             movu   m0, [tlq]
    ipred_dc_left_16bpc_table(&mut m, tl, w); //                       lea r5, [..left_table]; movsxd r6, [r5+wq*4]; jmp r6
    let stride3 = stride * 3; // lea stride3q, [strideq*3] (in .h4)
    ipred_dc_16bpc_splat(&mut m, dst, stride, stride3, w, h); //       movsxd wq, [r5+wq*4] (splat table); jmp wq
}

/// `ipred_dc_left_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_dc_left_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let mut m = regs();
    let dst = dst.cast::<u8>();
    let h = height; //                                                 mov    hd, hm
    let mut tl = topleft.cast::<u8>();
    tl = tl.wrapping_sub(h as usize); //                               sub    tlq, hq
    m[4] = zx(_mm_cvtsi32_si128(h)); //                               movd   xm4, hd
    tl = tl.wrapping_sub(h as usize); //                               sub    tlq, hq
    m[3] = zx(_mm_setzero_si128()); //                                pxor   xm3, xm3
    m[4] = zx(_mm_avg_epu16(lo(m[4]), lo(m[3]))); //                  pavgw  xm4, xm3
    let r6 = (h as u32).trailing_zeros(); //                           tzcnt  r6d, hd (h is a power of two >= 4)
    m[5] = zx(_mm_cvtsi32_si128(r6 as i32)); //                        movd   xm5, r6d
    m[0] = ld32_q1(tl); //                                             movu   m0, [tlq]
    let w = (width as u32).trailing_zeros(); //                        tzcnt  wd, wd (w is a power of two >= 4)
    ipred_dc_left_16bpc_table(&mut m, tl, r6); //                      movsxd r6, [r5+r6*4]; add r6, r5; jmp r6
    let stride3 = stride * 3; // lea stride3q, [strideq*3] (in .h4)
    ipred_dc_16bpc_splat(&mut m, dst, stride, stride3, w, h); //       movsxd wq, [r5+wq*4] (splat table); jmp wq
}

// ---- ipred_dc_16bpc: the loads `.h4` … `.h64` of the left column, then the sums `.w4` … `.w64` ----

/// `.h4`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_h4(m: &mut M, tl: *const u8) {
    m[0] = zx(ld8_q1(tl.wrapping_sub(8))); //                          movq   xm0, [tlq-8]
}

/// `.h8`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_h8(m: &mut M, tl: *const u8) {
    m[0] = zx(ld16_q1(tl.wrapping_sub(16))); //                        mova   xm0, [tlq-16]
}

/// `.h16`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_h16(m: &mut M, tl: *const u8) {
    m[0] = ld32_q1(tl.wrapping_sub(32)); //                            mova   m0, [tlq-32]
}

/// `.h32`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_h32(m: &mut M, tl: *const u8) {
    m[0] = ld32_q1(tl.wrapping_sub(64)); //                            mova   m0, [tlq-64]
    m[0] = _mm256_add_epi16(m[0], ld32_q1(tl.wrapping_sub(32))); //    paddw  m0, [tlq-32]
}

/// `.h64`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_h64(m: &mut M, tl: *const u8) {
    m[0] = ld32_q1(tl.wrapping_sub(128)); //                           mova   m0, [tlq-128]
    m[1] = ld32_q1(tl.wrapping_sub(96)); //                            mova   m1, [tlq- 96]
    m[0] = _mm256_add_epi16(m[0], ld32_q1(tl.wrapping_sub(64))); //    paddw  m0, [tlq- 64]
    m[1] = _mm256_add_epi16(m[1], ld32_q1(tl.wrapping_sub(32))); //    paddw  m1, [tlq- 32]
    m[0] = _mm256_add_epi16(m[0], m[1]); //                            paddw  m0, m1
}

/// `.w4`, `.w4_mul`, `.w4_end`, then `.s4`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_w4(m: &mut M, dst: *mut u8, stride: isize, stride3: isize, tl: *const u8, h: i32) {
    m[1] = zx(ld8_q1(tl.wrapping_add(2))); //                          movq   xm1, [tlq+2]
    m[0] = _mm256_add_epi16(m[0], m[4]); //                            paddw  m0, m4
    m[0] = _mm256_add_epi16(m[0], m[1]); //                            paddw  m0, m1
    m[1] = _mm256_srli_epi64::<32>(m[0]); //                           psrlq  m1, m0, 32
    m[0] = _mm256_add_epi16(m[0], m[1]); //                            paddw  m0, m1
    m[1] = _mm256_srli_epi32::<16>(m[0]); //                           psrld  m1, m0, 16
    m[0] = _mm256_add_epi16(m[0], m[1]); //                            paddw  m0, m1
    if h > 4 {
        //                                                             cmp hd, 4; jg .w4_mul
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); //            vextracti128 xm1, m0, 1
        m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //              paddw  xm0, xm1
        let r2 = (h * 2) as u32; //                                    lea    r2d, [hq*2]
        let mut r6 = 0xAAAB6667u32; //                                 mov    r6d, 0xAAAB6667
        r6 = r6.wrapping_shr(r2); //                                   shrx   r6d, r6d, r2d (count mod 32)
        m[1] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[3]))); //         punpckhwd xm1, xm0, xm3
        m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[3]))); //         punpcklwd xm0, xm3
        m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //              paddd  xm0, xm1
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); //                    movd   xm1, r6d
        m[0] = zx(_mm_srli_epi32::<2>(lo(m[0]))); //                  psrld  xm0, 2
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); //            pmulhuw xm0, xm1
        m[0] = zx(_mm_srli_epi16::<1>(lo(m[0]))); //                  psrlw  xm0, 1
    } else {
        m[0] = zx(_mm_srli_epi16::<3>(lo(m[0]))); //                  psrlw  xm0, 3; jmp .w4_end
    }
    m[0] = zx(_mm_broadcastw_epi16(lo(m[0]))); //                     .w4_end: vpbroadcastw xm0, xm0
    ipred_dc_16bpc_s4(m, dst, stride, stride3, h)
}

/// `.w8`, `.w8_end`, then `.s8`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_w8(m: &mut M, dst: *mut u8, stride: isize, stride3: isize, tl: *const u8, h: i32) {
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); //                vextracti128 xm1, m0, 1
    m[0] = zx(_mm_add_epi16(lo(m[0]), ld16_q1(tl.wrapping_add(2)))); // paddw xm0, [tlq+2]
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[4]))); //                  paddw  xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //                  paddw  xm0, xm1
    m[1] = zx(_mm_srli_epi32::<16>(lo(m[0]))); //                     psrld  xm1, xm0, 16
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //                  paddw  xm0, xm1
    m[0] = zx(_mm_blend_epi16::<0xAA>(lo(m[0]), lo(m[3]))); //        pblendw xm0, xm3, 0xAA
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); //                     psrlq  xm1, xm0, 32
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[1] = zx(_mm_srli_si128::<8>(lo(m[0]))); //                      psrldq xm1, xm0, 8
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[0] = zx(_mm_srl_epi32(lo(m[0]), lo(m[5]))); //                  psrld  xm0, xm5
    if h != 8 {
        //                                                             cmp hd, 8; je .w8_end
        let mut r6 = 0xAAABu32; //                                     mov    r6d, 0xAAAB
        let r2 = 0x6667u32; //                                         mov    r2d, 0x6667
        if h == 32 {
            r6 = r2; //                                                cmp hd, 32; cmovz r6d, r2d
        }
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); //                    movd   xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); //            pmulhuw xm0, xm1
        m[0] = zx(_mm_srli_epi16::<1>(lo(m[0]))); //                  psrlw  xm0, 1
    }
    m[0] = zx(_mm_broadcastw_epi16(lo(m[0]))); //                     .w8_end: vpbroadcastw xm0, xm0
    ipred_dc_16bpc_s8(m, dst, stride, stride3, h)
}

/// `.w16`, `.w16_end`, then `.s16`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_w16(m: &mut M, dst: *mut u8, stride: isize, stride3: isize, tl: *const u8, h: i32) {
    m[0] = _mm256_add_epi16(m[0], ld32_q1(tl.wrapping_add(2))); //     paddw  m0, [tlq+2]
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); //                vextracti128 xm1, m0, 1
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[4]))); //                  paddw  xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //                  paddw  xm0, xm1
    m[1] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[3]))); //             punpckhwd xm1, xm0, xm3
    m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[3]))); //             punpcklwd xm0, xm3
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); //                     psrlq  xm1, xm0, 32
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[1] = zx(_mm_srli_si128::<8>(lo(m[0]))); //                      psrldq xm1, xm0, 8
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[0] = zx(_mm_srl_epi32(lo(m[0]), lo(m[5]))); //                  psrld  xm0, xm5
    if h != 16 {
        //                                                             cmp hd, 16; je .w16_end
        let mut r6 = 0xAAABu32; //                                     mov    r6d, 0xAAAB
        let r2 = 0x6667u32; //                                         mov    r2d, 0x6667
        if h & (8 | 32) == 0 {
            r6 = r2; //                                                test hb, 8|32; cmovz r6d, r2d
        }
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); //                    movd   xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); //            pmulhuw xm0, xm1
        m[0] = zx(_mm_srli_epi16::<1>(lo(m[0]))); //                  psrlw  xm0, 1
    }
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); //                       .w16_end: vpbroadcastw m0, xm0
    ipred_dc_16bpc_s16(m, dst, stride, stride3, h)
}

/// `.w32`, `.w32_end`, then `.s32`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_w32(m: &mut M, dst: *mut u8, stride: isize, stride3: isize, tl: *const u8, h: i32) {
    m[0] = _mm256_add_epi16(m[0], ld32_q1(tl.wrapping_add(2))); //     paddw  m0, [tlq+ 2]
    m[0] = _mm256_add_epi16(m[0], ld32_q1(tl.wrapping_add(34))); //    paddw  m0, [tlq+34]
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); //                vextracti128 xm1, m0, 1
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[4]))); //                  paddw  xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //                  paddw  xm0, xm1
    m[1] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[3]))); //             punpcklwd xm1, xm0, xm3
    m[0] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[3]))); //             punpckhwd xm0, xm3
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); //                     psrlq  xm1, xm0, 32
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[1] = zx(_mm_srli_si128::<8>(lo(m[0]))); //                      psrldq xm1, xm0, 8
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[0] = zx(_mm_srl_epi32(lo(m[0]), lo(m[5]))); //                  psrld  xm0, xm5
    if h != 32 {
        //                                                             cmp hd, 32; je .w32_end
        let r2 = (h * 2) as u32; //                                    lea    r2d, [hq*2]
        let mut r6 = 0x6667AAABu32; //                                 mov    r6d, 0x6667AAAB
        r6 = r6.wrapping_shr(r2); //                                   shrx   r6d, r6d, r2d (count mod 32)
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); //                    movd   xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); //            pmulhuw xm0, xm1
        m[0] = zx(_mm_srli_epi16::<1>(lo(m[0]))); //                  psrlw  xm0, 1
    }
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); //                       .w32_end: vpbroadcastw m0, xm0
    m[1] = m[0]; //                                                    mova   m1, m0
    ipred_dc_16bpc_s32(m, dst, stride, stride3, h)
}

/// `.w64`, `.w64_end`, then `.s64`.
#[inline(always)]
unsafe fn ipred_dc_16bpc_w64(m: &mut M, dst: *mut u8, stride: isize, tl: *const u8, h: i32) {
    m[1] = ld32_q1(tl.wrapping_add(2)); //                             movu   m1, [tlq+ 2]
    m[0] = _mm256_add_epi16(m[0], ld32_q1(tl.wrapping_add(34))); //    paddw  m0, [tlq+34]
    m[1] = _mm256_add_epi16(m[1], ld32_q1(tl.wrapping_add(66))); //    paddw  m1, [tlq+66]
    m[0] = _mm256_add_epi16(m[0], ld32_q1(tl.wrapping_add(98))); //    paddw  m0, [tlq+98]
    m[0] = _mm256_add_epi16(m[0], m[1]); //                            paddw  m0, m1
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); //                vextracti128 xm1, m0, 1
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //                  paddw  xm0, xm1
    m[1] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[3]))); //             punpcklwd xm1, xm0, xm3
    m[0] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[3]))); //             punpckhwd xm0, xm3
    m[1] = zx(_mm_add_epi32(lo(m[1]), lo(m[4]))); //                  paddd  xm1, xm4
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); //                     psrlq  xm1, xm0, 32
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[1] = zx(_mm_srli_si128::<8>(lo(m[0]))); //                      psrldq xm1, xm0, 8
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); //                  paddd  xm0, xm1
    m[0] = zx(_mm_srl_epi32(lo(m[0]), lo(m[5]))); //                  psrld  xm0, xm5
    if h != 64 {
        //                                                             cmp hd, 64; je .w64_end
        let mut r6 = 0x6667AAABu32; //                                 mov    r6d, 0x6667AAAB
        r6 = r6.wrapping_shr(h as u32); //                             shrx   r6d, r6d, hd (count mod 32)
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); //                    movd   xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); //            pmulhuw xm0, xm1
        m[0] = zx(_mm_srli_epi16::<1>(lo(m[0]))); //                  psrlw  xm0, 1
    }
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); //                       .w64_end: vpbroadcastw m0, xm0
    m[1] = m[0]; //                                                    mova   m1, m0
    m[2] = m[0]; //                                                    mova   m2, m0
    m[3] = m[0]; //                                                    mova   m3, m0
    ipred_dc_16bpc_s64(m, dst, stride, h)
}

/// `ipred_dc_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_dc_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let mut m = regs();
    let dst = dst.cast::<u8>();
    let tl = topleft.cast::<u8>();
    let h = height; //                                                 movifnidn hd, hm
    let r6 = (h as u32).trailing_zeros(); //                           tzcnt  r6d, hd (h is a power of two >= 4)
    let r5 = (width + h) as u32; //                                    lea    r5d, [wq+hq]
    m[4] = zx(_mm_cvtsi32_si128(r5 as i32)); //                        movd   xm4, r5d
    let r5 = r5.trailing_zeros(); //                                   tzcnt  r5d, r5d (w + h >= 8)
    m[5] = zx(_mm_cvtsi32_si128(r5 as i32)); //                        movd   xm5, r5d
    let w = (width as u32).trailing_zeros(); //                        tzcnt  wd, wd (w is a power of two >= 4)
                                             // movsxd r6, [r5+r6*4]; movsxd wq, [r5+wq*4+5*4]: the two `match`es below
    m[3] = _mm256_setzero_si256(); //                                  pxor   m3, m3
    m[4] = zx(_mm_srli_epi16::<1>(lo(m[4]))); //                       psrlw  xm4, 1
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
    match r6 {
        //                                                             jmp    r6
        2 => ipred_dc_16bpc_h4(&mut m, tl),
        3 => ipred_dc_16bpc_h8(&mut m, tl),
        4 => ipred_dc_16bpc_h16(&mut m, tl),
        5 => ipred_dc_16bpc_h32(&mut m, tl),
        _ => ipred_dc_16bpc_h64(&mut m, tl),
    }
    match w {
        //                                                             jmp    wq
        2 => ipred_dc_16bpc_w4(&mut m, dst, stride, stride3, tl, h),
        3 => ipred_dc_16bpc_w8(&mut m, dst, stride, stride3, tl, h),
        4 => ipred_dc_16bpc_w16(&mut m, dst, stride, stride3, tl, h),
        5 => ipred_dc_16bpc_w32(&mut m, dst, stride, stride3, tl, h),
        _ => ipred_dc_16bpc_w64(&mut m, dst, stride, tl, h),
    }
}

/// `ipred_dc_128_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_dc_128_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    _topleft: *const u16,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
) {
    let mut m = regs();
    let dst = dst.cast::<u8>();
    let mut r6 = bitdepth_max as u32; //                               mov    r6d, r8m
    r6 >>= 11; //                                                      shr    r6d, 11 (0 at 10 bit, 1 at 12 bit)
    let w = (width as u32).trailing_zeros(); //                        tzcnt  wd, wd (w is a power of two >= 4)
    let h = height; //                                                 movifnidn hd, hm
                    // vpbroadcastd m0, [pw_512+r6*4]: r6 is 0 or 1 (bitdepth_max is 1023 or 4095), so the load is pw_512 or
                    // pw_2048 (the next label), inside RODATA.
    m[0] = _mm256_set1_epi32(rd32_q1(ro(pw_512, r6 as isize * 4)));
    m[1] = m[0]; //                                                    mova   m1, m0
    m[2] = m[0]; //                                                    mova   m2, m0
    m[3] = m[0]; //                                                    mova   m3, m0
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
    ipred_dc_16bpc_splat(&mut m, dst, stride, stride3, w, h); //       movsxd wq, [r5+wq*4]; add wq, r5; jmp wq
}

/// `ipred_v_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_v_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let mut m = regs();
    let dst = dst.cast::<u8>();
    let tl = topleft.cast::<u8>();
    let h = height; //                                                 movifnidn hd, hm
    m[0] = ld32_q1(tl.wrapping_add(2)); //                             movu   m0, [tlq+ 2]
    m[1] = ld32_q1(tl.wrapping_add(34)); //                            movu   m1, [tlq+34]
    m[2] = ld32_q1(tl.wrapping_add(66)); //                            movu   m2, [tlq+66]
    m[3] = ld32_q1(tl.wrapping_add(98)); //                            movu   m3, [tlq+98]
    let w = (width as u32).trailing_zeros(); //                        tzcnt  wd, wd (w is a power of two >= 4)
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
    ipred_dc_16bpc_splat(&mut m, dst, stride, stride3, w, h); //       movsxd wq, [r5+wq*4]; add wq, r5; jmp wq
}

// ---- ipred_h_16bpc (`IPRED_H w, store_type`) ----

/// `IPRED_H w, q|a` at `.w4` and `.w8` (`INIT_XMM`): four rows per iteration, each the broadcast left pixel.
#[inline(always)]
unsafe fn ipred_h_x_q1<const W: usize>(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    stride3: isize,
    mut tl: *const u8,
    mut h: i32,
) {
    loop {
        m[0] = zx(_mm_set1_epi16(rd16_q1(tl.wrapping_sub(2)))); //    vpbroadcastw m0, [tlq-2]
        m[1] = zx(_mm_set1_epi16(rd16_q1(tl.wrapping_sub(4)))); //    vpbroadcastw m1, [tlq-4]
        m[2] = zx(_mm_set1_epi16(rd16_q1(tl.wrapping_sub(6)))); //    vpbroadcastw m2, [tlq-6]
        m[3] = zx(_mm_set1_epi16(rd16_q1(tl.wrapping_sub(8)))); //    vpbroadcastw m3, [tlq-8]
        tl = tl.wrapping_sub(8); //                                    sub    tlq, 8
        if W == 4 {
            st8_q1(dst, lo(m[0])); //                                  movq   [dstq+strideq*0], m0
            st8_q1(at(dst, stride), lo(m[1])); //                      movq   [dstq+strideq*1], m1
            st8_q1(at(dst, stride * 2), lo(m[2])); //                  movq   [dstq+strideq*2], m2
            st8_q1(at(dst, stride3), lo(m[3])); //                     movq   [dstq+stride3q ], m3
        } else {
            st16_q1(dst, lo(m[0])); //                                 mova   [dstq+strideq*0], m0
            st16_q1(at(dst, stride), lo(m[1])); //                     mova   [dstq+strideq*1], m1
            st16_q1(at(dst, stride * 2), lo(m[2])); //                 mova   [dstq+strideq*2], m2
            st16_q1(at(dst, stride3), lo(m[3])); //                    mova   [dstq+stride3q ], m3
        }
        dst = at(dst, stride * 4); //                                  lea    dstq, [dstq+strideq*4]
        h -= 4; //                                                     sub    hd, 4
        if h <= 0 {
            break; //                                                  jg .w%1
        }
    }
}

/// `IPRED_H 16, a` at `.w16` (`INIT_YMM`).
#[inline(always)]
unsafe fn ipred_h_16bpc_w16(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, mut tl: *const u8, mut h: i32) {
    loop {
        m[0] = _mm256_set1_epi16(rd16_q1(tl.wrapping_sub(2))); //      vpbroadcastw m0, [tlq-2]
        m[1] = _mm256_set1_epi16(rd16_q1(tl.wrapping_sub(4))); //      vpbroadcastw m1, [tlq-4]
        m[2] = _mm256_set1_epi16(rd16_q1(tl.wrapping_sub(6))); //      vpbroadcastw m2, [tlq-6]
        m[3] = _mm256_set1_epi16(rd16_q1(tl.wrapping_sub(8))); //      vpbroadcastw m3, [tlq-8]
        tl = tl.wrapping_sub(8); //                                    sub    tlq, 8
        st32_q1(dst, m[0]); //                                         mova   [dstq+strideq*0], m0
        st32_q1(at(dst, stride), m[1]); //                             mova   [dstq+strideq*1], m1
        st32_q1(at(dst, stride * 2), m[2]); //                         mova   [dstq+strideq*2], m2
        st32_q1(at(dst, stride3), m[3]); //                            mova   [dstq+stride3q ], m3
        dst = at(dst, stride * 4); //                                  lea    dstq, [dstq+strideq*4]
        h -= 4; //                                                     sub    hd, 4
        if h <= 0 {
            break; //                                                  jg .w16
        }
    }
}

/// `.w32`.
#[inline(always)]
unsafe fn ipred_h_16bpc_w32(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, mut tl: *const u8, mut h: i32) {
    loop {
        m[0] = _mm256_set1_epi16(rd16_q1(tl.wrapping_sub(2))); //      vpbroadcastw m0, [tlq-2]
        m[1] = _mm256_set1_epi16(rd16_q1(tl.wrapping_sub(4))); //      vpbroadcastw m1, [tlq-4]
        m[2] = _mm256_set1_epi16(rd16_q1(tl.wrapping_sub(6))); //      vpbroadcastw m2, [tlq-6]
        m[3] = _mm256_set1_epi16(rd16_q1(tl.wrapping_sub(8))); //      vpbroadcastw m3, [tlq-8]
        tl = tl.wrapping_sub(8); //                                    sub    tlq, 8
        st32_q1(dst, m[0]); //                                         mova [dstq+strideq*0+32*0], m0
        st32_q1(at(dst, 32), m[0]); //                                 mova [dstq+strideq*0+32*1], m0
        st32_q1(at(dst, stride), m[1]); //                             mova [dstq+strideq*1+32*0], m1
        st32_q1(at(dst, stride + 32), m[1]); //                        mova [dstq+strideq*1+32*1], m1
        st32_q1(at(dst, stride * 2), m[2]); //                         mova [dstq+strideq*2+32*0], m2
        st32_q1(at(dst, stride * 2 + 32), m[2]); //                    mova [dstq+strideq*2+32*1], m2
        st32_q1(at(dst, stride3), m[3]); //                            mova [dstq+stride3q +32*0], m3
        st32_q1(at(dst, stride3 + 32), m[3]); //                       mova [dstq+stride3q +32*1], m3
        dst = at(dst, stride * 4); //                                  lea    dstq, [dstq+strideq*4]
        h -= 4; //                                                     sub    hd, 4
        if h <= 0 {
            break; //                                                  jg .w32
        }
    }
}

/// `.w64`.
#[inline(always)]
unsafe fn ipred_h_16bpc_w64(m: &mut M, mut dst: *mut u8, stride: isize, mut tl: *const u8, mut h: i32) {
    loop {
        m[0] = _mm256_set1_epi16(rd16_q1(tl.wrapping_sub(2))); //      vpbroadcastw m0, [tlq-2]
        m[1] = _mm256_set1_epi16(rd16_q1(tl.wrapping_sub(4))); //      vpbroadcastw m1, [tlq-4]
        tl = tl.wrapping_sub(4); //                                    sub    tlq, 4
        st32_q1(dst, m[0]); //                                         mova [dstq+strideq*0+32*0], m0
        st32_q1(at(dst, 32), m[0]); //                                 mova [dstq+strideq*0+32*1], m0
        st32_q1(at(dst, 64), m[0]); //                                 mova [dstq+strideq*0+32*2], m0
        st32_q1(at(dst, 96), m[0]); //                                 mova [dstq+strideq*0+32*3], m0
        st32_q1(at(dst, stride), m[1]); //                             mova [dstq+strideq*1+32*0], m1
        st32_q1(at(dst, stride + 32), m[1]); //                        mova [dstq+strideq*1+32*1], m1
        st32_q1(at(dst, stride + 64), m[1]); //                        mova [dstq+strideq*1+32*2], m1
        st32_q1(at(dst, stride + 96), m[1]); //                        mova [dstq+strideq*1+32*3], m1
        dst = at(dst, stride * 2); //                                  lea    dstq, [dstq+strideq*2]
        h -= 2; //                                                     sub    hd, 2
        if h <= 0 {
            break; //                                                  jg .w64
        }
    }
}

/// `ipred_h_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_h_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let mut m = regs();
    let dst = dst.cast::<u8>();
    let tl = topleft.cast::<u8>();
    let h = height; //                                                 movifnidn hd, hm
    let w = (width as u32).trailing_zeros(); //                        tzcnt  wd, wd (w is a power of two >= 4)
    let stride3 = stride * 3; //                                       lea    stride3q, [strideq*3]
    match w {
        //                                                             movsxd wq, [r5+wq*4]; add wq, r5; jmp wq
        2 => ipred_h_x_q1::<4>(&mut m, dst, stride, stride3, tl, h),
        3 => ipred_h_x_q1::<8>(&mut m, dst, stride, stride3, tl, h),
        4 => ipred_h_16bpc_w16(&mut m, dst, stride, stride3, tl, h),
        5 => ipred_h_16bpc_w32(&mut m, dst, stride, stride3, tl, h),
        _ => ipred_h_16bpc_w64(&mut m, dst, stride, tl, h),
    }
}

// ---- ipred_paeth_16bpc (`PAETH top, signed_ldiff, ldiff`) ----

/// `PAETH top, signed_ldiff, ldiff`: m1 = left, m3 = topleft, m`top` = top, m`signed_ldiff` = top - topleft,
/// m`ldiff` = |top - topleft|; the predicted pixels in m0 (m7 is scratch).
#[inline(always)]
unsafe fn paeth(m: &mut M, top: usize, signed_ldiff: usize, ldiff: usize) {
    m[0] = _mm256_add_epi16(m[signed_ldiff], m[1]); //                 paddw  m0, m%2, m1
    m[7] = _mm256_sub_epi16(m[3], m[0]); //                            psubw  m7, m3, m0  ; tldiff
    m[0] = _mm256_sub_epi16(m[0], m[top]); //                          psubw  m0, m%1     ; tdiff
    m[7] = _mm256_abs_epi16(m[7]); //                                  pabsw  m7, m7
    m[0] = _mm256_abs_epi16(m[0]); //                                  pabsw  m0, m0
    m[7] = _mm256_min_epi16(m[7], m[0]); //                            pminsw m7, m0
    m[0] = _mm256_cmpeq_epi16(m[0], m[7]); //                          pcmpeqw m0, m7
    m[7] = _mm256_cmpgt_epi16(m[ldiff], m[7]); //                      pcmpgtw m7, m%3, m7
    m[0] = _mm256_blendv_epi8(m[3], m[top], m[0]); //                  vpblendvb m0, m3, m%1, m0
    m[0] = _mm256_blendv_epi8(m[1], m[0], m[7]); //                    vpblendvb m0, m1, m0, m7
}

/// `movsldup m6, [base+ipred_hv_shuf]` (32 bytes of the label).
#[inline(always)]
unsafe fn ipred_hv_shuf_movsldup_q1() -> __m256i {
    _mm256_castps_si256(_mm256_moveldup_ps(_mm256_loadu_ps(ro(ipred_hv_shuf, 0).cast())))
}

/// `.w4`, `.w4_loop`.
#[inline(always)]
unsafe fn ipred_paeth_16bpc_w4(m: &mut M, mut dst: *mut u8, stride: isize, mut tl: *const u8, mut h: i32) {
    m[2] = _mm256_set1_epi64x(rd64_q1(tl.wrapping_add(2))); //         vpbroadcastq m2, [tlq+2] ; top
    m[6] = ipred_hv_shuf_movsldup_q1(); //                             movsldup m6, [base+ipred_hv_shuf]
    let r3 = stride * 3; //                                            lea    r3, [strideq*3]
    m[4] = _mm256_sub_epi16(m[2], m[3]); //                            psubw  m4, m2, m3
    m[5] = _mm256_abs_epi16(m[4]); //                                  pabsw  m5, m4
    loop {
        tl = tl.wrapping_sub(8); //                                    sub    tlq, 8
        m[1] = _mm256_set1_epi64x(rd64_q1(tl)); //                     vpbroadcastq m1, [tlq]
        m[1] = _mm256_shuffle_epi8(m[1], m[6]); //                     pshufb m1, m6 ; left
        paeth(m, 2, 4, 5); //                                          PAETH  2, 4, 5
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); //            vextracti128 xm1, m0, 1
        st8_q1(dst, lo(m[0])); //                                      movq   [dstq+strideq*0], xm0
        st8_q1(at(dst, stride), lo(m[1])); //                          movq   [dstq+strideq*1], xm1
        st8h_q1(at(dst, stride * 2), lo(m[0])); //                     movhps [dstq+strideq*2], xm0
        st8h_q1(at(dst, r3), lo(m[1])); //                             movhps [dstq+r3       ], xm1
        dst = at(dst, stride * 4); //                                  lea    dstq, [dstq+strideq*4]
        h -= 4; //                                                     sub    hd, 4
        if h <= 0 {
            break; //                                                  jg .w4_loop
        }
    }
}

/// `.w8`, `.w8_loop`.
#[inline(always)]
unsafe fn ipred_paeth_16bpc_w8(m: &mut M, mut dst: *mut u8, stride: isize, mut tl: *const u8, mut h: i32) {
    m[2] = _mm256_broadcastsi128_si256(ld16_q1(tl.wrapping_add(2))); // vbroadcasti128 m2, [tlq+2]
    m[6] = ipred_hv_shuf_movsldup_q1(); //                             movsldup m6, [base+ipred_hv_shuf]
    m[4] = _mm256_sub_epi16(m[2], m[3]); //                            psubw  m4, m2, m3
    m[5] = _mm256_abs_epi16(m[4]); //                                  pabsw  m5, m4
    loop {
        tl = tl.wrapping_sub(4); //                                    sub    tlq, 4
        m[1] = _mm256_set1_epi32(rd32_q1(tl)); //                      vpbroadcastd m1, [tlq]
        m[1] = _mm256_shuffle_epi8(m[1], m[6]); //                     pshufb m1, m6
        paeth(m, 2, 4, 5); //                                          PAETH  2, 4, 5
        st16_q1(dst, lo(m[0])); //                                     mova   [dstq+strideq*0], xm0
        st16_q1(at(dst, stride), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
        dst = at(dst, stride * 2); //                                  lea    dstq, [dstq+strideq*2]
        h -= 2; //                                                     sub    hd, 2
        if h <= 0 {
            break; //                                                  jg .w8_loop
        }
    }
}

/// `.w16`, `.w16_loop`.
#[inline(always)]
unsafe fn ipred_paeth_16bpc_w16(m: &mut M, mut dst: *mut u8, stride: isize, mut tl: *const u8, mut h: i32) {
    m[2] = ld32_q1(tl.wrapping_add(2)); //                             movu   m2, [tlq+2]
    m[4] = _mm256_sub_epi16(m[2], m[3]); //                            psubw  m4, m2, m3
    m[5] = _mm256_abs_epi16(m[4]); //                                  pabsw  m5, m4
    loop {
        tl = tl.wrapping_sub(2); //                                    sub    tlq, 2
        m[1] = _mm256_set1_epi16(rd16_q1(tl)); //                      vpbroadcastw m1, [tlq]
        paeth(m, 2, 4, 5); //                                          PAETH  2, 4, 5
        st32_q1(dst, m[0]); //                                         mova   [dstq], m0
        dst = at(dst, stride); //                                      add    dstq, strideq
        h -= 1; //                                                     dec    hd
        if h <= 0 {
            break; //                                                  jg .w16_loop
        }
    }
}

/// `.w32`, `.w32_loop` (the WIN64 spills of xmm8/xmm9 do not apply).
#[inline(always)]
unsafe fn ipred_paeth_16bpc_w32(m: &mut M, mut dst: *mut u8, stride: isize, mut tl: *const u8, mut h: i32) {
    m[2] = ld32_q1(tl.wrapping_add(2)); //                             movu   m2, [tlq+2]
    m[6] = ld32_q1(tl.wrapping_add(34)); //                            movu   m6, [tlq+34]
    m[4] = _mm256_sub_epi16(m[2], m[3]); //                            psubw  m4, m2, m3
    m[8] = _mm256_sub_epi16(m[6], m[3]); //                            psubw  m8, m6, m3
    m[5] = _mm256_abs_epi16(m[4]); //                                  pabsw  m5, m4
    m[9] = _mm256_abs_epi16(m[8]); //                                  pabsw  m9, m8
    loop {
        tl = tl.wrapping_sub(2); //                                    sub    tlq, 2
        m[1] = _mm256_set1_epi16(rd16_q1(tl)); //                      vpbroadcastw m1, [tlq]
        paeth(m, 2, 4, 5); //                                          PAETH  2, 4, 5
        st32_q1(dst, m[0]); //                                         mova   [dstq+32*0], m0
        paeth(m, 6, 8, 9); //                                          PAETH  6, 8, 9
        st32_q1(at(dst, 32), m[0]); //                                 mova   [dstq+32*1], m0
        dst = at(dst, stride); //                                      add    dstq, strideq
        h -= 1; //                                                     dec    hd
        if h <= 0 {
            break; //                                                  jg .w32_loop
        }
    }
}

/// `.w64`, `.w64_loop` (`WIN64_SPILL_XMM` does not apply).
#[inline(always)]
unsafe fn ipred_paeth_16bpc_w64(m: &mut M, mut dst: *mut u8, stride: isize, mut tl: *const u8, mut h: i32) {
    m[2] = ld32_q1(tl.wrapping_add(2)); //                             movu   m2, [tlq+ 2]
    m[6] = ld32_q1(tl.wrapping_add(34)); //                            movu   m6, [tlq+34]
    m[10] = ld32_q1(tl.wrapping_add(66)); //                           movu   m10, [tlq+66]
    m[13] = ld32_q1(tl.wrapping_add(98)); //                           movu   m13, [tlq+98]
    m[4] = _mm256_sub_epi16(m[2], m[3]); //                            psubw  m4, m2, m3
    m[8] = _mm256_sub_epi16(m[6], m[3]); //                            psubw  m8, m6, m3
    m[11] = _mm256_sub_epi16(m[10], m[3]); //                          psubw  m11, m10, m3
    m[14] = _mm256_sub_epi16(m[13], m[3]); //                          psubw  m14, m13, m3
    m[5] = _mm256_abs_epi16(m[4]); //                                  pabsw  m5, m4
    m[9] = _mm256_abs_epi16(m[8]); //                                  pabsw  m9, m8
    m[12] = _mm256_abs_epi16(m[11]); //                                pabsw  m12, m11
    m[15] = _mm256_abs_epi16(m[14]); //                                pabsw  m15, m14
    loop {
        tl = tl.wrapping_sub(2); //                                    sub    tlq, 2
        m[1] = _mm256_set1_epi16(rd16_q1(tl)); //                      vpbroadcastw m1, [tlq]
        paeth(m, 2, 4, 5); //                                          PAETH  2, 4, 5
        st32_q1(dst, m[0]); //                                         mova   [dstq+32*0], m0
        paeth(m, 6, 8, 9); //                                          PAETH  6, 8, 9
        st32_q1(at(dst, 32), m[0]); //                                 mova   [dstq+32*1], m0
        paeth(m, 10, 11, 12); //                                       PAETH 10, 11, 12
        st32_q1(at(dst, 64), m[0]); //                                 mova   [dstq+32*2], m0
        paeth(m, 13, 14, 15); //                                       PAETH 13, 14, 15
        st32_q1(at(dst, 96), m[0]); //                                 mova   [dstq+32*3], m0
        dst = at(dst, stride); //                                      add    dstq, strideq
        h -= 1; //                                                     dec    hd
        if h <= 0 {
            break; //                                                  jg .w64_loop
        }
    }
}

/// `ipred_paeth_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_paeth_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let mut m = regs();
    let dst = dst.cast::<u8>();
    let tl = topleft.cast::<u8>();
    let h = height; //                                                 movifnidn hd, hm
    let w = (width as u32).trailing_zeros(); //                        tzcnt  wd, wd (w is a power of two >= 4)
    m[3] = _mm256_set1_epi16(rd16_q1(tl)); //                          vpbroadcastw m3, [tlq] ; topleft
    match w {
        //                                                             movsxd wq, [r5+wq*4]; add wq, r5; jmp wq
        2 => ipred_paeth_16bpc_w4(&mut m, dst, stride, tl, h),
        3 => ipred_paeth_16bpc_w8(&mut m, dst, stride, tl, h),
        4 => ipred_paeth_16bpc_w16(&mut m, dst, stride, tl, h),
        5 => ipred_paeth_16bpc_w32(&mut m, dst, stride, tl, h),
        _ => ipred_paeth_16bpc_w64(&mut m, dst, stride, tl, h),
    }
}

// ---- pal_pred_16bpc ----

/// `.w4`.
#[inline(always)]
unsafe fn pal_pred_16bpc_w4(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    stride3: isize,
    mut idx: *const u8,
    mut h: i32,
) {
    loop {
        m[0] = zx(ld8_q1(idx)); //                                     movq   xm0, [idxq]
        idx = idx.wrapping_add(8); //                                  add    idxq, 8
        m[1] = zx(_mm_srli_epi16::<4>(lo(m[0]))); //                   psrlw  xm1, xm0, 4
        m[0] = zx(_mm_unpacklo_epi8(lo(m[0]), lo(m[1]))); //           punpcklbw xm0, xm1
        m[1] = zx(_mm_shuffle_epi8(lo(m[4]), lo(m[0]))); //            pshufb xm1, xm4, xm0
        m[2] = zx(_mm_shuffle_epi8(lo(m[5]), lo(m[0]))); //            pshufb xm2, xm5, xm0
        m[0] = zx(_mm_unpacklo_epi8(lo(m[1]), lo(m[2]))); //           punpcklbw xm0, xm1, xm2
        m[1] = zx(_mm_unpackhi_epi8(lo(m[1]), lo(m[2]))); //           punpckhbw xm1, xm2
        st8_q1(dst, lo(m[0])); //                                      movq   [dstq+strideq*0], xm0
        st8_q1(at(dst, stride * 2), lo(m[1])); //                      movq   [dstq+strideq*2], xm1
        st8h_q1(at(dst, stride), lo(m[0])); //                         movhps [dstq+strideq*1], xm0
        st8h_q1(at(dst, stride3), lo(m[1])); //                        movhps [dstq+stride3q ], xm1
        dst = at(dst, stride * 4); //                                  lea    dstq, [dstq+strideq*4]
        h -= 4; //                                                     sub    hd, 4
        if h <= 0 {
            break; //                                                  jg .w4
        }
    }
}

/// `.w8`.
#[inline(always)]
unsafe fn pal_pred_16bpc_w8(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    stride3: isize,
    mut idx: *const u8,
    mut h: i32,
) {
    loop {
        m[2] = _mm256_cvtepu8_epi16(ld16_q1(idx)); //                  pmovzxbw m2, [idxq]
        idx = idx.wrapping_add(16); //                                 add    idxq, 16
        m[1] = _mm256_slli_epi16::<4>(m[2]); //                        psllw  m1, m2, 4
        m[2] = _mm256_or_si256(m[2], m[1]); //                         por    m2, m1
        m[1] = _mm256_shuffle_epi8(m[4], m[2]); //                     pshufb m1, m4, m2
        m[2] = _mm256_shuffle_epi8(m[5], m[2]); //                     pshufb m2, m5, m2
        m[0] = _mm256_unpacklo_epi8(m[1], m[2]); //                    punpcklbw m0, m1, m2
        m[1] = _mm256_unpackhi_epi8(m[1], m[2]); //                    punpckhbw m1, m2
        st16_q1(dst, lo(m[0])); //                                     mova   [dstq+strideq*0], xm0
        st16_q1(at(dst, stride), lo(m[1])); //                         mova   [dstq+strideq*1], xm1
        st16_q1(at(dst, stride * 2), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*2], m0, 1
        st16_q1(at(dst, stride3), _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [dstq+stride3q ], m1, 1
        dst = at(dst, stride * 4); //                                  lea    dstq, [dstq+strideq*4]
        h -= 4; //                                                     sub    hd, 4
        if h <= 0 {
            break; //                                                  jg .w8
        }
    }
}

/// The head shared by `.w16`, `.w32` and `.w64`: 32 index bytes (64 pixels) to m2 (the first 32 pixels) and m3
/// (the next 32), then the first 32 pixels to m0, m1.
#[inline(always)]
unsafe fn pal_pred_16bpc_load64_q1(m: &mut M, idx: *const u8) {
    m[3] = _mm256_shuffle_epi32::<0xD8>(ld32_q1(idx)); //              pshufd m3, [idxq], q3120
                                                       // add idxq, 32: by the caller
    m[3] = _mm256_permute4x64_epi64::<0xD8>(m[3]); //                  vpermq m3, m3, q3120
    m[1] = _mm256_srli_epi16::<4>(m[3]); //                            psrlw  m1, m3, 4
    m[2] = _mm256_unpacklo_epi8(m[3], m[1]); //                        punpcklbw m2, m3, m1
    m[3] = _mm256_unpackhi_epi8(m[3], m[1]); //                        punpckhbw m3, m1
    m[1] = _mm256_shuffle_epi8(m[4], m[2]); //                         pshufb m1, m4, m2
    m[2] = _mm256_shuffle_epi8(m[5], m[2]); //                         pshufb m2, m5, m2
    m[0] = _mm256_unpacklo_epi8(m[1], m[2]); //                        punpcklbw m0, m1, m2
    m[1] = _mm256_unpackhi_epi8(m[1], m[2]); //                        punpckhbw m1, m2
}

/// The second half shared by `.w16`, `.w32` and `.w64`: the pixels of m3 to m0, m1.
#[inline(always)]
unsafe fn pal_pred_16bpc_second_q1(m: &mut M) {
    m[1] = _mm256_shuffle_epi8(m[4], m[3]); //                         pshufb m1, m4, m3
    m[3] = _mm256_shuffle_epi8(m[5], m[3]); //                         pshufb m3, m5, m3
    m[0] = _mm256_unpacklo_epi8(m[1], m[3]); //                        punpcklbw m0, m1, m3
    m[1] = _mm256_unpackhi_epi8(m[1], m[3]); //                        punpckhbw m1, m3
}

/// `.w16`.
#[inline(always)]
unsafe fn pal_pred_16bpc_w16(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    stride3: isize,
    mut idx: *const u8,
    mut h: i32,
) {
    loop {
        pal_pred_16bpc_load64_q1(m, idx);
        idx = idx.wrapping_add(32); //                                 add    idxq, 32
        st32_q1(dst, m[0]); //                                         mova   [dstq+strideq*0], m0
        st32_q1(at(dst, stride), m[1]); //                             mova   [dstq+strideq*1], m1
        pal_pred_16bpc_second_q1(m);
        st32_q1(at(dst, stride * 2), m[0]); //                         mova   [dstq+strideq*2], m0
        st32_q1(at(dst, stride3), m[1]); //                            mova   [dstq+stride3q ], m1
        dst = at(dst, stride * 4); //                                  lea    dstq, [dstq+strideq*4]
        h -= 4; //                                                     sub    hd, 4
        if h <= 0 {
            break; //                                                  jg .w16
        }
    }
}

/// `.w32`.
#[inline(always)]
unsafe fn pal_pred_16bpc_w32(m: &mut M, mut dst: *mut u8, stride: isize, mut idx: *const u8, mut h: i32) {
    loop {
        pal_pred_16bpc_load64_q1(m, idx);
        idx = idx.wrapping_add(32); //                                 add    idxq, 32
        st32_q1(dst, m[0]); //                                         mova   [dstq+ 0], m0
        st32_q1(at(dst, 32), m[1]); //                                 mova   [dstq+32], m1
        pal_pred_16bpc_second_q1(m);
        st32_q1(at(dst, stride), m[0]); //                             mova   [dstq+strideq+ 0], m0
        st32_q1(at(dst, stride + 32), m[1]); //                        mova   [dstq+strideq+32], m1
        dst = at(dst, stride * 2); //                                  lea    dstq, [dstq+strideq*2]
        h -= 2; //                                                     sub    hd, 2
        if h <= 0 {
            break; //                                                  jg .w32
        }
    }
}

/// `.w64`.
#[inline(always)]
unsafe fn pal_pred_16bpc_w64(m: &mut M, mut dst: *mut u8, stride: isize, mut idx: *const u8, mut h: i32) {
    loop {
        pal_pred_16bpc_load64_q1(m, idx);
        idx = idx.wrapping_add(32); //                                 add    idxq, 32
        st32_q1(dst, m[0]); //                                         mova   [dstq+32*0], m0
        st32_q1(at(dst, 32), m[1]); //                                 mova   [dstq+32*1], m1
        pal_pred_16bpc_second_q1(m);
        st32_q1(at(dst, 64), m[0]); //                                 mova   [dstq+32*2], m0
        st32_q1(at(dst, 96), m[1]); //                                 mova   [dstq+32*3], m1
        dst = at(dst, stride); //                                      add    dstq, strideq
        h -= 1; //                                                     dec    hd
        if h <= 0 {
            break; //                                                  jg .w64
        }
    }
}

/// `pal_pred_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_pal_pred_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    pal: *const u16,
    idx: *const u8,
    w: c_int,
    h: c_int,
) {
    let mut m = regs();
    let dst = dst.cast::<u8>();
    m[4] = _mm256_broadcastsi128_si256(ld16_q1(pal.cast())); //        vbroadcasti128 m4, [palq]
    let wi = (w as u32).trailing_zeros(); //                           tzcnt  wd, wm (w is a power of two >= 4)
    m[5] = _mm256_broadcastsi128_si256(ld16_q1(ro(pal_pred_shuf, 0))); // vbroadcasti128 m5, [pal_pred_shuf]
                                                                       // movifnidn hd, hm
    m[4] = _mm256_shuffle_epi8(m[4], m[5]); //                         pshufb m4, m5
    m[5] = _mm256_unpackhi_epi64(m[4], m[4]); //                       punpckhqdq m5, m4, m4
    let stride3 = stride * 3; //                                       lea    stride3q, [strideq*3]
    match wi {
        //                                                             movsxd wq, [r2+wq*4]; add wq, r2; jmp wq
        2 => pal_pred_16bpc_w4(&mut m, dst, stride, stride3, idx, h),
        3 => pal_pred_16bpc_w8(&mut m, dst, stride, stride3, idx, h),
        4 => pal_pred_16bpc_w16(&mut m, dst, stride, stride3, idx, h),
        5 => pal_pred_16bpc_w32(&mut m, dst, stride, idx, h),
        _ => pal_pred_16bpc_w64(&mut m, dst, stride, idx, h),
    }
}

// ---- part 2: `ipred16_avx2.asm` lines 650–1101 (smooth_v, smooth_h, SMOOTH_2D_END, smooth) and 4406–4869 (cfl_ac_420, cfl_ac_422, cfl_ac_444) ----
//
// Correspondence with the assembly (part 2):
// - Each `cglobal` is the `extern "C"` function of the same name (`dav1d_ipred_smooth_v_16bpc_avx2`, …); the
//   jump table `ipred_smooth{,_v,_h}_16bpc_avx2_table` / `ipred_cfl_ac_444_16bpc_avx2_table` that it indexes with
//   `tzcnt` of the width is a `match` on that value in the table's order, and each width label `.w4` … `.w64` is
//   one `#[inline(always)]` function `<cglobal>_w<N>` (`ipred_smooth_v_16bpc_w4`, …) with its row loop.
// - `SMOOTH_2D_END` is `smooth_2d_end` (register arguments as register numbers).
// - `cfl_ac_422` and `cfl_ac_444` jump into `cfl_ac_420`'s labels `.dc`, `.hpad` and `.w16_hpad`: these are
//   `ipred_cfl_ac_420_16bpc_dc`, `_hpad` and `_w16_hpad`, called (as tail calls) from all three. Labels reached
//   only by jumps inside one width path (`.w8_hpad`, `.w8_wpad1`, `.w16_wpad*`, `.w32_wpad*`, `.w32_hpad`) are
//   the branches and loops of that width's function, in the assembly's order.
// - General-purpose registers are locals named as `DEFINE_ARGS`/`PROLOGUE` names them (`hq`, `weightsq`,
//   `v_weightsq`, `r5` = the start of `ac`, …); pointers are byte pointers (`*mut u8`/`*const u8`), so offsets
//   are the assembly's byte offsets. `smooth`'s `.w64` advances its base register `r6` by 128 bytes per column
//   half: that is the local `r6` added to the weight offsets.
// - Memory operands: the helpers with the suffix `_q2` (`bcw_q2` = `vpbroadcastw m, [p]`, `bcd_q2`, `bcq_q2`,
//   `bci128_q2` = `vbroadcasti128`, `movsldup_q2`/`movshdup_q2` with a memory source, `ld_q2`/`ldx_q2` = 32/16-byte
//   loads, `st_q2`/`stx_q2`/`movq_st_q2`/`movhps_st_q2` the stores). Every load and store of caller memory is
//   unaligned (`mova` included: same bytes, same instruction).

/// `vpbroadcastw m, [p]`.
#[inline(always)]
unsafe fn bcw_q2(p: *const u8) -> __m256i {
    _mm256_set1_epi16(p.cast::<i16>().read_unaligned())
}

/// `vpbroadcastd m, [p]`.
#[inline(always)]
unsafe fn bcd_q2(p: *const u8) -> __m256i {
    _mm256_set1_epi32(p.cast::<i32>().read_unaligned())
}

/// `vpbroadcastq m, [p]`.
#[inline(always)]
unsafe fn bcq_q2(p: *const u8) -> __m256i {
    _mm256_set1_epi64x(p.cast::<i64>().read_unaligned())
}

/// `vbroadcasti128 m, [p]`.
#[inline(always)]
unsafe fn bci128_q2(p: *const u8) -> __m256i {
    _mm256_broadcastsi128_si256(_mm_loadu_si128(p.cast()))
}

/// `movsldup m, [p]` (32 bytes).
#[inline(always)]
unsafe fn movsldup_q2(p: *const u8) -> __m256i {
    _mm256_castps_si256(_mm256_moveldup_ps(_mm256_loadu_ps(p.cast())))
}

/// `movshdup m, [p]` (32 bytes).
#[inline(always)]
unsafe fn movshdup_q2(p: *const u8) -> __m256i {
    _mm256_castps_si256(_mm256_movehdup_ps(_mm256_loadu_ps(p.cast())))
}

/// `movu m, [p]` / `mova m, [p]` (32 bytes).
#[inline(always)]
unsafe fn ld_q2(p: *const u8) -> __m256i {
    _mm256_loadu_si256(p.cast())
}

/// `movu xm, [p]` / `mova xm, [p]` (16 bytes).
#[inline(always)]
unsafe fn ldx_q2(p: *const u8) -> __m128i {
    _mm_loadu_si128(p.cast())
}

/// `mova [p], m` (32 bytes).
#[inline(always)]
unsafe fn st_q2(p: *mut u8, x: __m256i) {
    _mm256_storeu_si256(p.cast(), x)
}

/// `mova [p], xm` (16 bytes).
#[inline(always)]
unsafe fn stx_q2(p: *mut u8, x: __m128i) {
    _mm_storeu_si128(p.cast(), x)
}

/// `movq [p], xm` (8 bytes).
#[inline(always)]
unsafe fn movq_st_q2(p: *mut u8, x: __m128i) {
    _mm_storel_epi64(p.cast(), x)
}

/// `movhps [p], xm` (the high 8 bytes). Written as a store of `_mm_extract_epi64::<1>` (one `vpextrq [p], xm, 1`):
/// `_mm_storeh_pd` on the integer value compiled to `vpshufd` + `vmovq`, two instructions for the assembly's one.
#[inline(always)]
unsafe fn movhps_st_q2(p: *mut u8, x: __m128i) {
    p.cast::<i64>().write_unaligned(_mm_extract_epi64::<1>(x))
}

/// `vextracti128 xm, m, 1` (the value; stored with `zx` or `stx_q2`).
#[inline(always)]
unsafe fn ex1_q2(x: __m256i) -> __m128i {
    _mm256_extracti128_si256::<1>(x)
}

/// `p + off` bytes of a read-only pointer, wrapping.
#[inline(always)]
fn cp_q2(p: *const u8, off: isize) -> *const u8 {
    p.wrapping_offset(off)
}

/// `tzcnt` of a block width (a power of two from 4 to 64 by the contract; the original selects these kernels on
/// AVX2 alone, so this is `trailing_zeros`, not BMI's `tzcnt`).
#[inline(always)]
fn tzcnt_q2(x: u32) -> u32 {
    x.trailing_zeros()
}

/// `[base+label+off]` with `base` the address of `RODATA`. Every access through it lies inside `RODATA`, as
/// with `ro`.
#[inline(always)]
fn rob_q2(base: *const u8, label: usize, off: isize) -> *const u8 {
    base.wrapping_add(label).wrapping_offset(off)
}

// ---- ipred_smooth_v_16bpc ----

/// `cglobal ipred_smooth_v_16bpc`: `m5` the bottom pixel, `weightsq` `smooth_weights_1d_16bpc + h*4`, `hq` = -h
/// (bytes `weightsq + hq*2` are the weights of the current row), then the width's path.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_smooth_v_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    let (dst, tl) = (dst as *mut u8, topleft as *const u8);
    let w = tzcnt_q2(width as u32); // tzcnt wd, wm (width is a power of two >= 4: never zero)
    let mut h = height as isize; // mov hd, hm
    let weights = ro(dav1d_smooth_weights_1d_16bpc, h * 4); // lea weightsq, [base+smooth_weights_1d_16bpc+hq*4]
    h = -h; // neg hq
    m[5] = bcw_q2(cp_q2(tl, h * 2)); // vpbroadcastw m5, [tlq+hq*2] ; bottom
                                     // movsxd wq, [r6+wq*4]; add wq, r6; jmp wq
    match w {
        2 => ipred_smooth_v_16bpc_w4(m, dst, stride, tl, weights, h),
        3 => ipred_smooth_v_16bpc_w8(m, dst, stride, tl, weights, h),
        4 => ipred_smooth_v_16bpc_w16(m, dst, stride, tl, weights, h),
        5 => ipred_smooth_v_16bpc_w32(m, dst, stride, tl, weights, h),
        6 => ipred_smooth_v_16bpc_w64(m, dst, stride, tl, weights, h),
        _ => unreachable!(), // outside the jump table (contract: 2..=6)
    }
}

/// `.w4` of `ipred_smooth_v_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_v_16bpc_w4(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    weights: *const u8,
    mut h: isize,
) {
    m[4] = bcq_q2(cp_q2(tl, 2)); // vpbroadcastq m4, [tlq+2] ; top
    m[3] = movsldup_q2(ro(ipred_hv_shuf, 0)); // movsldup m3, [base+ipred_hv_shuf]
    let r6 = stride * 3; // lea r6, [strideq*3]
    m[4] = _mm256_sub_epi16(m[4], m[5]); // psubw m4, m5 ; top - bottom
    loop {
        m[0] = bcq_q2(cp_q2(weights, h * 2)); // vpbroadcastq m0, [weightsq+hq*2]
        m[0] = _mm256_shuffle_epi8(m[0], m[3]); // pshufb m0, m3
        m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m0, m4
        m[0] = _mm256_add_epi16(m[0], m[5]); // paddw m0, m5
        m[1] = zx(ex1_q2(m[0])); // vextracti128 xm1, m0, 1
        movhps_st_q2(dst, lo(m[1])); // movhps [dstq+strideq*0], xm1
        movhps_st_q2(at(dst, stride), lo(m[0])); // movhps [dstq+strideq*1], xm0
        movq_st_q2(at(dst, stride * 2), lo(m[1])); // movq [dstq+strideq*2], xm1
        movq_st_q2(at(dst, r6), lo(m[0])); // movq [dstq+r6], xm0
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h += 4; // add hq, 4
        if h >= 0 {
            break; // jl .w4_loop
        }
    }
}

/// `.w8` of `ipred_smooth_v_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_v_16bpc_w8(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    weights: *const u8,
    mut h: isize,
) {
    m[4] = bci128_q2(cp_q2(tl, 2)); // vbroadcasti128 m4, [tlq+2]
    let base = RODATA.0.as_ptr(); // (`base`: the address of RODATA)
    m[3] = movsldup_q2(rob_q2(base, ipred_hv_shuf, 0)); // movsldup m3, [base+ipred_hv_shuf]
    let r6 = stride * 3; // lea r6, [strideq*3]
    m[4] = _mm256_sub_epi16(m[4], m[5]); // psubw m4, m5
    loop {
        m[0] = bcd_q2(cp_q2(weights, h * 2)); // vpbroadcastd m0, [weightsq+hq*2+0]
        m[1] = bcd_q2(cp_q2(weights, h * 2 + 4)); // vpbroadcastd m1, [weightsq+hq*2+4]
        m[0] = _mm256_shuffle_epi8(m[0], m[3]); // pshufb m0, m3
        m[1] = _mm256_shuffle_epi8(m[1], m[3]); // pshufb m1, m3
        m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m0, m4
        m[1] = _mm256_mulhrs_epi16(m[1], m[4]); // pmulhrsw m1, m4
        m[0] = _mm256_add_epi16(m[0], m[5]); // paddw m0, m5
        m[1] = _mm256_add_epi16(m[1], m[5]); // paddw m1, m5
        stx_q2(dst, ex1_q2(m[0])); // vextracti128 [dstq+strideq*0], m0, 1
        stx_q2(at(dst, stride), lo(m[0])); // mova [dstq+strideq*1], xm0
        stx_q2(at(dst, stride * 2), ex1_q2(m[1])); // vextracti128 [dstq+strideq*2], m1, 1
        stx_q2(at(dst, r6), lo(m[1])); // mova [dstq+r6], xm1
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h += 4; // add hq, 4
        if h >= 0 {
            break; // jl .w8_loop
        }
    }
}

/// `.w16` of `ipred_smooth_v_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_v_16bpc_w16(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    weights: *const u8,
    mut h: isize,
) {
    m[4] = ld_q2(cp_q2(tl, 2)); // movu m4, [tlq+2]
    let r6 = stride * 3; // lea r6, [strideq*3]
    m[4] = _mm256_sub_epi16(m[4], m[5]); // psubw m4, m5
    loop {
        m[0] = bcw_q2(cp_q2(weights, h * 2)); // vpbroadcastw m0, [weightsq+hq*2+0]
        m[1] = bcw_q2(cp_q2(weights, h * 2 + 2)); // vpbroadcastw m1, [weightsq+hq*2+2]
        m[2] = bcw_q2(cp_q2(weights, h * 2 + 4)); // vpbroadcastw m2, [weightsq+hq*2+4]
        m[3] = bcw_q2(cp_q2(weights, h * 2 + 6)); // vpbroadcastw m3, [weightsq+hq*2+6]
        m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // REPX {pmulhrsw x, m4}, m0, m1, m2, m3
        m[1] = _mm256_mulhrs_epi16(m[1], m[4]);
        m[2] = _mm256_mulhrs_epi16(m[2], m[4]);
        m[3] = _mm256_mulhrs_epi16(m[3], m[4]);
        m[0] = _mm256_add_epi16(m[0], m[5]); // REPX {paddw x, m5}, m0, m1, m2, m3
        m[1] = _mm256_add_epi16(m[1], m[5]);
        m[2] = _mm256_add_epi16(m[2], m[5]);
        m[3] = _mm256_add_epi16(m[3], m[5]);
        st_q2(dst, m[0]); // mova [dstq+strideq*0], m0
        st_q2(at(dst, stride), m[1]); // mova [dstq+strideq*1], m1
        st_q2(at(dst, stride * 2), m[2]); // mova [dstq+strideq*2], m2
        st_q2(at(dst, r6), m[3]); // mova [dstq+r6], m3
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h += 4; // add hq, 4
        if h >= 0 {
            break; // jl .w16_loop
        }
    }
}

/// `.w32` of `ipred_smooth_v_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_v_16bpc_w32(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    weights: *const u8,
    mut h: isize,
) {
    m[4] = ld_q2(cp_q2(tl, 2)); // movu m4, [tlq+ 2]
    m[6] = ld_q2(cp_q2(tl, 34)); // movu m6, [tlq+34]
    m[4] = _mm256_sub_epi16(m[4], m[5]); // psubw m4, m5
    m[6] = _mm256_sub_epi16(m[6], m[5]); // psubw m6, m5
    loop {
        m[1] = bcw_q2(cp_q2(weights, h * 2)); // vpbroadcastw m1, [weightsq+hq*2+0]
        m[3] = bcw_q2(cp_q2(weights, h * 2 + 2)); // vpbroadcastw m3, [weightsq+hq*2+2]
        m[0] = _mm256_mulhrs_epi16(m[4], m[1]); // pmulhrsw m0, m4, m1
        m[1] = _mm256_mulhrs_epi16(m[1], m[6]); // pmulhrsw m1, m6
        m[2] = _mm256_mulhrs_epi16(m[4], m[3]); // pmulhrsw m2, m4, m3
        m[3] = _mm256_mulhrs_epi16(m[3], m[6]); // pmulhrsw m3, m6
        m[0] = _mm256_add_epi16(m[0], m[5]); // REPX {paddw x, m5}, m0, m1, m2, m3
        m[1] = _mm256_add_epi16(m[1], m[5]);
        m[2] = _mm256_add_epi16(m[2], m[5]);
        m[3] = _mm256_add_epi16(m[3], m[5]);
        st_q2(dst, m[0]); // mova [dstq+strideq*0+32*0], m0
        st_q2(at(dst, 32), m[1]); // mova [dstq+strideq*0+32*1], m1
        st_q2(at(dst, stride), m[2]); // mova [dstq+strideq*1+32*0], m2
        st_q2(at(dst, stride + 32), m[3]); // mova [dstq+strideq*1+32*1], m3
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h += 2; // add hq, 2
        if h >= 0 {
            break; // jl .w32_loop
        }
    }
}

/// `.w64` of `ipred_smooth_v_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_v_16bpc_w64(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    weights: *const u8,
    mut h: isize,
) {
    m[3] = ld_q2(cp_q2(tl, 2)); // movu m3, [tlq+ 2]
    m[4] = ld_q2(cp_q2(tl, 34)); // movu m4, [tlq+34]
    m[6] = ld_q2(cp_q2(tl, 66)); // movu m6, [tlq+66]
    m[7] = ld_q2(cp_q2(tl, 98)); // movu m7, [tlq+98]
    m[3] = _mm256_sub_epi16(m[3], m[5]); // REPX {psubw x, m5}, m3, m4, m6, m7
    m[4] = _mm256_sub_epi16(m[4], m[5]);
    m[6] = _mm256_sub_epi16(m[6], m[5]);
    m[7] = _mm256_sub_epi16(m[7], m[5]);
    loop {
        m[2] = bcw_q2(cp_q2(weights, h * 2)); // vpbroadcastw m2, [weightsq+hq*2]
        m[0] = _mm256_mulhrs_epi16(m[3], m[2]); // pmulhrsw m0, m3, m2
        m[1] = _mm256_mulhrs_epi16(m[4], m[2]); // pmulhrsw m1, m4, m2
        m[0] = _mm256_add_epi16(m[0], m[5]); // paddw m0, m5
        m[1] = _mm256_add_epi16(m[1], m[5]); // paddw m1, m5
        st_q2(dst, m[0]); // mova [dstq+32*0], m0
        m[0] = _mm256_mulhrs_epi16(m[6], m[2]); // pmulhrsw m0, m6, m2
        st_q2(at(dst, 32), m[1]); // mova [dstq+32*1], m1
        m[1] = _mm256_mulhrs_epi16(m[7], m[2]); // pmulhrsw m1, m7, m2
        m[0] = _mm256_add_epi16(m[0], m[5]); // paddw m0, m5
        m[1] = _mm256_add_epi16(m[1], m[5]); // paddw m1, m5
        st_q2(at(dst, 64), m[0]); // mova [dstq+32*2], m0
        st_q2(at(dst, 96), m[1]); // mova [dstq+32*3], m1
        dst = at(dst, stride); // add dstq, strideq
        h += 1; // inc hq
        if h >= 0 {
            break; // jl .w64_loop
        }
    }
}

// ---- ipred_smooth_h_16bpc ----

/// `cglobal ipred_smooth_h_16bpc`: `m5` the right pixel, `hq` = 2*h, `tlq` moved down by 2*h bytes (so
/// `tlq + hq - 2` is the left pixel of the current row), then the width's path.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_smooth_h_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    let (dst, mut tl) = (dst as *mut u8, topleft as *const u8);
    let w = width as isize; // mov wd, wm
    let mut h = height as isize; // movifnidn hd, hm
    m[5] = bcw_q2(cp_q2(tl, w * 2)); // vpbroadcastw m5, [tlq+wq*2] ; right
    let w = tzcnt_q2(w as u32); // tzcnt wd, wd (width is a power of two >= 4: never zero)
    h += h; // add hd, hd
    tl = cp_q2(tl, -h); // sub tlq, hq
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
                              // movsxd wq, [r6+wq*4]; add wq, r6; jmp wq
    match w {
        2 => ipred_smooth_h_16bpc_w4(m, dst, stride, tl, h, stride3),
        3 => ipred_smooth_h_16bpc_w8(m, dst, stride, tl, h, stride3),
        4 => ipred_smooth_h_16bpc_w16(m, dst, stride, tl, h, stride3),
        5 => ipred_smooth_h_16bpc_w32(m, dst, stride, tl, h),
        _ => ipred_smooth_h_16bpc_w64(m, dst, stride, tl, h),
    }
}

/// `.w4` of `ipred_smooth_h_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_h_16bpc_w4(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: isize,
    stride3: isize,
) {
    m[4] = bcq_q2(ro(dav1d_smooth_weights_1d_16bpc, 4 * 2)); // vpbroadcastq m4, [base+smooth_weights_1d_16bpc+4*2]
    m[3] = movsldup_q2(ro(ipred_hv_shuf, 0)); // movsldup m3, [base+ipred_hv_shuf]
    loop {
        m[0] = bcq_q2(cp_q2(tl, h - 8)); // vpbroadcastq m0, [tlq+hq-8] ; left
        m[0] = _mm256_shuffle_epi8(m[0], m[3]); // pshufb m0, m3
        m[0] = _mm256_sub_epi16(m[0], m[5]); // psubw m0, m5 ; left - right
        m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m0, m4
        m[0] = _mm256_add_epi16(m[0], m[5]); // paddw m0, m5
        m[1] = zx(ex1_q2(m[0])); // vextracti128 xm1, m0, 1
        movq_st_q2(dst, lo(m[0])); // movq [dstq+strideq*0], xm0
        movq_st_q2(at(dst, stride), lo(m[1])); // movq [dstq+strideq*1], xm1
        movhps_st_q2(at(dst, stride * 2), lo(m[0])); // movhps [dstq+strideq*2], xm0
        movhps_st_q2(at(dst, stride3), lo(m[1])); // movhps [dstq+stride3q], xm1
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h -= 4 * 2; // sub hd, 4*2
        if h <= 0 {
            break; // jg .w4_loop
        }
    }
}

/// `.w8` of `ipred_smooth_h_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_h_16bpc_w8(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: isize,
    stride3: isize,
) {
    let base = RODATA.0.as_ptr(); // (`base`: the address of RODATA)
    m[4] = bci128_q2(ro(dav1d_smooth_weights_1d_16bpc, 8 * 2)); // vbroadcasti128 m4, [base+smooth_weights_1d_16bpc+8*2]
    m[3] = movsldup_q2(rob_q2(base, ipred_hv_shuf, 0)); // movsldup m3, [base+ipred_hv_shuf]
    loop {
        m[0] = bcd_q2(cp_q2(tl, h - 4)); // vpbroadcastd m0, [tlq+hq-4]
        m[1] = bcd_q2(cp_q2(tl, h - 8)); // vpbroadcastd m1, [tlq+hq-8]
        m[0] = _mm256_shuffle_epi8(m[0], m[3]); // pshufb m0, m3
        m[1] = _mm256_shuffle_epi8(m[1], m[3]); // pshufb m1, m3
        m[0] = _mm256_sub_epi16(m[0], m[5]); // psubw m0, m5
        m[1] = _mm256_sub_epi16(m[1], m[5]); // psubw m1, m5
        m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // pmulhrsw m0, m4
        m[1] = _mm256_mulhrs_epi16(m[1], m[4]); // pmulhrsw m1, m4
        m[0] = _mm256_add_epi16(m[0], m[5]); // paddw m0, m5
        m[1] = _mm256_add_epi16(m[1], m[5]); // paddw m1, m5
        stx_q2(dst, lo(m[0])); // mova [dstq+strideq*0], xm0
        stx_q2(at(dst, stride), ex1_q2(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
        stx_q2(at(dst, stride * 2), lo(m[1])); // mova [dstq+strideq*2], xm1
        stx_q2(at(dst, stride3), ex1_q2(m[1])); // vextracti128 [dstq+stride3q], m1, 1
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h -= 4 * 2; // sub hq, 4*2
        if h <= 0 {
            break; // jg .w8_loop
        }
    }
}

/// `.w16` of `ipred_smooth_h_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_h_16bpc_w16(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: isize,
    stride3: isize,
) {
    m[4] = ld_q2(ro(dav1d_smooth_weights_1d_16bpc, 16 * 2)); // movu m4, [base+smooth_weights_1d_16bpc+16*2]
    loop {
        m[3] = bcq_q2(cp_q2(tl, h - 8)); // vpbroadcastq m3, [tlq+hq-8]
        m[3] = _mm256_unpacklo_epi16(m[3], m[3]); // punpcklwd m3, m3
        m[3] = _mm256_sub_epi16(m[3], m[5]); // psubw m3, m5
        m[0] = _mm256_shuffle_epi32::<0xff>(m[3]); // pshufd m0, m3, q3333
        m[1] = _mm256_shuffle_epi32::<0xaa>(m[3]); // pshufd m1, m3, q2222
        m[2] = _mm256_shuffle_epi32::<0x55>(m[3]); // pshufd m2, m3, q1111
        m[3] = _mm256_shuffle_epi32::<0x00>(m[3]); // pshufd m3, m3, q0000
        m[0] = _mm256_mulhrs_epi16(m[0], m[4]); // REPX {pmulhrsw x, m4}, m0, m1, m2, m3
        m[1] = _mm256_mulhrs_epi16(m[1], m[4]);
        m[2] = _mm256_mulhrs_epi16(m[2], m[4]);
        m[3] = _mm256_mulhrs_epi16(m[3], m[4]);
        m[0] = _mm256_add_epi16(m[0], m[5]); // REPX {paddw x, m5}, m0, m1, m2, m3
        m[1] = _mm256_add_epi16(m[1], m[5]);
        m[2] = _mm256_add_epi16(m[2], m[5]);
        m[3] = _mm256_add_epi16(m[3], m[5]);
        st_q2(dst, m[0]); // mova [dstq+strideq*0], m0
        st_q2(at(dst, stride), m[1]); // mova [dstq+strideq*1], m1
        st_q2(at(dst, stride * 2), m[2]); // mova [dstq+strideq*2], m2
        st_q2(at(dst, stride3), m[3]); // mova [dstq+stride3q], m3
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h -= 4 * 2; // sub hq, 4*2
        if h <= 0 {
            break; // jg .w16_loop
        }
    }
}

/// `.w32` of `ipred_smooth_h_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_h_16bpc_w32(m: &mut M, mut dst: *mut u8, stride: isize, tl: *const u8, mut h: isize) {
    m[4] = ld_q2(ro(dav1d_smooth_weights_1d_16bpc, 32 * 2)); // movu m4, [base+smooth_weights_1d_16bpc+32*2]
    m[6] = ld_q2(ro(dav1d_smooth_weights_1d_16bpc, 32 * 3)); // movu m6, [base+smooth_weights_1d_16bpc+32*3]
    loop {
        m[1] = bcw_q2(cp_q2(tl, h - 2)); // vpbroadcastw m1, [tlq+hq-2]
        m[3] = bcw_q2(cp_q2(tl, h - 4)); // vpbroadcastw m3, [tlq+hq-4]
        m[1] = _mm256_sub_epi16(m[1], m[5]); // psubw m1, m5
        m[3] = _mm256_sub_epi16(m[3], m[5]); // psubw m3, m5
        m[0] = _mm256_mulhrs_epi16(m[4], m[1]); // pmulhrsw m0, m4, m1
        m[1] = _mm256_mulhrs_epi16(m[1], m[6]); // pmulhrsw m1, m6
        m[2] = _mm256_mulhrs_epi16(m[4], m[3]); // pmulhrsw m2, m4, m3
        m[3] = _mm256_mulhrs_epi16(m[3], m[6]); // pmulhrsw m3, m6
        m[0] = _mm256_add_epi16(m[0], m[5]); // REPX {paddw x, m5}, m0, m1, m2, m3
        m[1] = _mm256_add_epi16(m[1], m[5]);
        m[2] = _mm256_add_epi16(m[2], m[5]);
        m[3] = _mm256_add_epi16(m[3], m[5]);
        st_q2(dst, m[0]); // mova [dstq+strideq*0+32*0], m0
        st_q2(at(dst, 32), m[1]); // mova [dstq+strideq*0+32*1], m1
        st_q2(at(dst, stride), m[2]); // mova [dstq+strideq*1+32*0], m2
        st_q2(at(dst, stride + 32), m[3]); // mova [dstq+strideq*1+32*1], m3
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2 * 2; // sub hq, 2*2
        if h <= 0 {
            break; // jg .w32_loop
        }
    }
}

/// `.w64` of `ipred_smooth_h_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_h_16bpc_w64(m: &mut M, mut dst: *mut u8, stride: isize, tl: *const u8, mut h: isize) {
    m[3] = ld_q2(ro(dav1d_smooth_weights_1d_16bpc, 32 * 4)); // movu m3, [base+smooth_weights_1d_16bpc+32*4]
    m[4] = ld_q2(ro(dav1d_smooth_weights_1d_16bpc, 32 * 5)); // movu m4, [base+smooth_weights_1d_16bpc+32*5]
    m[6] = ld_q2(ro(dav1d_smooth_weights_1d_16bpc, 32 * 6)); // movu m6, [base+smooth_weights_1d_16bpc+32*6]
    m[7] = ld_q2(ro(dav1d_smooth_weights_1d_16bpc, 32 * 7)); // movu m7, [base+smooth_weights_1d_16bpc+32*7]
    loop {
        m[2] = bcw_q2(cp_q2(tl, h - 2)); // vpbroadcastw m2, [tlq+hq-2]
        m[2] = _mm256_sub_epi16(m[2], m[5]); // psubw m2, m5
        m[0] = _mm256_mulhrs_epi16(m[3], m[2]); // pmulhrsw m0, m3, m2
        m[1] = _mm256_mulhrs_epi16(m[4], m[2]); // pmulhrsw m1, m4, m2
        m[0] = _mm256_add_epi16(m[0], m[5]); // paddw m0, m5
        m[1] = _mm256_add_epi16(m[1], m[5]); // paddw m1, m5
        st_q2(dst, m[0]); // mova [dstq+32*0], m0
        m[0] = _mm256_mulhrs_epi16(m[6], m[2]); // pmulhrsw m0, m6, m2
        st_q2(at(dst, 32), m[1]); // mova [dstq+32*1], m1
        m[1] = _mm256_mulhrs_epi16(m[7], m[2]); // pmulhrsw m1, m7, m2
        m[0] = _mm256_add_epi16(m[0], m[5]); // paddw m0, m5
        m[1] = _mm256_add_epi16(m[1], m[5]); // paddw m1, m5
        st_q2(at(dst, 64), m[0]); // mova [dstq+32*2], m0
        st_q2(at(dst, 96), m[1]); // mova [dstq+32*3], m1
        dst = at(dst, stride); // add dstq, strideq
        h -= 1 * 2; // sub hq, 1*2
        if h <= 0 {
            break; // jg .w64_loop
        }
    }
}

// ---- ipred_smooth_16bpc ----

/// `SMOOTH_2D_END src1, src2, mul1, mul2, add1, add2`.
#[inline(always)]
unsafe fn smooth_2d_end(m: &mut M, s1: usize, s2: usize, mul1: usize, mul2: usize, add1: usize, add2: usize) {
    m[0] = _mm256_madd_epi16(m[s1], m[mul1]); // pmaddwd m0, m%1, m%3
    m[1] = _mm256_madd_epi16(m[s2], m[mul2]); // pmaddwd m1, m%2, m%4
    m[0] = _mm256_add_epi32(m[0], m[add1]); // paddd m0, m%5
    m[1] = _mm256_add_epi32(m[1], m[add2]); // paddd m1, m%6
    m[0] = _mm256_srli_epi32::<8>(m[0]); // psrld m0, 8
    m[1] = _mm256_srli_epi32::<8>(m[1]); // psrld m1, 8
    m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
    m[0] = _mm256_avg_epu16(m[0], m[5]); // pavgw m0, m5
}

/// `cglobal ipred_smooth_16bpc`: `m4` the right pixel, `tlq` moved down by h pixels (`[tlq]` the bottom pixel,
/// `tlq + hq*2 - 2` the left pixel of the current row), `m5` zero, `v_weightsq` `smooth_weights_2d_16bpc + h*4`,
/// then the width's path.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_smooth_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    _bitdepth_max: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    let (dst, mut tl) = (dst as *mut u8, topleft as *const u8);
    let w = width as isize; // mov wd, wm
    m[4] = bcw_q2(cp_q2(tl, w * 2)); // vpbroadcastw m4, [tlq+wq*2] ; right
    let w = tzcnt_q2(w as u32); // tzcnt wd, wd (width is a power of two >= 4: never zero)
    let h = height as isize; // mov hd, hm
    tl = cp_q2(tl, -h); // sub tlq, hq
    tl = cp_q2(tl, -h); // sub tlq, hq
    m[5] = _mm256_setzero_si256(); // pxor m5, m5
    let v_weights = ro(dav1d_smooth_weights_2d_16bpc, h * 4); // lea v_weightsq, [base+smooth_weights_2d_16bpc+hq*4]
                                                              // movsxd wq, [r6+wq*4]; add wq, r6; jmp wq
    match w {
        2 => ipred_smooth_16bpc_w4(m, dst, stride, tl, h, v_weights),
        3 => ipred_smooth_16bpc_w8(m, dst, stride, tl, h, v_weights),
        4 => ipred_smooth_16bpc_w16(m, dst, stride, tl, h, v_weights),
        5 => ipred_smooth_16bpc_w32(m, dst, stride, tl, h, v_weights),
        _ => ipred_smooth_16bpc_w64(m, dst, stride, tl, h, v_weights),
    }
}

/// `.w4` of `ipred_smooth_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_16bpc_w4(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: isize,
    mut v_weights: *const u8,
) {
    m[0] = bcw_q2(tl); // vpbroadcastw m0, [tlq] ; bottom
    m[6] = bcq_q2(cp_q2(tl, h * 2 + 2)); // vpbroadcastq m6, [tlq+hq*2+2]
    m[7] = movsldup_q2(ro(ipred_hv_shuf, 0)); // movsldup m7, [base+ipred_hv_shuf]
    m[9] = movshdup_q2(ro(ipred_hv_shuf, 0)); // movshdup m9, [base+ipred_hv_shuf]
    m[10] = bci128_q2(ro(dav1d_smooth_weights_2d_16bpc, 4 * 4)); // vbroadcasti128 m10, [base+smooth_weights_2d_16bpc+4*4]
    m[6] = _mm256_unpacklo_epi16(m[6], m[0]); // punpcklwd m6, m0 ; top, bottom
    m[8] = _mm256_unpacklo_epi64(m[9], m[9]); // punpcklqdq m8, m9, m9
    m[9] = _mm256_unpackhi_epi64(m[9], m[9]); // punpckhqdq m9, m9
    let r3 = stride * 3; // lea r3, [strideq*3]
    loop {
        m[3] = bcq_q2(cp_q2(tl, h * 2 - 8)); // vpbroadcastq m3, [tlq+hq*2-8]
        m[1] = bci128_q2(v_weights); // vbroadcasti128 m1, [v_weightsq]
        m[3] = _mm256_shuffle_epi8(m[3], m[7]); // pshufb m3, m7
        m[2] = _mm256_unpacklo_epi16(m[3], m[4]); // punpcklwd m2, m3, m4 ; left, right
        m[3] = _mm256_unpackhi_epi16(m[3], m[4]); // punpckhwd m3, m4
        m[2] = _mm256_madd_epi16(m[2], m[10]); // pmaddwd m2, m10
        m[3] = _mm256_madd_epi16(m[3], m[10]); // pmaddwd m3, m10
        m[0] = _mm256_shuffle_epi8(m[1], m[8]); // pshufb m0, m1, m8
        m[1] = _mm256_shuffle_epi8(m[1], m[9]); // pshufb m1, m9
        smooth_2d_end(m, 0, 1, 6, 6, 2, 3); // SMOOTH_2D_END 0, 1, 6, 6, 2, 3
        m[1] = zx(ex1_q2(m[0])); // vextracti128 xm1, m0, 1
        movq_st_q2(dst, lo(m[0])); // movq [dstq+strideq*0], xm0
        movq_st_q2(at(dst, stride), lo(m[1])); // movq [dstq+strideq*1], xm1
        movhps_st_q2(at(dst, stride * 2), lo(m[0])); // movhps [dstq+strideq*2], xm0
        movhps_st_q2(at(dst, r3), lo(m[1])); // movhps [dstq+r3], xm1
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        v_weights = cp_q2(v_weights, 16); // add v_weightsq, 16
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w4_loop
        }
    }
}

/// `.w8` of `ipred_smooth_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_16bpc_w8(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: isize,
    mut v_weights: *const u8,
) {
    m[0] = bcw_q2(tl); // vpbroadcastw m0, [tlq] ; bottom
    m[7] = bci128_q2(cp_q2(tl, h * 2 + 2)); // vbroadcasti128 m7, [tlq+hq*2+2]
    m[8] = movsldup_q2(ro(ipred_hv_shuf, 0)); // movsldup m8, [base+ipred_hv_shuf]
    m[9] = movshdup_q2(ro(ipred_hv_shuf, 0)); // movshdup m9, [base+ipred_hv_shuf]
    m[10] = bci128_q2(ro(dav1d_smooth_weights_2d_16bpc, 8 * 4 + 16 * 0)); // vbroadcasti128 m10, [base+smooth_weights_2d_16bpc+8*4+16*0]
    m[11] = bci128_q2(ro(dav1d_smooth_weights_2d_16bpc, 8 * 4 + 16 * 1)); // vbroadcasti128 m11, [base+smooth_weights_2d_16bpc+8*4+16*1]
    m[6] = _mm256_unpacklo_epi16(m[7], m[0]); // punpcklwd m6, m7, m0 ; top, bottom
    m[7] = _mm256_unpackhi_epi16(m[7], m[0]); // punpckhwd m7, m0
    loop {
        m[3] = bcd_q2(cp_q2(tl, h * 2 - 4)); // vpbroadcastd m3, [tlq+hq*2-4]
        m[1] = bcq_q2(v_weights); // vpbroadcastq m1, [v_weightsq]
        m[3] = _mm256_shuffle_epi8(m[3], m[8]); // pshufb m3, m8
        m[2] = _mm256_unpacklo_epi16(m[3], m[4]); // punpcklwd m2, m3, m4 ; left, right
        m[3] = _mm256_unpackhi_epi16(m[3], m[4]); // punpckhwd m3, m4
        m[2] = _mm256_madd_epi16(m[2], m[10]); // pmaddwd m2, m10
        m[3] = _mm256_madd_epi16(m[3], m[11]); // pmaddwd m3, m11
        m[1] = _mm256_shuffle_epi8(m[1], m[9]); // pshufb m1, m9
        smooth_2d_end(m, 1, 1, 6, 7, 2, 3); // SMOOTH_2D_END 1, 1, 6, 7, 2, 3
        stx_q2(dst, lo(m[0])); // mova [dstq+strideq*0], xm0
        stx_q2(at(dst, stride), ex1_q2(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        v_weights = cp_q2(v_weights, 8); // add v_weightsq, 8
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w8_loop
        }
    }
}

/// `.w16` of `ipred_smooth_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_16bpc_w16(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: isize,
    mut v_weights: *const u8,
) {
    let sw = dav1d_smooth_weights_2d_16bpc;
    m[0] = bcw_q2(tl); // vpbroadcastw m0, [tlq] ; bottom
    m[7] = ld_q2(cp_q2(tl, h * 2 + 2)); // movu m7, [tlq+hq*2+2]
    m[8] = zx(ldx_q2(ro(sw, 16 * 4 + 16 * 0))); // mova xm8, [base+smooth_weights_2d_16bpc+16*4+16*0]
    m[9] = zx(ldx_q2(ro(sw, 16 * 4 + 16 * 1))); // mova xm9, [base+smooth_weights_2d_16bpc+16*4+16*1]
    m[8] = _mm256_inserti128_si256::<1>(m[8], ldx_q2(ro(sw, 16 * 4 + 16 * 2))); // vinserti128 m8, [base+…+16*4+16*2], 1
    m[9] = _mm256_inserti128_si256::<1>(m[9], ldx_q2(ro(sw, 16 * 4 + 16 * 3))); // vinserti128 m9, [base+…+16*4+16*3], 1
    m[6] = _mm256_unpacklo_epi16(m[7], m[0]); // punpcklwd m6, m7, m0 ; top, bottom
    m[7] = _mm256_unpackhi_epi16(m[7], m[0]); // punpckhwd m7, m0
    loop {
        m[3] = bcd_q2(cp_q2(tl, h * 2 - 4)); // vpbroadcastd m3, [tlq+hq*2-4]
        m[1] = bcd_q2(v_weights); // vpbroadcastd m1, [v_weightsq+0]
        m[3] = _mm256_unpacklo_epi16(m[3], m[4]); // punpcklwd m3, m4 ; left, right
        m[2] = _mm256_shuffle_epi32::<0x55>(m[3]); // pshufd m2, m3, q1111
        m[10] = _mm256_madd_epi16(m[8], m[2]); // pmaddwd m10, m8, m2
        m[2] = _mm256_madd_epi16(m[2], m[9]); // pmaddwd m2, m9
        m[3] = _mm256_shuffle_epi32::<0x00>(m[3]); // pshufd m3, m3, q0000
        smooth_2d_end(m, 1, 1, 6, 7, 10, 2); // SMOOTH_2D_END 1, 1, 6, 7, 10, 2
        m[1] = bcd_q2(cp_q2(v_weights, 4)); // vpbroadcastd m1, [v_weightsq+4]
        m[2] = _mm256_madd_epi16(m[8], m[3]); // pmaddwd m2, m8, m3
        m[3] = _mm256_madd_epi16(m[3], m[9]); // pmaddwd m3, m9
        st_q2(dst, m[0]); // mova [dstq+strideq*0], m0
        smooth_2d_end(m, 1, 1, 6, 7, 2, 3); // SMOOTH_2D_END 1, 1, 6, 7, 2, 3
        st_q2(at(dst, stride), m[0]); // mova [dstq+strideq*1], m0
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        v_weights = cp_q2(v_weights, 8); // add v_weightsq, 8
        h -= 2; // sub hq, 2
        if h <= 0 {
            break; // jg .w16_loop
        }
    }
}

/// `.w32` of `ipred_smooth_16bpc`.
#[inline(always)]
unsafe fn ipred_smooth_16bpc_w32(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: isize,
    mut v_weights: *const u8,
) {
    let sw = dav1d_smooth_weights_2d_16bpc;
    m[0] = bcw_q2(tl); // vpbroadcastw m0, [tlq] ; bottom
    m[7] = ld_q2(cp_q2(tl, h * 2 + 2)); // movu m7, [tlq+hq*2+ 2]
    m[9] = ld_q2(cp_q2(tl, h * 2 + 34)); // movu m9, [tlq+hq*2+34]
    m[10] = zx(ldx_q2(ro(sw, 32 * 4 + 16 * 0))); // mova xm10, [base+smooth_weights_2d_16bpc+32*4+16*0]
    m[11] = zx(ldx_q2(ro(sw, 32 * 4 + 16 * 1))); // mova xm11, [base+smooth_weights_2d_16bpc+32*4+16*1]
    m[10] = _mm256_inserti128_si256::<1>(m[10], ldx_q2(ro(sw, 32 * 4 + 16 * 2))); // vinserti128 m10, [base+…+32*4+16*2], 1
    m[11] = _mm256_inserti128_si256::<1>(m[11], ldx_q2(ro(sw, 32 * 4 + 16 * 3))); // vinserti128 m11, [base+…+32*4+16*3], 1
    m[12] = zx(ldx_q2(ro(sw, 32 * 4 + 16 * 4))); // mova xm12, [base+smooth_weights_2d_16bpc+32*4+16*4]
    m[13] = zx(ldx_q2(ro(sw, 32 * 4 + 16 * 5))); // mova xm13, [base+smooth_weights_2d_16bpc+32*4+16*5]
    m[12] = _mm256_inserti128_si256::<1>(m[12], ldx_q2(ro(sw, 32 * 4 + 16 * 6))); // vinserti128 m12, [base+…+32*4+16*6], 1
    m[13] = _mm256_inserti128_si256::<1>(m[13], ldx_q2(ro(sw, 32 * 4 + 16 * 7))); // vinserti128 m13, [base+…+32*4+16*7], 1
    m[6] = _mm256_unpacklo_epi16(m[7], m[0]); // punpcklwd m6, m7, m0
    m[7] = _mm256_unpackhi_epi16(m[7], m[0]); // punpckhwd m7, m0
    m[8] = _mm256_unpacklo_epi16(m[9], m[0]); // punpcklwd m8, m9, m0
    m[9] = _mm256_unpackhi_epi16(m[9], m[0]); // punpckhwd m9, m0
    loop {
        m[3] = bcw_q2(cp_q2(tl, h * 2 - 2)); // vpbroadcastw m3, [tlq+hq*2-2]
        m[14] = bcd_q2(v_weights); // vpbroadcastd m14, [v_weightsq]
        m[3] = _mm256_unpacklo_epi16(m[3], m[4]); // punpcklwd m3, m4
        m[1] = _mm256_madd_epi16(m[10], m[3]); // pmaddwd m1, m10, m3
        m[2] = _mm256_madd_epi16(m[11], m[3]); // pmaddwd m2, m11, m3
        m[0] = _mm256_madd_epi16(m[6], m[14]); // pmaddwd m0, m6, m14
        m[0] = _mm256_add_epi32(m[0], m[1]); // paddd m0, m1
        m[1] = _mm256_madd_epi16(m[7], m[14]); // pmaddwd m1, m7, m14
        m[1] = _mm256_add_epi32(m[1], m[2]); // paddd m1, m2
        m[2] = _mm256_madd_epi16(m[12], m[3]); // pmaddwd m2, m12, m3
        m[3] = _mm256_madd_epi16(m[3], m[13]); // pmaddwd m3, m13
        m[0] = _mm256_srli_epi32::<8>(m[0]); // psrld m0, 8
        m[1] = _mm256_srli_epi32::<8>(m[1]); // psrld m1, 8
        m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
        m[0] = _mm256_avg_epu16(m[0], m[5]); // pavgw m0, m5
        st_q2(dst, m[0]); // mova [dstq+32*0], m0
        smooth_2d_end(m, 14, 14, 8, 9, 2, 3); // SMOOTH_2D_END 14, 14, 8, 9, 2, 3
        st_q2(at(dst, 32), m[0]); // mova [dstq+32*1], m0
        dst = at(dst, stride); // add dstq, strideq
        v_weights = cp_q2(v_weights, 4); // add v_weightsq, 4
        h -= 1; // dec hd
        if h <= 0 {
            break; // jg .w32_loop
        }
    }
}

/// `.w64` of `ipred_smooth_16bpc` (`PROLOGUE 0, 11, 16, dst, stride, tl, tl_base, h, v_weights, dummy,
/// v_weights_base, x, y, dst_base`): two halves of 32 columns (`xq` = 0, 32); `r6` (the table base the weight
/// operands are addressed from) moves by 16*8 bytes for the second half.
#[inline(always)]
unsafe fn ipred_smooth_16bpc_w64(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    h: isize,
    mut v_weights: *const u8,
) {
    let sw = dav1d_smooth_weights_2d_16bpc;
    let dst_base = dst; // mov dst_baseq, dstq
    let tl_base = tl; // mov tl_baseq, tlq
    let v_weights_base = v_weights; // mov v_weights_baseq, v_weightsq
    let mut x: isize = 0; // xor xq, xq
    let mut r6: isize = 0; // the displacement of r6 from its value at entry
    loop {
        let mut y = h; // mov yq, hq
        let mut tl = cp_q2(tl_base, h * 2); // lea tlq, [tl_baseq+hq*2]
        m[0] = bcw_q2(tl_base); // vpbroadcastw m0, [tl_baseq] ; bottom
        m[7] = ld_q2(cp_q2(tl, x * 2 + 2)); // movu m7, [tlq+xq*2+ 2]
        m[9] = ld_q2(cp_q2(tl, x * 2 + 34)); // movu m9, [tlq+xq*2+34]
        m[10] = zx(ldx_q2(ro(sw, r6 + 64 * 4 + 16 * 0))); // mova xm10, [base+smooth_weights_2d_16bpc+64*4+16*0]
        m[11] = zx(ldx_q2(ro(sw, r6 + 64 * 4 + 16 * 1))); // mova xm11, [base+smooth_weights_2d_16bpc+64*4+16*1]
        m[10] = _mm256_inserti128_si256::<1>(m[10], ldx_q2(ro(sw, r6 + 64 * 4 + 16 * 2))); // vinserti128 m10, [base+…+64*4+16*2], 1
        m[11] = _mm256_inserti128_si256::<1>(m[11], ldx_q2(ro(sw, r6 + 64 * 4 + 16 * 3))); // vinserti128 m11, [base+…+64*4+16*3], 1
        m[12] = zx(ldx_q2(ro(sw, r6 + 64 * 4 + 16 * 4))); // mova xm12, [base+smooth_weights_2d_16bpc+64*4+16*4]
        m[13] = zx(ldx_q2(ro(sw, r6 + 64 * 4 + 16 * 5))); // mova xm13, [base+smooth_weights_2d_16bpc+64*4+16*5]
        m[12] = _mm256_inserti128_si256::<1>(m[12], ldx_q2(ro(sw, r6 + 64 * 4 + 16 * 6))); // vinserti128 m12, [base+…+64*4+16*6], 1
        m[13] = _mm256_inserti128_si256::<1>(m[13], ldx_q2(ro(sw, r6 + 64 * 4 + 16 * 7))); // vinserti128 m13, [base+…+64*4+16*7], 1
        m[6] = _mm256_unpacklo_epi16(m[7], m[0]); // punpcklwd m6, m7, m0
        m[7] = _mm256_unpackhi_epi16(m[7], m[0]); // punpckhwd m7, m0
        m[8] = _mm256_unpacklo_epi16(m[9], m[0]); // punpcklwd m8, m9, m0
        m[9] = _mm256_unpackhi_epi16(m[9], m[0]); // punpckhwd m9, m0
        tl = cp_q2(tl_base, -2); // lea tlq, [tl_baseq-2]
        loop {
            m[3] = bcw_q2(cp_q2(tl, y * 2)); // vpbroadcastw m3, [tlq+yq*2]
            m[1] = bcd_q2(v_weights); // vpbroadcastd m1, [v_weightsq]
            m[3] = _mm256_unpacklo_epi16(m[3], m[4]); // punpcklwd m3, m4
            m[14] = _mm256_madd_epi16(m[10], m[3]); // pmaddwd m14, m10, m3
            m[15] = _mm256_madd_epi16(m[11], m[3]); // pmaddwd m15, m11, m3
            m[2] = _mm256_madd_epi16(m[12], m[3]); // pmaddwd m2, m12, m3
            m[3] = _mm256_madd_epi16(m[3], m[13]); // pmaddwd m3, m13
            m[0] = _mm256_madd_epi16(m[6], m[1]); // pmaddwd m0, m6, m1
            m[0] = _mm256_add_epi32(m[0], m[14]); // paddd m0, m14
            m[14] = _mm256_madd_epi16(m[7], m[1]); // pmaddwd m14, m7, m1
            m[14] = _mm256_add_epi32(m[14], m[15]); // paddd m14, m15
            m[0] = _mm256_srli_epi32::<8>(m[0]); // psrld m0, 8
            m[14] = _mm256_srli_epi32::<8>(m[14]); // psrld m14, 8
            m[0] = _mm256_packs_epi32(m[0], m[14]); // packssdw m0, m14
            m[0] = _mm256_avg_epu16(m[0], m[5]); // pavgw m0, m5
            st_q2(dst, m[0]); // mova [dstq+32*0], m0
            smooth_2d_end(m, 8, 9, 1, 1, 2, 3); // SMOOTH_2D_END 8, 9, 1, 1, 2, 3
            st_q2(at(dst, 32), m[0]); // mova [dstq+32*1], m0
            dst = at(dst, stride); // add dstq, strideq
            v_weights = cp_q2(v_weights, 4); // add v_weightsq, 4
            y -= 1; // dec yq
            if y <= 0 {
                break; // jg .w64_loop_y
            }
        }
        dst = at(dst_base, 32 * 2); // lea dstq, [dst_baseq+32*2]
        r6 += 16 * 8; // add r6, 16*8
        v_weights = v_weights_base; // mov v_weightsq, v_weights_baseq
        x += 32; // add xq, 32
        if x & 64 != 0 {
            break; // test xb, 64; jz .w64_loop_x
        }
    }
}

// ---- ipred_cfl_ac_420_16bpc, ipred_cfl_ac_422_16bpc, ipred_cfl_ac_444_16bpc ----

/// `.dc` of `ipred_cfl_ac_420_16bpc` (also the end of 422 and 444): the rounded average of the `ac` values
/// (sum in `m4`), subtracted from all of them. `r5` the start of `ac`, `ac` its end.
#[inline(always)]
unsafe fn ipred_cfl_ac_420_16bpc_dc(m: &mut M, ac: *mut u8, r5: *mut u8) {
    m[1] = zx(ex1_q2(m[4])); // vextracti128 xm1, m4, 1
    let mut r5 = (r5 as isize).wrapping_sub(ac as isize); // sub r5, acq ; -w*h*2
    let mut r1 = (r5 as u32).trailing_zeros(); // tzcnt r1d, r5d (-w*h*2 is never zero)
    m[4] = zx(_mm_add_epi32(lo(m[4]), lo(m[1]))); // paddd xm4, xm1
    r1 = r1.wrapping_sub(2); // sub r1d, 2
    m[1] = zx(_mm_unpackhi_epi64(lo(m[4]), lo(m[4]))); // punpckhqdq xm1, xm4, xm4
    m[0] = zx(_mm_cvtsi32_si128(r1 as i32)); // movd xm0, r1d
    m[1] = zx(_mm_add_epi32(lo(m[1]), lo(m[4]))); // paddd xm1, xm4
    m[4] = zx(_mm_shufflelo_epi16::<0x4e>(lo(m[1]))); // pshuflw xm4, xm1, q1032
    m[1] = zx(_mm_add_epi32(lo(m[1]), lo(m[4]))); // paddd xm1, xm4
    m[1] = zx(_mm_srl_epi32(lo(m[1]), lo(m[0]))); // psrld xm1, xm0
    m[0] = zx(_mm_setzero_si128()); // pxor xm0, xm0
    m[1] = zx(_mm_avg_epu16(lo(m[1]), lo(m[0]))); // pavgw xm1, xm0
    m[1] = _mm256_broadcastw_epi16(lo(m[1])); // vpbroadcastw m1, xm1
    loop {
        m[0] = ld_q2(at(ac, r5)); // mova m0, [acq+r5]
        m[0] = _mm256_sub_epi16(m[0], m[1]); // psubw m0, m1
        st_q2(at(ac, r5), m[0]); // mova [acq+r5], m0
        r5 += 32; // add r5, 32
        if r5 >= 0 {
            break; // jl .dc_loop
        }
    }
}

/// `.hpad` of `ipred_cfl_ac_420_16bpc` (also reached from 422 and 444): `m1` (two rows) repeated over the padded
/// rows, `m0` added to the sum per 64 bytes; then `.dc`.
#[inline(always)]
unsafe fn ipred_cfl_ac_420_16bpc_hpad(m: &mut M, mut ac: *mut u8, r5: *mut u8, mut hpad: i32) {
    loop {
        st_q2(ac, m[1]); // mova [acq+32*0], m1
        m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
        st_q2(at(ac, 32), m[1]); // mova [acq+32*1], m1
        ac = at(ac, 32 * 2); // add acq, 32*2
        hpad -= 4; // sub hpadd, 4
        if hpad <= 0 {
            break; // jg .hpad
        }
    }
    ipred_cfl_ac_420_16bpc_dc(m, ac, r5)
}

/// `.w16_hpad` of `ipred_cfl_ac_420_16bpc` (also reached from 422): falls into `.hpad`.
#[inline(always)]
unsafe fn ipred_cfl_ac_420_16bpc_w16_hpad(m: &mut M, ac: *mut u8, r5: *mut u8, mut hpad: i32) {
    hpad += hpad; // add hpadd, hpadd
    if hpad == 0 {
        return ipred_cfl_ac_420_16bpc_dc(m, ac, r5); // jz .dc
    }
    m[0] = _mm256_add_epi32(m[0], m[0]); // paddd m0, m0
    ipred_cfl_ac_420_16bpc_hpad(m, ac, r5, hpad)
}

/// `cglobal ipred_cfl_ac_420_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_ac_420_16bpc_avx2(
    ac: *mut i16,
    y: *const u16,
    stride: isize,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    let (ac, ypx, wpad) = (ac as *mut u8, y as *const u8, w_pad);
    let mut hpad = h_pad; // movifnidn hpadd, hpadm
    m[5] = bcd_q2(ro(z_filter_k, 32)); // vpbroadcastd m5, [pw_2]
    let mut h = ch; // mov hd, hm
    hpad <<= 2; // shl hpadd, 2
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    h -= hpad; // sub hd, hpadd
               // cmp dword wm, 8; jg .w16; je .w8
    if cw > 8 {
        ipred_cfl_ac_420_16bpc_w16(m, ac, ypx, stride, wpad, hpad, h)
    } else if cw == 8 {
        ipred_cfl_ac_420_16bpc_w8(m, ac, ypx, stride, wpad, hpad, h)
    } else {
        ipred_cfl_ac_420_16bpc_w4(m, ac, ypx, stride, hpad, h)
    }
}

/// `.w4` of `ipred_cfl_ac_420_16bpc`.
#[inline(always)]
unsafe fn ipred_cfl_ac_420_16bpc_w4(
    m: &mut M,
    mut ac: *mut u8,
    mut ypx: *const u8,
    stride: isize,
    mut hpad: i32,
    mut h: i32,
) {
    let r3 = stride * 3; // lea r3, [strideq*3]
    let r5 = ac; // mov r5, acq
    loop {
        m[0] = zx(ldx_q2(cp_q2(ypx, stride * 2))); // mova xm0, [ypxq+strideq*2]
        m[1] = zx(ldx_q2(cp_q2(ypx, r3))); // mova xm1, [ypxq+r3]
        m[0] = _mm256_inserti128_si256::<1>(m[0], ldx_q2(ypx)); // vinserti128 m0, [ypxq+strideq*0], 1
        m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_q2(cp_q2(ypx, stride))); // vinserti128 m1, [ypxq+strideq*1], 1
        ypx = cp_q2(ypx, stride * 4); // lea ypxq, [ypxq+strideq*4]
        m[0] = _mm256_madd_epi16(m[0], m[5]); // pmaddwd m0, m5
        m[1] = _mm256_madd_epi16(m[1], m[5]); // pmaddwd m1, m5
        m[0] = _mm256_add_epi32(m[0], m[1]); // paddd m0, m1
        m[1] = zx(ex1_q2(m[0])); // vextracti128 xm1, m0, 1
        m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
        m[1] = zx(_mm_packs_epi32(lo(m[1]), lo(m[0]))); // packssdw xm1, xm0
        stx_q2(ac, lo(m[1])); // mova [acq], xm1
        ac = at(ac, 16); // add acq, 16
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w4_loop
        }
    }
    if hpad == 0 {
        return ipred_cfl_ac_420_16bpc_dc(m, ac, r5); // test hpadd, hpadd; jz .dc
    }
    m[1] = _mm256_permute4x64_epi64::<0x55>(m[1]); // vpermq m1, m1, q1111
    m[0] = zx(_mm_slli_epi32::<2>(lo(m[0]))); // pslld xm0, 2
    loop {
        st_q2(ac, m[1]); // mova [acq], m1
        m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
        ac = at(ac, 32); // add acq, 32
        hpad -= 4; // sub hpadd, 4
        if hpad <= 0 {
            break; // jg .w4_hpad_loop
        }
    }
    ipred_cfl_ac_420_16bpc_dc(m, ac, r5) // jmp .dc
}

/// `.w8` of `ipred_cfl_ac_420_16bpc`, with `.w8_wpad1` and `.w8_hpad`.
#[inline(always)]
unsafe fn ipred_cfl_ac_420_16bpc_w8(
    m: &mut M,
    mut ac: *mut u8,
    mut ypx: *const u8,
    stride: isize,
    wpad: i32,
    hpad: i32,
    mut h: i32,
) {
    let r5 = ac; // mov r5, acq
    if wpad == 0 {
        // test wpadd, wpadd; jnz .w8_wpad1
        loop {
            m[0] = _mm256_madd_epi16(m[5], ld_q2(ypx)); // pmaddwd m0, m5, [ypxq+strideq*0]
            m[1] = _mm256_madd_epi16(m[5], ld_q2(cp_q2(ypx, stride))); // pmaddwd m1, m5, [ypxq+strideq*1]
            ypx = cp_q2(ypx, stride * 2); // lea ypxq, [ypxq+strideq*2]
            m[0] = _mm256_add_epi32(m[0], m[1]); // paddd m0, m1
            m[1] = zx(ex1_q2(m[0])); // vextracti128 xm1, m0, 1
            m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
            m[1] = zx(_mm_packs_epi32(lo(m[0]), lo(m[1]))); // packssdw xm1, xm0, xm1
            stx_q2(ac, lo(m[1])); // mova [acq], xm1
            ac = at(ac, 16); // add acq, 16
            h -= 1; // dec hd
            if h <= 0 {
                break; // jg .w8_loop
            }
        }
    } else {
        // .w8_wpad1:
        loop {
            m[0] = zx(_mm_madd_epi16(lo(m[5]), ldx_q2(ypx))); // pmaddwd xm0, xm5, [ypxq+strideq*0]
            m[3] = zx(_mm_madd_epi16(lo(m[5]), ldx_q2(cp_q2(ypx, stride)))); // pmaddwd xm3, xm5, [ypxq+strideq*1]
            ypx = cp_q2(ypx, stride * 2); // lea ypxq, [ypxq+strideq*2]
            m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[3]))); // paddd xm0, xm3
            m[3] = zx(_mm_shuffle_epi32::<0xff>(lo(m[0]))); // pshufd xm3, xm0, q3333
            m[1] = zx(_mm_packs_epi32(lo(m[0]), lo(m[3]))); // packssdw xm1, xm0, xm3
            m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[3]))); // paddd xm0, xm3
            m[4] = zx(_mm_add_epi32(lo(m[4]), lo(m[0]))); // paddd xm4, xm0
            stx_q2(ac, lo(m[1])); // mova [acq], xm1
            ac = at(ac, 16); // add acq, 16
            h -= 1; // dec hd
            if h <= 0 {
                break; // jg .w8_wpad1
            }
        }
        // jmp .w8_hpad
    }
    // .w8_hpad:
    if hpad == 0 {
        return ipred_cfl_ac_420_16bpc_dc(m, ac, r5); // test hpadd, hpadd; jz .dc
    }
    m[1] = _mm256_inserti128_si256::<1>(m[1], lo(m[1])); // vinserti128 m1, xm1, 1
    m[0] = _mm256_slli_epi32::<2>(m[0]); // pslld m0, 2
    ipred_cfl_ac_420_16bpc_hpad(m, ac, r5, hpad) // jmp .hpad
}

/// `.w16` of `ipred_cfl_ac_420_16bpc`, with `.w16_wpad`; ends in `.w16_hpad`.
#[inline(always)]
unsafe fn ipred_cfl_ac_420_16bpc_w16(
    m: &mut M,
    mut ac: *mut u8,
    mut ypx: *const u8,
    stride: isize,
    wpad: i32,
    hpad: i32,
    mut h: i32,
) {
    let r5 = ac; // mov r5, acq
    if wpad == 0 {
        // test wpadd, wpadd; jnz .w16_wpad
        loop {
            m[0] = _mm256_madd_epi16(m[5], ld_q2(ypx)); // pmaddwd m0, m5, [ypxq+strideq*0+ 0]
            m[2] = _mm256_madd_epi16(m[5], ld_q2(cp_q2(ypx, 32))); // pmaddwd m2, m5, [ypxq+strideq*0+32]
            m[1] = _mm256_madd_epi16(m[5], ld_q2(cp_q2(ypx, stride))); // pmaddwd m1, m5, [ypxq+strideq*1+ 0]
            m[3] = _mm256_madd_epi16(m[5], ld_q2(cp_q2(ypx, stride + 32))); // pmaddwd m3, m5, [ypxq+strideq*1+32]
            ypx = cp_q2(ypx, stride * 2); // lea ypxq, [ypxq+strideq*2]
            m[0] = _mm256_add_epi32(m[0], m[1]); // paddd m0, m1
            m[2] = _mm256_add_epi32(m[2], m[3]); // paddd m2, m3
            m[1] = _mm256_packs_epi32(m[0], m[2]); // packssdw m1, m0, m2
            m[0] = _mm256_add_epi32(m[0], m[2]); // paddd m0, m2
            m[1] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m1, m1, q3120
            m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
            st_q2(ac, m[1]); // mova [acq], m1
            ac = at(ac, 32); // add acq, 32
            h -= 1; // dec hd
            if h <= 0 {
                break; // jg .w16_loop
            }
        }
    } else {
        // .w16_wpad:
        loop {
            m[0] = ld_q2(ypx); // mova m0, [ypxq+strideq*0+ 0]
            m[1] = ld_q2(cp_q2(ypx, stride)); // mova m1, [ypxq+strideq*1+ 0]
                                              // cmp wpadd, 2; jl .w16_wpad1; je .w16_wpad2
            if wpad < 2 {
                // .w16_wpad1:
                m[2] = bcd_q2(cp_q2(ypx, 44)); // vpbroadcastd m2, [ypxq+strideq*0+44]
                m[3] = bcd_q2(cp_q2(ypx, stride + 44)); // vpbroadcastd m3, [ypxq+strideq*1+44]
                m[2] = _mm256_inserti128_si256::<0>(m[2], ldx_q2(cp_q2(ypx, 32))); // vinserti128 m2, [ypxq+strideq*0+32], 0
                m[3] = _mm256_inserti128_si256::<0>(m[3], ldx_q2(cp_q2(ypx, stride + 32)));
            // vinserti128 m3, [ypxq+strideq*1+32], 0
            } else if wpad == 2 {
                // .w16_wpad2:
                m[2] = bcd_q2(cp_q2(ypx, 28)); // vpbroadcastd m2, [ypxq+strideq*0+28]
                m[3] = bcd_q2(cp_q2(ypx, stride + 28)); // vpbroadcastd m3, [ypxq+strideq*1+28]
            } else {
                m[2] = bcd_q2(cp_q2(ypx, 12)); // vpbroadcastd m2, [ypxq+strideq*0+12]
                m[3] = bcd_q2(cp_q2(ypx, stride + 12)); // vpbroadcastd m3, [ypxq+strideq*1+12]
                m[0] = _mm256_blend_epi32::<0xf0>(m[0], m[2]); // vpblendd m0, m2, 0xf0
                m[1] = _mm256_blend_epi32::<0xf0>(m[1], m[3]); // vpblendd m1, m3, 0xf0
            }
            // .w16_wpad_end:
            ypx = cp_q2(ypx, stride * 2); // lea ypxq, [ypxq+strideq*2]
            m[0] = _mm256_madd_epi16(m[0], m[5]); // REPX {pmaddwd x, m5}, m0, m1, m2, m3
            m[1] = _mm256_madd_epi16(m[1], m[5]);
            m[2] = _mm256_madd_epi16(m[2], m[5]);
            m[3] = _mm256_madd_epi16(m[3], m[5]);
            m[0] = _mm256_add_epi32(m[0], m[1]); // paddd m0, m1
            m[2] = _mm256_add_epi32(m[2], m[3]); // paddd m2, m3
            m[1] = _mm256_packs_epi32(m[0], m[2]); // packssdw m1, m0, m2
            m[0] = _mm256_add_epi32(m[0], m[2]); // paddd m0, m2
            m[1] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m1, m1, q3120
            m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
            st_q2(ac, m[1]); // mova [acq], m1
            ac = at(ac, 32); // add acq, 32
            h -= 1; // dec hd
            if h <= 0 {
                break; // jg .w16_wpad
            }
        }
        // jmp .w16_hpad
    }
    ipred_cfl_ac_420_16bpc_w16_hpad(m, ac, r5, hpad)
}

/// `cglobal ipred_cfl_ac_422_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_ac_422_16bpc_avx2(
    ac: *mut i16,
    y: *const u16,
    stride: isize,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    let (ac, ypx, wpad) = (ac as *mut u8, y as *const u8, w_pad);
    let mut hpad = h_pad; // movifnidn hpadd, hpadm
    m[5] = bcd_q2(ro(z_filter_k, 0)); // vpbroadcastd m5, [pw_4]
    let mut h = ch; // mov hd, hm
    hpad <<= 2; // shl hpadd, 2
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    h -= hpad; // sub hd, hpadd
               // cmp dword wm, 8; jg .w16; je .w8
    if cw > 8 {
        ipred_cfl_ac_422_16bpc_w16(m, ac, ypx, stride, wpad, hpad, h)
    } else if cw == 8 {
        ipred_cfl_ac_422_16bpc_w8(m, ac, ypx, stride, wpad, hpad, h)
    } else {
        ipred_cfl_ac_422_16bpc_w4(m, ac, ypx, stride, hpad, h)
    }
}

/// `.w4` of `ipred_cfl_ac_422_16bpc`.
#[inline(always)]
unsafe fn ipred_cfl_ac_422_16bpc_w4(
    m: &mut M,
    mut ac: *mut u8,
    mut ypx: *const u8,
    stride: isize,
    mut hpad: i32,
    mut h: i32,
) {
    let r3 = stride * 3; // lea r3, [strideq*3]
    let r5 = ac; // mov r5, acq
    loop {
        m[0] = zx(ldx_q2(ypx)); // mova xm0, [ypxq+strideq*0]
        m[1] = zx(ldx_q2(cp_q2(ypx, stride))); // mova xm1, [ypxq+strideq*1]
        m[0] = _mm256_inserti128_si256::<1>(m[0], ldx_q2(cp_q2(ypx, stride * 2))); // vinserti128 m0, [ypxq+strideq*2], 1
        m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_q2(cp_q2(ypx, r3))); // vinserti128 m1, [ypxq+r3], 1
        ypx = cp_q2(ypx, stride * 4); // lea ypxq, [ypxq+strideq*4]
        m[0] = _mm256_madd_epi16(m[0], m[5]); // pmaddwd m0, m5
        m[1] = _mm256_madd_epi16(m[1], m[5]); // pmaddwd m1, m5
        m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
        m[0] = _mm256_packs_epi32(m[0], m[1]); // packssdw m0, m1
        m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1
        st_q2(ac, m[0]); // mova [acq], m0
        ac = at(ac, 32); // add acq, 32
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w4_loop
        }
    }
    if hpad == 0 {
        return ipred_cfl_ac_420_16bpc_dc(m, ac, r5); // test hpadd, hpadd; jz …420….dc
    }
    m[1] = zx(ex1_q2(m[1])); // vextracti128 xm1, m1, 1
    m[0] = _mm256_permute4x64_epi64::<0xff>(m[0]); // vpermq m0, m0, q3333
    m[1] = zx(_mm_slli_epi32::<2>(lo(m[1]))); // pslld xm1, 2
    loop {
        st_q2(ac, m[0]); // mova [acq], m0
        m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1
        ac = at(ac, 32); // add acq, 32
        hpad -= 4; // sub hpadd, 4
        if hpad <= 0 {
            break; // jg .w4_hpad_loop
        }
    }
    ipred_cfl_ac_420_16bpc_dc(m, ac, r5) // jmp …420….dc
}

/// `.w8` of `ipred_cfl_ac_422_16bpc`, with `.w8_wpad1` and `.w8_hpad`.
#[inline(always)]
unsafe fn ipred_cfl_ac_422_16bpc_w8(
    m: &mut M,
    mut ac: *mut u8,
    mut ypx: *const u8,
    stride: isize,
    wpad: i32,
    hpad: i32,
    mut h: i32,
) {
    let r5 = ac; // mov r5, acq
    if wpad == 0 {
        // test wpadd, wpadd; jnz .w8_wpad1
        loop {
            m[1] = _mm256_madd_epi16(m[5], ld_q2(ypx)); // pmaddwd m1, m5, [ypxq+strideq*0]
            m[0] = _mm256_madd_epi16(m[5], ld_q2(cp_q2(ypx, stride))); // pmaddwd m0, m5, [ypxq+strideq*1]
            ypx = cp_q2(ypx, stride * 2); // lea ypxq, [ypxq+strideq*2]
            m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1
            m[1] = _mm256_packs_epi32(m[1], m[0]); // packssdw m1, m0
            m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
            m[2] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m2, m1, q3120
            st_q2(ac, m[2]); // mova [acq], m2
            ac = at(ac, 32); // add acq, 32
            h -= 2; // sub hd, 2
            if h <= 0 {
                break; // jg .w8_loop
            }
        }
    } else {
        // .w8_wpad1:
        loop {
            m[1] = bcd_q2(cp_q2(ypx, 12)); // vpbroadcastd m1, [ypxq+strideq*0+12]
            m[0] = bcd_q2(cp_q2(ypx, stride + 12)); // vpbroadcastd m0, [ypxq+strideq*1+12]
            m[1] = _mm256_inserti128_si256::<0>(m[1], ldx_q2(ypx)); // vinserti128 m1, [ypxq+strideq*0+ 0], 0
            m[0] = _mm256_inserti128_si256::<0>(m[0], ldx_q2(cp_q2(ypx, stride))); // vinserti128 m0, [ypxq+strideq*1+ 0], 0
            ypx = cp_q2(ypx, stride * 2); // lea ypxq, [ypxq+strideq*2]
            m[1] = _mm256_madd_epi16(m[1], m[5]); // pmaddwd m1, m5
            m[0] = _mm256_madd_epi16(m[0], m[5]); // pmaddwd m0, m5
            m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1
            m[1] = _mm256_packs_epi32(m[1], m[0]); // packssdw m1, m0
            m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
            m[2] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m2, m1, q3120
            st_q2(ac, m[2]); // mova [acq], m2
            ac = at(ac, 32); // add acq, 32
            h -= 2; // sub hd, 2
            if h <= 0 {
                break; // jg .w8_wpad1
            }
        }
        // jmp .w8_hpad
    }
    // .w8_hpad:
    if hpad == 0 {
        return ipred_cfl_ac_420_16bpc_dc(m, ac, r5); // test hpadd, hpadd; jz …420….dc
    }
    m[1] = _mm256_permute4x64_epi64::<0xdd>(m[1]); // vpermq m1, m1, q3131
    m[0] = _mm256_slli_epi32::<2>(m[0]); // pslld m0, 2
    ipred_cfl_ac_420_16bpc_hpad(m, ac, r5, hpad) // jmp …420….hpad
}

/// `.w16` of `ipred_cfl_ac_422_16bpc`, with `.w16_wpad`; ends in 420's `.w16_hpad`.
#[inline(always)]
unsafe fn ipred_cfl_ac_422_16bpc_w16(
    m: &mut M,
    mut ac: *mut u8,
    mut ypx: *const u8,
    stride: isize,
    wpad: i32,
    hpad: i32,
    mut h: i32,
) {
    let r5 = ac; // mov r5, acq
    if wpad == 0 {
        // test wpadd, wpadd; jnz .w16_wpad
        loop {
            m[2] = _mm256_madd_epi16(m[5], ld_q2(ypx)); // pmaddwd m2, m5, [ypxq+strideq*0+ 0]
            m[1] = _mm256_madd_epi16(m[5], ld_q2(cp_q2(ypx, 32))); // pmaddwd m1, m5, [ypxq+strideq*0+32]
            m[0] = _mm256_madd_epi16(m[5], ld_q2(cp_q2(ypx, stride))); // pmaddwd m0, m5, [ypxq+strideq*1+ 0]
            m[3] = _mm256_madd_epi16(m[5], ld_q2(cp_q2(ypx, stride + 32))); // pmaddwd m3, m5, [ypxq+strideq*1+32]
            ypx = cp_q2(ypx, stride * 2); // lea ypxq, [ypxq+strideq*2]
            m[4] = _mm256_add_epi32(m[4], m[2]); // paddd m4, m2
            m[2] = _mm256_packs_epi32(m[2], m[1]); // packssdw m2, m1
            m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1
            m[1] = _mm256_packs_epi32(m[0], m[3]); // packssdw m1, m0, m3
            m[0] = _mm256_add_epi32(m[0], m[3]); // paddd m0, m3
            m[2] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m2, m2, q3120
            m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
            m[1] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m1, m1, q3120
            st_q2(ac, m[2]); // mova [acq+32*0], m2
            st_q2(at(ac, 32), m[1]); // mova [acq+32*1], m1
            ac = at(ac, 32 * 2); // add acq, 32*2
            h -= 2; // sub hd, 2
            if h <= 0 {
                break; // jg .w16_loop
            }
        }
    } else {
        // .w16_wpad:
        loop {
            m[2] = ld_q2(ypx); // mova m2, [ypxq+strideq*0+ 0]
            m[0] = ld_q2(cp_q2(ypx, stride)); // mova m0, [ypxq+strideq*1+ 0]
                                              // cmp wpadd, 2; jl .w16_wpad1; je .w16_wpad2
            if wpad < 2 {
                // .w16_wpad1:
                m[1] = bcd_q2(cp_q2(ypx, 44)); // vpbroadcastd m1, [ypxq+strideq*0+44]
                m[3] = bcd_q2(cp_q2(ypx, stride + 44)); // vpbroadcastd m3, [ypxq+strideq*1+44]
                m[1] = _mm256_inserti128_si256::<0>(m[1], ldx_q2(cp_q2(ypx, 32))); // vinserti128 m1, [ypxq+strideq*0+32], 0
                m[3] = _mm256_inserti128_si256::<0>(m[3], ldx_q2(cp_q2(ypx, stride + 32)));
            // vinserti128 m3, [ypxq+strideq*1+32], 0
            } else if wpad == 2 {
                // .w16_wpad2:
                m[1] = bcd_q2(cp_q2(ypx, 28)); // vpbroadcastd m1, [ypxq+strideq*0+28]
                m[3] = bcd_q2(cp_q2(ypx, stride + 28)); // vpbroadcastd m3, [ypxq+strideq*1+28]
            } else {
                m[1] = bcd_q2(cp_q2(ypx, 12)); // vpbroadcastd m1, [ypxq+strideq*0+12]
                m[3] = bcd_q2(cp_q2(ypx, stride + 12)); // vpbroadcastd m3, [ypxq+strideq*1+12]
                m[2] = _mm256_blend_epi32::<0xf0>(m[2], m[1]); // vpblendd m2, m1, 0xf0
                m[0] = _mm256_blend_epi32::<0xf0>(m[0], m[3]); // vpblendd m0, m3, 0xf0
            }
            // .w16_wpad_end:
            ypx = cp_q2(ypx, stride * 2); // lea ypxq, [ypxq+strideq*2]
            m[2] = _mm256_madd_epi16(m[2], m[5]); // REPX {pmaddwd x, m5}, m2, m0, m1, m3
            m[0] = _mm256_madd_epi16(m[0], m[5]);
            m[1] = _mm256_madd_epi16(m[1], m[5]);
            m[3] = _mm256_madd_epi16(m[3], m[5]);
            m[4] = _mm256_add_epi32(m[4], m[2]); // paddd m4, m2
            m[2] = _mm256_packs_epi32(m[2], m[1]); // packssdw m2, m1
            m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1
            m[1] = _mm256_packs_epi32(m[0], m[3]); // packssdw m1, m0, m3
            m[0] = _mm256_add_epi32(m[0], m[3]); // paddd m0, m3
            m[2] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m2, m2, q3120
            m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
            m[1] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m1, m1, q3120
            st_q2(ac, m[2]); // mova [acq+32*0], m2
            st_q2(at(ac, 32), m[1]); // mova [acq+32*1], m1
            ac = at(ac, 32 * 2); // add acq, 32*2
            h -= 2; // sub hd, 2
            if h <= 0 {
                break; // jg .w16_wpad
            }
        }
    }
    ipred_cfl_ac_420_16bpc_w16_hpad(m, ac, r5, hpad) // jmp …420….w16_hpad
}

/// `cglobal ipred_cfl_ac_444_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_ac_444_16bpc_avx2(
    ac: *mut i16,
    y: *const u16,
    stride: isize,
    w_pad: c_int,
    h_pad: c_int,
    cw: c_int,
    ch: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    let (ac, ypx, wpad) = (ac as *mut u8, y as *const u8, w_pad);
    let w = (cw as u32).trailing_zeros(); // tzcnt wd, wm (cw is a power of two >= 4: never zero)
    let mut hpad = h_pad; // movifnidn hpadd, hpadm
    m[5] = bcd_q2(ro(pw_1, 0)); // vpbroadcastd m5, [pw_1]
    hpad <<= 2; // shl hpadd, 2
    let mut h = ch; // mov hd, hm
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    h -= hpad; // sub hd, hpadd
               // movsxd wq, [r6+wq*4]; add wq, r6; jmp wq
    match w {
        2 => ipred_cfl_ac_444_16bpc_w4(m, ac, ypx, stride, hpad, h),
        3 => ipred_cfl_ac_444_16bpc_w8(m, ac, ypx, stride, hpad, h),
        4 => ipred_cfl_ac_444_16bpc_w16(m, ac, ypx, stride, wpad, hpad, h),
        _ => ipred_cfl_ac_444_16bpc_w32(m, ac, ypx, stride, wpad, hpad, h),
    }
}

/// `.w4` of `ipred_cfl_ac_444_16bpc`.
#[inline(always)]
unsafe fn ipred_cfl_ac_444_16bpc_w4(
    m: &mut M,
    mut ac: *mut u8,
    mut ypx: *const u8,
    stride: isize,
    hpad: i32,
    mut h: i32,
) {
    let r3 = stride * 3; // lea r3, [strideq*3]
    let r5 = ac; // mov r5, acq
    loop {
        m[0] = zx(_mm_loadl_epi64(ypx.cast())); // movq xm0, [ypxq+strideq*0]
        m[0] = zx(_mm_castpd_si128(_mm_loadh_pd(
            _mm_castsi128_pd(lo(m[0])),
            cp_q2(ypx, stride).cast(),
        ))); // movhps xm0, [ypxq+strideq*1]
        m[1] = bcq_q2(cp_q2(ypx, stride * 2)); // vpbroadcastq m1, [ypxq+strideq*2]
        m[2] = bcq_q2(cp_q2(ypx, r3)); // vpbroadcastq m2, [ypxq+r3]
        ypx = cp_q2(ypx, stride * 4); // lea ypxq, [ypxq+strideq*4]
        m[0] = _mm256_blend_epi32::<0x30>(m[0], m[1]); // vpblendd m0, m1, 0x30
        m[0] = _mm256_blend_epi32::<0xc0>(m[0], m[2]); // vpblendd m0, m2, 0xc0
        m[0] = _mm256_slli_epi16::<3>(m[0]); // psllw m0, 3
        m[1] = _mm256_madd_epi16(m[0], m[5]); // pmaddwd m1, m0, m5
        st_q2(ac, m[0]); // mova [acq], m0
        ac = at(ac, 32); // add acq, 32
        m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w4_loop
        }
    }
    if hpad == 0 {
        return ipred_cfl_ac_420_16bpc_dc(m, ac, r5); // test hpadd, hpadd; jz …420….dc
    }
    m[0] = _mm256_permute4x64_epi64::<0xff>(m[0]); // vpermq m0, m0, q3333
    m[1] = _mm256_add_epi32(m[1], m[1]); // paddd m1, m1
    st_q2(ac, m[0]); // mova [acq+32*0], m0
    m[1] = _mm256_permute4x64_epi64::<0xff>(m[1]); // vpermq m1, m1, q3333
    st_q2(at(ac, 32), m[0]); // mova [acq+32*1], m0
    ac = at(ac, 32 * 2); // add acq, 32*2
    m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1
    ipred_cfl_ac_420_16bpc_dc(m, ac, r5) // jmp …420….dc
}

/// `.w8` of `ipred_cfl_ac_444_16bpc`.
#[inline(always)]
unsafe fn ipred_cfl_ac_444_16bpc_w8(
    m: &mut M,
    mut ac: *mut u8,
    mut ypx: *const u8,
    stride: isize,
    hpad: i32,
    mut h: i32,
) {
    let r3 = stride * 3; // lea r3, [strideq*3]
    let r5 = ac; // mov r5, acq
    loop {
        m[2] = zx(ldx_q2(ypx)); // mova xm2, [ypxq+strideq*0]
        m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_q2(cp_q2(ypx, stride))); // vinserti128 m2, [ypxq+strideq*1], 1
        m[1] = zx(ldx_q2(cp_q2(ypx, stride * 2))); // mova xm1, [ypxq+strideq*2]
        m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_q2(cp_q2(ypx, r3))); // vinserti128 m1, [ypxq+r3], 1
        ypx = cp_q2(ypx, stride * 4); // lea ypxq, [ypxq+strideq*4]
        m[2] = _mm256_slli_epi16::<3>(m[2]); // psllw m2, 3
        m[1] = _mm256_slli_epi16::<3>(m[1]); // psllw m1, 3
        st_q2(ac, m[2]); // mova [acq+32*0], m2
        m[2] = _mm256_madd_epi16(m[2], m[5]); // pmaddwd m2, m5
        st_q2(at(ac, 32), m[1]); // mova [acq+32*1], m1
        m[0] = _mm256_madd_epi16(m[1], m[5]); // pmaddwd m0, m1, m5
        ac = at(ac, 32 * 2); // add acq, 32*2
        m[4] = _mm256_add_epi32(m[4], m[2]); // paddd m4, m2
        m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w8_loop
        }
    }
    if hpad == 0 {
        return ipred_cfl_ac_420_16bpc_dc(m, ac, r5); // test hpadd, hpadd; jz …420….dc
    }
    m[1] = _mm256_permute2x128_si256::<0x11>(m[1], m[1]); // vperm2i128 m1, m1, 0x11
    m[0] = _mm256_slli_epi32::<2>(m[0]); // pslld m0, 2
    m[2] = _mm256_setzero_si256(); // pxor m2, m2
    m[0] = _mm256_blend_epi32::<0x0f>(m[0], m[2]); // vpblendd m0, m2, 0x0f
    ipred_cfl_ac_420_16bpc_hpad(m, ac, r5, hpad) // jmp …420….hpad
}

/// `.w16` of `ipred_cfl_ac_444_16bpc`, with `.w16_wpad2`.
#[inline(always)]
unsafe fn ipred_cfl_ac_444_16bpc_w16(
    m: &mut M,
    mut ac: *mut u8,
    mut ypx: *const u8,
    stride: isize,
    wpad: i32,
    mut hpad: i32,
    mut h: i32,
) {
    let r5 = ac; // mov r5, acq
    loop {
        m[2] = ld_q2(ypx); // mova m2, [ypxq+strideq*0]
        m[1] = ld_q2(cp_q2(ypx, stride)); // mova m1, [ypxq+strideq*1]
        if wpad != 0 {
            // test wpadd, wpadd; jnz .w16_wpad2
            m[3] = bcw_q2(cp_q2(ypx, 14)); // vpbroadcastw m3, [ypxq+strideq*0+14]
            m[0] = bcw_q2(cp_q2(ypx, stride + 14)); // vpbroadcastw m0, [ypxq+strideq*1+14]
            m[2] = _mm256_blend_epi32::<0xf0>(m[2], m[3]); // vpblendd m2, m3, 0xf0
            m[1] = _mm256_blend_epi32::<0xf0>(m[1], m[0]); // vpblendd m1, m0, 0xf0
                                                           // jmp .w16_wpad_end
        }
        // .w16_wpad_end:
        ypx = cp_q2(ypx, stride * 2); // lea ypxq, [ypxq+strideq*2]
        m[2] = _mm256_slli_epi16::<3>(m[2]); // psllw m2, 3
        m[1] = _mm256_slli_epi16::<3>(m[1]); // psllw m1, 3
        st_q2(ac, m[2]); // mova [acq+32*0], m2
        m[2] = _mm256_madd_epi16(m[2], m[5]); // pmaddwd m2, m5
        st_q2(at(ac, 32), m[1]); // mova [acq+32*1], m1
        m[0] = _mm256_madd_epi16(m[1], m[5]); // pmaddwd m0, m1, m5
        ac = at(ac, 32 * 2); // add acq, 32*2
        m[4] = _mm256_add_epi32(m[4], m[2]); // paddd m4, m2
        m[4] = _mm256_add_epi32(m[4], m[0]); // paddd m4, m0
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w16_loop
        }
    }
    hpad += hpad; // add hpadd, hpadd
    if hpad == 0 {
        return ipred_cfl_ac_420_16bpc_dc(m, ac, r5); // jz …420….dc
    }
    m[0] = _mm256_add_epi32(m[0], m[0]); // paddd m0, m0
    ipred_cfl_ac_420_16bpc_hpad(m, ac, r5, hpad) // jmp …420….hpad
}

/// `.w32` of `ipred_cfl_ac_444_16bpc`, with `.w32_wpad*` and `.w32_hpad`.
#[inline(always)]
unsafe fn ipred_cfl_ac_444_16bpc_w32(
    m: &mut M,
    mut ac: *mut u8,
    mut ypx: *const u8,
    stride: isize,
    wpad: i32,
    mut hpad: i32,
    mut h: i32,
) {
    let r5 = ac; // mov r5, acq
    if wpad == 0 {
        // test wpadd, wpadd; jnz .w32_wpad
        loop {
            m[0] = ld_q2(ypx); // mova m0, [ypxq+ 0]
            m[1] = ld_q2(cp_q2(ypx, 32)); // mova m1, [ypxq+32]
            ypx = cp_q2(ypx, stride); // add ypxq, strideq
            m[0] = _mm256_slli_epi16::<3>(m[0]); // psllw m0, 3
            m[1] = _mm256_slli_epi16::<3>(m[1]); // psllw m1, 3
            m[2] = _mm256_madd_epi16(m[0], m[5]); // pmaddwd m2, m0, m5
            st_q2(ac, m[0]); // mova [acq+32*0], m0
            m[3] = _mm256_madd_epi16(m[1], m[5]); // pmaddwd m3, m1, m5
            st_q2(at(ac, 32), m[1]); // mova [acq+32*1], m1
            ac = at(ac, 32 * 2); // add acq, 32*2
            m[2] = _mm256_add_epi32(m[2], m[3]); // paddd m2, m3
            m[4] = _mm256_add_epi32(m[4], m[2]); // paddd m4, m2
            h -= 1; // dec hd
            if h <= 0 {
                break; // jg .w32_loop
            }
        }
    } else {
        // .w32_wpad:
        loop {
            m[0] = ld_q2(ypx); // mova m0, [ypxq+ 0]
                               // cmp wpadd, 4; jl .w32_wpad2; je .w32_wpad4
            if wpad < 4 {
                // .w32_wpad2:
                m[1] = bcw_q2(cp_q2(ypx, 46)); // vpbroadcastw m1, [ypxq+46]
                m[1] = _mm256_inserti128_si256::<0>(m[1], ldx_q2(cp_q2(ypx, 32)));
            // vinserti128 m1, [ypxq+32], 0
            } else if wpad == 4 {
                // .w32_wpad4:
                m[1] = bcw_q2(cp_q2(ypx, 30)); // vpbroadcastw m1, [ypxq+30]
            } else {
                m[1] = bcw_q2(cp_q2(ypx, 14)); // vpbroadcastw m1, [ypxq+14]
                m[0] = _mm256_blend_epi32::<0xf0>(m[0], m[1]); // vpblendd m0, m1, 0xf0
            }
            // .w32_wpad_end:
            ypx = cp_q2(ypx, stride); // add ypxq, strideq
            m[0] = _mm256_slli_epi16::<3>(m[0]); // psllw m0, 3
            m[1] = _mm256_slli_epi16::<3>(m[1]); // psllw m1, 3
            m[2] = _mm256_madd_epi16(m[0], m[5]); // pmaddwd m2, m0, m5
            st_q2(ac, m[0]); // mova [acq+32*0], m0
            m[3] = _mm256_madd_epi16(m[1], m[5]); // pmaddwd m3, m1, m5
            st_q2(at(ac, 32), m[1]); // mova [acq+32*1], m1
            ac = at(ac, 32 * 2); // add acq, 32*2
            m[2] = _mm256_add_epi32(m[2], m[3]); // paddd m2, m3
            m[4] = _mm256_add_epi32(m[4], m[2]); // paddd m4, m2
            h -= 1; // dec hd
            if h <= 0 {
                break; // jg .w32_wpad
            }
        }
        // jmp .w32_hpad
    }
    // .w32_hpad:
    if hpad == 0 {
        return ipred_cfl_ac_420_16bpc_dc(m, ac, r5); // test hpadd, hpadd; jz …420….dc
    }
    m[2] = _mm256_add_epi32(m[2], m[2]); // paddd m2, m2
    loop {
        st_q2(ac, m[0]); // mova [acq+32*0], m0
        st_q2(at(ac, 32), m[1]); // mova [acq+32*1], m1
        m[4] = _mm256_add_epi32(m[4], m[2]); // paddd m4, m2
        st_q2(at(ac, 64), m[0]); // mova [acq+32*2], m0
        st_q2(at(ac, 96), m[1]); // mova [acq+32*3], m1
        ac = at(ac, 32 * 4); // add acq, 32*4
        hpad -= 2; // sub hpadd, 2
        if hpad <= 0 {
            break; // jg .w32_hpad_loop
        }
    }
    ipred_cfl_ac_420_16bpc_dc(m, ac, r5) // jmp …420….dc
}

// ---- part 3: `ipred16_avx2.asm` lines 1102–1841 (z1) ----
//
// `cglobal ipred_z1_16bpc` is `dav1d_ipred_z1_16bpc_avx2`; its jump table (`ipred_z1_16bpc_avx2_table`: w4 … w64,
// indexed by `tzcnt w`) is the `match` there, and each width label with the code it falls or jumps into
// (`.wN_upsample*`, `.wN_no_upsample`, `.wN_no_intra_edge_filter`, `.wN_filter*`, `.wN_main`, `.wN_loop`,
// `.wN_end_loop`) is `ipred_z1_16bpc_wN`, with the branches as `if`s in the assembly's order. The `call`ed label
// `.filter_strength` is `ipred_z1_16bpc_filter_strength`. The general-purpose registers are locals named as
// `DEFINE_ARGS` names them (`dx`, `maxbase`, `h`, `angle`, `tl`, `dst`) or by number (`r2`, `r3`, `r5`).
//
// Frames (`ALLOC_STACK`): each width's frame is a local that is zero-initialized at the point of the
// path where the assembly first writes it (the upsampling and edge-filter paths; the unfiltered paths do not touch
// it), and it is larger than the assembly's: the main loops load from `tl + base*2` for every row they compute,
// also for rows and columns whose result the `vpblendvb` with `top[max_base_x]` then discards, and those loads
// reach beyond the assembly's frame (into its caller's stack) when `tl` points into the frame. The sizes cover the
// largest load of each loop (see the frame sizes below).

/// The frame of one width path: `N * 32` bytes. Only 8-byte aligned (the assembly's is 32-byte aligned): every access
/// to it is an unaligned load or store, and a 32-byte aligned local costs the function a realigned stack frame.
type FrameQ3<const N: usize> = [[u64; 4]; N];

/// `[p + off]`: a byte address (wrapping; no claim that it is in bounds until it is accessed).
#[inline(always)]
fn p_q3(p: *const u8, off: isize) -> *const u8 {
    p.wrapping_offset(off)
}

/// `[p + base*2 + off]`: a pixel address (wrapping). Indexed in pixels, so that the compiler puts the `*2` into
/// the address (`[tlq+r3*2]`) as the assembly does instead of folding it into the shift (`shr 5; and`).
#[inline(always)]
fn pb_q3(p: *const u8, base: u32, off: isize) -> *const u8 {
    (p as *const u16).wrapping_add(base as usize).wrapping_byte_offset(off) as *const u8
}

/// A 32-byte load (`movu`/`mova` m, or a 32-byte memory operand).
#[inline(always)]
unsafe fn ld_q3(p: *const u8) -> __m256i {
    _mm256_loadu_si256(p.cast())
}

/// A 16-byte load (`movu`/`mova` xm, or a 16-byte memory operand).
#[inline(always)]
unsafe fn ldx_q3(p: *const u8) -> __m128i {
    _mm_loadu_si128(p.cast())
}

/// `movzx r, word [p]`.
#[inline(always)]
unsafe fn ldw_q3(p: *const u8) -> u32 {
    (p as *const u16).read_unaligned() as u32
}

/// `vpbroadcastw m, [p]`.
#[inline(always)]
unsafe fn bcw_q3(p: *const u8) -> __m256i {
    _mm256_set1_epi16((p as *const i16).read_unaligned())
}

/// `vpbroadcastd m, [label+off]` (read-only data).
#[inline(always)]
unsafe fn bcd_q3(label: usize, off: isize) -> __m256i {
    _mm256_set1_epi32((ro(label, off) as *const i32).read_unaligned())
}

/// A 32-byte store (`mova [p], m`).
#[inline(always)]
unsafe fn st_q3(p: *mut u8, x: __m256i) {
    _mm256_storeu_si256(p.cast(), x)
}

/// A 16-byte store (`mova [p], xm`, `vextracti128 [p], m, 1`).
#[inline(always)]
unsafe fn stx_q3(p: *mut u8, x: __m128i) {
    _mm_storeu_si128(p.cast(), x)
}

/// `movhps [p], xm`: the high 8 bytes of `x` to `p`. Written as the store of `pextrq` (`vpextrq [p], xm, 1`, the
/// same bytes and one instruction): LLVM lowers `_mm_storeh_pd` here to `vpshufd` and `vmovq` (two).
#[inline(always)]
unsafe fn sth_q3(p: *mut u8, x: __m128i) {
    (p as *mut i64).write_unaligned(_mm_extract_epi64::<1>(x))
}

// The frame sizes. The assembly's frame and the size here:
// w4 64 → 160 bytes, w8 64 → 96, w16 96 → 160, w32 160 → 224, w64 256 → 384. The largest load of each main loop
// when `tl` is the frame: `xpos` of a loaded row is below `maxbase*64 + (rows per iteration - 1) * dx`, or
// `rows per iteration * dx` in the first iteration, with `dx <= 1023` (`dr_intra_derivative`), so `base` is at
// most 63 (w4: 4 rows; `maxbase <= 9`), 32 (w8: 2 rows; `maxbase <= 17`), 48 (w16: 2 rows; `maxbase <= 33`), 64
// (w32: 1 row; `maxbase <= 65`), 126 (w64: 1 row; `maxbase <= 127`), and the loads end at `base*2` + 16, 18, 34,
// 66, 130 bytes: 142, 82, 130, 194, 382. The upsampling paths of w4 and w8 have their own frame of the assembly's
// 64 bytes (`fr_us`): there `d < 40`, so `dx <= 51` before it is doubled, and `h <= 8`, so `base <= 8*102/64 = 12`
// and the loads end at byte 40 (w4) and 56 (w8) at the latest.

/// `.filter_strength` (`call`ed by w4, w8 and w16): the bit mask whose population count is the filter strength
/// (`r5d`); `angle` is shifted right by 8 as the assembly does (`is_sm << 1`).
#[inline(always)]
unsafe fn ipred_z1_16bpc_filter_strength(m: &mut M, maxbase: u32, angle: &mut i32) -> u32 {
    m[0] = zx(_mm_cvtsi32_si128(maxbase as i32)); // movd xm0, maxbased
                                                  // lea r3, [z_filter_t0] (the base of the loads below)
    m[1] = zx(_mm_cvtsi32_si128(*angle)); // movd xm1, angled
    *angle = ((*angle as u32) >> 8) as i32; // shr angled, 8 ; is_sm << 1
    m[0] = _mm256_broadcastb_epi8(lo(m[0])); // vpbroadcastb m0, xm0
    m[1] = _mm256_broadcastb_epi8(lo(m[1])); // vpbroadcastb m1, xm1
    m[0] = _mm256_cmpeq_epi8(m[0], ld_q3(ro(z_filter_wh, 0))); // pcmpeqb m0, [base+z_filter_wh]
                                                               // The callers come here only without 0x400 (`test angled, 0x400`), so `angle` is 0 or 2 now: z_filter_t0 or
                                                               // z_filter_t1, inside RODATA.
    m[2] = zx(ldx_q3(ro(z_filter_t0, *angle as isize * 8))); // mova xm2, [r3+angleq*8]
    m[0] = _mm256_and_si256(m[0], m[1]); // pand m0, m1
    m[0] = _mm256_cmpgt_epi8(m[0], m[2]); // pcmpgtb m0, m2
    _mm256_movemask_epi8(m[0]) as u32 // pmovmskb r5d, m0 ; ret
}

/// `[base+z_filter_k-4+r5*4+12*k]`: `r5` is the popcount of `.filter_strength`'s mask, 1 to 3 (each `z_filter_wh`
/// value occurs at most three times), so the dword lies inside `z_filter_k`.
#[inline(always)]
unsafe fn ipred_z1_16bpc_filter_k_q3(r5: u32, k: isize) -> __m256i {
    bcd_q3(z_filter_k, -4 + r5 as isize * 4 + 12 * k)
}

/// `.w4` up to its `RET`s.
#[inline(always)]
unsafe fn ipred_z1_16bpc_w4(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    tl: *const u8,
    h: i32,
    angle: i32,
    dx: u32,
    bdmax: i32,
) {
    let (mut dst, mut tl, mut h, mut angle, mut dx) = (dst, tl, h, angle, dx);
    let mut fr: FrameQ3<5>; // ALLOC_STACK -64, 7 (see the frame sizes above)
    let mut fr_us: FrameQ3<2>; // the same frame on the upsampling path: 64 bytes suffice there
    let mut maxbase: u32;
    'no_upsample: {
        if (angle as u8) >= 40 {
            break 'no_upsample; // cmp angleb, 40 ; jae .w4_no_upsample
        }
        let mut r3 = angle.wrapping_sub(1024); // lea r3d, [angleq-1024]
        r3 >>= 7; // sar r3d, 7
        r3 = r3.wrapping_add(h); // add r3d, hd
        if r3 > 0 {
            break 'no_upsample; // jg .w4_no_upsample ; !enable_intra_edge_filter || h > 8 || (h == 8 && is_sm)
        }
        fr_us = [[0; 4]; 2];
        let rsp = fr_us.as_mut_ptr().cast::<u8>();
        m[3] = zx(_mm_set1_epi16((p_q3(tl, 14) as *const i16).read_unaligned())); // vpbroadcastw xm3, [tlq+14]
        m[1] = zx(ldx_q3(tl)); // movu xm1, [tlq+ 0] ; 1 2 3 4 5 6 7 8
        m[0] = zx(_mm_alignr_epi8::<4>(lo(m[3]), lo(m[1]))); // palignr xm0, xm3, xm1, 4 ; 3 4 5 6 7 8 8 8
        m[0] = zx(_mm_add_epi16(lo(m[0]), ldx_q3(p_q3(tl, -2)))); // paddw xm0, [tlq- 2] ; 0 1 2 3 4 5 6 7
        dx = dx.wrapping_add(dx); // add dxd, dxd
        m[2] = zx(_mm_alignr_epi8::<2>(lo(m[3]), lo(m[1]))); // palignr xm2, xm3, xm1, 2 ; 2 3 4 5 6 7 8 8
        m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[1]))); // paddw xm2, xm1
        m[0] = zx(_mm_sub_epi16(lo(m[2]), lo(m[0]))); // psubw xm0, xm2, xm0
        m[0] = zx(_mm_srai_epi16::<3>(lo(m[0]))); // psraw xm0, 3
        m[4] = zx(_mm_setzero_si128()); // pxor xm4, xm4
        m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[0]))); // paddw xm2, xm0
        m[0] = zx(_mm_set1_epi16(bdmax as i16)); // vpbroadcastw xm0, r8m ; pixel_max
        stx_q3(rsp.wrapping_add(32), lo(m[3])); // mova [rsp+32], xm3
        m[3] = zx(_mm_cvtsi32_si128(dx as i32)); // movd xm3, dxd
        m[2] = zx(_mm_max_epi16(lo(m[2]), lo(m[4]))); // pmaxsw xm2, xm4
        let mut r3 = dx; // mov r3d, dxd
        m[2] = zx(_mm_avg_epu16(lo(m[2]), lo(m[4]))); // pavgw xm2, xm4
        m[3] = _mm256_broadcastw_epi16(lo(m[3])); // vpbroadcastw m3, xm3
        m[2] = zx(_mm_min_epi16(lo(m[2]), lo(m[0]))); // pminsw xm2, xm0
        m[0] = zx(_mm_unpacklo_epi16(lo(m[1]), lo(m[2]))); // punpcklwd xm0, xm1, xm2
        m[1] = zx(_mm_unpackhi_epi16(lo(m[1]), lo(m[2]))); // punpckhwd xm1, xm2
        let r5 = stride * 3; // lea r5, [strideq*3]
        m[2] = _mm256_bslli_epi128::<8>(m[3]); // pslldq m2, m3, 8
        stx_q3(rsp, lo(m[0])); // mova [rsp+ 0], xm0
        stx_q3(rsp.wrapping_add(16), lo(m[1])); // mova [rsp+16], xm1
        m[6] = _mm256_add_epi16(m[3], m[3]); // paddw m6, m3, m3
        m[3] = _mm256_add_epi16(m[3], m[2]); // paddw m3, m2
        m[4] = _mm256_blend_epi32::<0xf0>(m[4], m[6]); // vpblendd m4, m6, 0xf0
        m[6] = _mm256_add_epi16(m[6], m[6]); // paddw m6, m6
        m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4 ; xpos0 xpos1 xpos2 xpos3
        m[4] = _mm256_broadcastsi128_si256(ldx_q3(ro(z_upsample, 0))); // vbroadcasti128 m4, [z_upsample]
        let rsp = rsp as *const u8;
        loop {
            // .w4_upsample_loop
            let mut r2 = r3.wrapping_add(dx); // lea r2d, [r3+dxq]
            r3 >>= 6; // shr r3d, 6 ; base0
            m[1] = zx(ldx_q3(pb_q3(rsp, r3, 0))); // movu xm1, [rsp+r3*2]
            r3 = r2.wrapping_add(dx); // lea r3d, [r2+dxq]
            r2 >>= 6; // shr r2d, 6 ; base1
            m[2] = zx(ldx_q3(pb_q3(rsp, r2, 0))); // movu xm2, [rsp+r2*2]
            r2 = r3.wrapping_add(dx); // lea r2d, [r3+dxq]
            r3 >>= 6; // shr r3d, 6 ; base2
            m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_q3(pb_q3(rsp, r3, 0))); // vinserti128 m1, [rsp+r3*2], 1
            r3 = r2.wrapping_add(dx); // lea r3d, [r2+dxq]
            r2 >>= 6; // shr r2d, 6 ; base3
            m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_q3(pb_q3(rsp, r2, 0))); // vinserti128 m2, [rsp+r2*2], 1
            m[1] = _mm256_shuffle_epi8(m[1], m[4]); // pshufb m1, m4
            m[2] = _mm256_shuffle_epi8(m[2], m[4]); // pshufb m2, m4
            m[0] = _mm256_unpacklo_epi64(m[1], m[2]); // punpcklqdq m0, m1, m2
            m[1] = _mm256_unpackhi_epi64(m[1], m[2]); // punpckhqdq m1, m2
            m[2] = _mm256_and_si256(m[5], m[3]); // pand m2, m5, m3 ; frac
            m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
            m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
            m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
            m[3] = _mm256_add_epi16(m[3], m[6]); // paddw m3, m6 ; xpos += dx
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
            _mm_storel_epi64(dst.cast(), lo(m[0])); // movq [dstq+strideq*0], xm0
            sth_q3(dst.wrapping_offset(stride), lo(m[0])); // movhps [dstq+strideq*1], xm0
            _mm_storel_epi64(dst.wrapping_offset(stride * 2).cast(), lo(m[1])); // movq [dstq+strideq*2], xm1
            sth_q3(dst.wrapping_offset(r5), lo(m[1])); // movhps [dstq+r5], xm1
            dst = dst.wrapping_offset(stride * 4); // lea dstq, [dstq+strideq*4]
            h -= 4; // sub hd, 4
            if h <= 0 {
                return; // jg .w4_upsample_loop ; RET
            }
        }
    }
    // .w4_no_upsample
    maxbase = 7; // mov maxbased, 7
    'main: {
        if angle & 0x400 != 0 {
            break 'main; // test angled, 0x400 ; jnz .w4_main ; !enable_intra_edge_filter
        }
        maxbase = (h + 3) as u32; // lea maxbased, [hq+3]
        let mut r5 = ipred_z1_16bpc_filter_strength(m, maxbase, &mut angle); // call .filter_strength
        maxbase = 7; // mov maxbased, 7
        if r5 == 0 {
            break 'main; // test r5d, r5d ; jz .w4_main ; filter_strength == 0
        }
        fr = [[0; 4]; 5];
        let rsp = fr.as_mut_ptr().cast::<u8>();
        r5 = r5.count_ones(); // popcnt r5d, r5d
        m[3] = zx(_mm_set1_epi16((p_q3(tl, 14) as *const i16).read_unaligned())); // vpbroadcastw xm3, [tlq+14]
        m[0] = zx(ldx_q3(p_q3(tl, -2))); // mova xm0, [tlq- 2] ; 0 1 2 3 4 5 6 7
        m[1] = zx(lo(ipred_z1_16bpc_filter_k_q3(r5, 1))); // vpbroadcastd xm1, [base+z_filter_k-4+r5*4+12*1]
        m[4] = zx(lo(ipred_z1_16bpc_filter_k_q3(r5, 0))); // vpbroadcastd xm4, [base+z_filter_k-4+r5*4+12*0]
        m[2] = zx(_mm_alignr_epi8::<4>(lo(m[3]), lo(m[0]))); // palignr xm2, xm3, xm0, 4 ; 2 3 4 5 6 7 8 8
        m[1] = zx(_mm_mullo_epi16(lo(m[1]), ldx_q3(tl))); // pmullw xm1, [tlq+ 0] ; 1 2 3 4 5 6 7 8
        m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[0]))); // paddw xm2, xm0
        m[2] = zx(_mm_mullo_epi16(lo(m[2]), lo(m[4]))); // pmullw xm2, xm4
        (rsp.wrapping_add(16) as *mut i32).write_unaligned(_mm_cvtsi128_si32(lo(m[3]))); // movd [rsp+16], xm3
        if r5 == 3 {
            // cmp r5d, 3 ; jne .w4_3tap
            m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
            m[2] = zx(_mm_alignr_epi8::<6>(lo(m[3]), lo(m[0]))); // palignr xm2, xm3, xm0, 6 ; 3 4 5 6 7 8 8 8
            m[0] = zx(_mm_blend_epi16::<0xfe>(lo(m[0]), ldx_q3(p_q3(tl, -4)))); // pblendw xm0, [tlq-4], 0xfe ; 0 0 1 2 3 4 5 6
            let r3 = ldw_q3(p_q3(tl, 14)); // movzx r3d, word [tlq+14]
            let mut r2 = ldw_q3(p_q3(tl, 12)); // movzx r2d, word [tlq+12]
            maxbase += 1; // inc maxbased
            m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[0]))); // paddw xm2, xm0
            r2 = r2.wrapping_sub(r3); // sub r2d, r3d
            m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[2]))); // paddw xm2, xm2
            r2 = r2.wrapping_add(r3.wrapping_mul(8)).wrapping_add(4); // lea r2d, [r2+r3*8+4]
            r2 >>= 3; // shr r2d, 3 ; (1 * top[6] + 7 * top[7] + 4) >> 3
            (rsp.wrapping_add(16) as *mut u16).write_unaligned(r2 as u16); // mov [rsp+16], r2w
        }
        // .w4_3tap
        m[0] = zx(_mm_setzero_si128()); // pxor xm0, xm0
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
        tl = rsp as *const u8; // mov tlq, rsp
        m[1] = zx(_mm_srli_epi16::<3>(lo(m[1]))); // psrlw xm1, 3
        maxbase = maxbase.wrapping_add(1).wrapping_sub(((h as u32) < 8) as u32); // cmp hd, 8 ; sbb maxbased, -1
        m[0] = zx(_mm_avg_epu16(lo(m[0]), lo(m[1]))); // pavgw xm0, xm1
        stx_q3(rsp, lo(m[0])); // mova [tlq], xm0
    }
    // .w4_main
    m[3] = zx(_mm_cvtsi32_si128(dx as i32)); // movd xm3, dxd
    m[1] = _mm256_set1_epi64x((ro(z_base_inc, 0) as *const i64).read_unaligned()); // vpbroadcastq m1, [z_base_inc]
    m[6] = bcw_q3(pb_q3(tl, maxbase, 0)); // vpbroadcastw m6, [tlq+maxbaseq*2] ; top[max_base_x]
    maxbase <<= 6; // shl maxbased, 6
    m[3] = _mm256_broadcastw_epi16(lo(m[3])); // vpbroadcastw m3, xm3
    m[0] = zx(_mm_cvtsi32_si128(maxbase as i32)); // movd xm0, maxbased
    let mut r3 = dx; // mov r3d, dxd ; xpos
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    m[4] = _mm256_add_epi16(m[3], m[3]); // paddw m4, m3, m3
    m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0 ; -max_base_x
    m[3] = _mm256_blend_epi32::<0xcc>(m[3], m[4]); // vpblendd m3, m4, 0xcc
    m[0] = _mm256_add_epi16(m[4], m[3]); // paddw m0, m4, m3
    m[3] = _mm256_blend_epi32::<0xf0>(m[3], m[0]); // vpblendd m3, m0, 0xf0 ; xpos0 xpos1 xpos2 xpos3
    m[4] = _mm256_add_epi16(m[4], m[4]); // paddw m4, m4
    m[3] = _mm256_add_epi16(m[3], m[1]); // paddw m3, m1
                                         // Not in the assembly: the step of four rows, so that the next row's `xpos` below is one addition (see the
                                         // part notes; the compiler otherwise turned the chain of `lea`s into four counters and four additions).
    let dx4 = dx.wrapping_mul(4);
    loop {
        // .w4_loop
        let x0 = r3;
        let mut r5 = r3.wrapping_add(dx); // lea r5d, [r3+dxq]
        r3 >>= 6; // shr r3d, 6 ; base0
        m[1] = zx(ldx_q3(pb_q3(tl, r3, 0))); // movu xm1, [tlq+r3*2]
        r3 = r5.wrapping_add(dx); // lea r3d, [r5+dxq]
        r5 >>= 6; // shr r5d, 6 ; base1
        m[2] = zx(ldx_q3(pb_q3(tl, r5, 0))); // movu xm2, [tlq+r5*2]
        r5 = r3.wrapping_add(dx); // lea r5d, [r3+dxq]
        r3 >>= 6; // shr r3d, 6 ; base2
        m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_q3(pb_q3(tl, r3, 0))); // vinserti128 m1, [tlq+r3*2], 1
        r3 = x0.wrapping_add(dx4); // lea r3d, [r5+dxq] (the same value: xpos0 + 4*dx)
        r5 >>= 6; // shr r5d, 6 ; base3
        m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_q3(pb_q3(tl, r5, 0))); // vinserti128 m2, [tlq+r5*2], 1
        m[0] = _mm256_unpacklo_epi64(m[1], m[2]); // punpcklqdq m0, m1, m2
        m[1] = _mm256_bsrli_epi128::<2>(m[1]); // psrldq m1, 2
        m[2] = _mm256_bslli_epi128::<6>(m[2]); // pslldq m2, 6
        m[1] = _mm256_blend_epi32::<0xcc>(m[1], m[2]); // vpblendd m1, m2, 0xcc
        m[2] = _mm256_and_si256(m[5], m[3]); // pand m2, m5, m3
        m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[2] = _mm256_srai_epi16::<15>(m[3]); // psraw m2, m3, 15 ; xpos < max_base_x
        m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[0] = _mm256_blendv_epi8(m[6], m[0], m[2]); // vpblendvb m0, m6, m0, m2
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        _mm_storel_epi64(dst.cast(), lo(m[0])); // movq [dstq+strideq*0], xm0
        sth_q3(dst.wrapping_offset(stride), lo(m[0])); // movhps [dstq+strideq*1], xm0
        dst = dst.wrapping_offset(stride * 2); // lea dstq, [dstq+strideq*2]
        _mm_storel_epi64(dst.cast(), lo(m[1])); // movq [dstq+strideq*0], xm1
        sth_q3(dst.wrapping_offset(stride), lo(m[1])); // movhps [dstq+strideq*1], xm1
        h -= 4; // sub hd, 4
        if h == 0 {
            return; // jz .w4_end
        }
        dst = dst.wrapping_offset(stride * 2); // lea dstq, [dstq+strideq*2]
        if r3 >= maxbase {
            break; // cmp r3d, maxbased ; jb .w4_loop
        }
    }
    let r6 = stride * 3; // lea r6, [strideq*3]
    loop {
        // .w4_end_loop
        _mm_storel_epi64(dst.cast(), lo(m[6])); // movq [dstq+strideq*0], xm6
        _mm_storel_epi64(dst.wrapping_offset(stride).cast(), lo(m[6])); // movq [dstq+strideq*1], xm6
        _mm_storel_epi64(dst.wrapping_offset(stride * 2).cast(), lo(m[6])); // movq [dstq+strideq*2], xm6
        _mm_storel_epi64(dst.wrapping_offset(r6).cast(), lo(m[6])); // movq [dstq+r6], xm6
        dst = dst.wrapping_offset(stride * 4); // lea dstq, [dstq+strideq*4]
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w4_end_loop
        }
    }
    // .w4_end: RET
}

/// `.w8` up to its `RET`s.
#[inline(always)]
unsafe fn ipred_z1_16bpc_w8(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    tl: *const u8,
    h: i32,
    angle: i32,
    dx: u32,
    bdmax: i32,
) {
    let (mut dst, mut tl, mut h, mut angle, mut dx) = (dst, tl, h, angle, dx);
    let mut fr: FrameQ3<3>; // ALLOC_STACK -64, 7 (see the frame sizes above)
    let mut fr_us: FrameQ3<2>; // the same frame on the upsampling path: 64 bytes suffice there
    let mut r3 = (angle.wrapping_add(216) as u32 & !0xff) | (h as u32 & 0xff); // lea r3d, [angleq+216] ; mov r3b, hb
    if r3 <= 8 {
        // cmp r3d, 8 ; ja .w8_no_upsample ; !enable_intra_edge_filter || is_sm || d >= 40 || h > 8
        fr_us = [[0; 4]; 2];
        let rsp = fr_us.as_mut_ptr().cast::<u8>();
        m[2] = ld_q3(p_q3(tl, 2)); // movu m2, [tlq+2] ; 2 3 4 5 6 7 8 9   a b c d e f g _
        m[0] = ld_q3(p_q3(tl, 4)); // movu m0, [tlq+4] ; 3 4 5 6 7 8 9 a   b c d e f g _ _
        m[1] = ld_q3(tl); // movu m1, [tlq+0] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        if h == 4 {
            // cmp hd, 4 ; jne .w8_upsample_h8 ; awkward single-pixel edge case
            m[0] = _mm256_blend_epi32::<0x20>(m[0], m[2]); // vpblendd m0, m2, 0x20 ; 3 4 5 6 7 8 9 a   b c c _ _ _ _ _
        }
        // .w8_upsample_h8
        m[2] = _mm256_add_epi16(m[2], m[1]); // paddw m2, m1
        m[0] = _mm256_add_epi16(m[0], ld_q3(p_q3(tl, -2))); // paddw m0, [tlq-2] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
        dx = dx.wrapping_add(dx); // add dxd, dxd
        m[0] = _mm256_sub_epi16(m[2], m[0]); // psubw m0, m2, m0
        m[0] = _mm256_srai_epi16::<3>(m[0]); // psraw m0, 3
        m[4] = _mm256_setzero_si256(); // pxor m4, m4
        m[2] = _mm256_add_epi16(m[2], m[0]); // paddw m2, m0
        m[0] = _mm256_set1_epi16(bdmax as i16); // vpbroadcastw m0, r8m
        m[3] = zx(_mm_cvtsi32_si128(dx as i32)); // movd xm3, dxd
        m[2] = _mm256_max_epi16(m[2], m[4]); // pmaxsw m2, m4
        r3 = dx; // mov r3d, dxd
        m[2] = _mm256_avg_epu16(m[2], m[4]); // pavgw m2, m4
        m[3] = _mm256_broadcastw_epi16(lo(m[3])); // vpbroadcastw m3, xm3
        m[2] = _mm256_min_epi16(m[2], m[0]); // pminsw m2, m0
        m[0] = _mm256_unpacklo_epi16(m[1], m[2]); // punpcklwd m0, m1, m2
        m[1] = _mm256_unpackhi_epi16(m[1], m[2]); // punpckhwd m1, m2
        m[4] = _mm256_broadcastsi128_si256(ldx_q3(ro(z_upsample, 0))); // vbroadcasti128 m4, [z_upsample]
        stx_q3(rsp, lo(m[0])); // mova [rsp+ 0], xm0
        stx_q3(rsp.wrapping_add(16), lo(m[1])); // mova [rsp+16], xm1
        m[6] = _mm256_add_epi16(m[3], m[3]); // paddw m6, m3, m3
        stx_q3(rsp.wrapping_add(32), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [rsp+32], m0, 1
        stx_q3(rsp.wrapping_add(48), _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [rsp+48], m1, 1
        m[3] = _mm256_blend_epi32::<0xf0>(m[3], m[6]); // vpblendd m3, m6, 0xf0 ; xpos0 xpos1
        let rsp = rsp as *const u8;
        loop {
            // .w8_upsample_loop
            let mut r2 = r3.wrapping_add(dx); // lea r2d, [r3+dxq]
            r3 >>= 6; // shr r3d, 6 ; base0
            m[1] = zx(ldx_q3(pb_q3(rsp, r3, 0))); // movu xm1, [rsp+r3*2]
            m[2] = zx(ldx_q3(pb_q3(rsp, r3, 16))); // movu xm2, [rsp+r3*2+16]
            r3 = r2.wrapping_add(dx); // lea r3d, [r2+dxq]
            r2 >>= 6; // shr r2d, 6 ; base1
            m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_q3(pb_q3(rsp, r2, 0))); // vinserti128 m1, [rsp+r2*2], 1
            m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_q3(pb_q3(rsp, r2, 16))); // vinserti128 m2, [rsp+r2*2+16], 1
            m[1] = _mm256_shuffle_epi8(m[1], m[4]); // pshufb m1, m4
            m[2] = _mm256_shuffle_epi8(m[2], m[4]); // pshufb m2, m4
            m[0] = _mm256_unpacklo_epi64(m[1], m[2]); // punpcklqdq m0, m1, m2
            m[1] = _mm256_unpackhi_epi64(m[1], m[2]); // punpckhqdq m1, m2
            m[2] = _mm256_and_si256(m[5], m[3]); // pand m2, m5, m3
            m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
            m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
            m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
            m[3] = _mm256_add_epi16(m[3], m[6]); // paddw m3, m6
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            stx_q3(dst, lo(m[0])); // mova [dstq+strideq*0], xm0
            stx_q3(dst.wrapping_offset(stride), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
            dst = dst.wrapping_offset(stride * 2); // lea dstq, [dstq+strideq*2]
            h -= 2; // sub hd, 2
            if h <= 0 {
                return; // jg .w8_upsample_loop ; RET
            }
        }
    }
    // .w8_no_upsample
    let mut maxbase = (h + 7) as u32; // lea maxbased, [hq+7]
    'main: {
        if angle & 0x400 != 0 {
            // test angled, 0x400 ; jnz .w8_no_intra_edge_filter
            maxbase &= 7; // and maxbased, 7
            maxbase |= 8; // or maxbased, 8 ; imin(h+7, 15)
            break 'main; // jmp .w8_main
        }
        let mut r5 = ipred_z1_16bpc_filter_strength(m, maxbase, &mut angle); // call .filter_strength
        if r5 == 0 {
            break 'main; // test r5d, r5d ; jz .w8_main
        }
        fr = [[0; 4]; 3];
        let rsp = fr.as_mut_ptr().cast::<u8>();
        r5 = r5.count_ones(); // popcnt r5d, r5d
        m[1] = ipred_z1_16bpc_filter_k_q3(r5, 1); // vpbroadcastd m1, [base+z_filter_k-4+r5*4+12*1]
        m[4] = ipred_z1_16bpc_filter_k_q3(r5, 0); // vpbroadcastd m4, [base+z_filter_k-4+r5*4+12*0]
        m[0] = ld_q3(p_q3(tl, -2)); // mova m0, [tlq-2] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
        m[2] = ld_q3(tl); // movu m2, [tlq+0] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[1] = _mm256_mullo_epi16(m[1], m[2]); // pmullw m1, m2
        if h < 8 {
            // cmp hd, 8 ; jl .w8_filter_h4
            m[3] = _mm256_shufflelo_epi16::<0xf9>(m[2]); // pshuflw m3, m2, q3321
            m[3] = _mm256_inserti128_si256::<0>(m[3], ldx_q3(p_q3(tl, 2))); // vinserti128 m3, [tlq+2], 0 ; 2 3 4 5 6 7 8 9   a b c c _ _ _ _
        } else {
            m[2] = _mm256_unpackhi_epi16(m[2], m[2]); // punpckhwd m2, m2
            m[3] = _mm256_blend_epi32::<0x7f>(m[2], ld_q3(p_q3(tl, 2))); // vpblendd m3, m2, [tlq+2], 0x7f ; 2 3 4 5 6 7 8 9   a b c d e f g g
            if h != 8 {
                // je .w8_filter_end ; 8x4 and 8x8 are always 3-tap
                let mut r3 = ldw_q3(p_q3(tl, 30)); // movzx r3d, word [tlq+30]
                maxbase = 16; // mov maxbased, 16
                (rsp.wrapping_add(32) as *mut u32).write_unaligned(r3); // mov [rsp+32], r3d
                if r5 == 3 {
                    // cmp r5d, 3 ; jne .w8_filter_end
                    m[6] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[0]))); // punpcklwd xm6, xm0, xm0
                    m[2] = _mm256_blend_epi32::<0x7f>(m[2], ld_q3(p_q3(tl, 4))); // vpblendd m2, [tlq+4], 0x7f ; 3 4 5 6 7 8 9 a   b c d e f g g g
                    m[6] = _mm256_blend_epi32::<0xfe>(m[6], ld_q3(p_q3(tl, -4))); // vpblendd m6, [tlq-4], 0xfe ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
                    let mut r5 = ldw_q3(p_q3(tl, 28)); // movzx r5d, word [tlq+28]
                    (rsp.wrapping_add(34) as *mut u16).write_unaligned(r3 as u16); // mov [rsp+34], r3w
                    m[2] = _mm256_add_epi16(m[2], m[6]); // paddw m2, m6
                    r5 = r5.wrapping_sub(r3); // sub r5d, r3d
                    maxbase += 1; // inc maxbased
                    m[2] = _mm256_add_epi16(m[2], m[2]); // paddw m2, m2
                    r3 = r5.wrapping_add(r3.wrapping_mul(8)).wrapping_add(4); // lea r3d, [r5+r3*8+4]
                    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
                    r3 >>= 3; // shr r3d, 3
                    (rsp.wrapping_add(32) as *mut u16).write_unaligned(r3 as u16);
                    // mov [rsp+32], r3w
                } // jmp .w8_filter_end
            }
        }
        // .w8_filter_end
        m[0] = _mm256_add_epi16(m[0], m[3]); // paddw m0, m3
        m[0] = _mm256_mullo_epi16(m[0], m[4]); // pmullw m0, m4
        tl = rsp as *const u8; // mov tlq, rsp
        m[2] = _mm256_setzero_si256(); // pxor m2, m2
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[0] = _mm256_srli_epi16::<3>(m[0]); // psrlw m0, 3
        m[0] = _mm256_avg_epu16(m[0], m[2]); // pavgw m0, m2
        st_q3(rsp, m[0]); // mova [tlq], m0
    }
    // .w8_main
    m[3] = zx(_mm_cvtsi32_si128(dx as i32)); // movd xm3, dxd
    m[1] = _mm256_broadcastsi128_si256(ldx_q3(ro(z_base_inc, 0))); // vbroadcasti128 m1, [z_base_inc]
    m[6] = bcw_q3(pb_q3(tl, maxbase, 0)); // vpbroadcastw m6, [tlq+maxbaseq*2]
    maxbase <<= 6; // shl maxbased, 6
    m[3] = _mm256_broadcastw_epi16(lo(m[3])); // vpbroadcastw m3, xm3
    m[0] = zx(_mm_cvtsi32_si128(maxbase as i32)); // movd xm0, maxbased
    r3 = dx; // mov r3d, dxd
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    m[4] = _mm256_add_epi16(m[3], m[3]); // paddw m4, m3, m3
    m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
    m[3] = _mm256_blend_epi32::<0xf0>(m[3], m[4]); // vpblendd m3, m4, 0xf0 ; xpos0 xpos1
    m[3] = _mm256_add_epi16(m[3], m[1]); // paddw m3, m1
    loop {
        // .w8_loop
        let mut r5 = r3.wrapping_add(dx); // lea r5d, [r3+dxq]
        r3 >>= 6; // shr r3d, 6
        m[0] = zx(ldx_q3(pb_q3(tl, r3, 0))); // movu xm0, [tlq+r3*2]
        m[1] = zx(ldx_q3(pb_q3(tl, r3, 2))); // movu xm1, [tlq+r3*2+2]
        r3 = r5.wrapping_add(dx); // lea r3d, [r5+dxq]
        r5 >>= 6; // shr r5d, 6
        m[0] = _mm256_inserti128_si256::<1>(m[0], ldx_q3(pb_q3(tl, r5, 0))); // vinserti128 m0, [tlq+r5*2], 1
        m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_q3(pb_q3(tl, r5, 2))); // vinserti128 m1, [tlq+r5*2+2], 1
        m[2] = _mm256_and_si256(m[5], m[3]); // pand m2, m5, m3
        m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[2] = _mm256_srai_epi16::<15>(m[3]); // psraw m2, m3, 15
        m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[0] = _mm256_blendv_epi8(m[6], m[0], m[2]); // vpblendvb m0, m6, m0, m2
        stx_q3(dst, lo(m[0])); // mova [dstq+strideq*0], xm0
        stx_q3(dst.wrapping_offset(stride), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
        h -= 2; // sub hd, 2
        if h == 0 {
            return; // jz .w8_end
        }
        dst = dst.wrapping_offset(stride * 2); // lea dstq, [dstq+strideq*2]
        if r3 >= maxbase {
            break; // cmp r3d, maxbased ; jb .w8_loop
        }
    }
    loop {
        // .w8_end_loop
        stx_q3(dst, lo(m[6])); // mova [dstq+strideq*0], xm6
        stx_q3(dst.wrapping_offset(stride), lo(m[6])); // mova [dstq+strideq*1], xm6
        dst = dst.wrapping_offset(stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w8_end_loop
        }
    }
    // .w8_end: RET
}

/// `.w16` up to its `RET`s.
#[inline(always)]
unsafe fn ipred_z1_16bpc_w16(m: &mut M, dst: *mut u8, stride: isize, tl: *const u8, h: i32, angle: i32, dx: u32) {
    let (mut dst, mut tl, mut h, mut angle) = (dst, tl, h, angle);
    let mut fr: FrameQ3<5>; // ALLOC_STACK -96, 7 (see the frame sizes above)
    let mut maxbase = (h + 15) as u32; // lea maxbased, [hq+15]
    'main: {
        if angle & 0x400 != 0 {
            // test angled, 0x400 ; jnz .w16_no_intra_edge_filter
            maxbase &= 15; // and maxbased, 15
            maxbase |= 16; // or maxbased, 16 ; imin(h+15, 31)
            break 'main; // jmp .w16_main
        }
        let mut r5 = ipred_z1_16bpc_filter_strength(m, maxbase, &mut angle); // call .filter_strength
        if r5 == 0 {
            break 'main; // test r5d, r5d ; jz .w16_main
        }
        fr = [[0; 4]; 5];
        let rsp = fr.as_mut_ptr().cast::<u8>();
        r5 = r5.count_ones(); // popcnt r5d, r5d
        m[0] = ld_q3(p_q3(tl, -2)); // mova m0, [tlq-2] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
        m[1] = _mm256_add_epi16(m[0], ld_q3(p_q3(tl, 2))); // paddw m1, m0, [tlq+2] ; 2 3 4 5 6 7 8 9   a b c d e f g h
        'end2: {
            if r5 == 3 {
                // cmp r5d, 3 ; jne .w16_filter_3tap
                m[2] = bcd_q3(pw_3, 0); // vpbroadcastd m2, [base+pw_3]
                m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[0]))); // punpcklwd xm0, xm0
                m[0] = _mm256_blend_epi32::<0xfe>(m[0], ld_q3(p_q3(tl, -4))); // vpblendd m0, [tlq-4], 0xfe ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
                m[1] = _mm256_add_epi16(m[1], ld_q3(tl)); // paddw m1, [tlq+0] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
                m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
                m[0] = _mm256_avg_epu16(m[0], ld_q3(p_q3(tl, 4))); // pavgw m0, [tlq+4] ; 3 4 5 6 7 8 9 a   b c d e f g h i
                m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
                m[0] = _mm256_srli_epi16::<2>(m[0]); // psrlw m0, 2
                m[3] = ld_q3(p_q3(tl, 32)); // movu m3, [tlq+32] ; 2 3 4 5 6 7 8 9   a b c d e f g h
                m[2] = _mm256_add_epi16(m[2], ld_q3(p_q3(tl, 28))); // paddw m2, [tlq+28] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
                m[1] = _mm256_add_epi16(m[3], ld_q3(p_q3(tl, 30))); // paddw m1, m3, [tlq+30] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
                if h < 8 {
                    // cmp hd, 8 ; jl .w16_filter_5tap_h4
                    m[4] = zx(_mm_shufflelo_epi16::<0xfe>(lo(m[3]))); // pshuflw xm4, xm3, q3332 ; 4 5 5 5
                    m[3] = zx(_mm_shufflelo_epi16::<0xf9>(lo(m[3]))); // pshuflw xm3, xm3, q3321 ; 3 4 5 5
                    m[2] = zx(_mm_avg_epu16(lo(m[2]), lo(m[4]))); // pavgw xm2, xm4
                    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[3]))); // paddw xm1, xm3
                    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
                    m[1] = zx(_mm_srli_epi16::<2>(lo(m[1]))); // psrlw xm1, 2
                    break 'end2; // jmp .w16_filter_end2
                }
                m[3] = _mm256_unpackhi_epi16(m[3], m[3]); // punpckhwd m3, m3
                if h == 8 {
                    // je .w16_filter_5tap_h8
                    m[4] = zx(_mm_blend_epi32::<0x07>(lo(m[3]), ldx_q3(p_q3(tl, 36)))); // vpblendd xm4, xm3, [tlq+36], 0x07 ; 4 5 6 7 8 9 9 9
                    m[3] = zx(_mm_blend_epi32::<0x07>(lo(m[3]), ldx_q3(p_q3(tl, 34)))); // vpblendd xm3, [tlq+34], 0x07 ; 3 4 5 6 7 8 9 9
                    m[2] = zx(_mm_avg_epu16(lo(m[2]), lo(m[4]))); // pavgw xm2, xm4
                    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[3]))); // paddw xm1, xm3
                    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
                    m[1] = zx(_mm_srli_epi16::<2>(lo(m[1]))); // psrlw xm1, 2
                    break 'end2; // jmp .w16_filter_end2
                }
                m[4] = _mm256_blend_epi32::<0x7f>(m[3], ld_q3(p_q3(tl, 36))); // vpblendd m4, m3, [tlq+36], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h h h
                m[3] = _mm256_blend_epi32::<0x7f>(m[3], ld_q3(p_q3(tl, 34))); // vpblendd m3, [tlq+34], 0x7f ; 3 4 5 6 7 8 9 a   b c d e f g h h
                let r3 = ldw_q3(p_q3(tl, 62)); // movzx r3d, word [tlq+62]
                let mut r2 = ldw_q3(p_q3(tl, 60)); // movzx r2d, word [tlq+60]
                m[2] = _mm256_avg_epu16(m[2], m[4]); // pavgw m2, m4
                r2 = r2.wrapping_sub(r3); // sub r2d, r3d
                m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3
                r2 = r2.wrapping_add(r3.wrapping_mul(8)).wrapping_add(4); // lea r2d, [r2+r3*8+4]
                m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
                r2 >>= 3; // shr r2d, 3
                m[1] = _mm256_srli_epi16::<2>(m[1]); // psrlw m1, 2
                (rsp.wrapping_add(66) as *mut u16).write_unaligned(r3 as u16); // mov [rsp+66], r3w
                (rsp.wrapping_add(64) as *mut u16).write_unaligned(r2 as u16); // mov [rsp+64], r2w
                                                                               // mov tlq, rsp (again at .w16_filter_end2)
                if h > 16 {
                    maxbase = 33; // mov r3d, 33 ; cmp hd, 16 ; cmovg maxbased, r3d
                }
                break 'end2; // jmp .w16_filter_end2
            }
            // .w16_filter_3tap
            m[3] = ipred_z1_16bpc_filter_k_q3(r5, 1); // vpbroadcastd m3, [base+z_filter_k-4+r5*4+12*1]
            m[4] = ipred_z1_16bpc_filter_k_q3(r5, 0); // vpbroadcastd m4, [base+z_filter_k-4+r5*4+12*0]
            m[0] = _mm256_mullo_epi16(m[3], ld_q3(tl)); // pmullw m0, m3, [tlq+0] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
            m[2] = ld_q3(p_q3(tl, 32)); // movu m2, [tlq+32] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
            m[1] = _mm256_mullo_epi16(m[1], m[4]); // pmullw m1, m4
            m[3] = _mm256_mullo_epi16(m[3], m[2]); // pmullw m3, m2
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            if h == 8 {
                // cmp hd, 8 ; je .w16_filter_3tap_h8
                m[2] = zx(_mm_bsrli_si128::<2>(lo(m[2]))); // psrldq xm2, 2
                m[2] = zx(_mm_shufflehi_epi16::<0xa4>(lo(m[2]))); // pshufhw xm2, xm2, q2210 ; 2 3 4 5 6 7 8 8
            } else if h < 8 {
                // jl .w16_filter_3tap_h4
                m[2] = zx(_mm_shufflelo_epi16::<0xf9>(lo(m[2]))); // pshuflw xm2, xm2, q3321 ; 2 3 4 4 _ _ _ _
            } else {
                m[2] = _mm256_unpackhi_epi16(m[2], m[2]); // punpckhwd m2, m2
                m[2] = _mm256_blend_epi32::<0x7f>(m[2], ld_q3(p_q3(tl, 34))); // vpblendd m2, [tlq+34], 0x7f ; 2 3 4 5 6 7 8 9   a b c d e f g g
            }
            // .w16_filter_end
            m[2] = _mm256_add_epi16(m[2], ld_q3(p_q3(tl, 30))); // paddw m2, [tlq+30] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
            m[2] = _mm256_mullo_epi16(m[2], m[4]); // pmullw m2, m4
            m[0] = _mm256_srli_epi16::<3>(m[0]); // psrlw m0, 3
            m[1] = _mm256_setzero_si256(); // pxor m1, m1
            m[2] = _mm256_add_epi16(m[2], m[3]); // paddw m2, m3
            m[2] = _mm256_srli_epi16::<3>(m[2]); // psrlw m2, 3
            m[0] = _mm256_avg_epu16(m[0], m[1]); // pavgw m0, m1
            m[1] = _mm256_avg_epu16(m[1], m[2]); // pavgw m1, m2
        }
        // .w16_filter_end2
        tl = rsp as *const u8; // mov tlq, rsp
        st_q3(rsp, m[0]); // mova [tlq+ 0], m0
        st_q3(rsp.wrapping_add(32), m[1]); // mova [tlq+32], m1
    }
    // .w16_main
    m[4] = zx(_mm_cvtsi32_si128(dx as i32)); // movd xm4, dxd
    m[6] = bcw_q3(pb_q3(tl, maxbase, 0)); // vpbroadcastw m6, [tlq+maxbaseq*2]
    maxbase <<= 6; // shl maxbased, 6
    m[4] = _mm256_broadcastw_epi16(lo(m[4])); // vpbroadcastw m4, xm4
    m[0] = zx(_mm_cvtsi32_si128(maxbase as i32)); // movd xm0, maxbased
    let mut r3 = dx; // mov r3d, dxd
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    m[3] = _mm256_add_epi16(m[4], ld_q3(ro(z_base_inc, 0))); // paddw m3, m4, [z_base_inc]
    m[3] = _mm256_sub_epi16(m[3], m[0]); // psubw m3, m0
    loop {
        // .w16_loop
        let mut r5 = r3.wrapping_add(dx); // lea r5d, [r3+dxq]
        r3 >>= 6; // shr r3d, 6
        m[0] = ld_q3(pb_q3(tl, r3, 0)); // movu m0, [tlq+r3*2]
        m[1] = ld_q3(pb_q3(tl, r3, 2)); // movu m1, [tlq+r3*2+2]
        r3 = r5.wrapping_add(dx); // lea r3d, [r5+dxq]
        r5 >>= 6; // shr r5d, 6
        m[2] = _mm256_and_si256(m[5], m[3]); // pand m2, m5, m3
        m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[2] = _mm256_srai_epi16::<15>(m[3]); // psraw m2, m3, 15
        m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4
        m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
        m[0] = ld_q3(pb_q3(tl, r5, 0)); // movu m0, [tlq+r5*2]
        m[2] = _mm256_blendv_epi8(m[6], m[1], m[2]); // vpblendvb m2, m6, m1, m2
        m[1] = ld_q3(pb_q3(tl, r5, 2)); // movu m1, [tlq+r5*2+2]
        st_q3(dst, m[2]); // mova [dstq+strideq*0], m2
        m[2] = _mm256_and_si256(m[5], m[3]); // pand m2, m5, m3
        m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[2] = _mm256_srai_epi16::<15>(m[3]); // psraw m2, m3, 15
        m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[0] = _mm256_blendv_epi8(m[6], m[0], m[2]); // vpblendvb m0, m6, m0, m2
        st_q3(dst.wrapping_offset(stride), m[0]); // mova [dstq+strideq*1], m0
        h -= 2; // sub hd, 2
        if h == 0 {
            return; // jz .w16_end
        }
        dst = dst.wrapping_offset(stride * 2); // lea dstq, [dstq+strideq*2]
        if r3 >= maxbase {
            break; // cmp r3d, maxbased ; jb .w16_loop
        }
    }
    loop {
        // .w16_end_loop
        st_q3(dst, m[6]); // mova [dstq+strideq*0], m6
        st_q3(dst.wrapping_offset(stride), m[6]); // mova [dstq+strideq*1], m6
        dst = dst.wrapping_offset(stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w16_end_loop
        }
    }
    // .w16_end: RET
}

/// The edge filter of `.w32` and `.w64` from `vpbroadcastd m2, [pw_3]` to the end of `.wN_filter_loop`: the
/// 5-tap filter of 16 pixels per iteration into the frame. Returns the advanced `tl` and `r3` and the last
/// `r5` (the flags of the last `sub r5d, 16` decide `.w32_filter_h8`).
#[inline(always)]
unsafe fn ipred_z1_16bpc_filter_loop_q3(m: &mut M, tl: *const u8, rsp: *mut u8, r5: i32) -> (*const u8, *mut u8, i32) {
    let (mut tl, mut r5) = (tl, r5);
    m[2] = bcd_q3(pw_3, 0); // vpbroadcastd m2, [pw_3]
    m[0] = ld_q3(p_q3(tl, -2)); // mova m0, [tlq-2] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    m[1] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[0]))); // punpcklwd xm1, xm0, xm0
    m[1] = _mm256_blend_epi32::<0xfe>(m[1], ld_q3(p_q3(tl, -4))); // vpblendd m1, [tlq-4], 0xfe ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    m[0] = _mm256_add_epi16(m[0], ld_q3(tl)); // paddw m0, [tlq+0] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[0] = _mm256_add_epi16(m[0], ld_q3(p_q3(tl, 2))); // paddw m0, [tlq+2] ; 2 3 4 5 6 7 8 9   a b c d e f g h
    m[1] = _mm256_avg_epu16(m[1], ld_q3(p_q3(tl, 4))); // pavgw m1, [tlq+4] ; 3 4 5 6 7 8 9 a   b c d e f g h i
    let mut r3 = rsp; // mov r3, rsp
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
                                         // lea r5d, [maxbaseq-31] (w32) / lea r5d, [hq+32] (w64): the caller's `r5`
    m[0] = _mm256_srli_epi16::<2>(m[0]); // psrlw m0, 2
    st_q3(r3, m[0]); // mova [r3], m0
    loop {
        // .wN_filter_loop
        m[0] = ld_q3(p_q3(tl, 30)); // mova m0, [tlq+30]
        m[1] = _mm256_add_epi16(m[2], ld_q3(p_q3(tl, 28))); // paddw m1, m2, [tlq+28]
        tl = p_q3(tl, 32); // add tlq, 32
        m[0] = _mm256_add_epi16(m[0], ld_q3(tl)); // paddw m0, [tlq+0]
        m[1] = _mm256_avg_epu16(m[1], ld_q3(p_q3(tl, 4))); // pavgw m1, [tlq+4]
        m[0] = _mm256_add_epi16(m[0], ld_q3(p_q3(tl, 2))); // paddw m0, [tlq+2]
        r3 = r3.wrapping_add(32); // add r3, 32
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[0] = _mm256_srli_epi16::<2>(m[0]); // psrlw m0, 2
        st_q3(r3, m[0]); // mova [r3], m0
        r5 -= 16; // sub r5d, 16
        if r5 <= 0 {
            break; // jg .wN_filter_loop
        }
    }
    (tl, r3, r5)
}

/// `.w32` up to its `RET`s.
#[inline(always)]
unsafe fn ipred_z1_16bpc_w32(m: &mut M, dst: *mut u8, stride: isize, tl: *const u8, h: i32, angle: i32, dx: u32) {
    let (mut dst, mut tl, mut h) = (dst, tl, h);
    let mut fr: FrameQ3<7>; // ALLOC_STACK -160, 8 (see the frame sizes above)
    let mut maxbase = (h + 31) as u32; // lea maxbased, [hq+31]
    if h as u32 > 32 {
        maxbase = 63; // mov r3d, 63 ; cmp hd, 32 ; cmova maxbased, r3d
    }
    if angle & 0x400 == 0 {
        // test angled, 0x400 ; jnz .w32_main
        fr = [[0; 4]; 7];
        let rsp = fr.as_mut_ptr().cast::<u8>();
        let r5 = maxbase as i32 - 31; // lea r5d, [maxbaseq-31]
        let (t, r3, r5) = ipred_z1_16bpc_filter_loop_q3(m, tl, rsp, r5);
        tl = t;
        m[0] = ld_q3(p_q3(tl, 32)); // movu m0, [tlq+32] ; 2 3 4 5 6 7 8 9   a b c d e f g h
        m[1] = _mm256_unpackhi_epi16(m[0], m[0]); // punpckhwd m1, m0, m0
        m[2] = _mm256_add_epi16(m[2], ld_q3(p_q3(tl, 28))); // paddw m2, [tlq+28] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
        m[0] = _mm256_add_epi16(m[0], ld_q3(p_q3(tl, 30))); // paddw m0, [tlq+30] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        if r5 < 0 {
            // jl .w32_filter_h8
            m[3] = zx(_mm_blend_epi32::<0x07>(lo(m[1]), ldx_q3(p_q3(tl, 36)))); // vpblendd xm3, xm1, [tlq+36], 0x07 ; 4 5 6 7 8 9 9 9
            m[1] = zx(_mm_blend_epi32::<0x07>(lo(m[1]), ldx_q3(p_q3(tl, 34)))); // vpblendd xm1, [tlq+34], 0x07 ; 3 4 5 6 7 8 9 9
            m[2] = zx(_mm_avg_epu16(lo(m[2]), lo(m[3]))); // pavgw xm2, xm3
            m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
            tl = rsp as *const u8; // mov tlq, rsp
            m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); // paddw xm0, xm2
            m[0] = zx(_mm_srli_epi16::<2>(lo(m[0]))); // psrlw xm0, 2
            stx_q3(r3.wrapping_add(32), lo(m[0])); // mova [r3+32], xm0
        } else {
            m[3] = _mm256_blend_epi32::<0x7f>(m[1], ld_q3(p_q3(tl, 36))); // vpblendd m3, m1, [tlq+36], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h h h
            m[1] = _mm256_blend_epi32::<0x7f>(m[1], ld_q3(p_q3(tl, 34))); // vpblendd m1, [tlq+34], 0x7f ; 3 4 5 6 7 8 9 a   b c d e f g h h
            let r5 = ldw_q3(p_q3(tl, 62)); // movzx r5d, word [tlq+62]
            let mut r2 = ldw_q3(p_q3(tl, 60)); // movzx r2d, word [tlq+60]
            m[2] = _mm256_avg_epu16(m[2], m[3]); // pavgw m2, m3
            r2 = r2.wrapping_sub(r5); // sub r2d, r5d
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            r2 = r2.wrapping_add(r5.wrapping_mul(8)).wrapping_add(4); // lea r2d, [r2+r5*8+4]
            m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
            r2 >>= 3; // shr r2d, 3
            m[0] = _mm256_srli_epi16::<2>(m[0]); // psrlw m0, 2
            st_q3(r3.wrapping_add(32), m[0]); // mova [r3+32], m0
            (r3.wrapping_add(66) as *mut u16).write_unaligned(r5 as u16); // mov [r3+66], r5w
            (r3.wrapping_add(64) as *mut u16).write_unaligned(r2 as u16); // mov [r3+64], r2w
            tl = rsp as *const u8; // mov tlq, rsp
            if h == 64 {
                maxbase = 65; // mov r3d, 65 ; cmp hd, 64 ; cmove maxbased, r3d
            } // jmp .w32_main
        }
    }
    // .w32_main
    m[4] = zx(_mm_cvtsi32_si128(dx as i32)); // movd xm4, dxd
    m[6] = bcw_q3(pb_q3(tl, maxbase, 0)); // vpbroadcastw m6, [tlq+maxbaseq*2]
    maxbase <<= 6; // shl maxbased, 6
    m[4] = _mm256_broadcastw_epi16(lo(m[4])); // vpbroadcastw m4, xm4
    m[0] = zx(_mm_cvtsi32_si128(maxbase as i32)); // movd xm0, maxbased
    let mut r5 = dx; // mov r5d, dxd
    m[7] = bcd_q3(pw_m1024, 0); // vpbroadcastd m7, [pw_m1024] ; -16 * 64
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    m[3] = _mm256_add_epi16(m[4], ld_q3(ro(z_base_inc, 0))); // paddw m3, m4, [z_base_inc]
    m[3] = _mm256_sub_epi16(m[3], m[0]); // psubw m3, m0
    loop {
        // .w32_loop
        let r3 = r5 >> 6; // mov r3d, r5d ; shr r3d, 6
        m[0] = ld_q3(pb_q3(tl, r3, 0)); // movu m0, [tlq+r3*2]
        m[1] = ld_q3(pb_q3(tl, r3, 2)); // movu m1, [tlq+r3*2+2]
        m[2] = _mm256_and_si256(m[5], m[3]); // pand m2, m5, m3
        m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[1] = _mm256_srai_epi16::<15>(m[3]); // psraw m1, m3, 15
        m[0] = _mm256_blendv_epi8(m[6], m[0], m[1]); // vpblendvb m0, m6, m0, m1
        st_q3(dst, m[0]); // mova [dstq+32*0], m0
        m[0] = ld_q3(pb_q3(tl, r3, 32)); // movu m0, [tlq+r3*2+32]
        m[1] = ld_q3(pb_q3(tl, r3, 34)); // movu m1, [tlq+r3*2+34]
        r5 = r5.wrapping_add(dx); // add r5d, dxd
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[2] = _mm256_cmpgt_epi16(m[7], m[3]); // pcmpgtw m2, m7, m3
        m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[0] = _mm256_blendv_epi8(m[6], m[0], m[2]); // vpblendvb m0, m6, m0, m2
        st_q3(dst.wrapping_add(32), m[0]); // mova [dstq+32*1], m0
        h -= 1; // dec hd
        if h == 0 {
            return; // jz .w32_end
        }
        dst = dst.wrapping_offset(stride); // add dstq, strideq
        if r5 >= maxbase {
            break; // cmp r5d, maxbased ; jb .w32_loop
        }
    }
    loop {
        // .w32_end_loop
        st_q3(dst, m[6]); // mova [dstq+32*0], m6
        st_q3(dst.wrapping_add(32), m[6]); // mova [dstq+32*1], m6
        dst = dst.wrapping_offset(stride); // add dstq, strideq
        h -= 1; // dec hd
        if h <= 0 {
            break; // jg .w32_end_loop
        }
    }
    // .w32_end: RET
}

/// `.w64` up to its `RET`s.
#[inline(always)]
unsafe fn ipred_z1_16bpc_w64(m: &mut M, dst: *mut u8, stride: isize, tl: *const u8, h: i32, angle: i32, dx: u32) {
    let (mut dst, mut tl, mut h) = (dst, tl, h);
    let mut fr: FrameQ3<12>; // ALLOC_STACK -256, 10 (see the frame sizes above)
    let mut maxbase = (h + 63) as u32; // lea maxbased, [hq+63]
    if angle & 0x400 == 0 {
        // test angled, 0x400 ; jnz .w64_main
        fr = [[0; 4]; 12];
        let rsp = fr.as_mut_ptr().cast::<u8>();
        let (t, r3, _) = ipred_z1_16bpc_filter_loop_q3(m, tl, rsp, h + 32); // lea r5d, [hq+32]
        tl = t;
        m[0] = ld_q3(p_q3(tl, 32)); // movu m0, [tlq+32] ; 2 3 4 5 6 7 8 9   a b c d e f g h
        m[1] = _mm256_unpackhi_epi16(m[0], m[0]); // punpckhwd m1, m0, m0
        m[2] = _mm256_add_epi16(m[2], ld_q3(p_q3(tl, 28))); // paddw m2, [tlq+28] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
        m[0] = _mm256_add_epi16(m[0], ld_q3(p_q3(tl, 30))); // paddw m0, [tlq+30] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[3] = _mm256_blend_epi32::<0x7f>(m[1], ld_q3(p_q3(tl, 36))); // vpblendd m3, m1, [tlq+36], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h h h
        m[1] = _mm256_blend_epi32::<0x7f>(m[1], ld_q3(p_q3(tl, 34))); // vpblendd m1, [tlq+34], 0x7f ; 3 4 5 6 7 8 9 a   b c d e f g h h
        m[2] = _mm256_avg_epu16(m[2], m[3]); // pavgw m2, m3
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
        tl = rsp as *const u8; // mov tlq, rsp
        m[0] = _mm256_srli_epi16::<2>(m[0]); // psrlw m0, 2
        st_q3(r3.wrapping_add(32), m[0]); // mova [r3+32], m0
    }
    // .w64_main
    m[4] = zx(_mm_cvtsi32_si128(dx as i32)); // movd xm4, dxd
    m[6] = bcw_q3(pb_q3(tl, maxbase, 0)); // vpbroadcastw m6, [tlq+maxbaseq*2]
    maxbase <<= 6; // shl maxbased, 6
    m[4] = _mm256_broadcastw_epi16(lo(m[4])); // vpbroadcastw m4, xm4
    m[0] = zx(_mm_cvtsi32_si128(maxbase as i32)); // movd xm0, maxbased
    let mut r5 = dx; // mov r5d, dxd
    m[7] = bcd_q3(pw_m1024, 0); // vpbroadcastd m7, [pw_m1024] ; -16 * 64
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    m[3] = _mm256_add_epi16(m[4], ld_q3(ro(z_base_inc, 0))); // paddw m3, m4, [z_base_inc]
    m[8] = _mm256_add_epi16(m[7], m[7]); // paddw m8, m7, m7 ; -32 * 64
    m[3] = _mm256_sub_epi16(m[3], m[0]); // psubw m3, m0
    m[9] = _mm256_add_epi16(m[8], m[7]); // paddw m9, m8, m7 ; -48 * 64
    loop {
        // .w64_loop
        let r3 = r5 >> 6; // mov r3d, r5d ; shr r3d, 6
        m[0] = ld_q3(pb_q3(tl, r3, 0)); // movu m0, [tlq+r3*2]
        m[1] = ld_q3(pb_q3(tl, r3, 2)); // movu m1, [tlq+r3*2+2]
        m[2] = _mm256_and_si256(m[5], m[3]); // pand m2, m5, m3
        m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[1] = _mm256_srai_epi16::<15>(m[3]); // psraw m1, m3, 15
        m[0] = _mm256_blendv_epi8(m[6], m[0], m[1]); // vpblendvb m0, m6, m0, m1
        st_q3(dst, m[0]); // mova [dstq+32*0], m0
        m[0] = ld_q3(pb_q3(tl, r3, 32)); // movu m0, [tlq+r3*2+32]
        m[1] = ld_q3(pb_q3(tl, r3, 34)); // movu m1, [tlq+r3*2+34]
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[1] = _mm256_cmpgt_epi16(m[7], m[3]); // pcmpgtw m1, m7, m3
        m[0] = _mm256_blendv_epi8(m[6], m[0], m[1]); // vpblendvb m0, m6, m0, m1
        st_q3(dst.wrapping_add(32), m[0]); // mova [dstq+32*1], m0
        m[0] = ld_q3(pb_q3(tl, r3, 64)); // movu m0, [tlq+r3*2+64]
        m[1] = ld_q3(pb_q3(tl, r3, 66)); // movu m1, [tlq+r3*2+66]
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[1] = _mm256_cmpgt_epi16(m[8], m[3]); // pcmpgtw m1, m8, m3
        m[0] = _mm256_blendv_epi8(m[6], m[0], m[1]); // vpblendvb m0, m6, m0, m1
        st_q3(dst.wrapping_add(64), m[0]); // mova [dstq+32*2], m0
        m[0] = ld_q3(pb_q3(tl, r3, 96)); // movu m0, [tlq+r3*2+96]
        m[1] = ld_q3(pb_q3(tl, r3, 98)); // movu m1, [tlq+r3*2+98]
        r5 = r5.wrapping_add(dx); // add r5d, dxd
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[2] = _mm256_cmpgt_epi16(m[9], m[3]); // pcmpgtw m2, m9, m3
        m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[0] = _mm256_blendv_epi8(m[6], m[0], m[2]); // vpblendvb m0, m6, m0, m2
        st_q3(dst.wrapping_add(96), m[0]); // mova [dstq+32*3], m0
        h -= 1; // dec hd
        if h == 0 {
            return; // jz .w64_end
        }
        dst = dst.wrapping_offset(stride); // add dstq, strideq
        if r5 >= maxbase {
            break; // cmp r5d, maxbased ; jb .w64_loop
        }
    }
    loop {
        // .w64_end_loop
        st_q3(dst, m[6]); // mova [dstq+32*0], m6
        st_q3(dst.wrapping_add(32), m[6]); // mova [dstq+32*1], m6
        st_q3(dst.wrapping_add(64), m[6]); // mova [dstq+32*2], m6
        st_q3(dst.wrapping_add(96), m[6]); // mova [dstq+32*3], m6
        dst = dst.wrapping_offset(stride); // add dstq, strideq
        h -= 1; // dec hd
        if h <= 0 {
            break; // jg .w64_end_loop
        }
    }
    // .w64_end: RET
}

/// `ipred_z1_16bpc`.
///
/// # Safety
/// The contract of `angular_ipred_fn` (src/ipred.h) at 16 bpc: `dst` is a `width`x`height` block with `stride`
/// bytes between rows, `topleft` points into the edge array (`topleft[0..]` the row above with its top-left
/// pixel), `width` and `height` are powers of two from 4 to 64, `angle` is below 90 with the flags 0x200 and 0x400,
/// `bitdepth_max` is 0x3ff or 0xfff; the CPU supports AVX2. The kernel reads the edge array beyond the C kernel's
/// set (see the part notes), as the assembly does.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_z1_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
) {
    let mut m = regs();
    // lea r6, [ipred_z1_16bpc_avx2_table]
    let w = (width as u32).trailing_zeros(); // tzcnt wd, wm (width is a power of two, never zero)
    let mut angle = angle; // movifnidn angled, anglem
    let h = height; // movifnidn hd, hm
    let r7 = core::ptr::addr_of!(crate::common::tables::dav1d_dr_intra_derivative) as *const u8; // lea r7, [dr_intra_derivative]
                                                                                                 // movsxd wq, [r6+wq*4]
    let tl = p_q3(topleft as *const u8, 2); // add tlq, 2
                                            // add wq, r6
    let mut dx = angle as u32; // mov dxd, angled
    dx &= 0x7e; // and dxd, 0x7e
    angle = angle.wrapping_add(165); // add angled, 165 ; ~90
    let dx = (r7.wrapping_add(dx as usize) as *const u16).read_unaligned() as u32; // movzx dxd, word [r7+dxq]
    angle ^= 0x4ff; // xor angled, 0x4ff ; d = 90 - angle
    m[5] = bcd_q3(pw_62, 0); // vpbroadcastd m5, [pw_62]
    let dst = dst as *mut u8;
    match w {
        // jmp wq
        2 => ipred_z1_16bpc_w4(&mut m, dst, stride, tl, h, angle, dx, bitdepth_max),
        3 => ipred_z1_16bpc_w8(&mut m, dst, stride, tl, h, angle, dx, bitdepth_max),
        4 => ipred_z1_16bpc_w16(&mut m, dst, stride, tl, h, angle, dx),
        5 => ipred_z1_16bpc_w32(&mut m, dst, stride, tl, h, angle, dx),
        _ => ipred_z1_16bpc_w64(&mut m, dst, stride, tl, h, angle, dx),
    }
}

// ---- part 4: `ipred16_avx2.asm` lines 1842–2683 (z2) ----
//
// `cglobal ipred_z2_16bpc` is `dav1d_ipred_z2_16bpc_avx2`; its body is `ipred_z2_16bpc` (from the `cglobal` to
// `jmp wq`), and the jump table `ipred_z2_16bpc_avx2_table` is the `match` on `tzcnt(w)` there. The labels are
// functions named after them: `.w4`, `.w8`, `.w16`, `.w32`, `.w64` are `ipred_z2_16bpc_w4` … `_w64`; the labels
// reached by `jmp` from several places are their own functions (`_w4_filter_left`, `_w4_main`,
// `_w4_main_upsample_left`, `_w8_filter_left`, `_w8_filter_left_s12` (also `.w16_filter_left_s12`), `_w8_main`,
// `_w8_main_upsample_left` (with `.w8_loop0` … `.w8_end`), `_w32_filter_above`, `_filter_left_s3`,
// `_filter_left_s3b` (with `.filter_left_s3_end`), `_filter_left_s3_h8`); labels reached only by falling through
// or by one branch are `if`/`else` blocks and loops inside them. The two `call`ed subroutines are
// `ipred_z2_16bpc_filter_strength` (returns `r3d`) and `ipred_z2_16bpc_upsample_left`.
// The general-purpose registers that live across those functions are the fields of `GprQ4` (named as
// `DEFINE_ARGS` names them; `r6m`/`r7m`/`r8m` are `max_width`/`max_height`/`bitdepth_max`); the others are locals
// named after their register. The stack frame (`352` bytes) is `rsp` (see `ipred_z2_16bpc`); the spill slots
// `[rsp+284]`, `[rsp+288]` and `[rsp+320]` of `.w8_main` are locals.

/// The general-purpose registers of `ipred_z2_16bpc` that live across its labels.
struct GprQ4 {
    dst: *mut u8,      // dstq
    stride: isize,     // strideq
    tl: *mut u8,       // tlq (read only)
    h: u32,            // hd
    angle: u32,        // angled
    dx: u32,           // dxd (`neg dxd` clears the upper half of dxq)
    dy: u64,           // dyq
    r10: u32,          // r10d
    rsp: *mut u8,      // rsp: the frame
    max_width: c_int,  // r6m
    max_height: c_int, // r7m
    pixel_max: c_int,  // r8m (bitdepth_max)
}

/// `qABCD` shuffle immediates.
const fn qq4(a: i32, b: i32, c: i32, d: i32) -> i32 {
    a << 6 | b << 4 | c << 2 | d
}

// Memory operands: `p + off` bytes (caller memory and the frame; unaligned forms, same bytes).
#[inline(always)]
unsafe fn ld_q4(p: *mut u8, off: isize) -> __m256i {
    _mm256_loadu_si256(at(p, off) as *const __m256i)
}
#[inline(always)]
unsafe fn ldx_q4(p: *mut u8, off: isize) -> __m128i {
    _mm_loadu_si128(at(p, off) as *const __m128i)
}
#[inline(always)]
unsafe fn ldq_q4(p: *mut u8, off: isize) -> __m128i {
    _mm_loadl_epi64(at(p, off) as *const __m128i)
}
#[inline(always)]
unsafe fn ldd_q4(p: *mut u8, off: isize) -> __m128i {
    _mm_cvtsi32_si128((at(p, off) as *const i32).read_unaligned())
}
#[inline(always)]
unsafe fn st_q4(p: *mut u8, off: isize, x: __m256i) {
    _mm256_storeu_si256(at(p, off) as *mut __m256i, x)
}
#[inline(always)]
unsafe fn stx_q4(p: *mut u8, off: isize, x: __m128i) {
    _mm_storeu_si128(at(p, off) as *mut __m128i, x)
}
#[inline(always)]
unsafe fn stq_q4(p: *mut u8, off: isize, x: __m128i) {
    _mm_storel_epi64(at(p, off) as *mut __m128i, x)
}
/// `movhps [p+off], x` (written as `pextrq [p+off], x, 1`, the same 8 bytes in one instruction: the compiler
/// turned `_mm_storeh_pd` on an integer vector into `pshufd` and `movq`).
#[inline(always)]
unsafe fn sth_q4(p: *mut u8, off: isize, x: __m128i) {
    (at(p, off) as *mut i64).write_unaligned(_mm_extract_epi64::<1>(x))
}
#[inline(always)]
unsafe fn std_q4(p: *mut u8, off: isize, x: __m128i) {
    (at(p, off) as *mut i32).write_unaligned(_mm_cvtsi128_si32(x))
}
// Read-only operands `[base+label+off]` (inside `RODATA`; non-constant offsets are bounded at the use).
#[inline(always)]
unsafe fn rd_q4(label: usize, off: isize) -> __m256i {
    _mm256_loadu_si256(ro(label, off) as *const __m256i)
}
#[inline(always)]
unsafe fn rdx_q4(label: usize, off: isize) -> __m128i {
    _mm_loadu_si128(ro(label, off) as *const __m128i)
}
/// `vpbroadcastd ymm, [label+off]`.
#[inline(always)]
unsafe fn rbd_q4(label: usize, off: isize) -> __m256i {
    _mm256_set1_epi32((ro(label, off) as *const i32).read_unaligned())
}
/// `vpbroadcastd xmm, [label+off]`.
#[inline(always)]
unsafe fn rbdx_q4(label: usize, off: isize) -> __m128i {
    _mm_set1_epi32((ro(label, off) as *const i32).read_unaligned())
}
/// `vpbroadcastq ymm, [label+off]`.
#[inline(always)]
unsafe fn rbq_q4(label: usize, off: isize) -> __m256i {
    _mm256_set1_epi64x((ro(label, off) as *const i64).read_unaligned())
}
/// `vbroadcasti128 ymm, [label+off]`.
#[inline(always)]
unsafe fn rbi_q4(label: usize, off: isize) -> __m256i {
    _mm256_broadcastsi128_si256(_mm_loadu_si128(ro(label, off) as *const __m128i))
}
/// `vpgatherdq dst, [base+idx*2], mask`. The instruction clears its mask register; the intrinsic leaves it, so
/// the `mova`s around each gather that save and restore the mask are register bookkeeping only.
#[inline(always)]
unsafe fn gather_q4(src: __m256i, base: *mut u8, idx: __m128i, mask: __m256i) -> __m256i {
    _mm256_mask_i32gather_epi64::<2>(src, base as *const i64, idx, mask)
}

/// `ipred_z2_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_z2_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    ipred_z2_16bpc(
        dst,
        stride,
        topleft,
        width,
        height,
        angle,
        max_width,
        max_height,
        bitdepth_max,
    )
}

/// `cglobal ipred_z2_16bpc, 3, 12, 12, 352` to `jmp wq`.
#[inline(always)]
unsafe fn ipred_z2_16bpc(
    dst: *mut u16,
    stride: isize,
    tl: *const u16,
    w: c_int,
    h: c_int,
    angle: c_int,
    max_width: c_int,
    max_height: c_int,
    bitdepth_max: c_int,
) {
    let mut m = regs();
    // The frame (`352` bytes at `rsp`, 32-byte slots, with one slot below `rsp`): `.w8_leftonly_loop`'s gathers
    // read up to 2 bytes below `rsp` (h = 64) and the gathers reach `rsp+344`, in lanes that the blends discard
    // or pixels the shuffles drop (the assembly reads whatever its stack holds there); bytes the assembly never
    // writes (above `rsp+160` for w4/w8) are read by such lanes too. Slots 1..=5 (`rsp+0` … `rsp+160`) are
    // written whole by the five `mova [rsp+…]` below before any load; every other slot is zeroed here.
    let mut frame = core::mem::MaybeUninit::<[__m256i; 12]>::uninit();
    let slots = frame.as_mut_ptr() as *mut __m256i;
    slots.write(_mm256_setzero_si256());
    for i in 6..12 {
        slots.add(i).write(_mm256_setzero_si256());
    }
    let rsp = at(slots as *mut u8, 32);
    let tl = tl as *mut u8;
    // `lea r9, [ipred_z2_16bpc_avx2_table]`; `base` is `RODATA` (`ro`).
    let wt = (w as u32).trailing_zeros(); // tzcnt wd, wm (w is a power of two >= 4, never zero)
    let mut angle = angle as u32; // movifnidn angled, anglem
    let h = h as u32; // movifnidn hd, hm
    let der = core::ptr::addr_of!(crate::common::tables::dav1d_dr_intra_derivative) as *const u8; // lea dxq, [dr_intra_derivative-90]
    m[1] = ld_q4(tl, 0); // mova m1, [tlq- 0]
    let mut dyd = angle & 0xff; // movzx dyd, angleb
    angle ^= 0x400; // xor angled, 0x400
    m[2] = ld_q4(tl, -32); // mova m2, [tlq- 32]
                           // mov r8, dxq; sub dxq, dyq (below: the byte offset `180 - angle` into the table)
    m[3] = ld_q4(tl, -64); // mova m3, [tlq- 64]
    m[4] = ld_q4(tl, -96); // mova m4, [tlq- 96]
    dyd &= !1; // and dyd, ~1
    m[5] = ld_q4(tl, -128); // mova m5, [tlq-128]
                            // and dxq, ~1; angle is 91..179 (low byte), so both offsets lie inside the 44 entries.
    let dy = (der.add(dyd as usize - 90) as *const u16).read_unaligned() as u64; // movzx dyd, word [r8+dyq] ; angle - 90
    let dx = (der.add((180 - (angle & 0xff)) as usize & !1) as *const u16).read_unaligned() as u32; // movzx dxd, word [dxq+270] ; 180 - angle
    m[11] = rbd_q4(pw_62, 0); // vpbroadcastd m11, [base+pw_62]
    _mm256_store_si256(at(rsp, 128) as *mut __m256i, m[1]); // mova [rsp+128], m1
    _mm256_store_si256(at(rsp, 96) as *mut __m256i, m[2]); // mova [rsp+ 96], m2
    _mm256_store_si256(at(rsp, 64) as *mut __m256i, m[3]); // mova [rsp+ 64], m3
    let dx = dx.wrapping_neg(); // neg dxd
    _mm256_store_si256(at(rsp, 32) as *mut __m256i, m[4]); // mova [rsp+ 32], m4
    let dy = dy.wrapping_neg(); // neg dyq
    _mm256_store_si256(at(rsp, 0) as *mut __m256i, m[5]); // mova [rsp+  0], m5
    let mut g = GprQ4 {
        dst: dst as *mut u8,
        stride,
        tl,
        h,
        angle,
        dx,
        dy,
        r10: 0,
        rsp,
        max_width,
        max_height,
        pixel_max: bitdepth_max,
    };
    match wt {
        // jmp wq
        2 => ipred_z2_16bpc_w4(&mut m, &mut g),
        3 => ipred_z2_16bpc_w8(&mut m, &mut g),
        4 => ipred_z2_16bpc_w16(&mut m, &mut g),
        5 => ipred_z2_16bpc_w32(&mut m, &mut g),
        _ => ipred_z2_16bpc_w64(&mut m, &mut g),
    }
}

/// `.filter_strength` (`call`ed): `r3d` in, the strength mask out (`pmovmskb r3d, m0`); sets `m7`, `m8`, `m9`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_filter_strength(m: &mut M, r3d: u32, angle: u32) -> u32 {
    m[8] = zx(_mm_cvtsi32_si128(r3d as i32)); // movd xm8, r3d
    let mut r3d = angle; // mov r3d, angled
    m[7] = zx(_mm_cvtsi32_si128(angle as i32)); // movd xm7, angled
    m[8] = _mm256_broadcastb_epi8(lo(m[8])); // vpbroadcastb m8, xm8
    r3d >>= 8; // shr r3d, 8 ; is_sm << 1
    m[7] = _mm256_broadcastb_epi8(lo(m[7])); // vpbroadcastb m7, xm7
    m[8] = _mm256_cmpeq_epi8(m[8], rd_q4(z_filter_wh, 0)); // pcmpeqb m8, [base+z_filter_wh]
                                                           // r3d is 0 or 2 here (angle < 0x400 after the callers' arithmetic): z_filter_t0 or z_filter_t1.
    m[9] = zx(rdx_q4(z_filter_t0, (r3d * 8) as isize)); // mova xm9, [r9+r3*8]
    m[0] = _mm256_and_si256(m[8], m[7]); // pand m0, m8, m7
    m[0] = _mm256_cmpgt_epi8(m[0], m[9]); // pcmpgtb m0, m9
    _mm256_movemask_epi8(m[0]) as u32 // pmovmskb r3d, m0
}

/// `.upsample_left` (`call`ed; h4/h8): writes `[rsp+96]` … `[rsp+128]` (`[rsp+96+gprsize]` in the subroutine).
#[inline(always)]
unsafe fn ipred_z2_16bpc_upsample_left(m: &mut M, g: &mut GprQ4) {
    let tl = g.tl;
    m[0] = zx(ldx_q4(tl, -16)); // mova xm0, [tlq-16] ; 8 7 6 5 4 3 2 1
    m[1] = zx(ldx_q4(tl, -14)); // movu xm1, [tlq-14] ; 7 6 5 4 3 2 1 0
    m[4] = zx(_mm_set1_epi16(g.pixel_max as i16)); // vpbroadcastw xm4, r8m ; pixel_max
    if g.h == 8 {
        // cmp hd, 8; je .upsample_left_h8
        m[2] = zx(_mm_blend_epi16::<0xfe>(lo(m[0]), ldx_q4(tl, -18))); // pblendw xm2, xm0, [tlq-18], 0xfe
        m[3] = zx(_mm_blend_epi16::<0x7f>(lo(m[1]), ldx_q4(tl, -12))); // pblendw xm3, xm1, [tlq-12], 0x7f
    } else {
        m[2] = zx(_mm_shufflehi_epi16::<{ qq4(2, 1, 0, 0) }>(lo(m[0]))); // pshufhw xm2, xm0, q2100
        m[3] = zx(_mm_shufflehi_epi16::<{ qq4(3, 3, 2, 1) }>(lo(m[1]))); // pshufhw xm3, xm1, q3321
    }
    // .upsample_left_end:
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[0]))); // paddw xm1, xm0
    m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[3]))); // paddw xm2, xm3
    m[2] = zx(_mm_sub_epi16(lo(m[1]), lo(m[2]))); // psubw xm2, xm1, xm2
    g.dy = g.dy.wrapping_add(g.dy); // add dyq, dyq
    m[2] = zx(_mm_srai_epi16::<3>(lo(m[2]))); // psraw xm2, 3
    m[3] = _mm256_setzero_si256(); // pxor xm3, xm3
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
    m[1] = zx(_mm_max_epi16(lo(m[1]), lo(m[3]))); // pmaxsw xm1, xm3
    m[1] = zx(_mm_avg_epu16(lo(m[1]), lo(m[3]))); // pavgw xm1, xm3
    m[1] = zx(_mm_min_epi16(lo(m[1]), lo(m[4]))); // pminsw xm1, xm4
    m[2] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[1]))); // punpcklwd xm2, xm0, xm1
    m[0] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[1]))); // punpckhwd xm0, xm1
    _mm_store_si128(at(g.rsp, 96) as *mut __m128i, lo(m[2])); // mova [rsp+ 96+gprsize], xm2
    _mm_store_si128(at(g.rsp, 112) as *mut __m128i, lo(m[0])); // mova [rsp+112+gprsize], xm0
}

/// `.w4`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_w4(m: &mut M, g: &mut GprQ4) {
    let tl = g.tl;
    m[10] = rbi_q4(z2_x_shuf, 0); // vbroadcasti128 m10, [base+z2_x_shuf]
    m[6] = rbq_q4(z_base_inc, 2); // vpbroadcastq m6, [base+z_base_inc+2]
    let mut r8d = g.dx.wrapping_add(65 << 6); // lea r8d, [dxq+(65<<6)] ; xpos
    let mut r10d: u32 = (63 - 4) << 6; // mov r10d, (63-4)<<6
    if g.angle & 0x400 != 0 {
        // test angled, 0x400; jnz .w4_main ; !enable_intra_edge_filter
        return ipred_z2_16bpc_w4_main(m, g, r8d, r10d);
    }
    let mut r3d = g.h + 2; // lea r3d, [hq+2]
    g.angle = g.angle.wrapping_add(1022); // add angled, 1022
    r3d <<= 6; // shl r3d, 6
    if r3d & g.angle == 0 {
        // test r3d, angled; jnz .w4_no_upsample_above ; angle >= 130 || h > 8 || (is_sm && h == 8)
        m[0] = zx(ldq_q4(tl, 2)); // movq xm0, [tlq+2] ; 1 2 3 4
        m[1] = zx(ldq_q4(tl, 0)); // movq xm1, [tlq+0] ; 0 1 2 3
        m[2] = zx(_mm_shufflelo_epi16::<{ qq4(3, 3, 2, 1) }>(lo(m[0]))); // pshuflw xm2, xm0, q3321 ; 2 3 4 4
        m[3] = zx(_mm_shufflelo_epi16::<{ qq4(2, 1, 0, 0) }>(lo(m[1]))); // pshuflw xm3, xm1, q2100 ; 0 0 1 2
        m[4] = zx(_mm_set1_epi16(g.pixel_max as i16)); // vpbroadcastw xm4, r8m ; pixel_max
        m[10] = rbi_q4(z_upsample, 0); // vbroadcasti128 m10, [base+z_upsample]
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[0]))); // paddw xm1, xm0
        m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[3]))); // paddw xm2, xm3
        r8d = r8d.wrapping_add(g.dx).wrapping_add(1 << 6); // lea r8d, [r8+dxq+(1<<6)]
        m[2] = zx(_mm_sub_epi16(lo(m[1]), lo(m[2]))); // psubw xm2, xm1, xm2
        g.dx = g.dx.wrapping_add(g.dx); // add dxd, dxd
        m[2] = zx(_mm_srai_epi16::<3>(lo(m[2]))); // psraw xm2, 3
        m[3] = _mm256_setzero_si256(); // pxor xm3, xm3
        r10d = r10d.wrapping_sub(3 << 6); // sub r10d, 3<<6
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
        m[6] = _mm256_add_epi16(m[6], m[6]); // paddw m6, m6
        m[1] = zx(_mm_max_epi16(lo(m[1]), lo(m[3]))); // pmaxsw xm1, xm3
        g.angle = g.angle.wrapping_sub(1075); // sub angled, 1075 ; angle - 53
        m[1] = zx(_mm_avg_epu16(lo(m[1]), lo(m[3]))); // pavgw xm1, xm3
        let r3 = g.h + 3; // lea r3d, [hq+3]
        m[1] = zx(_mm_min_epi16(lo(m[1]), lo(m[4]))); // pminsw xm1, xm4
        g.angle ^= 0x7f; // xor angled, 0x7f ; 180 - angle
        m[1] = zx(_mm_unpacklo_epi16(lo(m[1]), lo(m[0]))); // punpcklwd xm1, xm0
        stx_q4(g.rsp, 130, lo(m[1])); // movu [rsp+130], xm1
        let r3d = ipred_z2_16bpc_filter_strength(m, r3, g.angle); // call .filter_strength
        return ipred_z2_16bpc_w4_filter_left(m, g, r3d, r8d, r10d); // jmp .w4_filter_left
    }
    // .w4_no_upsample_above:
    let r3 = g.h + 3; // lea r3d, [hq+3]
    g.angle = g.angle.wrapping_sub(1112); // sub angled, 1112 ; angle - 90
    let mut r3 = ipred_z2_16bpc_filter_strength(m, r3, g.angle); // call .filter_strength
    if r3 != 0 {
        // test r3d, r3d; jz .w4_no_filter_above
        r3 = r3.count_ones(); // popcnt r3d, r3d
                              // r3 is the filter strength, 1..3: the z_filter_k loads stay inside its 36 bytes.
        let k = (r3 * 4) as isize - 4;
        m[4] = zx(rbdx_q4(z_filter_k, k + 12)); // vpbroadcastd xm4, [base+z_filter_k-4+r3*4+12*1]
        m[5] = zx(rbdx_q4(z_filter_k, k)); // vpbroadcastd xm5, [base+z_filter_k-4+r3*4+12*0]
        m[0] = zx(_mm_srli_si128::<2>(lo(m[1]))); // psrldq xm0, xm1, 2 ; 1 2 3 4
        m[2] = zx(_mm_shufflelo_epi16::<{ qq4(2, 1, 0, 0) }>(lo(m[1]))); // pshuflw xm2, xm1, q2100 ; 0 0 1 2
        m[4] = zx(_mm_mullo_epi16(lo(m[4]), lo(m[0]))); // pmullw xm4, xm0
        m[3] = zx(_mm_shufflelo_epi16::<{ qq4(3, 3, 2, 1) }>(lo(m[0]))); // pshuflw xm3, xm0, q3321 ; 2 3 4 4
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[3]))); // paddw xm1, xm3
        m[3] = zx(_mm_shufflelo_epi16::<{ qq4(3, 3, 3, 2) }>(lo(m[0]))); // pshuflw xm3, xm0, q3332 ; 3 4 4 4
        m[1] = zx(_mm_mullo_epi16(lo(m[1]), lo(m[5]))); // pmullw xm1, xm5
        m[5] = zx(rbdx_q4(z_filter_k, k + 24)); // vpbroadcastd xm5, [base+z_filter_k-4+r3*4+12*2]
        m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[3]))); // paddw xm2, xm3
        m[3] = zx(_mm_set1_epi32(g.max_width)); // vpbroadcastd xm3, r6m ; max_width
        m[2] = zx(_mm_mullo_epi16(lo(m[2]), lo(m[5]))); // pmullw xm2, xm5
        m[3] = zx(_mm_packs_epi32(lo(m[3]), lo(m[3]))); // packssdw xm3, xm3
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[4]))); // paddw xm1, xm4
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
        m[3] = zx(_mm_sub_epi16(lo(m[3]), rdx_q4(pw_1to16, 0))); // psubw xm3, [base+pw_1to16]
        m[4] = _mm256_setzero_si256(); // pxor xm4, xm4
        m[1] = zx(_mm_srli_epi16::<3>(lo(m[1]))); // psrlw xm1, 3
        m[3] = zx(_mm_min_epi16(lo(m[3]), lo(m[11]))); // pminsw xm3, xm11 ; clip to byte range
        m[1] = zx(_mm_avg_epu16(lo(m[1]), lo(m[4]))); // pavgw xm1, xm4
        m[1] = zx(_mm_blendv_epi8(lo(m[1]), lo(m[0]), lo(m[3]))); // vpblendvb xm1, xm0, xm3
        stq_q4(g.rsp, 130, lo(m[1])); // movq [rsp+130], xm1
    }
    // .w4_no_filter_above:
    let mut r3d = g.h + 2; // lea r3d, [hq+2]
    g.angle = g.angle.wrapping_add(973); // add angled, 973 ; angle + 883
    r3d <<= 6; // shl r3d, 6
    if r3d & g.angle == 0 {
        // test r3d, angled; jz .w4_upsample_left ; angle <= 140 || h > 8 || (is_sm && h == 8)
        // .w4_upsample_left:
        ipred_z2_16bpc_upsample_left(m, g); // call .upsample_left
        let r11: isize = -16; // mov r11, -16
        m[9] = rbi_q4(z_upsample, 0); // vbroadcasti128 m9, [base+z_upsample]
        return ipred_z2_16bpc_w4_main_upsample_left(m, g, r8d, r10d, r11); // jmp .w4_main_upsample_left
    }
    m[0] = zx(rbdx_q4(pb_90, 0)); // vpbroadcastd xm0, [base+pb_90]
    m[0] = zx(_mm_sub_epi8(lo(m[0]), lo(m[7]))); // psubb xm0, xm7 ; 180 - angle
    m[0] = zx(_mm_and_si128(lo(m[0]), lo(m[8]))); // pand xm0, xm8 ; reuse from previous filter_strength call
    m[0] = zx(_mm_cmpgt_epi8(lo(m[0]), lo(m[9]))); // pcmpgtb xm0, xm9
    let r3d = _mm_movemask_epi8(lo(m[0])) as u32; // pmovmskb r3d, xm0
    ipred_z2_16bpc_w4_filter_left(m, g, r3d, r8d, r10d)
}

/// `.w4_filter_left` to `.w4_filter_left_end2`'s store, then `.w4_main`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_w4_filter_left(m: &mut M, g: &mut GprQ4, r3d: u32, r8d: u32, r10d: u32) {
    let tl = g.tl;
    if r3d == 0 {
        // test r3d, r3d; jz .w4_main
        return ipred_z2_16bpc_w4_main(m, g, r8d, r10d);
    }
    let r3 = r3d.count_ones(); // popcnt r3d, r3d
    m[0] = ld_q4(tl, -32); // mova m0, [tlq-32] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    m[5] = _mm256_set1_epi32(g.max_height); // vpbroadcastd m5, r7m ; max_height
    if r3 == 3 {
        // cmp r3d, 3; je .w4_filter_left_s3
        // .w4_filter_left_s3: ; can only be h16
        m[2] = ld_q4(tl, -30); // movu m2, [tlq-30] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[4] = rbd_q4(pw_3, 0); // vpbroadcastd m4, [base+pw_3]
        m[1] = _mm256_add_epi16(m[0], m[2]); // paddw m1, m0, m2
        m[2] = _mm256_unpackhi_epi16(m[2], m[2]); // punpckhwd m2, m2
        m[2] = _mm256_blend_epi32::<0x7f>(m[2], ld_q4(tl, -28)); // vpblendd m2, [tlq-28], 0x7f ; 2 3 4 5 6 7 8 9   a b c d e f g g
        m[3] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[0]))); // punpcklwd xm3, xm0, xm0
        m[2] = _mm256_add_epi16(m[2], m[4]); // paddw m2, m4
        m[4] = _mm256_blend_epi32::<0xfe>(m[3], ld_q4(tl, -34)); // vpblendd m4, m3, [tlq-34], 0xfe ; 0 0 1 2 3 4 5 6   8 8 9 a b c d e
        m[3] = _mm256_blend_epi32::<0xfe>(m[3], ld_q4(tl, -36)); // vpblendd m3, [tlq-36], 0xfe ; 0 0 0 1 2 3 4 5   6 8 8 9 a b c d
        m[1] = _mm256_add_epi16(m[1], m[4]); // paddw m1, m4
        m[2] = _mm256_avg_epu16(m[2], m[3]); // pavgw m2, m3
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        m[1] = _mm256_srli_epi16::<2>(m[1]); // psrlw m1, 2
                                             // jmp .w4_filter_left_end2
    } else {
        // r3 is the filter strength, 1 or 2: the z_filter_k loads stay inside its 36 bytes.
        let k = (r3 * 4) as isize - 4;
        m[2] = rbd_q4(z_filter_k, k + 12); // vpbroadcastd m2, [base+z_filter_k-4+r3*4+12*1]
        m[3] = rbd_q4(z_filter_k, k); // vpbroadcastd m3, [base+z_filter_k-4+r3*4+12*0]
        m[2] = _mm256_mullo_epi16(m[2], m[0]); // pmullw m2, m0
        if g.h < 8 {
            // cmp hd, 8; jl .w4_filter_left_h4
            m[1] = _mm256_shufflehi_epi16::<{ qq4(2, 1, 0, 0) }>(m[0]); // pshufhw m1, m0, q2100 ; _ _ _ _ _ _ _ _   _ _ _ _ c c d e
        } else {
            m[4] = ld_q4(tl, -34); // movu m4, [tlq-34]
            m[1] = _mm256_unpacklo_epi16(m[0], m[0]); // punpcklwd m1, m0, m0
            m[1] = _mm256_blend_epi32::<0xee>(m[1], m[4]); // vpblendd m1, m4, 0xee ; 0 0 1 2 3 4 5 6   8 8 9 a b c d e
            if g.h != 8 {
                // je .w4_filter_left_end
                m[1] = _mm256_blend_epi32::<0x10>(m[1], m[4]); // vpblendd m1, m4, 0x10 ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
            }
        }
        // .w4_filter_left_end:
        m[1] = _mm256_add_epi16(m[1], ld_q4(tl, -30)); // paddw m1, [tlq-30] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[1] = _mm256_mullo_epi16(m[1], m[3]); // pmullw m1, m3
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        m[2] = _mm256_setzero_si256(); // pxor m2, m2
        m[1] = _mm256_srli_epi16::<3>(m[1]); // psrlw m1, 3
        m[1] = _mm256_avg_epu16(m[1], m[2]); // pavgw m1, m2
    }
    // .w4_filter_left_end2:
    m[5] = _mm256_packs_epi32(m[5], m[5]); // packssdw m5, m5
    m[5] = _mm256_sub_epi16(m[5], rd_q4(pw_16to1, 0)); // psubw m5, [base+pw_16to1]
    m[5] = _mm256_min_epi16(m[5], m[11]); // pminsw m5, m11
    m[1] = _mm256_blendv_epi8(m[1], m[0], m[5]); // vpblendvb m1, m0, m5
    _mm256_store_si256(at(g.rsp, 96) as *mut __m256i, m[1]); // mova [rsp+96], m1
    ipred_z2_16bpc_w4_main(m, g, r8d, r10d)
}

/// `.w4_main`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_w4_main(m: &mut M, g: &mut GprQ4, r8d: u32, r10d: u32) {
    m[9] = rbi_q4(z2_x_shuf, 0); // vbroadcasti128 m9, [base+z2_x_shuf]
    let r11: isize = -8; // mov r11, -8
    ipred_z2_16bpc_w4_main_upsample_left(m, g, r8d, r10d, r11)
}

/// `.w4_main_upsample_left` to `.w4_end`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_w4_main_upsample_left(m: &mut M, g: &mut GprQ4, r8d: u32, r10d: u32, r11: isize) {
    let rsp = g.rsp;
    let stride = g.stride;
    let dx = g.dx;
    m[5] = zx(_mm_cvtsi32_si128(g.dy as i32)); // movd xm5, dyd
    m[4] = rd_q4(z2_y_shuf_h4, 0); // mova m4, [base+z2_y_shuf_h4]
    let mut r2d = r8d; // mov r2d, r8d
    m[0] = zx(_mm_cvtsi32_si128(dx as i32)); // movd xm0, dxd
    m[5] = _mm256_broadcastw_epi16(lo(m[5])); // vpbroadcastw m5, xm5
    let r5 = g.dy.rotate_right(5); // rorx r5, dyq, 5
    let r8 = (g.dy as u32).wrapping_mul(3); // lea r8d, [dyq*3]
    m[5] = _mm256_mullo_epi16(m[5], rd_q4(z2_ymul, 0)); // pmullw m5, [base+z2_ymul]
    let r9 = g.dy.rotate_right(4); // rorx r9, dyq, 4
    let mut dyd = ((g.dy as u32 as i32) >> 6) as u32; // sar dyd, 6
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    let r8 = ((r8 as i32) >> 6) as u32; // sar r8d, 6
    m[5] = _mm256_and_si256(m[5], m[11]); // pand m5, m11 ; frac_y
    dyd = dyd.wrapping_neg(); // neg dyd
    m[5] = _mm256_slli_epi16::<9>(m[5]); // psllw m5, 9
    let r5 = (r5 as u32).wrapping_add(dyd); // add r5d, dyd
    let r8 = r8.wrapping_add(dyd); // add r8d, dyd
    let r9 = (r9 as u32).wrapping_add(dyd); // add r9d, dyd
    m[7] = _mm256_add_epi16(m[0], m[0]); // paddw m7, m0, m0
    let mut dyq = at(rsp, dyd as isize * 2 + 126); // lea dyq, [rsp+dyq*2+126]
    m[0] = _mm256_blend_epi32::<0xcc>(m[0], m[7]); // vpblendd m0, m7, 0xcc
    dyq = at(dyq, r11); // add dyq, r11
                        // The three column offsets are >= 0 (ceil(k*|dy|/64) - ceil(|dy|/64)), so the 32-bit registers used as
                        // 64-bit index registers below are these values.
    let r5 = r5.wrapping_neg() as isize; // neg r5d
    m[1] = _mm256_add_epi16(m[0], m[7]); // paddw m1, m0, m7
    let r8 = r8.wrapping_neg() as isize; // neg r8d
    m[0] = _mm256_blend_epi32::<0xf0>(m[0], m[1]); // vpblendd m0, m1, 0xf0 ; xpos0 xpos1 xpos2 xpos3
    let r9 = r9.wrapping_neg() as isize; // neg r9d
    m[7] = _mm256_add_epi16(m[7], m[7]); // paddw m7, m7
    m[6] = _mm256_add_epi16(m[6], m[0]); // paddw m6, m0
    let mut r3 = 0isize;
    let mut leftonly = true;
    loop {
        // .w4_loop:
        let mut r3d = r2d.wrapping_add(dx); // lea r3d, [r2+dxq]
        r2d >>= 6; // shr r2d, 6 ; base_x0
        m[1] = zx(ldx_q4(rsp, r2d as isize * 2)); // movu xm1, [rsp+r2*2]
        r2d = r3d.wrapping_add(dx); // lea r2d, [r3+dxq]
        r3d >>= 6; // shr r3d, 6 ; base_x1
        m[3] = zx(ldx_q4(rsp, r3d as isize * 2)); // movu xm3, [rsp+r3*2]
        r3d = r2d.wrapping_add(dx); // lea r3d, [r2+dxq]
        r2d >>= 6; // shr r2d, 6 ; base_x2
        m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_q4(rsp, r2d as isize * 2)); // vinserti128 m1, [rsp+r2*2], 1
        r2d = r3d.wrapping_add(dx); // lea r2d, [r3+dxq]
        r3d >>= 6; // shr r3d, 6 ; base_x3
        m[3] = _mm256_inserti128_si256::<1>(m[3], ldx_q4(rsp, r3d as isize * 2)); // vinserti128 m3, [rsp+r3*2], 1
        m[1] = _mm256_shuffle_epi8(m[1], m[10]); // pshufb m1, m10 ; a0 a1 a2 a3 A0 A1 A2 A3
        m[3] = _mm256_shuffle_epi8(m[3], m[10]); // pshufb m3, m10 ; b0 b1 b2 b3 B0 B1 B2 B3
        m[2] = _mm256_and_si256(m[11], m[6]); // pand m2, m11, m6
        m[0] = _mm256_unpacklo_epi64(m[1], m[3]); // punpcklqdq m0, m1, m3
        m[1] = _mm256_unpackhi_epi64(m[1], m[3]); // punpckhqdq m1, m3
        m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        if (r3d as i32) < 64 {
            // cmp r3d, 64; jge .w4_toponly
            m[2] = zx(ldx_q4(dyq, 0)); // movu xm2, [dyq]
            m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_q4(dyq, r8 * 2)); // vinserti128 m2, [dyq+r8*2], 1
            m[3] = zx(ldx_q4(dyq, r5 * 2)); // movu xm3, [dyq+r5*2]
            m[3] = _mm256_inserti128_si256::<1>(m[3], ldx_q4(dyq, r9 * 2)); // vinserti128 m3, [dyq+r9*2], 1
            m[2] = _mm256_shuffle_epi8(m[2], m[9]); // pshufb m2, m9
            m[3] = _mm256_shuffle_epi8(m[3], m[9]); // pshufb m3, m9
            m[1] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m1, m2, m3 ; a3 b3 a2 b2 a1 b1 a0 b0
            m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
            m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1
            m[2] = _mm256_mulhrs_epi16(m[2], m[5]); // pmulhrsw m2, m5
            m[3] = _mm256_srai_epi16::<15>(m[6]); // psraw m3, m6, 15 ; base_x < topleft
            m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
            m[1] = _mm256_permutevar8x32_epi32(m[1], m[4]); // vpermd m1, m4, m1
            m[0] = _mm256_blendv_epi8(m[0], m[1], m[3]); // vpblendvb m0, m1, m3
        }
        // .w4_toponly:
        m[6] = _mm256_add_epi16(m[6], m[7]); // paddw m6, m7 ; xpos += dx
        r3 = stride * 3; // lea r3, [strideq*3]
        dyq = at(dyq, r11); // add dyq, r11
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        stq_q4(g.dst, 0, lo(m[0])); // movq [dstq+strideq*0], xm0
        sth_q4(g.dst, stride, lo(m[0])); // movhps [dstq+strideq*1], xm0
        stq_q4(g.dst, stride * 2, lo(m[1])); // movq [dstq+strideq*2], xm1
        sth_q4(g.dst, r3, lo(m[1])); // movhps [dstq+r3], xm1
        g.h -= 4; // sub hd, 4
        if g.h == 0 {
            // jz .w4_end
            leftonly = false;
            break;
        }
        g.dst = at(g.dst, stride * 4); // lea dstq, [dstq+strideq*4]
        if (r2d as i32) < (r10d as i32) {
            // cmp r2d, r10d; jge .w4_loop
            break;
        }
    }
    if leftonly {
        loop {
            // .w4_leftonly_loop:
            m[1] = zx(ldx_q4(dyq, 0)); // movu xm1, [dyq]
            m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_q4(dyq, r8 * 2)); // vinserti128 m1, [dyq+r8*2], 1
            m[2] = zx(ldx_q4(dyq, r5 * 2)); // movu xm2, [dyq+r5*2]
            m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_q4(dyq, r9 * 2)); // vinserti128 m2, [dyq+r9*2], 1
            dyq = at(dyq, r11); // add dyq, r11
            m[1] = _mm256_shuffle_epi8(m[1], m[9]); // pshufb m1, m9
            m[2] = _mm256_shuffle_epi8(m[2], m[9]); // pshufb m2, m9
            m[0] = _mm256_unpackhi_epi16(m[1], m[2]); // punpckhwd m0, m1, m2
            m[1] = _mm256_unpacklo_epi16(m[1], m[2]); // punpcklwd m1, m2
            m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
            m[1] = _mm256_mulhrs_epi16(m[1], m[5]); // pmulhrsw m1, m5
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[0] = _mm256_permutevar8x32_epi32(m[0], m[4]); // vpermd m0, m4, m0
            m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
            stq_q4(g.dst, 0, lo(m[0])); // movq [dstq+strideq*0], xm0
            sth_q4(g.dst, stride, lo(m[0])); // movhps [dstq+strideq*1], xm0
            stq_q4(g.dst, stride * 2, lo(m[1])); // movq [dstq+strideq*2], xm1
            sth_q4(g.dst, r3, lo(m[1])); // movhps [dstq+r3], xm1
            g.dst = at(g.dst, stride * 4); // lea dstq, [dstq+strideq*4]
            g.h -= 4; // sub hd, 4
            if (g.h as i32) <= 0 {
                // jg .w4_leftonly_loop
                break;
            }
        }
    }
    // .w4_end: RET
}

/// `.w8`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_w8(m: &mut M, g: &mut GprQ4) {
    let tl = g.tl;
    g.r10 = g.h; // mov r10d, hd
    if g.angle & 0x400 != 0 {
        // test angled, 0x400; jnz .w8_main
        return ipred_z2_16bpc_w8_main(m, g);
    }
    let mut r3d = g.angle.wrapping_add(126); // lea r3d, [angleq+126]
    let mut r8d: u32 = 0; // xor r8d, r8d
    r3d = (r3d & !0xff) | (g.h & 0xff); // mov r3b, hb
    if r3d <= 8 {
        // cmp r3d, 8; ja .w8_no_upsample_above ; angle >= 130 || h > 8 || is_sm
        m[0] = zx(ldx_q4(tl, 2)); // movu xm0, [tlq+2] ; 1 2 3 4 5 6 7 8
        m[1] = zx(ldx_q4(tl, 0)); // mova xm1, [tlq+0] ; 0 1 2 3 4 5 6 7
        m[2] = zx(_mm_blend_epi16::<0x7f>(lo(m[0]), ldx_q4(tl, 4))); // pblendw xm2, xm0, [tlq+4], 0x7f ; 2 3 4 5 6 7 8 8
        m[3] = zx(_mm_blend_epi16::<0xfe>(lo(m[1]), ldx_q4(tl, -2))); // pblendw xm3, xm1, [tlq-2], 0xfe ; 0 0 1 2 3 4 5 6
        m[4] = zx(_mm_set1_epi16(g.pixel_max as i16)); // vpbroadcastw xm4, r8m ; pixel_max
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[0]))); // paddw xm1, xm0
        m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[3]))); // paddw xm2, xm3
        r8d = !r8d; // not r8d
        m[2] = zx(_mm_sub_epi16(lo(m[1]), lo(m[2]))); // psubw xm2, xm1, xm2
        g.dx = g.dx.wrapping_add(g.dx); // add dxd, dxd
        m[2] = zx(_mm_srai_epi16::<3>(lo(m[2]))); // psraw xm2, 3
        g.angle = g.angle.wrapping_sub(53); // sub angled, 53 ; angle - 53
        m[3] = _mm256_setzero_si256(); // pxor xm3, xm3
        m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[1]))); // paddw xm2, xm1
        let r3 = g.h + 7; // lea r3d, [hq+7]
        m[2] = zx(_mm_max_epi16(lo(m[2]), lo(m[3]))); // pmaxsw xm2, xm3
        g.angle ^= 0x7f; // xor angled, 0x7f ; 180 - angle
        m[2] = zx(_mm_avg_epu16(lo(m[2]), lo(m[3]))); // pavgw xm2, xm3
        m[2] = zx(_mm_min_epi16(lo(m[2]), lo(m[4]))); // pminsw xm2, xm4
        m[1] = zx(_mm_unpacklo_epi16(lo(m[2]), lo(m[0]))); // punpcklwd xm1, xm2, xm0
        m[2] = zx(_mm_unpackhi_epi16(lo(m[2]), lo(m[0]))); // punpckhwd xm2, xm0
        stx_q4(g.rsp, 130, lo(m[1])); // movu [rsp+130], xm1
        stx_q4(g.rsp, 146, lo(m[2])); // movu [rsp+146], xm2
        let r3d = ipred_z2_16bpc_filter_strength(m, r3, g.angle); // call .filter_strength
        return ipred_z2_16bpc_w8_filter_left(m, g, r3d, r8d); // jmp .w8_filter_left
    }
    // .w8_no_upsample_above:
    let r3 = g.h + 7; // lea r3d, [hq+7]
    g.angle = g.angle.wrapping_sub(90); // sub angled, 90 ; angle - 90
    let mut r3 = ipred_z2_16bpc_filter_strength(m, r3, g.angle); // call .filter_strength
    if r3 != 0 {
        // test r3d, r3d; jz .w8_no_filter_above
        r3 = r3.count_ones(); // popcnt r3d, r3d
                              // r3 is the filter strength, 1..3: the z_filter_k loads stay inside its 36 bytes.
        let k = (r3 * 4) as isize - 4;
        m[4] = zx(rbdx_q4(z_filter_k, k + 12)); // vpbroadcastd xm4, [base+z_filter_k-4+r3*4+12*1]
        m[5] = zx(rbdx_q4(z_filter_k, k)); // vpbroadcastd xm5, [base+z_filter_k-4+r3*4+12*0]
        m[6] = zx(rbdx_q4(z_filter_k, k + 24)); // vpbroadcastd xm6, [base+z_filter_k-4+r3*4+12*2]
        m[0] = zx(ldx_q4(tl, 2)); // movu xm0, [tlq+2] ; 1 2 3 4 5 6 7 8 x
        m[2] = zx(_mm_blend_epi16::<0xfe>(lo(m[1]), ldx_q4(tl, -2))); // pblendw xm2, xm1, [tlq-2], 0xfe ; 0 0 1 2 3 4 5 6 x
        m[4] = zx(_mm_mullo_epi16(lo(m[4]), lo(m[0]))); // pmullw xm4, xm0
        m[3] = zx(_mm_blend_epi16::<0x7f>(lo(m[0]), ldx_q4(tl, 4))); // pblendw xm3, xm0, [tlq+4], 0x7f ; 2 3 4 5 6 7 8 8 x
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[3]))); // paddw xm1, xm3
        m[3] = zx(_mm_blend_epi32::<0x07>(lo(m[3]), ldx_q4(tl, 6))); // vpblendd xm3, [tlq+6], 0x07 ; 3 4 5 6 7 8 8 8 x
        m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[3]))); // paddw xm2, xm3
        m[3] = zx(_mm_set1_epi32(g.max_width)); // vpbroadcastd xm3, r6m ; max_width
        m[1] = zx(_mm_mullo_epi16(lo(m[1]), lo(m[5]))); // pmullw xm1, xm5
        m[2] = zx(_mm_mullo_epi16(lo(m[2]), lo(m[6]))); // pmullw xm2, xm6
        m[3] = zx(_mm_packs_epi32(lo(m[3]), lo(m[3]))); // packssdw xm3, xm3
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[4]))); // paddw xm1, xm4
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
        m[3] = zx(_mm_sub_epi16(lo(m[3]), rdx_q4(pw_1to16, 0))); // psubw xm3, [base+pw_1to16]
        m[4] = _mm256_setzero_si256(); // pxor xm4, xm4
        m[1] = zx(_mm_srli_epi16::<3>(lo(m[1]))); // psrlw xm1, 3
        m[3] = zx(_mm_min_epi16(lo(m[3]), lo(m[11]))); // pminsw xm3, xm11
        m[1] = zx(_mm_avg_epu16(lo(m[1]), lo(m[4]))); // pavgw xm1, xm4
        m[1] = zx(_mm_blendv_epi8(lo(m[1]), lo(m[0]), lo(m[3]))); // vpblendvb xm1, xm0, xm3
        stx_q4(g.rsp, 130, lo(m[1])); // movu [rsp+130], xm1
    }
    // .w8_no_filter_above:
    let mut r3d = g.angle.wrapping_sub(51); // lea r3d, [angleq-51]
    r3d = (r3d & !0xff) | (g.h & 0xff); // mov r3b, hb
    if r3d <= 8 {
        // cmp r3d, 8; jbe .w8_upsample_left ; angle > 140 && h <= 8 && !is_sm
        // .w8_upsample_left:
        ipred_z2_16bpc_upsample_left(m, g); // call .upsample_left
        m[7] = rbi_q4(z2_y_shuf_us, 0); // vbroadcasti128 m7, [base+z2_y_shuf_us]
        let r11 = at(g.rsp, 118); // lea r11, [rsp+118]
        let r8: isize = -8; // mov r8, -8
        return ipred_z2_16bpc_w8_main_upsample_left(m, g, r11, r8); // jmp .w8_main_upsample_left
    }
    m[0] = rbd_q4(pb_90, 0); // vpbroadcastd m0, [base+pb_90]
    m[0] = _mm256_sub_epi8(m[0], m[7]); // psubb m0, m7
    m[0] = _mm256_and_si256(m[0], m[8]); // pand m0, m8
    m[0] = _mm256_cmpgt_epi8(m[0], m[9]); // pcmpgtb m0, m9
    let r3d = _mm256_movemask_epi8(m[0]) as u32; // pmovmskb r3d, m0
    ipred_z2_16bpc_w8_filter_left(m, g, r3d, r8d)
}

/// `.w8_filter_left`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_w8_filter_left(m: &mut M, g: &mut GprQ4, r3d: u32, r8d: u32) {
    if r3d == 0 {
        // test r3d, r3d; jz .w8_main
        return ipred_z2_16bpc_w8_main(m, g);
    }
    let r3 = r3d.count_ones(); // popcnt r3d, r3d
    if r3 != 3 {
        // cmp r3d, 3; jne .w8_filter_left_s12
        return ipred_z2_16bpc_w8_filter_left_s12(m, g, r3, r8d);
    }
    m[6] = rbd_q4(pw_3, 0); // vpbroadcastd m6, [base+pw_3]
    m[7] = rbd_q4(z2_ymul8, 20); // vpbroadcastd m7, [base+pw_16]
                                 // cmp hd, 16 ; flags needed for later (evaluated from hd in .filter_left_s3b)
    ipred_z2_16bpc_filter_left_s3b(m, g) // jmp .filter_left_s3b
}

/// `.w8_filter_left_s12` (also `.w16_filter_left_s12`, which clears `r8d` first) to `.w8_main`, through the
/// upsample main loop (`; upsample_main`) when `r8d` is set (upsampled top edge).
#[inline(always)]
unsafe fn ipred_z2_16bpc_w8_filter_left_s12(m: &mut M, g: &mut GprQ4, r3: u32, r8d: u32) {
    let tl = g.tl;
    m[0] = ld_q4(tl, -32); // mova m0, [tlq-32] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    m[5] = _mm256_set1_epi32(g.max_height); // vpbroadcastd m5, r7m ; max_height
                                            // r3 is the filter strength, 1 or 2: the z_filter_k loads stay inside its 36 bytes.
    let k = (r3 * 4) as isize - 4;
    m[2] = rbd_q4(z_filter_k, k + 12); // vpbroadcastd m2, [base+z_filter_k-4+r3*4+12*1]
    m[3] = rbd_q4(z_filter_k, k); // vpbroadcastd m3, [base+z_filter_k-4+r3*4+12*0]
    m[2] = _mm256_mullo_epi16(m[2], m[0]); // pmullw m2, m0
    if g.h < 8 {
        // cmp hd, 8; jl .w8_filter_left_h4
        m[1] = _mm256_shufflehi_epi16::<{ qq4(2, 1, 0, 0) }>(m[0]); // pshufhw m1, m0, q2100 ; _ _ _ _ _ _ _ _   _ _ _ _ c c d e
    } else {
        m[4] = ld_q4(tl, -34); // movu m4, [tlq-34]
        m[1] = _mm256_unpacklo_epi16(m[0], m[0]); // punpcklwd m1, m0, m0
        m[1] = _mm256_blend_epi32::<0xee>(m[1], m[4]); // vpblendd m1, m4, 0xee ; 0 0 1 2 3 4 5 6   8 8 9 a b c d e
        if g.h != 8 {
            // je .w8_filter_left_end
            m[1] = _mm256_blend_epi32::<0x10>(m[1], m[4]); // vpblendd m1, m4, 0x10 ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
        }
    }
    // .w8_filter_left_end:
    m[1] = _mm256_add_epi16(m[1], ld_q4(tl, -30)); // paddw m1, [tlq-30] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    m[1] = _mm256_mullo_epi16(m[1], m[3]); // pmullw m1, m3
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[2] = _mm256_setzero_si256(); // pxor m2, m2
    m[1] = _mm256_srli_epi16::<3>(m[1]); // psrlw m1, 3
    m[1] = _mm256_avg_epu16(m[1], m[2]); // pavgw m1, m2
    m[5] = _mm256_packs_epi32(m[5], m[5]); // packssdw m5, m5
    m[5] = _mm256_sub_epi16(m[5], rd_q4(pw_16to1, 0)); // psubw m5, [base+pw_16to1]
    m[5] = _mm256_min_epi16(m[5], m[11]); // pminsw m5, m11
    m[1] = _mm256_blendv_epi8(m[1], m[0], m[5]); // vpblendvb m1, m0, m5
    _mm256_store_si256(at(g.rsp, 96) as *mut __m256i, m[1]); // mova [rsp+96], m1
    if r8d == 0 {
        // test r8d, r8d; jz .w8_main
        return ipred_z2_16bpc_w8_main(m, g);
    }
    // ; upsample_main
    let rsp = g.rsp;
    let stride = g.stride;
    let dx = g.dx;
    m[10] = rbi_q4(z_upsample, 0); // vbroadcasti128 m10, [base+z_upsample]
    m[7] = rbi_q4(z2_y_shuf, 0); // vbroadcasti128 m7, [base+z2_y_shuf]
    let mut r5 = at(rsp, 120); // lea r5, [rsp+120]
    m[1] = zx(_mm_cvtsi32_si128(g.dy as i32)); // movd xm1, dyd
    m[4] = rbi_q4(z_base_inc, 2); // vbroadcasti128 m4, [base+z_base_inc+2]
    m[2] = zx(_mm_cvtsi32_si128(dx as i32)); // movd xm2, dxd
    m[1] = _mm256_broadcastw_epi16(lo(m[1])); // vpbroadcastw m1, xm1
    m[2] = _mm256_broadcastw_epi16(lo(m[2])); // vpbroadcastw m2, xm2
                                              // mov r7, dstq (r7 is not read on this path)
    m[4] = _mm256_add_epi16(m[4], m[4]); // paddw m4, m4
    m[0] = _mm256_mullo_epi16(m[1], rd_q4(z2_ymul8, 0)); // pmullw m0, m1, [base+z2_ymul8]
    m[5] = _mm256_add_epi16(m[2], m[2]); // paddw m5, m2, m2
    m[1] = zx(_mm_slli_epi16::<3>(lo(m[1]))); // psllw xm1, 3
    m[2] = _mm256_blend_epi32::<0xf0>(m[2], m[5]); // vpblendd m2, m5, 0xf0
    let mut r2d = dx.wrapping_add(66 << 6); // lea r2d, [dxq+(66<<6)] ; xpos
    m[4] = _mm256_add_epi16(m[4], m[2]); // paddw m4, m2
    m[6] = _mm256_shuffle_epi32::<{ qq4(2, 0, 2, 0) }>(m[0]); // pshufd m6, m0, q2020
    m[0] = zx(_mm_srai_epi16::<6>(lo(m[0]))); // psraw xm0, 6
    m[1] = _mm256_setzero_si256(); // pxor xm1, xm1
    m[8] = zx(_mm_sub_epi16(lo(m[1]), lo(m[0]))); // psubw xm8, xm1, xm0
    m[6] = _mm256_and_si256(m[6], m[11]); // pand m6, m11
    m[9] = zx(_mm_unpackhi_epi16(lo(m[8]), lo(m[1]))); // punpckhwd xm9, xm8, xm1
    m[6] = _mm256_slli_epi16::<9>(m[6]); // psllw m6, 9
    m[8] = zx(_mm_unpacklo_epi16(lo(m[8]), lo(m[1]))); // punpcklwd xm8, xm1
    loop {
        // .w8_upsample_above_loop:
        let mut r3d = r2d.wrapping_add(dx); // lea r3d, [r2+dxq]
        r2d >>= 6; // shr r2d, 6
        m[1] = zx(ldx_q4(rsp, r2d as isize * 2)); // movu xm1, [rsp+r2*2]
        m[2] = zx(ldx_q4(rsp, r2d as isize * 2 + 16)); // movu xm2, [rsp+r2*2+16]
        r2d = r3d.wrapping_add(dx); // lea r2d, [r3+dxq]
        r3d >>= 6; // shr r3d, 6
        m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_q4(rsp, r3d as isize * 2)); // vinserti128 m1, [rsp+r3*2], 1
        m[2] = _mm256_inserti128_si256::<1>(m[2], ldx_q4(rsp, r3d as isize * 2 + 16)); // vinserti128 m2, [rsp+r3*2+16], 1
        m[1] = _mm256_shuffle_epi8(m[1], m[10]); // pshufb m1, m10
        m[2] = _mm256_shuffle_epi8(m[2], m[10]); // pshufb m2, m10
        m[0] = _mm256_unpacklo_epi64(m[1], m[2]); // punpcklqdq m0, m1, m2 ; a0 b0 c0 d0 e0 f0 g0 h0
        m[1] = _mm256_unpackhi_epi64(m[1], m[2]); // punpckhqdq m1, m2
        m[2] = _mm256_and_si256(m[11], m[4]); // pand m2, m11, m4
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        if (r3d as i32) < 64 {
            // cmp r3d, 64; jge .w8_upsample_above_toponly
            m[1] = m[5]; // mova m1, m5
            m[3] = gather_q4(m[3], r5, lo(m[9]), m[5]); // vpgatherdq m3, [r5+xm9*2], m5
                                                        // mova m5, m1
            m[2] = gather_q4(m[2], r5, lo(m[8]), m[1]); // vpgatherdq m2, [r5+xm8*2], m1
            m[3] = _mm256_shuffle_epi8(m[3], m[7]); // pshufb m3, m7
            m[2] = _mm256_shuffle_epi8(m[2], m[7]); // pshufb m2, m7
            m[1] = _mm256_unpacklo_epi32(m[2], m[3]); // punpckldq m1, m2, m3
            m[2] = _mm256_unpackhi_epi32(m[2], m[3]); // punpckhdq m2, m3
            m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1
            m[2] = _mm256_mulhrs_epi16(m[2], m[6]); // pmulhrsw m2, m6
            m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
            m[1] = _mm256_permute4x64_epi64::<{ qq4(3, 1, 2, 0) }>(m[1]); // vpermq m1, m1, q3120
            m[2] = _mm256_srai_epi16::<15>(m[4]); // psraw m2, m4, 15
            m[0] = _mm256_blendv_epi8(m[0], m[1], m[2]); // vpblendvb m0, m1, m2
        }
        // .w8_upsample_above_toponly:
        m[4] = _mm256_add_epi16(m[4], m[5]); // paddw m4, m5
        r5 = at(r5, -4); // sub r5, 4
        stx_q4(g.dst, 0, lo(m[0])); // mova [dstq+strideq*0], xm0
        stx_q4(g.dst, stride, _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
        g.h -= 2; // sub hd, 2
        if g.h == 0 {
            // jz .w8_ret
            return;
        }
        g.dst = at(g.dst, stride * 2); // lea dstq, [dstq+strideq*2]
    }
}

/// `.w8_main`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_w8_main(m: &mut M, g: &mut GprQ4) {
    m[7] = rbi_q4(z2_y_shuf, 0); // vbroadcasti128 m7, [base+z2_y_shuf]
    let r11 = at(g.rsp, 120); // lea r11, [rsp+120]
    let r8: isize = -4; // mov r8, -4
    ipred_z2_16bpc_w8_main_upsample_left(m, g, r11, r8)
}

/// `.w8_main_upsample_left` to `.w8_ret` (all widths from 8 on, in strips of 8 columns: `.w8_loop0`).
#[inline(always)]
unsafe fn ipred_z2_16bpc_w8_main_upsample_left(m: &mut M, g: &mut GprQ4, r11: *mut u8, r8: isize) {
    let rsp = g.rsp;
    let stride = g.stride;
    let dx = g.dx;
    m[1] = zx(_mm_cvtsi32_si128(g.dy as i32)); // movd xm1, dyd
    m[4] = rbi_q4(z_base_inc, 2); // vbroadcasti128 m4, [base+z_base_inc+2]
    m[2] = zx(_mm_cvtsi32_si128(dx as i32)); // movd xm2, dxd
    m[1] = _mm256_broadcastw_epi16(lo(m[1])); // vpbroadcastw m1, xm1
    m[2] = _mm256_broadcastw_epi16(lo(m[2])); // vpbroadcastw m2, xm2
    let mut r7 = g.dst; // mov r7, dstq
    m[0] = _mm256_mullo_epi16(m[1], rd_q4(z2_ymul8, 0)); // pmullw m0, m1, [base+z2_ymul8]
    m[5] = _mm256_add_epi16(m[2], m[2]); // paddw m5, m2, m2
    m[1] = zx(_mm_slli_epi16::<3>(lo(m[1]))); // psllw xm1, 3
    m[2] = _mm256_blend_epi32::<0xf0>(m[2], m[5]); // vpblendd m2, m5, 0xf0 ; xpos0 xpos1
    let mut r9d = dx.wrapping_add(65 << 6); // lea r9d, [dxq+(65<<6)] ; xpos
    m[4] = _mm256_add_epi16(m[4], m[2]); // paddw m4, m2
    let s284 = _mm_cvtsi128_si32(lo(m[1])); // movd [rsp+284], xm1 (a local)
    loop {
        // .w8_loop0:
        let mut r2d = r9d; // mov r2d, r9d
        let s288 = m[0]; // mova [rsp+288], m0 (a local)
        let mut r5 = r11; // mov r5, r11
        let s320 = m[4]; // mova [rsp+320], m4 (a local)
        m[6] = _mm256_shuffle_epi32::<{ qq4(2, 0, 2, 0) }>(m[0]); // pshufd m6, m0, q2020
        m[0] = zx(_mm_srai_epi16::<6>(lo(m[0]))); // psraw xm0, 6
        m[1] = _mm256_setzero_si256(); // pxor xm1, xm1
        m[8] = zx(_mm_sub_epi16(lo(m[1]), lo(m[0]))); // psubw xm8, xm1, xm0 ; base_y
        m[6] = _mm256_and_si256(m[6], m[11]); // pand m6, m11 ; frac_y
        m[9] = zx(_mm_unpackhi_epi16(lo(m[8]), lo(m[1]))); // punpckhwd xm9, xm8, xm1 ; base_y 2 3 6 7
        m[6] = _mm256_slli_epi16::<9>(m[6]); // psllw m6, 9
        m[8] = zx(_mm_unpacklo_epi16(lo(m[8]), lo(m[1]))); // punpcklwd xm8, xm1 ; base_y 0 1 4 5
        let mut leftonly = true;
        loop {
            // .w8_loop:
            let mut r3d = r2d.wrapping_add(dx); // lea r3d, [r2+dxq]
            r2d >>= 6; // shr r2d, 6 ; base_x0
            m[0] = zx(ldx_q4(rsp, r2d as isize * 2)); // movu xm0, [rsp+r2*2]
            m[1] = zx(ldx_q4(rsp, r2d as isize * 2 + 2)); // movu xm1, [rsp+r2*2+2]
            r2d = r3d.wrapping_add(dx); // lea r2d, [r3+dxq]
            r3d >>= 6; // shr r3d, 6 ; base_x1
            m[0] = _mm256_inserti128_si256::<1>(m[0], ldx_q4(rsp, r3d as isize * 2)); // vinserti128 m0, [rsp+r3*2], 1
            m[1] = _mm256_inserti128_si256::<1>(m[1], ldx_q4(rsp, r3d as isize * 2 + 2)); // vinserti128 m1, [rsp+r3*2+2], 1
            m[2] = _mm256_and_si256(m[11], m[4]); // pand m2, m11, m4
            m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
            m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
            m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            if (r3d as i32) < 64 {
                // cmp r3d, 64; jge .w8_toponly
                m[1] = m[5]; // mova m1, m5
                m[3] = gather_q4(m[3], r5, lo(m[9]), m[5]); // vpgatherdq m3, [r5+xm9*2], m5
                                                            // mova m5, m1
                m[2] = gather_q4(m[2], r5, lo(m[8]), m[1]); // vpgatherdq m2, [r5+xm8*2], m1
                m[3] = _mm256_shuffle_epi8(m[3], m[7]); // pshufb m3, m7 ; c0 d0 c1 d1               g0 h0 g1 h1
                m[2] = _mm256_shuffle_epi8(m[2], m[7]); // pshufb m2, m7 ; a0 b0 a1 b1               e0 f0 e1 f1
                m[1] = _mm256_unpacklo_epi32(m[2], m[3]); // punpckldq m1, m2, m3 ; a0 b0 c0 d0 a1 b1 c1 d1   e0 f0 g0 h0 e1 f1 g1 h1
                m[2] = _mm256_unpackhi_epi32(m[2], m[3]); // punpckhdq m2, m3
                m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1
                m[2] = _mm256_mulhrs_epi16(m[2], m[6]); // pmulhrsw m2, m6
                m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
                m[1] = _mm256_permute4x64_epi64::<{ qq4(3, 1, 2, 0) }>(m[1]); // vpermq m1, m1, q3120
                m[2] = _mm256_srai_epi16::<15>(m[4]); // psraw m2, m4, 15 ; base_x < topleft
                m[0] = _mm256_blendv_epi8(m[0], m[1], m[2]); // vpblendvb m0, m1, m2
            }
            // .w8_toponly:
            m[4] = _mm256_add_epi16(m[4], m[5]); // paddw m4, m5 ; xpos += dx
            r5 = at(r5, r8); // add r5, r8
            stx_q4(g.dst, 0, lo(m[0])); // mova [dstq+strideq*0], xm0
            stx_q4(g.dst, stride, _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
            g.h -= 2; // sub hd, 2
            if g.h == 0 {
                // jz .w8_end
                leftonly = false;
                break;
            }
            g.dst = at(g.dst, stride * 2); // lea dstq, [dstq+strideq*2]
            if (r2d as i32) < (63 - 8) << 6 {
                // cmp r2d, (63-8)<<6; jge .w8_loop
                break;
            }
        }
        if leftonly {
            loop {
                // .w8_leftonly_loop:
                m[0] = m[5]; // mova m0, m5
                m[4] = gather_q4(m[4], r5, lo(m[9]), m[5]); // vpgatherdq m4, [r5+xm9*2], m5
                                                            // mova m5, m0
                m[3] = gather_q4(m[3], r5, lo(m[8]), m[0]); // vpgatherdq m3, [r5+xm8*2], m0
                r5 = at(r5, r8); // add r5, r8
                m[2] = _mm256_shuffle_epi8(m[4], m[7]); // pshufb m2, m4, m7
                m[1] = _mm256_shuffle_epi8(m[3], m[7]); // pshufb m1, m3, m7
                m[0] = _mm256_unpacklo_epi32(m[1], m[2]); // punpckldq m0, m1, m2
                m[1] = _mm256_unpackhi_epi32(m[1], m[2]); // punpckhdq m1, m2
                m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
                m[1] = _mm256_mulhrs_epi16(m[1], m[6]); // pmulhrsw m1, m6
                m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
                m[0] = _mm256_permute4x64_epi64::<{ qq4(3, 1, 2, 0) }>(m[0]); // vpermq m0, m0, q3120
                stx_q4(g.dst, 0, lo(m[0])); // mova [dstq+strideq*0], xm0
                stx_q4(g.dst, stride, _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
                g.dst = at(g.dst, stride * 2); // lea dstq, [dstq+strideq*2]
                g.h -= 2; // sub hd, 2
                if (g.h as i32) <= 0 {
                    // jg .w8_leftonly_loop
                    break;
                }
            }
        }
        // .w8_end:
        g.r10 = g.r10.wrapping_sub(1 << 8); // sub r10d, 1<<8
        if (g.r10 as i32) < 0 {
            // jl .w8_ret
            return;
        }
        m[0] = _mm256_set1_epi32(s284); // vpbroadcastd m0, [rsp+284]
        r7 = at(r7, 16); // add r7, 16
        m[0] = _mm256_add_epi16(m[0], s288); // paddw m0, [rsp+288] ; base_y += 8*dy
        r9d = r9d.wrapping_add(8 << 6); // add r9d, 8<<6
        m[4] = rbd_q4(pw_512, 0); // vpbroadcastd m4, [pw_512]
        g.h = g.r10 & 0xff; // movzx hd, r10b
        m[4] = _mm256_add_epi16(m[4], s320); // paddw m4, [rsp+320] ; base_x += 8*64
        g.dst = r7; // mov dstq, r7
    } // jmp .w8_loop0
}

/// `.w16`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_w16(m: &mut M, g: &mut GprQ4) {
    let tl = g.tl;
    m[0] = zx(ldd_q4(tl, 32)); // movd xm0, [tlq+32]
    g.r10 = g.h + (1 << 8); // lea r10d, [hq+(1<<8)]
    std_q4(g.rsp, 160, lo(m[0])); // movd [rsp+160], xm0
    if g.angle & 0x400 != 0 {
        // test angled, 0x400; jnz .w8_main
        return ipred_z2_16bpc_w8_main(m, g);
    }
    let r3 = g.h + 15; // lea r3d, [hq+15]
    g.angle = g.angle.wrapping_sub(90); // sub angled, 90
    let mut r3 = ipred_z2_16bpc_filter_strength(m, r3, g.angle); // call .filter_strength
    if r3 != 0 {
        // test r3d, r3d; jz .w16_no_filter_above
        r3 = r3.count_ones(); // popcnt r3d, r3d
                              // r3 is the filter strength, 1..3: the z_filter_k loads stay inside its 36 bytes.
        let k = (r3 * 4) as isize - 4;
        m[4] = rbd_q4(z_filter_k, k + 12); // vpbroadcastd m4, [base+z_filter_k-4+r3*4+12*1]
        m[5] = rbd_q4(z_filter_k, k); // vpbroadcastd m5, [base+z_filter_k-4+r3*4+12*0]
        m[6] = rbd_q4(z_filter_k, k + 24); // vpbroadcastd m6, [base+z_filter_k-4+r3*4+12*2]
        m[0] = ld_q4(tl, 2); // movu m0, [tlq+2] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[2] = zx(_mm_unpacklo_epi16(lo(m[1]), lo(m[1]))); // punpcklwd xm2, xm1, xm1
        m[2] = _mm256_blend_epi32::<0xfe>(m[2], ld_q4(tl, -2)); // vpblendd m2, [tlq-2], 0xfe ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
        m[3] = _mm256_unpackhi_epi16(m[0], m[0]); // punpckhwd m3, m0, m0
        m[4] = _mm256_mullo_epi16(m[4], m[0]); // pmullw m4, m0
        m[3] = _mm256_blend_epi32::<0x7f>(m[3], ld_q4(tl, 4)); // vpblendd m3, [tlq+4], 0x7f ; 2 3 4 5 6 7 8 9   a b c d e f g g
        m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3
        m[3] = _mm256_blend_epi32::<0x7f>(m[3], ld_q4(tl, 6)); // vpblendd m3, [tlq+6], 0x7f ; 3 4 5 6 7 8 9 a   b c d e f g g g
        m[2] = _mm256_add_epi16(m[2], m[3]); // paddw m2, m3
        m[3] = _mm256_set1_epi32(g.max_width); // vpbroadcastd m3, r6m ; max_width
        m[1] = _mm256_mullo_epi16(m[1], m[5]); // pmullw m1, m5
        m[2] = _mm256_mullo_epi16(m[2], m[6]); // pmullw m2, m6
        m[3] = _mm256_packs_epi32(m[3], m[3]); // packssdw m3, m3
        m[1] = _mm256_add_epi16(m[1], m[4]); // paddw m1, m4
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        m[3] = _mm256_sub_epi16(m[3], rd_q4(pw_1to16, 0)); // psubw m3, [base+pw_1to16]
        m[4] = _mm256_setzero_si256(); // pxor m4, m4
        m[1] = _mm256_srli_epi16::<3>(m[1]); // psrlw m1, 3
        m[3] = _mm256_min_epi16(m[3], m[11]); // pminsw m3, m11
        m[1] = _mm256_avg_epu16(m[1], m[4]); // pavgw m1, m4
        m[1] = _mm256_blendv_epi8(m[1], m[0], m[3]); // vpblendvb m1, m0, m3
        st_q4(g.rsp, 130, m[1]); // movu [rsp+130], m1
    }
    // .w16_no_filter_above:
    m[0] = rbd_q4(pb_90, 0); // vpbroadcastd m0, [base+pb_90]
    m[0] = _mm256_sub_epi8(m[0], m[7]); // psubb m0, m7
    m[0] = _mm256_and_si256(m[0], m[8]); // pand m0, m8
    m[0] = _mm256_cmpgt_epi8(m[0], m[9]); // pcmpgtb m0, m9
    let r3 = _mm256_movemask_epi8(m[0]) as u32; // pmovmskb r3d, m0
    if r3 == 0 {
        // test r3d, r3d; jz .w8_main
        return ipred_z2_16bpc_w8_main(m, g);
    }
    let r3 = r3.count_ones(); // popcnt r3d, r3d
    if r3 != 3 {
        // cmp r3d, 3; jne .w16_filter_left_s12
        // .w16_filter_left_s12:
        let r8d = 0; // xor r8d, r8d
        return ipred_z2_16bpc_w8_filter_left_s12(m, g, r3, r8d);
    }
    m[6] = rbd_q4(pw_3, 0); // vpbroadcastd m6, [base+pw_3]
    m[7] = rbd_q4(z2_ymul8, 20); // vpbroadcastd m7, [base+pw_16]
    if g.h != 4 {
        // cmp hd, 4; jne .filter_left_s3
        return ipred_z2_16bpc_filter_left_s3(m, g);
    }
    m[0] = zx(ldq_q4(tl, -8)); // movq xm0, [tlq-8] ; 0 1 2 3
    m[1] = zx(ldq_q4(tl, -6)); // movq xm1, [tlq-6] ; 1 2 3 4
    m[5] = zx(_mm_set1_epi32(g.max_height)); // vpbroadcastd xm5, r7m ; max_height
    m[4] = zx(_mm_loadl_epi64(ro(pw_16to1, 24) as *const __m128i)); // movq xm4, [base+pw_16to1+24] ; 4to1
    m[2] = zx(_mm_shufflelo_epi16::<{ qq4(2, 1, 0, 0) }>(lo(m[0]))); // pshuflw xm2, xm0, q2100 ; 0 0 1 2
    m[3] = zx(_mm_shufflelo_epi16::<{ qq4(3, 3, 2, 1) }>(lo(m[1]))); // pshuflw xm3, xm1, q3321 ; 2 3 4 4
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[0]))); // paddw xm1, xm0
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
    m[2] = zx(_mm_shufflelo_epi16::<{ qq4(1, 0, 0, 0) }>(lo(m[0]))); // pshuflw xm2, xm0, q1000 ; 0 0 0 1
    m[3] = zx(_mm_add_epi16(lo(m[3]), lo(m[6]))); // paddw xm3, xm6
    m[5] = zx(_mm_packs_epi32(lo(m[5]), lo(m[5]))); // packssdw xm5, xm5
    m[2] = zx(_mm_avg_epu16(lo(m[2]), lo(m[3]))); // pavgw xm2, xm3
    m[5] = zx(_mm_sub_epi16(lo(m[5]), lo(m[4]))); // psubw xm5, xm4
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
    m[5] = zx(_mm_min_epi16(lo(m[5]), lo(m[11]))); // pminsw xm5, xm11
    m[1] = zx(_mm_srli_epi16::<2>(lo(m[1]))); // psrlw xm1, 2
    m[1] = zx(_mm_blendv_epi8(lo(m[1]), lo(m[0]), lo(m[5]))); // vpblendvb xm1, xm0, xm5
    stq_q4(g.rsp, 120, lo(m[1])); // movq [rsp+120], xm1
    ipred_z2_16bpc_w8_main(m, g) // jmp .w8_main
}

/// `.w32`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_w32(m: &mut M, g: &mut GprQ4) {
    let tl = g.tl;
    m[2] = ld_q4(tl, 32); // mova m2, [tlq+32]
    m[0] = zx(ldd_q4(tl, 64)); // movd xm0, [tlq+64]
    g.r10 = g.h + (3 << 8); // lea r10d, [hq+(3<<8)]
    _mm256_store_si256(at(g.rsp, 160) as *mut __m256i, m[2]); // mova [rsp+160], m2
    std_q4(g.rsp, 192, lo(m[0])); // movd [rsp+192], xm0
    if g.angle & 0x400 != 0 {
        // test angled, 0x400; jnz .w8_main
        return ipred_z2_16bpc_w8_main(m, g);
    }
    m[6] = rbd_q4(pw_3, 0); // vpbroadcastd m6, [base+pw_3]
    m[0] = _mm256_set1_epi32(g.max_width); // vpbroadcastd m0, r6m ; max_width
    m[7] = rbd_q4(z2_ymul8, 20); // vpbroadcastd m7, [base+pw_16]
    let r3: isize = 32; // mov r3d, 32
    m[0] = _mm256_packs_epi32(m[0], m[0]); // packssdw m0, m0
    m[0] = _mm256_sub_epi16(m[0], rd_q4(pw_1to16, 0)); // psubw m0, [base+pw_1to16]
    m[8] = _mm256_min_epi16(m[0], m[11]); // pminsw m8, m0, m11
    m[9] = _mm256_sub_epi16(m[8], m[7]); // psubw m9, m8, m7
    ipred_z2_16bpc_w32_filter_above(m, g, r3)
}

/// `.w32_filter_above`, then `.filter_left_s3`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_w32_filter_above(m: &mut M, g: &mut GprQ4, r3: isize) {
    let tl = g.tl;
    m[0] = ld_q4(tl, 2); // movu m0, [tlq+2]
    m[4] = zx(_mm_unpacklo_epi16(lo(m[1]), lo(m[1]))); // punpcklwd xm4, xm1, xm1
    m[2] = _mm256_add_epi16(m[6], ld_q4(tl, 6)); // paddw m2, m6, [tlq+6]
    m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
    m[4] = _mm256_blend_epi32::<0xfe>(m[4], ld_q4(tl, -2)); // vpblendd m4, [tlq-2], 0xfe ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
    m[1] = _mm256_add_epi16(m[1], ld_q4(tl, 4)); // paddw m1, [tlq+4]
    m[3] = ld_q4(tl, r3 + 2); // movu m3, [tlq+r3+2]
    m[5] = _mm256_add_epi16(m[6], ld_q4(tl, r3 - 2)); // paddw m5, m6, [tlq+r3-2]
    m[2] = _mm256_avg_epu16(m[2], m[4]); // pavgw m2, m4
    m[4] = _mm256_unpackhi_epi16(m[3], m[3]); // punpckhwd m4, m3, m3
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[2] = _mm256_blend_epi32::<0x7f>(m[4], ld_q4(tl, r3 + 6)); // vpblendd m2, m4, [tlq+r3+6], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h h h
    m[4] = _mm256_blend_epi32::<0x7f>(m[4], ld_q4(tl, r3 + 4)); // vpblendd m4, [tlq+r3+4], 0x7f ; 3 4 5 6 7 8 9 a   b c d e f g h h
    m[2] = _mm256_avg_epu16(m[2], m[5]); // pavgw m2, m5
    m[5] = _mm256_add_epi16(m[3], ld_q4(tl, r3)); // paddw m5, m3, [tlq+r3]
    m[4] = _mm256_add_epi16(m[4], m[5]); // paddw m4, m5
    m[1] = _mm256_srli_epi16::<2>(m[1]); // psrlw m1, 2
    m[2] = _mm256_add_epi16(m[2], m[4]); // paddw m2, m4
    m[1] = _mm256_blendv_epi8(m[1], m[0], m[8]); // vpblendvb m1, m0, m8
    m[2] = _mm256_srli_epi16::<2>(m[2]); // psrlw m2, 2
    m[2] = _mm256_blendv_epi8(m[2], m[3], m[9]); // vpblendvb m2, m3, m9
    st_q4(g.rsp, 130, m[1]); // movu [rsp+130], m1
    st_q4(g.rsp, r3 + 130, m[2]); // movu [rsp+r3+130], m2
    ipred_z2_16bpc_filter_left_s3(m, g)
}

/// `.filter_left_s3`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_filter_left_s3(m: &mut M, g: &mut GprQ4) {
    if g.h < 16 {
        // cmp hd, 16; jl .filter_left_s3_h8 ; h8
        return ipred_z2_16bpc_filter_left_s3_h8(m, g);
    }
    ipred_z2_16bpc_filter_left_s3b(m, g)
}

/// `.filter_left_s3b` and `.filter_left_s3_end`, then `.w8_main`. Both callers end in `cmp hd, 16`, whose flags
/// the `je` and `jnp` below test (no instruction in between writes the flags): they are evaluated from `hd`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_filter_left_s3b(m: &mut M, g: &mut GprQ4) {
    let tl = g.tl;
    m[0] = ld_q4(tl, -32); // mova m0, [tlq-32] ; 2 3 4 5 6 7 8 9   a b c d e f g h
    m[2] = ld_q4(tl, -30); // movu m2, [tlq-30] ; 3 4 5 6 7 8 9 a   b c d e f g h i
    m[5] = _mm256_set1_epi32(g.max_height); // vpbroadcastd m5, r7m ; max_height
    m[1] = _mm256_add_epi16(m[0], m[2]); // paddw m1, m0, m2
    m[2] = _mm256_unpackhi_epi16(m[2], m[2]); // punpckhwd m2, m2
    let mut r3 = g.h as u64; // mov r3d, hd
    m[2] = _mm256_blend_epi32::<0x7f>(m[2], ld_q4(tl, -28)); // vpblendd m2, [tlq-28], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h i i
    m[5] = _mm256_packs_epi32(m[5], m[5]); // packssdw m5, m5
    r3 = !r3; // not r3
    m[5] = _mm256_sub_epi16(m[5], rd_q4(pw_16to1, 0)); // psubw m5, [base+pw_16to1]
    m[2] = _mm256_add_epi16(m[2], m[6]); // paddw m2, m6
    m[8] = _mm256_min_epi16(m[11], m[5]); // pminsw m8, m11, m5
    if g.h != 16 {
        // je .filter_left_s3_end ; h16
        m[1] = _mm256_add_epi16(m[1], ld_q4(tl, -34)); // paddw m1, [tlq-34] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[2] = _mm256_avg_epu16(m[2], ld_q4(tl, -36)); // pavgw m2, [tlq-36] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        m[1] = _mm256_srli_epi16::<2>(m[1]); // psrlw m1, 2
        m[3] = _mm256_blendv_epi8(m[1], m[0], m[8]); // vpblendvb m3, m1, m0, m8
        m[0] = ld_q4(tl, -64); // mova m0, [tlq-64] ; 2 3 4 5 6 7 8 9   a b c d e f g h
        m[1] = _mm256_add_epi16(m[0], ld_q4(tl, -62)); // paddw m1, m0, [tlq-62] ; 3 4 5 6 7 8 9 a   b c d e f g h i
        m[2] = _mm256_add_epi16(m[6], ld_q4(tl, -60)); // paddw m2, m6, [tlq-60] ; 4 5 6 7 8 9 a b   c d e f g h i j
        m[8] = _mm256_sub_epi16(m[8], m[7]); // psubw m8, m7
        _mm256_store_si256(at(g.rsp, 96) as *mut __m256i, m[3]); // mova [rsp+96], m3
                                                                 // jnp .filter_left_s3_end ; h32: the parity flag of the low byte of hd - 16 (h is 32 or 64 here)
        if (g.h.wrapping_sub(16) as u8).count_ones() & 1 == 0 {
            m[5] = ld_q4(tl, -96); // mova m5, [tlq-96]
            m[1] = _mm256_add_epi16(m[1], ld_q4(tl, -66)); // paddw m1, [tlq-66]
            m[2] = _mm256_avg_epu16(m[2], ld_q4(tl, -68)); // pavgw m2, [tlq-68]
            m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
            m[4] = _mm256_add_epi16(m[5], ld_q4(tl, -94)); // paddw m4, m5, [tlq-94]
            m[2] = _mm256_add_epi16(m[6], ld_q4(tl, -92)); // paddw m2, m6, [tlq-92]
            m[1] = _mm256_srli_epi16::<2>(m[1]); // psrlw m1, 2
            m[4] = _mm256_add_epi16(m[4], ld_q4(tl, -98)); // paddw m4, [tlq- 98]
            m[2] = _mm256_avg_epu16(m[2], ld_q4(tl, -100)); // pavgw m2, [tlq-100]
            m[3] = _mm256_blendv_epi8(m[1], m[0], m[8]); // vpblendvb m3, m1, m0, m8
            m[0] = ld_q4(tl, -128); // mova m0, [tlq-128]
            m[8] = _mm256_sub_epi16(m[8], m[7]); // psubw m8, m7
            m[4] = _mm256_add_epi16(m[4], m[2]); // paddw m4, m2
            m[1] = _mm256_add_epi16(m[0], ld_q4(tl, -126)); // paddw m1, m0, [tlq-126]
            m[2] = _mm256_add_epi16(m[6], ld_q4(tl, -124)); // paddw m2, m6, [tlq-124]
            m[4] = _mm256_srli_epi16::<2>(m[4]); // psrlw m4, 2
            _mm256_store_si256(at(g.rsp, 64) as *mut __m256i, m[3]); // mova [rsp+64], m3
            m[4] = _mm256_blendv_epi8(m[4], m[5], m[8]); // vpblendvb m4, m5, m8
            m[8] = _mm256_sub_epi16(m[8], m[7]); // psubw m8, m7
            _mm256_store_si256(at(g.rsp, 32) as *mut __m256i, m[4]); // mova [rsp+32], m4
        }
    }
    // .filter_left_s3_end:
    let r3 = r3 as i64 as isize; // r3 = -h-1
    m[3] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[0]))); // punpcklwd xm3, xm0, xm0
    m[4] = _mm256_blend_epi32::<0xfe>(m[3], ld_q4(g.tl, r3 * 2)); // vpblendd m4, m3, [tlq+r3*2], 0xfe ; 2 2 3 4 5 6 7 8   9 a b c d e f g
    m[3] = _mm256_blend_epi32::<0xfe>(m[3], ld_q4(g.tl, r3 * 2 - 2)); // vpblendd m3, [tlq+r3*2-2], 0xfe ; 2 2 2 3 4 5 6 7   8 9 a b c d e f
    m[1] = _mm256_add_epi16(m[1], m[4]); // paddw m1, m4
    m[2] = _mm256_avg_epu16(m[2], m[3]); // pavgw m2, m3
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[1] = _mm256_srli_epi16::<2>(m[1]); // psrlw m1, 2
    m[1] = _mm256_blendv_epi8(m[1], m[0], m[8]); // vpblendvb m1, m0, m8
    _mm256_store_si256(at(g.rsp, r3 * 2 + 130) as *mut __m256i, m[1]); // mova [rsp+r3*2+130], m1
    ipred_z2_16bpc_w8_main(m, g) // jmp .w8_main
}

/// `.filter_left_s3_h8`, then `.w8_main`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_filter_left_s3_h8(m: &mut M, g: &mut GprQ4) {
    let tl = g.tl;
    m[0] = zx(ldx_q4(tl, -16)); // mova xm0, [tlq-16] ; 0 1 2 3 4 5 6 7
    m[3] = zx(ldx_q4(tl, -14)); // movu xm3, [tlq-14] ; 1 2 3 4 5 6 7 8
    m[2] = zx(_mm_blend_epi16::<0xfe>(lo(m[0]), ldx_q4(tl, -18))); // pblendw xm2, xm0, [tlq-18], 0xfe ; 0 0 1 2 3 4 5 6
    m[5] = zx(_mm_set1_epi32(g.max_height)); // vpbroadcastd xm5, r7m ; max_height
    m[1] = zx(_mm_add_epi16(lo(m[0]), lo(m[3]))); // paddw xm1, xm0, xm3
    m[3] = zx(_mm_blend_epi16::<0x7f>(lo(m[3]), ldx_q4(tl, -12))); // pblendw xm3, [tlq-12], 0x7f ; 2 3 4 5 6 7 8 8
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
    m[2] = zx(_mm_blend_epi32::<0x0e>(lo(m[2]), ldx_q4(tl, -20))); // vpblendd xm2, [tlq-20], 0x0e ; 0 0 0 1 2 3 4 5
    m[3] = zx(_mm_add_epi16(lo(m[3]), lo(m[6]))); // paddw xm3, xm6
    m[5] = zx(_mm_packs_epi32(lo(m[5]), lo(m[5]))); // packssdw xm5, xm5
    m[2] = zx(_mm_avg_epu16(lo(m[2]), lo(m[3]))); // pavgw xm2, xm3
    m[5] = zx(_mm_sub_epi16(lo(m[5]), rdx_q4(pw_16to1, 16))); // psubw xm5, [base+pw_16to1+16] ; 8to1
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
    m[5] = zx(_mm_min_epi16(lo(m[5]), lo(m[11]))); // pminsw xm5, xm11
    m[1] = zx(_mm_srli_epi16::<2>(lo(m[1]))); // psrlw xm1, 2
    m[1] = zx(_mm_blendv_epi8(lo(m[1]), lo(m[0]), lo(m[5]))); // vpblendvb xm1, xm0, xm5
    _mm_store_si128(at(g.rsp, 112) as *mut __m128i, lo(m[1])); // mova [rsp+112], xm1
    ipred_z2_16bpc_w8_main(m, g) // jmp .w8_main
}

/// `.w64`.
#[inline(always)]
unsafe fn ipred_z2_16bpc_w64(m: &mut M, g: &mut GprQ4) {
    let tl = g.tl;
    m[2] = ld_q4(tl, 32); // mova m2, [tlq+ 32]
    m[3] = ld_q4(tl, 64); // mova m3, [tlq+ 64]
    m[4] = ld_q4(tl, 96); // mova m4, [tlq+ 96]
    m[0] = zx(ldd_q4(tl, 128)); // movd xm0, [tlq+128]
    g.r10 = g.h + (7 << 8); // lea r10d, [hq+(7<<8)]
    _mm256_store_si256(at(g.rsp, 160) as *mut __m256i, m[2]); // mova [rsp+160], m2
    _mm256_store_si256(at(g.rsp, 192) as *mut __m256i, m[3]); // mova [rsp+192], m3
    _mm256_store_si256(at(g.rsp, 224) as *mut __m256i, m[4]); // mova [rsp+224], m4
    std_q4(g.rsp, 256, lo(m[0])); // movd [rsp+256], xm0
    if g.angle & 0x400 != 0 {
        // test angled, 0x400; jnz .w8_main
        return ipred_z2_16bpc_w8_main(m, g);
    }
    m[6] = rbd_q4(pw_3, 0); // vpbroadcastd m6, [base+pw_3]
    m[0] = ld_q4(tl, 34); // movu m0, [tlq+34] ; 2 3 4 5 6 7 8 9   a b c d e f g h
    m[2] = _mm256_add_epi16(m[6], ld_q4(tl, 30)); // paddw m2, m6, [tlq+30] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
    m[5] = _mm256_add_epi16(m[0], ld_q4(tl, 32)); // paddw m5, m0, [tlq+32] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
    m[2] = _mm256_avg_epu16(m[2], ld_q4(tl, 38)); // pavgw m2, [tlq+38] ; 4 5 6 7 8 9 a b   c d e f g h h h
    m[5] = _mm256_add_epi16(m[5], ld_q4(tl, 36)); // paddw m5, [tlq+36] ; 3 4 5 6 7 8 9 a   b c d e f g h h
    m[4] = ld_q4(tl, 66); // movu m4, [tlq+66]
    m[3] = _mm256_add_epi16(m[6], ld_q4(tl, 62)); // paddw m3, m6, [tlq+62]
    m[7] = _mm256_add_epi16(m[4], ld_q4(tl, 64)); // paddw m7, m4, [tlq+64]
    m[3] = _mm256_avg_epu16(m[3], ld_q4(tl, 70)); // pavgw m3, [tlq+70]
    m[7] = _mm256_add_epi16(m[7], ld_q4(tl, 68)); // paddw m7, [tlq+68]
    m[2] = _mm256_add_epi16(m[2], m[5]); // paddw m2, m5
    m[5] = _mm256_set1_epi32(g.max_width); // vpbroadcastd m5, r6m ; max_width
    let r3: isize = 96; // mov r3d, 96
    m[5] = _mm256_packs_epi32(m[5], m[5]); // packssdw m5, m5
    m[3] = _mm256_add_epi16(m[3], m[7]); // paddw m3, m7
    m[5] = _mm256_sub_epi16(m[5], rd_q4(pw_1to16, 0)); // psubw m5, [base+pw_1to16]
    m[2] = _mm256_srli_epi16::<2>(m[2]); // psrlw m2, 2
    m[7] = rbd_q4(z2_ymul8, 20); // vpbroadcastd m7, [base+pw_16]
    m[3] = _mm256_srli_epi16::<2>(m[3]); // psrlw m3, 2
    m[8] = _mm256_min_epi16(m[11], m[5]); // pminsw m8, m11, m5
    m[9] = _mm256_sub_epi16(m[8], m[7]); // psubw m9, m8, m7
    m[2] = _mm256_blendv_epi8(m[2], m[0], m[9]); // vpblendvb m2, m0, m9
    m[9] = _mm256_sub_epi16(m[9], m[7]); // psubw m9, m7
    m[3] = _mm256_blendv_epi8(m[3], m[4], m[9]); // vpblendvb m3, m4, m9
    m[9] = _mm256_sub_epi16(m[9], m[7]); // psubw m9, m7
    st_q4(g.rsp, 162, m[2]); // movu [rsp+162], m2
    st_q4(g.rsp, 194, m[3]); // movu [rsp+194], m3
    ipred_z2_16bpc_w32_filter_above(m, g, r3) // jmp .w32_filter_above
}

// ---- part 5: `ipred16_avx2.asm` lines 2684–3723 (z3) ----

// Memory operands of part 5. Caller memory (the edge, `dst`) is reached through raw pointers with unaligned
// accesses of exactly the assembly's bytes; the frames are this part's locals.

/// `movu/mova xmm, [p+off]`.
#[inline(always)]
unsafe fn lx_q5(p: *const u8, off: isize) -> __m128i {
    _mm_loadu_si128(p.wrapping_offset(off) as *const __m128i)
}

/// `movu/mova ymm, [p+off]`.
#[inline(always)]
unsafe fn ly_q5(p: *const u8, off: isize) -> __m256i {
    _mm256_loadu_si256(p.wrapping_offset(off) as *const __m256i)
}

/// `movu ymm, [p+idx*2+off]`: a word index scaled by the addressing mode, as in `[tlq+r4*2+2]` (an `i16`-typed
/// offset keeps LLVM from rewriting `sar r, 6` + scale 2 into `sar r, 5` + `and r, -2`).
#[inline(always)]
unsafe fn lyw_q5(p: *const u8, idx: isize, off: isize) -> __m256i {
    _mm256_loadu_si256((p.wrapping_offset(off) as *const u16).wrapping_offset(idx) as *const __m256i)
}

/// `movu xmm, [p+idx*2+off]` (see `lyw_q5`).
#[inline(always)]
unsafe fn lxw_q5(p: *const u8, idx: isize, off: isize) -> __m128i {
    _mm_loadu_si128((p.wrapping_offset(off) as *const u16).wrapping_offset(idx) as *const __m128i)
}

/// `movu/mova [p+off], xmm`.
#[inline(always)]
unsafe fn sx_q5(p: *mut u8, off: isize, x: __m128i) {
    _mm_storeu_si128(at(p, off) as *mut __m128i, x)
}

/// `movu/mova [p+off], ymm`.
#[inline(always)]
unsafe fn sy_q5(p: *mut u8, off: isize, x: __m256i) {
    _mm256_storeu_si256(at(p, off) as *mut __m256i, x)
}

/// `movq [p+off], xmm`.
#[inline(always)]
unsafe fn movq_q5(p: *mut u8, off: isize, x: __m128i) {
    _mm_storel_epi64(at(p, off) as *mut __m128i, x)
}

/// `movhps [p+off], xmm`: the high 8 bytes, stored with `pextrq [p+off], xmm, 1` (one instruction, the same bytes;
/// LLVM lowers `_mm_storeh_pd` of integer data to `pshufd` + `movq`).
#[inline(always)]
unsafe fn movhps_q5(p: *mut u8, off: isize, x: __m128i) {
    core::ptr::write_unaligned(at(p, off) as *mut i64, _mm_extract_epi64::<1>(x))
}

/// `movzx r32, word [p+off]`.
#[inline(always)]
unsafe fn ldw_q5(p: *const u8, off: isize) -> i32 {
    core::ptr::read_unaligned(p.wrapping_offset(off) as *const u16) as i32
}

/// `mov [p+off], r16`.
#[inline(always)]
unsafe fn stw_q5(p: *mut u8, off: isize, v: i32) {
    core::ptr::write_unaligned(at(p, off) as *mut u16, v as u16)
}

/// `movd [p+off], xmm`.
#[inline(always)]
unsafe fn movd_st_q5(p: *mut u8, off: isize, x: __m128i) {
    core::ptr::write_unaligned(at(p, off) as *mut i32, _mm_cvtsi128_si32(x))
}

/// `vpbroadcastw ymm, [p+off]`.
#[inline(always)]
unsafe fn bcw_q5(p: *const u8, off: isize) -> __m256i {
    _mm256_set1_epi16(core::ptr::read_unaligned(p.wrapping_offset(off) as *const i16))
}

/// `vpbroadcastw ymm, [p+idx*2]`: a word index scaled by the addressing mode (see `lyw_q5`; with a byte offset
/// `idx*2` of a negated index LLVM subtracts the doubled positive value in extra instructions).
#[inline(always)]
unsafe fn bcww_q5(p: *const u8, idx: isize) -> __m256i {
    _mm256_set1_epi16(core::ptr::read_unaligned((p as *const i16).wrapping_offset(idx)))
}

/// `vpbroadcastd ymm, [p]` (read-only data).
#[inline(always)]
unsafe fn bcd_q5(p: *const u8) -> __m256i {
    _mm256_set1_epi32(core::ptr::read_unaligned(p as *const i32))
}

/// An `ALLOC_STACK` frame of a z3 path: `N` bytes, zeroed whole where a path first uses it (`zero_frame_q5`). The
/// assembly's `rsp` is `G` bytes above its start (`G` a guard for the loads below `rsp` that the assembly makes and
/// whose lanes it discards; the bounds are given at each path). 16-byte aligned, not 32 as `ALLOC_STACK` aligns it:
/// every access to it is an unaligned load or store (the same bytes), and a 32-byte alignment made LLVM realign the
/// stack with a frame pointer (`push rbp`, `mov rbp, rsp`, `and rsp`, `lea rsp, [rbp-…]`, `pop rbp`) in every
/// height function, where nothing else needs it.
#[repr(C, align(16))]
struct FrameQ5<const N: usize>([u8; N]);

/// Zeroes an `ALLOC_STACK` frame whole. Called at the start of each path that uses the frame (the upsampling and
/// the edge filter write into it and point `tlq` into it); the unfiltered path reads the caller's edge and never forms
/// a pointer into the frame, so it does not pay for the stores.
#[inline(always)]
unsafe fn zero_frame_q5<const N: usize>(fp: *mut FrameQ5<N>) {
    fp.write_bytes(0, 1)
}

/// The 32-byte slots that `.h32_loop`/`.h64_loop` push below the frame (`sub rsp, 64`/`128` per column) and the
/// transposes read back.
#[repr(C, align(32))]
struct ColsQ5<const N: usize>(core::mem::MaybeUninit<[__m256i; N]>);

#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_z3_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
) {
    // cglobal ipred_z3_16bpc, 4, 9, 0, dst, stride, tl, w, h, angle, dy, org_w, maxbase
    // lea r6, [ipred_z3_16bpc_avx2_table]
    let h = (height as u32).trailing_zeros(); // tzcnt hd, hm (h is a power of two >= 4, never zero)
                                              // The rest of the prologue (`sub tlq, 2` up to `vpbroadcastd m5, [pw_62]`) runs before the jump, as in the
                                              // assembly; `mov org_wd, wd` is the argument `org_w` of h32 and h64.
    let m = &mut regs();
    let (tl, angle, dy) = ipred_z3_16bpc_entry_q5(m, topleft, angle);
    let dst = dst as *mut u8;
    // movsxd hq, [r6+hq*4] ; add hq, r6 ; jmp hq
    match h {
        2 => ipred_z3_16bpc_h4(m, dst, stride, tl, width, angle, dy, bitdepth_max),
        3 => ipred_z3_16bpc_h8(m, dst, stride, tl, width, angle, dy, bitdepth_max),
        4 => ipred_z3_16bpc_h16(m, dst, stride, tl, width, angle, dy),
        5 => ipred_z3_16bpc_h32(m, dst, stride, tl, width, angle, dy, width),
        _ => ipred_z3_16bpc_h64(m, dst, stride, tl, width, angle, dy, width), // 6: heights are 4..=64
    }
}

/// The entry's prologue from `movifnidn` to `vpbroadcastd m5, [pw_62]` (before the jump table): returns `tlq`,
/// `angled`, `dyd`.
#[inline(always)]
unsafe fn ipred_z3_16bpc_entry_q5(m: &mut M, topleft: *const u16, angle: c_int) -> (*const u8, i32, i32) {
    let mut angle = angle; // movifnidn angled, anglem
    let r7 =
        (core::ptr::addr_of!(crate::common::tables::dav1d_dr_intra_derivative) as *const u8).wrapping_add(45 * 2 - 1); // lea r7, [dr_intra_derivative+45*2-1]
    let tl = (topleft as *const u8).wrapping_offset(-2); // sub tlq, 2
    angle -= 180; // sub angled, 180
    let mut dy = angle; // mov dyd, angled
    dy = dy.wrapping_neg(); // neg dyd
    angle ^= 0x400; // xor angled, 0x400
    let dyq = (dy as u32 as u64 | !0x7e) as isize; // or dyq, ~0x7e (bits 1..6 of -d: dr_intra_derivative[(90-d)>>1])
    let dy = ldw_q5(r7, dyq); // movzx dyd, word [r7+dyq]
    m[5] = bcd_q5(ro(pw_62, 0)); // vpbroadcastd m5, [pw_62]
    (tl, angle, dy)
}

/// `.filter_strength` (h4/h8/h16; `call`ed): the mask whose population count is the edge filter's strength.
/// `angle` (flag 0x400 clear here) becomes `is_sm << 1` (`shr angled, 8`).
#[inline(always)]
unsafe fn ipred_z3_16bpc_filter_strength(m: &mut M, maxbase: i32, angle: &mut i32) -> u32 {
    // lea r4, [z_filter_t0] (%define base r4-z_filter_t0)
    m[0] = zx(_mm_cvtsi32_si128(maxbase)); // movd xm0, maxbased
    m[1] = zx(_mm_cvtsi32_si128(*angle)); // movd xm1, angled
    *angle = ((*angle as u32) >> 8) as i32; // shr angled, 8 ; is_sm << 1
    m[0] = _mm256_broadcastb_epi8(lo(m[0])); // vpbroadcastb m0, xm0
    m[1] = _mm256_broadcastb_epi8(lo(m[1])); // vpbroadcastb m1, xm1
    m[0] = _mm256_cmpeq_epi8(m[0], ly_q5(ro(z_filter_wh, 0), 0)); // pcmpeqb m0, [base+z_filter_wh]
    m[0] = _mm256_and_si256(m[0], m[1]); // pand m0, m1
    m[1] = zx(lx_q5(ro(z_filter_t0, 0), *angle as isize * 8)); // mova xm1, [r4+angleq*8] (angle 0 or 2: t0 or t1)
    m[0] = _mm256_cmpgt_epi8(m[0], m[1]); // pcmpgtb m0, m1
    _mm256_movemask_epi8(m[0]) as u32 // pmovmskb r5d, m0
}

/// `vpbroadcastd [base+z_filter_k-4+r5*4+12*k]` (r5 = strength 1..=3: inside `z_filter_k`).
#[inline(always)]
unsafe fn ipred_z3_16bpc_filter_k_q5(r5: u32, k: isize) -> __m256i {
    bcd_q5(ro(z_filter_k, -4 + r5 as isize * 4 + 12 * k))
}

/// `.h4`.
#[inline(always)]
unsafe fn ipred_z3_16bpc_h4(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    mut w: i32,
    mut angle: i32,
    mut dy: i32,
    pixel_max: i32,
) {
    // ALLOC_STACK -64, 7: zeroed whole, with 128 bytes below rsp: the first group of `.h4_loop` reads down to
    // rsp-104 (four columns, base up to (4*1023)>>6 = 63, 16 bytes at tlq+2*base with tlq = rsp+22).
    // Zeroed whole by `zero_frame_q5` where a path first uses it (not on the unfiltered path, which never points
    // into it).
    let mut frame = core::mem::MaybeUninit::<FrameQ5<{ 128 + 64 }>>::uninit();
    let fp = frame.as_mut_ptr();
    let rsp = (fp as *mut u8).wrapping_add(128);
    let r7 = stride * 3; // lea r7, [strideq*3]
    let mut maxbase: i32;
    'no_upsample: {
        if angle as u8 >= 40 {
            break 'no_upsample; // cmp angleb, 40 ; jae .h4_no_upsample
        }
        let mut r4 = angle - 1024; // lea r4d, [angleq-1024]
        r4 >>= 7; // sar r4d, 7
        r4 += w; // add r4d, wd
        if r4 > 0 {
            break 'no_upsample; // jg .h4_no_upsample ; !enable_intra_edge_filter || w > 8 || (w == 8 && is_sm)
        }
        zero_frame_q5(fp); // the frame, first used on this path
        m[2] = zx(lx_q5(tl, -14)); // mova xm2, [tlq-14] ; 0 1 2 3 4 5 6 7
        m[1] = zx(_mm_blend_epi16::<0xfe>(lo(m[2]), lx_q5(tl, -16))); // pblendw xm1, xm2, [tlq-16], 0xfe ; 0 0 1 2 3 4 5 6
        m[0] = zx(_mm_blend_epi32::<0x0e>(lo(m[1]), lx_q5(tl, -18))); // vpblendd xm0, xm1, [tlq-18], 0x0e ; 0 0 0 1 2 3 4 5
        m[3] = zx(_mm_shuffle_epi32::<0x00>(lo(m[1]))); // pshufd xm3, xm1, q0000
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
        m[0] = zx(_mm_add_epi16(lo(m[0]), lx_q5(tl, -12))); // paddw xm0, [tlq-12] ; 1 2 3 4 5 6 7 8
        m[4] = zx(_mm_set1_epi16(pixel_max as i16)); // vpbroadcastw xm4, r8m ; pixel_max
        dy += dy; // add dyd, dyd
        m[0] = zx(_mm_sub_epi16(lo(m[1]), lo(m[0]))); // psubw xm0, xm1, xm0
        sx_q5(rsp, 0, lo(m[3])); // mova [rsp+ 0], xm3
        m[3] = zx(_mm_cvtsi32_si128(dy)); // movd xm3, dyd
        m[0] = zx(_mm_srai_epi16::<3>(lo(m[0]))); // psraw xm0, 3
        dy = dy.wrapping_neg(); // neg dyd
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[0]))); // paddw xm1, xm0
        m[0] = _mm256_setzero_si256(); // pxor xm0, xm0
        let mut r2 = dy + (16 << 6) + 63; // lea r2d, [dyq+(16<<6)+63] ; ypos
        m[1] = zx(_mm_max_epi16(lo(m[1]), lo(m[0]))); // pmaxsw xm1, xm0
        m[1] = zx(_mm_avg_epu16(lo(m[1]), lo(m[0]))); // pavgw xm1, xm0
        m[3] = _mm256_broadcastw_epi16(lo(m[3])); // vpbroadcastw m3, xm3
        m[1] = zx(_mm_min_epi16(lo(m[1]), lo(m[4]))); // pminsw xm1, xm4
        m[0] = zx(_mm_unpackhi_epi16(lo(m[1]), lo(m[2]))); // punpckhwd xm0, xm1, xm2
        m[1] = zx(_mm_unpacklo_epi16(lo(m[1]), lo(m[2]))); // punpcklwd xm1, xm2
        m[2] = _mm256_add_epi16(m[3], m[3]); // paddw m2, m3, m3
        sx_q5(rsp, 32, lo(m[0])); // mova [rsp+32], xm0
        m[3] = _mm256_unpacklo_epi16(m[3], m[2]); // punpcklwd m3, m2
        sx_q5(rsp, 16, lo(m[1])); // mova [rsp+16], xm1
        m[4] = _mm256_add_epi16(m[2], m[2]); // paddw m4, m2, m2
        m[2] = _mm256_add_epi16(m[2], m[3]); // paddw m2, m3
        m[3] = _mm256_blend_epi32::<0xf0>(m[3], m[2]); // vpblendd m3, m2, 0xf0 ; ypos0 ypos1   ypos2 ypos3
        loop {
            // .h4_upsample_loop: the bases are 4..16 (r2 <= 1087, w <= 8 columns of 2*dy <= 102): rsp+8..rsp+48
            let mut r4 = r2 + dy; // lea r4d, [r2+dyq]
            r2 = ((r2 as u32) >> 6) as i32; // shr r2d, 6
            m[1] = zx(lxw_q5(rsp, r2 as isize, 0)); // movu xm1, [rsp+r2*2]
            r2 = r4 + dy; // lea r2d, [r4+dyq]
            r4 = ((r4 as u32) >> 6) as i32; // shr r4d, 6
            m[2] = zx(lxw_q5(rsp, r4 as isize, 0)); // movu xm2, [rsp+r4*2]
            r4 = r2 + dy; // lea r4d, [r2+dyq]
            r2 = ((r2 as u32) >> 6) as i32; // shr r2d, 6
            m[1] = _mm256_inserti128_si256::<1>(m[1], lxw_q5(rsp, r2 as isize, 0)); // vinserti128 m1, [rsp+r2*2], 1
            r2 = r4 + dy; // lea r2d, [r4+dyq]
            r4 = ((r4 as u32) >> 6) as i32; // shr r4d, 6
            m[2] = _mm256_inserti128_si256::<1>(m[2], lxw_q5(rsp, r4 as isize, 0)); // vinserti128 m2, [rsp+r4*2], 1
            m[0] = _mm256_srli_epi32::<16>(m[1]); // psrld m0, m1, 16
            m[0] = _mm256_blend_epi16::<0xaa>(m[0], m[2]); // pblendw m0, m2, 0xaa
            m[2] = _mm256_slli_epi32::<16>(m[2]); // pslld m2, 16
            m[1] = _mm256_blend_epi16::<0xaa>(m[1], m[2]); // pblendw m1, m2, 0xaa
            m[2] = _mm256_and_si256(m[5], m[3]); // pand m2, m5, m3
            m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
            m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
            m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
            m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4
            m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
            m[2] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm2, m1, 1
            m[0] = zx(_mm_unpackhi_epi32(lo(m[1]), lo(m[2]))); // punpckhdq xm0, xm1, xm2 ; a1 b1 c1 d1 a0 b0 c0 d0
            m[1] = zx(_mm_unpacklo_epi32(lo(m[1]), lo(m[2]))); // punpckldq xm1, xm2 ; a3 b3 c3 d3 a2 b2 c2 d2
            movhps_q5(dst, 0, lo(m[0])); // movhps [dstq+strideq*0], xm0
            movq_q5(dst, stride, lo(m[0])); // movq [dstq+strideq*1], xm0
            movhps_q5(dst, stride * 2, lo(m[1])); // movhps [dstq+strideq*2], xm1
            movq_q5(dst, r7, lo(m[1])); // movq [dstq+r7], xm1
            dst = at(dst, 8); // add dstq, 8
            w -= 4; // sub wd, 4
            if w <= 0 {
                break; // jg .h4_upsample_loop
            }
        }
        return; // RET
    }
    // .h4_no_upsample:
    maxbase = 7; // mov maxbased, 7
    'main: {
        if angle & 0x400 != 0 {
            break 'main; // test angled, 0x400 ; jnz .h4_main ; !enable_intra_edge_filter
        }
        maxbase = w + 3; // lea maxbased, [wq+3]
        let mut r5 = ipred_z3_16bpc_filter_strength(m, maxbase, &mut angle); // call .filter_strength
        maxbase = 7; // mov maxbased, 7
        if r5 == 0 {
            break 'main; // test r5d, r5d ; jz .h4_main ; filter_strength == 0
        }
        zero_frame_q5(fp); // the frame, first used on this path
        r5 = r5.count_ones(); // popcnt r5d, r5d
        m[0] = zx(lx_q5(tl, -14)); // mova xm0, [tlq-14] ; 0 1 2 3 4 5 6 7
        m[3] = zx(lx_q5(tl, -12)); // movu xm3, [tlq-12] ; 1 2 3 4 5 6 7 8
        m[2] = zx(lo(ipred_z3_16bpc_filter_k_q5(r5, 1))); // vpbroadcastd xm2, [base+z_filter_k-4+r5*4+12*1]
        m[4] = zx(lo(ipred_z3_16bpc_filter_k_q5(r5, 0))); // vpbroadcastd xm4, [base+z_filter_k-4+r5*4+12*0]
        m[2] = zx(_mm_mullo_epi16(lo(m[2]), lo(m[0]))); // pmullw xm2, xm0
        m[0] = zx(_mm_blend_epi16::<0xfe>(lo(m[0]), lx_q5(tl, -16))); // pblendw xm0, [tlq-16], 0xfe ; 0 0 1 2 3 4 5 6
        m[1] = zx(_mm_add_epi16(lo(m[0]), lo(m[3]))); // paddw xm1, xm0, xm3
        movd_st_q5(rsp, 12, lo(m[0])); // movd [rsp+12], xm0
        m[1] = zx(_mm_mullo_epi16(lo(m[1]), lo(m[4]))); // pmullw xm1, xm4
        if r5 == 3 {
            // cmp r5d, 3 ; jne .h4_filter_3tap
            m[3] = zx(_mm_blend_epi16::<0x7f>(lo(m[3]), lx_q5(tl, -10))); // pblendw xm3, [tlq-10], 0x7f ; 2 3 4 5 6 7 8 8
            m[0] = zx(_mm_blend_epi32::<0x0e>(lo(m[0]), lx_q5(tl, -18))); // vpblendd xm0, [tlq-18], 0x0e ; 0 0 0 1 2 3 4 5
            let r4 = ldw_q5(tl, -14); // movzx r4d, word [tlq-14]
            let mut r2 = ldw_q5(tl, -12); // movzx r2d, word [tlq-12]
            maxbase += 1; // inc maxbased
            m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
            m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[3]))); // paddw xm0, xm3
            r2 -= r4; // sub r2d, r4d
            m[2] = zx(_mm_add_epi16(lo(m[0]), lo(m[0]))); // paddw xm2, xm0, xm0
            r2 = r2 + r4 * 8 + 4; // lea r2d, [r2+r4*8+4]
            r2 = ((r2 as u32) >> 3) as i32; // shr r2d, 3
            stw_q5(rsp, 14, r2); // mov [rsp+14], r2w
        }
        // .h4_filter_3tap:
        m[0] = _mm256_setzero_si256(); // pxor xm0, xm0
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
        tl = rsp.wrapping_add(30); // lea tlq, [rsp+30]
        m[1] = zx(_mm_srli_epi16::<3>(lo(m[1]))); // psrlw xm1, 3
        maxbase += (w as u32 >= 8) as i32; // cmp wd, 8 ; sbb maxbased, -1
        m[0] = zx(_mm_avg_epu16(lo(m[0]), lo(m[1]))); // pavgw xm0, xm1
        sx_q5(rsp, 16, lo(m[0])); // mova [rsp+16], xm0
    }
    // .h4_main:
    m[3] = zx(_mm_cvtsi32_si128(dy)); // movd xm3, dyd
    let mut maxbaseq = (maxbase as u32 as isize).wrapping_neg(); // neg maxbaseq (maxbased as a 32-bit write left it)
    m[1] = _mm256_broadcastsi128_si256(lx_q5(ro(z_base_inc, 0), 0)); // vbroadcasti128 m1, [z_base_inc]
    m[6] = bcww_q5(tl, maxbaseq); // vpbroadcastw m6, [tlq+maxbaseq*2]
    maxbaseq <<= 6; // shl maxbased, 6 (64-bit here, one negated value for the address and the shift: the same low
                    // 32 bits, and every later use reads only those)
    m[3] = _mm256_broadcastw_epi16(lo(m[3])); // vpbroadcastw m3, xm3
    let mut r4 = (maxbaseq as i32).wrapping_add(3 * 64) as isize; // lea r4d, [maxbaseq+3*64]
    let dyq = -(dy as isize); // neg dyq
    m[2] = zx(_mm_cvtsi32_si128(r4 as i32)); // movd xm2, r4d
    tl = tl.wrapping_offset(-8); // sub tlq, 8
    r4 = dyq + 63; // lea r4, [dyq+63] ; ypos
    m[1] = _mm256_unpacklo_epi16(m[1], m[1]); // punpcklwd m1, m1
    m[0] = _mm256_add_epi16(m[3], m[3]); // paddw m0, m3, m3
    m[2] = _mm256_broadcastw_epi16(lo(m[2])); // vpbroadcastw m2, xm2
    m[3] = _mm256_unpacklo_epi16(m[3], m[0]); // punpcklwd m3, m0
    m[4] = _mm256_add_epi16(m[0], m[0]); // paddw m4, m0, m0
    m[0] = _mm256_add_epi16(m[0], m[3]); // paddw m0, m3
    m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1
    m[3] = _mm256_blend_epi32::<0xf0>(m[3], m[0]); // vpblendd m3, m0, 0xf0 ; ypos0 ypos1   ypos2 ypos3
    let maxbased = (maxbaseq as i32) | 63; // or maxbased, 63
    m[3] = _mm256_add_epi16(m[3], m[2]); // paddw m3, m2
    loop {
        // .h4_loop:
        let mut r5 = r4 + dyq; // lea r5, [r4+dyq]
        r4 >>= 6; // sar r4, 6 ; base0
        m[1] = zx(lxw_q5(tl, r4, 0)); // movu xm1, [tlq+r4*2]
        r4 = r5 + dyq; // lea r4, [r5+dyq]
        r5 >>= 6; // sar r5, 6 ; base1
        m[2] = zx(lxw_q5(tl, r5, 0)); // movu xm2, [tlq+r5*2]
        r5 = r4 + dyq; // lea r5, [r4+dyq]
        r4 >>= 6; // sar r4, 6 ; base2
        m[1] = _mm256_inserti128_si256::<1>(m[1], lxw_q5(tl, r4, 0)); // vinserti128 m1, [tlq+r4*2], 1
        r4 = r5 + dyq; // lea r4, [r5+dyq]
        r5 >>= 6; // sar r5, 6 ; base3
        m[2] = _mm256_inserti128_si256::<1>(m[2], lxw_q5(tl, r5, 0)); // vinserti128 m2, [tlq+r5*2], 1
        m[0] = _mm256_unpackhi_epi16(m[1], m[2]); // punpckhwd m0, m1, m2
        m[1] = _mm256_unpacklo_epi16(m[1], m[2]); // punpcklwd m1, m2
        m[2] = _mm256_and_si256(m[5], m[3]); // pand m2, m5, m3
        m[0] = _mm256_alignr_epi8::<4>(m[0], m[1]); // palignr m0, m1, 4 ; a3 b3 a2 b2 a1 b1 a0 b0   c3 d3 c2 d2 c1 d1 c0 d0
        m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
        m[2] = _mm256_srai_epi16::<15>(m[3]); // psraw m2, m3, 15 ; ypos < max_base_y
        m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4
        m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
        m[1] = _mm256_blendv_epi8(m[6], m[1], m[2]); // vpblendvb m1, m6, m1, m2
        m[2] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm2, m1, 1
        m[0] = zx(_mm_unpackhi_epi32(lo(m[1]), lo(m[2]))); // punpckhdq xm0, xm1, xm2 ; a1 b1 c1 d1 a0 b0 c0 d0
        m[1] = zx(_mm_unpacklo_epi32(lo(m[1]), lo(m[2]))); // punpckldq xm1, xm2 ; a3 b3 c3 d3 a2 b2 c2 d2
        movhps_q5(dst, 0, lo(m[0])); // movhps [dstq+strideq*0], xm0
        movq_q5(dst, stride, lo(m[0])); // movq [dstq+strideq*1], xm0
        movhps_q5(dst, stride * 2, lo(m[1])); // movhps [dstq+strideq*2], xm1
        movq_q5(dst, r7, lo(m[1])); // movq [dstq+r7], xm1
        w -= 4; // sub wd, 4
        if w == 0 {
            return; // jz .h4_end
        }
        dst = at(dst, 8); // add dstq, 8
        if r4 as i32 <= maxbased {
            break; // cmp r4d, maxbased ; jg .h4_loop
        }
    }
    loop {
        // .h4_end_loop:
        movq_q5(dst, 0, lo(m[6])); // movq [dstq+strideq*0], xm6
        movq_q5(dst, stride, lo(m[6])); // movq [dstq+strideq*1], xm6
        movq_q5(dst, stride * 2, lo(m[6])); // movq [dstq+strideq*2], xm6
        movq_q5(dst, r7, lo(m[6])); // movq [dstq+r7], xm6
        dst = at(dst, 8); // add dstq, 8
        w -= 4; // sub wd, 4
        if w <= 0 {
            break; // jg .h4_end_loop
        }
    }
    // .h4_end: RET
}

/// `.h8`.
#[inline(always)]
unsafe fn ipred_z3_16bpc_h8(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    mut w: i32,
    mut angle: i32,
    mut dy: i32,
    pixel_max: i32,
) {
    let mut r4 = angle + 216; // lea r4d, [angleq+216]
                              // ALLOC_STACK -64, 8: zeroed whole, with 96 bytes below rsp: the first group of `.h8_loop` reads down to
                              // rsp-80 (base up to 63, 18 bytes at tlq+2*base with tlq = rsp+46).
                              // Zeroed whole by `zero_frame_q5` where a path first uses it (not on the unfiltered path, which never points
                              // into it).
    let mut frame = core::mem::MaybeUninit::<FrameQ5<{ 96 + 64 }>>::uninit();
    let fp = frame.as_mut_ptr();
    let rsp = (fp as *mut u8).wrapping_add(96);
    r4 = (r4 & !0xff) | (w & 0xff); // mov r4b, wb
    let r7 = stride * 3; // lea r7, [strideq*3]
    if r4 as u32 <= 8 {
        // cmp r4d, 8 ; ja .h8_no_upsample ; !enable_intra_edge_filter || is_sm || d >= 40 || w > 8
        zero_frame_q5(fp); // the frame, first used on this path
        m[2] = ly_q5(tl, -30); // mova m2, [tlq-30] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
        m[1] = _mm256_add_epi16(m[2], ly_q5(tl, -32)); // paddw m1, m2, [tlq-32] ; _ 0 1 2 3 4 5 6   7 8 9 a b c d e
        m[0] = ly_q5(tl, -34); // movu m0, [tlq-34] ; _ _ 0 1 2 3 4 5   6 7 8 9 a b c d
        if w != 8 {
            // cmp wd, 8 ; je .h8_upsample_w8
            m[3] = zx(_mm_shufflehi_epi16::<0x40>(lo(m[2]))); // pshufhw xm3, xm2, q1000
            m[0] = _mm256_blend_epi32::<0x0f>(m[0], m[3]); // vpblendd m0, m3, 0x0f ; _ _ _ _ 4 4 4 5   6 7 8 9 a b c d
        }
        // .h8_upsample_w8:
        m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -28)); // paddw m0, [tlq-28] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[4] = _mm256_set1_epi16(pixel_max as i16); // vpbroadcastw m4, r8m ; pixel_max
        dy += dy; // add dyd, dyd
        m[0] = _mm256_sub_epi16(m[1], m[0]); // psubw m0, m1, m0
        m[6] = zx(_mm_cvtsi32_si128(dy)); // movd xm6, dyd
        m[0] = _mm256_srai_epi16::<3>(m[0]); // psraw m0, 3
        dy = dy.wrapping_neg(); // neg dyd
        m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
        m[0] = _mm256_setzero_si256(); // pxor m0, m0
        m[1] = _mm256_max_epi16(m[1], m[0]); // pmaxsw m1, m0
        let mut r4 = dy + (16 << 6) + 63; // lea r4d, [dyq+(16<<6)+63] ; ypos
        m[1] = _mm256_avg_epu16(m[1], m[0]); // pavgw m1, m0
        m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
        m[1] = _mm256_min_epi16(m[1], m[4]); // pminsw m1, m4
        m[0] = _mm256_unpackhi_epi16(m[1], m[2]); // punpckhwd m0, m1, m2
        m[1] = _mm256_unpacklo_epi16(m[1], m[2]); // punpcklwd m1, m2
        sx_q5(rsp, 48, _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [rsp+48], m0, 1
        sx_q5(rsp, 32, _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [rsp+32], m1, 1
        m[7] = _mm256_add_epi16(m[6], m[6]); // paddw m7, m6, m6
        sx_q5(rsp, 16, lo(m[0])); // mova [rsp+16], xm0
        sx_q5(rsp, 0, lo(m[1])); // mova [rsp+ 0], xm1
        m[6] = _mm256_unpacklo_epi16(m[6], m[7]); // punpcklwd m6, m7 ; ypos0 ypos1
        loop {
            // .h8_upsample_loop: the bases are 4..16 (as at h4): rsp+8..rsp+64
            let mut r2 = r4 + dy; // lea r2d, [r4+dyq]
            r4 = ((r4 as u32) >> 6) as i32; // shr r4d, 6 ; base0
            m[1] = lyw_q5(rsp, r4 as isize, 0); // movu m1, [rsp+r4*2]
            r4 = r2 + dy; // lea r4d, [r2+dyq]
            r2 = ((r2 as u32) >> 6) as i32; // shr r2d, 6 ; base1
            m[2] = lyw_q5(rsp, r2 as isize, 0); // movu m2, [rsp+r2*2]
            r2 = r4 + dy; // lea r2d, [r4+dyq]
            r4 = ((r4 as u32) >> 6) as i32; // shr r4d, 6 ; base2
            m[3] = lyw_q5(rsp, r4 as isize, 0); // movu m3, [rsp+r4*2]
            r4 = r2 + dy; // lea r4d, [r2+dyq]
            r2 = ((r2 as u32) >> 6) as i32; // shr r2d, 6 ; base3
            m[4] = lyw_q5(rsp, r2 as isize, 0); // movu m4, [rsp+r2*2]
            m[0] = _mm256_srli_epi32::<16>(m[1]); // psrld m0, m1, 16
            m[0] = _mm256_blend_epi16::<0xaa>(m[0], m[2]); // pblendw m0, m2, 0xaa
            m[2] = _mm256_slli_epi32::<16>(m[2]); // pslld m2, 16
            m[1] = _mm256_blend_epi16::<0xaa>(m[1], m[2]); // pblendw m1, m2, 0xaa
            m[2] = _mm256_srli_epi32::<16>(m[3]); // psrld m2, m3, 16
            m[2] = _mm256_blend_epi16::<0xaa>(m[2], m[4]); // pblendw m2, m4, 0xaa
            m[4] = _mm256_slli_epi32::<16>(m[4]); // pslld m4, 16
            m[3] = _mm256_blend_epi16::<0xaa>(m[3], m[4]); // pblendw m3, m4, 0xaa
            m[4] = _mm256_and_si256(m[5], m[6]); // pand m4, m5, m6
            m[6] = _mm256_add_epi16(m[6], m[7]); // paddw m6, m7
            m[4] = _mm256_slli_epi16::<9>(m[4]); // psllw m4, 9
            m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
            m[1] = _mm256_mulhrs_epi16(m[1], m[4]); // pmulhrsw m1, m4
            m[4] = _mm256_and_si256(m[5], m[6]); // pand m4, m5, m6
            m[4] = _mm256_slli_epi16::<9>(m[4]); // psllw m4, 9
            m[3] = _mm256_sub_epi16(m[3], m[2]); // psubw m3, m2
            m[3] = _mm256_mulhrs_epi16(m[3], m[4]); // pmulhrsw m3, m4
            m[6] = _mm256_add_epi16(m[6], m[7]); // paddw m6, m7
            let r2p = at(dst, stride * 4); // lea r2, [dstq+strideq*4]
            m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
            m[3] = _mm256_add_epi16(m[3], m[2]); // paddw m3, m2
            m[0] = _mm256_unpackhi_epi32(m[1], m[3]); // punpckhdq m0, m1, m3
            m[1] = _mm256_unpacklo_epi32(m[1], m[3]); // punpckldq m1, m3
            m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1
            m[3] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm3, m1, 1
            movhps_q5(r2p, 0, lo(m[0])); // movhps [r2+strideq*0], xm0
            movq_q5(r2p, stride, lo(m[0])); // movq [r2+strideq*1], xm0
            movhps_q5(r2p, stride * 2, lo(m[1])); // movhps [r2+strideq*2], xm1
            movq_q5(r2p, r7, lo(m[1])); // movq [r2+r7], xm1
            movhps_q5(dst, 0, lo(m[2])); // movhps [dstq+strideq*0], xm2
            movq_q5(dst, stride, lo(m[2])); // movq [dstq+strideq*1], xm2
            movhps_q5(dst, stride * 2, lo(m[3])); // movhps [dstq+strideq*2], xm3
            movq_q5(dst, r7, lo(m[3])); // movq [dstq+r7], xm3
            dst = at(dst, 8); // add dstq, 8
            w -= 4; // sub wd, 4
            if w <= 0 {
                break; // jg .h8_upsample_loop
            }
        }
        return; // RET
    }
    // .h8_no_upsample:
    let mut maxbase = w + 7; // lea maxbased, [wq+7]
    'main: {
        if angle & 0x400 != 0 {
            // test angled, 0x400 ; jnz .h8_no_intra_edge_filter
            // .h8_no_intra_edge_filter:
            maxbase &= 7; // and maxbased, 7
            maxbase |= 8; // or maxbased, 8 ; imin(w+7, 15)
            break 'main; // jmp .h8_main
        }
        let mut r5 = ipred_z3_16bpc_filter_strength(m, maxbase, &mut angle); // call .filter_strength
        if r5 == 0 {
            break 'main; // test r5d, r5d ; jz .h8_main
        }
        zero_frame_q5(fp); // the frame, first used on this path
        r5 = r5.count_ones(); // popcnt r5d, r5d
        m[0] = ly_q5(tl, -30); // mova m0, [tlq-30] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
        m[3] = ly_q5(tl, -28); // movu m3, [tlq-28] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[2] = ipred_z3_16bpc_filter_k_q5(r5, 1); // vpbroadcastd m2, [base+z_filter_k-4+r5*4+12*1]
        m[4] = ipred_z3_16bpc_filter_k_q5(r5, 0); // vpbroadcastd m4, [base+z_filter_k-4+r5*4+12*0]
        m[2] = _mm256_mullo_epi16(m[2], m[0]); // pmullw m2, m0
        if w >= 8 {
            // cmp wd, 8 ; jl .h8_filter_w4
            m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[0]))); // punpcklwd xm0, xm0
            m[1] = _mm256_blend_epi32::<0xfe>(m[0], ly_q5(tl, -32)); // vpblendd m1, m0, [tlq-32], 0xfe ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
            movd_st_q5(rsp, 28, lo(m[0])); // movd [rsp+28], xm0
            m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3
            let r4 = 16; // mov r4d, 16
            m[1] = _mm256_mullo_epi16(m[1], m[4]); // pmullw m1, m4
            if w > 8 {
                maxbase = r4; // cmovg maxbased, r4d
            }
            if r5 == 3 {
                // cmp r5d, 3 ; jne .h8_filter_3tap
                m[3] = _mm256_unpackhi_epi16(m[3], m[3]); // punpckhwd m3, m3
                m[0] = _mm256_blend_epi32::<0xfe>(m[0], ly_q5(tl, -34)); // vpblendd m0, [tlq-34], 0xfe ; 0 0 0 1 2 3 4 5   6 7 8 9 a b c d
                m[3] = _mm256_blend_epi32::<0x7f>(m[3], ly_q5(tl, -26)); // vpblendd m3, [tlq-26], 0x7f ; 2 3 4 5 6 7 8 9   a b c d e f g g
                let r4 = ldw_q5(tl, -30); // movzx r4d, word [tlq-30]
                let mut r2 = ldw_q5(tl, -28); // movzx r2d, word [tlq-28]
                maxbase += 1; // inc maxbased
                m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
                m[0] = _mm256_add_epi16(m[0], m[3]); // paddw m0, m3
                r2 -= r4; // sub r2d, r4d
                m[2] = _mm256_add_epi16(m[0], m[0]); // paddw m2, m0, m0
                r2 = r2 + r4 * 8 + 4; // lea r2d, [r2+r4*8+4]
                r2 = ((r2 as u32) >> 3) as i32; // shr r2d, 3
                stw_q5(rsp, 30, r2); // mov [rsp+30], r2w
            } // jmp .h8_filter_3tap
        } else {
            // .h8_filter_w4:
            m[1] = zx(_mm_shufflehi_epi16::<0x90>(lo(m[0]))); // pshufhw xm1, xm0, q2100
            m[1] = _mm256_inserti128_si256::<1>(m[1], lx_q5(tl, -16)); // vinserti128 m1, [tlq-16], 1 ; _ _ _ _ 4 4 5 6   7 8 9 a b c d e
            m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3
            m[1] = _mm256_mullo_epi16(m[1], m[4]); // pmullw m1, m4
        }
        // .h8_filter_3tap:
        m[0] = _mm256_setzero_si256(); // pxor m0, m0
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        tl = rsp.wrapping_add(62); // lea tlq, [rsp+62]
        m[1] = _mm256_srli_epi16::<3>(m[1]); // psrlw m1, 3
        m[0] = _mm256_avg_epu16(m[0], m[1]); // pavgw m0, m1
        sy_q5(rsp, 32, m[0]); // mova [rsp+32], m0
    }
    // .h8_main:
    m[4] = zx(_mm_cvtsi32_si128(dy)); // movd xm4, dyd
    let mut maxbaseq = (maxbase as u32 as isize).wrapping_neg(); // neg maxbaseq (maxbased as a 32-bit write left it)
    m[1] = _mm256_broadcastsi128_si256(lx_q5(ro(z_base_inc, 0), 0)); // vbroadcasti128 m1, [z_base_inc]
    m[7] = bcww_q5(tl, maxbaseq); // vpbroadcastw m7, [tlq+maxbaseq*2]
    maxbaseq <<= 6; // shl maxbased, 6 (64-bit here, one negated value for the address and the shift: the same low
                    // 32 bits, and every later use reads only those)
    m[4] = _mm256_broadcastw_epi16(lo(m[4])); // vpbroadcastw m4, xm4
    let mut r4 = (maxbaseq as i32).wrapping_add(7 * 64) as isize; // lea r4d, [maxbaseq+7*64]
    let dyq = -(dy as isize); // neg dyq
    m[2] = zx(_mm_cvtsi32_si128(r4 as i32)); // movd xm2, r4d
    tl = tl.wrapping_offset(-16); // sub tlq, 16
    r4 = dyq + 63; // lea r4, [dyq+63]
    m[6] = _mm256_add_epi16(m[4], m[4]); // paddw m6, m4, m4
    m[2] = _mm256_broadcastw_epi16(lo(m[2])); // vpbroadcastw m2, xm2
    m[4] = _mm256_blend_epi32::<0xf0>(m[4], m[6]); // vpblendd m4, m6, 0xf0 ; ypos0 ypos1
    m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1
    let maxbased = (maxbaseq as i32) | 63; // or maxbased, 63
    m[4] = _mm256_add_epi16(m[4], m[2]); // paddw m4, m2
    loop {
        // .h8_loop:
        let mut r5 = r4 + dyq; // lea r5, [r4+dyq]
        r4 >>= 6; // sar r4, 6 ; base0
        m[0] = zx(lxw_q5(tl, r4, 2)); // movu xm0, [tlq+r4*2+2]
        m[1] = zx(lxw_q5(tl, r4, 0)); // movu xm1, [tlq+r4*2]
        r4 = r5 + dyq; // lea r4, [r5+dyq]
        r5 >>= 6; // sar r5, 6 ; base1
        m[0] = _mm256_inserti128_si256::<1>(m[0], lxw_q5(tl, r5, 2)); // vinserti128 m0, [tlq+r5*2+2], 1
        m[1] = _mm256_inserti128_si256::<1>(m[1], lxw_q5(tl, r5, 0)); // vinserti128 m1, [tlq+r5*2], 1
        r5 = r4 + dyq; // lea r5, [r4+dyq]
        r4 >>= 6; // sar r4, 6 ; base2
        m[3] = _mm256_and_si256(m[5], m[4]); // pand m3, m5, m4
        m[3] = _mm256_slli_epi16::<9>(m[3]); // psllw m3, 9
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        m[3] = _mm256_srai_epi16::<15>(m[4]); // psraw m3, m4, 15
        m[4] = _mm256_add_epi16(m[4], m[6]); // paddw m4, m6
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[1] = zx(lxw_q5(tl, r4, 2)); // movu xm1, [tlq+r4*2+2]
        m[2] = zx(lxw_q5(tl, r4, 0)); // movu xm2, [tlq+r4*2]
        r4 = r5 + dyq; // lea r4, [r5+dyq]
        r5 >>= 6; // sar r5, 6 ; base3
        m[0] = _mm256_blendv_epi8(m[7], m[0], m[3]); // vpblendvb m0, m7, m0, m3
        m[1] = _mm256_inserti128_si256::<1>(m[1], lxw_q5(tl, r5, 2)); // vinserti128 m1, [tlq+r5*2+2], 1
        m[2] = _mm256_inserti128_si256::<1>(m[2], lxw_q5(tl, r5, 0)); // vinserti128 m2, [tlq+r5*2], 1
        m[3] = _mm256_and_si256(m[5], m[4]); // pand m3, m5, m4
        m[3] = _mm256_slli_epi16::<9>(m[3]); // psllw m3, 9
        m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1
        m[2] = _mm256_mulhrs_epi16(m[2], m[3]); // pmulhrsw m2, m3
        m[3] = _mm256_srai_epi16::<15>(m[4]); // psraw m3, m4, 15
        m[4] = _mm256_add_epi16(m[4], m[6]); // paddw m4, m6
        let r5p = at(dst, stride * 4); // lea r5, [dstq+strideq*4]
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        m[1] = _mm256_blendv_epi8(m[7], m[1], m[3]); // vpblendvb m1, m7, m1, m3
        m[2] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m2, m0, m1 ; a3 c3 a2 c2 a1 c1 a0 c0   b3 d3 b2 d2 b1 d1 b0 d0
        m[3] = zx(_mm256_extracti128_si256::<1>(m[2])); // vextracti128 xm3, m2, 1
        m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1 ; a7 c7 a6 c6 a5 c5 a4 c5   b7 d7 b6 d6 b5 d5 b4 d4
        m[1] = zx(_mm_unpackhi_epi16(lo(m[2]), lo(m[3]))); // punpckhwd xm1, xm2, xm3 ; a1 b1 c1 d1 a0 b0 c0 d0
        m[2] = zx(_mm_unpacklo_epi16(lo(m[2]), lo(m[3]))); // punpcklwd xm2, xm3 ; a3 b3 c3 d3 a2 b2 c2 d2
        m[3] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm3, m0, 1
        movhps_q5(dst, 0, lo(m[1])); // movhps [dstq+strideq*0], xm1
        movq_q5(dst, stride, lo(m[1])); // movq [dstq+strideq*1], xm1
        movhps_q5(dst, stride * 2, lo(m[2])); // movhps [dstq+strideq*2], xm2
        movq_q5(dst, r7, lo(m[2])); // movq [dstq+r7], xm2
        m[1] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[3]))); // punpckhwd xm1, xm0, xm3 ; a5 b5 c5 d5 a4 b4 c4 d4
        m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[3]))); // punpcklwd xm0, xm3 ; a7 b7 c7 d7 a6 b6 c6 d6
        movhps_q5(r5p, 0, lo(m[1])); // movhps [r5+strideq*0], xm1
        movq_q5(r5p, stride, lo(m[1])); // movq [r5+strideq*1], xm1
        movhps_q5(r5p, stride * 2, lo(m[0])); // movhps [r5+strideq*2], xm0
        movq_q5(r5p, r7, lo(m[0])); // movq [r5+r7], xm0
        w -= 4; // sub wd, 4
        if w == 0 {
            return; // jz .h8_end
        }
        dst = at(dst, 8); // add dstq, 8
        if r4 as i32 <= maxbased {
            break; // cmp r4d, maxbased ; jg .h8_loop
        }
    }
    let r6 = stride * 5; // lea r6, [strideq*5]
    let r2 = stride + r7 * 2; // lea r2, [strideq+r7*2] ; stride*7
    if w & 4 != 0 {
        // test wd, 4 ; jz .h8_end_loop
        movq_q5(dst, 0, lo(m[7])); // movq [dstq+strideq*0], xm7
        movq_q5(dst, stride, lo(m[7])); // movq [dstq+strideq*1], xm7
        movq_q5(dst, stride * 2, lo(m[7])); // movq [dstq+strideq*2], xm7
        movq_q5(dst, r7, lo(m[7])); // movq [dstq+r7], xm7
        movq_q5(dst, stride * 4, lo(m[7])); // movq [dstq+strideq*4], xm7
        movq_q5(dst, r6, lo(m[7])); // movq [dstq+r6], xm7
        movq_q5(dst, r7 * 2, lo(m[7])); // movq [dstq+r7*2], xm7
        movq_q5(dst, r2, lo(m[7])); // movq [dstq+r2], xm7
        dst = at(dst, 8); // add dstq, 8
        w -= 4; // sub wd, 4
        if w == 0 {
            return; // jz .h8_end
        }
    }
    loop {
        // .h8_end_loop:
        sx_q5(dst, 0, lo(m[7])); // mova [dstq+strideq*0], xm7
        sx_q5(dst, stride, lo(m[7])); // mova [dstq+strideq*1], xm7
        sx_q5(dst, stride * 2, lo(m[7])); // mova [dstq+strideq*2], xm7
        sx_q5(dst, r7, lo(m[7])); // mova [dstq+r7], xm7
        sx_q5(dst, stride * 4, lo(m[7])); // mova [dstq+strideq*4], xm7
        sx_q5(dst, r6, lo(m[7])); // mova [dstq+r6], xm7
        sx_q5(dst, r7 * 2, lo(m[7])); // mova [dstq+r7*2], xm7
        sx_q5(dst, r2, lo(m[7])); // mova [dstq+r2], xm7
        dst = at(dst, 16); // add dstq, 16
        w -= 8; // sub wd, 8
        if w <= 0 {
            break; // jg .h8_end_loop
        }
    }
    // .h8_end: RET
}

/// `.h16`.
#[inline(always)]
unsafe fn ipred_z3_16bpc_h16(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    mut w: i32,
    mut angle: i32,
    dy: i32,
) {
    // ALLOC_STACK -96, 10: zeroed whole, with 64 bytes below rsp: the first group of `.h16_loop` reads down to
    // rsp-64 (base up to 63, 34 bytes at tlq+2*base with tlq = rsp+62).
    // Zeroed whole by `zero_frame_q5` where a path first uses it (not on the unfiltered path, which never points
    // into it).
    let mut frame = core::mem::MaybeUninit::<FrameQ5<{ 64 + 96 }>>::uninit();
    let fp = frame.as_mut_ptr();
    let rsp = (fp as *mut u8).wrapping_add(64);
    let mut maxbase = w + 15; // lea maxbased, [wq+15]
    let r7 = stride * 3; // lea r7, [strideq*3]
    'main: {
        if angle & 0x400 != 0 {
            // test angled, 0x400 ; jnz .h16_no_intra_edge_filter
            // .h16_no_intra_edge_filter:
            maxbase &= 15; // and maxbased, 15
            maxbase |= 16; // or maxbased, 16 ; imin(w+15, 31)
            break 'main; // jmp .h16_main
        }
        let mut r5 = ipred_z3_16bpc_filter_strength(m, maxbase, &mut angle); // call .filter_strength
        if r5 == 0 {
            break 'main; // test r5d, r5d ; jz .h16_main ; filter_strength == 0
        }
        zero_frame_q5(fp); // the frame, first used on this path
        r5 = r5.count_ones(); // popcnt r5d, r5d
        m[0] = ly_q5(tl, -28); // movu m0, [tlq-28] ; 3 4 5 6 7 8 9 a   b c d e f g h i
        m[1] = _mm256_add_epi16(m[0], ly_q5(tl, -32)); // paddw m1, m0, [tlq-32] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[6] = ipred_z3_16bpc_filter_k_q5(r5, 1); // vpbroadcastd m6, [base+z_filter_k-4+r5*4+12*1]
        m[7] = ipred_z3_16bpc_filter_k_q5(r5, 0); // vpbroadcastd m7, [base+z_filter_k-4+r5*4+12*0]
        m[2] = _mm256_mullo_epi16(m[6], ly_q5(tl, -30)); // pmullw m2, m6, [tlq-30] ; 2 3 4 5 6 7 8 9   a b c d e f g h
        m[1] = _mm256_mullo_epi16(m[1], m[7]); // pmullw m1, m7
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        if w <= 8 {
            // cmp wd, 8 ; jg .h16_filter_w16
            m[3] = zx(lx_q5(tl, -46)); // mova xm3, [tlq-46] ; 0 1 2 3 4 5 6 7
            m[6] = zx(_mm_mullo_epi16(lo(m[6]), lo(m[3]))); // pmullw xm6, xm3
            let five_tap;
            if w < 8 {
                // jl .h16_filter_w4
                // .h16_filter_w4:
                m[3] = zx(_mm_shufflehi_epi16::<0x90>(lo(m[3]))); // pshufhw xm3, xm3, q2100 ; _ _ _ _ 4 4 5 6
                five_tap = r5 == 3; // cmp r5d, 3 ; jne .h16_filter_w8_3tap
                if five_tap {
                    m[4] = zx(_mm_shufflehi_epi16::<0x90>(lo(m[3]))); // pshufhw xm4, xm3, q2100 ; _ _ _ _ 4 4 4 5
                } // jmp .h16_filter_w8_5tap
            } else {
                m[3] = zx(_mm_blend_epi16::<0xfe>(lo(m[3]), lx_q5(tl, -48))); // pblendw xm3, [tlq-48], 0xfe ; 0 0 1 2 3 4 5 6
                five_tap = r5 == 3; // cmp r5d, 3 ; jne .h16_filter_w8_3tap
                if five_tap {
                    m[4] = zx(_mm_blend_epi32::<0x0e>(lo(m[3]), lx_q5(tl, -50)));
                    // vpblendd xm4, xm3, [tlq-50], 0x0e ; 0 0 0 1 2 3 4 5
                }
            }
            if five_tap {
                // .h16_filter_w8_5tap:
                m[0] = _mm256_unpackhi_epi16(m[0], m[0]); // punpckhwd m0, m0
                m[0] = _mm256_blend_epi32::<0x7f>(m[0], ly_q5(tl, -26)); // vpblendd m0, [tlq-26], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h i i
                m[4] = zx(_mm_add_epi16(lo(m[4]), lx_q5(tl, -42))); // paddw xm4, [tlq-42] ; 2 3 4 5 6 7 8 9
                m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -34)); // paddw m0, [tlq-34] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
                m[4] = zx(_mm_add_epi16(lo(m[4]), lo(m[4]))); // paddw xm4, xm4
                m[0] = _mm256_add_epi16(m[0], m[0]); // paddw m0, m0
                m[6] = zx(_mm_add_epi16(lo(m[6]), lo(m[4]))); // paddw xm6, xm4
                m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
            }
            // .h16_filter_w8_3tap:
            m[3] = zx(_mm_add_epi16(lo(m[3]), lx_q5(tl, -44))); // paddw xm3, [tlq-44] ; 1 2 3 4 5 6 7 8
            m[3] = zx(_mm_mullo_epi16(lo(m[3]), lo(m[7]))); // pmullw xm3, xm7
            m[0] = _mm256_setzero_si256(); // pxor m0, m0
            m[3] = zx(_mm_add_epi16(lo(m[3]), lo(m[6]))); // paddw xm3, xm6
            m[3] = zx(_mm_srli_epi16::<3>(lo(m[3]))); // psrlw xm3, 3
            m[3] = zx(_mm_avg_epu16(lo(m[3]), lo(m[0]))); // pavgw xm3, xm0
            sx_q5(rsp, 48, lo(m[3])); // mova [rsp+48], xm3
                                      // jmp .h16_filter_end
        } else {
            // .h16_filter_w16:
            m[3] = ly_q5(tl, -62); // mova m3, [tlq-62] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
            m[6] = _mm256_mullo_epi16(m[6], m[3]); // pmullw m6, m3
            m[3] = zx(_mm_unpacklo_epi16(lo(m[3]), lo(m[3]))); // punpcklwd xm3, xm3
            m[4] = _mm256_blend_epi32::<0xfe>(m[3], ly_q5(tl, -64)); // vpblendd m4, m3, [tlq-64], 0xfe ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
            m[4] = _mm256_add_epi16(m[4], ly_q5(tl, -60)); // paddw m4, [tlq-60] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
            let r4 = 32; // mov r4d, 32
            if w > 16 {
                maxbase = r4; // cmp wd, 16 ; cmovg maxbased, r4d
            }
            movd_st_q5(rsp, 28, lo(m[3])); // movd [rsp+28], xm3
            m[4] = _mm256_mullo_epi16(m[4], m[7]); // pmullw m4, m7
            if r5 == 3 {
                // cmp r5d, 3 ; jne .h16_filter_w16_3tap
                m[0] = _mm256_unpackhi_epi16(m[0], m[0]); // punpckhwd m0, m0
                m[3] = _mm256_blend_epi32::<0xfe>(m[3], ly_q5(tl, -66)); // vpblendd m3, [tlq-66], 0xfe ; 0 0 0 1 2 3 4 5   6 7 8 9 a b c d
                m[0] = _mm256_blend_epi32::<0x7f>(m[0], ly_q5(tl, -26)); // vpblendd m0, [tlq-26], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h i i
                m[3] = _mm256_add_epi16(m[3], ly_q5(tl, -58)); // paddw m3, [tlq-58] ; 2 3 4 5 6 7 8 9   a b c d e f g h
                m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -34)); // paddw m0, [tlq-34] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
                let r4 = ldw_q5(tl, -62); // movzx r4d, word [tlq-62]
                let mut r2 = ldw_q5(tl, -60); // movzx r2d, word [tlq-60]
                maxbase |= 1; // or maxbased, 1
                m[3] = _mm256_add_epi16(m[3], m[3]); // paddw m3, m3
                r2 -= r4; // sub r2d, r4d
                m[0] = _mm256_add_epi16(m[0], m[0]); // paddw m0, m0
                r2 = r2 + r4 * 8 + 4; // lea r2d, [r2+r4*8+4]
                m[4] = _mm256_add_epi16(m[4], m[3]); // paddw m4, m3
                r2 = ((r2 as u32) >> 3) as i32; // shr r2d, 3
                m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
                stw_q5(rsp, 30, r2); // mov [rsp+30], r2w
            }
            // .h16_filter_w16_3tap:
            m[0] = _mm256_setzero_si256(); // pxor m0, m0
            m[4] = _mm256_add_epi16(m[4], m[6]); // paddw m4, m6
            m[4] = _mm256_srli_epi16::<3>(m[4]); // psrlw m4, 3
            m[4] = _mm256_avg_epu16(m[4], m[0]); // pavgw m4, m0
            sy_q5(rsp, 32, m[4]); // mova [rsp+32], m4
        }
        // .h16_filter_end:
        m[1] = _mm256_srli_epi16::<3>(m[1]); // psrlw m1, 3
        tl = rsp.wrapping_add(94); // lea tlq, [rsp+94]
        m[1] = _mm256_avg_epu16(m[1], m[0]); // pavgw m1, m0
        sy_q5(rsp, 64, m[1]); // mova [rsp+64], m1
    }
    // .h16_main:
    m[8] = zx(_mm_cvtsi32_si128(dy)); // movd xm8, dyd
    let mut maxbaseq = (maxbase as u32 as isize).wrapping_neg(); // neg maxbaseq (maxbased as a 32-bit write left it)
    m[9] = bcww_q5(tl, maxbaseq); // vpbroadcastw m9, [tlq+maxbaseq*2]
    maxbaseq <<= 6; // shl maxbased, 6 (64-bit here, one negated value for the address and the shift: the same low
                    // 32 bits, and every later use reads only those)
    m[8] = _mm256_broadcastw_epi16(lo(m[8])); // vpbroadcastw m8, xm8
    let mut r4 = (maxbaseq as i32).wrapping_add(dy + 15 * 64) as isize; // lea r4d, [maxbaseq+dyq+15*64]
    let dyq = -(dy as isize); // neg dyq
    m[7] = zx(_mm_cvtsi32_si128(r4 as i32)); // movd xm7, r4d
    tl = tl.wrapping_offset(-32); // sub tlq, 32
    r4 = dyq + 63; // lea r4, [dyq+63]
    m[7] = _mm256_broadcastw_epi16(lo(m[7])); // vpbroadcastw m7, xm7
    let maxbased = (maxbaseq as i32) | 63; // or maxbased, 63
    m[7] = _mm256_sub_epi16(m[7], ly_q5(ro(z_base_inc, 0), 0)); // psubw m7, [z_base_inc]
    loop {
        // .h16_loop:
        let mut r5 = r4 + dyq; // lea r5, [r4+dyq]
        r4 >>= 6; // sar r4, 6 ; base0
        m[0] = lyw_q5(tl, r4, 2); // movu m0, [tlq+r4*2+2]
        m[2] = lyw_q5(tl, r4, 0); // movu m2, [tlq+r4*2]
        r4 = r5 + dyq; // lea r4, [r5+dyq]
        r5 >>= 6; // sar r5, 6 ; base1
        m[1] = lyw_q5(tl, r5, 2); // movu m1, [tlq+r5*2+2]
        m[3] = lyw_q5(tl, r5, 0); // movu m3, [tlq+r5*2]
        r5 = r4 + dyq; // lea r5, [r4+dyq]
        r4 >>= 6; // sar r4, 6 ; base3
        m[6] = _mm256_and_si256(m[5], m[7]); // pand m6, m5, m7
        m[6] = _mm256_slli_epi16::<9>(m[6]); // psllw m6, 9
        m[2] = _mm256_sub_epi16(m[2], m[0]); // psubw m2, m0
        m[2] = _mm256_mulhrs_epi16(m[2], m[6]); // pmulhrsw m2, m6
        m[6] = _mm256_srai_epi16::<15>(m[7]); // psraw m6, m7, 15
        m[7] = _mm256_add_epi16(m[7], m[8]); // paddw m7, m8
        m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
        m[2] = lyw_q5(tl, r4, 2); // movu m2, [tlq+r4*2+2]
        m[4] = lyw_q5(tl, r4, 0); // movu m4, [tlq+r4*2]
        r4 = r5 + dyq; // lea r4, [r5+dyq]
        r5 >>= 6; // sar r5, 6 ; base3
        m[0] = _mm256_blendv_epi8(m[9], m[0], m[6]); // vpblendvb m0, m9, m0, m6
        m[6] = _mm256_and_si256(m[5], m[7]); // pand m6, m5, m7
        m[6] = _mm256_slli_epi16::<9>(m[6]); // psllw m6, 9
        m[3] = _mm256_sub_epi16(m[3], m[1]); // psubw m3, m1
        m[3] = _mm256_mulhrs_epi16(m[3], m[6]); // pmulhrsw m3, m6
        m[6] = _mm256_srai_epi16::<15>(m[7]); // psraw m6, m7, 15
        m[7] = _mm256_add_epi16(m[7], m[8]); // paddw m7, m8
        m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3
        m[1] = _mm256_blendv_epi8(m[9], m[1], m[6]); // vpblendvb m1, m9, m1, m6
        m[6] = _mm256_and_si256(m[5], m[7]); // pand m6, m5, m7
        m[6] = _mm256_slli_epi16::<9>(m[6]); // psllw m6, 9
        m[4] = _mm256_sub_epi16(m[4], m[2]); // psubw m4, m2
        m[4] = _mm256_mulhrs_epi16(m[4], m[6]); // pmulhrsw m4, m6
        m[6] = _mm256_srai_epi16::<15>(m[7]); // psraw m6, m7, 15
        m[7] = _mm256_add_epi16(m[7], m[8]); // paddw m7, m8
        m[2] = _mm256_add_epi16(m[2], m[4]); // paddw m2, m4
        m[3] = lyw_q5(tl, r5, 2); // movu m3, [tlq+r5*2+2]
        m[4] = lyw_q5(tl, r5, 0); // movu m4, [tlq+r5*2]
        m[2] = _mm256_blendv_epi8(m[9], m[2], m[6]); // vpblendvb m2, m9, m2, m6
        m[6] = _mm256_and_si256(m[5], m[7]); // pand m6, m5, m7
        m[6] = _mm256_slli_epi16::<9>(m[6]); // psllw m6, 9
        m[4] = _mm256_sub_epi16(m[4], m[3]); // psubw m4, m3
        m[4] = _mm256_mulhrs_epi16(m[4], m[6]); // pmulhrsw m4, m6
        m[6] = _mm256_srai_epi16::<15>(m[7]); // psraw m6, m7, 15
        m[7] = _mm256_add_epi16(m[7], m[8]); // paddw m7, m8
        let mut r5p = at(dst, stride * 4); // lea r5, [dstq+strideq*4]
        m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4
        m[3] = _mm256_blendv_epi8(m[9], m[3], m[6]); // vpblendvb m3, m9, m3, m6
        m[4] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m4, m0, m1 ; ab bb aa ba a9 b9 a8 b8   a3 b3 a2 b2 a1 b1 a0 b0
        m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1 ; af bf ae be ad bd ac bc   a7 b7 a6 b6 a5 b5 a4 b4
        m[1] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m1, m2, m3 ; cb db ca da c9 d9 c8 d8   c3 d3 c2 d2 c1 d1 c0 d0
        m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3 ; cf df ce de cd dd cc dc   c7 d7 c6 d6 c5 d5 c4 d4
        m[3] = _mm256_unpackhi_epi32(m[4], m[1]); // punpckhdq m3, m4, m1 ; a9 b9 c9 d9 a8 b8 c8 d8   a1 b1 c1 d1 a0 b0 c0 d0
        m[6] = zx(_mm256_extracti128_si256::<1>(m[3])); // vextracti128 xm6, m3, 1
        m[4] = _mm256_unpacklo_epi32(m[4], m[1]); // punpckldq m4, m1 ; ab bb cb db aa ba ca da   a3 b3 c3 d3 a2 b2 c2 d2
        m[1] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m1, m0, m2 ; ad bd cd dd ac bc cc dc   a5 b5 c5 d5 a4 b4 c4 d4
        m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2 ; af bf cf df ae be ce de   a7 b7 c7 d7 a6 b6 c6 d6
        m[2] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm2, m4, 1
        movhps_q5(dst, 0, lo(m[6])); // movhps [dstq+strideq*0], xm6
        movq_q5(dst, stride, lo(m[6])); // movq [dstq+strideq*1], xm6
        m[6] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm6, m1, 1
        movhps_q5(dst, stride * 2, lo(m[2])); // movhps [dstq+strideq*2], xm2
        movq_q5(dst, r7, lo(m[2])); // movq [dstq+r7], xm2
        m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1
        movhps_q5(r5p, 0, lo(m[6])); // movhps [r5+strideq*0], xm6
        movq_q5(r5p, stride, lo(m[6])); // movq [r5+strideq*1], xm6
        movhps_q5(r5p, stride * 2, lo(m[2])); // movhps [r5+strideq*2], xm2
        movq_q5(r5p, r7, lo(m[2])); // movq [r5+r7], xm2
        r5p = at(dst, stride * 8); // lea r5, [dstq+strideq*8]
        movhps_q5(r5p, 0, lo(m[3])); // movhps [r5+strideq*0], xm3
        movq_q5(r5p, stride, lo(m[3])); // movq [r5+strideq*1], xm3
        movhps_q5(r5p, stride * 2, lo(m[4])); // movhps [r5+strideq*2], xm4
        movq_q5(r5p, r7, lo(m[4])); // movq [r5+r7], xm4
        r5p = at(r5p, stride * 4); // lea r5, [r5+strideq*4]
        movhps_q5(r5p, 0, lo(m[1])); // movhps [r5+strideq*0], xm1
        movq_q5(r5p, stride, lo(m[1])); // movq [r5+strideq*1], xm1
        movhps_q5(r5p, stride * 2, lo(m[0])); // movhps [r5+strideq*2], xm0
        movq_q5(r5p, r7, lo(m[0])); // movq [r5+r7], xm0
        w -= 4; // sub wd, 4
        if w == 0 {
            return; // jz .h16_end
        }
        dst = at(dst, 8); // add dstq, 8
        if r4 as i32 <= maxbased {
            break; // cmp r4d, maxbased ; jg .h16_loop
        }
    }
    let mut h = 4; // mov hd, 4
    loop {
        // .h16_end_loop0:
        let mut r6 = w; // mov r6d, wd
        let r2 = dst; // mov r2, dstq
        'end_w4: {
            if w & 4 != 0 {
                // test wb, 4 ; jz .h16_end_loop
                movq_q5(dst, 0, lo(m[9])); // movq [dstq+strideq*0], xm9
                movq_q5(dst, stride, lo(m[9])); // movq [dstq+strideq*1], xm9
                movq_q5(dst, stride * 2, lo(m[9])); // movq [dstq+strideq*2], xm9
                movq_q5(dst, r7, lo(m[9])); // movq [dstq+r7], xm9
                r6 &= 120; // and r6d, 120
                if r6 == 0 {
                    break 'end_w4; // jz .h16_end_w4
                }
                dst = at(dst, 8); // add dstq, 8
            }
            loop {
                // .h16_end_loop:
                sx_q5(dst, 0, lo(m[9])); // mova [dstq+strideq*0], xm9
                sx_q5(dst, stride, lo(m[9])); // mova [dstq+strideq*1], xm9
                sx_q5(dst, stride * 2, lo(m[9])); // mova [dstq+strideq*2], xm9
                sx_q5(dst, r7, lo(m[9])); // mova [dstq+r7], xm9
                dst = at(dst, 16); // add dstq, 16
                r6 -= 8; // sub r6d, 8
                if r6 <= 0 {
                    break; // jg .h16_end_loop
                }
            }
        }
        // .h16_end_w4:
        dst = at(r2, stride * 4); // lea dstq, [r2+strideq*4]
        h -= 1; // dec hd
        if h <= 0 {
            break; // jg .h16_end_loop0
        }
    }
    // .h16_end: RET
}

/// `.h32`.
#[inline(always)]
unsafe fn ipred_z3_16bpc_h32(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    mut w: i32,
    angle: i32,
    dy: i32,
    mut org_w: i32,
) {
    // ALLOC_STACK -160, 9: zeroed whole, with 64 bytes below rsp: `.h32_loop` reads down to rsp-34 (a column runs
    // while its base is below maxbase <= 65: 34 bytes at tlq+2*base-64 with tlq = rsp+158).
    // Zeroed whole by `zero_frame_q5` where a path first uses it (not on the unfiltered path, which never points
    // into it).
    let mut frame = core::mem::MaybeUninit::<FrameQ5<{ 64 + 160 }>>::uninit();
    let fp = frame.as_mut_ptr();
    let rsp = (fp as *mut u8).wrapping_add(64);
    // The column slots below the frame (`sub rsp, 64` per column, up to w = 64 columns). Every slot the transposes
    // read was written before: the loops push exactly w (= org_w) columns, the transposes read org_w columns.
    let mut cols = ColsQ5::<{ 2 * 64 }>(core::mem::MaybeUninit::uninit());
    let mut sp = (cols.0.as_mut_ptr() as *mut u8).wrapping_add(64 * 64);
    let mut maxbase = w + 31; // lea maxbased, [wq+31]
    maxbase &= 31; // and maxbased, 31
    maxbase |= 32; // or maxbased, 32 ; imin(w+31, 63)
    'main: {
        if angle & 0x400 != 0 {
            break 'main; // test angled, 0x400 ; jnz .h32_main
        }
        zero_frame_q5(fp); // the frame, first used on this path
        m[2] = bcd_q5(ro(pw_3, 0)); // vpbroadcastd m2, [pw_3]
        m[0] = ly_q5(tl, -28); // movu m0, [tlq-28] ; 3 4 5 6 7 8 9 a   b c d e f g h i
        m[1] = _mm256_unpackhi_epi16(m[0], m[0]); // punpckhwd m1, m0, m0
        m[1] = _mm256_blend_epi32::<0x7f>(m[1], ly_q5(tl, -26)); // vpblendd m1, [tlq-26], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h i i
        m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -30)); // paddw m0, [tlq-30] ; 2 3 4 5 6 7 8 9   a b c d e f g h
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -32)); // paddw m0, [tlq-32] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[1] = _mm256_avg_epu16(m[1], ly_q5(tl, -34)); // pavgw m1, [tlq-34] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
        let mut r4 = rsp.wrapping_add(128); // lea r4, [rsp+128]
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        let mut r5 = maxbase - 31; // lea r5d, [maxbaseq-31]
        m[0] = _mm256_srli_epi16::<2>(m[0]); // psrlw m0, 2
        sy_q5(r4, 0, m[0]); // mova [r4], m0
        loop {
            // .h32_filter_loop:
            m[0] = ly_q5(tl, -62); // mova m0, [tlq-62]
            m[1] = _mm256_add_epi16(m[2], ly_q5(tl, -66)); // paddw m1, m2, [tlq-66]
            m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -64)); // paddw m0, [tlq-64]
            m[1] = _mm256_avg_epu16(m[1], ly_q5(tl, -58)); // pavgw m1, [tlq-58]
            m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -60)); // paddw m0, [tlq-60]
            tl = tl.wrapping_offset(-32); // sub tlq, 32
            r4 = r4.wrapping_offset(-32); // sub r4, 32
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[0] = _mm256_srli_epi16::<2>(m[0]); // psrlw m0, 2
            sy_q5(r4, 0, m[0]); // mova [r4], m0
            r5 -= 16; // sub r5d, 16
            if r5 <= 0 {
                break; // jg .h32_filter_loop
            }
        }
        if r5 < 0 {
            // jl .h32_filter_h8
            // .h32_filter_h8:
            m[0] = zx(lx_q5(tl, -46)); // mova xm0, [tlq-46] ; 0 1 2 3 4 5 6 7
            m[1] = zx(_mm_blend_epi16::<0xfe>(lo(m[0]), lx_q5(tl, -48))); // pblendw xm1, xm0, [tlq-48], 0xfe ; 0 0 1 2 3 4 5 6
            m[2] = zx(_mm_add_epi16(lo(m[2]), lx_q5(tl, -42))); // paddw xm2, [tlq-42] ; 2 3 4 5 6 7 8 9
            m[0] = zx(_mm_add_epi16(lo(m[0]), lx_q5(tl, -44))); // paddw xm0, [tlq-44] ; 1 2 3 4 5 6 7 8
            m[3] = zx(_mm_blend_epi32::<0x0e>(lo(m[1]), lx_q5(tl, -50))); // vpblendd xm3, xm1, [tlq-50], 0x0e ; 0 0 0 1 2 3 4 5
            tl = rsp.wrapping_add(158); // lea tlq, [rsp+158]
            m[2] = zx(_mm_avg_epu16(lo(m[2]), lo(m[3]))); // pavgw xm2, xm3
            m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
            m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); // paddw xm0, xm2
            m[0] = zx(_mm_srli_epi16::<2>(lo(m[0]))); // psrlw xm0, 2
            sx_q5(r4, -16, lo(m[0])); // mova [r4-16], xm0
            break 'main;
        }
        m[0] = ly_q5(tl, -62); // mova m0, [tlq-62] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
        m[1] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[0]))); // punpcklwd xm1, xm0, xm0
        m[2] = _mm256_add_epi16(m[2], ly_q5(tl, -58)); // paddw m2, [tlq-58] ; 2 3 4 5 6 7 8 9   a b c d e f g h
        m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -60)); // paddw m0, [tlq-60] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[3] = _mm256_blend_epi32::<0xfe>(m[1], ly_q5(tl, -66)); // vpblendd m3, m1, [tlq-66], 0xfe ; 0 0 0 1 2 3 4 5   6 7 8 9 a b c d
        m[1] = _mm256_blend_epi32::<0xfe>(m[1], ly_q5(tl, -64)); // vpblendd m1, [tlq-64], 0xfe ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
        let r5 = ldw_q5(tl, -62); // movzx r5d, word [tlq-62]
        let mut r2 = ldw_q5(tl, -60); // movzx r2d, word [tlq-60]
        m[2] = _mm256_avg_epu16(m[2], m[3]); // pavgw m2, m3
        r2 -= r5; // sub r2d, r5d
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        r2 = r2 + r5 * 8 + 4; // lea r2d, [r2+r5*8+4]
        m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
        r2 = ((r2 as u32) >> 3) as i32; // shr r2d, 3
        m[0] = _mm256_srli_epi16::<2>(m[0]); // psrlw m0, 2
        sy_q5(r4, -32, m[0]); // mova [r4-32], m0
        stw_q5(r4, -36, r5); // mov [r4-36], r5w
        stw_q5(r4, -34, r2); // mov [r4-34], r2w
        tl = rsp.wrapping_add(158); // lea tlq, [rsp+158]
        let r4 = 65; // mov r4d, 65
        if w == 64 {
            maxbase = r4; // cmp wd, 64 ; cmove maxbased, r4d
        } // jmp .h32_main
    }
    // .h32_main:
    m[6] = zx(_mm_cvtsi32_si128(dy)); // movd xm6, dyd
    let mut maxbaseq = (maxbase as u32 as isize).wrapping_neg(); // neg maxbaseq (maxbased as a 32-bit write left it)
    m[7] = bcww_q5(tl, maxbaseq); // vpbroadcastw m7, [tlq+maxbaseq*2]
    maxbaseq <<= 6; // shl maxbased, 6 (64-bit here, one negated value for the address and the shift: the same low
                    // 32 bits, and every later use reads only those)
    m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
    let mut r4 = (maxbaseq as i32).wrapping_add(dy + 15 * 64) as isize; // lea r4d, [maxbaseq+dyq+15*64]
    let dyq = -(dy as isize); // neg dyq
    m[4] = zx(_mm_cvtsi32_si128(r4 as i32)); // movd xm4, r4d
    m[8] = bcd_q5(ro(pw_m1024, 0)); // vpbroadcastd m8, [pw_m1024]
    r4 = dyq + 63; // lea r4, [dyq+63]
    m[4] = _mm256_broadcastw_epi16(lo(m[4])); // vpbroadcastw m4, xm4
    let maxbased = (maxbaseq as i32) | 63; // or maxbased, 63
    m[4] = _mm256_sub_epi16(m[4], ly_q5(ro(z_base_inc, 0), 0)); // psubw m4, [z_base_inc]
    'transpose: {
        loop {
            // .h32_loop:
            let mut r5 = r4; // mov r5, r4
            r5 >>= 6; // sar r5, 6
            m[1] = lyw_q5(tl, r5, -64); // movu m1, [tlq+r5*2-64]
            m[0] = lyw_q5(tl, r5, -62); // movu m0, [tlq+r5*2-62]
            m[3] = _mm256_and_si256(m[5], m[4]); // pand m3, m5, m4
            m[3] = _mm256_slli_epi16::<9>(m[3]); // psllw m3, 9
            m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
            m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
            m[2] = _mm256_cmpgt_epi16(m[8], m[4]); // pcmpgtw m2, m8, m4
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[0] = _mm256_blendv_epi8(m[7], m[0], m[2]); // vpblendvb m0, m7, m0, m2
            m[2] = lyw_q5(tl, r5, -32); // movu m2, [tlq+r5*2-32]
            m[1] = lyw_q5(tl, r5, -30); // movu m1, [tlq+r5*2-30]
            r4 += dyq; // add r4, dyq
            sp = sp.wrapping_offset(-64); // sub rsp, 64
            m[2] = _mm256_sub_epi16(m[2], m[1]); // psubw m2, m1
            m[2] = _mm256_mulhrs_epi16(m[2], m[3]); // pmulhrsw m2, m3
            m[3] = _mm256_srai_epi16::<15>(m[4]); // psraw m3, m4, 15
            m[4] = _mm256_add_epi16(m[4], m[6]); // paddw m4, m6
            _mm256_store_si256(sp as *mut __m256i, m[0]); // mova [rsp+32*0], m0
            m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
            m[1] = _mm256_blendv_epi8(m[7], m[1], m[3]); // vpblendvb m1, m7, m1, m3
            _mm256_store_si256(sp.wrapping_add(32) as *mut __m256i, m[1]); // mova [rsp+32*1], m1
            w -= 1; // dec wd
            if w == 0 {
                break 'transpose; // jz .h32_transpose
            }
            if r4 as i32 <= maxbased {
                break; // cmp r4d, maxbased ; jg .h32_loop
            }
        }
        loop {
            // .h32_end_loop:
            sp = sp.wrapping_offset(-64); // sub rsp, 64
            _mm256_store_si256(sp as *mut __m256i, m[7]); // mova [rsp+32*0], m7
            _mm256_store_si256(sp.wrapping_add(32) as *mut __m256i, m[7]); // mova [rsp+32*1], m7
            w -= 1; // dec wd
            if w <= 0 {
                break; // jg .h32_end_loop
            }
        }
    }
    // .h32_transpose:
    let r3 = stride * 3; // lea r3, [strideq*3]
    let r4 = stride * 5; // lea r4, [strideq*5]
    let r8 = dst; // mov r8, dstq
    let r5 = stride + r3 * 2; // lea r5, [strideq+r3*2]
    loop {
        // .h32_transpose_loop0:
        let mut r6 = sp.wrapping_add(32); // lea r6, [rsp+32]
        let mut r2 = at(r8, org_w as isize * 2 - 16); // lea r2, [r8+org_wq*2-16]
        loop {
            // .h32_transpose_loop:
            m[0] = _mm256_load_si256(r6.wrapping_add(64 * 7) as *const __m256i); // mova m0, [r6+64*7]
            m[1] = _mm256_load_si256(r6.wrapping_add(64 * 6) as *const __m256i); // mova m1, [r6+64*6]
            m[2] = _mm256_load_si256(r6.wrapping_add(64 * 5) as *const __m256i); // mova m2, [r6+64*5]
            m[3] = _mm256_load_si256(r6.wrapping_add(64 * 4) as *const __m256i); // mova m3, [r6+64*4]
            m[4] = _mm256_load_si256(r6.wrapping_add(64 * 3) as *const __m256i); // mova m4, [r6+64*3]
            m[5] = _mm256_load_si256(r6.wrapping_add(64 * 2) as *const __m256i); // mova m5, [r6+64*2]
            m[6] = _mm256_load_si256(r6.wrapping_add(64 * 1) as *const __m256i); // mova m6, [r6+64*1]
            m[7] = _mm256_load_si256(r6 as *const __m256i); // mova m7, [r6+64*0]
            m[8] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m8, m0, m1 ; a3 b3 a2 b2 a1 b1 a0 b0
            m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1 ; a7 b7 a6 b6 a5 b5 a4 b4
            m[1] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m1, m2, m3 ; c3 d3 c2 d2 c1 d1 c0 d0
            m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3 ; c7 d7 c6 d6 c5 d5 c4 d4
            m[3] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m3, m4, m5 ; e3 f3 e2 f2 e1 f1 e0 f0
            m[4] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m4, m5 ; e7 f7 e6 f6 e5 f5 e4 f4
            m[5] = _mm256_unpackhi_epi16(m[6], m[7]); // punpckhwd m5, m6, m7 ; g3 h3 g2 h2 g1 h1 g0 h0
            m[6] = _mm256_unpacklo_epi16(m[6], m[7]); // punpcklwd m6, m7 ; g7 h7 g6 h6 g5 h5 g4 h4
            let dstq = at(r2, stride * 8); // lea dstq, [r2+strideq*8]
            r6 = r6.wrapping_offset(-32); // sub r6, 32
            m[7] = _mm256_unpackhi_epi32(m[8], m[1]); // punpckhdq m7, m8, m1 ; a1 b1 c1 d1 a0 b0 c0 d0
            m[8] = _mm256_unpacklo_epi32(m[8], m[1]); // punpckldq m8, m1 ; a3 b3 c3 d3 a2 b2 c2 d2
            m[1] = _mm256_unpackhi_epi32(m[3], m[5]); // punpckhdq m1, m3, m5 ; e1 f1 g1 h1 e0 f0 g0 h0
            m[3] = _mm256_unpacklo_epi32(m[3], m[5]); // punpckldq m3, m5 ; e3 f3 g3 h3 e2 f2 g2 h2
            m[5] = _mm256_unpackhi_epi64(m[7], m[1]); // punpckhqdq m5, m7, m1 ;  8  0
            sx_q5(r2, 0, _mm256_extracti128_si256::<1>(m[5])); // vextracti128 [r2+strideq*0], m5, 1
            m[7] = _mm256_unpacklo_epi64(m[7], m[1]); // punpcklqdq m7, m1 ;  9  1
            sx_q5(dstq, 0, lo(m[5])); // mova [dstq+strideq*0], xm5
            m[1] = _mm256_unpackhi_epi64(m[8], m[3]); // punpckhqdq m1, m8, m3 ; 10  2
            sx_q5(r2, stride, _mm256_extracti128_si256::<1>(m[7])); // vextracti128 [r2+strideq*1], m7, 1
            m[8] = _mm256_unpacklo_epi64(m[8], m[3]); // punpcklqdq m8, m3 ; 11  3
            sx_q5(dstq, stride, lo(m[7])); // mova [dstq+strideq*1], xm7
            m[3] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m3, m0, m2 ; a5 b5 c5 d5 a4 b4 c4 d4
            sx_q5(r2, stride * 2, _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [r2+strideq*2], m1, 1
            m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2 ; a7 b7 c7 d7 a6 b6 c6 d6
            sx_q5(dstq, stride * 2, lo(m[1])); // mova [dstq+strideq*2], xm1
            m[2] = _mm256_unpackhi_epi32(m[4], m[6]); // punpckhdq m2, m4, m6 ; e5 f5 g5 h5 e4 f4 g4 h4
            sx_q5(r2, r3, _mm256_extracti128_si256::<1>(m[8])); // vextracti128 [r2+r3], m8, 1
            m[4] = _mm256_unpacklo_epi32(m[4], m[6]); // punpckldq m4, m6 ; e7 f7 g7 h7 e6 f6 g6 h6
            sx_q5(dstq, r3, lo(m[8])); // mova [dstq+r3], xm8
            m[6] = _mm256_unpackhi_epi64(m[3], m[2]); // punpckhqdq m6, m3, m2 ; 12  4
            sx_q5(r2, stride * 4, _mm256_extracti128_si256::<1>(m[6])); // vextracti128 [r2+strideq*4], m6, 1
            m[3] = _mm256_unpacklo_epi64(m[3], m[2]); // punpcklqdq m3, m2 ; 13  5
            sx_q5(dstq, stride * 4, lo(m[6])); // mova [dstq+strideq*4], xm6
            m[2] = _mm256_unpackhi_epi64(m[0], m[4]); // punpckhqdq m2, m0, m4 ; 14  6
            sx_q5(r2, r4, _mm256_extracti128_si256::<1>(m[3])); // vextracti128 [r2+r4], m3, 1
            m[0] = _mm256_unpacklo_epi64(m[0], m[4]); // punpcklqdq m0, m4 ; 15  7
            sx_q5(dstq, r4, lo(m[3])); // mova [dstq+r4], xm3
            sx_q5(r2, r3 * 2, _mm256_extracti128_si256::<1>(m[2])); // vextracti128 [r2+r3*2], m2, 1
            sx_q5(dstq, r3 * 2, lo(m[2])); // mova [dstq+r3*2], xm2
            sx_q5(r2, r5, _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [r2+r5], m0, 1
            sx_q5(dstq, r5, lo(m[0])); // mova [dstq+r5], xm0
            r2 = at(dstq, stride * 8); // lea r2, [dstq+strideq*8]
            if (r6 as usize) < (sp as usize) {
                break; // cmp r6, rsp ; jae .h32_transpose_loop
            }
        }
        sp = sp.wrapping_add(64 * 8); // add rsp, 64*8
        org_w -= 8; // sub org_wd, 8
        if org_w <= 0 {
            break; // jg .h32_transpose_loop0
        }
    }
    // .h32_end: RET
}

/// `.h64`.
#[inline(always)]
unsafe fn ipred_z3_16bpc_h64(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    mut w: i32,
    angle: i32,
    dy: i32,
    mut org_w: i32,
) {
    // ALLOC_STACK -256, 10: zeroed whole, with 128 bytes below rsp: `.h64_loop` reads down to rsp-126 (a column
    // runs while its base is below maxbase <= 127: 130 bytes at tlq+2*base-128 with tlq = rsp+254).
    // Zeroed whole by `zero_frame_q5` where a path first uses it (not on the unfiltered path, which never points
    // into it).
    let mut frame = core::mem::MaybeUninit::<FrameQ5<{ 128 + 256 }>>::uninit();
    let fp = frame.as_mut_ptr();
    let rsp = (fp as *mut u8).wrapping_add(128);
    // The column slots below the frame (`sub rsp, 128` per column, up to w = 64 columns). Every slot the
    // transposes read was written before: the loops push exactly w (= org_w) columns, the transposes read org_w.
    let mut cols = ColsQ5::<{ 4 * 64 }>(core::mem::MaybeUninit::uninit());
    let mut sp = (cols.0.as_mut_ptr() as *mut u8).wrapping_add(128 * 64);
    let maxbase = w + 63; // lea maxbased, [wq+63]
    'main: {
        if angle & 0x400 != 0 {
            break 'main; // test angled, 0x400 ; jnz .h64_main
        }
        zero_frame_q5(fp); // the frame, first used on this path
        m[2] = bcd_q5(ro(pw_3, 0)); // vpbroadcastd m2, [pw_3]
        m[0] = ly_q5(tl, -28); // movu m0, [tlq-28] ; 3 4 5 6 7 8 9 a   b c d e f g h i
        m[1] = _mm256_unpackhi_epi16(m[0], m[0]); // punpckhwd m1, m0, m0
        m[1] = _mm256_blend_epi32::<0x7f>(m[1], ly_q5(tl, -26)); // vpblendd m1, [tlq-26], 0x7f ; 4 5 6 7 8 9 a b   c d e f g h i i
        m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -30)); // paddw m0, [tlq-30] ; 2 3 4 5 6 7 8 9   a b c d e f g h
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -32)); // paddw m0, [tlq-32] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[1] = _mm256_avg_epu16(m[1], ly_q5(tl, -34)); // pavgw m1, [tlq-34] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
        let mut r4 = rsp.wrapping_add(224); // lea r4, [rsp+224]
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        let mut r5 = w + 32; // lea r5d, [wq+32]
        m[0] = _mm256_srli_epi16::<2>(m[0]); // psrlw m0, 2
        sy_q5(r4, 0, m[0]); // mova [r4], m0
        loop {
            // .h64_filter_loop:
            m[0] = ly_q5(tl, -62); // mova m0, [tlq-62]
            m[1] = _mm256_add_epi16(m[2], ly_q5(tl, -66)); // paddw m1, m2, [tlq-66]
            m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -64)); // paddw m0, [tlq-64]
            m[1] = _mm256_avg_epu16(m[1], ly_q5(tl, -58)); // pavgw m1, [tlq-58]
            m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -60)); // paddw m0, [tlq-60]
            tl = tl.wrapping_offset(-32); // sub tlq, 32
            r4 = r4.wrapping_offset(-32); // sub r4, 32
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[0] = _mm256_srli_epi16::<2>(m[0]); // psrlw m0, 2
            sy_q5(r4, 0, m[0]); // mova [r4], m0
            r5 -= 16; // sub r5d, 16
            if r5 <= 0 {
                break; // jg .h64_filter_loop
            }
        }
        m[0] = ly_q5(tl, -62); // mova m0, [tlq-62] ; 0 1 2 3 4 5 6 7   8 9 a b c d e f
        m[1] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[0]))); // punpcklwd xm1, xm0, xm0
        m[2] = _mm256_add_epi16(m[2], ly_q5(tl, -58)); // paddw m2, [tlq-58] ; 2 3 4 5 6 7 8 9   a b c d e f g h
        m[0] = _mm256_add_epi16(m[0], ly_q5(tl, -60)); // paddw m0, [tlq-60] ; 1 2 3 4 5 6 7 8   9 a b c d e f g
        m[3] = _mm256_blend_epi32::<0xfe>(m[1], ly_q5(tl, -66)); // vpblendd m3, m1, [tlq-66], 0xfe ; 0 0 0 1 2 3 4 5   6 7 8 9 a b c d
        m[1] = _mm256_blend_epi32::<0xfe>(m[1], ly_q5(tl, -64)); // vpblendd m1, [tlq-64], 0xfe ; 0 0 1 2 3 4 5 6   7 8 9 a b c d e
        tl = rsp.wrapping_add(254); // lea tlq, [rsp+254]
        m[2] = _mm256_avg_epu16(m[2], m[3]); // pavgw m2, m3
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
        m[0] = _mm256_srli_epi16::<2>(m[0]); // psrlw m0, 2
        sy_q5(r4, -32, m[0]); // mova [r4-32], m0
    }
    // .h64_main:
    let mut maxbaseq = (maxbase as u32 as isize).wrapping_neg(); // neg maxbaseq (maxbased as a 32-bit write left it)
    m[4] = zx(_mm_cvtsi32_si128(dy)); // movd xm4, dyd
    m[6] = bcww_q5(tl, maxbaseq); // vpbroadcastw m6, [tlq+maxbaseq*2]
    maxbaseq <<= 6; // shl maxbased, 6 (64-bit here, one negated value for the address and the shift: the same low
                    // 32 bits, and every later use reads only those)
    m[4] = _mm256_broadcastw_epi16(lo(m[4])); // vpbroadcastw m4, xm4
    let mut r4 = (maxbaseq as i32).wrapping_add(dy + 15 * 64) as isize; // lea r4d, [maxbaseq+dyq+15*64]
    let dyq = -(dy as isize); // neg dyq
    m[7] = bcd_q5(ro(pw_m1024, 0)); // vpbroadcastd m7, [pw_m1024]
    m[3] = zx(_mm_cvtsi32_si128(r4 as i32)); // movd xm3, r4d
    r4 = dyq + 63; // lea r4, [dyq+63]
    m[8] = _mm256_add_epi16(m[7], m[7]); // paddw m8, m7, m7
    m[3] = _mm256_broadcastw_epi16(lo(m[3])); // vpbroadcastw m3, xm3
    let maxbased = (maxbaseq as i32) | 63; // or maxbased, 63
    m[9] = _mm256_add_epi16(m[8], m[7]); // paddw m9, m8, m7
    m[3] = _mm256_sub_epi16(m[3], ly_q5(ro(z_base_inc, 0), 0)); // psubw m3, [z_base_inc]
    'transpose: {
        loop {
            // .h64_loop:
            let mut r5 = r4; // mov r5, r4
            r5 >>= 6; // sar r5, 6
            m[1] = lyw_q5(tl, r5, -128); // movu m1, [tlq+r5*2-128]
            m[0] = lyw_q5(tl, r5, -126); // movu m0, [tlq+r5*2-126]
            m[2] = _mm256_and_si256(m[5], m[3]); // pand m2, m5, m3
            m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
            m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
            m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
            sp = sp.wrapping_offset(-128); // sub rsp, 128
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[1] = _mm256_cmpgt_epi16(m[9], m[3]); // pcmpgtw m1, m9, m3
            m[0] = _mm256_blendv_epi8(m[6], m[0], m[1]); // vpblendvb m0, m6, m0, m1
            _mm256_store_si256(sp as *mut __m256i, m[0]); // mova [rsp+32*0], m0
            m[1] = lyw_q5(tl, r5, -96); // movu m1, [tlq+r5*2-96]
            m[0] = lyw_q5(tl, r5, -94); // movu m0, [tlq+r5*2-94]
            m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
            m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[1] = _mm256_cmpgt_epi16(m[8], m[3]); // pcmpgtw m1, m8, m3
            m[0] = _mm256_blendv_epi8(m[6], m[0], m[1]); // vpblendvb m0, m6, m0, m1
            _mm256_store_si256(sp.wrapping_add(32) as *mut __m256i, m[0]); // mova [rsp+32*1], m0
            m[1] = lyw_q5(tl, r5, -64); // movu m1, [tlq+r5*2-64]
            m[0] = lyw_q5(tl, r5, -62); // movu m0, [tlq+r5*2-62]
            m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
            m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[1] = _mm256_cmpgt_epi16(m[7], m[3]); // pcmpgtw m1, m7, m3
            m[0] = _mm256_blendv_epi8(m[6], m[0], m[1]); // vpblendvb m0, m6, m0, m1
            _mm256_store_si256(sp.wrapping_add(64) as *mut __m256i, m[0]); // mova [rsp+32*2], m0
            m[1] = lyw_q5(tl, r5, -32); // movu m1, [tlq+r5*2-32]
            m[0] = lyw_q5(tl, r5, -30); // movu m0, [tlq+r5*2-30]
            m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
            m[1] = _mm256_mulhrs_epi16(m[1], m[2]); // pmulhrsw m1, m2
            r4 += dyq; // add r4, dyq
            m[2] = _mm256_srai_epi16::<15>(m[3]); // psraw m2, m3, 15
            m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[0] = _mm256_blendv_epi8(m[6], m[0], m[2]); // vpblendvb m0, m6, m0, m2
            _mm256_store_si256(sp.wrapping_add(96) as *mut __m256i, m[0]); // mova [rsp+32*3], m0
            w -= 1; // dec wd
            if w == 0 {
                break 'transpose; // jz .h64_transpose
            }
            if r4 as i32 <= maxbased {
                break; // cmp r4d, maxbased ; jg .h64_loop
            }
        }
        loop {
            // .h64_end_loop:
            sp = sp.wrapping_offset(-128); // sub rsp, 128
            _mm256_store_si256(sp as *mut __m256i, m[6]); // mova [rsp+32*0], m6
            _mm256_store_si256(sp.wrapping_add(32) as *mut __m256i, m[6]); // mova [rsp+32*1], m6
            _mm256_store_si256(sp.wrapping_add(64) as *mut __m256i, m[6]); // mova [rsp+32*2], m6
            _mm256_store_si256(sp.wrapping_add(96) as *mut __m256i, m[6]); // mova [rsp+32*3], m6
            w -= 1; // dec wd
            if w <= 0 {
                break; // jg .h64_end_loop
            }
        }
    }
    // .h64_transpose:
    let r2 = stride * 3; // lea r2, [strideq*3]
    let r3 = stride * 5; // lea r3, [strideq*5]
    let r5 = dst; // mov r5, dstq
    let r4 = stride + r2 * 2; // lea r4, [strideq+r2*2]
    loop {
        // .h64_transpose_loop0:
        let mut r6 = sp.wrapping_add(112); // lea r6, [rsp+112]
        let mut dstq = at(r5, org_w as isize * 2 - 32); // lea dstq, [r5+org_wq*2-32]
        loop {
            // .h64_transpose_loop:
            m[0] = zx(_mm_load_si128(r6.wrapping_add(128 * 15) as *const __m128i)); // mova xm0, [r6+128*15]
            m[0] = _mm256_inserti128_si256::<1>(m[0], _mm_load_si128(r6.wrapping_add(128 * 7) as *const __m128i)); // vinserti128 m0, [r6+128* 7], 1
            m[1] = zx(_mm_load_si128(r6.wrapping_add(128 * 14) as *const __m128i)); // mova xm1, [r6+128*14]
            m[1] = _mm256_inserti128_si256::<1>(m[1], _mm_load_si128(r6.wrapping_add(128 * 6) as *const __m128i)); // vinserti128 m1, [r6+128* 6], 1
            m[2] = zx(_mm_load_si128(r6.wrapping_add(128 * 13) as *const __m128i)); // mova xm2, [r6+128*13]
            m[2] = _mm256_inserti128_si256::<1>(m[2], _mm_load_si128(r6.wrapping_add(128 * 5) as *const __m128i)); // vinserti128 m2, [r6+128* 5], 1
            m[3] = zx(_mm_load_si128(r6.wrapping_add(128 * 12) as *const __m128i)); // mova xm3, [r6+128*12]
            m[3] = _mm256_inserti128_si256::<1>(m[3], _mm_load_si128(r6.wrapping_add(128 * 4) as *const __m128i)); // vinserti128 m3, [r6+128* 4], 1
            m[4] = zx(_mm_load_si128(r6.wrapping_add(128 * 11) as *const __m128i)); // mova xm4, [r6+128*11]
            m[4] = _mm256_inserti128_si256::<1>(m[4], _mm_load_si128(r6.wrapping_add(128 * 3) as *const __m128i)); // vinserti128 m4, [r6+128* 3], 1
            m[5] = zx(_mm_load_si128(r6.wrapping_add(128 * 10) as *const __m128i)); // mova xm5, [r6+128*10]
            m[5] = _mm256_inserti128_si256::<1>(m[5], _mm_load_si128(r6.wrapping_add(128 * 2) as *const __m128i)); // vinserti128 m5, [r6+128* 2], 1
            m[6] = zx(_mm_load_si128(r6.wrapping_add(128 * 9) as *const __m128i)); // mova xm6, [r6+128* 9]
            m[6] = _mm256_inserti128_si256::<1>(m[6], _mm_load_si128(r6.wrapping_add(128 * 1) as *const __m128i)); // vinserti128 m6, [r6+128* 1], 1
            m[7] = zx(_mm_load_si128(r6.wrapping_add(128 * 8) as *const __m128i)); // mova xm7, [r6+128* 8]
            m[7] = _mm256_inserti128_si256::<1>(m[7], _mm_load_si128(r6 as *const __m128i)); // vinserti128 m7, [r6+128* 0], 1
            m[8] = _mm256_unpackhi_epi16(m[0], m[1]); // punpckhwd m8, m0, m1
            m[0] = _mm256_unpacklo_epi16(m[0], m[1]); // punpcklwd m0, m1
            m[1] = _mm256_unpackhi_epi16(m[2], m[3]); // punpckhwd m1, m2, m3
            m[2] = _mm256_unpacklo_epi16(m[2], m[3]); // punpcklwd m2, m3
            m[3] = _mm256_unpackhi_epi16(m[4], m[5]); // punpckhwd m3, m4, m5
            m[4] = _mm256_unpacklo_epi16(m[4], m[5]); // punpcklwd m4, m5
            m[5] = _mm256_unpackhi_epi16(m[6], m[7]); // punpckhwd m5, m6, m7
            m[6] = _mm256_unpacklo_epi16(m[6], m[7]); // punpcklwd m6, m7
            r6 = r6.wrapping_offset(-16); // sub r6, 16
            m[7] = _mm256_unpackhi_epi32(m[8], m[1]); // punpckhdq m7, m8, m1
            m[8] = _mm256_unpacklo_epi32(m[8], m[1]); // punpckldq m8, m1
            m[1] = _mm256_unpackhi_epi32(m[3], m[5]); // punpckhdq m1, m3, m5
            m[3] = _mm256_unpacklo_epi32(m[3], m[5]); // punpckldq m3, m5
            m[5] = _mm256_unpackhi_epi64(m[7], m[1]); // punpckhqdq m5, m7, m1
            m[7] = _mm256_unpacklo_epi64(m[7], m[1]); // punpcklqdq m7, m1
            m[1] = _mm256_unpackhi_epi64(m[8], m[3]); // punpckhqdq m1, m8, m3
            m[8] = _mm256_unpacklo_epi64(m[8], m[3]); // punpcklqdq m8, m3
            m[3] = _mm256_unpackhi_epi32(m[0], m[2]); // punpckhdq m3, m0, m2
            sy_q5(dstq, 0, m[5]); // mova [dstq+strideq*0], m5
            m[0] = _mm256_unpacklo_epi32(m[0], m[2]); // punpckldq m0, m2
            sy_q5(dstq, stride, m[7]); // mova [dstq+strideq*1], m7
            m[2] = _mm256_unpackhi_epi32(m[4], m[6]); // punpckhdq m2, m4, m6
            sy_q5(dstq, stride * 2, m[1]); // mova [dstq+strideq*2], m1
            m[4] = _mm256_unpacklo_epi32(m[4], m[6]); // punpckldq m4, m6
            sy_q5(dstq, r2, m[8]); // mova [dstq+r2], m8
            m[6] = _mm256_unpackhi_epi64(m[3], m[2]); // punpckhqdq m6, m3, m2
            sy_q5(dstq, stride * 4, m[6]); // mova [dstq+strideq*4], m6
            m[3] = _mm256_unpacklo_epi64(m[3], m[2]); // punpcklqdq m3, m2
            sy_q5(dstq, r3, m[3]); // mova [dstq+r3], m3
            m[2] = _mm256_unpackhi_epi64(m[0], m[4]); // punpckhqdq m2, m0, m4
            sy_q5(dstq, r2 * 2, m[2]); // mova [dstq+r2*2], m2
            m[0] = _mm256_unpacklo_epi64(m[0], m[4]); // punpcklqdq m0, m4
            sy_q5(dstq, r4, m[0]); // mova [dstq+r4], m0
            dstq = at(dstq, stride * 8); // lea dstq, [dstq+strideq*8]
            if (r6 as usize) < (sp as usize) {
                break; // cmp r6, rsp ; jae .h64_transpose_loop
            }
        }
        sp = sp.wrapping_add(128 * 16); // add rsp, 128*16
        org_w -= 16; // sub org_wd, 16
        if org_w <= 0 {
            break; // jg .h64_transpose_loop0
        }
    }
    // .h64_end: RET
}

// ---- part 6: `ipred16_avx2.asm` lines 3724–4405 (FILTER_1BLK, FILTER_2BLK, filter; IPRED_CFL, cfl_top, cfl_left, cfl, cfl_128) ----
//
// Correspondence with the assembly (part 6):
// - `FILTER_1BLK` and `FILTER_2BLK` are `filter_1blk` and `filter_2blk` (register arguments as register numbers;
//   `filter_1blk` takes the shuffle's value, so that both the register form `xm%4` and the memory form
//   `[base+filter_shuf2]` are one function). `cglobal ipred_filter_16bpc` is `dav1d_ipred_filter_16bpc_avx2`; its
//   jump table is a `match` on `tzcnt(w)`; the width paths `.w4`, `.w8`, `.w16`, `.w32` are
//   `ipred_filter_16bpc_w{4,8,16,32}`, `.w16_main` (tail-jumped from `.w16`, `call`ed from `.w32`) is
//   `ipred_filter_16bpc_w16_main`, `.main` is `ipred_filter_16bpc_main`. The row loops keep the assembly's
//   counters: the code from `.wN_loop_start` to the loop branch is the loop body, the code from `.wN_loop` to
//   `.wN_loop_start` its tail; `.w32_loop_left` / `.w32_loop_last` are the branches on `h` at the end of the body.
// - `IPRED_CFL` is `ipred_cfl`. `cglobal ipred_cfl_16bpc` is `dav1d_ipred_cfl_16bpc_avx2`; its two jumps
//   (`.h4`…`.h32` by `tzcnt(h)`, `.w4`…`.w32` by `tzcnt(w)`) are `match`es; `.wN` with its `.wN_end` is
//   `ipred_cfl_16bpc_wN`, the splat loops `.s4`…`.s32` (`ipred_cfl_splat_16bpc_avx2_table`, also the target of
//   cfl_top, cfl_left and cfl_128) are `ipred_cfl_16bpc_sN`, dispatched by `ipred_cfl_16bpc_splat`. The labels
//   `.h32` → `.h16` → `.h8` → `.h4` of cfl_left (which cfl_top also jumps into, indexed by `tzcnt(w)`) are
//   `ipred_cfl_left_16bpc_h{32,16,8,4}`, each ending in the next (fall-through), dispatched by
//   `ipred_cfl_left_16bpc_h`.

/// `tl + off` bytes for a read-only edge pointer, wrapping (no claim that the address is in bounds until it is
/// accessed).
#[inline(always)]
fn atc_q6(p: *const u8, off: isize) -> *const u8 {
    p.wrapping_offset(off)
}

/// `[p]`, 32 bytes, unaligned.
#[inline(always)]
unsafe fn ld32_q6(p: *const u8) -> __m256i {
    _mm256_loadu_si256(p as *const __m256i)
}

/// `[p]`, 16 bytes, unaligned.
#[inline(always)]
unsafe fn ld16_q6(p: *const u8) -> __m128i {
    _mm_loadu_si128(p as *const __m128i)
}

/// A 16-bit word at `p` (memory operand of `pinsrw`).
#[inline(always)]
unsafe fn ldw_q6(p: *const u8) -> i32 {
    (p as *const i16).read_unaligned() as i32
}

/// `FILTER_1BLK dst, src, tmp, shuf, bdmax`; `shuf` is the shuffle operand's value (`xm%4` or the memory operand).
#[inline(always)]
unsafe fn filter_1blk(m: &mut M, d: usize, s: usize, t: usize, shuf: __m128i, bd: usize) {
    m[s] = zx(_mm_shuffle_epi8(lo(m[s]), shuf)); // pshufb xm%2, xm%4 (or %4)
    m[s] = _mm256_inserti128_si256::<1>(m[s], lo(m[s])); // vinserti128 m%2, xm%2, 1
    m[d] = _mm256_shuffle_epi32::<0x00>(m[s]); // pshufd m%1, m%2, q0000
    m[d] = _mm256_madd_epi16(m[d], m[2]); // pmaddwd m%1, m2
    m[t] = _mm256_shuffle_epi32::<0x55>(m[s]); // pshufd m%3, m%2, q1111
    m[t] = _mm256_madd_epi16(m[t], m[3]); // pmaddwd m%3, m3
    m[d] = _mm256_add_epi32(m[d], m[1]); // paddd m%1, m1
    m[d] = _mm256_add_epi32(m[d], m[t]); // paddd m%1, m%3
    m[t] = _mm256_shuffle_epi32::<0xaa>(m[s]); // pshufd m%3, m%2, q2222
    m[t] = _mm256_madd_epi16(m[t], m[4]); // pmaddwd m%3, m4
    m[d] = _mm256_add_epi32(m[d], m[t]); // paddd m%1, m%3
    m[t] = _mm256_shuffle_epi32::<0xff>(m[s]); // pshufd m%3, m%2, q3333
    m[t] = _mm256_madd_epi16(m[t], m[5]); // pmaddwd m%3, m5
    m[d] = _mm256_add_epi32(m[d], m[t]); // paddd m%1, m%3
    m[d] = _mm256_srai_epi32::<4>(m[d]); // psrad m%1, 4
    m[d] = _mm256_packus_epi32(m[d], m[d]); // packusdw m%1, m%1
    m[d] = _mm256_min_epi16(m[d], m[bd]); // pminsw m%1, m%5
}

/// `FILTER_2BLK dst, src, tmp_dst, tmp_src, tmp, shuf, bdmax`.
#[inline(always)]
unsafe fn filter_2blk(m: &mut M, d: usize, s: usize, td: usize, ts: usize, t: usize, shuf: usize, bd: usize) {
    m[s] = _mm256_shuffle_epi8(m[s], m[shuf]); // pshufb m%2, m%6
    m[ts] = _mm256_permute4x64_epi64::<0xee>(m[s]); // vpermq m%4, m%2, q3232
    m[s] = _mm256_inserti128_si256::<1>(m[s], lo(m[s])); // vinserti128 m%2, xm%2, 1
    m[d] = _mm256_shuffle_epi32::<0x00>(m[s]); // pshufd m%1, m%2, q0000
    m[td] = _mm256_shuffle_epi32::<0x00>(m[ts]); // pshufd m%3, m%4, q0000
    m[d] = _mm256_madd_epi16(m[d], m[2]); // pmaddwd m%1, m2
    m[td] = _mm256_madd_epi16(m[td], m[2]); // pmaddwd m%3, m2
    m[d] = _mm256_add_epi32(m[d], m[1]); // paddd m%1, m1
    m[td] = _mm256_add_epi32(m[td], m[1]); // paddd m%3, m1
    m[t] = _mm256_shuffle_epi32::<0x55>(m[s]); // pshufd m%5, m%2, q1111
    m[t] = _mm256_madd_epi16(m[t], m[3]); // pmaddwd m%5, m3
    m[d] = _mm256_add_epi32(m[d], m[t]); // paddd m%1, m%5
    m[t] = _mm256_shuffle_epi32::<0x55>(m[ts]); // pshufd m%5, m%4, q1111
    m[t] = _mm256_madd_epi16(m[t], m[3]); // pmaddwd m%5, m3
    m[td] = _mm256_add_epi32(m[td], m[t]); // paddd m%3, m%5
    m[t] = _mm256_shuffle_epi32::<0xaa>(m[s]); // pshufd m%5, m%2, q2222
    m[t] = _mm256_madd_epi16(m[t], m[4]); // pmaddwd m%5, m4
    m[d] = _mm256_add_epi32(m[d], m[t]); // paddd m%1, m%5
    m[t] = _mm256_shuffle_epi32::<0xaa>(m[ts]); // pshufd m%5, m%4, q2222
    m[t] = _mm256_madd_epi16(m[t], m[4]); // pmaddwd m%5, m4
    m[td] = _mm256_add_epi32(m[td], m[t]); // paddd m%3, m%5
    m[t] = _mm256_shuffle_epi32::<0xff>(m[s]); // pshufd m%5, m%2, q3333
    m[t] = _mm256_madd_epi16(m[t], m[5]); // pmaddwd m%5, m5
    m[d] = _mm256_add_epi32(m[d], m[t]); // paddd m%1, m%5
    m[t] = _mm256_shuffle_epi32::<0xff>(m[ts]); // pshufd m%5, m%4, q3333
    m[t] = _mm256_madd_epi16(m[t], m[5]); // pmaddwd m%5, m5
    m[td] = _mm256_add_epi32(m[td], m[t]); // paddd m%3, m%5
    m[d] = _mm256_srai_epi32::<4>(m[d]); // psrad m%1, 4
    m[td] = _mm256_srai_epi32::<4>(m[td]); // psrad m%3, 4
    m[d] = _mm256_packus_epi32(m[d], m[td]); // packusdw m%1, m%3
    m[d] = _mm256_min_epi16(m[d], m[bd]); // pminsw m%1, m%7
}

/// `ipred_filter_16bpc.main`.
#[inline(always)]
unsafe fn ipred_filter_16bpc_main(m: &mut M) {
    filter_2blk(m, 9, 8, 6, 7, 0, 14, 15); // FILTER_2BLK 9, 8, 6, 7, 0, 14, 15
}

/// `ipred_filter_16bpc.w4`.
#[inline(always)]
unsafe fn ipred_filter_16bpc_w4(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    mut h: isize,
    bdmax: c_int,
) {
    m[8] = zx(ld16_q6(ro(filter_shuf2, 0))); // mova xm8, [base+filter_shuf2]
    m[9] = _mm256_set1_epi16(bdmax as i16); // vpbroadcastw m9, r8m
    let r7 = 6 + h * 2; // lea r7, [6+hq*2]
    tl = atc_q6(tl, -r7); // sub tlq, r7
    loop {
        // .w4_loop_start
        filter_1blk(m, 6, 0, 7, lo(m[8]), 9); // FILTER_1BLK 6, 0, 7, 8, 9
        m[0] = zx(_mm256_extracti128_si256::<1>(m[6])); // vextracti128 xm0, m6, 1
        _mm_storel_epi64(dst as *mut __m128i, lo(m[6])); // movq [dstq+strideq*0], xm6
        _mm_storel_epi64(at(dst, stride) as *mut __m128i, lo(m[0])); // movq [dstq+strideq*1], xm0
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w4_loop
        }
        // .w4_loop
        let q = (atc_q6(tl, h * 2) as *const i64).read_unaligned();
        m[0] = zx(_mm_insert_epi64::<0>(lo(m[0]), q)); // pinsrq xm0, [tlq+hq*2], 0
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
    }
}

/// `ipred_filter_16bpc.w8`.
#[inline(always)]
unsafe fn ipred_filter_16bpc_w8(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    mut h: isize,
    bdmax: c_int,
) {
    m[14] = _mm256_broadcastsi128_si256(ld16_q6(ro(filter_shuf3, 0))); // vbroadcasti128 m14, [base+filter_shuf3]
    m[15] = _mm256_set1_epi16(bdmax as i16); // vpbroadcastw m15, r8m
    filter_1blk(m, 10, 0, 7, ld16_q6(ro(filter_shuf2, 0)), 15); // FILTER_1BLK 10, 0, 7, [base+filter_shuf2], 15
    m[6] = _mm256_permute4x64_epi64::<0x72>(m[10]); // vpermq m6, m10, q1302
    m[8] = _mm256_bslli_epi128::<4>(m[0]); // pslldq m8, m0, 4
    m[7] = _mm256_bsrli_epi128::<2>(m[6]); // psrldq m7, m6, 2
    m[0] = _mm256_bsrli_epi128::<10>(m[6]); // psrldq m0, m6, 10
    m[7] = _mm256_unpacklo_epi16(m[7], m[0]); // punpcklwd m7, m0
    m[8] = _mm256_blend_epi32::<0x33>(m[8], m[6]); // vpblendd m8, m6, 0x33
    m[8] = _mm256_blend_epi32::<0x40>(m[8], m[7]); // vpblendd m8, m7, 0x40
    m[8] = _mm256_blend_epi32::<0x30>(m[8], ld32_q6(atc_q6(tl, -6))); // vpblendd m8, [tlq-6], 0x30
    let r7 = 16 + h * 2; // lea r7, [16+hq*2]
    tl = atc_q6(tl, -r7); // sub tlq, r7
    loop {
        // .w8_loop_start
        m[8] = _mm256_blend_epi32::<0x0c>(m[8], ld32_q6(atc_q6(tl, h * 2))); // vpblendd m8, [tlq+hq*2], 0x0C
        ipred_filter_16bpc_main(m); // call .main
        m[10] = _mm256_blend_epi32::<0xcc>(m[10], m[9]); // vpblendd m10, m9, 0xCC
        _mm_storeu_si128(dst as *mut __m128i, lo(m[10])); // mova [dstq+strideq*0], xm10
        _mm_storeu_si128(at(dst, stride) as *mut __m128i, _mm256_extracti128_si256::<1>(m[10])); // vextracti128 [dstq+strideq*1], m10, 1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w8_loop
        }
        // .w8_loop
        m[8] = _mm256_permute4x64_epi64::<0x72>(m[9]); // vpermq m8, m9, q1302
        m[6] = _mm256_permute4x64_epi64::<0x8d>(m[9]); // vpermq m6, m9, q2031
        m[0] = _mm256_bsrli_epi128::<2>(m[6]); // psrldq m0, m6, 2
        m[6] = _mm256_bsrli_epi128::<10>(m[6]); // psrldq m6, 10
        m[6] = _mm256_unpacklo_epi16(m[6], m[0]); // punpcklwd m6, m0
        m[8] = _mm256_blend_epi32::<0x80>(m[8], m[7]); // vpblendd m8, m7, 0x80
        m[8] = _mm256_blend_epi32::<0x40>(m[8], m[6]); // vpblendd m8, m6, 0x40
        m[10] = m[9]; // mova m10, m9
    }
}

/// `ipred_filter_16bpc.w16_main`: the left 16 columns (all of `.w16`, the first half of `.w32`). `s` is the
/// 32-byte slot `[rsp+8]` (the caller's frame; written at `.w16_loop_start` before `.w16_loop` reads it). Returns
/// `tlq` as the label leaves it (`.w32` continues from it).
#[inline(always)]
unsafe fn ipred_filter_16bpc_w16_main(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    mut h: isize,
    s: *mut __m256i,
) -> *const u8 {
    m[10] = zx(ld16_q6(ro(filter_shuf2, 0))); // mova xm10, [base+filter_shuf2]
    filter_1blk(m, 13, 0, 6, lo(m[10]), 15); // FILTER_1BLK 13, 0, 6, 10, 15
    m[12] = _mm256_permute4x64_epi64::<0xd8>(m[13]); // vpermq m12, m13, q3120
    m[14] = zx(ld16_q6(ro(filter_shuf3, 0))); // mova xm14, [base+filter_shuf3]
    m[14] = _mm256_inserti128_si256::<1>(m[14], ld16_q6(ro(filter_shuf1, 0))); // vinserti128 m14, [base+filter_shuf1], 1
    m[0] = _mm256_broadcastq_epi64(_mm_loadl_epi64(atc_q6(tl, 10) as *const __m128i)); // vpbroadcastq m0, [tlq+10]
    m[0] = _mm256_blend_epi32::<0x4c>(m[0], ld32_q6(atc_q6(tl, -16))); // vpblendd m0, [tlq-16], 0x4C
    m[6] = _mm256_bsrli_epi128::<8>(m[12]); // psrldq m6, m12, 8
    m[0] = _mm256_blend_epi32::<0x03>(m[0], m[6]); // vpblendd m0, m6, 0x03
    m[6] = _mm256_unpacklo_epi16(m[6], m[12]); // punpcklwd m6, m12
    m[0] = _mm256_blend_epi32::<0x80>(m[0], m[6]); // vpblendd m0, m6, 0x80
    filter_2blk(m, 12, 0, 6, 7, 8, 14, 15); // FILTER_2BLK 12, 0, 6, 7, 8, 14, 15
    m[13] = _mm256_blend_epi32::<0xcc>(m[13], m[12]); // vpblendd m13, m12, 0xCC
    m[12] = _mm256_permute4x64_epi64::<0x8d>(m[12]); // vpermq m12, m12, q2031
    m[6] = zx(_mm_bsrli_si128::<2>(lo(m[12]))); // psrldq xm6, xm12, 2
    m[8] = zx(_mm_bsrli_si128::<12>(lo(m[12]))); // psrldq xm8, xm12, 12
    m[6] = zx(_mm_blend_epi32::<0x01>(lo(m[6]), lo(m[8]))); // vpblendd xm6, xm8, 0x01
    m[6] = zx(_mm_blend_epi16::<0xf8>(lo(m[6]), ld16_q6(atc_q6(tl, 10)))); // pblendw xm6, [tlq+10], 0xF8
    filter_1blk(m, 11, 6, 8, lo(m[10]), 15); // FILTER_1BLK 11, 6, 8, 10, 15
    m[11] = _mm256_permute4x64_epi64::<0xd8>(m[11]); // vpermq m11, m11, q3120
    m[9] = _mm256_shuffle_epi32::<0x4e>(m[11]); // pshufd m9, m11, q1032
    m[8] = ld32_q6(atc_q6(tl, 6)); // movu m8, [tlq+6]
    m[8] = _mm256_shuffle_epi32::<0xc9>(m[8]); // pshufd m8, m8, q3021
    m[8] = _mm256_shufflehi_epi16::<0xe1>(m[8]); // pshufhw m8, m8, q3201
    m[9] = _mm256_blend_epi32::<0x70>(m[9], m[8]); // vpblendd m9, m8, 0x70
    _mm_storeu_si128(dst as *mut __m128i, lo(m[13])); // mova [dstq+strideq*0], xm13
    _mm_storeu_si128(at(dst, stride) as *mut __m128i, _mm256_extracti128_si256::<1>(m[13])); // vextracti128 [dstq+strideq*1], m13, 1
    let r7 = 20 + h * 2; // lea r7, [20+hq*2]
    tl = atc_q6(tl, -r7); // sub tlq, r7
    m[6] = _mm256_permute4x64_epi64::<0x1b>(m[12]); // vpermq m6, m12, q0123
    loop {
        // .w16_loop_start
        m[13] = m[12]; // mova m13, m12
        m[0] = _mm256_blend_epi32::<0x0c>(m[0], ld32_q6(atc_q6(tl, h * 2))); // vpblendd m0, [tlq+hq*2], 0x0C
        m[7] = _mm256_bsrli_epi128::<8>(m[12]); // psrldq m7, m12, 8
        m[7] = _mm256_unpacklo_epi16(m[7], m[12]); // punpcklwd m7, m12
        m[0] = _mm256_blend_epi32::<0x33>(m[0], m[6]); // vpblendd m0, m6, 0x33
        m[0] = _mm256_blend_epi32::<0x80>(m[0], m[7]); // vpblendd m0, m7, 0x80
        filter_2blk(m, 10, 0, 6, 7, 8, 14, 15); // FILTER_2BLK 10, 0, 6, 7, 8, 14, 15
        m[12] = _mm256_permute4x64_epi64::<0x8d>(m[10]); // vpermq m12, m10, q2031
        _mm256_store_si256(s, m[0]); // mova [rsp+8], m0
        m[8] = _mm256_bsrli_epi128::<8>(m[11]); // psrldq m8, m11, 8
        m[6] = zx(_mm_bsrli_si128::<2>(lo(m[12]))); // psrldq xm6, xm12, 2
        m[7] = zx(_mm_bsrli_si128::<10>(lo(m[12]))); // psrldq xm7, xm12, 10
        m[0] = zx(_mm_bsrli_si128::<2>(lo(m[13]))); // psrldq xm0, xm13, 2
        m[8] = _mm256_unpacklo_epi16(m[8], m[11]); // punpcklwd m8, m11
        m[7] = zx(_mm_unpacklo_epi16(lo(m[7]), lo(m[6]))); // punpcklwd xm7, xm6
        m[8] = _mm256_blend_epi32::<0x73>(m[8], m[9]); // vpblendd m8, m9, 0x73
        m[8] = _mm256_blend_epi32::<0x04>(m[8], m[7]); // vpblendd m8, m7, 0x04
        m[8] = _mm256_blend_epi32::<0x08>(m[8], m[0]); // vpblendd m8, m0, 0x08
        ipred_filter_16bpc_main(m); // call .main
        m[8] = _mm256_permute4x64_epi64::<0xd8>(m[11]); // vpermq m8, m11, q3120
        m[6] = _mm256_blend_epi32::<0xcc>(m[8], m[9]); // vpblendd m6, m8, m9, 0xCC
        _mm_storeu_si128(at(dst, 16) as *mut __m128i, lo(m[6])); // mova [dstq+strideq*0+16], xm6
        _mm_storeu_si128(
            at(dst, stride + 16) as *mut __m128i,
            _mm256_extracti128_si256::<1>(m[6]),
        ); // vextracti128 [dstq+strideq*1+16], m6, 1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w16_loop
        }
        // .w16_loop
        m[13] = _mm256_permute4x64_epi64::<0xfa>(m[13]); // vpermq m13, m13, q3322
        m[11] = _mm256_permute4x64_epi64::<0x88>(m[9]); // vpermq m11, m9, q2020
        m[9] = _mm256_permute4x64_epi64::<0x72>(m[9]); // vpermq m9, m9, q1302
        m[6] = _mm256_permute4x64_epi64::<0x1b>(m[12]); // vpermq m6, m12, q0123
        m[7] = _mm256_bsrli_epi128::<4>(m[7]); // psrldq m7, 4
        m[13] = _mm256_blend_epi32::<0xcc>(m[13], m[10]); // vpblendd m13, m10, 0xCC
        m[9] = _mm256_blend_epi32::<0x40>(m[9], m[7]); // vpblendd m9, m7, 0x40
        m[0] = _mm256_load_si256(s); // mova m0, [rsp+8]
        _mm_storeu_si128(dst as *mut __m128i, lo(m[13])); // mova [dstq+strideq*0], xm13
        _mm_storeu_si128(at(dst, stride) as *mut __m128i, _mm256_extracti128_si256::<1>(m[13]));
        // vextracti128 [dstq+strideq*1], m13, 1
    }
    ipred_filter_16bpc_end_q6(m, dst, stride);
    tl // ret
}

/// The last block pair after `.w16_loop` and `.w32_loop` (the same twelve instructions at both places).
#[inline(always)]
unsafe fn ipred_filter_16bpc_end_q6(m: &mut M, dst: *mut u8, stride: isize) {
    m[8] = _mm256_permute4x64_epi64::<0xd8>(m[9]); // vpermq m8, m9, q3120
    m[0] = zx(_mm256_extracti128_si256::<1>(m[8])); // vextracti128 xm0, m8, 1
    m[11] = zx(_mm_shuffle_epi32::<0x4e>(lo(m[11]))); // pshufd xm11, xm11, q1032
    m[0] = zx(_mm_blend_epi32::<0x02>(lo(m[0]), lo(m[11]))); // vpblendd xm0, xm11, 0x02
    m[6] = zx(_mm_bsrli_si128::<2>(lo(m[8]))); // psrldq xm6, xm8, 2
    m[7] = zx(_mm_bsrli_si128::<12>(lo(m[8]))); // psrldq xm7, xm8, 12
    m[0] = zx(_mm_blend_epi16::<0x4>(lo(m[0]), lo(m[6]))); // pblendw xm0, xm6, 0x4
    m[0] = zx(_mm_blend_epi16::<0x2>(lo(m[0]), lo(m[7]))); // pblendw xm0, xm7, 0x2
    filter_1blk(m, 6, 0, 7, ld16_q6(ro(filter_shuf2, 0)), 15); // FILTER_1BLK 6, 0, 7, [base+filter_shuf2], 15
    m[12] = _mm256_permute4x64_epi64::<0x72>(m[13]); // vpermq m12, m13, q1302
    m[12] = _mm256_blend_epi32::<0xcc>(m[12], m[10]); // vpblendd m12, m10, 0xCC
    m[9] = _mm256_blend_epi32::<0xcc>(m[9], m[6]); // vpblendd m9, m6, 0xCC
    _mm_storeu_si128(dst as *mut __m128i, lo(m[12])); // mova [dstq+strideq*0+ 0], xm12
    _mm_storeu_si128(at(dst, 16) as *mut __m128i, lo(m[9])); // mova [dstq+strideq*0+16], xm9
    _mm_storeu_si128(at(dst, stride) as *mut __m128i, _mm256_extracti128_si256::<1>(m[12])); // vextracti128 [dstq+strideq*1+ 0], m12, 1
    _mm_storeu_si128(
        at(dst, stride + 16) as *mut __m128i,
        _mm256_extracti128_si256::<1>(m[9]),
    ); // vextracti128 [dstq+strideq*1+16], m9, 1
}

/// `ipred_filter_16bpc.w16`.
#[inline(always)]
unsafe fn ipred_filter_16bpc_w16(m: &mut M, dst: *mut u8, stride: isize, tl: *const u8, mut h: isize, bdmax: c_int) {
    // ALLOC_STACK 32, 16: the frame is the slot `[rsp+8]` of `.w16_main`, a local here (written before it is read,
    // see `ipred_filter_16bpc_w16_main`; initialized anyway, the compiler keeps it in a register).
    let mut s = _mm256_setzero_si256();
    m[15] = _mm256_set1_epi16(bdmax as i16); // vpbroadcastw m15, r8m
    h -= 2; // sub hd, 2
    ipred_filter_16bpc_w16_main(m, dst, stride, tl, h, &mut s); // TAIL_CALL .w16_main, 0
}

/// The `ALLOC_STACK 64, 16` frame of `.w32`: `[rsp+0]` the 32-byte slot (also `.w16_main`'s `[rsp+8]`, which
/// `call` shifts by the return address), `[rsp+32]` the left pixels of the next rows. 96 bytes, because the load
/// `[rsp+32+r7*4-12]` (r7 up to 4) reaches byte 67; zeroed whole.
#[repr(C, align(32))]
struct FilterFrameQ6([u8; 96]);

/// `ipred_filter_16bpc.w32`.
#[inline(always)]
unsafe fn ipred_filter_16bpc_w32(m: &mut M, dst: *mut u8, stride: isize, tl: *const u8, mut h: isize, bdmax: c_int) {
    let mut frame = FilterFrameQ6([0; 96]); // ALLOC_STACK 64, 16 (zeroed whole)
    let rsp = frame.0.as_mut_ptr();
    m[15] = _mm256_set1_epi16(bdmax as i16); // vpbroadcastw m15, r8m
    h -= 2; // sub hd, 2
    let r3 = at(dst, 32); // lea r3, [dstq+32]
    let mut r5 = h * 2 + 20; // lea r5d, [hd*2+20]
    let mut tl = ipred_filter_16bpc_w16_main(m, dst, stride, tl, h, rsp as *mut __m256i); // call .w16_main
    let mut dst = r3; // mov dstq, r3
    tl = atc_q6(tl, r5 + 32); // lea tlq, [tlq+r5+32]
    r5 -= 20; // sub r5d, 20
    r5 >>= 1; // shr r5d, 1
    r5 -= 2; // sub r5d, 2
    let mut left = at(dst, stride * 2 - 2) as *const u8; // lea r4, [dstq+strideq*2-2]
    let mut h = r5; // DEFINE_ARGS dst, stride, tl, stride3, left, h
    let stride3 = stride * 3; // lea stride3q, [strideq*3]
    m[8] = ld32_q6(atc_q6(tl, -6)); // movu m8, [tlq-6]
    m[10] = zx(ld16_q6(ro(filter_shuf2, 0))); // mova xm10, [base+filter_shuf2]
    m[0] = zx(_mm_insert_epi16::<2>(lo(m[8]), ldw_q6(at(dst, -2)))); // pinsrw xm0, xm8, [dstq+strideq*0-2], 2
    m[0] = zx(_mm_insert_epi16::<1>(lo(m[0]), ldw_q6(at(dst, stride - 2)))); // pinsrw xm0, xm0, [dstq+strideq*1-2], 1
    m[9] = zx(_mm_insert_epi16::<5>(lo(m[9]), ldw_q6(left))); // pinsrw xm9, [leftq+strideq*0], 5
    m[9] = zx(_mm_insert_epi16::<4>(lo(m[9]), ldw_q6(atc_q6(left, stride)))); // pinsrw xm9, [leftq+strideq*1], 4
    filter_1blk(m, 13, 0, 6, lo(m[10]), 15); // FILTER_1BLK 13, 0, 6, 10, 15
    m[12] = _mm256_permute4x64_epi64::<0xd8>(m[13]); // vpermq m12, m13, q3120
    m[14] = zx(ld16_q6(ro(filter_shuf3, 0))); // mova xm14, [base+filter_shuf3]
    m[14] = _mm256_inserti128_si256::<1>(m[14], ld16_q6(ro(filter_shuf1, 0))); // vinserti128 m14, [base+filter_shuf1], 1
    m[6] = _mm256_bsrli_epi128::<8>(m[12]); // psrldq m6, m12, 8
    m[7] = _mm256_unpacklo_epi16(m[6], m[12]); // punpcklwd m7, m6, m12
    m[0] = _mm256_blend_epi32::<0x03>(m[0], m[6]); // vpblendd m0, m6, 0x03
    m[0] = _mm256_blend_epi32::<0x80>(m[0], m[7]); // vpblendd m0, m7, 0x80
    m[0] = _mm256_blend_epi32::<0x30>(m[0], m[8]); // vpblendd m0, m8, 0x30
    m[0] = _mm256_blend_epi32::<0x04>(m[0], m[9]); // vpblendd m0, m9, 0x04
    filter_2blk(m, 12, 0, 6, 7, 8, 14, 15); // FILTER_2BLK 12, 0, 6, 7, 8, 14, 15
    m[13] = _mm256_blend_epi32::<0xcc>(m[13], m[12]); // vpblendd m13, m12, 0xCC
    m[9] = zx(_mm_insert_epi16::<3>(lo(m[9]), ldw_q6(atc_q6(left, stride * 2)))); // pinsrw xm9, [leftq+strideq*2], 3
    m[9] = zx(_mm_insert_epi16::<2>(lo(m[9]), ldw_q6(atc_q6(left, stride3)))); // pinsrw xm9, [leftq+stride3q ], 2
    left = atc_q6(left, stride * 4); // lea leftq, [leftq+strideq*4]
    m[9] = zx(_mm_insert_epi16::<1>(lo(m[9]), ldw_q6(left))); // pinsrw xm9, [leftq+strideq*0], 1
    m[9] = zx(_mm_insert_epi16::<0>(lo(m[9]), ldw_q6(atc_q6(left, stride)))); // pinsrw xm9, [leftq+strideq*1], 0
    _mm_storel_epi64(rsp.add(32) as *mut __m128i, lo(m[9])); // movq [rsp+32], xm9
    let mut r7: i32 = 1; // mov r7d, 1
    m[8] = _mm256_bslli_epi128::<4>(m[9]); // pslldq m8, m9, 4
    m[0] = _mm256_blend_epi32::<0x0c>(m[0], m[8]); // vpblendd m0, m8, 0x0C
    m[12] = _mm256_permute4x64_epi64::<0x8d>(m[12]); // vpermq m12, m12, q2031
    m[6] = zx(_mm_bsrli_si128::<2>(lo(m[12]))); // psrldq xm6, xm12, 2
    m[7] = zx(_mm_bsrli_si128::<12>(lo(m[12]))); // psrldq xm7, xm12, 12
    m[6] = zx(_mm_blend_epi32::<0x01>(lo(m[6]), lo(m[7]))); // vpblendd xm6, xm7, 0x01
    m[6] = zx(_mm_blend_epi16::<0xf8>(lo(m[6]), ld16_q6(atc_q6(tl, 10)))); // pblendw xm6, [tlq+10], 0xF8
    filter_1blk(m, 11, 6, 7, lo(m[10]), 15); // FILTER_1BLK 11, 6, 7, 10, 15
    m[11] = _mm256_permute4x64_epi64::<0xd8>(m[11]); // vpermq m11, m11, q3120
    m[9] = _mm256_shuffle_epi32::<0x4e>(m[11]); // pshufd m9, m11, q1032
    m[8] = _mm256_broadcastsi128_si256(ld16_q6(atc_q6(tl, 22))); // vbroadcasti128 m8, [tlq+22]
    m[8] = _mm256_shuffle_epi32::<0xc9>(m[8]); // pshufd m8, m8, q3021
    m[8] = _mm256_shufflehi_epi16::<0xe1>(m[8]); // pshufhw m8, m8, q3201
    m[9] = _mm256_blend_epi32::<0x70>(m[9], m[8]); // vpblendd m9, m8, 0x70
    _mm_storeu_si128(dst as *mut __m128i, lo(m[13])); // mova [dstq+strideq*0], xm13
    _mm_storeu_si128(at(dst, stride) as *mut __m128i, _mm256_extracti128_si256::<1>(m[13])); // vextracti128 [dstq+strideq*1], m13, 1
    m[6] = _mm256_permute4x64_epi64::<0x1b>(m[12]); // vpermq m6, m12, q0123
    loop {
        // .w32_loop_start
        m[13] = m[12]; // mova m13, m12
        m[7] = _mm256_bsrli_epi128::<8>(m[12]); // psrldq m7, m12, 8
        m[7] = _mm256_unpacklo_epi16(m[7], m[12]); // punpcklwd m7, m12
        m[0] = _mm256_blend_epi32::<0x33>(m[0], m[6]); // vpblendd m0, m6, 0x33
        m[0] = _mm256_blend_epi32::<0x80>(m[0], m[7]); // vpblendd m0, m7, 0x80
        filter_2blk(m, 10, 0, 6, 7, 8, 14, 15); // FILTER_2BLK 10, 0, 6, 7, 8, 14, 15
        m[12] = _mm256_permute4x64_epi64::<0x8d>(m[10]); // vpermq m12, m10, q2031
        _mm256_store_si256(rsp as *mut __m256i, m[0]); // mova [rsp+0], m0
        m[8] = _mm256_bsrli_epi128::<8>(m[11]); // psrldq m8, m11, 8
        m[6] = zx(_mm_bsrli_si128::<2>(lo(m[12]))); // psrldq xm6, xm12, 2
        m[7] = zx(_mm_bsrli_si128::<10>(lo(m[12]))); // psrldq xm7, xm12, 10
        m[0] = zx(_mm_bsrli_si128::<2>(lo(m[13]))); // psrldq xm0, xm13, 2
        m[8] = _mm256_unpacklo_epi16(m[8], m[11]); // punpcklwd m8, m11
        m[7] = zx(_mm_unpacklo_epi16(lo(m[7]), lo(m[6]))); // punpcklwd xm7, xm6
        m[8] = _mm256_blend_epi32::<0x73>(m[8], m[9]); // vpblendd m8, m9, 0x73
        m[8] = _mm256_blend_epi32::<0x04>(m[8], m[7]); // vpblendd m8, m7, 0x04
        m[8] = _mm256_blend_epi32::<0x08>(m[8], m[0]); // vpblendd m8, m0, 0x08
        ipred_filter_16bpc_main(m); // call .main
        m[8] = _mm256_permute4x64_epi64::<0xd8>(m[11]); // vpermq m8, m11, q3120
        m[6] = _mm256_blend_epi32::<0xcc>(m[8], m[9]); // vpblendd m6, m8, m9, 0xCC
        _mm_storeu_si128(at(dst, 16) as *mut __m128i, lo(m[6])); // mova [dstq+strideq*0+16], xm6
        _mm_storeu_si128(
            at(dst, stride + 16) as *mut __m128i,
            _mm256_extracti128_si256::<1>(m[6]),
        ); // vextracti128 [dstq+strideq*1+16], m6, 1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2; // sub hd, 2
        if h > 0 {
            // jg .w32_loop_left
            // .w32_loop_left
            m[0] = _mm256_load_si256(rsp as *const __m256i); // mova m0, [rsp+0]
                                                             // r7 is 1..=4 here: the 32 bytes at 32+r7*4-12 = 24..=36 lie inside the 96-byte frame.
            let p = rsp.add((32 + r7 * 4 - 12) as usize);
            m[0] = _mm256_blend_epi32::<0x0c>(m[0], ld32_q6(p)); // vpblendd m0, [rsp+32+r7*4-12], 0x0C
            r7 -= 1; // dec r7d
            if r7 <= 0 && h != 2 {
                // jg .w32_loop / cmp hd, 2 / je .w32_loop
                m[6] = zx(_mm_insert_epi16::<6>(lo(m[6]), ldw_q6(rsp.add(32)))); // pinsrw xm6, [rsp+32], 6
                m[6] = zx(_mm_insert_epi16::<5>(lo(m[6]), ldw_q6(atc_q6(left, stride * 2)))); // pinsrw xm6, [leftq+strideq*2], 5
                m[6] = zx(_mm_insert_epi16::<4>(lo(m[6]), ldw_q6(atc_q6(left, stride3)))); // pinsrw xm6, [leftq+stride3q ], 4
                left = atc_q6(left, stride * 4); // lea leftq, [leftq+strideq*4]
                m[6] = zx(_mm_insert_epi16::<3>(lo(m[6]), ldw_q6(left))); // pinsrw xm6, [leftq+strideq*0], 3
                m[6] = zx(_mm_insert_epi16::<2>(lo(m[6]), ldw_q6(atc_q6(left, stride)))); // pinsrw xm6, [leftq+strideq*1], 2
                m[6] = zx(_mm_insert_epi16::<1>(lo(m[6]), ldw_q6(atc_q6(left, stride * 2)))); // pinsrw xm6, [leftq+strideq*2], 1
                m[6] = zx(_mm_insert_epi16::<0>(lo(m[6]), ldw_q6(atc_q6(left, stride3)))); // pinsrw xm6, [leftq+stride3q ], 0
                left = atc_q6(left, stride * 4); // lea leftq, [leftq+strideq*4]
                _mm_storeu_si128(rsp.add(36) as *mut __m128i, lo(m[6])); // movu [rsp+36], xm6
                m[6] = zx(_mm_insert_epi16::<1>(lo(m[6]), ldw_q6(left))); // pinsrw xm6, [leftq+strideq*0], 1
                m[6] = zx(_mm_insert_epi16::<0>(lo(m[6]), ldw_q6(atc_q6(left, stride)))); // pinsrw xm6, [leftq+strideq*1], 0
                (rsp.add(32) as *mut i32).write_unaligned(_mm_cvtsi128_si32(lo(m[6]))); // movd [rsp+32], xm6
                r7 = 4; // mov r7d, 4
            }
        } else if h == 0 {
            // jz .w32_loop_last
            // .w32_loop_last
            m[0] = _mm256_load_si256(rsp as *const __m256i); // mova m0, [rsp+0]
        } else {
            break;
        }
        // .w32_loop
        m[13] = _mm256_permute4x64_epi64::<0xfa>(m[13]); // vpermq m13, m13, q3322
        m[11] = _mm256_permute4x64_epi64::<0x88>(m[9]); // vpermq m11, m9, q2020
        m[9] = _mm256_permute4x64_epi64::<0x72>(m[9]); // vpermq m9, m9, q1302
        m[6] = _mm256_permute4x64_epi64::<0x1b>(m[12]); // vpermq m6, m12, q0123
        m[7] = _mm256_bsrli_epi128::<4>(m[7]); // psrldq m7, 4
        m[13] = _mm256_blend_epi32::<0xcc>(m[13], m[10]); // vpblendd m13, m10, 0xCC
        m[9] = _mm256_blend_epi32::<0x40>(m[9], m[7]); // vpblendd m9, m7, 0x40
        _mm_storeu_si128(dst as *mut __m128i, lo(m[13])); // mova [dstq+strideq*0], xm13
        _mm_storeu_si128(at(dst, stride) as *mut __m128i, _mm256_extracti128_si256::<1>(m[13]));
        // vextracti128 [dstq+strideq*1], m13, 1
    }
    ipred_filter_16bpc_end_q6(m, dst, stride); // RET
}

/// `ipred_filter_16bpc` (filter intra prediction; `angle & 0xff` is the filter index, `filt_idx < 5` by contract).
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_filter_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
    bitdepth_max: c_int,
) {
    let mut regs_ = regs();
    let m = &mut regs_;
    let dst = dst as *mut u8;
    let tl = topleft as *const u8;
    let r6 = core::ptr::addr_of!(crate::common::tables::dav1d_filter_intra_taps) as *const u8; // lea r6, [filter_intra_taps]
    let wd = (width as u32).trailing_zeros(); // tzcnt wd, wm (width is a power of two, 4..=32, never zero)
    let mut filter = (angle as u32 & 0xff) as usize; // movzx filterd, filterb
    filter <<= 6; // shl filterd, 6
                  // The filter index is below 5 (the contract; C asserts it): the four 16-byte loads lie inside its 64-byte row
                  // of the 5x64 table.
    let filterq = r6.add(filter); // add filterq, r6
    m[0] = _mm256_broadcastsi128_si256(ld16_q6(atc_q6(tl, -6))); // vbroadcasti128 m0, [tlq-6]
    m[1] = _mm256_set1_epi32((ro(pd_8, 0) as *const i32).read_unaligned()); // vpbroadcastd m1, [base+pd_8]
    m[2] = _mm256_cvtepi8_epi16(ld16_q6(filterq)); // pmovsxbw m2, [filterq+16*0]
    m[3] = _mm256_cvtepi8_epi16(ld16_q6(filterq.add(16))); // pmovsxbw m3, [filterq+16*1]
    m[4] = _mm256_cvtepi8_epi16(ld16_q6(filterq.add(32))); // pmovsxbw m4, [filterq+16*2]
    m[5] = _mm256_cvtepi8_epi16(ld16_q6(filterq.add(48))); // pmovsxbw m5, [filterq+16*3]
    let h = height as isize; // mov hd, hm
    match wd {
        // jmp wq (ipred_filter_16bpc_avx2_table: w4, w8, w16, w32)
        2 => ipred_filter_16bpc_w4(m, dst, stride, tl, h, bitdepth_max),
        3 => ipred_filter_16bpc_w8(m, dst, stride, tl, h, bitdepth_max),
        4 => ipred_filter_16bpc_w16(m, dst, stride, tl, h, bitdepth_max),
        _ => ipred_filter_16bpc_w32(m, dst, stride, tl, h, bitdepth_max),
    }
}

/// `IPRED_CFL ac`: ac in `m[n]`, pixels out (`m1` alpha, `m2` `|alpha| << 9`, `m0` dc).
#[inline(always)]
unsafe fn ipred_cfl(m: &mut M, n: usize) {
    m[3] = _mm256_sign_epi16(m[n], m[1]); // psignw m3, m%1, m1
    m[n] = _mm256_abs_epi16(m[n]); // pabsw m%1, m%1
    m[n] = _mm256_mulhrs_epi16(m[n], m[2]); // pmulhrsw m%1, m2
    m[n] = _mm256_sign_epi16(m[n], m[3]); // psignw m%1, m3
    m[n] = _mm256_add_epi16(m[n], m[0]); // paddw m%1, m0
}

/// `ipred_cfl_16bpc.s4`.
#[inline(always)]
unsafe fn ipred_cfl_16bpc_s4(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut ac: *const u8,
    mut h: isize,
    alpha: c_int,
) {
    m[1] = _mm256_set1_epi16(alpha as i16); // vpbroadcastw m1, alpham
    let r6 = stride * 3; // lea r6, [strideq*3]
    m[2] = _mm256_abs_epi16(m[1]); // pabsw m2, m1
    m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
    loop {
        // .s4_loop
        m[4] = ld32_q6(ac); // mova m4, [acq]
        ipred_cfl(m, 4); // IPRED_CFL 4
        m[4] = _mm256_max_epi16(m[4], m[6]); // pmaxsw m4, m6
        m[4] = _mm256_min_epi16(m[4], m[7]); // pminsw m4, m7
        m[5] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm5, m4, 1
        _mm_storel_epi64(dst as *mut __m128i, lo(m[4])); // movq [dstq+strideq*0], xm4
        _mm_storel_epi64(at(dst, stride * 2) as *mut __m128i, lo(m[5])); // movq [dstq+strideq*2], xm5
        _mm_storeh_pd(at(dst, stride) as *mut f64, _mm_castsi128_pd(lo(m[4]))); // movhps [dstq+strideq*1], xm4
        _mm_storeh_pd(at(dst, r6) as *mut f64, _mm_castsi128_pd(lo(m[5]))); // movhps [dstq+r6       ], xm5
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        ac = atc_q6(ac, 32); // add acq, 32
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .s4_loop
        }
    }
}

/// `ipred_cfl_16bpc.s8`.
#[inline(always)]
unsafe fn ipred_cfl_16bpc_s8(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut ac: *const u8,
    mut h: isize,
    alpha: c_int,
) {
    m[1] = _mm256_set1_epi16(alpha as i16); // vpbroadcastw m1, alpham
    let r6 = stride * 3; // lea r6, [strideq*3]
    m[2] = _mm256_abs_epi16(m[1]); // pabsw m2, m1
    m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
    loop {
        // .s8_loop
        m[4] = ld32_q6(ac); // mova m4, [acq]
        m[5] = ld32_q6(atc_q6(ac, 32)); // mova m5, [acq+32]
        ipred_cfl(m, 4); // IPRED_CFL 4
        ipred_cfl(m, 5); // IPRED_CFL 5
        m[4] = _mm256_max_epi16(m[4], m[6]); // pmaxsw m4, m6
        m[5] = _mm256_max_epi16(m[5], m[6]); // pmaxsw m5, m6
        m[4] = _mm256_min_epi16(m[4], m[7]); // pminsw m4, m7
        m[5] = _mm256_min_epi16(m[5], m[7]); // pminsw m5, m7
        _mm_storeu_si128(dst as *mut __m128i, lo(m[4])); // mova [dstq+strideq*0], xm4
        _mm_storeu_si128(at(dst, stride * 2) as *mut __m128i, lo(m[5])); // mova [dstq+strideq*2], xm5
        _mm_storeu_si128(at(dst, stride) as *mut __m128i, _mm256_extracti128_si256::<1>(m[4])); // vextracti128 [dstq+strideq*1], m4, 1
        _mm_storeu_si128(at(dst, r6) as *mut __m128i, _mm256_extracti128_si256::<1>(m[5])); // vextracti128 [dstq+r6       ], m5, 1
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        ac = atc_q6(ac, 64); // add acq, 64
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .s8_loop
        }
    }
}

/// `ipred_cfl_16bpc.s16`.
#[inline(always)]
unsafe fn ipred_cfl_16bpc_s16(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut ac: *const u8,
    mut h: isize,
    alpha: c_int,
) {
    m[1] = _mm256_set1_epi16(alpha as i16); // vpbroadcastw m1, alpham
    m[2] = _mm256_abs_epi16(m[1]); // pabsw m2, m1
    m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
    loop {
        // .s16_loop
        m[4] = ld32_q6(ac); // mova m4, [acq]
        m[5] = ld32_q6(atc_q6(ac, 32)); // mova m5, [acq+32]
        ipred_cfl(m, 4); // IPRED_CFL 4
        ipred_cfl(m, 5); // IPRED_CFL 5
        m[4] = _mm256_max_epi16(m[4], m[6]); // pmaxsw m4, m6
        m[5] = _mm256_max_epi16(m[5], m[6]); // pmaxsw m5, m6
        m[4] = _mm256_min_epi16(m[4], m[7]); // pminsw m4, m7
        m[5] = _mm256_min_epi16(m[5], m[7]); // pminsw m5, m7
        _mm256_storeu_si256(dst as *mut __m256i, m[4]); // mova [dstq+strideq*0], m4
        _mm256_storeu_si256(at(dst, stride) as *mut __m256i, m[5]); // mova [dstq+strideq*1], m5
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        ac = atc_q6(ac, 64); // add acq, 64
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .s16_loop
        }
    }
}

/// `ipred_cfl_16bpc.s32`.
#[inline(always)]
unsafe fn ipred_cfl_16bpc_s32(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut ac: *const u8,
    mut h: isize,
    alpha: c_int,
) {
    m[1] = _mm256_set1_epi16(alpha as i16); // vpbroadcastw m1, alpham
    m[2] = _mm256_abs_epi16(m[1]); // pabsw m2, m1
    m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
    loop {
        // .s32_loop
        m[4] = ld32_q6(ac); // mova m4, [acq]
        m[5] = ld32_q6(atc_q6(ac, 32)); // mova m5, [acq+32]
        ipred_cfl(m, 4); // IPRED_CFL 4
        ipred_cfl(m, 5); // IPRED_CFL 5
        m[4] = _mm256_max_epi16(m[4], m[6]); // pmaxsw m4, m6
        m[5] = _mm256_max_epi16(m[5], m[6]); // pmaxsw m5, m6
        m[4] = _mm256_min_epi16(m[4], m[7]); // pminsw m4, m7
        m[5] = _mm256_min_epi16(m[5], m[7]); // pminsw m5, m7
        _mm256_storeu_si256(dst as *mut __m256i, m[4]); // mova [dstq+32*0], m4
        _mm256_storeu_si256(at(dst, 32) as *mut __m256i, m[5]); // mova [dstq+32*1], m5
        dst = at(dst, stride); // add dstq, strideq
        ac = atc_q6(ac, 64); // add acq, 64
        h -= 1; // dec hd
        if h <= 0 {
            break; // jg .s32_loop
        }
    }
}

/// `jmp wq` into `ipred_cfl_splat_16bpc_avx2_table` (s4, s8, s16, s32) by `tzcnt(w)`.
#[inline(always)]
unsafe fn ipred_cfl_16bpc_splat(
    m: &mut M,
    wd: u32,
    dst: *mut u8,
    stride: isize,
    ac: *const u8,
    h: isize,
    alpha: c_int,
) {
    match wd {
        2 => ipred_cfl_16bpc_s4(m, dst, stride, ac, h, alpha),
        3 => ipred_cfl_16bpc_s8(m, dst, stride, ac, h, alpha),
        4 => ipred_cfl_16bpc_s16(m, dst, stride, ac, h, alpha),
        _ => ipred_cfl_16bpc_s32(m, dst, stride, ac, h, alpha),
    }
}

/// `ipred_cfl_left_16bpc.h4`.
#[inline(always)]
unsafe fn ipred_cfl_left_16bpc_h4(m: &mut M) {
    m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[6]))); // punpcklwd xm0, xm6
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); // psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
    m[1] = zx(_mm_bsrli_si128::<8>(lo(m[0]))); // psrldq xm1, xm0, 8
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[4]))); // paddd xm0, xm4
    m[0] = zx(_mm_srl_epi32(lo(m[0]), lo(m[5]))); // psrld xm0, xm5
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
}

/// `ipred_cfl_left_16bpc.h8`.
#[inline(always)]
unsafe fn ipred_cfl_left_16bpc_h8(m: &mut M) {
    m[1] = zx(_mm_bsrli_si128::<8>(lo(m[0]))); // psrldq xm1, xm0, 8
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    ipred_cfl_left_16bpc_h4(m);
}

/// `ipred_cfl_left_16bpc.h16`.
#[inline(always)]
unsafe fn ipred_cfl_left_16bpc_h16(m: &mut M) {
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    ipred_cfl_left_16bpc_h8(m);
}

/// `ipred_cfl_left_16bpc.h32`.
#[inline(always)]
unsafe fn ipred_cfl_left_16bpc_h32(m: &mut M, tl: *const u8) {
    m[0] = _mm256_add_epi16(m[0], ld32_q6(atc_q6(tl, 32))); // paddw m0, [tlq+32]
    ipred_cfl_left_16bpc_h16(m);
}

/// `jmp r6` into `ipred_cfl_left_16bpc_avx2_table` (h4, h8, h16, h32).
#[inline(always)]
unsafe fn ipred_cfl_left_16bpc_h(m: &mut M, i: u32, tl: *const u8) {
    match i {
        2 => ipred_cfl_left_16bpc_h4(m),
        3 => ipred_cfl_left_16bpc_h8(m),
        4 => ipred_cfl_left_16bpc_h16(m),
        _ => ipred_cfl_left_16bpc_h32(m, tl),
    }
}

/// `ipred_cfl_top_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_top_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    bitdepth_max: c_int,
) {
    let mut regs_ = regs();
    let m = &mut regs_;
    let h = height as isize; // movifnidn hd, hm
    let tl = atc_q6(topleft as *const u8, 2); // add tlq, 2
    m[4] = zx(_mm_cvtsi32_si128(width)); // movd xm4, wd
    m[6] = _mm256_setzero_si256(); // pxor m6, m6
    m[7] = _mm256_set1_epi16(bitdepth_max as i16); // vpbroadcastw m7, r7m
    m[4] = zx(_mm_avg_epu16(lo(m[4]), lo(m[6]))); // pavgw xm4, xm6
    let wd = (width as u32).trailing_zeros(); // tzcnt wd, wd (width is a power of two, never zero)
    m[5] = zx(_mm_cvtsi32_si128(wd as i32)); // movd xm5, wd
    m[0] = ld32_q6(tl); // movu m0, [tlq]
    ipred_cfl_left_16bpc_h(m, wd, tl); // jmp r6 (ipred_cfl_left_16bpc_avx2_table by tzcnt(w))
    ipred_cfl_16bpc_splat(m, wd, dst as *mut u8, stride, ac as *const u8, h, alpha);
    // jmp wq
}

/// `ipred_cfl_left_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_left_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    bitdepth_max: c_int,
) {
    let mut regs_ = regs();
    let m = &mut regs_;
    let h = height as isize; // mov hd, hm
    let mut tl = atc_q6(topleft as *const u8, -h); // sub tlq, hq
    m[4] = zx(_mm_cvtsi32_si128(height)); // movd xm4, hd
    tl = atc_q6(tl, -h); // sub tlq, hq
    m[6] = _mm256_setzero_si256(); // pxor m6, m6
    m[7] = _mm256_set1_epi16(bitdepth_max as i16); // vpbroadcastw m7, r7m
    m[4] = zx(_mm_avg_epu16(lo(m[4]), lo(m[6]))); // pavgw xm4, xm6
    let r6 = (height as u32).trailing_zeros(); // tzcnt r6d, hd (height is a power of two, never zero)
    m[5] = zx(_mm_cvtsi32_si128(r6 as i32)); // movd xm5, r6d
    m[0] = ld32_q6(tl); // movu m0, [tlq]
    let wd = (width as u32).trailing_zeros(); // tzcnt wd, wd (width is a power of two, never zero)
    ipred_cfl_left_16bpc_h(m, r6, tl); // jmp r6
    ipred_cfl_16bpc_splat(m, wd, dst as *mut u8, stride, ac as *const u8, h, alpha);
    // jmp wq
}

/// `ipred_cfl_16bpc.w4` with `.w4_mul` and `.w4_end`.
#[inline(always)]
unsafe fn ipred_cfl_16bpc_w4(m: &mut M, tl: *const u8, h: isize) {
    m[1] = zx(_mm_loadl_epi64(atc_q6(tl, 2) as *const __m128i)); // movq xm1, [tlq+2]
    m[0] = _mm256_add_epi16(m[0], m[4]); // paddw m0, m4
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
    m[1] = _mm256_srli_epi64::<32>(m[0]); // psrlq m1, m0, 32
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
    m[1] = _mm256_srli_epi32::<16>(m[0]); // psrld m1, m0, 16
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
    if h > 4 {
        // cmp hd, 4 / jg .w4_mul
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
        let r2 = (h * 2) as u32; // lea r2d, [hq*2]
        let r6 = 0xAAAB6667u32.wrapping_shr(r2); // mov r6d, 0xAAAB6667 / shrx r6d, r6d, r2d (count mod 32)
        m[1] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[6]))); // punpckhwd xm1, xm0, xm6
        m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[6]))); // punpcklwd xm0, xm6
        m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); // movd xm1, r6d
        m[0] = zx(_mm_srli_epi32::<2>(lo(m[0]))); // psrld xm0, 2
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); // pmulhuw xm0, xm1
        m[0] = zx(_mm_srli_epi16::<1>(lo(m[0]))); // psrlw xm0, 1
    } else {
        m[0] = zx(_mm_srli_epi16::<3>(lo(m[0]))); // psrlw xm0, 3
    }
    // .w4_end
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
}

/// `ipred_cfl_16bpc.w8` with `.w8_end`.
#[inline(always)]
unsafe fn ipred_cfl_16bpc_w8(m: &mut M, tl: *const u8, h: isize) {
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    m[0] = zx(_mm_add_epi16(lo(m[0]), ld16_q6(atc_q6(tl, 2)))); // paddw xm0, [tlq+2]
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[4]))); // paddw xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[1] = zx(_mm_srli_epi32::<16>(lo(m[0]))); // psrld xm1, xm0, 16
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[0] = zx(_mm_blend_epi16::<0xaa>(lo(m[0]), lo(m[6]))); // pblendw xm0, xm6, 0xAA
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); // psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
    m[1] = zx(_mm_bsrli_si128::<8>(lo(m[0]))); // psrldq xm1, xm0, 8
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
    m[0] = zx(_mm_srl_epi32(lo(m[0]), lo(m[5]))); // psrld xm0, xm5
    if h != 8 {
        // cmp hd, 8 / je .w8_end
        let mut r6: u32 = 0xAAAB; // mov r6d, 0xAAAB
        let r2: u32 = 0x6667; // mov r2d, 0x6667
        if h == 32 {
            r6 = r2; // cmp hd, 32 / cmovz r6d, r2d
        }
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); // movd xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); // pmulhuw xm0, xm1
        m[0] = zx(_mm_srli_epi16::<1>(lo(m[0]))); // psrlw xm0, 1
    }
    // .w8_end
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
}

/// `ipred_cfl_16bpc.w16` with `.w16_end`.
#[inline(always)]
unsafe fn ipred_cfl_16bpc_w16(m: &mut M, tl: *const u8, h: isize) {
    m[0] = _mm256_add_epi16(m[0], ld32_q6(atc_q6(tl, 2))); // paddw m0, [tlq+2]
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[4]))); // paddw xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[1] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[6]))); // punpckhwd xm1, xm0, xm6
    m[0] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[6]))); // punpcklwd xm0, xm6
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); // psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
    m[1] = zx(_mm_bsrli_si128::<8>(lo(m[0]))); // psrldq xm1, xm0, 8
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
    m[0] = zx(_mm_srl_epi32(lo(m[0]), lo(m[5]))); // psrld xm0, xm5
    if h != 16 {
        // cmp hd, 16 / je .w16_end
        let mut r6: u32 = 0xAAAB; // mov r6d, 0xAAAB
        let r2: u32 = 0x6667; // mov r2d, 0x6667
        if h & (8 | 32) == 0 {
            r6 = r2; // test hb, 8|32 / cmovz r6d, r2d
        }
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); // movd xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); // pmulhuw xm0, xm1
        m[0] = zx(_mm_srli_epi16::<1>(lo(m[0]))); // psrlw xm0, 1
    }
    // .w16_end
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
}

/// `ipred_cfl_16bpc.w32` with `.w32_end`.
#[inline(always)]
unsafe fn ipred_cfl_16bpc_w32(m: &mut M, tl: *const u8, h: isize) {
    m[0] = _mm256_add_epi16(m[0], ld32_q6(atc_q6(tl, 2))); // paddw m0, [tlq+ 2]
    m[0] = _mm256_add_epi16(m[0], ld32_q6(atc_q6(tl, 34))); // paddw m0, [tlq+34]
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[4]))); // paddw xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[1] = zx(_mm_unpacklo_epi16(lo(m[0]), lo(m[6]))); // punpcklwd xm1, xm0, xm6
    m[0] = zx(_mm_unpackhi_epi16(lo(m[0]), lo(m[6]))); // punpckhwd xm0, xm6
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); // psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
    m[1] = zx(_mm_bsrli_si128::<8>(lo(m[0]))); // psrldq xm1, xm0, 8
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
    m[0] = zx(_mm_srl_epi32(lo(m[0]), lo(m[5]))); // psrld xm0, xm5
    if h != 32 {
        // cmp hd, 32 / je .w32_end
        let r2 = (h * 2) as u32; // lea r2d, [hq*2]
        let r6 = 0x6667AAABu32.wrapping_shr(r2); // mov r6d, 0x6667AAAB / shrx r6d, r6d, r2d (count mod 32)
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); // movd xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); // pmulhuw xm0, xm1
        m[0] = zx(_mm_srli_epi16::<1>(lo(m[0]))); // psrlw xm0, 1
    }
    // .w32_end
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
}

/// `ipred_cfl_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    topleft: *const u16,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    bitdepth_max: c_int,
) {
    let mut regs_ = regs();
    let m = &mut regs_;
    let tl = topleft as *const u8;
    let h = height as isize; // movifnidn hd, hm
    let r6 = (height as u32).trailing_zeros(); // tzcnt r6d, hd (height is a power of two, never zero)
    let t0 = (width + height) as u32; // lea t0d, [wq+hq]
    m[4] = zx(_mm_cvtsi32_si128(t0 as i32)); // movd xm4, t0d
    let t0 = t0.trailing_zeros(); // tzcnt t0d, t0d (w + h is never zero)
    m[5] = zx(_mm_cvtsi32_si128(t0 as i32)); // movd xm5, t0d
    let wd = (width as u32).trailing_zeros(); // tzcnt wd, wd (width is a power of two, never zero)
    m[4] = zx(_mm_srli_epi16::<1>(lo(m[4]))); // psrlw xm4, 1
    m[6] = _mm256_setzero_si256(); // pxor m6, m6
    m[7] = _mm256_set1_epi16(bitdepth_max as i16); // vpbroadcastw m7, r7m
    match r6 {
        // jmp r6 (ipred_cfl_16bpc_avx2_table: h4, h8, h16, h32)
        2 => m[0] = zx(_mm_loadl_epi64(atc_q6(tl, -8) as *const __m128i)), // .h4: movq xm0, [tlq-8]
        3 => m[0] = zx(ld16_q6(atc_q6(tl, -16))),                          // .h8: mova xm0, [tlq-16]
        4 => m[0] = ld32_q6(atc_q6(tl, -32)),                              // .h16: mova m0, [tlq-32]
        _ => {
            m[0] = ld32_q6(atc_q6(tl, -64)); // .h32: mova m0, [tlq-64]
            m[0] = _mm256_add_epi16(m[0], ld32_q6(atc_q6(tl, -32))); // paddw m0, [tlq-32]
        }
    }
    // jmp wq (ipred_cfl_16bpc_avx2_table + 4*4: w4, w8, w16, w32; each falls into its .sN)
    match wd {
        2 => ipred_cfl_16bpc_w4(m, tl, h),
        3 => ipred_cfl_16bpc_w8(m, tl, h),
        4 => ipred_cfl_16bpc_w16(m, tl, h),
        _ => ipred_cfl_16bpc_w32(m, tl, h),
    }
    ipred_cfl_16bpc_splat(m, wd, dst as *mut u8, stride, ac as *const u8, h, alpha);
}

/// `ipred_cfl_128_16bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_128_16bpc_avx2(
    dst: *mut u16,
    stride: isize,
    _topleft: *const u16,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
    bitdepth_max: c_int,
) {
    let mut regs_ = regs();
    let m = &mut regs_;
    let r6 = (bitdepth_max as u32) >> 11; // mov r6d, r7m / shr r6d, 11
    let wd = (width as u32).trailing_zeros(); // tzcnt wd, wd (width is a power of two, never zero)
    let h = height as isize; // movifnidn hd, hm
                             // bitdepth_max is 0x3ff or 0xfff (10 or 12 bit), so r6 is 0 or 1: pw_512 or pw_2048, inside RODATA.
    let dc = (ro(pw_512, r6 as isize * 4) as *const i32).read_unaligned();
    m[0] = _mm256_set1_epi32(dc); // vpbroadcastd m0, [t0-ipred_cfl_splat_16bpc_avx2_table+pw_512+r6*4]
    m[6] = _mm256_setzero_si256(); // pxor m6, m6
    m[7] = _mm256_set1_epi16(bitdepth_max as i16); // vpbroadcastw m7, r7m
    ipred_cfl_16bpc_splat(m, wd, dst as *mut u8, stride, ac as *const u8, h, alpha);
    // jmp wq
}
