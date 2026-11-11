#![cfg(all(feature = "jpeg-read", feature = "webp-read", feature = "webp-write"))]

use ferrixel::pipeline::{
    Crop, ImageFormat, InputProfilePolicy, InspectionLimits, OutputFormat, ProcessingErrorKind,
    ResizeFit, ResizeOptions, TransformOptions, WebpOptions, inspect, transcode, transcode_resized,
};

struct RowCapture;
struct CapturedRows(Vec<u8>);

impl ferrixel::pipeline::EncoderBackend for RowCapture {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "test-row-capture"
    }
    fn begin(
        &self,
        _: &ferrixel::pipeline::EncodeRequest,
    ) -> Result<Box<dyn ferrixel::pipeline::RowEncoder + '_>, ferrixel::pipeline::EncodeError> {
        Ok(Box::new(CapturedRows(Vec::new())))
    }
}

impl ferrixel::pipeline::RowEncoder for CapturedRows {
    fn write_row(&mut self, _: u32, row: &[u8]) -> Result<(), ferrixel::pipeline::EncodeError> {
        self.0.extend_from_slice(row);
        Ok(())
    }
    fn finish(
        self: Box<Self>,
    ) -> Result<ferrixel::pipeline::EncodedImage, ferrixel::pipeline::EncodeError> {
        Ok(ferrixel::pipeline::EncodedImage::new(
            OutputFormat::WebP,
            self.0,
            0,
            false,
        ))
    }
}

fn options() -> TransformOptions {
    TransformOptions {
        output_format: OutputFormat::WebP,
        webp: WebpOptions::Lossless { level: 4 },
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        transform_threads: 1,
        ..TransformOptions::default()
    }
}

#[test]
fn upright_inspection_and_crop_before_cover_match_exact_pipeline() {
    let jpeg = include_bytes!("fixtures/orientation/jpeg-6.jpg");
    let meta = inspect(jpeg, ImageFormat::Jpeg, InspectionLimits::default()).unwrap();
    assert_eq!((meta.stored_width, meta.stored_height), (24, 16));
    assert_eq!((meta.width, meta.height), (16, 24));
    assert!(!meta.has_alpha);
    let crop = Crop {
        x: 2,
        y: 5,
        width: 12,
        height: 15,
    };
    let actual = transcode_resized(
        jpeg,
        ImageFormat::Jpeg,
        TransformOptions {
            crop: Some(crop),
            ..options()
        },
        ResizeOptions {
            width: Some(6),
            height: Some(6),
            fit: ResizeFit::Cover,
            without_enlargement: false,
        },
    )
    .unwrap();
    let exact = transcode(
        jpeg,
        ImageFormat::Jpeg,
        TransformOptions {
            crop: Some(Crop {
                x: 2,
                y: 6,
                width: 12,
                height: 12,
            }),
            output_width: 6,
            output_height: 6,
            ..options()
        },
    )
    .unwrap();
    assert_eq!((actual.width, actual.height), (6, 6));
    assert_eq!((exact.width, exact.height), (6, 6));
    assert_eq!(actual.bytes, exact.bytes);
    assert!(actual.report.orientation_buffer_bytes > 0);
    let webp = inspect(
        &actual.bytes,
        ImageFormat::WebP,
        InspectionLimits::default(),
    )
    .unwrap();
    assert_eq!((webp.width, webp.height), (6, 6));
    assert!(webp.rendered_icc.is_some());
    let again = transcode_resized(
        &actual.bytes,
        ImageFormat::WebP,
        TransformOptions {
            profile_policy: InputProfilePolicy::UseRenderedIcc,
            ..options()
        },
        ResizeOptions {
            width: Some(3),
            height: None,
            ..ResizeOptions::default()
        },
    );
    #[cfg(feature = "icc")]
    {
        let again = again.unwrap();
        assert_eq!((again.width, again.height), (3, 3));
    }
    #[cfg(not(feature = "icc"))]
    assert_eq!(
        again.unwrap_err().kind(),
        ProcessingErrorKind::FeatureDisabled
    );
}

#[test]
fn webp_input_resizes_and_fill_matches_legacy_exact_output() {
    let webp = include_bytes!("fixtures/webp/reference-lossy.webp");
    let resize = ResizeOptions {
        width: Some(4),
        height: Some(3),
        fit: ResizeFit::Fill,
        without_enlargement: false,
    };
    let legacy = transcode(
        webp,
        ImageFormat::WebP,
        TransformOptions {
            output_width: 4,
            output_height: 3,
            ..options()
        },
    )
    .unwrap();
    let actual = transcode_resized(webp, ImageFormat::WebP, options(), resize).unwrap();
    assert_eq!((actual.width, actual.height), (4, 3));
    assert_eq!(actual.bytes, legacy.bytes);
}

#[test]
fn resized_backend_receives_the_same_rows_as_manual_cover_without_enlargement() {
    use ferrixel::pipeline::{transcode_resized_with, transcode_with};
    let jpeg = include_bytes!("fixtures/orientation/jpeg-6.jpg");
    let actual = transcode_resized_with(
        jpeg,
        ImageFormat::Jpeg,
        options(),
        ResizeOptions {
            width: Some(100),
            height: Some(100),
            fit: ResizeFit::Cover,
            without_enlargement: true,
        },
        &RowCapture,
    )
    .unwrap();
    let expected = transcode_with(
        jpeg,
        ImageFormat::Jpeg,
        TransformOptions {
            crop: Some(Crop {
                x: 0,
                y: 4,
                width: 16,
                height: 16,
            }),
            output_width: 16,
            output_height: 16,
            ..options()
        },
        &RowCapture,
    )
    .unwrap();
    assert_eq!((actual.width, actual.height), (16, 16));
    assert_eq!(actual.bytes, expected.bytes);
}

#[test]
fn inspection_enforces_byte_and_pixel_limits_and_does_not_assume_color() {
    let jpeg = include_bytes!("fixtures/sample.jpg");
    let info = inspect(jpeg, ImageFormat::Jpeg, InspectionLimits::default()).unwrap();
    assert!(info.rendered_icc.is_none());
    assert!(info.rendered_cicp.is_none());
    // Header inspection is not whole-image validation: the missing EOI is
    // detected only when the processing path actually decodes the pixels.
    let truncated = &jpeg[..jpeg.len() - 2];
    assert!(inspect(truncated, ImageFormat::Jpeg, InspectionLimits::default()).is_ok());
    assert_eq!(
        transcode_resized(
            truncated,
            ImageFormat::Jpeg,
            options(),
            ResizeOptions {
                width: Some(2),
                ..ResizeOptions::default()
            }
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::MalformedInput
    );
    assert_eq!(
        inspect(
            jpeg,
            ImageFormat::Jpeg,
            InspectionLimits {
                max_input_bytes: jpeg.len() - 1,
                ..InspectionLimits::default()
            }
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::InputLimitExceeded
    );
    assert_eq!(
        inspect(
            jpeg,
            ImageFormat::Jpeg,
            InspectionLimits {
                max_input_pixels: 1,
                ..InspectionLimits::default()
            }
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::InputLimitExceeded
    );
    assert_eq!(
        inspect(
            jpeg,
            ImageFormat::Jpeg,
            InspectionLimits {
                max_input_bytes: 0,
                ..InspectionLimits::default()
            }
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::InvalidOptions
    );
    assert_eq!(
        transcode_resized(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                profile_policy: InputProfilePolicy::Strict,
                ..options()
            },
            ResizeOptions {
                width: Some(3),
                ..ResizeOptions::default()
            }
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::UnsupportedColor
    );
    assert_eq!(
        transcode_resized(&[], ImageFormat::Jpeg, options(), ResizeOptions::default())
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::InvalidOptions
    );
}

#[cfg(feature = "avif-write")]
#[test]
fn jpeg_thumbnail_reaches_avif() {
    let image = transcode_resized(
        include_bytes!("fixtures/orientation/jpeg-6.jpg"),
        ImageFormat::Jpeg,
        TransformOptions {
            output_format: OutputFormat::Avif,
            profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
            speed: 10,
            ..TransformOptions::default()
        },
        ResizeOptions {
            width: Some(8),
            height: None,
            ..ResizeOptions::default()
        },
    )
    .unwrap();
    assert_eq!((image.width, image.height), (8, 12));
    assert_eq!(&image.bytes[4..8], b"ftyp");
}
