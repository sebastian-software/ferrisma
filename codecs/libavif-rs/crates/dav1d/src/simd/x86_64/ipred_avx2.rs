// Copyright © 2018-2021, VideoLAN and dav1d authors
// Copyright © 2018-2021, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! Rebuilt from `upstream/dav1d/src/x86/ipred_avx2.asm` (8 bpc) with AVX2
//! intrinsics (engine ADR-0044 D2, libavif-rs ADR 0062).
//!
//! The 22 entry points the original assigns on an AVX2 machine:
//! `dav1d_ipred_{dc,dc_128,dc_top,dc_left,h,v,paeth,smooth,smooth_h,smooth_v,
//! z1,z2,z3,filter}_8bpc_avx2`, `dav1d_ipred_cfl{,_128,_top,_left}_8bpc_avx2`,
//! `dav1d_ipred_cfl_ac_{420,422,444}_8bpc_avx2` and `dav1d_pal_pred_8bpc_avx2`.
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
//!   that defines them (`_p1` … `_p7`).
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
//!   from 4 to 64) go to the last arm, except in dc/h/v/paeth/pal_pred,
//!   where they reach `unreachable!()` (the assembly's jump through bytes
//!   beyond its table is undefined; here it is a checked panic).
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
//! - the edge (`topleft`): dc_top and v read 32 or 64 pixels at every
//!   width, dc_left 32 from `tl-h`, paeth (4x4), smooth and smooth_h
//!   (`tl[-64..-1]` from w16 on, whatever `h`), filter and cfl_top/cfl_left
//!   whole vectors; z1 reads up to `topleft[198]`, z2 `topleft[-69..67]`, z3
//!   down to `topleft[-191]`. In the decoder `topleft` is `edge_8bpc + 128`
//!   (inter-intra: `+ 32`) in a 257-pixel array. The kernels whose loads can
//!   leave that array (z1, z3, and smooth and smooth_h from w16 on) copy
//!   the pixels the C kernel reads into a zero-initialized local first and
//!   run on the copy (`crate::simd::copy_edge`; not in the original); test
//!   `the_kernels_read_only_the_edge_array` in `simd/x86/ipred.rs`;
//! - `cfl_ac` loads 16 or 32 bytes of luma rows where C reads 8 or 16
//!   (`.w8_wpad`, `.w16_pad1`, 444 `.w8`), inside the luma rows' stride.
//!
//! Safety (argued once for the family): the callers (`recon_tmpl`, the
//! test) pass a destination block of `width` x `height` pixels with its
//! stride in bytes, `topleft` as above, `width` and `height` powers of two
//! from 4 to 64 (the ones the original's tables have), the angle with its
//! flag bits, `ac` with `width * height` values and `alpha`, `pal` with 8
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

/// `SECTION_RODATA` of `ipred_avx2.asm` up to its jump tables, byte for byte (taken from the original's object
/// file): one array with a constant per label, because the assembly may address a table at offsets that reach
/// into the next one. The jump tables that follow (code addresses) are not part of it; they become `match`es.
#[repr(C, align(64))]
pub(crate) struct Rodata(pub(crate) [u8; 1024]);
pub(crate) static RODATA: Rodata = Rodata([
    0x80, 0x7f, 0x80, 0x7f, 0x7f, 0x80, 0x00, 0xff, 0x7f, 0x80, 0x15, 0xea, 0xd5, 0x2a, 0xc0, 0x3f, // 0x000
    0x7f, 0x80, 0x45, 0xba, 0x12, 0xed, 0xe9, 0x16, 0xc9, 0x36, 0xb2, 0x4d, 0xa5, 0x5a, 0xa0, 0x5f, // 0x010
    0x7f, 0x80, 0x61, 0x9e, 0x44, 0xbb, 0x2a, 0xd5, 0x11, 0xee, 0xfb, 0x04, 0xe6, 0x19, 0xd4, 0x2b, // 0x020
    0xc4, 0x3b, 0xb6, 0x49, 0xab, 0x54, 0xa1, 0x5e, 0x9a, 0x65, 0x94, 0x6b, 0x91, 0x6e, 0x90, 0x6f, // 0x030
    0x7f, 0x80, 0x70, 0x8f, 0x61, 0x9e, 0x52, 0xad, 0x44, 0xbb, 0x36, 0xc9, 0x29, 0xd6, 0x1d, 0xe2, // 0x040
    0x11, 0xee, 0x05, 0xfa, 0xfa, 0x05, 0xef, 0x10, 0xe5, 0x1a, 0xdc, 0x23, 0xd3, 0x2c, 0xca, 0x35, // 0x050
    0xc2, 0x3d, 0xbb, 0x44, 0xb4, 0x4b, 0xad, 0x52, 0xa7, 0x58, 0xa2, 0x5d, 0x9d, 0x62, 0x99, 0x66, // 0x060
    0x95, 0x6a, 0x91, 0x6e, 0x8e, 0x71, 0x8c, 0x73, 0x8a, 0x75, 0x89, 0x76, 0x88, 0x77, 0x88, 0x77, // 0x070
    0x7f, 0x80, 0x78, 0x87, 0x70, 0x8f, 0x69, 0x96, 0x61, 0x9e, 0x5a, 0xa5, 0x52, 0xad, 0x4b, 0xb4, // 0x080
    0x44, 0xbb, 0x3d, 0xc2, 0x36, 0xc9, 0x30, 0xcf, 0x29, 0xd6, 0x23, 0xdc, 0x1c, 0xe3, 0x16, 0xe9, // 0x090
    0x10, 0xef, 0x0a, 0xf5, 0x05, 0xfa, 0xff, 0x00, 0xf9, 0x06, 0xf4, 0x0b, 0xef, 0x10, 0xea, 0x15, // 0x0a0
    0xe5, 0x1a, 0xe0, 0x1f, 0xdb, 0x24, 0xd6, 0x29, 0xd2, 0x2d, 0xcd, 0x32, 0xc9, 0x36, 0xc5, 0x3a, // 0x0b0
    0xc1, 0x3e, 0xbd, 0x42, 0xb9, 0x46, 0xb6, 0x49, 0xb2, 0x4d, 0xaf, 0x50, 0xac, 0x53, 0xa9, 0x56, // 0x0c0
    0xa6, 0x59, 0xa3, 0x5c, 0xa0, 0x5f, 0x9d, 0x62, 0x9b, 0x64, 0x99, 0x66, 0x96, 0x69, 0x94, 0x6b, // 0x0d0
    0x92, 0x6d, 0x90, 0x6f, 0x8f, 0x70, 0x8d, 0x72, 0x8c, 0x73, 0x8a, 0x75, 0x89, 0x76, 0x88, 0x77, // 0x0e0
    0x87, 0x78, 0x86, 0x79, 0x86, 0x79, 0x85, 0x7a, 0x85, 0x7a, 0x84, 0x7b, 0x84, 0x7b, 0x84, 0x7b, // 0x0f0
    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, // 0x100
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, // 0x110
    0x20, 0x1f, 0x1e, 0x1d, 0x1c, 0x1b, 0x1a, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, // 0x120
    0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, // 0x130
    0x07, 0x07, 0x0b, 0x0b, 0x0f, 0x0f, 0x13, 0x13, 0x13, 0x17, 0x17, 0x17, 0x1f, 0x1f, 0x1f, 0x27, // 0x140
    0x27, 0x27, 0x2f, 0x2f, 0x2f, 0x3f, 0x3f, 0x3f, 0x4f, 0x4f, 0x4f, 0xff, 0x00, 0x10, 0x00, 0x10, // 0x150
    0x00, 0x14, 0x00, 0x14, 0x08, 0x10, 0x08, 0x10, 0x20, 0x10, 0x20, 0x10, 0x18, 0x14, 0x18, 0x14, // 0x160
    0x10, 0x10, 0x10, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x08, 0x00, // 0x170
    0x00, 0x00, 0x00, 0x01, 0x01, 0x02, 0x02, 0x03, 0x03, 0x04, 0x04, 0x05, 0x05, 0x06, 0x06, 0x07, // 0x180
    0x07, 0x08, 0x08, 0x09, 0x09, 0x0a, 0x0a, 0x0b, 0x0b, 0x0c, 0x0c, 0x0d, 0x0d, 0x0e, 0x0e, 0x0f, // 0x190
    0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x0f, 0x80, 0x80, 0x80, 0x80, 0x24, 0xfc, 0x24, 0xfc, // 0x1a0
    0x08, 0x07, 0x07, 0x06, 0x06, 0x05, 0x05, 0x04, 0x04, 0x03, 0x03, 0x02, 0x02, 0x01, 0x01, 0x00, // 0x1b0
    0x37, 0x7f, 0x27, 0x7f, 0x27, 0x7f, 0x07, 0x0f, 0x1f, 0x07, 0x0f, 0x1f, 0x00, 0x03, 0x1f, 0x00, // 0x1c0
    0x27, 0x3f, 0x13, 0x2f, 0x13, 0x2f, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x00, 0x00, 0x00, 0x00, // 0x1d0
    0x01, 0x00, 0x02, 0x01, 0x03, 0x02, 0x04, 0x03, 0x05, 0x04, 0x06, 0x05, 0x07, 0x06, 0x08, 0x07, // 0x1e0
    0x02, 0x03, 0x03, 0x04, 0x04, 0x05, 0x05, 0x06, 0x06, 0x07, 0x07, 0x08, 0x08, 0x08, 0x08, 0x08, // 0x1f0
    0x07, 0x06, 0x0f, 0x0e, 0x05, 0x04, 0x0d, 0x0c, 0x03, 0x02, 0x0b, 0x0a, 0x01, 0x00, 0x09, 0x08, // 0x200
    0x00, 0x01, 0x01, 0x02, 0x02, 0x03, 0x03, 0x04, 0x08, 0x09, 0x09, 0x0a, 0x0a, 0x0b, 0x0b, 0x0c, // 0x210
    0x03, 0x02, 0x07, 0x06, 0x0b, 0x0a, 0x0f, 0x0e, 0x02, 0x01, 0x06, 0x05, 0x0a, 0x09, 0x0e, 0x0d, // 0x220
    0x07, 0x06, 0x0f, 0x0e, 0x06, 0x05, 0x0e, 0x0d, 0x05, 0x04, 0x0d, 0x0c, 0x04, 0x03, 0x0c, 0x0b, // 0x230
    0x04, 0x03, 0x03, 0x02, 0x02, 0x01, 0x01, 0x00, 0x0c, 0x0b, 0x0b, 0x0a, 0x0a, 0x09, 0x09, 0x08, // 0x240
    0x00, 0x04, 0x08, 0x0c, 0x01, 0x05, 0x09, 0x0d, 0x02, 0x06, 0x0a, 0x0e, 0x03, 0x07, 0x0b, 0x0f, // 0x250
    0x00, 0x00, 0x40, 0x00, 0x80, 0x00, 0xc0, 0x00, 0x00, 0x01, 0x40, 0x01, 0x80, 0x01, 0xc0, 0x01, // 0x260
    0x00, 0x04, 0x40, 0x04, 0x80, 0x04, 0xc0, 0x04, 0x00, 0x05, 0x40, 0x05, 0x80, 0x05, 0xc0, 0x05, // 0x270
    0x40, 0x00, 0x80, 0x00, 0xc0, 0x00, 0x00, 0x01, 0x40, 0x01, 0x80, 0x01, 0xc0, 0x01, 0x00, 0x02, // 0x280
    0x40, 0x02, 0x80, 0x02, 0xc0, 0x02, 0x00, 0x03, 0x40, 0x03, 0x80, 0x03, 0xc0, 0x03, 0x00, 0x04, // 0x290
    0x01, 0x00, 0x02, 0x00, 0x03, 0x00, 0x04, 0x00, 0x05, 0x00, 0x06, 0x00, 0x07, 0x00, 0x08, 0x00, // 0x2a0
    0x09, 0x00, 0x0a, 0x00, 0x0b, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x0e, 0x00, 0x0f, 0x00, 0x10, 0x00, // 0x2b0
    0x5a, 0x5a, 0x5a, 0x5a, 0x0e, 0x0e, 0x0e, 0x0e, 0x1b, 0x1b, 0x1b, 0x1b, 0x1f, 0x1f, 0x1f, 0x1f, // 0x2c0
    0x20, 0x20, 0x20, 0x20, 0x0c, 0x0c, 0x0c, 0x0c, 0x01, 0x00, 0x01, 0x00, 0x05, 0xff, 0xff, 0xff, // 0x2d0
    0x0a, 0x04, 0x0a, 0x04, 0x25, 0x06, 0x05, 0x06, 0x67, 0x09, 0x07, 0x09, 0x48, 0xff, 0x08, 0xff, // 0x2e0
    0x10, 0x04, 0x00, 0x04, 0x35, 0x06, 0x05, 0x06, 0x77, 0x0b, 0x07, 0x0b, 0x5f, 0xff, 0x0f, 0xff, // 0x2f0
    0x03, 0x04, 0x03, 0x04, 0x05, 0x06, 0x05, 0x06, 0x07, 0x02, 0x07, 0x02, 0x01, 0xff, 0x01, 0xff, // 0x300
    0x03, 0x04, 0x03, 0x04, 0x05, 0x06, 0x05, 0x06, 0x07, 0x0b, 0x07, 0x0b, 0x7f, 0x81, 0x7f, 0x81, // 0x310
    0x00, 0x01, 0x00, 0x01, 0x04, 0x05, 0x04, 0x05, 0x08, 0x09, 0x08, 0x09, 0x0c, 0x0d, 0x0c, 0x0d, // 0x320
    0x02, 0x03, 0x02, 0x03, 0x06, 0x07, 0x06, 0x07, 0x0a, 0x0b, 0x0a, 0x0b, 0x0e, 0x0f, 0x0e, 0x0f, // 0x330
    0x07, 0x07, 0x07, 0x07, 0x03, 0x03, 0x03, 0x03, 0x05, 0x05, 0x05, 0x05, 0x01, 0x01, 0x01, 0x01, // 0x340
    0x06, 0x06, 0x06, 0x06, 0x02, 0x02, 0x02, 0x02, 0x04, 0x04, 0x04, 0x04, 0x40, 0x00, 0x40, 0x00, // 0x350
    0x00, 0xff, 0x01, 0xff, 0x02, 0xff, 0x03, 0xff, 0x04, 0xff, 0x05, 0xff, 0x06, 0xff, 0x07, 0xff, // 0x360
    0x07, 0xff, 0x07, 0xff, 0x07, 0xff, 0x07, 0xff, 0x07, 0xff, 0x07, 0xff, 0x07, 0xff, 0x07, 0xff, // 0x370
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, // 0x380
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x06, 0x07, 0x06, 0x07, 0x06, 0x07, 0x06, 0x07, // 0x390
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, // 0x3a0
    0x0e, 0x0f, 0x0e, 0x0f, 0x0e, 0x0f, 0x0e, 0x0f, 0x0e, 0x0f, 0x0e, 0x0f, 0x0e, 0x0f, 0x0e, 0x0f, // 0x3b0
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x06, 0x07, 0x06, 0x07, 0x06, 0x07, 0x06, 0x07, // 0x3c0
    0x06, 0x07, 0x06, 0x07, 0x06, 0x07, 0x06, 0x07, 0x06, 0x07, 0x06, 0x07, 0x06, 0x07, 0x06, 0x07, // 0x3d0
    0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00, // 0x3e0
    0x3e, 0x00, 0x3e, 0x00, 0x80, 0x00, 0x80, 0x00, 0xff, 0x00, 0xff, 0x00, 0x00, 0x02, 0x00, 0x02, // 0x3f0
]);
pub(crate) const smooth_weights: usize = 0x000;
pub(crate) const pb_1to32: usize = 0x100;
pub(crate) const pb_32to1: usize = 0x120;
pub(crate) const pb_16to1: usize = 0x130;
pub(crate) const z_filter_wh: usize = 0x140;
pub(crate) const z_filter_k: usize = 0x15c;
pub(crate) const dav1d_z_filter_s: usize = 0x180;
pub(crate) const pb_128: usize = 0x1a8;
pub(crate) const pb_36_m4: usize = 0x1ac;
pub(crate) const z3_shuf: usize = 0x1b0;
pub(crate) const z_filter_t0: usize = 0x1c0;
pub(crate) const z_filter_t1: usize = 0x1d0;
pub(crate) const z_upsample1: usize = 0x1e0;
pub(crate) const z_upsample2: usize = 0x1f0;
pub(crate) const z2_upsample: usize = 0x200;
pub(crate) const z1_shuf_w4: usize = 0x210;
pub(crate) const z2_shuf_h2: usize = 0x220;
pub(crate) const z2_shuf_h4: usize = 0x230;
pub(crate) const z3_shuf_w4: usize = 0x240;
pub(crate) const z_transpose4: usize = 0x250;
pub(crate) const z_base_inc: usize = 0x260;
pub(crate) const z2_base_inc: usize = 0x280;
pub(crate) const z2_ymul: usize = 0x2a0;
pub(crate) const z2_y_shuf_h4: usize = 0x2c0;
pub(crate) const filter_shuf1: usize = 0x2e0;
pub(crate) const filter_shuf2: usize = 0x300;
pub(crate) const filter_shuf3: usize = 0x310;
pub(crate) const pb_127_m127: usize = 0x31c;
pub(crate) const ipred_v_shuf: usize = 0x320;
pub(crate) const ipred_h_shuf: usize = 0x340;
pub(crate) const pw_64: usize = 0x35c;
pub(crate) const cfl_ac_444_w16_pad1_shuffle: usize = 0x360;
pub(crate) const cfl_ac_w16_pad_shuffle: usize = 0x380;
pub(crate) const cfl_ac_w8_pad1_shuffle: usize = 0x390;
pub(crate) const pb_15to0: usize = 0x3e0;
pub(crate) const pw_62: usize = 0x3f0;
pub(crate) const pw_128: usize = 0x3f4;
pub(crate) const pw_255: usize = 0x3f8;
pub(crate) const pw_512: usize = 0x3fc;

/// `[label+off]`: the address of a read-only operand. Every access through it must lie inside `RODATA`; say at
/// the use why it does when the offset is not a constant.
#[inline(always)]
pub(crate) fn ro(label: usize, off: isize) -> *const u8 {
    RODATA.0.as_ptr().wrapping_add(label).wrapping_offset(off)
}

// ---- part 1: `ipred_avx2.asm` lines 181–668 (dc_top, dc_left, dc, dc_128, v, IPRED_H/h, PAETH/paeth) and 5278–5362 (pal_pred) ----

// Memory operands of part 1. Every access to caller memory is unaligned where the assembly uses `mova` (same
// bytes, same instruction); `tl` and `dst` advance with wrapping arithmetic (`at`, `a_p1`).

/// `p + off` bytes for a read (wrapping: no claim that the address is in bounds until it is accessed).
#[inline(always)]
fn a_p1(p: *const u8, off: isize) -> *const u8 {
    p.wrapping_offset(off)
}

/// `movd xm, [p]` (4 bytes; the rest of the register is cleared).
#[inline(always)]
unsafe fn movd_p1(p: *const u8) -> __m128i {
    _mm_cvtsi32_si128(p.cast::<i32>().read_unaligned())
}

/// `movq xm, [p]` (8 bytes; the rest of the register is cleared).
#[inline(always)]
unsafe fn movq_p1(p: *const u8) -> __m128i {
    _mm_loadl_epi64(p.cast())
}

/// A 16-byte memory operand (`movu xm, [p]`, `mova xm, [p]`).
#[inline(always)]
unsafe fn x_p1(p: *const u8) -> __m128i {
    _mm_loadu_si128(p.cast())
}

/// A 32-byte memory operand (`movu m, [p]`, `mova m, [p]`, `vpermq m, [p], …`).
#[inline(always)]
unsafe fn y_p1(p: *const u8) -> __m256i {
    _mm256_loadu_si256(p.cast())
}

/// `vbroadcasti128 m, [p]`.
#[inline(always)]
unsafe fn vbroadcasti128_p1(p: *const u8) -> __m256i {
    _mm256_broadcastsi128_si256(x_p1(p))
}

/// `vpbroadcastq m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastq_p1(p: *const u8) -> __m256i {
    _mm256_set1_epi64x(p.cast::<i64>().read_unaligned())
}

/// `vpbroadcastd m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastd_p1(p: *const u8) -> __m256i {
    _mm256_set1_epi32(p.cast::<i32>().read_unaligned())
}

/// `vpbroadcastb m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastb_p1(p: *const u8) -> __m256i {
    _mm256_set1_epi8(p.cast::<i8>().read())
}

/// `vpbroadcastb xm, [p]` (`INIT_XMM`: the upper half is cleared).
#[inline(always)]
unsafe fn vpbroadcastb_x_p1(p: *const u8) -> __m256i {
    zx(_mm_set1_epi8(p.cast::<i8>().read()))
}

/// `movd [p], xm`.
#[inline(always)]
unsafe fn movd_st_p1(p: *mut u8, x: __m256i) {
    p.cast::<i32>().write_unaligned(_mm_cvtsi128_si32(lo(x)))
}

/// `pextrd [p], xm, I`.
#[inline(always)]
unsafe fn pextrd_st_p1<const I: i32>(p: *mut u8, x: __m256i) {
    p.cast::<i32>().write_unaligned(_mm_extract_epi32::<I>(lo(x)))
}

/// `movq [p], xm`.
#[inline(always)]
unsafe fn movq_st_p1(p: *mut u8, x: __m256i) {
    _mm_storel_epi64(p.cast(), lo(x))
}

/// `movhps [p], xm`: the high quadword, as the unaligned write of an extracted element (`vpextrq [p], xm, 1`,
/// the same 8 bytes in one instruction).
#[inline(always)]
unsafe fn movhps_st_p1(p: *mut u8, x: __m256i) {
    p.cast::<i64>().write_unaligned(_mm_extract_epi64::<1>(lo(x)))
}

/// `mova [p], xm` (16 bytes).
#[inline(always)]
unsafe fn movx_st_p1(p: *mut u8, x: __m256i) {
    _mm_storeu_si128(p.cast(), lo(x))
}

/// `mova [p], m` (32 bytes).
#[inline(always)]
unsafe fn movy_st_p1(p: *mut u8, x: __m256i) {
    _mm256_storeu_si256(p.cast(), x)
}

/// `vextracti128 [p], m, 1`.
#[inline(always)]
unsafe fn vextracti128_st_p1(p: *mut u8, x: __m256i) {
    _mm_storeu_si128(p.cast(), _mm256_extracti128_si256::<1>(x))
}

/// `vextracti128 xmD, mS, 1` (the upper half of the destination is cleared).
#[inline(always)]
unsafe fn vextracti128_p1(x: __m256i) -> __m256i {
    zx(_mm256_extracti128_si256::<1>(x))
}

/// `tzcnt` of a block dimension (a power of two from 4 to 64; the original selects these kernels on AVX2 alone, so
/// this is `trailing_zeros`, not BMI's `tzcnt`). A `match` on it stands for the jump table; an index outside the
/// table's range is outside the contract and reaches `unreachable!()`.
#[inline(always)]
fn tzcnt_p1(x: c_int) -> u32 {
    (x as u32).trailing_zeros()
}

// ---- ipred_dc_8bpc: .s4 … .s64 (also the targets of ipred_dc_splat_avx2_table) ----

/// `ipred_dc_8bpc.s4`: `h` rows of the four bytes of `xm0`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_s4(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, mut h: c_int) {
    loop {
        movd_st_p1(dst, m[0]); //                                  movd   [dstq+strideq*0], xm0
        movd_st_p1(at(dst, stride), m[0]); //                      movd   [dstq+strideq*1], xm0
        movd_st_p1(at(dst, stride * 2), m[0]); //                  movd   [dstq+strideq*2], xm0
        movd_st_p1(at(dst, stride3), m[0]); //                     movd   [dstq+stride3q ], xm0
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        h -= 4; //                                                 sub hd, 4
        if h <= 0 {
            break; //                                              jg .s4
        }
    }
    // RET
}

/// `ipred_dc_8bpc.s8`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_s8(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, mut h: c_int) {
    loop {
        movq_st_p1(dst, m[0]); //                                  movq   [dstq+strideq*0], xm0
        movq_st_p1(at(dst, stride), m[0]); //                      movq   [dstq+strideq*1], xm0
        movq_st_p1(at(dst, stride * 2), m[0]); //                  movq   [dstq+strideq*2], xm0
        movq_st_p1(at(dst, stride3), m[0]); //                     movq   [dstq+stride3q ], xm0
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        h -= 4; //                                                 sub hd, 4
        if h <= 0 {
            break; //                                              jg .s8
        }
    }
    // RET
}

/// `ipred_dc_8bpc.s16`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_s16(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, mut h: c_int) {
    loop {
        movx_st_p1(dst, m[0]); //                                  mova   [dstq+strideq*0], xm0
        movx_st_p1(at(dst, stride), m[0]); //                      mova   [dstq+strideq*1], xm0
        movx_st_p1(at(dst, stride * 2), m[0]); //                  mova   [dstq+strideq*2], xm0
        movx_st_p1(at(dst, stride3), m[0]); //                     mova   [dstq+stride3q ], xm0
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        h -= 4; //                                                 sub hd, 4
        if h <= 0 {
            break; //                                              jg .s16
        }
    }
    // RET
}

/// `ipred_dc_8bpc.s32`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_s32(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, mut h: c_int) {
    loop {
        movy_st_p1(dst, m[0]); //                                  mova   [dstq+strideq*0], m0
        movy_st_p1(at(dst, stride), m[0]); //                      mova   [dstq+strideq*1], m0
        movy_st_p1(at(dst, stride * 2), m[0]); //                  mova   [dstq+strideq*2], m0
        movy_st_p1(at(dst, stride3), m[0]); //                     mova   [dstq+stride3q ], m0
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        h -= 4; //                                                 sub hd, 4
        if h <= 0 {
            break; //                                              jg .s32
        }
    }
    // RET
}

/// `ipred_dc_8bpc.s64`: `m0` left, `m1` right.
#[inline(always)]
unsafe fn ipred_dc_8bpc_s64(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, mut h: c_int) {
    loop {
        movy_st_p1(dst, m[0]); //                                  mova [dstq+strideq*0+32*0], m0
        movy_st_p1(at(dst, 32), m[1]); //                          mova [dstq+strideq*0+32*1], m1
        movy_st_p1(at(dst, stride), m[0]); //                      mova [dstq+strideq*1+32*0], m0
        movy_st_p1(at(dst, stride + 32), m[1]); //                 mova [dstq+strideq*1+32*1], m1
        movy_st_p1(at(dst, stride * 2), m[0]); //                  mova [dstq+strideq*2+32*0], m0
        movy_st_p1(at(dst, stride * 2 + 32), m[1]); //             mova [dstq+strideq*2+32*1], m1
        movy_st_p1(at(dst, stride3), m[0]); //                     mova [dstq+stride3q +32*0], m0
        movy_st_p1(at(dst, stride3 + 32), m[1]); //                mova [dstq+stride3q +32*1], m1
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        h -= 4; //                                                 sub hd, 4
        if h <= 0 {
            break; //                                              jg .s64
        }
    }
    // RET
}

/// `jmp wq` through `ipred_dc_splat_avx2_table` (`ipred_dc_avx2_table + 10*4`: `.s4` … `.s64` of
/// `ipred_dc_8bpc`), index `tzcnt(w)` (2..=6 by the contract).
#[inline(always)]
unsafe fn ipred_dc_splat_p1(m: &mut M, dst: *mut u8, stride: isize, stride3: isize, h: c_int, wl: u32) {
    match wl {
        2 => ipred_dc_8bpc_s4(m, dst, stride, stride3, h),
        3 => ipred_dc_8bpc_s8(m, dst, stride, stride3, h),
        4 => ipred_dc_8bpc_s16(m, dst, stride, stride3, h),
        5 => ipred_dc_8bpc_s32(m, dst, stride, stride3, h),
        6 => ipred_dc_8bpc_s64(m, dst, stride, stride3, h),
        _ => unreachable!(), // outside the jump table (contract: 2..=6)
    }
}

// ---- ipred_dc_left_8bpc: .h64 … .h4 (also run by ipred_dc_top_8bpc) ----
// The labels fall through into each other; each is a function that ends in a call of the next. `m0` holds the
// negated pair sums (`pmaddubsw` with -1), `m2` -1 bytes, `xm3` 0x8000 >> log2(n). `.h4` ends in `jmp wq`, which
// the callers do (`ipred_dc_splat_p1`).

/// `ipred_dc_left_8bpc.h64` (`tl` is `tlq`: `[tlq+32]`, unaligned when dc_top jumps here).
#[inline(always)]
unsafe fn ipred_dc_left_8bpc_h64(m: &mut M, tl: *const u8) {
    m[1] = y_p1(a_p1(tl, 32)); //                                  movu m1, [tlq+32]
    m[1] = _mm256_maddubs_epi16(m[1], m[2]); //                    pmaddubsw m1, m2
    m[0] = _mm256_add_epi16(m[0], m[1]); //                        paddw m0, m1
    ipred_dc_left_8bpc_h32(m);
}

/// `ipred_dc_left_8bpc.h32`.
#[inline(always)]
unsafe fn ipred_dc_left_8bpc_h32(m: &mut M) {
    m[1] = vextracti128_p1(m[0]); //                               vextracti128 xm1, m0, 1
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    ipred_dc_left_8bpc_h16(m);
}

/// `ipred_dc_left_8bpc.h16`.
#[inline(always)]
unsafe fn ipred_dc_left_8bpc_h16(m: &mut M) {
    m[1] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[0]))); //          punpckhqdq xm1, xm0, xm0
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    ipred_dc_left_8bpc_h8(m);
}

/// `ipred_dc_left_8bpc.h8`.
#[inline(always)]
unsafe fn ipred_dc_left_8bpc_h8(m: &mut M) {
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); //                  psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    ipred_dc_left_8bpc_h4(m);
}

/// `ipred_dc_left_8bpc.h4` up to `jmp wq` (the stride3 `lea` is the caller's).
#[inline(always)]
unsafe fn ipred_dc_left_8bpc_h4(m: &mut M) {
    m[0] = zx(_mm_madd_epi16(lo(m[0]), lo(m[2]))); //              pmaddwd xm0, xm2
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[3]))); //            pmulhrsw xm0, xm3
                                                     //                                                             lea stride3q, [strideq*3]
    m[0] = _mm256_broadcastb_epi8(lo(m[0])); //                    vpbroadcastb m0, xm0
    m[1] = m[0]; //                                                mova m1, m0
}

/// `jmp r6` through `ipred_dc_left_avx2_table` (`.h4` … `.h64` of `ipred_dc_left_8bpc`), index `tzcnt(n)`
/// (2..=6 by the contract).
#[inline(always)]
unsafe fn ipred_dc_left_jmp_p1(m: &mut M, tl: *const u8, nl: u32) {
    match nl {
        2 => ipred_dc_left_8bpc_h4(m),
        3 => ipred_dc_left_8bpc_h8(m),
        4 => ipred_dc_left_8bpc_h16(m),
        5 => ipred_dc_left_8bpc_h32(m),
        6 => ipred_dc_left_8bpc_h64(m, tl),
        _ => unreachable!(), // outside the jump table (contract: 2..=6)
    }
}

/// `ipred_dc_top_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_dc_top_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    //                                                             lea r5, [ipred_dc_left_avx2_table]
    let wl = tzcnt_p1(width); //                                   tzcnt wd, wm
    let tl = a_p1(topleft, 1); //                                  inc tlq
    m[0] = y_p1(tl); //                                            movu m0, [tlq]
    let h = height; //                                             movifnidn hd, hm
    let r6 = 0x8000u32 >> wl; //                                   mov r6d, 0x8000; shrx r6d, r6d, wd
    m[3] = zx(_mm_cvtsi32_si128(r6 as i32)); //                    movd xm3, r6d
                                             //                                                             movsxd r6, [r5+wq*4]
    m[2] = _mm256_cmpeq_epi32(m[2], m[2]); //                      pcmpeqd m2, m2
    m[0] = _mm256_maddubs_epi16(m[0], m[2]); //                    pmaddubsw m0, m2
                                             //                                                             add r6, r5 / add r5, … / movsxd wq, … / add wq, r5
    ipred_dc_left_jmp_p1(m, tl, wl); //                            jmp r6
    let stride3 = stride * 3; //                                   lea stride3q, [strideq*3] (.h4)
    ipred_dc_splat_p1(m, dst, stride, stride3, h, wl); //          jmp wq
}

/// `ipred_dc_left_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_dc_left_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    let h = height; //                                             mov hd, hm
    let r6 = tzcnt_p1(h); //                                       tzcnt r6d, hd
    let tl = a_p1(topleft, -(h as isize)); //                      sub tlq, hq
    let wl = tzcnt_p1(width); //                                   tzcnt wd, wm
    m[0] = y_p1(tl); //                                            movu m0, [tlq]
    let r5 = 0x8000u32 >> r6; //                                   mov r5d, 0x8000; shrx r5d, r5d, r6d
    m[3] = zx(_mm_cvtsi32_si128(r5 as i32)); //                    movd xm3, r5d
                                             //                                                             lea r5, [ipred_dc_left_avx2_table]; movsxd r6, …
    m[2] = _mm256_cmpeq_epi32(m[2], m[2]); //                      pcmpeqd m2, m2
    m[0] = _mm256_maddubs_epi16(m[0], m[2]); //                    pmaddubsw m0, m2
                                             //                                                             add r6, r5 / add r5, … / movsxd wq, … / add wq, r5
    ipred_dc_left_jmp_p1(m, tl, r6); //                            jmp r6
    let stride3 = stride * 3; //                                   lea stride3q, [strideq*3] (.h4)
    ipred_dc_splat_p1(m, dst, stride, stride3, h, wl); //          jmp wq
}

// ---- ipred_dc_8bpc: .h4 … .h64 (the left sums), .w4 … .w64 (top sums, division, then .sN) ----
// `m3` holds -1 bytes, `xm4` (w+h)/2 in its first word, `xm5` tzcnt(w+h).

/// `ipred_dc_8bpc.h4` up to `jmp wq`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_h4(m: &mut M, tl: *const u8) {
    m[0] = zx(movd_p1(a_p1(tl, -4))); //                           movd xm0, [tlq-4]
    m[0] = zx(_mm_maddubs_epi16(lo(m[0]), lo(m[3]))); //           pmaddubsw xm0, xm3
}

/// `ipred_dc_8bpc.h8` up to `jmp wq`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_h8(m: &mut M, tl: *const u8) {
    m[0] = zx(movq_p1(a_p1(tl, -8))); //                           movq xm0, [tlq-8]
    m[0] = zx(_mm_maddubs_epi16(lo(m[0]), lo(m[3]))); //           pmaddubsw xm0, xm3
}

/// `ipred_dc_8bpc.h16` up to `jmp wq`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_h16(m: &mut M, tl: *const u8) {
    m[0] = zx(x_p1(a_p1(tl, -16))); //                             mova xm0, [tlq-16]
    m[0] = zx(_mm_maddubs_epi16(lo(m[0]), lo(m[3]))); //           pmaddubsw xm0, xm3
}

/// `ipred_dc_8bpc.h32` up to `jmp wq`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_h32(m: &mut M, tl: *const u8) {
    m[0] = y_p1(a_p1(tl, -32)); //                                 mova m0, [tlq-32]
    m[0] = _mm256_maddubs_epi16(m[0], m[3]); //                    pmaddubsw m0, m3
}

/// `ipred_dc_8bpc.h64` up to `jmp wq`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_h64(m: &mut M, tl: *const u8) {
    m[0] = y_p1(a_p1(tl, -64)); //                                 mova m0, [tlq-64]
    m[1] = y_p1(a_p1(tl, -32)); //                                 mova m1, [tlq-32]
    m[0] = _mm256_maddubs_epi16(m[0], m[3]); //                    pmaddubsw m0, m3
    m[1] = _mm256_maddubs_epi16(m[1], m[3]); //                    pmaddubsw m1, m3
    m[0] = _mm256_add_epi16(m[0], m[1]); //                        paddw m0, m1
}

/// `ipred_dc_8bpc.w4` with `.w4_mul`, `.w4_end` and `.s4`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_w4(m: &mut M, dst: *mut u8, stride: isize, stride3: isize, tl: *const u8, h: c_int) {
    m[1] = zx(movd_p1(a_p1(tl, 1))); //                            movd xm1, [tlq+1]
    m[1] = zx(_mm_maddubs_epi16(lo(m[1]), lo(m[3]))); //           pmaddubsw xm1, xm3
    m[0] = zx(_mm_sub_epi16(lo(m[0]), lo(m[4]))); //               psubw xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    m[0] = zx(_mm_madd_epi16(lo(m[0]), lo(m[3]))); //              pmaddwd xm0, xm3
    if h > 4 {
        //                                                         cmp hd, 4; jg .w4_mul
        // .w4_mul:
        m[1] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[0]))); //      punpckhqdq xm1, xm0, xm0
        let r2 = (h as u32) * 2; //                                lea r2d, [hq*2]
        let r6 = 0x55563334u32; //                                 mov r6d, 0x55563334
        m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //           paddw xm0, xm1
        let r6 = r6.wrapping_shr(r2); //                           shrx r6d, r6d, r2d (count mod 32)
        m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); //              psrlq xm1, xm0, 32
        m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //           paddw xm0, xm1
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); //                movd xm1, r6d
        m[0] = zx(_mm_srli_epi16::<2>(lo(m[0]))); //               psrlw xm0, 2
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); //         pmulhuw xm0, xm1
    } else {
        m[0] = zx(_mm_srli_epi16::<3>(lo(m[0]))); //               psrlw xm0, 3; jmp .w4_end
    }
    // .w4_end:
    m[0] = zx(_mm_broadcastb_epi8(lo(m[0]))); //                   vpbroadcastb xm0, xm0
    ipred_dc_8bpc_s4(m, dst, stride, stride3, h);
}

/// `ipred_dc_8bpc.w8` with `.w8_end` and `.s8`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_w8(m: &mut M, dst: *mut u8, stride: isize, stride3: isize, tl: *const u8, h: c_int) {
    m[1] = zx(movq_p1(a_p1(tl, 1))); //                            movq xm1, [tlq+1]
    m[2] = vextracti128_p1(m[0]); //                               vextracti128 xm2, m0, 1
    m[1] = zx(_mm_maddubs_epi16(lo(m[1]), lo(m[3]))); //           pmaddubsw xm1, xm3
    m[0] = zx(_mm_sub_epi16(lo(m[0]), lo(m[4]))); //               psubw xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); //               paddw xm0, xm2
    m[2] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[0]))); //          punpckhqdq xm2, xm0, xm0
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); //               paddw xm0, xm2
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); //                  psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    m[0] = zx(_mm_madd_epi16(lo(m[0]), lo(m[3]))); //              pmaddwd xm0, xm3
    m[0] = zx(_mm_srl_epi16(lo(m[0]), lo(m[5]))); //               psrlw xm0, xm5
    if h != 8 {
        //                                                         cmp hd, 8; je .w8_end
        let mut r6 = 0x5556u32; //                                 mov r6d, 0x5556
        let r2 = 0x3334u32; //                                     mov r2d, 0x3334
        if h == 32 {
            r6 = r2; //                                            cmp hd, 32; cmove r6d, r2d
        }
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); //                movd xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); //         pmulhuw xm0, xm1
    }
    // .w8_end:
    m[0] = zx(_mm_broadcastb_epi8(lo(m[0]))); //                   vpbroadcastb xm0, xm0
    ipred_dc_8bpc_s8(m, dst, stride, stride3, h);
}

/// `ipred_dc_8bpc.w16` with `.w16_end` and `.s16`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_w16(m: &mut M, dst: *mut u8, stride: isize, stride3: isize, tl: *const u8, h: c_int) {
    m[1] = zx(x_p1(a_p1(tl, 1))); //                               movu xm1, [tlq+1]
    m[2] = vextracti128_p1(m[0]); //                               vextracti128 xm2, m0, 1
    m[1] = zx(_mm_maddubs_epi16(lo(m[1]), lo(m[3]))); //           pmaddubsw xm1, xm3
    m[0] = zx(_mm_sub_epi16(lo(m[0]), lo(m[4]))); //               psubw xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); //               paddw xm0, xm2
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    m[1] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[0]))); //          punpckhqdq xm1, xm0, xm0
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); //                  psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    m[0] = zx(_mm_madd_epi16(lo(m[0]), lo(m[3]))); //              pmaddwd xm0, xm3
    m[0] = zx(_mm_srl_epi16(lo(m[0]), lo(m[5]))); //               psrlw xm0, xm5
    if h != 16 {
        //                                                         cmp hd, 16; je .w16_end
        let mut r6 = 0x5556u32; //                                 mov r6d, 0x5556
        let r2 = 0x3334u32; //                                     mov r2d, 0x3334
        if h as u8 & (8 | 32) == 0 {
            r6 = r2; //                                            test hb, 8|32; cmovz r6d, r2d
        }
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); //                movd xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); //         pmulhuw xm0, xm1
    }
    // .w16_end:
    m[0] = zx(_mm_broadcastb_epi8(lo(m[0]))); //                   vpbroadcastb xm0, xm0
    ipred_dc_8bpc_s16(m, dst, stride, stride3, h);
}

/// `ipred_dc_8bpc.w32` with `.w32_end` and `.s32`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_w32(m: &mut M, dst: *mut u8, stride: isize, stride3: isize, tl: *const u8, h: c_int) {
    m[1] = y_p1(a_p1(tl, 1)); //                                   movu m1, [tlq+1]
    m[1] = _mm256_maddubs_epi16(m[1], m[3]); //                    pmaddubsw m1, m3
    m[0] = _mm256_add_epi16(m[0], m[1]); //                        paddw m0, m1
    m[1] = vextracti128_p1(m[0]); //                               vextracti128 xm1, m0, 1
    m[0] = zx(_mm_sub_epi16(lo(m[0]), lo(m[4]))); //               psubw xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    m[1] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[0]))); //          punpckhqdq xm1, xm0, xm0
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); //                  psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    m[0] = zx(_mm_madd_epi16(lo(m[0]), lo(m[3]))); //              pmaddwd xm0, xm3
    m[0] = zx(_mm_srl_epi16(lo(m[0]), lo(m[5]))); //               psrlw xm0, xm5
    if h != 32 {
        //                                                         cmp hd, 32; je .w32_end
        let r2 = (h as u32) * 2; //                                lea r2d, [hq*2]
        let r6 = 0x33345556u32.wrapping_shr(r2); //                mov r6d, 0x33345556; shrx r6d, r6d, r2d
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); //                movd xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); //         pmulhuw xm0, xm1
    }
    // .w32_end:
    m[0] = _mm256_broadcastb_epi8(lo(m[0])); //                    vpbroadcastb m0, xm0
    ipred_dc_8bpc_s32(m, dst, stride, stride3, h);
}

/// `ipred_dc_8bpc.w64` with `.w64_end` and `.s64`.
#[inline(always)]
unsafe fn ipred_dc_8bpc_w64(m: &mut M, dst: *mut u8, stride: isize, stride3: isize, tl: *const u8, h: c_int) {
    m[1] = y_p1(a_p1(tl, 1)); //                                   movu m1, [tlq+ 1]
    m[2] = y_p1(a_p1(tl, 33)); //                                  movu m2, [tlq+33]
    m[1] = _mm256_maddubs_epi16(m[1], m[3]); //                    pmaddubsw m1, m3
    m[2] = _mm256_maddubs_epi16(m[2], m[3]); //                    pmaddubsw m2, m3
    m[0] = _mm256_add_epi16(m[0], m[1]); //                        paddw m0, m1
    m[0] = _mm256_add_epi16(m[0], m[2]); //                        paddw m0, m2
    m[1] = vextracti128_p1(m[0]); //                               vextracti128 xm1, m0, 1
    m[0] = zx(_mm_sub_epi16(lo(m[0]), lo(m[4]))); //               psubw xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    m[1] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[0]))); //          punpckhqdq xm1, xm0, xm0
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); //                  psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); //               paddw xm0, xm1
    m[0] = zx(_mm_madd_epi16(lo(m[0]), lo(m[3]))); //              pmaddwd xm0, xm3
    m[0] = zx(_mm_srl_epi16(lo(m[0]), lo(m[5]))); //               psrlw xm0, xm5
    if h != 64 {
        //                                                         cmp hd, 64; je .w64_end
        let r6 = 0x33345556u32.wrapping_shr(h as u32); //          mov r6d, 0x33345556; shrx r6d, r6d, hd
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); //                movd xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); //         pmulhuw xm0, xm1
    }
    // .w64_end:
    m[0] = _mm256_broadcastb_epi8(lo(m[0])); //                    vpbroadcastb m0, xm0
    m[1] = m[0]; //                                                mova m1, m0
    ipred_dc_8bpc_s64(m, dst, stride, stride3, h);
}

/// `ipred_dc_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_dc_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    let tl = topleft;
    let h = height; //                                             movifnidn hd, hm
    let w = width; //                                              movifnidn wd, wm
    let r6 = tzcnt_p1(h); //                                       tzcnt r6d, hd
    let r5 = (w + h) as u32; //                                    lea r5d, [wq+hq]
    m[4] = zx(_mm_cvtsi32_si128(r5 as i32)); //                    movd xm4, r5d
    let r5 = r5.trailing_zeros(); //                               tzcnt r5d, r5d (w+h is at least 8)
    m[5] = zx(_mm_cvtsi32_si128(r5 as i32)); //                    movd xm5, r5d
                                             //                                                             lea r5, [ipred_dc_avx2_table]
    let wl = tzcnt_p1(w); //                                       tzcnt wd, wd
                          //                                                             movsxd r6, [r5+r6*4]; movsxd wq, [r5+wq*4+5*4]
    m[3] = _mm256_cmpeq_epi32(m[3], m[3]); //                      pcmpeqd m3, m3
    m[4] = zx(_mm_srli_epi16::<1>(lo(m[4]))); //                   psrlw xm4, 1
                                              //                                                             add r6, r5; add wq, r5
    let stride3 = stride * 3; //                                   lea stride3q, [strideq*3]
    match r6 {
        //                                                         jmp r6
        2 => ipred_dc_8bpc_h4(m, tl),
        3 => ipred_dc_8bpc_h8(m, tl),
        4 => ipred_dc_8bpc_h16(m, tl),
        5 => ipred_dc_8bpc_h32(m, tl),
        6 => ipred_dc_8bpc_h64(m, tl),
        _ => unreachable!(), // outside the jump table (contract: 2..=6)
    }
    match wl {
        //                                                         jmp wq
        2 => ipred_dc_8bpc_w4(m, dst, stride, stride3, tl, h),
        3 => ipred_dc_8bpc_w8(m, dst, stride, stride3, tl, h),
        4 => ipred_dc_8bpc_w16(m, dst, stride, stride3, tl, h),
        5 => ipred_dc_8bpc_w32(m, dst, stride, stride3, tl, h),
        6 => ipred_dc_8bpc_w64(m, dst, stride, stride3, tl, h),
        _ => unreachable!(), // outside the jump table (contract: 2..=6)
    }
}

/// `ipred_dc_128_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_dc_128_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    _topleft: *const u8,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    //                                                             lea r5, [ipred_dc_splat_avx2_table]
    let wl = tzcnt_p1(width); //                                   tzcnt wd, wm
    let h = height; //                                             movifnidn hd, hm
                    //                                                             movsxd wq, [r5+wq*4]
    m[0] = vpbroadcastd_p1(ro(pb_128, 0)); //                      vpbroadcastd m0, [r5-ipred_dc_splat_avx2_table+pb_128]
    m[1] = m[0]; //                                                mova m1, m0
                 //                                                             add wq, r5
    let stride3 = stride * 3; //                                   lea stride3q, [strideq*3]
    ipred_dc_splat_p1(m, dst, stride, stride3, h, wl); //          jmp wq
}

/// `ipred_v_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_v_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    let tl = topleft;
    //                                                             lea r5, [ipred_dc_splat_avx2_table]
    let wl = tzcnt_p1(width); //                                   tzcnt wd, wm
    m[0] = y_p1(a_p1(tl, 1)); //                                   movu m0, [tlq+ 1]
    m[1] = y_p1(a_p1(tl, 33)); //                                  movu m1, [tlq+33]
    let h = height; //                                             movifnidn hd, hm
                    //                                                             movsxd wq, [r5+wq*4]; add wq, r5
    let stride3 = stride * 3; //                                   lea stride3q, [strideq*3]
    ipred_dc_splat_p1(m, dst, stride, stride3, h, wl); //          jmp wq
}

// ---- ipred_h_8bpc ----

/// `%macro IPRED_H w, store_type` with its loop (`jg .w%1`) and `RET`: rows of the left pixels, four per
/// iteration. `W` 4, 8, 16 are `INIT_XMM` (`vpbroadcastb xm`; `movd`, `movq`, `mova xm` stores), 32 `INIT_YMM`.
#[inline(always)]
unsafe fn ipred_h<const W: usize>(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, tl: *const u8, h: c_int) {
    let mut tl = tl;
    let mut h = h;
    loop {
        if W <= 16 {
            m[0] = vpbroadcastb_x_p1(a_p1(tl, -1)); //             vpbroadcastb m0, [tlq-1]
            m[1] = vpbroadcastb_x_p1(a_p1(tl, -2)); //             vpbroadcastb m1, [tlq-2]
            m[2] = vpbroadcastb_x_p1(a_p1(tl, -3)); //             vpbroadcastb m2, [tlq-3]
            tl = a_p1(tl, -4); //                                  sub tlq, 4
            m[3] = vpbroadcastb_x_p1(tl); //                       vpbroadcastb m3, [tlq+0]
        } else {
            m[0] = vpbroadcastb_p1(a_p1(tl, -1)); //               vpbroadcastb m0, [tlq-1]
            m[1] = vpbroadcastb_p1(a_p1(tl, -2)); //               vpbroadcastb m1, [tlq-2]
            m[2] = vpbroadcastb_p1(a_p1(tl, -3)); //               vpbroadcastb m2, [tlq-3]
            tl = a_p1(tl, -4); //                                  sub tlq, 4
            m[3] = vpbroadcastb_p1(tl); //                         vpbroadcastb m3, [tlq+0]
        }
        let rows = [dst, at(dst, stride), at(dst, stride * 2), at(dst, stride3)];
        for (r, p) in rows.into_iter().enumerate() {
            //                                                     mov%2 [dstq+strideq*0/1/2, stride3q], m0/1/2/3
            match W {
                4 => movd_st_p1(p, m[r]),
                8 => movq_st_p1(p, m[r]),
                16 => movx_st_p1(p, m[r]),
                _ => movy_st_p1(p, m[r]),
            }
        }
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        h -= 4; //                                                 sub hd, 4
        if h <= 0 {
            break; //                                              jg .w%1
        }
    }
    // RET
}

/// `ipred_h_8bpc.w64`.
#[inline(always)]
unsafe fn ipred_h_8bpc_w64(m: &mut M, mut dst: *mut u8, stride: isize, stride3: isize, tl: *const u8, h: c_int) {
    let mut tl = tl;
    let mut h = h;
    loop {
        m[0] = vpbroadcastb_p1(a_p1(tl, -1)); //                   vpbroadcastb m0, [tlq-1]
        m[1] = vpbroadcastb_p1(a_p1(tl, -2)); //                   vpbroadcastb m1, [tlq-2]
        m[2] = vpbroadcastb_p1(a_p1(tl, -3)); //                   vpbroadcastb m2, [tlq-3]
        tl = a_p1(tl, -4); //                                      sub tlq, 4
        m[3] = vpbroadcastb_p1(tl); //                             vpbroadcastb m3, [tlq+0]
        movy_st_p1(dst, m[0]); //                                  mova [dstq+strideq*0+32*0], m0
        movy_st_p1(at(dst, 32), m[0]); //                          mova [dstq+strideq*0+32*1], m0
        movy_st_p1(at(dst, stride), m[1]); //                      mova [dstq+strideq*1+32*0], m1
        movy_st_p1(at(dst, stride + 32), m[1]); //                 mova [dstq+strideq*1+32*1], m1
        movy_st_p1(at(dst, stride * 2), m[2]); //                  mova [dstq+strideq*2+32*0], m2
        movy_st_p1(at(dst, stride * 2 + 32), m[2]); //             mova [dstq+strideq*2+32*1], m2
        movy_st_p1(at(dst, stride3), m[3]); //                     mova [dstq+stride3q +32*0], m3
        movy_st_p1(at(dst, stride3 + 32), m[3]); //                mova [dstq+stride3q +32*1], m3
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        h -= 4; //                                                 sub hd, 4
        if h <= 0 {
            break; //                                              jg .w64
        }
    }
    // RET
}

/// `ipred_h_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_h_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    let tl = topleft;
    //                                                             lea r5, [ipred_h_avx2_table]
    let wl = tzcnt_p1(width); //                                   tzcnt wd, wm
    let h = height; //                                             movifnidn hd, hm
                    //                                                             movsxd wq, [r5+wq*4]; add wq, r5
    let stride3 = stride * 3; //                                   lea stride3q, [strideq*3]
    match wl {
        //                                                         jmp wq
        2 => ipred_h::<4>(m, dst, stride, stride3, tl, h), //      .w4: IPRED_H 4, d
        3 => ipred_h::<8>(m, dst, stride, stride3, tl, h), //      .w8: IPRED_H 8, q
        4 => ipred_h::<16>(m, dst, stride, stride3, tl, h), //     .w16: IPRED_H 16, a
        5 => ipred_h::<32>(m, dst, stride, stride3, tl, h), //     .w32: IPRED_H 32, a (INIT_YMM)
        6 => ipred_h_8bpc_w64(m, dst, stride, stride3, tl, h),
        _ => unreachable!(), // outside the jump table (contract: 2..=6)
    }
}

// ---- ipred_paeth_8bpc ----

/// `%macro PAETH top, ldiff`: `m0` = the Paeth predictor of the top pixels `m[top]`, the left pixels `m3` and
/// the top-left pixel `m5`, with `m[ldiff]` = |top - topleft| and `m4` = 1 bytes; clobbers `m1`, `m2`.
#[inline(always)]
unsafe fn paeth(m: &mut M, top: usize, ldiff: usize) {
    m[1] = _mm256_avg_epu8(m[top], m[3]); //                       pavgb m1, m%1, m3
    m[0] = _mm256_xor_si256(m[top], m[3]); //                      pxor m0, m%1, m3
    m[0] = _mm256_and_si256(m[0], m[4]); //                        pand m0, m4
    m[2] = _mm256_subs_epu8(m[5], m[1]); //                        psubusb m2, m5, m1
    m[1] = _mm256_sub_epi8(m[1], m[0]); //                         psubb m1, m0
    m[1] = _mm256_subs_epu8(m[1], m[5]); //                        psubusb m1, m5
    m[1] = _mm256_or_si256(m[1], m[2]); //                         por m1, m2
    m[1] = _mm256_adds_epu8(m[1], m[1]); //                        paddusb m1, m1
    m[1] = _mm256_or_si256(m[1], m[0]); //                         por m1, m0 ; min(tldiff, 255)
    m[2] = _mm256_subs_epu8(m[5], m[3]); //                        psubusb m2, m5, m3
    m[0] = _mm256_subs_epu8(m[3], m[5]); //                        psubusb m0, m3, m5
    m[2] = _mm256_or_si256(m[2], m[0]); //                         por m2, m0 ; tdiff
    m[2] = _mm256_min_epu8(m[2], m[ldiff]); //                     pminub m2, m%2
    m[0] = _mm256_cmpeq_epi8(m[ldiff], m[2]); //                   pcmpeqb m0, m%2, m2 ; ldiff <= tdiff
    m[0] = _mm256_blendv_epi8(m[top], m[3], m[0]); //              vpblendvb m0, m%1, m3, m0
    m[1] = _mm256_min_epu8(m[1], m[2]); //                         pminub m1, m2
    m[1] = _mm256_cmpeq_epi8(m[1], m[2]); //                       pcmpeqb m1, m2 ; ldiff <= tldiff || tdiff <= tldiff
    m[0] = _mm256_blendv_epi8(m[5], m[0], m[1]); //                vpblendvb m0, m5, m0, m1
}

/// `ipred_paeth_8bpc.w4` with `.w4_loop` and `.ret`.
#[inline(always)]
unsafe fn ipred_paeth_8bpc_w4(m: &mut M, mut dst: *mut u8, stride: isize, mut tl: *const u8, mut h: c_int) {
    m[6] = vpbroadcastd_p1(a_p1(tl, 1)); //                        vpbroadcastd m6, [tlq+1] ; top
    m[8] = y_p1(ro(ipred_h_shuf, 0)); //                           mova m8, [base+ipred_h_shuf]
    let r3 = stride * 3; //                                        lea r3, [strideq*3]
    m[7] = _mm256_subs_epu8(m[5], m[6]); //                        psubusb m7, m5, m6
    m[0] = _mm256_subs_epu8(m[6], m[5]); //                        psubusb m0, m6, m5
    m[7] = _mm256_or_si256(m[7], m[0]); //                         por m7, m0 ; ldiff
    loop {
        // .w4_loop:
        tl = a_p1(tl, -8); //                                      sub tlq, 8
        m[3] = vpbroadcastq_p1(tl); //                             vpbroadcastq m3, [tlq]
        m[3] = _mm256_shuffle_epi8(m[3], m[8]); //                 pshufb m3, m8 ; left
        paeth(m, 6, 7); //                                         PAETH 6, 7
        m[1] = vextracti128_p1(m[0]); //                           vextracti128 xm1, m0, 1
        movd_st_p1(dst, m[0]); //                                  movd   [dstq+strideq*0], xm0
        movd_st_p1(at(dst, stride), m[1]); //                      movd   [dstq+strideq*1], xm1
        pextrd_st_p1::<2>(at(dst, stride * 2), m[0]); //           pextrd [dstq+strideq*2], xm0, 2
        pextrd_st_p1::<2>(at(dst, r3), m[1]); //                   pextrd [dstq+r3       ], xm1, 2
        if h == 4 {
            break; //                                              cmp hd, 4; je .ret
        }
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        pextrd_st_p1::<1>(dst, m[0]); //                           pextrd [dstq+strideq*0], xm0, 1
        pextrd_st_p1::<1>(at(dst, stride), m[1]); //               pextrd [dstq+strideq*1], xm1, 1
        pextrd_st_p1::<3>(at(dst, stride * 2), m[0]); //           pextrd [dstq+strideq*2], xm0, 3
        pextrd_st_p1::<3>(at(dst, r3), m[1]); //                   pextrd [dstq+r3       ], xm1, 3
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        h -= 8; //                                                 sub hd, 8
        if h <= 0 {
            break; //                                              jg .w4_loop
        }
    }
    // .ret: RET
}

/// `ipred_paeth_8bpc.w8` with `.w8_loop`.
#[inline(always)]
unsafe fn ipred_paeth_8bpc_w8(m: &mut M, mut dst: *mut u8, stride: isize, mut tl: *const u8, mut h: c_int) {
    m[6] = vpbroadcastq_p1(a_p1(tl, 1)); //                        vpbroadcastq m6, [tlq+1]
    m[8] = y_p1(ro(ipred_h_shuf, 0)); //                           mova m8, [base+ipred_h_shuf]
    let r3 = stride * 3; //                                        lea r3, [strideq*3]
    m[7] = _mm256_subs_epu8(m[5], m[6]); //                        psubusb m7, m5, m6
    m[0] = _mm256_subs_epu8(m[6], m[5]); //                        psubusb m0, m6, m5
    m[7] = _mm256_or_si256(m[7], m[0]); //                         por m7, m0
    loop {
        // .w8_loop:
        tl = a_p1(tl, -4); //                                      sub tlq, 4
        m[3] = vpbroadcastd_p1(tl); //                             vpbroadcastd m3, [tlq]
        m[3] = _mm256_shuffle_epi8(m[3], m[8]); //                 pshufb m3, m8
        paeth(m, 6, 7); //                                         PAETH 6, 7
        m[1] = vextracti128_p1(m[0]); //                           vextracti128 xm1, m0, 1
        movq_st_p1(dst, m[0]); //                                  movq   [dstq+strideq*0], xm0
        movq_st_p1(at(dst, stride), m[1]); //                      movq   [dstq+strideq*1], xm1
        movhps_st_p1(at(dst, stride * 2), m[0]); //                movhps [dstq+strideq*2], xm0
        movhps_st_p1(at(dst, r3), m[1]); //                        movhps [dstq+r3       ], xm1
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        h -= 4; //                                                 sub hd, 4
        if h <= 0 {
            break; //                                              jg .w8_loop
        }
    }
    // RET
}

/// `ipred_paeth_8bpc.w16` with `.w16_loop`.
#[inline(always)]
unsafe fn ipred_paeth_8bpc_w16(m: &mut M, mut dst: *mut u8, stride: isize, mut tl: *const u8, mut h: c_int) {
    m[6] = vbroadcasti128_p1(a_p1(tl, 1)); //                      vbroadcasti128 m6, [tlq+1]
    m[8] = zx(lo(m[4])); //                                        mova xm8, xm4 ; lower half = 1, upper half = 0
    m[7] = _mm256_subs_epu8(m[5], m[6]); //                        psubusb m7, m5, m6
    m[0] = _mm256_subs_epu8(m[6], m[5]); //                        psubusb m0, m6, m5
    m[7] = _mm256_or_si256(m[7], m[0]); //                         por m7, m0
    loop {
        // .w16_loop:
        tl = a_p1(tl, -2); //                                      sub tlq, 2
        m[3] = vpbroadcastd_p1(tl); //                             vpbroadcastd m3, [tlq]
        m[3] = _mm256_shuffle_epi8(m[3], m[8]); //                 pshufb m3, m8
        paeth(m, 6, 7); //                                         PAETH 6, 7
        movx_st_p1(dst, m[0]); //                                  mova [dstq+strideq*0], xm0
        vextracti128_st_p1(at(dst, stride), m[0]); //              vextracti128 [dstq+strideq*1], m0, 1
        dst = at(dst, stride * 2); //                              lea dstq, [dstq+strideq*2]
        h -= 2; //                                                 sub hd, 2
        if h <= 0 {
            break; //                                              jg .w16_loop
        }
    }
    // RET
}

/// `ipred_paeth_8bpc.w32` with `.w32_loop`.
#[inline(always)]
unsafe fn ipred_paeth_8bpc_w32(m: &mut M, mut dst: *mut u8, stride: isize, mut tl: *const u8, mut h: c_int) {
    m[6] = y_p1(a_p1(tl, 1)); //                                   movu m6, [tlq+1]
    m[7] = _mm256_subs_epu8(m[5], m[6]); //                        psubusb m7, m5, m6
    m[0] = _mm256_subs_epu8(m[6], m[5]); //                        psubusb m0, m6, m5
    m[7] = _mm256_or_si256(m[7], m[0]); //                         por m7, m0
    loop {
        // .w32_loop:
        tl = a_p1(tl, -1); //                                      dec tlq
        m[3] = vpbroadcastb_p1(tl); //                             vpbroadcastb m3, [tlq]
        paeth(m, 6, 7); //                                         PAETH 6, 7
        movy_st_p1(dst, m[0]); //                                  mova [dstq], m0
        dst = at(dst, stride); //                                  add dstq, strideq
        h -= 1; //                                                 dec hd
        if h <= 0 {
            break; //                                              jg .w32_loop
        }
    }
    // RET
}

/// `ipred_paeth_8bpc.w64` with `.w64_loop` (the `WIN64` save of `xmm9` does not apply).
#[inline(always)]
unsafe fn ipred_paeth_8bpc_w64(m: &mut M, mut dst: *mut u8, stride: isize, mut tl: *const u8, mut h: c_int) {
    m[6] = y_p1(a_p1(tl, 1)); //                                   movu m6, [tlq+ 1]
    m[7] = y_p1(a_p1(tl, 33)); //                                  movu m7, [tlq+33]
    m[8] = _mm256_subs_epu8(m[5], m[6]); //                        psubusb m8, m5, m6
    m[0] = _mm256_subs_epu8(m[6], m[5]); //                        psubusb m0, m6, m5
    m[9] = _mm256_subs_epu8(m[5], m[7]); //                        psubusb m9, m5, m7
    m[1] = _mm256_subs_epu8(m[7], m[5]); //                        psubusb m1, m7, m5
    m[8] = _mm256_or_si256(m[8], m[0]); //                         por m8, m0
    m[9] = _mm256_or_si256(m[9], m[1]); //                         por m9, m1
    loop {
        // .w64_loop:
        tl = a_p1(tl, -1); //                                      dec tlq
        m[3] = vpbroadcastb_p1(tl); //                             vpbroadcastb m3, [tlq]
        paeth(m, 6, 8); //                                         PAETH 6, 8
        movy_st_p1(dst, m[0]); //                                  mova [dstq+32*0], m0
        paeth(m, 7, 9); //                                         PAETH 7, 9
        movy_st_p1(at(dst, 32), m[0]); //                          mova [dstq+32*1], m0
        dst = at(dst, stride); //                                  add dstq, strideq
        h -= 1; //                                                 dec hd
        if h <= 0 {
            break; //                                              jg .w64_loop
        }
    }
    // RET
}

/// `ipred_paeth_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_paeth_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    let tl = topleft;
    //                                                             lea r5, [ipred_paeth_avx2_table]
    let wl = tzcnt_p1(width); //                                   tzcnt wd, wm
    m[5] = vpbroadcastb_p1(tl); //                                 vpbroadcastb m5, [tlq] ; topleft
    let h = height; //                                             movifnidn hd, hm
                    //                                                             movsxd wq, [r5+wq*4]
    m[4] = vpbroadcastd_p1(ro(ipred_h_shuf, 12)); //               vpbroadcastd m4, [base+pb_1]
                                                  //                                                             add wq, r5
    match wl {
        //                                                         jmp wq
        2 => ipred_paeth_8bpc_w4(m, dst, stride, tl, h),
        3 => ipred_paeth_8bpc_w8(m, dst, stride, tl, h),
        4 => ipred_paeth_8bpc_w16(m, dst, stride, tl, h),
        5 => ipred_paeth_8bpc_w32(m, dst, stride, tl, h),
        6 => ipred_paeth_8bpc_w64(m, dst, stride, tl, h),
        _ => unreachable!(), // outside the jump table (contract: 2..=6)
    }
}

// ---- pal_pred_8bpc ----
// `m4` holds the eight palette entries twice; each index byte holds two 4-bit indices (low nibble first).

/// `pal_pred_8bpc.w4`.
#[inline(always)]
unsafe fn pal_pred_8bpc_w4(m: &mut M, mut dst: *mut u8, stride: isize, r2: isize, mut idx: *const u8, mut h: c_int) {
    loop {
        m[0] = zx(movq_p1(idx)); //                                movq xm0, [idxq]
        idx = a_p1(idx, 8); //                                     add idxq, 8
        m[1] = zx(_mm_srli_epi16::<4>(lo(m[0]))); //               psrlw xm1, xm0, 4
        m[0] = zx(_mm_unpacklo_epi8(lo(m[0]), lo(m[1]))); //       punpcklbw xm0, xm1
        m[0] = zx(_mm_shuffle_epi8(lo(m[4]), lo(m[0]))); //        pshufb xm0, xm4, xm0
        movd_st_p1(dst, m[0]); //                                  movd   [dstq+strideq*0], xm0
        pextrd_st_p1::<1>(at(dst, stride), m[0]); //               pextrd [dstq+strideq*1], xm0, 1
        pextrd_st_p1::<2>(at(dst, stride * 2), m[0]); //           pextrd [dstq+strideq*2], xm0, 2
        pextrd_st_p1::<3>(at(dst, r2), m[0]); //                   pextrd [dstq+r2       ], xm0, 3
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        h -= 4; //                                                 sub hd, 4
        if h <= 0 {
            break; //                                              jg .w4
        }
    }
    // RET
}

/// `pal_pred_8bpc.w8`.
#[inline(always)]
unsafe fn pal_pred_8bpc_w8(m: &mut M, mut dst: *mut u8, stride: isize, r2: isize, mut idx: *const u8, mut h: c_int) {
    loop {
        m[2] = zx(x_p1(idx)); //                                   movu xm2, [idxq]
        idx = a_p1(idx, 16); //                                    add idxq, 16
        m[1] = zx(_mm_shuffle_epi8(lo(m[4]), lo(m[2]))); //        pshufb xm1, xm4, xm2
        m[2] = zx(_mm_srli_epi16::<4>(lo(m[2]))); //               psrlw xm2, 4
        m[2] = zx(_mm_shuffle_epi8(lo(m[4]), lo(m[2]))); //        pshufb xm2, xm4, xm2
        m[0] = zx(_mm_unpacklo_epi8(lo(m[1]), lo(m[2]))); //       punpcklbw xm0, xm1, xm2
        m[1] = zx(_mm_unpackhi_epi8(lo(m[1]), lo(m[2]))); //       punpckhbw xm1, xm2
        movq_st_p1(dst, m[0]); //                                  movq   [dstq+strideq*0], xm0
        movhps_st_p1(at(dst, stride), m[0]); //                    movhps [dstq+strideq*1], xm0
        movq_st_p1(at(dst, stride * 2), m[1]); //                  movq   [dstq+strideq*2], xm1
        movhps_st_p1(at(dst, r2), m[1]); //                        movhps [dstq+r2       ], xm1
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        h -= 4; //                                                 sub hd, 4
        if h <= 0 {
            break; //                                              jg .w8
        }
    }
    // RET
}

/// `pal_pred_8bpc.w16`.
#[inline(always)]
unsafe fn pal_pred_8bpc_w16(m: &mut M, mut dst: *mut u8, stride: isize, r2: isize, mut idx: *const u8, mut h: c_int) {
    loop {
        m[2] = y_p1(idx); //                                       movu m2, [idxq]
        idx = a_p1(idx, 32); //                                    add idxq, 32
        m[1] = _mm256_shuffle_epi8(m[4], m[2]); //                 pshufb m1, m4, m2
        m[2] = _mm256_srli_epi16::<4>(m[2]); //                    psrlw m2, 4
        m[2] = _mm256_shuffle_epi8(m[4], m[2]); //                 pshufb m2, m4, m2
        m[0] = _mm256_unpacklo_epi8(m[1], m[2]); //                punpcklbw m0, m1, m2
        m[1] = _mm256_unpackhi_epi8(m[1], m[2]); //                punpckhbw m1, m2
        movx_st_p1(dst, m[0]); //                                  mova [dstq+strideq*0], xm0
        movx_st_p1(at(dst, stride), m[1]); //                      mova [dstq+strideq*1], xm1
        vextracti128_st_p1(at(dst, stride * 2), m[0]); //          vextracti128 [dstq+strideq*2], m0, 1
        vextracti128_st_p1(at(dst, r2), m[1]); //                  vextracti128 [dstq+r2       ], m1, 1
        dst = at(dst, stride * 4); //                              lea dstq, [dstq+strideq*4]
        h -= 4; //                                                 sub hd, 4
        if h <= 0 {
            break; //                                              jg .w16
        }
    }
    // RET
}

/// `q3120` (x86inc's shuffle immediate `qABCD` = `A<<6|B<<4|C<<2|D`).
const Q3120_P1: i32 = 3 << 6 | 1 << 4 | 2 << 2 | 0;

/// `pal_pred_8bpc.w32`.
#[inline(always)]
unsafe fn pal_pred_8bpc_w32(m: &mut M, mut dst: *mut u8, stride: isize, mut idx: *const u8, mut h: c_int) {
    loop {
        m[2] = _mm256_permute4x64_epi64::<Q3120_P1>(y_p1(idx)); // vpermq m2, [idxq], q3120
        idx = a_p1(idx, 32); //                                    add idxq, 32
        m[1] = _mm256_shuffle_epi8(m[4], m[2]); //                 pshufb m1, m4, m2
        m[2] = _mm256_srli_epi16::<4>(m[2]); //                    psrlw m2, 4
        m[2] = _mm256_shuffle_epi8(m[4], m[2]); //                 pshufb m2, m4, m2
        m[0] = _mm256_unpacklo_epi8(m[1], m[2]); //                punpcklbw m0, m1, m2
        m[1] = _mm256_unpackhi_epi8(m[1], m[2]); //                punpckhbw m1, m2
        movy_st_p1(dst, m[0]); //                                  mova [dstq+strideq*0], m0
        movy_st_p1(at(dst, stride), m[1]); //                      mova [dstq+strideq*1], m1
        dst = at(dst, stride * 2); //                              lea dstq, [dstq+strideq*2]
        h -= 2; //                                                 sub hd, 2
        if h <= 0 {
            break; //                                              jg .w32
        }
    }
    // RET
}

/// `pal_pred_8bpc.w64`.
#[inline(always)]
unsafe fn pal_pred_8bpc_w64(m: &mut M, mut dst: *mut u8, stride: isize, mut idx: *const u8, mut h: c_int) {
    loop {
        m[2] = _mm256_permute4x64_epi64::<Q3120_P1>(y_p1(idx)); // vpermq m2, [idxq], q3120
        idx = a_p1(idx, 32); //                                    add idxq, 32
        m[1] = _mm256_shuffle_epi8(m[4], m[2]); //                 pshufb m1, m4, m2
        m[2] = _mm256_srli_epi16::<4>(m[2]); //                    psrlw m2, 4
        m[2] = _mm256_shuffle_epi8(m[4], m[2]); //                 pshufb m2, m4, m2
        m[0] = _mm256_unpacklo_epi8(m[1], m[2]); //                punpcklbw m0, m1, m2
        m[1] = _mm256_unpackhi_epi8(m[1], m[2]); //                punpckhbw m1, m2
        movy_st_p1(dst, m[0]); //                                  mova [dstq+32*0], m0
        movy_st_p1(at(dst, 32), m[1]); //                          mova [dstq+32*1], m1
        dst = at(dst, stride); //                                  add dstq, strideq
        h -= 1; //                                                 dec hd
        if h <= 0 {
            break; //                                              jg .w64
        }
    }
    // RET
}

/// `pal_pred_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_pal_pred_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    pal: *const u8,
    idx: *const u8,
    w: c_int,
    h: c_int,
) {
    let mut m = regs();
    let m = &mut m;
    m[4] = vpbroadcastq_p1(pal); //                                vpbroadcastq m4, [palq]
                                 //                                                             lea r2, [pal_pred_avx2_table]
    let wl = tzcnt_p1(w); //                                       tzcnt wd, wm
                          //                                                             movifnidn hd, hm; movsxd wq, [r2+wq*4]; add wq, r2
    let r2 = stride * 3; //                                        lea r2, [strideq*3]
    match wl {
        //                                                         jmp wq
        2 => pal_pred_8bpc_w4(m, dst, stride, r2, idx, h),
        3 => pal_pred_8bpc_w8(m, dst, stride, r2, idx, h),
        4 => pal_pred_8bpc_w16(m, dst, stride, r2, idx, h),
        5 => pal_pred_8bpc_w32(m, dst, stride, idx, h),
        6 => pal_pred_8bpc_w64(m, dst, stride, idx, h),
        _ => unreachable!(), // outside the jump table (contract: 2..=6)
    }
}

// ---- part 2: `ipred_avx2.asm` lines 669–1309 (SMOOTH, smooth_v, smooth_h, SMOOTH_2D_END, smooth) ----
//
// Each `cglobal` is the `extern "C"` function of the same name; its jump table (`ipred_smooth*_avx2_table`, indexed
// by `tzcnt w`) is a `match` that calls the width path `.w4` … `.w64`, one `#[inline(always)]` function each
// (`ipred_smooth_v_8bpc_w4`, …). `.prep` (smooth_h) and `.prep_v` (smooth), which the assembly `call`s, are
// `ipred_smooth_h_8bpc_prep` and `ipred_smooth_8bpc_prep_v`; their `[rsp+gprsize+32*n]` is the caller's
// `ALLOC_STACK` frame at `32*n` (a local, written whole by the prep function before any load). The macros `SMOOTH`
// and `SMOOTH_2D_END` are `smooth` and `smooth_2d_end` (register arguments as register numbers; the two addends of
// `SMOOTH_2D_END`, a register or a memory operand, are passed as values).

/// `vpbroadcastb m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastb_p2(p: *const u8) -> __m256i {
    _mm256_set1_epi8(p.read() as i8)
}

/// `vpbroadcastw m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastw_p2(p: *const u8) -> __m256i {
    _mm256_set1_epi16((p as *const i16).read_unaligned())
}

/// `vpbroadcastd m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastd_p2(p: *const u8) -> __m256i {
    _mm256_set1_epi32((p as *const i32).read_unaligned())
}

/// `vpbroadcastd xm, [p]` (the upper half is zero).
#[inline(always)]
unsafe fn vpbroadcastd_x_p2(p: *const u8) -> __m256i {
    zx(_mm_set1_epi32((p as *const i32).read_unaligned()))
}

/// `vpbroadcastq m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastq_p2(p: *const u8) -> __m256i {
    _mm256_set1_epi64x((p as *const i64).read_unaligned())
}

/// `vbroadcasti128 m, [p]`.
#[inline(always)]
unsafe fn vbroadcasti128_p2(p: *const u8) -> __m256i {
    _mm256_broadcastsi128_si256(_mm_loadu_si128(p as *const __m128i))
}

/// `mova`/`movu m, [p]` (32 bytes).
#[inline(always)]
unsafe fn movu_p2(p: *const u8) -> __m256i {
    _mm256_loadu_si256(p as *const __m256i)
}

/// `mova xm, [p]` (the upper half is zero).
#[inline(always)]
unsafe fn movx_p2(p: *const u8) -> __m256i {
    zx(_mm_loadu_si128(p as *const __m128i))
}

/// `vinserti128 m, [p], 1`.
#[inline(always)]
unsafe fn vinserti128_p2(a: __m256i, p: *const u8) -> __m256i {
    _mm256_inserti128_si256::<1>(a, _mm_loadu_si128(p as *const __m128i))
}

/// `vpermq m, [p], q3120`.
#[inline(always)]
unsafe fn vpermq_q3120_p2(p: *const u8) -> __m256i {
    _mm256_permute4x64_epi64::<0xd8>(_mm256_loadu_si256(p as *const __m256i))
}

/// `movd [p], xm`.
#[inline(always)]
unsafe fn movd_st_p2(p: *mut u8, x: __m128i) {
    (p as *mut i32).write_unaligned(_mm_cvtsi128_si32(x))
}

/// `pextrd [p], xm, I`.
#[inline(always)]
unsafe fn pextrd_st_p2<const I: i32>(p: *mut u8, x: __m128i) {
    (p as *mut i32).write_unaligned(_mm_extract_epi32::<I>(x))
}

/// `movq [p], xm`.
#[inline(always)]
unsafe fn movq_st_p2(p: *mut u8, x: __m128i) {
    _mm_storel_epi64(p as *mut __m128i, x)
}

/// `movhps [p], xm`: the high 8 bytes, stored as one instruction (`_mm_storeh_pd` on the integer value compiled
/// to `vpshufd` + `vmovq`).
#[inline(always)]
unsafe fn movhps_st_p2(p: *mut u8, x: __m128i) {
    (p as *mut i64).write_unaligned(_mm_extract_epi64::<1>(x))
}

/// `mova [p], xm`.
#[inline(always)]
unsafe fn movx_st_p2(p: *mut u8, x: __m128i) {
    _mm_storeu_si128(p as *mut __m128i, x)
}

/// `vextracti128 [p], m, 1`.
#[inline(always)]
unsafe fn vextracti128_st_p2(p: *mut u8, x: __m256i) {
    _mm_storeu_si128(p as *mut __m128i, _mm256_extracti128_si256::<1>(x))
}

/// `mova [p], m`.
#[inline(always)]
unsafe fn movu_st_p2(p: *mut u8, x: __m256i) {
    _mm256_storeu_si256(p as *mut __m256i, x)
}

/// `p + off` bytes for a read-only pointer, wrapping.
#[inline(always)]
fn atc_p2(p: *const u8, off: isize) -> *const u8 {
    p.wrapping_offset(off)
}

/// `[base+label+off]`: `ro(label, off)` relative to `base`, the address of `RODATA`.
#[inline(always)]
fn rob_p2(base: *const u8, label: usize, off: isize) -> *const u8 {
    base.wrapping_add(label).wrapping_offset(off)
}

/// `SMOOTH src1, src2, mul1, mul2, add1, add2`.
#[inline(always)]
unsafe fn smooth(m: &mut M, s1: usize, s2: usize, mu1: usize, mu2: usize, a1: usize, a2: usize) {
    // w * a         = (w - 128) * a + 128 * a
    // (256 - w) * b = (127 - w) * b + 129 * b
    m[0] = _mm256_maddubs_epi16(m[mu1], m[s1]); // pmaddubsw m0, m%3, m%1
    m[1] = _mm256_maddubs_epi16(m[mu2], m[s2]); // pmaddubsw m1, m%4, m%2
    m[0] = _mm256_add_epi16(m[0], m[a1]); // paddw m0, m%5
    m[1] = _mm256_add_epi16(m[1], m[a2]); // paddw m1, m%6
    m[0] = _mm256_srli_epi16::<8>(m[0]); // psrlw m0, 8
    m[1] = _mm256_srli_epi16::<8>(m[1]); // psrlw m1, 8
    m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
}

/// `SMOOTH_2D_END src1, src2, mul1, mul2, add1, add2`. `add1`/`add2` (`%5`/`%6`, a register `m%5` or a memory
/// operand) are passed as values: no call site names `m0`/`m1`, the only registers the macro writes before it
/// reads them.
#[inline(always)]
unsafe fn smooth_2d_end(m: &mut M, s1: usize, s2: usize, mu1: usize, mu2: usize, a1: __m256i, a2: __m256i) {
    m[0] = _mm256_maddubs_epi16(m[mu1], m[s1]); // pmaddubsw m0, m%3, m%1
    m[1] = _mm256_maddubs_epi16(m[mu2], m[s2]); // pmaddubsw m1, m%4, m%2
    m[0] = _mm256_add_epi16(m[0], a1); // paddw m0, %5
    m[1] = _mm256_add_epi16(m[1], a2); // paddw m1, %6
    m[0] = _mm256_avg_epu16(m[0], m[2]); // pavgw m0, m2
    m[1] = _mm256_avg_epu16(m[1], m[3]); // pavgw m1, m3
    m[0] = _mm256_srli_epi16::<8>(m[0]); // psrlw m0, 8
    m[1] = _mm256_srli_epi16::<8>(m[1]); // psrlw m1, 8
    m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
}

// ---------------------------------------------------------------- smooth_v

/// `cglobal ipred_smooth_v_8bpc, 3, 7, 0, dst, stride, tl, w, h, weights`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_smooth_v_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let m = &mut regs();
    let base = RODATA.0.as_ptr(); // lea r6, [ipred_smooth_v_avx2_table] (`base`: the address of RODATA)
    let tl = topleft as *mut u8;
    // tzcnt wd, wm (w is a power of two from 4 to 64, never zero)
    let w = (width as u32).trailing_zeros();
    let mut h = height as isize; // mov hd, hm
    m[0] = vpbroadcastd_p2(rob_p2(base, pb_127_m127, 0)); // vpbroadcastd m0, [base+pb_127_m127]
    m[1] = vpbroadcastd_p2(rob_p2(base, pw_128, 0)); // vpbroadcastd m1, [base+pw_128]
    let weights = rob_p2(base, smooth_weights, h * 4); // lea weightsq, [base+smooth_weights+hq*4]
    h = -h; // neg hq
    m[5] = vpbroadcastb_p2(at(tl, h)); // vpbroadcastb m5, [tlq+hq] ; bottom
                                       // movsxd wq, [r6+wq*4] / add wq, r6 / jmp wq
    match w {
        2 => ipred_smooth_v_8bpc_w4(m, base, dst, stride, tl, h, weights),
        3 => ipred_smooth_v_8bpc_w8(m, base, dst, stride, tl, h, weights),
        4 => ipred_smooth_v_8bpc_w16(m, base, dst, stride, tl, h, weights),
        5 => ipred_smooth_v_8bpc_w32(m, base, dst, stride, tl, h, weights),
        _ => ipred_smooth_v_8bpc_w64(m, base, dst, stride, tl, h, weights),
    }
}

/// `.w4` of `ipred_smooth_v_8bpc` (`h` is `hq`, negative).
#[inline(always)]
unsafe fn ipred_smooth_v_8bpc_w4(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    tl: *mut u8,
    mut h: isize,
    weights: *const u8,
) {
    m[2] = vpbroadcastd_p2(at(tl, 1)); // vpbroadcastd m2, [tlq+1]
    m[2] = _mm256_unpacklo_epi8(m[2], m[5]); // punpcklbw m2, m5 ; top, bottom
    m[5] = movu_p2(rob_p2(base, ipred_v_shuf, 0)); // mova m5, [base+ipred_v_shuf]
    let r3 = stride * 3; // lea r3, [strideq*3]
    m[4] = _mm256_unpacklo_epi32(m[5], m[5]); // punpckldq m4, m5, m5
    m[5] = _mm256_unpackhi_epi32(m[5], m[5]); // punpckhdq m5, m5
    m[3] = _mm256_maddubs_epi16(m[2], m[0]); // pmaddubsw m3, m2, m0
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2 ;   1 * top + 256 * bottom + 128, overflow is ok
    m[3] = _mm256_add_epi16(m[3], m[1]); // paddw m3, m1 ; 128 * top + 129 * bottom + 128
    loop {
        // .w4_loop:
        m[1] = vbroadcasti128_p2(atc_p2(weights, h * 2)); // vbroadcasti128 m1, [weightsq+hq*2]
        m[0] = _mm256_shuffle_epi8(m[1], m[4]); // pshufb m0, m1, m4
        m[1] = _mm256_shuffle_epi8(m[1], m[5]); // pshufb m1, m5
        smooth(m, 0, 1, 2, 2, 3, 3); // SMOOTH 0, 1, 2, 2, 3, 3
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        movd_st_p2(at(dst, 0), lo(m[0])); // movd [dstq+strideq*0], xm0
        movd_st_p2(at(dst, stride), lo(m[1])); // movd [dstq+strideq*1], xm1
        pextrd_st_p2::<1>(at(dst, stride * 2), lo(m[0])); // pextrd [dstq+strideq*2], xm0, 1
        pextrd_st_p2::<1>(at(dst, r3), lo(m[1])); // pextrd [dstq+r3       ], xm1, 1
        if h as i32 == -4 {
            return; // cmp hd, -4 / je .ret
        }
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        pextrd_st_p2::<2>(at(dst, 0), lo(m[0])); // pextrd [dstq+strideq*0], xm0, 2
        pextrd_st_p2::<2>(at(dst, stride), lo(m[1])); // pextrd [dstq+strideq*1], xm1, 2
        pextrd_st_p2::<3>(at(dst, stride * 2), lo(m[0])); // pextrd [dstq+strideq*2], xm0, 3
        pextrd_st_p2::<3>(at(dst, r3), lo(m[1])); // pextrd [dstq+r3       ], xm1, 3
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h += 8; // add hq, 8
        if h >= 0 {
            break; // jl .w4_loop
        }
    }
}

/// `.w8` of `ipred_smooth_v_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_v_8bpc_w8(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    tl: *mut u8,
    mut h: isize,
    weights: *const u8,
) {
    m[2] = vpbroadcastq_p2(at(tl, 1)); // vpbroadcastq m2, [tlq+1]
    m[2] = _mm256_unpacklo_epi8(m[2], m[5]); // punpcklbw m2, m5
    m[5] = movu_p2(rob_p2(base, ipred_v_shuf, 0)); // mova m5, [base+ipred_v_shuf]
    let r3 = stride * 3; // lea r3, [strideq*3]
    m[4] = _mm256_shuffle_epi32::<0x00>(m[5]); // pshufd m4, m5, q0000
    m[5] = _mm256_shuffle_epi32::<0x55>(m[5]); // pshufd m5, m5, q1111
    m[3] = _mm256_maddubs_epi16(m[2], m[0]); // pmaddubsw m3, m2, m0
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[3] = _mm256_add_epi16(m[3], m[1]); // paddw m3, m1
    loop {
        // .w8_loop:
        m[1] = vpbroadcastq_p2(atc_p2(weights, h * 2)); // vpbroadcastq m1, [weightsq+hq*2]
        m[0] = _mm256_shuffle_epi8(m[1], m[4]); // pshufb m0, m1, m4
        m[1] = _mm256_shuffle_epi8(m[1], m[5]); // pshufb m1, m5
        smooth(m, 0, 1, 2, 2, 3, 3); // SMOOTH 0, 1, 2, 2, 3, 3
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        movq_st_p2(at(dst, 0), lo(m[0])); // movq [dstq+strideq*0], xm0
        movq_st_p2(at(dst, stride), lo(m[1])); // movq [dstq+strideq*1], xm1
        movhps_st_p2(at(dst, stride * 2), lo(m[0])); // movhps [dstq+strideq*2], xm0
        movhps_st_p2(at(dst, r3), lo(m[1])); // movhps [dstq+r3       ], xm1
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h += 4; // add hq, 4
        if h >= 0 {
            break; // jl .w8_loop
        }
    }
}

/// `.w16` of `ipred_smooth_v_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_v_8bpc_w16(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    tl: *mut u8,
    mut h: isize,
    weights: *const u8,
) {
    // WIN64_SPILL_XMM 7
    m[3] = vbroadcasti128_p2(at(tl, 1)); // vbroadcasti128 m3, [tlq+1]
    m[6] = movu_p2(rob_p2(base, ipred_v_shuf, 0)); // mova m6, [base+ipred_v_shuf]
    m[2] = _mm256_unpacklo_epi8(m[3], m[5]); // punpcklbw m2, m3, m5
    m[3] = _mm256_unpackhi_epi8(m[3], m[5]); // punpckhbw m3, m5
    m[4] = _mm256_maddubs_epi16(m[2], m[0]); // pmaddubsw m4, m2, m0
    m[5] = _mm256_maddubs_epi16(m[3], m[0]); // pmaddubsw m5, m3, m0
    m[0] = _mm256_add_epi16(m[1], m[2]); // paddw m0, m1, m2
    m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3
    m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
    m[5] = _mm256_add_epi16(m[5], m[1]); // paddw m5, m1
    loop {
        // .w16_loop:
        m[1] = vpbroadcastd_p2(atc_p2(weights, h * 2)); // vpbroadcastd m1, [weightsq+hq*2]
        m[1] = _mm256_shuffle_epi8(m[1], m[6]); // pshufb m1, m6
        smooth(m, 1, 1, 2, 3, 4, 5); // SMOOTH 1, 1, 2, 3, 4, 5
        movx_st_p2(at(dst, 0), lo(m[0])); // mova [dstq+strideq*0], xm0
        vextracti128_st_p2(at(dst, stride), m[0]); // vextracti128 [dstq+strideq*1], m0, 1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h += 2; // add hq, 2
        if h >= 0 {
            break; // jl .w16_loop
        }
    }
}

/// `.w32` of `ipred_smooth_v_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_v_8bpc_w32(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    tl: *mut u8,
    mut h: isize,
    weights: *const u8,
) {
    // WIN64_SPILL_XMM 6
    m[3] = movu_p2(at(tl, 1)); // movu m3, [tlq+1]
    m[2] = _mm256_unpacklo_epi8(m[3], m[5]); // punpcklbw m2, m3, m5
    m[3] = _mm256_unpackhi_epi8(m[3], m[5]); // punpckhbw m3, m5
    m[4] = _mm256_maddubs_epi16(m[2], m[0]); // pmaddubsw m4, m2, m0
    m[5] = _mm256_maddubs_epi16(m[3], m[0]); // pmaddubsw m5, m3, m0
    m[0] = _mm256_add_epi16(m[1], m[2]); // paddw m0, m1, m2
    m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3
    m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
    m[5] = _mm256_add_epi16(m[5], m[1]); // paddw m5, m1
    loop {
        // .w32_loop:
        m[1] = vpbroadcastw_p2(atc_p2(weights, h * 2)); // vpbroadcastw m1, [weightsq+hq*2]
        smooth(m, 1, 1, 2, 3, 4, 5); // SMOOTH 1, 1, 2, 3, 4, 5
        movu_st_p2(dst, m[0]); // mova [dstq], m0
        dst = at(dst, stride); // add dstq, strideq
        h += 1; // inc hq
        if h >= 0 {
            break; // jl .w32_loop
        }
    }
}

/// `.w64` of `ipred_smooth_v_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_v_8bpc_w64(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    tl: *mut u8,
    mut h: isize,
    weights: *const u8,
) {
    // WIN64_SPILL_XMM 11
    m[4] = movu_p2(at(tl, 1)); // movu m4, [tlq+ 1]
    m[8] = movu_p2(at(tl, 33)); // movu m8, [tlq+33]
    m[3] = _mm256_unpacklo_epi8(m[4], m[5]); // punpcklbw m3, m4, m5
    m[4] = _mm256_unpackhi_epi8(m[4], m[5]); // punpckhbw m4, m5
    m[7] = _mm256_unpacklo_epi8(m[8], m[5]); // punpcklbw m7, m8, m5
    m[8] = _mm256_unpackhi_epi8(m[8], m[5]); // punpckhbw m8, m5
    m[5] = _mm256_maddubs_epi16(m[3], m[0]); // pmaddubsw m5, m3, m0
    m[6] = _mm256_maddubs_epi16(m[4], m[0]); // pmaddubsw m6, m4, m0
    m[9] = _mm256_maddubs_epi16(m[7], m[0]); // pmaddubsw m9, m7, m0
    m[10] = _mm256_maddubs_epi16(m[8], m[0]); // pmaddubsw m10, m8, m0
    m[2] = _mm256_add_epi16(m[1], m[3]); // paddw m2, m1, m3
    m[5] = _mm256_add_epi16(m[5], m[2]); // paddw m5, m2
    m[2] = _mm256_add_epi16(m[1], m[4]); // paddw m2, m1, m4
    m[6] = _mm256_add_epi16(m[6], m[2]); // paddw m6, m2
    m[0] = _mm256_add_epi16(m[1], m[7]); // paddw m0, m1, m7
    m[9] = _mm256_add_epi16(m[9], m[0]); // paddw m9, m0
    m[1] = _mm256_add_epi16(m[1], m[8]); // paddw m1, m8
    m[10] = _mm256_add_epi16(m[10], m[1]); // paddw m10, m1
    loop {
        // .w64_loop:
        m[2] = vpbroadcastw_p2(atc_p2(weights, h * 2)); // vpbroadcastw m2, [weightsq+hq*2]
        smooth(m, 2, 2, 3, 4, 5, 6); // SMOOTH 2, 2, 3, 4, 5, 6
        movu_st_p2(at(dst, 32 * 0), m[0]); // mova [dstq+32*0], m0
        smooth(m, 2, 2, 7, 8, 9, 10); // SMOOTH 2, 2, 7, 8, 9, 10
        movu_st_p2(at(dst, 32 * 1), m[0]); // mova [dstq+32*1], m0
        dst = at(dst, stride); // add dstq, strideq
        h += 1; // inc hq
        if h >= 0 {
            break; // jl .w64_loop
        }
    }
}

// ---------------------------------------------------------------- smooth_h

/// `cglobal ipred_smooth_h_8bpc, 3, 7, 0, dst, stride, tl, w, h`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_smooth_h_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    // Not in the original: from 16 columns on the kernel loads `topleft[-64..-1]` whatever the height, before the
    // array of an inter-intra edge (`edge + 32`); those calls run on a copy of the pixels the C kernel reads,
    // `topleft[-height..=width]`, with zeros around them (`copy_edge`).
    let mut edge: [u8; 257];
    let topleft = if width >= 16 {
        edge = [0; 257];
        crate::simd::copy_edge(&mut edge, 128, topleft, -(height as isize), width as isize)
    } else {
        topleft
    };
    let m = &mut regs();
    let base = RODATA.0.as_ptr(); // lea r5, [ipred_smooth_h_avx2_table] (`base`: the address of RODATA)
    let tl = topleft as *mut u8;
    let w = width as isize; // mov wd, wm
    m[3] = vpbroadcastb_p2(at(tl, w)); // vpbroadcastb m3, [tlq+wq] ; right
                                       // tzcnt wd, wd (w is a power of two from 4 to 64, never zero)
    let w = (w as u32).trailing_zeros();
    let h = height as isize; // mov hd, hm
    m[4] = vpbroadcastd_p2(rob_p2(base, pb_127_m127, 0)); // vpbroadcastd m4, [base+pb_127_m127]
    m[5] = vpbroadcastd_p2(rob_p2(base, pw_128, 0)); // vpbroadcastd m5, [base+pw_128]
                                                     // movsxd wq, [r5+wq*4] / add wq, r5 / jmp wq
    match w {
        2 => ipred_smooth_h_8bpc_w4(m, base, dst, stride, tl, h),
        3 => ipred_smooth_h_8bpc_w8(m, base, dst, stride, tl, h),
        4 => ipred_smooth_h_8bpc_w16(m, base, dst, stride, tl, h),
        5 => ipred_smooth_h_8bpc_w32(m, base, dst, stride, tl, h),
        _ => ipred_smooth_h_8bpc_w64(m, base, dst, stride, tl, h),
    }
}

/// `.w4` of `ipred_smooth_h_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_h_8bpc_w4(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *mut u8,
    mut h: isize,
) {
    // WIN64_SPILL_XMM 8
    m[6] = vpbroadcastq_p2(rob_p2(base, smooth_weights, 4 * 2)); // vpbroadcastq m6, [base+smooth_weights+4*2]
    m[7] = movu_p2(rob_p2(base, ipred_h_shuf, 0)); // mova m7, [base+ipred_h_shuf]
    tl = at(tl, -8); // sub tlq, 8
    tl = at(tl, -h); // sub tlq, hq
    let r3 = stride * 3; // lea r3, [strideq*3]
    loop {
        // .w4_loop:
        m[2] = vpbroadcastq_p2(at(tl, h)); // vpbroadcastq m2, [tlq+hq]
        m[2] = _mm256_shuffle_epi8(m[2], m[7]); // pshufb m2, m7
        m[1] = _mm256_unpacklo_epi8(m[2], m[3]); // punpcklbw m1, m2, m3 ; left, right
        m[2] = _mm256_unpackhi_epi8(m[2], m[3]); // punpckhbw m2, m3
        m[0] = _mm256_maddubs_epi16(m[1], m[4]); // pmaddubsw m0, m1, m4 ; 127 * left - 127 * right
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1     ; 128 * left + 129 * right
        m[1] = _mm256_maddubs_epi16(m[1], m[6]); // pmaddubsw m1, m6
        m[1] = _mm256_add_epi16(m[1], m[5]); // paddw m1, m5
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[1] = _mm256_maddubs_epi16(m[2], m[4]); // pmaddubsw m1, m2, m4
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        m[2] = _mm256_maddubs_epi16(m[2], m[6]); // pmaddubsw m2, m6
        m[2] = _mm256_add_epi16(m[2], m[5]); // paddw m2, m5
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        m[0] = _mm256_srli_epi16::<8>(m[0]); // psrlw m0, 8
        m[1] = _mm256_srli_epi16::<8>(m[1]); // psrlw m1, 8
        m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        movd_st_p2(at(dst, 0), lo(m[0])); // movd [dstq+strideq*0], xm0
        movd_st_p2(at(dst, stride), lo(m[1])); // movd [dstq+strideq*1], xm1
        pextrd_st_p2::<2>(at(dst, stride * 2), lo(m[0])); // pextrd [dstq+strideq*2], xm0, 2
        pextrd_st_p2::<2>(at(dst, r3), lo(m[1])); // pextrd [dstq+r3       ], xm1, 2
        if h as i32 == 4 {
            return; // cmp hd, 4 / je .ret
        }
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        pextrd_st_p2::<1>(at(dst, 0), lo(m[0])); // pextrd [dstq+strideq*0], xm0, 1
        pextrd_st_p2::<1>(at(dst, stride), lo(m[1])); // pextrd [dstq+strideq*1], xm1, 1
        pextrd_st_p2::<3>(at(dst, stride * 2), lo(m[0])); // pextrd [dstq+strideq*2], xm0, 3
        pextrd_st_p2::<3>(at(dst, r3), lo(m[1])); // pextrd [dstq+r3       ], xm1, 3
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h -= 8; // sub hd, 8
        if h <= 0 {
            break; // jg .w4_loop
        }
    }
}

/// `.w8` of `ipred_smooth_h_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_h_8bpc_w8(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *mut u8,
    mut h: isize,
) {
    // WIN64_SPILL_XMM 8
    m[6] = vbroadcasti128_p2(rob_p2(base, smooth_weights, 8 * 2)); // vbroadcasti128 m6, [base+smooth_weights+8*2]
    m[7] = movu_p2(rob_p2(base, ipred_h_shuf, 0)); // mova m7, [base+ipred_h_shuf]
    tl = at(tl, -4); // sub tlq, 4
    let r3 = stride * 3; // lea r3, [strideq*3]
    tl = at(tl, -h); // sub tlq, hq
    loop {
        // .w8_loop:
        m[2] = vpbroadcastd_p2(at(tl, h)); // vpbroadcastd m2, [tlq+hq]
        m[2] = _mm256_shuffle_epi8(m[2], m[7]); // pshufb m2, m7
        m[1] = _mm256_unpacklo_epi8(m[2], m[3]); // punpcklbw m1, m2, m3
        m[2] = _mm256_unpackhi_epi8(m[2], m[3]); // punpckhbw m2, m3
        m[0] = _mm256_maddubs_epi16(m[1], m[4]); // pmaddubsw m0, m1, m4
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[1] = _mm256_maddubs_epi16(m[1], m[6]); // pmaddubsw m1, m6
        m[1] = _mm256_add_epi16(m[1], m[5]); // paddw m1, m5
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[1] = _mm256_maddubs_epi16(m[2], m[4]); // pmaddubsw m1, m2, m4
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        m[2] = _mm256_maddubs_epi16(m[2], m[6]); // pmaddubsw m2, m6
        m[2] = _mm256_add_epi16(m[2], m[5]); // paddw m2, m5
        m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
        m[0] = _mm256_srli_epi16::<8>(m[0]); // psrlw m0, 8
        m[1] = _mm256_srli_epi16::<8>(m[1]); // psrlw m1, 8
        m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        movq_st_p2(at(dst, 0), lo(m[0])); // movq [dstq+strideq*0], xm0
        movq_st_p2(at(dst, stride), lo(m[1])); // movq [dstq+strideq*1], xm1
        movhps_st_p2(at(dst, stride * 2), lo(m[0])); // movhps [dstq+strideq*2], xm0
        movhps_st_p2(at(dst, r3), lo(m[1])); // movhps [dstq+r3       ], xm1
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w8_loop
        }
    }
}

/// `.w16` of `ipred_smooth_h_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_h_8bpc_w16(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *mut u8,
    mut h: isize,
) {
    // ALLOC_STACK 32*4, 8: the frame. `ipred_smooth_h_8bpc_prep` writes all of its 128 bytes (four 32-byte
    // stores) before the loop below reads any of it, so it needs no initialization.
    let mut frame = core::mem::MaybeUninit::<[__m256i; 4]>::uninit();
    let rsp = frame.as_mut_ptr() as *mut u8;
    let mut r3 = at(rsp, 64 * 2 - 4); // lea r3, [rsp+64*2-4]
    ipred_smooth_h_8bpc_prep(m, rsp, &mut r3, &mut tl, h); // call .prep
    tl = at(tl, -2); // sub tlq, 2
    m[6] = vpbroadcastd_x_p2(rob_p2(base, ipred_h_shuf, 12)); // vpbroadcastd xm6, [base+pb_1]
    m[7] = movx_p2(rob_p2(base, ipred_v_shuf, 16)); // mova xm7, [base+ipred_v_shuf+16]
    m[7] = vinserti128_p2(m[7], rob_p2(base, ipred_v_shuf, 0)); // vinserti128 m7, [base+ipred_v_shuf+ 0], 1
    m[4] = vbroadcasti128_p2(rob_p2(base, smooth_weights, 16 * 2)); // vbroadcasti128 m4, [base+smooth_weights+16*2]
    m[5] = vbroadcasti128_p2(rob_p2(base, smooth_weights, 16 * 3)); // vbroadcasti128 m5, [base+smooth_weights+16*3]
    loop {
        // .w16_loop:
        m[1] = vpbroadcastd_p2(at(tl, h)); // vpbroadcastd m1, [tlq+hq]
        m[2] = vpbroadcastd_p2(at(r3, h * 2)); // vpbroadcastd m2, [r3+hq*2]
        m[1] = _mm256_shuffle_epi8(m[1], m[6]); // pshufb m1, m6
        m[1] = _mm256_unpacklo_epi8(m[1], m[3]); // punpcklbw m1, m3
        m[2] = _mm256_shuffle_epi8(m[2], m[7]); // pshufb m2, m7
        smooth(m, 4, 5, 1, 1, 2, 2); // SMOOTH 4, 5, 1, 1, 2, 2
        movx_st_p2(at(dst, 0), lo(m[0])); // mova [dstq+strideq*0], xm0
        vextracti128_st_p2(at(dst, stride), m[0]); // vextracti128 [dstq+strideq*1], m0, 1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w16_loop
        }
    }
}

/// `.w32` of `ipred_smooth_h_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_h_8bpc_w32(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *mut u8,
    mut h: isize,
) {
    // ALLOC_STACK 32*4: the frame, written whole by `ipred_smooth_h_8bpc_prep` before any load (as in `.w16`).
    let mut frame = core::mem::MaybeUninit::<[__m256i; 4]>::uninit();
    let rsp = frame.as_mut_ptr() as *mut u8;
    let mut r3 = at(rsp, 64 * 2 - 2); // lea r3, [rsp+64*2-2]
    ipred_smooth_h_8bpc_prep(m, rsp, &mut r3, &mut tl, h); // call .prep
    tl = at(tl, -1); // dec tlq
    m[4] = movx_p2(rob_p2(base, smooth_weights, 16 * 4)); // mova xm4, [base+smooth_weights+16*4]
    m[4] = vinserti128_p2(m[4], rob_p2(base, smooth_weights, 16 * 6)); // vinserti128 m4, [base+smooth_weights+16*6], 1
    m[5] = movx_p2(rob_p2(base, smooth_weights, 16 * 5)); // mova xm5, [base+smooth_weights+16*5]
    m[5] = vinserti128_p2(m[5], rob_p2(base, smooth_weights, 16 * 7)); // vinserti128 m5, [base+smooth_weights+16*7], 1
    loop {
        // .w32_loop:
        m[1] = vpbroadcastb_p2(at(tl, h)); // vpbroadcastb m1, [tlq+hq]
        m[1] = _mm256_unpacklo_epi8(m[1], m[3]); // punpcklbw m1, m3
        m[2] = vpbroadcastw_p2(at(r3, h * 2)); // vpbroadcastw m2, [r3+hq*2]
        smooth(m, 4, 5, 1, 1, 2, 2); // SMOOTH 4, 5, 1, 1, 2, 2
        movu_st_p2(dst, m[0]); // mova [dstq], m0
        dst = at(dst, stride); // add dstq, strideq
        h -= 1; // dec hd
        if h <= 0 {
            break; // jg .w32_loop
        }
    }
}

/// `.w64` of `ipred_smooth_h_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_h_8bpc_w64(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *mut u8,
    mut h: isize,
) {
    // ALLOC_STACK 32*4, 9: the frame, written whole by `ipred_smooth_h_8bpc_prep` before any load (as in `.w16`).
    let mut frame = core::mem::MaybeUninit::<[__m256i; 4]>::uninit();
    let rsp = frame.as_mut_ptr() as *mut u8;
    let mut r3 = at(rsp, 64 * 2 - 2); // lea r3, [rsp+64*2-2]
    ipred_smooth_h_8bpc_prep(m, rsp, &mut r3, &mut tl, h); // call .prep
                                                           // add r5, smooth_weights+16*15-ipred_smooth_h_avx2_table
    let r5 = rob_p2(base, smooth_weights, 16 * 15);
    tl = at(tl, -1); // dec tlq
    m[5] = movx_p2(atc_p2(r5, -16 * 7)); // mova xm5, [r5-16*7]
    m[5] = vinserti128_p2(m[5], atc_p2(r5, -16 * 5)); // vinserti128 m5, [r5-16*5], 1
    m[6] = movx_p2(atc_p2(r5, -16 * 6)); // mova xm6, [r5-16*6]
    m[6] = vinserti128_p2(m[6], atc_p2(r5, -16 * 4)); // vinserti128 m6, [r5-16*4], 1
    m[7] = movx_p2(atc_p2(r5, -16 * 3)); // mova xm7, [r5-16*3]
    m[7] = vinserti128_p2(m[7], atc_p2(r5, -16 * 1)); // vinserti128 m7, [r5-16*1], 1
    m[8] = movx_p2(atc_p2(r5, -16 * 2)); // mova xm8, [r5-16*2]
    m[8] = vinserti128_p2(m[8], atc_p2(r5, -16 * 0)); // vinserti128 m8, [r5-16*0], 1
    loop {
        // .w64_loop:
        m[2] = vpbroadcastb_p2(at(tl, h)); // vpbroadcastb m2, [tlq+hq]
        m[2] = _mm256_unpacklo_epi8(m[2], m[3]); // punpcklbw m2, m3
        m[4] = vpbroadcastw_p2(at(r3, h * 2)); // vpbroadcastw m4, [r3+hq*2]
        smooth(m, 5, 6, 2, 2, 4, 4); // SMOOTH 5, 6, 2, 2, 4, 4
        movu_st_p2(at(dst, 32 * 0), m[0]); // mova [dstq+32*0], m0
        smooth(m, 7, 8, 2, 2, 4, 4); // SMOOTH 7, 8, 2, 2, 4, 4
        movu_st_p2(at(dst, 32 * 1), m[0]); // mova [dstq+32*1], m0
        dst = at(dst, stride); // add dstq, strideq
        h -= 1; // dec hd
        if h <= 0 {
            break; // jg .w64_loop
        }
    }
}

/// `.prep` of `ipred_smooth_h_8bpc`: `128 * left + 129 * right + 128` for the 64 pixels `tl[-64..-1]` as words into
/// the caller's frame (`rsp`, the frame's 128 bytes; word `k` is the one of `tl[k - 64]`); then moves `r3` and `tl`
/// back by `h` and `2 * h` bytes.
#[inline(always)]
unsafe fn ipred_smooth_h_8bpc_prep(m: &mut M, rsp: *mut u8, r3: &mut *mut u8, tl: &mut *mut u8, h: isize) {
    m[2] = vpermq_q3120_p2(at(*tl, -32 * 1)); // vpermq m2, [tlq-32*1], q3120
    m[1] = _mm256_unpackhi_epi8(m[2], m[3]); // punpckhbw m1, m2, m3
    m[2] = _mm256_unpacklo_epi8(m[2], m[3]); // punpcklbw m2, m3
    m[0] = _mm256_maddubs_epi16(m[1], m[4]); // pmaddubsw m0, m1, m4 ; 127 * left - 127 * right
    m[1] = _mm256_add_epi16(m[1], m[5]); // paddw m1, m5     ;   1 * left + 256 * right + 128
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1     ; 128 * left + 129 * right + 128
    m[1] = _mm256_maddubs_epi16(m[2], m[4]); // pmaddubsw m1, m2, m4
    m[2] = _mm256_add_epi16(m[2], m[5]); // paddw m2, m5
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[2] = vpermq_q3120_p2(at(*tl, -32 * 2)); // vpermq m2, [tlq-32*2], q3120
    _mm256_store_si256(at(rsp, 32 * 3) as *mut __m256i, m[0]); // mova [rsp+gprsize+32*3], m0
    _mm256_store_si256(at(rsp, 32 * 2) as *mut __m256i, m[1]); // mova [rsp+gprsize+32*2], m1
    m[1] = _mm256_unpackhi_epi8(m[2], m[3]); // punpckhbw m1, m2, m3
    m[2] = _mm256_unpacklo_epi8(m[2], m[3]); // punpcklbw m2, m3
    m[0] = _mm256_maddubs_epi16(m[1], m[4]); // pmaddubsw m0, m1, m4
    m[1] = _mm256_add_epi16(m[1], m[5]); // paddw m1, m5
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
    m[1] = _mm256_maddubs_epi16(m[2], m[4]); // pmaddubsw m1, m2, m4
    m[2] = _mm256_add_epi16(m[2], m[5]); // paddw m2, m5
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    _mm256_store_si256(at(rsp, 32 * 1) as *mut __m256i, m[0]); // mova [rsp+gprsize+32*1], m0
    _mm256_store_si256(at(rsp, 32 * 0) as *mut __m256i, m[1]); // mova [rsp+gprsize+32*0], m1
    *r3 = at(*r3, -h); // sub r3, hq
    *tl = at(*tl, -h); // sub tlq, hq
    *r3 = at(*r3, -h); // sub r3, hq
}

// ---------------------------------------------------------------- smooth

/// `cglobal ipred_smooth_8bpc, 3, 7, 0, dst, stride, tl, w, h, v_weights`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_smooth_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    _angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    // Not in the original: from 16 columns on the kernel loads `topleft[-64..-1]` whatever the height, before the
    // array of an inter-intra edge (`edge + 32`); those calls run on a copy of the pixels the C kernel reads,
    // `topleft[-height..=width]`, with zeros around them (`copy_edge`).
    let mut edge: [u8; 257];
    let topleft = if width >= 16 {
        edge = [0; 257];
        crate::simd::copy_edge(&mut edge, 128, topleft, -(height as isize), width as isize)
    } else {
        topleft
    };
    let m = &mut regs();
    let base = RODATA.0.as_ptr(); // lea r6, [ipred_smooth_avx2_table] (`base`: the address of RODATA)
    let tl = topleft as *mut u8;
    let w = width as isize; // mov wd, wm
    m[4] = vpbroadcastb_p2(at(tl, w)); // vpbroadcastb m4, [tlq+wq] ; right
                                       // tzcnt wd, wd (w is a power of two from 4 to 64, never zero)
    let w = (w as u32).trailing_zeros();
    let h = height as isize; // mov hd, hm
    let r5 = at(tl, -h); // mov r5, tlq / sub r5, hq
    m[5] = vpbroadcastd_p2(rob_p2(base, pb_127_m127, 0)); // vpbroadcastd m5, [base+pb_127_m127]
    m[0] = vpbroadcastb_p2(r5); // vpbroadcastb m0, [r5] ; bottom
    m[3] = vpbroadcastd_p2(rob_p2(base, pw_255, 0)); // vpbroadcastd m3, [base+pw_255]
    let v_weights = rob_p2(base, smooth_weights, h * 2); // lea v_weightsq, [base+smooth_weights+hq*2]
                                                         // movsxd wq, [r6+wq*4] / add wq, r6 / jmp wq
    match w {
        2 => ipred_smooth_8bpc_w4(m, base, dst, stride, tl, h, v_weights),
        3 => ipred_smooth_8bpc_w8(m, base, dst, stride, tl, h, v_weights),
        4 => ipred_smooth_8bpc_w16(m, base, dst, stride, tl, h, v_weights),
        5 => ipred_smooth_8bpc_w32(m, base, dst, stride, tl, h, v_weights),
        _ => ipred_smooth_8bpc_w64(m, base, dst, stride, tl, h, v_weights),
    }
}

/// `.w4` of `ipred_smooth_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_8bpc_w4(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *mut u8,
    mut h: isize,
    mut v_weights: *const u8,
) {
    // WIN64_SPILL_XMM 12
    m[10] = movu_p2(rob_p2(base, ipred_h_shuf, 0)); // mova m10, [base+ipred_h_shuf]
    m[11] = vpbroadcastq_p2(rob_p2(base, smooth_weights, 4 * 2)); // vpbroadcastq m11, [base+smooth_weights+4*2]
    m[7] = movu_p2(rob_p2(base, ipred_v_shuf, 0)); // mova m7, [base+ipred_v_shuf]
    m[8] = vpbroadcastd_p2(at(tl, 1)); // vpbroadcastd m8, [tlq+1]
    tl = at(tl, -8); // sub tlq, 8
    let r3 = stride * 3; // lea r3, [strideq*3]
    tl = at(tl, -h); // sub tlq, hq
    m[8] = _mm256_unpacklo_epi8(m[8], m[0]); // punpcklbw m8, m0 ; top, bottom
    m[6] = _mm256_shuffle_epi32::<0xa0>(m[7]); // pshufd m6, m7, q2200
    m[7] = _mm256_shuffle_epi32::<0xf5>(m[7]); // pshufd m7, m7, q3311
    m[9] = _mm256_maddubs_epi16(m[8], m[5]); // pmaddubsw m9, m8, m5
    m[3] = _mm256_add_epi16(m[3], m[8]); // paddw m3, m8 ;   1 * top + 255 * bottom + 255
    m[9] = _mm256_add_epi16(m[9], m[3]); // paddw m9, m3 ; 128 * top + 129 * bottom + 255
    loop {
        // .w4_loop:
        m[1] = vpbroadcastq_p2(at(tl, h)); // vpbroadcastq m1, [tlq+hq]
        m[1] = _mm256_shuffle_epi8(m[1], m[10]); // pshufb m1, m10
        m[0] = _mm256_unpacklo_epi8(m[1], m[4]); // punpcklbw m0, m1, m4 ; left, right
        m[1] = _mm256_unpackhi_epi8(m[1], m[4]); // punpckhbw m1, m4
        m[2] = _mm256_maddubs_epi16(m[0], m[5]); // pmaddubsw m2, m0, m5 ; 127 * left - 127 * right
        m[3] = _mm256_maddubs_epi16(m[1], m[5]); // pmaddubsw m3, m1, m5
        m[2] = _mm256_add_epi16(m[2], m[0]); // paddw m2, m0     ; 128 * left + 129 * right
        m[3] = _mm256_add_epi16(m[3], m[1]); // paddw m3, m1
        m[0] = _mm256_maddubs_epi16(m[0], m[11]); // pmaddubsw m0, m11
        m[1] = _mm256_maddubs_epi16(m[1], m[11]); // pmaddubsw m1, m11
        m[2] = _mm256_add_epi16(m[2], m[0]); // paddw m2, m0
        m[3] = _mm256_add_epi16(m[3], m[1]); // paddw m3, m1
        m[1] = vbroadcasti128_p2(v_weights); // vbroadcasti128 m1, [v_weightsq]
        v_weights = atc_p2(v_weights, 16); // add v_weightsq, 16
        m[0] = _mm256_shuffle_epi8(m[1], m[6]); // pshufb m0, m1, m6
        m[1] = _mm256_shuffle_epi8(m[1], m[7]); // pshufb m1, m7
        smooth_2d_end(m, 0, 1, 8, 8, m[9], m[9]); // SMOOTH_2D_END 0, 1, 8, 8, 9, 9
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        movd_st_p2(at(dst, 0), lo(m[0])); // movd [dstq+strideq*0], xm0
        movd_st_p2(at(dst, stride), lo(m[1])); // movd [dstq+strideq*1], xm1
        pextrd_st_p2::<2>(at(dst, stride * 2), lo(m[0])); // pextrd [dstq+strideq*2], xm0, 2
        pextrd_st_p2::<2>(at(dst, r3), lo(m[1])); // pextrd [dstq+r3       ], xm1, 2
        if h as i32 == 4 {
            return; // cmp hd, 4 / je .ret
        }
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        pextrd_st_p2::<1>(at(dst, 0), lo(m[0])); // pextrd [dstq+strideq*0], xm0, 1
        pextrd_st_p2::<1>(at(dst, stride), lo(m[1])); // pextrd [dstq+strideq*1], xm1, 1
        pextrd_st_p2::<3>(at(dst, stride * 2), lo(m[0])); // pextrd [dstq+strideq*2], xm0, 3
        pextrd_st_p2::<3>(at(dst, r3), lo(m[1])); // pextrd [dstq+r3       ], xm1, 3
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h -= 8; // sub hd, 8
        if h <= 0 {
            break; // jg .w4_loop
        }
    }
}

/// `.w8` of `ipred_smooth_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_8bpc_w8(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *mut u8,
    mut h: isize,
    mut v_weights: *const u8,
) {
    // WIN64_SPILL_XMM 12
    m[10] = movu_p2(rob_p2(base, ipred_h_shuf, 0)); // mova m10, [base+ipred_h_shuf]
    m[11] = vbroadcasti128_p2(rob_p2(base, smooth_weights, 8 * 2)); // vbroadcasti128 m11, [base+smooth_weights+8*2]
    m[7] = movu_p2(rob_p2(base, ipred_v_shuf, 0)); // mova m7, [base+ipred_v_shuf]
    m[8] = vpbroadcastq_p2(at(tl, 1)); // vpbroadcastq m8, [tlq+1]
    tl = at(tl, -4); // sub tlq, 4
    let r3 = stride * 3; // lea r3, [strideq*3]
    tl = at(tl, -h); // sub tlq, hq
    m[8] = _mm256_unpacklo_epi8(m[8], m[0]); // punpcklbw m8, m0
    m[6] = _mm256_shuffle_epi32::<0x00>(m[7]); // pshufd m6, m7, q0000
    m[7] = _mm256_shuffle_epi32::<0x55>(m[7]); // pshufd m7, m7, q1111
    m[9] = _mm256_maddubs_epi16(m[8], m[5]); // pmaddubsw m9, m8, m5
    m[3] = _mm256_add_epi16(m[3], m[8]); // paddw m3, m8
    m[9] = _mm256_add_epi16(m[9], m[3]); // paddw m9, m3
    loop {
        // .w8_loop:
        m[1] = vpbroadcastd_p2(at(tl, h)); // vpbroadcastd m1, [tlq+hq]
        m[1] = _mm256_shuffle_epi8(m[1], m[10]); // pshufb m1, m10
        m[0] = _mm256_unpacklo_epi8(m[1], m[4]); // punpcklbw m0, m1, m4
        m[1] = _mm256_unpackhi_epi8(m[1], m[4]); // punpckhbw m1, m4
        m[2] = _mm256_maddubs_epi16(m[0], m[5]); // pmaddubsw m2, m0, m5
        m[3] = _mm256_maddubs_epi16(m[1], m[5]); // pmaddubsw m3, m1, m5
        m[2] = _mm256_add_epi16(m[2], m[0]); // paddw m2, m0
        m[3] = _mm256_add_epi16(m[3], m[1]); // paddw m3, m1
        m[0] = _mm256_maddubs_epi16(m[0], m[11]); // pmaddubsw m0, m11
        m[1] = _mm256_maddubs_epi16(m[1], m[11]); // pmaddubsw m1, m11
        m[2] = _mm256_add_epi16(m[2], m[0]); // paddw m2, m0
        m[3] = _mm256_add_epi16(m[3], m[1]); // paddw m3, m1
        m[1] = vpbroadcastq_p2(v_weights); // vpbroadcastq m1, [v_weightsq]
        v_weights = atc_p2(v_weights, 8); // add v_weightsq, 8
        m[0] = _mm256_shuffle_epi8(m[1], m[6]); // pshufb m0, m1, m6
        m[1] = _mm256_shuffle_epi8(m[1], m[7]); // pshufb m1, m7
        smooth_2d_end(m, 0, 1, 8, 8, m[9], m[9]); // SMOOTH_2D_END 0, 1, 8, 8, 9, 9
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        movq_st_p2(at(dst, 0), lo(m[0])); // movq [dstq+strideq*0], xm0
        movq_st_p2(at(dst, stride), lo(m[1])); // movq [dstq+strideq*1], xm1
        movhps_st_p2(at(dst, stride * 2), lo(m[0])); // movhps [dstq+strideq*2], xm0
        movhps_st_p2(at(dst, r3), lo(m[1])); // movhps [dstq+r3       ], xm1
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w8_loop
        }
    }
}

/// `.w16` of `ipred_smooth_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_8bpc_w16(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *mut u8,
    mut h: isize,
    mut v_weights: *const u8,
) {
    // ALLOC_STACK -32*4, 14: the frame. `ipred_smooth_8bpc_prep_v` writes all of its 128 bytes (four 32-byte
    // stores) before the loop below reads any of it, so it needs no initialization.
    let mut frame = core::mem::MaybeUninit::<[__m256i; 4]>::uninit();
    let rsp = frame.as_mut_ptr() as *mut u8;
    m[11] = vbroadcasti128_p2(at(tl, 1)); // vbroadcasti128 m11, [tlq+1]
    let mut r3 = at(rsp, 64 * 2 - 4); // lea r3, [rsp+64*2-4]
    m[10] = _mm256_unpacklo_epi8(m[11], m[0]); // punpcklbw m10, m11, m0 ; top, bottom
    m[11] = _mm256_unpackhi_epi8(m[11], m[0]); // punpckhbw m11, m0
    ipred_smooth_8bpc_prep_v(m, rsp, &mut r3, &mut tl, h); // call .prep_v
    tl = at(tl, -2); // sub tlq, 2
    m[12] = _mm256_maddubs_epi16(m[10], m[5]); // pmaddubsw m12, m10, m5
    m[13] = _mm256_maddubs_epi16(m[11], m[5]); // pmaddubsw m13, m11, m5
    m[5] = vpbroadcastd_x_p2(rob_p2(base, ipred_h_shuf, 12)); // vpbroadcastd xm5, [base+pb_1]
    m[9] = movu_p2(rob_p2(base, ipred_v_shuf, 0)); // mova m9, [base+ipred_v_shuf]
    m[6] = vbroadcasti128_p2(rob_p2(base, smooth_weights, 16 * 2)); // vbroadcasti128 m6, [base+smooth_weights+16*2]
    m[7] = vbroadcasti128_p2(rob_p2(base, smooth_weights, 16 * 3)); // vbroadcasti128 m7, [base+smooth_weights+16*3]
    m[8] = _mm256_permute2x128_si256::<0x01>(m[9], m[9]); // vperm2i128 m8, m9, m9, 0x01
    m[0] = _mm256_add_epi16(m[10], m[3]); // paddw m0, m10, m3
    m[3] = _mm256_add_epi16(m[3], m[11]); // paddw m3, m11
    m[12] = _mm256_add_epi16(m[12], m[0]); // paddw m12, m0
    m[13] = _mm256_add_epi16(m[13], m[3]); // paddw m13, m3
    loop {
        // .w16_loop:
        m[3] = vpbroadcastd_p2(at(tl, h)); // vpbroadcastd m3, [tlq+hq]
        m[0] = vpbroadcastd_p2(at(r3, h * 2)); // vpbroadcastd m0, [r3+hq*2]
        m[1] = vpbroadcastd_p2(v_weights); // vpbroadcastd m1, [v_weightsq]
        v_weights = atc_p2(v_weights, 4); // add v_weightsq, 4
        m[3] = _mm256_shuffle_epi8(m[3], m[5]); // pshufb m3, m5
        m[3] = _mm256_unpacklo_epi8(m[3], m[4]); // punpcklbw m3, m4 ; left, right
        m[2] = _mm256_maddubs_epi16(m[3], m[6]); // pmaddubsw m2, m3, m6
        m[3] = _mm256_maddubs_epi16(m[3], m[7]); // pmaddubsw m3, m7
        m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
        m[1] = _mm256_shuffle_epi8(m[1], m[9]); // pshufb m1, m9
        m[2] = _mm256_add_epi16(m[2], m[0]); // paddw m2, m0
        m[3] = _mm256_add_epi16(m[3], m[0]); // paddw m3, m0
        smooth_2d_end(m, 1, 1, 10, 11, m[12], m[13]); // SMOOTH_2D_END 1, 1, 10, 11, 12, 13
        movx_st_p2(at(dst, 0), lo(m[0])); // mova [dstq+strideq*0], xm0
        vextracti128_st_p2(at(dst, stride), m[0]); // vextracti128 [dstq+strideq*1], m0, 1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w16_loop
        }
    }
}

/// `.w32` of `ipred_smooth_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_8bpc_w32(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *mut u8,
    mut h: isize,
    mut v_weights: *const u8,
) {
    // ALLOC_STACK -32*4, 11: the frame, written whole by `ipred_smooth_8bpc_prep_v` before any load (as in `.w16`).
    let mut frame = core::mem::MaybeUninit::<[__m256i; 4]>::uninit();
    let rsp = frame.as_mut_ptr() as *mut u8;
    m[8] = movu_p2(at(tl, 1)); // movu m8, [tlq+1]
    let mut r3 = at(rsp, 64 * 2 - 2); // lea r3, [rsp+64*2-2]
    m[7] = _mm256_unpacklo_epi8(m[8], m[0]); // punpcklbw m7, m8, m0
    m[8] = _mm256_unpackhi_epi8(m[8], m[0]); // punpckhbw m8, m0
    ipred_smooth_8bpc_prep_v(m, rsp, &mut r3, &mut tl, h); // call .prep_v
    tl = at(tl, -1); // dec tlq
    m[9] = _mm256_maddubs_epi16(m[7], m[5]); // pmaddubsw m9, m7, m5
    m[10] = _mm256_maddubs_epi16(m[8], m[5]); // pmaddubsw m10, m8, m5
    m[5] = movx_p2(rob_p2(base, smooth_weights, 16 * 4)); // mova xm5, [base+smooth_weights+16*4]
    m[5] = vinserti128_p2(m[5], rob_p2(base, smooth_weights, 16 * 6)); // vinserti128 m5, [base+smooth_weights+16*6], 1
    m[6] = movx_p2(rob_p2(base, smooth_weights, 16 * 5)); // mova xm6, [base+smooth_weights+16*5]
    m[6] = vinserti128_p2(m[6], rob_p2(base, smooth_weights, 16 * 7)); // vinserti128 m6, [base+smooth_weights+16*7], 1
    m[0] = _mm256_add_epi16(m[7], m[3]); // paddw m0, m7, m3
    m[3] = _mm256_add_epi16(m[3], m[8]); // paddw m3, m8
    m[9] = _mm256_add_epi16(m[9], m[0]); // paddw m9, m0
    m[10] = _mm256_add_epi16(m[10], m[3]); // paddw m10, m3
    loop {
        // .w32_loop:
        m[3] = vpbroadcastb_p2(at(tl, h)); // vpbroadcastb m3, [tlq+hq]
        m[3] = _mm256_unpacklo_epi8(m[3], m[4]); // punpcklbw m3, m4
        m[0] = vpbroadcastw_p2(at(r3, h * 2)); // vpbroadcastw m0, [r3+hq*2]
        m[1] = vpbroadcastw_p2(v_weights); // vpbroadcastw m1, [v_weightsq]
        v_weights = atc_p2(v_weights, 2); // add v_weightsq, 2
        m[2] = _mm256_maddubs_epi16(m[3], m[5]); // pmaddubsw m2, m3, m5
        m[3] = _mm256_maddubs_epi16(m[3], m[6]); // pmaddubsw m3, m6
        m[2] = _mm256_add_epi16(m[2], m[0]); // paddw m2, m0
        m[3] = _mm256_add_epi16(m[3], m[0]); // paddw m3, m0
        smooth_2d_end(m, 1, 1, 7, 8, m[9], m[10]); // SMOOTH_2D_END 1, 1, 7, 8, 9, 10
        movu_st_p2(dst, m[0]); // mova [dstq], m0
        dst = at(dst, stride); // add dstq, strideq
        h -= 1; // dec hd
        if h <= 0 {
            break; // jg .w32_loop
        }
    }
}

/// `.w64` of `ipred_smooth_8bpc`.
#[inline(always)]
unsafe fn ipred_smooth_8bpc_w64(
    m: &mut M,
    base: *const u8,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *mut u8,
    mut h: isize,
    mut v_weights: *const u8,
) {
    // ALLOC_STACK -32*8, 16: the frame. Its first 128 bytes are written whole by `ipred_smooth_8bpc_prep_v`, the
    // other 128 by the four stores at `r6` below, all before the loop reads any of it; no initialization needed.
    let mut frame = core::mem::MaybeUninit::<[__m256i; 8]>::uninit();
    let rsp = frame.as_mut_ptr() as *mut u8;
    m[13] = movu_p2(at(tl, 1)); // movu m13, [tlq+1 ]
    m[15] = movu_p2(at(tl, 33)); // movu m15, [tlq+33]
                                 // add r6, smooth_weights+16*15-ipred_smooth_avx2_table
    let r6 = rob_p2(base, smooth_weights, 16 * 15);
    let mut r3 = at(rsp, 64 * 2 - 2); // lea r3, [rsp+64*2-2]
    m[12] = _mm256_unpacklo_epi8(m[13], m[0]); // punpcklbw m12, m13, m0
    m[13] = _mm256_unpackhi_epi8(m[13], m[0]); // punpckhbw m13, m0
    m[14] = _mm256_unpacklo_epi8(m[15], m[0]); // punpcklbw m14, m15, m0
    m[15] = _mm256_unpackhi_epi8(m[15], m[0]); // punpckhbw m15, m0
    ipred_smooth_8bpc_prep_v(m, rsp, &mut r3, &mut tl, h); // call .prep_v
    tl = at(tl, -1); // dec tlq
    m[0] = _mm256_maddubs_epi16(m[12], m[5]); // pmaddubsw m0, m12, m5
    m[1] = _mm256_maddubs_epi16(m[13], m[5]); // pmaddubsw m1, m13, m5
    m[2] = _mm256_maddubs_epi16(m[14], m[5]); // pmaddubsw m2, m14, m5
    m[5] = _mm256_maddubs_epi16(m[15], m[5]); // pmaddubsw m5, m15, m5
    m[8] = movx_p2(atc_p2(r6, -16 * 7)); // mova xm8, [r6-16*7]
    m[8] = vinserti128_p2(m[8], atc_p2(r6, -16 * 5)); // vinserti128 m8, [r6-16*5], 1
    m[9] = movx_p2(atc_p2(r6, -16 * 6)); // mova xm9, [r6-16*6]
    m[9] = vinserti128_p2(m[9], atc_p2(r6, -16 * 4)); // vinserti128 m9, [r6-16*4], 1
    m[10] = movx_p2(atc_p2(r6, -16 * 3)); // mova xm10, [r6-16*3]
    m[10] = vinserti128_p2(m[10], atc_p2(r6, -16 * 1)); // vinserti128 m10, [r6-16*1], 1
    m[11] = movx_p2(atc_p2(r6, -16 * 2)); // mova xm11, [r6-16*2]
    m[11] = vinserti128_p2(m[11], atc_p2(r6, -16 * 0)); // vinserti128 m11, [r6-16*0], 1
    let r6 = at(rsp, 32 * 4); // lea r6, [rsp+32*4]
    m[0] = _mm256_add_epi16(m[0], m[3]); // paddw m0, m3
    m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3
    m[2] = _mm256_add_epi16(m[2], m[3]); // paddw m2, m3
    m[3] = _mm256_add_epi16(m[3], m[5]); // paddw m3, m5
    m[0] = _mm256_add_epi16(m[0], m[12]); // paddw m0, m12
    m[1] = _mm256_add_epi16(m[1], m[13]); // paddw m1, m13
    m[2] = _mm256_add_epi16(m[2], m[14]); // paddw m2, m14
    m[3] = _mm256_add_epi16(m[3], m[15]); // paddw m3, m15
    _mm256_store_si256(at(r6, 32 * 0) as *mut __m256i, m[0]); // mova [r6+32*0], m0
    _mm256_store_si256(at(r6, 32 * 1) as *mut __m256i, m[1]); // mova [r6+32*1], m1
    _mm256_store_si256(at(r6, 32 * 2) as *mut __m256i, m[2]); // mova [r6+32*2], m2
    _mm256_store_si256(at(r6, 32 * 3) as *mut __m256i, m[3]); // mova [r6+32*3], m3
    loop {
        // .w64_loop:
        m[5] = vpbroadcastb_p2(at(tl, h)); // vpbroadcastb m5, [tlq+hq]
        m[5] = _mm256_unpacklo_epi8(m[5], m[4]); // punpcklbw m5, m4
        m[6] = vpbroadcastw_p2(at(r3, h * 2)); // vpbroadcastw m6, [r3+hq*2]
        m[7] = vpbroadcastw_p2(v_weights); // vpbroadcastw m7, [v_weightsq]
        v_weights = atc_p2(v_weights, 2); // add v_weightsq, 2
        m[2] = _mm256_maddubs_epi16(m[5], m[8]); // pmaddubsw m2, m5, m8
        m[3] = _mm256_maddubs_epi16(m[5], m[9]); // pmaddubsw m3, m5, m9
        m[2] = _mm256_add_epi16(m[2], m[6]); // paddw m2, m6
        m[3] = _mm256_add_epi16(m[3], m[6]); // paddw m3, m6
        let a5 = _mm256_load_si256(at(r6, 32 * 0) as *const __m256i); // [r6+32*0]
        let a6 = _mm256_load_si256(at(r6, 32 * 1) as *const __m256i); // [r6+32*1]
        smooth_2d_end(m, 7, 7, 12, 13, a5, a6); // SMOOTH_2D_END 7, 7, 12, 13, [r6+32*0], [r6+32*1]
        movu_st_p2(at(dst, 32 * 0), m[0]); // mova [dstq+32*0], m0
        m[2] = _mm256_maddubs_epi16(m[5], m[10]); // pmaddubsw m2, m5, m10
        m[3] = _mm256_maddubs_epi16(m[5], m[11]); // pmaddubsw m3, m5, m11
        m[2] = _mm256_add_epi16(m[2], m[6]); // paddw m2, m6
        m[3] = _mm256_add_epi16(m[3], m[6]); // paddw m3, m6
        let a5 = _mm256_load_si256(at(r6, 32 * 2) as *const __m256i); // [r6+32*2]
        let a6 = _mm256_load_si256(at(r6, 32 * 3) as *const __m256i); // [r6+32*3]
        smooth_2d_end(m, 7, 7, 14, 15, a5, a6); // SMOOTH_2D_END 7, 7, 14, 15, [r6+32*2], [r6+32*3]
        movu_st_p2(at(dst, 32 * 1), m[0]); // mova [dstq+32*1], m0
        dst = at(dst, stride); // add dstq, strideq
        h -= 1; // dec hd
        if h <= 0 {
            break; // jg .w64_loop
        }
    }
}

/// `.prep_v` of `ipred_smooth_8bpc`: `128 * left + 129 * right` for the 64 pixels `tl[-64..-1]` as words into the
/// caller's frame (`rsp`, its first 128 bytes; word `k` is the one of `tl[k - 64]`); then moves `r3` and `tl` back
/// by `h` and `2 * h` bytes.
#[inline(always)]
unsafe fn ipred_smooth_8bpc_prep_v(m: &mut M, rsp: *mut u8, r3: &mut *mut u8, tl: &mut *mut u8, h: isize) {
    m[2] = vpermq_q3120_p2(at(*tl, -32 * 1)); // vpermq m2, [tlq-32*1], q3120
    m[1] = _mm256_unpackhi_epi8(m[2], m[4]); // punpckhbw m1, m2, m4
    m[2] = _mm256_unpacklo_epi8(m[2], m[4]); // punpcklbw m2, m4
    m[0] = _mm256_maddubs_epi16(m[1], m[5]); // pmaddubsw m0, m1, m5 ; 127 * left - 127 * right
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1     ; 128 * left + 129 * right
    m[1] = _mm256_maddubs_epi16(m[2], m[5]); // pmaddubsw m1, m2, m5
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[2] = vpermq_q3120_p2(at(*tl, -32 * 2)); // vpermq m2, [tlq-32*2], q3120
    _mm256_store_si256(at(rsp, 32 * 3) as *mut __m256i, m[0]); // mova [rsp+gprsize+32*3], m0
    _mm256_store_si256(at(rsp, 32 * 2) as *mut __m256i, m[1]); // mova [rsp+gprsize+32*2], m1
    m[1] = _mm256_unpackhi_epi8(m[2], m[4]); // punpckhbw m1, m2, m4
    m[2] = _mm256_unpacklo_epi8(m[2], m[4]); // punpcklbw m2, m4
    m[0] = _mm256_maddubs_epi16(m[1], m[5]); // pmaddubsw m0, m1, m5
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
    m[1] = _mm256_maddubs_epi16(m[2], m[5]); // pmaddubsw m1, m2, m5
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    _mm256_store_si256(at(rsp, 32 * 1) as *mut __m256i, m[0]); // mova [rsp+gprsize+32*1], m0
    _mm256_store_si256(at(rsp, 32 * 0) as *mut __m256i, m[1]); // mova [rsp+gprsize+32*0], m1
    *r3 = at(*r3, -h); // sub r3, hq
    *tl = at(*tl, -h); // sub tlq, hq
    *r3 = at(*r3, -h); // sub r3, hq
}

// ---- part 3: `ipred_avx2.asm` lines 1310–2131 (z1) ----
//
// `cglobal ipred_z1_8bpc` is `dav1d_ipred_z1_8bpc_avx2`; its jump table `ipred_z1_avx2_table` (w4 … w64, indexed
// by `tzcnt(w)`) is the `match` there. Each width label is a function `ipred_z1_8bpc_w<N>` (with
// `.w4_no_upsample` and `.w8_no_upsample` as functions of their own, because the upsampling paths return before
// them); `.w<N>_main` (reached by `jmp`/fall-through from the filtered and the unfiltered edge) is
// `ipred_z1_8bpc_w<N>_main`; the `call`ed `.filter_strength` is `ipred_z1_8bpc_filter_strength`. The
// general-purpose registers are locals named after `DEFINE_ARGS` (`dx`, `maxbase`, `angle`) or after the register
// (`r2`, `r3`, `r5`); `tlq` is `tl`, which points to the edge or, after an edge filter, to the frame.

/// x86inc's shuffle immediates `qABCD` = `A<<6|B<<4|C<<2|D`.
const Q2121_P3: i32 = 2 << 6 | 1 << 4 | 2 << 2 | 1;
const Q1021_P3: i32 = 1 << 6 | 0 << 4 | 2 << 2 | 1;
const Q3120_P3: i32 = 3 << 6 | 1 << 4 | 2 << 2 | 0;

// The assembly's `%define`s of byte constants inside other tables (lines 120–136).
/// `%define pb_0to15 cfl_ac_w16_pad_shuffle`.
const pb_0to15_p3: usize = cfl_ac_w16_pad_shuffle;
/// `%define pb_8 (z_upsample2 +12)`.
const pb_8_p3: usize = z_upsample2 + 12;
/// `%define pb_12 (z2_y_shuf_h4+20)`.
const pb_12_p3: usize = z2_y_shuf_h4 + 20;
/// `%define pb_14 (z2_y_shuf_h4+ 4)`.
const pb_14_p3: usize = z2_y_shuf_h4 + 4;
/// `%define pb_32 (z2_y_shuf_h4+16)`.
const pb_32_p3: usize = z2_y_shuf_h4 + 16;

/// A stack frame (`ALLOC_STACK`). The assembly aligns its frames to 32 bytes for `mova`; the rebuild stores to
/// them with unaligned moves (same count) and leaves the alignment to the ABI's 16 bytes, which spares the
/// realignment of the stack pointer.
#[repr(C, align(16))]
struct FrameP3<const N: usize>([u8; N]);

/// The largest `dx` of a z1 angle: `dav1d_dr_intra_derivative[1]` (angle 3).
const DX_MAX_P3: usize = 1023;

/// The length of a frame that holds every byte the main loop loads from it after an edge filter: the main loop
/// runs `rows` rows per iteration at `xpos = dx, 2*dx, …`; its first iteration reads rows up to `rows*dx`, every
/// later one starts below `max_base*64` (the loop's exit test) and reads rows up to `(rows-1)*dx` beyond that;
/// each row loads `ext` bytes from `xpos >> 6` on. The assembly's frames are smaller (16, 32, 64, 96 and 128 bytes
/// at w 4 … 64): its loads that reach beyond them read the caller's stack, and the `vpblendvb` with
/// `top[max_base_x]` discards those bytes. The rebuild's frames are this long and zeroed whole (deviation; see the
/// report of part 3).
const fn frame_len_p3(rows: usize, max_base: usize, ext: usize) -> usize {
    let first = rows * DX_MAX_P3;
    let later = max_base * 64 - 1 + (rows - 1) * DX_MAX_P3;
    let base = (if first > later { first } else { later }) >> 6;
    (base + ext + 31) & !31
}

// Memory operands (suffix `_p3`: candidates for the prelude). Every access to caller memory is an unaligned
// access of exactly the bytes the instruction's memory operand covers.

/// `p + off` bytes for a read (wrapping: no claim that the address is in bounds until it is accessed).
#[inline(always)]
fn a_p3(p: *const u8, off: isize) -> *const u8 {
    p.wrapping_offset(off)
}

/// A 16-byte memory operand (`movu xm, [p]`, `mova xm, [p]`, `vinserti128 m, [p], 1`, `pminub xm, [p]`, …).
#[inline(always)]
unsafe fn x_p3(p: *const u8) -> __m128i {
    _mm_loadu_si128(p.cast())
}

/// A 32-byte memory operand (`movu m, [p]`, `pminub m, [p]`, `psubw m, [p]`, …).
#[inline(always)]
unsafe fn y_p3(p: *const u8) -> __m256i {
    _mm256_loadu_si256(p.cast())
}

/// `movq xm, [p]` (8 bytes; the rest of the register is cleared).
#[inline(always)]
unsafe fn movq_p3(p: *const u8) -> __m128i {
    _mm_loadl_epi64(p.cast())
}

/// `movhps xm, [p]`: the low quadword of `xm` and the 8 bytes at `p` as the high one (an unaligned read where
/// `_mm_loadh_pd` reads through an aligned `*const f64`; the 8 bytes are only moved).
#[inline(always)]
unsafe fn movhps_p3(x: __m128i, p: *const u8) -> __m128i {
    let x = _mm_castsi128_pd(x);
    _mm_castpd_si128(_mm_setr_pd(_mm_cvtsd_f64(x), p.cast::<f64>().read_unaligned()))
}

/// `vbroadcasti128 m, [p]`.
#[inline(always)]
unsafe fn vbroadcasti128_p3(p: *const u8) -> __m256i {
    _mm256_broadcastsi128_si256(x_p3(p))
}

/// `vpbroadcastq m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastq_p3(p: *const u8) -> __m256i {
    _mm256_set1_epi64x(p.cast::<i64>().read_unaligned())
}

/// `vpbroadcastd m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastd_p3(p: *const u8) -> __m256i {
    _mm256_set1_epi32(p.cast::<i32>().read_unaligned())
}

/// `vpbroadcastd xm, [p]`.
#[inline(always)]
unsafe fn vpbroadcastd_x_p3(p: *const u8) -> __m128i {
    _mm_set1_epi32(p.cast::<i32>().read_unaligned())
}

/// `vpbroadcastb m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastb_p3(p: *const u8) -> __m256i {
    _mm256_set1_epi8(p.cast::<i8>().read())
}

/// `movd xm, r32d`.
#[inline(always)]
unsafe fn movd_r_p3(r: u32) -> __m256i {
    zx(_mm_cvtsi32_si128(r as i32))
}

/// `shufps mD, mA, mB, IMM` on integer registers.
#[inline(always)]
unsafe fn shufps_p3<const IMM: i32>(a: __m256i, b: __m256i) -> __m256i {
    _mm256_castps_si256(_mm256_shuffle_ps::<IMM>(_mm256_castsi256_ps(a), _mm256_castsi256_ps(b)))
}

/// `vinserti128 mD, mA, [p], I`.
#[inline(always)]
unsafe fn vinserti128_p3<const I: i32>(a: __m256i, p: *const u8) -> __m256i {
    _mm256_inserti128_si256::<I>(a, x_p3(p))
}

/// `movd [p], xm`.
#[inline(always)]
unsafe fn movd_st_p3(p: *mut u8, x: __m128i) {
    p.cast::<i32>().write_unaligned(_mm_cvtsi128_si32(x))
}

/// `pextrd [p], xm, I`.
#[inline(always)]
unsafe fn pextrd_st_p3<const I: i32>(p: *mut u8, x: __m128i) {
    p.cast::<i32>().write_unaligned(_mm_extract_epi32::<I>(x))
}

/// `movq [p], xm`.
#[inline(always)]
unsafe fn movq_st_p3(p: *mut u8, x: __m128i) {
    _mm_storel_epi64(p.cast(), x)
}

/// `movhps [p], xm`: the high quadword, as the unaligned write of an extracted element (`vpextrq [p], xm, 1`,
/// the same 8 bytes in one instruction).
#[inline(always)]
unsafe fn movhps_st_p3(p: *mut u8, x: __m128i) {
    p.cast::<i64>().write_unaligned(_mm_extract_epi64::<1>(x))
}

/// `.filter_strength` (w4/w8/w16, `call`ed): compares `maxbase` (`w + h - 1` in the C kernel's terms) with
/// `z_filter_wh` and the angle `d` with the thresholds `z_filter_t0`/`t1` (`is_sm`) in parallel; the number of
/// set bits of the result is the filter strength. Writes `m0` (`maxbase` in every byte, which `.w8` and `.w16`
/// use afterwards), `m1`, `m2`; returns `r5d` (the assembly's `angle` register, which it overwrites).
#[inline(always)]
unsafe fn ipred_z1_8bpc_filter_strength(m: &mut M, maxbase: u32, mut angle: u32) -> u32 {
    // %define base r3-z_filter_t0 / lea r3, [z_filter_t0]: the operands are `ro(label, …)`
    m[0] = movd_r_p3(maxbase); // movd xm0, maxbased
    m[2] = movd_r_p3(angle); // movd xm2, angled
    angle >>= 8; // shr angled, 8 ; is_sm << 1
    m[0] = _mm256_broadcastb_epi8(lo(m[0])); // vpbroadcastb m0, xm0
    m[2] = _mm256_broadcastb_epi8(lo(m[2])); // vpbroadcastb m2, xm2
    m[1] = _mm256_cmpeq_epi8(m[0], y_p3(ro(z_filter_wh, 0))); // pcmpeqb m1, m0, [base+z_filter_wh]
    m[1] = _mm256_and_si256(m[1], m[2]); // pand m1, m2
                                         // `angle >> 8` is `is_sm << 1`, 0 or 2: the callers come here only with bit 10 (0x400 after the entry's
                                         // `xor`) clear, and the contract keeps the angle's flags in 0x600, so this is `z_filter_t0` or `z_filter_t1`.
    m[2] = zx(x_p3(ro(z_filter_t0, angle as isize * 8))); // mova xm2, [r3+angleq*8] ; upper ymm half zero
    m[1] = _mm256_cmpgt_epi8(m[1], m[2]); // pcmpgtb m1, m2
    _mm256_movemask_epi8(m[1]) as u32 // pmovmskb r5d, m1 / ret
}

/// `.w4`: the upsampling path, or `.w4_no_upsample`.
#[inline(always)]
unsafe fn ipred_z1_8bpc_w4(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: i32,
    angle: u32,
    mut dx: u32,
) {
    if angle & 0xff >= 40 {
        // cmp angleb, 40
        return ipred_z1_8bpc_w4_no_upsample(m, dst, stride, tl, h, angle, dx); // jae .w4_no_upsample
    }
    let mut r3 = (angle as i32).wrapping_sub(1024); // lea r3d, [angleq-1024]
    r3 >>= 7; // sar r3d, 7
    r3 = r3.wrapping_add(h); // add r3d, hd
    if r3 > 0 {
        return ipred_z1_8bpc_w4_no_upsample(m, dst, stride, tl, h, angle, dx); // jg .w4_no_upsample
    }
    // ALLOC_STACK -32, 8: zeroed whole. The loop below loads 8 bytes at `xpos >> 6` for xpos up to `h * 2*dx`
    // with `h <= 8` (this path) and `dx <= 51` (`d < 40`: the angle is above 50), so at most bytes 0..20.
    let mut fr = FrameP3([0u8; 32]);
    let rsp = fr.0.as_mut_ptr();
    m[1] = zx(x_p3(a_p3(tl, -1))); // mova xm1, [tlq-1]
    m[0] = zx(_mm_shuffle_epi8(lo(m[1]), x_p3(ro(z_upsample1, 0)))); // pshufb xm0, xm1, [z_upsample1]
    m[1] = zx(_mm_shuffle_epi8(lo(m[1]), x_p3(ro(z_upsample2, 0)))); // pshufb xm1, [z_upsample2]
    m[2] = zx(vpbroadcastd_x_p3(ro(pb_36_m4, 0))); // vpbroadcastd xm2, [pb_36_m4]
    dx = dx.wrapping_add(dx); // add dxd, dxd
    m[0] = zx(_mm_maddubs_epi16(lo(m[0]), lo(m[2]))); // pmaddubsw xm0, xm2
    pextrd_st_p3::<3>(at(rsp, 16), lo(m[1])); // pextrd [rsp+16], xm1, 3 ; top[max_base_x]
    m[1] = zx(_mm_maddubs_epi16(lo(m[1]), lo(m[2]))); // pmaddubsw xm1, xm2
    m[7] = movd_r_p3(dx); // movd xm7, dxd
    let mut r3 = dx; // mov r3d, dxd ; xpos
    m[7] = _mm256_broadcastw_epi16(lo(m[7])); // vpbroadcastw m7, xm7
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[0]))); // paddw xm1, xm0
    m[0] = zx(movq_p3(tl)); // movq xm0, [tlq]
    m[1] = zx(_mm_mulhrs_epi16(lo(m[1]), lo(m[3]))); // pmulhrsw xm1, xm3
    m[6] = _mm256_bslli_epi128::<8>(m[7]); // pslldq m6, m7, 8
    m[2] = zx(_mm_add_epi16(lo(m[7]), lo(m[7]))); // paddw xm2, xm7, xm7
    let r2 = stride * 3; // lea r2, [strideq*3]
    m[6] = _mm256_add_epi16(m[6], m[7]); // paddw m6, m7
    m[1] = zx(_mm_packus_epi16(lo(m[1]), lo(m[1]))); // packuswb xm1, xm1
    m[6] = _mm256_add_epi16(m[6], m[2]); // paddw m6, m2 ; xpos2 xpos3 xpos0 xpos1
    m[0] = zx(_mm_unpacklo_epi8(lo(m[0]), lo(m[1]))); // punpcklbw xm0, xm1
    m[7] = _mm256_slli_epi16::<2>(m[7]); // psllw m7, 2
    _mm_storeu_si128(rsp.cast(), lo(m[0])); // mova [rsp], xm0
    let rsp = rsp as *const u8;
    loop {
        // .w4_upsample_loop
        let mut r5 = r3.wrapping_add(dx); // lea r5d, [r3+dxq]
        r3 >>= 6; // shr r3d, 6 ; base0
        m[1] = vpbroadcastq_p3(a_p3(rsp, r3 as isize)); // vpbroadcastq m1, [rsp+r3]
        r3 = r5.wrapping_add(dx); // lea r3d, [r5+dxq]
        r5 >>= 6; // shr r5d, 6 ; base1
        m[2] = vpbroadcastq_p3(a_p3(rsp, r5 as isize)); // vpbroadcastq m2, [rsp+r5]
        r5 = r3.wrapping_add(dx); // lea r5d, [r3+dxq]
        r3 >>= 6; // shr r3d, 6 ; base2
        m[0] = zx(movq_p3(a_p3(rsp, r3 as isize))); // movq xm0, [rsp+r3]
        r3 = r5.wrapping_add(dx); // lea r3d, [r5+dxq]
        r5 >>= 6; // shr r5d, 6 ; base3
        m[0] = zx(movhps_p3(lo(m[0]), a_p3(rsp, r5 as isize))); // movhps xm0, [rsp+r5]
        m[1] = _mm256_blend_epi32::<0xc0>(m[1], m[2]); // vpblendd m1, m2, 0xc0
        m[2] = _mm256_and_si256(m[4], m[6]); // pand m2, m4, m6 ; frac
        m[0] = _mm256_blend_epi32::<0xf0>(m[0], m[1]); // vpblendd m0, m1, 0xf0
        m[1] = _mm256_sub_epi16(m[5], m[2]); // psubw m1, m5, m2 ; 64-frac
        m[2] = _mm256_slli_epi16::<8>(m[2]); // psllw m2, 8
        m[1] = _mm256_or_si256(m[1], m[2]); // por m1, m2 ; 64-frac, frac
        m[0] = _mm256_maddubs_epi16(m[0], m[1]); // pmaddubsw m0, m1
        m[6] = _mm256_add_epi16(m[6], m[7]); // paddw m6, m7 ; xpos += dx
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[0] = _mm256_packus_epi16(m[0], m[0]); // packuswb m0, m0
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        movd_st_p3(at(dst, stride * 2), lo(m[0])); // movd [dstq+strideq*2], xm0
        pextrd_st_p3::<1>(at(dst, r2), lo(m[0])); // pextrd [dstq+r2], xm0, 1
        movd_st_p3(dst, lo(m[1])); // movd [dstq+strideq*0], xm1
        pextrd_st_p3::<1>(at(dst, stride), lo(m[1])); // pextrd [dstq+strideq*1], xm1, 1
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w4_upsample_loop
        }
    }
} // RET

/// `.w4_no_upsample`: the edge filter (into the frame), then `.w4_main`.
#[inline(always)]
unsafe fn ipred_z1_8bpc_w4_no_upsample(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    h: i32,
    angle: u32,
    dx: u32,
) {
    // ALLOC_STACK -16, 11: written (zeroed whole, `frame_len_p3`) only where the edge filter runs.
    let mut fr: FrameP3<{ frame_len_p3(4, 9, 8) }>;
    let mut maxbase: u32 = 7; // mov maxbased, 7
    if angle & 0x400 == 0 {
        // test angled, 0x400 ; !enable_intra_edge_filter / jnz .w4_main
        maxbase = (h + 3) as u32; // lea maxbased, [hq+3]
        let mut r5 = ipred_z1_8bpc_filter_strength(m, maxbase, angle); // call .filter_strength
        maxbase = 7; // mov maxbased, 7
        if r5 != 0 {
            // test r5d, r5d / jz .w4_main ; filter_strength == 0
            fr = FrameP3([0; frame_len_p3(4, 9, 8)]);
            let rsp = fr.0.as_mut_ptr();
            r5 = r5.count_ones(); // popcnt r5d, r5d
            let k = r5 as isize * 4 - 4; // [base+z_filter_k-4+r5*4+12*n]: r5 is 1..=3
            m[7] = vpbroadcastd_p3(ro(pb_8_p3, 0)); // vpbroadcastd m7, [base+pb_8]
            m[2] = vbroadcasti128_p3(a_p3(tl, -1)); // vbroadcasti128 m2, [tlq-1]
            m[1] = _mm256_min_epu8(m[7], y_p3(ro(dav1d_z_filter_s, 0))); // pminub m1, m7, [base+z_filter_s]
            m[8] = vpbroadcastd_p3(ro(z_filter_k, k + 12 * 0)); // vpbroadcastd m8, [base+z_filter_k-4+r5*4+12*0]
            m[7] = _mm256_min_epu8(m[7], y_p3(ro(dav1d_z_filter_s, 8))); // pminub m7, [base+z_filter_s+8]
            m[9] = vpbroadcastd_p3(ro(z_filter_k, k + 12 * 1)); // vpbroadcastd m9, [base+z_filter_k-4+r5*4+12*1]
            m[10] = vpbroadcastd_p3(ro(z_filter_k, k + 12 * 2)); // vpbroadcastd m10, [base+z_filter_k-4+r5*4+12*2]
            m[0] = _mm256_shuffle_epi8(m[2], m[1]); // pshufb m0, m2, m1
            m[1] = shufps_p3::<Q2121_P3>(m[1], m[7]); // shufps m1, m7, q2121
            m[0] = _mm256_maddubs_epi16(m[0], m[8]); // pmaddubsw m0, m8
            m[1] = _mm256_shuffle_epi8(m[2], m[1]); // pshufb m1, m2, m1
            m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
            m[2] = _mm256_shuffle_epi8(m[2], m[7]); // pshufb m2, m7
            m[2] = _mm256_maddubs_epi16(m[2], m[10]); // pmaddubsw m2, m10
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
            m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
            let r3 = 9; // mov r3d, 9
            tl = rsp; // mov tlq, rsp
            if h != 4 {
                maxbase = r3; // cmp hd, 4 / cmovne maxbased, r3d
            }
            m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
            m[0] = zx(_mm_packus_epi16(lo(m[0]), lo(m[1]))); // packuswb xm0, xm1
            _mm_storeu_si128(rsp.cast(), lo(m[0])); // mova [tlq], xm0
        }
    }
    ipred_z1_8bpc_w4_main(m, dst, stride, tl, h, dx, maxbase)
}

/// `.w4_main` … `.w4_end`.
#[inline(always)]
unsafe fn ipred_z1_8bpc_w4_main(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: i32,
    dx: u32,
    mut maxbase: u32,
) {
    m[6] = movd_r_p3(dx); // movd xm6, dxd
    m[0] = vpbroadcastq_p3(ro(z_base_inc, 0)); // vpbroadcastq m0, [z_base_inc] ; base_inc << 6
    m[7] = vpbroadcastb_p3(a_p3(tl, maxbase as isize)); // vpbroadcastb m7, [tlq+maxbaseq]
    maxbase <<= 6; // shl maxbased, 6
    m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
    let mut r3 = dx; // mov r3d, dxd ; xpos
    m[9] = movd_r_p3(maxbase); // movd xm9, maxbased
    m[9] = _mm256_broadcastw_epi16(lo(m[9])); // vpbroadcastw m9, xm9
    m[8] = vbroadcasti128_p3(ro(z1_shuf_w4, 0)); // vbroadcasti128 m8, [z1_shuf_w4]
    m[7] = _mm256_srli_epi16::<8>(m[7]); // psrlw m7, 8 ; top[max_base_x]
    m[10] = _mm256_add_epi16(m[6], m[6]); // paddw m10, m6, m6
    m[9] = _mm256_sub_epi16(m[9], m[0]); // psubw m9, m0 ; max_base_x
    m[6] = _mm256_blend_epi32::<0xcc>(m[6], m[10]); // vpblendd m6, m10, 0xcc
    m[0] = zx(lo(m[10])); // mova xm0, xm10
    m[6] = _mm256_add_epi16(m[6], m[0]); // paddw m6, m0 ; xpos2 xpos3 xpos0 xpos1
    m[10] = _mm256_add_epi16(m[10], m[10]); // paddw m10, m10
    loop {
        // .w4_loop
        let mut r5 = r3.wrapping_add(dx); // lea r5d, [r3+dxq]
        r3 >>= 6; // shr r3d, 6 ; base0
        m[1] = vpbroadcastq_p3(a_p3(tl, r3 as isize)); // vpbroadcastq m1, [tlq+r3]
        r3 = r5.wrapping_add(dx); // lea r3d, [r5+dxq]
        r5 >>= 6; // shr r5d, 6 ; base1
        m[2] = vpbroadcastq_p3(a_p3(tl, r5 as isize)); // vpbroadcastq m2, [tlq+r5]
        r5 = r3.wrapping_add(dx); // lea r5d, [r3+dxq]
        r3 >>= 6; // shr r3d, 6 ; base2
        m[0] = zx(movq_p3(a_p3(tl, r3 as isize))); // movq xm0, [tlq+r3]
        r3 = r5.wrapping_add(dx); // lea r3d, [r5+dxq]
        r5 >>= 6; // shr r5d, 6 ; base3
        m[0] = zx(movhps_p3(lo(m[0]), a_p3(tl, r5 as isize))); // movhps xm0, [tlq+r5]
        m[1] = _mm256_blend_epi32::<0xc0>(m[1], m[2]); // vpblendd m1, m2, 0xc0
        m[2] = _mm256_and_si256(m[4], m[6]); // pand m2, m4, m6 ; frac
        m[0] = _mm256_blend_epi32::<0xf0>(m[0], m[1]); // vpblendd m0, m1, 0xf0
        m[1] = _mm256_sub_epi16(m[5], m[2]); // psubw m1, m5, m2 ; 64-frac
        m[2] = _mm256_slli_epi16::<8>(m[2]); // psllw m2, 8
        m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
        m[1] = _mm256_or_si256(m[1], m[2]); // por m1, m2 ; 64-frac, frac
        m[0] = _mm256_maddubs_epi16(m[0], m[1]); // pmaddubsw m0, m1
        m[1] = _mm256_cmpgt_epi16(m[9], m[6]); // pcmpgtw m1, m9, m6 ; base < max_base_x
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[6] = _mm256_add_epi16(m[6], m[10]); // paddw m6, m10 ; xpos += dx
        let r5p = at(dst, stride * 2); // lea r5, [dstq+strideq*2]
        m[0] = _mm256_blendv_epi8(m[7], m[0], m[1]); // vpblendvb m0, m7, m0, m1
        m[0] = _mm256_packus_epi16(m[0], m[0]); // packuswb m0, m0
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        movd_st_p3(r5p, lo(m[0])); // movd [r5+strideq*0], xm0
        pextrd_st_p3::<1>(at(r5p, stride), lo(m[0])); // pextrd [r5+strideq*1], xm0, 1
        movd_st_p3(dst, lo(m[1])); // movd [dstq+strideq*0], xm1
        pextrd_st_p3::<1>(at(dst, stride), lo(m[1])); // pextrd [dstq+strideq*1], xm1, 1
        h -= 4; // sub hd, 4
        if h == 0 {
            return; // jz .w4_end
        }
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        if r3 >= maxbase {
            break; // cmp r3d, maxbased / jb .w4_loop
        }
    }
    m[7] = zx(_mm_packus_epi16(lo(m[7]), lo(m[7]))); // packuswb xm7, xm7
    let r6 = stride * 3; // lea r6, [strideq*3]
    loop {
        // .w4_end_loop
        movd_st_p3(dst, lo(m[7])); // movd [dstq+strideq*0], xm7
        movd_st_p3(at(dst, stride), lo(m[7])); // movd [dstq+strideq*1], xm7
        movd_st_p3(at(dst, stride * 2), lo(m[7])); // movd [dstq+strideq*2], xm7
        movd_st_p3(at(dst, r6), lo(m[7])); // movd [dstq+r6], xm7
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w4_end_loop
        }
    }
} // .w4_end: RET

/// `.w8`: the upsampling path, or `.w8_no_upsample`.
#[inline(always)]
unsafe fn ipred_z1_8bpc_w8(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: i32,
    angle: u32,
    mut dx: u32,
) {
    let mut r3 = angle.wrapping_add(216); // lea r3d, [angleq+216]
    r3 = r3 & !0xff | (h as u32 & 0xff); // mov r3b, hb
    if r3 > 8 {
        // cmp r3d, 8
        return ipred_z1_8bpc_w8_no_upsample(m, dst, stride, tl, h, angle, dx); // ja .w8_no_upsample ; !enable_intra_edge_filter || is_sm || d >= 40 || h > 8
    }
    // ALLOC_STACK -32, 8: written whole by `mova [rsp], m0` before the loop's first load (which reads 16 bytes
    // at `xpos >> 6` for xpos up to `h * 2*dx`, `h <= 8`, `dx <= 51`: at most bytes 0..28).
    let mut fr = FrameP3([0u8; 32]);
    let rsp = fr.0.as_mut_ptr();
    m[2] = zx(x_p3(ro(dav1d_z_filter_s, 6))); // movu xm2, [z_filter_s+6]
    m[0] = zx(x_p3(a_p3(tl, -1))); // mova xm0, [tlq-1]
    m[6] = movd_r_p3(h as u32); // movd xm6, hd
    m[0] = vinserti128_p3::<1>(m[0], a_p3(tl, 7)); // vinserti128 m0, [tlq+7], 1
    m[6] = zx(_mm_broadcastb_epi8(lo(m[6]))); // vpbroadcastb xm6, xm6
    m[1] = vbroadcasti128_p3(ro(z_upsample1, 0)); // vbroadcasti128 m1, [z_upsample1]
    m[6] = zx(_mm_min_epu8(lo(m[6]), lo(m[2]))); // pminub xm6, xm2
    m[7] = vpbroadcastd_p3(ro(pb_36_m4, 0)); // vpbroadcastd m7, [pb_36_m4]
    m[2] = _mm256_inserti128_si256::<1>(m[2], lo(m[6])); // vinserti128 m2, xm6, 1
    dx = dx.wrapping_add(dx); // add dxd, dxd
    m[1] = _mm256_shuffle_epi8(m[0], m[1]); // pshufb m1, m0, m1
    m[2] = _mm256_shuffle_epi8(m[0], m[2]); // pshufb m2, m0, m2
    m[6] = movd_r_p3(dx); // movd xm6, dxd
    m[1] = _mm256_maddubs_epi16(m[1], m[7]); // pmaddubsw m1, m7
    m[2] = _mm256_maddubs_epi16(m[2], m[7]); // pmaddubsw m2, m7
    m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
    let mut r3 = dx; // mov r3d, dxd
    m[0] = _mm256_bsrli_epi128::<1>(m[0]); // psrldq m0, 1
    let r2 = stride * 3; // lea r2, [strideq*3]
    m[7] = _mm256_add_epi16(m[6], m[6]); // paddw m7, m6, m6
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[6] = _mm256_blend_epi32::<0xf0>(m[6], m[7]); // vpblendd m6, m7, 0xf0
    m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
    m[2] = _mm256_bslli_epi128::<8>(m[7]); // pslldq m2, m7, 8
    m[7] = _mm256_add_epi16(m[7], m[7]); // paddw m7, m7
    m[6] = _mm256_add_epi16(m[6], m[2]); // paddw m6, m2
    m[1] = _mm256_packus_epi16(m[1], m[1]); // packuswb m1, m1
    m[0] = _mm256_unpacklo_epi8(m[0], m[1]); // punpcklbw m0, m1
    _mm256_storeu_si256(rsp.cast(), m[0]); // mova [rsp], m0
    let rsp = rsp as *const u8;
    loop {
        // .w8_upsample_loop
        let mut r5 = r3.wrapping_add(dx); // lea r5d, [r3+dxq]
        r3 >>= 6; // shr r3d, 6 ; base0
        m[0] = zx(x_p3(a_p3(rsp, r3 as isize))); // movu xm0, [rsp+r3]
        r3 = r5.wrapping_add(dx); // lea r3d, [r5+dxq]
        r5 >>= 6; // shr r5d, 6 ; base1
        m[0] = vinserti128_p3::<1>(m[0], a_p3(rsp, r5 as isize)); // vinserti128 m0, [rsp+r5], 1
        r5 = r3.wrapping_add(dx); // lea r5d, [r3+dxq]
        r3 >>= 6; // shr r3d, 6 ; base2
        m[1] = _mm256_and_si256(m[4], m[6]); // pand m1, m4, m6
        m[2] = _mm256_sub_epi16(m[5], m[1]); // psubw m2, m5, m1
        m[1] = _mm256_slli_epi16::<8>(m[1]); // psllw m1, 8
        m[2] = _mm256_or_si256(m[2], m[1]); // por m2, m1
        m[1] = _mm256_unpacklo_epi64(m[2], m[2]); // punpcklqdq m1, m2, m2 ; frac0 frac1
        m[0] = _mm256_maddubs_epi16(m[0], m[1]); // pmaddubsw m0, m1
        m[1] = zx(x_p3(a_p3(rsp, r3 as isize))); // movu xm1, [rsp+r3]
        r3 = r5.wrapping_add(dx); // lea r3d, [r5+dxq]
        r5 >>= 6; // shr r5d, 6 ; base3
        m[1] = vinserti128_p3::<1>(m[1], a_p3(rsp, r5 as isize)); // vinserti128 m1, [rsp+r5], 1
        m[2] = _mm256_unpackhi_epi64(m[2], m[2]); // punpckhqdq m2, m2 ; frac2 frac3
        m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[6] = _mm256_add_epi16(m[6], m[7]); // paddw m6, m7
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        movq_st_p3(dst, lo(m[0])); // movq [dstq+strideq*0], xm0
        movhps_st_p3(at(dst, stride * 2), lo(m[0])); // movhps [dstq+strideq*2], xm0
        movq_st_p3(at(dst, stride), lo(m[1])); // movq [dstq+strideq*1], xm1
        movhps_st_p3(at(dst, r2), lo(m[1])); // movhps [dstq+r2], xm1
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w8_upsample_loop
        }
    }
} // RET

/// `.w8_no_upsample` (and `.w8_no_intra_edge_filter`): the edge filter (into the frame), then `.w8_main`.
#[inline(always)]
unsafe fn ipred_z1_8bpc_w8_no_upsample(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    h: i32,
    angle: u32,
    dx: u32,
) {
    // ALLOC_STACK -32, 10: written (zeroed whole, `frame_len_p3`) only where the edge filter runs.
    let mut fr: FrameP3<{ frame_len_p3(2, 17, 16) }>;
    let mut maxbase = (h + 7) as u32; // lea maxbased, [hq+7]
    if angle & 0x400 != 0 {
        // test angled, 0x400 / jnz .w8_no_intra_edge_filter
        maxbase &= 7; // and maxbased, 7
        maxbase |= 8; // or maxbased, 8 ; imin(h+7, 15)
                      // jmp .w8_main
    } else {
        let mut r5 = ipred_z1_8bpc_filter_strength(m, maxbase, angle); // call .filter_strength
        if r5 != 0 {
            // test r5d, r5d / jz .w8_main ; filter_strength == 0
            fr = FrameP3([0; frame_len_p3(2, 17, 16)]);
            let rsp = fr.0.as_mut_ptr();
            r5 = r5.count_ones(); // popcnt r5d, r5d
            let k = r5 as isize * 4 - 4; // [base+z_filter_k-4+r5*4+12*n]: r5 is 1..=3
            m[2] = zx(x_p3(tl)); // movu xm2, [tlq]
            m[1] = zx(_mm_min_epu8(lo(m[0]), x_p3(ro(dav1d_z_filter_s, 14)))); // pminub xm1, xm0, [base+z_filter_s+14]
            m[2] = vinserti128_p3::<1>(m[2], a_p3(tl, -1)); // vinserti128 m2, [tlq-1], 1
            m[1] = vinserti128_p3::<1>(m[1], ro(dav1d_z_filter_s, 0)); // vinserti128 m1, [base+z_filter_s+ 0], 1
            m[7] = vpbroadcastd_p3(ro(z_filter_k, k + 12 * 0)); // vpbroadcastd m7, [base+z_filter_k-4+r5*4+12*0]
            m[0] = zx(_mm_min_epu8(lo(m[0]), x_p3(ro(dav1d_z_filter_s, 22)))); // pminub xm0, [base+z_filter_s+22]
            m[0] = vinserti128_p3::<1>(m[0], ro(dav1d_z_filter_s, 8)); // vinserti128 m0, [base+z_filter_s+ 8], 1
            m[6] = _mm256_shuffle_epi8(m[2], m[1]); // pshufb m6, m2, m1
            m[6] = _mm256_maddubs_epi16(m[6], m[7]); // pmaddubsw m6, m7
            m[7] = vpbroadcastd_p3(ro(z_filter_k, k + 12 * 1)); // vpbroadcastd m7, [base+z_filter_k-4+r5*4+12*1]
            let r3 = a_p3(tl, 15).read() as u32; // movzx r3d, byte [tlq+15]
            m[1] = shufps_p3::<Q2121_P3>(m[1], m[0]); // shufps m1, m0, q2121
            m[1] = _mm256_shuffle_epi8(m[2], m[1]); // pshufb m1, m2, m1
            m[1] = _mm256_maddubs_epi16(m[1], m[7]); // pmaddubsw m1, m7
            m[1] = _mm256_add_epi16(m[1], m[6]); // paddw m1, m6
            r5 = r5.wrapping_sub(3); // sub r5d, 3
            if r5 == 0 {
                // jnz .w8_3tap
                // filter_strength == 3 uses a 5-tap filter instead of a 3-tap one, which also results in an
                // awkward edge case where out[w*2] is slightly different from out[max_base_x] when h > w.
                m[7] = vpbroadcastd_p3(ro(z_filter_k, 4 * 8)); // vpbroadcastd m7, [z_filter_k+4*8]
                let mut r2 = a_p3(tl, 14).read() as u32; // movzx r2d, byte [tlq+14]
                m[2] = _mm256_shuffle_epi8(m[2], m[0]); // pshufb m2, m0
                m[2] = _mm256_maddubs_epi16(m[2], m[7]); // pmaddubsw m2, m7
                r2 = r2.wrapping_sub(r3); // sub r2d, r3d
                r2 = r2.wrapping_add(r3 * 8).wrapping_add(4); // lea r2d, [r2+r3*8+4]
                r2 >>= 3; // shr r2d, 3 ; (tlq[w*2-2] + tlq[w*2-1]*7 + 4) >> 3
                *rsp.add(16) = r2 as u8; // mov [rsp+16], r2b
                m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
            }
            // .w8_3tap:
            m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
            r5 = ((r5 as i32) >> 1) as u32; // sar r5d, 1
            tl = rsp; // mov tlq, rsp
            r5 = r5.wrapping_add(17); // add r5d, 17 ; w*2 + (filter_strength == 3)
            if h - 16 >= 0 {
                maxbase = r5; // cmp hd, 16 / cmovns maxbased, r5d
            }
            *rsp.add(r5 as usize) = r3 as u8; // mov [tlq+r5], r3b (r5 is 16 or 17)
            m[0] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm0, m1, 1
            m[0] = zx(_mm_packus_epi16(lo(m[0]), lo(m[1]))); // packuswb xm0, xm1
            _mm_storeu_si128(rsp.cast(), lo(m[0])); // mova [tlq], xm0
        }
    }
    ipred_z1_8bpc_w8_main(m, dst, stride, tl, h, dx, maxbase)
}

/// `.w8_main` … `.w8_end`.
#[inline(always)]
unsafe fn ipred_z1_8bpc_w8_main(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: i32,
    dx: u32,
    mut maxbase: u32,
) {
    m[2] = movd_r_p3(dx); // movd xm2, dxd
    m[0] = vbroadcasti128_p3(ro(z_base_inc, 0)); // vbroadcasti128 m0, [z_base_inc]
    m[2] = _mm256_broadcastw_epi16(lo(m[2])); // vpbroadcastw m2, xm2
    m[7] = vpbroadcastb_p3(a_p3(tl, maxbase as isize)); // vpbroadcastb m7, [tlq+maxbaseq]
    maxbase <<= 6; // shl maxbased, 6
    m[9] = movd_r_p3(maxbase); // movd xm9, maxbased
    m[8] = vbroadcasti128_p3(ro(dav1d_z_filter_s, 2)); // vbroadcasti128 m8, [z_filter_s+2]
    m[9] = _mm256_broadcastw_epi16(lo(m[9])); // vpbroadcastw m9, xm9
    m[7] = _mm256_srli_epi16::<8>(m[7]); // psrlw m7, 8
    m[9] = _mm256_sub_epi16(m[9], m[0]); // psubw m9, m0
    let mut r3 = dx; // mov r3d, dxd
    m[6] = _mm256_add_epi16(m[2], m[2]); // paddw m6, m2, m2
    m[2] = _mm256_blend_epi32::<0xf0>(m[2], m[6]); // vpblendd m2, m6, 0xf0
    loop {
        // .w8_loop
        let mut r5 = r3.wrapping_add(dx); // lea r5d, [r3+dxq]
        r3 >>= 6; // shr r3d, 6
        m[0] = _mm256_and_si256(m[4], m[2]); // pand m0, m4, m2
        m[1] = _mm256_sub_epi16(m[5], m[0]); // psubw m1, m5, m0
        m[0] = _mm256_slli_epi16::<8>(m[0]); // psllw m0, 8
        m[1] = _mm256_or_si256(m[1], m[0]); // por m1, m0
        m[0] = zx(x_p3(a_p3(tl, r3 as isize))); // movu xm0, [tlq+r3]
        r3 = r5.wrapping_add(dx); // lea r3d, [r5+dxq]
        r5 >>= 6; // shr r5d, 6 ; base1
        m[0] = vinserti128_p3::<1>(m[0], a_p3(tl, r5 as isize)); // vinserti128 m0, [tlq+r5], 1
        m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
        m[0] = _mm256_maddubs_epi16(m[0], m[1]); // pmaddubsw m0, m1
        m[1] = _mm256_cmpgt_epi16(m[9], m[2]); // pcmpgtw m1, m9, m2
        m[2] = _mm256_add_epi16(m[2], m[6]); // paddw m2, m6
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[0] = _mm256_blendv_epi8(m[7], m[0], m[1]); // vpblendvb m0, m7, m0, m1
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        m[0] = zx(_mm_packus_epi16(lo(m[0]), lo(m[1]))); // packuswb xm0, xm1
        movq_st_p3(dst, lo(m[0])); // movq [dstq+strideq*0], xm0
        movhps_st_p3(at(dst, stride), lo(m[0])); // movhps [dstq+strideq*1], xm0
        h -= 2; // sub hd, 2
        if h == 0 {
            return; // jz .w8_end
        }
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        if r3 >= maxbase {
            break; // cmp r3d, maxbased / jb .w8_loop
        }
    }
    m[7] = zx(_mm_packus_epi16(lo(m[7]), lo(m[7]))); // packuswb xm7, xm7
    loop {
        // .w8_end_loop
        movq_st_p3(dst, lo(m[7])); // movq [dstq+strideq*0], xm7
        movq_st_p3(at(dst, stride), lo(m[7])); // movq [dstq+strideq*1], xm7
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w8_end_loop
        }
    }
} // .w8_end: RET

/// `.w16` (and `.w16_no_intra_edge_filter`): the edge filter (into the frame), then `.w16_main`.
#[inline(always)]
unsafe fn ipred_z1_8bpc_w16(m: &mut M, dst: *mut u8, stride: isize, mut tl: *const u8, h: i32, angle: u32, dx: u32) {
    // ALLOC_STACK -64, 12: written (zeroed whole, `frame_len_p3`) only where the edge filter runs.
    let mut fr: FrameP3<{ frame_len_p3(2, 33, 24) }>;
    let mut maxbase = (h + 15) as u32; // lea maxbased, [hq+15]
    if angle & 0x400 != 0 {
        // test angled, 0x400 / jnz .w16_no_intra_edge_filter
        maxbase &= 15; // and maxbased, 15
        maxbase |= 16; // or maxbased, 16 ; imin(h+15, 31)
                       // jmp .w16_main
    } else {
        let mut r5 = ipred_z1_8bpc_filter_strength(m, maxbase, angle); // call .filter_strength
        if r5 != 0 {
            // test r5d, r5d / jz .w16_main ; filter_strength == 0
            fr = FrameP3([0; frame_len_p3(2, 33, 24)]);
            let rsp = fr.0.as_mut_ptr();
            r5 = r5.count_ones(); // popcnt r5d, r5d
            let k = r5 as isize * 4 - 4; // [base+z_filter_k-4+r5*4+12*n]: r5 is 1..=3
            m[1] = vpbroadcastd_p3(ro(pb_12_p3, 0)); // vpbroadcastd m1, [base+pb_12]
            m[6] = vbroadcasti128_p3(ro(dav1d_z_filter_s, 8)); // vbroadcasti128 m6, [base+z_filter_s+8]
            m[2] = vinserti128_p3::<0>(m[6], ro(dav1d_z_filter_s, 0)); // vinserti128 m2, m6, [base+z_filter_s], 0
            m[6] = vinserti128_p3::<1>(m[6], ro(dav1d_z_filter_s, 16)); // vinserti128 m6, [base+z_filter_s+16], 1
            m[10] = zx(x_p3(a_p3(tl, -1))); // mova xm10, [tlq-1]
            m[10] = vinserti128_p3::<1>(m[10], a_p3(tl, 3)); // vinserti128 m10, [tlq+3], 1
            m[9] = vpbroadcastd_p3(ro(z_filter_k, k + 12 * 0)); // vpbroadcastd m9, [base+z_filter_k-4+r5*4+12*0]
            m[7] = vbroadcasti128_p3(ro(dav1d_z_filter_s, 14)); // vbroadcasti128 m7, [base+z_filter_s+14]
            m[8] = vinserti128_p3::<0>(m[7], ro(dav1d_z_filter_s, 6)); // vinserti128 m8, m7, [base+z_filter_s+6], 0
            m[7] = vinserti128_p3::<1>(m[7], ro(dav1d_z_filter_s, 22)); // vinserti128 m7, [base+z_filter_s+22], 1
            m[0] = _mm256_sub_epi16(m[0], m[1]); // psubw m0, m1
            m[11] = zx(x_p3(a_p3(tl, 12))); // movu xm11, [tlq+12]
            m[11] = vinserti128_p3::<1>(m[11], a_p3(tl, 16)); // vinserti128 m11, [tlq+16], 1
            m[8] = _mm256_min_epu8(m[8], m[0]); // pminub m8, m0
            m[7] = _mm256_min_epu8(m[7], m[0]); // pminub m7, m0
            m[0] = _mm256_shuffle_epi8(m[10], m[2]); // pshufb m0, m10, m2
            m[2] = shufps_p3::<Q2121_P3>(m[2], m[6]); // shufps m2, m6, q2121
            m[0] = _mm256_maddubs_epi16(m[0], m[9]); // pmaddubsw m0, m9
            m[1] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m1, m11, m8
            m[8] = shufps_p3::<Q2121_P3>(m[8], m[7]); // shufps m8, m7, q2121
            m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
            m[9] = vpbroadcastd_p3(ro(z_filter_k, k + 12 * 1)); // vpbroadcastd m9, [base+z_filter_k-4+r5*4+12*1]
            let r3 = a_p3(tl, 31).read() as u32; // movzx r3d, byte [tlq+31]
            m[2] = _mm256_shuffle_epi8(m[10], m[2]); // pshufb m2, m10, m2
            m[2] = _mm256_maddubs_epi16(m[2], m[9]); // pmaddubsw m2, m9
            m[8] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m8, m11, m8
            m[8] = _mm256_maddubs_epi16(m[8], m[9]); // pmaddubsw m8, m9
            m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
            m[1] = _mm256_add_epi16(m[1], m[8]); // paddw m1, m8
            r5 = r5.wrapping_sub(3); // sub r5d, 3
            if r5 == 0 {
                // jnz .w16_3tap
                m[9] = vpbroadcastd_p3(ro(z_filter_k, 4 * 8)); // vpbroadcastd m9, [z_filter_k+4*8]
                let mut r2 = a_p3(tl, 30).read() as u32; // movzx r2d, byte [tlq+30]
                m[10] = _mm256_shuffle_epi8(m[10], m[6]); // pshufb m10, m6
                m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
                m[11] = _mm256_shuffle_epi8(m[11], m[7]); // pshufb m11, m7
                m[11] = _mm256_maddubs_epi16(m[11], m[9]); // pmaddubsw m11, m9
                r2 = r2.wrapping_sub(r3); // sub r2d, r3d
                r2 = r2.wrapping_add(r3 * 8).wrapping_add(4); // lea r2d, [r2+r3*8+4]
                r2 >>= 3; // shr r2d, 3
                *rsp.add(32) = r2 as u8; // mov [rsp+32], r2b
                m[0] = _mm256_add_epi16(m[0], m[10]); // paddw m0, m10
                m[1] = _mm256_add_epi16(m[1], m[11]); // paddw m1, m11
            }
            // .w16_3tap:
            m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
            m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
            r5 = ((r5 as i32) >> 1) as u32; // sar r5d, 1
            tl = rsp; // mov tlq, rsp
            r5 = r5.wrapping_add(33); // add r5d, 33
            if h - 32 >= 0 {
                maxbase = r5; // cmp hd, 32 / cmovns maxbased, r5d
            }
            *rsp.add(r5 as usize) = r3 as u8; // mov [tlq+r5], r3b (r5 is 32 or 33)
            m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
            m[0] = _mm256_permute4x64_epi64::<Q3120_P3>(m[0]); // vpermq m0, m0, q3120
            _mm256_storeu_si256(rsp.cast(), m[0]); // mova [tlq], m0
        }
    }
    ipred_z1_8bpc_w16_main(m, dst, stride, tl, h, dx, maxbase)
}

/// `.w16_main` … `.w16_end`.
#[inline(always)]
unsafe fn ipred_z1_8bpc_w16_main(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: i32,
    dx: u32,
    mut maxbase: u32,
) {
    m[6] = movd_r_p3(dx); // movd xm6, dxd
    m[0] = vbroadcasti128_p3(ro(z_base_inc, 0)); // vbroadcasti128 m0, [z_base_inc]
    m[7] = vpbroadcastb_p3(a_p3(tl, maxbase as isize)); // vpbroadcastb m7, [tlq+maxbaseq]
    maxbase <<= 6; // shl maxbased, 6
    m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
    m[9] = movd_r_p3(maxbase); // movd xm9, maxbased
    m[8] = vbroadcasti128_p3(ro(dav1d_z_filter_s, 2)); // vbroadcasti128 m8, [z_filter_s+2]
    m[9] = _mm256_broadcastw_epi16(lo(m[9])); // vpbroadcastw m9, xm9
    let mut r3 = dx; // mov r3d, dxd
    m[9] = _mm256_sub_epi16(m[9], m[0]); // psubw m9, m0
    m[11] = _mm256_add_epi16(m[6], m[6]); // paddw m11, m6, m6
    m[10] = _mm256_sub_epi16(m[9], m[3]); // psubw m10, m9, m3 ; 64*8
    m[6] = _mm256_blend_epi32::<0xf0>(m[6], m[11]); // vpblendd m6, m11, 0xf0
    loop {
        // .w16_loop
        let mut r5 = r3.wrapping_add(dx); // lea r5d, [r3+dxq]
        r3 >>= 6; // shr r3d, 6 ; base0
        m[1] = _mm256_and_si256(m[4], m[6]); // pand m1, m4, m6
        m[2] = _mm256_sub_epi16(m[5], m[1]); // psubw m2, m5, m1
        m[1] = _mm256_slli_epi16::<8>(m[1]); // psllw m1, 8
        m[2] = _mm256_or_si256(m[2], m[1]); // por m2, m1
        m[0] = zx(x_p3(a_p3(tl, r3 as isize))); // movu xm0, [tlq+r3+0]
        m[1] = zx(x_p3(a_p3(tl, r3 as isize + 8))); // movu xm1, [tlq+r3+8]
        r3 = r5.wrapping_add(dx); // lea r3d, [r5+dxq]
        r5 >>= 6; // shr r5d, 6 ; base1
        m[0] = vinserti128_p3::<1>(m[0], a_p3(tl, r5 as isize)); // vinserti128 m0, [tlq+r5+0], 1
        m[1] = vinserti128_p3::<1>(m[1], a_p3(tl, r5 as isize + 8)); // vinserti128 m1, [tlq+r5+8], 1
        m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
        m[1] = _mm256_shuffle_epi8(m[1], m[8]); // pshufb m1, m8
        m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
        m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
        m[1] = _mm256_cmpgt_epi16(m[9], m[6]); // pcmpgtw m1, m9, m6
        m[2] = _mm256_cmpgt_epi16(m[10], m[6]); // pcmpgtw m2, m10, m6
        m[1] = _mm256_packs_epi16(m[1], m[2]); // packsswb m1, m2
        m[6] = _mm256_add_epi16(m[6], m[11]); // paddw m6, m11
        m[0] = _mm256_blendv_epi8(m[7], m[0], m[1]); // vpblendvb m0, m7, m0, m1
        _mm_storeu_si128(dst.cast(), lo(m[0])); // mova [dstq+strideq*0], xm0
        _mm_storeu_si128(at(dst, stride).cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
        h -= 2; // sub hd, 2
        if h == 0 {
            return; // jz .w16_end
        }
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        if r3 >= maxbase {
            break; // cmp r3d, maxbased / jb .w16_loop
        }
    }
    loop {
        // .w16_end_loop
        _mm_storeu_si128(dst.cast(), lo(m[7])); // mova [dstq+strideq*0], xm7
        _mm_storeu_si128(at(dst, stride).cast(), lo(m[7])); // mova [dstq+strideq*1], xm7
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w16_end_loop
        }
    }
} // .w16_end: RET

/// `.w32`: the edge filter (into the frame), then `.w32_main`.
#[inline(always)]
unsafe fn ipred_z1_8bpc_w32(m: &mut M, dst: *mut u8, stride: isize, mut tl: *const u8, h: i32, angle: u32, dx: u32) {
    // ALLOC_STACK -96, 15: written (zeroed whole, `frame_len_p3`) only where the edge filter runs.
    let mut fr: FrameP3<{ frame_len_p3(1, 65, 40) }>;
    let mut r3 = (h + 31) as u32; // lea r3d, [hq+31]
    let mut maxbase: u32 = 63; // mov maxbased, 63
    if h - 32 < 0 {
        maxbase = r3; // cmp hd, 32 / cmovs maxbased, r3d
    }
    if angle & 0x400 == 0 {
        // test angled, 0x400 ; !enable_intra_edge_filter / jnz .w32_main
        fr = FrameP3([0; frame_len_p3(1, 65, 40)]);
        let rsp = fr.0.as_mut_ptr();
        m[0] = vbroadcasti128_p3(ro(pb_0to15_p3, 0)); // vbroadcasti128 m0, [pb_0to15]
        r3 = r3.wrapping_sub(29); // sub r3d, 29 ; h+2
        m[13] = zx(x_p3(a_p3(tl, 29))); // movu xm13, [tlq+29] ; 32-39
        m[1] = movd_r_p3(r3); // movd xm1, r3d
        m[14] = zx(x_p3(a_p3(tl, 37))); // movu xm14, [tlq+37] ; 40-47
        r3 = r3.wrapping_sub(8); // sub r3d, 8 ; h-6
        m[14] = vinserti128_p3::<1>(m[14], a_p3(tl, 51)); // vinserti128 m14, [tlq+51], 1 ; 56-63
        m[1] = zx(_mm_broadcastb_epi8(lo(m[1]))); // vpbroadcastb xm1, xm1
        m[11] = zx(x_p3(a_p3(tl, -1))); // mova xm11, [tlq- 1] ; 0- 7
        m[11] = vinserti128_p3::<1>(m[11], a_p3(tl, 13)); // vinserti128 m11, [tlq+13], 1 ; 16-23
        m[2] = movd_r_p3(r3); // movd xm2, r3d
        m[12] = zx(x_p3(a_p3(tl, 5))); // movu xm12, [tlq+ 5] ; 8-15
        m[12] = vinserti128_p3::<1>(m[12], a_p3(tl, 19)); // vinserti128 m12, [tlq+19], 1 ; 24-31
        m[1] = zx(_mm_min_epu8(lo(m[1]), lo(m[0]))); // pminub xm1, xm0 ; clip 32x8
        m[7] = y_p3(ro(dav1d_z_filter_s, 0)); // mova m7, [z_filter_s+0]
        m[13] = zx(_mm_shuffle_epi8(lo(m[13]), lo(m[1]))); // pshufb xm13, xm1
        m[1] = vpbroadcastd_p3(ro(pb_12_p3, 0)); // vpbroadcastd m1, [pb_12]
        m[2] = zx(_mm_broadcastb_epi8(lo(m[2]))); // vpbroadcastb xm2, xm2
        m[13] = vinserti128_p3::<1>(m[13], a_p3(tl, 43)); // vinserti128 m13, [tlq+43], 1 ; 48-55
        m[8] = vinserti128_p3::<1>(m[7], ro(dav1d_z_filter_s, 4)); // vinserti128 m8, m7, [z_filter_s+4], 1
        m[2] = _mm256_blend_epi32::<0xf0>(m[2], m[1]); // vpblendd m2, m1, 0xf0
        m[7] = vinserti128_p3::<0>(m[7], ro(dav1d_z_filter_s, 12)); // vinserti128 m7, [z_filter_s+12], 0
        m[2] = _mm256_min_epu8(m[2], m[0]); // pminub m2, m0 ; clip 32x16 and 32x(32|64)
        m[9] = vpbroadcastd_p3(ro(z_filter_k, 4 * 2 + 12 * 0)); // vpbroadcastd m9, [z_filter_k+4*2+12*0]
        m[14] = _mm256_shuffle_epi8(m[14], m[2]); // pshufb m14, m2
        m[0] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m0, m11, m8
        m[8] = shufps_p3::<Q1021_P3>(m[8], m[7]); // shufps m8, m7, q1021
        m[0] = _mm256_maddubs_epi16(m[0], m[9]); // pmaddubsw m0, m9
        m[2] = _mm256_shuffle_epi8(m[12], m[8]); // pshufb m2, m12, m8
        m[2] = _mm256_maddubs_epi16(m[2], m[9]); // pmaddubsw m2, m9
        m[1] = _mm256_shuffle_epi8(m[13], m[8]); // pshufb m1, m13, m8
        m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
        m[6] = _mm256_shuffle_epi8(m[14], m[8]); // pshufb m6, m14, m8
        m[6] = _mm256_maddubs_epi16(m[6], m[9]); // pmaddubsw m6, m9
        m[9] = vpbroadcastd_p3(ro(z_filter_k, 4 * 2 + 12 * 1)); // vpbroadcastd m9, [z_filter_k+4*2+12*1]
        m[10] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m10, m11, m8
        m[8] = shufps_p3::<Q2121_P3>(m[8], m[7]); // shufps m8, m7, q2121
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[0] = _mm256_add_epi16(m[0], m[10]); // paddw m0, m10
        m[10] = _mm256_shuffle_epi8(m[12], m[8]); // pshufb m10, m12, m8
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[2] = _mm256_add_epi16(m[2], m[10]); // paddw m2, m10
        m[10] = _mm256_shuffle_epi8(m[13], m[8]); // pshufb m10, m13, m8
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[1] = _mm256_add_epi16(m[1], m[10]); // paddw m1, m10
        m[10] = _mm256_shuffle_epi8(m[14], m[8]); // pshufb m10, m14, m8
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[6] = _mm256_add_epi16(m[6], m[10]); // paddw m6, m10
        m[9] = vpbroadcastd_p3(ro(z_filter_k, 4 * 2 + 12 * 2)); // vpbroadcastd m9, [z_filter_k+4*2+12*2]
        m[11] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m11, m8
        m[11] = _mm256_maddubs_epi16(m[11], m[9]); // pmaddubsw m11, m9
        m[12] = _mm256_shuffle_epi8(m[12], m[7]); // pshufb m12, m7
        m[12] = _mm256_maddubs_epi16(m[12], m[9]); // pmaddubsw m12, m9
        r3 = a_p3(tl, 63).read() as u32; // movzx r3d, byte [tlq+63]
        let mut r2 = a_p3(tl, 62).read() as u32; // movzx r2d, byte [tlq+62]
        m[0] = _mm256_add_epi16(m[0], m[11]); // paddw m0, m11
        m[2] = _mm256_add_epi16(m[2], m[12]); // paddw m2, m12
        m[13] = _mm256_shuffle_epi8(m[13], m[7]); // pshufb m13, m7
        m[13] = _mm256_maddubs_epi16(m[13], m[9]); // pmaddubsw m13, m9
        m[14] = _mm256_shuffle_epi8(m[14], m[7]); // pshufb m14, m7
        m[14] = _mm256_maddubs_epi16(m[14], m[9]); // pmaddubsw m14, m9
        m[1] = _mm256_add_epi16(m[1], m[13]); // paddw m1, m13
        m[6] = _mm256_add_epi16(m[6], m[14]); // paddw m6, m14
        r2 = r2.wrapping_sub(r3); // sub r2d, r3d
        r2 = r2.wrapping_add(r3 * 8).wrapping_add(4); // lea r2d, [r2+r3*8+4] ; edge case for 32x64
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[2] = _mm256_mulhrs_epi16(m[2], m[3]); // pmulhrsw m2, m3
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        m[6] = _mm256_mulhrs_epi16(m[6], m[3]); // pmulhrsw m6, m3
        r2 >>= 3; // shr r2d, 3
        *rsp.add(64) = r2 as u8; // mov [rsp+64], r2b
        tl = rsp; // mov tlq, rsp
        *rsp.add(65) = r3 as u8; // mov [tlq+65], r3b
        r3 = 65; // mov r3d, 65
        if h == 64 {
            maxbase = r3; // cmp hd, 64 / cmove maxbased, r3d
        }
        m[0] = _mm256_packus_epi16(m[0], m[2]); // packuswb m0, m2
        m[1] = _mm256_packus_epi16(m[1], m[6]); // packuswb m1, m6
        _mm256_storeu_si256(rsp.cast(), m[0]); // mova [tlq+ 0], m0
        _mm256_storeu_si256(rsp.add(32).cast(), m[1]); // mova [tlq+32], m1
    }
    ipred_z1_8bpc_w32_main(m, dst, stride, tl, h, dx, maxbase)
}

/// `.w32_main` … `.w32_end`.
#[inline(always)]
unsafe fn ipred_z1_8bpc_w32_main(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: i32,
    dx: u32,
    mut maxbase: u32,
) {
    m[6] = movd_r_p3(dx); // movd xm6, dxd
    m[7] = vpbroadcastb_p3(a_p3(tl, maxbase as isize)); // vpbroadcastb m7, [tlq+maxbaseq]
    maxbase <<= 6; // shl maxbased, 6
    m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
    m[9] = movd_r_p3(maxbase); // movd xm9, maxbased
    m[8] = vbroadcasti128_p3(ro(dav1d_z_filter_s, 2)); // vbroadcasti128 m8, [z_filter_s+2]
    m[9] = _mm256_broadcastw_epi16(lo(m[9])); // vpbroadcastw m9, xm9
    let mut r5 = dx; // mov r5d, dxd
    m[9] = _mm256_sub_epi16(m[9], y_p3(ro(z_base_inc, 0))); // psubw m9, [z_base_inc]
    m[11] = m[6]; // mova m11, m6
    m[10] = _mm256_sub_epi16(m[9], m[3]); // psubw m10, m9, m3 ; 64*8
    loop {
        // .w32_loop
        let mut r3 = r5; // mov r3d, r5d
        r3 >>= 6; // shr r3d, 6
        m[1] = _mm256_and_si256(m[4], m[6]); // pand m1, m4, m6
        m[2] = _mm256_sub_epi16(m[5], m[1]); // psubw m2, m5, m1
        m[1] = _mm256_slli_epi16::<8>(m[1]); // psllw m1, 8
        m[2] = _mm256_or_si256(m[2], m[1]); // por m2, m1
        m[0] = y_p3(a_p3(tl, r3 as isize)); // movu m0, [tlq+r3+0]
        m[1] = y_p3(a_p3(tl, r3 as isize + 8)); // movu m1, [tlq+r3+8]
        r5 = r5.wrapping_add(dx); // add r5d, dxd
        m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
        m[1] = _mm256_shuffle_epi8(m[1], m[8]); // pshufb m1, m8
        m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
        m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
        m[1] = _mm256_cmpgt_epi16(m[9], m[6]); // pcmpgtw m1, m9, m6
        m[2] = _mm256_cmpgt_epi16(m[10], m[6]); // pcmpgtw m2, m10, m6
        m[1] = _mm256_packs_epi16(m[1], m[2]); // packsswb m1, m2
        m[6] = _mm256_add_epi16(m[6], m[11]); // paddw m6, m11
        m[0] = _mm256_blendv_epi8(m[7], m[0], m[1]); // vpblendvb m0, m7, m0, m1
        _mm256_storeu_si256(dst.cast(), m[0]); // mova [dstq], m0
        h -= 1; // dec hd
        if h == 0 {
            return; // jz .w32_end
        }
        dst = at(dst, stride); // add dstq, strideq
        if r5 >= maxbase {
            break; // cmp r5d, maxbased / jb .w32_loop
        }
    }
    if h & 1 != 0 {
        // test hb, 1 / jz .w32_end_loop
        _mm256_storeu_si256(dst.cast(), m[7]); // mova [dstq], m7
        dst = at(dst, stride); // add dstq, strideq
        h -= 1; // dec hd
        if h == 0 {
            return; // jz .w32_end
        }
    }
    loop {
        // .w32_end_loop
        _mm256_storeu_si256(dst.cast(), m[7]); // mova [dstq+strideq*0], m7
        _mm256_storeu_si256(at(dst, stride).cast(), m[7]); // mova [dstq+strideq*1], m7
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w32_end_loop
        }
    }
} // .w32_end: RET

/// `.w64`: the edge filter (into the frame), then `.w64_main`.
#[inline(always)]
unsafe fn ipred_z1_8bpc_w64(m: &mut M, dst: *mut u8, stride: isize, mut tl: *const u8, h: i32, angle: u32, dx: u32) {
    // ALLOC_STACK -128, 16: written (zeroed whole, `frame_len_p3`) only where the edge filter runs.
    let mut fr: FrameP3<{ frame_len_p3(1, 127, 72) }>;
    let maxbase = (h + 63) as u32; // lea maxbased, [hq+63]
    if angle & 0x400 == 0 {
        // test angled, 0x400 ; !enable_intra_edge_filter / jnz .w64_main
        fr = FrameP3([0; frame_len_p3(1, 127, 72)]);
        let rsp = fr.0.as_mut_ptr();
        m[11] = zx(x_p3(a_p3(tl, -1))); // mova xm11, [tlq- 1] ; 0- 7
        m[11] = vinserti128_p3::<1>(m[11], a_p3(tl, 13)); // vinserti128 m11, [tlq+13], 1 ; 16-23
        m[12] = zx(x_p3(a_p3(tl, 5))); // movu xm12, [tlq+ 5] ; 8-15
        m[12] = vinserti128_p3::<1>(m[12], a_p3(tl, 19)); // vinserti128 m12, [tlq+19], 1 ; 24-31
        m[7] = y_p3(ro(dav1d_z_filter_s, 0)); // mova m7, [z_filter_s+0]
        m[8] = vinserti128_p3::<1>(m[7], ro(dav1d_z_filter_s, 4)); // vinserti128 m8, m7, [z_filter_s+4], 1
        m[7] = vinserti128_p3::<0>(m[7], ro(dav1d_z_filter_s, 12)); // vinserti128 m7, [z_filter_s+12], 0
        m[9] = vpbroadcastd_p3(ro(z_filter_k, 4 * 2 + 12 * 0)); // vpbroadcastd m9, [z_filter_k+4*2+12*0]
        m[13] = zx(x_p3(a_p3(tl, 29))); // movu xm13, [tlq+29] ; 32-39
        m[13] = vinserti128_p3::<1>(m[13], a_p3(tl, 43)); // vinserti128 m13, [tlq+43], 1 ; 48-55
        m[14] = zx(x_p3(a_p3(tl, 37))); // movu xm14, [tlq+37] ; 40-47
        m[14] = vinserti128_p3::<1>(m[14], a_p3(tl, 51)); // vinserti128 m14, [tlq+51], 1 ; 56-63
        m[0] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m0, m11, m8
        m[8] = shufps_p3::<Q1021_P3>(m[8], m[7]); // shufps m8, m7, q1021
        m[0] = _mm256_maddubs_epi16(m[0], m[9]); // pmaddubsw m0, m9
        m[2] = _mm256_shuffle_epi8(m[12], m[8]); // pshufb m2, m12, m8
        m[2] = _mm256_maddubs_epi16(m[2], m[9]); // pmaddubsw m2, m9
        m[1] = _mm256_shuffle_epi8(m[13], m[8]); // pshufb m1, m13, m8
        m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
        m[6] = _mm256_shuffle_epi8(m[14], m[8]); // pshufb m6, m14, m8
        m[6] = _mm256_maddubs_epi16(m[6], m[9]); // pmaddubsw m6, m9
        m[9] = vpbroadcastd_p3(ro(z_filter_k, 4 * 2 + 12 * 1)); // vpbroadcastd m9, [z_filter_k+4*2+12*1]
        m[10] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m10, m11, m8
        m[15] = shufps_p3::<Q2121_P3>(m[8], m[7]); // shufps m15, m8, m7, q2121
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[0] = _mm256_add_epi16(m[0], m[10]); // paddw m0, m10
        m[10] = _mm256_shuffle_epi8(m[12], m[15]); // pshufb m10, m12, m15
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[2] = _mm256_add_epi16(m[2], m[10]); // paddw m2, m10
        m[10] = _mm256_shuffle_epi8(m[13], m[15]); // pshufb m10, m13, m15
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[1] = _mm256_add_epi16(m[1], m[10]); // paddw m1, m10
        m[10] = _mm256_shuffle_epi8(m[14], m[15]); // pshufb m10, m14, m15
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[6] = _mm256_add_epi16(m[6], m[10]); // paddw m6, m10
        m[10] = vpbroadcastd_p3(ro(z_filter_k, 4 * 2 + 12 * 2)); // vpbroadcastd m10, [z_filter_k+4*2+12*2]
        m[11] = _mm256_shuffle_epi8(m[11], m[15]); // pshufb m11, m15
        m[11] = _mm256_maddubs_epi16(m[11], m[10]); // pmaddubsw m11, m10
        m[12] = _mm256_shuffle_epi8(m[12], m[7]); // pshufb m12, m7
        m[12] = _mm256_maddubs_epi16(m[12], m[10]); // pmaddubsw m12, m10
        m[13] = _mm256_shuffle_epi8(m[13], m[7]); // pshufb m13, m7
        m[13] = _mm256_maddubs_epi16(m[13], m[10]); // pmaddubsw m13, m10
        m[14] = _mm256_shuffle_epi8(m[14], m[7]); // pshufb m14, m7
        m[14] = _mm256_maddubs_epi16(m[14], m[10]); // pmaddubsw m14, m10
        m[0] = _mm256_add_epi16(m[0], m[11]); // paddw m0, m11
        m[2] = _mm256_add_epi16(m[2], m[12]); // paddw m2, m12
        m[1] = _mm256_add_epi16(m[1], m[13]); // paddw m1, m13
        m[6] = _mm256_add_epi16(m[6], m[14]); // paddw m6, m14
        m[11] = zx(x_p3(a_p3(tl, 61))); // movu xm11, [tlq+ 61] ; 64- 71
        m[11] = vinserti128_p3::<1>(m[11], a_p3(tl, 75)); // vinserti128 m11, [tlq+ 75], 1 ; 80- 87
        m[12] = zx(x_p3(a_p3(tl, 69))); // movu xm12, [tlq+ 69] ; 72- 79
        m[12] = vinserti128_p3::<1>(m[12], a_p3(tl, 83)); // vinserti128 m12, [tlq+ 83], 1 ; 88- 95
        m[13] = zx(x_p3(a_p3(tl, 93))); // movu xm13, [tlq+ 93] ; 96-103
        m[13] = vinserti128_p3::<1>(m[13], a_p3(tl, 107)); // vinserti128 m13, [tlq+107], 1 ; 112-119
        m[14] = zx(x_p3(a_p3(tl, 101))); // movu xm14, [tlq+101] ; 104-111
        m[14] = vinserti128_p3::<1>(m[14], a_p3(tl, 115)); // vinserti128 m14, [tlq+115], 1 ; 120-127
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[2] = _mm256_mulhrs_epi16(m[2], m[3]); // pmulhrsw m2, m3
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        m[6] = _mm256_mulhrs_epi16(m[6], m[3]); // pmulhrsw m6, m3
        let r3 = (h - 20) as u32; // lea r3d, [hq-20]
        tl = rsp; // mov tlq, rsp
        m[0] = _mm256_packus_epi16(m[0], m[2]); // packuswb m0, m2
        m[1] = _mm256_packus_epi16(m[1], m[6]); // packuswb m1, m6
        m[2] = zx(vpbroadcastd_x_p3(ro(pb_14_p3, 0))); // vpbroadcastd xm2, [pb_14]
        m[6] = vbroadcasti128_p3(ro(pb_0to15_p3, 0)); // vbroadcasti128 m6, [pb_0to15]
        _mm256_storeu_si256(rsp.cast(), m[0]); // mova [tlq+32*0], m0
        _mm256_storeu_si256(rsp.add(32).cast(), m[1]); // mova [tlq+32*1], m1
        m[0] = movd_r_p3(r3); // movd xm0, r3d
        m[1] = vpbroadcastd_p3(ro(pb_12_p3, 0)); // vpbroadcastd m1, [pb_12]
        m[0] = _mm256_broadcastb_epi8(lo(m[0])); // vpbroadcastb m0, xm0
        m[0] = _mm256_add_epi8(m[0], m[2]); // paddb m0, m2
        m[0] = _mm256_min_epu8(m[0], m[6]); // pminub m0, m6 ; clip 64x16 and 64x32
        m[12] = _mm256_shuffle_epi8(m[12], m[0]); // pshufb m12, m0
        m[1] = _mm256_min_epu8(m[1], m[6]); // pminub m1, m6 ; clip 64x64
        m[14] = _mm256_shuffle_epi8(m[14], m[1]); // pshufb m14, m1
        m[0] = _mm256_shuffle_epi8(m[11], m[7]); // pshufb m0, m11, m7
        m[0] = _mm256_maddubs_epi16(m[0], m[10]); // pmaddubsw m0, m10
        m[2] = _mm256_shuffle_epi8(m[12], m[7]); // pshufb m2, m12, m7
        m[2] = _mm256_maddubs_epi16(m[2], m[10]); // pmaddubsw m2, m10
        m[1] = _mm256_shuffle_epi8(m[13], m[7]); // pshufb m1, m13, m7
        m[1] = _mm256_maddubs_epi16(m[1], m[10]); // pmaddubsw m1, m10
        m[6] = _mm256_shuffle_epi8(m[14], m[7]); // pshufb m6, m14, m7
        m[6] = _mm256_maddubs_epi16(m[6], m[10]); // pmaddubsw m6, m10
        m[7] = _mm256_shuffle_epi8(m[11], m[15]); // pshufb m7, m11, m15
        m[7] = _mm256_maddubs_epi16(m[7], m[9]); // pmaddubsw m7, m9
        m[10] = _mm256_shuffle_epi8(m[12], m[15]); // pshufb m10, m12, m15
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[0] = _mm256_add_epi16(m[0], m[7]); // paddw m0, m7
        m[7] = _mm256_shuffle_epi8(m[13], m[15]); // pshufb m7, m13, m15
        m[7] = _mm256_maddubs_epi16(m[7], m[9]); // pmaddubsw m7, m9
        m[2] = _mm256_add_epi16(m[2], m[10]); // paddw m2, m10
        m[10] = _mm256_shuffle_epi8(m[14], m[15]); // pshufb m10, m14, m15
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[1] = _mm256_add_epi16(m[1], m[7]); // paddw m1, m7
        m[6] = _mm256_add_epi16(m[6], m[10]); // paddw m6, m10
        m[9] = vpbroadcastd_p3(ro(z_filter_k, 4 * 2 + 12 * 0)); // vpbroadcastd m9, [z_filter_k+4*2+12*0]
        m[11] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m11, m8
        m[11] = _mm256_maddubs_epi16(m[11], m[9]); // pmaddubsw m11, m9
        m[12] = _mm256_shuffle_epi8(m[12], m[8]); // pshufb m12, m8
        m[12] = _mm256_maddubs_epi16(m[12], m[9]); // pmaddubsw m12, m9
        m[13] = _mm256_shuffle_epi8(m[13], m[8]); // pshufb m13, m8
        m[13] = _mm256_maddubs_epi16(m[13], m[9]); // pmaddubsw m13, m9
        m[14] = _mm256_shuffle_epi8(m[14], m[8]); // pshufb m14, m8
        m[14] = _mm256_maddubs_epi16(m[14], m[9]); // pmaddubsw m14, m9
        m[0] = _mm256_add_epi16(m[0], m[11]); // paddw m0, m11
        m[2] = _mm256_add_epi16(m[2], m[12]); // paddw m2, m12
        m[1] = _mm256_add_epi16(m[1], m[13]); // paddw m1, m13
        m[6] = _mm256_add_epi16(m[6], m[14]); // paddw m6, m14
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[2] = _mm256_mulhrs_epi16(m[2], m[3]); // pmulhrsw m2, m3
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        m[6] = _mm256_mulhrs_epi16(m[6], m[3]); // pmulhrsw m6, m3
        m[0] = _mm256_packus_epi16(m[0], m[2]); // packuswb m0, m2
        m[1] = _mm256_packus_epi16(m[1], m[6]); // packuswb m1, m6
        _mm256_storeu_si256(rsp.add(64).cast(), m[0]); // mova [tlq+32*2], m0
        _mm256_storeu_si256(rsp.add(96).cast(), m[1]); // mova [tlq+32*3], m1
    }
    ipred_z1_8bpc_w64_main(m, dst, stride, tl, h, dx, maxbase)
}

/// `.w64_main` … `.w64_end`.
#[inline(always)]
unsafe fn ipred_z1_8bpc_w64_main(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: i32,
    dx: u32,
    mut maxbase: u32,
) {
    m[12] = movd_r_p3(dx); // movd xm12, dxd
    m[7] = vpbroadcastb_p3(a_p3(tl, maxbase as isize)); // vpbroadcastb m7, [tlq+maxbaseq]
    let mut r3 = dx.wrapping_sub(64); // lea r3d, [dxq-64]
    maxbase <<= 6; // shl maxbased, 6
    m[12] = _mm256_broadcastw_epi16(lo(m[12])); // vpbroadcastw m12, xm12
    r3 = r3.wrapping_sub(maxbase); // sub r3d, maxbased
    m[8] = vbroadcasti128_p3(ro(dav1d_z_filter_s, 2)); // vbroadcasti128 m8, [z_filter_s+2]
    m[6] = movd_r_p3(r3); // movd xm6, r3d
    let mut r5 = dx; // mov r5d, dxd
    m[10] = y_p3(ro(pb_1to32, 0)); // mova m10, [pb_1to32]
    m[11] = vpbroadcastd_p3(ro(pb_32_p3, 0)); // vpbroadcastd m11, [pb_32]
    m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
    loop {
        // .w64_loop
        let mut r3 = r5; // mov r3d, r5d
        r3 >>= 6; // shr r3d, 6
        m[0] = y_p3(a_p3(tl, r3 as isize)); // movu m0, [tlq+r3+ 0]
        m[1] = y_p3(a_p3(tl, r3 as isize + 8)); // movu m1, [tlq+r3+ 8]
        m[2] = _mm256_and_si256(m[4], m[6]); // pand m2, m4, m6
        m[9] = _mm256_sub_epi16(m[5], m[2]); // psubw m9, m5, m2
        m[2] = _mm256_slli_epi16::<8>(m[2]); // psllw m2, 8
        m[9] = _mm256_or_si256(m[9], m[2]); // por m9, m2
        m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
        m[1] = _mm256_shuffle_epi8(m[1], m[8]); // pshufb m1, m8
        m[0] = _mm256_maddubs_epi16(m[0], m[9]); // pmaddubsw m0, m9
        m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
        m[2] = _mm256_srai_epi16::<6>(m[6]); // psraw m2, m6, 6
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        m[2] = _mm256_packs_epi16(m[2], m[2]); // packsswb m2, m2
        m[2] = _mm256_add_epi8(m[2], m[10]); // paddb m2, m10
        m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
        m[0] = _mm256_blendv_epi8(m[7], m[0], m[2]); // vpblendvb m0, m7, m0, m2
        _mm256_storeu_si256(dst.cast(), m[0]); // mova [dstq+ 0], m0
        m[0] = y_p3(a_p3(tl, r3 as isize + 32)); // movu m0, [tlq+r3+32]
        m[1] = y_p3(a_p3(tl, r3 as isize + 40)); // movu m1, [tlq+r3+40]
        r5 = r5.wrapping_add(dx); // add r5d, dxd
        m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
        m[1] = _mm256_shuffle_epi8(m[1], m[8]); // pshufb m1, m8
        m[0] = _mm256_maddubs_epi16(m[0], m[9]); // pmaddubsw m0, m9
        m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
        m[2] = _mm256_add_epi8(m[2], m[11]); // paddb m2, m11
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        m[6] = _mm256_add_epi16(m[6], m[12]); // paddw m6, m12
        m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
        m[0] = _mm256_blendv_epi8(m[7], m[0], m[2]); // vpblendvb m0, m7, m0, m2
        _mm256_storeu_si256(at(dst, 32).cast(), m[0]); // mova [dstq+32], m0
        h -= 1; // dec hd
        if h == 0 {
            return; // jz .w64_end
        }
        dst = at(dst, stride); // add dstq, strideq
        if r5 >= maxbase {
            break; // cmp r5d, maxbased / jb .w64_loop
        }
    }
    loop {
        // .w64_end_loop
        _mm256_storeu_si256(dst.cast(), m[7]); // mova [dstq+ 0], m7
        _mm256_storeu_si256(at(dst, 32).cast(), m[7]); // mova [dstq+32], m7
        dst = at(dst, stride); // add dstq, strideq
        h -= 1; // dec hd
        if h <= 0 {
            break; // jg .w64_end_loop
        }
    }
} // .w64_end: RET

/// `ipred_z1_8bpc`: the entry (`cglobal` up to `jmp wq`).
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_z1_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    tl: *const u8,
    w: c_int,
    h: c_int,
    angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    // Not in the original: the kernel loads up to `tl[198]`, behind the caller's array; it runs on a copy of the
    // pixels the C kernel reads, `tl[0..=w+h]`, with zeros around them (`copy_edge`).
    let mut edge = [0u8; 64 + 256];
    let tl = crate::simd::copy_edge(&mut edge, 64, tl, 0, (w + h) as isize);
    let mut m = regs();
    let m = &mut m;
    // lea r6, [ipred_z1_avx2_table]
    let wt = (w as u32).trailing_zeros(); // tzcnt wd, wm (w is 4..=64, a power of two: never zero)
    let mut angle = angle as u32; // movifnidn angled, anglem
    let h = h as i32; // movifnidn hd, hm
                      // lea r7, [dr_intra_derivative]
    let tl = tl.wrapping_add(1); // inc tlq
                                 // movsxd wq, [r6+wq*4] / add wq, r6: the `match` below
    let mut dx = angle; // mov dxd, angled
    dx &= 0x7e; // and dxd, 0x7e
    angle = angle.wrapping_add(165); // add angled, 165 ; ~90
                                     // movzx dxd, word [r7+dxq]: the contract keeps `angle & 511` in 3..=87 (z1), so the byte offset `dx` is at
                                     // most 86, inside the table's 88 bytes.
    let table = core::ptr::addr_of!(crate::common::tables::dav1d_dr_intra_derivative) as *const u8;
    dx = table.wrapping_add(dx as usize).cast::<u16>().read_unaligned() as u32;
    angle ^= 0x4ff; // xor angled, 0x4ff ; d = 90 - angle
    m[3] = vpbroadcastd_p3(ro(pw_512, 0)); // vpbroadcastd m3, [pw_512]
    m[4] = vpbroadcastd_p3(ro(pw_62, 0)); // vpbroadcastd m4, [pw_62]
    m[5] = vpbroadcastd_p3(ro(pw_64, 0)); // vpbroadcastd m5, [pw_64]
                                          // jmp wq (`ipred_z1_avx2_table`: w4, w8, w16, w32, w64; the contract keeps `w` in 4..=64, a power of two)
    match wt {
        2 => ipred_z1_8bpc_w4(m, dst, stride, tl, h, angle, dx),
        3 => ipred_z1_8bpc_w8(m, dst, stride, tl, h, angle, dx),
        4 => ipred_z1_8bpc_w16(m, dst, stride, tl, h, angle, dx),
        5 => ipred_z1_8bpc_w32(m, dst, stride, tl, h, angle, dx),
        6 => ipred_z1_8bpc_w64(m, dst, stride, tl, h, angle, dx),
        _ => {}
    }
}

// ---- part 4: `ipred_avx2.asm` lines 2132–2987 (z2) ----
//
// `cglobal ipred_z2_8bpc` is `dav1d_ipred_z2_8bpc_avx2`. Its general-purpose registers that live across labels are
// the fields of `GprP4` (named as `DEFINE_ARGS` and the assembly name them; `fp` is `rsp`), its `ALLOC_STACK` frame
// of 224 bytes is `FrameP4`. The jump table `ipred_z2_avx2_table` is the `match` on `tzcnt(w)` in the entry point.
// Labels: `.w4`, `.w8`, `.w16`, `.w32`, `.w64` up to their filter labels are `ipred_z2_8bpc_w{4,8,16,32,64}` (the
// edge preparation; each returns true where the assembly continues at `.w4_filter_left`, `.w8_filter_left`,
// `.w16_filter_left` or, for w32 and w64, `.w32_filter_above` and then `.w16_filter_left`, false where it jumps to
// the main label); `.w4_filter_left`, `.w8_filter_left`, `.w16_filter_left` and `.w32_filter_above` (labels jumped
// to from several places) are functions of their own, called at one site of the entry point, `.w4_main`/`.w8_main`/`.w16_main` with their
// loops (`.wN_loop`, `.wN_toponly`, `.wN_leftonly_loop`, `.w16_loop0`, `.w16_end`) are `ipred_z2_8bpc_wN_main`,
// and the `call`ed subroutines `.filter_strength`, `.upsample_above`, `.upsample_left` are
// `ipred_z2_8bpc_filter_strength`, `_upsample_above`, `_upsample_left` (their `[rsp+gprsize+N]` is the frame at
// `N`). Every function but the exported one is `#[inline(always)]` without `#[target_feature]`.

/// The general-purpose registers of `ipred_z2_8bpc` that carry values across its labels, and `rsp`.
struct GprP4 {
    /// `dstq`.
    dst: *mut u8,
    /// `strideq`.
    stride: isize,
    /// `tlq`.
    tl: *const u8,
    /// `hd` (in `.w16_main`, the rows left in the current 16-pixel column).
    h: i32,
    /// `angled` (with the flags; `xor`ed with 0x400 at the entry).
    angle: i32,
    /// `dxd` (negated at the entry, doubled with an upsampled top edge).
    dx: i32,
    /// `dyd` (negated at the entry); from `.w4`/`.w8` on `dyq`, the step of `r5` (-4, or -8 with an upsampled
    /// left edge).
    dy: i32,
    /// `r2d`: the x position (`xpos`) of the next row in `.w4_main`/`.w8_main`/`.w16_main`.
    r2: i32,
    /// `r3d`.
    r3: i32,
    /// `r5`: the left edge pointer of the gathers (`left-7`, `left-3`) in the frame. A pointer, as in the assembly:
    /// as an offset from `rsp` the compiler adds `rsp` to it before every gather (two instructions).
    r5: *mut u8,
    /// `r8d`: the x position below which the rows use the left edge only (w4, w8); rows and columns left (w16+).
    r8: i32,
    /// `r6m`.
    max_width: c_int,
    /// `r7m`.
    max_height: c_int,
    /// `rsp`: the frame's first byte.
    fp: *mut u8,
}

/// The `ALLOC_STACK` frame of `ipred_z2_8bpc` (224 bytes at `rsp`) with 32 bytes below it: in `.w16_main`'s last
/// rows at h = 64 a gathered dword starts at `rsp-1` (`r5 = rsp-2`, index at least 1; its first byte is never
/// used). Written whole at the entry (the 32 bytes below `rsp` and `rsp+96..224` zero, `rsp+0..96` the copy of the
/// edge): the gathers of `.w4_main`/`.w8_main` read up to `rsp+192` and the top loads of the 64-pixel-wide blocks
/// up to `rsp+136`, bytes the assembly never writes (and discards).
#[repr(C, align(32))]
struct FrameP4([u8; 32 + 224]);

// `%define`s of the read-only data (`ipred_avx2.asm` lines 129–145) that this part uses.
const pb_0to15_p4: usize = cfl_ac_w16_pad_shuffle;
const pb_4_p4: usize = ipred_h_shuf + 24;
const pb_5_p4: usize = ipred_h_shuf + 8;
const pb_8_p4: usize = z_upsample2 + 12;
const pb_32_p4: usize = z2_y_shuf_h4 + 16;
const pb_90_p4: usize = z2_y_shuf_h4;

/// `[base+z_filter_k-4+r3*4+12*k]`: `r3` is a filter strength 1..=3 (the `popcnt` of a mask in which at most three
/// bytes of `z_filter_wh` match), so the address lies in `z_filter_k`.
#[inline(always)]
fn zk_p4(r3: i32, k: isize) -> *const u8 {
    ro(z_filter_k, r3 as isize * 4 - 4 + 12 * k)
}

#[inline(always)]
unsafe fn ld128_p4(p: *const u8) -> __m128i {
    _mm_loadu_si128(p as *const __m128i)
}

#[inline(always)]
unsafe fn ld256_p4(p: *const u8) -> __m256i {
    _mm256_loadu_si256(p as *const __m256i)
}

/// `vbroadcasti128 m, [p]`.
#[inline(always)]
unsafe fn bci128_p4(p: *const u8) -> __m256i {
    _mm256_broadcastsi128_si256(ld128_p4(p))
}

/// `vinserti128 m, a, [p], 1`.
#[inline(always)]
unsafe fn ins1_p4(a: __m256i, p: *const u8) -> __m256i {
    _mm256_inserti128_si256::<1>(a, ld128_p4(p))
}

/// `vpbroadcastd m, [p]`.
#[inline(always)]
unsafe fn bcd_p4(p: *const u8) -> __m256i {
    _mm256_set1_epi32((p as *const i32).read_unaligned())
}

/// `vpbroadcastd xm, [p]`.
#[inline(always)]
unsafe fn bcdx_p4(p: *const u8) -> __m128i {
    _mm_set1_epi32((p as *const i32).read_unaligned())
}

#[inline(always)]
unsafe fn rd32_p4(p: *const u8) -> i32 {
    (p as *const i32).read_unaligned()
}

#[inline(always)]
unsafe fn rd64_p4(p: *const u8) -> i64 {
    (p as *const i64).read_unaligned()
}

#[inline(always)]
unsafe fn st32_p4(p: *mut u8, v: i32) {
    (p as *mut i32).write_unaligned(v)
}

#[inline(always)]
unsafe fn st128_p4(p: *mut u8, v: __m128i) {
    _mm_storeu_si128(p as *mut __m128i, v)
}

#[inline(always)]
unsafe fn st256_p4(p: *mut u8, v: __m256i) {
    _mm256_storeu_si256(p as *mut __m256i, v)
}

/// `movq [p], xm`.
#[inline(always)]
unsafe fn stq_p4(p: *mut u8, v: __m128i) {
    _mm_storel_epi64(p as *mut __m128i, v)
}

/// `movhps [p], xm`.
#[inline(always)]
unsafe fn sthq_p4(p: *mut u8, v: __m128i) {
    (p as *mut i64).write_unaligned(_mm_extract_epi64::<1>(v))
}

/// `shufps xm, a, b, I`.
#[inline(always)]
unsafe fn shufpsx_p4<const I: i32>(a: __m128i, b: __m128i) -> __m128i {
    _mm_castps_si128(_mm_shuffle_ps::<I>(_mm_castsi128_ps(a), _mm_castsi128_ps(b)))
}

/// `shufps m, a, b, I`.
#[inline(always)]
unsafe fn shufps_p4<const I: i32>(a: __m256i, b: __m256i) -> __m256i {
    _mm256_castps_si256(_mm256_shuffle_ps::<I>(_mm256_castsi256_ps(a), _mm256_castsi256_ps(b)))
}

/// `popcnt r3d, r3d`. POPCNT is its own CPUID feature, which the original's AVX2 selection does not check; the
/// count is the same.
#[inline(always)]
fn popcnt_p4(x: i32) -> i32 {
    x.count_ones() as i32
}

/// `ipred_z2_8bpc` (`ipred_avx2.asm` lines 2132–2986).
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_z2_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    angle: c_int,
    max_width: c_int,
    max_height: c_int,
) {
    // ALLOC_STACK 224 (see `FrameP4`); `fp` is `rsp`. Written whole below, before any load: the 32 bytes below
    // `rsp` and `rsp+96..224` here, `rsp+0..96` by the three stores of the edge.
    let mut frame = core::mem::MaybeUninit::<FrameP4>::uninit();
    let fp = (frame.as_mut_ptr() as *mut u8).add(32);
    for off in [-32, 96, 128, 160, 192] {
        st256_p4(at(fp, off), _mm256_setzero_si256());
    }
    let mut mm = regs();
    let m = &mut mm;
    let mut g = GprP4 {
        dst,
        stride,
        tl: topleft,
        h: height,
        angle,
        dx: 0,
        dy: 0,
        r2: 0,
        r3: 0,
        r5: fp,
        r8: 0,
        max_width,
        max_height,
        fp,
    };
    let g = &mut g;
    let w = width.trailing_zeros(); // tzcnt wd, wm (w is a power of two, 4 to 64: never zero)
    let mut dxq = (core::ptr::addr_of!(crate::common::tables::dav1d_dr_intra_derivative) as *const u8).wrapping_sub(90); // lea dxq, [dr_intra_derivative-90]
    g.dy = angle & 0xff; // movzx dyd, angleb
    g.angle ^= 0x400; // xor angled, 0x400
    let r8 = dxq; // mov r8, dxq
    dxq = dxq.wrapping_sub(g.dy as usize); // sub dxq, dyq
    m[2] = ld256_p4(g.tl.wrapping_offset(-64)); // mova m2, [tlq-64]
    m[0] = ld256_p4(g.tl.wrapping_offset(-32)); // mova m0, [tlq-32]
    m[1] = ld256_p4(g.tl); // mova m1, [tlq]
    g.dy &= !1; // and dyd, ~1
    dxq = dxq.map_addr(|a| a & !1); // and dxq, ~1
    g.dy = (r8.wrapping_add(g.dy as usize) as *const u16).read_unaligned() as i32; // movzx dyd, word [r8+dyq] ; angle - 90 (angle 91..=179: inside the table)
    g.dx = (dxq.wrapping_add(270) as *const u16).read_unaligned() as i32; // movzx dxd, word [dxq+270] ; 180 - angle (inside the table)
    m[13] = bcd_p4(ro(pw_512, 0)); // vpbroadcastd m13, [base+pw_512]
    m[14] = bcd_p4(ro(pw_62, 0)); // vpbroadcastd m14, [base+pw_62]
    m[15] = bcd_p4(ro(pw_64, 0)); // vpbroadcastd m15, [base+pw_64]
    st256_p4(at(fp, 0), m[2]); // mova [rsp+ 0], m2
    st256_p4(at(fp, 32), m[0]); // mova [rsp+32], m0
    st256_p4(at(fp, 64), m[1]); // mova [rsp+64], m1
    g.dx = -g.dx; // neg dxd
    g.dy = -g.dy; // neg dyd
    match w {
        // jmp wq (each label reached from several places is called at one site)
        2 => {
            if ipred_z2_8bpc_w4(m, g) {
                ipred_z2_8bpc_w4_filter_left(m, g);
            }
            ipred_z2_8bpc_w4_main(m, g);
        }
        3 => {
            if ipred_z2_8bpc_w8(m, g) {
                ipred_z2_8bpc_w8_filter_left(m, g);
            }
            ipred_z2_8bpc_w8_main(m, g);
        }
        _ => {
            let filter_left = if w == 4 {
                ipred_z2_8bpc_w16(m, g)
            } else {
                let filter_above = if w == 5 {
                    ipred_z2_8bpc_w32(m, g)
                } else {
                    ipred_z2_8bpc_w64(m, g)
                };
                if filter_above {
                    ipred_z2_8bpc_w32_filter_above(m, g);
                }
                filter_above
            };
            if filter_left {
                ipred_z2_8bpc_w16_filter_left(m, g);
            }
            ipred_z2_8bpc_w16_main(m, g);
        }
    }
}

/// `.filter_strength` (`call`ed): `r3d` = w + h (+ an offset) in, the mask of `z_filter_t0`/`z_filter_t1` out;
/// leaves `m7` (angle), `m8` (the `z_filter_wh` match) and `m9` (the thresholds) for the left edge's test.
#[inline(always)]
unsafe fn ipred_z2_8bpc_filter_strength(m: &mut M, g: &mut GprP4) {
    m[8] = zx(_mm_cvtsi32_si128(g.r3)); // movd xm8, r3d
    g.r3 = g.angle; // mov r3d, angled
    m[7] = zx(_mm_cvtsi32_si128(g.angle)); // movd xm7, angled
    m[8] = _mm256_broadcastb_epi8(lo(m[8])); // vpbroadcastb m8, xm8
    g.r3 = ((g.r3 as u32) >> 8) as i32; // shr r3d, 8 ; is_sm << 1
    m[7] = _mm256_broadcastb_epi8(lo(m[7])); // vpbroadcastb m7, xm7
    m[8] = _mm256_cmpeq_epi8(m[8], ld256_p4(ro(z_filter_wh, 0))); // pcmpeqb m8, [base+z_filter_wh]
                                                                  // r3 is 0 or 2 (the angle without the edge flag is below 0x400): z_filter_t0 or z_filter_t1.
    m[9] = zx(ld128_p4(ro(z_filter_t0, g.r3 as isize * 8))); // mova xm9, [r9+r3*8]
    m[0] = _mm256_and_si256(m[8], m[7]); // pand m0, m8, m7
    m[0] = _mm256_cmpgt_epi8(m[0], m[9]); // pcmpgtb m0, m9
    g.r3 = _mm256_movemask_epi8(m[0]); // pmovmskb r3d, m0
}

/// `.upsample_above` (`call`ed; w4/w8).
#[inline(always)]
unsafe fn ipred_z2_8bpc_upsample_above(m: &mut M, g: &mut GprP4) {
    m[2] = zx(_mm_shuffle_epi8(lo(m[1]), ld128_p4(ro(z_upsample1, -2)))); // pshufb xm2, xm1, [base+z_upsample1-2]
    m[3] = zx(_mm_min_epu8(lo(m[3]), ld128_p4(ro(dav1d_z_filter_s, 4)))); // pminub xm3, [base+z_filter_s+4]
    m[4] = zx(bcdx_p4(ro(pb_36_m4, 0))); // vpbroadcastd xm4, [base+pb_36_m4]
    m[10] = bci128_p4(ro(pb_0to15_p4, 0)); // vbroadcasti128 m10, [base+pb_0to15]
    m[3] = zx(_mm_shuffle_epi8(lo(m[1]), lo(m[3]))); // pshufb xm3, xm1, xm3
    m[2] = zx(_mm_maddubs_epi16(lo(m[2]), lo(m[4]))); // pmaddubsw xm2, xm4
    m[3] = zx(_mm_maddubs_epi16(lo(m[3]), lo(m[4]))); // pmaddubsw xm3, xm4
    g.r2 = g.r2 + g.dx + (1 << 6); // lea r2d, [r2+dxq+(1<<6)]
    g.dx += g.dx; // add dxd, dxd
    m[2] = zx(_mm_add_epi16(lo(m[2]), lo(m[3]))); // paddw xm2, xm3
    m[2] = zx(_mm_mulhrs_epi16(lo(m[2]), lo(m[13]))); // pmulhrsw xm2, xm13
    g.r8 -= 3 << 6; // sub r8d, 3<<6
    m[6] = _mm256_add_epi16(m[6], m[6]); // paddw m6, m6
    m[2] = zx(_mm_packus_epi16(lo(m[2]), lo(m[2]))); // packuswb xm2, xm2
    m[1] = zx(_mm_unpacklo_epi8(lo(m[1]), lo(m[2]))); // punpcklbw xm1, xm2
    st128_p4(at(g.fp, 64), lo(m[1])); // mova [rsp+gprsize+64], xm1
}

/// `.upsample_left` (`call`ed; h4/h8).
#[inline(always)]
unsafe fn ipred_z2_8bpc_upsample_left(m: &mut M, g: &mut GprP4) {
    g.r3 = g.h; // mov r3d, hd
    g.r3 &= 4; // and r3d, 4
    m[2] = zx(_mm_cvtsi32_si128(rd32_p4(at(g.fp, 64)))); // movd xm2, [rsp+gprsize+64]
    m[0] = zx(_mm_set1_epi64x(rd64_p4(at(g.fp, 56)))); // movddup xm0, [rsp+gprsize+56]
    m[1] = zx(_mm_cvtsi32_si128(g.r3)); // movd xm1, r3d
    m[2] = zx(_mm_alignr_epi8::<1>(lo(m[2]), lo(m[0]))); // palignr xm2, xm0, 1
    m[1] = zx(_mm_broadcastb_epi8(lo(m[1]))); // vpbroadcastb xm1, xm1
    m[2] = zx(_mm_shuffle_epi8(lo(m[2]), ld128_p4(ro(dav1d_z_filter_s, 18)))); // pshufb xm2, [base+z_filter_s+18]
    m[3] = zx(bcdx_p4(ro(pb_36_m4, 0))); // vpbroadcastd xm3, [base+pb_36_m4]
    m[1] = zx(_mm_max_epu8(lo(m[1]), ld128_p4(ro(z_upsample1, -2)))); // pmaxub xm1, [base+z_upsample1-2]
    m[1] = zx(_mm_shuffle_epi8(lo(m[0]), lo(m[1]))); // pshufb xm1, xm0, xm1
    m[2] = zx(_mm_maddubs_epi16(lo(m[2]), lo(m[3]))); // pmaddubsw xm2, xm3
    m[1] = zx(_mm_maddubs_epi16(lo(m[1]), lo(m[3]))); // pmaddubsw xm1, xm3
    m[5] = zx(_mm_add_epi16(lo(m[5]), lo(m[5]))); // paddw xm5, xm5
    g.dy += g.dy; // add dyq, dyq
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[2]))); // paddw xm1, xm2
    m[1] = zx(_mm_mulhrs_epi16(lo(m[1]), lo(m[13]))); // pmulhrsw xm1, xm13
    m[11] = bci128_p4(ro(z2_upsample, 0)); // vbroadcasti128 m11, [base+z2_upsample]
    m[5] = zx(_mm_add_epi16(lo(m[5]), lo(m[15]))); // paddw xm5, xm15
    m[1] = zx(_mm_packus_epi16(lo(m[1]), lo(m[1]))); // packuswb xm1, xm1
    m[0] = zx(_mm_unpacklo_epi8(lo(m[0]), lo(m[1]))); // punpcklbw xm0, xm1
    st128_p4(at(g.fp, 48), lo(m[0])); // mova [rsp+gprsize+48], xm0
}

/// `.w4` up to `.w4_filter_left`; true where the assembly continues at `.w4_filter_left`, false where it
/// continues at `.w4_main`.
#[inline(always)]
unsafe fn ipred_z2_8bpc_w4(m: &mut M, g: &mut GprP4) -> bool {
    m[6] = _mm256_set1_epi64x(rd64_p4(ro(z2_base_inc, 0))); // vpbroadcastq m6, [base+z2_base_inc] ; base_inc << 6
    m[10] = bci128_p4(ro(z1_shuf_w4, 0)); // vbroadcasti128 m10, [base+z1_shuf_w4]
    m[11] = bci128_p4(ro(z2_shuf_h4, 0)); // vbroadcasti128 m11, [base+z2_shuf_h4]
    g.r2 = g.dx + (65 << 6); // lea r2d, [dxq+(65<<6)] ; xpos
    m[5] = zx(_mm_cvtsi32_si128(g.dy)); // movd xm5, dyd
    g.r8 = (63 - 4) << 6; // mov r8d, (63-4)<<6
    g.dy = -4; // mov dyq, -4
    m[5] = zx(_mm_shufflelo_epi16::<0x00>(lo(m[5]))); // pshuflw xm5, xm5, q0000
    m[5] = zx(_mm_mullo_epi16(lo(m[5]), ld128_p4(ro(z2_ymul, 0)))); // pmullw xm5, [base+z2_ymul]
    if g.angle & 0x400 != 0 {
        return false; // test angled, 0x400 ; jnz .w4_main ; !enable_intra_edge_filter
    }
    g.r3 = g.h + 2; // lea r3d, [hq+2]
    g.angle += 1022; // add angled, 1022
    g.r3 <<= 6; // shl r3d, 6
    if g.r3 & g.angle == 0 {
        // test r3d, angled ; jnz .w4_no_upsample_above ; angle >= 130 || h > 8 || (is_sm && h == 8)
        m[3] = zx(bcdx_p4(ro(pb_4_p4, 0))); // vpbroadcastd xm3, [base+pb_4]
        ipred_z2_8bpc_upsample_above(m, g); // call .upsample_above
        g.angle -= 1075; // sub angled, 1075 ; angle - 53
        g.r3 = g.h + 3; // lea r3d, [hq+3]
        g.angle ^= 0x7f; // xor angled, 0x7f ; 180 - angle
        ipred_z2_8bpc_filter_strength(m, g); // call .filter_strength
        return true; // jmp .w4_filter_left
    }
    // .w4_no_upsample_above:
    g.r3 = g.h + 3; // lea r3d, [hq+3]
    g.angle -= 1112; // sub angled, 1112 ; angle - 90
    ipred_z2_8bpc_filter_strength(m, g); // call .filter_strength
    if g.r3 != 0 {
        // test r3d, r3d ; jz .w4_no_filter_above
        g.r3 = popcnt_p4(g.r3); // popcnt r3d, r3d
        m[2] = zx(bcdx_p4(ro(pb_4_p4, 0))); // vpbroadcastd xm2, [base+pb_4]
        m[2] = zx(_mm_min_epu8(lo(m[2]), ld128_p4(ro(dav1d_z_filter_s, 0)))); // pminub xm2, [base+z_filter_s]
        m[0] = zx(bcdx_p4(zk_p4(g.r3, 0))); // vpbroadcastd xm0, [base+z_filter_k-4+r3*4+12*0]
        m[4] = zx(bcdx_p4(zk_p4(g.r3, 1))); // vpbroadcastd xm4, [base+z_filter_k-4+r3*4+12*1]
        m[3] = zx(_mm_shuffle_epi8(lo(m[1]), lo(m[2]))); // pshufb xm3, xm1, xm2 ; 00 01 12 23
        m[2] = zx(_mm_shuffle_epi32::<0x39>(lo(m[2]))); // pshufd xm2, xm2, q0321
        m[0] = zx(_mm_maddubs_epi16(lo(m[3]), lo(m[0]))); // pmaddubsw xm0, xm3, xm0
        m[2] = zx(_mm_shuffle_epi8(lo(m[1]), lo(m[2]))); // pshufb xm2, xm1, xm2 ; 12 23 34 44
        m[2] = zx(_mm_maddubs_epi16(lo(m[2]), lo(m[4]))); // pmaddubsw xm2, xm4
        m[4] = zx(bcdx_p4(zk_p4(g.r3, 2))); // vpbroadcastd xm4, [base+z_filter_k-4+r3*4+12*2]
        m[3] = zx(_mm_unpackhi_epi64(lo(m[3]), lo(m[3]))); // punpckhqdq xm3, xm3 ; 34 44 44 44
        m[3] = zx(_mm_maddubs_epi16(lo(m[3]), lo(m[4]))); // pmaddubsw xm3, xm4
        m[4] = zx(_mm_set1_epi32(g.max_width)); // vpbroadcastd xm4, r6m ; max_width
        m[4] = zx(_mm_packs_epi32(lo(m[4]), lo(m[4]))); // packssdw xm4, xm4
        m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); // paddw xm0, xm2
        m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[3]))); // paddw xm0, xm3
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[13]))); // pmulhrsw xm0, xm13
        m[4] = zx(_mm_packs_epi16(lo(m[4]), lo(m[4]))); // packsswb xm4, xm4
        m[1] = zx(_mm_srli_epi64::<8>(lo(m[1]))); // psrlq xm1, 8
        m[4] = zx(_mm_sub_epi8(lo(m[4]), ld128_p4(ro(pb_1to32, 0)))); // psubb xm4, [base+pb_1to32]
        m[0] = zx(_mm_packus_epi16(lo(m[0]), lo(m[0]))); // packuswb xm0, xm0
        m[0] = zx(_mm_blendv_epi8(lo(m[0]), lo(m[1]), lo(m[4]))); // vpblendvb xm0, xm1, xm4
        st32_p4(at(g.fp, 65), _mm_cvtsi128_si32(lo(m[0]))); // movd [rsp+65], xm0
    }
    // .w4_no_filter_above:
    g.r3 = g.h + 2; // lea r3d, [hq+2]
    g.angle += 973; // add angled, 973 ; angle + 883
    g.r3 <<= 6; // shl r3d, 6
    if g.r3 & g.angle == 0 {
        // test r3d, angled ; jz .w4_upsample_left ; angle <= 140 || h > 8 || (is_sm && h == 8)
        // .w4_upsample_left:
        ipred_z2_8bpc_upsample_left(m, g); // call .upsample_left
        return false; // falls into .w4_main
    }
    m[0] = zx(bcdx_p4(ro(pb_90_p4, 0))); // vpbroadcastd xm0, [base+pb_90]
    m[0] = zx(_mm_sub_epi8(lo(m[0]), lo(m[7]))); // psubb xm0, xm7 ; 180 - angle
    m[0] = zx(_mm_and_si128(lo(m[0]), lo(m[8]))); // pand xm0, xm8 ; reuse from previous filter_strength call
    m[0] = zx(_mm_cmpgt_epi8(lo(m[0]), lo(m[9]))); // pcmpgtb xm0, xm9
    g.r3 = _mm_movemask_epi8(lo(m[0])); // pmovmskb r3d, xm0
    true
}

/// `.w4_filter_left` up to `jmp .w4_main`.
#[inline(always)]
unsafe fn ipred_z2_8bpc_w4_filter_left(m: &mut M, g: &mut GprP4) {
    if g.r3 == 0 {
        return; // test r3d, r3d ; jz .w4_main
    }
    g.r3 = popcnt_p4(g.r3); // popcnt r3d, r3d
    let mut r5 = 10; // mov r5d, 10
    let h_lt_16 = g.h - 16 < 0; // cmp hd, 16
    m[2] = zx(ld128_p4(at(g.fp, 49))); // movu xm2, [rsp+49]
    m[2] = ins1_p4(m[2], at(g.fp, 43)); // vinserti128 m2, [rsp+43], 1
    if h_lt_16 {
        r5 = g.h; // cmovs r5d, hd
    }
    r5 ^= 15; // xor r5d, 15 ; h == 16 ? 5 : 15 - h
    m[0] = zx(_mm_cvtsi32_si128(r5)); // movd xm0, r5d
    m[1] = bci128_p4(ro(dav1d_z_filter_s, 12)); // vbroadcasti128 m1, [base+z_filter_s+12]
    m[4] = bci128_p4(ro(dav1d_z_filter_s, 16)); // vbroadcasti128 m4, [base+z_filter_s+16]
    m[3] = ins1_p4(m[1], ro(dav1d_z_filter_s, 8)); // vinserti128 m3, m1, [z_filter_s+8], 1
    m[1] = _mm256_blend_epi32::<0x0f>(m[1], m[4]); // vpblendd m1, m4, 0x0f
    m[4] = _mm256_inserti128_si256::<0>(m[4], ld128_p4(ro(dav1d_z_filter_s, 20))); // vinserti128 m4, [base+z_filter_s+20], 0
    m[0] = _mm256_broadcastb_epi8(lo(m[0])); // vpbroadcastb m0, xm0
    m[0] = _mm256_max_epu8(m[0], m[3]); // pmaxub m0, m3
    m[3] = bcd_p4(zk_p4(g.r3, 0)); // vpbroadcastd m3, [base+z_filter_k-4+r3*4+12*0]
    m[0] = _mm256_shuffle_epi8(m[2], m[0]); // pshufb m0, m2, m0
    m[0] = _mm256_maddubs_epi16(m[0], m[3]); // pmaddubsw m0, m3
    m[3] = bcd_p4(zk_p4(g.r3, 1)); // vpbroadcastd m3, [base+z_filter_k-4+r3*4+12*1]
    m[1] = _mm256_shuffle_epi8(m[2], m[1]); // pshufb m1, m2, m1
    m[1] = _mm256_maddubs_epi16(m[1], m[3]); // pmaddubsw m1, m3
    m[3] = bcd_p4(zk_p4(g.r3, 2)); // vpbroadcastd m3, [base+z_filter_k-4+r3*4+12*2]
    m[2] = _mm256_shuffle_epi8(m[2], m[4]); // pshufb m2, m4
    m[2] = _mm256_maddubs_epi16(m[2], m[3]); // pmaddubsw m2, m3
    m[4] = zx(_mm_set1_epi32(g.max_height)); // vpbroadcastd xm4, r7m ; max_height
    m[4] = zx(_mm_packs_epi32(lo(m[4]), lo(m[4]))); // packssdw xm4, xm4
    m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[1] = _mm256_mulhrs_epi16(m[1], m[13]); // pmulhrsw m1, m13
    m[4] = zx(_mm_packs_epi16(lo(m[4]), lo(m[4]))); // packsswb xm4, xm4
    m[0] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm0, m1, 1
    m[4] = zx(_mm_sub_epi8(lo(m[4]), ld128_p4(ro(pb_16to1, 0)))); // psubb xm4, [base+pb_16to1]
    m[0] = zx(_mm_packus_epi16(lo(m[0]), lo(m[1]))); // packuswb xm0, xm1
    m[0] = zx(_mm_blendv_epi8(lo(m[0]), ld128_p4(at(g.fp, 48)), lo(m[4]))); // vpblendvb xm0, [rsp+48], xm4
    st128_p4(at(g.fp, 48), lo(m[0])); // mova [rsp+48], xm0
} // jmp .w4_main

/// The gather indices of `.w4_main`/`.w8_main` from the four `base_y` words in the low half of `x`: their
/// dwords, sign-extended (`pmovsxwd`). The assembly zero-extends them (`punpcklwd` with a zero register). The
/// two agree for every `base_y` in `0..=0x7fff`, and `base_y` is `-((dy * k) >> 6)` (`k` = 1..=8, `dy` negated
/// at the entry), or `-((2 * dy * k + 64) >> 6)` after `.upsample_left`: at least 0 whenever the derivative of
/// the angle (`dav1d_dr_intra_derivative`) is not 0, which holds for every AV1 angle. The table holds 0 for the
/// angles AV1 does not have ("Values that are 0 will never be used", src/tables.c). With such an angle and the
/// upsampled left edge, `base_y` is -1: zero-extended, the index 65535 makes the assembly's gather read 8
/// bytes 64 KiB above its frame on the stack, which faults where nothing is mapped there (the unit tests'
/// angles include such angles, `crate::simd::edge_test::check`); sign-extended, the gather reads `r5-1`
/// inside the frame. Not in the original.
#[inline(always)]
unsafe fn gather_index_p4(x: __m128i) -> __m128i {
    _mm_cvtepi16_epi32(x)
}

/// `.w4_main` with `.w4_loop`, `.w4_toponly`, `.w4_leftonly_loop` and `.w4_end`.
#[inline(always)]
unsafe fn ipred_z2_8bpc_w4_main(m: &mut M, g: &mut GprP4) {
    let fp = g.fp;
    m[0] = zx(_mm_cvtsi32_si128(g.dx)); // movd xm0, dxd
    m[12] = ld256_p4(ro(z2_y_shuf_h4, 0)); // mova m12, [base+z2_y_shuf_h4]
    g.r5 = at(fp, 56); // lea r5, [rsp+56] ; left-7
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
    let r9 = g.stride * 3; // lea r9, [strideq*3]
    m[1] = zx(_mm_srai_epi16::<6>(lo(m[5]))); // psraw xm1, xm5, 6
    m[5] = zx(_mm_and_si128(lo(m[5]), lo(m[14]))); // pand xm5, xm14 ; frac_y
    m[2] = zx(_mm_setzero_si128()); // pxor xm2, xm2
    m[7] = _mm256_add_epi16(m[0], m[0]); // paddw m7, m0, m0
    m[4] = zx(_mm_sub_epi16(lo(m[2]), lo(m[1]))); // psubw xm4, xm2, xm1 ; base_y
    m[0] = _mm256_blend_epi32::<0xcc>(m[0], m[7]); // vpblendd m0, m7, 0xcc
    m[1] = zx(lo(m[7])); // mova xm1, xm7
    // punpcklwd xm4, xm2: the assembly zero-extends base_y to the gather indices; the rebuild sign-extends it
    // (`gather_index_p4`), the same indices for every AV1 angle (see there).
    m[4] = zx(gather_index_p4(lo(m[4]))); // punpcklwd xm4, xm2
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1 ; xpos2 xpos3 xpos0 xpos1
    m[1] = zx(_mm_sub_epi16(lo(m[15]), lo(m[5]))); // psubw xm1, xm15, xm5 ; 64-frac_y
    m[5] = zx(_mm_slli_epi16::<8>(lo(m[5]))); // psllw xm5, 8
    m[7] = _mm256_add_epi16(m[7], m[7]); // paddw m7, m7
    m[6] = _mm256_add_epi16(m[6], m[0]); // paddw m6, m0
    m[5] = zx(_mm_or_si128(lo(m[5]), lo(m[1]))); // por xm5, xm1 ; 64-frac_y, frac_y
    m[5] = _mm256_broadcastq_epi64(lo(m[5])); // vpbroadcastq m5, xm5
    loop {
        // .w4_loop:
        g.r3 = g.r2 + g.dx; // lea r3d, [r2+dxq]
        g.r2 = ((g.r2 as u32) >> 6) as i32; // shr r2d, 6 ; base_x0
        m[1] = _mm256_set1_epi64x(rd64_p4(at(fp, g.r2 as isize))); // vpbroadcastq m1, [rsp+r2]
        g.r2 = g.r3 + g.dx; // lea r2d, [r3+dxq]
        g.r3 = ((g.r3 as u32) >> 6) as i32; // shr r3d, 6 ; base_x1
        m[2] = _mm256_set1_epi64x(rd64_p4(at(fp, g.r3 as isize))); // vpbroadcastq m2, [rsp+r3]
        g.r3 = g.r2 + g.dx; // lea r3d, [r2+dxq]
        g.r2 = ((g.r2 as u32) >> 6) as i32; // shr r2d, 6 ; base_x2
        m[0] = zx(_mm_loadl_epi64(at(fp, g.r2 as isize) as *const __m128i)); // movq xm0, [rsp+r2]
        g.r2 = g.r3 + g.dx; // lea r2d, [r3+dxq]
        g.r3 = ((g.r3 as u32) >> 6) as i32; // shr r3d, 6 ; base_x3
        m[0] = zx(_mm_unpacklo_epi64(
            lo(m[0]),
            _mm_loadl_epi64(at(fp, g.r3 as isize) as *const __m128i),
        )); // movhps xm0, [rsp+r3]
        m[1] = _mm256_blend_epi32::<0xc0>(m[1], m[2]); // vpblendd m1, m2, 0xc0
        m[2] = _mm256_and_si256(m[14], m[6]); // pand m2, m14, m6 ; frac_x
        m[0] = _mm256_blend_epi32::<0xf0>(m[0], m[1]); // vpblendd m0, m1, 0xf0
        m[1] = _mm256_sub_epi16(m[15], m[2]); // psubw m1, m15, m2 ; 64-frac_x
        m[2] = _mm256_slli_epi16::<8>(m[2]); // psllw m2, 8
        m[0] = _mm256_shuffle_epi8(m[0], m[10]); // pshufb m0, m10
        m[1] = _mm256_or_si256(m[1], m[2]); // por m1, m2 ; 64-frac_x, frac_x
        m[0] = _mm256_maddubs_epi16(m[0], m[1]); // pmaddubsw m0, m1
        if g.r3 < 64 {
            // cmp r3d, 64 ; jge .w4_toponly
            m[1] = m[7]; // mova m1, m7 ; arbitrary negative value
                         // All four lanes are gathered (the mask is negative); the frame is zeroed whole and the
                         // indices stay within rsp+28..rsp+192 (see FrameP4). The gather clears m1; the next line writes it.
            m[3] = _mm256_mask_i32gather_epi64::<1>(m[3], g.r5 as *const i64, lo(m[4]), m[1]); // vpgatherdq m3, [r5+xm4], m1
            m[1] = _mm256_shuffle_epi8(m[3], m[11]); // pshufb m1, m3, m11
            m[1] = _mm256_permutevar8x32_epi32(m[1], m[12]); // vpermd m1, m12, m1
            m[1] = _mm256_maddubs_epi16(m[1], m[5]); // pmaddubsw m1, m5
            m[2] = _mm256_srai_epi16::<15>(m[6]); // psraw m2, m6, 15 ; base_x < topleft
            m[0] = _mm256_blendv_epi8(m[0], m[1], m[2]); // vpblendvb m0, m1, m2
        }
        // .w4_toponly:
        m[0] = _mm256_mulhrs_epi16(m[0], m[13]); // pmulhrsw m0, m13
        m[6] = _mm256_add_epi16(m[6], m[7]); // paddw m6, m7 ; xpos += dx
        g.r5 = at(g.r5, g.dy as isize); // add r5, dyq
        m[0] = _mm256_packus_epi16(m[0], m[0]); // packuswb m0, m0
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        st32_p4(at(g.dst, g.stride * 2), _mm_cvtsi128_si32(lo(m[0]))); // movd [dstq+strideq*2], xm0
        st32_p4(at(g.dst, r9), _mm_extract_epi32::<1>(lo(m[0]))); // pextrd [dstq+r9], xm0, 1
        st32_p4(g.dst, _mm_cvtsi128_si32(lo(m[1]))); // movd [dstq+strideq*0], xm1
        st32_p4(at(g.dst, g.stride), _mm_extract_epi32::<1>(lo(m[1]))); // pextrd [dstq+strideq*1], xm1, 1
        g.h -= 4; // sub hd, 4
        if g.h == 0 {
            return; // jz .w4_end ; RET
        }
        g.dst = at(g.dst, g.stride * 4); // lea dstq, [dstq+strideq*4]
        if g.r2 < g.r8 {
            break; // cmp r2d, r8d ; jge .w4_loop
        }
    }
    loop {
        // .w4_leftonly_loop:
        m[1] = m[7]; // mova m1, m7
        m[2] = _mm256_mask_i32gather_epi64::<1>(m[2], g.r5 as *const i64, lo(m[4]), m[1]); // vpgatherdq m2, [r5+xm4], m1
        g.r5 = at(g.r5, g.dy as isize); // add r5, dyq
        m[0] = _mm256_shuffle_epi8(m[2], m[11]); // pshufb m0, m2, m11
        m[0] = _mm256_permutevar8x32_epi32(m[0], m[12]); // vpermd m0, m12, m0
        m[0] = _mm256_maddubs_epi16(m[0], m[5]); // pmaddubsw m0, m5
        m[0] = _mm256_mulhrs_epi16(m[0], m[13]); // pmulhrsw m0, m13
        m[0] = _mm256_packus_epi16(m[0], m[0]); // packuswb m0, m0
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        st32_p4(at(g.dst, g.stride * 2), _mm_cvtsi128_si32(lo(m[0]))); // movd [dstq+strideq*2], xm0
        st32_p4(at(g.dst, r9), _mm_extract_epi32::<1>(lo(m[0]))); // pextrd [dstq+r9], xm0, 1
        st32_p4(g.dst, _mm_cvtsi128_si32(lo(m[1]))); // movd [dstq+strideq*0], xm1
        st32_p4(at(g.dst, g.stride), _mm_extract_epi32::<1>(lo(m[1]))); // pextrd [dstq+strideq*1], xm1, 1
        g.dst = at(g.dst, g.stride * 4); // lea dstq, [dstq+strideq*4]
        g.h -= 4; // sub hd, 4
        if g.h <= 0 {
            break; // jg .w4_leftonly_loop
        }
    }
    // .w4_end: RET
}

/// `.w8` up to `.w8_filter_left`; true where the assembly continues at `.w8_filter_left`, false where it
/// continues at `.w8_main`.
#[inline(always)]
unsafe fn ipred_z2_8bpc_w8(m: &mut M, g: &mut GprP4) -> bool {
    m[6] = bci128_p4(ro(z2_base_inc, 0)); // vbroadcasti128 m6, [base+z2_base_inc] ; base_inc << 6
    m[5] = zx(_mm_cvtsi32_si128(g.dy)); // movd xm5, dyd
    m[10] = bci128_p4(ro(dav1d_z_filter_s, 2)); // vbroadcasti128 m10, [base+z_filter_s+2]
    m[11] = bci128_p4(ro(z2_shuf_h4, 0)); // vbroadcasti128 m11, [base+z2_shuf_h4]
    g.r2 = g.dx + (65 << 6); // lea r2d, [dxq+(65<<6)] ; xpos
    m[5] = zx(_mm_broadcastw_epi16(lo(m[5]))); // vpbroadcastw xm5, xm5
    g.r8 = (63 - 8) << 6; // mov r8d, (63-8)<<6
    g.dy = -4; // mov dyq, -4
    m[5] = zx(_mm_mullo_epi16(lo(m[5]), ld128_p4(ro(z2_ymul, 0)))); // pmullw xm5, [base+z2_ymul]
    if g.angle & 0x400 != 0 {
        return false; // test angled, 0x400 ; jnz .w8_main
    }
    g.r3 = g.angle + 126; // lea r3d, [angleq+126]
    g.r3 = (g.r3 & !0xff) | (g.h & 0xff); // mov r3b, hb
    if g.r3 as u32 <= 8 {
        // cmp r3d, 8 ; ja .w8_no_upsample_above ; angle >= 130 || h > 8 || is_sm
        m[3] = zx(bcdx_p4(ro(pb_8_p4, 0))); // vpbroadcastd xm3, [base+pb_8]
        sthq_p4(at(g.fp, 80), lo(m[1])); // movhps [rsp+80], xm1
        ipred_z2_8bpc_upsample_above(m, g); // call .upsample_above
        g.angle -= 53; // sub angled, 53 ; angle - 53
        g.r3 = g.h + 7; // lea r3d, [hq+7]
        g.angle ^= 0x7f; // xor angled, 0x7f ; 180 - angle
        ipred_z2_8bpc_filter_strength(m, g); // call .filter_strength
        return true; // jmp .w8_filter_left
    }
    // .w8_no_upsample_above:
    g.r3 = g.h + 7; // lea r3d, [hq+7]
    g.angle -= 90; // sub angled, 90 ; angle - 90
    ipred_z2_8bpc_filter_strength(m, g); // call .filter_strength
    if g.r3 != 0 {
        // test r3d, r3d ; jz .w8_no_filter_above
        g.r3 = popcnt_p4(g.r3); // popcnt r3d, r3d
        m[3] = zx(bcdx_p4(ro(pb_8_p4, 0))); // vpbroadcastd xm3, [base+pb_8]
        m[3] = zx(_mm_min_epu8(lo(m[3]), ld128_p4(ro(dav1d_z_filter_s, 8)))); // pminub xm3, [base+z_filter_s+8]
        m[0] = zx(bcdx_p4(zk_p4(g.r3, 0))); // vpbroadcastd xm0, [base+z_filter_k-4+r3*4+12*0]
        m[4] = zx(bcdx_p4(zk_p4(g.r3, 1))); // vpbroadcastd xm4, [base+z_filter_k-4+r3*4+12*1]
        m[2] = zx(_mm_shuffle_epi8(lo(m[1]), ld128_p4(ro(dav1d_z_filter_s, 0)))); // pshufb xm2, xm1, [base+z_filter_s] ; 00 01 12 23 34 45 56 67
        m[0] = zx(_mm_maddubs_epi16(lo(m[2]), lo(m[0]))); // pmaddubsw xm0, xm2, xm0
        m[3] = zx(_mm_shuffle_epi8(lo(m[1]), lo(m[3]))); // pshufb xm3, xm1, xm3 ; 34 45 56 67 78 88 88 88
        m[2] = zx(shufpsx_p4::<0x99>(lo(m[2]), lo(m[3]))); // shufps xm2, xm3, q2121 ; 12 23 34 45 56 67 78 88
        m[2] = zx(_mm_maddubs_epi16(lo(m[2]), lo(m[4]))); // pmaddubsw xm2, xm4
        m[4] = zx(bcdx_p4(zk_p4(g.r3, 2))); // vpbroadcastd xm4, [base+z_filter_k-4+r3*4+12*2]
        m[3] = zx(_mm_maddubs_epi16(lo(m[3]), lo(m[4]))); // pmaddubsw xm3, xm4
        m[4] = zx(_mm_set1_epi32(g.max_width)); // vpbroadcastd xm4, r6m ; max_width
        m[4] = zx(_mm_packs_epi32(lo(m[4]), lo(m[4]))); // packssdw xm4, xm4
        m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); // paddw xm0, xm2
        m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[3]))); // paddw xm0, xm3
        m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[13]))); // pmulhrsw xm0, xm13
        m[4] = zx(_mm_packs_epi16(lo(m[4]), lo(m[4]))); // packsswb xm4, xm4
        m[1] = zx(_mm_srli_si128::<1>(lo(m[1]))); // psrldq xm1, 1
        m[4] = zx(_mm_sub_epi8(lo(m[4]), ld128_p4(ro(pb_1to32, 0)))); // psubb xm4, [base+pb_1to32]
        m[0] = zx(_mm_packus_epi16(lo(m[0]), lo(m[0]))); // packuswb xm0, xm0
        m[0] = zx(_mm_blendv_epi8(lo(m[0]), lo(m[1]), lo(m[4]))); // vpblendvb xm0, xm1, xm4
        stq_p4(at(g.fp, 65), lo(m[0])); // movq [rsp+65], xm0
    }
    // .w8_no_filter_above:
    g.r3 = g.angle - 51; // lea r3d, [angleq-51]
    g.r3 = (g.r3 & !0xff) | (g.h & 0xff); // mov r3b, hb
    if g.r3 as u32 <= 8 {
        // cmp r3d, 8 ; jbe .w8_upsample_left ; angle > 140 && h <= 8 && !is_sm
        // .w8_upsample_left:
        ipred_z2_8bpc_upsample_left(m, g); // call .upsample_left
        return false; // falls into .w8_main
    }
    m[0] = bcd_p4(ro(pb_90_p4, 0)); // vpbroadcastd m0, [base+pb_90]
    m[0] = _mm256_sub_epi8(m[0], m[7]); // psubb m0, m7
    m[0] = _mm256_and_si256(m[0], m[8]); // pand m0, m8
    m[0] = _mm256_cmpgt_epi8(m[0], m[9]); // pcmpgtb m0, m9
    g.r3 = _mm256_movemask_epi8(m[0]); // pmovmskb r3d, m0
    true
}

/// `.w8_filter_left` with `.w8_filter_left_h16` and `.w8_filter_left_top16`, up to `jmp .w8_main`.
#[inline(always)]
unsafe fn ipred_z2_8bpc_w8_filter_left(m: &mut M, g: &mut GprP4) {
    if g.r3 == 0 {
        return; // test r3d, r3d ; jz .w8_main
    }
    g.r3 = popcnt_p4(g.r3); // popcnt r3d, r3d
    m[7] = bcd_p4(zk_p4(g.r3, 0)); // vpbroadcastd m7, [base+z_filter_k-4+r3*4+12*0]
    m[8] = bcd_p4(zk_p4(g.r3, 1)); // vpbroadcastd m8, [base+z_filter_k-4+r3*4+12*1]
    m[9] = bcd_p4(zk_p4(g.r3, 2)); // vpbroadcastd m9, [base+z_filter_k-4+r3*4+12*2]
    if g.h == 32 {
        // cmp hd, 32 ; jne .w8_filter_left_h16
        m[2] = zx(ld128_p4(at(g.fp, 27))); // movu xm2, [rsp+27]
        m[2] = ins1_p4(m[2], at(g.fp, 35)); // vinserti128 m2, [rsp+35], 1
        m[0] = zx(bcdx_p4(ro(pb_5_p4, 0))); // vpbroadcastd xm0, [base+pb_5]
        m[3] = bci128_p4(ro(dav1d_z_filter_s, 8)); // vbroadcasti128 m3, [base+z_filter_s+ 8]
        m[1] = bci128_p4(ro(dav1d_z_filter_s, 12)); // vbroadcasti128 m1, [base+z_filter_s+12]
        m[4] = bci128_p4(ro(dav1d_z_filter_s, 16)); // vbroadcasti128 m4, [base+z_filter_s+16]
        m[3] = _mm256_max_epu8(m[3], m[0]); // pmaxub m3, m0
        m[3] = _mm256_shuffle_epi8(m[2], m[3]); // pshufb m3, m2, m3
        m[3] = _mm256_maddubs_epi16(m[3], m[7]); // pmaddubsw m3, m7
        m[1] = _mm256_shuffle_epi8(m[2], m[1]); // pshufb m1, m2, m1
        m[1] = _mm256_maddubs_epi16(m[1], m[8]); // pmaddubsw m1, m8
        m[2] = _mm256_shuffle_epi8(m[2], m[4]); // pshufb m2, m4
        m[2] = _mm256_maddubs_epi16(m[2], m[9]); // pmaddubsw m2, m9
        m[3] = _mm256_add_epi16(m[3], m[1]); // paddw m3, m1
        m[3] = _mm256_add_epi16(m[3], m[2]); // paddw m3, m2
        m[3] = _mm256_mulhrs_epi16(m[3], m[13]); // pmulhrsw m3, m13
                                                 // jmp .w8_filter_left_top16
    } else {
        // .w8_filter_left_h16:
        let mut r5 = 10; // mov r5d, 10
        if g.h - 16 < 0 {
            // cmp hd, 16
            r5 = g.h; // cmovs r5d, hd
        }
        r5 ^= 15; // xor r5d, 15 ; h == 16 ? 5 : 15 - h
        m[0] = zx(_mm_cvtsi32_si128(r5)); // movd xm0, r5d
        m[0] = _mm256_broadcastb_epi8(lo(m[0])); // vpbroadcastb m0, xm0
    }
    // .w8_filter_left_top16:
    m[1] = bci128_p4(ro(dav1d_z_filter_s, 12)); // vbroadcasti128 m1, [base+z_filter_s+12]
    m[2] = ins1_p4(m[1], ro(dav1d_z_filter_s, 8)); // vinserti128 m2, m1, [base+z_filter_s+8], 1
    m[4] = bci128_p4(ro(dav1d_z_filter_s, 16)); // vbroadcasti128 m4, [base+z_filter_s+16]
    m[1] = _mm256_blend_epi32::<0x0f>(m[1], m[4]); // vpblendd m1, m4, 0x0f
    m[4] = _mm256_inserti128_si256::<0>(m[4], ld128_p4(ro(dav1d_z_filter_s, 20))); // vinserti128 m4, [base+z_filter_s+20], 0
    m[0] = _mm256_max_epu8(m[0], m[2]); // pmaxub m0, m2
    m[2] = zx(ld128_p4(at(g.fp, 49))); // movu xm2, [rsp+49]
    m[2] = ins1_p4(m[2], at(g.fp, 43)); // vinserti128 m2, [rsp+43], 1
    m[0] = _mm256_shuffle_epi8(m[2], m[0]); // pshufb m0, m2, m0
    m[0] = _mm256_maddubs_epi16(m[0], m[7]); // pmaddubsw m0, m7
    m[7] = _mm256_set1_epi32(g.max_height); // vpbroadcastd m7, r7m ; max_height
    m[1] = _mm256_shuffle_epi8(m[2], m[1]); // pshufb m1, m2, m1
    m[1] = _mm256_maddubs_epi16(m[1], m[8]); // pmaddubsw m1, m8
    m[2] = _mm256_shuffle_epi8(m[2], m[4]); // pshufb m2, m4
    m[2] = _mm256_maddubs_epi16(m[2], m[9]); // pmaddubsw m2, m9
    m[7] = _mm256_packs_epi32(m[7], m[7]); // packssdw m7, m7
    m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
    m[7] = _mm256_packs_epi16(m[7], m[7]); // packsswb m7, m7
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[1] = _mm256_mulhrs_epi16(m[1], m[13]); // pmulhrsw m1, m13
    m[7] = _mm256_sub_epi8(m[7], ld256_p4(ro(pb_32to1, 0))); // psubb m7, [base+pb_32to1]
    m[3] = _mm256_packus_epi16(m[3], m[1]); // packuswb m3, m1
    m[3] = _mm256_permute4x64_epi64::<0x78>(m[3]); // vpermq m3, m3, q1320
    m[3] = _mm256_blendv_epi8(m[3], ld256_p4(at(g.fp, 32)), m[7]); // vpblendvb m3, [rsp+32], m7
    st256_p4(at(g.fp, 32), m[3]); // mova [rsp+32], m3
} // jmp .w8_main

/// `.w8_main` with `.w8_loop`, `.w8_toponly`, `.w8_leftonly_loop` and `.w8_end`.
#[inline(always)]
unsafe fn ipred_z2_8bpc_w8_main(m: &mut M, g: &mut GprP4) {
    let fp = g.fp;
    m[3] = zx(_mm_cvtsi32_si128(g.dx)); // movd xm3, dxd
    g.r5 = at(fp, 56); // lea r5, [rsp+56] ; left-7
    m[1] = zx(_mm_shuffle_epi32::<0xd8>(lo(m[5]))); // pshufd xm1, xm5, q3120
    m[5] = zx(_mm_and_si128(lo(m[5]), lo(m[14]))); // pand xm5, xm14
    m[3] = _mm256_broadcastw_epi16(lo(m[3])); // vpbroadcastw m3, xm3
    m[0] = zx(_mm_setzero_si128()); // pxor xm0, xm0
    m[2] = zx(_mm_sub_epi16(lo(m[15]), lo(m[5]))); // psubw xm2, xm15, xm5
    m[1] = zx(_mm_srai_epi16::<6>(lo(m[1]))); // psraw xm1, 6
    let r9 = g.stride * 3; // lea r9, [strideq*3]
    m[7] = _mm256_add_epi16(m[3], m[3]); // paddw m7, m3, m3
    m[9] = zx(_mm_sub_epi16(lo(m[0]), lo(m[1]))); // psubw xm9, xm0, xm1 ; base_y
    m[5] = zx(_mm_slli_epi16::<8>(lo(m[5]))); // psllw xm5, 8
    // punpcklwd/punpckhwd with xm0 = 0: zero-extended in the assembly, sign-extended here (`gather_index_p4`).
    m[8] = zx(gather_index_p4(lo(m[9]))); // punpcklwd xm8, xm9, xm0 ; base_y 0, 1, 4, 5
    m[3] = _mm256_blend_epi32::<0xf0>(m[3], m[7]); // vpblendd m3, m7, 0xf0 ; xpos0 xpos1
    m[5] = zx(_mm_or_si128(lo(m[5]), lo(m[2]))); // por xm5, xm2 ; 64-frac_y, frac_y
    m[9] = zx(gather_index_p4(_mm_unpackhi_epi64(lo(m[9]), lo(m[9])))); // punpckhwd xm9, xm0 ; base_y 2, 3, 6, 7
    m[6] = _mm256_add_epi16(m[6], m[3]); // paddw m6, m3
    m[12] = _mm256_inserti128_si256::<1>(m[5], lo(m[5])); // vinserti128 m12, m5, xm5, 1
    loop {
        // .w8_loop:
        g.r3 = g.r2 + g.dx; // lea r3d, [r2+dxq]
        g.r2 = ((g.r2 as u32) >> 6) as i32; // shr r2d, 6 ; base_x0
        m[0] = zx(ld128_p4(at(fp, g.r2 as isize))); // movu xm0, [rsp+r2]
        g.r2 = g.r3 + g.dx; // lea r2d, [r3+dxq]
        g.r3 = ((g.r3 as u32) >> 6) as i32; // shr r3d, 6 ; base_x1
        m[0] = ins1_p4(m[0], at(fp, g.r3 as isize)); // vinserti128 m0, [rsp+r3], 1
        g.r3 = g.r2 + g.dx; // lea r3d, [r2+dxq]
        g.r2 = ((g.r2 as u32) >> 6) as i32; // shr r2d, 6 ; base_x2
        m[1] = zx(ld128_p4(at(fp, g.r2 as isize))); // movu xm1, [rsp+r2]
        g.r2 = g.r3 + g.dx; // lea r2d, [r3+dxq]
        g.r3 = ((g.r3 as u32) >> 6) as i32; // shr r3d, 6 ; base_x3
        m[1] = ins1_p4(m[1], at(fp, g.r3 as isize)); // vinserti128 m1, [rsp+r3], 1
        m[2] = _mm256_and_si256(m[14], m[6]); // pand m2, m14, m6
        m[4] = _mm256_adds_epi16(m[6], m[7]); // paddsw m4, m6, m7
        m[5] = _mm256_sub_epi16(m[15], m[2]); // psubw m5, m15, m2
        m[2] = _mm256_slli_epi16::<8>(m[2]); // psllw m2, 8
        m[0] = _mm256_shuffle_epi8(m[0], m[10]); // pshufb m0, m10
        m[2] = _mm256_or_si256(m[2], m[5]); // por m2, m5
        m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
        m[2] = _mm256_and_si256(m[14], m[4]); // pand m2, m14, m4
        m[5] = _mm256_sub_epi16(m[15], m[2]); // psubw m5, m15, m2
        m[2] = _mm256_slli_epi16::<8>(m[2]); // psllw m2, 8
        m[1] = _mm256_shuffle_epi8(m[1], m[10]); // pshufb m1, m10
        m[2] = _mm256_or_si256(m[2], m[5]); // por m2, m5
        m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
        if g.r3 < 64 {
            // cmp r3d, 64 ; jge .w8_toponly
            // All lanes are gathered (the masks are negative); the frame is zeroed whole and the indices stay
            // within rsp+28..rsp+192 (see FrameP4). Each gather clears its mask; m7 is restored from m5 as in the
            // assembly, and m5 is written before it is read again.
            m[5] = m[7]; // mova m5, m7
            m[3] = _mm256_mask_i32gather_epi64::<1>(m[3], g.r5 as *const i64, lo(m[9]), m[7]); // vpgatherdq m3, [r5+xm9], m7
            m[7] = m[5]; // mova m7, m5
            m[2] = _mm256_mask_i32gather_epi64::<1>(m[2], g.r5 as *const i64, lo(m[8]), m[5]); // vpgatherdq m2, [r5+xm8], m5
            m[3] = _mm256_shuffle_epi8(m[3], m[11]); // pshufb m3, m11
            m[2] = _mm256_shuffle_epi8(m[2], m[11]); // pshufb m2, m11
            m[5] = _mm256_unpacklo_epi32(m[2], m[3]); // punpckldq m5, m2, m3 ; a0 b0 c0 d0 a1 b1 c1 d1   e0 f0 g0 h0 e1 f1 g1 h1
            m[2] = _mm256_unpackhi_epi32(m[2], m[3]); // punpckhdq m2, m3 ; a2 b2 c2 d2 a3 b3 c3 d3   e2 f2 g2 h2 e3 f3 g3 h3
            m[5] = _mm256_permute4x64_epi64::<0xd8>(m[5]); // vpermq m5, m5, q3120 ; y0 y1
            m[2] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m2, m2, q3120 ; y2 y3
            m[5] = _mm256_maddubs_epi16(m[5], m[12]); // pmaddubsw m5, m12
            m[2] = _mm256_maddubs_epi16(m[2], m[12]); // pmaddubsw m2, m12
            m[6] = _mm256_srai_epi16::<15>(m[6]); // psraw m6, 15 ; base_x < topleft
            m[0] = _mm256_blendv_epi8(m[0], m[5], m[6]); // vpblendvb m0, m5, m6
            m[3] = _mm256_srai_epi16::<15>(m[4]); // psraw m3, m4, 15
            m[1] = _mm256_blendv_epi8(m[1], m[2], m[3]); // vpblendvb m1, m2, m3
        }
        // .w8_toponly:
        m[0] = _mm256_mulhrs_epi16(m[0], m[13]); // pmulhrsw m0, m13
        m[1] = _mm256_mulhrs_epi16(m[1], m[13]); // pmulhrsw m1, m13
        m[6] = _mm256_add_epi16(m[4], m[7]); // paddw m6, m4, m7 ; xpos += dx
        g.r5 = at(g.r5, g.dy as isize); // add r5, dyq
        m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        stq_p4(g.dst, lo(m[0])); // movq [dstq+strideq*0], xm0
        sthq_p4(at(g.dst, g.stride * 2), lo(m[0])); // movhps [dstq+strideq*2], xm0
        stq_p4(at(g.dst, g.stride), lo(m[1])); // movq [dstq+strideq*1], xm1
        sthq_p4(at(g.dst, r9), lo(m[1])); // movhps [dstq+r9], xm1
        g.h -= 4; // sub hd, 4
        if g.h == 0 {
            return; // jz .w8_end ; RET
        }
        g.dst = at(g.dst, g.stride * 4); // lea dstq, [dstq+strideq*4]
        if g.r2 < g.r8 {
            break; // cmp r2d, r8d ; jge .w8_loop
        }
    }
    loop {
        // .w8_leftonly_loop:
        m[0] = m[7]; // mova m0, m7
        m[5] = _mm256_mask_i32gather_epi64::<1>(m[5], g.r5 as *const i64, lo(m[9]), m[7]); // vpgatherdq m5, [r5+xm9], m7
        m[7] = m[0]; // mova m7, m0
        m[3] = _mm256_mask_i32gather_epi64::<1>(m[3], g.r5 as *const i64, lo(m[8]), m[0]); // vpgatherdq m3, [r5+xm8], m0
        g.r5 = at(g.r5, g.dy as isize); // add r5, dyq
        m[2] = _mm256_shuffle_epi8(m[5], m[11]); // pshufb m2, m5, m11
        m[1] = _mm256_shuffle_epi8(m[3], m[11]); // pshufb m1, m3, m11
        m[0] = _mm256_unpacklo_epi32(m[1], m[2]); // punpckldq m0, m1, m2
        m[1] = _mm256_unpackhi_epi32(m[1], m[2]); // punpckhdq m1, m2
        m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
        m[1] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m1, m1, q3120
        m[0] = _mm256_maddubs_epi16(m[0], m[12]); // pmaddubsw m0, m12
        m[1] = _mm256_maddubs_epi16(m[1], m[12]); // pmaddubsw m1, m12
        m[0] = _mm256_mulhrs_epi16(m[0], m[13]); // pmulhrsw m0, m13
        m[1] = _mm256_mulhrs_epi16(m[1], m[13]); // pmulhrsw m1, m13
        m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        stq_p4(g.dst, lo(m[0])); // movq [dstq+strideq*0], xm0
        sthq_p4(at(g.dst, g.stride * 2), lo(m[0])); // movhps [dstq+strideq*2], xm0
        stq_p4(at(g.dst, g.stride), lo(m[1])); // movq [dstq+strideq*1], xm1
        sthq_p4(at(g.dst, r9), lo(m[1])); // movhps [dstq+r9], xm1
        g.dst = at(g.dst, g.stride * 4); // lea dstq, [dstq+strideq*4]
        g.h -= 4; // sub hd, 4
        if g.h <= 0 {
            break; // jg .w8_leftonly_loop
        }
    }
    // .w8_end: RET
}

/// `.w16` up to `.w16_filter_left`; true where the assembly continues at `.w16_filter_left`, false where it jumps
/// to `.w16_main`.
#[inline(always)]
unsafe fn ipred_z2_8bpc_w16(m: &mut M, g: &mut GprP4) -> bool {
    g.r8 = g.h; // mov r8d, hd
    if g.angle & 0x400 != 0 {
        return false; // test angled, 0x400 ; jnz .w16_main
    }
    g.r3 = g.h + 15; // lea r3d, [hq+15]
    g.angle -= 90; // sub angled, 90
    ipred_z2_8bpc_filter_strength(m, g); // call .filter_strength
    if g.r3 != 0 {
        // test r3d, r3d ; jz .w16_no_filter_above
        g.r3 = popcnt_p4(g.r3); // popcnt r3d, r3d
        m[6] = bci128_p4(g.tl.wrapping_offset(1)); // vbroadcasti128 m6, [tlq+1]
        m[2] = zx(ld128_p4(ro(dav1d_z_filter_s, 0))); // mova xm2, [base+z_filter_s]
        m[2] = ins1_p4(m[2], ro(dav1d_z_filter_s, 14)); // vinserti128 m2, [base+z_filter_s+14], 1 ; 00 01 12 23 34 45 56 67   67 78 89 9a ab bc cd de
        m[3] = zx(ld128_p4(ro(dav1d_z_filter_s, 8))); // movu xm3, [base+z_filter_s+8]
        m[3] = ins1_p4(m[3], ro(dav1d_z_filter_s, 22)); // vinserti128 m3, [base+z_filter_s+22], 1 ; 34 45 56 67 78 89 9a ab   ab bc cd de ef ff ff ff
        m[1] = _mm256_blend_epi32::<0xf0>(m[1], m[6]); // vpblendd m1, m6, 0xf0
        m[0] = bcd_p4(zk_p4(g.r3, 0)); // vpbroadcastd m0, [base+z_filter_k-4+r3*4+12*0]
        m[4] = bcd_p4(zk_p4(g.r3, 1)); // vpbroadcastd m4, [base+z_filter_k-4+r3*4+12*1]
        m[5] = bcd_p4(zk_p4(g.r3, 2)); // vpbroadcastd m5, [base+z_filter_k-4+r3*4+12*2]
        m[2] = _mm256_shuffle_epi8(m[1], m[2]); // pshufb m2, m1, m2
        m[1] = _mm256_shuffle_epi8(m[1], m[3]); // pshufb m1, m3
        m[0] = _mm256_maddubs_epi16(m[2], m[0]); // pmaddubsw m0, m2, m0
        m[2] = shufps_p4::<0x99>(m[2], m[1]); // shufps m2, m1, q2121 ; 12 23 34 45 56 67 78 89   89 9a ab bc cd de ef ff
        m[2] = _mm256_maddubs_epi16(m[2], m[4]); // pmaddubsw m2, m4
        m[1] = _mm256_maddubs_epi16(m[1], m[5]); // pmaddubsw m1, m5
        m[4] = zx(_mm_set1_epi32(g.max_width)); // vpbroadcastd xm4, r6m ; max_width
        m[4] = zx(_mm_packs_epi32(lo(m[4]), lo(m[4]))); // packssdw xm4, xm4
        m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
        m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
        m[0] = _mm256_mulhrs_epi16(m[0], m[13]); // pmulhrsw m0, m13
        m[4] = zx(_mm_packs_epi16(lo(m[4]), lo(m[4]))); // packsswb xm4, xm4
        m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1
        m[4] = zx(_mm_sub_epi8(lo(m[4]), ld128_p4(ro(pb_1to32, 0)))); // psubb xm4, [base+pb_1to32]
        m[0] = zx(_mm_packus_epi16(lo(m[0]), lo(m[2]))); // packuswb xm0, xm2
        m[0] = zx(_mm_blendv_epi8(lo(m[0]), lo(m[6]), lo(m[4]))); // vpblendvb xm0, xm6, xm4
        st128_p4(at(g.fp, 65), lo(m[0])); // movu [rsp+65], xm0
    }
    // .w16_no_filter_above:
    m[0] = bcd_p4(ro(pb_90_p4, 0)); // vpbroadcastd m0, [base+pb_90]
    m[0] = _mm256_sub_epi8(m[0], m[7]); // psubb m0, m7
    m[0] = _mm256_and_si256(m[0], m[8]); // pand m0, m8
    m[0] = _mm256_cmpgt_epi8(m[0], m[9]); // pcmpgtb m0, m9
    g.r3 = _mm256_movemask_epi8(m[0]); // pmovmskb r3d, m0
    if g.r3 == 0 {
        return false; // test r3d, r3d ; jz .w16_main
    }
    g.r3 = popcnt_p4(g.r3); // popcnt r3d, r3d
    m[7] = bcd_p4(zk_p4(g.r3, 0)); // vpbroadcastd m7, [base+z_filter_k-4+r3*4+12*0]
    m[8] = bcd_p4(zk_p4(g.r3, 1)); // vpbroadcastd m8, [base+z_filter_k-4+r3*4+12*1]
    m[9] = bcd_p4(zk_p4(g.r3, 2)); // vpbroadcastd m9, [base+z_filter_k-4+r3*4+12*2]
    true
}

/// `.w16_filter_left` with `_h32`, `_top32`, `_h16` and `_top16`, up to `.w16_main`.
#[inline(always)]
unsafe fn ipred_z2_8bpc_w16_filter_left(m: &mut M, g: &mut GprP4) {
    let tl = g.tl;
    m[6] = _mm256_set1_epi32(g.max_height); // vpbroadcastd m6, r7m ; max_height
    m[6] = _mm256_packs_epi32(m[6], m[6]); // packssdw m6, m6
    m[6] = _mm256_packs_epi16(m[6], m[6]); // packsswb m6, m6
    if g.h >= 32 {
        // cmp hd, 32 ; jl .w16_filter_left_h16
        m[0] = zx(bcdx_p4(ro(pb_5_p4, 0))); // vpbroadcastd xm0, [base+pb_5]
        m[10] = bci128_p4(ro(dav1d_z_filter_s, 8)); // vbroadcasti128 m10, [base+z_filter_s+ 8]
        m[11] = bci128_p4(ro(dav1d_z_filter_s, 12)); // vbroadcasti128 m11, [base+z_filter_s+12]
        m[12] = bci128_p4(ro(dav1d_z_filter_s, 16)); // vbroadcasti128 m12, [base+z_filter_s+16]
        if g.h != 32 {
            // je .w16_filter_left_h32
            m[3] = ld256_p4(tl.wrapping_offset(-69)); // movu m3, [tlq-69]
            m[5] = ld256_p4(tl.wrapping_offset(-61)); // movu m5, [tlq-61]
            m[1] = _mm256_max_epu8(m[10], m[0]); // pmaxub m1, m10, m0
            m[1] = _mm256_shuffle_epi8(m[3], m[1]); // pshufb m1, m3, m1
            m[1] = _mm256_maddubs_epi16(m[1], m[7]); // pmaddubsw m1, m7
            m[2] = _mm256_shuffle_epi8(m[3], m[11]); // pshufb m2, m3, m11
            m[2] = _mm256_maddubs_epi16(m[2], m[8]); // pmaddubsw m2, m8
            m[3] = _mm256_shuffle_epi8(m[3], m[12]); // pshufb m3, m12
            m[3] = _mm256_maddubs_epi16(m[3], m[9]); // pmaddubsw m3, m9
            m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
            m[2] = _mm256_shuffle_epi8(m[5], m[10]); // pshufb m2, m5, m10
            m[2] = _mm256_maddubs_epi16(m[2], m[7]); // pmaddubsw m2, m7
            m[4] = _mm256_shuffle_epi8(m[5], m[11]); // pshufb m4, m5, m11
            m[4] = _mm256_maddubs_epi16(m[4], m[8]); // pmaddubsw m4, m8
            m[5] = _mm256_shuffle_epi8(m[5], m[12]); // pshufb m5, m12
            m[5] = _mm256_maddubs_epi16(m[5], m[9]); // pmaddubsw m5, m9
            m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3
            m[3] = bcd_p4(ro(pb_32_p4, 0)); // vpbroadcastd m3, [base+pb_32]
            m[3] = _mm256_add_epi8(m[3], ld256_p4(ro(pb_32to1, 0))); // paddb m3, [base+pb_32to1]
            m[2] = _mm256_add_epi16(m[2], m[4]); // paddw m2, m4
            m[2] = _mm256_add_epi16(m[2], m[5]); // paddw m2, m5
            m[1] = _mm256_mulhrs_epi16(m[1], m[13]); // pmulhrsw m1, m13
            m[2] = _mm256_mulhrs_epi16(m[2], m[13]); // pmulhrsw m2, m13
            m[3] = _mm256_sub_epi8(m[6], m[3]); // psubb m3, m6, m3
            m[1] = _mm256_packus_epi16(m[1], m[2]); // packuswb m1, m2
            m[1] = _mm256_blendv_epi8(m[1], ld256_p4(tl.wrapping_offset(-64)), m[3]); // vpblendvb m1, [tlq-64], m3
            st256_p4(g.fp, m[1]); // mova [rsp], m1
                                  // jmp .w16_filter_left_top32
        } else {
            // .w16_filter_left_h32:
            m[10] = _mm256_max_epu8(m[10], m[0]); // pmaxub m10, m0
        }
        // .w16_filter_left_top32:
        m[2] = zx(ld128_p4(tl.wrapping_offset(-37))); // movu xm2, [tlq-37]
        m[2] = ins1_p4(m[2], tl.wrapping_offset(-29)); // vinserti128 m2, [tlq-29], 1
        m[3] = _mm256_shuffle_epi8(m[2], m[10]); // pshufb m3, m2, m10
        m[1] = _mm256_shuffle_epi8(m[2], m[11]); // pshufb m1, m2, m11
        m[2] = _mm256_shuffle_epi8(m[2], m[12]); // pshufb m2, m12
        m[3] = _mm256_maddubs_epi16(m[3], m[7]); // pmaddubsw m3, m7
        m[1] = _mm256_maddubs_epi16(m[1], m[8]); // pmaddubsw m1, m8
        m[2] = _mm256_maddubs_epi16(m[2], m[9]); // pmaddubsw m2, m9
        m[3] = _mm256_add_epi16(m[3], m[1]); // paddw m3, m1
        m[3] = _mm256_add_epi16(m[3], m[2]); // paddw m3, m2
        m[3] = _mm256_mulhrs_epi16(m[3], m[13]); // pmulhrsw m3, m13
                                                 // jmp .w16_filter_left_top16
    } else {
        // .w16_filter_left_h16:
        let mut r5 = 10; // mov r5d, 10
        if g.h - 16 < 0 {
            // cmp hd, 16
            r5 = g.h; // cmovs r5d, hd
        }
        r5 ^= 15; // xor r5d, 15 ; h == 16 ? 5 : 15 - h
        m[0] = zx(_mm_cvtsi32_si128(r5)); // movd xm0, r5d
        m[0] = _mm256_broadcastb_epi8(lo(m[0])); // vpbroadcastb m0, xm0
    }
    // .w16_filter_left_top16:
    m[2] = zx(ld128_p4(tl.wrapping_offset(-15))); // movu xm2, [tlq-15]
    m[2] = ins1_p4(m[2], tl.wrapping_offset(-21)); // vinserti128 m2, [tlq-21], 1
    m[1] = bci128_p4(ro(dav1d_z_filter_s, 12)); // vbroadcasti128 m1, [base+z_filter_s+12]
    m[4] = bci128_p4(ro(dav1d_z_filter_s, 16)); // vbroadcasti128 m4, [base+z_filter_s+16]
    m[5] = ins1_p4(m[1], ro(dav1d_z_filter_s, 8)); // vinserti128 m5, m1, [base+z_filter_s+8], 1 ; 56 67 78 89 9a ab bc cd   34 45 56 67 78 89 9a ab
    m[1] = _mm256_blend_epi32::<0x0f>(m[1], m[4]); // vpblendd m1, m4, 0x0f ; 78 89 9a ab bc cd de ef   56 67 78 89 9a ab bc cd
    m[4] = _mm256_inserti128_si256::<0>(m[4], ld128_p4(ro(dav1d_z_filter_s, 20))); // vinserti128 m4, [base+z_filter_s+20], 0 ; 9a ab bc cd de ef ff ff   78 89 9a ab bc cd de ef
    m[0] = _mm256_max_epu8(m[0], m[5]); // pmaxub m0, m5
    m[0] = _mm256_shuffle_epi8(m[2], m[0]); // pshufb m0, m2, m0
    m[0] = _mm256_maddubs_epi16(m[0], m[7]); // pmaddubsw m0, m7
    m[1] = _mm256_shuffle_epi8(m[2], m[1]); // pshufb m1, m2, m1
    m[1] = _mm256_maddubs_epi16(m[1], m[8]); // pmaddubsw m1, m8
    m[2] = _mm256_shuffle_epi8(m[2], m[4]); // pshufb m2, m4
    m[2] = _mm256_maddubs_epi16(m[2], m[9]); // pmaddubsw m2, m9
    m[6] = _mm256_sub_epi8(m[6], ld256_p4(ro(pb_32to1, 0))); // psubb m6, [base+pb_32to1]
    m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[1] = _mm256_mulhrs_epi16(m[1], m[13]); // pmulhrsw m1, m13
    m[3] = _mm256_packus_epi16(m[3], m[1]); // packuswb m3, m1
    m[3] = _mm256_permute4x64_epi64::<0x78>(m[3]); // vpermq m3, m3, q1320
    m[3] = _mm256_blendv_epi8(m[3], ld256_p4(tl.wrapping_offset(-32)), m[6]); // vpblendvb m3, [tlq-32], m6
    st256_p4(at(g.fp, 32), m[3]); // mova [rsp+32], m3
}

/// `.w16_main` with `.w16_loop0`, `.w16_loop`, `.w16_toponly`, `.w16_leftonly_loop`, `.w16_end` and `.w16_ret`
/// (w16, w32, w64: one 16-pixel-wide column per pass of `.w16_loop0`).
#[inline(always)]
unsafe fn ipred_z2_8bpc_w16_main(m: &mut M, g: &mut GprP4) {
    let fp = g.fp;
    m[1] = zx(_mm_cvtsi32_si128(g.dy)); // movd xm1, dyd
    m[10] = bci128_p4(ro(dav1d_z_filter_s, 2)); // vbroadcasti128 m10, [base+z_filter_s+2]
    m[7] = zx(_mm_cvtsi32_si128(g.dx)); // movd xm7, dxd
    m[11] = bci128_p4(ro(z2_shuf_h2, 0)); // vbroadcasti128 m11, [base+z2_shuf_h2]
    m[1] = _mm256_broadcastw_epi16(lo(m[1])); // vpbroadcastw m1, xm1
    m[7] = _mm256_broadcastw_epi16(lo(m[7])); // vpbroadcastw m7, xm7
    let mut r7 = g.dst; // mov r7, dstq
    m[0] = _mm256_mullo_epi16(m[1], ld256_p4(ro(z2_ymul, 0))); // pmullw m0, m1, [base+z2_ymul]
    m[1] = zx(_mm_slli_epi16::<4>(lo(m[1]))); // psllw xm1, 4
    m[6] = _mm256_add_epi16(m[7], ld256_p4(ro(z2_base_inc, 0))); // paddw m6, m7, [base+z2_base_inc]
    let mut r9 = g.dx + (65 << 6); // lea r9d, [dxq+(65<<6)] ; xpos
    st32_p4(at(fp, 156), _mm_cvtsi128_si32(lo(m[1]))); // movd [rsp+156], xm1
    loop {
        // .w16_loop0:
        g.r2 = r9; // mov r2d, r9d
        st256_p4(at(fp, 160), m[0]); // mova [rsp+160], m0
        g.r5 = at(fp, 60); // lea r5, [rsp+60] ; left-3
        st256_p4(at(fp, 192), m[6]); // mova [rsp+192], m6
        m[1] = _mm256_setzero_si256(); // pxor m1, m1
        m[2] = _mm256_srai_epi16::<6>(m[0]); // psraw m2, m0, 6
        m[0] = _mm256_and_si256(m[0], m[14]); // pand m0, m14
        m[9] = _mm256_sub_epi16(m[1], m[2]); // psubw m9, m1, m2 ; base_y
        m[12] = _mm256_sub_epi16(m[15], m[0]); // psubw m12, m15, m0
        m[8] = _mm256_unpacklo_epi16(m[9], m[1]); // punpcklwd m8, m9, m1 ; base_y  0,  1,  2,  3,     8,  9, 10, 11
        m[0] = _mm256_slli_epi16::<8>(m[0]); // psllw m0, 8
        m[9] = _mm256_unpackhi_epi16(m[9], m[1]); // punpckhwd m9, m1 ; base_y  4,  5,  6,  7,    12, 13, 14, 15
        m[12] = _mm256_or_si256(m[12], m[0]); // por m12, m0 ; 64-frac_y, frac_y
        'w16_end: {
            loop {
                // .w16_loop:
                g.r3 = g.r2 + g.dx; // lea r3d, [r2+dxq]
                g.r2 = ((g.r2 as u32) >> 6) as i32; // shr r2d, 6 ; base_x0
                m[0] = zx(ld128_p4(at(fp, g.r2 as isize))); // movu xm0, [rsp+r2]
                m[0] = ins1_p4(m[0], at(fp, g.r2 as isize + 8)); // vinserti128 m0, [rsp+r2+8], 1
                g.r2 = g.r3 + g.dx; // lea r2d, [r3+dxq]
                g.r3 = ((g.r3 as u32) >> 6) as i32; // shr r3d, 6 ; base_x1
                m[1] = zx(ld128_p4(at(fp, g.r3 as isize))); // movu xm1, [rsp+r3]
                m[1] = ins1_p4(m[1], at(fp, g.r3 as isize + 8)); // vinserti128 m1, [rsp+r3+8], 1
                m[2] = _mm256_and_si256(m[14], m[6]); // pand m2, m14, m6
                m[5] = _mm256_adds_epi16(m[6], m[7]); // paddsw m5, m6, m7
                m[3] = _mm256_sub_epi16(m[15], m[2]); // psubw m3, m15, m2
                m[2] = _mm256_slli_epi16::<8>(m[2]); // psllw m2, 8
                m[0] = _mm256_shuffle_epi8(m[0], m[10]); // pshufb m0, m10
                m[2] = _mm256_or_si256(m[2], m[3]); // por m2, m3
                m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
                m[2] = _mm256_and_si256(m[14], m[5]); // pand m2, m14, m5
                m[3] = _mm256_sub_epi16(m[15], m[2]); // psubw m3, m15, m2
                m[2] = _mm256_slli_epi16::<8>(m[2]); // psllw m2, 8
                m[1] = _mm256_shuffle_epi8(m[1], m[10]); // pshufb m1, m10
                m[2] = _mm256_or_si256(m[2], m[3]); // por m2, m3
                m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
                if g.r3 < 64 {
                    // cmp r3d, 64 ; jge .w16_toponly
                    // Only the columns whose second row uses the left edge are gathered (base_x < 0, so the
                    // index lies in the left edge; the first byte of a dword may be rsp-1, see FrameP4); the
                    // other lanes keep m4/m3 and are blended out below. Each gather clears its mask, which the
                    // next line writes.
                    m[2] = _mm256_unpackhi_epi16(m[5], m[5]); // punpckhwd m2, m5, m5 ; mask out unnecessary loads
                    m[4] = _mm256_mask_i32gather_epi32::<1>(m[4], g.r5 as *const i32, m[9], m[2]); // vpgatherdd m4, [r5+m9], m2
                    m[2] = _mm256_unpacklo_epi16(m[5], m[5]); // punpcklwd m2, m5, m5
                    m[3] = _mm256_mask_i32gather_epi32::<1>(m[3], g.r5 as *const i32, m[8], m[2]); // vpgatherdd m3, [r5+m8], m2
                    m[4] = _mm256_shuffle_epi8(m[4], m[11]); // pshufb m4, m11 ; e0 f0 g0 h0 e1 f1 g1 h1   m0 n0 o0 p0 m1 n1 o1 p1
                    m[3] = _mm256_shuffle_epi8(m[3], m[11]); // pshufb m3, m11 ; a0 b0 c0 d0 a1 b1 c1 d1   i0 j0 k0 l0 i1 j1 k1 l1
                    m[2] = _mm256_unpacklo_epi64(m[3], m[4]); // punpcklqdq m2, m3, m4 ; y0
                    m[3] = _mm256_unpackhi_epi64(m[3], m[4]); // punpckhqdq m3, m4 ; y1
                    m[2] = _mm256_maddubs_epi16(m[2], m[12]); // pmaddubsw m2, m12
                    m[3] = _mm256_maddubs_epi16(m[3], m[12]); // pmaddubsw m3, m12
                    m[6] = _mm256_srai_epi16::<15>(m[6]); // psraw m6, 15 ; base_x < topleft
                    m[0] = _mm256_blendv_epi8(m[0], m[2], m[6]); // vpblendvb m0, m2, m6
                    m[6] = _mm256_srai_epi16::<15>(m[5]); // psraw m6, m5, 15
                    m[1] = _mm256_blendv_epi8(m[1], m[3], m[6]); // vpblendvb m1, m3, m6
                }
                // .w16_toponly:
                m[0] = _mm256_mulhrs_epi16(m[0], m[13]); // pmulhrsw m0, m13
                m[1] = _mm256_mulhrs_epi16(m[1], m[13]); // pmulhrsw m1, m13
                m[6] = _mm256_add_epi16(m[5], m[7]); // paddw m6, m5, m7 ; xpos += dx
                g.r5 = at(g.r5, -2); // sub r5, 2
                m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
                m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
                st128_p4(g.dst, lo(m[0])); // mova [dstq+strideq*0], xm0
                st128_p4(at(g.dst, g.stride), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
                g.h -= 2; // sub hd, 2
                if g.h == 0 {
                    break 'w16_end; // jz .w16_end
                }
                g.dst = at(g.dst, g.stride * 2); // lea dstq, [dstq+strideq*2]
                if g.r2 < (63 - 16) << 6 {
                    break; // cmp r2d, (63-16)<<6 ; jge .w16_loop
                }
            }
            loop {
                // .w16_leftonly_loop:
                // Every column of these rows uses the left edge (base_x < 0): all lanes are gathered.
                m[0] = m[7]; // mova m0, m7
                m[4] = _mm256_mask_i32gather_epi32::<1>(m[4], g.r5 as *const i32, m[9], m[7]); // vpgatherdd m4, [r5+m9], m7
                m[7] = m[0]; // mova m7, m0
                m[3] = _mm256_mask_i32gather_epi32::<1>(m[3], g.r5 as *const i32, m[8], m[0]); // vpgatherdd m3, [r5+m8], m0
                g.r5 = at(g.r5, -2); // sub r5, 2
                m[2] = _mm256_shuffle_epi8(m[4], m[11]); // pshufb m2, m4, m11
                m[1] = _mm256_shuffle_epi8(m[3], m[11]); // pshufb m1, m3, m11
                m[0] = _mm256_unpacklo_epi64(m[1], m[2]); // punpcklqdq m0, m1, m2
                m[1] = _mm256_unpackhi_epi64(m[1], m[2]); // punpckhqdq m1, m2
                m[0] = _mm256_maddubs_epi16(m[0], m[12]); // pmaddubsw m0, m12
                m[1] = _mm256_maddubs_epi16(m[1], m[12]); // pmaddubsw m1, m12
                m[0] = _mm256_mulhrs_epi16(m[0], m[13]); // pmulhrsw m0, m13
                m[1] = _mm256_mulhrs_epi16(m[1], m[13]); // pmulhrsw m1, m13
                m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
                m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
                st128_p4(g.dst, lo(m[0])); // mova [dstq+strideq*0], xm0
                st128_p4(at(g.dst, g.stride), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+strideq*1], m0, 1
                g.dst = at(g.dst, g.stride * 2); // lea dstq, [dstq+strideq*2]
                g.h -= 2; // sub hd, 2
                if g.h <= 0 {
                    break; // jg .w16_leftonly_loop
                }
            }
        }
        // .w16_end:
        g.r8 -= 1 << 8; // sub r8d, 1<<8
        if g.r8 < 0 {
            return; // jl .w16_ret ; RET
        }
        m[0] = bcd_p4(at(fp, 156)); // vpbroadcastd m0, [rsp+156]
        m[0] = _mm256_add_epi16(m[0], ld256_p4(at(fp, 160))); // paddw m0, [rsp+160] ; base_y += 16*dy
        m[6] = _mm256_add_epi16(m[13], ld256_p4(at(fp, 192))); // paddw m6, m13, [rsp+192]
        r7 = at(r7, 16); // add r7, 16
        r9 += 16 << 6; // add r9d, 16<<6
        g.h = g.r8 & 0xff; // movzx hd, r8b
        g.dst = r7; // mov dstq, r7
        m[6] = _mm256_add_epi16(m[6], m[13]); // paddw m6, m13 ; base_x += 16*64
    } // jmp .w16_loop0
}

/// `.w32` up to `.w32_filter_above`; true where the assembly falls into `.w32_filter_above`, false where it jumps
/// to `.w16_main`.
#[inline(always)]
unsafe fn ipred_z2_8bpc_w32(m: &mut M, g: &mut GprP4) -> bool {
    let tl = g.tl;
    m[2] = ld256_p4(tl.wrapping_offset(32)); // mova m2, [tlq+32]
    g.r8 = g.h + (1 << 8); // lea r8d, [hq+(1<<8)]
    st256_p4(at(g.fp, 96), m[2]); // mova [rsp+96], m2
    if g.angle & 0x400 != 0 {
        return false; // test angled, 0x400 ; jnz .w16_main
    }
    m[7] = bcd_p4(ro(z_filter_k, 4 * 2 + 12 * 0)); // vpbroadcastd m7, [base+z_filter_k+4*2+12*0]
    m[8] = bcd_p4(ro(z_filter_k, 4 * 2 + 12 * 1)); // vpbroadcastd m8, [base+z_filter_k+4*2+12*1]
    m[9] = bcd_p4(ro(z_filter_k, 4 * 2 + 12 * 2)); // vpbroadcastd m9, [base+z_filter_k+4*2+12*2]
    m[5] = zx(ld128_p4(ro(dav1d_z_filter_s, 0))); // mova xm5, [base+z_filter_s]
    m[5] = ins1_p4(m[5], ro(dav1d_z_filter_s, 10)); // vinserti128 m5, [base+z_filter_s+10], 1 ; 00 01 12 23 34 45 56 67   45 56 67 78 89 9a ab bc
    m[1] = ins1_p4(m[1], tl.wrapping_offset(11)); // vinserti128 m1, [tlq+11], 1
    m[6] = zx(ld128_p4(ro(dav1d_z_filter_s, 12))); // movu xm6, [base+z_filter_s+12]
    m[6] = ins1_p4(m[6], ro(dav1d_z_filter_s, 22)); // vinserti128 m6, [base+z_filter_s+22], 1 ; 56 67 78 89 9a ab bc cd   ab bc cd de ef ff ff ff
    m[3] = zx(ld128_p4(tl.wrapping_offset(6))); // movu xm3, [tlq+ 6]
    m[3] = ins1_p4(m[3], tl.wrapping_offset(17)); // vinserti128 m3, [tlq+17], 1
    m[10] = _mm256_set1_epi32(g.max_width); // vpbroadcastd m10, r6m ; max_width
    m[10] = _mm256_packs_epi32(m[10], m[10]); // packssdw m10, m10
    m[10] = _mm256_packs_epi16(m[10], m[10]); // packsswb m10, m10
    true
}

/// `.w32_filter_above` up to `jmp .w16_filter_left`.
#[inline(always)]
unsafe fn ipred_z2_8bpc_w32_filter_above(m: &mut M, g: &mut GprP4) {
    m[0] = _mm256_shuffle_epi8(m[1], m[5]); // pshufb m0, m1, m5
    m[4] = shufps_p4::<0x49>(m[5], m[6]); // shufps m4, m5, m6, q1021 ; 12 23 34 45 56 67 78 89   67 78 89 9a ab bc cd de
    m[0] = _mm256_maddubs_epi16(m[0], m[7]); // pmaddubsw m0, m7
    m[2] = _mm256_shuffle_epi8(m[1], m[4]); // pshufb m2, m1, m4
    m[5] = shufps_p4::<0x9e>(m[5], m[6]); // shufps m5, m6, q2132 ; 34 45 56 67 78 89 9a ab   89 9a ab bc cd de ef ff
    m[2] = _mm256_maddubs_epi16(m[2], m[8]); // pmaddubsw m2, m8
    m[1] = _mm256_shuffle_epi8(m[1], m[5]); // pshufb m1, m5
    m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
    m[1] = _mm256_shuffle_epi8(m[3], m[4]); // pshufb m1, m3, m4
    m[1] = _mm256_maddubs_epi16(m[1], m[7]); // pmaddubsw m1, m7
    m[2] = _mm256_shuffle_epi8(m[3], m[5]); // pshufb m2, m3, m5
    m[2] = _mm256_maddubs_epi16(m[2], m[8]); // pmaddubsw m2, m8
    m[3] = _mm256_shuffle_epi8(m[3], m[6]); // pshufb m3, m6
    m[3] = _mm256_maddubs_epi16(m[3], m[9]); // pmaddubsw m3, m9
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[1] = _mm256_add_epi16(m[1], m[3]); // paddw m1, m3
    m[0] = _mm256_mulhrs_epi16(m[0], m[13]); // pmulhrsw m0, m13
    m[1] = _mm256_mulhrs_epi16(m[1], m[13]); // pmulhrsw m1, m13
    m[10] = _mm256_sub_epi8(m[10], ld256_p4(ro(pb_1to32, 0))); // psubb m10, [base+pb_1to32]
    m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
    m[0] = _mm256_blendv_epi8(m[0], ld256_p4(g.tl.wrapping_offset(1)), m[10]); // vpblendvb m0, [tlq+1], m10
    st256_p4(at(g.fp, 65), m[0]); // movu [rsp+65], m0
} // jmp .w16_filter_left

/// `.w64` up to `jmp .w32_filter_above`; true where the assembly continues at `.w32_filter_above`, false where it
/// jumps to `.w16_main`.
#[inline(always)]
unsafe fn ipred_z2_8bpc_w64(m: &mut M, g: &mut GprP4) -> bool {
    let tl = g.tl;
    m[2] = ld256_p4(tl.wrapping_offset(32)); // mova m2, [tlq+32]
    g.r3 = rd32_p4(tl.wrapping_offset(64)); // mov r3d, [tlq+64]
    g.r8 = g.h + (3 << 8); // lea r8d, [hq+(3<<8)]
    st256_p4(at(g.fp, 96), m[2]); // mova [rsp+ 96], m2
    st32_p4(at(g.fp, 128), g.r3); // mov [rsp+128], r3d
    if g.angle & 0x400 != 0 {
        return false; // test angled, 0x400 ; jnz .w16_main
    }
    m[7] = bcd_p4(ro(z_filter_k, 4 * 2 + 12 * 0)); // vpbroadcastd m7, [base+z_filter_k+4*2+12*0]
    m[8] = bcd_p4(ro(z_filter_k, 4 * 2 + 12 * 1)); // vpbroadcastd m8, [base+z_filter_k+4*2+12*1]
    m[9] = bcd_p4(ro(z_filter_k, 4 * 2 + 12 * 2)); // vpbroadcastd m9, [base+z_filter_k+4*2+12*2]
    m[6] = zx(ld128_p4(ro(dav1d_z_filter_s, 4))); // movu xm6, [base+z_filter_s+ 4]
    m[6] = ins1_p4(m[6], ro(dav1d_z_filter_s, 10)); // vinserti128 m6, [base+z_filter_s+10], 1 ; 12 23 34 45 56 67 78 89   45 56 67 78 89 9a ab bc
    m[3] = zx(ld128_p4(tl.wrapping_offset(30))); // movu xm3, [tlq+30]
    m[3] = ins1_p4(m[3], tl.wrapping_offset(43)); // vinserti128 m3, [tlq+43], 1
    m[5] = zx(ld128_p4(ro(dav1d_z_filter_s, 16))); // movu xm5, [base+z_filter_s+16]
    m[5] = ins1_p4(m[5], ro(dav1d_z_filter_s, 22)); // vinserti128 m5, [base+z_filter_s+22], 1 ; 78 89 9a ab bc cd de ef   ab bc cd de ef ff ff ff
    m[0] = _mm256_shuffle_epi8(m[3], m[6]); // pshufb m0, m3, m6
    m[4] = shufps_p4::<0x49>(m[6], m[5]); // shufps m4, m6, m5, q1021 ; 34 45 56 67 78 89 9a ab   67 78 89 9a ab bc cd de
    m[0] = _mm256_maddubs_epi16(m[0], m[7]); // pmaddubsw m0, m7
    m[2] = _mm256_shuffle_epi8(m[3], m[4]); // pshufb m2, m3, m4
    m[6] = shufps_p4::<0x9e>(m[6], m[5]); // shufps m6, m5, q2132 ; 56 67 78 89 9a ab bc cd   89 9a ab bc cd de ef ff
    m[2] = _mm256_maddubs_epi16(m[2], m[8]); // pmaddubsw m2, m8
    m[3] = _mm256_shuffle_epi8(m[3], m[6]); // pshufb m3, m6
    m[3] = _mm256_maddubs_epi16(m[3], m[9]); // pmaddubsw m3, m9
    m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
    m[0] = _mm256_add_epi16(m[0], m[3]); // paddw m0, m3
    m[2] = zx(ld128_p4(tl.wrapping_offset(36))); // movu xm2, [tlq+36]
    m[2] = ins1_p4(m[2], tl.wrapping_offset(49)); // vinserti128 m2, [tlq+49], 1
    m[10] = _mm256_set1_epi32(g.max_width); // vpbroadcastd m10, r6m ; max_width
    m[4] = _mm256_shuffle_epi8(m[2], m[4]); // pshufb m4, m2, m4
    m[4] = _mm256_maddubs_epi16(m[4], m[7]); // pmaddubsw m4, m7
    m[3] = _mm256_shuffle_epi8(m[2], m[6]); // pshufb m3, m2, m6
    m[3] = _mm256_maddubs_epi16(m[3], m[8]); // pmaddubsw m3, m8
    m[2] = _mm256_shuffle_epi8(m[2], m[5]); // pshufb m2, m5
    m[2] = _mm256_maddubs_epi16(m[2], m[9]); // pmaddubsw m2, m9
    m[10] = _mm256_packs_epi32(m[10], m[10]); // packssdw m10, m10
    m[3] = _mm256_add_epi16(m[3], m[4]); // paddw m3, m4
    m[2] = _mm256_add_epi16(m[2], m[3]); // paddw m2, m3
    m[3] = bcd_p4(ro(pb_32_p4, 0)); // vpbroadcastd m3, [base+pb_32]
    m[0] = _mm256_mulhrs_epi16(m[0], m[13]); // pmulhrsw m0, m13
    m[2] = _mm256_mulhrs_epi16(m[2], m[13]); // pmulhrsw m2, m13
    m[10] = _mm256_packs_epi16(m[10], m[10]); // packsswb m10, m10
    m[5] = zx(ld128_p4(ro(dav1d_z_filter_s, 0))); // mova xm5, [base+z_filter_s]
    m[5] = ins1_p4(m[5], ro(dav1d_z_filter_s, 6)); // vinserti128 m5, [base+z_filter_s+6], 1
    m[3] = _mm256_sub_epi8(m[10], m[3]); // psubb m3, m10, m3
    m[3] = _mm256_sub_epi8(m[3], ld256_p4(ro(pb_1to32, 0))); // psubb m3, [base+pb_1to32]
    m[1] = ins1_p4(m[1], tl.wrapping_offset(13)); // vinserti128 m1, [tlq+13], 1
    m[0] = _mm256_packus_epi16(m[0], m[2]); // packuswb m0, m2
    m[0] = _mm256_blendv_epi8(m[0], ld256_p4(tl.wrapping_offset(33)), m[3]); // vpblendvb m0, [tlq+33], m3
    m[3] = zx(ld128_p4(tl.wrapping_offset(6))); // movu xm3, [tlq+ 6]
    m[3] = ins1_p4(m[3], tl.wrapping_offset(19)); // vinserti128 m3, [tlq+19], 1
    st256_p4(at(g.fp, 97), m[0]); // movu [rsp+97], m0
    true // jmp .w32_filter_above
}

// ---- part 5: `ipred_avx2.asm` lines 2988–4140 (z3) ----
//
// `cglobal ipred_z3_8bpc` is `dav1d_ipred_z3_8bpc_avx2`: the prologue up to `jmp hq`, then the jump table
// `ipred_z3_avx2_table` (h4 … h64) as a `match` on `tzcnt(h)`. Each `.hN` path is `ipred_z3_8bpc_hN` and its
// labels are functions named after them (`ipred_z3_8bpc_h4_no_upsample`, `ipred_z3_8bpc_h8_main`, …); the
// `call`ed `.filter_strength` is `ipred_z3_8bpc_filter_strength`. The general-purpose registers are locals named
// as the `cglobal` line names them (`dy`, `org_w`, `maxbase`, `r4`, …). The filtered edges (`ALLOC_STACK`) are
// zeroed locals with 64 bytes below and 32 above the assembly's frame (the main loops read beyond the frame and
// blend those lanes away); the upsampled edges are 32-byte locals written whole before the first load. The rows
// the main loops push below the frame (`sub rsp, …` / `mova [rsp], …`) for the transposes go into a local array
// of exactly the slots one block pushes (the paths are instantiated per width for that), filled top down.

/// `[p+off]`, 16 bytes (`movu`/`mova xm`, the memory operand of `vinserti128`).
#[inline(always)]
unsafe fn ld16_p5(p: *const u8, off: isize) -> __m128i {
    _mm_loadu_si128(p.wrapping_offset(off).cast())
}

/// `[p+off]`, 32 bytes.
#[inline(always)]
unsafe fn ld32_p5(p: *const u8, off: isize) -> __m256i {
    _mm256_loadu_si256(p.wrapping_offset(off).cast())
}

/// `[p+off]`, 8 bytes (`movq`, `movhps`, `vpbroadcastq` operands).
#[inline(always)]
unsafe fn ld8_p5(p: *const u8, off: isize) -> i64 {
    p.wrapping_offset(off).cast::<i64>().read_unaligned()
}

/// `vpbroadcastd m, [p]`.
#[inline(always)]
unsafe fn bcd_p5(p: *const u8) -> __m256i {
    _mm256_set1_epi32(p.cast::<i32>().read_unaligned())
}

/// `vbroadcasti128 m, [p+off]`.
#[inline(always)]
unsafe fn bci_p5(p: *const u8, off: isize) -> __m256i {
    _mm256_broadcastsi128_si256(ld16_p5(p, off))
}

/// `shufps a, b, imm` on integer vectors.
#[inline(always)]
unsafe fn shufps_p5<const IMM: i32>(a: __m256i, b: __m256i) -> __m256i {
    _mm256_castps_si256(_mm256_shuffle_ps::<IMM>(_mm256_castsi256_ps(a), _mm256_castsi256_ps(b)))
}

/// `movd [p0], x` / `pextrd [p1], x, 1` / `pextrd [p2], x, 2` / `pextrd [p3], x, 3`: four rows of four pixels.
#[inline(always)]
unsafe fn st4x4_p5(p: [*mut u8; 4], x: __m128i) {
    p[0].cast::<i32>().write_unaligned(_mm_cvtsi128_si32(x)); // movd   [p0], x
    p[1].cast::<i32>().write_unaligned(_mm_extract_epi32::<1>(x)); // pextrd [p1], x, 1
    p[2].cast::<i32>().write_unaligned(_mm_extract_epi32::<2>(x)); // pextrd [p2], x, 2
    p[3].cast::<i32>().write_unaligned(_mm_extract_epi32::<3>(x)); // pextrd [p3], x, 3
}

/// `movq [p], x`.
#[inline(always)]
unsafe fn movq_st_p5(p: *mut u8, x: __m128i) {
    _mm_storel_epi64(p.cast(), x)
}

/// `movhps [p], x`.
#[inline(always)]
unsafe fn movhps_st_p5(p: *mut u8, x: __m128i) {
    p.cast::<i64>().write_unaligned(_mm_extract_epi64::<1>(x))
}

/// `movhps x, [p+off]`.
#[inline(always)]
unsafe fn movhps_ld_p5(x: __m128i, p: *const u8, off: isize) -> __m128i {
    _mm_insert_epi64::<1>(x, ld8_p5(p, off))
}

/// `ipred_z3_8bpc` (`cglobal ipred_z3_8bpc, 4, 9, 0, dst, stride, tl, w, h, angle, dy, org_w, maxbase`).
///
/// # Safety
/// As for the family (module documentation), and the CPU supports AVX2 (the init selects this function under
/// `DAV1D_X86_CPU_FLAG_AVX2`).
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_z3_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    angle: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    // Not in the original: the kernel loads down to `topleft[-191]`, before the caller's array; it runs on a copy
    // of the pixels the C kernel reads, `topleft[-(width+height)..=0]`, with zeros around them (`copy_edge`).
    let mut edge = [0u8; 256 + 64];
    let topleft = crate::simd::copy_edge(&mut edge, 256, topleft, -((width + height) as isize), 0);
    let mut m = regs();
    let m = &mut m;
    // lea r6, [ipred_z3_avx2_table]
    let h = (height as u32).trailing_zeros(); // tzcnt hd, hm (h is a power of two >= 4, never zero)
                                              // movifnidn angled, anglem
                                              // lea r7, [dr_intra_derivative+45*2-1]
    let r7 = core::ptr::addr_of!(crate::common::tables::dav1d_dr_intra_derivative)
        .cast::<u8>()
        .wrapping_offset(45 * 2 - 1);
    let tl = topleft.wrapping_offset(-1); // dec tlq
                                          // movsxd hq, [r6+hq*4] / add hq, r6: the jump table, the `match` below
    let mut angle = angle.wrapping_sub(180); // sub angled, 180
    let mut dy = angle as u32; // mov dyd, angled
    dy = dy.wrapping_neg(); // neg dyd
    angle ^= 0x400; // xor angled, 0x400
    let dyq = (dy as i64) | !0x7e; // or dyq, ~0x7e
                                   // movzx dyd, word [r7+dyq]: `r7 + dyq` is dr_intra_derivative + ((180 - angle) & 0x7e) - 38, the C kernel's
                                   // dr_intra_derivative[(270 - angle) >> 1]; inside the table for the contract's angles (181..=269 without flags).
    let dy = r7.wrapping_offset(dyq as isize).cast::<u16>().read_unaligned() as i64;
    m[3] = bcd_p5(ro(pw_512, 0)); // vpbroadcastd m3, [pw_512]
    m[4] = bcd_p5(ro(pw_62, 0)); // vpbroadcastd m4, [pw_62]
    m[5] = bcd_p5(ro(pw_64, 0)); // vpbroadcastd m5, [pw_64]
    let org_w = width; // mov org_wd, wd
    match h {
        // jmp hq
        2 => ipred_z3_8bpc_h4(m, dst, stride, tl, width, angle, dy),
        3 => ipred_z3_8bpc_h8(m, dst, stride, tl, width, angle, dy, org_w),
        4 => ipred_z3_8bpc_h16(m, dst, stride, tl, width, angle, dy, org_w),
        5 => ipred_z3_8bpc_h32(m, dst, stride, tl, width, angle, dy, org_w),
        _ => ipred_z3_8bpc_h64(m, dst, stride, tl, width, angle, dy, org_w),
    }
}

/// `.filter_strength` (h4/h8/h16, `call`ed): `m0` = `maxbase` in every byte, the strength mask in the return
/// value (`r5d`); `angle` becomes `is_sm << 1` (`shr angled, 8`). The `base` register `r4` is `z_filter_t0`, so
/// the callers' `[base+label]` are `ro(label, …)`.
#[inline(always)]
unsafe fn ipred_z3_8bpc_filter_strength(m: &mut M, maxbase: i32, angle: &mut i32) -> u32 {
    // %define base r4-z_filter_t0 / lea r4, [z_filter_t0]
    m[0] = zx(_mm_cvtsi32_si128(maxbase)); // movd xm0, maxbased
    m[2] = zx(_mm_cvtsi32_si128(*angle)); // movd xm2, angled
    *angle = ((*angle as u32) >> 8) as i32; // shr angled, 8 ; is_sm << 1
    m[0] = _mm256_broadcastb_epi8(lo(m[0])); // vpbroadcastb m0, xm0
    m[2] = _mm256_broadcastb_epi8(lo(m[2])); // vpbroadcastb m2, xm2
    m[1] = _mm256_cmpeq_epi8(m[0], ld32_p5(ro(z_filter_wh, 0), 0)); // pcmpeqb m1, m0, [base+z_filter_wh]
    m[1] = _mm256_and_si256(m[1], m[2]); // pand m1, m2
                                         // mova xm2, [r4+angleq*8]: z_filter_t0 or z_filter_t1 (angle is 0 or 2 here: bit 10 is clear on this path)
    m[2] = zx(ld16_p5(ro(z_filter_t0, *angle as isize * 8), 0));
    m[1] = _mm256_cmpgt_epi8(m[1], m[2]); // pcmpgtb m1, m2
    _mm256_movemask_epi8(m[1]) as u32 // pmovmskb r5d, m1
                                      // ret
}

/// `.h4`.
#[inline(always)]
unsafe fn ipred_z3_8bpc_h4(m: &mut M, dst: *mut u8, stride: isize, tl: *const u8, w: i32, angle: i32, dy: i64) {
    let r7 = stride * 3; // lea r7, [strideq*3]
    if (angle as u8) < 40 {
        // cmp angleb, 40 / jae .h4_no_upsample
        let mut r4 = angle.wrapping_sub(1024); // lea r4d, [angleq-1024]
        r4 >>= 7; // sar r4d, 7
        r4 = r4.wrapping_add(w); // add r4d, wd
        if r4 <= 0 {
            // jg .h4_no_upsample ; !enable_intra_edge_filter || w > 8 || (w == 8 && is_sm)
            return ipred_z3_8bpc_h4_upsample(m, dst, stride, tl, w, dy, r7);
        }
    }
    ipred_z3_8bpc_h4_no_upsample(m, dst, stride, tl, w, angle, dy, r7)
}

/// `.h4` from `ALLOC_STACK -32, 9` to its `RET` (the upsampled edge).
#[inline(always)]
unsafe fn ipred_z3_8bpc_h4_upsample(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut w: i32,
    mut dy: i64,
    r7: isize,
) {
    // ALLOC_STACK -32, 9: the 32-byte frame; both of its 16-byte halves are stored below ([rsp+16], then
    // [rsp]) before the loop's first load. The loop loads 8 bytes at rsp + ((k*2*dy) >> 6), k = 1..=w <= 8:
    // with d = angle - 180 < 40 on this path, dy <= 51, so at most rsp+12+8 <= rsp+32.
    let mut frame = core::mem::MaybeUninit::<[__m256i; 1]>::uninit();
    let rsp = frame.as_mut_ptr().cast::<u8>();
    m[8] = zx(ld16_p5(tl, -7)); // movu xm8, [tlq-7]
    m[0] = zx(_mm_shuffle_epi8(lo(m[8]), ld16_p5(ro(z_upsample1, -4), 0))); // pshufb xm0, xm8, [z_upsample1-4]
    m[2] = zx(_mm_broadcastb_epi8(lo(m[8]))); // vpbroadcastb xm2, xm8
    m[1] = zx(_mm_shuffle_epi8(lo(m[8]), ld16_p5(ro(dav1d_z_filter_s, 2), 0))); // pshufb xm1, xm8, [z_filter_s+2]
    _mm_store_si128(rsp.add(16).cast(), lo(m[2])); // mova [rsp+16], xm2 ; top[max_base_y]
    m[2] = zx(_mm256_castsi256_si128(bcd_p5(ro(pb_36_m4, 0)))); // vpbroadcastd xm2, [pb_36_m4]
    dy += dy; // add dyd, dyd
    m[0] = zx(_mm_maddubs_epi16(lo(m[0]), lo(m[2]))); // pmaddubsw xm0, xm2
    m[1] = zx(_mm_maddubs_epi16(lo(m[1]), lo(m[2]))); // pmaddubsw xm1, xm2
    m[7] = zx(_mm_cvtsi32_si128(dy as i32)); // movd xm7, dyd
    let mut r2 = dy as u32; // mov r2d, dyd
    m[7] = _mm256_broadcastw_epi16(lo(m[7])); // vpbroadcastw m7, xm7
    m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[0]))); // paddw xm1, xm0
    m[1] = zx(_mm_mulhrs_epi16(lo(m[1]), lo(m[3]))); // pmulhrsw xm1, xm3
    m[6] = _mm256_bslli_epi128::<8>(m[7]); // pslldq m6, m7, 8
    m[2] = zx(_mm_add_epi16(lo(m[7]), lo(m[7]))); // paddw xm2, xm7, xm7
    m[6] = _mm256_add_epi16(m[6], m[7]); // paddw m6, m7
    m[1] = zx(_mm_packus_epi16(lo(m[1]), lo(m[1]))); // packuswb xm1, xm1
    m[6] = _mm256_add_epi16(m[6], m[2]); // paddw m6, m2
    m[1] = zx(_mm_unpacklo_epi8(lo(m[1]), lo(m[8]))); // punpcklbw xm1, xm8
    m[8] = zx(ld16_p5(ro(z_transpose4, 0), 0)); // mova xm8, [z_transpose4]
    m[7] = _mm256_slli_epi16::<2>(m[7]); // psllw m7, 2
    m[1] = zx(_mm_shuffle_epi8(lo(m[1]), ld16_p5(ro(pb_15to0, 0), 0))); // pshufb xm1, [pb_15to0]
    _mm_store_si128(rsp.cast(), lo(m[1])); // mova [rsp], xm1
    let rsp = rsp as *const u8;
    loop {
        // .h4_upsample_loop:
        let mut r4 = r2.wrapping_add(dy as u32); // lea r4d, [r2+dyq]
        r2 >>= 6; // shr r2d, 6
        m[1] = _mm256_set1_epi64x(ld8_p5(rsp, r2 as isize)); // vpbroadcastq m1, [rsp+r2]
        r2 = r4.wrapping_add(dy as u32); // lea r2d, [r4+dyq]
        r4 >>= 6; // shr r4d, 6
        m[2] = _mm256_set1_epi64x(ld8_p5(rsp, r4 as isize)); // vpbroadcastq m2, [rsp+r4]
        r4 = r2.wrapping_add(dy as u32); // lea r4d, [r2+dyq]
        r2 >>= 6; // shr r2d, 6
        m[0] = zx(_mm_loadl_epi64(rsp.add(r2 as usize).cast())); // movq xm0, [rsp+r2]
        r2 = r4.wrapping_add(dy as u32); // lea r2d, [r4+dyq]
        r4 >>= 6; // shr r4d, 6
        m[0] = zx(movhps_ld_p5(lo(m[0]), rsp, r4 as isize)); // movhps xm0, [rsp+r4]
        m[1] = _mm256_blend_epi32::<0xc0>(m[1], m[2]); // vpblendd m1, m2, 0xc0
        m[2] = _mm256_and_si256(m[4], m[6]); // pand m2, m4, m6
        m[0] = _mm256_blend_epi32::<0xf0>(m[0], m[1]); // vpblendd m0, m1, 0xf0
        m[1] = _mm256_sub_epi16(m[5], m[2]); // psubw m1, m5, m2
        m[2] = _mm256_slli_epi16::<8>(m[2]); // psllw m2, 8
        m[1] = _mm256_or_si256(m[1], m[2]); // por m1, m2
        m[0] = _mm256_maddubs_epi16(m[0], m[1]); // pmaddubsw m0, m1
        m[6] = _mm256_add_epi16(m[6], m[7]); // paddw m6, m7
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        m[1] = zx(_mm_packus_epi16(lo(m[1]), lo(m[0]))); // packuswb xm1, xm0
        m[1] = zx(_mm_shuffle_epi8(lo(m[1]), lo(m[8]))); // pshufb xm1, xm8
        st4x4_p5([dst, at(dst, stride), at(dst, stride * 2), at(dst, r7)], lo(m[1])); // movd/pextrd [dstq+…], xm1
        dst = at(dst, 4); // add dstq, 4
        w -= 4; // sub wd, 4
        if w <= 0 {
            break; // jg .h4_upsample_loop
        }
    }
    // RET
}

/// `.h4_no_upsample` (with the filter of the edge into the frame) and `.h4_main`'s caller.
#[inline(always)]
unsafe fn ipred_z3_8bpc_h4_no_upsample(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    w: i32,
    mut angle: i32,
    dy: i64,
    r7: isize,
) {
    // ALLOC_STACK -16, 12: the frame (16 bytes at rsp), zeroed whole with 64 bytes below and 48 above it when
    // the edge is filtered into it: `.h4_main` loads 8 bytes at tl-4-base with tl = rsp+15 and base from 0
    // (rsp+11..rsp+18) to at most 63 (rsp-52; 4*dy >> 6 in the first iteration, dy <= 1023; later iterations
    // run only while their first base is below maxbase <= 9). The lanes outside the frame are blended away.
    let mut frame: [__m256i; 4];
    let mut maxbase: i64 = 7; // mov maxbased, 7
    if angle & 0x400 == 0 {
        // test angled, 0x400 / jnz .h4_main ; !enable_intra_edge_filter
        maxbase = (w + 3) as i64; // lea maxbased, [wq+3]
        let mut r5 = ipred_z3_8bpc_filter_strength(m, maxbase as i32, &mut angle); // call .filter_strength
        maxbase = 7; // mov maxbased, 7
        if r5 != 0 {
            // test r5d, r5d / jz .h4_main ; filter_strength == 0
            frame = [_mm256_setzero_si256(); 4];
            let rsp = frame.as_mut_ptr().cast::<u8>().add(64);
            r5 = r5.count_ones(); // popcnt r5d, r5d
            let k = r5 as isize * 4 - 4; // z_filter_k-4+r5*4 (r5 is 1..=3)
            m[7] = bcd_p5(ro(ipred_h_shuf, 0)); // vpbroadcastd m7, [base+pb_7]
            m[2] = bci_p5(tl, -14); // vbroadcasti128 m2, [tlq-14]
            m[1] = _mm256_max_epu8(m[7], ld32_p5(ro(dav1d_z_filter_s, -4), 0)); // pmaxub m1, m7, [base+z_filter_s-4]
            m[8] = bcd_p5(ro(z_filter_k, k + 12 * 0)); // vpbroadcastd m8, [base+z_filter_k-4+r5*4+12*0]
            m[7] = _mm256_max_epu8(m[7], ld32_p5(ro(dav1d_z_filter_s, 4), 0)); // pmaxub m7, [base+z_filter_s+4]
            m[9] = bcd_p5(ro(z_filter_k, k + 12 * 1)); // vpbroadcastd m9, [base+z_filter_k-4+r5*4+12*1]
            m[10] = bcd_p5(ro(z_filter_k, k + 12 * 2)); // vpbroadcastd m10, [base+z_filter_k-4+r5*4+12*2]
            m[0] = _mm256_shuffle_epi8(m[2], m[1]); // pshufb m0, m2, m1
            m[1] = shufps_p5::<0x99>(m[1], m[7]); // shufps m1, m7, q2121
            m[0] = _mm256_maddubs_epi16(m[0], m[8]); // pmaddubsw m0, m8
            m[1] = _mm256_shuffle_epi8(m[2], m[1]); // pshufb m1, m2, m1
            m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
            m[2] = _mm256_shuffle_epi8(m[2], m[7]); // pshufb m2, m7
            m[2] = _mm256_maddubs_epi16(m[2], m[10]); // pmaddubsw m2, m10
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
            m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
            let r4 = 9; // mov r4d, 9
            tl = rsp.add(15); // lea tlq, [rsp+15]
            if w != 4 {
                maxbase = r4; // cmp wd, 4 / cmovne maxbased, r4d
            }
            m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
            m[0] = zx(_mm_packus_epi16(lo(m[0]), lo(m[1]))); // packuswb xm0, xm1
            _mm_store_si128(rsp.cast(), lo(m[0])); // mova [rsp], xm0
        }
    }
    ipred_z3_8bpc_h4_main(m, dst, stride, tl, w, maxbase, dy, r7)
}

/// `.h4_main` to its `RET`.
#[inline(always)]
unsafe fn ipred_z3_8bpc_h4_main(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    mut w: i32,
    maxbase: i64,
    mut dy: i64,
    r7: isize,
) {
    m[6] = zx(_mm_cvtsi32_si128(dy as i32)); // movd xm6, dyd
    m[0] = _mm256_set1_epi64x(ld8_p5(ro(z_base_inc, 0), 0)); // vpbroadcastq m0, [z_base_inc] ; base_inc << 6
    let r4 = tl; // mov r4, tlq
    tl = tl.wrapping_offset(-4); // sub tlq, 4
    dy = -dy; // neg dyq
    m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
    let r4 = r4.wrapping_offset(-maxbase as isize); // sub r4, maxbaseq
    let mut mb = (maxbase as i32) << 6; // shl maxbased, 6
    m[7] = _mm256_set1_epi8(*r4 as i8); // vpbroadcastb m7, [r4]
    let mut r4 = dy + 63; // lea r4, [dyq+63] ; ypos
    m[9] = zx(_mm_cvtsi32_si128(mb)); // movd xm9, maxbased
    mb = !mb; // not maxbased
    m[8] = bci_p5(ro(z3_shuf_w4, 0), 0); // vbroadcasti128 m8, [z3_shuf_w4]
    mb = mb.wrapping_add(64); // add maxbased, 64
    m[9] = _mm256_broadcastw_epi16(lo(m[9])); // vpbroadcastw m9, xm9
    m[7] = _mm256_srli_epi16::<8>(m[7]); // psrlw m7, 8 ; top[max_base_y]
    m[10] = _mm256_add_epi16(m[6], m[6]); // paddw m10, m6, m6
    m[9] = _mm256_sub_epi16(m[9], m[0]); // psubw m9, m0 ; max_base_y
    m[6] = _mm256_blend_epi32::<0xcc>(m[6], m[10]); // vpblendd m6, m10, 0xcc
    m[0] = zx(lo(m[10])); // mova xm0, xm10
    m[6] = _mm256_add_epi16(m[6], m[0]); // paddw m6, m0 ; ypos2 ypos3 ypos0 ypos1
    m[10] = _mm256_add_epi16(m[10], m[10]); // paddw m10, m10
    m[11] = zx(ld16_p5(ro(z_transpose4, 0), 0)); // mova xm11, [z_transpose4]
    loop {
        // .h4_loop:
        let mut r5 = r4 + dy; // lea r5, [r4+dyq]
        r4 >>= 6; // sar r4, 6 ; base0
        m[1] = _mm256_set1_epi64x(ld8_p5(tl, r4 as isize)); // vpbroadcastq m1, [tlq+r4]
        r4 = r5 + dy; // lea r4, [r5+dyq]
        r5 >>= 6; // sar r5, 6 ; base1
        m[2] = _mm256_set1_epi64x(ld8_p5(tl, r5 as isize)); // vpbroadcastq m2, [tlq+r5]
        r5 = r4 + dy; // lea r5, [r4+dyq]
        r4 >>= 6; // sar r4, 6 ; base2
        m[0] = zx(_mm_loadl_epi64(tl.wrapping_offset(r4 as isize).cast())); // movq xm0, [tlq+r4]
        r4 = r5 + dy; // lea r4, [r5+dyq]
        r5 >>= 6; // sar r5, 6 ; base3
        m[0] = zx(movhps_ld_p5(lo(m[0]), tl, r5 as isize)); // movhps xm0, [tlq+r5]
        m[1] = _mm256_blend_epi32::<0xc0>(m[1], m[2]); // vpblendd m1, m2, 0xc0
        m[2] = _mm256_and_si256(m[4], m[6]); // pand m2, m4, m6 ; frac
        m[0] = _mm256_blend_epi32::<0xf0>(m[0], m[1]); // vpblendd m0, m1, 0xf0
        m[1] = _mm256_sub_epi16(m[5], m[2]); // psubw m1, m5, m2 ; 64-frac
        m[2] = _mm256_slli_epi16::<8>(m[2]); // psllw m2, 8
        m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
        m[1] = _mm256_or_si256(m[1], m[2]); // por m1, m2 ; 64-frac, frac
        m[0] = _mm256_maddubs_epi16(m[0], m[1]); // pmaddubsw m0, m1
        m[1] = _mm256_cmpgt_epi16(m[9], m[6]); // pcmpgtw m1, m9, m6 ; base < max_base_y
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[6] = _mm256_add_epi16(m[6], m[10]); // paddw m6, m10 ; ypos += dy
        m[0] = _mm256_blendv_epi8(m[7], m[0], m[1]); // vpblendvb m0, m7, m0, m1
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        m[1] = zx(_mm_packus_epi16(lo(m[1]), lo(m[0]))); // packuswb xm1, xm0
        m[1] = zx(_mm_shuffle_epi8(lo(m[1]), lo(m[11]))); // pshufb xm1, xm11 ; transpose
        st4x4_p5([dst, at(dst, stride), at(dst, stride * 2), at(dst, r7)], lo(m[1])); // movd/pextrd [dstq+…], xm1
        w -= 4; // sub wd, 4
        if w == 0 {
            return; // jz .h4_end
        }
        dst = at(dst, 4); // add dstq, 4
        if r4 as i32 <= mb {
            break; // cmp r4d, maxbased / jg .h4_loop
        }
    }
    m[7] = zx(_mm_packus_epi16(lo(m[7]), lo(m[7]))); // packuswb xm7, xm7
    let mut dst = dst;
    loop {
        // .h4_end_loop:
        let x = _mm_cvtsi128_si32(lo(m[7]));
        dst.cast::<i32>().write_unaligned(x); // movd [dstq+strideq*0], xm7
        at(dst, stride).cast::<i32>().write_unaligned(x); // movd [dstq+strideq*1], xm7
        at(dst, stride * 2).cast::<i32>().write_unaligned(x); // movd [dstq+strideq*2], xm7
        at(dst, r7).cast::<i32>().write_unaligned(x); // movd [dstq+r7       ], xm7
        dst = at(dst, 4); // add dstq, 4
        w -= 4; // sub wd, 4
        if w <= 0 {
            break; // jg .h4_end_loop
        }
    }
    // .h4_end: RET
}

/// `.h8`.
#[inline(always)]
unsafe fn ipred_z3_8bpc_h8(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    tl: *const u8,
    w: i32,
    angle: i32,
    dy: i64,
    org_w: i32,
) {
    let mut r4 = angle.wrapping_add(216) as u32; // lea r4d, [angleq+216]
    r4 = (r4 & !0xff) | (w as u32 & 0xff); // mov r4b, wb
    if r4 > 8 {
        // cmp r4d, 8 / ja .h8_no_upsample ; !enable_intra_edge_filter || is_sm || d >= 40 || w > 8
        return ipred_z3_8bpc_h8_no_upsample(m, dst, stride, tl, w, angle, dy, org_w);
    }
    ipred_z3_8bpc_h8_upsample(m, dst, stride, tl, w, dy, r4)
}

/// `.h8` from `ALLOC_STACK -32, 8` to its `RET` (the upsampled edge).
#[inline(always)]
unsafe fn ipred_z3_8bpc_h8_upsample(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut w: i32,
    mut dy: i64,
    mut r4: u32,
) {
    // ALLOC_STACK -32, 8: the 32-byte frame, written whole ([rsp+0] and [rsp+16]) before the loop's first load.
    // The loop loads 16 bytes at rsp + ((k*2*dy) >> 6), k = 1..=w <= 8: with d < 40 here, dy <= 51, so at most
    // rsp+12+16 <= rsp+32.
    let mut frame = core::mem::MaybeUninit::<[__m256i; 1]>::uninit();
    let rsp = frame.as_mut_ptr().cast::<u8>();
    r4 &= 4; // and r4d, 4
    m[0] = zx(ld16_p5(tl, -15)); // mova xm0, [tlq-15]
    m[0] = _mm256_inserti128_si256::<1>(m[0], ld16_p5(tl, -9)); // vinserti128 m0, [tlq- 9], 1
    m[1] = zx(_mm_cvtsi32_si128(r4 as i32)); // movd xm1, r4d
    m[2] = zx(ld16_p5(ro(dav1d_z_filter_s, 2), 0)); // movu xm2, [z_filter_s+2]
    m[2] = _mm256_inserti128_si256::<1>(m[2], ld16_p5(ro(dav1d_z_filter_s, 6), 0)); // vinserti128 m2, [z_filter_s+6], 1
    m[1] = zx(_mm_broadcastb_epi8(lo(m[1]))); // vpbroadcastb xm1, xm1 ; w & 4
    m[7] = bcd_p5(ro(pb_36_m4, 0)); // vpbroadcastd m7, [pb_36_m4]
    m[1] = zx(_mm_max_epu8(lo(m[1]), ld16_p5(ro(z_upsample1, -4), 0))); // pmaxub xm1, [z_upsample1-4] ; clip 4x8
    m[1] = _mm256_inserti128_si256::<1>(m[1], ld16_p5(ro(z_upsample1, 0), 0)); // vinserti128 m1, [z_upsample1], 1
    dy += dy; // add dyd, dyd
    m[1] = _mm256_shuffle_epi8(m[0], m[1]); // pshufb m1, m0, m1
    m[2] = _mm256_shuffle_epi8(m[0], m[2]); // pshufb m2, m0, m2
    m[0] = _mm256_inserti128_si256::<1>(m[0], ld16_p5(tl, -7)); // vinserti128 m0, [tlq-7], 1
    m[6] = zx(_mm_cvtsi32_si128(dy as i32)); // movd xm6, dyd
    m[1] = _mm256_maddubs_epi16(m[1], m[7]); // pmaddubsw m1, m7
    m[2] = _mm256_maddubs_epi16(m[2], m[7]); // pmaddubsw m2, m7
    m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
    let mut r2 = dy as u32; // mov r2d, dyd
    let r5 = stride * 3; // lea r5, [strideq*3]
    m[7] = _mm256_add_epi16(m[6], m[6]); // paddw m7, m6, m6
    m[1] = _mm256_add_epi16(m[1], m[2]); // paddw m1, m2
    m[6] = _mm256_blend_epi32::<0xf0>(m[6], m[7]); // vpblendd m6, m7, 0xf0
    m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
    m[2] = _mm256_bslli_epi128::<8>(m[7]); // pslldq m2, m7, 8
    m[7] = _mm256_add_epi16(m[7], m[7]); // paddw m7, m7
    m[6] = _mm256_add_epi16(m[6], m[2]); // paddw m6, m2
    m[2] = bci_p5(ro(pb_15to0, 0), 0); // vbroadcasti128 m2, [pb_15to0]
    m[1] = _mm256_packus_epi16(m[1], m[1]); // packuswb m1, m1
    m[1] = _mm256_unpacklo_epi8(m[1], m[0]); // punpcklbw m1, m0
    m[1] = _mm256_shuffle_epi8(m[1], m[2]); // pshufb m1, m2
    _mm_store_si128(rsp.cast(), _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [rsp+ 0], m1, 1
    _mm_store_si128(rsp.add(16).cast(), lo(m[1])); // mova [rsp+16], xm1
    let rsp = rsp as *const u8;
    loop {
        // .h8_upsample_loop:
        let mut r4 = r2.wrapping_add(dy as u32); // lea r4d, [r2+dyq]
        r2 >>= 6; // shr r2d, 6 ; base0
        m[0] = zx(ld16_p5(rsp, r2 as isize)); // movu xm0, [rsp+r2]
        r2 = r4.wrapping_add(dy as u32); // lea r2d, [r4+dyq]
        r4 >>= 6; // shr r4d, 6 ; base1
        m[0] = _mm256_inserti128_si256::<1>(m[0], ld16_p5(rsp, r4 as isize)); // vinserti128 m0, [rsp+r4], 1
        r4 = r2.wrapping_add(dy as u32); // lea r4d, [r2+dyq]
        r2 >>= 6; // shr r2d, 6 ; base2
        m[1] = _mm256_and_si256(m[4], m[6]); // pand m1, m4, m6
        m[2] = _mm256_sub_epi16(m[5], m[1]); // psubw m2, m5, m1
        m[1] = _mm256_slli_epi16::<8>(m[1]); // psllw m1, 8
        m[2] = _mm256_or_si256(m[2], m[1]); // por m2, m1
        m[1] = _mm256_unpacklo_epi64(m[2], m[2]); // punpcklqdq m1, m2, m2 ; frac0 frac1
        m[0] = _mm256_maddubs_epi16(m[0], m[1]); // pmaddubsw m0, m1
        m[1] = zx(ld16_p5(rsp, r2 as isize)); // movu xm1, [rsp+r2]
        r2 = r4.wrapping_add(dy as u32); // lea r2d, [r4+dyq]
        r4 >>= 6; // shr r4d, 6 ; base3
        m[1] = _mm256_inserti128_si256::<1>(m[1], ld16_p5(rsp, r4 as isize)); // vinserti128 m1, [rsp+r4], 1
        m[2] = _mm256_unpackhi_epi64(m[2], m[2]); // punpckhqdq m2, m2 ; frac2 frac3
        m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[6] = _mm256_add_epi16(m[6], m[7]); // paddw m6, m7
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        let r4 = at(dst, stride * 4); // lea r4, [dstq+strideq*4]
        m[1] = _mm256_slli_epi16::<8>(m[1]); // psllw m1, 8
        m[0] = _mm256_or_si256(m[0], m[1]); // por m0, m1
        m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
        m[2] = zx(_mm_unpacklo_epi8(lo(m[0]), lo(m[1]))); // punpcklbw xm2, xm0, xm1
        m[0] = zx(_mm_unpackhi_epi8(lo(m[0]), lo(m[1]))); // punpckhbw xm0, xm1
        st4x4_p5([dst, at(dst, stride), at(dst, stride * 2), at(dst, r5)], lo(m[2])); // movd/pextrd [dstq+…], xm2
        st4x4_p5([r4, at(r4, stride), at(r4, stride * 2), at(r4, r5)], lo(m[0])); // movd/pextrd [r4+…], xm0
        dst = at(dst, 4); // add dstq, 4
        w -= 4; // sub wd, 4
        if w <= 0 {
            break; // jg .h8_upsample_loop
        }
    }
    // RET
}

/// `.h8_no_upsample` (with `.h8_no_intra_edge_filter`, the filter of the edge into the frame, `.h8_3tap`) and
/// `.h8_main`'s caller.
#[inline(always)]
unsafe fn ipred_z3_8bpc_h8_no_upsample(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    w: i32,
    mut angle: i32,
    dy: i64,
    org_w: i32,
) {
    // ALLOC_STACK -32, 10: the frame (32 bytes at rsp), zeroed whole with 64 bytes below and 32 above it when
    // the edge is filtered into it (rsp+14..rsp+32 written): `.h8_main` loads 16 bytes at tl-8-base with
    // tl = rsp+31 and base from 0 (up to rsp+38) to at most 32 (rsp-9; 2*dy >> 6 in the first iteration, later
    // the first base of an iteration is below maxbase <= 17). The lanes outside the written bytes are blended away.
    let mut frame: [__m256i; 4];
    let mut maxbase = w + 7; // lea maxbased, [wq+7]
    if angle & 0x400 != 0 {
        // test angled, 0x400 / jnz .h8_no_intra_edge_filter
        // .h8_no_intra_edge_filter:
        maxbase &= 7; // and maxbased, 7
        maxbase |= 8; // or maxbased, 8 ; imin(w+7, 15)
                      // jmp .h8_main
    } else {
        let mut r5 = ipred_z3_8bpc_filter_strength(m, maxbase, &mut angle); // call .filter_strength
        if r5 != 0 {
            // test r5d, r5d / jz .h8_main ; filter_strength == 0
            frame = [_mm256_setzero_si256(); 4];
            let rsp = frame.as_mut_ptr().cast::<u8>().add(64);
            r5 = r5.count_ones(); // popcnt r5d, r5d
            let k = r5 as isize * 4 - 4; // z_filter_k-4+r5*4 (r5 is 1..=3)
            m[6] = zx(_mm256_castsi256_si128(bcd_p5(ro(dav1d_z_filter_s, 32)))); // vpbroadcastd xm6, [base+pb_15]
            m[1] = zx(_mm_cmpeq_epi8(lo(m[1]), lo(m[1]))); // pcmpeqb xm1, xm1
            m[6] = zx(_mm_subs_epu8(lo(m[6]), lo(m[0]))); // psubusb xm6, xm0
            m[6] = zx(_mm_sub_epi8(lo(m[6]), lo(m[1]))); // psubb xm6, xm1 ; w == 4 ? 5 : 1
            m[2] = zx(ld16_p5(tl, -16)); // movu xm2, [tlq-16]
            m[1] = zx(_mm_max_epu8(lo(m[6]), ld16_p5(ro(dav1d_z_filter_s, 0), 0))); // pmaxub xm1, xm6, [base+z_filter_s]
            m[2] = _mm256_inserti128_si256::<1>(m[2], ld16_p5(tl, -14)); // vinserti128 m2, [tlq-14], 1
            m[1] = _mm256_inserti128_si256::<1>(m[1], ld16_p5(ro(dav1d_z_filter_s, 12), 0)); // vinserti128 m1, [base+z_filter_s+12], 1
            m[7] = bcd_p5(ro(z_filter_k, k + 12 * 0)); // vpbroadcastd m7, [base+z_filter_k-4+r5*4+12*0]
            m[6] = zx(_mm_max_epu8(lo(m[6]), ld16_p5(ro(dav1d_z_filter_s, 8), 0))); // pmaxub xm6, [base+z_filter_s+ 8]
            m[6] = _mm256_inserti128_si256::<1>(m[6], ld16_p5(ro(dav1d_z_filter_s, 20), 0)); // vinserti128 m6, [base+z_filter_s+20], 1
            m[0] = _mm256_shuffle_epi8(m[2], m[1]); // pshufb m0, m2, m1
            m[0] = _mm256_maddubs_epi16(m[0], m[7]); // pmaddubsw m0, m7
            m[7] = bcd_p5(ro(z_filter_k, k + 12 * 1)); // vpbroadcastd m7, [base+z_filter_k-4+r5*4+12*1]
            let r4 = *tl.wrapping_offset(-15) as u32; // movzx r4d, byte [tlq-15]
            m[1] = shufps_p5::<0x99>(m[1], m[6]); // shufps m1, m6, q2121
            m[1] = _mm256_shuffle_epi8(m[2], m[1]); // pshufb m1, m2, m1
            m[1] = _mm256_maddubs_epi16(m[1], m[7]); // pmaddubsw m1, m7
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            let mut r5 = (r5 as i32).wrapping_sub(3); // sub r5d, 3
            if r5 == 0 {
                // jnz .h8_3tap
                m[7] = bcd_p5(ro(z_filter_k, 4 * 8)); // vpbroadcastd m7, [z_filter_k+4*8]
                let mut r2 = *tl.wrapping_offset(-14) as u32; // movzx r2d, byte [tlq-14]
                m[2] = _mm256_shuffle_epi8(m[2], m[6]); // pshufb m2, m6
                m[2] = _mm256_maddubs_epi16(m[2], m[7]); // pmaddubsw m2, m7
                r2 = r2.wrapping_sub(r4); // sub r2d, r4d
                r2 = r2.wrapping_add(r4 * 8 + 4); // lea r2d, [r2+r4*8+4]
                r2 >>= 3; // shr r2d, 3
                *rsp.add(15) = r2 as u8; // mov [rsp+15], r2b
                m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
            }
            // .h8_3tap:
            m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
            r5 >>= 1; // sar r5d, 1
            let tlm = rsp.add(31); // lea tlq, [rsp+31]
            r5 += 17; // add r5d, 17
            if w >= 16 {
                maxbase = r5; // cmp wd, 16 / cmovns maxbased, r5d
            }
            // neg r5 / mov [tlq+r5], r4b: r5 is 16 or 17, rsp+15 or rsp+14
            *tlm.sub(r5 as usize) = r4 as u8;
            m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
            m[0] = zx(_mm_packus_epi16(lo(m[0]), lo(m[1]))); // packuswb xm0, xm1
            _mm_storeu_si128(tlm.sub(15).cast(), lo(m[0])); // mova [tlq-15], xm0
            tl = tlm;
        }
    }
    // .h8_main: the transposes store w/2 rows of 16 bytes (one per two columns)
    match w {
        4 => ipred_z3_8bpc_h8_main::<2>(m, dst, stride, tl, w, maxbase, dy, org_w),
        8 => ipred_z3_8bpc_h8_main::<4>(m, dst, stride, tl, w, maxbase, dy, org_w),
        16 => ipred_z3_8bpc_h8_main::<8>(m, dst, stride, tl, w, maxbase, dy, org_w),
        _ => ipred_z3_8bpc_h8_main::<16>(m, dst, stride, tl, w, maxbase, dy, org_w),
    }
}

/// `.h8_main` to its `RET` (`.h8_loop`, `.h8_end_loop`, `.h8_transpose`, `.h8_w8_loop`, `.h8_w8`); `N` = w/2,
/// the rows pushed below the frame.
#[inline(always)]
unsafe fn ipred_z3_8bpc_h8_main<const N: usize>(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    mut w: i32,
    maxbase: i32,
    mut dy: i64,
    mut org_w: i32,
) {
    // The rows `sub rsp, 8*2` / `mova [rsp], xm0` pushes: `rsp` points to the last slot pushed. `.h8_loop` and `.h8_end_loop`
    // store one slot per `sub wd, 2` from w = 2*N down to 0, top slot first: all N slots before the
    // transpose's first load.
    let mut stack = core::mem::MaybeUninit::<[__m128i; N]>::uninit();
    let mut rsp = stack.as_mut_ptr().cast::<__m128i>().add(N);
    m[2] = zx(_mm_cvtsi32_si128(dy as i32)); // movd xm2, dyd
    m[0] = bci_p5(ro(z_base_inc, 0), 0); // vbroadcasti128 m0, [z_base_inc]
    let r4 = tl; // mov r4, tlq
    tl = tl.wrapping_offset(-8); // sub tlq, 8
    dy = -dy; // neg dyq
    m[2] = _mm256_broadcastw_epi16(lo(m[2])); // vpbroadcastw m2, xm2
    let r4 = r4.wrapping_offset(-(maxbase as isize)); // sub r4, maxbaseq
    let mut mb = maxbase << 6; // shl maxbased, 6
    m[7] = _mm256_set1_epi8(*r4 as i8); // vpbroadcastb m7, [r4]
    let mut r4 = dy + 63; // lea r4, [dyq+63]
    m[9] = zx(_mm_cvtsi32_si128(mb)); // movd xm9, maxbased
    mb = !mb; // not maxbased
    m[8] = bci_p5(ro(z3_shuf, 0), 0); // vbroadcasti128 m8, [z3_shuf]
    mb = mb.wrapping_add(64); // add maxbased, 64
    m[9] = _mm256_broadcastw_epi16(lo(m[9])); // vpbroadcastw m9, xm9
    m[7] = _mm256_srli_epi16::<8>(m[7]); // psrlw m7, 8
    m[9] = _mm256_sub_epi16(m[9], m[0]); // psubw m9, m0
    m[6] = _mm256_add_epi16(m[2], m[2]); // paddw m6, m2, m2
    m[2] = _mm256_blend_epi32::<0x0f>(m[2], m[6]); // vpblendd m2, m6, 0x0f
    'transpose: {
        loop {
            // .h8_loop:
            let mut r5 = r4 + dy; // lea r5, [r4+dyq]
            r4 >>= 6; // sar r4, 6
            m[0] = _mm256_and_si256(m[4], m[2]); // pand m0, m4, m2
            m[1] = _mm256_sub_epi16(m[5], m[0]); // psubw m1, m5, m0
            m[0] = _mm256_slli_epi16::<8>(m[0]); // psllw m0, 8
            m[1] = _mm256_or_si256(m[1], m[0]); // por m1, m0
            m[0] = bci_p5(tl, r4 as isize); // vbroadcasti128 m0, [tlq+r4]
            r4 = r5 + dy; // lea r4, [r5+dyq]
            r5 >>= 6; // sar r5, 6
            m[0] = _mm256_inserti128_si256::<0>(m[0], ld16_p5(tl, r5 as isize)); // vinserti128 m0, [tlq+r5], 0
            rsp = rsp.sub(1); // sub rsp, 8*2
            m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
            m[0] = _mm256_maddubs_epi16(m[0], m[1]); // pmaddubsw m0, m1
            m[1] = _mm256_cmpgt_epi16(m[9], m[2]); // pcmpgtw m1, m9, m2
            m[2] = _mm256_add_epi16(m[2], m[6]); // paddw m2, m6
            m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
            m[0] = _mm256_blendv_epi8(m[7], m[0], m[1]); // vpblendvb m0, m7, m0, m1
            m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
            m[0] = zx(_mm_slli_epi16::<8>(lo(m[0]))); // psllw xm0, 8
            m[0] = zx(_mm_or_si128(lo(m[0]), lo(m[1]))); // por xm0, xm1 ; interleave rows (partial transpose)
            _mm_store_si128(rsp, lo(m[0])); // mova [rsp], xm0
            w -= 2; // sub wd, 2
            if w == 0 {
                break 'transpose; // jz .h8_transpose
            }
            if r4 as i32 <= mb {
                break; // cmp r4d, maxbased / jg .h8_loop
            }
        }
        m[0] = zx(_mm_packus_epi16(lo(m[7]), lo(m[7]))); // packuswb xm0, xm7, xm7
        loop {
            // .h8_end_loop:
            rsp = rsp.sub(1); // sub rsp, 8*2
            _mm_store_si128(rsp, lo(m[0])); // mova [rsp], xm0
            w -= 2; // sub wd, 2
            if w <= 0 {
                break; // jg .h8_end_loop
            }
        }
    }
    // .h8_transpose:
    m[2] = zx(_mm_load_si128(rsp.add(1))); // mova xm2, [rsp+16*1]
    org_w -= 8; // sub org_wd, 8
    let r2 = stride * 3; // lea r2, [strideq*3]
    if org_w >= 0 {
        dst = at(dst, org_w as isize); // lea r6, [dstq+org_wq] / cmovns dstq, r6
    }
    m[1] = zx(_mm_unpacklo_epi16(lo(m[2]), lo(m[0]))); // punpcklwd xm1, xm2, xm0
    m[2] = zx(_mm_unpackhi_epi16(lo(m[2]), lo(m[0]))); // punpckhwd xm2, xm0
    let mut r6 = at(dst, stride * 4); // lea r6, [dstq+strideq*4]
    if org_w < 0 {
        // jge .h8_w8
        // add rsp, 16*2
        st4x4_p5([dst, at(dst, stride), at(dst, stride * 2), at(dst, r2)], lo(m[1])); // movd/pextrd [dstq+…], xm1
        st4x4_p5([r6, at(r6, stride), at(r6, stride * 2), at(r6, r2)], lo(m[2])); // movd/pextrd [r6+…], xm2
        return; // jmp .h8_end
    }
    loop {
        // .h8_w8: ; w8/w16/w32
        m[0] = zx(_mm_load_si128(rsp.add(2))); // mova xm0, [rsp+16*2]
        m[4] = zx(_mm_load_si128(rsp.add(3))); // mova xm4, [rsp+16*3]
        rsp = rsp.add(4); // add rsp, 16*4
        m[3] = zx(_mm_unpacklo_epi16(lo(m[4]), lo(m[0]))); // punpcklwd xm3, xm4, xm0
        m[4] = zx(_mm_unpackhi_epi16(lo(m[4]), lo(m[0]))); // punpckhwd xm4, xm0
        m[0] = zx(_mm_unpacklo_epi32(lo(m[3]), lo(m[1]))); // punpckldq xm0, xm3, xm1
        m[3] = zx(_mm_unpackhi_epi32(lo(m[3]), lo(m[1]))); // punpckhdq xm3, xm1
        m[1] = zx(_mm_unpacklo_epi32(lo(m[4]), lo(m[2]))); // punpckldq xm1, xm4, xm2
        m[4] = zx(_mm_unpackhi_epi32(lo(m[4]), lo(m[2]))); // punpckhdq xm4, xm2
        movq_st_p5(dst, lo(m[0])); // movq   [dstq+strideq*0], xm0
        movhps_st_p5(at(dst, stride), lo(m[0])); // movhps [dstq+strideq*1], xm0
        movq_st_p5(at(dst, stride * 2), lo(m[3])); // movq   [dstq+strideq*2], xm3
        movhps_st_p5(at(dst, r2), lo(m[3])); // movhps [dstq+r2       ], xm3
        movq_st_p5(r6, lo(m[1])); // movq   [r6  +strideq*0], xm1
        movhps_st_p5(at(r6, stride), lo(m[1])); // movhps [r6  +strideq*1], xm1
        movq_st_p5(at(r6, stride * 2), lo(m[4])); // movq   [r6  +strideq*2], xm4
        movhps_st_p5(at(r6, r2), lo(m[4])); // movhps [r6  +r2       ], xm4
        dst = at(dst, -8); // sub dstq, 8
        r6 = at(r6, -8); // sub r6, 8
        org_w -= 8; // sub org_wd, 8
        if org_w < 0 {
            break; // jge .h8_w8_loop
        }
        // .h8_w8_loop:
        m[0] = zx(_mm_load_si128(rsp)); // mova xm0, [rsp+16*0]
        m[2] = zx(_mm_load_si128(rsp.add(1))); // mova xm2, [rsp+16*1]
        m[1] = zx(_mm_unpacklo_epi16(lo(m[2]), lo(m[0]))); // punpcklwd xm1, xm2, xm0
        m[2] = zx(_mm_unpackhi_epi16(lo(m[2]), lo(m[0]))); // punpckhwd xm2, xm0
    }
    // .h8_end: RET
}

/// `.h16` (with `.h16_no_intra_edge_filter`, the filter of the edge into the frame, `.h16_3tap`) and
/// `.h16_main`'s caller.
#[inline(always)]
unsafe fn ipred_z3_8bpc_h16(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    w: i32,
    mut angle: i32,
    dy: i64,
    org_w: i32,
) {
    // ALLOC_STACK -64, 12: the frame (64 bytes at rsp), zeroed whole with 64 bytes below and 32 above it when
    // the edge is filtered into it (rsp+30..rsp+64 written): `.h16_main` loads 16 bytes at tl-8-base and
    // tl-16-base with tl = rsp+63 and base from 0 (up to rsp+71) to at most 48 (rsp-1; 2*dy >> 6 in the first
    // iteration, later the first base of an iteration is below maxbase <= 33). The lanes outside the written
    // bytes are blended away.
    let mut frame: [__m256i; 5];
    let mut maxbase = w + 15; // lea maxbased, [wq+15]
    if angle & 0x400 != 0 {
        // test angled, 0x400 / jnz .h16_no_intra_edge_filter
        // .h16_no_intra_edge_filter:
        maxbase &= 15; // and maxbased, 15
        maxbase |= 16; // or maxbased, 16 ; imin(w+15, 31)
                       // jmp .h16_main
    } else {
        let mut r5 = ipred_z3_8bpc_filter_strength(m, maxbase, &mut angle); // call .filter_strength
        if r5 != 0 {
            // test r5d, r5d / jz .h16_main ; filter_strength == 0
            frame = [_mm256_setzero_si256(); 5];
            let rsp = frame.as_mut_ptr().cast::<u8>().add(64);
            r5 = r5.count_ones(); // popcnt r5d, r5d
            let k = r5 as isize * 4 - 4; // z_filter_k-4+r5*4 (r5 is 1..=3)
            m[11] = bcd_p5(ro(z2_y_shuf_h4, 8)); // vpbroadcastd m11, [base+pb_27]
            m[1] = bcd_p5(ro(ipred_h_shuf, 12)); // vpbroadcastd m1, [base+pb_1]
            m[6] = bci_p5(ro(dav1d_z_filter_s, 12), 0); // vbroadcasti128 m6, [base+z_filter_s+12]
            m[2] = _mm256_inserti128_si256::<0>(m[6], ld16_p5(ro(dav1d_z_filter_s, 4), 0)); // vinserti128 m2, m6, [base+z_filter_s+4], 0
            m[6] = _mm256_inserti128_si256::<1>(m[6], ld16_p5(ro(dav1d_z_filter_s, 20), 0)); // vinserti128 m6, [base+z_filter_s+20], 1
            m[10] = zx(ld16_p5(tl, -18)); // movu xm10, [tlq-18]
            m[10] = _mm256_inserti128_si256::<1>(m[10], ld16_p5(tl, -14)); // vinserti128 m10, [tlq-14], 1
            m[9] = bcd_p5(ro(z_filter_k, k + 12 * 0)); // vpbroadcastd m9, [base+z_filter_k-4+r5*4+12*0]
            m[7] = bci_p5(ro(dav1d_z_filter_s, 8), 0); // vbroadcasti128 m7, [base+z_filter_s+8]
            m[8] = _mm256_inserti128_si256::<0>(m[7], ld16_p5(ro(dav1d_z_filter_s, 0), 0)); // vinserti128 m8, m7, [base+z_filter_s+0], 0
            m[7] = _mm256_inserti128_si256::<1>(m[7], ld16_p5(ro(dav1d_z_filter_s, 16), 0)); // vinserti128 m7, [base+z_filter_s+16], 1
            m[11] = _mm256_subs_epu8(m[11], m[0]); // psubusb m11, m0
            m[1] = _mm256_or_si256(m[1], m[11]); // por m1, m11
            m[11] = zx(ld16_p5(tl, -32)); // movu xm11, [tlq-32]
            m[11] = _mm256_inserti128_si256::<1>(m[11], ld16_p5(tl, -28)); // vinserti128 m11, [tlq-28], 1
            m[8] = _mm256_max_epu8(m[8], m[1]); // pmaxub m8, m1
            m[7] = _mm256_max_epu8(m[7], m[1]); // pmaxub m7, m1
            m[0] = _mm256_shuffle_epi8(m[10], m[2]); // pshufb m0, m10, m2
            m[2] = shufps_p5::<0x99>(m[2], m[6]); // shufps m2, m6, q2121
            m[0] = _mm256_maddubs_epi16(m[0], m[9]); // pmaddubsw m0, m9
            m[1] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m1, m11, m8
            m[8] = shufps_p5::<0x99>(m[8], m[7]); // shufps m8, m7, q2121
            m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
            m[9] = bcd_p5(ro(z_filter_k, k + 12 * 1)); // vpbroadcastd m9, [base+z_filter_k-4+r5*4+12*1]
            let r4 = *tl.wrapping_offset(-31) as u32; // movzx r4d, byte [tlq-31]
            m[2] = _mm256_shuffle_epi8(m[10], m[2]); // pshufb m2, m10, m2
            m[2] = _mm256_maddubs_epi16(m[2], m[9]); // pmaddubsw m2, m9
            m[8] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m8, m11, m8
            m[8] = _mm256_maddubs_epi16(m[8], m[9]); // pmaddubsw m8, m9
            m[0] = _mm256_add_epi16(m[0], m[2]); // paddw m0, m2
            m[1] = _mm256_add_epi16(m[1], m[8]); // paddw m1, m8
            let mut r5 = (r5 as i32).wrapping_sub(3); // sub r5d, 3
            if r5 == 0 {
                // jnz .h16_3tap
                m[9] = bcd_p5(ro(z_filter_k, 4 * 8)); // vpbroadcastd m9, [z_filter_k+4*8]
                let mut r2 = *tl.wrapping_offset(-30) as u32; // movzx r2d, byte [tlq-30]
                m[10] = _mm256_shuffle_epi8(m[10], m[6]); // pshufb m10, m6
                m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
                m[11] = _mm256_shuffle_epi8(m[11], m[7]); // pshufb m11, m7
                m[11] = _mm256_maddubs_epi16(m[11], m[9]); // pmaddubsw m11, m9
                r2 = r2.wrapping_sub(r4); // sub r2d, r4d
                r2 = r2.wrapping_add(r4 * 8 + 4); // lea r2d, [r2+r4*8+4]
                r2 >>= 3; // shr r2d, 3
                *rsp.add(31) = r2 as u8; // mov [rsp+31], r2b
                m[0] = _mm256_add_epi16(m[0], m[10]); // paddw m0, m10
                m[1] = _mm256_add_epi16(m[1], m[11]); // paddw m1, m11
            }
            // .h16_3tap:
            m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
            m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
            r5 >>= 1; // sar r5d, 1
            let tlm = rsp.add(63); // lea tlq, [rsp+63]
            r5 += 33; // add r5d, 33
            if w >= 32 {
                maxbase = r5; // cmp wd, 32 / cmovns maxbased, r5d
            }
            // neg r5 / mov [tlq+r5], r4b: r5 is 32 or 33, rsp+31 or rsp+30
            *tlm.sub(r5 as usize) = r4 as u8;
            m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
            m[0] = _mm256_permute4x64_epi64::<0x8d>(m[0]); // vpermq m0, m0, q2031
            _mm256_storeu_si256(tlm.sub(31).cast(), m[0]); // mova [tlq-31], m0
            tl = tlm;
        }
    }
    // .h16_main: the transposes store w/2 rows of 32 bytes (one per two columns)
    match w {
        4 => ipred_z3_8bpc_h16_main::<2>(m, dst, stride, tl, w, maxbase, dy, org_w),
        8 => ipred_z3_8bpc_h16_main::<4>(m, dst, stride, tl, w, maxbase, dy, org_w),
        16 => ipred_z3_8bpc_h16_main::<8>(m, dst, stride, tl, w, maxbase, dy, org_w),
        32 => ipred_z3_8bpc_h16_main::<16>(m, dst, stride, tl, w, maxbase, dy, org_w),
        _ => ipred_z3_8bpc_h16_main::<32>(m, dst, stride, tl, w, maxbase, dy, org_w),
    }
}

/// `.h16_main` to its `RET` (`.h16_loop`, `.h16_end_loop`, `.h16_transpose`, `.h16_w8_loop`, `.h16_w8`); `N` =
/// w/2, the rows pushed below the frame.
#[inline(always)]
unsafe fn ipred_z3_8bpc_h16_main<const N: usize>(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    mut w: i32,
    maxbase: i32,
    mut dy: i64,
    mut org_w: i32,
) {
    // The rows `sub rsp, 32` / `mova [rsp], m0` pushes: `rsp` points to the last slot pushed. `.h16_loop` and `.h16_end_loop`
    // store one slot per `sub wd, 2` from w = 2*N down to 0, top slot first: all N slots before the
    // transpose's first load.
    let mut stack = core::mem::MaybeUninit::<[__m256i; N]>::uninit();
    let mut rsp = stack.as_mut_ptr().cast::<__m256i>().add(N);
    m[6] = zx(_mm_cvtsi32_si128(dy as i32)); // movd xm6, dyd
    m[0] = bci_p5(ro(z_base_inc, 0), 0); // vbroadcasti128 m0, [z_base_inc]
    let r4 = tl; // mov r4, tlq
    tl = tl.wrapping_offset(-8); // sub tlq, 8
    dy = -dy; // neg dyq
    m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
    let r4 = r4.wrapping_offset(-(maxbase as isize)); // sub r4, maxbaseq
    let mut mb = maxbase << 6; // shl maxbased, 6
    m[7] = _mm256_set1_epi8(*r4 as i8); // vpbroadcastb m7, [r4]
    let mut r4 = dy + 63; // lea r4, [dyq+63]
    m[9] = zx(_mm_cvtsi32_si128(mb)); // movd xm9, maxbased
    mb = !mb; // not maxbased
    m[8] = bci_p5(ro(z3_shuf, 0), 0); // vbroadcasti128 m8, [z3_shuf]
    mb = mb.wrapping_add(64); // add maxbased, 64
    m[9] = _mm256_broadcastw_epi16(lo(m[9])); // vpbroadcastw m9, xm9
    m[9] = _mm256_sub_epi16(m[9], m[0]); // psubw m9, m0
    m[11] = _mm256_add_epi16(m[6], m[6]); // paddw m11, m6, m6
    m[10] = _mm256_sub_epi16(m[9], m[3]); // psubw m10, m9, m3 ; 64*8
    m[6] = _mm256_blend_epi32::<0xf0>(m[6], m[11]); // vpblendd m6, m11, 0xf0
    'transpose: {
        loop {
            // .h16_loop:
            let mut r5 = r4 + dy; // lea r5, [r4+dyq]
            r4 >>= 6; // sar r4, 6
            m[1] = _mm256_and_si256(m[4], m[6]); // pand m1, m4, m6
            m[2] = _mm256_sub_epi16(m[5], m[1]); // psubw m2, m5, m1
            m[1] = _mm256_slli_epi16::<8>(m[1]); // psllw m1, 8
            m[2] = _mm256_or_si256(m[2], m[1]); // por m2, m1
            m[0] = zx(ld16_p5(tl, r4 as isize)); // movu xm0, [tlq+r4-0]
            m[1] = zx(ld16_p5(tl, r4 as isize - 8)); // movu xm1, [tlq+r4-8]
            r4 = r5 + dy; // lea r4, [r5+dyq]
            r5 >>= 6; // sar r5, 6
            m[0] = _mm256_inserti128_si256::<1>(m[0], ld16_p5(tl, r5 as isize)); // vinserti128 m0, [tlq+r5-0], 1
            m[1] = _mm256_inserti128_si256::<1>(m[1], ld16_p5(tl, r5 as isize - 8)); // vinserti128 m1, [tlq+r5-8], 1
            rsp = rsp.sub(1); // sub rsp, 32
            m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
            m[1] = _mm256_shuffle_epi8(m[1], m[8]); // pshufb m1, m8
            m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
            m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
            m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
            m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
            m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
            m[1] = _mm256_cmpgt_epi16(m[9], m[6]); // pcmpgtw m1, m9, m6
            m[2] = _mm256_cmpgt_epi16(m[10], m[6]); // pcmpgtw m2, m10, m6
            m[1] = _mm256_packs_epi16(m[1], m[2]); // packsswb m1, m2
            m[6] = _mm256_add_epi16(m[6], m[11]); // paddw m6, m11
            m[0] = _mm256_blendv_epi8(m[7], m[0], m[1]); // vpblendvb m0, m7, m0, m1
            m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
            _mm256_store_si256(rsp, m[0]); // mova [rsp], m0
            w -= 2; // sub wd, 2
            if w == 0 {
                break 'transpose; // jz .h16_transpose
            }
            if r4 as i32 <= mb {
                break; // cmp r4d, maxbased / jg .h16_loop
            }
        }
        m[0] = m[7]; // mova m0, m7
        loop {
            // .h16_end_loop:
            rsp = rsp.sub(1); // sub rsp, 32
            _mm256_store_si256(rsp, m[7]); // mova [rsp], m7
            w -= 2; // sub wd, 2
            if w <= 0 {
                break; // jg .h16_end_loop
            }
        }
    }
    // .h16_transpose:
    m[2] = _mm256_load_si256(rsp.add(1)); // mova m2, [rsp+32*1]
    org_w -= 8; // sub org_wd, 8
    let r2 = stride * 3; // lea r2, [strideq*3]
    if org_w >= 0 {
        dst = at(dst, org_w as isize); // lea r6, [dstq+org_wq] / cmovns dstq, r6
    }
    m[1] = _mm256_unpacklo_epi8(m[2], m[0]); // punpcklbw m1, m2, m0
    m[2] = _mm256_unpackhi_epi8(m[2], m[0]); // punpckhbw m2, m0
    let r3 = stride * 5; // lea r3, [strideq*5]
    m[0] = _mm256_unpacklo_epi8(m[1], m[2]); // punpcklbw m0, m1, m2
    m[1] = _mm256_unpackhi_epi8(m[1], m[2]); // punpckhbw m1, m2
    let r4 = stride + r2 * 2; // lea r4, [strideq+r2*2] ; stride*7
    if org_w < 0 {
        // jge .h16_w8
        // add rsp, 32*2
        st4x4_p5([dst, at(dst, stride), at(dst, stride * 2), at(dst, r2)], lo(m[0])); // movd/pextrd [dstq+…], xm0
        m[0] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm0, m0, 1
        st4x4_p5(
            [at(dst, stride * 4), at(dst, r3), at(dst, r2 * 2), at(dst, r4)],
            lo(m[1]),
        ); // movd/pextrd [dstq+…], xm1
        dst = at(dst, stride * 8); // lea dstq, [dstq+strideq*8]
        m[1] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm1, m1, 1
        st4x4_p5([dst, at(dst, stride), at(dst, stride * 2), at(dst, r2)], lo(m[0])); // movd/pextrd [dstq+…], xm0
        st4x4_p5(
            [at(dst, stride * 4), at(dst, r3), at(dst, r2 * 2), at(dst, r4)],
            lo(m[1]),
        ); // movd/pextrd [dstq+…], xm1
        return; // jmp .h16_end
    }
    loop {
        // .h16_w8:
        m[2] = _mm256_load_si256(rsp.add(2)); // mova m2, [rsp+32*2]
        m[4] = _mm256_load_si256(rsp.add(3)); // mova m4, [rsp+32*3]
        let r6 = at(dst, stride * 8); // lea r6, [dstq+strideq*8]
        rsp = rsp.add(4); // add rsp, 32*4
        m[3] = _mm256_unpacklo_epi8(m[4], m[2]); // punpcklbw m3, m4, m2
        m[4] = _mm256_unpackhi_epi8(m[4], m[2]); // punpckhbw m4, m2
        m[2] = _mm256_unpacklo_epi8(m[3], m[4]); // punpcklbw m2, m3, m4
        m[3] = _mm256_unpackhi_epi8(m[3], m[4]); // punpckhbw m3, m4
        m[4] = _mm256_unpacklo_epi32(m[2], m[0]); // punpckldq m4, m2, m0
        m[2] = _mm256_unpackhi_epi32(m[2], m[0]); // punpckhdq m2, m0
        m[0] = _mm256_unpacklo_epi32(m[3], m[1]); // punpckldq m0, m3, m1
        m[3] = _mm256_unpackhi_epi32(m[3], m[1]); // punpckhdq m3, m1
        movq_st_p5(dst, lo(m[4])); // movq   [dstq+strideq*0], xm4
        movhps_st_p5(at(dst, stride), lo(m[4])); // movhps [dstq+strideq*1], xm4
        m[4] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm4, m4, 1
        movq_st_p5(at(dst, stride * 2), lo(m[2])); // movq   [dstq+strideq*2], xm2
        movhps_st_p5(at(dst, r2), lo(m[2])); // movhps [dstq+r2       ], xm2
        m[2] = zx(_mm256_extracti128_si256::<1>(m[2])); // vextracti128 xm2, m2, 1
        movq_st_p5(at(dst, stride * 4), lo(m[0])); // movq   [dstq+strideq*4], xm0
        movhps_st_p5(at(dst, r3), lo(m[0])); // movhps [dstq+r3       ], xm0
        m[0] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm0, m0, 1
        movq_st_p5(at(dst, r2 * 2), lo(m[3])); // movq   [dstq+r2*2     ], xm3
        movhps_st_p5(at(dst, r4), lo(m[3])); // movhps [dstq+r4       ], xm3
        m[3] = zx(_mm256_extracti128_si256::<1>(m[3])); // vextracti128 xm3, m3, 1
        movq_st_p5(r6, lo(m[4])); // movq     [r6+strideq*0], xm4
        movhps_st_p5(at(r6, stride), lo(m[4])); // movhps   [r6+strideq*1], xm4
        movq_st_p5(at(r6, stride * 2), lo(m[2])); // movq     [r6+strideq*2], xm2
        movhps_st_p5(at(r6, r2), lo(m[2])); // movhps   [r6+r2       ], xm2
        movq_st_p5(at(r6, stride * 4), lo(m[0])); // movq     [r6+strideq*4], xm0
        movhps_st_p5(at(r6, r3), lo(m[0])); // movhps   [r6+r3       ], xm0
        movq_st_p5(at(r6, r2 * 2), lo(m[3])); // movq     [r6+r2*2     ], xm3
        movhps_st_p5(at(r6, r4), lo(m[3])); // movhps   [r6+r4       ], xm3
        dst = at(dst, -8); // sub dstq, 8
        org_w -= 8; // sub org_wd, 8
        if org_w < 0 {
            break; // jge .h16_w8_loop
        }
        // .h16_w8_loop:
        m[0] = _mm256_load_si256(rsp); // mova m0, [rsp+32*0]
        m[2] = _mm256_load_si256(rsp.add(1)); // mova m2, [rsp+32*1]
        m[1] = _mm256_unpacklo_epi8(m[2], m[0]); // punpcklbw m1, m2, m0
        m[2] = _mm256_unpackhi_epi8(m[2], m[0]); // punpckhbw m2, m0
        m[0] = _mm256_unpacklo_epi8(m[1], m[2]); // punpcklbw m0, m1, m2
        m[1] = _mm256_unpackhi_epi8(m[1], m[2]); // punpckhbw m1, m2
    }
    // .h16_end: RET
}

/// `.h32` (with the filter of the edge into the frame) and `.h32_main`'s caller.
#[inline(always)]
unsafe fn ipred_z3_8bpc_h32(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    w: i32,
    angle: i32,
    dy: i64,
    org_w: i32,
) {
    // ALLOC_STACK -96, 15: the frame (96 bytes at rsp), zeroed whole with 64 bytes below and 32 above it when
    // the edge is filtered into it (rsp+30..rsp+96 written): `.h32_main` loads 16 bytes at tl-8-base-{0,8,16,24}
    // with tl = rsp+95 and base from 0 (up to rsp+103) to at most 64 (rsp-1; dy >> 6 <= 15 in the first
    // iteration, later the base is below maxbase <= 65). The lanes outside the written bytes are blended away.
    let mut frame: [__m256i; 6];
    let mut maxbase = w + 31; // lea maxbased, [wq+31]
    maxbase &= 31; // and maxbased, 31
    maxbase |= 32; // or maxbased, 32 ; imin(w+31, 63)
    if angle & 0x400 == 0 {
        // test angled, 0x400 / jnz .h32_main ; !enable_intra_edge_filter
        frame = [_mm256_setzero_si256(); 6];
        let rsp = frame.as_mut_ptr().cast::<u8>().add(64);
        m[0] = bci_p5(ro(cfl_ac_w16_pad_shuffle, 0), 0); // vbroadcasti128 m0, [pb_0to15]
        let mut r4 = 21i32; // mov r4d, 21
        let mut r5 = 3i32; // mov r5d, 3
        m[11] = zx(ld16_p5(tl, -66)); // movu xm11, [tlq-66] ; 56-63
        m[11] = _mm256_inserti128_si256::<1>(m[11], ld16_p5(tl, -52)); // vinserti128 m11, [tlq-52], 1 ; 40-47
        r4 -= w; // sub r4d, wd ; 21-w
        if r4 >= 0 {
            r5 = r4; // cmovns r5d, r4d
        }
        m[12] = zx(ld16_p5(tl, -58)); // movu xm12, [tlq-58] ; 48-55
        m[12] = _mm256_inserti128_si256::<1>(m[12], ld16_p5(tl, -44)); // vinserti128 m12, [tlq-44], 1 ; 32-39
        r4 -= 8; // sub r4d, 8 ; 13-w
        m[1] = zx(_mm_cvtsi32_si128(r5)); // movd xm1, r5d
        m[13] = zx(ld16_p5(tl, -34)); // movu xm13, [tlq-34] ; 24-31
        m[13] = _mm256_inserti128_si256::<1>(m[13], ld16_p5(tl, -20)); // vinserti128 m13, [tlq-20], 1 ;  8-15
        m[2] = zx(_mm_cvtsi32_si128(r4)); // movd xm2, r4d
        m[1] = _mm256_broadcastb_epi8(lo(m[1])); // vpbroadcastb m1, xm1
        m[14] = zx(ld16_p5(tl, -28)); // movu xm14, [tlq-28] ; 16-23
        m[14] = _mm256_inserti128_si256::<1>(m[14], ld16_p5(tl, -14)); // vinserti128 m14, [tlq-14], 1 ;  0- 7
        m[2] = _mm256_broadcastb_epi8(lo(m[2])); // vpbroadcastb m2, xm2
        m[1] = _mm256_max_epi8(m[1], m[0]); // pmaxsb m1, m0 ; clip 16x32 and (32|64)x32
        m[7] = ld32_p5(ro(dav1d_z_filter_s, 4), 0); // movu m7, [z_filter_s+4]
        m[11] = _mm256_shuffle_epi8(m[11], m[1]); // pshufb m11, m1
        m[8] = _mm256_inserti128_si256::<1>(m[7], ld16_p5(ro(dav1d_z_filter_s, 8), 0)); // vinserti128 m8, m7, [z_filter_s+8], 1
        m[7] = _mm256_inserti128_si256::<0>(m[7], ld16_p5(ro(dav1d_z_filter_s, 16), 0)); // vinserti128 m7, [z_filter_s+16], 0
        m[2] = _mm256_max_epi8(m[2], m[0]); // pmaxsb m2, m0 ; clip 8x32
        m[9] = bcd_p5(ro(z_filter_k, 4 * 2 + 12 * 0)); // vpbroadcastd m9, [z_filter_k+4*2+12*0]
        m[12] = _mm256_shuffle_epi8(m[12], m[2]); // pshufb m12, m2
        m[0] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m0, m11, m8
        m[0] = _mm256_maddubs_epi16(m[0], m[9]); // pmaddubsw m0, m9
        m[2] = _mm256_shuffle_epi8(m[12], m[8]); // pshufb m2, m12, m8
        m[2] = _mm256_maddubs_epi16(m[2], m[9]); // pmaddubsw m2, m9
        m[1] = _mm256_shuffle_epi8(m[13], m[8]); // pshufb m1, m13, m8
        m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
        m[8] = shufps_p5::<0x49>(m[8], m[7]); // shufps m8, m7, q1021
        m[6] = _mm256_shuffle_epi8(m[14], m[8]); // pshufb m6, m14, m8
        m[6] = _mm256_maddubs_epi16(m[6], m[9]); // pmaddubsw m6, m9
        m[9] = bcd_p5(ro(z_filter_k, 4 * 2 + 12 * 1)); // vpbroadcastd m9, [z_filter_k+4*2+12*1]
        m[10] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m10, m11, m8
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[0] = _mm256_add_epi16(m[0], m[10]); // paddw m0, m10
        m[10] = _mm256_shuffle_epi8(m[12], m[8]); // pshufb m10, m12, m8
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[2] = _mm256_add_epi16(m[2], m[10]); // paddw m2, m10
        m[10] = _mm256_shuffle_epi8(m[13], m[8]); // pshufb m10, m13, m8
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[8] = shufps_p5::<0x99>(m[8], m[7]); // shufps m8, m7, q2121
        m[1] = _mm256_add_epi16(m[1], m[10]); // paddw m1, m10
        m[10] = _mm256_shuffle_epi8(m[14], m[8]); // pshufb m10, m14, m8
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[6] = _mm256_add_epi16(m[6], m[10]); // paddw m6, m10
        m[9] = bcd_p5(ro(z_filter_k, 4 * 2 + 12 * 2)); // vpbroadcastd m9, [z_filter_k+4*2+12*2]
        m[11] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m11, m8
        m[11] = _mm256_maddubs_epi16(m[11], m[9]); // pmaddubsw m11, m9
        m[12] = _mm256_shuffle_epi8(m[12], m[8]); // pshufb m12, m8
        m[12] = _mm256_maddubs_epi16(m[12], m[9]); // pmaddubsw m12, m9
        let r4 = *tl.wrapping_offset(-63) as u32; // movzx r4d, byte [tlq-63]
        let mut r2 = *tl.wrapping_offset(-62) as u32; // movzx r2d, byte [tlq-62]
        m[0] = _mm256_add_epi16(m[0], m[11]); // paddw m0, m11
        m[2] = _mm256_add_epi16(m[2], m[12]); // paddw m2, m12
        m[13] = _mm256_shuffle_epi8(m[13], m[8]); // pshufb m13, m8
        m[13] = _mm256_maddubs_epi16(m[13], m[9]); // pmaddubsw m13, m9
        m[14] = _mm256_shuffle_epi8(m[14], m[7]); // pshufb m14, m7
        m[14] = _mm256_maddubs_epi16(m[14], m[9]); // pmaddubsw m14, m9
        m[1] = _mm256_add_epi16(m[1], m[13]); // paddw m1, m13
        m[6] = _mm256_add_epi16(m[6], m[14]); // paddw m6, m14
        r2 = r2.wrapping_sub(r4); // sub r2d, r4d
        r2 = r2.wrapping_add(r4 * 8 + 4); // lea r2d, [r2+r4*8+4] ; edge case for 64x32
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[2] = _mm256_mulhrs_epi16(m[2], m[3]); // pmulhrsw m2, m3
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        m[6] = _mm256_mulhrs_epi16(m[6], m[3]); // pmulhrsw m6, m3
        r2 >>= 3; // shr r2d, 3
        *rsp.add(31) = r2 as u8; // mov [rsp+31], r2b
        let tlm = rsp.add(95); // lea tlq, [rsp+95]
        *tlm.sub(65) = r4 as u8; // mov [tlq-65], r4b
        let r4 = 65; // mov r4d, 65
        if w == 64 {
            maxbase = r4; // cmp wd, 64 / cmove maxbased, r4d
        }
        m[0] = _mm256_packus_epi16(m[0], m[2]); // packuswb m0, m2
        m[1] = _mm256_packus_epi16(m[1], m[6]); // packuswb m1, m6
        _mm256_storeu_si256(tlm.sub(63).cast(), m[0]); // mova [tlq-63], m0
        _mm256_storeu_si256(tlm.sub(31).cast(), m[1]); // mova [tlq-31], m1
        tl = tlm;
    }
    // .h32_main: the transpose stores w rows of 32 bytes (one per column)
    match w {
        8 => ipred_z3_8bpc_h32_main::<8>(m, dst, stride, tl, w, maxbase, dy, org_w),
        16 => ipred_z3_8bpc_h32_main::<16>(m, dst, stride, tl, w, maxbase, dy, org_w),
        32 => ipred_z3_8bpc_h32_main::<32>(m, dst, stride, tl, w, maxbase, dy, org_w),
        _ => ipred_z3_8bpc_h32_main::<64>(m, dst, stride, tl, w, maxbase, dy, org_w),
    }
}

/// `.h32_main` to its `RET` (`.h32_loop`, `.h32_end_loop`, `.h32_transpose`, `.h32_w8_loop`); `N` = w, the
/// rows pushed below the frame.
#[inline(always)]
unsafe fn ipred_z3_8bpc_h32_main<const N: usize>(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    mut w: i32,
    maxbase: i32,
    mut dy: i64,
    mut org_w: i32,
) {
    // The rows `sub rsp, 32` / `mova [rsp], …` pushes: `rsp` points to the last slot pushed. `.h32_loop` and `.h32_end_loop`
    // store one slot per `dec wd` from w = N down to 0, top slot first: all N slots before the transpose's
    // first load.
    let mut stack = core::mem::MaybeUninit::<[__m256i; N]>::uninit();
    let mut rsp = stack.as_mut_ptr().cast::<__m256i>().add(N);
    m[6] = zx(_mm_cvtsi32_si128(dy as i32)); // movd xm6, dyd
    let r4 = tl; // mov r4, tlq
    tl = tl.wrapping_offset(-8); // sub tlq, 8
    dy = -dy; // neg dyq
    m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
    let r4 = r4.wrapping_offset(-(maxbase as isize)); // sub r4, maxbaseq
    let mut mb = maxbase << 6; // shl maxbased, 6
    m[7] = _mm256_set1_epi8(*r4 as i8); // vpbroadcastb m7, [r4]
    let mut r4 = dy + 63; // lea r4, [dyq+63]
    m[9] = zx(_mm_cvtsi32_si128(mb)); // movd xm9, maxbased
    mb = !mb; // not maxbased
    m[8] = bci_p5(ro(z3_shuf, 0), 0); // vbroadcasti128 m8, [z3_shuf]
    mb = mb.wrapping_add(64); // add maxbased, 64
    m[9] = _mm256_broadcastw_epi16(lo(m[9])); // vpbroadcastw m9, xm9
    m[9] = _mm256_sub_epi16(m[9], ld32_p5(ro(z_base_inc, 0), 0)); // psubw m9, [z_base_inc]
    m[11] = m[6]; // mova m11, m6
    m[10] = _mm256_sub_epi16(m[9], m[3]); // psubw m10, m9, m3 ; 64*8
    'transpose: {
        loop {
            // .h32_loop:
            let r5 = r4 >> 6; // mov r5, r4 / sar r5, 6
            m[1] = _mm256_and_si256(m[4], m[6]); // pand m1, m4, m6
            m[2] = _mm256_sub_epi16(m[5], m[1]); // psubw m2, m5, m1
            m[1] = _mm256_slli_epi16::<8>(m[1]); // psllw m1, 8
            m[2] = _mm256_or_si256(m[2], m[1]); // por m2, m1
            m[0] = zx(ld16_p5(tl, r5 as isize)); // movu xm0, [tlq+r5- 0]
            m[0] = _mm256_inserti128_si256::<1>(m[0], ld16_p5(tl, r5 as isize - 16)); // vinserti128 m0, [tlq+r5-16], 1
            m[1] = zx(ld16_p5(tl, r5 as isize - 8)); // movu xm1, [tlq+r5- 8]
            m[1] = _mm256_inserti128_si256::<1>(m[1], ld16_p5(tl, r5 as isize - 24)); // vinserti128 m1, [tlq+r5-24], 1
            rsp = rsp.sub(1); // sub rsp, 32
            r4 += dy; // add r4, dyq
            m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
            m[1] = _mm256_shuffle_epi8(m[1], m[8]); // pshufb m1, m8
            m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
            m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
            m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
            m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
            m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
            m[1] = _mm256_cmpgt_epi16(m[9], m[6]); // pcmpgtw m1, m9, m6
            m[2] = _mm256_cmpgt_epi16(m[10], m[6]); // pcmpgtw m2, m10, m6
            m[1] = _mm256_packs_epi16(m[1], m[2]); // packsswb m1, m2
            m[6] = _mm256_add_epi16(m[6], m[11]); // paddw m6, m11
            m[0] = _mm256_blendv_epi8(m[7], m[0], m[1]); // vpblendvb m0, m7, m0, m1
            _mm256_store_si256(rsp, m[0]); // mova [rsp], m0
            w -= 1; // dec wd
            if w == 0 {
                break 'transpose; // jz .h32_transpose
            }
            if r4 as i32 <= mb {
                break; // cmp r4d, maxbased / jg .h32_loop
            }
        }
        loop {
            // .h32_end_loop:
            rsp = rsp.sub(1); // sub rsp, 32
            _mm256_store_si256(rsp, m[7]); // mova [rsp], m7
            w -= 1; // dec wd
            if w <= 0 {
                break; // jg .h32_end_loop
            }
        }
    }
    // .h32_transpose:
    dst = at(dst, org_w as isize - 8); // lea dstq, [dstq+org_wq-8]
    let r2 = stride * 3; // lea r2, [strideq*3]
    let r3 = stride * 5; // lea r3, [strideq*5]
    let r4 = stride + r2 * 2; // lea r4, [strideq+r2*2] ; stride*7
    loop {
        // .h32_w8_loop:
        m[7] = _mm256_load_si256(rsp); // mova m7, [rsp+32*0]
        m[6] = _mm256_load_si256(rsp.add(1)); // mova m6, [rsp+32*1]
        m[5] = _mm256_load_si256(rsp.add(2)); // mova m5, [rsp+32*2]
        m[4] = _mm256_load_si256(rsp.add(3)); // mova m4, [rsp+32*3]
        m[3] = _mm256_load_si256(rsp.add(4)); // mova m3, [rsp+32*4]
        m[2] = _mm256_load_si256(rsp.add(5)); // mova m2, [rsp+32*5]
        m[1] = _mm256_load_si256(rsp.add(6)); // mova m1, [rsp+32*6]
        m[0] = _mm256_load_si256(rsp.add(7)); // mova m0, [rsp+32*7]
        let mut r6 = at(dst, stride * 8); // lea r6, [dstq+strideq*8]
        rsp = rsp.add(8); // add rsp, 32*8
        m[8] = _mm256_unpacklo_epi8(m[0], m[1]); // punpcklbw m8, m0, m1
        m[0] = _mm256_unpackhi_epi8(m[0], m[1]); // punpckhbw m0, m1
        m[1] = _mm256_unpacklo_epi8(m[2], m[3]); // punpcklbw m1, m2, m3
        m[2] = _mm256_unpackhi_epi8(m[2], m[3]); // punpckhbw m2, m3
        m[3] = _mm256_unpacklo_epi8(m[4], m[5]); // punpcklbw m3, m4, m5
        m[4] = _mm256_unpackhi_epi8(m[4], m[5]); // punpckhbw m4, m5
        m[5] = _mm256_unpacklo_epi8(m[6], m[7]); // punpcklbw m5, m6, m7
        m[6] = _mm256_unpackhi_epi8(m[6], m[7]); // punpckhbw m6, m7
        m[7] = _mm256_unpacklo_epi16(m[8], m[1]); // punpcklwd m7, m8, m1
        m[8] = _mm256_unpackhi_epi16(m[8], m[1]); // punpckhwd m8, m1
        m[1] = _mm256_unpacklo_epi16(m[0], m[2]); // punpcklwd m1, m0, m2
        m[0] = _mm256_unpackhi_epi16(m[0], m[2]); // punpckhwd m0, m2
        m[2] = _mm256_unpacklo_epi16(m[3], m[5]); // punpcklwd m2, m3, m5
        m[3] = _mm256_unpackhi_epi16(m[3], m[5]); // punpckhwd m3, m5
        m[5] = _mm256_unpacklo_epi16(m[4], m[6]); // punpcklwd m5, m4, m6
        m[4] = _mm256_unpackhi_epi16(m[4], m[6]); // punpckhwd m4, m6
        m[6] = _mm256_unpacklo_epi32(m[7], m[2]); // punpckldq m6, m7, m2
        m[7] = _mm256_unpackhi_epi32(m[7], m[2]); // punpckhdq m7, m2
        m[2] = _mm256_unpacklo_epi32(m[8], m[3]); // punpckldq m2, m8, m3
        m[8] = _mm256_unpackhi_epi32(m[8], m[3]); // punpckhdq m8, m3
        m[3] = _mm256_unpacklo_epi32(m[1], m[5]); // punpckldq m3, m1, m5
        m[1] = _mm256_unpackhi_epi32(m[1], m[5]); // punpckhdq m1, m5
        m[5] = _mm256_unpacklo_epi32(m[0], m[4]); // punpckldq m5, m0, m4
        m[0] = _mm256_unpackhi_epi32(m[0], m[4]); // punpckhdq m0, m4
        movq_st_p5(dst, lo(m[6])); // movq   [dstq+strideq*0], xm6
        movhps_st_p5(at(dst, stride), lo(m[6])); // movhps [dstq+strideq*1], xm6
        m[6] = zx(_mm256_extracti128_si256::<1>(m[6])); // vextracti128 xm6, m6, 1
        movq_st_p5(at(dst, stride * 2), lo(m[7])); // movq   [dstq+strideq*2], xm7
        movhps_st_p5(at(dst, r2), lo(m[7])); // movhps [dstq+r2       ], xm7
        m[7] = zx(_mm256_extracti128_si256::<1>(m[7])); // vextracti128 xm7, m7, 1
        movq_st_p5(at(dst, stride * 4), lo(m[2])); // movq   [dstq+strideq*4], xm2
        movhps_st_p5(at(dst, r3), lo(m[2])); // movhps [dstq+r3       ], xm2
        m[2] = zx(_mm256_extracti128_si256::<1>(m[2])); // vextracti128 xm2, m2, 1
        movq_st_p5(at(dst, r2 * 2), lo(m[8])); // movq   [dstq+r2*2     ], xm8
        movhps_st_p5(at(dst, r4), lo(m[8])); // movhps [dstq+r4       ], xm8
        m[8] = zx(_mm256_extracti128_si256::<1>(m[8])); // vextracti128 xm8, m8, 1
        movq_st_p5(r6, lo(m[3])); // movq     [r6+strideq*0], xm3
        movhps_st_p5(at(r6, stride), lo(m[3])); // movhps   [r6+strideq*1], xm3
        m[3] = zx(_mm256_extracti128_si256::<1>(m[3])); // vextracti128 xm3, m3, 1
        movq_st_p5(at(r6, stride * 2), lo(m[1])); // movq     [r6+strideq*2], xm1
        movhps_st_p5(at(r6, r2), lo(m[1])); // movhps   [r6+r2       ], xm1
        m[1] = zx(_mm256_extracti128_si256::<1>(m[1])); // vextracti128 xm1, m1, 1
        movq_st_p5(at(r6, stride * 4), lo(m[5])); // movq     [r6+strideq*4], xm5
        movhps_st_p5(at(r6, r3), lo(m[5])); // movhps   [r6+r3       ], xm5
        m[5] = zx(_mm256_extracti128_si256::<1>(m[5])); // vextracti128 xm5, m5, 1
        movq_st_p5(at(r6, r2 * 2), lo(m[0])); // movq     [r6+r2*2     ], xm0
        movhps_st_p5(at(r6, r4), lo(m[0])); // movhps   [r6+r4       ], xm0
        r6 = at(r6, stride * 8); // lea r6, [r6+strideq*8]
        m[0] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm0, m0, 1
        movq_st_p5(r6, lo(m[6])); // movq     [r6+strideq*0], xm6
        movhps_st_p5(at(r6, stride), lo(m[6])); // movhps   [r6+strideq*1], xm6
        movq_st_p5(at(r6, stride * 2), lo(m[7])); // movq     [r6+strideq*2], xm7
        movhps_st_p5(at(r6, r2), lo(m[7])); // movhps   [r6+r2       ], xm7
        movq_st_p5(at(r6, stride * 4), lo(m[2])); // movq     [r6+strideq*4], xm2
        movhps_st_p5(at(r6, r3), lo(m[2])); // movhps   [r6+r3       ], xm2
        movq_st_p5(at(r6, r2 * 2), lo(m[8])); // movq     [r6+r2*2     ], xm8
        movhps_st_p5(at(r6, r4), lo(m[8])); // movhps   [r6+r4       ], xm8
        r6 = at(r6, stride * 8); // lea r6, [r6+strideq*8]
        movq_st_p5(r6, lo(m[3])); // movq     [r6+strideq*0], xm3
        movhps_st_p5(at(r6, stride), lo(m[3])); // movhps   [r6+strideq*1], xm3
        movq_st_p5(at(r6, stride * 2), lo(m[1])); // movq     [r6+strideq*2], xm1
        movhps_st_p5(at(r6, r2), lo(m[1])); // movhps   [r6+r2       ], xm1
        movq_st_p5(at(r6, stride * 4), lo(m[5])); // movq     [r6+strideq*4], xm5
        movhps_st_p5(at(r6, r3), lo(m[5])); // movhps   [r6+r3       ], xm5
        movq_st_p5(at(r6, r2 * 2), lo(m[0])); // movq     [r6+r2*2     ], xm0
        movhps_st_p5(at(r6, r4), lo(m[0])); // movhps   [r6+r4       ], xm0
        dst = at(dst, -8); // sub dstq, 8
        org_w -= 8; // sub org_wd, 8
        if org_w <= 0 {
            break; // jg .h32_w8_loop
        }
    }
    // RET
}

/// `.h64` (with the filter of the edge into the frame) and `.h64_main`'s caller.
#[inline(always)]
unsafe fn ipred_z3_8bpc_h64(
    m: &mut M,
    dst: *mut u8,
    stride: isize,
    mut tl: *const u8,
    w: i32,
    angle: i32,
    dy: i64,
    org_w: i32,
) {
    // ALLOC_STACK -128, 16: the frame (128 bytes at rsp), zeroed whole with 64 bytes below and 32 above it when
    // the edge is filtered into it (all 128 bytes written): `.h64_main` loads 32 bytes at
    // tl-base-{24,32,56,64} with tl = rsp+127 and base from 0 (up to rsp+135) to at most 126 (rsp-63; dy >> 6
    // <= 15 in the first iteration, later the base is below maxbase = w+63 <= 127). The lanes outside the
    // frame are blended away.
    let mut frame: [__m256i; 7];
    let maxbase = w + 63; // lea maxbased, [wq+63]
    if angle & 0x400 == 0 {
        // test angled, 0x400 / jnz .h64_main ; !enable_intra_edge_filter
        frame = [_mm256_setzero_si256(); 7];
        let rsp = frame.as_mut_ptr().cast::<u8>().add(64);
        let mut r4 = 21i32; // mov r4d, 21
        m[11] = zx(_mm_set1_epi8(*tl.wrapping_offset(-127) as i8)); // vpbroadcastb xm11, [tlq-127]
        m[11] = zx(_mm_blend_epi32::<0x0e>(lo(m[11]), ld16_p5(tl, -130))); // vpblendd xm11, [tlq-130], 0x0e ; 120-127
        r4 -= w; // sub r4d, wd ; 21-w
        let r5 = 3; // mov r5d, 3
        m[11] = _mm256_inserti128_si256::<1>(m[11], ld16_p5(tl, -116)); // vinserti128 m11, [tlq-116], 1 ; 104-111
        m[7] = ld32_p5(ro(dav1d_z_filter_s, 4), 0); // movu m7, [z_filter_s+4]
        if w == 32 {
            r4 = r5; // cmp wd, 32 / cmove r4d, r5d
        }
        m[8] = _mm256_inserti128_si256::<1>(m[7], ld16_p5(ro(dav1d_z_filter_s, 8), 0)); // vinserti128 m8, m7, [z_filter_s+8], 1
        m[6] = bci_p5(ro(cfl_ac_w16_pad_shuffle, 0), 0); // vbroadcasti128 m6, [pb_0to15]
        m[1] = zx(_mm_cvtsi32_si128(r4)); // movd xm1, r4d
        m[9] = bcd_p5(ro(z_filter_k, 4 * 2 + 12 * 0)); // vpbroadcastd m9, [z_filter_k+4*2+12*0]
        m[12] = zx(ld16_p5(tl, -122)); // movu xm12, [tlq-122] ; 112-119
        m[12] = _mm256_inserti128_si256::<1>(m[12], ld16_p5(tl, -108)); // vinserti128 m12, [tlq-108], 1 ;  96-103
        m[1] = _mm256_broadcastb_epi8(lo(m[1])); // vpbroadcastb m1, xm1
        m[13] = zx(ld16_p5(tl, -98)); // movu xm13, [tlq- 98] ;  88- 95
        m[13] = _mm256_inserti128_si256::<1>(m[13], ld16_p5(tl, -84)); // vinserti128 m13, [tlq- 84], 1 ;  72- 79
        m[14] = zx(ld16_p5(tl, -90)); // movu xm14, [tlq- 90] ;  80- 87
        m[14] = _mm256_inserti128_si256::<1>(m[14], ld16_p5(tl, -76)); // vinserti128 m14, [tlq- 76], 1 ;  64- 71
        m[7] = _mm256_inserti128_si256::<0>(m[7], ld16_p5(ro(dav1d_z_filter_s, 16), 0)); // vinserti128 m7, [z_filter_s+16], 0
        m[0] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m0, m11, m8
        m[0] = _mm256_maddubs_epi16(m[0], m[9]); // pmaddubsw m0, m9
        m[2] = _mm256_shuffle_epi8(m[12], m[8]); // pshufb m2, m12, m8
        m[2] = _mm256_maddubs_epi16(m[2], m[9]); // pmaddubsw m2, m9
        m[1] = _mm256_max_epi8(m[1], m[6]); // pmaxsb m1, m6 ; clip (16|32)x64
        m[13] = _mm256_shuffle_epi8(m[13], m[1]); // pshufb m13, m1
        m[1] = _mm256_shuffle_epi8(m[13], m[8]); // pshufb m1, m13, m8
        m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
        m[6] = _mm256_shuffle_epi8(m[14], m[8]); // pshufb m6, m14, m8
        m[6] = _mm256_maddubs_epi16(m[6], m[9]); // pmaddubsw m6, m9
        m[9] = bcd_p5(ro(z_filter_k, 4 * 2 + 12 * 1)); // vpbroadcastd m9, [z_filter_k+4*2+12*1]
        m[15] = shufps_p5::<0x49>(m[8], m[7]); // shufps m15, m8, m7, q1021
        m[10] = _mm256_shuffle_epi8(m[11], m[15]); // pshufb m10, m11, m15
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[0] = _mm256_add_epi16(m[0], m[10]); // paddw m0, m10
        m[10] = _mm256_shuffle_epi8(m[12], m[15]); // pshufb m10, m12, m15
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[2] = _mm256_add_epi16(m[2], m[10]); // paddw m2, m10
        m[10] = _mm256_shuffle_epi8(m[13], m[15]); // pshufb m10, m13, m15
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[1] = _mm256_add_epi16(m[1], m[10]); // paddw m1, m10
        m[10] = _mm256_shuffle_epi8(m[14], m[15]); // pshufb m10, m14, m15
        m[10] = _mm256_maddubs_epi16(m[10], m[9]); // pmaddubsw m10, m9
        m[6] = _mm256_add_epi16(m[6], m[10]); // paddw m6, m10
        m[9] = bcd_p5(ro(z_filter_k, 4 * 2 + 12 * 2)); // vpbroadcastd m9, [z_filter_k+4*2+12*2]
        m[10] = shufps_p5::<0x9e>(m[8], m[7]); // shufps m10, m8, m7, q2132
        m[11] = _mm256_shuffle_epi8(m[11], m[10]); // pshufb m11, m10
        m[11] = _mm256_maddubs_epi16(m[11], m[9]); // pmaddubsw m11, m9
        m[12] = _mm256_shuffle_epi8(m[12], m[10]); // pshufb m12, m10
        m[12] = _mm256_maddubs_epi16(m[12], m[9]); // pmaddubsw m12, m9
        m[13] = _mm256_shuffle_epi8(m[13], m[10]); // pshufb m13, m10
        m[13] = _mm256_maddubs_epi16(m[13], m[9]); // pmaddubsw m13, m9
        m[14] = _mm256_shuffle_epi8(m[14], m[10]); // pshufb m14, m10
        m[14] = _mm256_maddubs_epi16(m[14], m[9]); // pmaddubsw m14, m9
        m[0] = _mm256_add_epi16(m[0], m[11]); // paddw m0, m11
        m[2] = _mm256_add_epi16(m[2], m[12]); // paddw m2, m12
        m[1] = _mm256_add_epi16(m[1], m[13]); // paddw m1, m13
        m[6] = _mm256_add_epi16(m[6], m[14]); // paddw m6, m14
        m[11] = zx(ld16_p5(tl, -66)); // movu xm11, [tlq-66] ; 56-63
        m[11] = _mm256_inserti128_si256::<1>(m[11], ld16_p5(tl, -52)); // vinserti128 m11, [tlq-52], 1 ; 40-47
        m[12] = zx(ld16_p5(tl, -58)); // movu xm12, [tlq-58] ; 48-55
        m[12] = _mm256_inserti128_si256::<1>(m[12], ld16_p5(tl, -44)); // vinserti128 m12, [tlq-44], 1 ; 32-39
        m[13] = zx(ld16_p5(tl, -34)); // movu xm13, [tlq-34] ; 24-31
        m[13] = _mm256_inserti128_si256::<1>(m[13], ld16_p5(tl, -20)); // vinserti128 m13, [tlq-20], 1 ;  8-15
        m[14] = zx(ld16_p5(tl, -28)); // movu xm14, [tlq-28] ; 16-23
        m[14] = _mm256_inserti128_si256::<1>(m[14], ld16_p5(tl, -14)); // vinserti128 m14, [tlq-14], 1 ;  0- 7
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[2] = _mm256_mulhrs_epi16(m[2], m[3]); // pmulhrsw m2, m3
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        m[6] = _mm256_mulhrs_epi16(m[6], m[3]); // pmulhrsw m6, m3
        let tlm = rsp.add(127); // lea tlq, [rsp+127]
        m[0] = _mm256_packus_epi16(m[0], m[2]); // packuswb m0, m2
        m[1] = _mm256_packus_epi16(m[1], m[6]); // packuswb m1, m6
        _mm256_storeu_si256(tlm.sub(127).cast(), m[0]); // mova [tlq-127], m0
        _mm256_storeu_si256(tlm.sub(95).cast(), m[1]); // mova [tlq- 95], m1
        m[0] = _mm256_shuffle_epi8(m[11], m[10]); // pshufb m0, m11, m10
        m[0] = _mm256_maddubs_epi16(m[0], m[9]); // pmaddubsw m0, m9
        m[2] = _mm256_shuffle_epi8(m[12], m[10]); // pshufb m2, m12, m10
        m[2] = _mm256_maddubs_epi16(m[2], m[9]); // pmaddubsw m2, m9
        m[1] = _mm256_shuffle_epi8(m[13], m[10]); // pshufb m1, m13, m10
        m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
        m[6] = _mm256_shuffle_epi8(m[14], m[7]); // pshufb m6, m14, m7
        m[6] = _mm256_maddubs_epi16(m[6], m[9]); // pmaddubsw m6, m9
        m[9] = bcd_p5(ro(z_filter_k, 4 * 2 + 12 * 1)); // vpbroadcastd m9, [z_filter_k+4*2+12*1]
        m[7] = _mm256_shuffle_epi8(m[11], m[15]); // pshufb m7, m11, m15
        m[7] = _mm256_maddubs_epi16(m[7], m[9]); // pmaddubsw m7, m9
        m[0] = _mm256_add_epi16(m[0], m[7]); // paddw m0, m7
        m[7] = _mm256_shuffle_epi8(m[12], m[15]); // pshufb m7, m12, m15
        m[7] = _mm256_maddubs_epi16(m[7], m[9]); // pmaddubsw m7, m9
        m[2] = _mm256_add_epi16(m[2], m[7]); // paddw m2, m7
        m[7] = _mm256_shuffle_epi8(m[13], m[15]); // pshufb m7, m13, m15
        m[7] = _mm256_maddubs_epi16(m[7], m[9]); // pmaddubsw m7, m9
        m[1] = _mm256_add_epi16(m[1], m[7]); // paddw m1, m7
        m[7] = _mm256_shuffle_epi8(m[14], m[10]); // pshufb m7, m14, m10
        m[7] = _mm256_maddubs_epi16(m[7], m[9]); // pmaddubsw m7, m9
        m[6] = _mm256_add_epi16(m[6], m[7]); // paddw m6, m7
        m[9] = bcd_p5(ro(z_filter_k, 4 * 2 + 12 * 0)); // vpbroadcastd m9, [z_filter_k+4*2+12*0]
        m[11] = _mm256_shuffle_epi8(m[11], m[8]); // pshufb m11, m8
        m[11] = _mm256_maddubs_epi16(m[11], m[9]); // pmaddubsw m11, m9
        m[12] = _mm256_shuffle_epi8(m[12], m[8]); // pshufb m12, m8
        m[12] = _mm256_maddubs_epi16(m[12], m[9]); // pmaddubsw m12, m9
        m[13] = _mm256_shuffle_epi8(m[13], m[8]); // pshufb m13, m8
        m[13] = _mm256_maddubs_epi16(m[13], m[9]); // pmaddubsw m13, m9
        m[14] = _mm256_shuffle_epi8(m[14], m[15]); // pshufb m14, m15
        m[14] = _mm256_maddubs_epi16(m[14], m[9]); // pmaddubsw m14, m9
        m[0] = _mm256_add_epi16(m[0], m[11]); // paddw m0, m11
        m[2] = _mm256_add_epi16(m[2], m[12]); // paddw m2, m12
        m[1] = _mm256_add_epi16(m[1], m[13]); // paddw m1, m13
        m[6] = _mm256_add_epi16(m[6], m[14]); // paddw m6, m14
        m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
        m[2] = _mm256_mulhrs_epi16(m[2], m[3]); // pmulhrsw m2, m3
        m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
        m[6] = _mm256_mulhrs_epi16(m[6], m[3]); // pmulhrsw m6, m3
        m[0] = _mm256_packus_epi16(m[0], m[2]); // packuswb m0, m2
        m[1] = _mm256_packus_epi16(m[1], m[6]); // packuswb m1, m6
        _mm256_storeu_si256(tlm.sub(63).cast(), m[0]); // mova [tlq-63], m0
        _mm256_storeu_si256(tlm.sub(31).cast(), m[1]); // mova [tlq-31], m1
        tl = tlm;
    }
    // .h64_main: the transpose stores w rows of 64 bytes (one per column, two 32-byte slots)
    match w {
        16 => ipred_z3_8bpc_h64_main::<32>(m, dst, stride, tl, w, maxbase, dy, org_w),
        32 => ipred_z3_8bpc_h64_main::<64>(m, dst, stride, tl, w, maxbase, dy, org_w),
        _ => ipred_z3_8bpc_h64_main::<128>(m, dst, stride, tl, w, maxbase, dy, org_w),
    }
}

/// `.h64_main` to its `RET` (`.h64_loop`, `.h64_end_loop`, `.h64_transpose`, `.h64_transpose_loop0`,
/// `.h64_transpose_loop`); `N` = 2*w, the 32-byte slots pushed below the frame.
#[inline(always)]
unsafe fn ipred_z3_8bpc_h64_main<const N: usize>(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut w: i32,
    maxbase: i32,
    mut dy: i64,
    mut org_w: i32,
) {
    // The rows `sub rsp, 64` / `mova [rsp+32], …` / `mova [rsp], …` pushes: `rsp` points to the last slot pushed.
    // `.h64_loop` and `.h64_end_loop` store two slots per `dec wd` from w = N/2 down to 0, top slots first: all
    // N slots before the transpose's first load.
    let mut stack = core::mem::MaybeUninit::<[__m256i; N]>::uninit();
    let mut rsp = stack.as_mut_ptr().cast::<u8>().add(N * 32);
    m[12] = zx(_mm_cvtsi32_si128(dy as i32)); // movd xm12, dyd
    let maxbaseq = -(maxbase as i64); // neg maxbaseq
    m[8] = bci_p5(ro(z3_shuf, 0), 0); // vbroadcasti128 m8, [z3_shuf]
    m[7] = _mm256_set1_epi8(*tl.wrapping_offset(maxbaseq as isize) as i8); // vpbroadcastb m7, [tlq+maxbaseq]
    let mut mb = (maxbaseq as i32) << 6; // shl maxbased, 6
    m[12] = _mm256_broadcastw_epi16(lo(m[12])); // vpbroadcastw m12, xm12
    let r5 = (dy as i32).wrapping_add(mb).wrapping_sub(64); // lea r5d, [dyq+maxbaseq-64]
    dy = -dy; // neg dyq
    mb |= 63; // or maxbased, 63
    let mut r4 = dy + 63; // lea r4, [dyq+63]
    m[6] = zx(_mm_cvtsi32_si128(r5)); // movd xm6, r5d
    m[10] = zx(ld16_p5(ro(pb_1to32, 16), 0)); // mova xm10, [pb_1to32+16]
    m[10] = _mm256_inserti128_si256::<1>(m[10], ld16_p5(ro(pb_1to32, 0), 0)); // vinserti128 m10, [pb_1to32], 1
    m[11] = bcd_p5(ro(z2_y_shuf_h4, 16)); // vpbroadcastd m11, [pb_32]
    m[6] = _mm256_broadcastw_epi16(lo(m[6])); // vpbroadcastw m6, xm6
    'transpose: {
        loop {
            // .h64_loop:
            let r5 = r4 >> 6; // mov r5, r4 / sar r5, 6
            m[0] = ld32_p5(tl, r5 as isize - 24); // movu m0, [tlq+r5-24]
            m[1] = ld32_p5(tl, r5 as isize - 32); // movu m1, [tlq+r5-32]
            m[2] = _mm256_and_si256(m[4], m[6]); // pand m2, m4, m6
            m[9] = _mm256_sub_epi16(m[5], m[2]); // psubw m9, m5, m2
            m[2] = _mm256_slli_epi16::<8>(m[2]); // psllw m2, 8
            m[9] = _mm256_or_si256(m[9], m[2]); // por m9, m2
            m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
            m[1] = _mm256_shuffle_epi8(m[1], m[8]); // pshufb m1, m8
            m[0] = _mm256_maddubs_epi16(m[0], m[9]); // pmaddubsw m0, m9
            m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
            m[2] = _mm256_srai_epi16::<6>(m[6]); // psraw m2, m6, 6
            rsp = rsp.sub(64); // sub rsp, 64
            m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
            m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
            m[2] = _mm256_packs_epi16(m[2], m[2]); // packsswb m2, m2
            m[2] = _mm256_add_epi8(m[2], m[10]); // paddb m2, m10
            m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
            m[0] = _mm256_blendv_epi8(m[7], m[0], m[2]); // vpblendvb m0, m7, m0, m2
            _mm256_store_si256(rsp.add(32).cast(), m[0]); // mova [rsp+32], m0
            m[0] = ld32_p5(tl, r5 as isize - 56); // movu m0, [tlq+r5-56]
            m[1] = ld32_p5(tl, r5 as isize - 64); // movu m1, [tlq+r5-64]
            r4 += dy; // add r4, dyq
            m[0] = _mm256_shuffle_epi8(m[0], m[8]); // pshufb m0, m8
            m[1] = _mm256_shuffle_epi8(m[1], m[8]); // pshufb m1, m8
            m[0] = _mm256_maddubs_epi16(m[0], m[9]); // pmaddubsw m0, m9
            m[1] = _mm256_maddubs_epi16(m[1], m[9]); // pmaddubsw m1, m9
            m[2] = _mm256_add_epi8(m[2], m[11]); // paddb m2, m11
            m[0] = _mm256_mulhrs_epi16(m[0], m[3]); // pmulhrsw m0, m3
            m[1] = _mm256_mulhrs_epi16(m[1], m[3]); // pmulhrsw m1, m3
            m[6] = _mm256_add_epi16(m[6], m[12]); // paddw m6, m12
            m[0] = _mm256_packus_epi16(m[0], m[1]); // packuswb m0, m1
            m[0] = _mm256_blendv_epi8(m[7], m[0], m[2]); // vpblendvb m0, m7, m0, m2
            _mm256_store_si256(rsp.cast(), m[0]); // mova [rsp], m0
            w -= 1; // dec wd
            if w == 0 {
                break 'transpose; // jz .h64_transpose
            }
            if r4 as i32 <= mb {
                break; // cmp r4d, maxbased / jg .h64_loop
            }
        }
        loop {
            // .h64_end_loop:
            rsp = rsp.sub(64); // sub rsp, 64
            _mm256_store_si256(rsp.add(32).cast(), m[7]); // mova [rsp+32], m7
            _mm256_store_si256(rsp.cast(), m[7]); // mova [rsp+ 0], m7
            w -= 1; // dec wd
            if w <= 0 {
                break; // jg .h64_end_loop
            }
        }
    }
    // .h64_transpose:
    let r2 = stride * 3; // lea r2, [strideq*3]
    let r3 = stride * 5; // lea r3, [strideq*5]
    let r5 = stride * -8; // imul r5, strideq, -8
    dst = at(dst, org_w as isize - 16); // lea dstq, [dstq+org_wq-16]
    let r4 = stride + r2 * 2; // lea r4, [strideq+r2*2] ; stride*7
    let ld = |p: *mut u8, off: usize| _mm_load_si128(p.add(off).cast());
    loop {
        // .h64_transpose_loop0:
        let mut r6 = rsp.wrapping_add(16 * 3); // lea r6, [rsp+16*3]
        loop {
            // .h64_transpose_loop:
            m[0] = zx(ld(r6, 64 * 15)); // mova xm0, [r6+64*15]
            m[0] = _mm256_inserti128_si256::<1>(m[0], ld(r6, 64 * 7)); // vinserti128 m0, [r6+64* 7], 1
            m[1] = zx(ld(r6, 64 * 14)); // mova xm1, [r6+64*14]
            m[1] = _mm256_inserti128_si256::<1>(m[1], ld(r6, 64 * 6)); // vinserti128 m1, [r6+64* 6], 1
            m[2] = zx(ld(r6, 64 * 13)); // mova xm2, [r6+64*13]
            m[2] = _mm256_inserti128_si256::<1>(m[2], ld(r6, 64 * 5)); // vinserti128 m2, [r6+64* 5], 1
            m[3] = zx(ld(r6, 64 * 12)); // mova xm3, [r6+64*12]
            m[3] = _mm256_inserti128_si256::<1>(m[3], ld(r6, 64 * 4)); // vinserti128 m3, [r6+64* 4], 1
            m[4] = zx(ld(r6, 64 * 11)); // mova xm4, [r6+64*11]
            m[4] = _mm256_inserti128_si256::<1>(m[4], ld(r6, 64 * 3)); // vinserti128 m4, [r6+64* 3], 1
            m[5] = zx(ld(r6, 64 * 10)); // mova xm5, [r6+64*10]
            m[5] = _mm256_inserti128_si256::<1>(m[5], ld(r6, 64 * 2)); // vinserti128 m5, [r6+64* 2], 1
            m[6] = zx(ld(r6, 64 * 9)); // mova xm6, [r6+64* 9]
            m[6] = _mm256_inserti128_si256::<1>(m[6], ld(r6, 64 * 1)); // vinserti128 m6, [r6+64* 1], 1
            m[7] = zx(ld(r6, 64 * 8)); // mova xm7, [r6+64* 8]
            m[7] = _mm256_inserti128_si256::<1>(m[7], ld(r6, 0)); // vinserti128 m7, [r6+64* 0], 1
            r6 = r6.wrapping_sub(16); // sub r6, 16
            m[8] = _mm256_unpacklo_epi8(m[0], m[1]); // punpcklbw m8, m0, m1
            m[0] = _mm256_unpackhi_epi8(m[0], m[1]); // punpckhbw m0, m1
            m[1] = _mm256_unpacklo_epi8(m[2], m[3]); // punpcklbw m1, m2, m3
            m[2] = _mm256_unpackhi_epi8(m[2], m[3]); // punpckhbw m2, m3
            m[3] = _mm256_unpacklo_epi8(m[4], m[5]); // punpcklbw m3, m4, m5
            m[4] = _mm256_unpackhi_epi8(m[4], m[5]); // punpckhbw m4, m5
            m[5] = _mm256_unpacklo_epi8(m[6], m[7]); // punpcklbw m5, m6, m7
            m[6] = _mm256_unpackhi_epi8(m[6], m[7]); // punpckhbw m6, m7
            m[7] = _mm256_unpacklo_epi16(m[8], m[1]); // punpcklwd m7, m8, m1
            m[8] = _mm256_unpackhi_epi16(m[8], m[1]); // punpckhwd m8, m1
            m[1] = _mm256_unpacklo_epi16(m[0], m[2]); // punpcklwd m1, m0, m2
            m[0] = _mm256_unpackhi_epi16(m[0], m[2]); // punpckhwd m0, m2
            m[2] = _mm256_unpacklo_epi16(m[3], m[5]); // punpcklwd m2, m3, m5
            m[3] = _mm256_unpackhi_epi16(m[3], m[5]); // punpckhwd m3, m5
            m[5] = _mm256_unpacklo_epi16(m[4], m[6]); // punpcklwd m5, m4, m6
            m[4] = _mm256_unpackhi_epi16(m[4], m[6]); // punpckhwd m4, m6
            m[6] = _mm256_unpacklo_epi32(m[7], m[2]); // punpckldq m6, m7, m2
            m[7] = _mm256_unpackhi_epi32(m[7], m[2]); // punpckhdq m7, m2
            m[2] = _mm256_unpacklo_epi32(m[8], m[3]); // punpckldq m2, m8, m3
            m[8] = _mm256_unpackhi_epi32(m[8], m[3]); // punpckhdq m8, m3
            m[3] = _mm256_unpacklo_epi32(m[1], m[5]); // punpckldq m3, m1, m5
            m[1] = _mm256_unpackhi_epi32(m[1], m[5]); // punpckhdq m1, m5
            m[5] = _mm256_unpacklo_epi32(m[0], m[4]); // punpckldq m5, m0, m4
            m[0] = _mm256_unpackhi_epi32(m[0], m[4]); // punpckhdq m0, m4
            m[6] = _mm256_permute4x64_epi64::<0xd8>(m[6]); // vpermq m6, m6, q3120
            m[7] = _mm256_permute4x64_epi64::<0xd8>(m[7]); // vpermq m7, m7, q3120
            m[2] = _mm256_permute4x64_epi64::<0xd8>(m[2]); // vpermq m2, m2, q3120
            m[8] = _mm256_permute4x64_epi64::<0xd8>(m[8]); // vpermq m8, m8, q3120
            m[3] = _mm256_permute4x64_epi64::<0xd8>(m[3]); // vpermq m3, m3, q3120
            m[1] = _mm256_permute4x64_epi64::<0xd8>(m[1]); // vpermq m1, m1, q3120
            m[5] = _mm256_permute4x64_epi64::<0xd8>(m[5]); // vpermq m5, m5, q3120
            m[0] = _mm256_permute4x64_epi64::<0xd8>(m[0]); // vpermq m0, m0, q3120
            _mm_storeu_si128(dst.cast(), lo(m[6])); // mova [dstq+strideq*0], xm6
            _mm_storeu_si128(at(dst, stride).cast(), _mm256_extracti128_si256::<1>(m[6])); // vextracti128 [dstq+strideq*1], m6, 1
            _mm_storeu_si128(at(dst, stride * 2).cast(), lo(m[7])); // mova [dstq+strideq*2], xm7
            _mm_storeu_si128(at(dst, r2).cast(), _mm256_extracti128_si256::<1>(m[7])); // vextracti128 [dstq+r2], m7, 1
            _mm_storeu_si128(at(dst, stride * 4).cast(), lo(m[2])); // mova [dstq+strideq*4], xm2
            _mm_storeu_si128(at(dst, r3).cast(), _mm256_extracti128_si256::<1>(m[2])); // vextracti128 [dstq+r3], m2, 1
            _mm_storeu_si128(at(dst, r2 * 2).cast(), lo(m[8])); // mova [dstq+r2*2], xm8
            _mm_storeu_si128(at(dst, r4).cast(), _mm256_extracti128_si256::<1>(m[8])); // vextracti128 [dstq+r4], m8, 1
            dst = at(dst, -r5); // sub dstq, r5
            _mm_storeu_si128(dst.cast(), lo(m[3])); // mova [dstq+strideq*0], xm3
            _mm_storeu_si128(at(dst, stride).cast(), _mm256_extracti128_si256::<1>(m[3])); // vextracti128 [dstq+strideq*1], m3, 1
            _mm_storeu_si128(at(dst, stride * 2).cast(), lo(m[1])); // mova [dstq+strideq*2], xm1
            _mm_storeu_si128(at(dst, r2).cast(), _mm256_extracti128_si256::<1>(m[1])); // vextracti128 [dstq+r2], m1, 1
            _mm_storeu_si128(at(dst, stride * 4).cast(), lo(m[5])); // mova [dstq+strideq*4], xm5
            _mm_storeu_si128(at(dst, r3).cast(), _mm256_extracti128_si256::<1>(m[5])); // vextracti128 [dstq+r3], m5, 1
            _mm_storeu_si128(at(dst, r2 * 2).cast(), lo(m[0])); // mova [dstq+r2*2], xm0
            _mm_storeu_si128(at(dst, r4).cast(), _mm256_extracti128_si256::<1>(m[0])); // vextracti128 [dstq+r4], m0, 1
            dst = at(dst, -r5); // sub dstq, r5
            if r6 < rsp {
                break; // cmp r6, rsp / jae .h64_transpose_loop
            }
        }
        rsp = rsp.add(64 * 16); // add rsp, 64*16
        dst = at(dst, r5 * 8 - 16); // lea dstq, [dstq+r5*8-16]
        org_w -= 16; // sub org_wd, 16
        if org_w <= 0 {
            break; // jg .h64_transpose_loop0
        }
    }
    // .h64_end: RET
}

// ---- part 6: `ipred_avx2.asm` lines 4141–4673 (FILTER_XMM, FILTER_YMM, filter; IPRED_CFL, cfl_top, cfl_left, cfl, cfl_128) ----

// Memory operands of part 6 (suffix `_p6`: candidates for the prelude). Every access to caller memory is an
// unaligned access of exactly the bytes the instruction's memory operand covers.

/// `p + off` bytes for a read (wrapping: no claim that the address is in bounds until it is accessed).
#[inline(always)]
fn a_p6(p: *const u8, off: isize) -> *const u8 {
    p.wrapping_offset(off)
}

/// The 4 bytes at `p` (`movd`, `pinsrd`, `vpbroadcastd` memory operands).
#[inline(always)]
unsafe fn rd32_p6(p: *const u8) -> i32 {
    p.cast::<i32>().read_unaligned()
}

/// `movq xm, [p]` (8 bytes; the rest of the register is cleared).
#[inline(always)]
unsafe fn movq_p6(p: *const u8) -> __m128i {
    _mm_loadl_epi64(p.cast())
}

/// A 16-byte memory operand (`movu xm, [p]`, `mova xm, [p]`, `pshufb xm, [p]`, `vinserti128 m, [p], 0`).
#[inline(always)]
unsafe fn x_p6(p: *const u8) -> __m128i {
    _mm_loadu_si128(p.cast())
}

/// A 32-byte memory operand (`movu m, [p]`, `mova m, [p]`, `vpblendd m, [p], …`).
#[inline(always)]
unsafe fn y_p6(p: *const u8) -> __m256i {
    _mm256_loadu_si256(p.cast())
}

/// `vpbroadcastd xm, [p]`.
#[inline(always)]
unsafe fn vpbroadcastd_x_p6(p: *const u8) -> __m128i {
    _mm_set1_epi32(rd32_p6(p))
}

/// `vpbroadcastd m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastd_y_p6(p: *const u8) -> __m256i {
    _mm256_set1_epi32(rd32_p6(p))
}

/// `vbroadcasti128 m, [p]`.
#[inline(always)]
unsafe fn vbroadcasti128_p6(p: *const u8) -> __m256i {
    _mm256_broadcastsi128_si256(x_p6(p))
}

/// `pinsrd xmD, xmA, [p], I`.
#[inline(always)]
unsafe fn pinsrd_p6<const I: i32>(x: __m128i, p: *const u8) -> __m128i {
    _mm_insert_epi32::<I>(x, rd32_p6(p))
}

/// `movlps xmD, xmA, [p]`: the 8 bytes at `p` as the low quadword, `xmA`'s high quadword (an unaligned read
/// inserted as an element, `vpinsrq xmD, xmA, [p], 0`, one instruction, where `_mm_loadl_pd` would read through
/// an aligned `*const f64`; the 8 bytes are only moved).
#[inline(always)]
unsafe fn movlps_p6(x: __m128i, p: *const u8) -> __m128i {
    _mm_insert_epi64::<0>(x, p.cast::<i64>().read_unaligned())
}

/// `shufps xmD, xmA, xmB, IMM` on integer registers.
#[inline(always)]
unsafe fn shufps_p6<const IMM: i32>(a: __m128i, b: __m128i) -> __m128i {
    _mm_castps_si128(_mm_shuffle_ps::<IMM>(_mm_castsi128_ps(a), _mm_castsi128_ps(b)))
}

/// `movd [p], xm`.
#[inline(always)]
unsafe fn movd_st_p6(p: *mut u8, x: __m128i) {
    p.cast::<i32>().write_unaligned(_mm_cvtsi128_si32(x))
}

/// `pextrd [p], xm, I`.
#[inline(always)]
unsafe fn pextrd_st_p6<const I: i32>(p: *mut u8, x: __m128i) {
    p.cast::<i32>().write_unaligned(_mm_extract_epi32::<I>(x))
}

/// `movq [p], xm`.
#[inline(always)]
unsafe fn movq_st_p6(p: *mut u8, x: __m128i) {
    _mm_storel_epi64(p.cast(), x)
}

/// `movhps [p], xm`: the high quadword, as the unaligned write of an extracted element (`vpextrq [p], xm, 1`,
/// the same 8 bytes in one instruction).
#[inline(always)]
unsafe fn movhps_st_p6(p: *mut u8, x: __m128i) {
    p.cast::<i64>().write_unaligned(_mm_extract_epi64::<1>(x))
}

/// `mova [p], xm` (16 bytes).
#[inline(always)]
unsafe fn x_st_p6(p: *mut u8, x: __m128i) {
    _mm_storeu_si128(p.cast(), x)
}

/// `mova [p], m` (32 bytes).
#[inline(always)]
unsafe fn y_st_p6(p: *mut u8, x: __m256i) {
    _mm256_storeu_si256(p.cast(), x)
}

/// `vextracti128 [p], m, 1`.
#[inline(always)]
unsafe fn vextracti128_st_p6(p: *mut u8, x: __m256i) {
    _mm_storeu_si128(p.cast(), _mm256_extracti128_si256::<1>(x))
}

/// `pw_8` (`%define pw_8 (z_filter_k+32)` in the assembly's read-only data; not a label of its own).
const pw_8_p6: usize = z_filter_k + 32;

// x86inc's shuffle immediates `qABCD` = `A<<6|B<<4|C<<2|D`.
const Q0000_P6: i32 = 0;
const Q1111_P6: i32 = 1 << 6 | 1 << 4 | 1 << 2 | 1;
const Q2222_P6: i32 = 2 << 6 | 2 << 4 | 2 << 2 | 2;
const Q3333_P6: i32 = 3 << 6 | 3 << 4 | 3 << 2 | 3;
const Q3120_P6: i32 = 3 << 6 | 1 << 4 | 2 << 2 | 0;
const Q1032_P6: i32 = 1 << 6 | 0 << 4 | 3 << 2 | 2;
const Q2020_P6: i32 = 2 << 6 | 0 << 4 | 2 << 2 | 0;
const Q3131_P6: i32 = 3 << 6 | 1 << 4 | 3 << 2 | 1;

// ---- ipred_filter ----

/// `FILTER_XMM dst, src, tmp, shuf`: `shuf` is the operand of the first `pshufb` (`xm%4` for a register number,
/// the 16 bytes of a memory operand otherwise). `xm%2` is shuffled in place.
#[inline(always)]
unsafe fn filter_xmm(m: &mut M, d: usize, s: usize, t: usize, shuf: __m128i) {
    m[s] = zx(_mm_shuffle_epi8(lo(m[s]), shuf)); // pshufb xm%2, %4
    m[d] = zx(_mm_shuffle_epi32::<Q0000_P6>(lo(m[s]))); // pshufd xm%1, xm%2, q0000 ; p0 p1
    m[d] = zx(_mm_maddubs_epi16(lo(m[d]), lo(m[2]))); // pmaddubsw xm%1, xm2
    m[t] = zx(_mm_shuffle_epi32::<Q1111_P6>(lo(m[s]))); // pshufd xm%3, xm%2, q1111 ; p2 p3
    m[t] = zx(_mm_maddubs_epi16(lo(m[t]), lo(m[3]))); // pmaddubsw xm%3, xm3
    m[d] = zx(_mm_add_epi16(lo(m[d]), lo(m[1]))); // paddw xm%1, xm1
    m[d] = zx(_mm_add_epi16(lo(m[d]), lo(m[t]))); // paddw xm%1, xm%3
    m[t] = zx(_mm_shuffle_epi32::<Q2222_P6>(lo(m[s]))); // pshufd xm%3, xm%2, q2222 ; p4 p5
    m[t] = zx(_mm_maddubs_epi16(lo(m[t]), lo(m[4]))); // pmaddubsw xm%3, xm4
    m[d] = zx(_mm_add_epi16(lo(m[d]), lo(m[t]))); // paddw xm%1, xm%3
    m[t] = zx(_mm_shuffle_epi32::<Q3333_P6>(lo(m[s]))); // pshufd xm%3, xm%2, q3333 ; p6 __
    m[t] = zx(_mm_maddubs_epi16(lo(m[t]), lo(m[5]))); // pmaddubsw xm%3, xm5
    m[d] = zx(_mm_add_epi16(lo(m[d]), lo(m[t]))); // paddw xm%1, xm%3
    m[d] = zx(_mm_srai_epi16::<4>(lo(m[d]))); // psraw xm%1, 4
    m[d] = zx(_mm_packus_epi16(lo(m[d]), lo(m[d]))); // packuswb xm%1, xm%1
}

/// `FILTER_YMM dst, src, tmp, shuf` (all register numbers). `m%2` is shuffled in place.
#[inline(always)]
unsafe fn filter_ymm(m: &mut M, d: usize, s: usize, t: usize, sh: usize) {
    m[s] = _mm256_shuffle_epi8(m[s], m[sh]); // pshufb m%2, m%4
    m[d] = _mm256_shuffle_epi32::<Q0000_P6>(m[s]); // pshufd m%1, m%2, q0000
    m[d] = _mm256_maddubs_epi16(m[d], m[2]); // pmaddubsw m%1, m2
    m[t] = _mm256_shuffle_epi32::<Q1111_P6>(m[s]); // pshufd m%3, m%2, q1111
    m[t] = _mm256_maddubs_epi16(m[t], m[3]); // pmaddubsw m%3, m3
    m[d] = _mm256_add_epi16(m[d], m[1]); // paddw m%1, m1
    m[d] = _mm256_add_epi16(m[d], m[t]); // paddw m%1, m%3
    m[t] = _mm256_shuffle_epi32::<Q2222_P6>(m[s]); // pshufd m%3, m%2, q2222
    m[t] = _mm256_maddubs_epi16(m[t], m[4]); // pmaddubsw m%3, m4
    m[d] = _mm256_add_epi16(m[d], m[t]); // paddw m%1, m%3
    m[t] = _mm256_shuffle_epi32::<Q3333_P6>(m[s]); // pshufd m%3, m%2, q3333
    m[t] = _mm256_maddubs_epi16(m[t], m[5]); // pmaddubsw m%3, m5
    m[d] = _mm256_add_epi16(m[d], m[t]); // paddw m%1, m%3
    m[d] = _mm256_srai_epi16::<4>(m[d]); // psraw m%1, 4
    m[t] = _mm256_permute2x128_si256::<0x01>(m[d], m[d]); // vperm2i128 m%3, m%1, m%1, 0x01
    m[d] = _mm256_packus_epi16(m[d], m[t]); // packuswb m%1, m%3
}

/// `ipred_filter_8bpc.main` (`call`ed).
#[inline(always)]
unsafe fn ipred_filter_8bpc_main(m: &mut M) {
    filter_ymm(m, 7, 0, 9, 8); // FILTER_YMM 7, 0, 9, 8
}

/// `.w4` (`tl` is `tlq`, `h` is `hq`).
#[inline(always)]
unsafe fn ipred_filter_8bpc_w4(m: &mut M, mut dst: *mut u8, stride: isize, tl: *const u8, mut h: isize) {
    m[8] = zx(x_p6(ro(filter_shuf2, 0))); // mova xm8, [base+filter_shuf2]
    let tl = a_p6(tl, -3); // sub tlq, 3
    let tl = a_p6(tl, -h); // sub tlq, hq
                           // jmp .w4_loop_start: the loop below starts there; its head (`.w4_loop`) is at its end.
    loop {
        filter_xmm(m, 6, 0, 7, lo(m[8])); // FILTER_XMM 6, 0, 7, 8
        movd_st_p6(dst, lo(m[6])); // movd [dstq+strideq*0], xm6
        pextrd_st_p6::<1>(at(dst, stride), lo(m[6])); // pextrd [dstq+strideq*1], xm6, 1
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w4_loop
        }
        m[0] = zx(pinsrd_p6::<0>(lo(m[6]), a_p6(tl, h))); // pinsrd xm0, xm6, [tlq+hq], 0
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
    }
}

/// `.w8`.
#[inline(always)]
unsafe fn ipred_filter_8bpc_w8(m: &mut M, mut dst: *mut u8, stride: isize, tl: *const u8, mut h: isize) {
    m[8] = y_p6(ro(filter_shuf1, 0)); // mova m8, [base+filter_shuf1]
    filter_xmm(m, 7, 0, 6, x_p6(ro(filter_shuf2, 0))); // FILTER_XMM 7, 0, 6, [base+filter_shuf2]
    m[0] = vpbroadcastd_y_p6(a_p6(tl, 4)); // vpbroadcastd m0, [tlq+4]
    m[6] = vpbroadcastd_y_p6(a_p6(tl, 5)); // vpbroadcastd m6, [tlq+5]
    let tl = a_p6(tl, -4); // sub tlq, 4
    let tl = a_p6(tl, -h); // sub tlq, hq
    m[7] = _mm256_broadcastq_epi64(lo(m[7])); // vpbroadcastq m7, xm7
    m[7] = _mm256_blend_epi32::<0x20>(m[7], m[6]); // vpblendd m7, m6, 0x20
    loop {
        // .w8_loop:
        m[6] = zx(vpbroadcastd_x_p6(a_p6(tl, h))); // vpbroadcastd xm6, [tlq+hq]
        m[6] = _mm256_alignr_epi8::<12>(m[6], m[0]); // palignr m6, m0, 12
        m[0] = _mm256_blend_epi32::<0xeb>(m[6], m[7]); // vpblendd m0, m6, m7, 0xeb
        m[6] = zx(lo(m[7])); // mova xm6, xm7
        ipred_filter_8bpc_main(m); // call .main
        m[6] = zx(_mm_blend_epi32::<0x0c>(lo(m[6]), lo(m[7]))); // vpblendd xm6, xm7, 0x0c
        m[6] = zx(_mm_shuffle_epi32::<Q3120_P6>(lo(m[6]))); // pshufd xm6, xm6, q3120
        movq_st_p6(dst, lo(m[6])); // movq [dstq+strideq*0], xm6
        movhps_st_p6(at(dst, stride), lo(m[6])); // movhps [dstq+strideq*1], xm6
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w8_loop
        }
    }
}

/// From `.w32_loop_last` to the stores of the row pair (`.w16_loop` has the same instructions after its
/// `vpbroadcastd`; both use this function).
#[inline(always)]
unsafe fn ipred_filter_8bpc_w32_loop_last(m: &mut M, dst: *mut u8, stride: isize) {
    m[9] = _mm256_alignr_epi8::<12>(m[9], m[0]); // palignr m9, m0, 12
    m[0] = _mm256_blend_epi32::<0xe2>(m[9], m[7]); // vpblendd m0, m9, m7, 0xe2
    m[13] = zx(lo(m[7])); // mova xm13, xm7
    ipred_filter_8bpc_main(m); // call .main ; e0 f0 c1 d1   c1 d1 e0 f0
    m[9] = _mm256_blend_epi32::<0xf0>(m[12], m[10]); // vpblendd m9, m12, m10, 0xf0
    m[12] = _mm256_blend_epi32::<0xc0>(m[12], m[6]); // vpblendd m12, m6, 0xc0
    m[9] = _mm256_shuffle_epi32::<Q3333_P6>(m[9]); // pshufd m9, m9, q3333
    m[9] = _mm256_blend_epi32::<0xee>(m[9], m[6]); // vpblendd m9, m6, 0xee
    m[10] = _mm256_blend_epi32::<0x0c>(m[9], m[7]); // vpblendd m10, m9, m7, 0x0c
    filter_ymm(m, 6, 10, 9, 14); // FILTER_YMM 6, 10, 9, 14 ; c2 d2 a3 b3   a3 b3 c2 d2
    m[12] = _mm256_blend_epi32::<0x30>(m[12], m[6]); // vpblendd m12, m6, 0x30 ; a0 b0 a1 b1   a3 b3 a2 b2
    m[9] = _mm256_permutevar8x32_epi32(m[12], m[11]); // vpermd m9, m11, m12 ; a0 a1 a2 a3   b0 b1 b2 b3
    m[12] = zx(_mm_blend_epi32::<0x0c>(lo(m[13]), lo(m[7]))); // vpblendd xm12, xm13, xm7, 0x0c ; c0 d0 c1 d1
    x_st_p6(dst, lo(m[9])); // mova [dstq+strideq*0], xm9
    vextracti128_st_p6(at(dst, stride), m[9]); // vextracti128 [dstq+strideq*1], m9, 1
}

/// The last two rows after `.w16_loop` and `.w32_loop` (the same instructions at both places).
#[inline(always)]
unsafe fn ipred_filter_8bpc_end_rows_p6(m: &mut M, dst: *mut u8, stride: isize) {
    m[7] = zx(_mm_blend_epi32::<0x04>(lo(m[6]), lo(m[10]))); // vpblendd xm7, xm6, xm10, 0x04
    m[7] = zx(_mm_shuffle_epi32::<Q1032_P6>(lo(m[7]))); // pshufd xm7, xm7, q1032
    filter_xmm(m, 0, 7, 9, x_p6(ro(filter_shuf1, 16))); // FILTER_XMM 0, 7, 9, [base+filter_shuf1+16]
    m[6] = zx(_mm_blend_epi32::<0x0c>(lo(m[6]), lo(m[0]))); // vpblendd xm6, xm0, 0x0c ; c2 d2 c3 d3
    m[0] = zx(shufps_p6::<Q2020_P6>(lo(m[12]), lo(m[6]))); // shufps xm0, xm12, xm6, q2020 ; c0 c1 c2 c3
    m[6] = zx(shufps_p6::<Q3131_P6>(lo(m[12]), lo(m[6]))); // shufps xm6, xm12, xm6, q3131 ; d0 d1 d2 d3
    x_st_p6(dst, lo(m[0])); // mova [dstq+strideq*0], xm0
    x_st_p6(at(dst, stride), lo(m[6])); // mova [dstq+strideq*1], xm6
}

/// `.w16_main` (`call`ed by `.w16` and `.w32`): returns `tlq` as it leaves it.
#[inline(always)]
unsafe fn ipred_filter_8bpc_w16_main(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    tl: *const u8,
    mut h: isize,
) -> *const u8 {
    filter_xmm(m, 12, 0, 7, x_p6(ro(filter_shuf2, 0))); // FILTER_XMM 12, 0, 7, [base+filter_shuf2]
    m[0] = vpbroadcastd_y_p6(a_p6(tl, 5)); // vpbroadcastd m0, [tlq+5]
    m[0] = _mm256_blend_epi32::<0x14>(m[0], y_p6(a_p6(tl, -12))); // vpblendd m0, [tlq-12], 0x14
    m[8] = y_p6(ro(filter_shuf1, 0)); // mova m8, [base+filter_shuf1]
    m[7] = _mm256_broadcastq_epi64(lo(m[12])); // vpbroadcastq m7, xm12
    m[0] = _mm256_blend_epi32::<0xc2>(m[0], m[7]); // vpblendd m0, m7, 0xc2
    ipred_filter_8bpc_main(m); // call .main ; c0 d0 a1 b1   a1 b1 c0 d0
    m[9] = zx(movlps_p6(lo(m[7]), a_p6(tl, 5))); // movlps xm9, xm7, [tlq+5]
    m[14] = _mm256_inserti128_si256::<0>(m[8], x_p6(ro(filter_shuf3, 0))); // vinserti128 m14, m8, [base+filter_shuf3], 0
    m[12] = zx(_mm_blend_epi32::<0x0c>(lo(m[12]), lo(m[7]))); // vpblendd xm12, xm7, 0x0c ; a0 b0 a1 b1
    filter_xmm(m, 6, 9, 10, lo(m[14])); // FILTER_XMM 6, 9, 10, 14
    m[6] = _mm256_broadcastq_epi64(lo(m[6])); // vpbroadcastq m6, xm6 ; a2 b2 __ __ __ __ a2 b2
    m[9] = vpbroadcastd_y_p6(a_p6(tl, 13)); // vpbroadcastd m9, [tlq+13]
    m[10] = vpbroadcastd_y_p6(a_p6(tl, 12)); // vpbroadcastd m10, [tlq+12]
    m[11] = _mm256_srli_epi32::<4>(m[8]); // psrld m11, m8, 4
    m[6] = _mm256_blend_epi32::<0x20>(m[6], m[9]); // vpblendd m6, m9, 0x20 ; top
    let tl = a_p6(tl, -6); // sub tlq, 6
    let tl = a_p6(tl, -h); // sub tlq, hq
    loop {
        // .w16_loop:
        m[9] = zx(vpbroadcastd_x_p6(a_p6(tl, h))); // vpbroadcastd xm9, [tlq+hq]
        ipred_filter_8bpc_w32_loop_last(m, dst, stride); // palignr m9, m0, 12 … vextracti128 [dstq+strideq*1], m9, 1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w16_loop
        }
    }
    ipred_filter_8bpc_end_rows_p6(m, dst, stride); // vpblendd xm7, xm6, xm10, 0x04 … mova [dstq+strideq*1], xm6
    tl // ret
}

/// `.w16`.
#[inline(always)]
unsafe fn ipred_filter_8bpc_w16(m: &mut M, dst: *mut u8, stride: isize, tl: *const u8, mut h: isize) {
    h -= 2; // sub hd, 2
    ipred_filter_8bpc_w16_main(m, dst, stride, tl, h); // call .w16_main
}

/// `.w32`.
#[inline(always)]
unsafe fn ipred_filter_8bpc_w32(m: &mut M, dst: *mut u8, stride: isize, tl: *const u8, mut h: isize) {
    h -= 2; // sub hd, 2
    let r3 = at(dst, 16); // lea r3, [dstq+16]
    let mut r5 = h - 2; // lea r5d, [hq-2]
    let tl = ipred_filter_8bpc_w16_main(m, dst, stride, tl, h); // call .w16_main
    let tl = a_p6(tl, r5); // add tlq, r5
    let mut dst = r3; // mov dstq, r3
    let mut r3 = stride - 4; // lea r3, [strideq-4]
    let mut r4 = r3 + stride * 2; // lea r4, [r3+strideq*2]
    m[0] = zx(movq_p6(a_p6(tl, 21))); // movq xm0, [tlq+21]
    m[0] = zx(pinsrd_p6::<2>(lo(m[0]), at(dst, -4))); // pinsrd xm0, [dstq-4], 2
    m[0] = zx(pinsrd_p6::<3>(lo(m[0]), at(dst, r3))); // pinsrd xm0, [dstq+r3*1], 3
    filter_xmm(m, 12, 0, 7, lo(m[14])); // FILTER_XMM 12, 0, 7, 14 ; a0 b0 a0 b0
    m[7] = zx(movq_p6(at(dst, r3 * 2))); // movq xm7, [dstq+r3*2]
    m[7] = zx(pinsrd_p6::<2>(lo(m[7]), at(dst, r4))); // pinsrd xm7, [dstq+r4], 2
    m[7] = zx(_mm_alignr_epi8::<12>(lo(m[7]), lo(m[0]))); // palignr xm7, xm0, 12
    m[0] = vpbroadcastd_y_p6(a_p6(tl, 28)); // vpbroadcastd m0, [tlq+28]
    m[9] = vpbroadcastd_y_p6(a_p6(tl, 29)); // vpbroadcastd m9, [tlq+29]
    m[8] = vbroadcasti128_p6(ro(filter_shuf1, 16)); // vbroadcasti128 m8, [base+filter_shuf1+16]
    m[0] = _mm256_blend_epi32::<0x20>(m[0], m[9]); // vpblendd m0, m9, 0x20
    m[0] = _mm256_blend_epi32::<0x0f>(m[0], m[7]); // vpblendd m0, m7, 0x0f
    m[7] = _mm256_broadcastq_epi64(lo(m[12])); // vpbroadcastq m7, xm12
    m[0] = _mm256_blend_epi32::<0xc2>(m[0], m[7]); // vpblendd m0, m7, 0xc2
    ipred_filter_8bpc_main(m); // call .main ; c0 d0 a1 b1   a1 b1 c0 d0
    r3 += 2; // add r3, 2
    r4 += stride * 2; // lea r4, [r4+strideq*2]
    m[9] = zx(movlps_p6(lo(m[7]), a_p6(tl, 29))); // movlps xm9, xm7, [tlq+29]
    m[12] = zx(_mm_blend_epi32::<0x0c>(lo(m[12]), lo(m[7]))); // vpblendd xm12, xm7, 0x0c ; a0 b0 a1 b1
    filter_xmm(m, 6, 9, 10, lo(m[14])); // FILTER_XMM 6, 9, 10, 14
    m[6] = _mm256_broadcastq_epi64(lo(m[6])); // vpbroadcastq m6, xm6 ; a2 b2 __ __ __ __ a2 b2
    m[9] = vpbroadcastd_y_p6(a_p6(tl, 37)); // vpbroadcastd m9, [tlq+37]
    m[10] = vpbroadcastd_y_p6(a_p6(tl, 36)); // vpbroadcastd m10, [tlq+36]
    m[6] = _mm256_blend_epi32::<0x20>(m[6], m[9]); // vpblendd m6, m9, 0x20 ; top
    let mut reload = true;
    loop {
        if reload {
            // .w32_loop:
            m[9] = zx(movq_p6(at(dst, r3 * 4))); // movq xm9, [dstq+r3*4]
            m[9] = zx(pinsrd_p6::<2>(lo(m[9]), at(dst, r4))); // pinsrd xm9, [dstq+r4], 2
        }
        ipred_filter_8bpc_w32_loop_last(m, dst, stride); // .w32_loop_last: … vextracti128 [dstq+strideq*1], m9, 1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        r5 -= 2; // sub r5d, 2
        if r5 > 0 {
            reload = true; // jg .w32_loop
        } else if r5 == 0 {
            reload = false; // jz .w32_loop_last
        } else {
            break;
        }
    }
    ipred_filter_8bpc_end_rows_p6(m, dst, stride); // vpblendd xm7, xm6, xm10, 0x04 … mova [dstq+strideq*1], xm6
}

/// `ipred_filter_8bpc` (`cglobal ipred_filter_8bpc, 3, 7, 0, dst, stride, tl, w, h, filter`).
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_filter_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    filter: c_int,
    _max_width: c_int,
    _max_height: c_int,
) {
    let mut regs = regs();
    let m = &mut regs;
    let tl = topleft;
    // tzcnt wd, wm (`width` is 4, 8, 16 or 32: never zero, so `trailing_zeros` is `tzcnt`'s value)
    let w = (width as u32).trailing_zeros();
    let filter = (filter as u32) & 0xff; // movzx filterd, filterb
    let filter = filter << 6; // shl filterd, 6
                              // add filterq, r6 (r6 = filter_intra_taps: 5 sets of 64 signed bytes; the contract's `filter` is 0..=4, so
                              // the four 16-byte loads below stay inside the table)
    let filterq = (core::ptr::addr_of!(crate::common::tables::dav1d_filter_intra_taps) as *const u8)
        .wrapping_add(filter as usize);
    m[0] = zx(movq_p6(a_p6(tl, -3))); // movq xm0, [tlq-3] ; _ 6 5 0 1 2 3 4
    m[1] = vpbroadcastd_y_p6(ro(pw_8_p6, 0)); // vpbroadcastd m1, [base+pw_8]
    m[2] = vbroadcasti128_p6(a_p6(filterq, 16 * 0)); // vbroadcasti128 m2, [filterq+16*0]
    m[3] = vbroadcasti128_p6(a_p6(filterq, 16 * 1)); // vbroadcasti128 m3, [filterq+16*1]
    m[4] = vbroadcasti128_p6(a_p6(filterq, 16 * 2)); // vbroadcasti128 m4, [filterq+16*2]
    m[5] = vbroadcasti128_p6(a_p6(filterq, 16 * 3)); // vbroadcasti128 m5, [filterq+16*3]
    let h = height as u32 as isize; // mov hd, hm
                                    // jmp wq (ipred_filter_avx2_table: w4, w8, w16, w32)
    match w {
        2 => ipred_filter_8bpc_w4(m, dst, stride, tl, h),
        3 => ipred_filter_8bpc_w8(m, dst, stride, tl, h),
        4 => ipred_filter_8bpc_w16(m, dst, stride, tl, h),
        _ => ipred_filter_8bpc_w32(m, dst, stride, tl, h),
    }
}

// ---- ipred_cfl, cfl_top, cfl_left, cfl_128 ----

/// `IPRED_CFL n` (ac in, unpacked pixels out): `m0` the DC, `m1` alpha, `m2` `|alpha| << 9`; `m3` is clobbered.
#[inline(always)]
unsafe fn ipred_cfl(m: &mut M, n: usize) {
    m[3] = _mm256_sign_epi16(m[n], m[1]); // psignw m3, m%1, m1
    m[n] = _mm256_abs_epi16(m[n]); // pabsw m%1, m%1
    m[n] = _mm256_mulhrs_epi16(m[n], m[2]); // pmulhrsw m%1, m2
    m[n] = _mm256_sign_epi16(m[n], m[3]); // psignw m%1, m3
    m[n] = _mm256_add_epi16(m[n], m[0]); // paddw m%1, m0
}

/// `ipred_cfl_8bpc.s4` (`dst`, `stride`, `ac`, `h`, `alpha` as the registers and `alpham`).
#[inline(always)]
unsafe fn ipred_cfl_8bpc_s4(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut ac: *const i16,
    mut h: isize,
    alpha: c_int,
) {
    m[1] = _mm256_set1_epi16(alpha as i16); // vpbroadcastw m1, alpham
    let r6 = stride * 3; // lea r6, [strideq*3]
    m[2] = _mm256_abs_epi16(m[1]); // pabsw m2, m1
    m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
    loop {
        // .s4_loop:
        m[4] = y_p6(ac.cast()); // mova m4, [acq]
        ipred_cfl(m, 4); // IPRED_CFL 4
        m[4] = _mm256_packus_epi16(m[4], m[4]); // packuswb m4, m4
        m[5] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm5, m4, 1
        movd_st_p6(dst, lo(m[4])); // movd [dstq+strideq*0], xm4
        pextrd_st_p6::<1>(at(dst, stride), lo(m[4])); // pextrd [dstq+strideq*1], xm4, 1
        movd_st_p6(at(dst, stride * 2), lo(m[5])); // movd [dstq+strideq*2], xm5
        pextrd_st_p6::<1>(at(dst, r6), lo(m[5])); // pextrd [dstq+r6], xm5, 1
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        ac = ac.wrapping_byte_add(32); // add acq, 32
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .s4_loop
        }
    }
}

/// `ipred_cfl_8bpc.s8`.
#[inline(always)]
unsafe fn ipred_cfl_8bpc_s8(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut ac: *const i16,
    mut h: isize,
    alpha: c_int,
) {
    m[1] = _mm256_set1_epi16(alpha as i16); // vpbroadcastw m1, alpham
    let r6 = stride * 3; // lea r6, [strideq*3]
    m[2] = _mm256_abs_epi16(m[1]); // pabsw m2, m1
    m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
    loop {
        // .s8_loop:
        m[4] = y_p6(ac.cast()); // mova m4, [acq]
        m[5] = y_p6(ac.wrapping_byte_add(32).cast()); // mova m5, [acq+32]
        ipred_cfl(m, 4); // IPRED_CFL 4
        ipred_cfl(m, 5); // IPRED_CFL 5
        m[4] = _mm256_packus_epi16(m[4], m[5]); // packuswb m4, m5
        m[5] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm5, m4, 1
        movq_st_p6(dst, lo(m[4])); // movq [dstq+strideq*0], xm4
        movq_st_p6(at(dst, stride), lo(m[5])); // movq [dstq+strideq*1], xm5
        movhps_st_p6(at(dst, stride * 2), lo(m[4])); // movhps [dstq+strideq*2], xm4
        movhps_st_p6(at(dst, r6), lo(m[5])); // movhps [dstq+r6], xm5
        dst = at(dst, stride * 4); // lea dstq, [dstq+strideq*4]
        ac = ac.wrapping_byte_add(64); // add acq, 64
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .s8_loop
        }
    }
}

/// `ipred_cfl_8bpc.s16`.
#[inline(always)]
unsafe fn ipred_cfl_8bpc_s16(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut ac: *const i16,
    mut h: isize,
    alpha: c_int,
) {
    m[1] = _mm256_set1_epi16(alpha as i16); // vpbroadcastw m1, alpham
    m[2] = _mm256_abs_epi16(m[1]); // pabsw m2, m1
    m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
    loop {
        // .s16_loop:
        m[4] = y_p6(ac.cast()); // mova m4, [acq]
        m[5] = y_p6(ac.wrapping_byte_add(32).cast()); // mova m5, [acq+32]
        ipred_cfl(m, 4); // IPRED_CFL 4
        ipred_cfl(m, 5); // IPRED_CFL 5
        m[4] = _mm256_packus_epi16(m[4], m[5]); // packuswb m4, m5
        m[4] = _mm256_permute4x64_epi64::<Q3120_P6>(m[4]); // vpermq m4, m4, q3120
        x_st_p6(dst, lo(m[4])); // mova [dstq+strideq*0], xm4
        vextracti128_st_p6(at(dst, stride), m[4]); // vextracti128 [dstq+strideq*1], m4, 1
        dst = at(dst, stride * 2); // lea dstq, [dstq+strideq*2]
        ac = ac.wrapping_byte_add(64); // add acq, 64
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .s16_loop
        }
    }
}

/// `ipred_cfl_8bpc.s32`.
#[inline(always)]
unsafe fn ipred_cfl_8bpc_s32(
    m: &mut M,
    mut dst: *mut u8,
    stride: isize,
    mut ac: *const i16,
    mut h: isize,
    alpha: c_int,
) {
    m[1] = _mm256_set1_epi16(alpha as i16); // vpbroadcastw m1, alpham
    m[2] = _mm256_abs_epi16(m[1]); // pabsw m2, m1
    m[2] = _mm256_slli_epi16::<9>(m[2]); // psllw m2, 9
    loop {
        // .s32_loop:
        m[4] = y_p6(ac.cast()); // mova m4, [acq]
        m[5] = y_p6(ac.wrapping_byte_add(32).cast()); // mova m5, [acq+32]
        ipred_cfl(m, 4); // IPRED_CFL 4
        ipred_cfl(m, 5); // IPRED_CFL 5
        m[4] = _mm256_packus_epi16(m[4], m[5]); // packuswb m4, m5
        m[4] = _mm256_permute4x64_epi64::<Q3120_P6>(m[4]); // vpermq m4, m4, q3120
        y_st_p6(dst, m[4]); // mova [dstq], m4
        dst = at(dst, stride); // add dstq, strideq
        ac = ac.wrapping_byte_add(64); // add acq, 64
        h -= 1; // dec hd
        if h <= 0 {
            break; // jg .s32_loop
        }
    }
}

/// `jmp wq` through `ipred_cfl_splat_avx2_table` (s4, s8, s16, s32), indexed by `tzcnt(w)`.
#[inline(always)]
unsafe fn ipred_cfl_8bpc_splat_p6(
    m: &mut M,
    w: u32,
    dst: *mut u8,
    stride: isize,
    ac: *const i16,
    h: isize,
    alpha: c_int,
) {
    match w {
        2 => ipred_cfl_8bpc_s4(m, dst, stride, ac, h, alpha),
        3 => ipred_cfl_8bpc_s8(m, dst, stride, ac, h, alpha),
        4 => ipred_cfl_8bpc_s16(m, dst, stride, ac, h, alpha),
        _ => ipred_cfl_8bpc_s32(m, dst, stride, ac, h, alpha),
    }
}

/// `ipred_cfl_left_8bpc.h4` (the end of the chain `.h32` → `.h16` → `.h8` → `.h4`; `xm2` all ones, `xm3`
/// `0x8000 >> log2(n)`).
#[inline(always)]
unsafe fn ipred_cfl_left_8bpc_h4(m: &mut M) {
    m[0] = zx(_mm_madd_epi16(lo(m[0]), lo(m[2]))); // pmaddwd xm0, xm2
    m[0] = zx(_mm_mulhrs_epi16(lo(m[0]), lo(m[3]))); // pmulhrsw xm0, xm3
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
}

/// `ipred_cfl_left_8bpc.h8`, falling through into `.h4`.
#[inline(always)]
unsafe fn ipred_cfl_left_8bpc_h8(m: &mut M) {
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); // psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    ipred_cfl_left_8bpc_h4(m);
}

/// `ipred_cfl_left_8bpc.h16`, falling through into `.h8`.
#[inline(always)]
unsafe fn ipred_cfl_left_8bpc_h16(m: &mut M) {
    m[1] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[0]))); // punpckhqdq xm1, xm0, xm0
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    ipred_cfl_left_8bpc_h8(m);
}

/// `ipred_cfl_left_8bpc.h32`, falling through into `.h16`.
#[inline(always)]
unsafe fn ipred_cfl_left_8bpc_h32(m: &mut M) {
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    ipred_cfl_left_8bpc_h16(m);
}

/// `jmp r6` through `ipred_cfl_left_avx2_table` (h4, h8, h16, h32), then `jmp wq` into the splat table.
#[inline(always)]
unsafe fn ipred_cfl_left_8bpc_h_p6(m: &mut M, i: u32) {
    match i {
        2 => ipred_cfl_left_8bpc_h4(m),
        3 => ipred_cfl_left_8bpc_h8(m),
        4 => ipred_cfl_left_8bpc_h16(m),
        _ => ipred_cfl_left_8bpc_h32(m),
    }
}

/// `ipred_cfl_top_8bpc` (`cglobal ipred_cfl_top_8bpc, 3, 7, 6, dst, stride, tl, w, h, ac, alpha`).
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_top_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
) {
    let mut regs = regs();
    let m = &mut regs;
    // tzcnt wd, wm (`width` is 4, 8, 16 or 32: never zero)
    let w = (width as u32).trailing_zeros();
    let tl = a_p6(topleft, 1); // inc tlq
    m[0] = y_p6(tl); // movu m0, [tlq]
    let h = height as u32 as isize; // movifnidn hd, hm
    let r6 = 0x8000u32 >> w; // mov r6d, 0x8000 ; shrx r6d, r6d, wd
    m[3] = zx(_mm_cvtsi32_si128(r6 as i32)); // movd xm3, r6d
    m[2] = _mm256_set1_epi32(-1); // pcmpeqd m2, m2
    m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
    ipred_cfl_left_8bpc_h_p6(m, w); // movsxd r6, [t0+wq*4] … jmp r6
    ipred_cfl_8bpc_splat_p6(m, w, dst, stride, ac, h, alpha); // jmp wq
}

/// `ipred_cfl_left_8bpc` (`cglobal ipred_cfl_left_8bpc, 3, 7, 6, dst, stride, tl, w, h, ac, alpha`).
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_left_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
) {
    let mut regs = regs();
    let m = &mut regs;
    let h = height as u32 as isize; // mov hd, hm ; zero upper half
                                    // tzcnt r6d, hd (`height` is 4, 8, 16 or 32: never zero)
    let r6 = (height as u32).trailing_zeros();
    let tl = a_p6(topleft, -h); // sub tlq, hq
                                // tzcnt wd, wm (`width` is 4, 8, 16 or 32: never zero)
    let w = (width as u32).trailing_zeros();
    m[0] = y_p6(tl); // movu m0, [tlq]
    let t0 = 0x8000u32 >> r6; // mov t0d, 0x8000 ; shrx t0d, t0d, r6d
    m[3] = zx(_mm_cvtsi32_si128(t0 as i32)); // movd xm3, t0d
    m[2] = _mm256_set1_epi32(-1); // pcmpeqd m2, m2
    m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
    ipred_cfl_left_8bpc_h_p6(m, r6); // movsxd r6, [t0+r6*4] … jmp r6
    ipred_cfl_8bpc_splat_p6(m, w, dst, stride, ac, h, alpha); // jmp wq
}

/// `ipred_cfl_8bpc.h4`.
#[inline(always)]
unsafe fn ipred_cfl_8bpc_h4(m: &mut M, tl: *const u8) {
    m[0] = zx(_mm_cvtsi32_si128(rd32_p6(a_p6(tl, -4)))); // movd xm0, [tlq-4]
    m[0] = zx(_mm_maddubs_epi16(lo(m[0]), lo(m[3]))); // pmaddubsw xm0, xm3
}

/// `ipred_cfl_8bpc.h8`.
#[inline(always)]
unsafe fn ipred_cfl_8bpc_h8(m: &mut M, tl: *const u8) {
    m[0] = zx(movq_p6(a_p6(tl, -8))); // movq xm0, [tlq-8]
    m[0] = zx(_mm_maddubs_epi16(lo(m[0]), lo(m[3]))); // pmaddubsw xm0, xm3
}

/// `ipred_cfl_8bpc.h16`.
#[inline(always)]
unsafe fn ipred_cfl_8bpc_h16(m: &mut M, tl: *const u8) {
    m[0] = zx(x_p6(a_p6(tl, -16))); // mova xm0, [tlq-16]
    m[0] = zx(_mm_maddubs_epi16(lo(m[0]), lo(m[3]))); // pmaddubsw xm0, xm3
}

/// `ipred_cfl_8bpc.h32`.
#[inline(always)]
unsafe fn ipred_cfl_8bpc_h32(m: &mut M, tl: *const u8) {
    m[0] = y_p6(a_p6(tl, -32)); // mova m0, [tlq-32]
    m[0] = _mm256_maddubs_epi16(m[0], m[3]); // pmaddubsw m0, m3
}

/// `ipred_cfl_8bpc.w4` with `.w4_mul` and `.w4_end` (`xm4` `(w + h) >> 1`, `xm5` `tzcnt(w + h)`).
#[inline(always)]
unsafe fn ipred_cfl_8bpc_w4(m: &mut M, tl: *const u8, h: isize) {
    m[1] = zx(_mm_cvtsi32_si128(rd32_p6(a_p6(tl, 1)))); // movd xm1, [tlq+1]
    m[1] = zx(_mm_maddubs_epi16(lo(m[1]), lo(m[3]))); // pmaddubsw xm1, xm3
    m[0] = zx(_mm_sub_epi16(lo(m[0]), lo(m[4]))); // psubw xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[0] = zx(_mm_madd_epi16(lo(m[0]), lo(m[3]))); // pmaddwd xm0, xm3
    if h > 4 {
        // cmp hd, 4 ; jg .w4_mul
        // .w4_mul:
        m[1] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[0]))); // punpckhqdq xm1, xm0, xm0
        let r2 = (h * 2) as u32; // lea r2d, [hq*2]
        let r6 = 0x55563334u32; // mov r6d, 0x55563334
        m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
        let r6 = r6.wrapping_shr(r2); // shrx r6d, r6d, r2d (the count is taken modulo 32, as by `shrx`)
        m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); // psrlq xm1, xm0, 32
        m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); // movd xm1, r6d
        m[0] = zx(_mm_srli_epi16::<2>(lo(m[0]))); // psrlw xm0, 2
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); // pmulhuw xm0, xm1
    } else {
        m[0] = zx(_mm_srli_epi16::<3>(lo(m[0]))); // psrlw xm0, 3 ; jmp .w4_end
    }
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // .w4_end: vpbroadcastw m0, xm0
}

/// `ipred_cfl_8bpc.w8` with `.w8_end`.
#[inline(always)]
unsafe fn ipred_cfl_8bpc_w8(m: &mut M, tl: *const u8, h: isize) {
    m[1] = zx(movq_p6(a_p6(tl, 1))); // movq xm1, [tlq+1]
    m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1
    m[1] = zx(_mm_maddubs_epi16(lo(m[1]), lo(m[3]))); // pmaddubsw xm1, xm3
    m[0] = zx(_mm_sub_epi16(lo(m[0]), lo(m[4]))); // psubw xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); // paddw xm0, xm2
    m[2] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[0]))); // punpckhqdq xm2, xm0, xm0
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); // paddw xm0, xm2
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); // psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[0] = zx(_mm_madd_epi16(lo(m[0]), lo(m[3]))); // pmaddwd xm0, xm3
    m[0] = zx(_mm_srl_epi16(lo(m[0]), lo(m[5]))); // psrlw xm0, xm5
    if h != 8 {
        // cmp hd, 8 ; je .w8_end
        let mut r6 = 0x5556u32; // mov r6d, 0x5556
        let r2 = 0x3334u32; // mov r2d, 0x3334
        if h == 32 {
            r6 = r2; // cmp hd, 32 ; cmove r6d, r2d
        }
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); // movd xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); // pmulhuw xm0, xm1
    }
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // .w8_end: vpbroadcastw m0, xm0
}

/// `ipred_cfl_8bpc.w16` with `.w16_end`.
#[inline(always)]
unsafe fn ipred_cfl_8bpc_w16(m: &mut M, tl: *const u8, h: isize) {
    m[1] = zx(x_p6(a_p6(tl, 1))); // movu xm1, [tlq+1]
    m[2] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm2, m0, 1
    m[1] = zx(_mm_maddubs_epi16(lo(m[1]), lo(m[3]))); // pmaddubsw xm1, xm3
    m[0] = zx(_mm_sub_epi16(lo(m[0]), lo(m[4]))); // psubw xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[2]))); // paddw xm0, xm2
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[1] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[0]))); // punpckhqdq xm1, xm0, xm0
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); // psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[0] = zx(_mm_madd_epi16(lo(m[0]), lo(m[3]))); // pmaddwd xm0, xm3
    m[0] = zx(_mm_srl_epi16(lo(m[0]), lo(m[5]))); // psrlw xm0, xm5
    if h != 16 {
        // cmp hd, 16 ; je .w16_end
        let mut r6 = 0x5556u32; // mov r6d, 0x5556
        let r2 = 0x3334u32; // mov r2d, 0x3334
        if h & (8 | 32) == 0 {
            r6 = r2; // test hb, 8|32 ; cmovz r6d, r2d
        }
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); // movd xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); // pmulhuw xm0, xm1
    }
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // .w16_end: vpbroadcastw m0, xm0
}

/// `ipred_cfl_8bpc.w32` with `.w32_end`.
#[inline(always)]
unsafe fn ipred_cfl_8bpc_w32(m: &mut M, tl: *const u8, h: isize) {
    m[1] = y_p6(a_p6(tl, 1)); // movu m1, [tlq+1]
    m[1] = _mm256_maddubs_epi16(m[1], m[3]); // pmaddubsw m1, m3
    m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    m[0] = zx(_mm_sub_epi16(lo(m[0]), lo(m[4]))); // psubw xm0, xm4
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[1] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[0]))); // punpckhqdq xm1, xm0, xm0
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); // psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
    m[0] = zx(_mm_madd_epi16(lo(m[0]), lo(m[3]))); // pmaddwd xm0, xm3
    m[0] = zx(_mm_srl_epi16(lo(m[0]), lo(m[5]))); // psrlw xm0, xm5
    if h != 32 {
        // cmp hd, 32 ; je .w32_end
        let r2 = (h * 2) as u32; // lea r2d, [hq*2]
        let r6 = 0x33345556u32; // mov r6d, 0x33345556
        let r6 = r6.wrapping_shr(r2); // shrx r6d, r6d, r2d (the count is taken modulo 32, as by `shrx`)
        m[1] = zx(_mm_cvtsi32_si128(r6 as i32)); // movd xm1, r6d
        m[0] = zx(_mm_mulhi_epu16(lo(m[0]), lo(m[1]))); // pmulhuw xm0, xm1
    }
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // .w32_end: vpbroadcastw m0, xm0
}

/// `ipred_cfl_8bpc` (`cglobal ipred_cfl_8bpc, 3, 7, 6, dst, stride, tl, w, h, ac, alpha`).
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    topleft: *const u8,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
) {
    let mut regs = regs();
    let m = &mut regs;
    let tl = topleft;
    let h = height as u32 as isize; // movifnidn hd, hm
    let w = width as u32; // movifnidn wd, wm
                          // tzcnt r6d, hd (`height` is 4, 8, 16 or 32: never zero)
    let r6 = (height as u32).trailing_zeros();
    let t0 = w + h as u32; // lea t0d, [wq+hq]
    m[4] = zx(_mm_cvtsi32_si128(t0 as i32)); // movd xm4, t0d
    let t0 = t0.trailing_zeros(); // tzcnt t0d, t0d (w + h is at least 8: never zero)
    m[5] = zx(_mm_cvtsi32_si128(t0 as i32)); // movd xm5, t0d
    let w = w.trailing_zeros(); // tzcnt wd, wd (`width` is 4, 8, 16 or 32: never zero)
    m[3] = _mm256_set1_epi32(-1); // pcmpeqd m3, m3
    m[4] = zx(_mm_srli_epi16::<1>(lo(m[4]))); // psrlw xm4, 1
                                              // jmp r6 (ipred_cfl_avx2_table: h4, h8, h16, h32)
    match r6 {
        2 => ipred_cfl_8bpc_h4(m, tl),
        3 => ipred_cfl_8bpc_h8(m, tl),
        4 => ipred_cfl_8bpc_h16(m, tl),
        _ => ipred_cfl_8bpc_h32(m, tl),
    }
    // jmp wq (ipred_cfl_avx2_table: w4, w8, w16, w32), each falling through into its `.sN`
    match w {
        2 => {
            ipred_cfl_8bpc_w4(m, tl, h);
            ipred_cfl_8bpc_s4(m, dst, stride, ac, h, alpha)
        }
        3 => {
            ipred_cfl_8bpc_w8(m, tl, h);
            ipred_cfl_8bpc_s8(m, dst, stride, ac, h, alpha)
        }
        4 => {
            ipred_cfl_8bpc_w16(m, tl, h);
            ipred_cfl_8bpc_s16(m, dst, stride, ac, h, alpha)
        }
        _ => {
            ipred_cfl_8bpc_w32(m, tl, h);
            ipred_cfl_8bpc_s32(m, dst, stride, ac, h, alpha)
        }
    }
}

/// `ipred_cfl_128_8bpc` (`cglobal ipred_cfl_128_8bpc, 3, 7, 6, dst, stride, tl, w, h, ac, alpha`).
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_128_8bpc_avx2(
    dst: *mut u8,
    stride: isize,
    _topleft: *const u8,
    width: c_int,
    height: c_int,
    ac: *const i16,
    alpha: c_int,
) {
    let mut regs = regs();
    let m = &mut regs;
    // tzcnt wd, wm (`width` is 4, 8, 16 or 32: never zero)
    let w = (width as u32).trailing_zeros();
    let h = height as u32 as isize; // movifnidn hd, hm
    m[0] = vpbroadcastd_y_p6(ro(pw_128, 0)); // vpbroadcastd m0, [t0-ipred_cfl_splat_avx2_table+pw_128]
    ipred_cfl_8bpc_splat_p6(m, w, dst, stride, ac, h, alpha); // jmp wq
}

// ---- part 7: `ipred_avx2.asm` lines 4674–5277 (cfl_ac_420, cfl_ac_422, cfl_ac_444) ----
//
// The three `cglobal`s `ipred_cfl_ac_{420,422,444}_8bpc` are `dav1d_ipred_cfl_ac_{420,422,444}_8bpc_avx2`. The
// general-purpose registers are locals named as `DEFINE_ARGS` names them (`ac`, `y`, `stride`, `wpad`, `hpad`,
// `h`, `sz`, `ac_bak`; `w` and `h` are the arguments `cw` and `ch`). Each width path (`.w4`, `.w8`, `.w16`,
// `.w32`, with `.w8_wpad`, `.w16_wpad`, `.w32_wpad` as branches on `wpad`) is
// `ipred_cfl_ac_<ss>_8bpc_w<N>`; the labels reached from two places are functions of their own:
// `.w8_hpad` (`ipred_cfl_ac_<ss>_8bpc_w8_hpad`), `.w16_hpad_loop` / `.w16_hpad`, `.w32_hpad_loop` and
// `.calc_avg` with its `.sub_loop` (`ipred_cfl_ac_<ss>_8bpc_calc_avg`); 444's `.calc_avg_mul` is
// `ipred_cfl_ac_444_8bpc_calc_avg_mul`. Every width path ends in `.calc_avg` (444 w4/w8: `.calc_avg_mul`), so the
// entry point runs the width path and then the average. The jump tables (`ipred_cfl_ac_420_avx2_table`, `_422_`:
// `w16_pad1..3` indexed by `wpad`; `_444_`: `w32_pad1..3` indexed by `wpad / 2` and `w4..w32` indexed by
// `tzcnt(w)`) are `match`es; the `jmp iptrq` at the end of each padded row is the `match` inside the row loop.

/// x86inc's shuffle immediates `qABCD` = `A<<6|B<<4|C<<2|D`.
const Q1111_P7: i32 = 1 << 6 | 1 << 4 | 1 << 2 | 1;
const Q3232_P7: i32 = 3 << 6 | 2 << 4 | 3 << 2 | 2;
const Q3333_P7: i32 = 3 << 6 | 3 << 4 | 3 << 2 | 3;

/// `p + n * stride` bytes, wrapping (the luma pointer is advanced past its last row by the loops).
#[inline(always)]
fn ys_p7(p: *const u8, stride: isize, n: isize) -> *const u8 {
    p.wrapping_offset(stride.wrapping_mul(n))
}

/// `movd xm, [p]` (4 bytes; the rest of the register is cleared).
#[inline(always)]
unsafe fn movd_p7(p: *const u8) -> __m128i {
    _mm_cvtsi32_si128(p.cast::<i32>().read_unaligned())
}

/// `movq xm, [p]` (8 bytes; the rest of the register is cleared).
#[inline(always)]
unsafe fn movq_p7(p: *const u8) -> __m128i {
    _mm_loadl_epi64(p.cast())
}

/// `movhps xm, [p]`: the low quadword of `x` and the 8 bytes at `p` as the high one (an unaligned read where
/// `_mm_loadh_pd` reads through an aligned `*const f64`; the 8 bytes are only moved).
#[inline(always)]
unsafe fn movhps_p7(x: __m128i, p: *const u8) -> __m128i {
    let x = _mm_castsi128_pd(x);
    _mm_castpd_si128(_mm_setr_pd(_mm_cvtsd_f64(x), p.cast::<f64>().read_unaligned()))
}

/// `pinsrd xm, [p], I`.
#[inline(always)]
unsafe fn pinsrd_p7<const I: i32>(x: __m128i, p: *const u8) -> __m128i {
    _mm_insert_epi32::<I>(x, p.cast::<i32>().read_unaligned())
}

/// A 16-byte memory operand (`mova xm, [p]`, `vinserti128 m, [p], 1`, `pmovzxbw m, [p]`, `vbroadcasti128`).
#[inline(always)]
unsafe fn x_p7(p: *const u8) -> __m128i {
    _mm_loadu_si128(p.cast())
}

/// A 32-byte memory operand (`mova m, [p]`).
#[inline(always)]
unsafe fn y_p7(p: *const u8) -> __m256i {
    _mm256_loadu_si256(p.cast())
}

/// `vpbroadcastd m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastd_p7(p: *const u8) -> __m256i {
    _mm256_set1_epi32(p.cast::<i32>().read_unaligned())
}

/// `vpbroadcastq m, [p]`.
#[inline(always)]
unsafe fn vpbroadcastq_p7(p: *const u8) -> __m256i {
    _mm256_set1_epi64x(p.cast::<i64>().read_unaligned())
}

/// `vbroadcasti128 m, [p]`.
#[inline(always)]
unsafe fn vbroadcasti128_p7(p: *const u8) -> __m256i {
    _mm256_broadcastsi128_si256(x_p7(p))
}

/// `mova [p], xm`.
#[inline(always)]
unsafe fn st_x_p7(p: *mut i16, x: __m128i) {
    _mm_storeu_si128(p.cast(), x)
}

/// `mova [p], m`.
#[inline(always)]
unsafe fn st_y_p7(p: *mut i16, x: __m256i) {
    _mm256_storeu_si256(p.cast(), x)
}

/// `.sub_loop` of the three `.calc_avg`s: subtract the average in `m0` from the `sz` values at `ac_bak`.
#[inline(always)]
unsafe fn ipred_cfl_ac_8bpc_sub_loop_p7(m: &mut M, mut ac_bak: *mut i16, mut sz: i32) {
    loop {
        m[1] = _mm256_loadu_si256(ac_bak.cast()); // mova m1, [ac_bakq]
        m[1] = _mm256_sub_epi16(m[1], m[0]); // psubw m1, m0
        st_y_p7(ac_bak, m[1]); // mova [ac_bakq], m1
        ac_bak = at(ac_bak, 32); // add ac_bakq, 32
        sz -= 16; // sub szd, 16
        if sz <= 0 {
            break; // jg .sub_loop
        }
    }
}

/// The tail of the three `.calc_avg`s, from the horizontal sum of `xm0` + `xm1` (32-bit lanes) to the broadcast
/// average in `m0`: `(sum + (sz >> 1)) >> log2(sz)`.
#[inline(always)]
unsafe fn ipred_cfl_ac_8bpc_avg_tail_p7(m: &mut M, sz: i32) {
    // `sz` is cw * ch >= 16, never zero.
    let r1 = sz.trailing_zeros() as i32; // tzcnt r1d, szd
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1 (444: paddd xm0, xm4, xm1, done by the caller)
    m[2] = zx(_mm_cvtsi32_si128(r1)); // movd xm2, r1d
    m[3] = zx(_mm_cvtsi32_si128(sz)); // movd xm3, szd
    m[1] = zx(_mm_unpackhi_epi64(lo(m[0]), lo(m[0]))); // punpckhqdq xm1, xm0, xm0
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
    m[3] = zx(_mm_srai_epi32::<1>(lo(m[3]))); // psrad xm3, 1
    m[1] = zx(_mm_srli_epi64::<32>(lo(m[0]))); // psrlq xm1, xm0, 32
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[3]))); // paddd xm0, xm3
    m[0] = zx(_mm_add_epi32(lo(m[0]), lo(m[1]))); // paddd xm0, xm1
    m[0] = zx(_mm_sra_epi32(lo(m[0]), lo(m[2]))); // psrad xm0, xm2
    m[0] = _mm256_broadcastw_epi16(lo(m[0])); // vpbroadcastw m0, xm0
}

// ---------------------------------------------------------------------------------------------------------------
// cfl_ac_420
// ---------------------------------------------------------------------------------------------------------------

/// `ipred_cfl_ac_420_8bpc` `.w4` (w_pad is 0 at this width).
#[inline(always)]
unsafe fn ipred_cfl_ac_420_8bpc_w4(
    m: &mut M,
    mut ac: *mut i16,
    mut y: *const u8,
    stride: isize,
    mut hpad: i32,
    mut h: i32,
) {
    let stride3 = stride.wrapping_mul(3); // lea stride3q, [strideq*3]
    loop {
        // .w4_loop
        m[0] = zx(movq_p7(y)); // movq xm0, [yq]
        m[1] = zx(movq_p7(ys_p7(y, stride, 1))); // movq xm1, [yq+strideq]
        m[0] = zx(movhps_p7(lo(m[0]), ys_p7(y, stride, 2))); // movhps xm0, [yq+strideq*2]
        m[1] = zx(movhps_p7(lo(m[1]), y.wrapping_offset(stride3))); // movhps xm1, [yq+stride3q]
        m[0] = zx(_mm_maddubs_epi16(lo(m[0]), lo(m[2]))); // pmaddubsw xm0, xm2
        m[1] = zx(_mm_maddubs_epi16(lo(m[1]), lo(m[2]))); // pmaddubsw xm1, xm2
        m[0] = zx(_mm_add_epi16(lo(m[0]), lo(m[1]))); // paddw xm0, xm1
        st_x_p7(ac, lo(m[0])); // mova [acq], xm0
        m[4] = zx(_mm_add_epi16(lo(m[4]), lo(m[0]))); // paddw xm4, xm0
        y = ys_p7(y, stride, 4); // lea yq, [yq+strideq*4]
        ac = at(ac, 16); // add acq, 16
        h -= 2; // sub hd, 2
        if h <= 0 {
            break; // jg .w4_loop
        }
    }
    if hpad == 0 {
        return; // test hpadd, hpadd; jz .calc_avg
    }
    m[0] = _mm256_permute4x64_epi64::<Q1111_P7>(m[0]); // vpermq m0, m0, q1111
    loop {
        // .w4_hpad_loop
        st_y_p7(ac, m[0]); // mova [acq], m0
        m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
        ac = at(ac, 32); // add acq, 32
        hpad -= 4; // sub hpadd, 4
        if hpad <= 0 {
            break; // jg .w4_hpad_loop
        }
    }
    // jmp .calc_avg
}

/// `ipred_cfl_ac_420_8bpc` `.w8_hpad` (reached from `.w8_loop` and `.w8_wpad_loop`).
#[inline(always)]
unsafe fn ipred_cfl_ac_420_8bpc_w8_hpad(m: &mut M, mut ac: *mut i16, mut hpad: i32) {
    m[0] = _mm256_permute4x64_epi64::<Q3232_P7>(m[0]); // vpermq m0, m0, q3232
    loop {
        // .w8_hpad_loop
        st_y_p7(ac, m[0]); // mova [acq], m0
        m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
        ac = at(ac, 32); // add acq, 32
        hpad -= 2; // sub hpadd, 2
        if hpad <= 0 {
            break; // jg .w8_hpad_loop
        }
    }
    // jmp .calc_avg
}

/// `ipred_cfl_ac_420_8bpc` `.w8` with `.w8_wpad`.
#[inline(always)]
unsafe fn ipred_cfl_ac_420_8bpc_w8(
    m: &mut M,
    mut ac: *mut i16,
    mut y: *const u8,
    stride: isize,
    wpad: i32,
    hpad: i32,
    mut h: i32,
) {
    let stride3 = stride.wrapping_mul(3); // lea stride3q, [strideq*3]
    if wpad == 0 {
        // test wpadd, wpadd; jnz .w8_wpad
        loop {
            // .w8_loop
            m[0] = zx(x_p7(y)); // mova xm0, [yq]
            m[1] = zx(x_p7(ys_p7(y, stride, 1))); // mova xm1, [yq+strideq]
            m[0] = _mm256_inserti128_si256::<1>(m[0], x_p7(ys_p7(y, stride, 2))); // vinserti128 m0, [yq+strideq*2], 1
            m[1] = _mm256_inserti128_si256::<1>(m[1], x_p7(y.wrapping_offset(stride3))); // vinserti128 m1, [yq+stride3q], 1
            m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
            m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            st_y_p7(ac, m[0]); // mova [acq], m0
            m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
            y = ys_p7(y, stride, 4); // lea yq, [yq+strideq*4]
            ac = at(ac, 32); // add acq, 32
            h -= 2; // sub hd, 2
            if h <= 0 {
                break; // jg .w8_loop
            }
        }
        if hpad == 0 {
            return; // test hpadd, hpadd; jz .calc_avg
        }
        // jmp .w8_hpad
    } else {
        // .w8_wpad
        m[3] = vbroadcasti128_p7(ro(cfl_ac_w8_pad1_shuffle, 0)); // vbroadcasti128 m3, [cfl_ac_w8_pad1_shuffle]
        loop {
            // .w8_wpad_loop
            m[0] = zx(movq_p7(y)); // movq xm0, [yq]
            m[1] = zx(movq_p7(ys_p7(y, stride, 1))); // movq xm1, [yq+strideq]
            m[0] = _mm256_inserti128_si256::<1>(m[0], x_p7(ys_p7(y, stride, 2))); // vinserti128 m0, [yq+strideq*2], 1
            m[1] = _mm256_inserti128_si256::<1>(m[1], x_p7(y.wrapping_offset(stride3))); // vinserti128 m1, [yq+stride3q], 1
            m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
            m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[0] = _mm256_shuffle_epi8(m[0], m[3]); // pshufb m0, m3
            st_y_p7(ac, m[0]); // mova [acq], m0
            m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
            y = ys_p7(y, stride, 4); // lea yq, [yq+strideq*4]
            ac = at(ac, 32); // add acq, 32
            h -= 2; // sub hd, 2
            if h <= 0 {
                break; // jg .w8_wpad_loop
            }
        }
        if hpad == 0 {
            return; // test hpadd, hpadd; jz .calc_avg
        }
    }
    ipred_cfl_ac_420_8bpc_w8_hpad(m, ac, hpad);
}

/// `ipred_cfl_ac_420_8bpc` `.w16_hpad_loop` (reached from `.w16_loop` and `.w16_wpad_done`).
#[inline(always)]
unsafe fn ipred_cfl_ac_420_8bpc_w16_hpad_loop(m: &mut M, mut ac: *mut i16, mut hpad: i32) {
    loop {
        st_y_p7(ac, m[0]); // mova [acq], m0
        m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
        ac = at(ac, 32); // add acq, 32
        hpad -= 1; // dec hpadd
        if hpad <= 0 {
            break; // jg .w16_hpad_loop
        }
    }
    // fall-through to .calc_avg
}

/// `ipred_cfl_ac_420_8bpc` `.w16` with `.w16_wpad` and its jump table `w16_pad1`, `w16_pad2`, `w16_pad3`.
#[inline(always)]
unsafe fn ipred_cfl_ac_420_8bpc_w16(
    m: &mut M,
    mut ac: *mut i16,
    mut y: *const u8,
    stride: isize,
    wpad: i32,
    hpad: i32,
    mut h: i32,
) {
    if wpad == 0 {
        // test wpadd, wpadd; jnz .w16_wpad
        loop {
            // .w16_loop
            m[0] = y_p7(y); // mova m0, [yq]
            m[1] = y_p7(ys_p7(y, stride, 1)); // mova m1, [yq+strideq]
            m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
            m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            st_y_p7(ac, m[0]); // mova [acq], m0
            m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
            y = ys_p7(y, stride, 2); // lea yq, [yq+strideq*2]
            ac = at(ac, 32); // add acq, 32
            h -= 1; // dec hd
            if h <= 0 {
                break; // jg .w16_loop
            }
        }
        if hpad == 0 {
            return; // test hpadd, hpadd; jz .calc_avg
        }
        // jmp .w16_hpad_loop
    } else {
        // .w16_wpad
        let wpad = wpad << 2; // shl wpadd, 2
                              // mova m3, [iptrq+cfl_ac_w16_pad_shuffle-ipred_cfl_ac_420_avx2_table+wpadq*8-32]: the shuffle of
                              // `w_pad` 1..=3 (the contract: 0 <= w_pad and w_pad * 4 < cw = 16), 32 bytes at 0x380, 0x3a0 or 0x3c0,
                              // inside `RODATA`.
        m[3] = y_p7(ro(cfl_ac_w16_pad_shuffle, (wpad * 8 - 32) as isize));
        // movsxd wpadq, [iptrq+wpadq+4]; add iptrq, wpadq: the table entry `w_pad - 1`; `w_pad` is 1..=3 by
        // the contract (an index outside the table is undefined in the original; here it takes `w16_pad3`).
        let k = wpad >> 2;
        loop {
            // jmp iptrq
            match k {
                1 => {
                    // .w16_pad1
                    m[0] = y_p7(y); // mova m0, [yq]
                    m[1] = y_p7(ys_p7(y, stride, 1)); // mova m1, [yq+strideq]
                }
                2 => {
                    // .w16_pad2
                    m[0] = vbroadcasti128_p7(y); // vbroadcasti128 m0, [yq]
                    m[1] = vbroadcasti128_p7(ys_p7(y, stride, 1)); // vbroadcasti128 m1, [yq+strideq]
                }
                _ => {
                    // .w16_pad3
                    m[0] = vpbroadcastq_p7(y); // vpbroadcastq m0, [yq]
                    m[1] = vpbroadcastq_p7(ys_p7(y, stride, 1)); // vpbroadcastq m1, [yq+strideq]
                }
            }
            // .w16_wpad_end
            m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
            m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
            m[0] = _mm256_add_epi16(m[0], m[1]); // paddw m0, m1
            m[0] = _mm256_shuffle_epi8(m[0], m[3]); // pshufb m0, m3
            st_y_p7(ac, m[0]); // mova [acq], m0
            m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
            y = ys_p7(y, stride, 2); // lea yq, [yq+strideq*2]
            ac = at(ac, 32); // add acq, 32
            h -= 1; // dec hd
            if h == 0 {
                break; // jz .w16_wpad_done
            }
        }
        // .w16_wpad_done
        if hpad == 0 {
            return; // test hpadd, hpadd; jz .calc_avg
        }
    }
    ipred_cfl_ac_420_8bpc_w16_hpad_loop(m, ac, hpad);
}

/// `ipred_cfl_ac_420_8bpc` `.calc_avg` and `.sub_loop`.
#[inline(always)]
unsafe fn ipred_cfl_ac_420_8bpc_calc_avg(m: &mut M, ac_bak: *mut i16, sz: i32) {
    m[2] = vpbroadcastd_p7(ro(z2_y_shuf_h4, 24)); // vpbroadcastd m2, [pw_1]
    m[0] = _mm256_madd_epi16(m[4], m[2]); // pmaddwd m0, m4, m2
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    ipred_cfl_ac_8bpc_avg_tail_p7(m, sz);
    ipred_cfl_ac_8bpc_sub_loop_p7(m, ac_bak, sz);
}

/// `ipred_cfl_ac_420_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_ac_420_8bpc_avx2(
    ac: *mut i16,
    y: *const u8,
    stride: isize,
    wpad: c_int,
    hpad: c_int,
    w: c_int,
    h: c_int,
) {
    let mut m = regs();
    let mut hpad = hpad; // movifnidn hpadd, hpadm
    let mut h = h; // mov hd, hm
    let mut sz = w; // mov szd, wd
    let ac_bak = ac; // mov ac_bakq, acq
    sz = sz.wrapping_mul(h); // imul szd, hd
    hpad <<= 2; // shl hpadd, 2
    h -= hpad; // sub hd, hpadd
    m[2] = vpbroadcastd_p7(ro(ipred_h_shuf, 20)); // vpbroadcastd m2, [pb_2]
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    if w > 8 {
        // cmp wd, 8; jg .w16
        ipred_cfl_ac_420_8bpc_w16(&mut m, ac, y, stride, wpad, hpad, h);
    } else if w == 8 {
        // je .w8
        ipred_cfl_ac_420_8bpc_w8(&mut m, ac, y, stride, wpad, hpad, h);
    } else {
        ipred_cfl_ac_420_8bpc_w4(&mut m, ac, y, stride, hpad, h);
    }
    ipred_cfl_ac_420_8bpc_calc_avg(&mut m, ac_bak, sz);
}

// ---------------------------------------------------------------------------------------------------------------
// cfl_ac_422
// ---------------------------------------------------------------------------------------------------------------

/// `ipred_cfl_ac_422_8bpc` `.w4` (w_pad is 0 at this width).
#[inline(always)]
unsafe fn ipred_cfl_ac_422_8bpc_w4(
    m: &mut M,
    mut ac: *mut i16,
    mut y: *const u8,
    stride: isize,
    mut hpad: i32,
    mut h: i32,
) {
    let stride3 = stride.wrapping_mul(3); // lea stride3q, [strideq*3]
    loop {
        // .w4_loop
        m[1] = zx(movq_p7(y)); // movq xm1, [yq]
        m[1] = zx(movhps_p7(lo(m[1]), ys_p7(y, stride, 1))); // movhps xm1, [yq+strideq]
        m[0] = zx(movq_p7(ys_p7(y, stride, 2))); // movq xm0, [yq+strideq*2]
        m[0] = zx(movhps_p7(lo(m[0]), y.wrapping_offset(stride3))); // movhps xm0, [yq+stride3q]
        m[0] = zx(_mm_maddubs_epi16(lo(m[0]), lo(m[2]))); // pmaddubsw xm0, xm2
        m[1] = zx(_mm_maddubs_epi16(lo(m[1]), lo(m[2]))); // pmaddubsw xm1, xm2
        st_x_p7(ac, lo(m[1])); // mova [acq], xm1
        st_x_p7(at(ac, 16), lo(m[0])); // mova [acq+16], xm0
        m[4] = zx(_mm_add_epi16(lo(m[4]), lo(m[0]))); // paddw xm4, xm0
        m[5] = zx(_mm_add_epi16(lo(m[5]), lo(m[1]))); // paddw xm5, xm1
        y = ys_p7(y, stride, 4); // lea yq, [yq+strideq*4]
        ac = at(ac, 32); // add acq, 32
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w4_loop
        }
    }
    if hpad == 0 {
        return; // test hpadd, hpadd; jz .calc_avg
    }
    m[0] = _mm256_permute4x64_epi64::<Q1111_P7>(m[0]); // vpermq m0, m0, q1111
    loop {
        // .w4_hpad_loop
        st_y_p7(ac, m[0]); // mova [acq], m0
        m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
        ac = at(ac, 32); // add acq, 32
        hpad -= 4; // sub hpadd, 4
        if hpad <= 0 {
            break; // jg .w4_hpad_loop
        }
    }
    // jmp .calc_avg
}

/// `ipred_cfl_ac_422_8bpc` `.w8_hpad` (reached from `.w8_loop` and `.w8_wpad_loop`).
#[inline(always)]
unsafe fn ipred_cfl_ac_422_8bpc_w8_hpad(m: &mut M, mut ac: *mut i16, mut hpad: i32) {
    m[0] = _mm256_permute4x64_epi64::<Q3232_P7>(m[0]); // vpermq m0, m0, q3232
    loop {
        // .w8_hpad_loop
        st_y_p7(ac, m[0]); // mova [acq], m0
        m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
        ac = at(ac, 32); // add acq, 32
        hpad -= 2; // sub hpadd, 2
        if hpad <= 0 {
            break; // jg .w8_hpad_loop
        }
    }
    // jmp .calc_avg
}

/// `ipred_cfl_ac_422_8bpc` `.w8` with `.w8_wpad`.
#[inline(always)]
unsafe fn ipred_cfl_ac_422_8bpc_w8(
    m: &mut M,
    mut ac: *mut i16,
    mut y: *const u8,
    stride: isize,
    wpad: i32,
    hpad: i32,
    mut h: i32,
) {
    let stride3 = stride.wrapping_mul(3); // lea stride3q, [strideq*3]
    if wpad == 0 {
        // test wpadd, wpadd; jnz .w8_wpad
        loop {
            // .w8_loop
            m[1] = zx(x_p7(y)); // mova xm1, [yq]
            m[1] = _mm256_inserti128_si256::<1>(m[1], x_p7(ys_p7(y, stride, 1))); // vinserti128 m1, [yq+strideq], 1
            m[0] = zx(x_p7(ys_p7(y, stride, 2))); // mova xm0, [yq+strideq*2]
            m[0] = _mm256_inserti128_si256::<1>(m[0], x_p7(y.wrapping_offset(stride3))); // vinserti128 m0, [yq+stride3q], 1
            m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
            m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
            st_y_p7(ac, m[1]); // mova [acq], m1
            st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
            m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
            m[5] = _mm256_add_epi16(m[5], m[1]); // paddw m5, m1
            y = ys_p7(y, stride, 4); // lea yq, [yq+strideq*4]
            ac = at(ac, 64); // add acq, 64
            h -= 4; // sub hd, 4
            if h <= 0 {
                break; // jg .w8_loop
            }
        }
        if hpad == 0 {
            return; // test hpadd, hpadd; jz .calc_avg
        }
        // jmp .w8_hpad
    } else {
        // .w8_wpad
        m[3] = vbroadcasti128_p7(ro(cfl_ac_w8_pad1_shuffle, 0)); // vbroadcasti128 m3, [cfl_ac_w8_pad1_shuffle]
        loop {
            // .w8_wpad_loop
            m[1] = zx(movq_p7(y)); // movq xm1, [yq]
            m[1] = _mm256_inserti128_si256::<1>(m[1], x_p7(ys_p7(y, stride, 1))); // vinserti128 m1, [yq+strideq], 1
            m[0] = zx(movq_p7(ys_p7(y, stride, 2))); // movq xm0, [yq+strideq*2]
            m[0] = _mm256_inserti128_si256::<1>(m[0], x_p7(y.wrapping_offset(stride3))); // vinserti128 m0, [yq+stride3q], 1
            m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
            m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
            m[0] = _mm256_shuffle_epi8(m[0], m[3]); // pshufb m0, m3
            m[1] = _mm256_shuffle_epi8(m[1], m[3]); // pshufb m1, m3
            st_y_p7(ac, m[1]); // mova [acq], m1
            st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
            m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
            m[5] = _mm256_add_epi16(m[5], m[1]); // paddw m5, m1
            y = ys_p7(y, stride, 4); // lea yq, [yq+strideq*4]
            ac = at(ac, 64); // add acq, 64
            h -= 4; // sub hd, 4
            if h <= 0 {
                break; // jg .w8_wpad_loop
            }
        }
        if hpad == 0 {
            return; // test hpadd, hpadd; jz .calc_avg
        }
    }
    ipred_cfl_ac_422_8bpc_w8_hpad(m, ac, hpad);
}

/// `ipred_cfl_ac_422_8bpc` `.w16_hpad_loop` (reached from `.w16_loop` and `.w16_wpad_done`).
#[inline(always)]
unsafe fn ipred_cfl_ac_422_8bpc_w16_hpad_loop(m: &mut M, mut ac: *mut i16, mut hpad: i32) {
    loop {
        st_y_p7(ac, m[0]); // mova [acq], m0
        st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
        m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
        m[5] = _mm256_add_epi16(m[5], m[0]); // paddw m5, m0
        ac = at(ac, 64); // add acq, 64
        hpad -= 2; // sub hpadd, 2
        if hpad <= 0 {
            break; // jg .w16_hpad_loop
        }
    }
    // fall-through to .calc_avg
}

/// `ipred_cfl_ac_422_8bpc` `.w16` with `.w16_wpad` and its jump table `w16_pad1`, `w16_pad2`, `w16_pad3`.
#[inline(always)]
unsafe fn ipred_cfl_ac_422_8bpc_w16(
    m: &mut M,
    mut ac: *mut i16,
    mut y: *const u8,
    stride: isize,
    wpad: i32,
    hpad: i32,
    mut h: i32,
) {
    if wpad == 0 {
        // test wpadd, wpadd; jnz .w16_wpad
        loop {
            // .w16_loop
            m[1] = y_p7(y); // mova m1, [yq]
            m[0] = y_p7(ys_p7(y, stride, 1)); // mova m0, [yq+strideq]
            m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
            m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
            st_y_p7(ac, m[1]); // mova [acq], m1
            st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
            m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
            m[5] = _mm256_add_epi16(m[5], m[1]); // paddw m5, m1
            y = ys_p7(y, stride, 2); // lea yq, [yq+strideq*2]
            ac = at(ac, 64); // add acq, 64
            h -= 2; // sub hd, 2
            if h <= 0 {
                break; // jg .w16_loop
            }
        }
        if hpad == 0 {
            return; // test hpadd, hpadd; jz .calc_avg
        }
        // jmp .w16_hpad_loop
    } else {
        // .w16_wpad
        let wpad = wpad << 2; // shl wpadd, 2
                              // mova m3, [iptrq+cfl_ac_w16_pad_shuffle-ipred_cfl_ac_422_avx2_table+wpadq*8-32]: the shuffle of
                              // `w_pad` 1..=3 (the contract: 0 <= w_pad and w_pad * 4 < cw = 16), 32 bytes at 0x380, 0x3a0 or 0x3c0,
                              // inside `RODATA`.
        m[3] = y_p7(ro(cfl_ac_w16_pad_shuffle, (wpad * 8 - 32) as isize));
        // movsxd wpadq, [iptrq+wpadq+4]; add iptrq, wpadq: the table entry `w_pad - 1`; `w_pad` is 1..=3 by
        // the contract (an index outside the table is undefined in the original; here it takes `w16_pad3`).
        let k = wpad >> 2;
        loop {
            // jmp iptrq
            match k {
                1 => {
                    // .w16_pad1
                    m[1] = y_p7(y); // mova m1, [yq]
                    m[0] = y_p7(ys_p7(y, stride, 1)); // mova m0, [yq+strideq]
                }
                2 => {
                    // .w16_pad2
                    m[1] = vbroadcasti128_p7(y); // vbroadcasti128 m1, [yq]
                    m[0] = vbroadcasti128_p7(ys_p7(y, stride, 1)); // vbroadcasti128 m0, [yq+strideq]
                }
                _ => {
                    // .w16_pad3
                    m[1] = vpbroadcastq_p7(y); // vpbroadcastq m1, [yq]
                    m[0] = vpbroadcastq_p7(ys_p7(y, stride, 1)); // vpbroadcastq m0, [yq+strideq]
                }
            }
            // .w16_wpad_end
            m[0] = _mm256_maddubs_epi16(m[0], m[2]); // pmaddubsw m0, m2
            m[1] = _mm256_maddubs_epi16(m[1], m[2]); // pmaddubsw m1, m2
            m[0] = _mm256_shuffle_epi8(m[0], m[3]); // pshufb m0, m3
            m[1] = _mm256_shuffle_epi8(m[1], m[3]); // pshufb m1, m3
            st_y_p7(ac, m[1]); // mova [acq], m1
            st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
            m[4] = _mm256_add_epi16(m[4], m[0]); // paddw m4, m0
            m[5] = _mm256_add_epi16(m[5], m[1]); // paddw m5, m1
            y = ys_p7(y, stride, 2); // lea yq, [yq+strideq*2]
            ac = at(ac, 64); // add acq, 64
            h -= 2; // sub hd, 2
            if h == 0 {
                break; // jz .w16_wpad_done
            }
        }
        // .w16_wpad_done
        if hpad == 0 {
            return; // test hpadd, hpadd; jz .calc_avg
        }
    }
    ipred_cfl_ac_422_8bpc_w16_hpad_loop(m, ac, hpad);
}

/// `ipred_cfl_ac_422_8bpc` `.calc_avg` and `.sub_loop`.
#[inline(always)]
unsafe fn ipred_cfl_ac_422_8bpc_calc_avg(m: &mut M, ac_bak: *mut i16, sz: i32) {
    m[2] = vpbroadcastd_p7(ro(z2_y_shuf_h4, 24)); // vpbroadcastd m2, [pw_1]
    m[5] = _mm256_madd_epi16(m[5], m[2]); // pmaddwd m5, m5, m2
    m[0] = _mm256_madd_epi16(m[4], m[2]); // pmaddwd m0, m4, m2
    m[0] = _mm256_add_epi32(m[0], m[5]); // paddd m0, m5
    m[1] = zx(_mm256_extracti128_si256::<1>(m[0])); // vextracti128 xm1, m0, 1
    ipred_cfl_ac_8bpc_avg_tail_p7(m, sz);
    ipred_cfl_ac_8bpc_sub_loop_p7(m, ac_bak, sz);
}

/// `ipred_cfl_ac_422_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_ac_422_8bpc_avx2(
    ac: *mut i16,
    y: *const u8,
    stride: isize,
    wpad: c_int,
    hpad: c_int,
    w: c_int,
    h: c_int,
) {
    let mut m = regs();
    let mut hpad = hpad; // movifnidn hpadd, hpadm
    let mut h = h; // mov hd, hm
    let mut sz = w; // mov szd, wd
    let ac_bak = ac; // mov ac_bakq, acq
    sz = sz.wrapping_mul(h); // imul szd, hd
    hpad <<= 2; // shl hpadd, 2
    h -= hpad; // sub hd, hpadd
    m[2] = vpbroadcastd_p7(ro(ipred_h_shuf, 24)); // vpbroadcastd m2, [pb_4]
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    m[5] = _mm256_setzero_si256(); // pxor m5, m5
    if w > 8 {
        // cmp wd, 8; jg .w16
        ipred_cfl_ac_422_8bpc_w16(&mut m, ac, y, stride, wpad, hpad, h);
    } else if w == 8 {
        // je .w8
        ipred_cfl_ac_422_8bpc_w8(&mut m, ac, y, stride, wpad, hpad, h);
    } else {
        ipred_cfl_ac_422_8bpc_w4(&mut m, ac, y, stride, hpad, h);
    }
    ipred_cfl_ac_422_8bpc_calc_avg(&mut m, ac_bak, sz);
}

// ---------------------------------------------------------------------------------------------------------------
// cfl_ac_444
// ---------------------------------------------------------------------------------------------------------------

/// `ipred_cfl_ac_444_8bpc` `.w4` (w_pad is 0 at this width).
#[inline(always)]
unsafe fn ipred_cfl_ac_444_8bpc_w4(
    m: &mut M,
    mut ac: *mut i16,
    mut y: *const u8,
    stride: isize,
    mut hpad: i32,
    mut h: i32,
) {
    let stride3 = stride.wrapping_mul(3); // lea stride3q, [strideq*3]
    m[2] = zx(_mm_setzero_si128()); // pxor xm2, xm2
    loop {
        // .w4_loop
        m[1] = zx(movd_p7(y)); // movd xm1, [yq]
        m[0] = zx(movd_p7(ys_p7(y, stride, 2))); // movd xm0, [yq+strideq*2]
        m[1] = zx(pinsrd_p7::<1>(lo(m[1]), ys_p7(y, stride, 1))); // pinsrd xm1, [yq+strideq], 1
        m[0] = zx(pinsrd_p7::<1>(lo(m[0]), y.wrapping_offset(stride3))); // pinsrd xm0, [yq+stride3q], 1
        m[1] = zx(_mm_unpacklo_epi8(lo(m[1]), lo(m[2]))); // punpcklbw xm1, xm2
        m[0] = zx(_mm_unpacklo_epi8(lo(m[0]), lo(m[2]))); // punpcklbw xm0, xm2
        m[1] = zx(_mm_slli_epi16::<3>(lo(m[1]))); // psllw xm1, 3
        m[0] = zx(_mm_slli_epi16::<3>(lo(m[0]))); // psllw xm0, 3
        st_x_p7(ac, lo(m[1])); // mova [acq], xm1
        st_x_p7(at(ac, 16), lo(m[0])); // mova [acq+16], xm0
        m[1] = zx(_mm_add_epi16(lo(m[1]), lo(m[0]))); // paddw xm1, xm0
        m[4] = zx(_mm_add_epi16(lo(m[4]), lo(m[1]))); // paddw xm4, xm1
        y = ys_p7(y, stride, 4); // lea yq, [yq+strideq*4]
        ac = at(ac, 32); // add acq, 32
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w4_loop
        }
    }
    if hpad == 0 {
        return; // test hpadd, hpadd; jz .calc_avg_mul
    }
    m[0] = zx(_mm_shuffle_epi32::<Q3232_P7>(lo(m[0]))); // pshufd xm0, xm0, q3232
    m[1] = zx(_mm_add_epi16(lo(m[0]), lo(m[0]))); // paddw xm1, xm0, xm0
    loop {
        // .w4_hpad_loop
        st_x_p7(ac, lo(m[0])); // mova [acq], xm0
        st_x_p7(at(ac, 16), lo(m[0])); // mova [acq+16], xm0
        m[4] = zx(_mm_add_epi16(lo(m[4]), lo(m[1]))); // paddw xm4, xm1
        ac = at(ac, 32); // add acq, 32
        hpad -= 4; // sub hpadd, 4
        if hpad <= 0 {
            break; // jg .w4_hpad_loop
        }
    }
    // jmp .calc_avg_mul
}

/// `ipred_cfl_ac_444_8bpc` `.w8` (no `w_pad` path: w_pad is 0 at this width in the original's use).
#[inline(always)]
unsafe fn ipred_cfl_ac_444_8bpc_w8(
    m: &mut M,
    mut ac: *mut i16,
    mut y: *const u8,
    stride: isize,
    mut hpad: i32,
    mut h: i32,
) {
    let stride3 = stride.wrapping_mul(3); // lea stride3q, [strideq*3]
    m[2] = _mm256_setzero_si256(); // pxor m2, m2
    loop {
        // .w8_loop
        m[1] = zx(movq_p7(y)); // movq xm1, [yq]
        m[0] = zx(movq_p7(ys_p7(y, stride, 2))); // movq xm0, [yq+strideq*2]
        m[1] = _mm256_inserti128_si256::<1>(m[1], x_p7(ys_p7(y, stride, 1))); // vinserti128 m1, [yq+strideq], 1
        m[0] = _mm256_inserti128_si256::<1>(m[0], x_p7(y.wrapping_offset(stride3))); // vinserti128 m0, [yq+stride3q], 1
        m[1] = _mm256_unpacklo_epi8(m[1], m[2]); // punpcklbw m1, m2
        m[0] = _mm256_unpacklo_epi8(m[0], m[2]); // punpcklbw m0, m2
        m[1] = _mm256_slli_epi16::<3>(m[1]); // psllw m1, 3
        m[0] = _mm256_slli_epi16::<3>(m[0]); // psllw m0, 3
        st_y_p7(ac, m[1]); // mova [acq], m1
        st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
        m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
        m[4] = _mm256_add_epi16(m[4], m[1]); // paddw m4, m1
        y = ys_p7(y, stride, 4); // lea yq, [yq+strideq*4]
        ac = at(ac, 64); // add acq, 64
        h -= 4; // sub hd, 4
        if h <= 0 {
            break; // jg .w8_loop
        }
    }
    if hpad == 0 {
        return; // test hpadd, hpadd; jz .calc_avg_mul
    }
    m[0] = _mm256_permute4x64_epi64::<Q3232_P7>(m[0]); // vpermq m0, m0, q3232
    m[1] = _mm256_add_epi16(m[0], m[0]); // paddw m1, m0, m0
    loop {
        // .w8_hpad_loop
        st_y_p7(ac, m[0]); // mova [acq], m0
        st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
        m[4] = _mm256_add_epi16(m[4], m[1]); // paddw m4, m1
        ac = at(ac, 64); // add acq, 64
        hpad -= 4; // sub hpadd, 4
        if hpad <= 0 {
            break; // jg .w8_hpad_loop
        }
    }
    // jmp .calc_avg_mul
}

/// `ipred_cfl_ac_444_8bpc` `.w16_hpad` (reached from `.w16_loop` and `.w16_wpad_loop`).
#[inline(always)]
unsafe fn ipred_cfl_ac_444_8bpc_w16_hpad(m: &mut M, mut ac: *mut i16, mut hpad: i32) {
    m[1] = _mm256_add_epi16(m[0], m[0]); // paddw m1, m0, m0
    m[1] = _mm256_madd_epi16(m[1], m[5]); // pmaddwd m1, m5
    loop {
        // .w16_hpad_loop
        st_y_p7(ac, m[0]); // mova [acq], m0
        st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
        m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1
        ac = at(ac, 64); // add acq, 64
        hpad -= 2; // sub hpadd, 2
        if hpad <= 0 {
            break; // jg .w16_hpad_loop
        }
    }
    // jmp .calc_avg
}

/// `ipred_cfl_ac_444_8bpc` `.w16` with `.w16_wpad` (`w_pad` 2, the only one the original handles at this width).
#[inline(always)]
unsafe fn ipred_cfl_ac_444_8bpc_w16(
    m: &mut M,
    mut ac: *mut i16,
    mut y: *const u8,
    stride: isize,
    wpad: i32,
    hpad: i32,
    mut h: i32,
) {
    if wpad == 0 {
        // test wpadd, wpadd; jnz .w16_wpad
        loop {
            // .w16_loop
            m[1] = _mm256_cvtepu8_epi16(x_p7(y)); // pmovzxbw m1, [yq]
            m[0] = _mm256_cvtepu8_epi16(x_p7(ys_p7(y, stride, 1))); // pmovzxbw m0, [yq+strideq]
            m[1] = _mm256_slli_epi16::<3>(m[1]); // psllw m1, 3
            m[0] = _mm256_slli_epi16::<3>(m[0]); // psllw m0, 3
            st_y_p7(ac, m[1]); // mova [acq], m1
            st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
            m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
            m[1] = _mm256_madd_epi16(m[1], m[5]); // pmaddwd m1, m5
            m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1
            y = ys_p7(y, stride, 2); // lea yq, [yq+strideq*2]
            ac = at(ac, 64); // add acq, 64
            h -= 2; // sub hd, 2
            if h <= 0 {
                break; // jg .w16_loop
            }
        }
        if hpad == 0 {
            return; // test hpadd, hpadd; jz .calc_avg
        }
        // jmp .w16_hpad
    } else {
        // .w16_wpad
        m[3] = y_p7(ro(cfl_ac_444_w16_pad1_shuffle, 0)); // mova m3, [cfl_ac_444_w16_pad1_shuffle]
        loop {
            // .w16_wpad_loop
            m[1] = vpbroadcastq_p7(y); // vpbroadcastq m1, [yq]
            m[0] = vpbroadcastq_p7(ys_p7(y, stride, 1)); // vpbroadcastq m0, [yq+strideq]
            m[1] = _mm256_shuffle_epi8(m[1], m[3]); // pshufb m1, m3
            m[0] = _mm256_shuffle_epi8(m[0], m[3]); // pshufb m0, m3
            m[1] = _mm256_slli_epi16::<3>(m[1]); // psllw m1, 3
            m[0] = _mm256_slli_epi16::<3>(m[0]); // psllw m0, 3
            st_y_p7(ac, m[1]); // mova [acq], m1
            st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
            m[1] = _mm256_add_epi16(m[1], m[0]); // paddw m1, m0
            m[1] = _mm256_madd_epi16(m[1], m[5]); // pmaddwd m1, m5
            m[4] = _mm256_add_epi32(m[4], m[1]); // paddd m4, m1
            y = ys_p7(y, stride, 2); // lea yq, [yq+strideq*2]
            ac = at(ac, 64); // add acq, 64
            h -= 2; // sub hd, 2
            if h <= 0 {
                break; // jg .w16_wpad_loop
            }
        }
        if hpad == 0 {
            return; // test hpadd, hpadd; jz .calc_avg
        }
    }
    ipred_cfl_ac_444_8bpc_w16_hpad(m, ac, hpad);
}

/// `ipred_cfl_ac_444_8bpc` `.w32_hpad_loop` (reached from `.w32_loop` and `.w32_wpad_done`).
#[inline(always)]
unsafe fn ipred_cfl_ac_444_8bpc_w32_hpad_loop(m: &mut M, mut ac: *mut i16, mut hpad: i32) {
    loop {
        st_y_p7(ac, m[1]); // mova [acq], m1
        st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
        m[4] = _mm256_add_epi32(m[4], m[2]); // paddd m4, m2
        ac = at(ac, 64); // add acq, 64
        hpad -= 1; // dec hpadd
        if hpad <= 0 {
            break; // jg .w32_hpad_loop
        }
    }
    // jmp .calc_avg
}

/// `ipred_cfl_ac_444_8bpc` `.w32` with `.w32_wpad` and its jump table `w32_pad1`, `w32_pad2`, `w32_pad3`.
#[inline(always)]
unsafe fn ipred_cfl_ac_444_8bpc_w32(
    m: &mut M,
    mut ac: *mut i16,
    mut y: *const u8,
    stride: isize,
    wpad: i32,
    hpad: i32,
    mut h: i32,
) {
    if wpad == 0 {
        // test wpadd, wpadd; jnz .w32_wpad
        loop {
            // .w32_loop
            m[1] = _mm256_cvtepu8_epi16(x_p7(y)); // pmovzxbw m1, [yq]
            m[0] = _mm256_cvtepu8_epi16(x_p7(y.wrapping_offset(16))); // pmovzxbw m0, [yq+16]
            m[1] = _mm256_slli_epi16::<3>(m[1]); // psllw m1, 3
            m[0] = _mm256_slli_epi16::<3>(m[0]); // psllw m0, 3
            st_y_p7(ac, m[1]); // mova [acq], m1
            st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
            m[2] = _mm256_add_epi16(m[1], m[0]); // paddw m2, m1, m0
            m[2] = _mm256_madd_epi16(m[2], m[5]); // pmaddwd m2, m5
            m[4] = _mm256_add_epi32(m[4], m[2]); // paddd m4, m2
            y = y.wrapping_offset(stride); // add yq, strideq
            ac = at(ac, 64); // add acq, 64
            h -= 1; // dec hd
            if h <= 0 {
                break; // jg .w32_loop
            }
        }
        if hpad == 0 {
            return; // test hpadd, hpadd; jz .calc_avg
        }
        // jmp .w32_hpad_loop
    } else {
        // .w32_wpad
        let wpad = wpad + wpad; // add wpadd, wpadd
        m[3] = y_p7(ro(cfl_ac_444_w16_pad1_shuffle, 0)); // mova m3, [iptrq+cfl_ac_444_w16_pad1_shuffle-…table]
                                                         // movsxd wpadq, [iptrq+wpadq+4]; add iptrq, wpadq: the table entry `w_pad / 2 - 1`; `w_pad` is 2, 4 or 6
                                                         // here (the original's use: the test's and the decoder's 4:4:4 `w_pad` is even, and w_pad * 4 < cw = 32;
                                                         // any other value indexes outside the table or between its entries, undefined in the original; here it
                                                         // takes `w32_pad3`).
        let k = wpad;
        loop {
            // jmp iptrq
            match k {
                4 => {
                    // .w32_pad1
                    m[1] = _mm256_cvtepu8_epi16(x_p7(y)); // pmovzxbw m1, [yq]
                    m[0] = vpbroadcastq_p7(y.wrapping_offset(16)); // vpbroadcastq m0, [yq+16]
                    m[0] = _mm256_shuffle_epi8(m[0], m[3]); // pshufb m0, m3
                }
                8 => {
                    // .w32_pad2
                    m[1] = _mm256_cvtepu8_epi16(x_p7(y)); // pmovzxbw m1, [yq]
                    m[0] = _mm256_shufflehi_epi16::<Q3333_P7>(m[1]); // pshufhw m0, m1, q3333
                    m[0] = _mm256_permute4x64_epi64::<Q3333_P7>(m[0]); // vpermq m0, m0, q3333
                }
                _ => {
                    // .w32_pad3
                    m[1] = vpbroadcastq_p7(y); // vpbroadcastq m1, [yq]
                    m[1] = _mm256_shuffle_epi8(m[1], m[3]); // pshufb m1, m3
                    m[0] = _mm256_permute4x64_epi64::<Q3232_P7>(m[1]); // vpermq m0, m1, q3232
                }
            }
            // .w32_wpad_end
            m[1] = _mm256_slli_epi16::<3>(m[1]); // psllw m1, 3
            m[0] = _mm256_slli_epi16::<3>(m[0]); // psllw m0, 3
            st_y_p7(ac, m[1]); // mova [acq], m1
            st_y_p7(at(ac, 32), m[0]); // mova [acq+32], m0
            m[2] = _mm256_add_epi16(m[1], m[0]); // paddw m2, m1, m0
            m[2] = _mm256_madd_epi16(m[2], m[5]); // pmaddwd m2, m5
            m[4] = _mm256_add_epi32(m[4], m[2]); // paddd m4, m2
            y = y.wrapping_offset(stride); // add yq, strideq
            ac = at(ac, 64); // add acq, 64
            h -= 1; // dec hd
            if h == 0 {
                break; // jz .w32_wpad_done
            }
        }
        // .w32_wpad_done
        if hpad == 0 {
            return; // test hpadd, hpadd; jz .calc_avg
        }
    }
    ipred_cfl_ac_444_8bpc_w32_hpad_loop(m, ac, hpad);
}

/// `ipred_cfl_ac_444_8bpc` `.calc_avg_mul` (reached from `.w4` and `.w8`, falls through to `.calc_avg`).
#[inline(always)]
unsafe fn ipred_cfl_ac_444_8bpc_calc_avg_mul(m: &mut M) {
    m[4] = _mm256_madd_epi16(m[4], m[5]); // pmaddwd m4, m5
}

/// `ipred_cfl_ac_444_8bpc` `.calc_avg` and `.sub_loop`.
#[inline(always)]
unsafe fn ipred_cfl_ac_444_8bpc_calc_avg(m: &mut M, ac_bak: *mut i16, sz: i32) {
    m[1] = zx(_mm256_extracti128_si256::<1>(m[4])); // vextracti128 xm1, m4, 1
                                                    // tzcnt r1d, szd and paddd xm0, xm4, xm1 (in the tail, as `xm0` + `xm1` after this copy)
    m[0] = m[4];
    ipred_cfl_ac_8bpc_avg_tail_p7(m, sz);
    ipred_cfl_ac_8bpc_sub_loop_p7(m, ac_bak, sz);
}

/// `ipred_cfl_ac_444_8bpc`.
#[target_feature(enable = "avx2")]
pub unsafe extern "C" fn dav1d_ipred_cfl_ac_444_8bpc_avx2(
    ac: *mut i16,
    y: *const u8,
    stride: isize,
    wpad: c_int,
    hpad: c_int,
    w: c_int,
    h: c_int,
) {
    let mut m = regs();
    let mut hpad = hpad; // movifnidn hpadd, hpadm
    let mut h = h; // mov hd, hm
    let mut sz = w; // mov szd, wd
    sz = sz.wrapping_mul(h); // imul szd, hd
    hpad <<= 2; // shl hpadd, 2
    h -= hpad; // sub hd, hpadd
    m[4] = _mm256_setzero_si256(); // pxor m4, m4
    m[5] = vpbroadcastd_p7(ro(z2_y_shuf_h4, 24)); // vpbroadcastd m5, [pw_1]
                                                  // tzcnt r8d, wd (w is 4, 8, 16 or 32, never zero); lea r5, [ipred_cfl_ac_444_avx2_table];
                                                  // movsxd r8, [r5+r8*4+12]; add r5, r8: the table entries `w4`, `w8`, `w16`, `w32` (a width outside the
                                                  // contract would index outside the table, undefined in the original; here it takes `w32`).
    let r8 = w.trailing_zeros();
    let ac_bak = ac; // mov ac_bakq, acq
    match r8 {
        // jmp r5
        2 => {
            ipred_cfl_ac_444_8bpc_w4(&mut m, ac, y, stride, hpad, h);
            ipred_cfl_ac_444_8bpc_calc_avg_mul(&mut m);
        }
        3 => {
            ipred_cfl_ac_444_8bpc_w8(&mut m, ac, y, stride, hpad, h);
            ipred_cfl_ac_444_8bpc_calc_avg_mul(&mut m);
        }
        4 => ipred_cfl_ac_444_8bpc_w16(&mut m, ac, y, stride, wpad, hpad, h),
        _ => ipred_cfl_ac_444_8bpc_w32(&mut m, ac, y, stride, wpad, hpad, h),
    }
    ipred_cfl_ac_444_8bpc_calc_avg(&mut m, ac_bak, sz);
}
