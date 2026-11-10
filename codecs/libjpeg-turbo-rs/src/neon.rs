//! The Neon loads and stores of the kernels, over arrays instead of
//! pointers (ADR 0007, decision 3). Not a translation of an upstream file.
//!
//! Each function has the name and the vector type of the intrinsic it
//! replaces, so that a kernel module that imports it by name, next to a glob
//! import of `core::arch::aarch64`, calls it where the C code calls the
//! intrinsic. Where the intrinsic takes a pointer, the function takes a
//! reference to an array of exactly the elements that the instruction reads
//! or writes.
//!
//! - The whole-vector loads and stores pass the array's pointer to the
//!   intrinsic in one `unsafe` block. The reference covers every element that
//!   the instruction reads or writes, and the instruction needs no alignment
//!   beyond the element type's, which the array has.
//! - The lane stores write the lane values with safe code. Upstream stores
//!   some of them through a `uint8_t` row pointer cast to `uint16_t *` or
//!   `uint64_t *`; these functions take the row's bytes and store the lanes'
//!   native-endian bytes, which is what the instruction writes, without the
//!   alignment that a Rust `u16` or `u64` pointer requires.
//!
//! The module exists only with `WITH_SIMD`, on AArch64, where every target
//! enables Neon.

use core::arch::aarch64::{
    self, int16x4_t, int16x4x3_t, int16x4x4_t, int16x8_t, uint8x8_t, uint8x8x2_t, uint8x8x3_t,
    uint8x8x4_t, uint8x16_t, uint8x16x2_t, uint8x16x3_t, uint8x16x4_t, uint16x4x2_t, uint16x4x4_t,
    uint64x2_t, vget_lane_u8, vget_lane_u16, vgetq_lane_u64,
};

/// `vld1_s16`: loads 4 elements.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
#[must_use]
pub fn vld1_s16(a: &[i16; 4]) -> int16x4_t {
    // SAFETY: `a` holds the 4 elements that the load reads.
    unsafe { aarch64::vld1_s16(a.as_ptr()) }
}

/// `vld1_s16_x3`: loads 12 elements into 3 vectors.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
#[must_use]
pub fn vld1_s16_x3(a: &[i16; 12]) -> int16x4x3_t {
    // SAFETY: `a` holds the 12 elements that the load reads.
    unsafe { aarch64::vld1_s16_x3(a.as_ptr()) }
}

/// `vld1q_s16`: loads 8 elements.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
#[must_use]
pub fn vld1q_s16(a: &[i16; 8]) -> int16x8_t {
    // SAFETY: `a` holds the 8 elements that the load reads.
    unsafe { aarch64::vld1q_s16(a.as_ptr()) }
}

/// `vld1_u8`: loads 8 bytes.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
#[must_use]
pub fn vld1_u8(a: &[u8; 8]) -> uint8x8_t {
    // SAFETY: `a` holds the 8 bytes that the load reads.
    unsafe { aarch64::vld1_u8(a.as_ptr()) }
}

/// `vld1q_u8`: loads 16 bytes.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
#[must_use]
pub fn vld1q_u8(a: &[u8; 16]) -> uint8x16_t {
    // SAFETY: `a` holds the 16 bytes that the load reads.
    unsafe { aarch64::vld1q_u8(a.as_ptr()) }
}

/// `vld2_u8`: loads 16 bytes, de-interleaved into 2 vectors.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
#[must_use]
pub fn vld2_u8(a: &[u8; 16]) -> uint8x8x2_t {
    // SAFETY: `a` holds the 16 bytes that the load reads.
    unsafe { aarch64::vld2_u8(a.as_ptr()) }
}

/// `vst1_lane_u8`: stores lane `LANE` of `b`.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[target_feature(enable = "neon")]
#[inline]
pub fn vst1_lane_u8<const LANE: i32>(a: &mut u8, b: uint8x8_t) {
    *a = vget_lane_u8::<LANE>(b);
}

/// `vst1q_lane_u64` through a byte pointer: stores the 8 bytes of lane
/// `LANE` of `b`.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[target_feature(enable = "neon")]
#[inline]
pub fn vst1q_lane_u64<const LANE: i32>(a: &mut [u8; 8], b: uint64x2_t) {
    *a = vgetq_lane_u64::<LANE>(b).to_ne_bytes();
}

/// `vst1q_u8`: stores 16 bytes.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
pub fn vst1q_u8(a: &mut [u8; 16], b: uint8x16_t) {
    // SAFETY: `a` holds the 16 bytes that the store writes.
    unsafe { aarch64::vst1q_u8(a.as_mut_ptr(), b) }
}

/// `vst2_lane_u16` through a byte pointer: stores lane `LANE` of each of
/// the 2 vectors of `b`, interleaved, as 4 bytes.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[target_feature(enable = "neon")]
#[inline]
pub fn vst2_lane_u16<const LANE: i32>(a: &mut [u8; 4], b: uint16x4x2_t) {
    let (first, second) = a.split_at_mut(2);
    first.copy_from_slice(&vget_lane_u16::<LANE>(b.0).to_ne_bytes());
    second.copy_from_slice(&vget_lane_u16::<LANE>(b.1).to_ne_bytes());
}

/// `vst2q_u8`: stores 2 vectors of 16 bytes, interleaved.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
pub fn vst2q_u8(a: &mut [u8; 32], b: uint8x16x2_t) {
    // SAFETY: `a` holds the 32 bytes that the store writes.
    unsafe { aarch64::vst2q_u8(a.as_mut_ptr(), b) }
}

/// `vst3_lane_u8`: stores lane `LANE` of each of the 3 vectors of `b`,
/// interleaved.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[target_feature(enable = "neon")]
#[inline]
pub fn vst3_lane_u8<const LANE: i32>(a: &mut [u8; 3], b: uint8x8x3_t) {
    *a = [
        vget_lane_u8::<LANE>(b.0),
        vget_lane_u8::<LANE>(b.1),
        vget_lane_u8::<LANE>(b.2),
    ];
}

/// `vst3_u8`: stores 3 vectors of 8 bytes, interleaved.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
pub fn vst3_u8(a: &mut [u8; 24], b: uint8x8x3_t) {
    // SAFETY: `a` holds the 24 bytes that the store writes.
    unsafe { aarch64::vst3_u8(a.as_mut_ptr(), b) }
}

/// `vst3q_u8`: stores 3 vectors of 16 bytes, interleaved.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
pub fn vst3q_u8(a: &mut [u8; 48], b: uint8x16x3_t) {
    // SAFETY: `a` holds the 48 bytes that the store writes.
    unsafe { aarch64::vst3q_u8(a.as_mut_ptr(), b) }
}

/// `vst4_lane_u16` through a byte pointer: stores lane `LANE` of each of
/// the 4 vectors of `b`, interleaved, as 8 bytes.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[target_feature(enable = "neon")]
#[inline]
pub fn vst4_lane_u16<const LANE: i32>(a: &mut [u8; 8], b: uint16x4x4_t) {
    let lanes = [
        vget_lane_u16::<LANE>(b.0),
        vget_lane_u16::<LANE>(b.1),
        vget_lane_u16::<LANE>(b.2),
        vget_lane_u16::<LANE>(b.3),
    ];
    for (bytes, lane) in a.as_chunks_mut::<2>().0.iter_mut().zip(lanes) {
        *bytes = lane.to_ne_bytes();
    }
}

/// `vst4_lane_u8`: stores lane `LANE` of each of the 4 vectors of `b`,
/// interleaved.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[target_feature(enable = "neon")]
#[inline]
pub fn vst4_lane_u8<const LANE: i32>(a: &mut [u8; 4], b: uint8x8x4_t) {
    *a = [
        vget_lane_u8::<LANE>(b.0),
        vget_lane_u8::<LANE>(b.1),
        vget_lane_u8::<LANE>(b.2),
        vget_lane_u8::<LANE>(b.3),
    ];
}

/// `vst4_s16`: stores 4 vectors of 4 elements, interleaved.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
pub fn vst4_s16(a: &mut [i16; 16], b: int16x4x4_t) {
    // SAFETY: `a` holds the 16 elements that the store writes.
    unsafe { aarch64::vst4_s16(a.as_mut_ptr(), b) }
}

/// `vst4_u8`: stores 4 vectors of 8 bytes, interleaved.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
pub fn vst4_u8(a: &mut [u8; 32], b: uint8x8x4_t) {
    // SAFETY: `a` holds the 32 bytes that the store writes.
    unsafe { aarch64::vst4_u8(a.as_mut_ptr(), b) }
}

/// `vst4q_u8`: stores 4 vectors of 16 bytes, interleaved.
///
/// # Safety
///
/// Neon must be available, as it is on every AArch64 target.
#[allow(unsafe_code, reason = "ADR 0007: a pointer load or store")]
#[target_feature(enable = "neon")]
#[inline]
pub fn vst4q_u8(a: &mut [u8; 64], b: uint8x16x4_t) {
    // SAFETY: `a` holds the 64 bytes that the store writes.
    unsafe { aarch64::vst4q_u8(a.as_mut_ptr(), b) }
}
