//! Transparent lossy JPEG XL written by the engine and read back by the
//! engine (issue #222, ADR-0015, ADR-0032, ADR-0039).
//!
//! The lossy sink writes XYB-encoded, enum sRGB samples with one straight
//! 8-bit alpha channel. Each compiled JPEG XL decoder must declare that
//! alpha as straight, and the rendered-ICC stage must convert the three
//! color channels while carrying alpha through unchanged. Associated alpha
//! and the strict policy stay refused.
// Test images are small; their coordinates and samples fit every cast.
#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use std::sync::Mutex;

use crate::color::AlphaMode;
use crate::decoder::{DecodeSource, DecodedRow, DecoderInfo};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::jxl_lossy_tests::translucent;
use crate::jxl_sink::JxlSink;
use crate::memory::MemoryBudget;
use crate::pipeline::{
    EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat, InputProfilePolicy,
    JpegXlDecoder, JpegXlOptions, OutputFormat, ProcessingErrorKind, RowEncoder, TransformOptions,
    transcode_with,
};

use crate::jxl_color_tests::oracle;

#[allow(clippy::vec_init_then_push)] // Entries depend on the features.
fn decoders() -> Vec<JpegXlDecoder> {
    let mut all = Vec::new();
    #[cfg(feature = "jpeg-xl-read")]
    all.push(JpegXlDecoder::JxlOxide);
    #[cfg(feature = "jpeg-xl-own-read")]
    all.push(JpegXlDecoder::LibjxlRs);
    all
}

/// One decoder's adapter, as the pipeline opens it.
fn open(decoder: JpegXlDecoder, bytes: &[u8]) -> Result<Box<dyn DecodeSource + '_>, Error> {
    match decoder {
        #[cfg(feature = "jpeg-xl-read")]
        JpegXlDecoder::JxlOxide => Ok(Box::new(crate::codecs::jxl_oxide::JxlSource::new(bytes)?)),
        #[cfg(feature = "jpeg-xl-own-read")]
        JpegXlDecoder::LibjxlRs => Ok(Box::new(crate::codecs::jxl::JxlSource::new(bytes)?)),
        #[allow(unreachable_patterns)]
        _ => unreachable!(),
    }
}

fn info(decoder: JpegXlDecoder, bytes: &[u8]) -> Result<DecoderInfo, Error> {
    open(decoder, bytes).map(|source| source.info().clone())
}

/// The engine's lossy JPEG XL output (default distance 1.0) for RGBA rows.
fn encode_lossy(width: usize, height: usize, rgba: &[u8]) -> Vec<u8> {
    let options = TransformOptions {
        output_format: OutputFormat::JpegXl,
        jpeg_xl: JpegXlOptions::Lossy { distance: 1.0 },
        output_width: width as u32,
        output_height: height as u32,
        ..TransformOptions::default()
    };
    let budget = MemoryBudget::new(256 << 20);
    let size = Size::new(width as u32, height as u32).unwrap();
    let mut sink = JxlSink::new(size, options, &budget).unwrap();
    for (y, row) in rgba.chunks_exact(width * 4).enumerate() {
        sink.write_row(y as u32, row).unwrap();
    }
    sink.finish().unwrap().bytes
}

struct Capture(Mutex<Vec<u8>>);
struct CaptureRows<'a>(&'a Mutex<Vec<u8>>);

impl EncoderBackend for Capture {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "test-row-capture"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        Ok(Box::new(CaptureRows(&self.0)))
    }
}

impl RowEncoder for CaptureRows<'_> {
    fn write_row(&mut self, _: u32, row: &[u8]) -> Result<(), EncodeError> {
        self.0.lock().unwrap().extend_from_slice(row);
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(OutputFormat::WebP, Vec::new(), 0, false))
    }
}

/// The finished RGBA8 rows the engine produces for a JPEG XL input at
/// identity geometry, or the error kind.
fn read_back(
    bytes: &[u8],
    decoder: JpegXlDecoder,
    policy: InputProfilePolicy,
    size: (u32, u32),
    threads: usize,
) -> Result<Vec<u8>, ProcessingErrorKind> {
    let capture = Capture(Mutex::new(Vec::new()));
    transcode_with(
        bytes,
        ImageFormat::JpegXl,
        TransformOptions {
            output_format: OutputFormat::WebP,
            output_width: size.0,
            output_height: size.1,
            profile_policy: policy,
            jxl_decoder: decoder,
            transform_threads: threads,
            ..TransformOptions::default()
        },
        &capture,
    )
    .map_err(|error| error.kind())?;
    Ok(capture.0.into_inner().unwrap())
}

/// The decoder's own float samples (sRGB-encoded color, alpha), straight
/// from its adapter, without the engine's color stage.
fn raw_samples(decoder: JpegXlDecoder, bytes: &[u8]) -> Vec<f32> {
    let mut source = open(decoder, bytes).unwrap();
    let size = source.info().size;
    let mut samples = Vec::new();
    source
        .read_region(
            Rect::new(0, 0, size.width, size.height).unwrap(),
            &MemoryBudget::new(64 << 20),
            &mut |_, row| {
                let DecodedRow::F32(row) = row else {
                    panic!("JPEG XL adapters emit f32")
                };
                samples.extend_from_slice(row);
                Ok(())
            },
        )
        .unwrap();
    samples
}

/// Bound per color sample around the decoder's own output: the ADR-0023
/// moxcms versus reference bound (`2e-3` linear) that the rendered-ICC
/// path already documents for JPEG XL (`jxl_color_tests`).
const BOUND: f64 = 2e-3;

/// Issue #222: the engine reads its own transparent lossy JPEG XL through
/// the rendered-ICC stage with each decoder. Alpha equals the written
/// alpha exactly. Color over visible pixels lies within `BOUND` linear of
/// the decoder's own sRGB output (decoded by the independent IEC curve),
/// and within the lossy pins of `jxl_lossy_tests` against the written
/// rows. Sizes: an odd one-shot frame, a 2x2-group one-shot frame and a
/// streaming frame (12 groups); one transform thread and the parallel
/// executor (deferred conversion on the workers).
#[cfg(feature = "icc")]
#[test]
fn rendered_icc_reads_engine_written_straight_alpha() {
    // (width, height, max, mean): the pins of
    // `lossy_alpha_decodes_exactly_and_color_within_the_pinned_error`.
    for (width, height, max_bound, mean_bound) in
        [(9, 17, 48, 6.0), (300, 260, 26, 3.4), (800, 700, 26, 3.2)]
    {
        let written = translucent(width, height);
        let bytes = encode_lossy(width, height, &written);
        for decoder in decoders() {
            let declared = info(decoder, &bytes).unwrap();
            assert_eq!(declared.alpha, AlphaMode::Straight, "{decoder:?}");
            let raw = raw_samples(decoder, &bytes);
            for threads in [1, 4] {
                let label = format!("{decoder:?} {width}x{height} threads {threads}");
                let rows = read_back(
                    &bytes,
                    decoder,
                    InputProfilePolicy::UseRenderedIcc,
                    (width as u32, height as u32),
                    threads,
                )
                .unwrap_or_else(|kind| panic!("{label}: {kind:?}"));
                assert_eq!(rows.len(), written.len(), "{label}");
                let mut max = 0_u8;
                let mut sum = 0_u64;
                let mut visible = 0_u64;
                let mut worst_code = 0_u8;
                for ((out, input), decoded) in rows
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .zip(written.as_chunks::<4>().0)
                    .zip(raw.as_chunks::<4>().0)
                {
                    assert_eq!(out[3], input[3], "{label}: alpha passes through");
                    assert_eq!(oracle::alpha8(f64::from(decoded[3])), input[3], "{label}");
                    if input[3] == 0 {
                        // ADR-0014: alpha 0 is transparent black.
                        assert_eq!(out[..3], [0, 0, 0], "{label}");
                        continue;
                    }
                    visible += 1;
                    for c in 0..3 {
                        let linear = oracle::srgb_eotf(f64::from(decoded[c]).clamp(0.0, 1.0));
                        let lo = oracle::encode8(linear - BOUND);
                        let hi = oracle::encode8(linear + BOUND);
                        assert!(
                            (lo..=hi).contains(&out[c]),
                            "{label}: code {} outside {lo}..={hi}",
                            out[c]
                        );
                        worst_code = worst_code.max(out[c].abs_diff(oracle::encode8(linear)));
                        let d = out[c].abs_diff(input[c]);
                        max = max.max(d);
                        sum += u64::from(d);
                    }
                }
                #[allow(clippy::cast_precision_loss)] // Test-sized sample counts.
                let mean = sum as f64 / (visible.max(1) * 3) as f64;
                eprintln!(
                    "{label}: vs written max {max} mean {mean:.3}; vs decoder max {worst_code} codes"
                );
                assert!(
                    max <= max_bound && mean <= mean_bound,
                    "{label}: max {max} mean {mean}"
                );
            }
        }
    }
}

/// The strict policy is unchanged: the engine's own JPEG XL output, opaque
/// or transparent, is refused without `UseRenderedIcc`, because both
/// adapters describe XYB output with a rendered ICC profile (ADR-0015).
#[test]
fn strict_policy_still_refuses_engine_written_jpeg_xl() {
    let (width, height) = (32, 24);
    let transparent = encode_lossy(width, height, &translucent(width, height));
    let opaque = encode_lossy(width, height, &[90, 140, 200, 255].repeat(width * height));
    for decoder in decoders() {
        for (name, bytes) in [("transparent", &transparent), ("opaque", &opaque)] {
            assert_eq!(
                read_back(
                    bytes,
                    decoder,
                    InputProfilePolicy::Strict,
                    (width as u32, height as u32),
                    1
                ),
                Err(ProcessingErrorKind::UnsupportedColor),
                "{decoder:?} {name}"
            );
        }
    }
}

/// Associated (premultiplied) alpha is not guessed: jxl-oxide declares it
/// `Unspecified` and libjxl-rs `Premultiplied`, and the rendered-ICC stage
/// refuses both, as does every other policy.
#[test]
fn associated_alpha_is_refused() {
    let (width, height) = (32, 24);
    let straight = encode_lossy(width, height, &translucent(width, height));
    let associated = mark_alpha_associated(&straight);
    for decoder in decoders() {
        let declared = info(decoder, &associated).unwrap();
        let expected = match decoder {
            JpegXlDecoder::JxlOxide => AlphaMode::Unspecified,
            _ => AlphaMode::Premultiplied,
        };
        assert_eq!(declared.alpha, expected, "{decoder:?}");
        // The rewrite changes only the association: same size and layout.
        let plain = info(decoder, &straight).unwrap();
        assert_eq!(
            (declared.size, declared.layout),
            (plain.size, plain.layout),
            "{decoder:?}"
        );
        for policy in [
            InputProfilePolicy::Strict,
            InputProfilePolicy::AssumeSrgbIfMissing,
            #[cfg(feature = "icc")]
            InputProfilePolicy::UseRenderedIcc,
        ] {
            assert!(
                matches!(
                    read_back(
                        &associated,
                        decoder,
                        policy,
                        (width as u32, height as u32),
                        1
                    ),
                    Err(ProcessingErrorKind::UnsupportedInput
                        | ProcessingErrorKind::UnsupportedColor)
                ),
                "{decoder:?} {policy:?}"
            );
        }
        #[cfg(feature = "icc")]
        assert_eq!(
            read_back(
                &associated,
                decoder,
                InputProfilePolicy::UseRenderedIcc,
                (width as u32, height as u32),
                1
            ),
            Err(ProcessingErrorKind::UnsupportedInput),
            "{decoder:?}: refused for its alpha, not its color"
        );
    }
}

/// Bit reader over a bare JPEG XL codestream (LSB first, ISO/IEC 18181-1).
struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
}

impl Bits<'_> {
    fn read(&mut self, n: usize) -> u32 {
        let mut value = 0;
        for i in 0..n {
            let bit = (self.data[(self.pos + i) / 8] >> ((self.pos + i) % 8)) & 1;
            value |= u32::from(bit) << i;
        }
        self.pos += n;
        value
    }
    fn bool(&mut self) -> bool {
        self.read(1) == 1
    }
    /// `U32(d0, d1, d2, d3)`, each `(offset, bits)`.
    fn u32(&mut self, dists: [(u32, usize); 4]) -> u32 {
        let (offset, bits) = dists[self.read(2) as usize];
        offset + self.read(bits)
    }
    fn enumeration(&mut self) -> u32 {
        self.u32([(0, 0), (1, 0), (2, 4), (18, 6)])
    }
}

/// Rewrite the engine's lossy codestream so that its one alpha channel is
/// signalled as associated: the `all_default` alpha `ExtraChannelInfo` (one
/// bit) is replaced by the explicit bundle with `alpha_associated` set, and
/// every other header bit and all byte-aligned frame data are copied
/// unchanged (the approach of `tests/fixtures/jxl-color/tools/respell.py`).
/// Only the header fields the engine writes are supported.
fn mark_alpha_associated(data: &[u8]) -> Vec<u8> {
    const SIZE: [(u32, usize); 4] = [(1, 9), (1, 13), (1, 18), (1, 30)];
    assert_eq!(&data[..2], &[0xff, 0x0a], "bare codestream");
    let mut r = Bits { data, pos: 16 };
    // SizeHeader.
    let div8 = r.bool();
    if div8 {
        r.read(5);
    } else {
        r.u32(SIZE);
    }
    if r.read(3) == 0 {
        if div8 {
            r.read(5);
        } else {
            r.u32(SIZE);
        }
    }
    // ImageMetadata up to the extra channels.
    assert!(!r.bool(), "metadata is not all_default");
    let extra_fields = r.bool();
    if extra_fields {
        r.read(3);
        assert!(
            !r.bool() && !r.bool() && !r.bool(),
            "no size, preview, animation"
        );
    }
    assert!(!r.bool(), "integer samples");
    r.u32([(8, 0), (10, 0), (12, 0), (1, 6)]);
    r.bool(); // modular_16bit_buffers
    let extra = r.u32([(0, 0), (1, 0), (2, 4), (1, 12)]);
    assert_eq!(extra, 1, "one extra channel");
    let alpha_default = r.pos;
    assert!(r.bool(), "the alpha channel uses the default bundle");
    let after_alpha = r.pos;
    let xyb = r.bool();
    assert!(xyb, "lossy output is XYB");
    // ColourEncoding.
    if !r.bool() {
        assert!(!r.bool(), "no ICC profile");
        let space = r.enumeration();
        if space != 3 {
            assert_ne!(r.enumeration(), 2, "no custom white point");
        }
        if space != 1 && space != 3 {
            assert_ne!(r.enumeration(), 2, "no custom primaries");
        }
        if r.bool() {
            r.read(24);
        } else {
            r.enumeration();
        }
        r.enumeration();
    }
    if extra_fields && !r.bool() {
        r.read(16 + 16 + 1 + 16);
    }
    assert_eq!(r.read(2), 0, "no extensions");
    assert!(r.bool(), "default transform data");
    let header_end = r.pos;

    let bit = |i: usize| (data[i / 8] >> (i % 8)) & 1;
    let mut bits: Vec<u8> = (0..alpha_default).map(bit).collect();
    // Explicit ExtraChannelInfo: not all_default, type Alpha (enum 0),
    // 8-bit integer samples, dim_shift 0, empty name, alpha_associated.
    bits.extend([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    bits.extend((after_alpha..header_end).map(bit));
    while !bits.len().is_multiple_of(8) {
        bits.push(0);
    }
    let mut out: Vec<u8> = bits
        .as_chunks::<8>()
        .0
        .iter()
        .map(|byte| byte.iter().rev().fold(0, |acc, &b| (acc << 1) | b))
        .collect();
    out.extend_from_slice(&data[header_end.div_ceil(8)..]);
    out
}
