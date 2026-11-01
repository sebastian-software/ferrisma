// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Actual transform APIs; metadata-only paths do not observe undefined samples.
use super::*;
use jxl::jxl::modular::encoding::context_predict::weighted;
use jxl::jxl::modular::modular_image::{Channel, Image};
use jxl::jxl::modular::options::Predictor;
use jxl::jxl::modular::transform::enc_palette::{
    fwd_palette, palette_internal as enc_palette_internal, round_int,
};
use jxl::jxl::modular::transform::enc_rct::fwd_rct;
use jxl::jxl::modular::transform::enc_squeeze::fwd_squeeze;
use jxl::jxl::modular::transform::enc_transform::{compute_minmax, transform_forward};
use jxl::jxl::modular::transform::palette::{inv_palette, meta_palette, palette_internal};
use jxl::jxl::modular::transform::squeeze::*;
use jxl::jxl::modular::transform::squeeze_params::SqueezeParams;
use jxl::jxl::modular::transform::transform::{Transform, TransformId};
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("pal-") || kind.starts_with("sq-") || kind.starts_with("tr-")
}
pub(super) fn read_image(args: &mut std::str::SplitWhitespace<'_>) -> Image {
    let w = number(args);
    let h = number(args);
    let depth = number(args);
    let meta = number(args);
    let error = number::<u32>(args) != 0;
    let n = number(args);
    let mut image = Image::create(w, h, depth, 0).unwrap();
    image.nb_meta_channels = meta;
    image.error = error;
    for i in 0..n {
        let cw = number(args);
        let ch = number(args);
        let hs = number(args);
        let vs = number(args);
        let component = number(args);
        let mut c = Channel::create_with_shifts(cw, ch, hs, vs).unwrap();
        c.component = component;
        image::paint(&mut c.plane, 121 + i);
        for y in 0..ch {
            for x in 0..cw {
                c.row_mut(y)[x] = number(args);
            }
        }
        image.channel.push(c);
    }
    image
}
pub(super) fn metadata(out: &mut String, image: &Image) {
    write!(
        out,
        " {},{},{},{},{},{} {}",
        image.w,
        image.h,
        image.bitdepth,
        image.nb_meta_channels,
        u32::from(image.error),
        u32::from(image.empty()),
        image.channel.len()
    )
    .unwrap();
    for c in &image.channel {
        write!(
            out,
            " {},{},{},{},{},{},{},{},{},{}",
            c.w,
            c.h,
            c.hshift,
            c.vshift,
            c.component,
            c.plane.xsize(),
            c.plane.ysize(),
            c.plane.bytes_per_row(),
            c.plane.pixels_per_row(),
            u32::from(c.plane.bytes().is_some())
        )
        .unwrap();
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "tr-forward" => {
            let preset = number(args);
            let ordered_palette = number::<u32>(args) != 0;
            let lossy_palette = number::<u32>(args) != 0;
            let mut t = read_transform(args);
            t.ordered_palette = ordered_palette;
            t.lossy_palette = lossy_palette;
            let mut image = read_image(args);
            let mut header = weighted::Header::default();
            weighted::predictor_mode(preset, &mut header);
            let s = transform_forward(&mut t, &mut image, &header);
            let mut out = status_code(&s).to_string();
            image::transform(&mut out, &t);
            image::modular(&mut out, &image);
            out
        }
        "tr-minmax" => {
            let min = number::<u32>(args) != 0;
            let max = number::<u32>(args) != 0;
            let image = read_image(args);
            let (mut lo, mut hi) = (17, 23);
            compute_minmax(
                &image.channel[0],
                min.then_some(&mut lo),
                max.then_some(&mut hi),
            );
            format!("{lo} {hi}")
        }
        "pal-distance" => {
            let n: usize = number(args);
            let a: Vec<_> = (0..n).map(|_| f32::from_bits(number(args))).collect();
            let b: Vec<_> = (0..n).map(|_| number(args)).collect();
            enc_palette_internal::color_distance(&a, &b)
                .to_bits()
                .to_string()
        }
        "pal-quant" => {
            let n: usize = number(args);
            let size = number(args);
            let depth = number(args);
            let high = number::<u32>(args) != 0;
            let color: Vec<_> = (0..n).map(|_| number(args)).collect();
            enc_palette_internal::quantize_color_to_implicit_palette_index(
                &color, size, depth, high,
            )
            .to_string()
        }
        "pal-round" => round_int(number(args), number(args)).to_string(),
        "pal-forward" => {
            let begin = number(args);
            let end = number(args);
            let mut colors = number(args);
            let mut deltas = number(args);
            let ordered = number::<u32>(args) != 0;
            let lossy = number::<u32>(args) != 0;
            let mut pred = Predictor(number(args));
            let preset = number(args);
            let undo = number::<u32>(args) != 0;
            let mut image = read_image(args);
            let mut header = weighted::Header::default();
            weighted::predictor_mode(preset, &mut header);
            let status = fwd_palette(
                &mut image,
                begin,
                end,
                &mut colors,
                &mut deltas,
                ordered,
                lossy,
                &mut pred,
                &header,
            );
            let mut out = format!("{} {colors} {deltas} {}", status_code(&status), pred.0);
            image::modular(&mut out, &image);
            if status.is_ok() && undo {
                let s = inv_palette(&mut image, begin, colors, deltas, pred, &header);
                write!(out, " {}", status_code(&s)).unwrap();
                image::modular(&mut out, &image);
            }
            out
        }
        "pal-sort" => {
            let heap = number::<u32>(args) != 0;
            let reverse = number::<u32>(args) != 0;
            let count: usize = number(args);
            let mut values: Vec<(i64, i64)> =
                (0..count).map(|i| (number(args), i as i64)).collect();
            let mut cmp = |a: &(i64, i64), b: &(i64, i64)| {
                if reverse { a.0 > b.0 } else { a.0 < b.0 }
            };
            if heap {
                jxl::jxl::base::libcpp_sort::introsort(&mut values, 0, count, &mut cmp, 0, true);
            } else {
                jxl::jxl::base::libcpp_sort::sort_by(&mut values, cmp);
            }
            let mut out = count.to_string();
            for (key, id) in values {
                write!(out, " {key},{id}").unwrap();
            }
            out
        }
        "tr-meta" | "tr-inverse" => {
            let preset = number(args);
            let mut t = read_transform(args);
            let mut image = read_image(args);
            let mut header = weighted::Header::default();
            weighted::predictor_mode(preset, &mut header);
            let s = if kind == "tr-meta" {
                t.meta_apply(&mut image)
            } else {
                t.inverse(&mut image, &header)
            };
            let mut out = status_code(&s).to_string();
            image::transform(&mut out, &t);
            if kind == "tr-meta" {
                metadata(&mut out, &image);
            } else {
                image::modular(&mut out, &image);
            }
            out
        }
        "tr-undo" => {
            let preset = number(args);
            let count = number(args);
            let t = (0..count).map(|_| read_transform(args)).collect();
            let mut image = read_image(args);
            image.transform = t;
            let mut header = weighted::Header::default();
            weighted::predictor_mode(preset, &mut header);
            image.undo_transforms(&header);
            let mut out = "U".to_owned();
            image::modular(&mut out, &image);
            out
        }
        "tr-roundtrip" => {
            let kind = number(args);
            let mut image = read_image(args);
            let rct = Transform {
                max_delta_error: 77,
                rct_type: kind as u32,
                ..Transform::new(TransformId::RCT)
            };
            let s = fwd_rct(&mut image, 0, kind);
            let mut out = status_code(&s).to_string();
            if s.is_ok() {
                image.transform.push(rct);
            }
            let mut squeeze = Transform {
                max_delta_error: 77,
                ..Transform::new(TransformId::SQUEEZE)
            };
            default_squeeze_parameters(&mut squeeze.squeezes, &image);
            let s = fwd_squeeze(&mut image, squeeze.squeezes.clone());
            write!(out, " {}", status_code(&s)).unwrap();
            if s.is_ok() {
                image.transform.push(squeeze);
            }
            image::modular(&mut out, &image);
            image.undo_transforms(&weighted::Header::default());
            out.push_str(" U");
            image::modular(&mut out, &image);
            out
        }
        "sq-tendency" => smooth_tendency(number(args), number(args), number(args)).to_string(),
        "sq-check" => {
            let p = SqueezeParams {
                begin_c: number(args),
                num_c: number(args),
                ..Default::default()
            };
            status_code(&check_meta_squeeze_params(&p, number(args))).to_string()
        }
        "sq-default" => {
            let image = read_image(args);
            let mut p = vec![SqueezeParams::default()];
            default_squeeze_parameters(&mut p, &image);
            let mut out = "P".to_owned();
            params_observation(&mut out, &p);
            metadata(&mut out, &image);
            out
        }
        "sq-meta" | "sq-forward" | "sq-inverse" | "sq-roundtrip" => {
            let n = number(args);
            let mut p: Vec<_> = (0..n)
                .map(|_| SqueezeParams {
                    horizontal: number::<u32>(args) != 0,
                    in_place: number::<u32>(args) != 0,
                    begin_c: number(args),
                    num_c: number(args),
                })
                .collect();
            let mut image = read_image(args);
            if kind == "sq-roundtrip" && p.is_empty() {
                default_squeeze_parameters(&mut p, &image);
            }
            let s = match kind {
                "sq-meta" => meta_squeeze(&mut image, &mut p),
                "sq-inverse" => inv_squeeze(&mut image, &p),
                _ => fwd_squeeze(&mut image, p.clone()),
            };
            let mut out = status_code(&s).to_string();
            params_observation(&mut out, &p);
            if kind == "sq-meta" {
                metadata(&mut out, &image);
            } else {
                image::modular(&mut out, &image);
            }
            if kind == "sq-roundtrip" && s.is_ok() {
                let s = inv_squeeze(&mut image, &p);
                write!(out, " {}", status_code(&s)).unwrap();
                image::modular(&mut out, &image);
            }
            out
        }
        "pal-scale" => palette_internal::scale::<4>(number(args), number(args)).to_string(),
        "pal-value" => {
            let index = number(args);
            let c = number(args);
            let size = number(args);
            let stride = number::<i32>(args);
            let depth = number(args);
            let palette: Vec<_> = (0..6 * stride as usize)
                .map(|i| ((i * 17) ^ (i << 3)) as i32 - 8192)
                .collect();
            palette_internal::get_palette_value(&palette, index, c, size, stride, depth).to_string()
        }
        "pal-meta" => {
            let b = number(args);
            let e = number(args);
            let colors = number(args);
            let deltas = number(args);
            let lossy = number::<u32>(args) != 0;
            let mut image = read_image(args);
            let s = meta_palette(&mut image, b, e, colors, deltas, lossy);
            let mut out = status_code(&s).to_string();
            metadata(&mut out, &image);
            out
        }
        "pal-inverse" => {
            let b = number(args);
            let colors = number(args);
            let deltas = number(args);
            let p = Predictor(number(args));
            let preset = number(args);
            let mut image = read_image(args);
            let mut h = weighted::Header::default();
            weighted::predictor_mode(preset, &mut h);
            let s = inv_palette(&mut image, b, colors, deltas, p, &h);
            let mut out = status_code(&s).to_string();
            image::modular(&mut out, &image);
            out
        }
        _ => unreachable!(),
    }
}
fn read_transform(args: &mut std::str::SplitWhitespace<'_>) -> Transform {
    let id = TransformId(number(args));
    let mut t = Transform::new(id);
    t.max_delta_error = 77;
    t.begin_c = number(args);
    t.num_c = number(args);
    t.rct_type = number(args);
    t.nb_colors = number(args);
    t.nb_deltas = number(args);
    t.predictor = Predictor(number(args));
    let count = number(args);
    t.squeezes = (0..count)
        .map(|_| SqueezeParams {
            horizontal: number::<u32>(args) != 0,
            in_place: number::<u32>(args) != 0,
            begin_c: number(args),
            num_c: number(args),
        })
        .collect();
    t
}
fn params_observation(out: &mut String, params: &[SqueezeParams]) {
    write!(out, " {}", params.len()).unwrap();
    for p in params {
        write!(
            out,
            " {},{},{},{}",
            u32::from(p.horizontal),
            u32::from(p.in_place),
            p.begin_c,
            p.num_c
        )
        .unwrap();
    }
}
fn params_request(params: &[SqueezeParams]) -> String {
    let mut out = params.len().to_string();
    for p in params {
        write!(
            out,
            " {} {} {} {}",
            u32::from(p.horizontal),
            u32::from(p.in_place),
            p.begin_c,
            p.num_c
        )
        .unwrap();
    }
    out
}
struct FixtureChannel {
    w: usize,
    h: usize,
    hs: i32,
    vs: i32,
    component: i32,
    pixels: Vec<i32>,
}
fn fixture(w: usize, h: usize, component: i32, seed: &mut u64) -> FixtureChannel {
    FixtureChannel {
        w,
        h,
        hs: 0,
        vs: 0,
        component,
        pixels: (0..w * h)
            .map(|_| (next(seed) % 65536) as i32 - 32768)
            .collect(),
    }
}
fn image_request(
    w: usize,
    h: usize,
    depth: i32,
    meta: usize,
    channels: &[FixtureChannel],
) -> String {
    let mut out = format!("{w} {h} {depth} {meta} 0 {}", channels.len());
    for c in channels {
        write!(out, " {} {} {} {} {}", c.w, c.h, c.hs, c.vs, c.component).unwrap();
        for v in &c.pixels {
            write!(out, " {v}").unwrap();
        }
    }
    out
}
#[test]
fn palette_values_keep_implicit_cubes_delta_cycles_and_unsigned_scale() {
    let mut cases = Vec::new();
    let mut seed = 722;
    for depth in [0, 1, 7, 8, 9, 16, 24, 31] {
        for size in [0, 1, 7, 31, 64, 256] {
            for c in [0, 1, 2, 3, 5, usize::MAX] {
                for index in [
                    i32::MIN,
                    -10000,
                    -287,
                    -286,
                    -285,
                    -144,
                    -143,
                    -142,
                    -2,
                    -1,
                    0,
                    size - 1,
                    size,
                    size + 1,
                    size + 63,
                    size + 64,
                    size + 65,
                    size + 188,
                    size + 189,
                    i32::MAX,
                ] {
                    // Explicit entries require a valid channel; implicit entries return
                    // zero for every c>=3 before touching palette storage.
                    if c == usize::MAX && index >= 0 && index < size {
                        continue;
                    }
                    cases.push(format!("pal-value {index} {c} {size} 256 {depth}"));
                }
            }
        }
    }
    for depth in [0, 1, 8, 24, 31, 32, 63] {
        for value in [0, 1, 3, 4, u64::MAX, next(&mut seed)] {
            cases.push(format!("pal-scale {value} {depth}"));
        }
    }
    for depth in [1, 8, 16, 24, 31] {
        for c in 0..5 {
            for index in -400..0 {
                cases.push(format!("pal-value {index} {c} 7 256 {depth}"));
            }
        }
        for c in 0..4 {
            for index in 0..190 {
                cases.push(format!("pal-value {index} {c} 0 256 {depth}"));
            }
        }
    }
    for c in 0..6 {
        for index in 0..256 {
            cases.push(format!("pal-value {index} {c} 256 256 16"));
        }
    }
    compare(cases);
}

#[test]
fn palette_reference_sort_keeps_tied_element_order_and_heap_fallback() {
    let mut cases = Vec::new();
    let mut seed = 91237;
    for n in [
        0, 1, 2, 3, 4, 5, 6, 8, 23, 24, 25, 63, 127, 128, 129, 257, 1024, 4096,
    ] {
        for pattern in 0..12 {
            for reverse in 0..2 {
                let keys: Vec<i64> = (0..n)
                    .map(|i| match pattern {
                        0 => 0,
                        1 => i as i64,
                        2 => -(i as i64),
                        3 => (i / 7) as i64,
                        4 => ((n - i) / 7) as i64,
                        5 => i.min(n - i) as i64,
                        6 => (i % 5) as i64,
                        7 => (i % 2 * (n / 2) + i / 2) as i64,
                        8 => (next(&mut seed) % 3) as i64,
                        9 => (next(&mut seed) % 17) as i64,
                        10 => next(&mut seed) as i64,
                        _ => {
                            if i % 29 == 0 {
                                -1
                            } else {
                                i as i64
                            }
                        }
                    })
                    .collect();
                let data = keys
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ");
                cases.push(format!("pal-sort 0 {reverse} {n} {data}"));
                if n >= 24 {
                    cases.push(format!("pal-sort 1 {reverse} {n} {data}"));
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn palette_metadata_keeps_channel_ranges_and_failure_state() {
    let mut cases = Vec::new();
    let mut seed = 771;
    for n in 1..6 {
        for meta in 0..=n {
            for begin in 0..=n {
                for end in [begin, begin + 1, n - 1, n, u32::MAX as usize] {
                    let mut channels: Vec<_> =
                        (0..n).map(|c| fixture(3, 2, c as i32, &mut seed)).collect();
                    if n % 2 == 0 {
                        channels[n - 1].hs = 1;
                    }
                    cases.push(format!(
                        "pal-meta {begin} {end} {} {} {} {}",
                        n * 3,
                        n % 3,
                        n % 2,
                        image_request(3, 2, 16, meta, &channels)
                    ));
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn palette_inverse_reconstructs_all_predictors_and_preserves_channel_tags() {
    let mut cases = Vec::new();
    let mut seed = 88117;
    for i in 0..700 {
        let nb = 1 + i % 5;
        let size = [0, 1, 3, 17, 65][i % 5];
        let w = [0, 1, 2, 9, 17][i % 5];
        let h = 1 + i % 4;
        let begin = i % 3;
        let predictor = (i / 5) % 14;
        let delta = if i % 4 == 0 { 0 } else { size.min(i % 9) };
        let mut channels = vec![fixture(size, nb, 91, &mut seed)];
        channels[0].hs = -1;
        channels[0].vs = -1;
        for c in 0..begin {
            channels.push(fixture(w, h, c as i32 + 11, &mut seed));
        }
        let mut indices = fixture(w, h, 23, &mut seed);
        for (j, v) in indices.pixels.iter_mut().enumerate() {
            *v = match j % 8 {
                0 => i32::MIN,
                1 => -144,
                2 => -1,
                3 => size as i32,
                4 => (size + 64) as i32,
                5 => i32::MAX,
                _ => {
                    if size > 0 {
                        (j % size) as i32
                    } else {
                        0
                    }
                }
            };
        }
        channels.push(indices);
        channels.push(fixture(w, h, 31, &mut seed));
        let meta = if i % 11 == 0 {
            0
        } else if i % 7 == 0 {
            begin + 2
        } else {
            1
        };
        cases.push(format!(
            "pal-inverse {begin} {} {delta} {predictor} {} {}",
            size.saturating_sub(delta),
            i % 5,
            image_request(w, h, [1, 8, 16, 24, 32][i % 5], meta, &channels)
        ));
    }
    // Independent dimensions ensure nonempty single-channel palettes exercise
    // both clamps, while zero-size palettes select the implicit delta entry.
    for nb in [1, 3, 4] {
        for size in [0, 1, 7] {
            let mut channels = vec![
                fixture(size, nb, 91, &mut seed),
                fixture(9, 3, 23, &mut seed),
            ];
            for (i, v) in channels[1].pixels.iter_mut().enumerate() {
                *v = [i32::MIN, -1, 0, 1, 7, 64, 1000, i32::MAX][i % 8];
            }
            for predictor in [0, 5, 6, 14, 15, u32::MAX] {
                cases.push(format!(
                    "pal-inverse 0 {size} 0 {predictor} 0 {}",
                    image_request(9, 3, 8, 1, &channels)
                ));
            }
        }
    }
    for meta in [0, 1, 2, usize::MAX, i32::MAX as usize + 1] {
        let channels = vec![fixture(7, 3, 91, &mut seed), fixture(9, 3, 23, &mut seed)];
        cases.push(format!(
            "pal-inverse 0 {} 0 0 0 {}",
            u32::MAX,
            image_request(9, 3, 16, meta, &channels)
        ));
        let channels = vec![fixture(7, 0, 91, &mut seed), fixture(9, 3, 23, &mut seed)];
        cases.push(format!(
            "pal-inverse 0 7 0 0 0 {}",
            image_request(9, 3, 16, meta, &channels)
        ));
    }
    compare(cases);
}
#[test]
fn squeeze_tendency_and_parameter_validation_keep_signed_ties_and_wraps() {
    let mut cases = Vec::new();
    let mut seed = 919;
    for b in [-2147483648, -65535, -2, -1, 0, 1, 2, 65535, 2147483647] {
        for a in [-2147483648, -65535, -2, -1, 0, 1, 2, 65535, 2147483647] {
            for n in [-2147483648, -65535, -2, -1, 0, 1, 2, 65535, 2147483647] {
                cases.push(format!("sq-tendency {b} {a} {n}"));
            }
        }
    }
    for _ in 0..2000 {
        let b = next(&mut seed) as i32;
        let a = next(&mut seed) as i32;
        let n = next(&mut seed) as i32;
        cases.push(format!("sq-tendency {b} {a} {n}"));
    }
    for begin in [0, 1, 2, 17, 255, 0x7fffffff, 0x80000000, u32::MAX] {
        for n in [0, 1, 2, 17, 255, 0x7fffffff, 0x80000000, u32::MAX] {
            for c in [-1, 0, 1, 2, 17, 255, i32::MAX] {
                cases.push(format!("sq-check {begin} {n} {c}"));
            }
        }
    }
    compare(cases);
}
#[test]
fn squeeze_defaults_metadata_and_partial_errors_preserve_source_layout() {
    let mut cases = Vec::new();
    let mut seed = 981;
    for i in 0..400 {
        let w = [0, 1, 2, 7, 8, 9, 17, 65][i % 8];
        let h = [0, 1, 2, 7, 8, 9, 17, 65][(i / 8) % 8];
        let n = 1 + i % 5;
        let meta = i % n;
        let mut channels: Vec<_> = (0..n).map(|c| fixture(w, h, c as i32, &mut seed)).collect();
        if i % 7 == 0 && n > 1 {
            channels[1] = fixture(w / 2, h, 1, &mut seed);
        }
        let image = image_request(w, h, 16, meta, &channels);
        cases.push(format!("sq-default {image}"));
        cases.push(format!("sq-meta 0 {image}"));
    }
    for i in 0..400 {
        let n = 1 + i % 5;
        let mut channels: Vec<_> = (0..n).map(|c| fixture(9, 3, c as i32, &mut seed)).collect();
        for (c, ch) in channels.iter_mut().enumerate() {
            ch.hs = if c == n - 1 && i % 7 == 0 {
                31
            } else {
                (i % 4) as i32 - 1
            };
            ch.vs = if i % 11 == 0 { 31 } else { 0 };
        }
        let p = SqueezeParams {
            horizontal: i % 2 == 0,
            in_place: i % 3 != 0,
            begin_c: (i % (n + 1)) as u32,
            num_c: (i % 4) as u32,
        };
        let params = params_request(&[p]);
        let image = image_request(9, 3, 16, i % (n + 1), &channels);
        cases.push(format!("sq-meta {params} {image}"));
    }
    compare(cases);
}
#[test]
fn squeeze_forward_inverse_rasters_match_scalars_and_task_boundaries() {
    let mut cases = Vec::new();
    let mut seed = 123771;
    for i in 0..500 {
        let w = [0, 1, 2, 3, 8, 9, 17, 63, 64, 65, 129][i % 11];
        let h = [0, 1, 2, 3, 7, 8, 9, 17][(i / 11) % 8];
        let n = 1 + i % 4;
        let mut channels: Vec<_> = (0..n)
            .map(|c| fixture(w, h, c as i32 + 5, &mut seed))
            .collect();
        for c in &mut channels {
            c.hs = (i % 3) as i32 - 1;
            c.vs = (i % 4) as i32 - 1;
        }
        let p = match i % 4 {
            0 => Vec::new(),
            1 => vec![SqueezeParams {
                horizontal: true,
                in_place: true,
                begin_c: 0,
                num_c: n as u32,
            }],
            2 => vec![SqueezeParams {
                horizontal: false,
                in_place: false,
                begin_c: 0,
                num_c: n as u32,
            }],
            _ => vec![
                SqueezeParams {
                    horizontal: true,
                    in_place: true,
                    begin_c: 0,
                    num_c: n as u32,
                },
                SqueezeParams {
                    horizontal: false,
                    in_place: false,
                    begin_c: 0,
                    num_c: n as u32,
                },
            ],
        };
        let image = image_request(w, h, 16, 0, &channels);
        let params = params_request(&p);
        cases.push(format!("sq-roundtrip {params} {image}"));
        cases.push(format!("sq-forward {params} {image}"));
    }
    compare(cases);
}
#[test]
fn squeeze_inverse_full_width_values_and_corrupt_geometry_keep_partial_state() {
    let mut cases = Vec::new();
    let mut seed = 191177;
    for i in 0..600 {
        let horizontal = i % 2 == 0;
        let w: usize = [1, 2, 3, 9, 63, 64, 65, 129][i % 8];
        let h: usize = [1, 2, 3, 7, 8, 9, 17][(i / 8) % 7];
        let (aw, ah, rw, rh) = if horizontal {
            (w.div_ceil(2), h, w / 2, h)
        } else {
            (w, h.div_ceil(2), w, h / 2)
        };
        let mut channels = vec![
            fixture(2, 2, 7, &mut seed),
            fixture(aw, ah, 19, &mut seed),
            fixture(rw, rh, 23, &mut seed),
            fixture(2, 2, 29, &mut seed),
        ];
        for c in &mut channels[1..3] {
            c.hs = 3;
            c.vs = 5;
            for p in &mut c.pixels {
                *p = next(&mut seed) as i32;
            }
        }
        if i % 7 == 0 {
            channels[2] = fixture(rw + 1, rh, 23, &mut seed);
        }
        if i % 11 == 0 {
            channels[1] = fixture(aw, ah + 1, 19, &mut seed);
        }
        let p = SqueezeParams {
            horizontal,
            in_place: true,
            begin_c: 1,
            num_c: 1,
        };
        let image = image_request(w, h, 32, if i % 13 == 0 { 2 } else { 0 }, &channels);
        cases.push(format!("sq-inverse {} {image}", params_request(&[p])));
    }
    compare(cases);
}
#[test]
fn transform_dispatch_keeps_metadata_mutations_and_inverse_status() {
    let mut cases = Vec::new();
    let mut seed = 442155;
    for i in 0..400 {
        let n = 1 + i % 5;
        let mut channels: Vec<_> = (0..n).map(|c| fixture(9, 3, c as i32, &mut seed)).collect();
        let meta = i % (n + 1);
        if i % 7 == 0 {
            channels[n - 1] = fixture(8, 3, n as i32 - 1, &mut seed);
        }
        let kind = [0, 1, 2, 3, 4, u32::MAX][i % 6];
        let p = if kind == 2 && (i % 4 != 0 || meta == n) {
            vec![SqueezeParams {
                horizontal: i % 2 == 0,
                in_place: i % 3 == 0,
                begin_c: (i % n) as u32,
                num_c: 1,
            }]
        } else {
            Vec::new()
        };
        let t = format!(
            "{kind} {} {} {} {} {} {} {}",
            i % (n + 1),
            1 + i % 3,
            i % 42,
            i % 257,
            i % 3,
            i % 14,
            params_request(&p)
        );
        let image = image_request(9, 3, 16, meta, &channels);
        cases.push(format!("tr-meta {} {t} {image}", i % 5));
        // Inverse Squeeze uses an explicit schedule; an empty schedule is a
        // defined no-op and does not request default generation.
        cases.push(format!("tr-inverse {} {t} {image}", i % 5));
    }
    compare(cases);
}
#[test]
fn transform_stack_retains_failed_transform_and_pops_successful_prefix() {
    let mut cases = Vec::new();
    let mut seed = 66315;
    for i in 0..150 {
        let channels: Vec<_> = (0..3).map(|c| fixture(9, 3, c, &mut seed)).collect();
        let image = image_request(9, 3, 16, i % 4, &channels);
        let t = match i % 5 {
            0 => "2 4 0 3 6 0 0 0 0 0 0 3 7 0 0 0 0",
            1 => "2 0 0 3 6 0 0 0 0 4 0 3 6 0 0 0 0",
            2 => "2 1 0 3 6 0 0 5 0 0 0 3 0 0 0 0 0",
            3 => "1 2 0 3 6 0 0 0 1 1 1 0 1",
            _ => "0",
        };
        cases.push(format!("tr-undo {} {t} {image}", i % 5));
    }
    compare(cases);
}
#[test]
fn transform_stack_rct_and_squeeze_restore_rasters_in_source_order() {
    let mut cases = Vec::new();
    let mut seed = 99435;
    for kind in 0..42 {
        for (w, h) in [(1, 1), (9, 3), (17, 9), (65, 17)] {
            let channels: Vec<_> = (0..3).map(|c| fixture(w, h, c, &mut seed)).collect();
            cases.push(format!(
                "tr-roundtrip {kind} {}",
                image_request(w, h, 16, 0, &channels)
            ));
        }
    }
    compare(cases);
}

#[test]
fn palette_encoder_color_distance_rounding_and_cube_quantization_match_source_bits() {
    let mut cases = Vec::new();
    let mut seed = 883142;
    for i in 0..1800 {
        let n = i % 7;
        let mut data = String::new();
        for _ in 0..n {
            let value = (next(&mut seed) as i32 % 1000000) as f32 / 32.0;
            write!(data, " {}", value.to_bits()).unwrap();
        }
        for _ in 0..n {
            write!(data, " {}", next(&mut seed) as i32 % 1000000).unwrap();
        }
        cases.push(format!("pal-distance {n}{data}"));
    }
    for value in [
        0.0f32,
        -0.0,
        f32::from_bits(1),
        f32::from_bits(0x80000001),
        f32::MIN_POSITIVE,
        f32::MAX,
        -f32::MAX,
    ] {
        for n in 0..7 {
            for b in [-1, 0, 1, 31] {
                cases.push(format!(
                    "pal-distance {n} {} {}",
                    vec![value.to_bits().to_string(); n].join(" "),
                    vec![b.to_string(); n].join(" ")
                ));
            }
        }
    }
    for depth in [1, 2, 3, 7, 8, 9, 10, 12, 16, 24] {
        for n in 0..5 {
            for high in 0..2 {
                for i in 0..15 {
                    let max = (1i32 << depth) - 1;
                    let color: Vec<_> = (0..n)
                        .map(|_| match i {
                            0 => 0,
                            1 => max,
                            2 => max / 2,
                            _ => (next(&mut seed) % (max as u64 + 1)) as i32,
                        })
                        .collect();
                    cases.push(format!(
                        "pal-quant {n} {} {depth} {high} {}",
                        i * 17,
                        color
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(" ")
                    ));
                }
            }
        }
    }
    for div in [1, 2, 3, 6, 12, 24, 48, 96, 192, 768, 3072, 196608] {
        for i in -30..=30 {
            cases.push(format!("pal-round {} {div}", i * div / 2 + i % 3));
        }
    }
    compare(cases);
}

#[test]
fn palette_encoder_lossless_images_keep_palette_order_indices_and_failure_mutations() {
    let mut cases = Vec::new();
    let mut seed = 99455;
    for i in 0..900 {
        let (w, h) = [
            (0, 0),
            (0, 1),
            (1, 0),
            (1, 1),
            (3, 2),
            (9, 5),
            (17, 9),
            (31, 17),
            (65, 17),
        ][i % 9];
        let nb = [1, 2, 3, 4, 6][i % 5];
        let begin = usize::from(i % 4 == 0);
        let depth = [1, 2, 8, 10, 16, 24, 31, 32][i % 8];
        let colors = [0, 1, 2, 4, 16, 256, u32::MAX][i % 7];
        let mut channels: Vec<_> = (0..nb + begin + usize::from(i % 3 == 0))
            .map(|c| fixture(w, h, c as i32 + 21, &mut seed))
            .collect();
        for (c, ch) in channels.iter_mut().enumerate().skip(begin).take(nb) {
            for (j, pixel) in ch.pixels.iter_mut().enumerate() {
                let slot = match i % 8 {
                    0 => 0,
                    1 => j % 2,
                    2 => j % 5,
                    3 => j % 17,
                    4 => j % 189,
                    5 => next(&mut seed) as usize % 64,
                    6 => j,
                    _ => j % 3,
                };
                *pixel = match i % 8 {
                    0 => c as i32 * 32,
                    1 => (c * 32 + slot * 10) as i32,
                    2 => {
                        if slot == 0 {
                            0
                        } else {
                            255
                        }
                    }
                    3 => (slot * 41 + c * 72) as i32,
                    4 => palette_internal::get_palette_value(
                        &[],
                        slot as i32,
                        c - begin,
                        0,
                        0,
                        depth.min(31),
                    ),
                    5 => (slot * 17) as i32,
                    6 => (slot * 17 + c * 23) as i32,
                    _ => [-1000000, 1000000, 0][slot],
                };
            }
        }
        let meta = if i % 29 == 0 { begin + 1 } else { begin };
        let end = if i % 43 == 0 {
            channels.len() + 1
        } else {
            begin + nb - 1
        };
        if i % 31 == 0 && nb > 1 {
            channels[begin + 1].hs = 1;
        }
        let image = image_request(w, h, depth, meta, &channels);
        cases.push(format!(
            "pal-forward {begin} {end} {colors} 7 {} 0 {} {} 1 {image}",
            i % 2,
            i % 14,
            i % 5
        ));
    }
    compare(cases);
}

#[test]
fn palette_encoder_lossy_iterations_deltas_dithering_and_reconstruction_match_source() {
    let mut cases = Vec::new();
    let mut seed = 835152;
    for i in 0..150 {
        let (w, h) = [(1, 1), (3, 2), (9, 5), (17, 9), (31, 17)][i % 5];
        let depth = [1, 2, 7, 8, 9, 10, 12, 16][i % 8];
        // At >=8 bits, the source learns three delta rows. A fourth row is
        // uninitialized for those entries; the frame encoder uses RGB here.
        let nb = 3 + usize::from(depth < 8 && i % 7 == 0);
        let max = (1u32 << depth) - 1;
        let mut channels: Vec<_> = (0..nb)
            .map(|c| fixture(w, h, c as i32 + 11, &mut seed))
            .collect();
        for (c, ch) in channels.iter_mut().enumerate() {
            for (j, pixel) in ch.pixels.iter_mut().enumerate() {
                *pixel = match i % 6 {
                    0 => (max / 3) as i32,
                    1 => ((j % 2 * max as usize / 2 + c * max as usize / 7) & max as usize) as i32,
                    2 => {
                        ((j / w * max as usize / h + j % w * max as usize / w + c * 31)
                            & max as usize) as i32
                    }
                    3 => (next(&mut seed) & u64::from(max)) as i32,
                    4 => {
                        if j % w > 1 && j / w > 1 {
                            max as i32
                        } else {
                            0
                        }
                    }
                    _ => ((j * 17 + c * 37) & max as usize) as i32,
                };
            }
        }
        let colors = [0, 1, 2, 4, 16, 64][i % 6];
        cases.push(format!(
            "pal-forward 0 {} {colors} 11 {} 1 {} {} 1 {}",
            nb - 1,
            i % 2,
            [0, 3, 5, 6, 13][i % 5],
            i % 5,
            image_request(w, h, depth, 0, &channels)
        ));
    }
    // Larger repeated gradients populate the source's learned delta palette.
    for (i, depth) in [8, 10, 16].into_iter().enumerate() {
        let (w, h) = (65, 33);
        let max = (1usize << depth) - 1;
        let mut channels: Vec<_> = (0..3).map(|c| fixture(w, h, c, &mut seed)).collect();
        for (c, ch) in channels.iter_mut().enumerate() {
            for (j, pixel) in ch.pixels.iter_mut().enumerate() {
                *pixel = ((j % w * max / w + j / w * max / h + c * max / 3) & max) as i32;
            }
        }
        cases.push(format!(
            "pal-forward 0 2 4 0 1 1 13 {i} 1 {}",
            image_request(w, h, depth, 0, &channels)
        ));
    }
    compare(cases);
}

#[test]
fn forward_transform_dispatch_and_optional_minmax_outputs_match_source() {
    let mut cases = Vec::new();
    let mut seed = 773515;
    for i in 0..240 {
        let (w, h) = [(0, 0), (0, 1), (1, 1), (3, 2), (9, 5), (17, 9)][i % 6];
        let channels: Vec<_> = (0..3).map(|c| fixture(w, h, c + 1, &mut seed)).collect();
        let image = image_request(w, h, 16, i % 3, &channels);
        let t = match i % 5 {
            0 => format!("0 0 3 {} 0 0 0 0", i % 42),
            1 => "1 0 3 0 256 7 5 0".into(),
            2 => "2 0 3 0 0 0 0 0".into(),
            3 => "2 0 3 0 0 0 0 1 1 1 0 3".into(),
            _ => "4 0 3 0 0 0 0 0".into(),
        };
        cases.push(format!("tr-forward {} {} 0 {t} {image}", i % 5, i % 2));
        cases.push(format!("tr-minmax {} {} {image}", i % 2, i / 2 % 2));
    }
    compare(cases);
}
