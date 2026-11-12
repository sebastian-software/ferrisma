//! A small, complete consumer of the documented `ferrisma::pipeline` facade.
//!
//! It does what a thumbnail service does with untrusted uploads: read each
//! file through a byte cap, identify the format from its signature instead
//! of its name, preflight JPEG and WebP headers with explicit limits, choose
//! the color policy deliberately, transcode with explicit resource limits,
//! and handle every failure category without parsing messages. Only public
//! items are used, exactly as a crates.io dependent would use them.
//!
//! ```sh
//! # Thumbnail files into a directory (sRGB is assumed only with the flag):
//! cargo run --example consumer --features jpeg-read,webp-read,webp-write -- \
//!     [--assume-srgb] OUTPUT_DIR INPUT...
//! # Exercise the failure handling and limits against the repository fixtures:
//! cargo run --example consumer --features jpeg-read,webp-read,webp-write -- --self-check
//! ```
//!
//! Exit status: 0 when every input succeeded, 1 when at least one input was
//! rejected, 2 for usage or configuration errors.

#[cfg(all(feature = "jpeg-read", feature = "webp-read", feature = "webp-write"))]
mod consumer {
    use std::io::Read;
    use std::path::{Path, PathBuf};
    use std::process::ExitCode;

    use ferrisma::pipeline::{
        ImageFormat, InputProfilePolicy, InspectionLimits, Output, OutputFormat, ProcessingError,
        ProcessingErrorKind, ResizeFit, ResizeOptions, TransformOptions, WebpOptions, inspect,
        transcode_resized,
    };

    /// Service limits. Every value is explicit; none relies on a default.
    #[derive(Clone, Copy)]
    struct Limits {
        input_bytes: usize,
        input_pixels: u64,
        managed_bytes: usize,
        output_bytes: usize,
    }

    const SERVICE_LIMITS: Limits = Limits {
        input_bytes: 32 * 1024 * 1024,
        input_pixels: 40_000_000,
        managed_bytes: 128 * 1024 * 1024,
        output_bytes: 4 * 1024 * 1024,
    };

    const THUMBNAIL: ResizeOptions = ResizeOptions {
        width: Some(512),
        height: Some(512),
        fit: ResizeFit::Inside,
        without_enlargement: true,
    };

    /// What the service does with a failure.
    #[derive(Debug, PartialEq, Eq)]
    enum Disposition {
        /// The upload is unacceptable; report it to the uploader.
        Reject,
        /// The input may be valid, but this deployment cannot process it.
        Unsupported,
        /// Transient pressure; the caller may retry later or with more budget.
        Retry,
        /// Configuration or Ferrisma bug; alert an operator, do not retry.
        Operator,
    }

    /// Map the stable failure categories to service behavior. The wildcard arm
    /// is required: `ProcessingErrorKind` is `#[non_exhaustive]`, and a new
    /// category must not crash or silently succeed.
    fn disposition(error: &ProcessingError) -> Disposition {
        match error.kind() {
            ProcessingErrorKind::InputLimitExceeded | ProcessingErrorKind::MalformedInput => {
                Disposition::Reject
            }
            // A format this build does not include is a property of the
            // deployment, not of the upload.
            ProcessingErrorKind::UnsupportedInput
            | ProcessingErrorKind::UnsupportedColor
            | ProcessingErrorKind::FeatureDisabled => Disposition::Unsupported,
            ProcessingErrorKind::ResourceExhausted | ProcessingErrorKind::Cancelled => {
                Disposition::Retry
            }
            ProcessingErrorKind::InvalidOptions
            | ProcessingErrorKind::EncodeFailed
            | ProcessingErrorKind::Internal
            | _ => Disposition::Operator,
        }
    }

    /// Identify the container from its signature; file names are untrusted.
    fn sniff(bytes: &[u8]) -> Option<ImageFormat> {
        const JXL_CONTAINER: &[u8] = b"\0\0\0\x0cJXL \r\n\x87\n";
        let at = |range: std::ops::Range<usize>| bytes.get(range).unwrap_or_default();
        if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
            Some(ImageFormat::Jpeg)
        } else if at(0..4) == b"RIFF" && at(8..12) == b"WEBP" {
            Some(ImageFormat::WebP)
        } else if bytes.starts_with(&[0xff, 0x0a]) || bytes.starts_with(JXL_CONTAINER) {
            Some(ImageFormat::JpegXl)
        } else if at(4..8) == b"ftyp" && matches!(at(8..12), b"avif" | b"avis") {
            Some(ImageFormat::Avif)
        } else {
            None
        }
    }

    /// Read at most `limit` bytes; one more byte means the input is too large.
    fn read_bounded(path: &Path, limit: usize) -> std::io::Result<Option<Vec<u8>>> {
        let mut bytes = Vec::new();
        std::fs::File::open(path)?
            .take(limit as u64 + 1)
            .read_to_end(&mut bytes)?;
        Ok((bytes.len() <= limit).then_some(bytes))
    }

    fn options(limits: Limits, policy: InputProfilePolicy) -> TransformOptions {
        TransformOptions {
            output_format: OutputFormat::WebP,
            webp: WebpOptions::Lossy {
                quality: 80.0,
                speed: 6,
            },
            profile_policy: policy,
            max_input_bytes: limits.input_bytes,
            max_input_pixels: limits.input_pixels,
            max_managed_bytes: limits.managed_bytes,
            max_output_bytes: limits.output_bytes,
            ..TransformOptions::default()
        }
    }

    /// Choose the color policy from header evidence. Missing color evidence is
    /// treated as sRGB only when the operator opted in for this deployment.
    fn policy(
        bytes: &[u8],
        format: ImageFormat,
        limits: Limits,
        assume_srgb: bool,
    ) -> Result<InputProfilePolicy, ProcessingError> {
        if !matches!(format, ImageFormat::Jpeg | ImageFormat::WebP) {
            // Header inspection covers JPEG and WebP; other formats decide
            // during transcode and fail closed under the strict policy.
            return Ok(if assume_srgb {
                InputProfilePolicy::AssumeSrgbIfMissing
            } else {
                InputProfilePolicy::Strict
            });
        }
        let inspection = InspectionLimits {
            max_input_bytes: limits.input_bytes,
            max_input_pixels: limits.input_pixels,
        };
        let metadata = inspect(bytes, format, inspection)?;
        let untagged = metadata.rendered_icc.is_none() && metadata.rendered_cicp.is_none();
        Ok(if untagged && assume_srgb {
            InputProfilePolicy::AssumeSrgbIfMissing
        } else {
            InputProfilePolicy::Strict
        })
    }

    fn thumbnail(
        bytes: &[u8],
        format: ImageFormat,
        limits: Limits,
        assume_srgb: bool,
    ) -> Result<Output, ProcessingError> {
        let policy = policy(bytes, format, limits, assume_srgb)?;
        transcode_resized(bytes, format, options(limits, policy), THUMBNAIL)
    }

    fn describe(output: &Output) -> String {
        let report = &output.report;
        let mut text = format!(
            "{}x{} WebP, {} bytes; peak managed {} bytes, encoder frame {} bytes",
            output.width,
            output.height,
            output.bytes.len(),
            report.peak_managed_bytes,
            report.encoder_output_frame_bytes
        );
        if report.opaque_decoder_memory || report.opaque_encoder_memory {
            text.push_str("; codec scratch memory is outside the budget");
        }
        text
    }

    /// Write next to the destination, then rename, so readers never observe
    /// a partial thumbnail.
    fn write_atomically(destination: &Path, bytes: &[u8]) -> std::io::Result<()> {
        let temporary = destination.with_extension("webp.partial");
        std::fs::write(&temporary, bytes)?;
        std::fs::rename(&temporary, destination)
    }

    fn process(arguments: &[String]) -> ExitCode {
        let assume_srgb = arguments
            .first()
            .is_some_and(|flag| flag == "--assume-srgb");
        let arguments = &arguments[usize::from(assume_srgb)..];
        let [output_dir, inputs @ ..] = arguments else {
            return usage();
        };
        if inputs.is_empty() {
            return usage();
        }
        let output_dir = PathBuf::from(output_dir);
        if let Err(error) = std::fs::create_dir_all(&output_dir) {
            eprintln!("cannot create {}: {error}", output_dir.display());
            return ExitCode::from(2);
        }
        let mut rejected = 0_usize;
        for input in inputs {
            let path = Path::new(input);
            let bytes = match read_bounded(path, SERVICE_LIMITS.input_bytes) {
                Ok(Some(bytes)) => bytes,
                Ok(None) => {
                    println!("{input}: rejected, larger than the input limit");
                    rejected += 1;
                    continue;
                }
                Err(error) => {
                    println!("{input}: unreadable: {error}");
                    rejected += 1;
                    continue;
                }
            };
            let Some(format) = sniff(&bytes) else {
                println!("{input}: rejected, not a JPEG, WebP, JPEG XL, or AVIF signature");
                rejected += 1;
                continue;
            };
            match thumbnail(&bytes, format, SERVICE_LIMITS, assume_srgb) {
                Ok(output) => {
                    let name = path.file_stem().unwrap_or(path.as_os_str());
                    let destination = output_dir.join(name).with_extension("webp");
                    if let Err(error) = write_atomically(&destination, &output.bytes) {
                        println!("{input}: cannot write {}: {error}", destination.display());
                        rejected += 1;
                        continue;
                    }
                    println!("{input}: {}", describe(&output));
                }
                Err(error) => {
                    let disposition = disposition(&error);
                    println!("{input}: {disposition:?} ({:?}): {error}", error.kind());
                    if disposition == Disposition::Operator {
                        return ExitCode::from(2);
                    }
                    rejected += 1;
                }
            }
        }
        if rejected == 0 {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }

    /// Run the documented failure categories against the repository fixtures
    /// and fail if the facade reports anything else.
    #[allow(clippy::too_many_lines)] // One linear list of scenarios.
    fn self_check() -> ExitCode {
        let fixture = |name: &str| {
            std::fs::read(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("tests/fixtures")
                    .join(name),
            )
        };
        let (jpeg, webp) = match (fixture("sample.jpg"), fixture("webp/reference-lossy.webp")) {
            (Ok(jpeg), Ok(webp)) => (jpeg, webp),
            (Err(error), _) | (_, Err(error)) => {
                eprintln!("self-check needs the repository fixtures: {error}");
                return ExitCode::from(2);
            }
        };
        let strict = options(SERVICE_LIMITS, InputProfilePolicy::Strict);
        let assumed = options(SERVICE_LIMITS, InputProfilePolicy::AssumeSrgbIfMissing);
        let half = &jpeg[..jpeg.len() / 2];
        let garbage = [0_u8; 64];
        let run = |bytes: &[u8], format, options| {
            transcode_resized(bytes, format, options, THUMBNAIL).map(|output| describe(&output))
        };
        let limited = |limits| options(limits, InputProfilePolicy::AssumeSrgbIfMissing);
        let cases: Vec<(
            &str,
            Result<String, ProcessingError>,
            Option<ProcessingErrorKind>,
        )> = vec![
            (
                "untagged JPEG, explicit sRGB assumption",
                run(&jpeg, ImageFormat::Jpeg, assumed),
                None,
            ),
            (
                "WebP fixture, sRGB assumption only if untagged",
                thumbnail(&webp, ImageFormat::WebP, SERVICE_LIMITS, true)
                    .map(|output| describe(&output)),
                None,
            ),
            (
                "untagged JPEG under the strict default policy",
                run(&jpeg, ImageFormat::Jpeg, strict),
                Some(ProcessingErrorKind::UnsupportedColor),
            ),
            (
                "truncated JPEG",
                run(half, ImageFormat::Jpeg, assumed),
                Some(ProcessingErrorKind::MalformedInput),
            ),
            (
                "bytes that are not a WebP file",
                run(&garbage, ImageFormat::WebP, assumed),
                Some(ProcessingErrorKind::MalformedInput),
            ),
            (
                "compressed size above max_input_bytes",
                run(
                    &jpeg,
                    ImageFormat::Jpeg,
                    limited(Limits {
                        input_bytes: 64,
                        ..SERVICE_LIMITS
                    }),
                ),
                Some(ProcessingErrorKind::InputLimitExceeded),
            ),
            (
                "decoded size above max_input_pixels",
                run(
                    &jpeg,
                    ImageFormat::Jpeg,
                    limited(Limits {
                        input_pixels: 16,
                        ..SERVICE_LIMITS
                    }),
                ),
                Some(ProcessingErrorKind::InputLimitExceeded),
            ),
            (
                "managed budget below one working row",
                run(
                    &jpeg,
                    ImageFormat::Jpeg,
                    limited(Limits {
                        managed_bytes: 1024,
                        ..SERVICE_LIMITS
                    }),
                ),
                Some(ProcessingErrorKind::ResourceExhausted),
            ),
            (
                "encoded output above max_output_bytes",
                run(
                    &jpeg,
                    ImageFormat::Jpeg,
                    limited(Limits {
                        output_bytes: 64,
                        ..SERVICE_LIMITS
                    }),
                ),
                Some(ProcessingErrorKind::ResourceExhausted),
            ),
            (
                "header inspection with a pixel limit",
                inspect(
                    &jpeg,
                    ImageFormat::Jpeg,
                    InspectionLimits {
                        max_input_bytes: SERVICE_LIMITS.input_bytes,
                        max_input_pixels: 16,
                    },
                )
                .map(|metadata| format!("{}x{} header", metadata.width, metadata.height)),
                Some(ProcessingErrorKind::InputLimitExceeded),
            ),
        ];
        let mut failures = 0;
        for (name, result, expected) in cases {
            let actual = result.as_ref().err().map(ProcessingError::kind);
            let verdict = if actual == expected { "ok" } else { "MISMATCH" };
            failures += usize::from(actual != expected);
            match &result {
                Ok(summary) => println!("{verdict}: {name}: {summary}"),
                Err(error) => println!(
                    "{verdict}: {name}: {:?} -> {:?}: {error}",
                    error.kind(),
                    disposition(error)
                ),
            }
            if actual != expected {
                println!("  expected {expected:?}");
            }
        }
        if failures == 0 {
            println!("self-check passed");
            ExitCode::SUCCESS
        } else {
            println!("self-check failed: {failures} mismatches");
            ExitCode::FAILURE
        }
    }

    fn usage() -> ExitCode {
        eprintln!(
            "usage: consumer [--assume-srgb] OUTPUT_DIR INPUT...\n       consumer --self-check"
        );
        ExitCode::from(2)
    }

    pub fn main() -> ExitCode {
        let arguments: Vec<String> = std::env::args().skip(1).collect();
        if arguments.first().is_some_and(|flag| flag == "--self-check") {
            return self_check();
        }
        process(&arguments)
    }
}

#[cfg(all(feature = "jpeg-read", feature = "webp-read", feature = "webp-write"))]
fn main() -> std::process::ExitCode {
    consumer::main()
}

#[cfg(not(all(feature = "jpeg-read", feature = "webp-read", feature = "webp-write")))]
fn main() {
    eprintln!("Enable the jpeg-read, webp-read and webp-write features to run this example.");
}
