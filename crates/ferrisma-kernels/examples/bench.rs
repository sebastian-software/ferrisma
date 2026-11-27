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

/// Contiguous Lanczos3 taps per output position, normalized in `f64`, with
/// leading and trailing exact-`0.0` weights trimmed (matching
/// `src/quality.rs::taps_into` + `TapTable::contiguous`).
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
            let trim_start = weights.iter().position(|&w| w != 0.0).unwrap_or(0);
            let trim_end = weights
                .iter()
                .rposition(|&w| w != 0.0)
                .map_or(trim_start, |p| p + 1);
            weights.truncate(trim_end);
            weights.drain(..trim_start);
            let sum: f64 = weights.iter().sum();
            for weight in &mut weights {
                *weight /= sum;
            }
            (u32::try_from(first + trim_start).unwrap(), weights)
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

/// Contiguous Area taps per output position, normalized in `f64`, matching
/// `src/quality.rs::taps_into(Filter::Area)`.
fn area(input: usize, output: usize) -> Vec<(u32, Vec<f64>)> {
    #[allow(clippy::cast_precision_loss)]
    let ratio = input as f64 / output as f64;
    let scale = ratio.max(1.0);
    let radius = scale * 0.5;
    #[allow(clippy::cast_precision_loss)]
    let max_index = (input - 1) as f64;
    (0..output)
        .map(|at| {
            #[allow(clippy::cast_precision_loss)]
            let center = (at as f64 + 0.5) * ratio - 0.5;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let first = (center - radius - 1.0).floor().max(0.0) as usize;
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let last = (center + radius + 1.0).ceil().min(max_index) as usize;
            let mut first_kept = None;
            let mut weights = Vec::new();
            let mut sum = 0.0;
            for index in first..=last {
                #[allow(clippy::cast_precision_loss)]
                let sample = index as f64;
                let weight = ((sample + 0.5).min(center + radius)
                    - (sample - 0.5).max(center - radius))
                .max(0.0);
                if weight.abs() > f64::EPSILON {
                    first_kept.get_or_insert(index);
                    weights.push(weight);
                    sum += weight;
                }
            }
            for weight in &mut weights {
                *weight /= sum;
            }
            (u32::try_from(first_kept.unwrap()).unwrap(), weights)
        })
        .collect()
}

/// Contiguous 2-tap bilinear taps per output position.
fn bilinear(input: usize, output: usize) -> Taps {
    #[allow(clippy::cast_precision_loss)]
    let ratio = input as f64 / output as f64;
    let mut starts = Vec::with_capacity(output);
    let mut offsets = Vec::with_capacity(output + 1);
    offsets.push(0_u32);
    let mut weights = Vec::with_capacity(output * 2);
    for at in 0..output {
        #[allow(clippy::cast_precision_loss)]
        let pos = ((at as f64 + 0.5) * ratio - 0.5).clamp(0.0, (input - 1) as f64);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let first = (pos.floor() as usize).min(input.saturating_sub(2));
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
        let frac = (pos - first as f64).clamp(0.0, 1.0) as f32;
        starts.push(u32::try_from(first).unwrap());
        weights.push(1.0 - frac);
        if first + 1 < input {
            weights.push(frac);
        }
        offsets.push(u32::try_from(weights.len()).unwrap());
    }
    Taps::new(input, starts, offsets, weights).unwrap()
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

#[allow(clippy::too_many_lines)]
fn bench_shapes(available: &[Kernels], source: &[f32], rows: &[Vec<f32>]) {
    let ms = |d: Duration| d.as_secs_f64() * 1e3;
    let upscale_in = 1600;
    let upscale_out = 2400;
    let upscale_6 = lanczos(upscale_in, upscale_out);
    assert!(
        upscale_6[8..upscale_out - 8]
            .iter()
            .all(|(_, w)| w.len() == 6),
        "interior Lanczos3 upscale positions must execute exactly 6 taps"
    );
    let taps_6 = table(&upscale_6, upscale_in);
    let taps_2 = bilinear(upscale_in, upscale_out);
    let area_out = 4800;
    let area_3x = area(upscale_in, area_out);
    let area_min_taps = area_3x.iter().map(|(_, w)| w.len()).min().unwrap();
    let area_max_taps = area_3x.iter().map(|(_, w)| w.len()).max().unwrap();
    assert!(
        area_min_taps == 1 && area_max_taps == 2,
        "3x Area upscale (1600 -> 4800) must mix 1-tap and 2-tap runs, got {area_min_taps}..={area_max_taps}"
    );
    let taps_3x_area = table(&area_3x, upscale_in);
    let short_src = &source[..upscale_in * 4];

    let wide_rows = vec![vec![0.25_f32; 6000 * 4]; 24];
    let wide_refs: Vec<&[f32]> = wide_rows.iter().map(Vec::as_slice).collect();
    let wide_weights = vec![1.0_f32 / 24.0; 24];

    let one_refs = [rows[0].as_slice()];
    let one_weights = [1.0_f32];

    let deep_rows = vec![vec![0.25_f32; OUTPUT_WIDTH * 4]; 128];
    let deep_refs: Vec<&[f32]> = deep_rows.iter().map(Vec::as_slice).collect();
    let deep_weights = vec![1.0_f32 / 128.0; 128];

    let tiny_rows = vec![vec![0.25_f32; 64 * 4]; 4];
    let tiny_refs: Vec<&[f32]> = tiny_rows.iter().map(Vec::as_slice).collect();
    let tiny_weights = [0.25_f32; 4];

    // Contiguous power-of-two ring (64 rows × 8192 RGBA pixels = 128 KiB/row
    // in one flat allocation), matching `fast_execute::run_sequential`.
    let ring_width = 8192 * 4;
    let ring_taps = 64;
    let ring_storage = vec![0.25_f32; ring_width * ring_taps];
    let ring_refs: Vec<&[f32]> = ring_storage.chunks_exact(ring_width).collect();
    #[allow(clippy::cast_precision_loss)]
    let ring_weights = vec![1.0_f32 / ring_taps as f32; ring_taps];

    // Narrow shallow vertical ring (16 RGBA pixels = 64 f32s/row, 8-slot ring)
    // including per-row ring lookup and slice-array preparation, matching the
    // `0..=4`-tap fast path in `fast_execute::run_sequential`.
    let narrow_width = 16 * 4;
    let narrow_cap = 8;
    let narrow_ring = vec![0.25_f32; narrow_width * narrow_cap];
    let narrow_ranks: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let narrow_w1 = [1.0_f32];
    let narrow_w2 = [0.6_f32, 0.4_f32];

    println!();
    println!(
        "| level | h 6-tap ms | h 2-tap ms | h 3x-area ms | v 6000px×24t fused/axpy ms | v 1600px×1t fused/axpy ms | v 1600px×128t fused/axpy ms | v 64px×4t fused/axpy ms | v 8192px×64t ring fused/axpy ms | v 16px×1t prep fused/axpy ms | v 16px×2t prep fused/axpy ms |"
    );
    println!("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |");
    for kernels in available {
        let mut up_out = vec![0.0_f32; upscale_out * 4];
        let mut area_buf = vec![0.0_f32; area_out * 4];
        let mut wide_acc = vec![0.0_f32; 6000 * 4];
        let mut one_acc = vec![0.0_f32; OUTPUT_WIDTH * 4];
        let mut deep_acc = vec![0.0_f32; OUTPUT_WIDTH * 4];
        let mut tiny_acc = vec![0.0_f32; 64 * 4];
        let mut ring_acc = vec![0.0_f32; ring_width];
        let mut narrow_acc = vec![0.0_f32; narrow_width];

        let h6 = best_of(3, || {
            for _ in 0..2000 {
                kernels
                    .horizontal_rgba(black_box(short_src), &taps_6, &mut up_out)
                    .unwrap();
            }
        });
        let h2 = best_of(3, || {
            for _ in 0..2000 {
                kernels
                    .horizontal_rgba(black_box(short_src), &taps_2, &mut up_out)
                    .unwrap();
            }
        });
        let h_area = best_of(3, || {
            for _ in 0..5000 {
                kernels
                    .horizontal_rgba(black_box(short_src), &taps_3x_area, &mut area_buf)
                    .unwrap();
            }
        });
        let v_wide_fused = best_of(3, || {
            for _ in 0..500 {
                kernels
                    .vertical_rgba(black_box(&wide_refs), &wide_weights, &mut wide_acc)
                    .unwrap();
                black_box(&wide_acc);
            }
        });
        let v_wide_axpy = best_of(3, || {
            for _ in 0..500 {
                wide_acc.fill(0.0);
                for (&row, &w) in wide_refs.iter().zip(&wide_weights) {
                    kernels.axpy(&mut wide_acc, black_box(row), w).unwrap();
                }
                black_box(&wide_acc);
            }
        });
        let v_one_fused = best_of(3, || {
            for _ in 0..2000 {
                kernels
                    .vertical_rgba(black_box(&one_refs), &one_weights, &mut one_acc)
                    .unwrap();
                black_box(&one_acc);
            }
        });
        let v_one_axpy = best_of(3, || {
            for _ in 0..2000 {
                one_acc.fill(0.0);
                kernels
                    .axpy(&mut one_acc, black_box(one_refs[0]), one_weights[0])
                    .unwrap();
                black_box(&one_acc);
            }
        });
        let v_deep_fused = best_of(3, || {
            for _ in 0..200 {
                kernels
                    .vertical_rgba(black_box(&deep_refs), &deep_weights, &mut deep_acc)
                    .unwrap();
                black_box(&deep_acc);
            }
        });
        let v_deep_axpy = best_of(3, || {
            for _ in 0..200 {
                deep_acc.fill(0.0);
                for (&row, &w) in deep_refs.iter().zip(&deep_weights) {
                    kernels.axpy(&mut deep_acc, black_box(row), w).unwrap();
                }
                black_box(&deep_acc);
            }
        });
        let v_tiny_fused = best_of(3, || {
            for _ in 0..50_000 {
                kernels
                    .vertical_rgba(black_box(&tiny_refs), &tiny_weights, &mut tiny_acc)
                    .unwrap();
                black_box(&tiny_acc);
            }
        });
        let v_tiny_axpy = best_of(3, || {
            for _ in 0..50_000 {
                tiny_acc.fill(0.0);
                for (&row, &w) in tiny_refs.iter().zip(&tiny_weights) {
                    kernels.axpy(&mut tiny_acc, black_box(row), w).unwrap();
                }
                black_box(&tiny_acc);
            }
        });
        let v_ring_fused = best_of(3, || {
            for _ in 0..100 {
                kernels
                    .vertical_rgba(black_box(&ring_refs), &ring_weights, &mut ring_acc)
                    .unwrap();
                black_box(&ring_acc);
            }
        });
        let v_ring_axpy = best_of(3, || {
            for _ in 0..100 {
                ring_acc.fill(0.0);
                for (&row, &w) in ring_refs.iter().zip(&ring_weights) {
                    kernels.axpy(&mut ring_acc, black_box(row), w).unwrap();
                }
                black_box(&ring_acc);
            }
        });
        let lookup_narrow = |rank: usize| -> &[f32] {
            let slot = rank % narrow_cap;
            assert_eq!(narrow_ranks[slot], rank);
            &narrow_ring[slot * narrow_width..(slot + 1) * narrow_width]
        };
        let v_prep1_fused = best_of(3, || {
            for i in 0..200_000_usize {
                let r0 = black_box(i % narrow_cap);
                kernels
                    .vertical_rgba(&[lookup_narrow(r0)], &narrow_w1, &mut narrow_acc)
                    .unwrap();
                black_box(&narrow_acc);
            }
        });
        let v_prep1_axpy = best_of(3, || {
            for i in 0..200_000_usize {
                let r0 = black_box(i % narrow_cap);
                narrow_acc.fill(0.0);
                kernels
                    .axpy(&mut narrow_acc, lookup_narrow(r0), narrow_w1[0])
                    .unwrap();
                black_box(&narrow_acc);
            }
        });
        let v_prep2_fused = best_of(3, || {
            for i in 0..200_000_usize {
                let r0 = black_box(i % narrow_cap);
                let r1 = black_box((i + 1) % narrow_cap);
                kernels
                    .vertical_rgba(
                        &[lookup_narrow(r0), lookup_narrow(r1)],
                        &narrow_w2,
                        &mut narrow_acc,
                    )
                    .unwrap();
                black_box(&narrow_acc);
            }
        });
        let v_prep2_axpy = best_of(3, || {
            for i in 0..200_000_usize {
                let r0 = black_box(i % narrow_cap);
                let r1 = black_box((i + 1) % narrow_cap);
                narrow_acc.fill(0.0);
                kernels
                    .axpy(&mut narrow_acc, lookup_narrow(r0), narrow_w2[0])
                    .unwrap();
                kernels
                    .axpy(&mut narrow_acc, lookup_narrow(r1), narrow_w2[1])
                    .unwrap();
                black_box(&narrow_acc);
            }
        });
        println!(
            "| {} | {:.1} | {:.1} | {:.1} | {:.1} / {:.1} | {:.1} / {:.1} | {:.1} / {:.1} | {:.1} / {:.1} | {:.1} / {:.1} | {:.1} / {:.1} | {:.1} / {:.1} |",
            kernels.level(),
            ms(h6),
            ms(h2),
            ms(h_area),
            ms(v_wide_fused),
            ms(v_wide_axpy),
            ms(v_one_fused),
            ms(v_one_axpy),
            ms(v_deep_fused),
            ms(v_deep_axpy),
            ms(v_tiny_fused),
            ms(v_tiny_axpy),
            ms(v_ring_fused),
            ms(v_ring_axpy),
            ms(v_prep1_fused),
            ms(v_prep1_axpy),
            ms(v_prep2_fused),
            ms(v_prep2_axpy),
        );
    }
}

#[allow(clippy::too_many_lines)]
fn main() {
    let horizontal = lanczos(SOURCE_WIDTH, OUTPUT_WIDTH);
    let vertical = lanczos(SOURCE_HEIGHT, OUTPUT_HEIGHT);
    let taps = table(&horizontal, SOURCE_WIDTH);
    let source: Vec<f32> = (0..SOURCE_WIDTH * 4)
        .map(|i| f32::from(u16::try_from(i % 997).unwrap()) / 997.0)
        .collect();
    let rows = vec![vec![0.25_f32; OUTPUT_WIDTH * 4]; RING];
    let vertical_weights: Vec<Vec<f32>> = vertical
        .iter()
        .map(|(_, w)| {
            #[allow(clippy::cast_possible_truncation)]
            w.iter().map(|&v| v as f32).collect()
        })
        .collect();

    println!(
        "machine level: {}; shapes: horizontal {SOURCE_HEIGHT} rows {SOURCE_WIDTH} -> {OUTPUT_WIDTH}, vertical {OUTPUT_HEIGHT} rows of {OUTPUT_WIDTH}, Lanczos3; transfer: {SOURCE_HEIGHT} rows of {SOURCE_WIDTH}x3 (EOTF), {OUTPUT_HEIGHT} rows of {OUTPUT_WIDTH}x3 (OETF 16-bit)",
        ferrisma_kernels::machine_level()
    );
    println!(
        "| level | horizontal ms | vertical fused ms | vertical axpy ms | eotf 24MP ms | oetf 1.7MP ms | resample ms |"
    );
    println!("| --- | ---: | ---: | ---: | ---: | ---: | ---: |");
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
            "| powf-ref | - | - | - | {:.1} | {:.1} | - |",
            ms(eotf_time),
            ms(oetf_time)
        );
    }
    let available = Kernels::available();
    for kernels in &available {
        let mut out = vec![0.0_f32; OUTPUT_WIDTH * 4];
        let mut acc = vec![0.0_f32; OUTPUT_WIDTH * 4];
        let mut eotf_dst = vec![0.0_f64; SOURCE_WIDTH * 3];
        let mut oetf_dst = vec![0_u16; OUTPUT_WIDTH * 3];
        let mut row_slices: [&[f32]; 64] = [&[]; 64];
        let horizontal_time = best_of(3, || {
            for _ in 0..SOURCE_HEIGHT {
                kernels
                    .horizontal_rgba(black_box(&source), &taps, &mut out)
                    .unwrap();
            }
        });
        let vertical_fused_time = best_of(3, || {
            for (row, weights) in vertical_weights.iter().enumerate() {
                for (tap, slot) in row_slices[..weights.len()].iter_mut().enumerate() {
                    *slot = rows[(row + tap) % RING].as_slice();
                }
                kernels
                    .vertical_rgba(black_box(&row_slices[..weights.len()]), weights, &mut acc)
                    .unwrap();
                black_box(&acc);
            }
        });
        let vertical_axpy_time = best_of(3, || {
            for (row, weights) in vertical_weights.iter().enumerate() {
                acc.fill(0.0);
                for (tap, &weight) in weights.iter().enumerate() {
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
            "| {} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} | {:.1} |",
            kernels.level(),
            ms(horizontal_time),
            ms(vertical_fused_time),
            ms(vertical_axpy_time),
            ms(eotf_time),
            ms(oetf_time),
            ms(horizontal_time) + ms(vertical_fused_time)
        );
    }

    bench_shapes(&available, &source, &rows);
}
