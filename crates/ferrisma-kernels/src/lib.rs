//! Runtime-dispatched SIMD kernels for Ferrisma, behind a safe API
//! ([ADR-0020](../../docs/adr/0020-kernel-crate-and-isa-dispatch.md)).
//!
//! Every kernel is written once ([`body`]) against the internal
//! [`lanes::Lanes`] trait and instantiated per instruction-set level inside
//! a `#[target_feature]` entry. [`Kernels::best`] picks the highest level the
//! running machine supports up to [`DEFAULT_CEILING`] (AVX2 on x86-64, NEON
//! on aarch64; decision D6 of the ADR-0020 amendment of 2026-11-13);
//! [`Kernels::with_ceiling`]`(IsaLevel::Avx512)` is the explicit opt-in to
//! AVX-512. [`Kernels::for_level`] pins one level for differential tests,
//! benchmarks and bug reports.
//!
//! `unsafe` is confined to two modules: [`lanes`] (intrinsics and their
//! pointer loads and stores) and `dispatch` (calling a `#[target_feature]`
//! entry after runtime detection). Every other file is safe Rust.
//!
//! Kernels:
//!
//! - [`Kernels::horizontal_rgba`]: one row of a separable resample, four
//!   `f32` channels per pixel, with precomputed contiguous taps.
//! - [`Kernels::axpy`]: `acc += row * weight`, the vertical resample step.
//!
//! Equivalence: `axpy` is bit-identical at every level (one fused
//! multiply-add per element in the same order). `horizontal_rgba` is
//! bit-identical between `Scalar` and `Neon`; `Avx2` and `Avx512`
//! sum two and four taps per vector and differ by rounding only, bounded by
//! the differential tests.

// Kernel bodies and `Lanes` methods must inline into their
// `#[target_feature]` entry; otherwise intrinsics are compiled as calls
// outside the feature context (verified with `--emit=asm`, ADR-0020).
#![allow(clippy::inline_always)]

mod body;
mod dispatch;
mod lanes;

use std::fmt;
use std::sync::OnceLock;

/// Instruction-set levels in ascending order. A level a machine supports
/// but this crate does not implement falls back to the next lower one
/// (reason [`LevelReason::Fallback`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum IsaLevel {
    /// Portable Rust, the reference body.
    Scalar,
    /// 128-bit Advanced SIMD (aarch64 baseline).
    Neon,
    /// Scalable vectors, aarch64. Detected but not implemented: the scalar
    /// body compiled for SVE2 measured slower than `Neon` on Neoverse
    /// (ADR-0020), and stable Rust has no SVE intrinsics.
    Sve2,
    /// 256-bit AVX2 with FMA (x86-64-v3).
    Avx2,
    /// 512-bit AVX-512 F/BW/CD/DQ/VL with FMA (x86-64-v4).
    Avx512,
}

impl IsaLevel {
    /// Whether this crate has kernels for the level.
    #[must_use]
    pub const fn is_implemented(self) -> bool {
        matches!(self, Self::Scalar | Self::Neon | Self::Avx2 | Self::Avx512)
    }

    /// Short lowercase name for reports and logs.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Scalar => "scalar",
            Self::Neon => "neon",
            Self::Sve2 => "sve2",
            Self::Avx2 => "avx2",
            Self::Avx512 => "avx512",
        }
    }
}

impl fmt::Display for IsaLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

/// Why a level was chosen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum LevelReason {
    /// The highest level of the machine, and it is implemented.
    Detected,
    /// The machine supports more, but that level is not implemented.
    Fallback,
    /// Limited by a caller-supplied ceiling.
    Ceiling,
    /// Limited by [`DEFAULT_CEILING`]: the machine supports an implemented
    /// level above the v1.0 scope, which runs only on explicit request.
    DefaultCeiling,
}

/// The highest level [`Kernels::best`] selects: `Avx2` on x86-64 and `Neon`
/// on aarch64 (decision D6, ADR-0020 amendment of 2026-11-13). Higher levels
/// stay compiled and tested, but run only through an explicit ceiling such
/// as [`Kernels::with_ceiling`]`(IsaLevel::Avx512)`, so that default output
/// does not depend on whether the host has AVX-512 (issue #239).
pub const DEFAULT_CEILING: IsaLevel = if cfg!(target_arch = "aarch64") {
    IsaLevel::Neon
} else {
    IsaLevel::Avx2
};

/// The highest implemented level not above `ceiling` that `supported`
/// accepts. `supported` is injected so that tests can model machines this
/// host is not.
fn pick(ceiling: IsaLevel, supported: impl Fn(IsaLevel) -> bool) -> IsaLevel {
    [
        IsaLevel::Avx512,
        IsaLevel::Avx2,
        IsaLevel::Sve2,
        IsaLevel::Neon,
    ]
    .into_iter()
    .find(|&level| level <= ceiling && level.is_implemented() && supported(level))
    .unwrap_or(IsaLevel::Scalar)
}

/// Level and reason for an explicit `ceiling`, or for [`DEFAULT_CEILING`]
/// when `ceiling` is `None`, on a machine whose highest level is `machine`.
fn select(
    ceiling: Option<IsaLevel>,
    machine: IsaLevel,
    supported: impl Fn(IsaLevel) -> bool,
) -> (IsaLevel, LevelReason) {
    let explicit = ceiling.unwrap_or(IsaLevel::Avx512);
    let level = pick(ceiling.unwrap_or(DEFAULT_CEILING), &supported);
    let reason = if ceiling.is_none() && level != pick(explicit, &supported) {
        LevelReason::DefaultCeiling
    } else if explicit < machine {
        LevelReason::Ceiling
    } else if level == machine {
        LevelReason::Detected
    } else {
        LevelReason::Fallback
    };
    (level, reason)
}

/// The highest level the running machine supports, implemented or not.
/// Detected once and cached.
#[must_use]
pub fn machine_level() -> IsaLevel {
    static LEVEL: OnceLock<IsaLevel> = OnceLock::new();
    *LEVEL.get_or_init(dispatch::detect_machine)
}

/// Whether the running machine can execute `level`.
#[must_use]
pub fn supports(level: IsaLevel) -> bool {
    dispatch::supports(level)
}

/// Errors of the safe API: shapes that do not match the declared taps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KernelError {
    /// The level is not implemented or not supported by this machine.
    UnsupportedLevel(IsaLevel),
    /// A tap refers outside the source row, or the tap tables are malformed.
    InvalidTaps,
    /// Slice lengths do not match the kernel's declared shapes.
    LengthMismatch,
}

impl fmt::Display for KernelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedLevel(level) => {
                write!(formatter, "kernel level {level} is unavailable")
            }
            Self::InvalidTaps => formatter.write_str("invalid resample taps"),
            Self::LengthMismatch => formatter.write_str("kernel slice lengths do not match"),
        }
    }
}

impl std::error::Error for KernelError {}

/// Validated taps of one resample axis: output position `i` reads source
/// pixels `starts[i] .. starts[i] + n` with the `n` contiguous weights
/// `weights[offsets[i] .. offsets[i + 1]]`.
#[derive(Clone, Debug)]
pub struct Taps {
    source_pixels: usize,
    starts: Vec<u32>,
    offsets: Vec<u32>,
    weights: Vec<f32>,
}

impl Taps {
    /// Validate a tap table against the number of source pixels per row.
    ///
    /// # Errors
    /// [`KernelError::InvalidTaps`] if offsets are not ascending from 0 to
    /// `weights.len()`, or any tap run leaves the source row.
    pub fn new(
        source_pixels: usize,
        starts: Vec<u32>,
        offsets: Vec<u32>,
        weights: Vec<f32>,
    ) -> Result<Self, KernelError> {
        if offsets.len() != starts.len() + 1
            || offsets.first() != Some(&0)
            || usize::try_from(*offsets.last().unwrap_or(&0)).ok() != Some(weights.len())
        {
            return Err(KernelError::InvalidTaps);
        }
        for (position, &start) in starts.iter().enumerate() {
            let (begin, end) = (offsets[position], offsets[position + 1]);
            let count = end.checked_sub(begin).ok_or(KernelError::InvalidTaps)?;
            let last = usize::try_from(start)
                .ok()
                .and_then(|start| start.checked_add(usize::try_from(count).ok()?))
                .ok_or(KernelError::InvalidTaps)?;
            if last > source_pixels {
                return Err(KernelError::InvalidTaps);
            }
        }
        Ok(Self {
            source_pixels,
            starts,
            offsets,
            weights,
        })
    }

    /// Number of output positions.
    #[must_use]
    pub fn outputs(&self) -> usize {
        self.starts.len()
    }

    /// Number of source pixels per row these taps were validated for.
    #[must_use]
    pub fn source_pixels(&self) -> usize {
        self.source_pixels
    }
}

/// A table of kernels for one instruction-set level.
#[derive(Clone, Copy)]
pub struct Kernels {
    level: IsaLevel,
    reason: LevelReason,
    table: dispatch::Table,
}

impl fmt::Debug for Kernels {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Kernels")
            .field("level", &self.level)
            .field("reason", &self.reason)
            .finish_non_exhaustive()
    }
}

impl Kernels {
    /// The highest implemented level this machine supports, up to
    /// [`DEFAULT_CEILING`]. On an AVX-512 machine this is `Avx2` with reason
    /// [`LevelReason::DefaultCeiling`].
    #[must_use]
    pub fn best() -> Self {
        Self::selected(None)
    }

    /// The highest implemented, supported level not above `ceiling`. An
    /// explicit ceiling may exceed [`DEFAULT_CEILING`]:
    /// `with_ceiling(IsaLevel::Avx512)` is the opt-in to AVX-512.
    #[must_use]
    pub fn with_ceiling(ceiling: IsaLevel) -> Self {
        Self::selected(Some(ceiling))
    }

    fn selected(ceiling: Option<IsaLevel>) -> Self {
        let (level, reason) = select(ceiling, machine_level(), supports);
        Self {
            level,
            reason,
            table: dispatch::table(level).unwrap_or_else(dispatch::scalar_table),
        }
    }

    /// Exactly `level`.
    ///
    /// # Errors
    /// [`KernelError::UnsupportedLevel`] if the level is not implemented or
    /// the machine cannot run it.
    pub fn for_level(level: IsaLevel) -> Result<Self, KernelError> {
        let table = dispatch::table(level).ok_or(KernelError::UnsupportedLevel(level))?;
        Ok(Self {
            level,
            reason: if level == machine_level() {
                LevelReason::Detected
            } else {
                LevelReason::Ceiling
            },
            table,
        })
    }

    /// Every implemented level the running machine supports, ascending.
    #[must_use]
    pub fn available() -> Vec<Self> {
        [
            IsaLevel::Scalar,
            IsaLevel::Neon,
            IsaLevel::Sve2,
            IsaLevel::Avx2,
            IsaLevel::Avx512,
        ]
        .into_iter()
        .filter_map(|level| Self::for_level(level).ok())
        .collect()
    }

    /// The selected kernel level.
    #[must_use]
    pub fn level(&self) -> IsaLevel {
        self.level
    }

    /// Why the level was selected.
    #[must_use]
    pub fn reason(&self) -> LevelReason {
        self.reason
    }

    /// Resample one row: `out[i] = sum_k weights[k] * source[start_i + k]`
    /// per channel, four `f32` channels per pixel, taps summed in order.
    ///
    /// # Errors
    /// [`KernelError::LengthMismatch`] unless `source` holds
    /// `taps.source_pixels() * 4` and `out` holds `taps.outputs() * 4` values.
    pub fn horizontal_rgba(
        &self,
        source: &[f32],
        taps: &Taps,
        out: &mut [f32],
    ) -> Result<(), KernelError> {
        if Some(source.len()) != taps.source_pixels.checked_mul(4)
            || Some(out.len()) != taps.outputs().checked_mul(4)
        {
            return Err(KernelError::LengthMismatch);
        }
        (self.table.horizontal_rgba)(source, taps, out);
        Ok(())
    }

    /// `acc[i] = row[i].mul_add(weight, acc[i])` for every element.
    ///
    /// # Errors
    /// [`KernelError::LengthMismatch`] if the slices differ in length.
    pub fn axpy(&self, acc: &mut [f32], row: &[f32], weight: f32) -> Result<(), KernelError> {
        if acc.len() != row.len() {
            return Err(KernelError::LengthMismatch);
        }
        (self.table.axpy)(acc, row, weight);
        Ok(())
    }
}

/// Selection logic against injected feature sets (decision D6): this host
/// cannot execute AVX-512 or SVE2, so machines with those levels are
/// modeled rather than detected.
#[cfg(test)]
mod selection_tests {
    use super::{DEFAULT_CEILING, IsaLevel, LevelReason, select};

    /// A machine supporting exactly `levels` (plus `Scalar`), whose highest
    /// level is the last entry.
    fn machine(levels: &[IsaLevel]) -> (IsaLevel, impl Fn(IsaLevel) -> bool + '_) {
        let highest = levels.last().copied().unwrap_or(IsaLevel::Scalar);
        (highest, move |level| {
            level == IsaLevel::Scalar || levels.contains(&level)
        })
    }

    const X86_AVX512: &[IsaLevel] = &[IsaLevel::Avx2, IsaLevel::Avx512];
    const X86_AVX2: &[IsaLevel] = &[IsaLevel::Avx2];
    const ARM_SVE2: &[IsaLevel] = &[IsaLevel::Neon, IsaLevel::Sve2];
    const ARM_NEON: &[IsaLevel] = &[IsaLevel::Neon];

    #[test]
    fn the_default_ceiling_is_avx2_on_x86_64_and_neon_on_aarch64() {
        if cfg!(target_arch = "aarch64") {
            assert_eq!(DEFAULT_CEILING, IsaLevel::Neon);
        } else {
            assert_eq!(DEFAULT_CEILING, IsaLevel::Avx2);
        }
        assert!(DEFAULT_CEILING < IsaLevel::Avx512);
        assert_ne!(DEFAULT_CEILING, IsaLevel::Sve2);
    }

    #[test]
    fn an_avx512_machine_runs_avx2_by_default() {
        let (highest, supported) = machine(X86_AVX512);
        assert_eq!(
            select(Some(IsaLevel::Avx2), highest, &supported),
            (IsaLevel::Avx2, LevelReason::Ceiling)
        );
        if cfg!(target_arch = "x86_64") {
            assert_eq!(
                select(None, highest, &supported),
                (IsaLevel::Avx2, LevelReason::DefaultCeiling)
            );
        }
    }

    #[test]
    fn an_explicit_avx512_ceiling_restores_avx512() {
        let (highest, supported) = machine(X86_AVX512);
        assert_eq!(
            select(Some(IsaLevel::Avx512), highest, &supported),
            (IsaLevel::Avx512, LevelReason::Detected)
        );
    }

    #[test]
    fn an_avx2_machine_is_unaffected_by_the_default_ceiling() {
        let (highest, supported) = machine(X86_AVX2);
        let expected = (IsaLevel::Avx2, LevelReason::Detected);
        assert_eq!(
            select(Some(IsaLevel::Avx512), highest, &supported),
            expected
        );
        if cfg!(target_arch = "x86_64") {
            assert_eq!(select(None, highest, &supported), expected);
        }
    }

    #[test]
    fn aarch64_machines_run_neon_by_default_and_with_any_explicit_ceiling() {
        for levels in [ARM_NEON, ARM_SVE2] {
            let (highest, supported) = machine(levels);
            let reason = if highest == IsaLevel::Neon {
                LevelReason::Detected
            } else {
                // SVE2 is detected but not implemented.
                LevelReason::Fallback
            };
            if cfg!(target_arch = "aarch64") {
                assert_eq!(select(None, highest, &supported), (IsaLevel::Neon, reason));
            }
            assert_eq!(
                select(Some(IsaLevel::Avx512), highest, &supported),
                (IsaLevel::Neon, reason)
            );
            assert_eq!(
                select(Some(IsaLevel::Neon), highest, &supported).0,
                IsaLevel::Neon
            );
        }
    }

    #[test]
    fn a_machine_without_simd_runs_scalar() {
        let (highest, supported) = machine(&[]);
        assert_eq!(
            select(None, highest, &supported),
            (IsaLevel::Scalar, LevelReason::Detected)
        );
        assert_eq!(
            select(Some(IsaLevel::Avx512), highest, &supported),
            (IsaLevel::Scalar, LevelReason::Detected)
        );
    }
}
