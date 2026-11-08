//! The SSE2 and AVX loads and stores of the x86-64 kernels, over arrays
//! instead of pointers (ADR 0008, decision 4, after ADR 0007, decision 3).
//! Not a translation of an upstream file.
//!
//! Each function has the name of the intrinsic it calls, so that a kernel
//! module that imports it by name, next to a glob import of
//! `core::arch::x86_64`, calls it where the asm moves a vector between a
//! register and memory. The asm's aligned moves (`movdqa`, `vmovdqa`,
//! `movaps`) and its unaligned ones become the same unaligned intrinsics,
//! because a row slice carries no alignment; they move the same values.
//!
//! An intrinsic takes a pointer to a vector type whatever the elements in
//! memory are, so each function takes a reference to any array of plain
//! numbers that has the vector's size: an [`Xmmword`] for 128 bits, a
//! [`Ymmword`] for 256 bits and a [`Qword`] for the 64-bit `movq` forms.
//! Each passes the array's pointer to the intrinsic in one `unsafe` block.
//! The reference covers every byte that the instruction reads or writes, the
//! unaligned forms need no alignment, and every bit pattern is a valid value
//! of the array's element type, so a store writes valid values.
//!
//! Partial stores of fewer bytes (`movd`, `pextrw`, byte stores) need no
//! function here: a kernel takes the value out of the register with a value
//! intrinsic and writes its bytes with safe code.
//!
//! The module exists only with `WITH_SIMD`, on x86-64, where every target
//! enables SSE and SSE2. The AVX functions are called only from AVX2 kernels,
//! which `jsimd` selects only when the CPU has AVX2.

use core::arch::x86_64::{self, __m128, __m128i, __m256i};

mod sealed {
    /// Implemented only for the arrays of [`super::Xmmword`],
    /// [`super::Ymmword`] and [`super::Qword`].
    pub trait Sealed {}
}

/// An array of 16 bytes of plain numbers, which a 128-bit load or store
/// moves: an `XMMWORD` of `jsimdext.inc`.
pub trait Xmmword: sealed::Sealed + Copy {}

/// An array of 32 bytes of plain numbers, which a 256-bit load or store
/// moves: a `YMMWORD` of `jsimdext.inc`.
pub trait Ymmword: sealed::Sealed + Copy {}

/// An array of 8 bytes of plain numbers, which a 64-bit `movq` load or store
/// moves: a `QWORD` of `jsimdext.inc`.
pub trait Qword: sealed::Sealed + Copy {}

macro_rules! arrays {
    ($trait:ident: $($array:ty),+) => {
        $(
            impl sealed::Sealed for $array {}
            impl $trait for $array {}
        )+
    };
}

arrays!(Xmmword: [u8; 16], [i8; 16], [i16; 8], [u16; 8], [i32; 4], [u32; 4], [f32; 4]);
arrays!(Ymmword: [u8; 32], [i8; 32], [i16; 16], [u16; 16], [i32; 8], [u32; 8], [f32; 8]);
arrays!(Qword: [u8; 8], [i8; 8], [i16; 4], [u16; 4], [i32; 2], [u32; 2], [f32; 2]);

/// `_mm_loadu_si128` (`movdqu`, and the asm's `movdqa`): loads 16 bytes.
///
/// # Safety
///
/// SSE2 must be available, as it is on every x86-64 target.
#[allow(unsafe_code, reason = "ADR 0008: a pointer load or store")]
#[target_feature(enable = "sse2")]
#[inline]
#[must_use]
pub fn _mm_loadu_si128<T: Xmmword>(a: &T) -> __m128i {
    // SAFETY: `a` holds the 16 bytes that the unaligned load reads.
    unsafe { x86_64::_mm_loadu_si128(core::ptr::from_ref(a).cast()) }
}

/// `_mm_storeu_si128` (`movdqu`, and the asm's `movdqa`): stores 16 bytes.
///
/// # Safety
///
/// SSE2 must be available, as it is on every x86-64 target.
#[allow(unsafe_code, reason = "ADR 0008: a pointer load or store")]
#[target_feature(enable = "sse2")]
#[inline]
pub fn _mm_storeu_si128<T: Xmmword>(a: &mut T, b: __m128i) {
    // SAFETY: `a` holds the 16 bytes that the unaligned store writes, and
    // any bytes are valid elements of `T`.
    unsafe { x86_64::_mm_storeu_si128(core::ptr::from_mut(a).cast(), b) }
}

/// `_mm_loadl_epi64` (`movq` from memory): loads 8 bytes into the low half
/// and zeroes the high half.
///
/// # Safety
///
/// SSE2 must be available, as it is on every x86-64 target.
#[allow(unsafe_code, reason = "ADR 0008: a pointer load or store")]
#[target_feature(enable = "sse2")]
#[inline]
#[must_use]
pub fn _mm_loadl_epi64<T: Qword>(a: &T) -> __m128i {
    // SAFETY: `a` holds the 8 bytes that the load reads; `movq` needs no
    // alignment.
    unsafe { x86_64::_mm_loadl_epi64(core::ptr::from_ref(a).cast()) }
}

/// `_mm_storel_epi64` (`movq` to memory): stores the low 8 bytes.
///
/// # Safety
///
/// SSE2 must be available, as it is on every x86-64 target.
#[allow(unsafe_code, reason = "ADR 0008: a pointer load or store")]
#[target_feature(enable = "sse2")]
#[inline]
pub fn _mm_storel_epi64<T: Qword>(a: &mut T, b: __m128i) {
    // SAFETY: `a` holds the 8 bytes that the store writes, `movq` needs no
    // alignment, and any bytes are valid elements of `T`.
    unsafe { x86_64::_mm_storel_epi64(core::ptr::from_mut(a).cast(), b) }
}

/// `_mm_loadu_ps` (`movups`, and the asm's `movaps`): loads 4 floats.
///
/// # Safety
///
/// SSE must be available, as it is on every x86-64 target.
#[allow(unsafe_code, reason = "ADR 0008: a pointer load or store")]
#[target_feature(enable = "sse")]
#[inline]
#[must_use]
pub fn _mm_loadu_ps<T: Xmmword>(a: &T) -> __m128 {
    // SAFETY: `a` holds the 16 bytes that the unaligned load reads.
    unsafe { x86_64::_mm_loadu_ps(core::ptr::from_ref(a).cast()) }
}

/// `_mm_storeu_ps` (`movups`, and the asm's `movaps`): stores 4 floats.
///
/// # Safety
///
/// SSE must be available, as it is on every x86-64 target.
#[allow(unsafe_code, reason = "ADR 0008: a pointer load or store")]
#[target_feature(enable = "sse")]
#[inline]
pub fn _mm_storeu_ps<T: Xmmword>(a: &mut T, b: __m128) {
    // SAFETY: `a` holds the 16 bytes that the unaligned store writes, and
    // any bytes are valid elements of `T`.
    unsafe { x86_64::_mm_storeu_ps(core::ptr::from_mut(a).cast(), b) }
}

/// `_mm256_loadu_si256` (`vmovdqu`, and the asm's `vmovdqa`): loads 32
/// bytes.
///
/// # Safety
///
/// AVX must be available. Only AVX2 kernels call it, and `jsimd` selects
/// them only when the CPU has AVX2.
#[allow(unsafe_code, reason = "ADR 0008: a pointer load or store")]
#[target_feature(enable = "avx")]
#[inline]
#[must_use]
pub fn _mm256_loadu_si256<T: Ymmword>(a: &T) -> __m256i {
    // SAFETY: `a` holds the 32 bytes that the unaligned load reads.
    unsafe { x86_64::_mm256_loadu_si256(core::ptr::from_ref(a).cast()) }
}

/// `_mm256_storeu_si256` (`vmovdqu`, and the asm's `vmovdqa`): stores 32
/// bytes.
///
/// # Safety
///
/// AVX must be available. Only AVX2 kernels call it, and `jsimd` selects
/// them only when the CPU has AVX2.
#[allow(unsafe_code, reason = "ADR 0008: a pointer load or store")]
#[target_feature(enable = "avx")]
#[inline]
pub fn _mm256_storeu_si256<T: Ymmword>(a: &mut T, b: __m256i) {
    // SAFETY: `a` holds the 32 bytes that the unaligned store writes, and
    // any bytes are valid elements of `T`.
    unsafe { x86_64::_mm256_storeu_si256(core::ptr::from_mut(a).cast(), b) }
}
