// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::base::rect::Rect;
use jxl::jxl::chroma_from_luma::ColorCorrelation;
use jxl::jxl::dec_noise::{decode_noise, random_three_planes};
use jxl::jxl::image::Plane;
use jxl::jxl::noise::NoiseParams;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, Row, RowBuffers, RowStorage,
};
use jxl::jxl::render_pipeline::stage_noise::{get_add_noise_stage, get_convolve_noise_stage};
use jxl::jxl::xorshift128plus::Xorshift128Plus;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("noise-")
}
fn plane(out: &mut String, p: &Plane<f32>) {
    for y in 0..p.ysize() {
        for x in 0..p.xsize() {
            write!(out, " {}", p.row(y)[x].to_bits()).unwrap();
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "noise-rng" {
        let mode = number::<u32>(args);
        let [a, b, c, d]: [u64; 4] = std::array::from_fn(|_| number(args));
        let n = number::<usize>(args);
        let mut rng = if mode == 0 {
            Xorshift128Plus::new(a)
        } else {
            Xorshift128Plus::from_seeds(a as u32, b as u32, c as u32, d as u32)
        };
        let mut out = String::from("R");
        for _ in 0..n {
            for value in rng.fill() {
                write!(out, " {value}").unwrap();
            }
        }
        return out;
    }
    if kind == "noise-random" {
        let [visible, nonvisible, x0, y0, w, h]: [usize; 6] = std::array::from_fn(|_| number(args));
        let mut planes: [_; 3] = std::array::from_fn(|c| {
            let mut p = Plane::create(w + 5, h + 3).unwrap();
            for y in 0..h + 3 {
                p.row_mut(y)[..w + 5].fill(-17.0 - c as f32);
            }
            p
        });
        let [a, b, c] = &mut planes;
        let r = Rect::new(2, 1, w, h);
        random_three_planes(visible, nonvisible, x0, y0, [(a, r), (b, r), (c, r)]);
        let mut out = String::from("I");
        for p in &planes {
            plane(&mut out, p);
        }
        return out;
    }
    if kind == "noise-decode" {
        let skip = number(args);
        let bytes = unhex(args.next().unwrap());
        let mut params = NoiseParams {
            lut: std::array::from_fn(|_| f32::from_bits(number(args))),
        };
        let mut br = BitReader::new(&bytes);
        br.skip_bits(skip);
        let status = decode_noise(&mut br, &mut params);
        let bits = br.total_bits_consumed();
        let bounds = br.all_reads_within_bounds();
        let close = br.close();
        let mut out = format!(
            "{} {bits} {} {}",
            status_code(&status),
            u32::from(bounds.is_ok()),
            status_code(&close)
        );
        for v in params.lut {
            write!(out, " {}", v.to_bits()).unwrap();
        }
        return out;
    }
    assert_eq!(kind, "noise-row");
    let kind = number::<u32>(args);
    let [first, w, left, right, origin]: [usize; 5] = std::array::from_fn(|_| number(args));
    let params = NoiseParams {
        lut: std::array::from_fn(|_| f32::from_bits(number(args))),
    };
    let values: [f32; 8] = std::array::from_fn(|_| f32::from_bits(number(args)));
    let channels = first + 3;
    let len = origin + OFFSET + w + right + 3;
    let mut input: Vec<_> = (0..channels)
        .map(|c| {
            let mut p = Plane::create(len, 5).unwrap();
            for y in 0..5 {
                for x in 0..len {
                    p.row_mut(y)[x] = values[(c * 7 + y * 3 + x) % 8];
                }
            }
            p
        })
        .collect();
    let mut output: Vec<_> = (0..channels)
        .map(|c| {
            let mut p = Plane::create(len, 1).unwrap();
            p.row_mut(0)[..len].fill(-19.0 - c as f32);
            p
        })
        .collect();
    let correlation = ColorCorrelation::default();
    let stage = if kind != 0 {
        get_add_noise_stage(&params, &correlation, first)
    } else {
        get_convolve_noise_stage(first)
    };
    let ir = (0..channels)
        .map(|c| {
            (if kind != 0 { vec![2] } else { (0..5).collect() })
                .into_iter()
                .map(|y| Row {
                    storage: RowStorage::Input,
                    channel: c,
                    y,
                    x: origin,
                })
                .collect()
        })
        .collect();
    let or = (0..channels)
        .map(|c| {
            vec![Row {
                storage: RowStorage::Output,
                channel: c,
                y: 0,
                x: origin,
            }]
        })
        .collect();
    let status = stage.process_row(
        &ir,
        &or,
        &mut RowBuffers {
            input: &mut input,
            output: &mut output,
        },
        left,
        right,
        w,
        23,
        41,
        17,
    );
    let s = stage.settings();
    let mut out = format!(
        "{} {} {} {}",
        status_code(&status),
        stage.get_name(),
        s.border_x,
        s.border_y
    );
    for c in 0..channels + 2 {
        write!(out, " {}", stage.get_channel_mode(c) as i32).unwrap();
    }
    for p in &input {
        plane(&mut out, p);
    }
    for p in &output {
        plane(&mut out, p);
    }
    out
}

#[test]
fn noise_original_seed_schedules_and_random_plane_batches() {
    let mut cases = Vec::new();
    for mode in [0, 1] {
        for a in [0, 1, u32::MAX as u64, u64::MAX, 0x123456789abcdef0] {
            for n in [0, 1, 2, 17] {
                cases.push(format!("noise-rng {mode} {a} 123456789 4294967295 17 {n}"));
            }
        }
    }
    for w in [0, 1, 2, 15, 16, 17, 31, 32, 33, 65] {
        for h in [0, 1, 3] {
            for seeds in [
                "0 0 0 0",
                "7 13 256 768",
                "4294967296 4294967297 4294967298 4294967299",
            ] {
                cases.push(format!("noise-random {seeds} {w} {h}"));
            }
        }
    }
    compare(cases);
}
#[test]
fn noise_original_unaligned_and_truncated_parameter_reads() {
    let mut cases = Vec::new();
    let mut seed = 31;
    for len in 0..=14 {
        let bytes: Vec<_> = (0..len).map(|_| next(&mut seed) as u8).collect();
        for skip in 0..8 {
            cases.push(format!(
                "noise-decode {skip} {} {}",
                hex(&bytes),
                [0x7fa12345u32; 8].map(|v| v.to_string()).join(" ")
            ));
        }
    }
    compare(cases);
}
#[test]
fn noise_original_scalar_convolution_and_lut_application() {
    let mut cases = Vec::new();
    let patterns = [
        [-2.0, -0.0, 0.0, 0.01, 0.25, 0.5, 1.0, 2.0].map(f32::to_bits),
        [
            0, 0x80000000, 1, 0x80000001, 0x00800000, 0x80800000, 0x3f800001, 0xbf800001,
        ],
        [
            0x7fc12345, 0xffc98765, 0x7fa11111, 0xffa22222, 0x7f800000, 0xff800000, 1, 0x80000001,
        ],
    ];
    let luts = [
        [0.0; 8],
        [0.001, 0.0011, 0.125, 0.25, 0.5, 1.0, 2.0, -1.0],
        [
            f32::from_bits(0x7fa12345),
            0.0,
            0.125,
            0.5,
            1.0,
            -0.0,
            0.75,
            0.25,
        ],
    ];
    for kind in [0, 1] {
        for first in [3, 4, 6] {
            for w in [0, 1, 2, 7, 8, 9, 17] {
                for (left, right, origin) in [(0, 0, 0), (0, 0, 5), (2, 3, 4)] {
                    for values in patterns {
                        for lut in luts {
                            cases.push(format!(
                                "noise-row {kind} {first} {w} {left} {right} {origin} {} {}",
                                lut.map(|v| v.to_bits().to_string()).join(" "),
                                values.map(|v| v.to_string()).join(" ")
                            ));
                        }
                    }
                }
            }
        }
    }
    if let Some(path) = std::env::var_os("JXL_NOISE_REQUESTS") {
        std::fs::write(path, cases.join("\n") + "\n").unwrap();
    }
    // Bound output retention per original process while preserving every request.
    for batch in cases.chunks(24) {
        compare(batch.to_vec());
    }
}
