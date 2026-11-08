//! The port's decode speed, for `tools/bench.py` (ADR 0004).
//!
//! Usage: `bench [OPTION]... FILE`. Reads FILE into memory and decodes it
//! REPEATS times through the safe API: `Decoder::new`, `Decoder::start` with
//! the options, `Scanlines::read_rows` into one buffer for the whole image
//! and `Scanlines::finish`, as `tools/oracle/probes/wp9/bench.c` decodes
//! through the C library. Options:
//!
//! - `-repeats N`: decodes to time (default 10), after one untimed decode
//! - `-format F`: the output format (`rgb`, `rgbx`, `bgr`, `bgrx`, `xbgr`,
//!   `xrgb`, `rgba`, `bgra`, `abgr`, `argb`, `gray`, `ycbcr`, `cmyk`)
//! - `-scale N D`: scale by N/D
//! - `-dct M`: `islow`, `ifast` or `float`
//! - `-nofancy`: no fancy upsampling
//!
//! Prints `dims=`, `decodes=`, `nanoseconds=` and `checksum=` as the probe
//! does.

use std::process::ExitCode;
use std::time::Instant;

use jpeg::{DctMethod, Decoder, OutputOptions, PixelFormat, Scale};

/// Parses a format name.
fn format(name: &str) -> Option<PixelFormat> {
    Some(match name {
        "rgb" => PixelFormat::Rgb,
        "rgbx" => PixelFormat::Rgbx,
        "bgr" => PixelFormat::Bgr,
        "bgrx" => PixelFormat::Bgrx,
        "xbgr" => PixelFormat::Xbgr,
        "xrgb" => PixelFormat::Xrgb,
        "rgba" => PixelFormat::Rgba,
        "bgra" => PixelFormat::Bgra,
        "abgr" => PixelFormat::Abgr,
        "argb" => PixelFormat::Argb,
        "gray" => PixelFormat::Grayscale,
        "ycbcr" => PixelFormat::YCbCr,
        "cmyk" => PixelFormat::Cmyk,
        _ => return None,
    })
}

/// Decodes `data` once into `buffer` and returns the output's width, height
/// and components.
fn decode(
    data: &[u8],
    options: &OutputOptions,
    buffer: &mut Vec<u8>,
) -> Result<(u32, u32, u32), jpeg::Error> {
    let mut scanlines = Decoder::new(data)?.start(options)?;
    let info = *scanlines.info();
    buffer.resize(info.row_bytes() * info.height as usize, 0);
    let mut done = 0;
    while done < buffer.len() {
        let rows = scanlines.read_rows(&mut buffer[done..])?;
        done += rows as usize * info.row_bytes();
    }
    scanlines.finish()?;
    Ok((info.width, info.height, info.components))
}

fn main() -> ExitCode {
    let mut repeats = 10u32;
    let mut options = OutputOptions::default();
    let mut path = None;
    let args: Vec<String> = std::env::args().skip(1).collect();
    let value = |i: usize| args.get(i).map_or("", String::as_str);
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        match arg {
            "-repeats" => {
                repeats = value(i + 1).parse().unwrap_or(0);
                i += 1;
            }
            "-format" => {
                let Some(format) = format(value(i + 1)) else {
                    eprintln!("bench: unknown format {}", value(i + 1));
                    return ExitCode::from(2);
                };
                options.format = Some(format);
                i += 1;
            }
            "-scale" => {
                let num = value(i + 1).parse().unwrap_or(0);
                let denom = value(i + 2).parse().unwrap_or(0);
                options.scale = Scale { num, denom };
                i += 2;
            }
            "-dct" => {
                options.dct_method = match value(i + 1) {
                    "ifast" => DctMethod::IntFast,
                    "float" => DctMethod::Float,
                    _ => DctMethod::IntSlow,
                };
                i += 1;
            }
            "-nofancy" => options.fancy_upsampling = false,
            _ if !arg.starts_with('-') && path.is_none() => path = Some(arg.to_owned()),
            _ => {
                eprintln!("bench: unknown option {arg}");
                return ExitCode::from(2);
            }
        }
        i += 1;
    }
    let Some(path) = path.filter(|_| repeats > 0) else {
        eprintln!("usage: bench [-repeats N] [-format F] [-scale N D] [-dct M] [-nofancy] FILE");
        return ExitCode::from(2);
    };
    let data = match std::fs::read(&path) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("bench: {path}: {error}");
            return ExitCode::from(2);
        }
    };

    let mut buffer = Vec::new();
    let run = |buffer: &mut Vec<u8>| match decode(&data, &options, buffer) {
        Ok(dims) => Some(dims),
        Err(error) => {
            eprintln!("bench: {path}: {error}");
            None
        }
    };
    if run(&mut buffer).is_none() {
        return ExitCode::FAILURE;
    }
    let start = Instant::now();
    let mut dims = (0, 0, 0);
    for _ in 0..repeats {
        match run(&mut buffer) {
            Some(result) => dims = result,
            None => return ExitCode::FAILURE,
        }
    }
    let elapsed = start.elapsed();

    let mut hash: u64 = 1_469_598_103_934_665_603;
    for &byte in &buffer {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(1_099_511_628_211);
    }
    println!("dims={},{},{}", dims.0, dims.1, dims.2);
    println!("decodes={repeats}");
    println!("nanoseconds={}", elapsed.as_nanos());
    println!("checksum={hash:016x}");
    ExitCode::SUCCESS
}
