use webp::api::decode::{ColorMode, Config as DecodeConfig, Container, decode};
use webp::api::encode::{PixelLayout, PixelRows, encode_lossless, encode_lossy};

use crate::geometry::Size;
use crate::memory::MemoryBudget;
use crate::pipeline::{
    BatchItem, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, Engine, ImageFormat,
    InputProfilePolicy, OutputFormat, ProcessingErrorKind, RowEncoder, Threads, TransformOptions,
    WebpOptions, transcode, transcode_with,
};
use crate::pipeline::{OutputGeometry, OutputSettings, WebpLossless};
use crate::webp_sink::WebpSink;

struct RgbaBackend;
struct RgbaRows(Vec<u8>);
impl EncoderBackend for RgbaBackend {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "rgba-reference"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        Ok(Box::new(RgbaRows(Vec::new())))
    }
}
impl RowEncoder for RgbaRows {
    fn write_row(&mut self, _: u32, rgba: &[u8]) -> Result<(), EncodeError> {
        self.0.extend_from_slice(rgba);
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(OutputFormat::WebP, self.0, 0, false))
    }
}

fn input() -> Vec<u8> {
    let mut pixels = Vec::new();
    for y in 0..8_u8 {
        for x in 0..8_u8 {
            pixels.extend_from_slice(&[
                x * 29,
                y * 31,
                x * 7 + y * 11,
                if x == 0 && y == 0 { 0 } else { 32 + x * 20 },
            ]);
        }
    }
    encode_lossless(PixelRows {
        layout: PixelLayout::Rgba,
        data: &pixels,
        width: 8,
        height: 8,
        stride: 32,
    })
    .unwrap()
}

fn opaque_input() -> Vec<u8> {
    let pixels = [30_u8, 80, 140].repeat(64);
    encode_lossless(PixelRows {
        layout: PixelLayout::Rgb,
        data: &pixels,
        width: 8,
        height: 8,
        stride: 24,
    })
    .unwrap()
}

fn options(lossless: bool) -> TransformOptions {
    TransformOptions::new(OutputGeometry::Exact {
        width: 8,
        height: 8,
    })
    .with_output(OutputSettings::WebP(if lossless {
        WebpOptions::Lossless(WebpLossless::new(4))
    } else {
        WebpOptions::default()
    }))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
}

#[test]
fn still_webp_outputs_decode_with_straight_alpha_and_srgb_metadata() {
    for lossless in [false, true] {
        let output = transcode(&input(), ImageFormat::WebP, &options(lossless)).unwrap();
        assert_eq!(output.format, OutputFormat::WebP);
        assert_eq!(&output.bytes[..4], b"RIFF");
        assert_eq!(&output.bytes[8..12], b"WEBP");
        let container = Container::new(&output.bytes).unwrap();
        let icc = container.icc_profile().unwrap();
        assert_eq!(icc, include_bytes!("../assets/icc/sRGB2014.icc"));
        assert!(container.exif().is_none());
        let decoded = decode(&output.bytes, &DecodeConfig::new(ColorMode::Rgba)).unwrap();
        assert_eq!(decoded.width(), 8);
        assert_eq!(decoded.height(), 8);
        let rows = decoded.rows().unwrap();
        let first = rows.row(0).unwrap();
        assert!(first[3] < 255);
        assert!(first[3] < first[7]);
        if lossless {
            let expected =
                transcode_with(&input(), ImageFormat::WebP, &options(true), &RgbaBackend).unwrap();
            let actual: Vec<u8> = (0..8).flat_map(|y| rows.row(y).unwrap().to_vec()).collect();
            assert_eq!(actual, expected.bytes);
        }
        assert!(output.report.encoder_output_frame_bytes >= 8 * 8 * 4);
        assert!(output.report.opaque_encoder_memory);
    }
}

#[test]
fn webp_output_frame_and_file_limits_fail_before_retaining_result() {
    let tiny = options(false).with_max_managed_bytes(8 * 8 * 4 - 1);
    assert_eq!(
        transcode(&input(), ImageFormat::WebP, &tiny)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::ResourceExhausted
    );
    // The output limit has its own kind since 2026-11-16
    // (`ResourceExhausted` before).
    let tiny = options(false).with_max_output_bytes(1024);
    assert_eq!(
        transcode(&input(), ImageFormat::WebP, &tiny)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::OutputLimitExceeded
    );
}

#[test]
fn webp_encoder_threads_other_than_one_name_the_serial_encoder() {
    for threads in [0, 2] {
        let threaded = options(false).with_encoder_threads(
            std::num::NonZeroUsize::new(threads).map_or(Threads::Automatic, Threads::Count),
        );
        let error = transcode(&input(), ImageFormat::WebP, &threaded).unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::InvalidOptions);
        assert_eq!(
            error.to_string(),
            "the WebP encoder is serial; set encoder_threads to 1"
        );
        // An external backend validates its own thread count (ADR-0024).
        let threaded = options(true).with_encoder_threads(
            std::num::NonZeroUsize::new(threads).map_or(Threads::Automatic, Threads::Count),
        );
        assert!(transcode_with(&input(), ImageFormat::WebP, &threaded, &RgbaBackend).is_ok());
    }
    let output = transcode(&input(), ImageFormat::WebP, &options(false)).unwrap();
    assert_eq!(output.report.encoder_threads, 1);
}

#[test]
fn exact_final_byte_limit_accepts_opaque_lossy_alpha_and_lossless() {
    for (input, lossless) in [(opaque_input(), false), (input(), false), (input(), true)] {
        let options = options(lossless);
        let full = transcode(&input, ImageFormat::WebP, &options).unwrap();
        let at_limit = transcode(
            &input,
            ImageFormat::WebP,
            &options.clone().with_max_output_bytes(full.bytes.len()),
        )
        .unwrap();
        assert_eq!(at_limit.bytes, full.bytes);
        let too_small = transcode(
            &input,
            ImageFormat::WebP,
            &options.clone().with_max_output_bytes(full.bytes.len() - 1),
        );
        assert_eq!(
            too_small.unwrap_err().kind(),
            ProcessingErrorKind::OutputLimitExceeded
        );
    }
}

#[test]
fn lossy_converted_planes_are_reserved_before_encoding() {
    // 8×8 ARGB = 256 bytes; YUV420 plus alpha = 64 + 16 + 16 + 64.
    let budget = MemoryBudget::new(256 + 160 - 1);
    let mut sink = WebpSink::new(Size::new(8, 8).unwrap(), &options(false), &budget).unwrap();
    let row = [10_u8, 20, 30, 64].repeat(8);
    for y in 0..8 {
        sink.write_row(y, &row).unwrap();
    }
    assert!(sink.finish().is_err());
    assert_eq!(budget.snapshot().live, 0);
    assert_eq!(budget.snapshot().peak, 256);
}

#[test]
fn multithreaded_webp_transcode_matches_single_threaded_for_lossy_and_lossless() {
    let (width, height) = (545_u32, 81_u32);
    let make_input = |alpha: bool, lossy: bool| -> Vec<u8> {
        let channels = if alpha { 4 } else { 3 };
        let mut pixels = Vec::with_capacity(width as usize * height as usize * channels);
        for y in 0..height {
            for x in 0..width {
                pixels.extend_from_slice(&[
                    u8::try_from((x * 7 + y * 13) % 256).unwrap(),
                    u8::try_from(((x * 19) ^ (y * 29)) % 256).unwrap(),
                    u8::try_from((x * 3 + y * 5) % 256).unwrap(),
                ]);
                if alpha {
                    pixels.push(u8::try_from((x * 11 + y * 17) % 256).unwrap());
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
            width,
            height,
            stride: width as usize * channels,
        };
        if lossy {
            encode_lossy(rows, 78.0).unwrap()
        } else {
            encode_lossless(rows).unwrap()
        }
    };
    let four = Threads::Count(std::num::NonZeroUsize::new(4).unwrap());
    let two = Threads::Count(std::num::NonZeroUsize::new(2).unwrap());
    let base = TransformOptions::new(OutputGeometry::Exact {
        width: 256,
        height: 48,
    })
    .with_output(OutputSettings::WebP(WebpOptions::default()))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);

    for (alpha, lossy) in [(false, true), (true, true), (false, false), (true, false)] {
        let encoded = make_input(alpha, lossy);
        let single = transcode(
            &encoded,
            ImageFormat::WebP,
            &base.clone().with_transform_threads(Threads::ONE),
        )
        .unwrap();
        let multi = transcode(
            &encoded,
            ImageFormat::WebP,
            &base.clone().with_transform_threads(four),
        )
        .unwrap();
        assert_eq!(
            multi.bytes, single.bytes,
            "alpha={alpha} lossy={lossy}: multithreaded WebP transcode changed bytes"
        );
        let batch = Engine::new(two).unwrap().transcode_batch(&[BatchItem::new(
            &encoded,
            ImageFormat::WebP,
            base.clone().with_transform_threads(four),
        )]);
        let batched = batch.into_iter().next().unwrap().unwrap();
        assert_eq!(batched.bytes, single.bytes);
        assert_eq!(batched.report.transform_threads, 1);
    }
}
