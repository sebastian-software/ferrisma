//! Still WebP input through the safe workspace port. Crops bound only the
//! caller-owned output; decoder scratch and lossless ARGB remain opaque.

use webp::api::decode::{self, ColorMode, Config, Container, Features, Format, Output};

use crate::color::AlphaMode;
use crate::decoder::{
    ChannelLayout, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo, SampleType,
};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::memory::MemoryBudget;
use crate::orientation::Orientation;

pub(crate) struct WebpSource<'a> {
    bytes: &'a [u8],
    features: Features,
    info: DecoderInfo,
    orientation: Orientation,
}

impl<'a> WebpSource<'a> {
    pub(crate) fn new_bounded(bytes: &'a [u8], max_pixels: u64) -> Result<Self, Error> {
        let features = Features::parse(bytes).map_err(|_| Error::DecodeFailed)?;
        if features.has_animation {
            return Err(Error::UnsupportedAnimation);
        }
        let size = Size::new(features.width, features.height)?;
        let pixels = u64::from(size.width) * u64::from(size.height);
        if pixels > max_pixels {
            return Err(Error::InputPixelLimitExceeded {
                pixels,
                limit: max_pixels,
            });
        }
        let format = features.format.ok_or(Error::DecodeFailed)?;
        // Container metadata is attached to the image, while the codec's RGB
        // conversion has already produced the samples it describes.
        let container = Container::new(bytes).map_err(|_| Error::DecodeFailed)?;
        let orientation = match container.exif().map(read_exif_orientation) {
            None | Some(Ok(None)) => Orientation::IDENTITY,
            Some(Ok(Some(value))) => {
                Orientation::from_exif(value).ok_or(Error::UnsupportedImageTransform)?
            }
            Some(Err(())) => return Err(Error::DecodeFailed),
        };
        let access = if format == Format::Lossy {
            DecodeAccess::CodecRegionRequest
        } else {
            DecodeAccess::FullFrameFallback
        };
        Ok(Self {
            bytes,
            features,
            orientation,
            info: DecoderInfo {
                size,
                layout: if features.has_alpha {
                    ChannelLayout::Rgba
                } else {
                    ChannelLayout::Rgb
                },
                sample: SampleType::U8,
                access,
                alpha: if features.has_alpha {
                    AlphaMode::Straight
                } else {
                    AlphaMode::Opaque
                },
                rendered_icc: container.icc_profile().map(<[u8]>::to_vec),
                rendered_cicp: None,
            },
        })
    }
}

impl DecodeSource for WebpSource<'_> {
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
        // Lossless can use an even-expanded output crop. Lossy filtering and
        // fancy chroma upsampling alter pixels at crop edges, so retain the
        // full left/top context and width for the original RGB rendering.
        let lossy = self.features.format == Some(Format::Lossy);
        let left = if lossy { 0 } else { region.x & !1 };
        let top = if lossy { 0 } else { region.y & !1 };
        let skip_x = region.x - left;
        let skip_y = region.y - top;
        let width = if lossy {
            self.info.size.width
        } else {
            region
                .width
                .checked_add(skip_x)
                .ok_or(Error::GeometryOverflow)?
        };
        let mut height = region
            .height
            .checked_add(skip_y)
            .ok_or(Error::GeometryOverflow)?;
        if lossy {
            // Keep the decoder's documented maximum filter-row context.
            height = height.saturating_add(8).min(self.info.size.height - top);
        }
        let channels = usize::from(self.info.layout.channels());
        let mode = if self.features.has_alpha {
            ColorMode::Rgba
        } else {
            ColorMode::Rgb
        };
        let config = Config::new(mode).crop(left, top, width, height);
        let (decoded_width, decoded_height) = config
            .output_size(&self.features)
            .map_err(|_| Error::DecodeFailed)?;
        if (decoded_width, decoded_height) != (width, height) {
            return Err(Error::UnsupportedCodecLayout);
        }
        let stride = usize::try_from(width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(channels)
            .ok_or(Error::GeometryOverflow)?;
        let bytes = stride
            .checked_mul(usize::try_from(height).map_err(|_| Error::GeometryOverflow)?)
            .ok_or(Error::GeometryOverflow)?;
        let row_bytes = usize::try_from(region.width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(channels)
            .ok_or(Error::GeometryOverflow)?;
        let skip_bytes = usize::try_from(skip_x)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(channels)
            .ok_or(Error::GeometryOverflow)?;
        let mut decoded = budget.allocate(bytes)?;
        decode::decode_into(
            self.bytes,
            &config,
            Output::Rows {
                data: decoded.as_mut_slice(),
                stride,
            },
        )
        .map_err(|_| Error::DecodeFailed)?;
        let mut rows = 0_u64;
        for y in region.y..region.bottom() {
            let local_y = usize::try_from(y - top).map_err(|_| Error::GeometryOverflow)?;
            let start = local_y
                .checked_mul(stride)
                .and_then(|n| n.checked_add(skip_bytes))
                .ok_or(Error::GeometryOverflow)?;
            sink(
                y,
                DecodedRow::U8(&decoded.as_slice()[start..start + row_bytes]),
            )?;
            rows += 1;
        }
        Ok(DecodeReport {
            requested: region,
            access: self.info.access,
            rows,
            output_sample_bytes: rows
                .checked_mul(u64::try_from(row_bytes).map_err(|_| Error::GeometryOverflow)?)
                .ok_or(Error::GeometryOverflow)?,
            known_full_frame_buffers: u32::from(
                width == self.info.size.width && height == self.info.size.height,
            ),
            peak_engine_managed_bytes: budget.snapshot().peak,
            opaque_codec_memory_unbounded: true,
            engine_buffer_bytes: 0,
        })
    }
}

fn read_exif_orientation(payload: &[u8]) -> Result<Option<u16>, ()> {
    let tiff = payload.strip_prefix(b"Exif\0\0").unwrap_or(payload);
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
        let entry = ifd
            .checked_add(2)
            .and_then(|n| n.checked_add(index.checked_mul(12)?))
            .ok_or(())?;
        if u16_at(entry)? == 0x0112 {
            if u16_at(entry + 2)? != 3 || u32_at(entry + 4)? != 1 {
                return Err(());
            }
            return u16_at(entry + 8).map(Some);
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use webp::api::encode::{PixelLayout, PixelRows, encode_lossless, encode_lossy};

    fn image(alpha: bool, lossy: bool) -> Vec<u8> {
        let channels = if alpha { 4 } else { 3 };
        let mut pixels = Vec::new();
        for y in 0..5_u8 {
            for x in 0..7_u8 {
                pixels.extend_from_slice(&[x * 31, y * 41, x * 13 + y * 9]);
                if alpha {
                    pixels.push(100 + x * 10);
                }
            }
        }
        let rows = PixelRows {
            layout: if alpha {
                PixelLayout::Rgba
            } else {
                PixelLayout::Rgb
            },
            data: &pixels,
            width: 7,
            height: 5,
            stride: 7 * channels,
        };
        if lossy {
            encode_lossy(rows, 90.0).unwrap()
        } else {
            encode_lossless(rows).unwrap()
        }
    }

    fn chunk(tag: [u8; 4], payload: &[u8]) -> Vec<u8> {
        let mut out = tag.to_vec();
        out.extend_from_slice(&u32::try_from(payload.len()).unwrap().to_le_bytes());
        out.extend_from_slice(payload);
        if !payload.len().is_multiple_of(2) {
            out.push(0);
        }
        out
    }

    fn extended(image: &[u8], flags: u8, icc: Option<&[u8]>, exif: Option<&[u8]>) -> Vec<u8> {
        let mut body = chunk(*b"VP8X", &[flags, 0, 0, 0, 6, 0, 0, 4, 0, 0]);
        if let Some(icc) = icc {
            body.extend(chunk(*b"ICCP", icc));
        }
        body.extend_from_slice(&image[12..]);
        if let Some(exif) = exif {
            body.extend(chunk(*b"EXIF", exif));
        }
        let mut out = b"RIFF".to_vec();
        out.extend_from_slice(&u32::try_from(body.len() + 4).unwrap().to_le_bytes());
        out.extend_from_slice(b"WEBP");
        out.extend(body);
        out
    }

    fn tiff_orientation(value: u16) -> Vec<u8> {
        let mut tiff = b"II".to_vec();
        tiff.extend_from_slice(&42_u16.to_le_bytes());
        tiff.extend_from_slice(&8_u32.to_le_bytes());
        tiff.extend_from_slice(&1_u16.to_le_bytes());
        tiff.extend_from_slice(&0x0112_u16.to_le_bytes());
        tiff.extend_from_slice(&3_u16.to_le_bytes());
        tiff.extend_from_slice(&1_u32.to_le_bytes());
        tiff.extend_from_slice(&value.to_le_bytes());
        tiff.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
        tiff
    }

    #[test]
    fn odd_crops_match_full_decode_and_report_access_honestly() {
        for (alpha, lossy) in [(false, true), (false, false), (true, false), (true, true)] {
            let image = image(alpha, lossy);
            let mut source = WebpSource::new_bounded(&image, 35).unwrap();
            let channels = if alpha { 4 } else { 3 };
            assert_eq!(
                source.info().access,
                if lossy {
                    DecodeAccess::CodecRegionRequest
                } else {
                    DecodeAccess::FullFrameFallback
                }
            );
            assert_eq!(
                source.info().alpha,
                if alpha {
                    AlphaMode::Straight
                } else {
                    AlphaMode::Opaque
                }
            );
            let budget = MemoryBudget::new(7 * 5 * channels);
            let full = Rect::new(0, 0, 7, 5).unwrap();
            let mut rows = Vec::new();
            source
                .read_region(full, &budget, &mut |_, row| {
                    let DecodedRow::U8(row) = row else {
                        panic!("unexpected sample type")
                    };
                    rows.extend_from_slice(row);
                    Ok(())
                })
                .unwrap();
            let crop = Rect::new(1, 1, 4, 3).unwrap();
            let mut actual = Vec::new();
            let report = source
                .read_region(crop, &budget, &mut |_, row| {
                    let DecodedRow::U8(row) = row else {
                        panic!("unexpected sample type")
                    };
                    actual.extend_from_slice(row);
                    Ok(())
                })
                .unwrap();
            let expected: Vec<u8> = (1..4)
                .flat_map(|y| {
                    rows[(y * 7 + 1) * channels..(y * 7 + 5) * channels]
                        .iter()
                        .copied()
                })
                .collect();
            assert_eq!(actual, expected);
            assert_eq!(report.rows, 3);
            assert_eq!(report.output_sample_bytes, (3 * 4 * channels) as u64);
            assert_eq!(report.known_full_frame_buffers, u32::from(lossy));
            assert!(report.opaque_codec_memory_unbounded);
            assert_eq!(budget.snapshot().live, 0);
        }
    }

    #[test]
    fn checks_budget_and_propagates_sink_cancellation() {
        let image = image(false, false);
        let mut source = WebpSource::new_bounded(&image, 35).unwrap();
        let region = Rect::new(1, 1, 4, 3).unwrap();
        let budget = MemoryBudget::new(59);
        assert!(matches!(
            source.read_region(region, &budget, &mut |_, _| Ok(())),
            Err(Error::BudgetExceeded { .. })
        ));
        let budget = MemoryBudget::new(60);
        let mut calls = 0;
        assert_eq!(
            source.read_region(region, &budget, &mut |_, _| {
                calls += 1;
                Err(Error::Cancelled)
            }),
            Err(Error::Cancelled)
        );
        assert_eq!(calls, 1);
        assert_eq!(budget.snapshot().live, 0);
    }

    #[test]
    fn lossy_interior_crops_keep_full_decode_filter_context() {
        let mut pixels = Vec::new();
        for y in 0..33_u8 {
            for x in 0..35_u8 {
                pixels.extend_from_slice(&[x.wrapping_mul(17), y.wrapping_mul(19), x ^ y]);
            }
        }
        let image = encode_lossy(
            PixelRows {
                layout: PixelLayout::Rgb,
                data: &pixels,
                width: 35,
                height: 33,
                stride: 35 * 3,
            },
            72.0,
        )
        .unwrap();
        let mut source = WebpSource::new_bounded(&image, 35 * 33).unwrap();
        let budget = MemoryBudget::new(35 * 33 * 3);
        let mut full = Vec::new();
        source
            .read_region(Rect::new(0, 0, 35, 33).unwrap(), &budget, &mut |_, row| {
                let DecodedRow::U8(row) = row else {
                    panic!("unexpected sample type")
                };
                full.extend_from_slice(row);
                Ok(())
            })
            .unwrap();
        for crop in [
            Rect::new(1, 1, 9, 7).unwrap(),
            Rect::new(10, 8, 11, 9).unwrap(),
            Rect::new(21, 19, 13, 12).unwrap(),
        ] {
            let mut actual = Vec::new();
            source
                .read_region(crop, &budget, &mut |_, row| {
                    let DecodedRow::U8(row) = row else {
                        panic!("unexpected sample type")
                    };
                    actual.extend_from_slice(row);
                    Ok(())
                })
                .unwrap();
            let expected: Vec<u8> = (crop.y..crop.bottom())
                .flat_map(|y| {
                    full[((y * 35 + crop.x) * 3) as usize..((y * 35 + crop.right()) * 3) as usize]
                        .iter()
                        .copied()
                })
                .collect();
            assert_eq!(actual, expected, "{crop:?}");
        }
    }

    #[test]
    fn retained_libwebp_reference_matches_full_and_cropped_rgb() {
        let image = include_bytes!("../../tests/fixtures/webp/reference-lossy.webp");
        let expected = include_bytes!("../../tests/fixtures/webp/reference-lossy.rgb");
        let mut source = WebpSource::new_bounded(image, 128 * 128).unwrap();
        let budget = MemoryBudget::new(128 * 128 * 3);
        let mut full = Vec::new();
        source
            .read_region(
                Rect::new(0, 0, 128, 128).unwrap(),
                &budget,
                &mut |_, row| {
                    let DecodedRow::U8(row) = row else {
                        panic!("unexpected sample type")
                    };
                    full.extend_from_slice(row);
                    Ok(())
                },
            )
            .unwrap();
        assert_eq!(full, expected);
        let crop = Rect::new(17, 23, 43, 51).unwrap();
        let mut actual = Vec::new();
        let report = source
            .read_region(crop, &budget, &mut |_, row| {
                let DecodedRow::U8(row) = row else {
                    panic!("unexpected sample type")
                };
                actual.extend_from_slice(row);
                Ok(())
            })
            .unwrap();
        let expected_crop: Vec<u8> = (crop.y..crop.bottom())
            .flat_map(|y| {
                expected[((y * 128 + crop.x) * 3) as usize..((y * 128 + crop.right()) * 3) as usize]
                    .iter()
                    .copied()
            })
            .collect();
        assert_eq!(actual, expected_crop);
        assert_eq!(report.known_full_frame_buffers, 0);
        assert!(report.opaque_codec_memory_unbounded);
    }

    #[test]
    fn rejects_malformed_animation_and_excessive_canvas_before_output() {
        assert_eq!(
            WebpSource::new_bounded(b"garbage", 35).err(),
            Some(Error::DecodeFailed)
        );
        let image = image(false, false);
        assert!(matches!(
            WebpSource::new_bounded(&image, 34).err(),
            Some(Error::InputPixelLimitExceeded {
                pixels: 35,
                limit: 34
            })
        ));
        let animated = extended(&image, 0x02, None, None);
        assert_eq!(
            WebpSource::new_bounded(&animated, 35).err(),
            Some(Error::UnsupportedAnimation)
        );
    }

    #[test]
    fn keeps_rgb_icc_and_reports_source_orientation() {
        let image = image(false, false);
        let tiff = tiff_orientation(6);
        let file = extended(&image, 0x20 | 0x08, Some(b"profile"), Some(&tiff));
        let source = WebpSource::new_bounded(&file, 35).unwrap();
        assert_eq!(
            source.info().rendered_icc.as_deref(),
            Some(b"profile".as_slice())
        );
        assert_eq!(source.info().rendered_cicp, None);
        assert_eq!(source.orientation(), Orientation::from_exif(6).unwrap());
        let bad = extended(&image, 0x08, None, Some(&tiff_orientation(9)));
        assert_eq!(
            WebpSource::new_bounded(&bad, 35).err(),
            Some(Error::UnsupportedImageTransform)
        );
    }
}
