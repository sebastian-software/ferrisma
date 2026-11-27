//! Gray and gray-with-alpha input expanded to RGB and RGBA (owner decision
//! 2026-11-15, ADR-0043 D2): one operation for every format that carries
//! gray (JPEG, JPEG XL, AVIF), at one stage of the pipeline, after color
//! handling and before the graph, at the decoder's own sample precision.
//!
//! The expansion copies the gray sample into R, G and B and the alpha
//! sample after them, so it is exact: a typed gray signal (a JPEG XL enum
//! encoding, AVIF `nclx`) means the same color as R = G = B under the
//! RGB counterpart of that signal. A gray ICC profile is converted by the
//! rendered-ICC stage instead, which emits RGB itself (`cms`), so gray
//! never reaches an RGB profile and an RGB profile never reaches gray.

use crate::decoder::{ChannelLayout, DecodeReport, DecodeSource, DecodedRow, DecoderInfo};
use crate::discarded::DiscardedSet;
use crate::error::Error;
use crate::geometry::Rect;
use crate::memory::MemoryBudget;

pub(crate) struct GrayToRgb<'a> {
    source: Box<dyn DecodeSource + 'a>,
    info: DecoderInfo,
    alpha: bool,
}

impl<'a> GrayToRgb<'a> {
    /// Wrap a gray or gray-with-alpha source; any other layout is returned
    /// unchanged.
    pub(crate) fn wrap(source: Box<dyn DecodeSource + 'a>) -> Box<dyn DecodeSource + 'a> {
        let (layout, alpha) = match source.info().layout {
            ChannelLayout::Gray => (ChannelLayout::Rgb, false),
            ChannelLayout::GrayAlpha => (ChannelLayout::Rgba, true),
            _ => return source,
        };
        let mut info = source.info().clone();
        info.layout = layout;
        Box::new(Self {
            source,
            info,
            alpha,
        })
    }
}

/// Copy each gray sample into three color samples, and the alpha sample
/// after them; `out` has `3 + alpha` samples per input pixel.
fn expand<T: Copy>(gray: &[T], out: &mut [T], alpha: bool) {
    if alpha {
        for (dst, src) in out
            .as_chunks_mut::<4>()
            .0
            .iter_mut()
            .zip(gray.as_chunks::<2>().0)
        {
            *dst = [src[0], src[0], src[0], src[1]];
        }
    } else {
        for (dst, &src) in out.as_chunks_mut::<3>().0.iter_mut().zip(gray) {
            *dst = [src; 3];
        }
    }
}

impl DecodeSource for GrayToRgb<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }

    fn orientation(&self) -> crate::orientation::Orientation {
        self.source.orientation()
    }

    fn set_decode_threads(&mut self, threads: usize) {
        self.source.set_decode_threads(threads);
    }

    fn set_orientation_threads(&mut self, threads: usize) {
        self.source.set_orientation_threads(threads);
    }

    fn has_transposing_orientation(&self) -> bool {
        self.source.has_transposing_orientation()
    }

    fn discarded(&self) -> DiscardedSet {
        self.source.discarded()
    }

    fn streams_rows(&self) -> bool {
        self.source.streams_rows()
    }

    fn prepare_conversion(
        &mut self,
        expected_pixels: u64,
        budget: &MemoryBudget,
    ) -> Result<(), Error> {
        self.source.prepare_conversion(expected_pixels, budget)
    }

    fn set_downstream_headroom(&mut self, bytes: usize) {
        self.source.set_downstream_headroom(bytes);
    }

    fn read_region(
        &mut self,
        region: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        let (input, output) = if self.alpha { (2, 4) } else { (1, 3) };
        let width = usize::try_from(region.width).map_err(|_| Error::GeometryOverflow)?;
        let samples = width.checked_mul(output).ok_or(Error::GeometryOverflow)?;
        let alpha = self.alpha;
        // Rows arrive at the source's sample type; the other buffer stays
        // empty.
        let sample = self.info.sample;
        let float = sample == crate::decoder::SampleType::F32;
        let mut bytes = budget.allocate(if float { 0 } else { samples })?;
        let mut floats = budget.allocate_typed::<f32>(if float { samples } else { 0 })?;
        let mut report = self.source.read_region(region, budget, &mut |y, row| {
            if row.len() != width * input || row.sample_type() != sample {
                return Err(Error::IncompleteDecoderOutput);
            }
            match row {
                DecodedRow::U8(gray) => {
                    expand(gray, bytes.as_mut_slice(), alpha);
                    sink(y, DecodedRow::U8(bytes.as_slice()))
                }
                DecodedRow::F32(gray) => {
                    expand(gray, floats.as_mut_slice(), alpha);
                    sink(y, DecodedRow::F32(floats.as_slice()))
                }
            }
        })?;
        let sample_bytes = match self.info.sample {
            crate::decoder::SampleType::U8 => 1,
            crate::decoder::SampleType::F32 => 4,
        };
        report.output_sample_bytes = report
            .rows
            .checked_mul(u64::from(region.width))
            .and_then(|n| n.checked_mul(output as u64 * sample_bytes))
            .ok_or(Error::GeometryOverflow)?;
        report.peak_engine_managed_bytes = budget.snapshot().peak;
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::expand;

    #[test]
    fn expansion_copies_gray_into_rgb_and_keeps_alpha() {
        let mut rgb = [0_u8; 6];
        expand(&[7, 200], &mut rgb, false);
        assert_eq!(rgb, [7, 7, 7, 200, 200, 200]);
        let mut rgba = [0.0_f32; 8];
        expand(&[0.25, 0.5, -0.0, 1.0], &mut rgba, true);
        assert_eq!(
            rgba.map(f32::to_bits),
            [0.25, 0.25, 0.25, 0.5, -0.0, -0.0, -0.0, 1.0].map(f32::to_bits)
        );
    }
}
