use webp::api::encode::{PixelLayout, PixelRows, encode_lossless};

use crate::pipeline::{
    ImageFormat, InputProfilePolicy, ProcessingErrorKind, TransformOptions, transcode,
};

#[test]
fn still_webp_reaches_avif_and_untagged_color_requires_policy() {
    let mut pixels = Vec::new();
    for y in 0..8_u8 {
        for x in 0..8_u8 {
            pixels.extend_from_slice(&[x * 29, y * 31, x * 7 + y * 11]);
        }
    }
    let webp = encode_lossless(PixelRows {
        layout: PixelLayout::Rgb,
        data: &pixels,
        width: 8,
        height: 8,
        stride: 24,
    })
    .unwrap();
    let options = TransformOptions {
        output_width: 4,
        output_height: 4,
        speed: 9,
        ..TransformOptions::default()
    };
    assert_eq!(
        transcode(&webp, ImageFormat::WebP, options)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::UnsupportedColor,
    );
    let output = transcode(
        &webp,
        ImageFormat::WebP,
        TransformOptions {
            profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
            ..options
        },
    )
    .unwrap();
    assert_eq!(&output.avif_bytes[4..8], b"ftyp");
    assert_eq!(output.report.decoder_requests, 1);
    assert_eq!(output.report.decoder_full_frame_buffers, 1);
    assert!(output.report.opaque_decoder_memory);
}

#[test]
fn straight_alpha_webp_reaches_avif() {
    let mut pixels = Vec::new();
    for y in 0..8_u8 {
        for x in 0..8_u8 {
            pixels.extend_from_slice(&[x * 29, y * 31, x * 7 + y * 11, 32 + x * 20]);
        }
    }
    let webp = encode_lossless(PixelRows {
        layout: PixelLayout::Rgba,
        data: &pixels,
        width: 8,
        height: 8,
        stride: 32,
    })
    .unwrap();
    let output = transcode(
        &webp,
        ImageFormat::WebP,
        TransformOptions {
            output_width: 4,
            output_height: 4,
            profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
            speed: 9,
            ..TransformOptions::default()
        },
    )
    .unwrap();
    assert_eq!(&output.avif_bytes[4..8], b"ftyp");
    assert!(output.avif_bytes.len() > 40);
}
