//! Minimal M1 CLI, using exactly the same graph and executor as the Rust API.

use std::env;
use std::error::Error;
use std::path::Path;

use ferrixel::pipeline::{
    Crop, ImageFormat, InputProfilePolicy, ResizeFilter, TransformOptions, WorkingPrecision,
    transcode,
};

fn parse_format(path: &Path) -> Result<ImageFormat, Box<dyn Error>> {
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    match extension.as_str() {
        "jpg" | "jpeg" => Ok(ImageFormat::Jpeg),
        "jxl" => Ok(ImageFormat::JpegXl),
        "avif" => Ok(ImageFormat::Avif),
        _ => Err("unsupported file extension; expected JPEG, JXL or AVIF".into()),
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

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!(
            "usage: ferrixel input.jpg output.avif WIDTH HEIGHT [--assume-srgb|--use-rendered-icc] [--area] [--reference] [--quality=75] [--speed=6] [--budget-mb=256] [--max-input-mb=64] [--max-input-mpx=100] [--crop=x,y,w,h]"
        );
        std::process::exit(2);
    }
    let input_path = Path::new(&args[1]);
    let output_path = Path::new(&args[2]);
    if output_path.extension().and_then(|ext| ext.to_str()) != Some("avif") {
        return Err("output must have a .avif extension".into());
    }
    let mut options = TransformOptions {
        output_width: args[3].parse()?,
        output_height: args[4].parse()?,
        ..TransformOptions::default()
    };
    for option in args.iter().skip(5) {
        match option.as_str() {
            "--assume-srgb" => options.profile_policy = InputProfilePolicy::AssumeSrgbIfMissing,
            "--use-rendered-icc" => options.profile_policy = InputProfilePolicy::UseRenderedIcc,
            "--area" => options.filter = ResizeFilter::Area,
            "--reference" => options.precision = WorkingPrecision::F64Reference,
            option if option.starts_with("--quality=") => options.quality = option[10..].parse()?,
            option if option.starts_with("--speed=") => options.speed = option[8..].parse()?,
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
    }
    let format = parse_format(input_path)?;
    let data = std::fs::read(input_path)?;
    let output = transcode(&data, format, options)?;
    std::fs::write(output_path, &output.avif_bytes)?;
    println!(
        "Encoded {} bytes. Decoder requests: {}, known decoded full frames: {}, \
         peak engine-managed memory: {} bytes, encoder materialization: {} bytes.",
        output.avif_bytes.len(),
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
         decoder_requests={} decoder_full_frames={} precision={:?}",
        timings.open.as_micros(),
        timings.decode.as_micros(),
        timings.transform.as_micros(),
        timings.encode.as_micros(),
        timings.total.as_micros(),
        output.avif_bytes.len(),
        output.report.peak_managed_bytes,
        output.report.encoder_output_frame_bytes,
        output.report.orientation_buffer_bytes,
        output.report.decoder_requests,
        output.report.decoder_full_frame_buffers,
        output.report.working_precision,
    );
    if output.report.opaque_decoder_memory || output.report.opaque_encoder_memory {
        eprintln!(
            "Note: decoder and/or encoder internal allocations are not subject to this memory budget."
        );
    }
    Ok(())
}
