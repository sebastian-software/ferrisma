//! Bounded deterministic malformed-input smoke corpus. These are not a
//! replacement for libFuzzer/ASan or a full security review.

#[cfg(any(
    feature = "jpeg-read",
    feature = "jpeg-xl-read",
    feature = "jpeg-xl-own-read",
    feature = "avif-read"
))]
use std::panic::{AssertUnwindSafe, catch_unwind};

#[cfg(feature = "jpeg-read")]
#[test]
fn truncated_and_mutated_jpeg_headers_never_panic_in_adapter_setup() {
    use crate::codecs::jpeg::JpegSource;

    let complete = include_bytes!("../tests/fixtures/sample.jpg");
    for length in [0, 1, 2, 4, 8, 16, complete.len() / 2] {
        let data = &complete[..length.min(complete.len())];
        let result = catch_unwind(AssertUnwindSafe(|| JpegSource::new(data)));
        assert!(
            result.is_ok(),
            "JPEG header parser panicked at length {length}"
        );
    }
    let mut invalid = complete.to_vec();
    invalid[0] = 0;
    assert!(catch_unwind(AssertUnwindSafe(|| JpegSource::new(&invalid))).is_ok());
}

#[cfg(feature = "jpeg-xl-own-read")]
#[test]
fn truncated_and_mutated_jxl_headers_never_panic_in_adapter_setup() {
    use crate::codecs::jxl::JxlSource;

    let complete = include_bytes!("../tests/fixtures/sample.jxl");
    for length in [0, 1, 2, 4, 8, 16, complete.len() / 2] {
        let data = &complete[..length.min(complete.len())];
        let result = catch_unwind(AssertUnwindSafe(|| JxlSource::new(data)));
        assert!(
            result.is_ok(),
            "JXL header parser panicked at length {length}"
        );
    }
    let mut invalid = complete.to_vec();
    invalid[0] ^= 0xff;
    assert!(catch_unwind(AssertUnwindSafe(|| JxlSource::new(&invalid))).is_ok());
}

#[cfg(all(feature = "avif-read", feature = "avif-write"))]
#[test]
fn avif_decoder_rejects_truncated_container_without_panicking() {
    use crate::avif_sink::{AvifSink, test_settings};
    use crate::codecs::avif::AvifSource;
    use crate::geometry::Size;
    use crate::memory::MemoryBudget;

    let budget = MemoryBudget::new(8 << 20);
    let mut sink = AvifSink::new(
        Size::new(4, 4).unwrap(),
        false,
        test_settings(80.0, 10),
        &budget,
    )
    .unwrap();
    for row in 0..4 {
        sink.write_row(row, &[40, 60, 100, 255].repeat(4)).unwrap();
    }
    let file = sink.finish(&budget).unwrap().bytes;
    for length in [0, 1, 2, 4, 8, 16, file.len() / 2] {
        let truncated = &file[..length.min(file.len())];
        let result = catch_unwind(AssertUnwindSafe(|| AvifSource::new(truncated)));
        assert!(result.is_ok(), "AVIF adapter panicked at length {length}");
        assert!(
            result.unwrap().is_err(),
            "truncated container unexpectedly accepted"
        );
    }
    assert_eq!(budget.snapshot().live, 0);

    assert!(AvifSource::new_bounded(&file, 1).is_err());
    assert!(AvifSource::new_bounded(&file, 16).is_ok());
}

#[cfg(all(feature = "jpeg-read", feature = "avif-write"))]
#[test]
fn api_rejects_invalid_quality_and_mismatched_input_format() {
    use crate::pipeline::{ImageFormat, InputProfilePolicy, TransformOptions, transcode};
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions {
        output_width: 2,
        output_height: 2,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        ..TransformOptions::default()
    };
    assert!(transcode(jpeg, ImageFormat::Avif, options).is_err());
    assert!(transcode(&[0; 32], ImageFormat::Jpeg, options).is_err());
    for quality in [f32::NAN, f32::INFINITY, 0.0, 101.0] {
        assert!(
            transcode(
                jpeg,
                ImageFormat::Jpeg,
                TransformOptions { quality, ..options },
            )
            .is_err()
        );
    }
    assert!(
        transcode(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                output_width: 0,
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
                max_managed_bytes: 1,
                ..options
            },
        )
        .is_err()
    );
}

#[cfg(all(feature = "jpeg-read", feature = "avif-write"))]
#[test]
fn api_errors_carry_a_matchable_kind() {
    use crate::pipeline::{
        Crop, ImageFormat, InputProfilePolicy, ProcessingErrorKind as Kind, TransformOptions,
        transcode,
    };
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions {
        output_width: 2,
        output_height: 2,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        ..TransformOptions::default()
    };
    let kind =
        |bytes: &[u8], format, options| transcode(bytes, format, options).unwrap_err().kind();
    assert_eq!(
        kind(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                quality: 101.0,
                ..options
            }
        ),
        Kind::InvalidOptions
    );
    assert_eq!(
        kind(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                output_width: 0,
                ..options
            }
        ),
        Kind::InvalidOptions
    );
    let outside = Crop {
        x: 0,
        y: 0,
        width: u32::MAX,
        height: 1,
    };
    assert_eq!(
        kind(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                crop: Some(outside),
                ..options
            }
        ),
        Kind::InvalidOptions
    );
    assert_eq!(
        kind(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                max_input_bytes: 1,
                ..options
            }
        ),
        Kind::InputLimitExceeded
    );
    assert_eq!(
        kind(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                max_input_pixels: 1,
                ..options
            }
        ),
        Kind::InputLimitExceeded
    );
    assert_eq!(
        kind(&[0; 32], ImageFormat::Jpeg, options),
        Kind::MalformedInput
    );
    assert_eq!(
        kind(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                max_managed_bytes: 1,
                ..options
            }
        ),
        Kind::ResourceExhausted
    );
    #[cfg(not(feature = "jpeg-xl-read"))]
    assert_eq!(
        kind(jpeg, ImageFormat::JpegXl, options),
        Kind::FeatureDisabled
    );
}

// Preserve the default decoder contract while the workspace port remains opt-in.
#[cfg(feature = "jpeg-xl-read")]
#[test]
fn interim_truncated_and_mutated_jxl_headers_never_panic_in_adapter_setup() {
    interim_jxl_header_smoke();
}

// Minimized from the default Linux transcode campaign (run 43117625746,
// crash-510ca0e9c63aeae240305a2f57956ff3f3f7db72 without its selector byte):
// an enum colour encoding with an unknown colour space made jxl-color panic
// while synthesizing the rendered ICC profile.
#[cfg(feature = "jpeg-xl-read")]
#[test]
fn jxl_oxide_unknown_colour_space_is_a_controlled_error() {
    use crate::codecs::jxl_oxide::JxlSource;
    use crate::error::Error;

    let hostile = include_bytes!("../tests/fixtures/hostile/jxl-unknown-colour-space.jxl");
    let result = catch_unwind(AssertUnwindSafe(|| JxlSource::new(hostile)));
    let result = result.expect("JPEG XL adapter setup must not panic");
    assert!(matches!(result, Err(Error::UnsupportedColorEncoding)));
}

#[cfg(feature = "jpeg-xl-own-read")]
#[test]
fn workspace_jxl_unknown_colour_space_is_a_controlled_error() {
    use crate::codecs::jxl::JxlSource;

    let hostile = include_bytes!("../tests/fixtures/hostile/jxl-unknown-colour-space.jxl");
    let result = catch_unwind(AssertUnwindSafe(|| JxlSource::new(hostile)));
    assert!(result.expect("workspace JPEG XL setup must not panic").is_err());
}

#[cfg(all(feature = "jpeg-xl-read", feature = "webp-write"))]
#[test]
fn public_transcode_rejects_unknown_jxl_colour_space() {
    use crate::pipeline::{
        ImageFormat, InputProfilePolicy, OutputFormat, ProcessingErrorKind as Kind,
        TransformOptions, transcode,
    };

    let hostile = include_bytes!("../tests/fixtures/hostile/jxl-unknown-colour-space.jxl");
    for profile_policy in [
        InputProfilePolicy::UseRenderedIcc,
        InputProfilePolicy::Strict,
        InputProfilePolicy::AssumeSrgbIfMissing,
    ] {
        let options = TransformOptions {
            output_format: OutputFormat::WebP,
            output_width: 2,
            output_height: 2,
            quality: 75.0,
            speed: 6,
            profile_policy,
            ..TransformOptions::default()
        };
        let result = catch_unwind(AssertUnwindSafe(|| {
            transcode(hostile, ImageFormat::JpegXl, options)
        }))
        .expect("public JPEG XL processing must not panic");
        assert_eq!(result.unwrap_err().kind(), Kind::UnsupportedColor);
    }
}

#[cfg(feature = "jpeg-xl-read")]
fn interim_jxl_header_smoke() {
    use crate::codecs::jxl_oxide::JxlSource;

    let complete = include_bytes!("../tests/fixtures/sample.jxl");
    for length in [0, 1, 2, 4, 8, 16, complete.len() / 2] {
        let data = &complete[..length.min(complete.len())];
        let result = catch_unwind(AssertUnwindSafe(|| JxlSource::new(data)));
        assert!(
            result.is_ok(),
            "JXL header parser panicked at length {length}"
        );
    }
    let mut invalid = complete.to_vec();
    invalid[0] ^= 0xff;
    assert!(catch_unwind(AssertUnwindSafe(|| JxlSource::new(&invalid))).is_ok());
}
