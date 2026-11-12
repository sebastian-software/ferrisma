//! JPEG XL adapter: codec-level ROI request, float output scanlines.
//! ROI is a capability, not a guarantee of skipped internal codec work.
use crate::color::AlphaMode;
use jxl_oxide::{CropInfo, JxlImage, PixelFormat};
use std::io::Cursor;

use crate::decoder::{
    ChannelLayout, CodecWork, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo,
    SampleType,
};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::memory::MemoryBudget;

pub(crate) struct JxlSource {
    image: JxlImage,
    info: DecoderInfo,
}
impl JxlSource {
    pub(crate) fn new(bytes: &[u8]) -> Result<Self, Error> {
        let image = JxlImage::builder()
            .read(Cursor::new(bytes))
            .map_err(|_| Error::DecodeFailed)?;
        // `render_frame_cropped(0)` would silently return the first keyframe
        // of an animation. Still images only, as for WebP, AVIF and the
        // libjxl-rs backend; no first-frame policy is offered.
        if image.image_header().metadata.animation.is_some() {
            return Err(Error::UnsupportedAnimation);
        }
        reject_unrenderable_colour_encoding(&image)?;
        // jxl-oxide renders upright samples itself: `width()`/`height()` are
        // display dimensions, crop regions are display coordinates and the
        // pixel stream applies the header orientation (ADR-0019).
        let size = Size::new(image.width(), image.height())?;
        let layout = match image.pixel_format() {
            PixelFormat::Gray => ChannelLayout::Gray,
            PixelFormat::Graya => ChannelLayout::GrayAlpha,
            PixelFormat::Rgb => ChannelLayout::Rgb,
            PixelFormat::Rgba => ChannelLayout::Rgba,
            PixelFormat::Cmyk => ChannelLayout::Cmyk,
            PixelFormat::Cmyka => ChannelLayout::Cmyka,
        };
        let info = DecoderInfo {
            size,
            layout,
            sample: SampleType::F32,
            access: DecodeAccess::CodecRegionRequest,
            alpha: match layout {
                ChannelLayout::GrayAlpha | ChannelLayout::Rgba | ChannelLayout::Cmyka => {
                    AlphaMode::Unspecified
                }
                ChannelLayout::Gray | ChannelLayout::Rgb | ChannelLayout::Cmyk => AlphaMode::Opaque,
            },
            rendered_icc: Some(image.rendered_icc()),
            rendered_cicp: rendered_cicp(&image),
        };
        Ok(Self { image, info })
    }
}

/// The CICP of the emitted samples, only where it is verifiable (issue #41).
///
/// Without a CMS (`NullCms`, the engine's build) jxl-oxide 0.12.6 renders an
/// enum-signalled image in exactly that enum encoding, and an ICC-signalled
/// one either unchanged in the embedded profile (non-XYB), in an enum
/// encoding recognized from that profile, or in sRGB (XYB with an
/// unrecognized profile). `rendered_icc` always describes that result.
/// `rendered_cicp`, however, maps the primaries without the white point:
/// Display P3 (D65) is reported as CICP primaries 11, which H.273 defines
/// with the DCI white point. So the adapter derives the code points itself
/// from the header's enum encoding, which the renderer uses unchanged
/// because this adapter never requests another encoding, and reports no
/// CICP when the encoding came from an ICC profile or has no exact H.273
/// equivalent.
fn rendered_cicp(image: &JxlImage) -> Option<[u8; 4]> {
    use jxl_oxide::color::{ColourEncoding, ColourSpace, Primaries, WhitePoint};
    let reported = image.rendered_cicp()?;
    let ColourEncoding::Enum(header) = &image.image_header().metadata.colour_encoding else {
        return None;
    };
    if header.colour_space != ColourSpace::Rgb {
        return None;
    }
    let primaries = match (&header.white_point, &header.primaries) {
        (WhitePoint::D65, Primaries::Srgb) => 1,
        (WhitePoint::D65, Primaries::Bt2100) => 9,
        (WhitePoint::Dci, Primaries::P3) => 11,
        (WhitePoint::D65, Primaries::P3) => 12,
        _ => return None,
    };
    Some([primaries, reported[1], 0, 1])
}

/// jxl-color 0.11 synthesizes the rendered ICC profile from an enum colour
/// encoding with `panic!`/`todo!` for an unknown or XYB colour space, an
/// unknown transfer function, and divides by a zero gamma. The header is
/// attacker-controlled, so reject those encodings before any accessor that
/// synthesizes a profile or renders pixels. An embedded ICC profile is not
/// synthesized: jxl-render either parses it into a known enum encoding or keeps
/// the profile bytes, and it falls back to an sRGB enum encoding otherwise.
fn reject_unrenderable_colour_encoding(image: &JxlImage) -> Result<(), Error> {
    use jxl_oxide::color::{ColourEncoding, ColourSpace, TransferFunction};

    let ColourEncoding::Enum(encoding) = &image.image_header().metadata.colour_encoding else {
        return Ok(());
    };
    let known_space = matches!(encoding.colour_space, ColourSpace::Rgb | ColourSpace::Grey);
    let known_transfer = !matches!(
        encoding.tf,
        TransferFunction::Unknown | TransferFunction::Gamma { g: 0, .. }
    );
    if known_space && known_transfer {
        Ok(())
    } else {
        Err(Error::UnsupportedColorEncoding)
    }
}

impl DecodeSource for JxlSource {
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
        self.image.set_image_region(CropInfo {
            left: region.x,
            top: region.y,
            width: region.width,
            height: region.height,
        });
        let rendered = self
            .image
            .render_frame_cropped(0)
            .map_err(|_| Error::DecodeFailed)?;
        let mut stream = rendered.stream();
        if stream.width() != region.width
            || stream.height() != region.height
            || stream.channels() != u32::from(self.info.layout.channels())
        {
            return Err(Error::IncompleteDecoderOutput);
        }
        let row_samples = usize::try_from(region.width)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(usize::from(self.info.layout.channels()))
            .ok_or(Error::GeometryOverflow)?;
        let mut scratch = budget.allocate_typed::<f32>(row_samples)?;
        for y in region.y..region.bottom() {
            if stream.write_to_buffer(scratch.as_mut_slice()) != row_samples {
                return Err(Error::IncompleteDecoderOutput);
            }
            sink(y, DecodedRow::F32(scratch.as_slice()))?;
        }
        let output_bytes = u64::try_from(row_samples)
            .map_err(|_| Error::GeometryOverflow)?
            .checked_mul(u64::from(region.height))
            .and_then(|n| n.checked_mul(4))
            .ok_or(Error::GeometryOverflow)?;
        Ok(DecodeReport {
            requested: region,
            access: DecodeAccess::CodecRegionRequest,
            rows: u64::from(region.height),
            output_sample_bytes: output_bytes,
            known_full_frame_buffers: 0,
            peak_engine_managed_bytes: budget.snapshot().peak,
            opaque_codec_memory_unbounded: true,
            engine_buffer_bytes: 0,
            engine_copied_bytes: 0,
            // jxl-oxide returns exactly the requested display rectangle. Which
            // groups, reference frames and filter borders it reconstructed is
            // not observable through its API, so only the output is a bound.
            work: CodecWork::region_at_least(region),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::JxlSource;
    use crate::decoder::{DecodeSource, DecodedRow, WorkCount};
    use crate::error::Error;
    use crate::geometry::Rect;
    use crate::memory::MemoryBudget;
    use jxl_oxide::JxlImage;

    const DATA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/codecs/libjxl-rs/tests/data/");

    fn fixture(name: &str) -> Vec<u8> {
        std::fs::read(format!("{DATA}{name}")).unwrap()
    }

    fn read(bytes: &[u8], region: Rect) -> Result<Vec<f32>, Error> {
        let mut source = JxlSource::new(bytes)?;
        let mut samples = Vec::new();
        let report = source.read_region(region, &MemoryBudget::new(1 << 24), &mut |_, row| {
            let DecodedRow::F32(row) = row else {
                panic!("JPEG XL rows are float")
            };
            samples.extend_from_slice(row);
            Ok(())
        })?;
        // Only the returned rectangle is a bound; skipped work is unobserved.
        assert_eq!(report.work.codec_output, region);
        assert_eq!(
            report.work.reconstructed_pixels,
            WorkCount::AtLeast(u64::from(region.width) * u64::from(region.height))
        );
        Ok(samples)
    }

    /// Invert every byte of the frame-`frame` TOC groups whose kind (in its
    /// `Debug` form, since jxl-oxide does not re-export the type) `select`
    /// accepts. Returns the corrupted file and the number of inverted bytes.
    fn corrupt(bytes: &[u8], frame: usize, select: impl Fn(&str) -> bool) -> (Vec<u8>, usize) {
        let image = JxlImage::builder().read(bytes).unwrap();
        let frame = image.frame(frame).unwrap();
        let mut out = bytes.to_vec();
        let mut inverted = 0;
        for group in frame.toc().iter_bitstream_order() {
            if !select(&format!("{:?}", group.kind)) {
                continue;
            }
            let data = frame.data(group.kind).unwrap();
            let mut found = bytes
                .windows(data.len())
                .enumerate()
                .filter(|(_, w)| *w == data);
            let (at, _) = found.next().expect("group bytes are in the file");
            assert!(found.next().is_none(), "group bytes are unique");
            for byte in &mut out[at..at + data.len()] {
                *byte ^= 0xff;
            }
            inverted += data.len();
        }
        assert!(inverted > 0);
        (out, inverted)
    }

    /// The corrupted bytes are read when the output changes or fails. The
    /// file must still open: header and TOC parsing do not read group data,
    /// so a difference comes from rendering.
    fn uses_corrupted_bytes(clean: &[u8], corrupted: &[u8], region: Rect) -> bool {
        assert!(JxlSource::new(corrupted).is_ok());
        read(corrupted, region).map_or(true, |samples| samples != read(clean, region).unwrap())
    }

    /// #19: for multi-group `VarDCT` files, a crop that does not touch pass
    /// group 1 renders identically when every byte of that group is
    /// inverted, while the full frame and a crop over that group change or
    /// fail. jxl-oxide therefore skipped the group's entropy decoding here.
    /// This is evidence for these two files, not a general ROI guarantee,
    /// and the adapter still reports only a lower bound.
    #[test]
    fn multi_group_crop_skips_unrequested_pass_groups() {
        for name in [
            "jpeg-reconstruction/large-progressive.jxl",
            "jpeg-reconstruction/chunked-baseline.jxl",
        ] {
            let clean = fixture(name);
            let (corrupted, inverted) = corrupt(&clean, 0, |kind| {
                kind.starts_with("GroupPass") && kind.ends_with("group_idx: 1 }")
            });
            assert!(inverted >= 16, "{name}: {inverted} bytes");
            let full = Rect::new(0, 0, 257, 129).unwrap();
            let left = Rect::new(0, 0, 200, 129).unwrap();
            let right = Rect::new(250, 0, 7, 129).unwrap();
            assert!(uses_corrupted_bytes(&clean, &corrupted, full), "{name}");
            assert!(uses_corrupted_bytes(&clean, &corrupted, right), "{name}");
            assert_eq!(
                read(&corrupted, left).unwrap(),
                read(&clean, left).unwrap(),
                "{name}"
            );
        }
    }

    /// A single-TOC frame stores everything in one group: a top-left crop
    /// still consumes bytes that only lower or right pixels need.
    #[test]
    fn single_toc_crop_reads_the_whole_group() {
        let clean = fixture("ycbcr-jpeg-444.jxl");
        let image = JxlImage::builder().read(clean.as_slice()).unwrap();
        assert!(image.frame_by_keyframe(0).unwrap().toc().is_single_entry());
        let (corrupted, _) = corrupt(&clean, 0, |kind| kind == "All");
        assert!(uses_corrupted_bytes(
            &clean,
            &corrupted,
            Rect::new(0, 0, 4, 4).unwrap()
        ));
    }

    /// Patches copy from a reference-only frame, which jxl-oxide renders over
    /// its full extent: corrupting it changes or fails any crop.
    #[test]
    fn patch_reference_frame_is_decoded_for_any_crop() {
        let clean = fixture("repeated-glyphs-modular-none.jxl");
        let image = JxlImage::builder().read(clean.as_slice()).unwrap();
        assert_eq!(image.num_loaded_frames(), 2);
        assert!(image.frame_header(0).unwrap().flags.patches());
        let reference = (0..2)
            .find(|&index| {
                !image
                    .frame(index)
                    .unwrap()
                    .header()
                    .frame_type
                    .is_normal_frame()
            })
            .unwrap();
        let (corrupted, _) = corrupt(&clean, reference, |_| true);
        for region in [
            Rect::new(0, 0, 8, 8).unwrap(),
            Rect::new(120, 88, 8, 8).unwrap(),
        ] {
            assert!(
                uses_corrupted_bytes(&clean, &corrupted, region),
                "{region:?}"
            );
        }
    }

    /// Noise and splines are position-dependent; a crop must equal the
    /// corresponding slice of the full render.
    #[test]
    fn crops_with_noise_and_splines_match_the_full_render() {
        for name in ["effects_rgb_alpha8.jxl", "effects_gray_alpha16.jxl"] {
            let bytes = fixture(name);
            let image = JxlImage::builder().read(bytes.as_slice()).unwrap();
            let header = image.frame_header(0).unwrap();
            assert!(header.flags.noise() && header.flags.splines(), "{name}");
            let source = JxlSource::new(&bytes).unwrap();
            let channels = usize::from(source.info.layout.channels());
            let (width, height) = (source.info.size.width, source.info.size.height);
            let full = read(&bytes, Rect::new(0, 0, width, height).unwrap()).unwrap();
            for y in 0..height {
                for x in 0..width {
                    for (w, h) in [(1, 1), (width - x, height - y)] {
                        let region = Rect::new(x, y, w, h).unwrap();
                        let crop = read(&bytes, region).unwrap();
                        for (row, sy) in crop.chunks(w as usize * channels).zip(y..) {
                            let at = (sy * width + x) as usize * channels;
                            assert_eq!(row, &full[at..at + row.len()], "{name} {region:?}");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn animated_input_is_rejected_rather_than_reduced_to_its_first_frame() {
        for name in [
            "decoder-geometry/sequence-reference-overwrite-animation-1.jxl",
            "decoder-geometry/sequence-multigroup-257-3-0-0.jxl",
        ] {
            let animated = fixture(name);
            let image = JxlImage::builder().read(animated.as_slice()).unwrap();
            // Real multi-keyframe animations that jxl-oxide would otherwise
            // render as keyframe 0.
            assert!(image.image_header().metadata.animation.is_some(), "{name}");
            assert!(image.num_loaded_keyframes() > 1, "{name}");
            assert_eq!(
                JxlSource::new(&animated).err(),
                Some(Error::UnsupportedAnimation)
            );
        }
        // Several frames composited into one still keyframe remain a still.
        let composite = fixture("decoder-geometry/sequence-reference-overwrite-animation-0.jxl");
        assert!(JxlSource::new(&composite).is_ok());
    }
}
