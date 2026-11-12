//! Read-only JPEG source through the workspace codec's safe scanline API.
//! No JPEG encoder or JPEG-first core assumptions.
use crate::color::AlphaMode;
use jpeg::decode::{ColorSpace, Decoder, DecoderOptions, OutputOptions, PixelFormat};

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
fn output_options() -> OutputOptions {
    OutputOptions {
        format: Some(PixelFormat::Rgb),
        ..OutputOptions::default()
    }
}
fn decoder(bytes: &[u8]) -> Result<Decoder<'_>, Error> {
    // Missing entropy or EOI markers must not be disguised by libjpeg's
    // synthetic-EOI recovery warning.
    Decoder::with_options(
        bytes,
        &DecoderOptions {
            fail_on_warning: true,
            ..DecoderOptions::default()
        },
    )
    .map_err(|_| Error::DecodeFailed)
}
impl<'a> JpegSource<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Result<Self, Error> {
        let mut decoder = decoder(bytes)?;
        // The port does not emit RGB for CMYK/YCCK. Passing its raw four
        // channels through an RGB contract would mislabel the samples.
        if matches!(
            decoder.header().color_space,
            ColorSpace::Cmyk | ColorSpace::Ycck
        ) {
            return Err(Error::UnsupportedCodecLayout);
        }
        let output = decoder
            .output_info(&output_options())
            .map_err(|_| Error::DecodeFailed)?;
        let size = Size::new(output.width, output.height)?;
        if output.format != PixelFormat::Rgb || output.components != 3 {
            return Err(Error::UnsupportedCodecLayout);
        }
        // EXIF orientation describes how the stored samples are displayed;
        // the pipeline applies it (ADR-0019). Unknown values fail closed.
        let orientation = match decoder.exif().map(exif_orientation) {
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
                access: DecodeAccess::CodecRegionRequest,
                alpha: AlphaMode::Opaque,
                rendered_icc: decoder.icc_profile().map_err(|_| Error::DecodeFailed)?,
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
        let decoder = decoder(self.bytes)?;
        let mut scanlines = decoder
            .start(&output_options())
            .map_err(|_| Error::DecodeFailed)?;
        if scanlines.info().width != self.info.size.width
            || scanlines.info().height != self.info.size.height
            || scanlines.info().format != PixelFormat::Rgb
            || scanlines.info().components != 3
        {
            return Err(Error::UnsupportedCodecLayout);
        }
        // libjpeg expands the horizontal request to an iMCU boundary. The
        // returned offset, not the requested x, locates the sink's samples.
        // Fancy chroma upsampling needs adjacent columns at both edges;
        // without them an iMCU-aligned crop can change RGB edge samples.
        let crop_x = region.x.saturating_sub(1);
        let crop_right = region.right() + u32::from(region.right() < self.info.size.width);
        let crop_width = crop_right - crop_x;
        let (decoded_x, _) = scanlines
            .crop(crop_x, crop_width)
            .map_err(|_| Error::DecodeFailed)?;
        let row_bytes = scanlines.info().row_bytes();
        let mut row = budget.allocate(row_bytes)?;
        let skipped = scanlines
            .skip_rows(region.y)
            .map_err(|_| Error::DecodeFailed)?;
        if skipped != region.y {
            return Err(Error::DecodeFailed);
        }
        let decoded_offset = region
            .x
            .checked_sub(decoded_x)
            .ok_or(Error::UnsupportedCodecLayout)?;
        let x_bytes = usize::try_from(decoded_offset)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(3)
            .ok_or(Error::GeometryOverflow)?;
        let output_row_bytes = usize::try_from(region.width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(3)
            .ok_or(Error::GeometryOverflow)?;
        if x_bytes
            .checked_add(output_row_bytes)
            .is_none_or(|end| end > row_bytes)
        {
            return Err(Error::UnsupportedCodecLayout);
        }
        let mut rows = 0_u64;
        for y in region.y..region.bottom() {
            if scanlines
                .read_rows(row.as_mut_slice())
                .map_err(|_| Error::DecodeFailed)?
                != 1
            {
                return Err(Error::DecodeFailed);
            }
            sink(
                y,
                DecodedRow::U8(&row.as_slice()[x_bytes..x_bytes + output_row_bytes]),
            )?;
            rows += 1;
        }
        // A small ROI still validates the encoded tail. Without finishing,
        // truncation beyond the last requested row could look successful.
        let remaining = self.info.size.height - scanlines.next_row();
        if scanlines
            .skip_rows(remaining)
            .map_err(|_| Error::DecodeFailed)?
            != remaining
        {
            return Err(Error::DecodeFailed);
        }
        scanlines.finish().map_err(|_| Error::DecodeFailed)?;
        Ok(DecodeReport {
            requested: region,
            access: DecodeAccess::CodecRegionRequest,
            rows,
            output_sample_bytes: rows
                .checked_mul(u64::try_from(output_row_bytes).map_err(|_| Error::GeometryOverflow)?)
                .ok_or(Error::GeometryOverflow)?,
            known_full_frame_buffers: 0,
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
    fn reports_embedded_icc_without_changing_rgb_samples() {
        use crate::decoder::{DecodeSource, DecodedRow};
        use crate::geometry::Rect;
        use crate::memory::MemoryBudget;

        let jpeg = include_bytes!("../../tests/fixtures/sample.jpg");
        let profile = include_bytes!("../../assets/icc/sRGB2014.icc");
        let payload = [b"ICC_PROFILE\0\x01\x01".as_slice(), profile].concat();
        let length = u16::try_from(payload.len() + 2).unwrap().to_be_bytes();
        let tagged = [&jpeg[..2], &[0xff, 0xe2], &length, &payload, &jpeg[2..]].concat();
        let mut plain = JpegSource::new(jpeg).unwrap();
        let mut tagged = JpegSource::new(&tagged).unwrap();
        assert_eq!(
            tagged.info().rendered_icc.as_deref(),
            Some(profile.as_slice())
        );
        let region = Rect::new(1, 1, 4, 3).unwrap();
        let read = |source: &mut JpegSource<'_>| {
            let mut pixels = Vec::new();
            source
                .read_region(region, &MemoryBudget::new(21), &mut |_, row| {
                    let DecodedRow::U8(samples) = row else {
                        panic!("RGB8 expected")
                    };
                    pixels.extend_from_slice(samples);
                    Ok(())
                })
                .unwrap();
            pixels
        };
        assert_eq!(read(&mut plain), read(&mut tagged));
    }

    #[test]
    fn does_not_label_cmyk_samples_as_rgb() {
        assert_eq!(
            JpegSource::new(include_bytes!("../../tests/fixtures/cmyk.jpg")).err(),
            Some(Error::UnsupportedCodecLayout)
        );
    }

    #[test]
    fn cropped_read_rejects_a_truncated_tail() {
        use crate::decoder::DecodeSource;
        use crate::geometry::Rect;
        use crate::memory::MemoryBudget;

        let complete = include_bytes!("../../tests/fixtures/sample.jpg");
        let missing_eoi = &complete[..complete.len() - 2];
        let mut source = JpegSource::new(missing_eoi).unwrap();
        assert_eq!(
            source.read_region(
                Rect::new(0, 0, 2, 1).unwrap(),
                &MemoryBudget::new(21),
                &mut |_, _| Ok(())
            ),
            Err(Error::DecodeFailed)
        );
    }

    #[test]
    fn malformed_icc_is_not_treated_as_untagged_color() {
        let jpeg = include_bytes!("../../tests/fixtures/sample.jpg");
        // The marker promises two ICC chunks, but only the first is present.
        let payload = b"ICC_PROFILE\0\x01\x02bad";
        let length = u16::try_from(payload.len() + 2).unwrap().to_be_bytes();
        let tagged = [&jpeg[..2], &[0xff, 0xe2], &length, payload, &jpeg[2..]].concat();
        assert_eq!(JpegSource::new(&tagged).err(), Some(Error::DecodeFailed));
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
