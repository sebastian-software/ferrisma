//! The command-line front end, using exactly the same graph and executor as
//! the Rust API. It lives in the library so that encoder adapters outside
//! this crate (ADR-0024) can ship the same command with their backend.

use std::error::Error;
use std::io::{self, Read};
use std::path::Path;
use std::process::ExitCode;

use crate::pipeline::{
    AvifEncoder, ChromaSubsampling, Crop, EncoderBackend, ImageFormat, InputProfilePolicy,
    JpegXlDecoder, JpegXlOptions, KernelLevel, OutputFormat, ResizeFilter, TransformOptions,
    WebpOptions, WorkingPrecision, transcode, transcode_with,
};

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
        _ => Err("unsupported file extension; expected JPEG, WebP, JXL or AVIF".into()),
    }
}

fn parse_crop(value: &str) -> Result<Crop, Box<dyn Error>> {
    let parts: Vec<&str> = value.split(',').collect();
    if parts.len() != 4 {
        return Err("crop must be x,y,width,height".into());
    }
    Ok(Crop {
        x: parts[0].parse()?,
        y: parts[1].parse()?,
        width: parts[2].parse()?,
        height: parts[3].parse()?,
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
        "--jxl-decoder=jxl-oxide" => options.jxl_decoder = JpegXlDecoder::JxlOxide,
        "--jxl-decoder=libjxl-rs" => options.jxl_decoder = JpegXlDecoder::LibjxlRs,
        option if option.starts_with("--quality=") => match options.output_format {
            OutputFormat::Avif => options.quality = option[10..].parse()?,
            OutputFormat::WebP => match options.webp {
                WebpOptions::Lossy { speed, .. } => {
                    options.webp = WebpOptions::Lossy {
                        quality: option[10..].parse()?,
                        speed,
                    }
                }
                WebpOptions::Lossless { .. } => {
                    return Err("--quality is unsupported for lossless WebP".into());
                }
            },
            OutputFormat::JpegXl => {
                return Err(
                    "--quality is unsupported for JPEG XL; use --jxl-distance or --jxl-lossless"
                        .into(),
                );
            }
        },
        option if option.starts_with("--speed=") => match options.output_format {
            OutputFormat::Avif => options.speed = option[8..].parse()?,
            OutputFormat::WebP => match options.webp {
                WebpOptions::Lossy { quality, .. } => {
                    options.webp = WebpOptions::Lossy {
                        quality,
                        speed: option[8..].parse()?,
                    }
                }
                WebpOptions::Lossless { .. } => {
                    return Err("--speed is unsupported for lossless WebP; use --webp-level".into());
                }
            },
            OutputFormat::JpegXl => {
                return Err(
                    "--speed is unsupported for JPEG XL; the admitted efforts are fixed \
                     (7 lossy, 2 lossless)"
                        .into(),
                );
            }
        },
        "--kernel-level=scalar" => options.kernel_level_ceiling = Some(KernelLevel::Scalar),
        "--kernel-level=neon" => options.kernel_level_ceiling = Some(KernelLevel::Neon),
        "--kernel-level=avx2" => options.kernel_level_ceiling = Some(KernelLevel::Avx2),
        "--kernel-level=avx512" => options.kernel_level_ceiling = Some(KernelLevel::Avx512),
        "--chroma=444" | "--chroma=420" if options.output_format == OutputFormat::WebP => {
            return Err("--chroma is unsupported for WebP".into());
        }
        "--chroma=444" | "--chroma=420" if options.output_format == OutputFormat::JpegXl => {
            return Err("--chroma is unsupported for JPEG XL".into());
        }
        "--chroma=444" => options.chroma_subsampling = ChromaSubsampling::Yuv444,
        "--chroma=420" => options.chroma_subsampling = ChromaSubsampling::Yuv420,
        "--webp-lossless" if options.output_format == OutputFormat::WebP => {
            if !matches!(options.webp, WebpOptions::Lossless { .. }) {
                options.webp = WebpOptions::Lossless { level: 4 };
            }
        }
        "--webp-lossless" => return Err("--webp-lossless requires .webp output".into()),
        "--jxl-lossless" if options.output_format == OutputFormat::JpegXl => {
            options.jpeg_xl = JpegXlOptions::Lossless { effort: 2 };
        }
        "--jxl-lossless" => return Err("--jxl-lossless requires .jxl output".into()),
        option if option.starts_with("--jxl-distance=") => {
            if options.output_format != OutputFormat::JpegXl {
                return Err("--jxl-distance requires .jxl output".into());
            }
            // The range is checked by the transcode, before decoding.
            options.jpeg_xl = JpegXlOptions::Lossy {
                distance: option[15..].parse()?,
            };
        }
        option
            if option.starts_with("--webp-level=")
                && options.output_format == OutputFormat::WebP =>
        {
            options.webp = WebpOptions::Lossless {
                level: option[13..].parse()?,
            }
        }
        option if option.starts_with("--max-output-mb=") => {
            let megabytes: usize = option[16..].parse()?;
            options.max_output_bytes = megabytes
                .checked_mul(1024 * 1024)
                .ok_or("encoded output limit overflow")?;
        }
        option if option.starts_with("--transform-threads=") => {
            options.transform_threads = option[20..].parse()?;
        }
        option if option.starts_with("--encoder-threads=") => {
            options.encoder_threads = option[18..].parse()?;
        }
        option if option.starts_with("--budget-mb=") => {
            let megabytes: usize = option[12..].parse()?;
            options.max_managed_bytes = megabytes
                .checked_mul(1024 * 1024)
                .ok_or("memory budget overflow")?;
        }
        option if option.starts_with("--max-input-mb=") => {
            let megabytes: usize = option[15..].parse()?;
            options.max_input_bytes = megabytes
                .checked_mul(1024 * 1024)
                .ok_or("compressed input limit overflow")?;
        }
        option if option.starts_with("--max-input-mpx=") => {
            let megapixels: u64 = option[16..].parse()?;
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

/// Run the command with `args` (the program name first), encoding with
/// `backend`, or with the selected built-in sink when it is `None`. Explicit
/// `--avif-encoder` or `--avif-depth` flags cannot accompany an external
/// backend: its identity and depth are not available through the trait.
///
/// # Errors
/// Invalid arguments, unreadable input, unwritable output and every
/// transcode error.
#[allow(clippy::too_many_lines)] // Argument handling and two report lines.
pub fn run(
    args: &[String],
    backend: Option<&dyn EncoderBackend>,
) -> Result<ExitCode, Box<dyn Error>> {
    if args.len() < 5 {
        let program = args
            .first()
            .and_then(|path| Path::new(path).file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("ferrisma");
        eprintln!(
            "usage: {program} input.jpg|input.webp|input.jxl|input.avif output.avif|output.webp|output.jxl WIDTH HEIGHT [--webp-lossless --webp-level=4] [--jxl-distance=1.0 (0.05 to below 3.0)|--jxl-lossless] [--assume-srgb|--use-rendered-icc] [--jxl-decoder=libjxl-rs (default)|jxl-oxide] [--area] [--reference] [--avif-encoder=libavif (default with avif-own-write)|rav1e] [--avif-depth=8|10 (libavif, default 10)] [--quality=75 (selected encoder's own scale; rav1e 1-100, libavif 0-100, not comparable)] [--speed=6 (rav1e 1-10, libavif 0-10)] [--encoder-threads=1] [--transform-threads=0] [--chroma=444|420 (AVIF)] [--max-output-mb=256] [--budget-mb=256] [--max-input-mb=64] [--max-input-mpx=100] [--crop=x,y,w,h]"
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
        _ => return Err("output must have a .avif, .webp or .jxl extension".into()),
    };
    let mut options = TransformOptions {
        output_format,
        output_width: args[3].parse()?,
        output_height: args[4].parse()?,
        ..TransformOptions::default()
    };
    if output_extension == Some("webp")
        && args
            .iter()
            .skip(5)
            .any(|arg| arg == "--webp-lossless" || arg.starts_with("--webp-level="))
        && args
            .iter()
            .skip(5)
            .any(|arg| arg.starts_with("--quality=") || arg.starts_with("--speed="))
    {
        return Err(
            "--quality and --speed are unsupported for lossless WebP; use --webp-level".into(),
        );
    }
    if args.iter().skip(5).any(|arg| arg == "--jxl-lossless")
        && args
            .iter()
            .skip(5)
            .any(|arg| arg.starts_with("--jxl-distance="))
    {
        return Err("--jxl-distance and --jxl-lossless are mutually exclusive".into());
    }
    let mut avif_encoder = None;
    let mut avif_depth = None;
    for option in args.iter().skip(5) {
        if let Some(value) = option.strip_prefix("--avif-encoder=") {
            if avif_encoder.replace(value).is_some() {
                return Err("--avif-encoder may only be set once".into());
            }
        } else if let Some(value) = option.strip_prefix("--avif-depth=") {
            if avif_depth.replace(value.parse::<u8>()?).is_some() {
                return Err("--avif-depth may only be set once".into());
            }
        } else {
            apply_option(&mut options, option)?;
        }
    }
    if backend.is_some() && (avif_encoder.is_some() || avif_depth.is_some()) {
        return Err("AVIF encoder options cannot be combined with an external backend".into());
    }
    if output_extension != Some("avif") && (avif_encoder.is_some() || avif_depth.is_some()) {
        return Err("AVIF encoder options require .avif output".into());
    }
    // Without a selector the build's default encoder applies (libavif when
    // `avif-own-write` is compiled in). `--quality`/`--speed` are passed to
    // the selected encoder unchanged, on its own scale.
    let selected = match avif_encoder {
        None => AvifEncoder::default(),
        Some("rav1e") => AvifEncoder::Rav1e,
        Some("libavif") => AvifEncoder::Libavif { depth: 10 },
        Some(_) => return Err("--avif-encoder must be rav1e or libavif".into()),
    };
    options.avif_encoder = match (selected, avif_depth) {
        (encoder, None) => encoder,
        (AvifEncoder::Libavif { .. }, Some(depth)) => AvifEncoder::Libavif { depth },
        (_, Some(_)) => {
            return Err(
                "--avif-depth requires the libavif encoder (--avif-encoder=libavif)".into(),
            );
        }
    };
    let format = parse_format(input_path)?;
    let data = read_input(std::fs::File::open(input_path)?, options.max_input_bytes)?;
    let output = match backend {
        None => transcode(&data, format, options)?,
        Some(backend) => transcode_with(&data, format, options, backend)?,
    };
    std::fs::write(output_path, &output.bytes)?;
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
    // One stable key=value line for benchmark scripts; values are integers.
    println!(
        "report: open_us={} decode_us={} transform_us={} encode_us={} total_us={} \
         output_bytes={} peak_managed_bytes={} encoder_frame_bytes={} orientation_buffer_bytes={} \
         decoder_requests={} decoder_full_frames={} precision={:?} encoder_threads={} kernel_level={} transform_threads={} \
         encoder={}",
        timings.open.as_micros(),
        timings.decode.as_micros(),
        timings.transform.as_micros(),
        timings.encode.as_micros(),
        timings.total.as_micros(),
        output.bytes.len(),
        output.report.peak_managed_bytes,
        output.report.encoder_output_frame_bytes,
        output.report.orientation_buffer_bytes,
        output.report.decoder_requests,
        output.report.decoder_full_frame_buffers,
        output.report.working_precision,
        output.report.encoder_threads,
        output.report.kernel_level,
        output.report.transform_threads,
        output.report.encoder_backend,
    );
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
        work.reconstructed_pixels,
        work.entropy_rows,
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

#[cfg(test)]
mod tests {
    use super::{apply_option, read_input};
    use crate::pipeline::{JpegXlDecoder, TransformOptions};
    use std::io::{self, Cursor, Read};

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

    #[test]
    fn selects_jpeg_xl_output_settings_and_rejects_foreign_ones() {
        use crate::pipeline::{JpegXlOptions, OutputFormat};
        let jxl = TransformOptions {
            output_format: OutputFormat::JpegXl,
            ..TransformOptions::default()
        };
        let mut options = jxl;
        assert_eq!(options.jpeg_xl, JpegXlOptions::Lossy { distance: 1.0 });
        apply_option(&mut options, "--jxl-distance=0.5").unwrap();
        assert_eq!(options.jpeg_xl, JpegXlOptions::Lossy { distance: 0.5 });
        assert!(apply_option(&mut options, "--jxl-distance=abc").is_err());
        let mut options = jxl;
        apply_option(&mut options, "--jxl-lossless").unwrap();
        assert_eq!(options.jpeg_xl, JpegXlOptions::Lossless { effort: 2 });
        for flag in ["--quality=80", "--speed=4", "--chroma=420", "--chroma=444"] {
            let mut options = jxl;
            let error = apply_option(&mut options, flag).unwrap_err();
            assert!(error.to_string().contains("JPEG XL"), "{flag}: {error}");
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
        assert!(error.to_string().contains("mutually exclusive"), "{error}");
    }

    #[test]
    fn selects_jpeg_xl_decoder_explicitly() {
        let mut options = TransformOptions::default();
        // The workspace reader is the default whenever it is compiled in;
        // jxl-oxide is the default only when it is the sole JPEG XL backend.
        #[cfg(feature = "jpeg-xl-own-read")]
        assert_eq!(options.jxl_decoder, JpegXlDecoder::LibjxlRs);
        #[cfg(all(feature = "jpeg-xl-read", not(feature = "jpeg-xl-own-read")))]
        assert_eq!(options.jxl_decoder, JpegXlDecoder::JxlOxide);
        apply_option(&mut options, "--jxl-decoder=libjxl-rs").unwrap();
        assert_eq!(options.jxl_decoder, JpegXlDecoder::LibjxlRs);
        apply_option(&mut options, "--jxl-decoder=jxl-oxide").unwrap();
        assert_eq!(options.jxl_decoder, JpegXlDecoder::JxlOxide);
        assert!(apply_option(&mut options, "--jxl-decoder=unknown").is_err());
        // Without any JPEG XL backend the default names the workspace reader,
        // whose missing feature is then reported.
        #[cfg(not(any(feature = "jpeg-xl-read", feature = "jpeg-xl-own-read")))]
        assert_eq!(
            TransformOptions::default().jxl_decoder,
            JpegXlDecoder::LibjxlRs
        );
    }
}
