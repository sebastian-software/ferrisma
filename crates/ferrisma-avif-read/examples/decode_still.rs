//! Decode one still AVIF through the safe reader, the way the engine does:
//! the workspace libavif port with its default AV1 decoder (the dav1d port
//! since 2026-11-22, the AOM decoder before), one thread, and nearest chroma
//! upsampling to 8-bit RGB. An optional third argument chooses the decoder
//! by name, `dav1d` or `libaom` (the engine's `--avif-decoder=`), an
//! optional fourth the thread count (the engine's `threads`, default 1;
//! see `Decoder::set_threads` for what each decoder does with it).
//!
//! ```sh
//! cargo run --release -p ferrisma-avif-read --example decode_still -- input.avif output.ppm [dav1d|libaom] [threads]
//! ```
//!
//! The input is read into memory first. `elapsed_ms` covers parsing, AV1
//! decoding and the YUV to RGB conversion; reading the input and writing the
//! binary PPM (P6) output are outside the timer. Images without alpha are
//! accepted at any depth and written as 8-bit RGB, which is what the CI
//! performance inputs are.
use std::time::Instant;

use ferrisma_avif_read::{Codec, Decoder};

/// The engine's default decoded-pixel limit (`max_input_pixels`).
const MAX_PIXELS: u64 = 100_000_000;

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let codec = |name: &str| match name {
        "dav1d" => Codec::Dav1d,
        "libaom" => Codec::Libaom,
        _ => panic!("the decoder is dav1d or libaom"),
    };
    let threads = |count: &str| -> u32 { count.parse().expect("threads is a number") };
    let (input, output, codec, threads) = match arguments.as_slice() {
        [input, output] => (input, output, Codec::default(), 1),
        [input, output, name] => (input, output, codec(name), 1),
        [input, output, name, count] => (input, output, codec(name), threads(count)),
        _ => panic!("usage: decode_still input.avif output.ppm [dav1d|libaom] [threads]"),
    };
    let bytes = std::fs::read(input).expect("read input");

    let start = Instant::now();
    let (mut decoder, info) = Decoder::parse_with(&bytes, MAX_PIXELS, codec).expect("parse");
    assert!(!info.alpha, "decode_still expects AVIF without alpha");
    decoder.set_threads(threads);
    let mut pixels = vec![0_u8; info.width as usize * info.height as usize * 3];
    decoder
        .decode_u8(&mut pixels, info.width, info.height, false)
        .expect("decode");
    let elapsed = start.elapsed();
    drop(decoder);

    let mut ppm = format!("P6\n{} {}\n255\n", info.width, info.height).into_bytes();
    ppm.extend_from_slice(&pixels);
    std::fs::write(output, ppm).expect("write output");
    println!(
        "elapsed_ms={:.3} width={} height={} rgb_bytes={} threads={threads}",
        elapsed.as_secs_f64() * 1000.0,
        info.width,
        info.height,
        pixels.len()
    );
}
