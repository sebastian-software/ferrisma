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
    assert_eq!(image.report.decoder_full_frame_buffers, 0);
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

/// A backend that records what it is given; its "encoding" is the rows.
mod backend {
    use std::sync::Mutex;

    use crate::pipeline::{
        EncodeError, EncodeErrorKind, EncodeRequest, EncodedImage, EncoderBackend, RowEncoder,
    };

    #[derive(Default)]
    pub struct Recorder {
        pub requests: Mutex<Vec<EncodeRequest>>,
        /// Fail `begin`, a row (after this many rows) or `finish`.
        pub fail_begin: bool,
        pub fail_after_rows: Option<usize>,
        pub fail_finish: bool,
    }

    struct Rows<'a> {
        backend: &'a Recorder,
        request: EncodeRequest,
        next: u32,
        bytes: Vec<u8>,
    }

    impl EncoderBackend for Recorder {
        fn name(&self) -> &'static str {
            "recorder"
        }
        fn begin(&self, request: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
            if self.fail_begin {
                return Err(EncodeError::new(
                    EncodeErrorKind::InvalidOptions,
                    "no thanks",
                ));
            }
            self.requests.lock().unwrap().push(*request);
            Ok(Box::new(Rows {
                backend: self,
                request: *request,
                next: 0,
                bytes: Vec::new(),
            }))
        }
    }

    impl RowEncoder for Rows<'_> {
        fn write_row(&mut self, y: u32, rgba: &[u8]) -> Result<(), EncodeError> {
            assert_eq!(y, self.next, "rows arrive in order");
            assert_eq!(rgba.len(), 4 * self.request.width as usize);
            if self.backend.fail_after_rows == Some(y as usize) {
                return Err(EncodeError::new(EncodeErrorKind::Failed, "row refused"));
            }
            self.next += 1;
            self.bytes.extend_from_slice(rgba);
            Ok(())
        }
        fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
            assert_eq!(
                self.next, self.request.height,
                "finish follows the last row"
            );
            if self.backend.fail_finish {
                return Err(EncodeError::new(EncodeErrorKind::ResourceExhausted, "full"));
            }
            let frame = self.bytes.len();
            Ok(EncodedImage::new(self.bytes, frame, true))
        }
    }
}

#[test]
fn a_backend_receives_the_request_and_every_output_row() {
    use crate::pipeline::{ChromaSubsampling, transcode_with};
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions {
        output_width: 7,
        output_height: 5,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        quality: 42.5,
        // Outside rav1e's range: the backend validates its own scale.
        speed: 0,
        encoder_threads: 3,
        chroma_subsampling: ChromaSubsampling::Yuv420,
        ..TransformOptions::default()
    };
    let recorder = backend::Recorder::default();
    let output = transcode_with(jpeg, ImageFormat::Jpeg, options, &recorder).unwrap();
    let requests = recorder.requests.lock().unwrap().clone();
    assert_eq!(requests.len(), 1);
    let request = requests[0];
    assert_eq!((request.width, request.height), (7, 5));
    assert!(!request.has_alpha);
    assert!((request.quality - 42.5).abs() < f32::EPSILON);
    assert_eq!(request.speed, 0);
    assert_eq!(request.threads, 3);
    assert_eq!(request.chroma, ChromaSubsampling::Yuv420);
    // The "encoding" is the rows: opaque straight RGBA8, 7 x 5 pixels.
    assert_eq!(output.avif_bytes.len(), 7 * 5 * 4);
    assert!(output.avif_bytes.chunks(4).all(|pixel| pixel[3] == 255));
    assert_eq!(output.report.encoder_backend, "recorder");
    assert_eq!(output.report.encoder_output_frame_bytes, 7 * 5 * 4);
    assert!(output.report.opaque_encoder_memory);
    assert_eq!(
        transcode(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                speed: 9,
                ..options
            }
        )
        .unwrap()
        .report
        .encoder_backend,
        "rav1e"
    );
    // The rows do not depend on the transform thread count.
    for threads in [1, 4] {
        let again = transcode_with(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                transform_threads: threads,
                ..options
            },
            &recorder,
        )
        .unwrap();
        assert_eq!(again.avif_bytes, output.avif_bytes);
    }
}

#[test]
fn backend_errors_keep_their_kind_and_message() {
    use crate::pipeline::transcode_with;
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions {
        output_width: 6,
        output_height: 4,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        ..TransformOptions::default()
    };
    let cases = [
        (
            backend::Recorder {
                fail_begin: true,
                ..Default::default()
            },
            ProcessingErrorKind::InvalidOptions,
            "no thanks",
        ),
        (
            backend::Recorder {
                fail_after_rows: Some(2),
                ..Default::default()
            },
            ProcessingErrorKind::EncodeFailed,
            "row refused",
        ),
        (
            backend::Recorder {
                fail_finish: true,
                ..Default::default()
            },
            ProcessingErrorKind::ResourceExhausted,
            "full",
        ),
    ];
    for (recorder, kind, message) in cases {
        for precision in [
            crate::pipeline::WorkingPrecision::F32,
            crate::pipeline::WorkingPrecision::F64Reference,
        ] {
            let error = transcode_with(
                jpeg,
                ImageFormat::Jpeg,
                TransformOptions {
                    precision,
                    ..options
                },
                &recorder,
            )
            .unwrap_err();
            assert_eq!(error.kind(), kind, "{message}");
            assert!(error.to_string().contains(message), "{error}");
        }
    }
    // Input errors are reported as before, without asking the backend.
    let recorder = backend::Recorder::default();
    let error = transcode_with(&jpeg[..10], ImageFormat::Jpeg, options, &recorder).unwrap_err();
    assert_eq!(error.kind(), ProcessingErrorKind::MalformedInput);
    assert!(recorder.requests.lock().unwrap().is_empty());
}

#[test]
fn engine_batches_run_one_backend_encoder_per_image() {
    use crate::pipeline::{BatchItem, Engine};
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let items: Vec<BatchItem<'_>> = [(3, 3), (8, 6), (5, 9), (12, 4)]
        .iter()
        .map(|&(width, height)| BatchItem {
            encoded: jpeg,
            format: ImageFormat::Jpeg,
            options: TransformOptions {
                output_width: width,
                output_height: height,
                profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
                encoder_threads: 0,
                ..TransformOptions::default()
            },
        })
        .collect();
    let recorder = backend::Recorder::default();
    let results = Engine::new(3).transcode_batch_with(&items, &recorder);
    for (item, result) in items.iter().zip(&results) {
        let output = result.as_ref().unwrap();
        assert_eq!(
            output.avif_bytes.len(),
            (item.options.output_width * item.options.output_height * 4) as usize
        );
        assert_eq!(output.report.encoder_backend, "recorder");
    }
    let requests = recorder.requests.lock().unwrap();
    assert_eq!(requests.len(), items.len());
    // The engine runs every image with one encoder thread.
    assert!(requests.iter().all(|request| request.threads == 1));
}
