use crate::pipeline::{
    EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat, InputProfilePolicy,
    OutputFormat, RowEncoder, TransformOptions, transcode_with,
};
use crate::pipeline::{OutputGeometry, OutputSettings};

struct RawBackend;
struct MislabelBackend;
struct Rows {
    bytes: Vec<u8>,
}

impl EncoderBackend for RawBackend {
    fn format(&self) -> OutputFormat {
        OutputFormat::Avif
    }
    fn name(&self) -> &'static str {
        "raw-test"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        Ok(Box::new(Rows { bytes: Vec::new() }))
    }
}

impl EncoderBackend for MislabelBackend {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "mislabel-test"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        Ok(Box::new(Rows { bytes: Vec::new() }))
    }
}

impl RowEncoder for Rows {
    fn write_row(&mut self, _: u32, rgba: &[u8]) -> Result<(), EncodeError> {
        self.bytes.extend_from_slice(rgba);
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(OutputFormat::Avif, self.bytes, 0, false))
    }
}

#[test]
fn backend_runs_without_avif_write() {
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 4,
        height: 4,
    })
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
    let result = transcode_with(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        &options,
        &RawBackend,
    )
    .unwrap();
    assert_eq!(result.bytes.len(), 4 * 4 * 4);
    assert_eq!(result.report.encoder_backend, "raw-test");
    let mismatch = transcode_with(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        &options
            .clone()
            .with_output(OutputSettings::default_for(OutputFormat::WebP)),
        &RawBackend,
    );
    assert_eq!(
        mismatch.unwrap_err().kind(),
        crate::pipeline::ProcessingErrorKind::InvalidOptions
    );
    let mislabel = transcode_with(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        &options
            .clone()
            .with_output(OutputSettings::default_for(OutputFormat::WebP)),
        &MislabelBackend,
    );
    assert_eq!(
        mislabel.unwrap_err().kind(),
        crate::pipeline::ProcessingErrorKind::EncodeFailed
    );
    let capped = transcode_with(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        &options.clone().with_max_output_bytes(63),
        &RawBackend,
    );
    // `OutputLimitExceeded` since 2026-11-16 (`ResourceExhausted` before).
    assert_eq!(
        capped.unwrap_err().kind(),
        crate::pipeline::ProcessingErrorKind::OutputLimitExceeded
    );
}

struct BudgetedBackend {
    began: std::sync::atomic::AtomicBool,
}

impl EncoderBackend for BudgetedBackend {
    fn format(&self) -> OutputFormat {
        OutputFormat::Avif
    }

    fn name(&self) -> &'static str {
        "budget-test"
    }

    fn required_frame_bytes(&self, _: &EncodeRequest) -> Result<usize, EncodeError> {
        Ok(4096)
    }

    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        self.began.store(true, std::sync::atomic::Ordering::Relaxed);
        Ok(Box::new(Rows { bytes: Vec::new() }))
    }
}

#[test]
fn backend_frame_reservation_precedes_allocation_and_counts_toward_peak() {
    let backend = BudgetedBackend {
        began: std::sync::atomic::AtomicBool::new(false),
    };
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 4,
        height: 4,
    })
    .with_transform_threads(1)
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
    .with_max_managed_bytes(4095);
    let error = transcode_with(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        &options,
        &backend,
    )
    .unwrap_err();
    assert_eq!(
        error.kind(),
        crate::pipeline::ProcessingErrorKind::ResourceExhausted
    );
    assert!(!backend.began.load(std::sync::atomic::Ordering::Relaxed));

    let output = transcode_with(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        &options.clone().with_max_managed_bytes(64 * 1024),
        &backend,
    )
    .unwrap();
    assert!(backend.began.load(std::sync::atomic::Ordering::Relaxed));
    assert!(output.report.peak_managed_bytes > 4096);
    assert_eq!(output.bytes.len(), 4 * 4 * 4);
}

#[test]
fn oversized_per_image_threads_fail_before_opening_input() {
    use crate::pipeline::{ProcessingErrorKind, ResizeOptions, transcode};
    for options in [
        TransformOptions::new(OutputGeometry::Exact {
            width: 1,
            height: 1,
        })
        .with_transform_threads(257),
        TransformOptions::new(OutputGeometry::Exact {
            width: 1,
            height: 1,
        })
        .with_encoder_threads(usize::MAX),
    ] {
        assert_eq!(
            transcode(&[], ImageFormat::Jpeg, &options)
                .unwrap_err()
                .kind(),
            ProcessingErrorKind::InvalidOptions
        );
        let resize = ResizeOptions::default().with_width(4);
        assert_eq!(
            transcode(
                &[],
                ImageFormat::Jpeg,
                &options.with_geometry(OutputGeometry::Resize(resize))
            )
            .unwrap_err()
            .kind(),
            ProcessingErrorKind::InvalidOptions
        );
    }
}

/// Without `avif-write` built-in AVIF output reports its missing feature.
/// (Until 2026-11-15 this selected the libavif encoder explicitly.)
#[test]
fn built_in_avif_output_requires_the_feature() {
    use crate::pipeline::{InputProfilePolicy, ProcessingErrorKind, transcode};
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 2,
        height: 2,
    })
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
    assert_eq!(
        transcode(
            include_bytes!("../tests/fixtures/sample.jpg"),
            ImageFormat::Jpeg,
            &options
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::FeatureDisabled
    );
}

#[test]
fn batch_spawn_failure_does_not_start_images_and_reports_each_item() {
    use crate::pipeline::{BatchItem, Engine, ProcessingErrorKind};
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CountingBackend(AtomicUsize);
    impl EncoderBackend for CountingBackend {
        fn format(&self) -> OutputFormat {
            OutputFormat::Avif
        }
        fn name(&self) -> &'static str {
            "counting"
        }
        fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
            self.0.fetch_add(1, Ordering::Relaxed);
            Ok(Box::new(Rows { bytes: Vec::new() }))
        }
    }
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 4,
        height: 4,
    })
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
    let item = BatchItem::new(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        options,
    );
    let items = [item.clone(), item.clone(), item];
    let backend = CountingBackend(AtomicUsize::new(0));
    for successful in [0, 1, 2] {
        let _failure = crate::thread_spawn::fail_after(successful);
        let results = Engine::new(3).transcode_batch_with(&items, &backend);
        assert_eq!(results.len(), items.len());
        assert!(results.into_iter().all(|result| matches!(result, Err(error) if error.kind() == ProcessingErrorKind::ResourceExhausted)));
        assert_eq!(backend.0.load(Ordering::Relaxed), 0);
    }
    assert!(
        Engine::new(3)
            .transcode_batch_with(&items, &backend)
            .into_iter()
            .all(|result| result.is_ok())
    );
    assert_eq!(backend.0.load(Ordering::Relaxed), items.len());
}

/// Until 2026-11-16 a backend panic in one batch item re-panicked the
/// scope after the join and every result was lost (this test asserted the
/// propagation). Since then each item has its own panic boundary
/// (ADR-0040 amendment of 2026-11-16): the item fails with `Internal`.
#[test]
fn batch_returns_a_backend_panic_as_internal() {
    use crate::pipeline::{BatchItem, Engine, ProcessingErrorKind};

    struct Panics;
    impl EncoderBackend for Panics {
        fn format(&self) -> OutputFormat {
            OutputFormat::Avif
        }
        fn name(&self) -> &'static str {
            "panic-test"
        }
        fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
            panic!("intentional backend panic")
        }
    }
    let item = BatchItem::new(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        TransformOptions::new(OutputGeometry::Exact {
            width: 1,
            height: 1,
        })
        .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing),
    );
    let results = Engine::new(2).transcode_batch_with(&[item], &Panics);
    assert_eq!(results.len(), 1);
    let error = results.into_iter().next().unwrap().unwrap_err();
    assert_eq!(error.kind(), ProcessingErrorKind::Internal);
    assert!(
        error.to_string().contains("intentional backend panic"),
        "{error}"
    );
}
