//! Runtime detection and the per-level entry points. The only module that
//! calls `#[target_feature]` functions; each call sits in an `unsafe` block
//! justified by `table`, which hands out a level's entries only after
//! `supports` confirmed the machine can run them.

#![allow(unsafe_code)]

use crate::lanes::Scalar;
use crate::{IsaLevel, Taps, body};

#[derive(Clone, Copy)]
pub(crate) struct Table {
    pub horizontal_rgba: fn(&[f32], &Taps, &mut [f32]),
    pub axpy: fn(&mut [f32], &[f32], f32),
}

pub(crate) fn supports(level: IsaLevel) -> bool {
    match level {
        IsaLevel::Scalar => true,
        #[cfg(target_arch = "aarch64")]
        IsaLevel::Neon => std::arch::is_aarch64_feature_detected!("neon"),
        #[cfg(target_arch = "aarch64")]
        IsaLevel::Sve2 => std::arch::is_aarch64_feature_detected!("sve2"),
        #[cfg(target_arch = "x86_64")]
        IsaLevel::Avx2 => {
            std::arch::is_x86_feature_detected!("avx2")
                && std::arch::is_x86_feature_detected!("fma")
        }
        #[cfg(target_arch = "x86_64")]
        IsaLevel::Avx512 => {
            std::arch::is_x86_feature_detected!("avx512f")
                && std::arch::is_x86_feature_detected!("avx512bw")
                && std::arch::is_x86_feature_detected!("avx512cd")
                && std::arch::is_x86_feature_detected!("avx512dq")
                && std::arch::is_x86_feature_detected!("avx512vl")
        }
        #[allow(unreachable_patterns, reason = "the other architecture's levels")]
        _ => false,
    }
}

pub(crate) fn detect_machine() -> IsaLevel {
    [
        IsaLevel::Avx512,
        IsaLevel::Avx2,
        IsaLevel::Sve2,
        IsaLevel::Neon,
    ]
    .into_iter()
    .find(|&level| supports(level))
    .unwrap_or(IsaLevel::Scalar)
}

/// The portable entries, available on every machine.
pub(crate) fn scalar_table() -> Table {
    Table {
        horizontal_rgba: body::horizontal_rgba::<Scalar>,
        axpy: body::axpy::<Scalar>,
    }
}

/// Entries for `level`, or `None` if it is not implemented or the machine
/// cannot run it. This check is what makes the wrappers below sound.
pub(crate) fn table(level: IsaLevel) -> Option<Table> {
    if !level.is_implemented() || !supports(level) {
        return None;
    }
    match level {
        IsaLevel::Scalar => Some(scalar_table()),
        #[cfg(target_arch = "aarch64")]
        IsaLevel::Neon => Some(Table {
            horizontal_rgba: neon::horizontal_rgba,
            axpy: neon::axpy,
        }),
        #[cfg(target_arch = "x86_64")]
        IsaLevel::Avx2 => Some(Table {
            horizontal_rgba: avx2::horizontal_rgba,
            axpy: avx2::axpy,
        }),
        #[cfg(target_arch = "x86_64")]
        IsaLevel::Avx512 => Some(Table {
            horizontal_rgba: avx512::horizontal_rgba,
            axpy: avx512::axpy,
        }),
        _ => None,
    }
}

#[cfg(target_arch = "aarch64")]
mod neon {
    use crate::lanes::Neon;
    use crate::{Taps, body};

    #[target_feature(enable = "neon")]
    fn horizontal_rgba_entry(source: &[f32], taps: &Taps, out: &mut [f32]) {
        body::horizontal_rgba::<Neon>(source, taps, out);
    }

    #[target_feature(enable = "neon")]
    fn axpy_entry(acc: &mut [f32], row: &[f32], weight: f32) {
        body::axpy::<Neon>(acc, row, weight);
    }

    pub(super) fn horizontal_rgba(source: &[f32], taps: &Taps, out: &mut [f32]) {
        // SAFETY: this wrapper is only placed in a `Table` by `table` after
        // `supports(IsaLevel::Neon)` returned true.
        unsafe { horizontal_rgba_entry(source, taps, out) }
    }

    pub(super) fn axpy(acc: &mut [f32], row: &[f32], weight: f32) {
        // SAFETY: as for `horizontal_rgba`.
        unsafe { axpy_entry(acc, row, weight) }
    }
}

#[cfg(target_arch = "x86_64")]
mod avx2 {
    use crate::lanes::Avx2;
    use crate::{Taps, body};

    #[target_feature(enable = "avx2,fma")]
    fn horizontal_rgba_entry(source: &[f32], taps: &Taps, out: &mut [f32]) {
        body::horizontal_rgba::<Avx2>(source, taps, out);
    }

    #[target_feature(enable = "avx2,fma")]
    fn axpy_entry(acc: &mut [f32], row: &[f32], weight: f32) {
        body::axpy::<Avx2>(acc, row, weight);
    }

    pub(super) fn horizontal_rgba(source: &[f32], taps: &Taps, out: &mut [f32]) {
        // SAFETY: this wrapper is only placed in a `Table` by `table` after
        // `supports(IsaLevel::Avx2)` confirmed AVX2 and FMA.
        unsafe { horizontal_rgba_entry(source, taps, out) }
    }

    pub(super) fn axpy(acc: &mut [f32], row: &[f32], weight: f32) {
        // SAFETY: as for `horizontal_rgba`.
        unsafe { axpy_entry(acc, row, weight) }
    }
}

#[cfg(target_arch = "x86_64")]
mod avx512 {
    use crate::lanes::Avx512;
    use crate::{Taps, body};

    #[target_feature(enable = "avx512f,avx512bw,avx512cd,avx512dq,avx512vl,fma")]
    fn horizontal_rgba_entry(source: &[f32], taps: &Taps, out: &mut [f32]) {
        body::horizontal_rgba::<Avx512>(source, taps, out);
    }

    #[target_feature(enable = "avx512f,avx512bw,avx512cd,avx512dq,avx512vl,fma")]
    fn axpy_entry(acc: &mut [f32], row: &[f32], weight: f32) {
        body::axpy::<Avx512>(acc, row, weight);
    }

    pub(super) fn horizontal_rgba(source: &[f32], taps: &Taps, out: &mut [f32]) {
        // SAFETY: this wrapper is only placed in a `Table` by `table` after
        // `supports(IsaLevel::Avx512)` confirmed F, BW, CD, DQ and VL.
        unsafe { horizontal_rgba_entry(source, taps, out) }
    }

    pub(super) fn axpy(acc: &mut [f32], row: &[f32], weight: f32) {
        // SAFETY: as for `horizontal_rgba`.
        unsafe { axpy_entry(acc, row, weight) }
    }
}
