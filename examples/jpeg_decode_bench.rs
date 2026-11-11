//! Compare the engine's workspace JPEG decoder with the retained zune harness.
//! Usage: `cargo run --release --features jpeg-read --example jpeg_decode_bench -- FILE REPS`

use std::time::Instant;

use jpeg::decode::{Decoder, OutputOptions, PixelFormat};

fn decode(bytes: &[u8]) -> (Vec<u8>, u32, u32) {
    let options = OutputOptions {
        format: Some(PixelFormat::Rgb),
        ..OutputOptions::default()
    };
    let decoder = Decoder::new(bytes).expect("JPEG header");
    let mut scanlines = decoder.start(&options).expect("start JPEG decode");
    let info = *scanlines.info();
    assert_eq!(info.components, 3);
    let mut pixels = vec![0; info.row_bytes() * info.height as usize];
    for row in pixels.chunks_exact_mut(info.row_bytes()) {
        assert_eq!(scanlines.read_rows(row).expect("decode JPEG row"), 1);
    }
    scanlines.finish().expect("finish JPEG decode");
    (pixels, info.width, info.height)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 2, "usage: jpeg_decode_bench FILE REPS");
    let bytes = std::fs::read(&args[0]).expect("read JPEG");
    let reps: usize = args[1].parse().expect("repetitions");
    assert!(reps > 0);
    let mut times = Vec::with_capacity(reps);
    let mut last = None;
    for _ in 0..reps {
        drop(last.take());
        let start = Instant::now();
        let decoded = decode(&bytes);
        times.push(start.elapsed().as_secs_f64() * 1e3);
        last = Some(decoded);
    }
    times.sort_by(f64::total_cmp);
    let (pixels, width, height) = last.unwrap();
    std::hint::black_box(&pixels);
    println!(
        "libjpeg-turbo-rs\t{}\t1/1\t{width}x{height}\tu8\tbest={:.2}\tmedian={:.2}",
        args[0],
        times[0],
        times[reps / 2]
    );
}
