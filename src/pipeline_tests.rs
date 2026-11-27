#![cfg(all(feature = "jpeg-read", feature = "avif-write"))]
use crate::pipeline::Threads;
use crate::pipeline::{AvifOptions, OutputGeometry, OutputSettings};
use crate::pipeline::{
    ImageFormat, InputProfilePolicy, ProcessingErrorKind, ResizeFilter, TransformOptions, transcode,
};

#[test]
fn real_jpeg_reaches_valid_bytes_through_m1_graph() {
    let encoded = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 3,
        height: 3,
    })
    .with_output(OutputSettings::Avif(
        AvifOptions::default().with_quality(85).with_speed(9),
    ))
    .with_filter(ResizeFilter::Area)
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
    .with_max_managed_bytes(1024 * 1024);
    let image = transcode(encoded, ImageFormat::Jpeg, &options).unwrap();
    assert!(image.bytes.len() > 40);
    assert_eq!(&image.bytes[4..8], b"ftyp");
    assert_eq!(image.report.decoder_requests, 1);
    assert_eq!(image.report.decoder_full_frame_buffers, 0);
    assert_eq!(image.report.engine_full_frame_intermediates, 0);
    // The AVIF writer's YUV planes (8-bit 4:4:4 here) and streaming RGB strip are charged.
    assert!(image.report.encoder_output_frame_bytes >= 3 * 3 * 3);
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
        &options
            .clone()
            .with_precision(crate::pipeline::WorkingPrecision::F64Reference),
    )
    .unwrap();
    assert_eq!(
        reference.report.working_precision,
        crate::pipeline::WorkingPrecision::F64Reference
    );
    assert_eq!(&reference.bytes[4..8], b"ftyp");
}

#[test]
fn attribute_stage_timings_moves_sink_time_from_transform_to_encode() {
    use crate::pipeline::attribute_stage_timings;
    use std::time::Duration;

    let ms = Duration::from_millis;
    assert_eq!(
        attribute_stage_timings(ms(25), ms(18), ms(10)),
        (ms(7), ms(28))
    );
    assert_eq!(
        attribute_stage_timings(ms(25), Duration::ZERO, ms(10)),
        (ms(25), ms(10))
    );
    assert_eq!(
        attribute_stage_timings(ms(25), ms(25), Duration::ZERO),
        (Duration::ZERO, ms(25))
    );
    // Clamped if accumulated per-row sink readings slightly exceed the outer
    // transform window due to clock quantization.
    assert_eq!(
        attribute_stage_timings(ms(10), ms(12), ms(4)),
        (Duration::ZERO, ms(14))
    );
}

#[test]
fn row_encoder_intake_time_is_attributed_to_encode_not_transform() {
    use crate::pipeline::{
        EncodeError, EncodeRequest, EncodedImage, EncoderBackend, OutputFormat, RowEncoder,
        transcode_with,
    };
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{Duration, Instant};

    struct SlowRowIntakeBackend(Arc<AtomicU64>);
    struct SlowRowIntakeEncoder(Arc<AtomicU64>);
    impl EncoderBackend for SlowRowIntakeBackend {
        fn format(&self) -> OutputFormat {
            OutputFormat::Avif
        }
        fn name(&self) -> &'static str {
            "slow-intake"
        }
        fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
            Ok(Box::new(SlowRowIntakeEncoder(Arc::clone(&self.0))))
        }
    }
    impl RowEncoder for SlowRowIntakeEncoder {
        fn write_row(&mut self, y: u32, _: &[u8]) -> Result<(), EncodeError> {
            if y == 0 {
                let started = Instant::now();
                std::thread::sleep(Duration::from_millis(20));
                self.0.store(
                    u64::try_from(started.elapsed().as_nanos()).unwrap_or(u64::MAX),
                    Ordering::Relaxed,
                );
            }
            Ok(())
        }
        fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
            Ok(EncodedImage::new(
                OutputFormat::Avif,
                vec![1, 2, 3],
                0,
                false,
            ))
        }
    }

    let slept_ns = Arc::new(AtomicU64::new(0));
    let backend = SlowRowIntakeBackend(Arc::clone(&slept_ns));
    let encoded = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 3,
        height: 3,
    })
    .with_output(OutputSettings::default_for(OutputFormat::Avif))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE);
    let out = transcode_with(encoded, ImageFormat::Jpeg, &options, &backend).unwrap();
    let slept = Duration::from_nanos(slept_ns.load(Ordering::Relaxed));
    assert!(slept >= Duration::from_millis(15));
    let t = out.report.timings;
    assert!(
        t.encode >= slept,
        "row intake work ({slept:?}) must be counted under encode: {t:?}"
    );
    assert!(t.total >= t.open + t.decode + t.transform + t.encode);
}

#[test]
fn missing_color_information_requires_an_explicit_override() {
    let encoded = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 2,
        height: 2,
    })
    .with_profile_policy(InputProfilePolicy::Strict);
    assert_eq!(
        transcode(encoded, ImageFormat::Jpeg, &options)
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

    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 4,
        height: 3,
    })
    .with_output(OutputSettings::Avif(AvifOptions::default().with_speed(9)))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
    let encoded = transcode(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        &options,
    )
    .unwrap();
    let decoded = AvifSource::new(&encoded.bytes).unwrap();
    assert_eq!(
        decoded.info().size,
        crate::geometry::Size::new(4, 3).unwrap()
    );
    assert_eq!(decoded.info().layout.channels(), 3);
}

/// The only JPEG XL reader (ADR-0032) hands the rendered ICC profile to the
/// explicit ICC stage and reconstructs the full frame for the crop. (Until
/// 2026-11-15 a jxl-oxide twin of this test pinned a codec region request.)
#[cfg(all(feature = "jpeg-xl-read", feature = "icc", feature = "avif-write"))]
#[test]
fn jpeg_xl_icc_is_explicitly_transformed_and_encoded_from_a_full_frame() {
    use crate::pipeline::SourceAccess;
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 3,
        height: 2,
    })
    .with_output(OutputSettings::Avif(AvifOptions::default().with_speed(9)))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_max_managed_bytes(1024 * 1024);
    let image = transcode(
        include_bytes!("../tests/fixtures/sample.jxl"),
        ImageFormat::JpegXl,
        &options,
    )
    .unwrap();
    assert_eq!(&image.bytes[4..8], b"ftyp");
    assert_eq!(image.report.decoder_requests, 1);
    assert_eq!(image.report.engine_full_frame_intermediates, 0);
    assert_eq!(image.report.input_access, SourceAccess::FullFrameFallback);
    assert_eq!(image.report.decoder_full_frame_buffers, 1);
}

#[test]
fn compressed_bytes_and_decoded_pixel_caps_reject_before_processing() {
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 1,
        height: 1,
    })
    .with_output(OutputSettings::Avif(AvifOptions::default().with_speed(10)))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
    assert!(
        transcode(
            jpeg,
            ImageFormat::Jpeg,
            &options.clone().with_max_input_bytes(1)
        )
        .is_err()
    );
    assert!(
        transcode(
            jpeg,
            ImageFormat::Jpeg,
            &options.clone().with_max_input_pixels(1)
        )
        .is_err()
    );
    assert!(
        transcode(
            jpeg,
            ImageFormat::Jpeg,
            &options.clone().with_max_input_pixels(0)
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
        &TransformOptions::new(OutputGeometry::Exact {
            width: 8,
            height: 6,
        })
        .with_output(OutputSettings::Avif(AvifOptions::default().with_speed(10)))
        .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing),
    )
    .unwrap();
    let again = transcode(
        &srgb.bytes,
        ImageFormat::Avif,
        &TransformOptions::new(OutputGeometry::Exact {
            width: 4,
            height: 3,
        })
        .with_output(OutputSettings::Avif(AvifOptions::default().with_speed(10))),
    )
    .unwrap();
    assert_eq!(&again.bytes[4..8], b"ftyp");
    // Issue #212: the first run assumed sRGB, the second used the AVIF's
    // own CICP under the strict policy.
    assert_eq!(
        srgb.report.color_signal,
        crate::pipeline::ColorSignal::AssumedSrgb
    );
    assert_eq!(
        again.report.color_signal,
        crate::pipeline::ColorSignal::Cicp
    );
    // The dav1d port is the default AVIF decoder since 2026-11-22.
    assert_eq!(again.report.decoder_backend, "libavif/dav1d");
}

#[test]
fn exif_rotated_jpeg_is_cropped_in_display_coordinates() {
    let rotated = include_bytes!("../tests/fixtures/orientation/jpeg-6.jpg");
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 4,
        height: 6,
    })
    .with_output(OutputSettings::Avif(AvifOptions::default().with_speed(10)))
    // The stored image is 24×16; EXIF 6 displays it as 16×24.
    .with_crop(Some(crate::pipeline::Crop::new(0, 4, 16, 20)))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
    let output = transcode(rotated, ImageFormat::Jpeg, &options).unwrap();
    assert_eq!(&output.bytes[4..8], b"ftyp");
    assert_eq!(output.report.orientation_buffer_bytes, 16 * 20 * 3);
    // The same crop extends past the stored height if orientation is ignored.
    let upright = options
        .clone()
        .with_crop(Some(crate::pipeline::Crop::new(0, 4, 16, 20)));
    assert!(
        transcode(
            include_bytes!("../tests/fixtures/orientation/jpeg-1.jpg"),
            ImageFormat::Jpeg,
            &upright
        )
        .is_err()
    );
}

#[test]
fn encoder_threads_trade_determinism_for_latency_explicitly() {
    let encoded = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 64,
        height: 48,
    })
    .with_output(OutputSettings::Avif(AvifOptions::default().with_speed(10)))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
    // One thread (the default): byte-identical output on every run.
    let first = transcode(encoded, ImageFormat::Jpeg, &options).unwrap();
    let second = transcode(encoded, ImageFormat::Jpeg, &options).unwrap();
    assert_eq!(first.bytes, second.bytes);
    assert_eq!(first.report.encoder_threads, 1);
    // 0 resolves to the available cores and still yields a valid file.
    let all = transcode(
        encoded,
        ImageFormat::Jpeg,
        &options.clone().with_encoder_threads(Threads::Automatic),
    )
    .unwrap();
    assert!(all.report.encoder_threads >= 1);
    assert_eq!(&all.bytes[4..8], b"ftyp");
    assert_eq!(
        transcode(
            encoded,
            ImageFormat::Jpeg,
            &options.clone().with_encoder_threads(Threads::Count(
                std::num::NonZeroUsize::new(100_000).unwrap()
            ))
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
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 1,
        height: 1,
    })
    .with_output(OutputSettings::Avif(AvifOptions::default().with_speed(10)))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE);
    let sizes = [(3, 3), (8, 6), (5, 9), (1, 1), (12, 4), (7, 7)];
    let items: Vec<BatchItem<'_>> = sizes
        .iter()
        .map(|&(width, height)| {
            BatchItem::new(
                jpeg,
                ImageFormat::Jpeg,
                options
                    .clone()
                    .with_geometry(OutputGeometry::Exact { width, height }),
            )
        })
        .chain(std::iter::once(BatchItem::new(
            &jpeg[..10],
            ImageFormat::Jpeg,
            options.clone(),
        )))
        .collect();
    let engine = Engine::new(Threads::Count(std::num::NonZeroUsize::new(3).unwrap())).unwrap();
    assert_eq!(engine.threads(), 3);
    let results = engine.transcode_batch(&items);
    assert_eq!(results.len(), items.len());
    for (item, result) in items.iter().zip(&results).take(sizes.len()) {
        let single = transcode(item.encoded(), item.format(), &item.options().clone()).unwrap();
        let batched = result.as_ref().unwrap();
        assert_eq!(single.bytes, batched.bytes);
        assert_eq!(batched.report.transform_threads, 1);
        assert_eq!(batched.report.encoder_threads, 1);
    }
    // A truncated image fails on its own without affecting the others.
    assert_eq!(
        results.last().unwrap().as_ref().unwrap_err().kind(),
        ProcessingErrorKind::MalformedInput
    );
    assert!(Engine::new(Threads::Automatic).unwrap().threads() >= 1);
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
        fn format(&self) -> crate::pipeline::OutputFormat {
            crate::pipeline::OutputFormat::Avif
        }
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
            self.requests.lock().unwrap().push(request.clone());
            Ok(Box::new(Rows {
                backend: self,
                request: request.clone(),
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
            Ok(EncodedImage::new(
                crate::pipeline::OutputFormat::Avif,
                self.bytes,
                frame,
                true,
            ))
        }
    }
}

#[test]
fn a_backend_receives_the_request_and_every_output_row() {
    use crate::pipeline::{BitDepth, ChromaSubsampling, transcode_with};
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 7,
        height: 5,
    })
    // Every AVIF setting reaches the backend, which validates them on
    // its own scale (the depth too, since 2026-11-15).
    .with_output(OutputSettings::Avif(
        AvifOptions::default()
            .with_quality(42)
            .with_alpha_quality(17)
            .with_speed(0)
            .with_depth(BitDepth::Exact(8))
            .with_chroma(ChromaSubsampling::Yuv420),
    ))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
    .with_encoder_threads(Threads::Count(std::num::NonZeroUsize::new(3).unwrap()));
    let recorder = backend::Recorder::default();
    let output = transcode_with(jpeg, ImageFormat::Jpeg, &options, &recorder).unwrap();
    let requests = recorder.requests.lock().unwrap().clone();
    assert_eq!(requests.len(), 1);
    let request = &requests[0];
    assert_eq!((request.width, request.height), (7, 5));
    assert!(!request.has_alpha);
    assert_eq!(&request.output, options.output());
    assert_eq!(request.threads, 3);
    // The "encoding" is the rows: opaque straight RGBA8, 7 x 5 pixels.
    assert_eq!(output.bytes.len(), 7 * 5 * 4);
    assert!(output.bytes.chunks(4).all(|pixel| pixel[3] == 255));
    assert_eq!(output.report.encoder_backend, "recorder");
    assert_eq!(output.report.encoder_output_frame_bytes, 7 * 5 * 4);
    assert!(output.report.opaque_encoder_memory);
    // Without a backend, the built-in AVIF writer runs: the workspace
    // libavif writer (rav1e was selectable here until 2026-11-15).
    let default_backend = transcode(
        jpeg,
        ImageFormat::Jpeg,
        &options
            .clone()
            .with_output(OutputSettings::Avif(AvifOptions::default().with_speed(9))),
    )
    .unwrap()
    .report
    .encoder_backend;
    assert_eq!(default_backend, "libavif/aom");
    // The rows do not depend on the transform thread count.
    for threads in [1, 4] {
        let again = transcode_with(
            jpeg,
            ImageFormat::Jpeg,
            &options.clone().with_transform_threads(
                std::num::NonZeroUsize::new(threads).map_or(Threads::Automatic, Threads::Count),
            ),
            &recorder,
        )
        .unwrap();
        assert_eq!(again.bytes, output.bytes);
    }
}

#[test]
fn identity_passthrough_reports_one_transform_thread_through_facade() {
    use crate::pipeline::{KernelLevel, KernelLevelReason, transcode_with};
    let jpeg = include_bytes!("../tests/fixtures/decoder-work/paris-420.jpg");
    let four = Threads::Count(std::num::NonZeroUsize::new(4).unwrap());
    let recorder = backend::Recorder::default();

    let identity = transcode_with(
        jpeg,
        ImageFormat::Jpeg,
        &TransformOptions::new(OutputGeometry::Exact {
            width: 403,
            height: 302,
        })
        .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
        .with_transform_threads(four),
        &recorder,
    )
    .unwrap();
    assert_eq!(identity.report.transform_threads, 1);
    assert_eq!(identity.report.kernel_level, KernelLevel::Scalar);
    assert_eq!(
        identity.report.kernel_level_reason,
        KernelLevelReason::Bypassed
    );

    let resized = transcode_with(
        jpeg,
        ImageFormat::Jpeg,
        &TransformOptions::new(OutputGeometry::Exact {
            width: 200,
            height: 150,
        })
        .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
        .with_transform_threads(four),
        &recorder,
    )
    .unwrap();
    assert_eq!(resized.report.transform_threads, 4);
    assert_ne!(
        resized.report.kernel_level_reason,
        KernelLevelReason::Bypassed
    );
}

#[test]
fn backend_errors_keep_their_kind_and_message() {
    use crate::pipeline::transcode_with;
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 6,
        height: 4,
    })
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
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
                &options.clone().with_precision(precision),
                &recorder,
            )
            .unwrap_err();
            assert_eq!(error.kind(), kind, "{message}");
            assert!(error.to_string().contains(message), "{error}");
        }
    }
    // Input errors are reported as before, without asking the backend.
    let recorder = backend::Recorder::default();
    let error = transcode_with(&jpeg[..10], ImageFormat::Jpeg, &options, &recorder).unwrap_err();
    assert_eq!(error.kind(), ProcessingErrorKind::MalformedInput);
    assert!(recorder.requests.lock().unwrap().is_empty());
}

#[test]
fn engine_batches_run_one_backend_encoder_per_image() {
    use crate::pipeline::{BatchItem, Engine};
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let items: Vec<BatchItem<'_>> = [(3, 3), (8, 6), (5, 9), (12, 4)]
        .iter()
        .map(|&(width, height)| {
            BatchItem::new(
                jpeg,
                ImageFormat::Jpeg,
                TransformOptions::new(OutputGeometry::Exact { width, height })
                    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
                    .with_encoder_threads(Threads::Automatic),
            )
        })
        .collect();
    let recorder = backend::Recorder::default();
    let results = Engine::new(Threads::Count(std::num::NonZeroUsize::new(3).unwrap()))
        .unwrap()
        .transcode_batch_with(&items, &recorder);
    for (item, result) in items.iter().zip(&results) {
        let output = result.as_ref().unwrap();
        let OutputGeometry::Exact { width, height } = *item.options().geometry() else {
            panic!("exact geometry");
        };
        assert_eq!(output.bytes.len(), (width * height * 4) as usize);
        assert_eq!(output.report.encoder_backend, "recorder");
    }
    let requests = recorder.requests.lock().unwrap();
    assert_eq!(requests.len(), items.len());
    // The engine runs every image with one encoder thread.
    assert!(requests.iter().all(|request| request.threads == 1));
}

/// The depth rule for external backends (ADR-0021 amendment of
/// 2026-11-15): a backend that admits deeper rows receives the input's
/// depth as `u16` rows and nothing is reported; a backend with the default
/// `row_depths` keeps receiving 8-bit rows and the reduction is reported.
#[cfg(feature = "avif-read")]
mod deep_rows {
    use std::sync::Mutex;

    use crate::encoder_backend::{
        EncodeError, EncodeRequest, EncodedImage, EncoderBackend, RowEncoder,
    };
    use crate::pipeline::{
        Discarded, ImageFormat, InputProfilePolicy, OutputFormat, OutputGeometry, OutputSettings,
        TransformOptions, transcode_with,
    };

    const ANDROID: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/avif/"
    );

    #[derive(Default)]
    struct Deep {
        depths: &'static [u8],
        requests: Mutex<Vec<EncodeRequest>>,
        rows: Mutex<Vec<u16>>,
    }
    struct Rows<'a>(&'a Deep);

    impl EncoderBackend for Deep {
        fn format(&self) -> OutputFormat {
            OutputFormat::Avif
        }
        fn name(&self) -> &'static str {
            "deep"
        }
        fn row_depths(&self) -> &'static [u8] {
            self.depths
        }
        fn begin(&self, request: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
            self.requests.lock().unwrap().push(request.clone());
            Ok(Box::new(Rows(self)))
        }
    }
    impl RowEncoder for Rows<'_> {
        fn write_row(&mut self, _: u32, rgba: &[u8]) -> Result<(), EncodeError> {
            self.0
                .rows
                .lock()
                .unwrap()
                .extend(rgba.iter().map(|&v| u16::from(v)));
            Ok(())
        }
        fn write_row_u16(&mut self, _: u32, rgba: &[u16]) -> Result<(), EncodeError> {
            self.0.rows.lock().unwrap().extend_from_slice(rgba);
            Ok(())
        }
        fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
            Ok(EncodedImage::new(OutputFormat::Avif, vec![0], 0, true))
        }
    }

    fn run(file: &str, depths: &'static [u8]) -> (EncodeRequest, Vec<u16>, bool) {
        let bytes = std::fs::read(format!("{ANDROID}{file}")).unwrap();
        let backend = Deep {
            depths,
            ..Deep::default()
        };
        let options = TransformOptions::new(OutputGeometry::Exact {
            width: 64,
            height: 64,
        })
        .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
        let output = transcode_with(&bytes, ImageFormat::Avif, &options, &backend).unwrap();
        let request = backend.requests.lock().unwrap()[0].clone();
        let rows = backend.rows.lock().unwrap().clone();
        let reduced = output
            .report
            .discarded
            .iter()
            .any(|class| class == Discarded::PrecisionReduced);
        (request, rows, reduced)
    }

    /// A smooth gray ramp at `depth` with every code along x, as
    /// libavif-ready `u16` rows (gray, so that 4:2:0 loses no chroma and the
    /// round trip measures depth, not subsampling).
    fn ramp(width: u32, height: u32, depth: u8) -> Vec<Vec<u16>> {
        let max = (1_u32 << depth) - 1;
        (0..height)
            .map(|_| {
                (0..width)
                    .flat_map(|x| {
                        let v = x * max / (width - 1);
                        [v, v, v, max].map(|v| u16::try_from(v).unwrap())
                    })
                    .collect()
            })
            .collect()
    }

    fn encode(
        rows: &[Vec<u16>],
        width: u32,
        depth: u8,
        chroma: ferrisma_avif_write::Chroma,
    ) -> Vec<u8> {
        let mut frame = ferrisma_avif_write::Frame::new(ferrisma_avif_write::Config {
            width,
            height: u32::try_from(rows.len()).unwrap(),
            has_alpha: false,
            quality: 100,
            alpha_quality: 100,
            speed: 6,
            threads: 1,
            chroma,
            depth,
            sample_depth: depth,
        })
        .unwrap();
        for (y, row) in rows.iter().enumerate() {
            frame.write_row_u16(u32::try_from(y).unwrap(), row).unwrap();
        }
        frame.finish().unwrap()
    }

    /// The decoded samples of an AVIF at its own depth, through the engine's
    /// reader, at identity geometry.
    fn decode(bytes: &[u8], width: u32, height: u32) -> (EncodeRequest, Vec<u16>, bool) {
        let backend = Deep {
            depths: &[8, 10, 12, 16],
            ..Deep::default()
        };
        let options = TransformOptions::new(OutputGeometry::Exact { width, height })
            .with_profile_policy(InputProfilePolicy::Strict);
        let output = transcode_with(bytes, ImageFormat::Avif, &options, &backend).unwrap();
        let request = backend.requests.lock().unwrap()[0].clone();
        let rows = backend.rows.lock().unwrap().clone();
        let reduced = output
            .report
            .discarded
            .iter()
            .any(|class| class == Discarded::PrecisionReduced);
        (request, rows, reduced)
    }

    /// A 10-bit AVIF in, a 10-bit AVIF out (no resize, same color signal):
    /// nothing is reported, and the decoded output differs from the source
    /// ramp only by the codec (libavif's YUV round trip at quality 100), not
    /// by an 8-bit bottleneck, which would keep at most 256 levels per
    /// channel and an error of up to two 10-bit codes everywhere. The same
    /// at 12 bits, in 4:4:4 and 4:2:0.
    #[test]
    fn a_deep_avif_keeps_its_depth_through_avif_output() {
        use crate::pipeline::{AvifOptions, transcode};
        let (width, height) = (1024_u32, 16_u32);
        for depth in [10_u8, 12] {
            for chroma in [
                ferrisma_avif_write::Chroma::Yuv444,
                ferrisma_avif_write::Chroma::Yuv420,
            ] {
                let source = ramp(width, height, depth);
                let input = encode(&source, width, depth, chroma);
                let transcoded = transcode(
                    &input,
                    ImageFormat::Avif,
                    &TransformOptions::new(OutputGeometry::Exact { width, height })
                        .with_output(OutputSettings::Avif(
                            AvifOptions::default().with_quality(100),
                        ))
                        .with_profile_policy(InputProfilePolicy::Strict),
                )
                .unwrap();
                assert!(
                    !transcoded
                        .report
                        .discarded
                        .iter()
                        .any(|class| class == Discarded::PrecisionReduced),
                    "{depth}-bit {chroma:?}"
                );
                let (request, decoded, reduced) = decode(&transcoded.bytes, width, height);
                assert_eq!(request.depth, depth, "the output is {depth}-bit");
                assert!(!reduced);
                let expected: Vec<u16> = source.concat();
                let mut error_sum = 0_u64;
                let mut worst = 0;
                for (&a, &b) in decoded.iter().zip(&expected) {
                    let error = a.abs_diff(b);
                    worst = worst.max(error);
                    error_sum += u64::from(error);
                }
                #[allow(clippy::cast_precision_loss)]
                let mean = error_sum as f64 / expected.len() as f64;
                let mut red: Vec<u16> = decoded.chunks(4).map(|p| p[0]).collect();
                red.sort_unstable();
                red.dedup();
                eprintln!(
                    "{depth}-bit {chroma:?}: {} red levels, max error {worst}, mean {mean:.4}",
                    red.len()
                );
                // An 8-bit bottleneck keeps at most 256 levels.
                assert!(
                    red.len() > 900,
                    "{depth}-bit {chroma:?}: {} levels",
                    red.len()
                );
                assert!(mean < 1.0, "{depth}-bit {chroma:?}: mean error {mean}");
            }
        }
    }

    #[test]
    fn deep_rows_reach_a_backend_that_admits_them_and_are_reported_otherwise() {
        const ALL: &[u8] = &[8, 10, 12, 16];
        for (file, depth) in [
            ("fox.profile0.10bpc.yuv420.avif", 10_u8),
            ("fox.profile2.12bpc.yuv444.avif", 12),
            ("fox.profile0.8bpc.yuv420.avif", 8),
        ] {
            let (request, rows, reduced) = run(file, ALL);
            assert_eq!(request.depth, depth, "{file}");
            assert!(!reduced, "{file}");
            let max = (1_u32 << depth) - 1;
            assert!(rows.iter().all(|&v| u32::from(v) <= max), "{file}");
            assert!(rows.chunks(4).all(|p| u32::from(p[3]) == max), "{file}");
            let mut levels: Vec<u16> = rows.clone();
            levels.sort_unstable();
            levels.dedup();
            if depth > 8 {
                // Deeper than any 8-bit bottleneck: more than 256 levels.
                assert!(levels.len() > 256, "{file}: {} levels", levels.len());
            }
            // The default backend: 8-bit rows, and the reduction is named.
            let (request, rows, reduced) = run(file, &[8]);
            assert_eq!(request.depth, 8);
            assert!(rows.iter().all(|&v| v <= 255));
            assert_eq!(reduced, depth > 8, "{file}");
            // 16 is admitted but the input is 12 at most: the rule caps at
            // the input, never above it.
            let (request, _, _) = run(file, &[8, 16]);
            assert_eq!(request.depth, 8, "{file}");
        }
    }
}

/// Issue #219: `Engine::transcode_batch` with shared ICC profiles (large RGB
/// above cutoff, tiny RGB below cutoff, and Gray ICC) produces byte-identical
/// output to single-image `transcode` with `Threads::ONE`.
#[cfg(all(feature = "jpeg-read", feature = "webp-write", feature = "icc"))]
#[test]
fn engine_batch_with_shared_icc_matches_single_transcode_byte_for_byte() {
    use crate::pipeline::{
        BatchItem, Engine, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        Threads, TransformOptions, WebpLossy, WebpOptions, transcode,
    };

    let base_large = include_bytes!("../tests/fixtures/decoder-work/paris-420.jpg");
    let p3_icc = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc");
    let inject_icc = |jpeg: &[u8], icc: &[u8]| -> Vec<u8> {
        let mut out = Vec::with_capacity(jpeg.len() + icc.len() + 18);
        out.extend_from_slice(&jpeg[..2]);
        let len = u16::try_from(14 + icc.len() + 2).unwrap();
        out.extend_from_slice(&[0xff, 0xe2]);
        out.extend_from_slice(&len.to_be_bytes());
        out.extend_from_slice(b"ICC_PROFILE\0\x01\x01");
        out.extend_from_slice(icc);
        out.extend_from_slice(&jpeg[2..]);
        out
    };
    let large_p3 = inject_icc(base_large, p3_icc);
    let tiny_p3 = inject_icc(
        include_bytes!("../tests/fixtures/orientation/jpeg-1.jpg"),
        p3_icc,
    );
    let gray = include_bytes!("../tests/fixtures/parity/gray-gray-icc.jpg");

    let opts = TransformOptions::new(OutputGeometry::Exact {
        width: 32,
        height: 24,
    })
    .with_output(OutputSettings::WebP(WebpOptions::Lossy(
        WebpLossy::default().with_method(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE);

    let items = vec![
        BatchItem::new(&large_p3, ImageFormat::Jpeg, opts.clone()),
        BatchItem::new(&large_p3, ImageFormat::Jpeg, opts.clone()),
        BatchItem::new(&tiny_p3, ImageFormat::Jpeg, opts.clone()),
        BatchItem::new(&tiny_p3, ImageFormat::Jpeg, opts.clone()),
        BatchItem::new(gray, ImageFormat::Jpeg, opts.clone()),
        BatchItem::new(gray, ImageFormat::Jpeg, opts.clone()),
    ];
    let engine = Engine::new(Threads::Count(std::num::NonZeroUsize::new(4).unwrap())).unwrap();
    let batch = engine.transcode_batch(&items);
    assert_eq!(batch.len(), items.len());
    for (idx, (item, res)) in items.iter().zip(&batch).enumerate() {
        let got = res.as_ref().expect("batch item succeeded");
        let want = transcode(item.encoded(), item.format(), item.options()).unwrap();
        assert_eq!(got.bytes, want.bytes, "batch item {idx} byte-identity");
    }
}

/// Issue #219: RGB `ChannelTable` selection depends only on full pre-crop
/// source dimensions (preserving crop invariance under `ResizeFit::Cover`),
/// whereas `GrayTable` (bit-identical to `moxcms`) uses the resolved crop hint.
#[cfg(feature = "icc")]
#[test]
fn icc_pixel_hint_uses_resolved_cover_crop() {
    use crate::cms::{GRAY_TABLE_MIN_PIXELS, IccSrgbSource, RGB_TABLE_MIN_PIXELS};
    use crate::color::AlphaMode;
    use crate::decoder::{
        ChannelLayout, DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo,
        SampleType,
    };
    use crate::error::Error;
    use crate::geometry::{Rect, Size};
    use crate::memory::MemoryBudget;
    use crate::pipeline::{ResizeFit, ResizeOptions};

    struct DummySource(DecoderInfo);
    impl DecodeSource for DummySource {
        fn info(&self) -> &DecoderInfo {
            &self.0
        }
        fn read_region(
            &mut self,
            region: Rect,
            _budget: &MemoryBudget,
            _sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
        ) -> Result<DecodeReport, Error> {
            Ok(DecodeReport {
                requested: region,
                access: DecodeAccess::CodecRegionRequest,
                rows: 0,
                output_sample_bytes: 0,
                known_full_frame_buffers: 0,
                peak_engine_managed_bytes: 0,
                opaque_codec_memory_unbounded: false,
                engine_buffer_bytes: 0,
                engine_copied_bytes: 0,
                work: crate::decoder::CodecWork::region_at_least(region),
            })
        }
    }

    let panorama = Size::new(1000, 100).unwrap();
    let full_rect = Rect::new(0, 0, panorama.width, panorama.height).unwrap();
    let cover = ResizeOptions::default()
        .with_width(50)
        .with_height(100)
        .with_fit(ResizeFit::Cover);
    let (resolved_crop, _) = crate::resize_geometry::resolve(full_rect, &cover).unwrap();
    let hint = u64::from(resolved_crop.width) * u64::from(resolved_crop.height);
    assert_eq!(hint, 5_000);
    assert!(hint < RGB_TABLE_MIN_PIXELS);

    // RGB uses full source dimensions (100,000 >= 10,000) so cropping or Cover
    // never alters RGB pixels: `ChannelTable` stays active (`(0, 0)` scratch).
    let p3 = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc").to_vec();
    let mut rgb_source = IccSrgbSource::new_with_cache(
        Box::new(DummySource(DecoderInfo {
            depth: 8,
            size: panorama,
            layout: ChannelLayout::Rgb,
            sample: SampleType::U8,
            access: DecodeAccess::CodecRegionRequest,
            alpha: AlphaMode::Opaque,
            rendered_icc: Some(p3),
            rendered_cicp: None,
        })),
        None,
        hint,
        None,
    )
    .unwrap();
    let rgb_conv = rgb_source.defer_conversion().unwrap();
    assert_eq!(rgb_conv.scratch_channels(), (0, 0));

    // Gray uses the resolved Cover crop hint: 5×100 = 500 < 512 skips GrayTable.
    let narrow_cover = ResizeOptions::default()
        .with_width(5)
        .with_height(100)
        .with_fit(ResizeFit::Cover);
    let (narrow_crop, _) = crate::resize_geometry::resolve(full_rect, &narrow_cover).unwrap();
    let gray_hint = u64::from(narrow_crop.width) * u64::from(narrow_crop.height);
    assert_eq!(gray_hint, 500);
    assert!(gray_hint < GRAY_TABLE_MIN_PIXELS);
    let gray_icc = moxcms::ColorProfile::new_gray_with_gamma(2.2)
        .encode()
        .unwrap();
    let mut gray_source = IccSrgbSource::new_with_cache(
        Box::new(DummySource(DecoderInfo {
            depth: 8,
            size: panorama,
            layout: ChannelLayout::Gray,
            sample: SampleType::U8,
            access: DecodeAccess::CodecRegionRequest,
            alpha: AlphaMode::Opaque,
            rendered_icc: Some(gray_icc),
            rendered_cicp: None,
        })),
        None,
        gray_hint,
        None,
    )
    .unwrap();
    let gray_conv = gray_source.defer_conversion().unwrap();
    assert_eq!(gray_conv.scratch_channels(), (1, 3));
}

/// Regression for PR #297 Round-3 review (Finding #1): a `100×100` lossless
/// Display P3 WebP filled with `[7, 19, 202]` must produce identical pixels
/// in the uncropped `100×100` output and the overlapping `99×100` crop (no
/// resize), both standalone and after warming a batch cache.
#[cfg(all(feature = "webp-read", feature = "webp-write", feature = "icc"))]
#[test]
fn rgb_icc_conversion_is_invariant_across_crop_size_and_cache_warmth() {
    use crate::codecs::webp::WebpSource;
    use crate::decoder::{DecodeSource, DecodedRow};
    use crate::geometry::Rect;
    use crate::memory::MemoryBudget;
    use crate::pipeline::{
        BatchItem, Crop, Engine, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        Threads, TransformOptions, WebpLossless, WebpOptions, transcode,
    };
    use webp::api::encode::{PixelLayout, PixelRows, encode_lossless};

    fn chunk(tag: [u8; 4], payload: &[u8]) -> Vec<u8> {
        let mut out = tag.to_vec();
        out.extend_from_slice(&u32::try_from(payload.len()).unwrap().to_le_bytes());
        out.extend_from_slice(payload);
        if !payload.len().is_multiple_of(2) {
            out.push(0);
        }
        out
    }

    fn decode_rgb(bytes: &[u8]) -> Vec<u8> {
        let mut src = WebpSource::new_bounded(bytes, 100 * 100).unwrap();
        let size = src.info().size;
        let budget = MemoryBudget::new(4 << 20);
        let mut out = Vec::with_capacity(size.width as usize * size.height as usize * 3);
        src.read_region(
            Rect::new(0, 0, size.width, size.height).unwrap(),
            &budget,
            &mut |_, row| {
                let DecodedRow::U8(s) = row else {
                    panic!("expected u8");
                };
                out.extend_from_slice(s);
                Ok(())
            },
        )
        .unwrap();
        out
    }

    let p3_icc = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc");
    let pixels: Vec<u8> = std::iter::repeat_n([7_u8, 19, 202], 100 * 100)
        .flatten()
        .collect();
    let plain = encode_lossless(PixelRows {
        layout: PixelLayout::Rgb,
        data: &pixels,
        width: 100,
        height: 100,
        stride: 100 * 3,
    })
    .unwrap();
    // VP8X flags bit 0x20 = ICC profile; canvas 100×100 stored as width-1, height-1.
    let mut body = chunk(*b"VP8X", &[0x20, 0, 0, 0, 99, 0, 0, 99, 0, 0]);
    body.extend(chunk(*b"ICCP", p3_icc));
    body.extend_from_slice(&plain[12..]);
    let mut webp = b"RIFF".to_vec();
    webp.extend_from_slice(&u32::try_from(body.len() + 4).unwrap().to_le_bytes());
    webp.extend_from_slice(b"WEBP");
    webp.extend(body);

    let full_opts = TransformOptions::new(OutputGeometry::Exact {
        width: 100,
        height: 100,
    })
    .with_output(OutputSettings::WebP(WebpOptions::Lossless(
        WebpLossless::new(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE);
    let crop_opts = TransformOptions::new(OutputGeometry::Exact {
        width: 99,
        height: 100,
    })
    .with_crop(Some(Crop::new(0, 0, 99, 100)))
    .with_output(OutputSettings::WebP(WebpOptions::Lossless(
        WebpLossless::new(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE);

    let full_out = transcode(&webp, ImageFormat::WebP, &full_opts).unwrap();
    let crop_standalone = transcode(&webp, ImageFormat::WebP, &crop_opts).unwrap();

    // Also run inside a 1-worker batch where item 0 warms the ICC cache before item 1.
    let engine = Engine::new(Threads::ONE).unwrap();
    let batch = engine.transcode_batch(&[
        BatchItem::new(&webp, ImageFormat::WebP, full_opts),
        BatchItem::new(&webp, ImageFormat::WebP, crop_opts),
    ]);
    let batch_full = batch[0].as_ref().unwrap();
    let batch_crop = batch[1].as_ref().unwrap();
    assert_eq!(batch_full.bytes, full_out.bytes);
    assert_eq!(batch_crop.bytes, crop_standalone.bytes);

    let full_rgb = decode_rgb(&full_out.bytes);
    let crop_rgb = decode_rgb(&crop_standalone.bytes);
    for y in 0..100_usize {
        let full_row = &full_rgb[y * 100 * 3..y * 100 * 3 + 99 * 3];
        let crop_row = &crop_rgb[y * 99 * 3..(y + 1) * 99 * 3];
        assert_eq!(crop_row, full_row, "row {y} must match uncropped pixels");
    }
}

/// Regression for PR #297 Round-3 review (Finding #2): a gray ICC crop below
/// `GRAY_TABLE_MIN_PIXELS` (`65×7 = 455 < 512`) must have identical budget
/// admission standalone (cold miss) and in a batch right after an uncropped
/// item warms the cache.
#[cfg(all(feature = "jpeg-read", feature = "webp-write", feature = "icc"))]
#[test]
fn gray_icc_budget_admission_is_independent_of_batch_cache_warmth() {
    use crate::pipeline::{
        BatchItem, Crop, Engine, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        ProcessingErrorKind, Threads, TransformOptions, WebpLossy, WebpOptions, transcode,
    };

    let gray = include_bytes!("../tests/fixtures/parity/gray-gray-icc.jpg");
    let warm_opts = TransformOptions::new(OutputGeometry::Exact {
        width: 128,
        height: 128,
    })
    .with_output(OutputSettings::WebP(WebpOptions::Lossy(
        WebpLossy::default().with_method(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE)
    .with_max_managed_bytes(16 << 20);

    let crop_opts_with_budget = |budget: usize| {
        warm_opts
            .clone()
            .with_crop(Some(Crop::new(0, 0, 65, 7)))
            .with_max_managed_bytes(budget)
    };

    let baseline = transcode(gray, ImageFormat::Jpeg, &crop_opts_with_budget(16 << 20)).unwrap();
    let exact_peak = baseline.report.peak_managed_bytes;

    // 1 byte below the standalone peak must fail standalone AND in a warm batch.
    let tight_err = transcode(
        gray,
        ImageFormat::Jpeg,
        &crop_opts_with_budget(exact_peak - 1),
    )
    .unwrap_err();
    assert_eq!(tight_err.kind(), ProcessingErrorKind::ResourceExhausted);

    let engine = Engine::new(Threads::ONE).unwrap();
    let warm_batch_fail = engine.transcode_batch(&[
        BatchItem::new(gray, ImageFormat::Jpeg, warm_opts.clone()),
        BatchItem::new(
            gray,
            ImageFormat::Jpeg,
            crop_opts_with_budget(exact_peak - 1),
        ),
    ]);
    let warmed = warm_batch_fail[0].as_ref().unwrap();
    assert!(
        warmed.report.batch_icc_cache_peak_bytes > 0,
        "item 0 must warm the batch ICC cache"
    );
    let batch_err = warm_batch_fail[1].as_ref().unwrap_err();
    assert_eq!(batch_err.kind(), ProcessingErrorKind::ResourceExhausted);

    // At exact peak, both standalone and warm batch succeed with identical peak & bytes.
    let warm_batch_ok = engine.transcode_batch(&[
        BatchItem::new(gray, ImageFormat::Jpeg, warm_opts.clone()),
        BatchItem::new(gray, ImageFormat::Jpeg, crop_opts_with_budget(exact_peak)),
    ]);
    let batch_crop = warm_batch_ok[1].as_ref().unwrap();
    assert_eq!(batch_crop.report.peak_managed_bytes, exact_peak);
    assert_eq!(batch_crop.bytes, baseline.bytes);
}

/// Regression for PR #297 Round-5 review (Finding #2): eligible full-image
/// gray ICC (`>= 512` px, using `GrayTable`) accounts for the 4 KiB
/// `GrayTable` in `peak_managed_bytes` and enforces identical admission on
/// standalone, cache-disabled batch, cold batch miss, and warm batch hit.
#[cfg(all(feature = "jpeg-read", feature = "webp-write", feature = "icc"))]
#[test]
fn gray_table_budget_accounting_is_identical_across_cache_modes() {
    use crate::pipeline::{
        BatchItem, Engine, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        ProcessingErrorKind, Threads, TransformOptions, WebpLossy, WebpOptions, transcode,
    };

    let gray = include_bytes!("../tests/fixtures/parity/gray-gray-icc.jpg");
    let warm_opts = TransformOptions::new(OutputGeometry::Exact {
        width: 128,
        height: 128,
    })
    .with_output(OutputSettings::WebP(WebpOptions::Lossy(
        WebpLossy::default().with_method(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE)
    .with_max_managed_bytes(16 << 20);
    let engine = Engine::new(Threads::ONE).unwrap();

    let full_standalone = transcode(gray, ImageFormat::Jpeg, &warm_opts).unwrap();
    let full_peak = full_standalone.report.peak_managed_bytes;
    assert!(full_peak >= crate::cms::GRAY_TABLE_BYTES);
    assert_eq!(
        transcode(
            gray,
            ImageFormat::Jpeg,
            &warm_opts
                .clone()
                .with_max_managed_bytes(crate::cms::GRAY_TABLE_BYTES - 1),
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::ResourceExhausted
    );
    assert_eq!(
        transcode(
            gray,
            ImageFormat::Jpeg,
            &warm_opts.clone().with_max_managed_bytes(full_peak - 1),
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::ResourceExhausted
    );
    let full_batch = engine.transcode_batch(&[
        BatchItem::new(
            gray,
            ImageFormat::Jpeg,
            warm_opts.clone().with_max_managed_bytes(full_peak),
        ),
        BatchItem::new(
            gray,
            ImageFormat::Jpeg,
            warm_opts.clone().with_max_managed_bytes(full_peak),
        ),
        BatchItem::new(
            gray,
            ImageFormat::Jpeg,
            warm_opts.clone().with_max_managed_bytes(full_peak - 1),
        ),
    ]);
    let cold_miss = full_batch[0].as_ref().unwrap();
    let warm_hit = full_batch[1].as_ref().unwrap();
    assert_eq!(cold_miss.report.peak_managed_bytes, full_peak);
    assert_eq!(warm_hit.report.peak_managed_bytes, full_peak);
    assert_eq!(cold_miss.bytes, full_standalone.bytes);
    assert_eq!(warm_hit.bytes, full_standalone.bytes);
    assert_eq!(
        full_batch[2].as_ref().unwrap_err().kind(),
        ProcessingErrorKind::ResourceExhausted
    );
    let disabled_batch = Engine::new(Threads::ONE)
        .unwrap()
        .with_icc_cache_bytes(0)
        .transcode_batch(&[
            BatchItem::new(
                gray,
                ImageFormat::Jpeg,
                warm_opts.clone().with_max_managed_bytes(full_peak),
            ),
            BatchItem::new(
                gray,
                ImageFormat::Jpeg,
                warm_opts.with_max_managed_bytes(full_peak),
            ),
        ]);
    assert_eq!(
        disabled_batch[0]
            .as_ref()
            .unwrap()
            .report
            .peak_managed_bytes,
        full_peak
    );
    assert_eq!(
        disabled_batch[1]
            .as_ref()
            .unwrap()
            .report
            .peak_managed_bytes,
        full_peak
    );
}

/// Regression for PR #297 Round-6 review: an out-of-bounds crop on an eligible
/// gray ICC image under a `< 4 KiB` budget must skip `GrayTable` reservation
/// and fail with `InvalidOptions` (not `ResourceExhausted`), both standalone
/// and inside a warmed batch.
#[cfg(all(feature = "jpeg-read", feature = "webp-write", feature = "icc"))]
#[test]
fn gray_icc_invalid_crop_under_tiny_budget_returns_invalid_options() {
    use crate::pipeline::{
        BatchItem, Crop, Engine, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        ProcessingErrorKind, Threads, TransformOptions, WebpLossy, WebpOptions, transcode,
    };

    let gray = include_bytes!("../tests/fixtures/parity/gray-gray-icc.jpg");
    let warm_opts = TransformOptions::new(OutputGeometry::Exact {
        width: 32,
        height: 24,
    })
    .with_output(OutputSettings::WebP(WebpOptions::Lossy(
        WebpLossy::default().with_method(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE)
    .with_max_managed_bytes(4 << 20);

    let bad_crop_opts = warm_opts
        .clone()
        .with_crop(Some(Crop::new(0, 0, 10_000, 10_000)))
        .with_max_managed_bytes(1024);

    assert_eq!(
        transcode(gray, ImageFormat::Jpeg, &bad_crop_opts)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::InvalidOptions
    );

    let engine = Engine::new(Threads::ONE).unwrap();
    let batch = engine.transcode_batch(&[
        BatchItem::new(gray, ImageFormat::Jpeg, warm_opts),
        BatchItem::new(gray, ImageFormat::Jpeg, bad_crop_opts),
    ]);
    assert!(batch[0].is_ok());
    assert_eq!(
        batch[1].as_ref().unwrap_err().kind(),
        ProcessingErrorKind::InvalidOptions
    );
}

/// Regression for PR #297 Round-7 review (Finding #1): eligible 8-bit RGB
/// `ChannelTable` conversions reserve `RGB_TABLE_BYTES` (12 KiB) against the
/// image's `MemoryBudget` identically across standalone, cache-disabled batch,
/// cold miss, and warm hit; a `4,000`-byte budget fails with
/// `ResourceExhausted`, and an invalid out-of-bounds crop under a `1 KiB`
/// budget skips table setup and returns `InvalidOptions`.
#[cfg(all(feature = "jpeg-read", feature = "webp-write", feature = "icc"))]
#[test]
fn rgb_table_budget_accounting_and_invalid_crop_under_tiny_budget() {
    use crate::pipeline::{
        BatchItem, Crop, Engine, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        ProcessingErrorKind, Threads, TransformOptions, WebpLossy, WebpOptions, transcode,
    };

    let paris = include_bytes!("../tests/fixtures/decoder-work/paris-420.jpg");
    let p3_icc = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc");
    let mut paris_p3 = Vec::with_capacity(paris.len() + p3_icc.len() + 18);
    paris_p3.extend_from_slice(&paris[..2]);
    let len = u16::try_from(14 + p3_icc.len() + 2).unwrap();
    paris_p3.extend_from_slice(&[0xff, 0xe2]);
    paris_p3.extend_from_slice(&len.to_be_bytes());
    paris_p3.extend_from_slice(b"ICC_PROFILE\0\x01\x01");
    paris_p3.extend_from_slice(p3_icc);
    paris_p3.extend_from_slice(&paris[2..]);

    let crop1x1_opts = TransformOptions::new(OutputGeometry::Exact {
        width: 1,
        height: 1,
    })
    .with_crop(Some(Crop::new(0, 0, 1, 1)))
    .with_output(OutputSettings::WebP(WebpOptions::Lossy(
        WebpLossy::default().with_method(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE)
    .with_max_managed_bytes(4 << 20);

    // A 4,000-byte budget cannot admit a 12 KiB `ChannelTable`.
    assert_eq!(
        transcode(
            &paris_p3,
            ImageFormat::Jpeg,
            &crop1x1_opts.clone().with_max_managed_bytes(4_000),
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::ResourceExhausted
    );

    let standalone = transcode(&paris_p3, ImageFormat::Jpeg, &crop1x1_opts).unwrap();
    let peak = standalone.report.peak_managed_bytes;
    assert!(peak >= crate::cms::RGB_TABLE_BYTES);
    assert_eq!(
        transcode(
            &paris_p3,
            ImageFormat::Jpeg,
            &crop1x1_opts.clone().with_max_managed_bytes(peak - 1),
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::ResourceExhausted
    );

    let engine = Engine::new(Threads::ONE).unwrap();
    let batch = engine.transcode_batch(&[
        BatchItem::new(
            &paris_p3,
            ImageFormat::Jpeg,
            crop1x1_opts.clone().with_max_managed_bytes(peak),
        ),
        BatchItem::new(
            &paris_p3,
            ImageFormat::Jpeg,
            crop1x1_opts.clone().with_max_managed_bytes(peak),
        ),
        BatchItem::new(
            &paris_p3,
            ImageFormat::Jpeg,
            crop1x1_opts.clone().with_max_managed_bytes(peak - 1),
        ),
    ]);
    let cold_miss = batch[0].as_ref().unwrap();
    let warm_hit = batch[1].as_ref().unwrap();
    assert_eq!(cold_miss.report.peak_managed_bytes, peak);
    assert_eq!(warm_hit.report.peak_managed_bytes, peak);
    assert_eq!(cold_miss.bytes, standalone.bytes);
    assert_eq!(warm_hit.bytes, standalone.bytes);
    assert_eq!(
        batch[2].as_ref().unwrap_err().kind(),
        ProcessingErrorKind::ResourceExhausted
    );

    let disabled = Engine::new(Threads::ONE)
        .unwrap()
        .with_icc_cache_bytes(0)
        .transcode_batch(&[
            BatchItem::new(
                &paris_p3,
                ImageFormat::Jpeg,
                crop1x1_opts.clone().with_max_managed_bytes(peak),
            ),
            BatchItem::new(
                &paris_p3,
                ImageFormat::Jpeg,
                crop1x1_opts.clone().with_max_managed_bytes(peak),
            ),
        ]);
    assert_eq!(
        disabled[0].as_ref().unwrap().report.peak_managed_bytes,
        peak
    );
    assert_eq!(
        disabled[1].as_ref().unwrap().report.peak_managed_bytes,
        peak
    );
}

/// Regression for PR #297 Round-7 review (Finding #1): an invalid out-of-bounds
/// crop on an eligible RGB ICC image under a `1 KiB` budget skips `ChannelTable`
/// reservation and returns `InvalidOptions` (not `ResourceExhausted`), both
/// standalone and in a warmed batch.
#[cfg(all(feature = "jpeg-read", feature = "webp-write", feature = "icc"))]
#[test]
fn rgb_icc_invalid_crop_under_tiny_budget_returns_invalid_options() {
    use crate::pipeline::{
        BatchItem, Crop, Engine, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        ProcessingErrorKind, Threads, TransformOptions, WebpLossy, WebpOptions, transcode,
    };

    let paris = include_bytes!("../tests/fixtures/decoder-work/paris-420.jpg");
    let p3_icc = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc");
    let mut paris_p3 = Vec::with_capacity(paris.len() + p3_icc.len() + 18);
    paris_p3.extend_from_slice(&paris[..2]);
    let len = u16::try_from(14 + p3_icc.len() + 2).unwrap();
    paris_p3.extend_from_slice(&[0xff, 0xe2]);
    paris_p3.extend_from_slice(&len.to_be_bytes());
    paris_p3.extend_from_slice(b"ICC_PROFILE\0\x01\x01");
    paris_p3.extend_from_slice(p3_icc);
    paris_p3.extend_from_slice(&paris[2..]);

    let warm_opts = TransformOptions::new(OutputGeometry::Exact {
        width: 32,
        height: 24,
    })
    .with_output(OutputSettings::WebP(WebpOptions::Lossy(
        WebpLossy::default().with_method(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE)
    .with_max_managed_bytes(4 << 20);
    let bad_crop_opts = warm_opts
        .clone()
        .with_crop(Some(Crop::new(0, 0, 10_000, 10_000)))
        .with_max_managed_bytes(1024);

    assert_eq!(
        transcode(&paris_p3, ImageFormat::Jpeg, &bad_crop_opts)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::InvalidOptions
    );
    let engine = Engine::new(Threads::ONE).unwrap();
    let bad_batch = engine.transcode_batch(&[
        BatchItem::new(&paris_p3, ImageFormat::Jpeg, warm_opts),
        BatchItem::new(&paris_p3, ImageFormat::Jpeg, bad_crop_opts),
    ]);
    assert!(bad_batch[0].is_ok());
    assert_eq!(
        bad_batch[1].as_ref().unwrap_err().kind(),
        ProcessingErrorKind::InvalidOptions
    );
}

/// Regression for PR #297 Round-8 review (Finding #2): an unsupported output
/// depth (e.g. JPEG XL depth 7) with `max_managed_bytes = 0` on an eligible
/// gray ICC image must skip `GrayTable` reservation and fail with
/// `InvalidOptions` (not `ResourceExhausted`), both standalone and in a
/// warmed batch.
#[cfg(all(
    feature = "jpeg-read",
    feature = "webp-write",
    feature = "jpeg-xl-write",
    feature = "icc"
))]
#[test]
fn icc_unsupported_output_depth_under_zero_budget_returns_invalid_options() {
    use crate::pipeline::{
        BatchItem, BitDepth, Engine, ImageFormat, InputProfilePolicy, JpegXlOptions,
        OutputGeometry, OutputSettings, ProcessingErrorKind, Threads, TransformOptions, WebpLossy,
        WebpOptions, transcode,
    };

    let gray = include_bytes!("../tests/fixtures/parity/gray-gray-icc.jpg");
    let warm_opts = TransformOptions::new(OutputGeometry::Exact {
        width: 32,
        height: 24,
    })
    .with_output(OutputSettings::WebP(WebpOptions::Lossy(
        WebpLossy::default().with_method(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE)
    .with_max_managed_bytes(4 << 20);

    let bad_depth_opts = TransformOptions::new(OutputGeometry::Exact {
        width: 32,
        height: 24,
    })
    .with_output(OutputSettings::JpegXl(
        JpegXlOptions::default().with_depth(BitDepth::Exact(7)),
    ))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE)
    .with_max_managed_bytes(0);

    assert_eq!(
        transcode(gray, ImageFormat::Jpeg, &bad_depth_opts)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::InvalidOptions
    );

    let engine = Engine::new(Threads::ONE).unwrap();
    let batch = engine.transcode_batch(&[
        BatchItem::new(gray, ImageFormat::Jpeg, warm_opts),
        BatchItem::new(gray, ImageFormat::Jpeg, bad_depth_opts),
    ]);
    assert!(batch[0].is_ok());
    assert_eq!(
        batch[1].as_ref().unwrap_err().kind(),
        ProcessingErrorKind::InvalidOptions
    );
}

/// Regression for PR #297 Round-3 review (Finding #3): batch ICC cache peak is
/// observable in `ProcessingReport::batch_icc_cache_peak_bytes`, zero for
/// single-image `transcode`, single-item batches, and `with_icc_cache_bytes(0)`,
/// and strictly bounded by the configured allowance on multi-item batches.
#[cfg(all(feature = "jpeg-read", feature = "webp-write", feature = "icc"))]
#[test]
fn batch_icc_cache_observability_and_explicit_allowance() {
    use crate::pipeline::{
        BatchItem, Engine, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        Threads, TransformOptions, WebpLossy, WebpOptions, transcode,
    };

    let gray = include_bytes!("../tests/fixtures/parity/gray-gray-icc.jpg");
    let opts = TransformOptions::new(OutputGeometry::Exact {
        width: 32,
        height: 24,
    })
    .with_output(OutputSettings::WebP(WebpOptions::Lossy(
        WebpLossy::default().with_method(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE)
    .with_max_managed_bytes(4 << 20);

    // Single-image transcode reports 0.
    let single = transcode(gray, ImageFormat::Jpeg, &opts).unwrap();
    assert_eq!(single.report.batch_icc_cache_peak_bytes, 0);

    // Single-item batch uses 0 cache allowance and reports 0.
    let default_engine = Engine::new(Threads::ONE).unwrap();
    assert_eq!(default_engine.icc_cache_bytes(), None);
    let one_item =
        default_engine.transcode_batch(&[BatchItem::new(gray, ImageFormat::Jpeg, opts.clone())]);
    assert_eq!(
        one_item[0]
            .as_ref()
            .unwrap()
            .report
            .batch_icc_cache_peak_bytes,
        0
    );

    // Multi-item batch with explicit `with_icc_cache_bytes(0)` disables caching.
    let zero_engine = Engine::new(Threads::ONE).unwrap().with_icc_cache_bytes(0);
    assert_eq!(zero_engine.icc_cache_bytes(), Some(0));
    let disabled = zero_engine.transcode_batch(&[
        BatchItem::new(gray, ImageFormat::Jpeg, opts.clone()),
        BatchItem::new(gray, ImageFormat::Jpeg, opts.clone()),
    ]);
    for res in &disabled {
        let out = res.as_ref().unwrap();
        assert_eq!(out.report.batch_icc_cache_peak_bytes, 0);
        assert_eq!(out.bytes, single.bytes);
    }

    // Multi-item batch with explicit allowance reports non-zero peak <= allowance.
    let allowance = 64 * 1024;
    let bounded_engine = Engine::new(Threads::ONE)
        .unwrap()
        .with_icc_cache_bytes(allowance);
    assert_eq!(bounded_engine.icc_cache_bytes(), Some(allowance));
    let cached = bounded_engine.transcode_batch(&[
        BatchItem::new(gray, ImageFormat::Jpeg, opts.clone()),
        BatchItem::new(gray, ImageFormat::Jpeg, opts),
    ]);
    let peak0 = cached[0]
        .as_ref()
        .unwrap()
        .report
        .batch_icc_cache_peak_bytes;
    let peak1 = cached[1]
        .as_ref()
        .unwrap()
        .report
        .batch_icc_cache_peak_bytes;
    assert_eq!(peak0, peak1);
    assert!(peak0 > 0 && peak0 <= allowance, "peak={peak0}");
    assert_eq!(cached[0].as_ref().unwrap().bytes, single.bytes);
    assert_eq!(cached[1].as_ref().unwrap().bytes, single.bytes);
}

/// Regression for PR #297 Round-9 review (Finding #1): a `>= 10,000`-pixel
/// JPEG carrying a valid matrix-shaper RGB profile that maps black to a
/// nonzero value (so it fails the additivity probe) must admit and produce
/// byte-identical output for a `1×1` crop under a `4,000`-byte budget both
/// standalone (cold miss) and after warming the batch negative cache.
#[cfg(all(feature = "jpeg-read", feature = "webp-write", feature = "icc"))]
#[test]
fn non_additive_matrix_shaper_rgb_crop_under_tight_budget_matches_warm_negative_cache() {
    use crate::pipeline::{
        BatchItem, Crop, Engine, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        Threads, TransformOptions, WebpLossless, WebpOptions, transcode,
    };

    let mut profile = moxcms::ColorProfile::new_srgb();
    profile.cicp = None;
    let nonzero_black = moxcms::ToneReprCurve::Lut((0_u16..16).map(|i| 4096 + i * 4095).collect());
    profile.red_trc = Some(nonzero_black.clone());
    profile.green_trc = Some(nonzero_black.clone());
    profile.blue_trc = Some(nonzero_black);
    let non_additive_icc = profile.encode().unwrap();

    let paris = include_bytes!("../tests/fixtures/decoder-work/paris-420.jpg");
    let mut jpeg = Vec::with_capacity(paris.len() + non_additive_icc.len() + 18);
    jpeg.extend_from_slice(&paris[..2]);
    let seg_len = u16::try_from(non_additive_icc.len() + 16).unwrap();
    jpeg.extend_from_slice(&[0xff, 0xe2]);
    jpeg.extend_from_slice(&seg_len.to_be_bytes());
    jpeg.extend_from_slice(b"ICC_PROFILE\0\x01\x01");
    jpeg.extend_from_slice(&non_additive_icc);
    jpeg.extend_from_slice(&paris[2..]);

    let tight_1x1 = TransformOptions::new(OutputGeometry::Exact {
        width: 1,
        height: 1,
    })
    .with_crop(Some(Crop::new(0, 0, 1, 1)))
    .with_output(OutputSettings::WebP(WebpOptions::Lossless(
        WebpLossless::new(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE)
    .with_max_managed_bytes(4_000);

    let warm_full = tight_1x1.clone().with_max_managed_bytes(4 << 20);

    let standalone = transcode(&jpeg, ImageFormat::Jpeg, &tight_1x1)
        .expect("cold miss must probe additivity on stack before reserving 12 KiB");

    let engine = Engine::new(Threads::ONE)
        .unwrap()
        .with_icc_cache_bytes(64 * 1024);
    let batch = engine.transcode_batch(&[
        BatchItem::new(&jpeg, ImageFormat::Jpeg, warm_full),
        BatchItem::new(&jpeg, ImageFormat::Jpeg, tight_1x1),
    ]);
    assert!(batch[0].is_ok());
    let warmed = batch[1]
        .as_ref()
        .expect("warm negative cache hit must also succeed under 4,000 B budget");
    assert_eq!(standalone.bytes, warmed.bytes);
}

/// Regression for PR #297 Round-9 review (Finding #2): ICC lookup-table
/// admission runs only *after* all request, crop, depth, and encoder dimension
/// validations pass, while ICC profile validation still runs *first*.
/// - A `16,384×1` WebP request (exceeding WebP's `16,383` limit) with a
///   table-eligible Display P3 or gray ICC input and a `4,000`-byte budget
///   must return `InvalidOptions` (not `ResourceExhausted`), both standalone
///   and in a warmed batch.
/// - A corrupt ICC profile on the exact same `16,384×1` WebP + `4,000`-byte
///   budget request must still return `UnsupportedColor` first.
#[cfg(all(feature = "jpeg-read", feature = "webp-write", feature = "icc"))]
#[test]
fn webp_overlimit_dimensions_beat_icc_table_budget_while_corrupt_icc_beats_options() {
    use crate::pipeline::{
        BatchItem, Engine, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        ProcessingErrorKind, Threads, TransformOptions, WebpLossy, WebpOptions, transcode,
    };

    let inject_icc = |jpeg: &[u8], icc: &[u8]| -> Vec<u8> {
        let mut out = Vec::with_capacity(jpeg.len() + icc.len() + 18);
        out.extend_from_slice(&jpeg[..2]);
        let seg_len = u16::try_from(icc.len() + 16).unwrap();
        out.extend_from_slice(&[0xff, 0xe2]);
        out.extend_from_slice(&seg_len.to_be_bytes());
        out.extend_from_slice(b"ICC_PROFILE\0\x01\x01");
        out.extend_from_slice(icc);
        out.extend_from_slice(&jpeg[2..]);
        out
    };

    let paris = include_bytes!("../tests/fixtures/decoder-work/paris-420.jpg");
    let p3_icc = include_bytes!("../tests/fixtures/icc/DisplayP3-v4.icc");
    let paris_p3 = inject_icc(paris, p3_icc);
    let paris_corrupt_icc = inject_icc(paris, b"not-a-valid-icc-profile");
    let gray_icc_jpg = include_bytes!("../tests/fixtures/parity/gray-gray-icc.jpg");

    let warm_opts = TransformOptions::new(OutputGeometry::Exact {
        width: 32,
        height: 24,
    })
    .with_output(OutputSettings::WebP(WebpOptions::Lossy(
        WebpLossy::default().with_method(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE)
    .with_max_managed_bytes(4 << 20);

    let overlimit_webp = TransformOptions::new(OutputGeometry::Exact {
        width: 16_384,
        height: 1,
    })
    .with_output(OutputSettings::WebP(WebpOptions::Lossy(
        WebpLossy::default().with_method(0),
    )))
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE)
    .with_max_managed_bytes(4_000);

    // Valid Display P3 & gray ICC inputs return InvalidOptions (WebP > 16,383).
    for input in [&paris_p3[..], &gray_icc_jpg[..]] {
        assert_eq!(
            transcode(input, ImageFormat::Jpeg, &overlimit_webp)
                .unwrap_err()
                .kind(),
            ProcessingErrorKind::InvalidOptions
        );
        let engine = Engine::new(Threads::ONE).unwrap();
        let batch = engine.transcode_batch(&[
            BatchItem::new(input, ImageFormat::Jpeg, warm_opts.clone()),
            BatchItem::new(input, ImageFormat::Jpeg, overlimit_webp.clone()),
        ]);
        assert!(batch[0].is_ok());
        assert_eq!(
            batch[1].as_ref().unwrap_err().kind(),
            ProcessingErrorKind::InvalidOptions
        );
    }

    // Corrupt ICC profile still takes precedence over invalid WebP dimensions.
    assert_eq!(
        transcode(&paris_corrupt_icc, ImageFormat::Jpeg, &overlimit_webp)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::UnsupportedColor
    );
}

#[cfg(feature = "webp-read")]
#[test]
fn parallel_transform_sets_concurrent_row_producer_on_encode_request() {
    use crate::pipeline::{
        AvifOptions, Crop, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        Threads, TransformOptions, transcode_with,
    };

    let pixels = vec![128_u8; 256 * 256 * 4];
    let webp_bytes = webp::api::encode::encode_lossless(webp::api::encode::PixelRows {
        layout: webp::api::encode::PixelLayout::Rgba,
        data: &pixels,
        width: 256,
        height: 256,
        stride: 256 * 4,
    })
    .unwrap();
    // Large EXIF-oriented (orientation 5, transposing, upright 72x96 >= 64 rows) sRGB input.
    let oriented_webp = include_bytes!("../tests/fixtures/decoder-work/paris-o5-lossy.webp");

    let recorder = backend::Recorder::default();
    let base = |width, height, transform_threads| {
        TransformOptions::new(OutputGeometry::Exact { width, height })
            .with_output(OutputSettings::Avif(AvifOptions::default()))
            .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
            .with_transform_threads(Threads::Count(
                std::num::NonZeroUsize::new(transform_threads).unwrap(),
            ))
            .with_encoder_threads(Threads::Count(std::num::NonZeroUsize::new(4).unwrap()))
    };

    // 1. Parallel F32 resize (>= 64 rows, 4 transform threads): sets hint.
    transcode_with(&webp_bytes, ImageFormat::WebP, &base(80, 80, 4), &recorder).unwrap();
    // 2. Single-threaded transform: hint stays false (standalone multi-band conversion).
    transcode_with(&webp_bytes, ImageFormat::WebP, &base(80, 80, 1), &recorder).unwrap();
    // 3. 1:1 sRGB identity copy on non-oriented input (0 workers): hint stays false.
    transcode_with(
        &webp_bytes,
        ImageFormat::WebP,
        &base(256, 256, 4),
        &recorder,
    )
    .unwrap();
    // 4. 1:1 sRGB transcode of a large transposing EXIF-oriented input
    //    (72x96 >= 64 upright rows, 4 transform threads): `OrientedSource::emit_parallel`
    //    holds live gather workers while AVIF flushes strips, so hint MUST be true.
    transcode_with(
        oriented_webp,
        ImageFormat::WebP,
        &base(72, 96, 4),
        &recorder,
    )
    .unwrap();
    // 5. Same EXIF-oriented input cropped below 64 upright rows: orientation
    //    runs serially and identity skips resize, so hint stays false.
    transcode_with(
        oriented_webp,
        ImageFormat::WebP,
        &base(72, 32, 4).with_crop(Some(Crop::new(0, 0, 72, 32))),
        &recorder,
    )
    .unwrap();
    // 6. Non-oriented tall image cropped to 256x32 and resized to 128x32
    //    (< 64 source rows -> serial resize): hint stays false.
    transcode_with(
        &webp_bytes,
        ImageFormat::WebP,
        &base(128, 32, 4).with_crop(Some(Crop::new(0, 0, 256, 32))),
        &recorder,
    )
    .unwrap();
    // 7. WorkingPrecision::F64Reference clamps `transform_threads` to 1 before
    //    `OrientedSource`, so even on an EXIF-oriented >= 64-row input it spawns
    //    0 gather workers and leaves `concurrent_row_producer == false`.
    crate::thread_spawn::reset_spawn_count();
    transcode_with(
        oriented_webp,
        ImageFormat::WebP,
        &base(72, 96, 4).with_precision(crate::pipeline::WorkingPrecision::F64Reference),
        &recorder,
    )
    .unwrap();
    assert_eq!(crate::thread_spawn::spawn_count(), 0);

    let requests = recorder.requests.lock().unwrap().clone();
    assert_eq!(requests.len(), 7);
    assert!(requests[0].concurrent_row_producer);
    assert!(!requests[1].concurrent_row_producer);
    assert!(!requests[2].concurrent_row_producer);
    assert!(requests[3].concurrent_row_producer);
    assert!(!requests[4].concurrent_row_producer);
    assert!(!requests[5].concurrent_row_producer);
    assert!(!requests[6].concurrent_row_producer);
}
