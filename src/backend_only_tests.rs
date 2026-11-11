use crate::pipeline::{
    EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat, InputProfilePolicy,
    OutputFormat, RowEncoder, TransformOptions, transcode_with,
};

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
    let options = TransformOptions {
        output_width: 4,
        output_height: 4,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        ..TransformOptions::default()
    };
    let result = transcode_with(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        options,
        &RawBackend,
    )
    .unwrap();
    assert_eq!(result.bytes.len(), 4 * 4 * 4);
    assert_eq!(result.report.encoder_backend, "raw-test");
    let mismatch = transcode_with(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        TransformOptions {
            output_format: OutputFormat::WebP,
            ..options
        },
        &RawBackend,
    );
    assert_eq!(
        mismatch.unwrap_err().kind(),
        crate::pipeline::ProcessingErrorKind::InvalidOptions
    );
    let mislabel = transcode_with(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        TransformOptions {
            output_format: OutputFormat::WebP,
            ..options
        },
        &MislabelBackend,
    );
    assert_eq!(
        mislabel.unwrap_err().kind(),
        crate::pipeline::ProcessingErrorKind::EncodeFailed
    );
    let capped = transcode_with(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        TransformOptions {
            max_output_bytes: 63,
            ..options
        },
        &RawBackend,
    );
    assert_eq!(
        capped.unwrap_err().kind(),
        crate::pipeline::ProcessingErrorKind::ResourceExhausted
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
    let options = TransformOptions {
        output_width: 4,
        output_height: 4,
        transform_threads: 1,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        max_managed_bytes: 4095,
        ..TransformOptions::default()
    };
    let error = transcode_with(
        include_bytes!("../tests/fixtures/sample.jpg"),
        ImageFormat::Jpeg,
        options,
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
        TransformOptions {
            max_managed_bytes: 64 * 1024,
            ..options
        },
        &backend,
    )
    .unwrap();
    assert!(backend.began.load(std::sync::atomic::Ordering::Relaxed));
    assert!(output.report.peak_managed_bytes > 4096);
    assert_eq!(output.bytes.len(), 4 * 4 * 4);
}

#[test]
fn oversized_per_image_threads_fail_before_opening_input() {
    use crate::pipeline::{ProcessingErrorKind, ResizeOptions, transcode, transcode_resized};
    for options in [
        TransformOptions {
            transform_threads: 257,
            ..TransformOptions::default()
        },
        TransformOptions {
            encoder_threads: usize::MAX,
            ..TransformOptions::default()
        },
    ] {
        assert_eq!(
            transcode(&[], ImageFormat::Jpeg, options)
                .unwrap_err()
                .kind(),
            ProcessingErrorKind::InvalidOptions
        );
        let resize = ResizeOptions {
            width: Some(4),
            ..ResizeOptions::default()
        };
        assert_eq!(
            transcode_resized(&[], ImageFormat::Jpeg, options, resize)
                .unwrap_err()
                .kind(),
            ProcessingErrorKind::InvalidOptions
        );
    }
}

#[cfg(not(feature = "avif-own-write"))]
#[test]
fn own_avif_selection_requires_the_feature_before_opening_input() {
    use crate::pipeline::{AvifEncoder, ProcessingErrorKind, transcode};
    let options = TransformOptions {
        avif_encoder: AvifEncoder::Libavif { depth: 10 },
        ..TransformOptions::default()
    };
    assert_eq!(
        transcode(&[], ImageFormat::Jpeg, options)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::FeatureDisabled
    );
}
