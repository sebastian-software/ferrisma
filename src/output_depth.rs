//! Output sample depth (ADR-0021, amendment of 2026-11-15): the rows the
//! executors hand to a sink, at 8 bits or deeper, and the one rule that
//! picks the depth for every output format (ADR-0043).
//!
//! An 8-bit output row is straight-alpha RGBA as `u8`, exactly as before
//! 2026-11-15. A deeper row is the same layout as native-endian `u16`
//! samples scaled to the target depth: values `0..=2^depth - 1`, never an
//! 8-bit value shifted or padded. This is the form libavif's `avifRGBImage`
//! (`depth` 10, 12 or 16 with `uint16_t` pixels) and libjxl's `JXL_TYPE_UINT16`
//! with `JXL_BIT_DEPTH_FROM_CODESTREAM` take.

use crate::error::Error;
use crate::memory::{ManagedBuffer, MemoryBudget};

/// The engine's own sample depth: what every 8-bit reader delivers and every
/// sink accepted before 2026-11-15.
pub(crate) const EIGHT: u8 = 8;
/// The deepest integer sample a built-in output writes (JPEG XL).
pub(crate) const MAX_DEPTH: u8 = 16;

/// The largest code at `depth` bits (`2^depth - 1`), for 1 to 16 bits.
pub(crate) fn max_code(depth: u8) -> u16 {
    debug_assert!((1..=MAX_DEPTH).contains(&depth));
    u16::MAX >> (16 - u32::from(depth.clamp(1, MAX_DEPTH)))
}

/// One finished output row: straight-alpha RGBA, four samples per pixel.
#[derive(Clone, Copy, Debug)]
pub(crate) enum OutputRow<'a> {
    /// 8-bit samples.
    U8(&'a [u8]),
    /// Samples of more than 8 bits in `u16`, scaled to the output depth.
    U16(&'a [u16]),
}

/// The executors' output row storage, at the plan's depth.
pub(crate) enum OutputBuffer {
    U8(ManagedBuffer<u8>),
    U16(ManagedBuffer<u16>),
}

impl OutputBuffer {
    /// `samples` output samples at `depth`, charged to `budget`.
    pub(crate) fn allocate(
        depth: u8,
        samples: usize,
        budget: &MemoryBudget,
    ) -> Result<Self, Error> {
        Ok(if depth > EIGHT {
            Self::U16(budget.allocate_typed::<u16>(samples)?)
        } else {
            Self::U8(budget.allocate_typed::<u8>(samples)?)
        })
    }

    pub(crate) fn row(&self) -> OutputRow<'_> {
        match self {
            Self::U8(buffer) => OutputRow::U8(buffer.as_slice()),
            Self::U16(buffer) => OutputRow::U16(buffer.as_slice()),
        }
    }
}

/// Adapt an 8-bit sink to the depth-generic executors: an 8-bit plan only
/// produces 8-bit rows, so a deeper row here is an internal error.
pub(crate) fn eight_bit_sink<'s>(
    mut sink: impl FnMut(u32, &[u8]) -> Result<(), Error> + 's,
) -> impl FnMut(u32, OutputRow<'_>) -> Result<(), Error> + 's {
    move |y, row| match row {
        OutputRow::U8(bytes) => sink(y, bytes),
        OutputRow::U16(_) => Err(Error::IncompleteOutput),
    }
}
