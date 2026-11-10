// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compare original frame transform selection, costs, and tree merging.

use super::*;
use jxl::jxl::enc_modular::*;
use jxl::jxl::enc_modular_simd::estimate_cost;
use jxl::jxl::enc_params::CompressParams;
use jxl::jxl::modular::encoding::context_predict::weighted;
use jxl::jxl::modular::modular_image::Image;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("mf-")
}
pub(super) fn image(out: &mut String, image: &Image, pixels: bool) {
    transforms::metadata(out, image);
    write!(out, " {}", u32::from(pixels)).unwrap();
    if pixels {
        for c in &image.channel {
            for y in 0..c.plane.ysize() {
                for v in &c.plane.row(y)[..c.plane.xsize()] {
                    write!(out, " {v}").unwrap();
                }
            }
        }
    }
    write!(out, " {}", image.transform.len()).unwrap();
    for t in &image.transform {
        write!(
            out,
            " {},{},{},{},{},{},{},{},{} {}",
            t.id.0,
            t.begin_c,
            t.rct_type,
            t.num_c,
            t.nb_colors,
            t.nb_deltas,
            t.predictor.0,
            u32::from(t.ordered_palette),
            u32::from(t.lossy_palette),
            t.squeezes.len()
        )
        .unwrap();
        for s in &t.squeezes {
            write!(
                out,
                " {},{},{},{}",
                u32::from(s.horizontal),
                u32::from(s.in_place),
                s.begin_c,
                s.num_c
            )
            .unwrap();
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "mf-cost" => {
            let wp = number::<u32>(args) != 0;
            let preset = number(args);
            let im = transforms::read_image(args);
            let cost = if wp {
                Ok(estimate_wp_cost(&im, preset))
            } else {
                estimate_cost(&im)
            };
            let mut out = status_code(&cost.as_ref().map(|_| ()).map_err(|e| *e)).to_string();
            if let Ok(cost) = cost {
                write!(out, " {}", cost.to_bits()).unwrap();
            }
            image(&mut out, &im, true);
            out
        }
        "mf-quant" => {
            let q = number(args);
            let cw = number(args);
            let ch = number(args);
            let mut im = transforms::read_image(args);
            im.channel[0].w = cw;
            im.channel[0].h = ch;
            quantize_channel(&mut im.channel[0], q);
            let mut out = "Q".to_owned();
            image(&mut out, &im, true);
            out
        }
        "mf-transform" => {
            let maybe = number::<u32>(args) != 0;
            let force = number::<u32>(args) != 0;
            let tier = enc_frame::tier(number(args));
            let preset = number(args);
            let cost = f32::from_bits(number(args));
            let ordered = number::<u32>(args) != 0;
            let lossy = number::<u32>(args) != 0;
            let mut tr = transforms::read_transform(args);
            tr.ordered_palette = ordered;
            tr.lossy_palette = lossy;
            let mut im = transforms::read_image(args);
            let p = CompressParams {
                speed_tier: tier,
                ..Default::default()
            };
            let mut header = weighted::Header::default();
            weighted::predictor_mode(preset, &mut header);
            let result = if maybe {
                maybe_do_transform(&mut im, &tr, &p, &header, cost, force)
            } else {
                Ok(do_transform(&mut im, &tr, &header, force))
            };
            let mut out = status_code(&result.as_ref().map(|_| ()).map_err(|e| *e)).to_string();
            if let Ok(v) = result {
                write!(out, " {}", u32::from(v)).unwrap();
            }
            image(&mut out, &im, !force);
            out
        }
        "mf-palettes" => {
            let mut depth = number(args);
            let mut max = number(args);
            let tier = enc_frame::tier(number(args));
            let colors = number(args);
            let lossy = number::<u32>(args) != 0;
            let zero = number::<u32>(args) != 0;
            let percent = f32::from_bits(number(args));
            let mut im = transforms::read_image(args);
            let mut p = CompressParams {
                speed_tier: tier,
                palette_colors: colors,
                lossy_palette: lossy,
                ..Default::default()
            };
            p.options.zero_tokens = zero;
            let s = try_palettes(&mut im, &mut depth, &mut max, &p, percent);
            let mut out = format!("{} {depth} {max}", status_code(&s));
            image(&mut out, &im, !zero);
            out
        }
        "mf-merge" => {
            let begin = number(args);
            let end = number(args);
            let n: usize = number(args);
            let trees: Vec<_> = (0..n).map(|_| ma::read_tree(args)).collect();
            let ns: usize = number(args);
            let splits: Vec<_> = (0..ns).map(|_| number(args)).collect();
            let mut tree = ma::read_tree(args);
            let s = merge_trees(&trees, &splits, begin, end, &mut tree);
            let mut out = status_code(&s).to_string();
            ma::tree(&mut out, &tree);
            out
        }
        _ => panic!("unknown modular frame helper"),
    }
}

fn fixture(
    w: usize,
    h: usize,
    n: usize,
    meta: usize,
    depth: i32,
    mode: usize,
    seed: &mut u64,
) -> String {
    let mut out = format!("{w} {h} {depth} {meta} 0 {n}");
    for c in 0..n {
        write!(out, " {w} {h} 0 0 {c}").unwrap();
        for y in 0..h {
            for x in 0..w {
                let value = match mode {
                    0 => 0,
                    1 => c as i32 * 17 + 11,
                    2 => ((x + 3 * y) % 4) as i32 * (13 + c as i32),
                    3 => ((x + y) * (c + 1)) as i32,
                    4 => (next(seed) % 256) as i32,
                    5 => ((x ^ y) % 7) as i32 * (89 + c as i32),
                    6 => (next(seed) % 65536) as i32 - 32768,
                    _ => next(seed) as i32,
                };
                write!(out, " {value}").unwrap();
            }
        }
    }
    out
}

#[test]
fn modular_frame_scalar_costs_preserve_contexts_rounding_and_weighted_overflow() {
    let mut cases = Vec::new();
    let mut seed = 0x83f4_7db0_19ca_ae52;
    for (w, h) in [
        (0, 0),
        (0, 7),
        (1, 1),
        (1, 9),
        (9, 1),
        (7, 11),
        (31, 3),
        (32, 17),
        (65, 5),
    ] {
        for n in [0, 1, 3, 5] {
            for mode in 0..7 {
                let input = fixture(w, h, n, 0, 16, mode, &mut seed);
                cases.push(format!("mf-cost 0 0 {input}"));
                for preset in [0usize, 1, 2, 3, 4, 5, usize::MAX] {
                    cases.push(format!("mf-cost 1 {preset} {input}"));
                }
            }
        }
    }
    for i in 0..200 {
        let input = fixture(2 + i % 23, 1 + i % 11, 1 + i % 4, 0, 32, 7, &mut seed);
        cases.push(format!("mf-cost 1 {} {input}", i % 5));
    }
    compare(cases);
}

#[test]
fn modular_frame_quantization_uses_plane_extents_and_original_signed_division() {
    let mut cases = Vec::new();
    let mut seed = 0x6b02_144a_97dc_8f33;
    for q in [-255, -17, -3, -1, 1, 2, 3, 7, 16, 255, 65535] {
        for (w, h) in [(0, 0), (0, 5), (1, 1), (1, 19), (17, 3), (33, 7)] {
            let input = fixture(w, h, 1, 0, 16, 6, &mut seed);
            for (cw, ch) in [(w, h), (0, 0), (w / 2, h / 2)] {
                cases.push(format!("mf-quant {q} {cw} {ch} {input}"));
            }
        }
    }
    compare(cases);
}

#[test]
fn modular_frame_transform_attempts_keep_rejection_undo_metadata_and_stack_order() {
    let mut cases = Vec::new();
    let mut seed = 0xe9c2_7354_0a16_bffd;
    for i in 0..1300 {
        let w = 1 + i % 31;
        let h = 1 + (i / 31) % 7;
        let n = if i % 7 == 0 { 1 } else { 3 };
        let mode = i % 7;
        let input = fixture(w, h, n, 0, 8, mode, &mut seed);
        let (id, begin, num, rct, colors, pred) = match i % 7 {
            0 => (3, 0, 3, 6, 16, 0),
            1 => (0, 0, 3, 0, 16, 0),
            2 => (0, 0, 3, 6, 16, 0),
            3 => (0, 1, 3, 17, 16, 0),
            4 => (1, 0, n, 6, 4, 0),
            5 => (1, 0, n, 6, 256, 6),
            _ => (2, 0, 3, 6, 16, 0),
        };
        let force = i % 11 == 0;
        let maybe = i % 3 != 0;
        let tier = (i % 11) as i32 - 1;
        let cost = [-1.0f32, 0.0, 1.0, 12.0, 50000.0, f32::INFINITY, f32::NAN][i % 7];
        cases.push(format!("mf-transform {} {} {tier} {} {} {} 0 {id} {begin} {num} {rct} {colors} 0 {pred} 0 {input}",u32::from(maybe),u32::from(force),i%5,cost.to_bits(),i%2));
    }
    compare(cases);
}

#[test]
fn modular_frame_palette_selection_keeps_global_compaction_costs_and_metadata() {
    let mut cases = Vec::new();
    let mut seed = 0x2bc4_198f_7735_a60e;
    for i in 0..1200 {
        let w = 1 + i % 47;
        let h = 1 + (i / 47) % 11;
        let n = [1, 3, 4][i % 3];
        let lossy = i % 13 == 0 && n > 1;
        let zero = i % 17 == 0;
        let depth = if i % 7 == 0 { 16 } else { 8 };
        let input = fixture(w, h, n, 0, depth, i % 7, &mut seed);
        let tier = (i % 11) as i32 - 1;
        let colors = [0, 1, 4, 16, 256, 1024, -4, -256][(i / 3) % 8];
        let percent = if zero {
            0.0f32
        } else {
            [-1.0, 0.0, 12.5, 50.0, 95.0, 100.0, f32::NAN][(i / 5) % 7]
        };
        cases.push(format!(
            "mf-palettes {} {} {tier} {colors} {} {} {} {input}",
            depth + 1,
            if depth == 16 { 65535 } else { 255 },
            u32::from(lossy),
            u32::from(zero),
            percent.to_bits()
        ));
    }
    compare(cases);
}

fn tree_request(nodes: usize, seed: &mut u64) -> String {
    let mut out = nodes.to_string();
    for _ in 0..nodes {
        write!(
            out,
            " {} {} {} {} {} {} {}",
            next(seed) as i32,
            next(seed) as i32,
            next(seed) as i32,
            next(seed) as i32,
            next(seed) % 14,
            next(seed) as i64,
            next(seed) as u32
        )
        .unwrap();
    }
    out
}
#[test]
fn modular_frame_tree_merge_keeps_recursion_append_offsets_and_failure_state() {
    let mut cases = Vec::new();
    let mut seed = 0x96fe_31a0_c578_82dd;
    for i in 0..800 {
        let n = i % 21;
        let begin = if n == 0 { 0 } else { i % n };
        let end = if i % 5 == 0 { n + 1 } else { n };
        let mut case = format!("mf-merge {begin} {end} {n}");
        for t in 0..n {
            write!(case, " {}", tree_request((i + t) % 7, &mut seed)).unwrap();
        }
        let ns = if i % 7 == 0 { n } else { n + 1 };
        write!(case, " {ns}").unwrap();
        for s in 0..ns {
            write!(
                case,
                " {}",
                [0usize, usize::MAX, 1usize << 31, s * 91][s % 4]
            )
            .unwrap();
        }
        write!(case, " {}", tree_request(i % 6, &mut seed)).unwrap();
        cases.push(case);
    }
    compare(cases);
}
