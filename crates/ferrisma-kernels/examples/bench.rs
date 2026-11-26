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

/// Scalar `f64::powf` EOTF baseline matching pre-#281 `working_color::decode_srgb`.
fn powf_eotf(src: &[f32], dst: &mut [f64]) {
    for (&sample, out) in src.iter().zip(dst.iter_mut()) {
        let value = f64::from(sample);
        let mag = value.abs();
        let linear = if mag <= 0.04045 {
            mag / 12.92
        } else {
            ((mag + 0.055) / 1.055).powf(2.4)
        };
        *out = linear.copysign(value);
    }
}

/// Scalar `f64::powf` 16-bit OETF baseline matching pre-#281 `working_color::encode_linear_srgb`.
fn powf_oetf_u16(src: &[f32], max: u16, dst: &mut [u16]) {
    let scale = f64::from(max);
    for (&sample, out) in src.iter().zip(dst.iter_mut()) {
        let clamped = f64::from(sample).clamp(0.0, 1.0);
        let encoded = if clamped <= 0.003_130_8 {
            12.92 * clamped
        } else {
            1.055 * clamped.powf(1.0 / 2.4) - 0.055
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        {
            *out = (encoded * scale).round() as u16;
        }
    }
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
        "machine level: {}; shapes: horizontal {SOURCE_HEIGHT} rows {SOURCE_WIDTH} -> {OUTPUT_WIDTH}, vertical {OUTPUT_HEIGHT} rows of {OUTPUT_WIDTH}, Lanczos3; transfer: {SOURCE_HEIGHT} rows of {SOURCE_WIDTH}x3 (EOTF), {OUTPUT_HEIGHT} rows of {OUTPUT_WIDTH}x3 (OETF 16-bit)",
        ferrisma_kernels::machine_level()
    );
    println!(
        "| level | horizontal ms | vertical ms | eotf 24MP ms | oetf 1.7MP ms | resample ms |"
    );
    println!("| --- | ---: | ---: | ---: | ---: | ---: |");
    let eotf_src: Vec<f32> = (0..SOURCE_WIDTH * 3)
        .map(|i| f32::from(u16::try_from(i % 997).unwrap()) / 997.0)
        .collect();
    let oetf_src: Vec<f32> = (0..OUTPUT_WIDTH * 3)
        .map(|i| f32::from(u16::try_from(i % 997).unwrap()) / 997.0)
        .collect();
    let ms = |d: Duration| d.as_secs_f64() * 1e3;
    {
        let mut eotf_dst = vec![0.0_f64; SOURCE_WIDTH * 3];
        let mut oetf_dst = vec![0_u16; OUTPUT_WIDTH * 3];
        let eotf_time = best_of(3, || {
            for _ in 0..SOURCE_HEIGHT {
                powf_eotf(black_box(&eotf_src), &mut eotf_dst);
                black_box(&eotf_dst);
            }
        });
        let oetf_time = best_of(3, || {
            for _ in 0..OUTPUT_HEIGHT {
                powf_oetf_u16(black_box(&oetf_src), 65_535, &mut oetf_dst);
                black_box(&oetf_dst);
            }
        });
        println!(
            "| powf-ref | - | - | {:.1} | {:.1} | - |",
            ms(eotf_time),
            ms(oetf_time)
        );
    }
    for kernels in Kernels::available() {
        let mut out = vec![0.0_f32; OUTPUT_WIDTH * 4];
        let mut acc = vec![0.0_f32; OUTPUT_WIDTH * 4];
        let mut eotf_dst = vec![0.0_f64; SOURCE_WIDTH * 3];
        let mut oetf_dst = vec![0_u16; OUTPUT_WIDTH * 3];
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
        let eotf_time = best_of(3, || {
            for _ in 0..SOURCE_HEIGHT {
                kernels
                    .srgb_to_linear(black_box(&eotf_src), &mut eotf_dst)
                    .unwrap();
                black_box(&eotf_dst);
            }
        });
        let oetf_time = best_of(3, || {
            for _ in 0..OUTPUT_HEIGHT {
                kernels
                    .linear_to_srgb_u16(black_box(&oetf_src), 65_535, &mut oetf_dst)
                    .unwrap();
                black_box(&oetf_dst);
            }
        });
        println!(
            "| {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} |",
            kernels.level(),
            ms(horizontal_time),
            ms(vertical_time),
            ms(eotf_time),
            ms(oetf_time),
            ms(horizontal_time) + ms(vertical_time)
        );
    }
}
