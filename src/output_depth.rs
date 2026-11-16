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
#[cfg(test)] // The 8-bit contract of the executor tests.
pub(crate) fn eight_bit_sink<'s>(
    mut sink: impl FnMut(u32, &[u8]) -> Result<(), Error> + 's,
) -> impl FnMut(u32, OutputRow<'_>) -> Result<(), Error> + 's {
    move |y, row| match row {
        OutputRow::U8(bytes) => sink(y, bytes),
        OutputRow::U16(_) => Err(Error::IncompleteOutput),
    }
}

/// The depth rule (ADR-0021 amendment and ADR-0043 decided application of
/// 2026-11-15), one function for every output: an explicit `requested`
/// depth must be one the output admits (`admitted`, ascending, 8 first);
/// without one the output takes the largest admitted depth that is at most
/// the input's, and 8 when the input is shallower than every admitted
/// depth (8 holds it). The caller reports `PrecisionReduced` when the
/// result is below `input`.
pub(crate) fn resolve(requested: Option<u8>, input: u8, admitted: &[u8]) -> Result<u8, Error> {
    match requested {
        Some(depth) if admitted.contains(&depth) => Ok(depth),
        Some(_) => Err(Error::InvalidEncodeOptions),
        None => Ok(admitted
            .iter()
            .copied()
            .filter(|&depth| depth <= input)
            .max()
            .or_else(|| admitted.first().copied())
            .unwrap_or(EIGHT)),
    }
}

#[cfg(test)]
mod tests {
    use super::{max_code, resolve};

    #[test]
    fn the_depth_rule_follows_the_input_and_caps_at_the_output() {
        const AVIF: &[u8] = &[8, 10, 12];
        let jxl: Vec<u8> = (8..=16).collect();
        for (input, avif, jpeg_xl) in [
            (1, 8, 8),
            (8, 8, 8),
            (9, 8, 9),
            (10, 10, 10),
            (12, 12, 12),
            (14, 12, 14),
            (16, 12, 16),
        ] {
            assert_eq!(resolve(None, input, AVIF), Ok(avif), "{input}");
            assert_eq!(resolve(None, input, &jxl), Ok(jpeg_xl), "{input}");
            assert_eq!(resolve(None, input, &[8]), Ok(8), "{input}");
        }
        assert_eq!(resolve(Some(10), 8, AVIF), Ok(10));
        assert_eq!(resolve(Some(8), 12, AVIF), Ok(8));
        assert!(resolve(Some(9), 8, AVIF).is_err());
        assert!(resolve(Some(16), 16, AVIF).is_err());
        assert!(resolve(Some(10), 10, &[8]).is_err());
        assert_eq!(max_code(8), 255);
        assert_eq!(max_code(10), 1023);
        assert_eq!(max_code(12), 4095);
        assert_eq!(max_code(16), 65_535);
    }
}
