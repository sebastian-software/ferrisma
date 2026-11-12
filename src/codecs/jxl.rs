//! JPEG XL through the independent local libjxl wrapper. Its decoder has no
//! crop request: each read reconstructs an upright, full-frame F32 image.
//! The wrapper's image-out callback retains only the requested upright
//! region, so the engine-managed buffer is region-sized (ADR-0032).

use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, CodecWork, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo,
    SampleType,
};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::memory::MemoryBudget;
use ferrisma_jxl_decoder::{ImageInfo, Region, Transfer, decode_into, decode_region_into, inspect};

pub(crate) struct JxlSource<'a> {
    bytes: &'a [u8],
    image: ImageInfo,
    info: DecoderInfo,
    max_pixels: u64,
}

impl<'a> JxlSource<'a> {
    #[cfg(test)]
    pub(crate) fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        Self::new_bounded(bytes, 100_000_000)
    }
    pub(crate) fn new_bounded(bytes: &'a [u8], max_pixels: u64) -> Result<Self, Error> {
        let image = inspect(bytes, max_pixels).map_err(map_error)?;
        // PQ and HLG are refused from the transfer function itself (ADR-0016
        // rule 7, review of 2026-11-12): a gamut without an H.273 code point,
        // such as custom primaries or a white point other than D65 or DCI,
        // gives the synthesized approximation profile no `cicp` tag, and the
        // rendered-ICC stage would otherwise convert it as SDR. An
        // ICC-signalled DATA encoding has no enum transfer; it stays covered
        // by the profile checks of that stage.
        if matches!(image.data_transfer, Some(Transfer::Pq | Transfer::Hlg)) {
            return Err(Error::UnsupportedColorEncoding);
        }
        let size = Size::new(image.width, image.height)?;
        let (layout, alpha) = match image.channels {
            1 => (ChannelLayout::Gray, AlphaMode::Opaque),
            2 => (
                ChannelLayout::GrayAlpha,
                if image.alpha_premultiplied {
                    AlphaMode::Premultiplied
                } else {
                    AlphaMode::Straight
                },
            ),
            3 => (ChannelLayout::Rgb, AlphaMode::Opaque),
            4 => (
                ChannelLayout::Rgba,
                if image.alpha_premultiplied {
                    AlphaMode::Premultiplied
                } else {
                    AlphaMode::Straight
                },
            ),
            _ => return Err(Error::UnsupportedCodecLayout),
        };
        let info = DecoderInfo {
            size,
            layout,
            sample: SampleType::F32,
            access: DecodeAccess::FullFrameFallback,
            alpha,
            rendered_icc: Some(image.rendered_icc.clone()),
            rendered_cicp: None,
        };
        Ok(Self {
            bytes,
            image,
            info,
            max_pixels,
        })
    }
}

fn map_error(error: ferrisma_jxl_decoder::Error) -> Error {
    match error {
        ferrisma_jxl_decoder::Error::Unsupported => Error::UnsupportedCodecLayout,
        ferrisma_jxl_decoder::Error::Animation => Error::UnsupportedAnimation,
        ferrisma_jxl_decoder::Error::PixelLimit { pixels, limit } => {
            Error::InputPixelLimitExceeded { pixels, limit }
        }
        ferrisma_jxl_decoder::Error::BufferSize | ferrisma_jxl_decoder::Error::Decode => {
            Error::DecodeFailed
        }
        ferrisma_jxl_decoder::Error::Allocation => Error::AllocationFailed,
    }
}

impl DecodeSource for JxlSource<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }

    fn read_region(
        &mut self,
        region: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        if !region.fits(self.info.size) {
            return Err(Error::OutOfBounds);
        }
        // The decoder reconstructs the full frame; only the requested
        // upright rows and columns are retained, in a budgeted buffer. A
        // full-frame request uses the output-buffer path, which yields the
        // same samples without a callback per run (per pixel for the
        // transposing orientations).
        let wanted = Region {
            x: region.x,
            y: region.y,
            width: region.width,
            height: region.height,
        };
        let full_frame = wanted == Region::full(&self.image);
        let samples = self.image.region_sample_count(wanted).map_err(map_error)?;
        let mut retained = budget.allocate_typed::<f32>(samples)?;
        if full_frame {
            decode_into(
                self.bytes,
                &self.image,
                retained.as_mut_slice(),
                self.max_pixels,
            )
        } else {
            decode_region_into(
                self.bytes,
                &self.image,
                wanted,
                retained.as_mut_slice(),
                self.max_pixels,
            )
        }
        .map_err(map_error)?;
        let row_len = region.width as usize * self.image.channels as usize;
        for (y, row) in (region.y..).zip(retained.as_slice().chunks_exact(row_len)) {
            sink(y, DecodedRow::F32(row))?;
        }
        Ok(DecodeReport {
            requested: region,
            access: DecodeAccess::FullFrameFallback,
            rows: u64::from(region.height),
            output_sample_bytes: u64::from(region.width)
                * u64::from(region.height)
                * u64::from(self.image.channels)
                * 4,
            // Only a full-frame request retains a full-frame buffer.
            known_full_frame_buffers: u32::from(full_frame),
            peak_engine_managed_bytes: budget.snapshot().peak,
            opaque_codec_memory_unbounded: true,
            engine_buffer_bytes: 0,
            engine_copied_bytes: 0,
            work: CodecWork::full_frame(region, self.info.size),
        })
    }
}
