//! Read-only AVIF bridge via `avif-decode` (BSD-3-Clause).
//!
//! The constructor only parses container metadata and the AV1 sequence
//! header, so callers can apply their color policy before any pixel decode.
//! The first region request decodes the whole frame inside the codec; that
//! allocation is NOT constrained by Ferrixel's managed-memory budget.
//!
//! `avif-decode` converts YUV to full-range RGB with the bitstream's matrix
//! and range and keeps the coded primaries and transfer. The adapter reports
//! that rendered meaning as CICP `[primaries, transfer, 0, 1]` and any `colr`
//! ICC profile. It fails closed when the container `nclx` disagrees with the
//! bitstream, for monochrome input and for `irot`/`imir`/`clap` transforms.
//! 10/12-bit input is emitted as normalized float samples.

use avif_decode::{Decoder, Image};

use super::avif_meta::{PrimaryProperties, SequenceColor, primary_properties, sequence_color};
use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo, SampleType,
};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::memory::MemoryBudget;

/// Whether `avif-decode` applies the same YUV matrix for both codes. It
/// decodes unspecified matrix coefficients as BT.601, and BT.470BG uses the
/// BT.601 matrix (ITU-T H.273 table 4).
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
    // avif-decode maps BT.2020 constant luminance onto the non-constant matrix.
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
    bytes: &'a [u8],
    image: Option<Image>,
    info: DecoderInfo,
}

impl<'a> AvifSource<'a> {
    /// Parse metadata and check frame dimensions *before* the decoder
    /// allocates a pixel surface. This is not an RSS limit: parsing and the
    /// later decode can still allocate opaque codec memory.
    pub(crate) fn new_bounded(bytes: &'a [u8], max_pixels: u64) -> Result<Self, Error> {
        let mut reader = bytes;
        let parsed =
            avif_parse::AvifData::from_reader(&mut reader).map_err(|_| Error::DecodeFailed)?;
        let metadata = parsed
            .primary_item_metadata()
            .map_err(|_| Error::DecodeFailed)?;
        let coded_pixels = u64::from(metadata.max_frame_width.get())
            .checked_mul(u64::from(metadata.max_frame_height.get()))
            .ok_or(Error::GeometryOverflow)?;
        if coded_pixels > max_pixels {
            return Err(Error::InputPixelLimitExceeded {
                pixels: coded_pixels,
                limit: max_pixels,
            });
        }
        let sequence = sequence_color(&parsed.primary_item).map_err(|_| Error::DecodeFailed)?;
        let has_alpha = parsed.alpha_item.is_some();
        // Parsing above holds compressed AV1 data temporarily; drop it before
        // decoding. The downstream decoder parses the AVIF again.
        drop(parsed);
        let properties = primary_properties(bytes).map_err(|_| Error::DecodeFailed)?;
        if properties.rotation != 0 || properties.mirror.is_some() || properties.clean_aperture {
            return Err(Error::UnsupportedImageTransform);
        }
        if sequence.monochrome {
            return Err(Error::UnsupportedCodecLayout);
        }
        let (width, height) = properties.extent.ok_or(Error::DecodeFailed)?;
        let size = Size::new(width, height)?;
        let rendered_pixels = u64::from(size.width) * u64::from(size.height);
        if rendered_pixels > max_pixels {
            return Err(Error::InputPixelLimitExceeded {
                pixels: rendered_pixels,
                limit: max_pixels,
            });
        }
        let rendered_cicp = rendered_cicp(&properties, sequence)?;
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
                // avif-decode unpremultiplies images signaled as premultiplied.
                AlphaMode::Straight
            } else {
                AlphaMode::Opaque
            },
            access: DecodeAccess::FullFrameFallback,
            rendered_icc: properties.icc,
            rendered_cicp,
        };
        Ok(Self {
            bytes,
            image: None,
            info,
        })
    }

    pub(crate) fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        Self::new_bounded(bytes, 100_000_000)
    }

    /// Decode once and verify that the frame matches the advertised info.
    fn decoded(&mut self) -> Result<&Image, Error> {
        if self.image.is_none() {
            let image = Decoder::from_avif(self.bytes)
                .map_err(|_| Error::DecodeFailed)?
                .to_image()
                .map_err(|_| Error::DecodeFailed)?;
            let (width, height, layout, sample) = match &image {
                Image::Rgb8(f) => (f.width(), f.height(), ChannelLayout::Rgb, SampleType::U8),
                Image::Rgba8(f) => (f.width(), f.height(), ChannelLayout::Rgba, SampleType::U8),
                Image::Rgb16(f) => (f.width(), f.height(), ChannelLayout::Rgb, SampleType::F32),
                Image::Rgba16(f) => (f.width(), f.height(), ChannelLayout::Rgba, SampleType::F32),
                _ => return Err(Error::UnsupportedCodecLayout),
            };
            let size = self.info.size;
            if usize::try_from(size.width) != Ok(width)
                || usize::try_from(size.height) != Ok(height)
                || layout != self.info.layout
                || sample != self.info.sample
            {
                return Err(Error::DecodeFailed);
            }
            self.image = Some(image);
        }
        self.image.as_ref().ok_or(Error::DecodeFailed)
    }
}

/// Convert the decoder-owned full frame into borrowed interleaved output
/// rows. The temporary row buffers stay under Ferrixel's managed budget.
fn emit_decoded_rows(
    image: &Image,
    region: Rect,
    width: usize,
    scratch_u8: &mut Option<crate::memory::ManagedBuffer<u8>>,
    scratch_f32: &mut Option<crate::memory::ManagedBuffer<f32>>,
    sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
) -> Result<(), Error> {
    let left = usize::try_from(region.x).map_err(|_| Error::GeometryOverflow)?;
    for y in region.y..region.bottom() {
        let row_index = usize::try_from(y).map_err(|_| Error::GeometryOverflow)?;
        match image {
            Image::Rgb8(frame) => {
                let pixels = &frame[row_index][left..left + width];
                let scratch = scratch_u8.as_mut().ok_or(Error::UnsupportedCodecLayout)?;
                for (dst, pixel) in scratch
                    .as_mut_slice()
                    .as_chunks_mut::<3>()
                    .0
                    .iter_mut()
                    .zip(pixels)
                {
                    dst.copy_from_slice(&[pixel.r, pixel.g, pixel.b]);
                }
            }
            Image::Rgba8(frame) => {
                let pixels = &frame[row_index][left..left + width];
                let scratch = scratch_u8.as_mut().ok_or(Error::UnsupportedCodecLayout)?;
                for (dst, pixel) in scratch
                    .as_mut_slice()
                    .as_chunks_mut::<4>()
                    .0
                    .iter_mut()
                    .zip(pixels)
                {
                    dst.copy_from_slice(&[pixel.r, pixel.g, pixel.b, pixel.a]);
                }
            }
            Image::Rgb16(frame) => {
                let pixels = &frame[row_index][left..left + width];
                let scratch = scratch_f32.as_mut().ok_or(Error::UnsupportedCodecLayout)?;
                for (dst, pixel) in scratch
                    .as_mut_slice()
                    .as_chunks_mut::<3>()
                    .0
                    .iter_mut()
                    .zip(pixels)
                {
                    dst.copy_from_slice(&[
                        f32::from(pixel.r) / 65_535.0,
                        f32::from(pixel.g) / 65_535.0,
                        f32::from(pixel.b) / 65_535.0,
                    ]);
                }
            }
            Image::Rgba16(frame) => {
                let pixels = &frame[row_index][left..left + width];
                let scratch = scratch_f32.as_mut().ok_or(Error::UnsupportedCodecLayout)?;
                for (dst, pixel) in scratch
                    .as_mut_slice()
                    .as_chunks_mut::<4>()
                    .0
                    .iter_mut()
                    .zip(pixels)
                {
                    dst.copy_from_slice(&[
                        f32::from(pixel.r) / 65_535.0,
                        f32::from(pixel.g) / 65_535.0,
                        f32::from(pixel.b) / 65_535.0,
                        f32::from(pixel.a) / 65_535.0,
                    ]);
                }
            }
            _ => return Err(Error::UnsupportedCodecLayout),
        }
        if let Some(scratch) = &scratch_u8 {
            sink(y, DecodedRow::U8(scratch.as_slice()))?;
        } else if let Some(scratch) = &scratch_f32 {
            sink(y, DecodedRow::F32(scratch.as_slice()))?;
        } else {
            return Err(Error::UnsupportedCodecLayout);
        }
    }
    Ok(())
}

impl DecodeSource for AvifSource<'_> {
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
        let channels = usize::from(self.info.layout.channels());
        let width = usize::try_from(region.width).map_err(|_| Error::GeometryOverflow)?;
        let components = width.checked_mul(channels).ok_or(Error::GeometryOverflow)?;
        let mut scratch_u8 = if self.info.sample == SampleType::U8 {
            Some(budget.allocate(components)?)
        } else {
            None
        };
        let mut scratch_f32 = if self.info.sample == SampleType::F32 {
            Some(budget.allocate_typed::<f32>(components)?)
        } else {
            None
        };
        let image = self.decoded()?;
        emit_decoded_rows(
            image,
            region,
            width,
            &mut scratch_u8,
            &mut scratch_f32,
            sink,
        )?;
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
        })
    }
}

#[cfg(all(test, feature = "avif-write"))]
mod tests {
    use super::AvifSource;
    use crate::avif_sink::{AvifSink, EncodeSettings};
    use crate::decoder::{DecodeSource, DecodedRow, SampleType};
    use crate::error::Error;
    use crate::geometry::{Rect, Size};
    use crate::memory::MemoryBudget;

    fn encoded() -> Vec<u8> {
        let budget = MemoryBudget::new(1 << 20);
        let mut sink = AvifSink::new(Size::new(6, 4).unwrap(), &budget).unwrap();
        for row in 0..4 {
            sink.write_row(row, &[200, 40, 90, 255].repeat(6)).unwrap();
        }
        sink.finish(EncodeSettings {
            quality: 90.0,
            speed: 10,
        })
        .unwrap()
        .bytes
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
        assert!(source.image.is_none(), "metadata must not decode pixels");
        let budget = MemoryBudget::new(1 << 16);
        let mut rows = 0;
        source
            .read_region(Rect::new(0, 0, 6, 4).unwrap(), &budget, &mut |_, _| {
                rows += 1;
                Ok(())
            })
            .unwrap();
        assert_eq!(rows, 4);
        assert!(source.image.is_some());
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
}
