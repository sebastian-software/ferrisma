//! Kernel throughput per level for the resample of a 24 Mpx image to 1600
//! wide (6000×4000 → 1600×1067, Lanczos3), in `f32` and in the Q1.14
//! prototype (ADR-0021 rule 2). Synthetic data; the shapes match the M2
//! baseline. CI runs it on every runner so each hardware class reports.
//!
//! `cargo run --release -p ferrixel-kernels --example bench`

// Paired `_f32` / `_q14` names are the point of the comparison.
#![allow(clippy::similar_names)]

use std::hint::black_box;
use std::time::{Duration, Instant};

use ferrixel_kernels::{Kernels, Taps, TapsQ14};

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

fn tables(taps: &[(u32, Vec<f64>)], source: usize) -> (Taps, TapsQ14) {
    let largest = taps
        .iter()
        .flat_map(|(_, w)| w.iter().map(|v| v.abs()))
        .fold(0.0_f64, f64::max);
    // ADR-0021: the largest precision whose biggest weight fits `i16`.
    let precision = (14..=16)
        .rev()
        .find(|&p| largest * f64::from(1_u32 << p) <= 32767.0)
        .unwrap_or(14);
    let mut starts = Vec::new();
    let mut offsets = vec![0_u32];
    let mut float_weights = Vec::new();
    let mut fixed_weights = Vec::new();
    for (start, weights) in taps {
        starts.push(*start);
        #[allow(clippy::cast_possible_truncation)]
        float_weights.extend(weights.iter().map(|&w| w as f32));
        #[allow(clippy::cast_possible_truncation)]
        let mut fixed: Vec<i16> = weights
            .iter()
            .map(|&w| (w * f64::from(1_u32 << precision)).round() as i16)
            .collect();
        // Rows sum to exactly 2^p; the largest tap absorbs the difference.
        let error = (1_i32 << precision) - fixed.iter().map(|&w| i32::from(w)).sum::<i32>();
        let peak = (0..fixed.len())
            .max_by_key(|&i| fixed[i].unsigned_abs())
            .unwrap();
        fixed[peak] = i16::try_from(i32::from(fixed[peak]) + error).unwrap();
        fixed_weights.extend(fixed);
        offsets.push(u32::try_from(float_weights.len()).unwrap());
    }
    (
        Taps::new(source, starts.clone(), offsets.clone(), float_weights).unwrap(),
        TapsQ14::new(source, starts, offsets, fixed_weights, precision).unwrap(),
    )
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
    let (h_f32, h_q14) = tables(&horizontal, SOURCE_WIDTH);
    let source_f32: Vec<f32> = (0..SOURCE_WIDTH * 4)
        .map(|i| f32::from(u16::try_from(i % 997).unwrap()) / 997.0)
        .collect();
    let source_q14: Vec<i16> = (0..SOURCE_WIDTH * 4)
        .map(|i| i16::try_from(i * 16 % 16385).unwrap())
        .collect();
    let rows_f32 = vec![vec![0.25_f32; OUTPUT_WIDTH * 4]; RING];
    let rows_q14 = vec![vec![4096_i16; OUTPUT_WIDTH * 4]; RING];
    let vertical_counts: Vec<usize> = vertical.iter().map(|(_, w)| w.len()).collect();

    println!(
        "machine level: {}; shapes: horizontal {SOURCE_HEIGHT} rows {SOURCE_WIDTH} -> {OUTPUT_WIDTH}, vertical {OUTPUT_HEIGHT} rows of {OUTPUT_WIDTH}, Lanczos3",
        ferrixel_kernels::machine_level()
    );
    println!(
        "| level | f32 horizontal ms | f32 vertical ms | Q1.14 horizontal ms | Q1.14 vertical ms | Q1.14 speedup |"
    );
    println!("| --- | ---: | ---: | ---: | ---: | ---: |");
    for kernels in Kernels::available() {
        let mut out_f32 = vec![0.0_f32; OUTPUT_WIDTH * 4];
        let mut out_q14 = vec![0_i16; OUTPUT_WIDTH * 4];
        let mut acc_f32 = vec![0.0_f32; OUTPUT_WIDTH * 4];
        let mut acc_q14 = vec![0_i32; OUTPUT_WIDTH * 4];
        let hf = best_of(3, || {
            for _ in 0..SOURCE_HEIGHT {
                kernels
                    .horizontal_rgba(black_box(&source_f32), &h_f32, &mut out_f32)
                    .unwrap();
            }
        });
        let vf = best_of(3, || {
            for (row, &count) in vertical_counts.iter().enumerate() {
                acc_f32.fill(0.0);
                for tap in 0..count {
                    let weight = 1.0 / f32::from(u16::try_from(count).unwrap());
                    kernels
                        .axpy(
                            &mut acc_f32,
                            black_box(&rows_f32[(row + tap) % RING]),
                            weight,
                        )
                        .unwrap();
                }
                black_box(&acc_f32);
            }
        });
        let hq = best_of(3, || {
            for _ in 0..SOURCE_HEIGHT {
                kernels
                    .horizontal_rgba_q14(black_box(&source_q14), &h_q14, &mut out_q14)
                    .unwrap();
            }
        });
        let vq = best_of(3, || {
            for (row, &count) in vertical_counts.iter().enumerate() {
                acc_q14.fill(1 << 15);
                for tap in 0..count {
                    kernels
                        .axpy_q14(&mut acc_q14, black_box(&rows_q14[(row + tap) % RING]), 1024)
                        .unwrap();
                }
                black_box(&acc_q14);
            }
        });
        let ms = |d: Duration| d.as_secs_f64() * 1e3;
        println!(
            "| {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.2}× |",
            kernels.level(),
            ms(hf),
            ms(vf),
            ms(hq),
            ms(vq),
            (ms(hf) + ms(vf)) / (ms(hq) + ms(vq))
        );
    }
}
