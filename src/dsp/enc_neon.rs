// libwebp v1.6.0, src/dsp/enc_neon.c:
//
// Copyright 2012 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// ARM NEON version of speed-critical encoding functions.
//
// adapted from libvpx (https://www.webmproject.org/code/)

//! Translation of `src/dsp/enc_neon.c` from libwebp v1.6.0.
//!
//! Work package: WP20 (NEON).
//!
//! The NEON variants of the encoder DSP functions of `src/dsp/enc.c`, and
//! `IsFlat_NEON`, the NEON form of `IsFlat` that `src/dsp/quant.h` selects
//! with `WEBP_USE_NEON` (deviation 1). Each `_neon` function is a body that
//! upstream's `VP8EncDspInitNEON` assigns or one of its helpers; the safe
//! entry points, named after the assigned function without the suffix, are
//! what the dispatch functions of `src/dsp/enc.rs` and `src/dsp/quant.rs`
//! call on 64-bit ARM.
//!
//! `VP8EncDspInitNEON` assigns `VP8ITransform` (`ITransform_NEON`),
//! `VP8FTransform`, `VP8FTransformWHT`, `VP8TDisto4x4`, `VP8TDisto16x16`,
//! `VP8CollectHistogram`, `VP8SSE16x16`, `VP8SSE16x8`, `VP8SSE8x8`,
//! `VP8SSE4x4`, `VP8EncPredLuma4` (`Intra4Preds_NEON`, with `BPS == 32`),
//! `VP8EncPredLuma16` (`Intra16Preds_NEON`), `VP8EncQuantizeBlock` and
//! `VP8EncQuantizeBlockWHT` (both `QuantizeBlock_NEON`) and
//! `VP8EncQuantize2Blocks`. `VP8EncDspInit` calls it unconditionally on
//! 64-bit ARM (`WEBP_NEON_OMIT_C_CODE`), and its `!WEBP_NEON_OMIT_C_CODE`
//! guards leave exactly these pointers without a C assignment, so each of
//! those dispatch functions returns the NEON entry point there. `IsFlat` is
//! `IsFlat_NEON` at compile time on 64-bit ARM, so `is_flat` returns
//! [`is_flat`] there. `VP8FTransform2`, `VP8EncPredChroma8`, `VP8Mean16x4`,
//! `VP8Copy4x4` and `VP8Copy16x8` have no NEON variant; `FTransform2_C`
//! reaches `FTransform_NEON` through `VP8FTransform`, as in C.
//!
//! # Deviations from upstream
//!
//! 1. **`IsFlat_NEON` and `horizontal_add_uint32x4`** come from the
//!    `WEBP_USE_NEON` branch of `src/dsp/quant.h`, whose module
//!    (`src/dsp/quant.rs`) is portable and may not hold the `unsafe` call of
//!    a `#[target_feature]` function (ADR 0008). They live here, in the NEON
//!    module of the encoder, under their `quant.h` provenance; the portable
//!    `IsFlat` became `is_flat_c` and `is_flat` its dispatch function.
//! 2. **Loads, stores and alignment** follow the deviations 1 to 3 of
//!    `src/dsp/neon.rs`. The 2-byte lane loads of `LOAD_LANE_16b`
//!    (`vld1_lane_s16`) read the element from the slice and set the lane
//!    with `vset_lane_s16`, as the 4-byte lane loads do. The `uint8x8_t`
//!    stores of `QuantizeBlock_NEON` into the `int16_t` output go through
//!    `vst1_u8_halfwords`, a byte store as the C cast makes it.
//! 3. **The zero test of `QuantizeBlock_NEON`** reads `out` four entries at a
//!    time as a `uint64_t` in C; the translation tests the same four entries
//!    for a nonzero value, which is the same predicate.
//! 4. **Pointers** are slices from their first element, with `BPS` strides
//!    as in `src/dsp/enc.rs`. `Intra4Preds_NEON` takes the boundary buffer
//!    and the position of `top[0]` and loads the 16 bytes from `top[-5]`,
//!    which the iterator's 40-entry `i4_boundary` holds; `left` of the
//!    intra16 predictors is the buffer with the position of `left[0]`
//!    (`PredLeft`), and `NULL` is `None`.
//! 5. **Macros.** `DC4_VE4_HE4_TM4_NEON` and `RD4_VR4_LD4_VL4_NEON` read
//!    `qcombined` from the caller's scope; the functions take it as an
//!    argument, and the lane of the first is a const parameter, as the
//!    intrinsics need a constant. `LOAD_LANE_16b` and `LOAD_LANE_32b` are
//!    functions with a const lane. Locals keep the C names, also the upper
//!    case ones (`B1`, `D0D1`, `kCst937`), under `non_snake_case`; the local
//!    `C1` of `TransformPass_NEON` is `C1_`, apart from the constant `kC1`.
//! 6. **Structured results.** `Transpose8x2_NEON` and
//!    `Transpose4x4_S16_NEON` write through output pointers in C and
//!    return their vectors here; `TransformPass_NEON` and
//!    `AccumulateSSE16_NEON` take the vectors they update by `&mut`.
//!
//! # Excluded undefined behavior
//!
//! None. The C functions read and write only inside the blocks their callers
//! pass, and the slices check those accesses.
//!
//! # Checks not applicable
//!
//! 1. **The inline-assembly branches** of `ITransformOne_NEON` and
//!    `FTransform_NEON` (`!WEBP_USE_INTRINSICS`) and their tables
//!    `kCoeff16` and `kCoeff32` are not compiled on 64-bit ARM; see
//!    `src/dsp/neon.rs`.
//! 2. **The `vtbl2q_u8` branch of `QuantizeBlock_NEON`** is for Apple's
//!    clang before 6.3 (`__apple_build_version__ < 6020037`); Rust's
//!    intrinsics take the `vtbl4_u8` branch on every 64-bit ARM target,
//!    macOS included, so no `target_vendor` split is needed.
//! 3. **`Vld1U8x2` and `Vld1qU8x4`** use `vld1_u8_x2` and `vld1q_u8_x4`, the
//!    branch for clang 3.4 and later; the two-load fallbacks are for older
//!    compilers.
//! 4. **`WORK_AROUND_GCC`** is not defined, so the quantizers are compiled
//!    and assigned; `BPS` is 32 and `WEBP_AARCH64` is 1, so both intra
//!    predictors are.
//! 5. **`SumToInt_NEON` and `horizontal_add_uint32x4`** use `vaddvq_u32`
//!    (`WEBP_AARCH64`); the pairwise-add fallbacks are for 32-bit ARM.
//! 6. **`WEBP_RESTRICT`** has no counterpart: the slices cannot alias.
//! 7. **The assertion of `Quantize_NEON`** (`QFIX == 17`) tests a constant;
//!    it is a compile-time assertion.
//! 8. **Parity of `Disto4x4_NEON` and `Disto16x16_NEON` for weights that are
//!    not symmetric.** The kernels sum the weighted coefficients in
//!    transposed order, which `VP8TDisto4x4` permits by documenting `w` as a
//!    symmetric 4 by 4 matrix; the encoder passes `kWeightY` and
//!    `kWeightTrellis`, both symmetric, and for them the kernels equal the C
//!    functions (`tests/wp20_enc.rs`). For other weights upstream's kernels,
//!    and the translation, differ from C; `tests/wp10_dsp.rs`, whose records
//!    draw random weights, verifies the C translations directly (decided on
//!    issue #28).
//! 9. **Parity of `ITransform_NEON` for coefficients above about 5,000 in
//!    magnitude.** The kernel computes in saturating 16-bit lanes where C
//!    computes in `int`, so the two differ once an intermediate sum
//!    saturates; the encoder's dequantized coefficients stay below about
//!    2,300, and the parity test covers magnitudes up to 4,096.
//!    `tests/wp10_dsp.rs`, whose records reach 15,000, verifies the C
//!    translation directly.
//! 10. **`IsFlat_NEON` with no block and a negative threshold** answers
//!     `thresh >= 0`, false, where `IsFlat_C` answers true; the encoder
//!     passes 1, 8 or 16 blocks and the thresholds 0, 2 and 3.

use core::arch::aarch64::{
    int16x4_t, int16x4x4_t, int16x8_t, int16x8x2_t, int16x8x4_t, int32x2_t, int32x4x4_t, uint8x8_t,
    uint8x8x2_t, uint8x8x4_t, uint8x16_t, uint8x16x2_t, uint8x16x4_t, uint16x8_t, uint32x2_t,
    uint32x4_t, vabd_u8, vabdq_s16, vabdq_u8, vabs_s32, vabsq_s16, vadd_s16, vaddhn_s32, vaddl_s16,
    vaddl_u8, vaddlv_u8, vaddlvq_u8, vaddq_s16, vaddq_s32, vaddq_u16, vaddq_u32, vaddvq_u32,
    vaddw_u8, vceq_s16, vcombine_s16, vcombine_s64, vcombine_u16, vdup_laneq_u8, vdup_n_s16,
    vdup_n_u32, vdupq_laneq_u8, vdupq_n_s16, vdupq_n_s32, vdupq_n_u8, vdupq_n_u16, vdupq_n_u32,
    veorq_s16, vextq_u8, vget_high_s16, vget_high_s32, vget_high_u8, vget_high_u16, vget_lane_s32,
    vget_lane_u32, vget_low_s16, vget_low_s32, vget_low_u8, vget_low_u16, vgetq_lane_u8,
    vgetq_lane_u32, vhaddq_s32, vhaddq_u8, vhaddq_u32, vhsubq_s32, vminq_u16, vmlal_n_s16,
    vmlsl_n_s16, vmlsl_s16, vmovl_u8, vmovn_s32, vmull_n_s16, vmull_s16, vmull_u8, vmull_u16,
    vmulq_s16, vpadalq_u16, vpadd_s32, vpaddlq_u16, vqadd_s16, vqaddq_s16, vqdmulhq_n_s16,
    vqmovun_s16, vqrshrnh_n_u16, vqsub_s16, vqsubq_s16, vqsubq_u16, vqtbl1q_u8, vqtbl2q_u8,
    vreinterpret_s16_u16, vreinterpret_s32_s16, vreinterpret_s64_s32, vreinterpret_u8_s16,
    vreinterpret_u8_u32, vreinterpret_u32_u8, vreinterpretq_s16_s32, vreinterpretq_s16_s64,
    vreinterpretq_s16_u16, vreinterpretq_s32_s16, vreinterpretq_u8_u32, vreinterpretq_u16_s16,
    vreinterpretq_u32_u8, vrhaddq_u8, vrsraq_n_s16, vset_lane_s16, vset_lane_u8, vset_lane_u32,
    vsetq_lane_s16, vsetq_lane_u8, vsetq_lane_u32, vshlq_n_s16, vshr_n_s16, vshr_n_s32,
    vshrn_n_s32, vshrn_n_u32, vshrq_n_s16, vshrq_n_u16, vsraq_n_s16, vsub_s16, vsubl_s16, vsubl_u8,
    vsubq_s16, vsubq_s32, vtbl4_u8, vtrn_s16, vtrn_s32, vtrnq_s16, vtrnq_s32, vtstq_s16, vzipq_s16,
};

use crate::dsp::dsp::{
    BPS, MAX_COEFF_THRESH, VP8Histogram, WEBP_TRANSFORM_AC3_C1, WEBP_TRANSFORM_AC3_C2,
};
use crate::dsp::enc::{PredLeft, VP8_DSP_SCAN, vp8_set_histogram_data};
use crate::dsp::neon::{
    load_u32_ne, store_u32_ne, transpose4x4_neon, vld1_u8, vld1_u8_x2, vld1q_s16, vld1q_u8,
    vld1q_u8_x4, vld1q_u16, vld1q_u32, vld4_dup_u8, vst1_s16, vst1_u8, vst1_u8_halfwords,
    vst1q_s16, vst1q_u8,
};
use crate::enc::vp8i_enc::{
    I4DC4, I4HD4, I4RD4, I16DC16, I16HE16, I16TM16, I16VE16, MAX_LEVEL, QFIX, VP8Matrix,
};

/// `BPS` as an index stride.
const BPS_U: usize = BPS as usize;

/// The position of a prediction block inside the prediction buffer.
#[expect(clippy::cast_sign_loss, reason = "the block offsets are non-negative")]
const fn at(offset: i32) -> usize {
    offset as usize
}

//------------------------------------------------------------------------------
// Transforms (Paragraph 14.4)

// Inverse transform.
// This code is pretty much the same as TransformOne in the dec_neon.c, except
// for subtraction to *ref. See the comments there for algorithmic explanations.

// upstream: src/dsp/enc_neon.c kC1
#[expect(clippy::cast_possible_truncation, reason = "20091 fits int16_t")]
const C1: i16 = WEBP_TRANSFORM_AC3_C1 as i16;
// upstream: src/dsp/enc_neon.c kC2
/// Half of `kC2`, actually. See the comment above.
#[expect(clippy::cast_possible_truncation, reason = "17734 fits int16_t")]
const C2: i16 = (WEBP_TRANSFORM_AC3_C2 / 2) as i16;

// This code works but is *slower* than the inlined-asm version below
// (with gcc-4.6). So we disable it for now. Later, it'll be conditional to
// WEBP_USE_INTRINSICS define.
// With gcc-4.8, it's a little faster speed than inlined-assembly.

// upstream: src/dsp/enc_neon.c ConvertU8ToS16_NEON
/// Treats `v` as an `uint8x8_t` and zero extends to an `int16x8_t`.
#[target_feature(enable = "neon")]
#[must_use]
fn convert_u8_to_s16_neon(v: uint32x2_t) -> int16x8_t {
    vreinterpretq_s16_u16(vmovl_u8(vreinterpret_u8_u32(v)))
}

// upstream: src/dsp/enc_neon.c SaturateAndStore4x4_NEON
/// Performs unsigned 8b saturation on `dst01` and `dst23` storing the result
/// to the corresponding rows of `dst`.
#[target_feature(enable = "neon")]
fn saturate_and_store4x4_neon(dst: &mut [u8], dst01: int16x8_t, dst23: int16x8_t) {
    // Unsigned saturate to 8b.
    let dst01_u8 = vqmovun_s16(dst01);
    let dst23_u8 = vqmovun_s16(dst23);

    // Store the results.
    store_u32_ne(
        &mut dst[0..],
        vget_lane_u32::<0>(vreinterpret_u32_u8(dst01_u8)),
    );
    store_u32_ne(
        &mut dst[BPS_U..],
        vget_lane_u32::<1>(vreinterpret_u32_u8(dst01_u8)),
    );
    store_u32_ne(
        &mut dst[2 * BPS_U..],
        vget_lane_u32::<0>(vreinterpret_u32_u8(dst23_u8)),
    );
    store_u32_ne(
        &mut dst[3 * BPS_U..],
        vget_lane_u32::<1>(vreinterpret_u32_u8(dst23_u8)),
    );
}

// upstream: src/dsp/enc_neon.c Add4x4_NEON
#[target_feature(enable = "neon")]
fn add4x4_neon(row01: int16x8_t, row23: int16x8_t, ref_: &[u8], dst: &mut [u8]) {
    let mut dst01 = vdup_n_u32(0);
    let mut dst23 = vdup_n_u32(0);

    // Load the source pixels.
    dst01 = vset_lane_u32::<0>(load_u32_ne(&ref_[0..]), dst01);
    dst23 = vset_lane_u32::<0>(load_u32_ne(&ref_[2 * BPS_U..]), dst23);
    dst01 = vset_lane_u32::<1>(load_u32_ne(&ref_[BPS_U..]), dst01);
    dst23 = vset_lane_u32::<1>(load_u32_ne(&ref_[3 * BPS_U..]), dst23);

    {
        // Convert to 16b.
        let dst01_s16 = convert_u8_to_s16_neon(dst01);
        let dst23_s16 = convert_u8_to_s16_neon(dst23);

        // Descale with rounding.
        let out01 = vrsraq_n_s16::<3>(dst01_s16, row01);
        let out23 = vrsraq_n_s16::<3>(dst23_s16, row23);
        // Add the inverse transform.
        saturate_and_store4x4_neon(dst, out01, out23);
    }
}

// upstream: src/dsp/enc_neon.c Transpose8x2_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn transpose8x2_neon(in0: int16x8_t, in1: int16x8_t) -> int16x8x2_t {
    // a0 a1 a2 a3 | b0 b1 b2 b3   => a0 b0 c0 d0 | a1 b1 c1 d1
    // c0 c1 c2 c3 | d0 d1 d2 d3      a2 b2 c2 d2 | a3 b3 c3 d3
    let tmp0 = vzipq_s16(in0, in1); // a0 c0 a1 c1 a2 c2 ...
    // b0 d0 b1 d1 b2 d2 ...
    vzipq_s16(tmp0.0, tmp0.1)
}

// upstream: src/dsp/enc_neon.c TransformPass_NEON
#[target_feature(enable = "neon")]
#[expect(
    non_snake_case,
    reason = "the C names B1, C0, C1, D0, D1, E0, E_tmp and E1"
)]
fn transform_pass_neon(rows: &mut int16x8x2_t) {
    // {rows} = in0 | in4
    //          in8 | in12
    // B1 = in4 | in12
    let B1 = vcombine_s16(vget_high_s16(rows.0), vget_high_s16(rows.1));
    // C0 = kC1 * in4 | kC1 * in12
    // C1 = kC2 * in4 | kC2 * in12
    let C0 = vsraq_n_s16::<1>(B1, vqdmulhq_n_s16(B1, C1));
    let C1_ = vqdmulhq_n_s16(B1, C2);
    let a = vqadd_s16(vget_low_s16(rows.0), vget_low_s16(rows.1)); // in0 + in8
    let b = vqsub_s16(vget_low_s16(rows.0), vget_low_s16(rows.1)); // in0 - in8
    // c = kC2 * in4 - kC1 * in12
    // d = kC1 * in4 + kC2 * in12
    let c = vqsub_s16(vget_low_s16(C1_), vget_high_s16(C0));
    let d = vqadd_s16(vget_low_s16(C0), vget_high_s16(C1_));
    let D0 = vcombine_s16(a, b); // D0 = a | b
    let D1 = vcombine_s16(d, c); // D1 = d | c
    let E0 = vqaddq_s16(D0, D1); // a+d | b+c
    let E_tmp = vqsubq_s16(D0, D1); // a-d | b-c
    let E1 = vcombine_s16(vget_high_s16(E_tmp), vget_low_s16(E_tmp));
    *rows = transpose8x2_neon(E0, E1);
}

// upstream: src/dsp/enc_neon.c ITransformOne_NEON
#[target_feature(enable = "neon")]
fn i_transform_one_neon(ref_: &[u8], input: &[i16], dst: &mut [u8]) {
    let mut rows = int16x8x2_t(vld1q_s16(&input[0..]), vld1q_s16(&input[8..]));
    transform_pass_neon(&mut rows);
    transform_pass_neon(&mut rows);
    add4x4_neon(rows.0, rows.1, ref_, dst);
}

// upstream: src/dsp/enc_neon.c ITransform_NEON
#[target_feature(enable = "neon")]
fn i_transform_neon(ref_: &[u8], input: &[i16], dst: &mut [u8], do_two: bool) {
    i_transform_one_neon(ref_, input, dst);
    if do_two {
        i_transform_one_neon(&ref_[4..], &input[16..], &mut dst[4..]);
    }
}

/// Entry point of `ITransform_NEON` for `VP8ITransform`.
pub(crate) fn i_transform(ref_: &[u8], input: &[i16], dst: &mut [u8], do_two: bool) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { i_transform_neon(ref_, input, dst, do_two) }
}

// upstream: src/dsp/enc_neon.c Load4x4_NEON
/// Load all 4x4 pixels into a single `uint8x16_t` variable.
#[target_feature(enable = "neon")]
#[must_use]
fn load4x4_neon(src: &[u8]) -> uint8x16_t {
    let mut out = vdupq_n_u32(0);
    out = vsetq_lane_u32::<0>(load_u32_ne(&src[0..]), out);
    out = vsetq_lane_u32::<1>(load_u32_ne(&src[BPS_U..]), out);
    out = vsetq_lane_u32::<2>(load_u32_ne(&src[2 * BPS_U..]), out);
    out = vsetq_lane_u32::<3>(load_u32_ne(&src[3 * BPS_U..]), out);
    vreinterpretq_u8_u32(out)
}

// Forward transform.

// upstream: src/dsp/enc_neon.c Transpose4x4_S16_NEON
/// Returns `(out01, out32)`.
#[target_feature(enable = "neon")]
#[must_use]
#[expect(non_snake_case, reason = "the C names A, B, C, D, AB and CD")]
fn transpose4x4_s16_neon(
    A: int16x4_t,
    B: int16x4_t,
    C: int16x4_t,
    D: int16x4_t,
) -> (int16x8_t, int16x8_t) {
    let AB = vtrn_s16(A, B);
    let CD = vtrn_s16(C, D);
    let tmp02 = vtrn_s32(vreinterpret_s32_s16(AB.0), vreinterpret_s32_s16(CD.0));
    let tmp13 = vtrn_s32(vreinterpret_s32_s16(AB.1), vreinterpret_s32_s16(CD.1));
    let out01 = vreinterpretq_s16_s64(vcombine_s64(
        vreinterpret_s64_s32(tmp02.0),
        vreinterpret_s64_s32(tmp13.0),
    ));
    let out32 = vreinterpretq_s16_s64(vcombine_s64(
        vreinterpret_s64_s32(tmp13.1),
        vreinterpret_s64_s32(tmp02.1),
    ));
    (out01, out32)
}

// upstream: src/dsp/enc_neon.c DiffU8ToS16_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn diff_u8_to_s16_neon(a: uint8x8_t, b: uint8x8_t) -> int16x8_t {
    vreinterpretq_s16_u16(vsubl_u8(a, b))
}

// upstream: src/dsp/enc_neon.c FTransform_NEON
#[target_feature(enable = "neon")]
#[expect(
    non_snake_case,
    reason = "the C names S0, R0, D0D1, D2D3, D0 to D3 and the kCst constants"
)]
#[expect(clippy::similar_names, reason = "the C names a2_p_a3 and a3_m_a2")]
fn f_transform_neon(src: &[u8], ref_: &[u8], out: &mut [i16]) {
    // working 4x4 int16 variables
    let (mut d0d1, mut d3d2);
    {
        let S0 = load4x4_neon(src);
        let R0 = load4x4_neon(ref_);
        let D0D1 = diff_u8_to_s16_neon(vget_low_u8(S0), vget_low_u8(R0));
        let D2D3 = diff_u8_to_s16_neon(vget_high_u8(S0), vget_high_u8(R0));
        let D0 = vget_low_s16(D0D1);
        let D1 = vget_high_s16(D0D1);
        let D2 = vget_low_s16(D2D3);
        let D3 = vget_high_s16(D2D3);
        (d0d1, d3d2) = transpose4x4_s16_neon(D0, D1, D2, D3);
    }
    {
        // 1rst pass
        let kCst937 = vdupq_n_s32(937);
        let kCst1812 = vdupq_n_s32(1812);
        let a0a1 = vaddq_s16(d0d1, d3d2); // d0+d3 | d1+d2   (=a0|a1)
        let a3a2 = vsubq_s16(d0d1, d3d2); // d0-d3 | d1-d2   (=a3|a2)
        let a0a1_2 = vshlq_n_s16::<3>(a0a1);
        let tmp0 = vadd_s16(vget_low_s16(a0a1_2), vget_high_s16(a0a1_2));
        let tmp2 = vsub_s16(vget_low_s16(a0a1_2), vget_high_s16(a0a1_2));
        let a3_2217 = vmull_n_s16(vget_low_s16(a3a2), 2217);
        let a2_2217 = vmull_n_s16(vget_high_s16(a3a2), 2217);
        let a2_p_a3 = vmlal_n_s16(a2_2217, vget_low_s16(a3a2), 5352);
        let a3_m_a2 = vmlsl_n_s16(a3_2217, vget_high_s16(a3a2), 5352);
        let tmp1 = vshrn_n_s32::<9>(vaddq_s32(a2_p_a3, kCst1812));
        let tmp3 = vshrn_n_s32::<9>(vaddq_s32(a3_m_a2, kCst937));
        (d0d1, d3d2) = transpose4x4_s16_neon(tmp0, tmp1, tmp2, tmp3);
    }
    {
        // 2nd pass
        // the (1<<16) addition is for the replacement: a3!=0  <-> 1-(a3==0)
        let kCst12000 = vdupq_n_s32(12000 + (1 << 16));
        let kCst51000 = vdupq_n_s32(51000);
        let a0a1 = vaddq_s16(d0d1, d3d2); // d0+d3 | d1+d2   (=a0|a1)
        let a3a2 = vsubq_s16(d0d1, d3d2); // d0-d3 | d1-d2   (=a3|a2)
        let a0_k7 = vadd_s16(vget_low_s16(a0a1), vdup_n_s16(7));
        let out0 = vshr_n_s16::<4>(vadd_s16(a0_k7, vget_high_s16(a0a1)));
        let out2 = vshr_n_s16::<4>(vsub_s16(a0_k7, vget_high_s16(a0a1)));
        let a3_2217 = vmull_n_s16(vget_low_s16(a3a2), 2217);
        let a2_2217 = vmull_n_s16(vget_high_s16(a3a2), 2217);
        let a2_p_a3 = vmlal_n_s16(a2_2217, vget_low_s16(a3a2), 5352);
        let a3_m_a2 = vmlsl_n_s16(a3_2217, vget_high_s16(a3a2), 5352);
        let tmp1 = vaddhn_s32(a2_p_a3, kCst12000);
        let out3 = vaddhn_s32(a3_m_a2, kCst51000);
        let a3_eq_0 = vreinterpret_s16_u16(vceq_s16(vget_low_s16(a3a2), vdup_n_s16(0)));
        let out1 = vadd_s16(tmp1, a3_eq_0);
        vst1_s16(&mut out[0..], out0);
        vst1_s16(&mut out[4..], out1);
        vst1_s16(&mut out[8..], out2);
        vst1_s16(&mut out[12..], out3);
    }
}

/// Entry point of `FTransform_NEON` for `VP8FTransform`.
pub(crate) fn f_transform(src: &[u8], ref_: &[u8], out: &mut [i16]) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { f_transform_neon(src, ref_, out) }
}

// upstream: src/dsp/enc_neon.c LOAD_LANE_16b
/// Loads `src[*pos]` into lane `LANE` of `value` and steps `*pos` by `stride`.
#[target_feature(enable = "neon")]
#[must_use]
fn load_lane_16b<const LANE: i32>(
    value: int16x4_t,
    src: &[i16],
    pos: &mut usize,
    stride: usize,
) -> int16x4_t {
    let value = vset_lane_s16::<LANE>(src[*pos], value);
    *pos += stride;
    value
}

// upstream: src/dsp/enc_neon.c FTransformWHT_NEON
#[target_feature(enable = "neon")]
fn f_transform_wht_neon(src: &[i16], out: &mut [i16]) {
    let stride = 16;
    let zero = vdup_n_s16(0);
    let mut tmp0 = int32x4x4_t(
        vdupq_n_s32(0),
        vdupq_n_s32(0),
        vdupq_n_s32(0),
        vdupq_n_s32(0),
    );
    let mut in_ = int16x4x4_t(zero, zero, zero, zero);
    let mut pos = 0;
    in_.0 = load_lane_16b::<0>(in_.0, src, &mut pos, stride);
    in_.1 = load_lane_16b::<0>(in_.1, src, &mut pos, stride);
    in_.2 = load_lane_16b::<0>(in_.2, src, &mut pos, stride);
    in_.3 = load_lane_16b::<0>(in_.3, src, &mut pos, stride);
    in_.0 = load_lane_16b::<1>(in_.0, src, &mut pos, stride);
    in_.1 = load_lane_16b::<1>(in_.1, src, &mut pos, stride);
    in_.2 = load_lane_16b::<1>(in_.2, src, &mut pos, stride);
    in_.3 = load_lane_16b::<1>(in_.3, src, &mut pos, stride);
    in_.0 = load_lane_16b::<2>(in_.0, src, &mut pos, stride);
    in_.1 = load_lane_16b::<2>(in_.1, src, &mut pos, stride);
    in_.2 = load_lane_16b::<2>(in_.2, src, &mut pos, stride);
    in_.3 = load_lane_16b::<2>(in_.3, src, &mut pos, stride);
    in_.0 = load_lane_16b::<3>(in_.0, src, &mut pos, stride);
    in_.1 = load_lane_16b::<3>(in_.1, src, &mut pos, stride);
    in_.2 = load_lane_16b::<3>(in_.2, src, &mut pos, stride);
    in_.3 = load_lane_16b::<3>(in_.3, src, &mut pos, stride);

    {
        // a0 = in[0 * 16] + in[2 * 16]
        // a1 = in[1 * 16] + in[3 * 16]
        // a2 = in[1 * 16] - in[3 * 16]
        // a3 = in[0 * 16] - in[2 * 16]
        let a0 = vaddl_s16(in_.0, in_.2);
        let a1 = vaddl_s16(in_.1, in_.3);
        let a2 = vsubl_s16(in_.1, in_.3);
        let a3 = vsubl_s16(in_.0, in_.2);
        tmp0.0 = vaddq_s32(a0, a1);
        tmp0.1 = vaddq_s32(a3, a2);
        tmp0.2 = vsubq_s32(a3, a2);
        tmp0.3 = vsubq_s32(a0, a1);
    }
    {
        let tmp1 = transpose4x4_neon(tmp0);
        // a0 = tmp[0 + i] + tmp[ 8 + i]
        // a1 = tmp[4 + i] + tmp[12 + i]
        // a2 = tmp[4 + i] - tmp[12 + i]
        // a3 = tmp[0 + i] - tmp[ 8 + i]
        let a0 = vaddq_s32(tmp1.0, tmp1.2);
        let a1 = vaddq_s32(tmp1.1, tmp1.3);
        let a2 = vsubq_s32(tmp1.1, tmp1.3);
        let a3 = vsubq_s32(tmp1.0, tmp1.2);
        let b0 = vhaddq_s32(a0, a1); // (a0 + a1) >> 1
        let b1 = vhaddq_s32(a3, a2); // (a3 + a2) >> 1
        let b2 = vhsubq_s32(a3, a2); // (a3 - a2) >> 1
        let b3 = vhsubq_s32(a0, a1); // (a0 - a1) >> 1
        let out0 = vmovn_s32(b0);
        let out1 = vmovn_s32(b1);
        let out2 = vmovn_s32(b2);
        let out3 = vmovn_s32(b3);

        vst1_s16(&mut out[0..], out0);
        vst1_s16(&mut out[4..], out1);
        vst1_s16(&mut out[8..], out2);
        vst1_s16(&mut out[12..], out3);
    }
}

/// Entry point of `FTransformWHT_NEON` for `VP8FTransformWHT`.
pub(crate) fn f_transform_wht(input: &[i16], out: &mut [i16]) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { f_transform_wht_neon(input, out) }
}

//------------------------------------------------------------------------------
// Texture distortion
//
// We try to match the spectral content (weighted) between source and
// reconstructed samples.

// a 0123, b 0123
// a 4567, b 4567
// a 89ab, b 89ab
// a cdef, b cdef
//
// transpose
//
// a 048c, b 048c
// a 159d, b 159d
// a 26ae, b 26ae
// a 37bf, b 37bf
//

// upstream: src/dsp/enc_neon.c DistoTranspose4x4S16_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn disto_transpose4x4s16_neon(mut q4_in: int16x8x4_t) -> int16x8x4_t {
    let q2_tmp0 = vtrnq_s16(q4_in.0, q4_in.1);
    let q2_tmp1 = vtrnq_s16(q4_in.2, q4_in.3);
    let q2_tmp2 = vtrnq_s32(
        vreinterpretq_s32_s16(q2_tmp0.0),
        vreinterpretq_s32_s16(q2_tmp1.0),
    );
    let q2_tmp3 = vtrnq_s32(
        vreinterpretq_s32_s16(q2_tmp0.1),
        vreinterpretq_s32_s16(q2_tmp1.1),
    );
    q4_in.0 = vreinterpretq_s16_s32(q2_tmp2.0);
    q4_in.2 = vreinterpretq_s16_s32(q2_tmp2.1);
    q4_in.1 = vreinterpretq_s16_s32(q2_tmp3.0);
    q4_in.3 = vreinterpretq_s16_s32(q2_tmp3.1);
    q4_in
}

// upstream: src/dsp/enc_neon.c DistoHorizontalPass_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn disto_horizontal_pass_neon(q4_in: int16x8x4_t) -> int16x8x4_t {
    // {a0, a1} = {in[0] + in[2], in[1] + in[3]}
    // {a3, a2} = {in[0] - in[2], in[1] - in[3]}
    let q_a0 = vaddq_s16(q4_in.0, q4_in.2);
    let q_a1 = vaddq_s16(q4_in.1, q4_in.3);
    let q_a3 = vsubq_s16(q4_in.0, q4_in.2);
    let q_a2 = vsubq_s16(q4_in.1, q4_in.3);
    // tmp[0] = a0 + a1
    // tmp[1] = a3 + a2
    // tmp[2] = a3 - a2
    // tmp[3] = a0 - a1
    int16x8x4_t(
        vabsq_s16(vaddq_s16(q_a0, q_a1)),
        vabsq_s16(vaddq_s16(q_a3, q_a2)),
        vabdq_s16(q_a3, q_a2),
        vabdq_s16(q_a0, q_a1),
    )
}

// upstream: src/dsp/enc_neon.c DistoVerticalPass_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn disto_vertical_pass_neon(q4_in: uint8x8x4_t) -> int16x8x4_t {
    let q_a0 = vreinterpretq_s16_u16(vaddl_u8(q4_in.0, q4_in.2));
    let q_a1 = vreinterpretq_s16_u16(vaddl_u8(q4_in.1, q4_in.3));
    let q_a2 = vreinterpretq_s16_u16(vsubl_u8(q4_in.1, q4_in.3));
    let q_a3 = vreinterpretq_s16_u16(vsubl_u8(q4_in.0, q4_in.2));
    int16x8x4_t(
        vaddq_s16(q_a0, q_a1),
        vaddq_s16(q_a3, q_a2),
        vsubq_s16(q_a3, q_a2),
        vsubq_s16(q_a0, q_a1),
    )
}

// upstream: src/dsp/enc_neon.c DistoLoadW_NEON
#[target_feature(enable = "neon")]
#[must_use]
#[expect(clippy::similar_names, reason = "the C names q_w07 and q_w8f")]
fn disto_load_w_neon(w: &[u16; 16]) -> int16x4x4_t {
    let q_w07 = vld1q_u16(&w[0..]);
    let q_w8f = vld1q_u16(&w[8..]);
    int16x4x4_t(
        vget_low_s16(vreinterpretq_s16_u16(q_w07)),
        vget_high_s16(vreinterpretq_s16_u16(q_w07)),
        vget_low_s16(vreinterpretq_s16_u16(q_w8f)),
        vget_high_s16(vreinterpretq_s16_u16(q_w8f)),
    )
}

// upstream: src/dsp/enc_neon.c DistoSum_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn disto_sum_neon(q4_in: int16x8x4_t, d4_w: int16x4x4_t) -> int32x2_t {
    // sum += w[ 0] * abs(b0);
    // sum += w[ 4] * abs(b1);
    // sum += w[ 8] * abs(b2);
    // sum += w[12] * abs(b3);
    let mut q_sum0 = vmull_s16(d4_w.0, vget_low_s16(q4_in.0));
    let mut q_sum1 = vmull_s16(d4_w.1, vget_low_s16(q4_in.1));
    let mut q_sum2 = vmull_s16(d4_w.2, vget_low_s16(q4_in.2));
    let mut q_sum3 = vmull_s16(d4_w.3, vget_low_s16(q4_in.3));
    q_sum0 = vmlsl_s16(q_sum0, d4_w.0, vget_high_s16(q4_in.0));
    q_sum1 = vmlsl_s16(q_sum1, d4_w.1, vget_high_s16(q4_in.1));
    q_sum2 = vmlsl_s16(q_sum2, d4_w.2, vget_high_s16(q4_in.2));
    q_sum3 = vmlsl_s16(q_sum3, d4_w.3, vget_high_s16(q4_in.3));

    q_sum0 = vaddq_s32(q_sum0, q_sum1);
    q_sum2 = vaddq_s32(q_sum2, q_sum3);
    q_sum2 = vaddq_s32(q_sum0, q_sum2);
    let mut d_sum = vpadd_s32(vget_low_s32(q_sum2), vget_high_s32(q_sum2));
    d_sum = vpadd_s32(d_sum, d_sum);
    d_sum
}

// upstream: src/dsp/enc_neon.c LOAD_LANE_32b
/// Loads the 4 bytes at the start of `src` into lane `LANE` of `value`.
#[target_feature(enable = "neon")]
#[must_use]
fn load_lane_32b<const LANE: i32>(src: &[u8], value: uint32x2_t) -> uint32x2_t {
    vset_lane_u32::<LANE>(load_u32_ne(src), value)
}

// upstream: src/dsp/enc_neon.c Disto4x4_NEON
/// Hadamard transform. Returns the weighted sum of the absolute value of
/// transformed coefficients. `w[]` contains a row-major 4 by 4 symmetric
/// matrix.
#[target_feature(enable = "neon")]
#[must_use]
fn disto4x4_neon(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    let mut d_in_ab_0123 = vdup_n_u32(0);
    let mut d_in_ab_4567 = vdup_n_u32(0);
    let mut d_in_ab_89ab = vdup_n_u32(0);
    let mut d_in_ab_cdef = vdup_n_u32(0);

    // load data a, b
    d_in_ab_0123 = load_lane_32b::<0>(&a[0..], d_in_ab_0123);
    d_in_ab_4567 = load_lane_32b::<0>(&a[BPS_U..], d_in_ab_4567);
    d_in_ab_89ab = load_lane_32b::<0>(&a[2 * BPS_U..], d_in_ab_89ab);
    d_in_ab_cdef = load_lane_32b::<0>(&a[3 * BPS_U..], d_in_ab_cdef);
    d_in_ab_0123 = load_lane_32b::<1>(&b[0..], d_in_ab_0123);
    d_in_ab_4567 = load_lane_32b::<1>(&b[BPS_U..], d_in_ab_4567);
    d_in_ab_89ab = load_lane_32b::<1>(&b[2 * BPS_U..], d_in_ab_89ab);
    d_in_ab_cdef = load_lane_32b::<1>(&b[3 * BPS_U..], d_in_ab_cdef);
    let d4_in = uint8x8x4_t(
        vreinterpret_u8_u32(d_in_ab_0123),
        vreinterpret_u8_u32(d_in_ab_4567),
        vreinterpret_u8_u32(d_in_ab_89ab),
        vreinterpret_u8_u32(d_in_ab_cdef),
    );

    {
        // Vertical pass first to avoid a transpose (vertical and horizontal passes
        // are commutative because w/kWeightY is symmetric) and subsequent
        // transpose.
        let q4_v = disto_vertical_pass_neon(d4_in);
        let d4_w = disto_load_w_neon(w);
        // horizontal pass
        let q4_t = disto_transpose4x4s16_neon(q4_v);
        let q4_h = disto_horizontal_pass_neon(q4_t);
        let mut d_sum = disto_sum_neon(q4_h, d4_w);

        // abs(sum2 - sum1) >> 5
        d_sum = vabs_s32(d_sum);
        d_sum = vshr_n_s32::<5>(d_sum);
        vget_lane_s32::<0>(d_sum)
    }
}

/// Entry point of `Disto4x4_NEON` for `VP8TDisto4x4`.
#[must_use]
pub(crate) fn disto4x4(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { disto4x4_neon(a, b, w) }
}

// upstream: src/dsp/enc_neon.c Disto16x16_NEON
#[target_feature(enable = "neon")]
#[must_use]
#[expect(non_snake_case, reason = "the C name D")]
fn disto16x16_neon(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    let mut D = 0;
    for y in (0..16 * BPS_U).step_by(4 * BPS_U) {
        for x in (0..16).step_by(4) {
            D += disto4x4_neon(&a[x + y..], &b[x + y..], w);
        }
    }
    D
}

/// Entry point of `Disto16x16_NEON` for `VP8TDisto16x16`.
#[must_use]
pub(crate) fn disto16x16(a: &[u8], b: &[u8], w: &[u16; 16]) -> i32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { disto16x16_neon(a, b, w) }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/enc_neon.c CollectHistogram_NEON
#[target_feature(enable = "neon")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "MAX_COEFF_THRESH is 31, and the block offsets and bins are non-negative"
)]
fn collect_histogram_neon(
    ref_: &[u8],
    pred: &[u8],
    start_block: i32,
    end_block: i32,
    histo: &mut VP8Histogram,
) {
    let max_coeff_thresh = vdupq_n_u16(MAX_COEFF_THRESH as u16);
    let mut distribution = [0_i32; MAX_COEFF_THRESH as usize + 1];
    for j in start_block..end_block {
        let mut out = [0_i16; 16];
        let scan = VP8_DSP_SCAN[j as usize] as usize;
        f_transform_neon(&ref_[scan..], &pred[scan..], &mut out);
        {
            let a0 = vld1q_s16(&out[0..]);
            let b0 = vld1q_s16(&out[8..]);
            let a1 = vreinterpretq_u16_s16(vabsq_s16(a0));
            let b1 = vreinterpretq_u16_s16(vabsq_s16(b0));
            let a2 = vshrq_n_u16::<3>(a1);
            let b2 = vshrq_n_u16::<3>(b1);
            let a3 = vminq_u16(a2, max_coeff_thresh);
            let b3 = vminq_u16(b2, max_coeff_thresh);
            vst1q_s16(&mut out[0..], vreinterpretq_s16_u16(a3));
            vst1q_s16(&mut out[8..], vreinterpretq_s16_u16(b3));
            // Convert coefficients to bin.
            for &bin in &out {
                distribution[bin as usize] += 1;
            }
        }
    }
    vp8_set_histogram_data(&distribution, histo);
}

/// Entry point of `CollectHistogram_NEON` for `VP8CollectHistogram`.
pub(crate) fn collect_histogram(
    ref_: &[u8],
    pred: &[u8],
    start_block: i32,
    end_block: i32,
    histo: &mut VP8Histogram,
) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { collect_histogram_neon(ref_, pred, start_block, end_block, histo) }
}

//------------------------------------------------------------------------------

// upstream: src/dsp/enc_neon.c AccumulateSSE16_NEON
#[target_feature(enable = "neon")]
fn accumulate_sse16_neon(a: &[u8], b: &[u8], sum: &mut uint32x4_t) {
    let a0 = vld1q_u8(a);
    let b0 = vld1q_u8(b);
    let abs_diff = vabdq_u8(a0, b0);
    let prod1 = vmull_u8(vget_low_u8(abs_diff), vget_low_u8(abs_diff));
    let prod2 = vmull_u8(vget_high_u8(abs_diff), vget_high_u8(abs_diff));
    /* pair-wise adds and widen */
    let sum1 = vpaddlq_u16(prod1);
    let sum2 = vpaddlq_u16(prod2);
    *sum = vaddq_u32(*sum, vaddq_u32(sum1, sum2));
}

// upstream: src/dsp/enc_neon.c SumToInt_NEON
/// Horizontal sum of all four `uint32_t` values in `sum`.
#[target_feature(enable = "neon")]
#[must_use]
#[expect(clippy::cast_possible_wrap, reason = "C casts the uint32_t sum to int")]
fn sum_to_int_neon(sum: uint32x4_t) -> i32 {
    vaddvq_u32(sum) as i32
}

// upstream: src/dsp/enc_neon.c SSE16x16_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn sse16x16_neon(a: &[u8], b: &[u8]) -> i32 {
    let mut sum = vdupq_n_u32(0);
    for y in 0..16 {
        accumulate_sse16_neon(&a[y * BPS_U..], &b[y * BPS_U..], &mut sum);
    }
    sum_to_int_neon(sum)
}

/// Entry point of `SSE16x16_NEON` for `VP8SSE16x16`.
#[must_use]
pub(crate) fn sse16x16(a: &[u8], b: &[u8]) -> i32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { sse16x16_neon(a, b) }
}

// upstream: src/dsp/enc_neon.c SSE16x8_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn sse16x8_neon(a: &[u8], b: &[u8]) -> i32 {
    let mut sum = vdupq_n_u32(0);
    for y in 0..8 {
        accumulate_sse16_neon(&a[y * BPS_U..], &b[y * BPS_U..], &mut sum);
    }
    sum_to_int_neon(sum)
}

/// Entry point of `SSE16x8_NEON` for `VP8SSE16x8`.
#[must_use]
pub(crate) fn sse16x8(a: &[u8], b: &[u8]) -> i32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { sse16x8_neon(a, b) }
}

// upstream: src/dsp/enc_neon.c SSE8x8_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn sse8x8_neon(a: &[u8], b: &[u8]) -> i32 {
    let mut sum = vdupq_n_u32(0);
    for y in 0..8 {
        let a0 = vld1_u8(&a[y * BPS_U..]);
        let b0 = vld1_u8(&b[y * BPS_U..]);
        let abs_diff = vabd_u8(a0, b0);
        let prod = vmull_u8(abs_diff, abs_diff);
        sum = vpadalq_u16(sum, prod);
    }
    sum_to_int_neon(sum)
}

/// Entry point of `SSE8x8_NEON` for `VP8SSE8x8`.
#[must_use]
pub(crate) fn sse8x8(a: &[u8], b: &[u8]) -> i32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { sse8x8_neon(a, b) }
}

// upstream: src/dsp/enc_neon.c SSE4x4_NEON
#[target_feature(enable = "neon")]
#[must_use]
fn sse4x4_neon(a: &[u8], b: &[u8]) -> i32 {
    let a0 = load4x4_neon(a);
    let b0 = load4x4_neon(b);
    let abs_diff = vabdq_u8(a0, b0);
    let prod1 = vmull_u8(vget_low_u8(abs_diff), vget_low_u8(abs_diff));
    let prod2 = vmull_u8(vget_high_u8(abs_diff), vget_high_u8(abs_diff));
    /* pair-wise adds and widen */
    let sum1 = vpaddlq_u16(prod1);
    let sum2 = vpaddlq_u16(prod2);
    sum_to_int_neon(vaddq_u32(sum1, sum2))
}

/// Entry point of `SSE4x4_NEON` for `VP8SSE4x4`.
#[must_use]
pub(crate) fn sse4x4(a: &[u8], b: &[u8]) -> i32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { sse4x4_neon(a, b) }
}

//------------------------------------------------------------------------------

// Compilation with gcc-4.6.x is problematic for now.

// upstream: src/dsp/enc_neon.c Quantize_NEON
#[target_feature(enable = "neon")]
#[must_use]
#[expect(clippy::cast_possible_truncation, reason = "MAX_LEVEL is 2047")]
fn quantize_neon(input: &mut [i16], mtx: &VP8Matrix, offset: usize) -> int16x8_t {
    let sharp = vld1q_u16(&mtx.sharpen[offset..]);
    let q = vld1q_u16(&mtx.q[offset..]);
    let iq = vld1q_u16(&mtx.iq[offset..]);
    let bias0 = vld1q_u32(&mtx.bias[offset..]);
    let bias1 = vld1q_u32(&mtx.bias[offset + 4..]);

    let a = vld1q_s16(&input[offset..]); // in
    let b = vreinterpretq_u16_s16(vabsq_s16(a)); // coeff = abs(in)
    let sign = vshrq_n_s16::<15>(a); // sign
    let c = vaddq_u16(b, sharp); // + sharpen
    let m0 = vmull_u16(vget_low_u16(c), vget_low_u16(iq));
    let m1 = vmull_u16(vget_high_u16(c), vget_high_u16(iq));
    let m2 = vhaddq_u32(m0, bias0);
    let m3 = vhaddq_u32(m1, bias1); // (coeff * iQ + bias) >> 1
    let c0 = vcombine_u16(vshrn_n_u32::<16>(m2), vshrn_n_u32::<16>(m3)); // QFIX=17 = 16+1
    let c1 = vminq_u16(c0, vdupq_n_u16(MAX_LEVEL as u16));
    let c2 = veorq_s16(vreinterpretq_s16_u16(c1), sign);
    let c3 = vsubq_s16(c2, sign); // restore sign
    let c4 = vmulq_s16(c3, vreinterpretq_s16_u16(q));
    vst1q_s16(&mut input[offset..], c4);
    const { assert!(QFIX == 17) }; // this function can't work as is if QFIX != 16+1
    c3
}

// upstream: src/dsp/enc_neon.c kShuffles
static SHUFFLES: [[u8; 8]; 4] = [
    [0, 1, 2, 3, 8, 9, 16, 17],
    [10, 11, 4, 5, 6, 7, 12, 13],
    [18, 19, 24, 25, 26, 27, 20, 21],
    [14, 15, 22, 23, 28, 29, 30, 31],
];

// upstream: src/dsp/enc_neon.c QuantizeBlock_NEON
#[target_feature(enable = "neon")]
fn quantize_block_neon(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    let out0 = quantize_neon(input, mtx, 0);
    let out1 = quantize_neon(input, mtx, 8);
    // vtbl?_u8 are marked unavailable for iOS arm64 with Xcode < 6.3, use
    // non-standard versions there.
    let all_out = uint8x8x4_t(
        vreinterpret_u8_s16(vget_low_s16(out0)),
        vreinterpret_u8_s16(vget_high_s16(out0)),
        vreinterpret_u8_s16(vget_low_s16(out1)),
        vreinterpret_u8_s16(vget_high_s16(out1)),
    );
    let shuffles = uint8x8x4_t(
        vtbl4_u8(all_out, vld1_u8(&SHUFFLES[0])),
        vtbl4_u8(all_out, vld1_u8(&SHUFFLES[1])),
        vtbl4_u8(all_out, vld1_u8(&SHUFFLES[2])),
        vtbl4_u8(all_out, vld1_u8(&SHUFFLES[3])),
    );
    // Zigzag reordering
    vst1_u8_halfwords(&mut out[0..], shuffles.0);
    vst1_u8_halfwords(&mut out[4..], shuffles.1);
    vst1_u8_halfwords(&mut out[8..], shuffles.2);
    vst1_u8_halfwords(&mut out[12..], shuffles.3);
    // test zeros
    if out[0..4].iter().any(|&v| v != 0) {
        return 1;
    }
    if out[4..8].iter().any(|&v| v != 0) {
        return 1;
    }
    if out[8..12].iter().any(|&v| v != 0) {
        return 1;
    }
    if out[12..16].iter().any(|&v| v != 0) {
        return 1;
    }
    0
}

/// Entry point of `QuantizeBlock_NEON` for `VP8EncQuantizeBlock` and
/// `VP8EncQuantizeBlockWHT`.
pub(crate) fn quantize_block(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { quantize_block_neon(input, out, mtx) }
}

// upstream: src/dsp/enc_neon.c Quantize2Blocks_NEON
#[target_feature(enable = "neon")]
#[expect(clippy::identity_op, reason = "the C shift by 0 of the first block")]
fn quantize2_blocks_neon(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    let mut nz = quantize_block_neon(&mut input[..16], &mut out[..16], mtx) << 0;
    nz |= quantize_block_neon(&mut input[16..32], &mut out[16..32], mtx) << 1;
    nz
}

/// Entry point of `Quantize2Blocks_NEON` for `VP8EncQuantize2Blocks`.
pub(crate) fn quantize2_blocks(input: &mut [i16], out: &mut [i16], mtx: &VP8Matrix) -> i32 {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { quantize2_blocks_neon(input, out, mtx) }
}

//------------------------------------------------------------------------------
// Intra predictions (WEBP_AARCH64)

// upstream: src/dsp/enc_neon.c DC4_VE4_HE4_TM4_NEON
#[target_feature(enable = "neon")]
fn dc4_ve4_he4_tm4_neon<const LANE: i32>(
    dst: &mut [u8],
    qcombined: uint8x16x2_t,
    tbl: uint8x16_t,
    res: uint8x8_t,
) {
    let mut r = vqtbl2q_u8(qcombined, tbl);
    r = vreinterpretq_u8_u32(vsetq_lane_u32::<1>(
        vget_lane_u32::<LANE>(vreinterpret_u32_u8(res)),
        vreinterpretq_u32_u8(r),
    ));
    vst1q_u8(dst, r);
}

// upstream: src/dsp/enc_neon.c RD4_VR4_LD4_VL4_NEON
#[target_feature(enable = "neon")]
fn rd4_vr4_ld4_vl4_neon(dst: &mut [u8], qcombined: uint8x16x2_t, tbl: uint8x16_t) {
    let r = vqtbl2q_u8(qcombined, tbl);
    vst1q_u8(dst, r);
}

// upstream: src/dsp/enc_neon.c Vld1U8x2
#[target_feature(enable = "neon")]
#[must_use]
fn vld1u8x2(ptr: &[u8]) -> uint8x8x2_t {
    vld1_u8_x2(ptr)
}

// upstream: src/dsp/enc_neon.c Vld1qU8x4
#[target_feature(enable = "neon")]
#[must_use]
fn vld1q_u8x4(ptr: &[u8]) -> uint8x16x4_t {
    vld1q_u8_x4(ptr)
}

// upstream: src/dsp/enc_neon.c kLookupTbl1
// The static table of Intra4Preds_NEON.
//  0   1   2   3   4   5   6   7   8   9  10  11  12  13
//      L   K   J   I   X   A   B   C   D   E   F   G   H
//     -5  -4  -3  -2  -1   0   1   2   3   4   5   6   7
static LOOKUP_TBL1: [u8; 64] = [
    0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 12, 12, //
    3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, //
    4, 20, 21, 22, 3, 18, 2, 17, 3, 19, 4, 20, 2, 17, 1, 16, //
    2, 18, 3, 19, 1, 16, 31, 31, 1, 17, 2, 18, 31, 31, 31, 31,
];

// upstream: src/dsp/enc_neon.c kLookupTbl2
// The static table of Intra4Preds_NEON.
static LOOKUP_TBL2: [u8; 64] = [
    20, 21, 22, 23, 5, 6, 7, 8, 22, 23, 24, 25, 6, 7, 8, 9, //
    19, 20, 21, 22, 20, 21, 22, 23, 23, 24, 25, 26, 22, 23, 24, 25, //
    18, 19, 20, 21, 19, 5, 6, 7, 24, 25, 26, 27, 7, 8, 9, 26, //
    17, 18, 19, 20, 18, 20, 21, 22, 25, 26, 27, 28, 23, 24, 25, 27,
];

// upstream: src/dsp/enc_neon.c kLookupTbl3
// The static table of Intra4Preds_NEON.
static LOOKUP_TBL3: [u8; 64] = [
    30, 30, 30, 30, 0, 0, 0, 0, 21, 22, 23, 24, 19, 19, 19, 19, //
    30, 30, 30, 30, 0, 0, 0, 0, 21, 22, 23, 24, 18, 18, 18, 18, //
    30, 30, 30, 30, 0, 0, 0, 0, 21, 22, 23, 24, 17, 17, 17, 17, //
    30, 30, 30, 30, 0, 0, 0, 0, 21, 22, 23, 24, 16, 16, 16, 16,
];

// upstream: src/dsp/enc_neon.c Intra4Preds_NEON
/// `top` is the boundary buffer and `top_pos` the position of `top[0]` in it;
/// the function loads the 16 bytes from `top[-5]`.
#[target_feature(enable = "neon")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C assigns the rounded average of eight samples to uint8_t"
)]
fn intra4_preds_neon(dst: &mut [u8], top: &[u8], top_pos: usize) {
    let lookup_avgs1 = vld1q_u8x4(&LOOKUP_TBL1);
    let lookup_avgs2 = vld1q_u8x4(&LOOKUP_TBL2);
    let lookup_avgs3 = vld1q_u8x4(&LOOKUP_TBL3);

    let preload = vld1q_u8(&top[top_pos - 5..]);

    let a = vqtbl1q_u8(preload, lookup_avgs1.0);
    let b = preload;
    let c = vextq_u8::<2>(a, a);

    let mut avg3_all = vrhaddq_u8(vhaddq_u8(a, c), b);
    let avg2_all = vrhaddq_u8(a, b);

    let mut preload_x8 = vget_low_u8(c);
    preload_x8 = vset_lane_u8::<3>(vgetq_lane_u8::<0>(preload), preload_x8);

    let result_u8 = ((vaddlv_u8(preload_x8) + 4) >> 3) as u8;

    avg3_all = vsetq_lane_u8::<15>(vgetq_lane_u8::<0>(preload), avg3_all);
    avg3_all = vsetq_lane_u8::<14>(result_u8, avg3_all);

    let qcombined = uint8x16x2_t(avg2_all, avg3_all);

    let sub_a = vdup_laneq_u8::<4>(preload);

    // preload = {a,b,c,d,...} => full_b = {d,d,d,d,c,c,c,c,b,b,b,b,a,a,a,a}
    let full_b = vqtbl1q_u8(preload, lookup_avgs1.1);
    // preload = {a,b,c,d,...} => sub_c = {a,b,c,d,a,b,c,d,a,b,c,d,a,b,c,d}
    let sub_c = vreinterpret_u8_u32(vdup_n_u32(vgetq_lane_u32::<0>(vreinterpretq_u32_u8(
        vextq_u8::<5>(preload, preload),
    ))));

    let sub = vsubl_u8(sub_c, sub_a);
    let sum_lo = vaddw_u8(sub, vget_low_u8(full_b));
    let res_lo = vqmovun_s16(vreinterpretq_s16_u16(sum_lo));

    let sum_hi = vaddw_u8(sub, vget_high_u8(full_b));
    let res_hi = vqmovun_s16(vreinterpretq_s16_u16(sum_hi));

    // DC4, VE4, HE4, TM4
    let dc4 = at(I4DC4);
    dc4_ve4_he4_tm4_neon::<0>(&mut dst[dc4..], qcombined, lookup_avgs3.0, res_lo);
    dc4_ve4_he4_tm4_neon::<1>(&mut dst[dc4 + BPS_U..], qcombined, lookup_avgs3.1, res_lo);
    dc4_ve4_he4_tm4_neon::<0>(
        &mut dst[dc4 + 2 * BPS_U..],
        qcombined,
        lookup_avgs3.2,
        res_hi,
    );
    dc4_ve4_he4_tm4_neon::<1>(
        &mut dst[dc4 + 3 * BPS_U..],
        qcombined,
        lookup_avgs3.3,
        res_hi,
    );

    // RD4, VR4, LD4, VL4
    let rd4 = at(I4RD4);
    rd4_vr4_ld4_vl4_neon(&mut dst[rd4..], qcombined, lookup_avgs2.0);
    rd4_vr4_ld4_vl4_neon(&mut dst[rd4 + BPS_U..], qcombined, lookup_avgs2.1);
    rd4_vr4_ld4_vl4_neon(&mut dst[rd4 + 2 * BPS_U..], qcombined, lookup_avgs2.2);
    rd4_vr4_ld4_vl4_neon(&mut dst[rd4 + 3 * BPS_U..], qcombined, lookup_avgs2.3);

    // HD4, HU4
    let result0 = vqtbl2q_u8(qcombined, lookup_avgs1.2);
    let result1 = vqtbl2q_u8(qcombined, lookup_avgs1.3);

    let hd4 = at(I4HD4);
    vst1_u8(&mut dst[hd4..], vget_low_u8(result0));
    vst1_u8(&mut dst[hd4 + BPS_U..], vget_high_u8(result0));
    vst1_u8(&mut dst[hd4 + 2 * BPS_U..], vget_low_u8(result1));
    vst1_u8(&mut dst[hd4 + 3 * BPS_U..], vget_high_u8(result1));
}

/// Entry point of `Intra4Preds_NEON` for `VP8EncPredLuma4`.
pub(crate) fn intra4_preds(dst: &mut [u8], top: &[u8], top_pos: usize) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { intra4_preds_neon(dst, top, top_pos) }
}

// upstream: src/dsp/enc_neon.c Fill_NEON
#[target_feature(enable = "neon")]
fn fill_neon(dst: &mut [u8], value: u8) {
    let a = vdupq_n_u8(value);
    for i in 0..16 {
        vst1q_u8(&mut dst[BPS_U * i..], a);
    }
}

// upstream: src/dsp/enc_neon.c Fill16_NEON
#[target_feature(enable = "neon")]
fn fill16_neon(dst: &mut [u8], src: &[u8]) {
    let a = vld1q_u8(src);
    for i in 0..16 {
        vst1q_u8(&mut dst[BPS_U * i..], a);
    }
}

// upstream: src/dsp/enc_neon.c HorizontalPred16_NEON
#[target_feature(enable = "neon")]
fn horizontal_pred16_neon(dst: &mut [u8], left: PredLeft<'_>) {
    let Some((left, pos)) = left else {
        fill_neon(dst, 129);
        return;
    };

    let a = vld1q_u8(&left[pos..]);
    vst1q_u8(&mut dst[0..], vdupq_laneq_u8::<0>(a));
    vst1q_u8(&mut dst[BPS_U..], vdupq_laneq_u8::<1>(a));
    vst1q_u8(&mut dst[BPS_U * 2..], vdupq_laneq_u8::<2>(a));
    vst1q_u8(&mut dst[BPS_U * 3..], vdupq_laneq_u8::<3>(a));
    vst1q_u8(&mut dst[BPS_U * 4..], vdupq_laneq_u8::<4>(a));
    vst1q_u8(&mut dst[BPS_U * 5..], vdupq_laneq_u8::<5>(a));
    vst1q_u8(&mut dst[BPS_U * 6..], vdupq_laneq_u8::<6>(a));
    vst1q_u8(&mut dst[BPS_U * 7..], vdupq_laneq_u8::<7>(a));
    vst1q_u8(&mut dst[BPS_U * 8..], vdupq_laneq_u8::<8>(a));
    vst1q_u8(&mut dst[BPS_U * 9..], vdupq_laneq_u8::<9>(a));
    vst1q_u8(&mut dst[BPS_U * 10..], vdupq_laneq_u8::<10>(a));
    vst1q_u8(&mut dst[BPS_U * 11..], vdupq_laneq_u8::<11>(a));
    vst1q_u8(&mut dst[BPS_U * 12..], vdupq_laneq_u8::<12>(a));
    vst1q_u8(&mut dst[BPS_U * 13..], vdupq_laneq_u8::<13>(a));
    vst1q_u8(&mut dst[BPS_U * 14..], vdupq_laneq_u8::<14>(a));
    vst1q_u8(&mut dst[BPS_U * 15..], vdupq_laneq_u8::<15>(a));
}

// upstream: src/dsp/enc_neon.c VerticalPred16_NEON
#[target_feature(enable = "neon")]
fn vertical_pred16_neon(dst: &mut [u8], top: Option<&[u8]>) {
    if let Some(top) = top {
        fill16_neon(dst, top);
    } else {
        fill_neon(dst, 127);
    }
}

// upstream: src/dsp/enc_neon.c DCMode_NEON
#[target_feature(enable = "neon")]
fn dc_mode_neon(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    let s = if let Some(top) = top {
        let mut dc = vaddlvq_u8(vld1q_u8(top));
        if let Some((left, pos)) = left {
            // top and left present.
            dc += vaddlvq_u8(vld1q_u8(&left[pos..]));
            vqrshrnh_n_u16::<5>(dc)
        } else {
            // top but no left.
            vqrshrnh_n_u16::<4>(dc)
        }
    } else if let Some((left, pos)) = left {
        // left but no top.
        let dc = vaddlvq_u8(vld1q_u8(&left[pos..]));
        vqrshrnh_n_u16::<4>(dc)
    } else {
        // No top, no left, nothing.
        0x80
    };
    fill_neon(dst, s);
}

// upstream: src/dsp/enc_neon.c TrueMotionHelper_NEON
#[target_feature(enable = "neon")]
fn true_motion_helper_neon(
    dst: &mut [u8],
    outer: uint8x8_t,
    inner: uint8x8x2_t,
    a: uint16x8_t,
    i: usize,
    n: usize,
) {
    let mut r1 = vaddl_u8(outer, inner.0);
    r1 = vqsubq_u16(r1, a);
    let d1 = vqmovun_s16(vreinterpretq_s16_u16(r1));
    let mut r2 = vaddl_u8(outer, inner.1);
    r2 = vqsubq_u16(r2, a);
    let d2 = vqmovun_s16(vreinterpretq_s16_u16(r2));
    vst1_u8(&mut dst[BPS_U * (i * 4 + n)..], d1);
    vst1_u8(&mut dst[BPS_U * (i * 4 + n) + 8..], d2);
}

// upstream: src/dsp/enc_neon.c TrueMotion_NEON
#[target_feature(enable = "neon")]
fn true_motion_neon(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    let Some((left_buf, pos)) = left else {
        // True motion without left samples (hence: with default 129 value) is
        // equivalent to VE prediction where you just copy the top samples.
        // Note that if top samples are not available, the default value is then
        // 129, and not 127 as in the VerticalPred case.
        if top.is_some() {
            vertical_pred16_neon(dst, top);
        } else {
            fill_neon(dst, 129);
        }
        return;
    };

    // left is not NULL.
    let Some(top) = top else {
        horizontal_pred16_neon(dst, left);
        return;
    };

    // Neither left nor top are NULL.
    let a = vdupq_n_u16(u16::from(left_buf[pos - 1]));
    let inner = vld1u8x2(top);

    for i in 0..4 {
        let outer = vld4_dup_u8(&left_buf[pos + i * 4..]);

        true_motion_helper_neon(dst, outer.0, inner, a, i, 0);
        true_motion_helper_neon(dst, outer.1, inner, a, i, 1);
        true_motion_helper_neon(dst, outer.2, inner, a, i, 2);
        true_motion_helper_neon(dst, outer.3, inner, a, i, 3);
    }
}

// upstream: src/dsp/enc_neon.c Intra16Preds_NEON
#[target_feature(enable = "neon")]
fn intra16_preds_neon(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    dc_mode_neon(&mut dst[at(I16DC16)..], left, top);
    vertical_pred16_neon(&mut dst[at(I16VE16)..], top);
    horizontal_pred16_neon(&mut dst[at(I16HE16)..], left);
    true_motion_neon(&mut dst[at(I16TM16)..], left, top);
}

/// Entry point of `Intra16Preds_NEON` for `VP8EncPredLuma16`.
pub(crate) fn intra16_preds(dst: &mut [u8], left: PredLeft<'_>, top: Option<&[u8]>) {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { intra16_preds_neon(dst, left, top) }
}

//------------------------------------------------------------------------------
// From src/dsp/quant.h, the WEBP_USE_NEON branch (deviation 1).

// upstream: src/dsp/quant.h horizontal_add_uint32x4
#[target_feature(enable = "neon")]
#[must_use]
fn horizontal_add_uint32x4(a: uint32x4_t) -> u32 {
    vaddvq_u32(a)
}

// upstream: src/dsp/quant.h IsFlat_NEON
// (`IsFlat`, which the macro names `IsFlat_NEON` with WEBP_USE_NEON.)
#[target_feature(enable = "neon")]
#[must_use]
#[expect(
    clippy::cast_possible_wrap,
    reason = "C casts the uint32_t count to int"
)]
fn is_flat_neon(levels: &[i16], num_blocks: i32, thresh: i32) -> bool {
    let tst_ones = vdupq_n_s16(-1);
    let mut sum = vdupq_n_u32(0);
    let mut levels_pos = 0;

    for _ in 0..num_blocks {
        // Set DC to zero.
        let a_0 = vsetq_lane_s16::<0>(0, vld1q_s16(&levels[levels_pos..]));
        let a_1 = vld1q_s16(&levels[levels_pos + 8..]);

        let b_0 = vshrq_n_u16::<15>(vtstq_s16(a_0, tst_ones));
        let b_1 = vshrq_n_u16::<15>(vtstq_s16(a_1, tst_ones));

        sum = vpadalq_u16(sum, b_0);
        sum = vpadalq_u16(sum, b_1);

        levels_pos += 16;
    }
    thresh >= horizontal_add_uint32x4(sum) as i32
}

/// Entry point of `IsFlat_NEON` for `IsFlat`.
#[must_use]
pub(crate) fn is_flat(levels: &[i16], num_blocks: i32, thresh: i32) -> bool {
    // SAFETY: NEON is in the AArch64 baseline that this module is compiled for,
    // and upstream selects the NEON variant there unconditionally (ADR 0008).
    unsafe { is_flat_neon(levels, num_blocks, thresh) }
}

//------------------------------------------------------------------------------
// Entry point

// upstream: src/dsp/enc_neon.c VP8EncDspInitNEON
// The pointer assignments are the dispatch functions of `src/dsp/enc.rs`
// named in the module documentation (ADR 0008).
