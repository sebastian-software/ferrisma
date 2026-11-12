//! Read-only AVIF bridge via the workspace libavif port's libaom decoder.
//!
//! The constructor only parses container metadata and the AV1 sequence
//! header, so callers can apply their color policy before any pixel decode.
//! A region request decodes a full frame into an engine-budgeted RGB buffer;
//! codec scratch remains opaque and outside that budget.
//!
//! libavif converts YUV to full-range RGB with nearest-neighbor chroma
//! upsampling. The adapter reports that rendered meaning as CICP
//! `[primaries, transfer, 0, 1]` and any `colr`
//! ICC profile. It fails closed when the container `nclx` disagrees with the
//! bitstream, for monochrome input and for `clap`. `irot`/`imir` are reported
//! as the source orientation and applied by the pipeline (ADR-0019).
//! 10/12-bit input is emitted as normalized float samples.

use ferrisma_avif_read::{Decoder, ReadError};

use super::avif_meta::{MetaError, PrimaryProperties, SequenceColor, primary_properties};
use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo, SampleType,
};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::memory::MemoryBudget;
use crate::orientation::Orientation;

fn bridge_error(error: ReadError) -> Error {
    match error {
        ReadError::Allocation => Error::AllocationFailed,
        ReadError::InvalidInput | ReadError::OutputSize => Error::DecodeFailed,
        ReadError::Unsupported => Error::UnsupportedCodecLayout,
        ReadError::PixelLimit { pixels, limit } => Error::InputPixelLimitExceeded { pixels, limit },
    }
}

/// H.273 table 4 treats these matrix codes as equivalent BT.601 transforms.
fn same_decode_matrix(signalled: u16, bitstream: u8) -> bool {
    let bt601 = |code: u16| matches!(code, 2 | 5 | 6);
    signalled == u16::from(bitstream) || (bt601(signalled) && bt601(u16::from(bitstream)))
}

/// Describe the RGB samples emitted for this file as CICP, or `None` when
/// neither the container nor the bitstream specifies primaries or transfer.
pub(crate) fn rendered_cicp(
    properties: &PrimaryProperties,
    sequence: SequenceColor,
) -> Result<Option<[u8; 4]>, Error> {
    // Constant-luminance BT.2020 needs a separately verified output contract.
    if sequence.matrix == 10 {
        return Err(Error::UnsupportedCodecLayout);
    }
    let (primaries, transfer) = if let Some(nclx) = properties.nclx {
        let field_conflicts =
            |container: u16, bitstream: u8| bitstream != 2 && container != u16::from(bitstream);
        if !same_decode_matrix(nclx.matrix, sequence.matrix)
            || nclx.full_range != sequence.full_range
            || field_conflicts(nclx.primaries, sequence.primaries)
            || field_conflicts(nclx.transfer, sequence.transfer)
        {
            return Err(Error::ContradictoryColorSignals);
        }
        (
            u8::try_from(nclx.primaries).map_err(|_| Error::UnsupportedColorEncoding)?,
            u8::try_from(nclx.transfer).map_err(|_| Error::UnsupportedColorEncoding)?,
        )
    } else {
        (sequence.primaries, sequence.transfer)
    };
    if primaries == 2 && transfer == 2 {
        return Ok(None);
    }
    // Emitted samples are RGB (identity matrix) at full range.
    Ok(Some([primaries, transfer, 0, 1]))
}

pub(crate) struct AvifSource<'a> {
    decoder: Decoder<'a>,
    decoded: bool,
    info: DecoderInfo,
    orientation: Orientation,
}

impl<'a> AvifSource<'a> {
    /// Parse metadata and check frame dimensions *before* the decoder
    /// allocates a pixel surface. This is not an RSS limit: parsing and the
    /// later decode can still allocate opaque codec memory.
    pub(crate) fn new_bounded(bytes: &'a [u8], max_pixels: u64) -> Result<Self, Error> {
        let properties = primary_properties(bytes).map_err(|error| match error {
            MetaError::Allocation => Error::AllocationFailed,
            MetaError::ProfileTooLarge => Error::UnsupportedColorEncoding,
            _ => Error::DecodeFailed,
        })?;
        // irot/imir are applied by the pipeline (ADR-0019); clap and an imir
        // that precedes irot are not supported yet.
        if properties.clean_aperture || properties.mirror_before_rotation {
            return Err(Error::UnsupportedImageTransform);
        }
        let orientation = Orientation::from_avif(properties.rotation, properties.mirror)
            .ok_or(Error::UnsupportedImageTransform)?;
        let (width, height) = properties.extent.ok_or(Error::DecodeFailed)?;
        let size = Size::new(width, height)?;
        let rendered_pixels = u64::from(size.width) * u64::from(size.height);
        if rendered_pixels > max_pixels {
            return Err(Error::InputPixelLimitExceeded {
                pixels: rendered_pixels,
                limit: max_pixels,
            });
        }
        let (decoder, decoded_info) = Decoder::parse(bytes, max_pixels).map_err(bridge_error)?;
        let coded_pixels = u64::from(decoded_info.sequence.max_width)
            .checked_mul(u64::from(decoded_info.sequence.max_height))
            .ok_or(Error::GeometryOverflow)?;
        if coded_pixels > max_pixels {
            return Err(Error::InputPixelLimitExceeded {
                pixels: coded_pixels,
                limit: max_pixels,
            });
        }
        let sequence = SequenceColor {
            bit_depth: decoded_info.sequence.depth,
            monochrome: decoded_info.sequence.monochrome,
            primaries: decoded_info.sequence.primaries,
            transfer: decoded_info.sequence.transfer,
            matrix: decoded_info.sequence.matrix,
            full_range: decoded_info.sequence.full_range,
        };
        if sequence.monochrome {
            return Err(Error::UnsupportedCodecLayout);
        }
        let rendered_cicp = rendered_cicp(&properties, sequence)?;
        let has_alpha = decoded_info.alpha;
        if decoded_info.width != width
            || decoded_info.height != height
            || decoded_info.depth != u32::from(sequence.bit_depth)
            || decoded_info.image_count != 1
            || decoded_info.alpha_premultiplied && !has_alpha
        {
            return Err(Error::DecodeFailed);
        }
        if decoded_info.transform_flags & 2 != 0 {
            return Err(Error::UnsupportedImageTransform);
        }
        // libavif's parsed image describes container signaling. Retain the
        // independent AV1 sequence-header check above for emitted RGB meaning.
        if let Some(nclx) = properties.nclx
            && (decoded_info.primaries != nclx.primaries
                || decoded_info.transfer != nclx.transfer
                || decoded_info.matrix != nclx.matrix
                || decoded_info.full_range != nclx.full_range)
        {
            return Err(Error::ContradictoryColorSignals);
        }
        if decoded_info.icc != properties.icc {
            return Err(Error::ContradictoryColorSignals);
        }
        let layout = if has_alpha {
            ChannelLayout::Rgba
        } else {
            ChannelLayout::Rgb
        };
        let info = DecoderInfo {
            size,
            layout,
            sample: if sequence.bit_depth > 8 {
                SampleType::F32
            } else {
                SampleType::U8
            },
            alpha: if has_alpha {
                // The wrapper requests straight RGB output from libavif.
                AlphaMode::Straight
            } else {
                AlphaMode::Opaque
            },
            access: DecodeAccess::FullFrameFallback,
            rendered_icc: properties.icc,
            rendered_cicp,
        };
        Ok(Self {
            decoder,
            decoded: false,
            info,
            orientation,
        })
    }

    #[cfg(test)]
    pub(crate) fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        Self::new_bounded(bytes, 100_000_000)
    }
}

impl DecodeSource for AvifSource<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }

    fn orientation(&self) -> Orientation {
        self.orientation
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
        let size = self.info.size;
        let channels = usize::from(self.info.layout.channels());
        let full_samples = usize::try_from(size.width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(usize::try_from(size.height).map_err(|_| Error::GeometryOverflow)?)
            .and_then(|pixels| pixels.checked_mul(channels))
            .ok_or(Error::GeometryOverflow)?;
        let row_samples = usize::try_from(region.width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(channels)
            .ok_or(Error::GeometryOverflow)?;
        let left = usize::try_from(region.x).map_err(|_| Error::GeometryOverflow)?;
        let stride = usize::try_from(size.width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(channels)
            .ok_or(Error::GeometryOverflow)?;
        if self.info.sample == SampleType::U8 {
            let mut frame = budget.allocate(full_samples)?;
            self.decoder
                .decode_u8(
                    frame.as_mut_slice(),
                    size.width,
                    size.height,
                    self.info.layout == ChannelLayout::Rgba,
                )
                .map_err(bridge_error)?;
            self.decoded = true;
            for y in region.y..region.bottom() {
                let start = usize::try_from(y)
                    .map_err(|_| Error::GeometryOverflow)?
                    .checked_mul(stride)
                    .and_then(|n| n.checked_add(left * channels))
                    .ok_or(Error::GeometryOverflow)?;
                sink(
                    y,
                    DecodedRow::U8(&frame.as_slice()[start..start + row_samples]),
                )?;
            }
        } else {
            let mut frame = budget.allocate_typed::<u16>(full_samples)?;
            let mut scratch = budget.allocate_typed::<f32>(row_samples)?;
            self.decoder
                .decode_u16(
                    frame.as_mut_slice(),
                    size.width,
                    size.height,
                    self.info.layout == ChannelLayout::Rgba,
                )
                .map_err(bridge_error)?;
            self.decoded = true;
            for y in region.y..region.bottom() {
                let start = usize::try_from(y)
                    .map_err(|_| Error::GeometryOverflow)?
                    .checked_mul(stride)
                    .and_then(|n| n.checked_add(left * channels))
                    .ok_or(Error::GeometryOverflow)?;
                for (dst, &sample) in scratch
                    .as_mut_slice()
                    .iter_mut()
                    .zip(&frame.as_slice()[start..start + row_samples])
                {
                    *dst = f32::from(sample) / 65_535.0;
                }
                sink(y, DecodedRow::F32(scratch.as_slice()))?;
            }
        }
        Ok(DecodeReport {
            requested: region,
            access: DecodeAccess::FullFrameFallback,
            rows: u64::from(region.height),
            output_sample_bytes: u64::from(region.width)
                * u64::from(region.height)
                * channels as u64
                * if self.info.sample == SampleType::F32 {
                    4
                } else {
                    1
                },
            known_full_frame_buffers: 1,
            peak_engine_managed_bytes: budget.snapshot().peak,
            opaque_codec_memory_unbounded: true,
            engine_buffer_bytes: 0,
        })
    }
}

#[cfg(all(test, feature = "avif-write"))]
mod tests {
    use super::{AvifSource, Decoder};
    use crate::avif_sink::{AvifSink, test_settings};
    use crate::decoder::{DecodeSource, DecodedRow, SampleType};
    use crate::error::Error;
    use crate::geometry::{Rect, Size};
    use crate::memory::MemoryBudget;

    fn encoded() -> Vec<u8> {
        let budget = MemoryBudget::new(8 << 20);
        let mut sink = AvifSink::new(
            Size::new(6, 4).unwrap(),
            false,
            test_settings(90.0, 10),
            &budget,
        )
        .unwrap();
        for row in 0..4 {
            sink.write_row(row, &[200, 40, 90, 255].repeat(6)).unwrap();
        }
        sink.finish(&budget).unwrap().bytes
    }

    fn nclx_offset(file: &[u8]) -> usize {
        file.windows(8).position(|w| w == b"colrnclx").unwrap() + 8
    }

    #[test]
    fn real_encoder_output_reports_srgb_before_decoding() {
        let file = encoded();
        let mut source = AvifSource::new(&file).unwrap();
        assert_eq!(source.info().rendered_cicp, Some([1, 13, 0, 1]));
        assert_eq!(source.info().size, Size::new(6, 4).unwrap());
        assert!(!source.decoded, "metadata must not decode pixels");
        let budget = MemoryBudget::new(1 << 16);
        let mut rows = 0;
        source
            .read_region(Rect::new(0, 0, 6, 4).unwrap(), &budget, &mut |_, _| {
                rows += 1;
                Ok(())
            })
            .unwrap();
        assert_eq!(rows, 4);
        assert!(source.decoded);
    }

    #[test]
    fn full_frame_is_budgeted_and_released_on_cancellation() {
        let file = encoded();
        let mut source = AvifSource::new(&file).unwrap();
        let frame_bytes = 6 * 4 * 3 * 2;
        let row_bytes = 2 * 3 * 4;
        let too_small = MemoryBudget::new(frame_bytes - 1);
        assert!(matches!(
            source.read_region(Rect::new(1, 1, 2, 2).unwrap(), &too_small, &mut |_, _| Ok(
                ()
            )),
            Err(Error::BudgetExceeded { .. })
        ));
        assert_eq!(too_small.snapshot().live, 0);

        let budget = MemoryBudget::new(frame_bytes + row_bytes);
        let report = source
            .read_region(Rect::new(1, 1, 2, 2).unwrap(), &budget, &mut |_, _| Ok(()))
            .unwrap();
        assert_eq!(report.known_full_frame_buffers, 1);
        assert_eq!(report.peak_engine_managed_bytes, frame_bytes + row_bytes);
        assert_eq!(budget.snapshot().live, 0);
        assert_eq!(
            source.read_region(Rect::new(1, 1, 2, 2).unwrap(), &budget, &mut |_, _| {
                Err(Error::DecodeFailed)
            }),
            Err(Error::DecodeFailed)
        );
        assert_eq!(budget.snapshot().live, 0);
    }

    /// Identity-matrix RGB coding makes ravif write an explicit `colr` box.
    fn encoded_with_nclx() -> Vec<u8> {
        let pixels = vec![ravif::RGB8::new(200, 40, 90); 6 * 4];
        ravif::Encoder::new()
            .with_quality(90.0)
            .with_speed(10)
            .with_bit_depth(ravif::BitDepth::Ten)
            .with_internal_color_model(ravif::ColorModel::RGB)
            .encode_rgb(ravif::Img::new(pixels.as_slice(), 6, 4))
            .unwrap()
            .avif_file
    }

    #[test]
    fn explicit_nclx_ten_bit_rgb_is_normalized_and_reported_as_srgb() {
        let file = encoded_with_nclx();
        nclx_offset(&file);
        let mut source = AvifSource::new(&file).unwrap();
        assert_eq!(source.info().rendered_cicp, Some([1, 13, 0, 1]));
        assert_eq!(source.info().sample, SampleType::F32);
        let budget = MemoryBudget::new(1 << 16);
        let mut first = Vec::new();
        source
            .read_region(Rect::new(0, 0, 6, 1).unwrap(), &budget, &mut |_, row| {
                if let DecodedRow::F32(samples) = row {
                    first.extend_from_slice(&samples[..3]);
                }
                Ok(())
            })
            .unwrap();
        for (actual, expected) in first.iter().zip([200.0_f32, 40.0, 90.0]) {
            assert!((actual * 255.0 - expected).abs() < 3.0, "{first:?}");
        }
    }

    #[test]
    fn container_signal_that_contradicts_the_bitstream_is_rejected() {
        let file = encoded_with_nclx();
        let offset = nclx_offset(&file);
        // Declare PQ transfer while the coded sequence header says sRGB.
        let mut pq = file.clone();
        pq[offset + 2..offset + 4].copy_from_slice(&16_u16.to_be_bytes());
        assert_eq!(
            AvifSource::new(&pq).err(),
            Some(Error::ContradictoryColorSignals)
        );
        // Declare a YUV matrix although the payload is identity-coded RGB.
        let mut matrix = file.clone();
        matrix[offset + 4..offset + 6].copy_from_slice(&1_u16.to_be_bytes());
        assert_eq!(
            AvifSource::new(&matrix).err(),
            Some(Error::ContradictoryColorSignals)
        );
    }

    #[test]
    fn pinned_grid_and_twelve_bit_still_images_read_through_engine() {
        let fixtures: &[(&[u8], SampleType)] = &[
            (
                include_bytes!(
                    "../../codecs/libavif-rs/upstream/libavif/tests/data/sofa_grid1x5_420.avif"
                ),
                SampleType::U8,
            ),
            (
                include_bytes!(
                    "../../codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/avif/fox.profile2.12bpc.yuv444.avif"
                ),
                SampleType::F32,
            ),
        ];
        for (bytes, sample) in fixtures {
            let mut source = AvifSource::new(bytes).unwrap();
            assert_eq!(source.info().sample, *sample);
            let size = source.info().size;
            let budget = MemoryBudget::new(64 << 20);
            let mut rows = 0;
            source
                .read_region(
                    Rect::new(0, 0, size.width, size.height).unwrap(),
                    &budget,
                    &mut |_, _| {
                        rows += 1;
                        Ok(())
                    },
                )
                .unwrap();
            assert_eq!(rows, size.height);
        }
    }

    #[test]
    fn twelve_bit_rows_normalize_pinned_c_oracle_sixteen_bit_samples() {
        let bytes = include_bytes!(
            "../../codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/avif/fox.profile2.12bpc.yuv444.avif"
        );
        let (mut decoder, raw_info) = Decoder::parse(bytes, 100_000_000).unwrap();
        let mut raw = vec![0_u16; raw_info.width as usize * raw_info.height as usize * 3];
        decoder
            .decode_u16(&mut raw, raw_info.width, raw_info.height, false)
            .unwrap();
        let mut source = AvifSource::new(bytes).unwrap();
        let budget = MemoryBudget::new(64 << 20);
        source
            .read_region(Rect::new(0, 0, 2, 1).unwrap(), &budget, &mut |_, row| {
                let DecodedRow::F32(samples) = row else {
                    panic!("12-bit samples must be normalized floats");
                };
                for (actual, coded) in samples.iter().zip(&raw[..6]) {
                    assert_eq!(*actual, f32::from(*coded) / 65_535.0);
                }
                Ok(())
            })
            .unwrap();
    }
}
