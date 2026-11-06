//! Runtime-dispatched SIMD kernels for Ferrixel, behind a safe API
//! ([ADR-0020](../../docs/adr/0020-kernel-crate-and-isa-dispatch.md)).
//!
//! Every kernel is written once ([`body`]) against the internal
//! [`lanes::Lanes`] trait and instantiated per instruction-set level inside
//! a `#[target_feature]` entry. [`Kernels::best`] picks the highest level the
//! running machine supports; [`Kernels::for_level`] pins a lower one for
//! differential tests, benchmarks and bug reports.
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
//! bit-identical between `Scalar`, `Neon` and `Sve2`; `Avx2` and `Avx512`
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
pub enum IsaLevel {
    /// Portable Rust, the reference body.
    Scalar,
    /// 128-bit Advanced SIMD (aarch64 baseline).
    Neon,
    /// Scalable vectors, aarch64: the scalar body vectorized by LLVM, since
    /// stable Rust has no SVE intrinsics.
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
        matches!(
            self,
            Self::Scalar | Self::Neon | Self::Sve2 | Self::Avx2 | Self::Avx512
        )
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
pub enum LevelReason {
    /// The highest level of the machine, and it is implemented.
    Detected,
    /// The machine supports more, but that level is not implemented.
    Fallback,
    /// Limited by a caller-supplied ceiling.
    Ceiling,
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

/// Validated `i16` taps with `precision` fractional bits for the Q1.14
/// prototype (ADR-0021 rule 2). The caller guarantees the interval bound;
/// results are clamped to `i16` either way.
#[derive(Clone, Debug)]
pub struct TapsQ14 {
    source_pixels: usize,
    starts: Vec<u32>,
    offsets: Vec<u32>,
    weights: Vec<i16>,
    precision: u32,
}

impl TapsQ14 {
    /// # Errors
    /// [`KernelError::InvalidTaps`] for malformed tables, runs outside the
    /// source row, or a precision outside `1..=16`.
    pub fn new(
        source_pixels: usize,
        starts: Vec<u32>,
        offsets: Vec<u32>,
        weights: Vec<i16>,
        precision: u32,
    ) -> Result<Self, KernelError> {
        if !(1..=16).contains(&precision) {
            return Err(KernelError::InvalidTaps);
        }
        let float_weights = vec![0.0; weights.len()];
        let checked = Taps::new(source_pixels, starts, offsets, float_weights)?;
        Ok(Self {
            source_pixels,
            starts: checked.starts,
            offsets: checked.offsets,
            weights,
            precision,
        })
    }

    #[must_use]
    pub fn outputs(&self) -> usize {
        self.starts.len()
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
    /// The highest implemented level this machine supports.
    #[must_use]
    pub fn best() -> Self {
        Self::with_ceiling(IsaLevel::Avx512)
    }

    /// The highest implemented, supported level not above `ceiling`.
    #[must_use]
    pub fn with_ceiling(ceiling: IsaLevel) -> Self {
        let machine = machine_level();
        let candidates = [
            IsaLevel::Avx512,
            IsaLevel::Avx2,
            IsaLevel::Sve2,
            IsaLevel::Neon,
        ];
        let level = candidates
            .into_iter()
            .find(|&level| level <= ceiling && level.is_implemented() && supports(level))
            .unwrap_or(IsaLevel::Scalar);
        let reason = if ceiling < machine {
            LevelReason::Ceiling
        } else if level == machine {
            LevelReason::Detected
        } else {
            LevelReason::Fallback
        };
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

    #[must_use]
    pub fn level(&self) -> IsaLevel {
        self.level
    }

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

    /// Q1.14 prototype of `horizontal_rgba`: `i16` samples and weights,
    /// `i32` sums, rounded and shifted by the taps' precision.
    ///
    /// # Errors
    /// [`KernelError::LengthMismatch`] unless the slices match the taps.
    pub fn horizontal_rgba_q14(
        &self,
        source: &[i16],
        taps: &TapsQ14,
        out: &mut [i16],
    ) -> Result<(), KernelError> {
        if Some(source.len()) != taps.source_pixels.checked_mul(4)
            || Some(out.len()) != taps.outputs().checked_mul(4)
        {
            return Err(KernelError::LengthMismatch);
        }
        (self.table.horizontal_rgba_q14)(source, taps, out);
        Ok(())
    }

    /// Q1.14 prototype of `axpy`: `acc[i] += row[i] * weight` in `i32`.
    ///
    /// # Errors
    /// [`KernelError::LengthMismatch`] if the slices differ in length.
    pub fn axpy_q14(&self, acc: &mut [i32], row: &[i16], weight: i16) -> Result<(), KernelError> {
        if acc.len() != row.len() {
            return Err(KernelError::LengthMismatch);
        }
        (self.table.axpy_q14)(acc, row, weight);
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
