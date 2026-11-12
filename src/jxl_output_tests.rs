//! JPEG XL output (ADR-0039): admitted-profile errors, budget accounting and
//! independent round trips. The admitted profile is lossless, so every
//! decoded sample must equal the finished sRGB8 row the sink received:
//! the pixel error bound is zero, not a tolerance.
// Test images are small; their coordinates and samples fit every cast.
#![allow(clippy::cast_possible_truncation)]

use crate::error::Error;
use crate::geometry::Size;
use crate::jxl_sink::{JxlSink, known_codec_frame_bytes};
use crate::memory::MemoryBudget;
use crate::pipeline::{
    ChromaSubsampling, JpegXlOptions, OutputFormat, ProcessingErrorKind, TransformOptions,
};

/// Lossless output options; the lossy tests are in `jxl_lossy_tests`.
fn options(width: u32, height: u32) -> TransformOptions {
    TransformOptions {
        output_format: OutputFormat::JpegXl,
        jpeg_xl: JpegXlOptions::Lossless { effort: 2 },
        output_width: width,
        output_height: height,
        ..TransformOptions::default()
    }
}

/// Deterministic photo-like content: gradients plus a hashed texture, and
/// (optionally) straight alpha that includes zero with nonzero color.
fn rows(width: usize, height: usize, alpha: bool) -> Vec<u8> {
    let mut data = Vec::with_capacity(width * height * 4);
    for y in 0..height {
        for x in 0..width {
            let hash = (x as u32).wrapping_mul(2_654_435_761) ^ (y as u32).wrapping_mul(40_503);
            let noise = (hash >> 13) as u8 & 0x1f;
            data.extend_from_slice(&[
                ((x * 255) / width.max(2)) as u8 ^ noise,
                ((y * 255) / height.max(2)) as u8,
                ((x + y) * 7) as u8 ^ (noise << 2),
                if !alpha {
                    255
                } else if x == 0 {
                    0
                } else {
                    ((x * 13 + y * 5) % 256) as u8
                },
            ]);
        }
    }
    data
}

fn encode(
    width: usize,
    height: usize,
    data: &[u8],
    budget: &MemoryBudget,
) -> Result<Vec<u8>, Error> {
    let size = Size::new(width as u32, height as u32).unwrap();
    let mut sink = JxlSink::new(size, options(size.width, size.height), budget)?;
    for (y, row) in data.chunks_exact(width * 4).enumerate() {
        sink.write_row(y as u32, row)?;
    }
    Ok(sink.finish()?.bytes)
}

#[test]
fn rejects_requests_outside_the_admitted_profile_before_allocating() {
    let budget = MemoryBudget::new(1 << 20);
    let size = Size::new(8, 8).unwrap();
    for (changed, expected) in [
        (
            TransformOptions {
                jpeg_xl: JpegXlOptions::Lossless { effort: 7 },
                ..options(8, 8)
            },
            "effort",
        ),
        (
            TransformOptions {
                encoder_threads: 2,
                ..options(8, 8)
            },
            "serial",
        ),
    ] {
        let error = JxlSink::new(size, changed, &budget).err().unwrap();
        assert!(matches!(error, Error::UnsupportedEncoderProfile(_)));
        assert!(error.to_string().contains(expected), "{error}");
    }
    let chroma = TransformOptions {
        chroma_subsampling: ChromaSubsampling::Yuv420,
        ..options(8, 8)
    };
    assert_eq!(
        JxlSink::new(size, chroma, &budget).err(),
        Some(Error::InvalidEncodeOptions)
    );
    // Level 5 limits: 2^18 per side and 2^28 pixels.
    let wide = Size::new((1 << 18) + 1, 1).unwrap();
    assert!(matches!(
        JxlSink::new(wide, options(wide.width, 1), &budget).err(),
        Some(Error::UnsupportedEncoderProfile(_))
    ));
    let area = Size::new(1 << 15, (1 << 13) + 1).unwrap();
    assert!(matches!(
        JxlSink::new(area, options(area.width, area.height), &budget).err(),
        Some(Error::UnsupportedEncoderProfile(_))
    ));
    assert_eq!(budget.snapshot().peak, 0);
}

#[test]
fn input_frame_and_pinned_codec_plane_estimate_are_reserved() {
    let (width, height) = (9, 5);
    let frame = width * height * 4;
    let opaque = known_codec_frame_bytes(width, height, false).unwrap();
    // Regression pin, not evidence: these two equalities restate the sink's
    // own formula (Image3<f32> padded to 16x8, plus three i32 Modular
    // channels, plus two alpha planes) so an unintended change to it fails
    // here. They do not show that the encoder allocates exactly these planes;
    // that would need an allocation measurement inside libjxl-rs. The budget
    // checks below do test real behavior: the sink reserves what the
    // formula says before encoding.
    assert_eq!(opaque, 16 * 8 * 3 * 4 + width * height * 3 * 4);
    let alpha = known_codec_frame_bytes(width, height, true).unwrap();
    assert_eq!(alpha, opaque + width * height * 4 * 2);

    let data = rows(width, height, false);
    assert!(matches!(
        encode(width, height, &data, &MemoryBudget::new(frame - 1)),
        Err(Error::BudgetExceeded { .. })
    ));
    let short = MemoryBudget::new(frame + opaque - 1);
    assert!(matches!(
        encode(width, height, &data, &short),
        Err(Error::BudgetExceeded { .. })
    ));
    assert_eq!(short.snapshot().live, 0);
    assert_eq!(short.snapshot().peak, frame);
    let exact = MemoryBudget::new(frame + opaque);
    encode(width, height, &data, &exact).unwrap();
    assert_eq!(exact.snapshot().peak, frame + opaque);
    assert_eq!(exact.snapshot().live, 0);
    // Transparent rows need the alpha planes as well.
    let data = rows(width, height, true);
    assert!(encode(width, height, &data, &MemoryBudget::new(frame + opaque)).is_err());
    encode(width, height, &data, &MemoryBudget::new(frame + alpha)).unwrap();
}

#[test]
fn incomplete_or_misordered_rows_are_rejected() {
    let budget = MemoryBudget::new(1 << 20);
    let size = Size::new(4, 2).unwrap();
    let mut sink = JxlSink::new(size, options(4, 2), &budget).unwrap();
    assert!(sink.write_row(1, &[0; 16]).is_err());
    sink.write_row(0, &[0; 16]).unwrap();
    assert!(sink.write_row(1, &[0; 12]).is_err());
    assert!(sink.finish().is_err());
}

#[test]
fn facade_rejects_mismatched_and_disabled_settings_before_decoding() {
    // Invalid compressed input proves the check precedes opening a decoder.
    let garbage = b"not an image";
    for options in [
        TransformOptions {
            quality: 50.0,
            ..options(8, 8)
        },
        TransformOptions {
            webp: crate::pipeline::WebpOptions::Lossless { level: 4 },
            ..options(8, 8)
        },
        TransformOptions {
            jpeg_xl: JpegXlOptions::Lossless { effort: 3 },
            ..options(8, 8)
        },
        TransformOptions {
            encoder_threads: 0,
            ..options(8, 8)
        },
        TransformOptions {
            output_format: OutputFormat::WebP,
            jpeg_xl: JpegXlOptions::Lossless { effort: 3 },
            ..options(8, 8)
        },
    ] {
        let error =
            crate::pipeline::transcode(garbage, crate::pipeline::ImageFormat::JpegXl, options)
                .unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::InvalidOptions, "{error}");
    }
}

#[cfg(feature = "jpeg-xl-read")]
mod independent_decode {
    use jxl_oxide::color::{
        ColourEncoding, ColourSpace, Primaries, RenderingIntent, TransferFunction, WhitePoint,
    };
    use jxl_oxide::image::BitDepth;
    use jxl_oxide::{ExtraChannelType, JxlImage};

    use super::{encode, rows};
    use crate::memory::MemoryBudget;

    /// Decode with jxl-oxide, check the emitted signaling and return
    /// interleaved 8-bit samples plus the channel count.
    pub(super) fn decode_oxide(bytes: &[u8]) -> (u32, u32, usize, Vec<u8>) {
        let image = JxlImage::builder()
            .read(std::io::Cursor::new(bytes))
            .unwrap();
        let metadata = &image.image_header().metadata;
        assert!(!metadata.xyb_encoded, "lossless samples are not XYB");
        assert_eq!(metadata.orientation, 1);
        assert_eq!(
            metadata.bit_depth,
            BitDepth::IntegerSample { bits_per_sample: 8 }
        );
        let ColourEncoding::Enum(color) = &metadata.colour_encoding else {
            panic!("expected enum color, found an ICC profile")
        };
        assert_eq!(color.colour_space, ColourSpace::Rgb);
        assert_eq!(color.white_point, WhitePoint::D65);
        assert_eq!(color.primaries, Primaries::Srgb);
        assert_eq!(color.tf, TransferFunction::Srgb);
        assert_eq!(color.rendering_intent, RenderingIntent::Relative);
        for ec in &metadata.ec_info {
            assert_eq!(
                ec.ty,
                ExtraChannelType::Alpha {
                    alpha_associated: false
                }
            );
            assert_eq!(ec.bit_depth, BitDepth::IntegerSample { bits_per_sample: 8 });
        }
        let channels = 3 + metadata.ec_info.len();
        let render = image.render_frame(0).unwrap();
        let mut stream = render.stream();
        assert_eq!(stream.channels() as usize, channels);
        let mut samples = vec![0_u8; (stream.width() * stream.height()) as usize * channels];
        stream.write_to_buffer(&mut samples);
        (image.width(), image.height(), channels, samples)
    }

    pub(super) fn to_rgba(channels: usize, samples: &[u8]) -> Vec<u8> {
        if channels == 4 {
            return samples.to_vec();
        }
        samples
            .as_chunks::<3>()
            .0
            .iter()
            .flat_map(|rgb| [rgb[0], rgb[1], rgb[2], 255])
            .collect()
    }

    #[test]
    fn sink_round_trips_exactly_through_jxl_oxide() {
        // Odd sizes, every x mod 8 boundary, and a 2x2 group layout.
        for (width, height) in [(1, 1), (9, 17), (37, 19), (300, 260)] {
            for alpha in [false, true] {
                let data = rows(width, height, alpha);
                let bytes = encode(width, height, &data, &MemoryBudget::new(64 << 20)).unwrap();
                assert_eq!(&bytes[..2], &[0xff, 0x0a], "raw codestream signature");
                let (w, h, channels, samples) = decode_oxide(&bytes);
                assert_eq!((w as usize, h as usize), (width, height));
                // A one-pixel image with x == 0 has alpha 0, so it keeps alpha.
                assert_eq!(channels, if alpha { 4 } else { 3 });
                assert_eq!(
                    to_rgba(channels, &samples),
                    data,
                    "{width}x{height} {alpha}"
                );
            }
        }
    }

    #[cfg(feature = "jpeg-xl-own-read")]
    #[test]
    fn sink_round_trips_exactly_through_the_workspace_decoder() {
        for alpha in [false, true] {
            let (width, height) = (37, 19);
            let data = rows(width, height, alpha);
            let bytes = encode(width, height, &data, &MemoryBudget::new(64 << 20)).unwrap();
            let info = ferrisma_jxl_decoder::inspect(&bytes, 1 << 20).unwrap();
            assert_eq!((info.width, info.height), (37, 19));
            assert_eq!(info.channels, if alpha { 4 } else { 3 });
            assert!(!info.alpha_premultiplied);
            let mut output = vec![0_f32; info.sample_count().unwrap()];
            ferrisma_jxl_decoder::decode_into(&bytes, &info, &mut output, 1 << 20).unwrap();
            // 8-bit lossless samples decode to k/255 exactly up to float
            // rounding; rounding back must recover every sample.
            let samples: Vec<u8> = output
                .iter()
                .map(|&v| {
                    let scaled = v * 255.0;
                    assert!((scaled - scaled.round()).abs() < 1e-3, "{v}");
                    assert!((0.0..=255.0).contains(&scaled), "{v}");
                    #[allow(clippy::cast_sign_loss)] // Range checked above.
                    let sample = scaled.round() as u8;
                    sample
                })
                .collect();
            assert_eq!(to_rgba(info.channels as usize, &samples), data);
        }
    }
}

#[cfg(all(feature = "jpeg-xl-read", feature = "webp-read"))]
mod pipeline_round_trip {
    use webp::api::encode::{PixelLayout, PixelRows, encode_lossless};

    use super::independent_decode::{decode_oxide, to_rgba};
    use super::{options, rows};
    use crate::pipeline::{
        EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat, InputProfilePolicy,
        OutputFormat, ProcessingErrorKind, RowEncoder, TransformOptions, transcode, transcode_with,
    };

    /// Captures the finished rows the built-in sink would receive.
    struct Rows;
    struct Captured(Vec<u8>);
    impl EncoderBackend for Rows {
        fn format(&self) -> OutputFormat {
            OutputFormat::JpegXl
        }
        fn name(&self) -> &'static str {
            "rgba-reference"
        }
        fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
            Ok(Box::new(Captured(Vec::new())))
        }
    }
    impl RowEncoder for Captured {
        fn write_row(&mut self, _: u32, rgba: &[u8]) -> Result<(), EncodeError> {
            self.0.extend_from_slice(rgba);
            Ok(())
        }
        fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
            Ok(EncodedImage::new(OutputFormat::JpegXl, self.0, 0, false))
        }
    }

    fn webp(width: usize, height: usize, alpha: bool) -> Vec<u8> {
        encode_lossless(PixelRows {
            layout: PixelLayout::Rgba,
            data: &rows(width, height, alpha),
            width: width as u32,
            height: height as u32,
            stride: width * 4,
        })
        .unwrap()
    }

    fn srgb(width: u32, height: u32) -> TransformOptions {
        TransformOptions {
            profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
            ..options(width, height)
        }
    }

    #[test]
    fn transcoded_jpeg_xl_decodes_to_the_finished_rows() {
        // A downscale (resampled, quantized rows) and a same-size transcode.
        for (alpha, (width, height)) in [(false, (120, 90)), (true, (120, 90)), (true, (64, 48))] {
            let input = webp(120, 90, alpha);
            let output = transcode(&input, ImageFormat::WebP, srgb(width, height)).unwrap();
            assert_eq!(output.format, OutputFormat::JpegXl);
            assert_eq!(output.report.encoder_backend, "libjxl-rs");
            assert!(output.report.opaque_encoder_memory);
            let (w, h) = (width as usize, height as usize);
            assert!(output.report.encoder_output_frame_bytes >= w * h * 4);
            let expected =
                transcode_with(&input, ImageFormat::WebP, srgb(width, height), &Rows).unwrap();
            let (dw, dh, channels, samples) = decode_oxide(&output.bytes);
            assert_eq!((dw, dh), (width, height));
            assert_eq!(to_rgba(channels, &samples), expected.bytes);
            // Opaque finished rows drop the alpha channel; transparent keep it.
            assert_eq!(channels == 4, expected.bytes.chunks(4).any(|p| p[3] != 255));
        }
    }

    #[test]
    fn encoded_output_limit_applies_at_the_exact_byte() {
        let input = webp(40, 30, true);
        let full = transcode(&input, ImageFormat::WebP, srgb(40, 30)).unwrap();
        let exact = TransformOptions {
            max_output_bytes: full.bytes.len(),
            ..srgb(40, 30)
        };
        assert_eq!(
            transcode(&input, ImageFormat::WebP, exact).unwrap().bytes,
            full.bytes
        );
        let short = TransformOptions {
            max_output_bytes: full.bytes.len() - 1,
            ..srgb(40, 30)
        };
        assert_eq!(
            transcode(&input, ImageFormat::WebP, short)
                .unwrap_err()
                .kind(),
            ProcessingErrorKind::ResourceExhausted
        );
    }

    #[cfg(feature = "jpeg-read")]
    #[test]
    fn oriented_input_is_written_upright_with_orientation_one() {
        // EXIF orientation 6 rotates the stored 24x16 pattern by 90 degrees.
        let input = include_bytes!("../tests/fixtures/orientation/jpeg-6.jpg");
        let reference = include_bytes!("../tests/fixtures/orientation/jpeg-6.rgb");
        let options = srgb(16, 24);
        let output = transcode(input, ImageFormat::Jpeg, options).unwrap();
        let (w, h, channels, samples) = decode_oxide(&output.bytes);
        assert_eq!((w, h, channels), (16, 24, 3));
        let expected = transcode_with(input, ImageFormat::Jpeg, options, &Rows).unwrap();
        assert_eq!(to_rgba(channels, &samples), expected.bytes);
        // The independent libjpeg upright reference matches exactly on this
        // fixture (measured 2026-11-12); the fixture README allows up to 2
        // between JPEG decoders, so a future decoder change may relax this.
        assert_eq!(&samples[..], &reference[..]);
    }
}
