//! Reproducible AVIF encoder frame-memory and throughput benchmark (issue #60).
//!
//! Runs one synthetic still-image encode through `ferrisma-avif-write` so
//! `/usr/bin/time -l` observes peak process RSS without extra source buffers.
//!
//! Usage:
//! Build the release binary first so `/usr/bin/time -l` measures only the
//! benchmark process (not `cargo`):
//! `cargo build --release -p ferrisma-avif-write --example avif_memory_bench`
//! `/usr/bin/time -l target/release/examples/avif_memory_bench \
//!     <width> <height> <depth:8|10|12> <chroma:444|420> <alpha:opaque|alpha> <threads> <speed>`
//! Or run the tall-narrow `16x8192` conversion-band check (1T vs 8T):
//! `target/release/examples/avif_memory_bench tall-narrow [reps]`

use std::env;
use std::time::Instant;

use ferrisma_avif_write::{Chroma, Config, Frame, required_frame_bytes};

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

fn fill_row_u8(y: u32, width: u32, height: u32, has_alpha: bool, out: &mut [u8]) {
    for x in 0..width {
        let hash = x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503);
        let noise = ((hash >> 13) & 0x1f) as u8;
        let idx = x as usize * 4;
        out[idx] = ((x * 255) / width.max(2)) as u8 ^ noise;
        out[idx + 1] = ((y * 255) / height.max(2)) as u8 ^ (noise >> 1);
        out[idx + 2] = (((x + y) * 255) / (width + height).max(2)) as u8 ^ (noise << 1);
        out[idx + 3] = if has_alpha {
            ((x * 13 + y * 5) % 256) as u8
        } else {
            255
        };
    }
}

fn fill_row_u16(y: u32, width: u32, height: u32, depth: u8, has_alpha: bool, out: &mut [u16]) {
    let max = (1_u32 << depth) - 1;
    for x in 0..width {
        let hash = x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503);
        let noise = (hash >> 13) & 0x3f;
        let idx = x as usize * 4;
        out[idx] = (((x * max) / width.max(2)) ^ noise) as u16;
        out[idx + 1] = ((y * max) / height.max(2)) as u16;
        out[idx + 2] = (((x + y) * 37) & max) as u16;
        out[idx + 3] = if has_alpha {
            ((x * 13 * (max / 255) + y * 5) % (max + 1)) as u16
        } else {
            max as u16
        };
    }
}

fn run_once(config: Config) -> (usize, f64, f64, f64, usize, u64) {
    let required = required_frame_bytes(&config).expect("valid config");
    let total_start = Instant::now();
    let mut frame = Frame::new(config).expect("Frame::new");
    let feed_start = Instant::now();
    if config.depth > 8 {
        let mut row = vec![0_u16; config.width as usize * 4];
        for y in 0..config.height {
            fill_row_u16(
                y,
                config.width,
                config.height,
                config.depth,
                config.has_alpha,
                &mut row,
            );
            frame.write_row_u16(y, &row).expect("write_row_u16");
        }
    } else {
        let mut row = vec![0_u8; config.width as usize * 4];
        for y in 0..config.height {
            fill_row_u8(y, config.width, config.height, config.has_alpha, &mut row);
            frame.write_row(y, &row).expect("write_row");
        }
    }
    let feed_ms = feed_start.elapsed().as_secs_f64() * 1e3;
    let finish_start = Instant::now();
    let encoded = frame.finish().expect("finish");
    let finish_ms = finish_start.elapsed().as_secs_f64() * 1e3;
    let total_ms = total_start.elapsed().as_secs_f64() * 1e3;
    (
        required,
        feed_ms,
        finish_ms,
        total_ms,
        encoded.len(),
        fnv1a(&encoded),
    )
}

fn bench_tall_narrow(reps: usize) {
    let mut expected: Option<(usize, u64)> = None;
    for threads in [1_usize, 8_usize] {
        let config = Config {
            width: 16,
            height: 8192,
            has_alpha: false,
            quality: 60,
            alpha_quality: 60,
            speed: 10,
            threads,
            chroma: Chroma::Yuv444,
            depth: 8,
            sample_depth: 8,
        };
        let mut feed_times = Vec::with_capacity(reps);
        let mut total_times = Vec::with_capacity(reps);
        let mut last = (0, 0.0, 0.0, 0.0, 0, 0);
        for _ in 0..reps {
            last = run_once(config);
            feed_times.push(last.1);
            total_times.push(last.3);
        }
        feed_times.sort_by(f64::total_cmp);
        total_times.sort_by(f64::total_cmp);
        let (required, _, _, _, output_bytes, digest) = last;
        if let Some(prev) = expected {
            assert_eq!(
                (output_bytes, digest),
                prev,
                "tall-narrow output changed at threads={threads}"
            );
        } else {
            expected = Some((output_bytes, digest));
        }
        println!(
            "case=tall-narrow dims=16x8192 depth=8 chroma=444 alpha=opaque threads={threads} speed=10 reps={reps} required_bytes={required} feed_median_ms={:.2} total_best_ms={:.2} total_median_ms={:.2} output_bytes={output_bytes} sha_fnv={digest:016x}",
            feed_times[reps / 2],
            total_times[0],
            total_times[reps / 2],
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("tall-narrow") {
        let reps: usize = args.get(1).map_or(9, |s| s.parse().expect("reps"));
        assert!(reps > 0, "reps must be >= 1");
        bench_tall_narrow(reps);
        return;
    }
    assert_eq!(
        args.len(),
        7,
        "usage: avif_memory_bench <width> <height> <depth:8|10|12> <chroma:444|420> <opaque|alpha> <threads> <speed>\n       avif_memory_bench tall-narrow [reps]"
    );
    let width: u32 = args[0].parse().expect("width");
    let height: u32 = args[1].parse().expect("height");
    let depth: u8 = args[2].parse().expect("depth");
    let chroma = match args[3].as_str() {
        "444" => Chroma::Yuv444,
        "420" => Chroma::Yuv420,
        other => panic!("unknown chroma {other}"),
    };
    let has_alpha = match args[4].as_str() {
        "opaque" => false,
        "alpha" => true,
        other => panic!("unknown alpha {other}"),
    };
    let threads: usize = args[5].parse().expect("threads");
    let speed: u8 = args[6].parse().expect("speed");

    let config = Config {
        width,
        height,
        has_alpha,
        quality: 60,
        alpha_quality: 60,
        speed,
        threads,
        chroma,
        depth,
        sample_depth: depth,
    };
    let (required, feed_ms, finish_ms, total_ms, output_bytes, digest) = run_once(config);
    println!(
        "dims={width}x{height} depth={depth} chroma={} alpha={} threads={threads} speed={speed} required_bytes={required} feed_ms={feed_ms:.2} finish_ms={finish_ms:.2} total_ms={total_ms:.2} output_bytes={output_bytes} sha_fnv={digest:016x}",
        args[3], args[4],
    );
}
