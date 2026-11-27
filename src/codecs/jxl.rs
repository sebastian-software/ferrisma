//! JPEG XL through the independent local libjxl wrapper. Its decoder has no
//! crop request: each read reconstructs an upright, full-frame F32 image.
//! The wrapper's image-out callback retains only the requested upright
//! region, so the engine-managed buffer is region-sized (ADR-0032).

use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, CodecWork, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo,
    SampleType,
};
use crate::discarded::{Discarded, DiscardedSet};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::memory::MemoryBudget;
use ferrisma_jxl_decoder::{
    ColorSpace, EncodedColor, ImageInfo, Primaries, Region, Transfer, TransferFunction, WhitePoint,
    decode_into_with_threads, decode_region_into_with_threads, inspect,
};

pub(crate) struct JxlSource<'a> {
    bytes: &'a [u8],
    image: ImageInfo,
    info: DecoderInfo,
    max_pixels: u64,
    discarded: DiscardedSet,
    decode_threads: usize,
}

impl<'a> JxlSource<'a> {
    #[cfg(test)]
    pub(crate) fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        Self::new_bounded(bytes, 100_000_000)
    }
    pub(crate) fn new_bounded(bytes: &'a [u8], max_pixels: u64) -> Result<Self, Error> {
        // At inspection the wrapper's `Unsupported` means a layout outside
        // the engine contract: float or exponent samples, a color channel
        // count other than one or three, or extra channels besides one alpha.
        let image = inspect(bytes, max_pixels).map_err(|error| match error {
            ferrisma_jxl_decoder::Error::Unsupported => Error::UnsupportedLayout,
            error => map_error(error),
        })?;
        // PQ and HLG are refused from the transfer function itself (ADR-0016
        // rule 7, review of 2026-11-12): a gamut without an H.273 code point,
        // such as custom primaries or a white point other than D65 or DCI,
        // gives the synthesized approximation profile no `cicp` tag, and the
        // rendered-ICC stage would otherwise convert it as SDR. An
        // ICC-signalled DATA encoding has no enum transfer; it stays covered
        // by the profile checks of that stage.
        if matches!(image.data_transfer, Some(Transfer::Pq | Transfer::Hlg)) {
            return Err(Error::HdrTransfer);
        }
        let size = Size::new(image.width, image.height)?;
        // The bridge asks libjxl to un-premultiply associated alpha (its own
        // step, so the samples stay identical with the original), so alpha
        // always arrives straight; the conversion is reported below (owner
        // decision 2026-11-15, ADR-0043).
        let (layout, alpha) = match image.channels {
            1 => (ChannelLayout::Gray, AlphaMode::Opaque),
            2 => (ChannelLayout::GrayAlpha, AlphaMode::Straight),
            3 => (ChannelLayout::Rgb, AlphaMode::Opaque),
            4 => (ChannelLayout::Rgba, AlphaMode::Straight),
            _ => return Err(Error::UnsupportedLayout),
        };
        // A structurally signalled (enum) encoding with an H.273 code point
        // is reported as that typed signal, like AVIF `nclx`, and the
        // profile libjxl synthesizes from it is not reported beside it: it
        // is derived from the same enum and adds no evidence (owner
        // decision 2026-11-15, #222, ADR-0015 option 2). A file whose
        // codestream carries a real ICC profile keeps the rendered ICC,
        // whatever its DATA encoding, and so still needs `UseRenderedIcc`.
        let typed = if image.original_icc {
            None
        } else {
            image.data_encoding.and_then(h273)
        };
        let info = DecoderInfo {
            // Integer samples only (float is refused at inspection): the
            // deeper of color and alpha, so a deeper alpha is not lost.
            depth: u8::try_from(image.bits_per_sample.max(image.alpha_bits))
                .map_err(|_| Error::UnsupportedLayout)?,
            size,
            layout,
            sample: SampleType::F32,
            access: DecodeAccess::FullFrameFallback,
            alpha,
            rendered_icc: typed.is_none().then(|| image.rendered_icc.clone()),
            rendered_cicp: typed,
        };
        // ADR-0042 D2: an embedded profile, not a synthesized one.
        let mut discarded = super::jxl_container::container_discards(bytes);
        discarded.insert_if(image.original_icc, Discarded::IccProfile);
        discarded.insert_if(image.have_preview, Discarded::Preview);
        discarded.insert_if(image.alpha_premultiplied, Discarded::Unpremultiplied);
        Ok(Self {
            bytes,
            image,
            info,
            max_pixels,
            discarded,
            decode_threads: 1,
        })
    }
}

/// The H.273 code point (primaries, transfer, identity matrix, full range)
/// of an enum DATA encoding, or `None` when it has none (custom primaries or
/// white point, a gamma exponent, an unknown transfer). Only the code point
/// is named here; whether the engine converts it is the color policy's
/// decision, exactly as for AVIF `nclx`. JPEG XL's primaries and transfer
/// enums use the H.273 values; its P3 primaries are DCI-P3 (11) with the
/// DCI white point and Display P3 (12) with D65. A gray encoding has no
/// primaries: with the D65 white point its samples mean R = G = B in any
/// D65 RGB space, so it is named with sRGB primaries (1), as a monochrome
/// AVIF's `nclx` usually is, and follows the same policy rules as its RGB
/// counterpart (owner decision 2026-11-15, ADR-0043).
fn h273(encoding: EncodedColor) -> Option<[u8; 4]> {
    let primaries = match (
        encoding.color_space,
        encoding.white_point,
        encoding.primaries,
    ) {
        (ColorSpace::Rgb, WhitePoint::D65, Primaries::Srgb)
        | (ColorSpace::Gray, WhitePoint::D65, _) => 1,
        (ColorSpace::Rgb, WhitePoint::D65, Primaries::Bt2100) => 9,
        (ColorSpace::Rgb, WhitePoint::D65, Primaries::P3) => 12,
        (ColorSpace::Rgb, WhitePoint::Dci, Primaries::P3) => 11,
        _ => return None,
    };
    let transfer = match encoding.transfer {
        TransferFunction::Bt709 => 1,
        TransferFunction::Linear => 8,
        TransferFunction::Srgb => 13,
        TransferFunction::Pq => 16,
        TransferFunction::Dci => 17,
        TransferFunction::Hlg => 18,
        TransferFunction::Gamma | TransferFunction::Unknown => return None,
    };
    Some([primaries, transfer, 0, 1])
}

fn map_error(error: ferrisma_jxl_decoder::Error) -> Error {
    match error {
        ferrisma_jxl_decoder::Error::Unsupported | ferrisma_jxl_decoder::Error::ProfileTooLarge => {
            Error::UnsupportedCodecLayout
        }
        ferrisma_jxl_decoder::Error::Animation => Error::UnsupportedAnimation,
        ferrisma_jxl_decoder::Error::PixelLimit { pixels, limit } => {
            Error::InputPixelLimitExceeded { pixels, limit }
        }
        ferrisma_jxl_decoder::Error::BufferSize | ferrisma_jxl_decoder::Error::Decode => {
            Error::DecodeFailed
        }
        ferrisma_jxl_decoder::Error::Allocation => Error::AllocationFailed,
        ferrisma_jxl_decoder::Error::ThreadSpawn => Error::ThreadSpawnFailed,
    }
}

impl DecodeSource for JxlSource<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }

    fn discarded(&self) -> DiscardedSet {
        self.discarded
    }

    fn set_decode_threads(&mut self, threads: usize) {
        self.decode_threads = threads.max(1);
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
            decode_into_with_threads(
                self.bytes,
                &self.image,
                retained.as_mut_slice(),
                self.max_pixels,
                self.decode_threads,
            )
        } else {
            decode_region_into_with_threads(
                self.bytes,
                &self.image,
                wanted,
                retained.as_mut_slice(),
                self.max_pixels,
                self.decode_threads,
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

#[cfg(test)]
mod tests {
    use super::JxlSource;
    use crate::decoder::{DecodeSource, DecodedRow};
    use crate::geometry::Rect;
    use crate::memory::MemoryBudget;

    fn decode_bits(bytes: &[u8], region: Rect, threads: usize) -> Vec<u32> {
        let mut source = JxlSource::new_bounded(bytes, 1 << 24).unwrap();
        source.set_decode_threads(threads);
        let budget = MemoryBudget::new(64 << 20);
        let mut out = Vec::new();
        source
            .read_region(region, &budget, &mut |_, row| {
                let DecodedRow::F32(samples) = row else {
                    panic!("expected F32 rows from JxlSource");
                };
                out.extend(samples.iter().map(|sample| sample.to_bits()));
                Ok(())
            })
            .unwrap();
        assert_eq!(budget.snapshot().live, 0);
        out
    }

    #[test]
    fn thread_spawn_and_allocation_errors_map_to_distinct_engine_errors() {
        assert_eq!(
            super::map_error(ferrisma_jxl_decoder::Error::ThreadSpawn),
            crate::error::Error::ThreadSpawnFailed
        );
        assert_eq!(
            super::map_error(ferrisma_jxl_decoder::Error::Allocation),
            crate::error::Error::AllocationFailed
        );
    }

    #[test]
    fn set_decode_threads_clamps_to_at_least_one() {
        let bytes = include_bytes!("../../tests/fixtures/sample.jxl");
        let mut source = JxlSource::new_bounded(bytes, 35).unwrap();
        assert_eq!(source.decode_threads, 1);
        for (threads, expected) in [(0, 1), (1, 1), (4, 4), (8, 8), (0, 1)] {
            source.set_decode_threads(threads);
            assert_eq!(source.decode_threads, expected);
        }
    }

    #[test]
    fn decode_threads_do_not_change_samples_for_multigroup_full_and_crop() {
        for (name, bytes) in [
            (
                "vardct-multigroup",
                include_bytes!("../../tests/fixtures/jxl-deep/d10-2049x40-a0-d1.jxl").as_slice(),
            ),
            (
                "lossless-alpha-multigroup",
                include_bytes!("../../tests/fixtures/jxl-lossless/rgba-2049x40.jxl").as_slice(),
            ),
            (
                "lossless-rgb-multigroup",
                include_bytes!("../../tests/fixtures/jxl-lossless/rgb-300x260.jxl").as_slice(),
            ),
        ] {
            let source = JxlSource::new_bounded(bytes, 1 << 24).unwrap();
            let size = source.info().size;
            let full = Rect::new(0, 0, size.width, size.height).unwrap();
            let crop = Rect::new(
                size.width / 5,
                size.height / 5,
                (size.width / 2).max(1),
                (size.height / 2).max(1),
            )
            .unwrap();
            for region in [full, crop] {
                let serial = decode_bits(bytes, region, 1);
                for threads in [0, 2, 4, 8] {
                    assert_eq!(
                        decode_bits(bytes, region, threads),
                        serial,
                        "{name} region={region:?} threads={threads}"
                    );
                }
            }
        }
    }
}
