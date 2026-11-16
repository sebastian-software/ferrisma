#![cfg(all(feature = "jpeg-read", feature = "webp-read", feature = "webp-write"))]

use ferrisma::pipeline::{
    Crop, ImageFormat, InputProfilePolicy, InspectionLimits, OutputFormat, ProcessingErrorKind,
    ResizeFit, ResizeOptions, TransformOptions, WebpOptions, inspect, transcode, transcode_with,
};
use ferrisma::pipeline::{OutputGeometry, OutputSettings, WebpLossless};

struct RowCapture;
struct CapturedRows(Vec<u8>);

impl ferrisma::pipeline::EncoderBackend for RowCapture {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "test-row-capture"
    }
    fn begin(
        &self,
        _: &ferrisma::pipeline::EncodeRequest,
    ) -> Result<Box<dyn ferrisma::pipeline::RowEncoder + '_>, ferrisma::pipeline::EncodeError> {
        Ok(Box::new(CapturedRows(Vec::new())))
    }
}

impl ferrisma::pipeline::RowEncoder for CapturedRows {
    fn write_row(&mut self, _: u32, row: &[u8]) -> Result<(), ferrisma::pipeline::EncodeError> {
        self.0.extend_from_slice(row);
        Ok(())
    }
    fn finish(
        self: Box<Self>,
    ) -> Result<ferrisma::pipeline::EncodedImage, ferrisma::pipeline::EncodeError> {
        Ok(ferrisma::pipeline::EncodedImage::new(
            OutputFormat::WebP,
            self.0,
            0,
            false,
        ))
    }
}

fn options() -> TransformOptions {
    TransformOptions::new(OutputGeometry::Exact {
        width: 1,
        height: 1,
    })
    .with_output(OutputSettings::WebP(WebpOptions::Lossless(
        WebpLossless::new(4),
    )))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
    .with_transform_threads(1)
}

#[test]
fn upright_inspection_and_crop_before_cover_match_exact_pipeline() {
    let jpeg = include_bytes!("fixtures/orientation/jpeg-6.jpg");
    let meta = inspect(jpeg, ImageFormat::Jpeg, &InspectionLimits::default()).unwrap();
    assert_eq!((meta.stored_width, meta.stored_height), (24, 16));
    assert_eq!((meta.width, meta.height), (16, 24));
    assert!(!meta.has_alpha);
    let crop = Crop::new(2, 5, 12, 15);
    let actual = transcode(
        jpeg,
        ImageFormat::Jpeg,
        &(options().with_crop(Some(crop))).with_geometry(OutputGeometry::Resize(
            ResizeOptions::default()
                .with_width(6)
                .with_height(6)
                .with_fit(ResizeFit::Cover)
                .with_without_enlargement(false),
        )),
    )
    .unwrap();
    let exact = transcode(
        jpeg,
        ImageFormat::Jpeg,
        &options()
            .with_geometry(OutputGeometry::Exact {
                width: 6,
                height: 6,
            })
            .with_crop(Some(Crop::new(2, 6, 12, 12))),
    )
    .unwrap();
    assert_eq!((actual.width, actual.height), (6, 6));
    assert_eq!((exact.width, exact.height), (6, 6));
    assert_eq!(actual.bytes, exact.bytes);
    assert!(actual.report.orientation_buffer_bytes > 0);
    let webp = inspect(
        &actual.bytes,
        ImageFormat::WebP,
        &InspectionLimits::default(),
    )
    .unwrap();
    assert_eq!((webp.width, webp.height), (6, 6));
    assert!(webp.rendered_icc.is_some());
    let again = transcode(
        &actual.bytes,
        ImageFormat::WebP,
        &(options().with_profile_policy(InputProfilePolicy::UseRenderedIcc)).with_geometry(
            OutputGeometry::Resize(ResizeOptions::default().with_width(3)),
        ),
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
    let resize = ResizeOptions::default()
        .with_width(4)
        .with_height(3)
        .with_fit(ResizeFit::Fill)
        .with_without_enlargement(false);
    let legacy = transcode(
        webp,
        ImageFormat::WebP,
        &options().with_geometry(OutputGeometry::Exact {
            width: 4,
            height: 3,
        }),
    )
    .unwrap();
    let actual = transcode(
        webp,
        ImageFormat::WebP,
        &options().with_geometry(OutputGeometry::Resize(resize)),
    )
    .unwrap();
    assert_eq!((actual.width, actual.height), (4, 3));
    assert_eq!(actual.bytes, legacy.bytes);
}

#[test]
fn resized_backend_receives_the_same_rows_as_manual_cover_without_enlargement() {
    use ferrisma::pipeline::transcode_with;
    let jpeg = include_bytes!("fixtures/orientation/jpeg-6.jpg");
    let actual = transcode_with(
        jpeg,
        ImageFormat::Jpeg,
        &options().with_geometry(OutputGeometry::Resize(
            ResizeOptions::default()
                .with_width(100)
                .with_height(100)
                .with_fit(ResizeFit::Cover)
                .with_without_enlargement(true),
        )),
        &RowCapture,
    )
    .unwrap();
    let expected = transcode_with(
        jpeg,
        ImageFormat::Jpeg,
        &options()
            .with_geometry(OutputGeometry::Exact {
                width: 16,
                height: 16,
            })
            .with_crop(Some(Crop::new(0, 4, 16, 16))),
        &RowCapture,
    )
    .unwrap();
    assert_eq!((actual.width, actual.height), (16, 16));
    assert_eq!(actual.bytes, expected.bytes);
}

/// Until 2026-11-15 the resize entry points ignored `output_width` and
/// `output_height` (audit row Q1); from the owner decision of that day
/// (ADR-0042) until 2026-11-16 they refused a value other than the default
/// (this test was `resize_entry_points_refuse_an_exact_output_size`). Since
/// 2026-11-16 the size is one setting, `OutputGeometry`, exact or resize:
/// there is no second size to ignore or refuse, and the entry points are
/// `transcode` and `transcode_with` alone.
#[test]
fn the_geometry_is_one_setting() {
    let jpeg = include_bytes!("fixtures/sample.jpg");
    let resize = OutputGeometry::Resize(ResizeOptions::default().with_width(4));
    let options = options()
        .with_geometry(OutputGeometry::Exact {
            width: 4,
            height: 3,
        })
        .with_geometry(resize.clone());
    assert_eq!(options.geometry(), &resize);
    let output = transcode(jpeg, ImageFormat::Jpeg, &options).unwrap();
    let with = transcode_with(jpeg, ImageFormat::Jpeg, &options, &RowCapture).unwrap();
    assert_eq!((output.width, output.height), (with.width, with.height));
    assert_eq!(output.width, 4);
}

#[test]
fn inspection_enforces_byte_and_pixel_limits_and_does_not_assume_color() {
    let jpeg = include_bytes!("fixtures/sample.jpg");
    let info = inspect(jpeg, ImageFormat::Jpeg, &InspectionLimits::default()).unwrap();
    assert!(info.rendered_icc.is_none());
    assert!(info.rendered_cicp.is_none());
    // Header inspection is not whole-image validation: the missing EOI is
    // detected only when the processing path actually decodes the pixels.
    let truncated = &jpeg[..jpeg.len() - 2];
    assert!(inspect(truncated, ImageFormat::Jpeg, &InspectionLimits::default()).is_ok());
    assert_eq!(
        transcode(
            truncated,
            ImageFormat::Jpeg,
            &options().with_geometry(OutputGeometry::Resize(
                ResizeOptions::default().with_width(2)
            ))
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::MalformedInput
    );
    assert_eq!(
        inspect(
            jpeg,
            ImageFormat::Jpeg,
            &InspectionLimits::default().with_max_input_bytes(jpeg.len() - 1)
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::InputLimitExceeded
    );
    assert_eq!(
        inspect(
            jpeg,
            ImageFormat::Jpeg,
            &InspectionLimits::default().with_max_input_pixels(1)
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::InputLimitExceeded
    );
    assert_eq!(
        inspect(
            jpeg,
            ImageFormat::Jpeg,
            &InspectionLimits::default().with_max_input_bytes(0)
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::InvalidOptions
    );
    assert_eq!(
        transcode(
            jpeg,
            ImageFormat::Jpeg,
            &(options().with_profile_policy(InputProfilePolicy::Strict)).with_geometry(
                OutputGeometry::Resize(ResizeOptions::default().with_width(3))
            )
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::UnsupportedColor
    );
    assert_eq!(
        transcode(
            &[],
            ImageFormat::Jpeg,
            &options().with_geometry(OutputGeometry::Resize(ResizeOptions::default()))
        )
        .unwrap_err()
        .kind(),
        ProcessingErrorKind::InvalidOptions
    );
}

#[cfg(feature = "avif-write")]
#[test]
fn jpeg_thumbnail_reaches_avif() {
    let image = transcode(
        include_bytes!("fixtures/orientation/jpeg-6.jpg"),
        ImageFormat::Jpeg,
        &TransformOptions::new(OutputGeometry::Exact {
            width: 1,
            height: 1,
        })
        .with_output(OutputSettings::Avif(
            ferrisma::pipeline::AvifOptions::default().with_speed(10),
        ))
        .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
        .with_geometry(OutputGeometry::Resize(
            ResizeOptions::default().with_width(8),
        )),
    )
    .unwrap();
    assert_eq!((image.width, image.height), (8, 12));
    assert_eq!(&image.bytes[4..8], b"ftyp");
}
