//! Read-only JPEG fallback: full output materialization is explicit.
//! No JPEG encoder or JPEG-first core assumptions.
use crate::color::AlphaMode;
use zune_jpeg::JpegDecoder;
use zune_jpeg::zune_core::{bytestream::ZCursor, colorspace::ColorSpace, options::DecoderOptions};

use crate::decoder::{
    ChannelLayout, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo, SampleType,
};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::memory::MemoryBudget;
use crate::orientation::Orientation;

pub(crate) struct JpegSource<'a> {
    bytes: &'a [u8],
    info: DecoderInfo,
    orientation: Orientation,
}
fn decoder(bytes: &[u8]) -> JpegDecoder<ZCursor<&[u8]>> {
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
    JpegDecoder::new_with_options(ZCursor::new(bytes), options)
}
impl<'a> JpegSource<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        let mut decoder = decoder(bytes);
        decoder.decode_headers().map_err(|_| Error::DecodeFailed)?;
        let (width, height) = decoder.dimensions().ok_or(Error::DecodeFailed)?;
        let size = Size::new(
            u32::try_from(width).map_err(|_| Error::GeometryOverflow)?,
            u32::try_from(height).map_err(|_| Error::GeometryOverflow)?,
        )?;
        if decoder.output_colorspace() != Some(ColorSpace::RGB) {
            return Err(Error::UnsupportedCodecLayout);
        }
        // EXIF orientation describes how the stored samples are displayed;
        // the pipeline applies it (ADR-0019). Unknown values fail closed.
        let orientation = match decoder.exif().map(|exif| exif_orientation(exif)) {
            None | Some(Ok(None)) => Orientation::IDENTITY,
            Some(Ok(Some(value))) => {
                Orientation::from_exif(value).ok_or(Error::UnsupportedImageTransform)?
            }
            Some(Err(())) => return Err(Error::DecodeFailed),
        };
        Ok(Self {
            bytes,
            orientation,
            info: DecoderInfo {
                size,
                layout: ChannelLayout::Rgb,
                sample: SampleType::U8,
                access: DecodeAccess::FullFrameFallback,
                alpha: AlphaMode::Opaque,
                rendered_icc: decoder.icc_profile(),
                rendered_cicp: None,
            },
        })
    }
}
/// Read the IFD0 `Orientation` tag (0x0112) from EXIF data starting at its
/// TIFF header. `Ok(None)` means the tag is absent; `Err` means the TIFF
/// structure needed to find it is malformed.
pub(crate) fn exif_orientation(tiff: &[u8]) -> Result<Option<u16>, ()> {
    let little = match tiff.get(..2) {
        Some(b"II") => true,
        Some(b"MM") => false,
        _ => return Err(()),
    };
    let u16_at = |offset: usize| -> Result<u16, ()> {
        let bytes: [u8; 2] = tiff
            .get(offset..offset.checked_add(2).ok_or(())?)
            .ok_or(())?
            .try_into()
            .map_err(|_| ())?;
        Ok(if little {
            u16::from_le_bytes(bytes)
        } else {
            u16::from_be_bytes(bytes)
        })
    };
    let u32_at = |offset: usize| -> Result<u32, ()> {
        let bytes: [u8; 4] = tiff
            .get(offset..offset.checked_add(4).ok_or(())?)
            .ok_or(())?
            .try_into()
            .map_err(|_| ())?;
        Ok(if little {
            u32::from_le_bytes(bytes)
        } else {
            u32::from_be_bytes(bytes)
        })
    };
    if u16_at(2)? != 42 {
        return Err(());
    }
    let ifd = usize::try_from(u32_at(4)?).map_err(|_| ())?;
    let entries = usize::from(u16_at(ifd)?);
    for index in 0..entries {
        let entry = ifd + 2 + index * 12;
        if u16_at(entry)? != 0x0112 {
            continue;
        }
        // SHORT (type 3) with one value, stored in the first two value bytes.
        if u16_at(entry + 2)? != 3 || u32_at(entry + 4)? != 1 {
            return Err(());
        }
        return u16_at(entry + 8).map(Some);
    }
    Ok(None)
}

impl DecodeSource for JpegSource<'_> {
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
        let mut decoder = decoder(self.bytes);
        decoder.decode_headers().map_err(|_| Error::DecodeFailed)?;
        let width = usize::try_from(self.info.size.width).map_err(|_| Error::GeometryOverflow)?;
        let height = usize::try_from(self.info.size.height).map_err(|_| Error::GeometryOverflow)?;
        let frame_bytes = width
            .checked_mul(height)
            .and_then(|n| n.checked_mul(3))
            .ok_or(Error::GeometryOverflow)?;
        if decoder.output_buffer_size() != Some(frame_bytes) {
            return Err(Error::UnsupportedCodecLayout);
        }
        let mut frame = budget.allocate(frame_bytes)?;
        decoder
            .decode_into(frame.as_mut_slice())
            .map_err(|_| Error::DecodeFailed)?;
        let stride = width.checked_mul(3).ok_or(Error::GeometryOverflow)?;
        let x_bytes = usize::try_from(region.x)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(3)
            .ok_or(Error::GeometryOverflow)?;
        let row_bytes = usize::try_from(region.width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(3)
            .ok_or(Error::GeometryOverflow)?;
        let mut rows = 0_u64;
        for y in region.y..region.bottom() {
            let offset = usize::try_from(y)
                .map_err(|_| Error::GeometryOverflow)?
                .checked_mul(stride)
                .and_then(|i| i.checked_add(x_bytes))
                .ok_or(Error::GeometryOverflow)?;
            sink(
                y,
                DecodedRow::U8(&frame.as_slice()[offset..offset + row_bytes]),
            )?;
            rows += 1;
        }
        Ok(DecodeReport {
            requested: region,
            access: DecodeAccess::FullFrameFallback,
            rows,
            output_sample_bytes: rows
                .checked_mul(u64::try_from(row_bytes).map_err(|_| Error::GeometryOverflow)?)
                .ok_or(Error::GeometryOverflow)?,
            known_full_frame_buffers: 1,
            peak_engine_managed_bytes: budget.snapshot().peak,
            opaque_codec_memory_unbounded: true,
            engine_buffer_bytes: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{JpegSource, exif_orientation};
    use crate::error::Error;

    fn put16(data: &mut Vec<u8>, little: bool, value: u16) {
        data.extend_from_slice(&if little {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        });
    }

    fn put32(data: &mut Vec<u8>, little: bool, value: u32) {
        data.extend_from_slice(&if little {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        });
    }

    /// Little- or big-endian TIFF with one IFD0 entry.
    fn tiff(little: bool, tag: u16, kind: u16, count: u32, value: u16) -> Vec<u8> {
        let mut data = if little {
            b"II".to_vec()
        } else {
            b"MM".to_vec()
        };
        put16(&mut data, little, 42);
        put32(&mut data, little, 8);
        put16(&mut data, little, 1);
        put16(&mut data, little, tag);
        put16(&mut data, little, kind);
        put32(&mut data, little, count);
        put16(&mut data, little, value);
        data.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
        data
    }

    /// Insert an APP1 EXIF segment directly after SOI.
    fn with_exif(tiff: &[u8]) -> Vec<u8> {
        let jpeg = include_bytes!("../../tests/fixtures/sample.jpg");
        let length = u16::try_from(tiff.len() + 8).unwrap().to_be_bytes();
        [
            &jpeg[..2],
            &[0xff, 0xe1],
            &length,
            b"Exif\0\0",
            tiff,
            &jpeg[2..],
        ]
        .concat()
    }

    #[test]
    fn reads_orientation_in_both_byte_orders() {
        assert_eq!(exif_orientation(&tiff(true, 0x0112, 3, 1, 6)), Ok(Some(6)));
        assert_eq!(exif_orientation(&tiff(false, 0x0112, 3, 1, 8)), Ok(Some(8)));
        assert_eq!(exif_orientation(&tiff(true, 0x010f, 2, 4, 0)), Ok(None));
    }

    #[test]
    fn rejects_malformed_tiff_structures() {
        assert_eq!(exif_orientation(b"XX*\0"), Err(()));
        let mut magic = tiff(true, 0x0112, 3, 1, 1);
        magic[2] = 41;
        assert_eq!(exif_orientation(&magic), Err(()));
        assert_eq!(exif_orientation(&tiff(true, 0x0112, 4, 1, 1)), Err(()));
        let truncated = tiff(true, 0x0112, 3, 1, 1);
        assert_eq!(exif_orientation(&truncated[..12]), Err(()));
    }

    #[test]
    fn exif_orientation_is_reported_and_invalid_values_fail_closed() {
        use crate::decoder::DecodeSource;
        use crate::orientation::Orientation;
        for orientation in 1..=8 {
            let file = with_exif(&tiff(true, 0x0112, 3, 1, orientation));
            let source = JpegSource::new(&file).unwrap();
            assert_eq!(
                source.orientation(),
                Orientation::from_exif(orientation).unwrap()
            );
        }
        for invalid in [0, 9] {
            assert_eq!(
                JpegSource::new(&with_exif(&tiff(false, 0x0112, 3, 1, invalid))).err(),
                Some(Error::UnsupportedImageTransform)
            );
        }
        let untagged_file = with_exif(&tiff(true, 0x010f, 2, 4, 0));
        let untagged = JpegSource::new(&untagged_file).unwrap();
        assert!(untagged.orientation().is_identity());
        assert_eq!(
            JpegSource::new(&with_exif(b"MM\0*garbage")).err(),
            Some(Error::DecodeFailed)
        );
    }
}
