//! JPEG XL through the independent local libjxl wrapper. Its decoder has no
//! crop request: each read reconstructs an upright, full-frame F32 image.

use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo, SampleType,
};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::memory::MemoryBudget;
use ferrixel_jxl_decoder::{ImageInfo, decode_into, inspect};

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

fn map_error(error: ferrixel_jxl_decoder::Error) -> Error {
    match error {
        ferrixel_jxl_decoder::Error::Unsupported => Error::UnsupportedCodecLayout,
        ferrixel_jxl_decoder::Error::PixelLimit { pixels, limit } => {
            Error::InputPixelLimitExceeded { pixels, limit }
        }
        ferrixel_jxl_decoder::Error::BufferSize | ferrixel_jxl_decoder::Error::Decode => {
            Error::DecodeFailed
        }
        ferrixel_jxl_decoder::Error::Allocation => Error::AllocationFailed,
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
        let samples = self.image.sample_count().map_err(map_error)?;
        let mut frame = budget.allocate_typed::<f32>(samples)?;
        decode_into(
            self.bytes,
            &self.image,
            frame.as_mut_slice(),
            self.max_pixels,
        )
        .map_err(map_error)?;
        let channels = self.image.channels as usize;
        let stride = self.image.width as usize * channels;
        let row_start = region.x as usize * channels;
        let row_len = region.width as usize * channels;
        for y in region.y..region.bottom() {
            let start = y as usize * stride + row_start;
            sink(
                y,
                DecodedRow::F32(&frame.as_slice()[start..start + row_len]),
            )?;
        }
        Ok(DecodeReport {
            requested: region,
            access: DecodeAccess::FullFrameFallback,
            rows: u64::from(region.height),
            output_sample_bytes: u64::from(region.width)
                * u64::from(region.height)
                * u64::from(self.image.channels)
                * 4,
            known_full_frame_buffers: 1,
            peak_engine_managed_bytes: budget.snapshot().peak,
            opaque_codec_memory_unbounded: true,
            engine_buffer_bytes: 0,
        })
    }
}
