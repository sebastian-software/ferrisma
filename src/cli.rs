//! The command-line front end, using exactly the same graph and executor as
//! the Rust API. It lives in the library so that encoder adapters outside
//! this crate (ADR-0024) can ship the same command with their backend.

use std::error::Error;
use std::fmt;
use std::io::{self, Read};
use std::path::Path;
use std::process::ExitCode;
use std::str::FromStr;

use crate::pipeline::{
    ChromaSubsampling, Crop, EncoderBackend, ImageFormat, InputProfilePolicy, JpegXlOptions,
    KernelLevel, Output, OutputFormat, ProcessingError, ProcessingErrorKind, RefusalReason,
    ResizeFilter, TransformOptions, WebpOptions, WorkingPrecision, transcode, transcode_with,
};

/// A wrong command line: an unknown, repeated or contradictory flag, a
/// missing or unsupported positional argument, or a value that does not
/// parse. The command exits with status 2 for it (since 2026-11-16).
#[derive(Debug)]
struct Usage(String);

impl fmt::Display for Usage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for Usage {}

/// Mark `error` as a usage error, keeping its message.
fn usage(error: impl fmt::Display) -> Box<dyn Error> {
    Box::new(Usage(error.to_string()))
}

/// Parse the value of `option` after `prefix`, naming the flag when it does
/// not parse.
fn parse_value<T: FromStr>(option: &str, prefix: &str) -> Result<T, Box<dyn Error>>
where
    T::Err: fmt::Display,
{
    option[prefix.len()..]
        .parse()
        .map_err(|error| format!("invalid value in {option}: {error}").into())
}

// Bound the stream itself: file metadata can be absent or change after opening.
fn read_input(mut reader: impl Read, limit: usize) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    reader.by_ref().take(limit as u64).read_to_end(&mut data)?;
    let mut sentinel = [0];
    match reader.read_exact(&mut sentinel) {
        Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => Ok(data),
        Err(error) => Err(error),
        Ok(()) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("compressed input exceeds the {limit}-byte limit; increase --max-input-mb"),
        )),
    }
}

fn parse_format(path: &Path) -> Result<ImageFormat, Box<dyn Error>> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match extension.as_str() {
        "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
        "webp" => Ok(ImageFormat::WebP),
        "jxl" => Ok(ImageFormat::JpegXl),
        "avif" => Ok(ImageFormat::Avif),
        _ => Err(usage(
            "unsupported input file extension; expected .jpg, .jpeg, .webp, .jxl or .avif",
        )),
    }
}

fn parse_crop(value: &str) -> Result<Crop, Box<dyn Error>> {
    let parts: Vec<u32> = value
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(|error| format!("invalid value in --crop={value}: {error}"))?;
    let [x, y, width, height] = parts[..] else {
        return Err("crop must be x,y,width,height".into());
    };
    Ok(Crop {
        x,
        y,
        width,
        height,
    })
}

/// Apply one `--flag` or `--flag=value` to the options.
#[allow(clippy::too_many_lines)] // One flat match keeps every flag visible.
fn apply_option(options: &mut TransformOptions, option: &str) -> Result<(), Box<dyn Error>> {
    match option {
        "--assume-srgb" => options.profile_policy = InputProfilePolicy::AssumeSrgbIfMissing,
        "--use-rendered-icc" => options.profile_policy = InputProfilePolicy::UseRenderedIcc,
        "--area" => options.filter = ResizeFilter::Area,
        "--reference" => options.precision = WorkingPrecision::F64Reference,
        // The format-independent looking encoder flags were removed on
        // 2026-11-15: each output has its encoder's own flags (ADR-0043 D4).
        option
            if ["--quality=", "--speed=", "--chroma="]
                .iter()
                .any(|removed| option.starts_with(removed)) =>
        {
            return Err(format!(
                "unknown option: {option} (removed on 2026-11-15; encoder settings are per \
                 format: --avif-quality=, --avif-speed=, --avif-chroma=, --webp-quality=, \
                 --webp-method=, --jxl-distance=)"
            )
            .into());
        }
        "--kernel-level=scalar" => options.kernel_level_ceiling = Some(KernelLevel::Scalar),
        "--kernel-level=neon" => options.kernel_level_ceiling = Some(KernelLevel::Neon),
        "--kernel-level=avx2" => options.kernel_level_ceiling = Some(KernelLevel::Avx2),
        "--kernel-level=avx512" => options.kernel_level_ceiling = Some(KernelLevel::Avx512),
        option
            if AVIF_FLAGS.contains(&flag_name(option))
                && options.output_format != OutputFormat::Avif =>
        {
            return Err(format!("{} requires .avif output", flag_name(option)).into());
        }
        option if option.starts_with("--avif-quality=") => {
            options.avif.quality = parse_value(option, "--avif-quality=")?;
        }
        option if option.starts_with("--avif-alpha-quality=") => {
            options.avif.alpha_quality = parse_value(option, "--avif-alpha-quality=")?;
        }
        option if option.starts_with("--avif-speed=") => {
            options.avif.speed = parse_value(option, "--avif-speed=")?;
        }
        option if option.starts_with("--avif-depth=") => {
            // Without the flag the depth follows the input; the engine
            // refuses a depth other than 8, 10 or 12 before decoding.
            options.avif.depth = Some(parse_value(option, "--avif-depth=")?);
        }
        "--avif-chroma=444" => options.avif.chroma = ChromaSubsampling::Yuv444,
        "--avif-chroma=420" => options.avif.chroma = ChromaSubsampling::Yuv420,
        option
            if WEBP_FLAGS.contains(&flag_name(option))
                && options.output_format != OutputFormat::WebP =>
        {
            return Err(format!("{} requires .webp output", flag_name(option)).into());
        }
        "--webp-lossless" => {
            if !matches!(options.webp, WebpOptions::Lossless { .. }) {
                options.webp = WebpOptions::Lossless { level: 4 };
            }
        }
        option if option.starts_with("--webp-level=") => {
            options.webp = WebpOptions::Lossless {
                level: parse_value(option, "--webp-level=")?,
            };
        }
        // Lossy-only settings; `refuse_exclusive_flags` has already refused
        // them beside the lossless flags.
        option if option.starts_with("--webp-quality=") => {
            if let WebpOptions::Lossy { method, .. } = options.webp {
                options.webp = WebpOptions::Lossy {
                    quality: parse_value(option, "--webp-quality=")?,
                    method,
                };
            }
        }
        option if option.starts_with("--webp-method=") => {
            if let WebpOptions::Lossy { quality, .. } = options.webp {
                options.webp = WebpOptions::Lossy {
                    quality,
                    method: parse_value(option, "--webp-method=")?,
                };
            }
        }
        "--jxl-lossless" if options.output_format == OutputFormat::JpegXl => {
            // A `--jxl-depth=` given before stays.
            options.jpeg_xl = JpegXlOptions::Lossless {
                effort: 2,
                depth: options.jpeg_xl.depth(),
            };
        }
        "--jxl-lossless" => return Err("--jxl-lossless requires .jxl output".into()),
        option if option.starts_with("--jxl-depth=") => {
            if options.output_format != OutputFormat::JpegXl {
                return Err("--jxl-depth requires .jxl output".into());
            }
            // Without the flag the depth follows the input; the engine
            // refuses a depth outside 8 to 16 before decoding.
            options.jpeg_xl = options
                .jpeg_xl
                .with_depth(Some(parse_value(option, "--jxl-depth=")?));
        }
        option if option.starts_with("--jxl-distance=") => {
            if options.output_format != OutputFormat::JpegXl {
                return Err("--jxl-distance requires .jxl output".into());
            }
            // The range is checked by the transcode, before decoding.
            options.jpeg_xl = JpegXlOptions::Lossy {
                distance: parse_value(option, "--jxl-distance=")?,
                depth: options.jpeg_xl.depth(),
            };
        }
        option if option.starts_with("--max-output-mb=") => {
            let megabytes: usize = parse_value(option, "--max-output-mb=")?;
            options.max_output_bytes = megabytes
                .checked_mul(1024 * 1024)
                .ok_or("encoded output limit overflow")?;
        }
        option if option.starts_with("--transform-threads=") => {
            options.transform_threads = parse_value(option, "--transform-threads=")?;
        }
        option if option.starts_with("--encoder-threads=") => {
            options.encoder_threads = parse_value(option, "--encoder-threads=")?;
        }
        option if option.starts_with("--budget-mb=") => {
            let megabytes: usize = parse_value(option, "--budget-mb=")?;
            options.max_managed_bytes = megabytes
                .checked_mul(1024 * 1024)
                .ok_or("memory budget overflow")?;
        }
        option if option.starts_with("--max-input-mb=") => {
            let megabytes: usize = parse_value(option, "--max-input-mb=")?;
            options.max_input_bytes = megabytes
                .checked_mul(1024 * 1024)
                .ok_or("compressed input limit overflow")?;
        }
        option if option.starts_with("--max-input-mpx=") => {
            let megapixels: u64 = parse_value(option, "--max-input-mpx=")?;
            options.max_input_pixels = megapixels
                .checked_mul(1_000_000)
                .ok_or("decoded pixel limit overflow")?;
        }
        option if option.starts_with("--crop=") => {
            options.crop = Some(parse_crop(&option[7..])?);
        }
        _ => return Err(format!("unknown option: {option}").into()),
    }
    Ok(())
}

/// The AVIF writer's flags (libavif's own scales; ADR-0043 D4).
const AVIF_FLAGS: &[&str] = &[
    "--avif-quality",
    "--avif-alpha-quality",
    "--avif-speed",
    "--avif-depth",
    "--avif-chroma",
];

/// The WebP writer's flags (libwebp's own scales; ADR-0043 D4).
const WEBP_FLAGS: &[&str] = &[
    "--webp-quality",
    "--webp-method",
    "--webp-lossless",
    "--webp-level",
];

/// The flag of one argument: `--name` of `--name=value`, or the switch.
fn flag_name(option: &str) -> &str {
    option.split_once('=').map_or(option, |(name, _)| name)
}

/// Flags that contradict each other. Giving both of a pair is an error
/// that names the two flags; neither takes precedence (owner decision of
/// 2026-11-15, ADR-0042). Lossless WebP has no quality or method of its
/// own: its level sets both.
const EXCLUSIVE_FLAGS: &[(&str, &str)] = &[
    ("--assume-srgb", "--use-rendered-icc"),
    ("--jxl-lossless", "--jxl-distance"),
    ("--webp-lossless", "--webp-quality"),
    ("--webp-lossless", "--webp-method"),
    ("--webp-level", "--webp-quality"),
    ("--webp-level", "--webp-method"),
];

/// Refuse the first pair of [`EXCLUSIVE_FLAGS`] that `options` contains,
/// before any flag is applied.
fn refuse_exclusive_flags(options: &[String]) -> Result<(), Box<dyn Error>> {
    let given = |flag: &str| options.iter().any(|option| flag_name(option) == flag);
    match EXCLUSIVE_FLAGS
        .iter()
        .find(|(first, second)| given(first) && given(second))
    {
        Some((first, second)) => Err(format!("{first} and {second} are mutually exclusive").into()),
        None => Ok(()),
    }
}

/// True when `--version` or `-V` appears after the program name. It takes
/// precedence over every other argument, and no input is read.
fn is_version_request(args: &[String]) -> bool {
    args.iter()
        .skip(1)
        .any(|arg| arg == "--version" || arg == "-V")
}

/// True when `--help` or `-h` appears after the program name (since
/// 2026-11-16). It takes precedence over every argument except the version
/// flags, and no input is read.
fn is_help_request(args: &[String]) -> bool {
    args.iter()
        .skip(1)
        .any(|arg| arg == "--help" || arg == "-h")
}

/// The usage text for `--help` (standard output) and for missing
/// positional arguments (standard error).
fn usage_text(args: &[String]) -> String {
    let program = args
        .first()
        .and_then(|path| Path::new(path).file_name())
        .and_then(|name| name.to_str())
        .unwrap_or("ferrisma");
    format!(
        "usage: {program} --version|-V\n       {program} --help|-h\n       {program} input.jpg|input.webp|input.jxl|input.avif output.avif|output.webp|output.jxl WIDTH HEIGHT [--avif-quality=60 --avif-alpha-quality=60 (libavif 0-100) --avif-speed=6 (AOM cpu-used 0-10) --avif-depth=8|10|12 (default: the input's) --avif-chroma=444|420] [--webp-quality=75 (libwebp 1-100) --webp-method=4 (libwebp 0-6)|--webp-lossless --webp-level=4 (libwebp 0-9)] [--jxl-distance=1.0 (0.05 to below 3.0)|--jxl-lossless] [--jxl-depth=8..16 (default: the input's)] [--assume-srgb|--use-rendered-icc] [--area] [--reference] [--kernel-level=scalar|neon|avx2|avx512] [--encoder-threads=1] [--transform-threads=0] [--max-output-mb=256] [--budget-mb=1024] [--max-input-mb=64] [--max-input-mpx=100] [--crop=x,y,w,h]\n       Each flag may be given once. Mutually exclusive: --assume-srgb and --use-rendered-icc; --jxl-lossless and --jxl-distance; --webp-lossless or --webp-level and --webp-quality or --webp-method. Defaults are the original tools' (avifenc, cwebp, cjxl).\n       Exit status: 0 success, 1 processing error, 2 usage error."
    )
}

/// The exit status for an error of [`run`]: 2 when the command line is
/// wrong, including a value the engine refuses as
/// [`ProcessingErrorKind::InvalidOptions`] (a setting outside its scale, a
/// zero size or limit, a crop outside the image); 1 for every processing
/// error: unreadable input or unwritable output, a refused, malformed or too
/// large input, an exhausted resource, a missing feature, an encoder
/// failure and an internal error (since 2026-11-16).
fn exit_status(error: &(dyn Error + 'static)) -> u8 {
    if error.is::<Usage>() {
        return 2;
    }
    match error
        .downcast_ref::<ProcessingError>()
        .map(ProcessingError::kind)
    {
        Some(ProcessingErrorKind::InvalidOptions) => 2,
        _ => 1,
    }
}

/// The name of an error kind on the `failure:` line. Lower-case and
/// hyphenated, owned by this function, so renaming a Rust variant cannot
/// change the command's output.
fn kind_name(kind: ProcessingErrorKind) -> &'static str {
    match kind {
        ProcessingErrorKind::InvalidOptions => "invalid-options",
        ProcessingErrorKind::InputLimitExceeded => "input-limit-exceeded",
        ProcessingErrorKind::FeatureDisabled => "feature-disabled",
        ProcessingErrorKind::MalformedInput => "malformed-input",
        ProcessingErrorKind::UnsupportedInput => "unsupported-input",
        ProcessingErrorKind::UnsupportedColor => "unsupported-color",
        ProcessingErrorKind::ResourceExhausted => "resource-exhausted",
        ProcessingErrorKind::Cancelled => "cancelled",
        ProcessingErrorKind::EncodeFailed => "encode-failed",
        ProcessingErrorKind::Internal => "internal",
    }
}

/// The name of a refusal reason on the `failure:` line, as [`kind_name`].
fn reason_name(reason: RefusalReason) -> &'static str {
    match reason {
        RefusalReason::Animation => "animation",
        RefusalReason::ColorSignalMissing => "color-signal-missing",
        RefusalReason::RenderedIccRequiresOptIn => "rendered-icc-requires-opt-in",
        RefusalReason::CicpBesideIcc => "cicp-beside-icc",
        RefusalReason::HdrTransfer => "hdr-transfer",
        RefusalReason::UnsupportedColorEncoding => "unsupported-color-encoding",
        RefusalReason::RenderedIccMissing => "rendered-icc-missing",
        RefusalReason::ContradictoryColorSignals => "contradictory-color-signals",
        RefusalReason::UnsupportedLayout => "unsupported-layout",
        RefusalReason::AlphaAssociation => "alpha-association",
        RefusalReason::GeometricTransform => "geometric-transform",
        RefusalReason::UnsupportedEssentialProperty => "unsupported-essential-property",
        RefusalReason::IccColorSpaceMismatch => "icc-color-space-mismatch",
    }
}

/// The second error line for an error the engine returned, shaped like the
/// `report:` line: `failure: kind=<kind> reason=<reason or none>`.
fn failure_line(error: &ProcessingError) -> String {
    format!(
        "failure: kind={} reason={}",
        kind_name(error.kind()),
        error.refusal_reason().map_or("none", reason_name)
    )
}

/// Run the command as the `ferrisma` binary does and turn its result into
/// the process exit status (since 2026-11-16; experimental, like [`run`]).
/// An error is printed on standard error as one line `error: <message>`;
/// when the engine returned it, a second line `failure: kind=<kind>
/// reason=<reason or none>` names its kind and refusal reason in the
/// lower-case, hyphenated form of the `report:` line. Exit status 2 means
/// the command line is wrong, 1 any processing error; see [`run`].
#[must_use]
pub fn main(args: &[String], backend: Option<&dyn EncoderBackend>) -> ExitCode {
    match run(args, backend) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("error: {error}");
            if let Some(error) = error.downcast_ref::<ProcessingError>() {
                eprintln!("{}", failure_line(error));
            }
            ExitCode::from(exit_status(&*error))
        }
    }
}

/// The `--version` report, one item per line: the package version, the
/// target, the compiled-in engine features and the build identity (the
/// `FERRISMA_BUILD_ID` set at compile time, or `unknown`).
fn version_text(build_id: Option<&str>) -> String {
    let features: Vec<&str> = [
        (cfg!(feature = "jpeg-read"), "jpeg-read"),
        (cfg!(feature = "webp-read"), "webp-read"),
        (cfg!(feature = "webp-write"), "webp-write"),
        (cfg!(feature = "jpeg-xl-read"), "jpeg-xl-read"),
        (cfg!(feature = "jpeg-xl-write"), "jpeg-xl-write"),
        (cfg!(feature = "avif-read"), "avif-read"),
        (cfg!(feature = "avif-write"), "avif-write"),
        (cfg!(feature = "icc"), "icc"),
    ]
    .into_iter()
    .filter_map(|(enabled, name)| enabled.then_some(name))
    .collect();
    let features = if features.is_empty() {
        "none".to_owned()
    } else {
        features.join(", ")
    };
    let build = build_id.filter(|id| !id.is_empty()).unwrap_or("unknown");
    format!(
        "ferrisma {}\ntarget: {}-{}\nfeatures: {features}\nbuild: {build}",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::ARCH,
        std::env::consts::OS,
    )
}

/// Run the command with `args` (the program name first), encoding with
/// `backend`, or with the built-in sink when it is `None`. Every encoder
/// flag reaches an external backend in its request (`--avif-depth=` too,
/// since 2026-11-15). `--version` and `--help` print to standard output and
/// return success; missing positional arguments print the usage to standard
/// error and return exit status 2. [`main`] prints a returned error and
/// chooses its exit status.
///
/// # Errors
/// Invalid arguments, unreadable input, unwritable output and every
/// transcode error (a [`ProcessingError`]).
#[allow(clippy::too_many_lines)] // Argument handling and two report lines.
pub fn run(
    args: &[String],
    backend: Option<&dyn EncoderBackend>,
) -> Result<ExitCode, Box<dyn Error>> {
    if is_version_request(args) {
        println!("{}", version_text(option_env!("FERRISMA_BUILD_ID")));
        return Ok(ExitCode::SUCCESS);
    }
    if is_help_request(args) {
        println!("{}", usage_text(args));
        return Ok(ExitCode::SUCCESS);
    }
    if args.len() < 5 {
        eprintln!(
            "error: missing arguments; INPUT OUTPUT WIDTH HEIGHT are required\n{}",
            usage_text(args)
        );
        return Ok(ExitCode::from(2));
    }
    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);
    let output_extension = output_path.extension().and_then(|ext| ext.to_str());
    let output_format = match output_extension {
        Some("avif") => OutputFormat::Avif,
        Some("webp") => OutputFormat::WebP,
        // Lossy d1.0 by default, or --jxl-distance / --jxl-lossless;
        // requires the jpeg-xl-write feature (ADR-0039).
        Some("jxl") => OutputFormat::JpegXl,
        _ => return Err(usage("output must have a .avif, .webp or .jxl extension")),
    };
    let dimension = |name: &str, value: &str| {
        value
            .parse::<u32>()
            .map_err(|error| usage(format!("invalid {name} {value:?}: {error}")))
    };
    let mut options = TransformOptions {
        output_format,
        output_width: dimension("WIDTH", &args[3])?,
        output_height: dimension("HEIGHT", &args[4])?,
        ..TransformOptions::default()
    };
    refuse_exclusive_flags(&args[5..]).map_err(usage)?;
    let mut given: Vec<&str> = Vec::new();
    for option in args.iter().skip(5) {
        apply_option(&mut options, option).map_err(usage)?;
        // Owner decision of 2026-11-15 (ADR-0042): a repeated flag is an
        // error, never "the last one wins".
        let name = flag_name(option);
        if given.contains(&name) {
            return Err(usage(format!("{name} may only be given once")));
        }
        given.push(name);
    }
    let format = parse_format(input_path)?;
    let data = std::fs::File::open(input_path)
        .and_then(|file| read_input(file, options.max_input_bytes))
        .map_err(|error| format!("cannot read {}: {error}", input_path.display()))?;
    let output = match backend {
        None => transcode(&data, format, options)?,
        Some(backend) => transcode_with(&data, format, options, backend)?,
    };
    std::fs::write(output_path, &output.bytes)
        .map_err(|error| format!("cannot write {}: {error}", output_path.display()))?;
    println!(
        "Encoded {} bytes. Decoder requests: {}, known decoded full frames: {}, \
         peak engine-managed memory: {} bytes, encoder materialization: {} bytes.",
        output.bytes.len(),
        output.report.decoder_requests,
        output.report.decoder_full_frame_buffers,
        output.report.peak_managed_bytes,
        output.report.encoder_output_frame_bytes,
    );
    let timings = output.report.timings;
    println!(
        "Timings: open {:.1} ms, decode {:.1} ms, transform {:.1} ms, encode {:.1} ms, total {:.1} ms.",
        timings.open.as_secs_f64() * 1e3,
        timings.decode.as_secs_f64() * 1e3,
        timings.transform.as_secs_f64() * 1e3,
        timings.encode.as_secs_f64() * 1e3,
        timings.total.as_secs_f64() * 1e3,
    );
    println!("{}", report_line(&output));
    // Decoder work as rectangles `WxH+X+Y`; `>=` marks a lower bound. `crop`
    // and `requested` are upright display coordinates; `codec_request` and
    // `codec_output` are in the codec's frame (stored order, or display
    // order for the JPEG XL decoders, which orient themselves).
    let work = output.report.decoder_work;
    let rect = |r: crate::pipeline::Crop| format!("{}x{}+{}+{}", r.width, r.height, r.x, r.y);
    println!(
        "decoder_work: crop={} requested={} codec_request={} codec_output={} overfetch_pixels={} \
         reconstructed_pixels={} entropy_rows={} delivered_rows={} wrapper_copied_bytes={}",
        rect(work.crop),
        rect(work.requested),
        rect(work.codec_request),
        rect(work.codec_output),
        work.overfetch_pixels,
        work_count_value(work.reconstructed_pixels),
        work_count_value(work.entropy_rows),
        work.delivered_rows,
        work.wrapper_copied_bytes,
    );
    if output.report.opaque_decoder_memory || output.report.opaque_encoder_memory {
        eprintln!(
            "Note: decoder and/or encoder internal allocations are not subject to this memory budget."
        );
    }
    Ok(ExitCode::SUCCESS)
}

/// One stable `key=value` line for benchmark scripts. Values are integers,
/// except the precision, kernel level, the two backend names, the color
/// signal and `discarded` (the classes of ADR-0042 D2, comma-separated in
/// report order, or `none`); new keys are added before `discarded`, which
/// stays last (ADR-0040, ADR-0042). `input_depth` and `depth` (since
/// 2026-11-15) are the bits per sample read and written. Every value taken
/// from an enum comes from a name function of this module (since
/// 2026-11-16; `Debug` and the kernels crate's `Display` before), so
/// renaming a Rust variant cannot change the line. The backend names are
/// the report's strings, unchanged.
fn report_line(output: &Output) -> String {
    let report = &output.report;
    let timings = report.timings;
    format!(
        "report: open_us={} decode_us={} transform_us={} encode_us={} total_us={} \
         output_bytes={} peak_managed_bytes={} encoder_frame_bytes={} orientation_buffer_bytes={} \
         decoder_requests={} decoder_full_frames={} precision={} encoder_threads={} kernel_level={} transform_threads={} \
         encoder={} decoder={} color_signal={} input_depth={} depth={} discarded={}",
        timings.open.as_micros(),
        timings.decode.as_micros(),
        timings.transform.as_micros(),
        timings.encode.as_micros(),
        timings.total.as_micros(),
        output.bytes.len(),
        report.peak_managed_bytes,
        report.encoder_output_frame_bytes,
        report.orientation_buffer_bytes,
        report.decoder_requests,
        report.decoder_full_frame_buffers,
        precision_name(report.working_precision),
        report.encoder_threads,
        kernel_level_name(report.kernel_level),
        report.transform_threads,
        report.encoder_backend,
        report.decoder_backend,
        color_signal_name(report.color_signal),
        report.input_depth,
        report.output_depth,
        discarded_value(report.discarded),
    )
}

/// The `discarded=` value: class names joined by commas, or `none`.
fn discarded_value(discarded: crate::pipeline::DiscardedSet) -> String {
    if discarded.is_empty() {
        return "none".to_owned();
    }
    discarded
        .iter()
        .map(discarded_name)
        .collect::<Vec<_>>()
        .join(",")
}

/// The `precision=` value.
fn precision_name(precision: WorkingPrecision) -> &'static str {
    match precision {
        WorkingPrecision::F32 => "f32",
        WorkingPrecision::F64Reference => "f64-reference",
    }
}

/// The `kernel_level=` value; the same names as `--kernel-level=`.
fn kernel_level_name(level: KernelLevel) -> &'static str {
    match level {
        KernelLevel::Scalar => "scalar",
        KernelLevel::Neon => "neon",
        KernelLevel::Sve2 => "sve2",
        KernelLevel::Avx2 => "avx2",
        KernelLevel::Avx512 => "avx512",
        // A level a newer `ferrisma-kernels` adds without a name here.
        #[allow(unreachable_patterns)]
        _ => "unknown",
    }
}

/// The `color_signal=` value.
fn color_signal_name(signal: crate::pipeline::ColorSignal) -> &'static str {
    use crate::pipeline::ColorSignal;
    match signal {
        ColorSignal::Cicp => "cicp",
        ColorSignal::AssumedSrgb => "assumed-srgb",
        ColorSignal::RenderedIcc => "rendered-icc",
        ColorSignal::RenderedIccWithAgreeingCicp => "rendered-icc-with-agreeing-cicp",
    }
}

/// One class of the `discarded=` value.
fn discarded_name(class: crate::pipeline::Discarded) -> &'static str {
    use crate::pipeline::Discarded;
    match class {
        Discarded::GainMap => "gain-map",
        Discarded::Exif => "exif",
        Discarded::Xmp => "xmp",
        Discarded::OtherMetadata => "other-metadata",
        Discarded::IccProfile => "icc-profile",
        Discarded::UnknownContainerData => "unknown-container-data",
        Discarded::JpegReconstructionData => "jpeg-reconstruction-data",
        Discarded::Preview => "preview",
        Discarded::HiddenColor => "hidden-color",
        Discarded::Unpremultiplied => "unpremultiplied",
        Discarded::PrecisionReduced => "precision-reduced",
    }
}

/// A `decoder_work:` count: the number, or `>=` and the lower bound.
fn work_count_value(count: crate::pipeline::WorkCount) -> String {
    use crate::pipeline::WorkCount;
    match count {
        WorkCount::Exact(count) => count.to_string(),
        WorkCount::AtLeast(count) => format!(">={count}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_option, is_version_request, read_input, version_text};
    use crate::pipeline::TransformOptions;
    use std::io::{self, Cursor, Read};

    #[test]
    fn version_text_has_four_lines_in_order() {
        let text = version_text(Some("abc123"));
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 4, "{text}");
        assert_eq!(lines[0], format!("ferrisma {}", env!("CARGO_PKG_VERSION")));
        assert_eq!(
            lines[1],
            format!(
                "target: {}-{}",
                std::env::consts::ARCH,
                std::env::consts::OS
            )
        );
        assert!(lines[2].starts_with("features: "), "{}", lines[2]);
        assert_eq!(lines[3], "build: abc123");
        assert_eq!(version_text(None).lines().last(), Some("build: unknown"));
        assert_eq!(
            version_text(Some("")).lines().last(),
            Some("build: unknown")
        );
    }

    #[test]
    fn version_flags_are_recognized_anywhere_after_the_program_name() {
        let owned = |list: &[&str]| {
            list.iter()
                .map(|s| (*s).to_owned())
                .collect::<Vec<String>>()
        };
        assert!(is_version_request(&owned(&["ferrisma", "--version"])));
        assert!(is_version_request(&owned(&["ferrisma", "-V"])));
        assert!(is_version_request(&owned(&[
            "ferrisma", "in.jpg", "out.webp", "8", "8", "-V"
        ])));
        assert!(!is_version_request(&owned(&["ferrisma", "--versions"])));
        assert!(!is_version_request(&owned(&["--version"])));
        assert!(!is_version_request(&owned(&["ferrisma", "-v"])));
    }

    #[test]
    fn accepts_input_at_the_limit_and_rejects_one_byte_more() {
        assert_eq!(read_input(Cursor::new(b"abc"), 3).unwrap(), b"abc");
        let error = read_input(Cursor::new(b"abcd"), 3).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("--max-input-mb"));
        assert_eq!(read_input(Cursor::new(b""), 0).unwrap(), [] as [u8; 0]);
        assert!(read_input(Cursor::new(b"a"), 0).is_err());
    }

    #[test]
    fn bounds_streams_without_metadata_to_the_limit_plus_one_byte() {
        let mut reader = io::repeat(42).take(1_000_000);
        assert!(read_input(&mut reader, 3).is_err());
        assert_eq!(reader.limit(), 999_996);
    }

    #[test]
    fn propagates_read_errors() {
        struct Broken;
        impl Read for Broken {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::other("failed input"))
            }
        }
        assert_eq!(
            read_input(Broken, 3).unwrap_err().kind(),
            io::ErrorKind::Other
        );
    }

    #[test]
    fn kernel_level_defaults_to_none_and_avx512_is_explicit() {
        use crate::pipeline::KernelLevel;
        assert_eq!(TransformOptions::default().kernel_level_ceiling, None);
        for (flag, level) in [
            ("--kernel-level=scalar", KernelLevel::Scalar),
            ("--kernel-level=neon", KernelLevel::Neon),
            ("--kernel-level=avx2", KernelLevel::Avx2),
            ("--kernel-level=avx512", KernelLevel::Avx512),
        ] {
            let mut options = TransformOptions::default();
            apply_option(&mut options, flag).unwrap();
            assert_eq!(options.kernel_level_ceiling, Some(level), "{flag}");
        }
    }

    /// `--jxl-depth=` sets the depth of either profile, in any order with
    /// the profile flags (since 2026-11-15); it needs `.jxl` output.
    #[test]
    fn jxl_depth_is_kept_across_the_profile_flags() {
        use crate::pipeline::{JpegXlOptions, OutputFormat};
        let jxl = TransformOptions {
            output_format: OutputFormat::JpegXl,
            ..TransformOptions::default()
        };
        let mut options = jxl;
        apply_option(&mut options, "--jxl-depth=12").unwrap();
        apply_option(&mut options, "--jxl-lossless").unwrap();
        assert_eq!(
            options.jpeg_xl,
            JpegXlOptions::Lossless {
                effort: 2,
                depth: Some(12)
            }
        );
        let mut options = jxl;
        apply_option(&mut options, "--jxl-distance=0.5").unwrap();
        apply_option(&mut options, "--jxl-depth=16").unwrap();
        assert_eq!(options.jpeg_xl.depth(), Some(16));
        assert!(apply_option(&mut options, "--jxl-depth=x").is_err());
        let mut avif = TransformOptions::default();
        assert!(apply_option(&mut avif, "--jxl-depth=10").is_err());
    }

    #[test]
    fn selects_jpeg_xl_output_settings_and_rejects_foreign_ones() {
        use crate::pipeline::{JpegXlOptions, OutputFormat};
        let jxl = TransformOptions {
            output_format: OutputFormat::JpegXl,
            ..TransformOptions::default()
        };
        let mut options = jxl;
        assert_eq!(
            options.jpeg_xl,
            JpegXlOptions::Lossy {
                distance: 1.0,
                depth: None
            }
        );
        apply_option(&mut options, "--jxl-distance=0.5").unwrap();
        assert_eq!(
            options.jpeg_xl,
            JpegXlOptions::Lossy {
                distance: 0.5,
                depth: None
            }
        );
        assert!(apply_option(&mut options, "--jxl-distance=abc").is_err());
        let mut options = jxl;
        apply_option(&mut options, "--jxl-lossless").unwrap();
        assert_eq!(
            options.jpeg_xl,
            JpegXlOptions::Lossless {
                effort: 2,
                depth: None
            }
        );
        // Another output's flags name the output they need.
        for (flag, needs) in [
            ("--avif-quality=80", "--avif-quality requires .avif output"),
            ("--avif-speed=4", "--avif-speed requires .avif output"),
            ("--avif-chroma=420", "--avif-chroma requires .avif output"),
            ("--webp-quality=80", "--webp-quality requires .webp output"),
            ("--webp-method=4", "--webp-method requires .webp output"),
        ] {
            let mut options = jxl;
            let error = apply_option(&mut options, flag).unwrap_err();
            assert_eq!(error.to_string(), needs, "{flag}");
        }
        // JPEG XL flags require .jxl output.
        for flag in ["--jxl-lossless", "--jxl-distance=1"] {
            let error = apply_option(&mut TransformOptions::default(), flag).unwrap_err();
            assert!(error.to_string().contains(".jxl"), "{flag}: {error}");
        }
    }

    #[test]
    fn rejects_conflicting_jpeg_xl_modes() {
        let args: Vec<String> = [
            "ferrisma",
            "missing.jpg",
            "out.jxl",
            "8",
            "8",
            "--jxl-lossless",
            "--jxl-distance=1.5",
        ]
        .map(String::from)
        .to_vec();
        let error = super::run(&args, None).unwrap_err();
        assert_eq!(
            error.to_string(),
            "--jxl-lossless and --jxl-distance are mutually exclusive"
        );
    }

    /// Owner decision of 2026-11-15: every flag may be given once, and a
    /// pair of contradictory flags is refused with both names, before any
    /// input is read (the input file does not exist).
    #[test]
    fn refuses_repeated_and_contradictory_flags_before_reading_input() {
        let run = |output: &str, flags: &[&str]| {
            let mut args: Vec<String> = ["ferrisma", "missing.jpg", output, "8", "8"]
                .map(String::from)
                .to_vec();
            args.extend(flags.iter().map(|flag| (*flag).to_owned()));
            super::run(&args, None).unwrap_err().to_string()
        };
        for (output, flags, message) in [
            (
                "out.webp",
                &["--transform-threads=4", "--transform-threads=1"][..],
                "--transform-threads may only be given once",
            ),
            (
                "out.webp",
                &["--transform-threads=1", "--transform-threads=1"],
                "--transform-threads may only be given once",
            ),
            (
                "out.avif",
                &["--avif-quality=50", "--avif-quality=60"],
                "--avif-quality may only be given once",
            ),
            (
                "out.avif",
                &["--avif-depth=8", "--avif-depth=10"],
                "--avif-depth may only be given once",
            ),
            (
                "out.avif",
                &["--kernel-level=scalar", "--kernel-level=neon"],
                "--kernel-level may only be given once",
            ),
            (
                "out.avif",
                &["--area", "--area"],
                "--area may only be given once",
            ),
            (
                "out.webp",
                &["--webp-lossless", "--webp-lossless"],
                "--webp-lossless may only be given once",
            ),
            (
                "out.webp",
                &["--crop=0,0,4,4", "--crop=1,1,4,4"],
                "--crop may only be given once",
            ),
            (
                "out.webp",
                &["--use-rendered-icc", "--assume-srgb"],
                "--assume-srgb and --use-rendered-icc are mutually exclusive",
            ),
            (
                "out.webp",
                &["--webp-level=6", "--webp-quality=80"],
                "--webp-level and --webp-quality are mutually exclusive",
            ),
            (
                "out.webp",
                &["--webp-method=4", "--webp-lossless"],
                "--webp-lossless and --webp-method are mutually exclusive",
            ),
            (
                "out.jxl",
                &["--jxl-distance=1.5", "--jxl-lossless"],
                "--jxl-lossless and --jxl-distance are mutually exclusive",
            ),
        ] {
            assert_eq!(run(output, flags), message, "{flags:?}");
        }
        // An unknown flag stays unknown, even when repeated.
        assert!(
            run("out.webp", &["--fast", "--fast"]).starts_with("unknown option: --fast"),
            "repeated unknown flag"
        );
    }

    /// The `report:` line names both backends (issue #217), the color
    /// signal (issue #212) and what was discarded (ADR-0042 D2; nothing for
    /// this untagged JFIF file); keys are appended, so the existing ones
    /// keep their order.
    #[cfg(all(feature = "jpeg-read", feature = "webp-write"))]
    #[test]
    fn report_line_names_the_decoder_and_encoder_backends() {
        use crate::pipeline::{ImageFormat, InputProfilePolicy, OutputFormat, transcode};
        let output = transcode(
            include_bytes!("../tests/fixtures/sample.jpg"),
            ImageFormat::Jpeg,
            TransformOptions {
                output_format: OutputFormat::WebP,
                output_width: 8,
                output_height: 8,
                profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
                ..TransformOptions::default()
            },
        )
        .unwrap();
        let line = super::report_line(&output);
        assert!(line.starts_with("report: open_us="), "{line}");
        assert!(line.contains(" precision=f32 "), "{line}");
        assert!(
            line.ends_with(
                " encoder=libwebp-rs decoder=libjpeg-turbo-rs color_signal=assumed-srgb \
                 input_depth=8 depth=8 discarded=none"
            ),
            "{line}"
        );
    }

    /// Exit status 2 for a wrong command line, including the engine's
    /// `InvalidOptions`, and 1 for every other error, `Internal` from a
    /// caught panic included (since 2026-11-16).
    #[cfg(feature = "jpeg-read")]
    #[test]
    fn exit_status_separates_usage_from_processing_errors() {
        use super::{exit_status, failure_line, usage};
        use crate::pipeline::{
            EncodeError, EncodeRequest, EncoderBackend, ImageFormat, OutputFormat, RowEncoder,
            transcode, transcode_with,
        };
        struct Panics;
        impl EncoderBackend for Panics {
            fn format(&self) -> OutputFormat {
                OutputFormat::WebP
            }
            fn name(&self) -> &'static str {
                "test-panic"
            }
            fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
                panic!("test backend panics in begin")
            }
        }
        let jpeg = include_bytes!("../tests/fixtures/sample.jpg");
        let webp = TransformOptions {
            output_format: OutputFormat::WebP,
            output_width: 8,
            output_height: 8,
            ..TransformOptions::default()
        };
        assert_eq!(exit_status(&*usage("unknown option: --fast")), 2);
        let invalid = transcode(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                output_width: 0,
                ..webp
            },
        )
        .unwrap_err();
        assert_eq!(exit_status(&invalid), 2);
        assert_eq!(
            failure_line(&invalid),
            "failure: kind=invalid-options reason=none"
        );
        let refused = transcode(jpeg, ImageFormat::Jpeg, webp).unwrap_err();
        assert_eq!(exit_status(&refused), 1);
        assert_eq!(
            failure_line(&refused),
            "failure: kind=unsupported-color reason=color-signal-missing"
        );
        let internal = transcode_with(
            jpeg,
            ImageFormat::Jpeg,
            TransformOptions {
                profile_policy: crate::pipeline::InputProfilePolicy::AssumeSrgbIfMissing,
                ..webp
            },
            &Panics,
        )
        .unwrap_err();
        assert_eq!(exit_status(&internal), 1);
        assert_eq!(
            failure_line(&internal),
            "failure: kind=internal reason=none"
        );
        let io: Box<dyn std::error::Error> = "cannot read missing.jpg".into();
        assert_eq!(exit_status(&*io), 1);
    }

    /// The enum values of the `report:` and `decoder_work:` lines are the
    /// CLI's own names (since 2026-11-16): lower-case and hyphenated, not
    /// `Debug` and not the kernels crate's `Display`.
    #[test]
    fn report_value_names_are_owned_by_the_cli() {
        use super::{
            color_signal_name, discarded_name, kernel_level_name, precision_name, work_count_value,
        };
        use crate::pipeline::{ColorSignal, Discarded, KernelLevel, WorkCount, WorkingPrecision};
        assert_eq!(precision_name(WorkingPrecision::F32), "f32");
        assert_eq!(
            precision_name(WorkingPrecision::F64Reference),
            "f64-reference"
        );
        for (level, name) in [
            (KernelLevel::Scalar, "scalar"),
            (KernelLevel::Neon, "neon"),
            (KernelLevel::Sve2, "sve2"),
            (KernelLevel::Avx2, "avx2"),
            (KernelLevel::Avx512, "avx512"),
        ] {
            assert_eq!(kernel_level_name(level), name);
        }
        for (signal, name) in [
            (ColorSignal::Cicp, "cicp"),
            (ColorSignal::AssumedSrgb, "assumed-srgb"),
            (ColorSignal::RenderedIcc, "rendered-icc"),
            (
                ColorSignal::RenderedIccWithAgreeingCicp,
                "rendered-icc-with-agreeing-cicp",
            ),
        ] {
            assert_eq!(color_signal_name(signal), name);
        }
        for (class, name) in [
            (Discarded::GainMap, "gain-map"),
            (Discarded::Exif, "exif"),
            (Discarded::Xmp, "xmp"),
            (Discarded::OtherMetadata, "other-metadata"),
            (Discarded::IccProfile, "icc-profile"),
            (Discarded::UnknownContainerData, "unknown-container-data"),
            (
                Discarded::JpegReconstructionData,
                "jpeg-reconstruction-data",
            ),
            (Discarded::Preview, "preview"),
            (Discarded::HiddenColor, "hidden-color"),
            (Discarded::Unpremultiplied, "unpremultiplied"),
            (Discarded::PrecisionReduced, "precision-reduced"),
        ] {
            assert_eq!(discarded_name(class), name);
        }
        assert_eq!(work_count_value(WorkCount::Exact(7)), "7");
        assert_eq!(work_count_value(WorkCount::AtLeast(7)), ">=7");
    }

    /// Owner decision of 2026-11-15 (ADR-0043 D4): each output has its
    /// encoder's own flags on its own scale; the former
    /// format-independent `--quality=`, `--speed=` and `--chroma=` are
    /// unknown options that name the replacements.
    #[test]
    fn encoder_flags_are_per_format() {
        use crate::pipeline::{AvifOptions, ChromaSubsampling, OutputFormat, WebpOptions};
        let mut avif = TransformOptions::default();
        for flag in [
            "--avif-quality=70",
            "--avif-alpha-quality=90",
            "--avif-speed=8",
            "--avif-depth=10",
            "--avif-chroma=420",
        ] {
            apply_option(&mut avif, flag).unwrap();
        }
        assert_eq!(
            avif.avif,
            AvifOptions {
                quality: 70,
                alpha_quality: 90,
                speed: 8,
                depth: Some(10),
                chroma: ChromaSubsampling::Yuv420,
            }
        );
        let mut webp = TransformOptions {
            output_format: OutputFormat::WebP,
            ..TransformOptions::default()
        };
        apply_option(&mut webp, "--webp-quality=80").unwrap();
        apply_option(&mut webp, "--webp-method=6").unwrap();
        assert_eq!(
            webp.webp,
            WebpOptions::Lossy {
                quality: 80.0,
                method: 6
            }
        );
        let mut lossless = TransformOptions {
            output_format: OutputFormat::WebP,
            ..TransformOptions::default()
        };
        apply_option(&mut lossless, "--webp-level=9").unwrap();
        assert_eq!(lossless.webp, WebpOptions::Lossless { level: 9 });
        for flag in ["--webp-quality=80", "--webp-level=4", "--webp-lossless"] {
            let error = apply_option(&mut TransformOptions::default(), flag).unwrap_err();
            assert!(
                error.to_string().ends_with("requires .webp output"),
                "{error}"
            );
        }
        for flag in ["--quality=80", "--speed=6", "--chroma=420"] {
            let error = apply_option(&mut TransformOptions::default(), flag).unwrap_err();
            let message = error.to_string();
            assert!(
                message.starts_with(&format!("unknown option: {flag} (removed on 2026-11-15")),
                "{message}"
            );
            assert!(message.contains("--avif-quality="), "{message}");
        }
    }

    #[test]
    fn the_removed_backend_selections_are_unknown_options() {
        // One JPEG XL reader since 2026-11-15 (ADR-0032): the former
        // `--jxl-decoder=` selector is refused, not ignored.
        let mut options = TransformOptions::default();
        // So is the former `--avif-encoder=` (one AVIF writer since then).
        for flag in [
            "--jxl-decoder=libjxl-rs",
            "--jxl-decoder=jxl-oxide",
            "--avif-encoder=libavif",
            "--avif-encoder=rav1e",
        ] {
            let error = apply_option(&mut options, flag).unwrap_err();
            assert!(error.to_string().starts_with("unknown option"), "{error}");
        }
    }
}
