//! Lossy JPEG XL output (ADR-0039): the distance range, the alpha and
//! patch rejections, budget and output limits, and independent decodes.
//!
//! The pixel bounds below are regression pins measured on these fixtures
//! with margin, not conformance evidence: byte-exactness against libjxl is
//! the port's own gate (`encoder_public` parity tests), and the decoded
//! error of a lossy codec depends on content.
// Test images are small; their coordinates and samples fit every cast.
#![allow(clippy::cast_possible_truncation)]

use crate::error::Error;
use crate::geometry::Size;
use crate::jxl_sink::{JxlSink, estimated_lossy_codec_bytes};
use crate::memory::MemoryBudget;
use crate::pipeline::{
    ImageFormat, JpegXlOptions, OutputFormat, ProcessingError, ProcessingErrorKind,
    TransformOptions, transcode,
};

fn lossy(width: u32, height: u32, distance: f32) -> TransformOptions {
    TransformOptions {
        output_format: OutputFormat::JpegXl,
        jpeg_xl: JpegXlOptions::Lossy { distance },
        output_width: width,
        output_height: height,
        ..TransformOptions::default()
    }
}

/// Smooth photo-like RGBA content: gradients and a low-amplitude texture.
fn smooth(width: usize, height: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(width * height * 4);
    for y in 0..height {
        for x in 0..width {
            let hash = (x as u32).wrapping_mul(2_654_435_761) ^ (y as u32).wrapping_mul(40_503);
            let noise = (hash >> 13) as u8 & 0x07;
            data.extend_from_slice(&[
                ((x * 255) / width.max(2)) as u8 ^ noise,
                ((y * 255) / height.max(2)) as u8,
                (((x + y) * 255) / (width + height)) as u8 ^ (noise << 1),
                255,
            ]);
        }
    }
    data
}

/// Text-like RGBA content: copies of one small dark glyph on a flat light
/// background, as in the port's patch-detection parity corpus
/// (`glyph_image` in `codecs/libjxl-rs/tests/parity/encoder_coefficients.rs`,
/// quantized to 8 bits). `FindTextLikePatches` finds patches in it.
pub(crate) fn glyphs(width: usize, height: usize, glyph: usize, copies: usize) -> Vec<u8> {
    let mut s = 0x9e37_79b9_7f4a_7c15_u64 | 1;
    let mut rnd = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let mut data = [242_u8, 237, 230, 255].repeat(width * height);
    let shape: Vec<bool> = (0..glyph * glyph).map(|_| rnd() % 3 != 0).collect();
    let span = width.saturating_sub(glyph + 8).max(1);
    for k in 0..copies {
        let gx = 4 + (k * (glyph + 6)) % span;
        let gy = 4 + ((k * (glyph + 6)) / span) * (glyph + 6);
        if gy + glyph + 4 >= height {
            break;
        }
        for y in 0..glyph {
            for x in 0..glyph {
                if shape[y * glyph + x] {
                    let p = ((gy + y) * width + gx + x) * 4;
                    data[p..p + 3].copy_from_slice(&[26, 38, 51]);
                }
            }
        }
    }
    data
}

fn encode(
    width: usize,
    height: usize,
    data: &[u8],
    options: TransformOptions,
    budget: &MemoryBudget,
) -> Result<crate::pipeline::EncodedImage, Error> {
    let size = Size::new(width as u32, height as u32).unwrap();
    let mut sink = JxlSink::new(size, options, budget)?;
    for (y, row) in data.chunks_exact(width * 4).enumerate() {
        sink.write_row(y as u32, row)?;
    }
    sink.finish()
}

/// Maximum and mean absolute difference over the RGB samples of two RGBA
/// buffers.
#[cfg(feature = "jpeg-xl-read")]
fn rgb_error(decoded: &[u8], expected: &[u8]) -> (u8, f64) {
    assert_eq!(decoded.len(), expected.len());
    let mut max = 0;
    let mut sum = 0_u64;
    for (a, b) in decoded
        .as_chunks::<4>()
        .0
        .iter()
        .zip(expected.as_chunks::<4>().0)
    {
        for c in 0..3 {
            let d = a[c].abs_diff(b[c]);
            max = max.max(d);
            sum += u64::from(d);
        }
    }
    #[allow(clippy::cast_precision_loss)] // Test-sized sample counts.
    let mean = sum as f64 / (expected.len() / 4 * 3) as f64;
    (max, mean)
}

#[test]
fn lossy_distance_one_is_the_default() {
    assert_eq!(
        JpegXlOptions::default(),
        JpegXlOptions::Lossy { distance: 1.0 }
    );
}

#[test]
fn distances_outside_the_admitted_range_are_rejected_before_allocating() {
    let budget = MemoryBudget::new(1 << 20);
    let size = Size::new(8, 8).unwrap();
    for distance in [0.0, 0.049, 3.0, 3.5, 25.0, -1.0, f32::NAN, f32::INFINITY] {
        let error = JxlSink::new(size, lossy(8, 8, distance), &budget)
            .err()
            .unwrap();
        assert!(
            matches!(error, Error::UnsupportedEncoderProfile(_)),
            "{distance}"
        );
        assert!(error.to_string().contains("distance"), "{error}");
        // The facade rejects it before opening the (garbage) input.
        let kind = transcode(b"garbage", ImageFormat::JpegXl, lossy(8, 8, distance))
            .unwrap_err()
            .kind();
        assert_eq!(kind, ProcessingErrorKind::InvalidOptions, "{distance}");
    }
    assert_eq!(budget.snapshot().peak, 0);
    for distance in [0.05, 1.0, 2.999] {
        JxlSink::new(size, lossy(8, 8, distance), &budget).unwrap();
    }
}

#[test]
fn real_alpha_fails_at_the_first_transparent_row_and_opaque_alpha_is_dropped() {
    let budget = MemoryBudget::new(64 << 20);
    let (width, height) = (16, 4);
    let mut data = smooth(width, height);
    // Fully opaque RGBA rows encode; the alpha channel is not written.
    let opaque = encode(width, height, &data, lossy(16, 4, 1.0), &budget).unwrap();
    assert_eq!(&opaque.bytes[..2], &[0xff, 0x0a]);
    // One pixel below 255 in row 2 stops the sink there.
    data[(2 * width + 5) * 4 + 3] = 254;
    let size = Size::new(16, 4).unwrap();
    let mut sink = JxlSink::new(size, lossy(16, 4, 1.0), &budget).unwrap();
    for (y, row) in data.chunks_exact(width * 4).enumerate().take(2) {
        sink.write_row(y as u32, row).unwrap();
    }
    let error = sink
        .write_row(2, &data[2 * width * 4..3 * width * 4])
        .unwrap_err();
    assert_eq!(error, Error::LossyJpegXlAlpha);
    assert!(
        error
            .to_string()
            .contains("lossy JPEG XL with alpha is not supported yet; use lossless")
    );
    let public = ProcessingError::from(error);
    assert_eq!(public.kind(), ProcessingErrorKind::UnsupportedInput);
    // The same rows encode losslessly with their alpha channel.
    let lossless = TransformOptions {
        jpeg_xl: JpegXlOptions::Lossless { effort: 2 },
        ..lossy(16, 4, 1.0)
    };
    encode(width, height, &data, lossless, &budget).unwrap();
}

#[test]
fn text_like_content_reports_the_missing_patch_path() {
    let budget = MemoryBudget::new(64 << 20);
    let data = glyphs(64, 64, 5, 6);
    let error = encode(64, 64, &data, lossy(64, 64, 1.0), &budget).unwrap_err();
    assert_eq!(error, Error::JpegXlPatchesRequired);
    assert!(error.to_string().contains("patch path"), "{error}");
    assert_eq!(
        ProcessingError::from(error).kind(),
        ProcessingErrorKind::UnsupportedInput
    );
    assert_eq!(budget.snapshot().live, 0);
    // Lossless output of the same content is unaffected.
    let lossless = TransformOptions {
        jpeg_xl: JpegXlOptions::Lossless { effort: 2 },
        ..lossy(64, 64, 1.0)
    };
    encode(64, 64, &data, lossless, &budget).unwrap();
}

#[test]
fn input_frame_and_codec_estimate_are_reserved() {
    let (width, height) = (24, 16);
    let frame = width * height * 4;
    let codec = estimated_lossy_codec_bytes(width, height).unwrap();
    // Regression pin of the bridge's estimate (input copy, seven f32 planes
    // padded to 8x8, one group of i32 coefficients); it is an estimate from
    // the port's source, not a measurement.
    assert_eq!(codec, width * height * 3 + 24 * 16 * 28 + 3 * 256 * 256 * 4);
    let data = smooth(width, height);
    let short = MemoryBudget::new(frame + codec - 1);
    assert!(matches!(
        encode(width, height, &data, lossy(24, 16, 1.0), &short),
        Err(Error::BudgetExceeded { .. })
    ));
    assert_eq!(short.snapshot().live, 0);
    assert_eq!(short.snapshot().peak, frame);
    let exact = MemoryBudget::new(frame + codec);
    let encoded = encode(width, height, &data, lossy(24, 16, 1.0), &exact).unwrap();
    assert_eq!(exact.snapshot().peak, frame + codec);
    assert_eq!(exact.snapshot().live, 0);
    assert_eq!(encoded.frame_materialization_bytes, frame + codec);
    assert!(encoded.opaque_memory);
}

#[test]
fn the_output_limit_applies_at_the_exact_byte() {
    let budget = MemoryBudget::new(64 << 20);
    let data = smooth(40, 30);
    let full = encode(40, 30, &data, lossy(40, 30, 1.0), &budget).unwrap();
    let exact = TransformOptions {
        max_output_bytes: full.bytes.len(),
        ..lossy(40, 30, 1.0)
    };
    assert_eq!(
        encode(40, 30, &data, exact, &budget).unwrap().bytes,
        full.bytes
    );
    let short = TransformOptions {
        max_output_bytes: full.bytes.len() - 1,
        ..lossy(40, 30, 1.0)
    };
    assert!(matches!(
        encode(40, 30, &data, short, &budget),
        Err(Error::BudgetExceeded { .. })
    ));
    assert_eq!(budget.snapshot().live, 0);
}

#[test]
fn default_options_of_other_formats_still_match() {
    // The JPEG XL default changed to lossy; it must not make the default
    // options of the other formats look mismatched.
    for format in [OutputFormat::Avif, OutputFormat::WebP] {
        let options = TransformOptions {
            output_format: format,
            output_width: 8,
            output_height: 8,
            ..TransformOptions::default()
        };
        let kind = transcode(b"garbage", ImageFormat::JpegXl, options)
            .unwrap_err()
            .kind();
        assert_ne!(kind, ProcessingErrorKind::InvalidOptions, "{format:?}");
        // A JPEG XL setting with another format is still a mismatch.
        let mismatched = TransformOptions {
            jpeg_xl: JpegXlOptions::Lossless { effort: 2 },
            ..options
        };
        assert_eq!(
            transcode(b"garbage", ImageFormat::JpegXl, mismatched)
                .unwrap_err()
                .kind(),
            ProcessingErrorKind::InvalidOptions
        );
    }
}

#[cfg(feature = "jpeg-xl-read")]
mod independent_decode {
    use jxl_oxide::JxlImage;
    use jxl_oxide::color::{
        ColourEncoding, ColourSpace, Primaries, RenderingIntent, TransferFunction, WhitePoint,
    };

    use super::{encode, lossy, rgb_error, smooth};
    use crate::memory::MemoryBudget;

    /// Decode with jxl-oxide, check the emitted signaling (XYB-encoded, enum
    /// sRGB, orientation 1, no extra channels) and return RGBA8 samples.
    pub(crate) fn decode_oxide(bytes: &[u8]) -> (u32, u32, Vec<u8>) {
        assert_eq!(&bytes[..2], &[0xff, 0x0a], "raw codestream signature");
        let image = JxlImage::builder()
            .read(std::io::Cursor::new(bytes))
            .unwrap();
        let metadata = &image.image_header().metadata;
        assert!(metadata.xyb_encoded, "lossy samples are XYB-encoded");
        assert_eq!(metadata.orientation, 1);
        assert!(metadata.ec_info.is_empty(), "no alpha or extra channels");
        let ColourEncoding::Enum(color) = &metadata.colour_encoding else {
            panic!("expected enum color, found an ICC profile")
        };
        assert_eq!(color.colour_space, ColourSpace::Rgb);
        assert_eq!(color.white_point, WhitePoint::D65);
        assert_eq!(color.primaries, Primaries::Srgb);
        assert_eq!(color.tf, TransferFunction::Srgb);
        assert_eq!(color.rendering_intent, RenderingIntent::Relative);
        let render = image.render_frame(0).unwrap();
        let mut stream = render.stream();
        assert_eq!(stream.channels(), 3);
        let mut rgb = vec![0_u8; (stream.width() * stream.height()) as usize * 3];
        stream.write_to_buffer(&mut rgb);
        let rgba = rgb
            .as_chunks::<3>()
            .0
            .iter()
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect();
        (image.width(), image.height(), rgba)
    }

    /// d1.0 on the smooth synthetic content: one-shot (up to 2x2 groups) and
    /// streaming (4x3 groups, more than 8) paths. Measured on 2026-11-12
    /// (max / mean absolute RGB error against the input rows): 1x1 0 / 0,
    /// 9x17 37 / 4.00, 300x260 19 / 2.22, 800x700 18 / 2.09. The pins add
    /// about 30 % (at least a few levels) to the maximum and 50 % to the
    /// mean.
    #[test]
    fn lossy_output_decodes_within_the_pinned_error() {
        for (width, height, max_bound, mean_bound) in [
            (1, 1, 2, 0.5),
            (9, 17, 48, 6.0),
            (300, 260, 26, 3.4),
            (800, 700, 26, 3.2),
        ] {
            let data = smooth(width, height);
            let encoded = encode(
                width,
                height,
                &data,
                lossy(width as u32, height as u32, 1.0),
                &MemoryBudget::new(256 << 20),
            )
            .unwrap();
            let (w, h, decoded) = decode_oxide(&encoded.bytes);
            assert_eq!((w as usize, h as usize), (width, height));
            let (max, mean) = rgb_error(&decoded, &data);
            assert!(
                max <= max_bound && mean <= mean_bound,
                "{width}x{height}: max {max} mean {mean}"
            );
        }
    }

    #[cfg(feature = "jpeg-xl-own-read")]
    #[test]
    fn the_workspace_decoder_agrees_with_jxl_oxide() {
        let (width, height) = (300, 260);
        let data = smooth(width, height);
        let encoded = encode(
            width,
            height,
            &data,
            lossy(300, 260, 1.0),
            &MemoryBudget::new(64 << 20),
        )
        .unwrap();
        let (_, _, oxide) = decode_oxide(&encoded.bytes);
        let info = ferrisma_jxl_decoder::inspect(&encoded.bytes, 1 << 20).unwrap();
        assert_eq!((info.width, info.height, info.channels), (300, 260, 3));
        assert_eq!(
            info.data_transfer,
            Some(ferrisma_jxl_decoder::Transfer::Other)
        );
        let mut samples = vec![0_f32; info.sample_count().unwrap()];
        ferrisma_jxl_decoder::decode_into(&encoded.bytes, &info, &mut samples, 1 << 20).unwrap();
        #[allow(clippy::cast_sign_loss)] // Clamped to 0..=255 first.
        let own: Vec<u8> = samples
            .as_chunks::<3>()
            .0
            .iter()
            .flat_map(|p| {
                let q = |v: f32| (v * 255.0).round().clamp(0.0, 255.0) as u8;
                [q(p[0]), q(p[1]), q(p[2]), 255]
            })
            .collect();
        // Measured 2026-11-12: the two decoders differ by at most one level
        // (mean 0.006; float output rounded here, jxl-oxide's own u8
        // conversion there), and the workspace decode is 19 / 2.22 from the
        // input like jxl-oxide's.
        let (max, mean) = rgb_error(&own, &oxide);
        assert!(
            max <= 2 && mean <= 0.05,
            "own vs oxide: max {max} mean {mean}"
        );
        let (max, mean) = rgb_error(&own, &data);
        assert!(
            max <= 26 && mean <= 3.4,
            "own vs input: max {max} mean {mean}"
        );
    }
}

#[cfg(all(feature = "jpeg-xl-read", feature = "webp-read"))]
mod pipeline_round_trip {
    use webp::api::encode::{PixelLayout, PixelRows, encode_lossless};

    use super::independent_decode::decode_oxide;
    use super::{glyphs, lossy, rgb_error, smooth};
    use crate::pipeline::{
        EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat, InputProfilePolicy,
        JpegXlOptions, OutputFormat, ProcessingErrorKind, RowEncoder, TransformOptions, transcode,
        transcode_with,
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

    fn webp(width: usize, height: usize, data: &[u8]) -> Vec<u8> {
        encode_lossless(PixelRows {
            layout: PixelLayout::Rgba,
            data,
            width: width as u32,
            height: height as u32,
            stride: width * 4,
        })
        .unwrap()
    }

    fn srgb(options: TransformOptions) -> TransformOptions {
        TransformOptions {
            profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
            ..options
        }
    }

    #[test]
    fn transcoded_lossy_output_decodes_near_the_finished_rows() {
        let input = webp(120, 90, &smooth(120, 90));
        for (width, height) in [(120, 90), (64, 48)] {
            let options = srgb(lossy(width, height, 1.0));
            let output = transcode(&input, ImageFormat::WebP, options).unwrap();
            assert_eq!(output.format, OutputFormat::JpegXl);
            assert_eq!(output.report.encoder_backend, "libjxl-rs");
            assert!(output.report.opaque_encoder_memory);
            let expected = transcode_with(&input, ImageFormat::WebP, options, &Rows).unwrap();
            let (w, h, decoded) = decode_oxide(&output.bytes);
            assert_eq!((w, h), (width, height));
            // Measured 2026-11-12: 120x90 22 / 2.60, 64x48 23 / 1.67.
            let (max, mean) = rgb_error(&decoded, &expected.bytes);
            assert!(
                max <= 30 && mean <= 3.9,
                "{width}x{height}: max {max} mean {mean}"
            );
        }
    }

    #[test]
    fn transparent_input_fails_for_lossy_and_encodes_losslessly() {
        let mut data = smooth(32, 24);
        data[(5 * 32 + 7) * 4 + 3] = 0;
        let input = webp(32, 24, &data);
        let error = transcode(&input, ImageFormat::WebP, srgb(lossy(32, 24, 1.0))).unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::UnsupportedInput);
        assert!(error.to_string().contains("use lossless"), "{error}");
        let lossless = TransformOptions {
            jpeg_xl: JpegXlOptions::Lossless { effort: 2 },
            ..srgb(lossy(32, 24, 1.0))
        };
        transcode(&input, ImageFormat::WebP, lossless).unwrap();
    }

    #[test]
    fn text_like_input_reports_the_missing_patch_path() {
        let input = webp(64, 64, &glyphs(64, 64, 5, 6));
        let error = transcode(&input, ImageFormat::WebP, srgb(lossy(64, 64, 1.0))).unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::UnsupportedInput);
        assert!(error.to_string().contains("patch path"), "{error}");
    }

    #[cfg(feature = "jpeg-read")]
    #[test]
    fn oriented_input_is_written_upright_with_orientation_one() {
        // EXIF orientation 6 rotates the stored 24x16 pattern by 90 degrees.
        let input = include_bytes!("../tests/fixtures/orientation/jpeg-6.jpg");
        let options = srgb(lossy(16, 24, 1.0));
        let output = transcode(input, ImageFormat::Jpeg, options).unwrap();
        let (w, h, decoded) = decode_oxide(&output.bytes);
        assert_eq!((w, h), (16, 24));
        let expected = transcode_with(input, ImageFormat::Jpeg, options, &Rows).unwrap();
        // The 90-degree rotation is visible: a transposed decode would be far
        // off. Measured 2026-11-12 on this high-contrast 16x24 pattern:
        // max 61, mean 4.79.
        let (max, mean) = rgb_error(&decoded, &expected.bytes);
        assert!(max <= 80 && mean <= 7.2, "max {max} mean {mean}");
    }

    /// A real photograph: the 2268x1512 flower JPEG as a 600x400 thumbnail.
    #[cfg(feature = "jpeg-read")]
    #[test]
    fn photograph_thumbnail_decodes_near_the_finished_rows() {
        let input =
            include_bytes!("../codecs/libjxl-rs/tests/data/decode-test/flower.png.im_q85_420.jpg");
        let options = srgb(lossy(600, 400, 1.0));
        let output = transcode(input, ImageFormat::Jpeg, options).unwrap();
        let expected = transcode_with(input, ImageFormat::Jpeg, options, &Rows).unwrap();
        let (w, h, decoded) = decode_oxide(&output.bytes);
        assert_eq!((w, h), (600, 400));
        // Measured 2026-11-12: 73,020 bytes, max 79, mean 2.76.
        let (max, mean) = rgb_error(&decoded, &expected.bytes);
        assert!(max <= 100 && mean <= 4.1, "max {max} mean {mean}");
    }
}
