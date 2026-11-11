// libwebp v1.6.0, sharpyuv/sharpyuv.c:
//
// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Sharp RGB to YUV conversion.
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, sharpyuv/sharpyuv.h:
//
// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Sharp RGB to YUV conversion.

//! Translation of `sharpyuv/sharpyuv.c` and `sharpyuv/sharpyuv.h` from libwebp v1.6.0.
//!
//! Work package: WP5 (`SharpYUV`).
//!
//! Pointers into the caller's buffers become slices with a byte offset of their
//! first sample (ADR 0003). A C `void*` sample buffer is a byte slice: 8-bit
//! samples are one byte, and samples of 10, 12 or 16 bits are two native-endian
//! bytes at the byte positions that the C code's `uint16_t` indexing addresses.
//! Strides and steps keep their C type, in bytes, and rows are addressed from the
//! offset with a checked signed sum.
//!
//! `SharpYuvGetCPUInfo` is translated in `sharpyuv_cpu.rs`. `SharpYuvInit` is not
//! (see `# Representation`, item 1). Its argument replaces the CPU-information
//! pointer, which ADR 0003 leaves to the ABI module that exports the pointer, and
//! its gamma-table work is already done on first use here. The conversion
//! functions below do not call `SharpYuvInit`, because nothing in the Rust
//! conversion depends on it: the dispatch functions are direct calls and the gamma
//! tables are computed on first use.
//!
//! # Representation
//!
//! 1. **Dispatch and init-once state.** Dispatch pointers are plain functions
//!    over the portable `_c` implementations (ADR 0003). The functions that
//!    only assign pointers are absent: `SharpYuvInitDsp`, and the pointer and
//!    mutex state of `SharpYuvInit`. The runtime selection of the SSE2 and NEON
//!    variants belongs to the SIMD modules (ADR 0008); see item 1 of
//!    `# Deviations from upstream`.
//!    - `SharpYuvInit` stays absent. Forcing the gamma tables is the only part
//!      the Rust code can keep, and the Rust gamma tables already compute on
//!      first use. The rest of the function replaces the C pointer
//!      `SharpYuvGetCPUInfo` with the caller's argument. ADR 0003 gives that
//!      pointer no Rust state: the query is a function, and the ABI module that
//!      exports the pointer owns any override, which is the rule ADR 0003
//!      states for `VP8GetCPUInfo`. Dropping the argument would change what
//!      `SharpYuvGetCPUInfo` answers after `SharpYuvInit(f)` once the SIMD
//!      variants consult the query, so the function cannot be translated
//!      faithfully. No `SharpYUV` C ABI is planned (ADR 0004).
//!    - `sharpyuv_cpu.rs` translates `SharpYuvGetCPUInfo` as a function with
//!      that name that calls `vp8_get_cpu_info`, with the `VP8CPUInfo`
//!      signature. The C file is the re-compilation of `cpu.c` under another
//!      name, so the answers are the same query. The NULL pointer of the
//!      reference configuration is item 1 of `# Deviations from upstream` in
//!      `src/sharpyuv/sharpyuv_cpu.rs`.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **No `SharpYuvInit` call and no dispatch initialization.** C conversions
//!    call `SharpYuvInit`, which assigns the dispatch pointers, computes the
//!    gamma tables and stores the CPU-information function. In Rust the
//!    dispatch functions are direct calls (ADR 0003), the tables are `LazyLock`
//!    statics computed on first use. The ported SIMD dispatch (ADR 0008)
//!    selects kernels independently of `SharpYUV` C initialization; the public
//!    Rust API does not implement its callback override.
//! 2. **Negative strides and steps.** Rows and pixels are addressed with a
//!    checked signed sum from the origin, so a flipped plane works as in C when
//!    the origin is its last row or pixel (tested, groups `flipped-*`). A row
//!    that would start before the slice panics, where C reads memory before the
//!    buffer, which C leaves undefined.
//! 3. **Predicates return `bool`.** `SharpYuvConvert` and
//!    `SharpYuvConvertWithOptions` return `bool` in place of `int` 0 and 1
//!    (ADR 0003, predicate rule), and `SharpYuvOptionsInitInternal` returns an
//!    `Option`. `SharpYuvGetVersion` keeps its `i32`.
//! 4. **Integer width of sizes.** Buffer sizes are computed in `usize` and
//!    `u64`, where the C code multiplies `int` values (`w * 3` and similar)
//!    that can overflow for widths near `INT_MAX`. The inventory of
//!    `tests/wp5_convert.rs` has no such size; the allocation fails in both
//!    (item 3 of `# Checks not applicable` in `tests/wp5_convert.rs`).
//! 5. **Integer overflow of `RGBToYUVComponent`.** The C `int` products are
//!    undefined when they overflow; the Rust arithmetic panics in the dev and
//!    `checked` profiles. No input in the inventory or the seeded cases
//!    overflows, which the `checked` runs confirm.

use std::mem::size_of;

use super::sharpyuv_dsp::{sharp_yuv_filter_row, sharp_yuv_update_rgb, sharp_yuv_update_y};
use super::sharpyuv_gamma::GammaContext;

/// Sample access supplied by the caller. The C adapter owns its raw-pointer
/// safety contract; the WebP adapter checks all access through Rust slices.
pub trait SampleAccess {
    fn read(&self, channel: usize, row: usize, column: usize, bit_depth: i32) -> u16;
    fn write(&mut self, plane: usize, row: usize, column: usize, value: u16, bit_depth: i32);
}

/// The seven scratch allocations in their original order. Callers own their
/// allocation and failure policy; conversion only sees initialized storage.
pub struct Scratch<'a> {
    pub tmp_buffer: &'a mut [u16],
    pub best_y_base: &'a mut [u16],
    pub target_y_base: &'a mut [u16],
    pub best_rgb_y: &'a mut [u16],
    pub best_uv_base: &'a mut [i16],
    pub target_uv_base: &'a mut [i16],
    pub best_rgb_uv: &'a mut [i16],
}

/// DSP policy is selected at each package boundary. The AVIF C ABI keeps its
/// independent callback state; WebP retains its own runtime SIMD dispatch.
#[derive(Clone, Copy)]
pub struct Dsp {
    pub update_y: UpdateY,
    pub update_rgb: fn(&[i16], &[i16], &mut [i16], i32),
    pub filter_row: FilterRow,
}

pub type UpdateY = fn(&[u16], &[u16], &mut [u16], i32, i32) -> u64;
pub type FilterRow = fn(&[i16], &[i16], i32, &[u16], &mut [u16], i32);

impl Dsp {
    pub const SCALAR: Self = Self {
        update_y: sharp_yuv_update_y,
        update_rgb: sharp_yuv_update_rgb,
        filter_row: sharp_yuv_filter_row,
    };
}

// upstream: sharpyuv/sharpyuv.h SHARPYUV_VERSION_MAJOR
pub const SHARPYUV_VERSION_MAJOR: i32 = 0;
// upstream: sharpyuv/sharpyuv.h SHARPYUV_VERSION_MINOR
pub const SHARPYUV_VERSION_MINOR: i32 = 4;
// upstream: sharpyuv/sharpyuv.h SHARPYUV_VERSION_PATCH
pub const SHARPYUV_VERSION_PATCH: i32 = 2;

// upstream: sharpyuv/sharpyuv.h SHARPYUV_MAKE_VERSION
#[must_use]
pub const fn sharpyuv_make_version(major: i32, minor: i32, patch: i32) -> i32 {
    (major << 24) | (minor << 16) | patch
}

// upstream: sharpyuv/sharpyuv.h SHARPYUV_VERSION
pub const SHARPYUV_VERSION: i32 = sharpyuv_make_version(
    SHARPYUV_VERSION_MAJOR,
    SHARPYUV_VERSION_MINOR,
    SHARPYUV_VERSION_PATCH,
);

// upstream: sharpyuv/sharpyuv.c SharpYuvGetVersion
#[must_use]
pub const fn sharp_yuv_get_version() -> i32 {
    SHARPYUV_VERSION
}

// upstream: sharpyuv/sharpyuv.h SharpYuvConversionMatrix
/// RGB to YUV conversion matrix, in 16 bit fixed point.
///
/// `y_ = rgb_to_y[0] * r + rgb_to_y[1] * g + rgb_to_y[2] * b + rgb_to_y[3]`, and
/// likewise for `u_` and `v_`; the values are then divided by `1 << 16` and rounded.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SharpYuvConversionMatrix {
    /// Coefficients and offset of the Y component.
    pub rgb_to_y: [i32; 4],
    /// Coefficients and offset of the U component.
    pub rgb_to_u: [i32; 4],
    /// Coefficients and offset of the V component.
    pub rgb_to_v: [i32; 4],
}

// upstream: sharpyuv/sharpyuv.h SharpYuvTransferFunctionType
/// A transfer function as defined in H.273; a C enumeration stored as its `unsigned int`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SharpYuvTransferFunctionType(pub u32);

impl SharpYuvTransferFunctionType {
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionBt709
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionBt709: Self = Self(1);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionBt470M
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionBt470M: Self = Self(4);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionBt470Bg
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionBt470Bg: Self = Self(5);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionBt601
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionBt601: Self = Self(6);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionSmpte240
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionSmpte240: Self = Self(7);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionLinear
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionLinear: Self = Self(8);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionLog100
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionLog100: Self = Self(9);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionLog100_Sqrt10
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionLog100_Sqrt10: Self = Self(10);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionIec61966
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionIec61966: Self = Self(11);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionBt1361
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionBt1361: Self = Self(12);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionSrgb
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionSrgb: Self = Self(13);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionBt2020_10Bit
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionBt2020_10Bit: Self = Self(14);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionBt2020_12Bit
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionBt2020_12Bit: Self = Self(15);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionSmpte2084
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionSmpte2084: Self = Self(16);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionSmpte428
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionSmpte428: Self = Self(17);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionHlg
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionHlg: Self = Self(18);
    // upstream: sharpyuv/sharpyuv.h kSharpYuvTransferFunctionNum
    /// The number of transfer functions; not a transfer function.
    #[expect(non_upper_case_globals, reason = "C enumerator name")]
    pub const kSharpYuvTransferFunctionNum: Self = Self(19);
}

// upstream: sharpyuv/sharpyuv.h SharpYuvOptions
/// Conversion options: the matrix, which the caller owns, and the transfer function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SharpYuvOptions<'a> {
    /// The RGB to YUV matrix; it cannot be absent.
    pub yuv_matrix: &'a SharpYuvConversionMatrix,
    /// The transfer function of the RGB samples.
    pub transfer_type: SharpYuvTransferFunctionType,
}

// upstream: sharpyuv/sharpyuv.c kNumIterations
const NUM_ITERATIONS: u32 = 4;

// upstream: sharpyuv/sharpyuv.c YUV_FIX
const YUV_FIX: i32 = 16;

// upstream: sharpyuv/sharpyuv.c kYuvHalf
const YUV_HALF: i32 = 1 << (YUV_FIX - 1);

// upstream: sharpyuv/sharpyuv.c kMaxBitDepth
const MAX_BIT_DEPTH: i32 = 14;

// upstream: sharpyuv/sharpyuv.c GetPrecisionShift
// Returns the precision shift to use based on the input rgb_bit_depth.
fn get_precision_shift(rgb_bit_depth: i32) -> i32 {
    // Try to add 2 bits of precision if it fits in kMaxBitDepth. Otherwise remove
    // bits if needed.
    if rgb_bit_depth + 2 <= MAX_BIT_DEPTH {
        2
    } else {
        MAX_BIT_DEPTH - rgb_bit_depth
    }
}

// upstream: sharpyuv/sharpyuv.c clip_8b
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn clip_8b(v: i16) -> u8 {
    if i32::from(v) & !0xff == 0 {
        v as u8
    } else if v < 0 {
        0
    } else {
        255
    }
}

// upstream: sharpyuv/sharpyuv.c clip
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn clip(v: i16, max: i32) -> u16 {
    if v < 0 {
        0
    } else if i32::from(v) > max {
        max as u16
    } else {
        v as u16
    }
}

// upstream: sharpyuv/sharpyuv.c clip_bit_depth
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn clip_bit_depth(y: i32, bit_depth: i32) -> u16 {
    let max = (1 << bit_depth) - 1;
    if y & !max == 0 {
        y as u16
    } else if y < 0 {
        0
    } else {
        max as u16
    }
}

// upstream: sharpyuv/sharpyuv.c RGBToGray
#[expect(
    clippy::cast_possible_truncation,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn rgb_to_gray(r: i64, g: i64, b: i64) -> i32 {
    let luma = 13933 * r + 46871 * g + 4732 * b + i64::from(YUV_HALF);
    (luma >> YUV_FIX) as i32
}

// upstream: sharpyuv/sharpyuv.c ScaleDown
fn scale_down(a: u16, b: u16, c: u16, d: u16, rgb_bit_depth: i32, gamma: &GammaContext) -> u32 {
    let bit_depth = rgb_bit_depth + get_precision_shift(rgb_bit_depth);
    let a_lin = gamma.to_linear(a, bit_depth);
    let b_lin = gamma.to_linear(b, bit_depth);
    let c_lin = gamma.to_linear(c, bit_depth);
    let d_lin = gamma.to_linear(d, bit_depth);
    u32::from(gamma.to_gamma((a_lin + b_lin + c_lin + d_lin + 2) >> 2, bit_depth))
}

// upstream: sharpyuv/sharpyuv.c UpdateW
// `src` holds three rows of `w` samples (R, G, B); `dst` receives one row.
#[expect(
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn update_w(src: &[u16], dst: &mut [u16], w: usize, rgb_bit_depth: i32, gamma: &GammaContext) {
    let bit_depth = rgb_bit_depth + get_precision_shift(rgb_bit_depth);
    for (i, out) in dst[..w].iter_mut().enumerate() {
        let r = gamma.to_linear(src[i], bit_depth);
        let g = gamma.to_linear(src[w + i], bit_depth);
        let b = gamma.to_linear(src[2 * w + i], bit_depth);
        let y = rgb_to_gray(i64::from(r), i64::from(g), i64::from(b));
        *out = gamma.to_gamma(y as u32, bit_depth);
    }
}

// upstream: sharpyuv/sharpyuv.c UpdateChroma
// `src1` and `src2` hold the R, G and B rows of two luma rows; `dst` receives the
// three chroma differences of `uv_w` pixels.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn update_chroma(
    src1: &[u16],
    src2: &[u16],
    dst: &mut [i16],
    uv_w: usize,
    rgb_bit_depth: i32,
    gamma: &GammaContext,
) {
    for i in 0..uv_w {
        let base = 2 * i;
        let r = scale_down(
            src1[base],
            src1[base + 1],
            src2[base],
            src2[base + 1],
            rgb_bit_depth,
            gamma,
        ) as i32;
        let g = scale_down(
            src1[base + 2 * uv_w],
            src1[base + 2 * uv_w + 1],
            src2[base + 2 * uv_w],
            src2[base + 2 * uv_w + 1],
            rgb_bit_depth,
            gamma,
        ) as i32;
        let b = scale_down(
            src1[base + 4 * uv_w],
            src1[base + 4 * uv_w + 1],
            src2[base + 4 * uv_w],
            src2[base + 4 * uv_w + 1],
            rgb_bit_depth,
            gamma,
        ) as i32;
        let w_gray = rgb_to_gray(i64::from(r), i64::from(g), i64::from(b));
        dst[i] = (r - w_gray) as i16;
        dst[i + uv_w] = (g - w_gray) as i16;
        dst[i + 2 * uv_w] = (b - w_gray) as i16;
    }
}

// upstream: sharpyuv/sharpyuv.c StoreGray
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn store_gray(rgb: &[u16], y: &mut [u16], w: usize) {
    for (i, out) in y[..w].iter_mut().enumerate() {
        *out = rgb_to_gray(
            i64::from(rgb[i]),
            i64::from(rgb[w + i]),
            i64::from(rgb[2 * w + i]),
        ) as u16;
    }
}

// upstream: sharpyuv/sharpyuv.c Filter2
fn filter2(a: i32, b: i32, w0: i32, bit_depth: i32) -> u16 {
    let v0 = (a * 3 + b + 2) >> 2;
    clip_bit_depth(v0 + w0, bit_depth)
}

// upstream: sharpyuv/sharpyuv.c Shift
fn shift(v: i32, shift_amount: i32) -> i32 {
    if shift_amount >= 0 {
        v << shift_amount
    } else {
        v >> -shift_amount
    }
}

// The byte index of sample `count` of a row that is `step` bytes apart, from
// `origin`. A negative result would address memory before the buffer, which the
// C code may do with a negative stride; here it panics, as an index does.
#[expect(
    clippy::cast_possible_wrap,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn sample_index(origin: usize, count: usize, step: i32) -> usize {
    usize::try_from(origin as isize + count as isize * step as isize)
        .expect("sample address before the start of the buffer")
}

// Reads sample `index` of an input plane: one byte for 8 bits, two native-endian
// bytes otherwise, as the C code reads `uint8_t` and `uint16_t` samples.
fn read_sample(buf: &[u8], index: usize, bit_depth: i32) -> u16 {
    if bit_depth == 8 {
        u16::from(buf[index])
    } else {
        u16::from_ne_bytes([buf[index], buf[index + 1]])
    }
}

// Writes `value` at byte `index` of an output plane: one byte for 8 bits, two
// native-endian bytes otherwise.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn write_sample(buf: &mut [u8], index: usize, value: u16, bit_depth: i32) {
    if bit_depth <= 8 {
        buf[index] = value as u8;
    } else {
        buf[index..index + 2].copy_from_slice(&value.to_ne_bytes());
    }
}

struct SliceAccess<'a> {
    input: [(&'a [u8], usize); 3],
    output: [(&'a mut [u8], usize, i32); 3],
    rgb_step: i32,
    rgb_stride: i32,
}

impl SampleAccess for SliceAccess<'_> {
    fn read(&self, channel: usize, row: usize, column: usize, bit_depth: i32) -> u16 {
        let (bytes, origin) = self.input[channel];
        let index = sample_index(
            sample_index(origin, row, self.rgb_stride),
            column,
            self.rgb_step,
        );
        read_sample(bytes, index, bit_depth)
    }

    fn write(&mut self, plane: usize, row: usize, column: usize, value: u16, bit_depth: i32) {
        let (bytes, origin, stride) = &mut self.output[plane];
        let unit = if bit_depth <= 8 { 1 } else { 2 };
        let index = sample_index(*origin, row, *stride) + column * unit;
        write_sample(bytes, index, value, bit_depth);
    }
}

// upstream: sharpyuv/sharpyuv.c ImportOneRow
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn import_one_row(
    access: &impl SampleAccess,
    row: usize,
    rgb_bit_depth: i32,
    pic_width: usize,
    dst: &mut [u16],
    w: usize,
) {
    let shift_amount = get_precision_shift(rgb_bit_depth);
    for i in 0..pic_width {
        let r = access.read(0, row, i, rgb_bit_depth);
        let g = access.read(1, row, i, rgb_bit_depth);
        let b = access.read(2, row, i, rgb_bit_depth);
        dst[i] = shift(i32::from(r), shift_amount) as u16;
        dst[i + w] = shift(i32::from(g), shift_amount) as u16;
        dst[i + 2 * w] = shift(i32::from(b), shift_amount) as u16;
    }
    if pic_width & 1 == 1 {
        // replicate rightmost pixel
        dst[pic_width] = dst[pic_width - 1];
        dst[pic_width + w] = dst[pic_width + w - 1];
        dst[pic_width + 2 * w] = dst[pic_width + 2 * w - 1];
    }
}

// upstream: sharpyuv/sharpyuv.c InterpolateTwoRows
// `prev_uv`, `cur_uv` and `next_uv` are three chroma rows; `out1` and `out2` hold
// the R, G and B rows of the two luma rows that they interpolate.
#[expect(clippy::too_many_arguments, reason = "mirrors the C function")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn interpolate_two_rows(
    best_y: &[u16],
    prev_uv: &[i16],
    cur_uv: &[i16],
    next_uv: &[i16],
    w: usize,
    out1: &mut [u16],
    out2: &mut [u16],
    rgb_bit_depth: i32,
    dsp: Dsp,
) {
    let uv_w = w >> 1;
    let len = (w - 1) >> 1; // length to filter
    let bit_depth = rgb_bit_depth + get_precision_shift(rgb_bit_depth);
    // process each R/G/B segments in turn
    for k in 0..3 {
        let p = k * uv_w;
        let o = k * w;
        // special boundary case for i==0
        out1[o] = filter2(
            i32::from(cur_uv[p]),
            i32::from(prev_uv[p]),
            i32::from(best_y[0]),
            bit_depth,
        );
        out2[o] = filter2(
            i32::from(cur_uv[p]),
            i32::from(next_uv[p]),
            i32::from(best_y[w]),
            bit_depth,
        );

        (dsp.filter_row)(
            &cur_uv[p..],
            &prev_uv[p..],
            len as i32,
            &best_y[1..],
            &mut out1[o + 1..],
            bit_depth,
        );
        (dsp.filter_row)(
            &cur_uv[p..],
            &next_uv[p..],
            len as i32,
            &best_y[w + 1..],
            &mut out2[o + 1..],
            bit_depth,
        );

        // special boundary case for i == w - 1 when w is even
        if w & 1 == 0 {
            out1[o + w - 1] = filter2(
                i32::from(cur_uv[p + uv_w - 1]),
                i32::from(prev_uv[p + uv_w - 1]),
                i32::from(best_y[w - 1]),
                bit_depth,
            );
            out2[o + w - 1] = filter2(
                i32::from(cur_uv[p + uv_w - 1]),
                i32::from(next_uv[p + uv_w - 1]),
                i32::from(best_y[w - 1 + w]),
                bit_depth,
            );
        }
    }
}

// upstream: sharpyuv/sharpyuv.c RGBToYUVComponent
fn rgb_to_yuv_component(r: i32, g: i32, b: i32, coeffs: &[i32; 4], sfix: i32) -> i32 {
    let srounder = 1 << (YUV_FIX + sfix - 1);
    let luma = coeffs[0] * r + coeffs[1] * g + coeffs[2] * b + coeffs[3] + srounder;
    luma >> (YUV_FIX + sfix)
}

// upstream: sharpyuv/sharpyuv.c ConvertWRGBToYUV
#[expect(clippy::too_many_arguments, reason = "mirrors the C function")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::many_single_char_names,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn convert_wrgb_to_yuv(
    best_y: &[u16],
    best_uv: &[i16],
    access: &mut impl SampleAccess,
    rgb_bit_depth: i32,
    yuv_bit_depth: i32,
    width: usize,
    height: usize,
    yuv_matrix: &SharpYuvConversionMatrix,
) -> bool {
    let w = (width + 1) & !1;
    let h = (height + 1) & !1;
    let uv_w = w >> 1;
    let uv_h = h >> 1;
    let sfix = get_precision_shift(rgb_bit_depth);
    let yuv_max = (1 << yuv_bit_depth) - 1;

    let mut best_uv_index = 0_usize;
    for j in 0..height {
        for i in 0..width {
            let off = i >> 1;
            let w_value = i32::from(best_y[j * w + i]);
            let r = i32::from(best_uv[best_uv_index + off]) + w_value;
            let g = i32::from(best_uv[best_uv_index + off + uv_w]) + w_value;
            let b = i32::from(best_uv[best_uv_index + off + 2 * uv_w]) + w_value;
            let y = rgb_to_yuv_component(r, g, b, &yuv_matrix.rgb_to_y, sfix);
            let value = if yuv_bit_depth <= 8 {
                u16::from(clip_8b(y as i16))
            } else {
                clip(y as i16, yuv_max)
            };
            access.write(0, j, i, value, yuv_bit_depth);
        }
        best_uv_index += (j & 1) * 3 * uv_w;
    }

    for j in 0..uv_h {
        let base = j * 3 * uv_w;
        for i in 0..uv_w {
            // Note r, g and b values here are off by W, but a constant offset on all
            // 3 components doesn't change the value of u and v with a YCbCr matrix.
            let r = i32::from(best_uv[base + i]);
            let g = i32::from(best_uv[base + i + uv_w]);
            let b = i32::from(best_uv[base + i + 2 * uv_w]);
            let u = rgb_to_yuv_component(r, g, b, &yuv_matrix.rgb_to_u, sfix);
            let v = rgb_to_yuv_component(r, g, b, &yuv_matrix.rgb_to_v, sfix);
            if yuv_bit_depth <= 8 {
                access.write(1, j, i, u16::from(clip_8b(u as i16)), yuv_bit_depth);
                access.write(2, j, i, u16::from(clip_8b(v as i16)), yuv_bit_depth);
            } else {
                access.write(1, j, i, clip(u as i16, yuv_max), yuv_bit_depth);
                access.write(2, j, i, clip(v as i16, yuv_max), yuv_bit_depth);
            }
        }
    }
    true
}

// upstream: sharpyuv/sharpyuv.c SafeMalloc, SAFE_ALLOC
// Allocates `nmemb` zeroed elements of `T`, or returns `None` when the size does not
// fit in `size_t` or the allocation is refused. Rust initializes the memory that C
// leaves uninitialized; the C code never reads it before writing it.
fn safe_malloc<T: Default + Clone>(nmemb: u64) -> Option<Vec<T>> {
    let total_size = nmemb.checked_mul(size_of::<T>() as u64)?;
    usize::try_from(total_size).ok()?;
    let count = usize::try_from(nmemb).ok()?;
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(count).ok()?;
    buffer.resize(count, T::default());
    Some(buffer)
}

// upstream: sharpyuv/sharpyuv.c DoSharpArgbToYuv
#[expect(clippy::too_many_arguments, reason = "mirrors the C function")]
#[expect(clippy::too_many_lines, reason = "mirrors the C function")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn do_sharp_argb_to_yuv(
    access: &mut impl SampleAccess,
    scratch: Scratch<'_>,
    dsp: Dsp,
    rgb_bit_depth: i32,
    yuv_bit_depth: i32,
    width: usize,
    height: usize,
    yuv_matrix: &SharpYuvConversionMatrix,
    transfer_type: SharpYuvTransferFunctionType,
) -> bool {
    // we expand the right/bottom border if needed
    let w = (width + 1) & !1;
    let h = (height + 1) & !1;
    let uv_w = w >> 1;
    let uv_h = h >> 1;
    let y_bit_depth = rgb_bit_depth + get_precision_shift(rgb_bit_depth);
    let gamma = GammaContext::new(transfer_type);
    let mut prev_diff_y_sum: u64 = u64::MAX;

    let Scratch {
        tmp_buffer,
        best_y_base,
        target_y_base,
        best_rgb_y,
        best_uv_base,
        target_uv_base,
        best_rgb_uv,
    } = scratch;
    if tmp_buffer.len() < w * 6
        || best_y_base.len() < w * h
        || target_y_base.len() < w * h
        || best_rgb_y.len() < w * 2
        || best_uv_base.len() < uv_w * 3 * uv_h
        || target_uv_base.len() < uv_w * 3 * uv_h
        || best_rgb_uv.len() < uv_w * 3
    {
        return false;
    }
    let diff_y_threshold = (3.0_f64 * w as f64 * h as f64) as u64;

    // Import RGB samples to W/RGB representation.
    let mut best_y_index = 0_usize;
    let mut target_y_index = 0_usize;
    let mut best_uv_index = 0_usize;
    let mut target_uv_index = 0_usize;
    let mut j = 0_usize;
    while j < height {
        let is_last_row = j == height - 1;
        let (src1, src2) = tmp_buffer.split_at_mut(3 * w);

        // prepare two rows of input
        import_one_row(access, j, rgb_bit_depth, width, src1, w);
        if is_last_row {
            src2.copy_from_slice(src1);
        } else {
            import_one_row(access, j + 1, rgb_bit_depth, width, src2, w);
        }
        store_gray(src1, &mut best_y_base[best_y_index..], w);
        store_gray(src2, &mut best_y_base[best_y_index + w..], w);

        update_w(
            src1,
            &mut target_y_base[target_y_index..],
            w,
            rgb_bit_depth,
            &gamma,
        );
        update_w(
            src2,
            &mut target_y_base[target_y_index + w..],
            w,
            rgb_bit_depth,
            &gamma,
        );
        update_chroma(
            src1,
            src2,
            &mut target_uv_base[target_uv_index..],
            uv_w,
            rgb_bit_depth,
            &gamma,
        );
        best_uv_base[best_uv_index..best_uv_index + 3 * uv_w]
            .copy_from_slice(&target_uv_base[target_uv_index..target_uv_index + 3 * uv_w]);
        best_y_index += 2 * w;
        best_uv_index += 3 * uv_w;
        target_y_index += 2 * w;
        target_uv_index += 3 * uv_w;
        j += 2;
    }

    // Iterate and resolve clipping conflicts.
    for iter in 0..NUM_ITERATIONS {
        let mut cur_uv_index = 0_usize;
        let mut prev_uv_index = 0_usize;
        let mut diff_y_sum: u64 = 0;

        best_y_index = 0;
        best_uv_index = 0;
        target_y_index = 0;
        target_uv_index = 0;
        j = 0;
        loop {
            let next_uv_index = cur_uv_index + if j + 2 < h { 3 * uv_w } else { 0 };
            {
                let (src1, src2) = tmp_buffer.split_at_mut(3 * w);
                interpolate_two_rows(
                    &best_y_base[best_y_index..],
                    &best_uv_base[prev_uv_index..],
                    &best_uv_base[cur_uv_index..],
                    &best_uv_base[next_uv_index..],
                    w,
                    src1,
                    src2,
                    rgb_bit_depth,
                    dsp,
                );
                prev_uv_index = cur_uv_index;
                cur_uv_index = next_uv_index;

                update_w(src1, &mut best_rgb_y[..w], w, rgb_bit_depth, &gamma);
                update_w(src2, &mut best_rgb_y[w..2 * w], w, rgb_bit_depth, &gamma);
                update_chroma(src1, src2, best_rgb_uv, uv_w, rgb_bit_depth, &gamma);
            }

            // update two rows of Y and one row of RGB
            diff_y_sum += (dsp.update_y)(
                &target_y_base[target_y_index..],
                best_rgb_y,
                &mut best_y_base[best_y_index..],
                (2 * w) as i32,
                y_bit_depth,
            );
            (dsp.update_rgb)(
                &target_uv_base[target_uv_index..],
                best_rgb_uv,
                &mut best_uv_base[best_uv_index..],
                (3 * uv_w) as i32,
            );

            best_y_index += 2 * w;
            best_uv_index += 3 * uv_w;
            target_y_index += 2 * w;
            target_uv_index += 3 * uv_w;
            j += 2;
            if j >= h {
                break;
            }
        }
        // test exit condition
        if iter > 0 {
            if diff_y_sum < diff_y_threshold {
                break;
            }
            if diff_y_sum > prev_diff_y_sum {
                break;
            }
        }
        prev_diff_y_sum = diff_y_sum;
    }

    // final reconstruction
    convert_wrgb_to_yuv(
        best_y_base,
        best_uv_base,
        access,
        rgb_bit_depth,
        yuv_bit_depth,
        width,
        height,
        yuv_matrix,
    )
}

// upstream: sharpyuv/sharpyuv.h SharpYuvOptionsInitInternal
/// Returns the options for `yuv_matrix`, or `None` when `version` is not the version
/// of this library in its major and minor numbers (the C function returns 0 and
/// leaves `options` unchanged). The NULL checks of the C function are not
/// representable here.
#[must_use]
pub fn sharp_yuv_options_init_internal(
    yuv_matrix: &SharpYuvConversionMatrix,
    version: i32,
) -> Option<SharpYuvOptions<'_>> {
    let major = version >> 24;
    let minor = (version >> 16) & 0xff;
    // The C condition is (major == 0 && minor != 4) || major != 0, which is the
    // same as the condition below.
    if major != SHARPYUV_VERSION_MAJOR || minor != SHARPYUV_VERSION_MINOR {
        return None;
    }
    Some(SharpYuvOptions {
        yuv_matrix,
        transfer_type: SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSrgb,
    })
}

// upstream: sharpyuv/sharpyuv.h SharpYuvOptionsInit
#[must_use]
pub fn sharp_yuv_options_init(
    yuv_matrix: &SharpYuvConversionMatrix,
) -> Option<SharpYuvOptions<'_>> {
    sharp_yuv_options_init_internal(yuv_matrix, SHARPYUV_VERSION)
}

/// Lengths of the seven scratch buffers, in elements of their respective type.
#[must_use]
pub fn scratch_lengths(width: usize, height: usize) -> [usize; 7] {
    let w = (width + 1) & !1;
    let h = (height + 1) & !1;
    let uv_w = w / 2;
    let uv_h = h / 2;
    [
        w * 6,
        w * h,
        w * h,
        w * 2,
        uv_w * 3 * uv_h,
        uv_w * 3 * uv_h,
        uv_w * 3,
    ]
}

/// Arguments rejected by both public boundaries before pixel access.
#[must_use]
#[expect(
    clippy::too_many_arguments,
    reason = "mirrors the C validation boundary"
)]
pub fn valid_arguments(
    rgb_step: i32,
    rgb_stride: i32,
    rgb_bit_depth: i32,
    y_stride: i32,
    u_stride: i32,
    v_stride: i32,
    yuv_bit_depth: i32,
    width: i32,
    height: i32,
) -> bool {
    width >= 1
        && height >= 1
        && width != i32::MAX
        && height != i32::MAX
        && matches!(rgb_bit_depth, 8 | 10 | 12 | 16)
        && matches!(yuv_bit_depth, 8 | 10 | 12)
        && (rgb_bit_depth == 8 || (rgb_step % 2 == 0 && rgb_stride % 2 == 0))
        && (yuv_bit_depth == 8 || (y_stride % 2 == 0 && u_stride % 2 == 0 && v_stride % 2 == 0))
}

/// One conversion algorithm for the safe slice and raw C adapters. Storage is
/// already allocated and initialized before this call.
#[must_use]
#[expect(clippy::too_many_arguments, reason = "narrow adapter boundary")]
pub fn convert_with_access(
    access: &mut impl SampleAccess,
    scratch: Scratch<'_>,
    dsp: Dsp,
    rgb_bit_depth: i32,
    yuv_bit_depth: i32,
    width: usize,
    height: usize,
    yuv_matrix: &SharpYuvConversionMatrix,
    transfer_type: SharpYuvTransferFunctionType,
) -> bool {
    let rgb_max = (1 << rgb_bit_depth) - 1;
    let rgb_round = 1 << (rgb_bit_depth - 1);
    let yuv_max = (1 << yuv_bit_depth) - 1;
    let sfix = get_precision_shift(rgb_bit_depth);
    let mut scaled_matrix = *yuv_matrix;
    if rgb_bit_depth != yuv_bit_depth {
        for i in 0..3 {
            scaled_matrix.rgb_to_y[i] = (yuv_matrix.rgb_to_y[i] * yuv_max + rgb_round) / rgb_max;
            scaled_matrix.rgb_to_u[i] = (yuv_matrix.rgb_to_u[i] * yuv_max + rgb_round) / rgb_max;
            scaled_matrix.rgb_to_v[i] = (yuv_matrix.rgb_to_v[i] * yuv_max + rgb_round) / rgb_max;
        }
    }
    scaled_matrix.rgb_to_y[3] = shift(yuv_matrix.rgb_to_y[3], sfix);
    scaled_matrix.rgb_to_u[3] = shift(yuv_matrix.rgb_to_u[3], sfix);
    scaled_matrix.rgb_to_v[3] = shift(yuv_matrix.rgb_to_v[3], sfix);
    do_sharp_argb_to_yuv(
        access,
        scratch,
        dsp,
        rgb_bit_depth,
        yuv_bit_depth,
        width,
        height,
        &scaled_matrix,
        transfer_type,
    )
}

// upstream: sharpyuv/sharpyuv.c SharpYuvConvertWithOptions
/// Converts RGB to YUV with `options`. Returns `false` for the arguments that the C
/// function rejects; the output is then unchanged.
///
/// Each buffer is a slice whose byte `*_origin` is the first sample, and each
/// stride is in bytes between rows. The `rgb_step` is in bytes between two
/// horizontally adjacent pixels. Both 8-bit and wider samples are read and written
/// at these byte positions. As in the original WebP safe API, an undersized
/// slice or out-of-range origin panics when indexed; callers must provide the
/// storage addressed by their dimensions, strides, and steps.
#[must_use]
#[expect(clippy::too_many_arguments, reason = "mirrors the C signature")]
#[expect(
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
pub fn sharp_yuv_convert_with_options_using(
    r_ptr: &[u8],
    r_origin: usize,
    g_ptr: &[u8],
    g_origin: usize,
    b_ptr: &[u8],
    b_origin: usize,
    rgb_step: i32,
    rgb_stride: i32,
    rgb_bit_depth: i32,
    y_ptr: &mut [u8],
    y_origin: usize,
    y_stride: i32,
    u_ptr: &mut [u8],
    u_origin: usize,
    u_stride: i32,
    v_ptr: &mut [u8],
    v_origin: usize,
    v_stride: i32,
    yuv_bit_depth: i32,
    width: i32,
    height: i32,
    options: &SharpYuvOptions<'_>,
    dsp: Dsp,
) -> bool {
    if !valid_arguments(
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_stride,
        u_stride,
        v_stride,
        yuv_bit_depth,
        width,
        height,
    ) {
        return false;
    }
    let lengths = scratch_lengths(width as usize, height as usize);
    let Some(mut tmp_buffer) = safe_malloc::<u16>(lengths[0] as u64) else {
        return false;
    };
    let Some(mut best_y_base) = safe_malloc::<u16>(lengths[1] as u64) else {
        return false;
    };
    let Some(mut target_y_base) = safe_malloc::<u16>(lengths[2] as u64) else {
        return false;
    };
    let Some(mut best_rgb_y) = safe_malloc::<u16>(lengths[3] as u64) else {
        return false;
    };
    let Some(mut best_uv_base) = safe_malloc::<i16>(lengths[4] as u64) else {
        return false;
    };
    let Some(mut target_uv_base) = safe_malloc::<i16>(lengths[5] as u64) else {
        return false;
    };
    let Some(mut best_rgb_uv) = safe_malloc::<i16>(lengths[6] as u64) else {
        return false;
    };
    let scratch = Scratch {
        tmp_buffer: &mut tmp_buffer,
        best_y_base: &mut best_y_base,
        target_y_base: &mut target_y_base,
        best_rgb_y: &mut best_rgb_y,
        best_uv_base: &mut best_uv_base,
        target_uv_base: &mut target_uv_base,
        best_rgb_uv: &mut best_rgb_uv,
    };
    let mut access = SliceAccess {
        input: [(r_ptr, r_origin), (g_ptr, g_origin), (b_ptr, b_origin)],
        output: [
            (y_ptr, y_origin, y_stride),
            (u_ptr, u_origin, u_stride),
            (v_ptr, v_origin, v_stride),
        ],
        rgb_step,
        rgb_stride,
    };
    convert_with_access(
        &mut access,
        scratch,
        dsp,
        rgb_bit_depth,
        yuv_bit_depth,
        width as usize,
        height as usize,
        options.yuv_matrix,
        options.transfer_type,
    )
}

/// Scalar safe entry point for callers without an ISA-specific dispatch.
#[must_use]
#[expect(clippy::too_many_arguments, reason = "mirrors the C signature")]
pub fn sharp_yuv_convert_with_options(
    r_ptr: &[u8],
    r_origin: usize,
    g_ptr: &[u8],
    g_origin: usize,
    b_ptr: &[u8],
    b_origin: usize,
    rgb_step: i32,
    rgb_stride: i32,
    rgb_bit_depth: i32,
    y_ptr: &mut [u8],
    y_origin: usize,
    y_stride: i32,
    u_ptr: &mut [u8],
    u_origin: usize,
    u_stride: i32,
    v_ptr: &mut [u8],
    v_origin: usize,
    v_stride: i32,
    yuv_bit_depth: i32,
    width: i32,
    height: i32,
    options: &SharpYuvOptions<'_>,
) -> bool {
    sharp_yuv_convert_with_options_using(
        r_ptr,
        r_origin,
        g_ptr,
        g_origin,
        b_ptr,
        b_origin,
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_ptr,
        y_origin,
        y_stride,
        u_ptr,
        u_origin,
        u_stride,
        v_ptr,
        v_origin,
        v_stride,
        yuv_bit_depth,
        width,
        height,
        options,
        Dsp::SCALAR,
    )
}

// upstream: sharpyuv/sharpyuv.c SharpYuvConvert
/// Converts RGB to YUV with the sRGB transfer function and `yuv_matrix`; see
/// [`sharp_yuv_convert_with_options`] for the buffer conventions.
#[must_use]
#[expect(clippy::too_many_arguments, reason = "mirrors the C signature")]
pub fn sharp_yuv_convert(
    r_ptr: &[u8],
    r_origin: usize,
    g_ptr: &[u8],
    g_origin: usize,
    b_ptr: &[u8],
    b_origin: usize,
    rgb_step: i32,
    rgb_stride: i32,
    rgb_bit_depth: i32,
    y_ptr: &mut [u8],
    y_origin: usize,
    y_stride: i32,
    u_ptr: &mut [u8],
    u_origin: usize,
    u_stride: i32,
    v_ptr: &mut [u8],
    v_origin: usize,
    v_stride: i32,
    yuv_bit_depth: i32,
    width: i32,
    height: i32,
    yuv_matrix: &SharpYuvConversionMatrix,
) -> bool {
    let options = SharpYuvOptions {
        yuv_matrix,
        transfer_type: SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSrgb,
    };
    sharp_yuv_convert_with_options(
        r_ptr,
        r_origin,
        g_ptr,
        g_origin,
        b_ptr,
        b_origin,
        rgb_step,
        rgb_stride,
        rgb_bit_depth,
        y_ptr,
        y_origin,
        y_stride,
        u_ptr,
        u_origin,
        u_stride,
        v_ptr,
        v_origin,
        v_stride,
        yuv_bit_depth,
        width,
        height,
        &options,
    )
}
