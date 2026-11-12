//! A real photograph through the rendered-ICC policy against LittleCMS 2.19
//! (issue #17). See `tests/fixtures/icc/README.md` for provenance.
//!
//! - The libavif test photograph `paris_exif_xmp_icc.jpg` carries a compact
//!   Google sRGB IEC61966-2.1 ICC v4 profile. The engine decodes it with the
//!   workspace JPEG decoder, crops 96 x 64 at (150, 100) and converts with
//!   moxcms; LittleCMS converts the same crop as decoded by ImageMagick's
//!   libjpeg-turbo.
//! - The same decoded crop, tagged with `DisplayP3-v4.icc` and delivered as
//!   lossless WebP, gives real photographic content a wide-gamut meaning,
//!   including colors outside sRGB.

#![cfg(all(feature = "jpeg-read", feature = "webp-read", feature = "icc"))]
#![allow(
    clippy::chunks_exact_to_as_chunks,
    clippy::cast_possible_truncation,
    clippy::doc_markdown,
    clippy::trivially_copy_pass_by_ref
)]

#[path = "support/resize_oracle.rs"]
#[allow(
    clippy::cast_sign_loss,
    clippy::doc_markdown,
    clippy::many_single_char_names
)]
mod resize_oracle;

use ferrisma::pipeline::{
    Crop, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat,
    InputProfilePolicy, OutputFormat, ResizeFilter, RowEncoder, TransformOptions, transcode_with,
};
use std::sync::Mutex;
use webp::api::encode::{PixelLayout, PixelRows, encode_lossless};

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

const WIDTH: u32 = 96;
const HEIGHT: u32 = 64;

fn run(bytes: &[u8], format: ImageFormat, crop: Option<Crop>) -> Vec<u8> {
    let capture = Capture(Mutex::new(Vec::new()));
    transcode_with(
        bytes,
        format,
        TransformOptions {
            output_format: OutputFormat::WebP,
            output_width: WIDTH,
            output_height: HEIGHT,
            crop,
            filter: ResizeFilter::Area,
            profile_policy: InputProfilePolicy::UseRenderedIcc,
            transform_threads: 1,
            ..TransformOptions::default()
        },
        &capture,
    )
    .unwrap();
    capture.0.into_inner().unwrap()
}

fn littlecms(name: &str) -> Vec<f64> {
    let path = format!(
        "{}/tests/fixtures/icc/{name}.f32",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read(&path)
        .unwrap()
        .chunks_exact(4)
        .map(|b| f64::from(f32::from_le_bytes([b[0], b[1], b[2], b[3]])))
        .collect()
}

/// Bound in linear light per sample: ADR-0023's moxcms versus LittleCMS
/// bound for matrix-shaper profiles, `8e-4` (observed `6.54e-4` on the
/// committed profile samples). The identity geometry adds no filter gain.
const BOUND: f64 = 8e-4;

/// Every engine code must lie between the oracle's encodings of
/// `v - BOUND` and `v + BOUND`; returns how many oracle samples are outside
/// `[0, 1]` (out of the sRGB gamut).
fn assert_matches(actual: &[u8], expected: &[f64], label: &str) -> usize {
    assert_eq!(actual.len() / 4, expected.len() / 3, "{label}");
    let mut outside = 0;
    for (px, want) in actual.chunks_exact(4).zip(expected.chunks_exact(3)) {
        assert_eq!(px[3], 255);
        for c in 0..3 {
            outside += usize::from(!(0.0..=1.0).contains(&want[c]));
            let lo = resize_oracle::encode8(want[c] - BOUND);
            let hi = resize_oracle::encode8(want[c] + BOUND);
            assert!(
                (lo..=hi).contains(&px[c]),
                "{label}: code {} outside {lo}..={hi} (LittleCMS {})",
                px[c],
                want[c]
            );
        }
    }
    outside
}

#[test]
fn real_srgb_icc_photograph_matches_littlecms() {
    let jpeg =
        include_bytes!("../codecs/libavif-rs/upstream/libavif/tests/data/paris_exif_xmp_icc.jpg");
    let actual = run(
        jpeg,
        ImageFormat::Jpeg,
        Some(Crop {
            x: 150,
            y: 100,
            width: WIDTH,
            height: HEIGHT,
        }),
    );
    assert_matches(&actual, &littlecms("paris-crop-srgb-icc"), "paris sRGB ICC");
}

#[test]
fn photograph_as_display_p3_matches_littlecms_out_of_gamut() {
    let rgb = include_bytes!("fixtures/icc/paris-crop.rgb");
    let simple = encode_lossless(PixelRows {
        layout: PixelLayout::Rgb,
        data: rgb,
        width: WIDTH,
        height: HEIGHT,
        stride: WIDTH as usize * 3,
    })
    .unwrap();
    let icc = include_bytes!("fixtures/icc/DisplayP3-v4.icc");
    let actual = run(&with_icc(&simple, icc), ImageFormat::WebP, None);
    let outside = assert_matches(&actual, &littlecms("paris-crop-as-p3"), "paris as P3");
    println!("{outside} LittleCMS samples outside [0, 1]");
    assert!(
        outside > 0,
        "the crop must exercise out-of-sRGB-gamut colors"
    );
}

fn with_icc(simple: &[u8], icc: &[u8]) -> Vec<u8> {
    fn chunk(tag: &[u8; 4], payload: &[u8]) -> Vec<u8> {
        let mut out = tag.to_vec();
        out.extend_from_slice(&u32::try_from(payload.len()).unwrap().to_le_bytes());
        out.extend_from_slice(payload);
        if payload.len() % 2 == 1 {
            out.push(0);
        }
        out
    }
    let mut vp8x = vec![0x20, 0, 0, 0];
    vp8x.extend_from_slice(&(WIDTH - 1).to_le_bytes()[..3]);
    vp8x.extend_from_slice(&(HEIGHT - 1).to_le_bytes()[..3]);
    let mut body = chunk(b"VP8X", &vp8x);
    body.extend(chunk(b"ICCP", icc));
    body.extend_from_slice(&simple[12..]);
    let mut out = b"RIFF".to_vec();
    out.extend_from_slice(&u32::try_from(body.len() + 4).unwrap().to_le_bytes());
    out.extend_from_slice(b"WEBP");
    out.extend(body);
    out
}
