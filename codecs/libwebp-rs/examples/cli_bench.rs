//! A command-line counterpart of upstream `dwebp -ppm` and `cwebp` for the CI
//! performance comparison of the port with the pinned libwebp
//! (`scripts/bench/ci-perf/codec_webp.sh`).
//!
//! ```text
//! cli_bench decode <in.webp> <out.ppm> [--mt]
//! cli_bench encode <in.ppm> <out.webp> lossy|lossless [--mt]
//! ```
//!
//! `decode` reads the file into memory, decodes it to RGB into library memory
//! with the default options (fancy upsampling and the in-loop filter on), as
//! `dwebp -ppm` does, and writes a binary PPM with `dwebp`'s header. `encode`
//! reads a binary 8-bit PPM, imports it as `cwebp` does (YUV for lossy, ARGB
//! for lossless, `WebPPictureImportRGB`), and encodes it with the default
//! configuration (quality 75, method 4; `-lossless` for the lossless mode)
//! through a buffered file writer, as `cwebp` does, with statistics enabled
//! as `cwebp` without `-quiet` does. `--mt` sets one extra thread, as `-mt`
//! does on both tools.
//!
//! The program prints `elapsed_ms=<float>` for the timed region only, the same
//! region as the `-v` timers of the tools: `decode` (C: `DecodeWebP`, "Time to
//! decode picture") and `encode` (C: `WebPEncode` with the file writer, "Time
//! to encode picture"; the import happens before, as in `cwebp`'s read).

use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::ExitCode;
use std::time::Instant;

use webp::api::decode::{self, ColorMode};
use webp::api::encode::{self, Encoder, Picture, PixelLayout, PixelRows, SampleForm, WebPAuxStats};

type Error = Box<dyn std::error::Error>;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("cli_bench: {error}");
            eprintln!(
                "usage: cli_bench decode <in.webp> <out.ppm> [--mt]\n       \
                 cli_bench encode <in.ppm> <out.webp> lossy|lossless [--mt]"
            );
            ExitCode::FAILURE
        }
    }
}

fn run(args: &[String]) -> Result<(), Error> {
    let mt = args.iter().any(|a| a == "--mt");
    let rest: Vec<&str> = args
        .iter()
        .filter(|a| *a != "--mt")
        .map(String::as_str)
        .collect();
    match rest.as_slice() {
        ["decode", input, output] => decode_file(input, output, mt),
        ["encode", input, output, mode] => encode_file(input, output, mode, mt),
        _ => Err("invalid arguments".into()),
    }
}

fn decode_file(input: &str, output: &str, mt: bool) -> Result<(), Error> {
    let data = std::fs::read(input)?;
    let config = decode::Config::new(ColorMode::Rgb).multithreaded(mt);
    let start = Instant::now();
    let image = decode::decode(&data, &config)?;
    let elapsed = start.elapsed();
    let rows = image.rows().ok_or("the image has no packed rows")?;
    let mut out = BufWriter::new(File::create(output)?);
    write!(out, "P6\n{} {}\n255\n", image.width(), image.height())?;
    for row in rows.iter() {
        out.write_all(row)?;
    }
    out.flush()?;
    println!(
        "elapsed_ms={:.3} width={} height={}",
        elapsed.as_secs_f64() * 1000.0,
        image.width(),
        image.height()
    );
    Ok(())
}

/// Parses a binary PPM (P6, maxval 255) with optional comments in the header.
fn parse_ppm(data: &[u8]) -> Result<(u32, u32, &[u8]), Error> {
    let mut fields = Vec::new();
    let mut pos = 0;
    while fields.len() < 4 {
        while pos < data.len() && data[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if pos < data.len() && data[pos] == b'#' {
            while pos < data.len() && data[pos] != b'\n' {
                pos += 1;
            }
            continue;
        }
        let start = pos;
        while pos < data.len() && !data[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if start == pos {
            return Err("truncated PPM header".into());
        }
        fields.push(std::str::from_utf8(&data[start..pos])?);
    }
    // Exactly one whitespace byte separates the header from the samples.
    pos += 1;
    if fields[0] != "P6" || fields[3] != "255" {
        return Err("only binary 8-bit PPM (P6, maxval 255) is supported".into());
    }
    let width: u32 = fields[1].parse()?;
    let height: u32 = fields[2].parse()?;
    let len = usize::try_from(u64::from(width) * u64::from(height) * 3)?;
    let samples = data
        .get(pos..pos + len)
        .ok_or("the PPM ends before its samples")?;
    Ok((width, height, samples))
}

fn encode_file(input: &str, output: &str, mode: &str, mt: bool) -> Result<(), Error> {
    let lossless = match mode {
        "lossy" => false,
        "lossless" => true,
        _ => return Err(format!("unknown mode {mode}").into()),
    };
    let data = std::fs::read(input)?;
    let (width, height, samples) = parse_ppm(&data)?;
    let rows = PixelRows {
        layout: PixelLayout::Rgb,
        data: samples,
        width,
        height,
        stride: usize::try_from(width)? * 3,
    };
    // cwebp reads into ARGB only when the encoding needs it (lossless here).
    let form = if lossless {
        SampleForm::Argb
    } else {
        SampleForm::Yuva
    };
    let mut picture = Picture::import(rows, form)?;
    let config = encode::Config::new()
        .quality(75.0)
        .lossless(lossless)
        .multithreaded(mt);
    let mut stats = WebPAuxStats::default();
    let mut out = BufWriter::new(File::create(output)?);
    let mut write_failed = false;
    let start = Instant::now();
    Encoder::new(&config)
        .stats(&mut stats)
        .encode_to(&mut picture, |chunk| {
            let ok = out.write_all(chunk).is_ok();
            write_failed |= !ok;
            ok
        })?;
    out.flush()?;
    let elapsed = start.elapsed();
    if write_failed {
        return Err("cannot write the output".into());
    }
    println!(
        "elapsed_ms={:.3} bytes={}",
        elapsed.as_secs_f64() * 1000.0,
        stats.coded_size
    );
    Ok(())
}
