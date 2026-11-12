//! Kernel throughput per level for the resample of a 24 Mpx image to 1600
//! wide (6000×4000 → 1600×1067, Lanczos3). Synthetic data; the shapes
//! match the M2 baseline. CI runs it on every runner so each hardware class
//! reports. (A Q1.14 prototype was measured with this harness and dropped,
//! see ADR-0021.)
//!
//! `cargo run --release -p ferrisma-kernels --example bench`

use std::hint::black_box;
use std::time::{Duration, Instant};

use ferrisma_kernels::{Kernels, Taps};

const SOURCE_WIDTH: usize = 6000;
const SOURCE_HEIGHT: usize = 4000;
const OUTPUT_WIDTH: usize = 1600;
const OUTPUT_HEIGHT: usize = 1067;
const RING: usize = 32;

fn sinc(x: f64) -> f64 {
    if x.abs() < 1.0e-12 {
        1.0
    } else {
        let p = x * std::f64::consts::PI;
        p.sin() / p
    }
}

/// Contiguous Lanczos3 taps per output position, normalized in `f64`.
fn lanczos(input: usize, output: usize) -> Vec<(u32, Vec<f64>)> {
    #[allow(clippy::cast_precision_loss)]
    let ratio = input as f64 / output as f64;
    let scale = ratio.max(1.0);
    (0..output)
        .map(|at| {
            #[allow(clippy::cast_precision_loss)]
            let center = (at as f64 + 0.5) * ratio - 0.5;
            let radius = 3.0 * scale;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let first = (center - radius).floor().max(0.0) as usize;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let last = ((center + radius).ceil() as usize).min(input - 1);
            let mut weights: Vec<f64> = (first..=last)
                .map(|index| {
                    #[allow(clippy::cast_precision_loss)]
                    let d = (index as f64 - center) / scale;
                    if d.abs() < 3.0 {
                        sinc(d) * sinc(d / 3.0)
                    } else {
                        0.0
                    }
                })
                .collect();
            let sum: f64 = weights.iter().sum();
            for weight in &mut weights {
                *weight /= sum;
            }
            (u32::try_from(first).unwrap(), weights)
        })
        .collect()
}

fn table(taps: &[(u32, Vec<f64>)], source: usize) -> Taps {
    let mut starts = Vec::new();
    let mut offsets = vec![0_u32];
    let mut weights = Vec::new();
    for (start, run) in taps {
        starts.push(*start);
        #[allow(clippy::cast_possible_truncation)]
        weights.extend(run.iter().map(|&w| w as f32));
        offsets.push(u32::try_from(weights.len()).unwrap());
    }
    Taps::new(source, starts, offsets, weights).unwrap()
}

fn best_of<F: FnMut()>(runs: usize, mut body: F) -> Duration {
    (0..runs)
        .map(|_| {
            let started = Instant::now();
            body();
            started.elapsed()
        })
        .min()
        .unwrap()
}

fn main() {
    let horizontal = lanczos(SOURCE_WIDTH, OUTPUT_WIDTH);
    let vertical = lanczos(SOURCE_HEIGHT, OUTPUT_HEIGHT);
    let taps = table(&horizontal, SOURCE_WIDTH);
    let source: Vec<f32> = (0..SOURCE_WIDTH * 4)
        .map(|i| f32::from(u16::try_from(i % 997).unwrap()) / 997.0)
        .collect();
    let rows = vec![vec![0.25_f32; OUTPUT_WIDTH * 4]; RING];
    let vertical_counts: Vec<usize> = vertical.iter().map(|(_, w)| w.len()).collect();

    println!(
        "machine level: {}; shapes: horizontal {SOURCE_HEIGHT} rows {SOURCE_WIDTH} -> {OUTPUT_WIDTH}, vertical {OUTPUT_HEIGHT} rows of {OUTPUT_WIDTH}, Lanczos3",
        ferrisma_kernels::machine_level()
    );
    println!("| level | horizontal ms | vertical ms | total ms |");
    println!("| --- | ---: | ---: | ---: |");
    for kernels in Kernels::available() {
        let mut out = vec![0.0_f32; OUTPUT_WIDTH * 4];
        let mut acc = vec![0.0_f32; OUTPUT_WIDTH * 4];
        let horizontal_time = best_of(3, || {
            for _ in 0..SOURCE_HEIGHT {
                kernels
                    .horizontal_rgba(black_box(&source), &taps, &mut out)
                    .unwrap();
            }
        });
        let vertical_time = best_of(3, || {
            for (row, &count) in vertical_counts.iter().enumerate() {
                acc.fill(0.0);
                let weight = 1.0 / f32::from(u16::try_from(count).unwrap());
                for tap in 0..count {
                    kernels
                        .axpy(&mut acc, black_box(&rows[(row + tap) % RING]), weight)
                        .unwrap();
                }
                black_box(&acc);
            }
        });
        let ms = |d: Duration| d.as_secs_f64() * 1e3;
        println!(
            "| {} | {:.1} | {:.1} | {:.1} |",
            kernels.level(),
            ms(horizontal_time),
            ms(vertical_time),
            ms(horizontal_time) + ms(vertical_time)
        );
    }
}
