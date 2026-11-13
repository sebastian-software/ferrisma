// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Lossless effort-2 encode of a binary 8-bit RGB PPM through the port's
//! fixed Thunder configuration, for the CI performance comparison
//! (`scripts/bench/ci-perf/codec_jxl.sh`).
//!
//! ```text
//! encode_lossless input.ppm output.jxl
//! ```
//!
//! `LosslessThunderConfig` is the original's SetLossless, Thunder (effort 2),
//! codestream level 10, buffering 0 one-shot path with sRGB metadata, the
//! same profile the engine's lossless JPEG XL writer uses (ADR-0039). Its C
//! counterpart is `cjxl -d 0 -e 2 --codestream_level=10 --buffering=0`. The
//! port runs serially, so compare it with `cjxl --num_threads=0`.
//!
//! The timer covers configuration and `encode_pixels`; reading and parsing
//! the PPM and writing the file are excluded, like cjxl's timer around
//! `EncodeImageJXL`.

use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use std::process::ExitCode;
use std::time::Instant;

/// Parses a binary P6 PPM with maxval 255 into (width, height, pixels).
fn read_ppm(bytes: &[u8]) -> Result<(usize, usize, &[u8]), String> {
    let mut pos = 0;
    let mut fields = Vec::new();
    while fields.len() < 4 {
        while pos < bytes.len() && (bytes[pos].is_ascii_whitespace() || bytes[pos] == b'#') {
            if bytes[pos] == b'#' {
                while pos < bytes.len() && bytes[pos] != b'\n' {
                    pos += 1;
                }
            } else {
                pos += 1;
            }
        }
        let start = pos;
        while pos < bytes.len() && !bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        fields.push(std::str::from_utf8(&bytes[start..pos]).map_err(|e| e.to_string())?);
    }
    if fields[0] != "P6" || fields[3] != "255" {
        return Err("only binary 8-bit P6 PPM input is supported".into());
    }
    let width: usize = fields[1].parse().map_err(|_| "bad PPM width")?;
    let height: usize = fields[2].parse().map_err(|_| "bad PPM height")?;
    let pixels = bytes
        .get(pos + 1..pos + 1 + width * height * 3)
        .ok_or("truncated PPM raster")?;
    Ok((width, height, pixels))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: encode_lossless input.ppm output.jxl");
        return ExitCode::from(2);
    }
    let bytes = std::fs::read(&args[1]).expect("read input");
    let (width, height, pixels) = match read_ppm(&bytes) {
        Ok(parsed) => parsed,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    let begin = Instant::now();
    let encoded = LosslessThunderConfig::new(width, height, false, LosslessSampleDepth::Uint8)
        .and_then(|mut config| config.encode_pixels(pixels, None));
    let elapsed = begin.elapsed();
    let encoded = match encoded {
        Ok(encoded) => encoded,
        Err(status) => {
            eprintln!("encode failed: {status:?}");
            return ExitCode::FAILURE;
        }
    };
    std::fs::write(&args[2], encoded.as_slice()).expect("write output");
    println!(
        "dimensions={width}x{height} effort=2 distance=0 bytes={} elapsed_ms={:.3}",
        encoded.as_slice().len(),
        elapsed.as_secs_f64() * 1000.0
    );
    ExitCode::SUCCESS
}
