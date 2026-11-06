#![cfg(all(feature = "jpeg-read", feature = "avif-write"))]
use crate::pipeline::{
    ImageFormat, InputProfilePolicy, ProcessingErrorKind, ResizeFilter, TransformOptions, transcode,
};

#[test]
fn real_jpeg_reaches_valid_avif_bytes_through_m1_graph() {
    let encoded = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions {
        output_width: 3,
        output_height: 3,
        filter: ResizeFilter::Area,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        quality: 85.0,
        speed: 9,
        max_managed_bytes: 1024 * 1024,
        ..TransformOptions::default()
    };
    let image = transcode(encoded, ImageFormat::Jpeg, options).unwrap();
    assert!(image.avif_bytes.len() > 40);
    assert_eq!(&image.avif_bytes[4..8], b"ftyp");
    assert_eq!(image.report.decoder_requests, 1);
    assert_eq!(image.report.decoder_full_frame_buffers, 1);
    assert_eq!(image.report.engine_full_frame_intermediates, 0);
    // rav1e's 10-bit 4:4:4 input frame, including its padding, is charged.
    assert!(image.report.encoder_output_frame_bytes >= 3 * 3 * 3 * 2);
    assert!(image.report.peak_managed_bytes >= image.report.encoder_output_frame_bytes);
    // Stage timers are observations: the whole run covers every stage, and
    // encoding a real frame takes measurable time even on a tiny image.
    let timings = image.report.timings;
    assert!(timings.total >= timings.open + timings.decode + timings.transform + timings.encode);
    assert!(timings.encode > std::time::Duration::ZERO);
    assert_eq!(
        image.report.working_precision,
        crate::pipeline::WorkingPrecision::F32
    );
    let reference = transcode(
        encoded,
        ImageFormat::Jpeg,
        TransformOptions {
            precision: crate::pipeline::WorkingPrecision::F64Reference,
            ..options
        },
    )
    .unwrap();
    assert_eq!(
        reference.report.working_precision,
        crate::pipeline::WorkingPrecision::F64Reference
    );
    assert_eq!(&reference.avif_bytes[4..8], b"ftyp");
}

#[test]
fn missing_color_information_requires_an_explicit_override() {
    let encoded = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions {
        output_width: 2,
        output_height: 2,
        profile_policy: InputProfilePolicy::Strict,
        ..TransformOptions::default()
    };
    assert_eq!(
        transcode(encoded, ImageFormat::Jpeg, options)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::UnsupportedColor
    );
}

#[cfg(feature = "avif-read")]
#[test]
fn produced_avif_is_accepted_by_an_independent_decoder() {
    use crate::codecs::avif::AvifSource;
    use crate::decoder::DecodeSource;

    let options = TransformOptions {
        output_width: 4,
        output_height: 3,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        speed: 9,
        ..TransformOptions::default()
    };
    let encoded = transcode(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        options,
    )
    .unwrap();
    let decoded = AvifSource::new(&encoded.avif_bytes).unwrap();
    assert_eq!(
        decoded.info().size,
        crate::geometry::Size::new(4, 3).unwrap()
    );
    assert_eq!(decoded.info().layout.channels(), 3);
}

#[cfg(all(feature = "jpeg-xl-read", feature = "icc", feature = "avif-write"))]
#[test]
fn jpeg_xl_icc_can_be_explicitly_transformed_and_encoded() {
    let opts = TransformOptions {
        output_width: 3,
        output_height: 2,
        profile_policy: InputProfilePolicy::UseRenderedIcc,
        speed: 9,
        max_managed_bytes: 1024 * 1024,
        ..TransformOptions::default()
    };
    let image = transcode(
        include_bytes!("../tests/fixtures/sample.jxl"),
        ImageFormat::JpegXl,
        opts,
    )
    .unwrap();
    assert_eq!(&image.avif_bytes[4..8], b"ftyp");
    assert_eq!(image.report.decoder_requests, 1);
    assert_eq!(image.report.engine_full_frame_intermediates, 0);
}

#[test]
fn compressed_bytes_and_decoded_pixel_caps_reject_before_processing() {
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions {
        output_width: 1,
        output_height: 1,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        speed: 10,
        ..TransformOptions::default()
    };
    assert!(
        transcode(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                max_input_bytes: 1,
                ..options
            },
        )
        .is_err()
    );
    assert!(
        transcode(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                max_input_pixels: 1,
                ..options
            },
        )
        .is_err()
    );
    assert!(
        transcode(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                max_input_pixels: 0,
                ..options
            },
        )
        .is_err()
    );
}

#[cfg(feature = "avif-read")]
#[test]
fn signalled_srgb_avif_transcodes_under_the_strict_profile_policy() {
    let srgb = transcode(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        TransformOptions {
            output_width: 8,
            output_height: 6,
            profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
            speed: 10,
            ..TransformOptions::default()
        },
    )
    .unwrap();
    let again = transcode(
        &srgb.avif_bytes,
        ImageFormat::Avif,
        TransformOptions {
            output_width: 4,
            output_height: 3,
            speed: 10,
            ..TransformOptions::default()
        },
    )
    .unwrap();
    assert_eq!(&again.avif_bytes[4..8], b"ftyp");
}

#[test]
fn exif_rotated_jpeg_is_cropped_in_display_coordinates() {
    let rotated = include_bytes!("../tests/fixtures/orientation/jpeg-6.jpg");
    let options = TransformOptions {
        output_width: 4,
        output_height: 6,
        // The stored image is 24×16; EXIF 6 displays it as 16×24.
        crop: Some(crate::pipeline::Crop {
            x: 0,
            y: 4,
            width: 16,
            height: 20,
        }),
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        speed: 10,
        ..TransformOptions::default()
    };
    let output = transcode(rotated, ImageFormat::Jpeg, options).unwrap();
    assert_eq!(&output.avif_bytes[4..8], b"ftyp");
    assert_eq!(output.report.orientation_buffer_bytes, 16 * 20 * 3);
    // The same crop extends past the stored height if orientation is ignored.
    let upright = TransformOptions {
        crop: Some(crate::pipeline::Crop {
            x: 0,
            y: 4,
            width: 16,
            height: 20,
        }),
        ..options
    };
    assert!(
        transcode(
            include_bytes!("../tests/fixtures/orientation/jpeg-1.jpg"),
            ImageFormat::Jpeg,
            upright,
        )
        .is_err()
    );
}

#[test]
fn encoder_threads_trade_determinism_for_latency_explicitly() {
    let encoded = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions {
        output_width: 64,
        output_height: 48,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        speed: 10,
        ..TransformOptions::default()
    };
    // One thread (the default): byte-identical output on every run.
    let first = transcode(encoded, ImageFormat::Jpeg, options).unwrap();
    let second = transcode(encoded, ImageFormat::Jpeg, options).unwrap();
    assert_eq!(first.avif_bytes, second.avif_bytes);
    assert_eq!(first.report.encoder_threads, 1);
    // 0 resolves to the available cores and still yields a valid file.
    let all = transcode(
        encoded,
        ImageFormat::Jpeg,
        TransformOptions {
            encoder_threads: 0,
            ..options
        },
    )
    .unwrap();
    assert!(all.report.encoder_threads >= 1);
    assert_eq!(&all.avif_bytes[4..8], b"ftyp");
    assert_eq!(
        transcode(
            encoded,
            ImageFormat::Jpeg,
            TransformOptions {
                encoder_threads: 100_000,
                ..options
            },
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::InvalidOptions
    );
}

#[test]
fn engine_batches_match_single_image_results_in_order() {
    use crate::pipeline::{BatchItem, Engine};
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions {
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        speed: 10,
        transform_threads: 1,
        encoder_threads: 1,
        ..TransformOptions::default()
    };
    let sizes = [(3, 3), (8, 6), (5, 9), (1, 1), (12, 4), (7, 7)];
    let items: Vec<BatchItem<'_>> = sizes
        .iter()
        .map(|&(width, height)| BatchItem {
            encoded: jpeg,
            format: ImageFormat::Jpeg,
            options: TransformOptions {
                output_width: width,
                output_height: height,
                ..options
            },
        })
        .chain(std::iter::once(BatchItem {
            encoded: &jpeg[..10],
            format: ImageFormat::Jpeg,
            options,
        }))
        .collect();
    let engine = Engine::new(3);
    assert_eq!(engine.threads(), 3);
    let results = engine.transcode_batch(&items);
    assert_eq!(results.len(), items.len());
    for (item, result) in items.iter().zip(&results).take(sizes.len()) {
        let single = transcode(item.encoded, item.format, item.options).unwrap();
        let batched = result.as_ref().unwrap();
        assert_eq!(single.avif_bytes, batched.avif_bytes);
        assert_eq!(batched.report.transform_threads, 1);
        assert_eq!(batched.report.encoder_threads, 1);
    }
    // A truncated image fails on its own without affecting the others.
    assert_eq!(
        results.last().unwrap().as_ref().unwrap_err().kind(),
        ProcessingErrorKind::MalformedInput
    );
    assert!(Engine::new(0).threads() >= 1);
    assert!(engine.transcode_batch(&[]).is_empty());
}
