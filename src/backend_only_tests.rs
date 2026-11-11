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
