// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::base::rect::Rect;
use jxl::jxl::chroma_from_luma::ColorCorrelation;
use jxl::jxl::image::{Image3F, Plane};
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineStage, Row, RowBuffers, RowStorage,
};
use jxl::jxl::render_pipeline::stage_splines::{get_spline_stage, get_spline_stage_shared};
use jxl::jxl::splines::{QuantizedSpline, Spline, SplineDataView, SplinePoint, Splines};
use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("sp-")
}
fn float(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(number(args))
}
fn read_spline(args: &mut std::str::SplitWhitespace<'_>) -> Spline {
    let n = number(args);
    Spline {
        control_points: (0..n)
            .map(|_| SplinePoint::new(float(args), float(args)))
            .collect(),
        color_dct: std::array::from_fn(|_| std::array::from_fn(|_| float(args))),
        sigma_dct: std::array::from_fn(|_| float(args)),
    }
}
fn quant(out: &mut String, q: &QuantizedSpline) {
    write!(out, " Q {}", q.control_points().len()).unwrap();
    for &(x, y) in q.control_points() {
        write!(out, " {x} {y}").unwrap();
    }
    for c in q.color_dct() {
        for v in c {
            write!(out, " {v}").unwrap();
        }
    }
    for v in q.sigma_dct() {
        write!(out, " {v}").unwrap();
    }
}
fn spline(out: &mut String, s: &Spline) {
    write!(out, " S {}", s.control_points.len()).unwrap();
    for p in &s.control_points {
        write!(out, " {} {}", p.x.to_bits(), p.y.to_bits()).unwrap();
    }
    for c in &s.color_dct {
        for v in c {
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
    for v in s.sigma_dct {
        write!(out, " {}", v.to_bits()).unwrap();
    }
}
fn state(out: &mut String, s: &Splines<'_>) {
    write!(
        out,
        " H {} {} {} {} {} {} {} {} {}",
        u32::from(s.has_any()),
        s.quantization_adjustment(),
        s.data_sizes()[0],
        s.data_sizes()[1],
        s.storage_sizes()[0],
        s.storage_sizes()[1],
        s.storage_sizes()[2],
        u32::from(s.have_segment_indices()),
        u32::from(s.have_segment_y_start())
    )
    .unwrap();
    for q in s.stored_splines() {
        quant(out, q);
    }
    for p in s.stored_starting_points() {
        write!(out, " P {} {}", p.x.to_bits(), p.y.to_bits()).unwrap();
    }
}
fn cache(out: &mut String, s: &Splines<'_>, h: usize) {
    for p in s.segments() {
        write!(
            out,
            " G {} {} {} {} {}",
            p.center_x.to_bits(),
            p.center_y.to_bits(),
            p.maximum_distance.to_bits(),
            p.inv_sigma.to_bits(),
            p.sigma_over_4_times_intensity.to_bits()
        )
        .unwrap();
        for c in p.color {
            write!(out, " {}", c.to_bits()).unwrap();
        }
    }
    if let Some(starts) = s.segment_y_start() {
        out.push_str(" Y");
        for y in starts {
            write!(out, " {y}").unwrap();
        }
        if let Some(indices) = s.segment_indices() {
            out.push_str(" I");
            for i in &indices[..starts[h] as usize] {
                write!(out, " {i}").unwrap();
            }
        }
    }
}
fn pixels(out: &mut String, img: &Image3F) {
    out.push_str(" F");
    for c in 0..3 {
        for y in 0..img.ysize() {
            for x in 0..img.xsize() {
                write!(out, " {}", img.plane_row(c, y)[x].to_bits()).unwrap();
            }
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "sp-create" {
        let a = number(args);
        let cx = float(args);
        let cb = float(args);
        let size = number(args);
        let mut total = number(args);
        let sx = float(args);
        let sy = float(args);
        let original = read_spline(args);
        let result = QuantizedSpline::create(&original, a, cx, cb);
        let mut out = status_code(&result.as_ref().map(|_| ()).map_err(|e| *e)).to_string();
        if let Ok(q) = result {
            quant(&mut out, &q);
            let mut s = Spline {
                color_dct: [[123.25; 32]; 3],
                sigma_dct: [-77.5; 32],
                ..Spline::default()
            };
            let status = q.dequantize(
                SplinePoint::new(sx, sy),
                a,
                cx,
                cb,
                size,
                &mut total,
                &mut s,
            );
            write!(out, " D {} {total}", status_code(&status)).unwrap();
            spline(&mut out, &s);
        }
        return out;
    }
    if kind == "sp-decode" || kind == "sp-decode-sequence" {
        let count = if kind == "sp-decode-sequence" {
            number(args)
        } else {
            1
        };
        let mut s = Splines::new();
        let mut out = String::new();
        for i in 0..count {
            let pixels = number(args);
            let prefix = number(args);
            let bytes = unhex(args.next().unwrap());
            let mut br = BitReader::new(&bytes);
            br.skip_bits(prefix);
            let status = s.decode(&mut br, pixels);
            if i != 0 {
                out.push_str(" R ");
            }
            write!(out, "{} {}", status_code(&status), br.total_bits_consumed()).unwrap();
            state(&mut out, &s);
            write!(out, " C {}", status_code(&br.close())).unwrap();
        }
        return out;
    }
    assert_eq!(kind, "sp-cache");
    let w = number(args);
    let h = number(args);
    let n = number(args);
    let mut qs = Vec::new();
    let mut points = Vec::new();
    for _ in 0..n {
        let s = read_spline(args);
        qs.push(QuantizedSpline::create(&s, 0, 0.0, 1.0).unwrap());
        points.push(s.control_points[0]);
    }
    let mut s = Splines::new();
    s.set_data(SplineDataView {
        splines: &qs,
        starting_points: &points,
    });
    let status = s.initialize_draw_cache(w, h, &ColorCorrelation::default());
    let mut out = status_code(&status).to_string();
    state(&mut out, &s);
    cache(&mut out, &s, h);
    if status.is_ok() {
        let mut img = Image3F::create(w, h).unwrap();
        for c in 0..3 {
            for y in 0..h {
                for x in 0..w {
                    img.plane_row_mut(c, y)[x] =
                        ((x * 17 + y * 31 + c * 23) % 41) as f32 * 0.03125 - 0.625;
                }
            }
        }
        s.add_to(&mut img, Rect::new(0, 0, w, h));
        pixels(&mut out, &img);
        s.subtract_from(&mut img);
        pixels(&mut out, &img);
        s.add_to(
            &mut img,
            Rect::new(
                usize::from(w > 2),
                usize::from(h > 2),
                if w > 2 { w - 2 } else { w },
                if h > 2 { h - 2 } else { h },
            ),
        );
        pixels(&mut out, &img);
        let len = OFFSET + w + 4;
        let mut row: Vec<_> = (0..4).map(|_| Plane::create(len, 1).unwrap()).collect();
        for p in &mut row {
            p.row_mut(0)[..len].fill(0.125);
        }
        let ri = (0..4)
            .map(|c| {
                vec![Row {
                    storage: RowStorage::Input,
                    channel: c,
                    y: 0,
                    x: 0,
                }]
            })
            .collect();
        let mut stage: Box<dyn RenderPipelineStage + '_> = if n > 1 {
            Box::new(get_spline_stage_shared(Rc::new(RefCell::new(s))))
        } else {
            Box::new(get_spline_stage(&s))
        };
        let st = stage.process_row(
            &ri,
            &Vec::new(),
            &mut RowBuffers {
                input: &mut row,
                output: &mut [],
            },
            if n > 1 { 2 } else { 0 },
            2,
            w,
            0,
            h / 2,
            0,
        );
        write!(out, " T {} {}", status_code(&st), stage.get_name()).unwrap();
        for c in 0..4 {
            write!(out, " {}", stage.get_channel_mode(c) as i32).unwrap();
        }
        for r in row {
            for v in &r.row(0)[..len] {
                write!(out, " {}", v.to_bits()).unwrap();
            }
        }
    }
    out
}
fn input(s: &Spline) -> String {
    let mut out = s.control_points.len().to_string();
    for p in &s.control_points {
        write!(out, " {} {}", p.x.to_bits(), p.y.to_bits()).unwrap();
    }
    for c in &s.color_dct {
        for v in c {
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
    for v in s.sigma_dct {
        write!(out, " {}", v.to_bits()).unwrap();
    }
    out
}
fn sample(points: &[(f32, f32)], seed: u32, scale: f32) -> Spline {
    let mut state = seed;
    Spline {
        control_points: points
            .iter()
            .map(|&(x, y)| SplinePoint::new(x, y))
            .collect(),
        color_dct: std::array::from_fn(|c| {
            std::array::from_fn(|i| {
                state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                if i == 0 {
                    [0.25, 0.5, 0.75][c]
                } else {
                    ((state >> 16) as i32 - 32768) as f32 / 32768.0 * scale
                }
            })
        }),
        sigma_dct: std::array::from_fn(|i| if i == 0 { 1.25 } else { 0.0 }),
    }
}
#[test]
#[allow(
    clippy::needless_range_loop,
    reason = "Keep the index loops; stable Clippy versions differ."
)]
fn spline_create_dequantize_preserve_quantization_rounding_correlation_and_partial_failures() {
    let mut cases = Vec::new();
    for a in [i32::MIN, -128, -8, -1, 0, 1, 8, 127, 1 << 20, i32::MAX] {
        for (cx, cb) in [(0.0f32, 1.0f32), (0.75, -2.0), (-4.0, 4.0)] {
            for seed in 0..8 {
                let s = sample(
                    &[(3.5, 4.5), (8.25, 1.75), (-3.5, 9.5), (15.5, 8.5)],
                    seed,
                    4.0,
                );
                cases.push(format!(
                    "sp-create {a} {} {} 4096 0 {} {} {}",
                    cx.to_bits(),
                    cb.to_bits(),
                    s.control_points[0].x.to_bits(),
                    s.control_points[0].y.to_bits(),
                    input(&s)
                ));
            }
        }
    }
    for points in [
        vec![],
        vec![(0.0, 0.0)],
        vec![(0.0, 0.0), (8388608.0, 0.0)],
        vec![(0.0, 0.0), (1.0, 1.0), (8388608.0, 4.0)],
        vec![(0.0, 0.0), (8388607.0, 0.0), (16777214.0, 0.0)],
    ] {
        let s = sample(&points, 7, 0.0);
        cases.push(format!(
            "sp-create 0 0 {} 1 0 0 0 {}",
            1f32.to_bits(),
            input(&s)
        ));
    }
    for start in [8388607.0f32, 8388608.0, -8388608.0] {
        let s = sample(&[(0.0, 0.0), (2.0, 2.0)], 9, 0.0);
        cases.push(format!(
            "sp-create 0 0 {} 1 0 {} 0 {}",
            1f32.to_bits(),
            start.to_bits(),
            input(&s)
        ));
    }
    for total in [(1u64 << 32) - 1, 1u64 << 42, u64::MAX] {
        let s = sample(&[(0.0, 0.0), (1.0, 1.0)], 1, 100.0);
        cases.push(format!(
            "sp-create 0 0 {} 1 {total} 0 0 {}",
            1f32.to_bits(),
            input(&s)
        ));
    }
    for word in [
        0, 0x80000000, 1, 0x80000001, 0x7f7fffff, 0xff7fffff, 0x7f800000, 0xff800000,
    ] {
        let mut s = sample(&[(0.0, 0.0), (1.0, 1.0)], 123, 0.0);
        for c in 0..3 {
            s.color_dct[c].fill(f32::from_bits(word));
        }
        s.sigma_dct.fill(f32::from_bits(word));
        cases.push(format!(
            "sp-create 0 0 {} 64 0 0 0 {}",
            1f32.to_bits(),
            input(&s)
        ));
    }
    compare(cases);
}
#[test]
#[allow(
    clippy::needless_range_loop,
    reason = "Keep the index loops; stable Clippy versions differ."
)]
fn spline_cache_curves_ordered_scan_indices_pixels_subtraction_and_stage_match_original() {
    let mut cases = Vec::new();
    let paths = [
        vec![(1.0, 1.0), (9.0, 7.0)],
        vec![(-4.0, -2.0), (3.0, 8.0), (15.0, 3.0)],
        vec![(0.0, 0.0)],
        vec![(4.0, 3.0), (4.0, 3.0)],
        vec![(1.0, 1.0), (1.0, 9.0), (11.0, 1.0), (8.0, 7.0)],
    ];
    for &(w, h) in &[(1, 1), (2, 3), (7, 9), (16, 12)] {
        for (pi, path) in paths.iter().enumerate() {
            for seed in 0..4 {
                let mut s = sample(path, seed, if seed % 2 == 0 { 0.0 } else { 0.02 });
                if seed == 2 {
                    s.sigma_dct[0] = -1.25;
                }
                if seed == 3 {
                    s.sigma_dct[0] = 0.0;
                }
                cases.push(format!("sp-cache {w} {h} 1 {}", input(&s)));
                if pi == 0 {
                    let b = sample(&[(7.0, 1.0), (2.0, 8.0)], seed + 1, 0.01);
                    cases.push(format!("sp-cache {w} {h} 2 {} {}", input(&s), input(&b)));
                }
            }
        }
    }
    let mut seen_color = [[false; 32]; 3];
    let mut seen_sigma = [false; 32];
    for seed in 0..32 {
        let points: Vec<_> = (0..5)
            .map(|i| {
                (
                    (i * 7 + seed * 3) as f32 / 3.0 - 4.0,
                    ((i * 17 + seed * 13) % 39) as f32 / 3.0 - 2.0,
                )
            })
            .collect();
        let mut s = sample(&points, seed, 0.3);
        for i in 1..32 {
            s.sigma_dct[i] = (((seed * 13 + i as u32 * 17) % 61) as i32 - 30) as f32 * 0.02;
        }
        let quantized = QuantizedSpline::create(&s, 0, 0.0, 1.0).unwrap();
        for (c, seen) in seen_color.iter_mut().enumerate() {
            for (i, present) in seen.iter_mut().enumerate() {
                *present |= quantized.color_dct()[c][i] != 0;
            }
        }
        for (i, present) in seen_sigma.iter_mut().enumerate() {
            *present |= quantized.sigma_dct()[i] != 0;
        }
        cases.push(format!("sp-cache 13 11 1 {}", input(&s)));
    }
    assert!(
        seen_color
            .iter()
            .all(|seen| seen.iter().all(|&present| present)),
        "cache fixtures must retain every color DCT coefficient after source quantization"
    );
    assert!(
        seen_sigma.iter().all(|&present| present),
        "cache fixtures must retain every sigma DCT coefficient after source quantization"
    );
    cases.push("sp-cache 5 3 0".into());
    compare(cases);
}
fn raw_encode(
    prefix: usize,
    a: i32,
    points: &[(f32, f32)],
    cp: &[(i64, i64)],
    dct: [i32; 128],
) -> String {
    let mut out = format!("sp-encode-raw {prefix} {a} {}", points.len());
    for &(x, y) in points {
        write!(out, " {} {} {}", x.to_bits(), y.to_bits(), cp.len()).unwrap();
        for &(x, y) in cp {
            write!(out, " {x} {y}").unwrap();
        }
        for v in dct {
            write!(out, " {v}").unwrap();
        }
    }
    out
}
#[test]
fn spline_entropy_decode_genuine_original_streams_bounds_weird_number_truncation_and_damage() {
    let mut generators = Vec::new();
    for prefix in [0, 1, 3, 7] {
        for a in [-128, -1, 0, 1, 127] {
            for n in [1, 2, 4] {
                let points: Vec<_> = (0..n).map(|i| (i as f32 * 7.0, 3.0 + i as f32)).collect();
                let dct = std::array::from_fn(|i| if i % 9 == 0 { i as i32 - 64 } else { 0 });
                generators.push(raw_encode(
                    prefix,
                    a,
                    &points,
                    &[(1, 2), (-1, 3), (7, -8)],
                    dct,
                ));
            }
        }
    }
    for bad in [
        -(1i64 << 30),
        1i64 << 30,
        -(1i64 << 30) + 1,
        (1i64 << 30) - 1,
    ] {
        generators.push(raw_encode(0, 0, &[(1.0, 2.0)], &[(bad, 0)], [0; 128]));
    }
    for i in [0, 31, 32, 95, 96, 127] {
        let mut dct = [0; 128];
        dct[i] = i32::MIN;
        generators.push(raw_encode(0, 0, &[(1.0, 2.0)], &[], dct));
    }
    generators.push(raw_encode(0, 0, &[(8388608.0, 0.0)], &[], [0; 128]));
    generators.push(raw_encode(
        0,
        0,
        &[(1.0, 2.0), (8388608.0, 2.0)],
        &[],
        [0; 128],
    ));
    let streams = original_observations(&generators);
    let mut cases = Vec::new();
    for (g, encoded) in generators.iter().zip(streams) {
        let mut a = g.split_whitespace();
        a.next();
        let prefix: usize = number(&mut a);
        let fields: Vec<_> = encoded.split_whitespace().collect();
        assert_eq!(fields[0], "0");
        let bytes = unhex(fields[2]);
        for pixels in [0, 1, 2, 4, 6, 8, 16, 64, 4096] {
            cases.push(format!("sp-decode {pixels} {prefix} {}", hex(&bytes)));
        }
        for len in 0..bytes.len() {
            cases.push(format!("sp-decode 4096 {prefix} {}", hex(&bytes[..len])));
        }
        for i in 0..bytes.len().min(8) {
            let mut damaged = bytes.clone();
            damaged[i] ^= 0x80;
            cases.push(format!("sp-decode 4096 {prefix} {}", hex(&damaged)));
        }
    }
    for chunk in cases.chunks(64) {
        compare(chunk.to_vec());
    }
}

#[test]
fn spline_repeated_decode_preserves_old_span_sizes_and_partial_storage_on_error() {
    let streams = original_observations(&[
        raw_encode(0, 7, &[(1.0, 2.0), (7.0, 3.0)], &[(1, 2)], [0; 128]),
        raw_encode(0, -9, &[(1.0, 2.0)], &[(1 << 30, 0)], [0; 128]),
        raw_encode(0, -12, &[(8388608.0, 0.0)], &[], [0; 128]),
    ]);
    let hexes: Vec<_> = streams
        .iter()
        .map(|s| s.split_whitespace().nth(2).unwrap())
        .collect();
    let cases = vec![
        format!(
            "sp-decode-sequence 3 64 0 {} 64 0 {} 64 0 {}",
            hexes[0], hexes[1], hexes[0]
        ),
        format!(
            "sp-decode-sequence 3 64 0 {} 64 0 {} 64 0 {}",
            hexes[0], hexes[2], hexes[0]
        ),
        format!("sp-decode-sequence 2 64 0 {} 0 0 {}", hexes[0], hexes[1]),
    ];
    compare(cases);
}
