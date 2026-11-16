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
    // The AVIF writer's known RGBA and YUV frames are charged.
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
    assert_eq!(again.report.decoder_backend, "libavif/aom");
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
