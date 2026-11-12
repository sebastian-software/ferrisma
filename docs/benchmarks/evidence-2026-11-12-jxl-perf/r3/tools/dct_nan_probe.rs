//! R3 exactness probe (copy into codecs/libjxl-rs/examples/ to build):
//! hashes the inputs, outputs and scratch of compute_scaled_dct/idct,
//! transform_to_pixels and lowest_frequencies_from_dc over competing NaN
//! payloads, one line per case, for a word-exact diff between two builds.
//! `EXHAUSTIVE=1` instead covers every ordered pair of NaN positions for all
//! DCT/IDCT shapes of up to 256 values, the LowestFrequencies DCT of the
//! multi-block strategies of up to 64 blocks, and the pixel transforms of the
//! strategies of up to 4 blocks, printing one hash per group.
use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyType};
use jxl::jxl::dct::{compute_scaled_dct, compute_scaled_idct};
use jxl::jxl::dec_transforms::{lowest_frequencies_from_dc, transform_to_pixels};

fn fnv(h: &mut u64, buf: &[f32]) {
    for v in buf {
        for b in v.to_bits().to_le_bytes() {
            *h = (*h ^ u64::from(b)).wrapping_mul(0x100000001b3);
        }
    }
}

fn run(kind: u32, rows: usize, cols: usize, option: u32, values: &[u32]) -> u64 {
    let stride = cols + 3;
    let mut input: Vec<f32> = values.iter().map(|&v| f32::from_bits(v)).collect();
    input.extend_from_slice(&[-123.25; 4]);
    let out_len = match kind {
        0 => rows * cols,
        1 | 2 => rows * stride,
        _ => rows * cols * 64,
    } + 4;
    let mut output = vec![-123.25f32; out_len];
    let mut scratch = vec![-123.25f32; rows * cols * 4 + 3 * (rows + cols) + 4];
    match kind {
        0 => compute_scaled_dct(rows, cols, &input, stride, &mut output, &mut scratch),
        1 => compute_scaled_idct(rows, cols, &mut input, &mut output, stride, &mut scratch),
        2 => transform_to_pixels(
            AcStrategyType(option),
            &mut input,
            &mut output,
            stride,
            &mut scratch,
        ),
        _ => lowest_frequencies_from_dc(
            AcStrategyType(option),
            &input,
            stride,
            &mut output,
            &mut scratch,
        ),
    }
    if let Ok(want) = std::env::var("DUMP") {
        if want == format!("{kind} {rows} {cols}") {
            let w = |b: &[f32]| {
                b.iter()
                    .map(|v| format!("{:08x}", v.to_bits()))
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            println!("IN  {}\nOUT {}\nSCR {}", w(&input), w(&output), w(&scratch));
        }
    }
    let mut h = 0xcbf29ce484222325u64;
    fnv(&mut h, &input);
    fnv(&mut h, &output);
    fnv(&mut h, &scratch);
    h
}

// Every ordered pair of positions for the frequent shapes, all four kernels.
fn exhaustive() {
    let pairs: [(u32, u32); 2] = [(0x7fc12345, 0xffc98765), (0x7fa12345, 0xffc98765)];
    let mut shapes: Vec<(u32, usize, usize, u32)> = Vec::new();
    for (r, c) in [
        (4, 4),
        (4, 8),
        (8, 4),
        (8, 8),
        (8, 16),
        (16, 8),
        (16, 16),
        (1, 32),
        (32, 1),
        (2, 16),
        (16, 2),
    ] {
        shapes.push((0, r, c, 0));
        shapes.push((1, r, c, 0));
    }
    for r in [1usize, 2, 4, 8, 16, 32] {
        for c in [1usize, 2, 4, 8, 16, 32] {
            if r * c <= 256 {
                shapes.push((0, r, c, 0));
                shapes.push((1, r, c, 0));
            }
        }
    }
    for raw in 0..27u32 {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        if acs.is_multiblock() && acs.covered_blocks_x() * acs.covered_blocks_y() <= 64 {
            shapes.push((3, acs.covered_blocks_y(), acs.covered_blocks_x(), raw));
        }
        if acs.covered_blocks_x() * acs.covered_blocks_y() <= 4 {
            shapes.push((
                2,
                acs.covered_blocks_y() * 8,
                acs.covered_blocks_x() * 8,
                raw,
            ));
        }
    }
    for (kind, rows, cols, option) in shapes {
        let stride = cols + 3;
        let count = match kind {
            0 | 3 => rows * stride,
            _ => rows * cols,
        };
        let mut h = 0xcbf29ce484222325u64;
        for p in 0..count {
            for q in 0..count {
                if p == q {
                    continue;
                }
                for &(a, b) in &pairs {
                    let mut values = vec![0x3e000000u32; count];
                    values[p] = a;
                    values[q] = b;
                    h = (h ^ run(kind, rows, cols, option, &values)).wrapping_mul(0x100000001b3);
                }
            }
        }
        println!("X {kind} {rows} {cols} {option} {h:016x}");
    }
}

fn main() {
    if std::env::var_os("EXHAUSTIVE").is_some() {
        exhaustive();
        return;
    }
    let pairs: [(u32, u32); 8] = [
        (0x7fc12345, 0xffc98765),
        (0xffc98765, 0x7fc12345),
        (0x7fa12345, 0xffc98765),
        (0xffc98765, 0x7fa12345),
        (0x7fa12345, 0xffa98765),
        (0xffa98765, 0x7fa12345),
        (0x7f800000, 0xff800000),
        (0x7fc12345, 0x3f800000),
    ];
    let mut seed = 0x2718_2818u32;
    let mut next = || {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        seed
    };
    let mut cases = Vec::new();
    let sizes = [1usize, 2, 4, 8, 16, 32, 64];
    for kind in 0..2u32 {
        for &rows in &sizes {
            for &cols in &sizes {
                cases.push((kind, rows, cols, 0u32));
            }
        }
        for (r, c) in [
            (128, 128),
            (256, 256),
            (128, 256),
            (256, 128),
            (64, 128),
            (128, 64),
            (1, 256),
            (256, 1),
        ] {
            cases.push((kind, r, c, 0));
        }
    }
    for raw in 0..27u32 {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
        cases.push((
            2,
            acs.covered_blocks_y() * 8,
            acs.covered_blocks_x() * 8,
            raw,
        ));
        cases.push((3, acs.covered_blocks_y(), acs.covered_blocks_x(), raw));
    }
    let mut total = 0xcbf29ce484222325u64;
    let mut lines = 0;
    for &(kind, rows, cols, option) in &cases {
        let stride = cols + 3;
        let count = match kind {
            0 | 3 => rows * stride,
            _ => rows * cols,
        };
        let mut positions = vec![(0usize, 1usize.min(count - 1)), (0, count - 1)];
        for _ in 0..6 {
            let a = next() as usize % count;
            let b = next() as usize % count;
            positions.push((a, b));
        }
        for &(p, q) in &positions {
            for &(a, b) in &pairs {
                for base in [0x3e000000u32, 0] {
                    let mut values = vec![base; count];
                    for (i, v) in values.iter_mut().enumerate() {
                        if base != 0 {
                            *v = f32::to_bits(((i * 7 % 23) as f32 - 11.0) / 8.0);
                        }
                    }
                    values[p] = a;
                    if q != p {
                        values[q] = b;
                    }
                    let h = run(kind, rows, cols, option, &values);
                    println!(
                        "{kind} {rows} {cols} {option} {p} {q} {a:08x} {b:08x} {base:08x} {h:016x}"
                    );
                    total = (total ^ h).wrapping_mul(0x100000001b3);
                    lines += 1;
                }
            }
        }
    }
    eprintln!("cases={lines} total={total:016x}");
}
