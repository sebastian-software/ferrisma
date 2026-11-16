//! Bounded deterministic malformed-input smoke corpus. These are not a
//! replacement for libFuzzer/ASan or a full security review.

#[cfg(any(
    feature = "jpeg-read",
    feature = "jpeg-xl-read",
    all(
        feature = "avif-read",
        any(feature = "avif-write", feature = "webp-write")
    )
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

#[cfg(feature = "jpeg-xl-read")]
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
    use crate::codecs::avif::AvifSource;
    use crate::memory::MemoryBudget;
    use crate::pipeline::ChromaSubsampling;

    let budget = MemoryBudget::new(8 << 20);
    let file = crate::own_avif::encode_for_test(4, 4, ChromaSubsampling::Yuv444, 80.0, |_| {
        [40, 60, 100, 255].repeat(4)
    });
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
    use crate::pipeline::{
        AvifOptions, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        TransformOptions, transcode,
    };
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 2,
        height: 2,
    })
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
    assert!(transcode(jpeg, ImageFormat::Avif, &options).is_err());
    assert!(transcode(&[0; 32], ImageFormat::Jpeg, &options).is_err());
    // libavif validates quality on its scale, 0-100; quality 0 is valid
    // and not remapped. (Rav1e's 1-100 was checked here until 2026-11-15;
    // a fractional or non-finite quality cannot be expressed since the
    // scale became libavif's integer one on 2026-11-15.)
    {
        let libavif = options.clone();
        let with = |quality, alpha_quality| {
            libavif.clone().with_output(OutputSettings::Avif(
                AvifOptions::default()
                    .with_quality(quality)
                    .with_alpha_quality(alpha_quality),
            ))
        };
        for (quality, alpha_quality) in [(101, 60), (60, 101), (u8::MAX, u8::MAX)] {
            let result = transcode(jpeg, ImageFormat::Jpeg, &with(quality, alpha_quality));
            assert!(result.is_err(), "libavif quality {quality}/{alpha_quality}");
        }
        assert!(transcode(jpeg, ImageFormat::Jpeg, &with(0, 0)).is_ok());
    }
    assert!(
        transcode(
            jpeg,
            ImageFormat::Jpeg,
            &options.clone().with_geometry(OutputGeometry::Exact {
                width: 0,
                height: 2
            })
        )
        .is_err()
    );
    assert!(
        transcode(
            jpeg,
            ImageFormat::Jpeg,
            &options.clone().with_max_managed_bytes(1)
        )
        .is_err()
    );
}

#[cfg(all(feature = "jpeg-read", feature = "avif-write"))]
#[test]
fn api_errors_carry_a_matchable_kind() {
    use crate::pipeline::{
        AvifOptions, Crop, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings,
        ProcessingErrorKind as Kind, TransformOptions, transcode,
    };
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 2,
        height: 2,
    })
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
    let kind =
        |bytes: &[u8], format, options| transcode(bytes, format, &options).unwrap_err().kind();
    assert_eq!(
        kind(
            jpeg,
            ImageFormat::Jpeg,
            options.clone().with_output(OutputSettings::Avif(
                AvifOptions::default().with_quality(101)
            ))
        ),
        Kind::InvalidOptions
    );
    assert_eq!(
        kind(
            jpeg,
            ImageFormat::Jpeg,
            options.clone().with_geometry(OutputGeometry::Exact {
                width: 0,
                height: 2
            }),
        ),
        Kind::InvalidOptions
    );
    let outside = Crop::new(0, 0, u32::MAX, 1);
    assert_eq!(
        kind(
            jpeg,
            ImageFormat::Jpeg,
            options.clone().with_crop(Some(outside))
        ),
        Kind::InvalidOptions
    );
    assert_eq!(
        kind(
            jpeg,
            ImageFormat::Jpeg,
            options.clone().with_max_input_bytes(1)
        ),
        Kind::InputLimitExceeded
    );
    assert_eq!(
        kind(
            jpeg,
            ImageFormat::Jpeg,
            options.clone().with_max_input_pixels(1)
        ),
        Kind::InputLimitExceeded
    );
    assert_eq!(
        kind(&[0; 32], ImageFormat::Jpeg, options.clone()),
        Kind::MalformedInput
    );
    assert_eq!(
        kind(
            jpeg,
            ImageFormat::Jpeg,
            options.clone().with_max_managed_bytes(1)
        ),
        Kind::ResourceExhausted
    );
}

// Without the JPEG XL reader, JPEG XL input reports its missing feature.
#[cfg(all(
    feature = "jpeg-read",
    feature = "avif-write",
    not(feature = "jpeg-xl-read")
))]
#[test]
fn missing_jpeg_xl_reader_reports_feature_disabled() {
    use crate::pipeline::{
        ImageFormat, InputProfilePolicy, OutputGeometry, ProcessingErrorKind as Kind,
        TransformOptions, transcode,
    };
    let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
    let options = TransformOptions::new(OutputGeometry::Exact {
        width: 2,
        height: 2,
    })
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);
    let error = transcode(jpeg, ImageFormat::JpegXl, &options).unwrap_err();
    assert_eq!(error.kind(), Kind::FeatureDisabled);
}

// Minimized from the default Linux transcode campaign (run 43117625746,
// crash-510ca0e9c63aeae240305a2f57956ff3f3f7db72 without its selector byte):
// an enum colour encoding with an unknown colour space, which made jxl-color
// panic in the former jxl-oxide reader.
#[cfg(feature = "jpeg-xl-read")]
#[test]
fn workspace_jxl_unknown_colour_space_is_a_controlled_error() {
    use crate::codecs::jxl::JxlSource;

    let hostile = include_bytes!("../tests/fixtures/hostile/jxl-unknown-colour-space.jxl");
    let result = catch_unwind(AssertUnwindSafe(|| JxlSource::new(hostile)));
    assert!(
        result
            .expect("workspace JPEG XL setup must not panic")
            .is_err()
    );
}

// Minimized from the hosted fuzz smoke (run 43690971661,
// oom-444b7241bd4fef31ceb81b35807441cf9714188c, 12 bytes after `cargo fuzz
// tmin`, kept without its selector byte): a bare codestream whose single TOC
// entry declares the largest `U32` size, 4,211,712 + 2^30 - 1 bytes.
// jxl-oxide's jxl-frame (the former second reader) reserved that before
// copying the section; libjxl and the workspace reader report the 11-byte
// file as truncated.
#[cfg(feature = "jpeg-xl-read")]
#[test]
fn workspace_jxl_oversized_toc_section_is_a_controlled_error() {
    use crate::codecs::jxl::JxlSource;
    use crate::decoder::DecodeSource;
    use crate::error::Error;
    use crate::geometry::Rect;
    use crate::memory::MemoryBudget;

    // The header is valid, so setup succeeds; like libjxl, the decode finds
    // the section missing.
    let hostile = include_bytes!("../tests/fixtures/hostile/jxl-oversized-toc-section.jxl");
    let result = catch_unwind(AssertUnwindSafe(|| JxlSource::new(hostile)));
    let mut source = result
        .expect("workspace JPEG XL setup must not panic")
        .expect("the header is valid");
    let size = source.info().size;
    let region = Rect::new(0, 0, size.width, size.height).unwrap();
    let result = source.read_region(region, &MemoryBudget::new(1 << 24), &mut |_, _| Ok(()));
    assert_eq!(result.unwrap_err(), Error::DecodeFailed);
}

// The oversized-TOC file through the public API: the only JPEG XL reader
// ends it in a controlled error under every policy and input cap. (Until
// 2026-11-15 this test pinned jxl-oxide's `ResourceExhausted`; the workspace
// reader, like libjxl, reserves nothing for the declared section and finds
// the file truncated.)
#[cfg(all(feature = "jpeg-xl-read", feature = "webp-write"))]
#[test]
fn public_transcode_ends_an_oversized_jxl_toc_section_in_a_controlled_error() {
    use crate::pipeline::{
        ImageFormat, InputProfilePolicy, OutputFormat, OutputGeometry, OutputSettings,
        ProcessingErrorKind as Kind, TransformOptions, transcode,
    };

    let hostile = include_bytes!("../tests/fixtures/hostile/jxl-oversized-toc-section.jxl");
    let options = |max_input_bytes, profile_policy| {
        TransformOptions::new(OutputGeometry::Exact {
            width: 2,
            height: 2,
        })
        .with_output(OutputSettings::default_for(OutputFormat::WebP))
        .with_profile_policy(profile_policy)
        .with_max_input_bytes(max_input_bytes)
    };
    // The fuzz driver's cap and the default cap.
    for max_input_bytes in [
        512 * 1024,
        crate::pipeline::InspectionLimits::default().max_input_bytes(),
    ] {
        // The header's enum sRGB is a typed signal under every policy
        // (owner decision 2026-11-15, #222; before, the strict and
        // assume-if-missing policies refused its rendered ICC with
        // `UnsupportedColor` before decoding), so the decode runs and finds
        // the section missing.
        for (profile_policy, expected) in [
            (InputProfilePolicy::Strict, Kind::MalformedInput),
            (
                InputProfilePolicy::AssumeSrgbIfMissing,
                Kind::MalformedInput,
            ),
            #[cfg(feature = "icc")]
            (InputProfilePolicy::UseRenderedIcc, Kind::MalformedInput),
        ] {
            let result = catch_unwind(AssertUnwindSafe(|| {
                transcode(
                    hostile,
                    ImageFormat::JpegXl,
                    &options(max_input_bytes, profile_policy),
                )
            }))
            .expect("public JPEG XL processing must not panic");
            assert_eq!(
                result.unwrap_err().kind(),
                expected,
                "{profile_policy:?} at {max_input_bytes}"
            );
        }
    }
}

// The workspace reader, like libjxl, rejects the unknown colour space as
// invalid input under every policy. (Until 2026-11-15 this test pinned the
// jxl-oxide reader's `UnsupportedColor`, `UnsupportedColorEncoding`.)
#[cfg(all(feature = "jpeg-xl-read", feature = "webp-write"))]
#[test]
fn public_transcode_rejects_unknown_jxl_colour_space() {
    use crate::pipeline::{
        ImageFormat, InputProfilePolicy, OutputFormat, OutputGeometry, OutputSettings,
        ProcessingErrorKind as Kind, TransformOptions, transcode,
    };

    let hostile = include_bytes!("../tests/fixtures/hostile/jxl-unknown-colour-space.jxl");
    for profile_policy in [
        InputProfilePolicy::UseRenderedIcc,
        InputProfilePolicy::Strict,
        InputProfilePolicy::AssumeSrgbIfMissing,
    ] {
        let options = TransformOptions::new(OutputGeometry::Exact {
            width: 2,
            height: 2,
        })
        .with_output(OutputSettings::default_for(OutputFormat::WebP))
        .with_profile_policy(profile_policy);
        let result = catch_unwind(AssertUnwindSafe(|| {
            transcode(hostile, ImageFormat::JpegXl, &options)
        }))
        .expect("public JPEG XL processing must not panic");
        let error = result.unwrap_err();
        assert_eq!(
            error.kind(),
            Kind::MalformedInput,
            "{profile_policy:?}: {error}"
        );
    }
}

// Every upstream libavif test file, including grid, alpha, clap, gain-map,
// animation, HDR and proof-of-concept inputs, must end in a controlled public
// result under each explicit color policy. Selected files pin the expected
// reason; the rest only must not panic.
#[cfg(all(feature = "avif-read", feature = "webp-write"))]
#[test]
fn upstream_libavif_corpus_returns_controlled_public_results() {
    use crate::pipeline::{
        ImageFormat, InputProfilePolicy, OutputFormat, OutputGeometry, OutputSettings,
        ProcessingErrorKind as Kind, Threads, TransformOptions, transcode,
    };

    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("codecs/libavif-rs/upstream/libavif/tests/data");
    let mut paths: Vec<_> = std::fs::read_dir(&root)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "avif"))
        .collect();
    paths.sort();
    assert!(paths.len() > 50, "upstream AVIF corpus is missing");
    let expected = |name: &str, policy: InputProfilePolicy| -> Option<Result<(), Kind>> {
        Some(match name {
            // `clap` is rejected rather than silently ignored (ADR-0019), and
            // animation is not accepted as a still image.
            "clap_irot_imir_non_essential.avif" | "colors-animated-8bpc.avif" => {
                Err(Kind::UnsupportedInput)
            }
            // Grid and alpha stills reach the encoder.
            "abc_color_irot_alpha_irot.avif"
            | "color_grid_alpha_grid_shared_tiles.avif"
            | "sofa_grid1x5_420.avif" => Ok(()),
            // PQ/HLG is never reinterpreted as sRGB.
            "colors_hdr_rec2020.avif" | "seine_hdr_rec2020.avif" => Err(Kind::UnsupportedColor),
            // A container ICC profile needs the rendered-ICC transform.
            "paris_icc_exif_xmp.avif" if policy != InputProfilePolicy::UseRenderedIcc => {
                Err(Kind::UnsupportedColor)
            }
            _ => return None,
        })
    };
    for path in paths {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let bytes = std::fs::read(&path).unwrap();
        for profile_policy in [
            InputProfilePolicy::Strict,
            InputProfilePolicy::AssumeSrgbIfMissing,
            InputProfilePolicy::UseRenderedIcc,
        ] {
            let options = TransformOptions::new(OutputGeometry::Exact {
                width: 7,
                height: 5,
            })
            .with_output(OutputSettings::default_for(OutputFormat::WebP))
            .with_transform_threads(Threads::ONE)
            .with_profile_policy(profile_policy)
            .with_max_input_pixels(4_000_000);
            let result = catch_unwind(AssertUnwindSafe(|| {
                transcode(&bytes, ImageFormat::Avif, &options)
            }))
            .unwrap_or_else(|_| panic!("{name} panicked under {profile_policy:?}"));
            let actual = result
                .as_ref()
                .map(|_| ())
                .map_err(crate::pipeline::ProcessingError::kind);
            // Success is pinned only for the untagged-sRGB policy; the other
            // policies may reject the same file for its missing tag or alpha.
            if let Some(expected) = expected(&name, profile_policy)
                && (profile_policy == InputProfilePolicy::AssumeSrgbIfMissing || expected.is_err())
            {
                assert_eq!(actual, expected, "{name} under {profile_policy:?}");
            }
            if let Err(error) = &result {
                assert_ne!(error.kind(), Kind::Internal, "{name}: {error}");
            }
        }
    }
}
