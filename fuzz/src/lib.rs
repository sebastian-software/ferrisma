//! Shared decoder/encoder fuzz driver. The first byte is a control byte;
//! every remaining byte is passed unchanged to the selected image decoder.

use ferrisma::pipeline::{
    AvifEncoder, ImageFormat, InputProfilePolicy, JpegXlDecoder, Output, OutputFormat,
    ProcessingError, TransformOptions, WorkingPrecision, transcode,
};

const MAX_INPUT_BYTES: usize = 512 * 1024;
const MAX_INPUT_PIXELS: u64 = 1_000_000;
const MAX_OUTPUT_BYTES: usize = 512 * 1024;
const MAX_MANAGED_BYTES: usize = 2 * 1024 * 1024;

/// Returns `None` for a missing selector or an oversized input. Other errors
/// are expected fuzz outcomes; a successful result reached the output encoder.
///
/// Every lane pins its backends explicitly, so the engine defaults never
/// change what a lane covers. This lane keeps the selectable external
/// backends (jxl-oxide, rav1e); the workspace defaults are covered by
/// `run_own_jxl_case` and `run_own_avif_case`.
pub fn run_case(bytes: &[u8]) -> Option<Result<Output, ProcessingError>> {
    run_case_with_backends(bytes, JpegXlDecoder::JxlOxide, AvifEncoder::Rav1e)
}

/// Runs only JPEG XL inputs through the workspace decoder (the engine default).
#[cfg(feature = "own-jxl")]
pub fn run_own_jxl_case(bytes: &[u8]) -> Option<Result<Output, ProcessingError>> {
    if bytes.first()? & 3 != 1 {
        return None;
    }
    run_case_with_backends(bytes, JpegXlDecoder::LibjxlRs, AvifEncoder::Rav1e)
}

/// Runs AVIF-output inputs through the workspace AOM writer at the given depth.
pub fn run_own_avif_case(bytes: &[u8], depth: u8) -> Option<Result<Output, ProcessingError>> {
    if bytes.first()? & 4 != 0 {
        return None;
    }
    run_case_with_backends(
        bytes,
        JpegXlDecoder::JxlOxide,
        AvifEncoder::Libavif { depth },
    )
}

fn run_case_with_backends(
    bytes: &[u8],
    jxl_decoder: JpegXlDecoder,
    avif_encoder: AvifEncoder,
) -> Option<Result<Output, ProcessingError>> {
    let (&control, encoded) = bytes.split_first()?;
    if encoded.is_empty() || encoded.len() > MAX_INPUT_BYTES {
        return None;
    }
    let kind = match control & 3 {
        0 => ImageFormat::Jpeg,
        1 => ImageFormat::JpegXl,
        2 => ImageFormat::Avif,
        _ => ImageFormat::WebP,
    };
    let output_format = if control & 4 == 0 {
        OutputFormat::Avif
    } else {
        OutputFormat::WebP
    };
    // Select an explicit policy; no profile is guessed when one is present.
    // Existing selector 1's JXL fixture contains a rendered RGB ICC profile.
    let profile_policy = match (control >> 3) & 3 {
        0 if kind == ImageFormat::JpegXl => InputProfilePolicy::UseRenderedIcc,
        0 | 2 => InputProfilePolicy::AssumeSrgbIfMissing,
        1 => InputProfilePolicy::Strict,
        _ => InputProfilePolicy::UseRenderedIcc,
    };
    let precision = if control & 0x20 == 0 {
        WorkingPrecision::F32
    } else {
        WorkingPrecision::F64Reference
    };
    let (output_width, output_height) = match control >> 6 {
        0 => (1 + u32::from(control & 7), 1 + u32::from(encoded[0] % 8)),
        1 => (8192, 1),
        2 => (1, 8192),
        _ => (64, 64),
    };
    let options = TransformOptions {
        output_format,
        jxl_decoder,
        // A nondefault AVIF selection conflicts with WebP output; WebP keeps
        // the build's default selection, like the other shared AVIF options.
        avif_encoder: if output_format == OutputFormat::Avif {
            avif_encoder
        } else {
            AvifEncoder::default()
        },
        output_width,
        output_height,
        profile_policy,
        precision,
        // WebP requires these shared AVIF options to retain their defaults.
        quality: if output_format == OutputFormat::Avif {
            60.0
        } else {
            75.0
        },
        speed: if output_format == OutputFormat::Avif {
            10
        } else {
            6
        },
        transform_threads: 1,
        encoder_threads: 1,
        max_input_bytes: MAX_INPUT_BYTES,
        max_input_pixels: MAX_INPUT_PIXELS,
        max_output_bytes: MAX_OUTPUT_BYTES,
        max_managed_bytes: MAX_MANAGED_BYTES,
        ..TransformOptions::default()
    };
    Some(transcode(encoded, kind, options))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrisma::pipeline::ProcessingErrorKind;

    const JPEG: &[u8] = include_bytes!("../../tests/fixtures/sample.jpg");
    const JXL: &[u8] = include_bytes!("../../tests/fixtures/sample.jxl");
    const AVIF: &[u8] = include_bytes!("../../tests/fixtures/orientation/avif-1.avif");
    const WEBP: &[u8] = include_bytes!("../../tests/fixtures/webp/reference-lossy.webp");

    fn framed(selector: u8, image: &[u8]) -> Vec<u8> {
        let mut bytes = vec![selector];
        bytes.extend_from_slice(image);
        bytes
    }

    #[test]
    fn framed_seeds_reach_the_transform_and_both_encoders() {
        for (selector, image) in [(0, JPEG), (1, JXL), (2, AVIF), (3, WEBP)] {
            for (flag, format) in [(0, OutputFormat::Avif), (4, OutputFormat::WebP)] {
                let output = run_case(&framed(selector | flag, image))
                    .expect("input admitted by the driver")
                    .unwrap_or_else(|error| panic!("selector {}: {error}", selector | flag));
                assert_eq!(output.format, format);
                assert_eq!(output.report.decoder_requests, 1);
                assert!(output.width > 0 && output.height > 0);
                assert!(!output.bytes.is_empty());
                match format {
                    OutputFormat::Avif => assert_eq!(&output.bytes[4..8], b"ftyp"),
                    OutputFormat::WebP => assert_eq!(&output.bytes[..4], b"RIFF"),
                    // `OutputFormat` is non-exhaustive (ADR-0040).
                    other => panic!("unexpected output format {other:?}"),
                }
            }
        }
    }

    #[cfg(feature = "own-jxl")]
    #[test]
    fn workspace_jxl_reaches_both_encoders() {
        for (flag, format) in [(0, OutputFormat::Avif), (4, OutputFormat::WebP)] {
            let output = run_own_jxl_case(&framed(1 | flag, JXL))
                .expect("input admitted by the driver")
                .expect("workspace JPEG XL fixture must transcode");
            assert_eq!(output.format, format);
            assert_eq!(output.report.decoder_full_frame_buffers, 1);
            assert!(!output.bytes.is_empty());
        }
        assert!(run_own_jxl_case(&framed(0, JPEG)).is_none());
    }

    #[test]
    fn workspace_avif_writer_reaches_both_depths_from_every_input_format() {
        for depth in [8, 10] {
            for (selector, image) in [(0, JPEG), (1, JXL), (2, AVIF), (3, WEBP)] {
                let output = run_own_avif_case(&framed(selector, image), depth)
                    .expect("input admitted by the driver")
                    .expect("fixture must reach the workspace AVIF writer");
                assert_eq!(output.format, OutputFormat::Avif);
                assert_eq!(output.report.encoder_backend, "libavif-rs/aom");
                assert!(!output.bytes.is_empty());
            }
        }
        assert!(run_own_avif_case(&framed(4, JPEG), 8).is_none());
    }

    // The workflow supplies its generated corpus here, so these results cover
    // the exact seed bytes that are passed to libFuzzer, including rejections.
    #[test]
    fn campaign_seeds_report_reachability() {
        type CaseRunner = fn(&[u8]) -> Option<Result<Output, ProcessingError>>;
        let Some(root) = std::env::var_os("FERRISMA_FUZZ_SEEDS") else {
            return;
        };
        let lanes = [
            ("transcode", "default", run_case as CaseRunner),
            (
                "transcode_own_avif",
                "depth=8",
                (|bytes| run_own_avif_case(bytes, 8)) as CaseRunner,
            ),
            (
                "transcode_own_avif",
                "depth=10",
                (|bytes| run_own_avif_case(bytes, 10)) as CaseRunner,
            ),
            #[cfg(feature = "own-jxl")]
            (
                "transcode_own_jxl",
                "workspace-jxl",
                run_own_jxl_case as CaseRunner,
            ),
        ];
        for (lane, variant, run) in lanes {
            let mut paths: Vec<_> = std::fs::read_dir(std::path::Path::new(&root).join(lane))
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .collect();
            paths.sort();
            assert!(!paths.is_empty());
            for path in paths {
                let data = std::fs::read(&path).unwrap();
                let result = run(&data).expect("campaign seed must be admitted by the driver");
                let name = path.file_name().unwrap().to_string_lossy();
                if name == "icc-valid" || name == "avif-alpha" {
                    assert!(
                        result.is_ok(),
                        "the supported ICC/alpha seed must reach encoding"
                    );
                } else if name.starts_with("icc-tag-") {
                    assert!(
                        matches!(&result, Err(error) if error.kind() == ProcessingErrorKind::UnsupportedColor)
                    );
                }
                match result {
                    Ok(output) => {
                        assert!(output.report.decoder_requests > 0);
                        assert!(!output.bytes.is_empty());
                        if lane == "transcode_own_avif" {
                            assert_eq!(output.report.encoder_backend, "libavif-rs/aom");
                        }
                        eprintln!(
                            "{lane}/{name} ({variant}): encoded {:?} via {}",
                            output.format, output.report.encoder_backend
                        );
                    }
                    Err(error) => {
                        eprintln!("{lane}/{name} ({variant}): rejected {:?}", error.kind())
                    }
                }
            }
        }
    }

    #[test]
    fn strict_policy_and_input_caps_remain_enforced() {
        let strict_jpeg = run_case(&framed(8, JPEG)).unwrap().unwrap_err();
        assert_eq!(strict_jpeg.kind(), ProcessingErrorKind::UnsupportedColor);
        // An embedded ICC is not replaced by the assume-if-missing policy.
        let tagged_jxl = run_case(&framed(1 | 0x10, JXL)).unwrap().unwrap_err();
        assert_eq!(tagged_jxl.kind(), ProcessingErrorKind::UnsupportedColor);
        assert!(run_case(&[]).is_none());
        assert!(run_case(&[0]).is_none());
        assert!(run_case(&vec![0; MAX_INPUT_BYTES + 2]).is_none());

        // Only the JPEG SOF dimensions change; admission must reject the
        // declared geometry before attempting to render the truncated data.
        let mut oversized = JPEG.to_vec();
        let sof = oversized
            .windows(2)
            .position(|pair| pair == [0xff, 0xc0])
            .unwrap();
        oversized[sof + 5..sof + 7].copy_from_slice(&1001_u16.to_be_bytes());
        oversized[sof + 7..sof + 9].copy_from_slice(&1001_u16.to_be_bytes());
        let error = run_case(&framed(0, &oversized)).unwrap().unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::InputLimitExceeded);

        // The 8192x1 output lane is admitted by the driver but should not
        // bypass the 2 MiB engine budget while building its filter plan.
        let error = run_case(&framed(0x40, JPEG)).unwrap().unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::ResourceExhausted);
    }
}
