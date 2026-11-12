//! Decode one still AVIF through the safe reader, the way the engine does:
//! the workspace libavif port with its AOM decoder, one thread, and nearest
//! chroma upsampling to 8-bit RGB.
//!
//! ```sh
//! cargo run --release -p ferrisma-avif-read --example decode_still -- input.avif output.ppm
//! ```
//!
//! The input is read into memory first. `elapsed_ms` covers parsing, AV1
//! decoding and the YUV to RGB conversion; reading the input and writing the
//! binary PPM (P6) output are outside the timer. Only 8-bit images without
//! alpha are accepted, which is what the CI performance inputs are.
use std::time::Instant;

use ferrisma_avif_read::Decoder;

/// The engine's default decoded-pixel limit (`max_input_pixels`).
const MAX_PIXELS: u64 = 100_000_000;

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [input, output] = arguments.as_slice() else {
        panic!("usage: decode_still input.avif output.ppm");
    };
    let bytes = std::fs::read(input).expect("read input");

    let start = Instant::now();
    let (mut decoder, info) = Decoder::parse(&bytes, MAX_PIXELS).expect("parse");
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
