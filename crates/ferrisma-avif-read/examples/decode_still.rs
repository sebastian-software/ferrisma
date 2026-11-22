//! Decode one still AVIF through the safe reader, the way the engine does:
//! the workspace libavif port with its default AV1 decoder (the dav1d port
//! since 2026-11-22, the AOM decoder before), one thread, and nearest chroma
//! upsampling to 8-bit RGB. An optional third argument chooses the decoder
//! by name, `dav1d` or `libaom` (the engine's `--avif-decoder=`).
//!
//! ```sh
//! cargo run --release -p ferrisma-avif-read --example decode_still -- input.avif output.ppm [dav1d|libaom]
//! ```
//!
//! The input is read into memory first. `elapsed_ms` covers parsing, AV1
//! decoding and the YUV to RGB conversion; reading the input and writing the
//! binary PPM (P6) output are outside the timer. Only 8-bit images without
//! alpha are accepted, which is what the CI performance inputs are.
use std::time::Instant;

use ferrisma_avif_read::{Codec, Decoder};

/// The engine's default decoded-pixel limit (`max_input_pixels`).
const MAX_PIXELS: u64 = 100_000_000;

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let (input, output, codec) = match arguments.as_slice() {
        [input, output] => (input, output, Codec::default()),
        [input, output, codec] => (
            input,
            output,
            match codec.as_str() {
                "dav1d" => Codec::Dav1d,
                "libaom" => Codec::Libaom,
                _ => panic!("the decoder is dav1d or libaom"),
            },
        ),
        _ => panic!("usage: decode_still input.avif output.ppm [dav1d|libaom]"),
    };
    let bytes = std::fs::read(input).expect("read input");

    let start = Instant::now();
    let (mut decoder, info) = Decoder::parse_with(&bytes, MAX_PIXELS, codec).expect("parse");
    assert!(
        info.depth == 8 && !info.alpha,
        "decode_still expects 8-bit AVIF without alpha"
    );
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
        "elapsed_ms={:.3} width={} height={} rgb_bytes={}",
        elapsed.as_secs_f64() * 1000.0,
        info.width,
        info.height,
        pixels.len()
    );
}
