// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Inline predictor/state/tree comparisons against the actual original header.

use super::image::{channel, paint};
use super::rct::fill;
use super::*;
use jxl::jxl::modular::encoding::context_predict::*;
use jxl::jxl::modular::modular_image::{Channel, Image};
use jxl::jxl::modular::options::Predictor;
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("pred-")
}
fn values<T: std::fmt::Display>(out: &mut String, v: &[T]) {
    write!(out, " {}", v.len()).unwrap();
    for value in v {
        write!(out, " {value}").unwrap();
    }
}
fn header(out: &mut String, h: &weighted::Header) {
    write!(
        out,
        " {},{},{},{},{},{},{},{}",
        u32::from(h.all_default),
        h.p1_c,
        h.p2_c,
        h.p3_ca,
        h.p3_cb,
        h.p3_cc,
        h.p3_cd,
        h.p3_ce
    )
    .unwrap();
    for w in h.w {
        write!(out, " {w}").unwrap();
    }
}
fn state(out: &mut String, s: &weighted::State<'_>) {
    write!(out, " {}", s.pred).unwrap();
    for p in s.prediction {
        write!(out, " {p}").unwrap();
    }
    for errors in &s.pred_errors {
        values(out, errors);
    }
    values(out, &s.error);
}
fn fixture_tree() -> FlatTree {
    let mut tree = vec![FlatDecisionNode::default(); 5];
    tree[0] = FlatDecisionNode::new(9, 1);
    tree[0].set_splitval0(0);
    tree[0].set_property(0, 4);
    tree[0].set_property(1, 8);
    tree[0].set_splitval(0, 10000);
    tree[0].set_splitval(1, 0);
    let predictors = [
        Predictor::LEFT,
        Predictor::GRADIENT,
        Predictor::WEIGHTED,
        Predictor::AVERAGE4,
    ];
    let offsets = [-17, 0, 9, 127];
    let contexts = [11, 19, 23, 31];
    for i in 0..4 {
        tree[i + 1] = FlatDecisionNode::new(-1, contexts[i]);
        tree[i + 1].set_predictor(predictors[i]);
        tree[i + 1].set_multiplier(1 << i);
        tree[i + 1].set_predictor_offset(offsets[i]);
    }
    tree
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "pred-one" => {
            let p = Predictor(number(args));
            let v: [i64; 8] = std::array::from_fn(|_| number(args));
            format!(
                "{} {} {} {}",
                u32::from(predictor_has_weighted(p)),
                detail::predict_one(p, v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]),
                select(v[0], v[1], v[3]),
                clamped_gradient(v[1] as i32, v[0] as i32, v[3] as i32)
            )
        }
        "pred-preset" => {
            let mut h = weighted::Header::default();
            weighted::predictor_mode(number(args), &mut h);
            let mut out = "H".to_owned();
            header(&mut out, &h);
            out
        }
        "pred-header" => {
            let offset = number(args);
            let mut h = weighted::Header {
                p1_c: number(args),
                p2_c: number(args),
                p3_ca: number(args),
                p3_cb: number(args),
                p3_cc: number(args),
                p3_cd: number(args),
                p3_ce: number(args),
                w: std::array::from_fn(|_| number(args)),
                ..weighted::Header::default()
            };
            let can = bundle::can_encode(&mut h);
            let (ext, bits) = can.unwrap_or((0, 0));
            let mut out = format!("{} {ext} {bits}", status_code(&can.map(|_| ())));
            if can.is_ok() {
                let mut writer = BitWriter::new();
                prefix(&mut writer, offset);
                let write = bundle::write(&mut h, &mut writer, LayerType::Header, None);
                write!(out, " {} {}", status_code(&write), writer.bits_written()).unwrap();
                pad(&mut writer);
                write!(out, " {}", hex(writer.get_span())).unwrap();
                let mut decoded = weighted::Header::default();
                let mut br = BitReader::new(writer.get_span());
                br.skip_bits(offset);
                let read = bundle::read(&mut br, &mut decoded);
                write!(
                    out,
                    " {} {} {}",
                    status_code(&read),
                    br.total_bits_consumed(),
                    status_code(&br.close())
                )
                .unwrap();
                header(&mut out, &decoded);
            }
            header(&mut out, &h);
            out
        }
        "pred-header-read" => {
            let offset = number(args);
            let bytes = unhex(args.next().unwrap());
            let mut h = weighted::Header::default();
            weighted::predictor_mode(1, &mut h);
            let mut br = BitReader::new(&bytes);
            br.skip_bits(offset);
            let read = bundle::read(&mut br, &mut h);
            let mut out = format!(
                "{} {} {}",
                status_code(&read),
                br.total_bits_consumed(),
                status_code(&br.close())
            );
            header(&mut out, &h);
            out
        }
        "pred-math" => {
            let x = number(args);
            let max = number(args);
            let p = std::array::from_fn(|_| number(args));
            let w = std::array::from_fn(|_| number(args));
            let h = weighted::Header::default();
            let s = weighted::State::new(&h, 1, 1);
            let mut out = format!(
                "{} {} {}",
                s.error_weight(x, max),
                s.weighted_average(&p, w),
                weighted::State::add_bits(p[0])
            );
            for d in s.divlookup() {
                write!(out, " {d}").unwrap();
            }
            out
        }
        "pred-lookup" => {
            let n = number(args);
            let count = number(args);
            let mut tree = Vec::new();
            for _ in 0..n {
                let property0 = number(args);
                let a: i32 = number(args);
                let b = number(args);
                let c = number(args);
                let child = number(args);
                let t0: i32 = number(args);
                let t1: i32 = number(args);
                let mut node = FlatDecisionNode::new(property0, child);
                if property0 < 0 {
                    node.set_predictor(Predictor(a as u32));
                    node.set_multiplier(b);
                    node.set_predictor_offset(t0);
                } else {
                    node.set_splitval0(a);
                    node.set_splitval(0, b);
                    node.set_splitval(1, c);
                    node.set_property(0, t0 as i16);
                    node.set_property(1, t1 as i16);
                }
                tree.push(node);
            }
            let props: Vec<i32> = (0..count).map(|_| number(args)).collect();
            let lookup = MATreeLookup::new(&tree);
            let r = lookup.lookup(&props);
            format!(
                "{} {} {},{},{},{}",
                std::mem::size_of::<FlatDecisionNode>(),
                std::mem::align_of::<FlatDecisionNode>(),
                r.context,
                r.predictor.0,
                r.offset,
                r.multiplier
            )
        }
        "pred-references" => {
            let w = number(args);
            let h = number(args);
            let n = number(args);
            let current: usize = number(args);
            let y = number(args);
            let refw = number(args);
            let mode: u32 = number(args);
            let mut image = Image::create(w, h, 16, n).unwrap();
            fill(&mut image, args);
            for i in 0..current {
                if mode & 1 != 0 && i % 3 == 0 {
                    image.channel[i].w = w - 1;
                }
                if mode & 2 != 0 && i % 3 == 1 {
                    image.channel[i].hshift = 1;
                }
                if mode & 4 != 0 && i % 3 == 2 {
                    image.channel[i].vshift = 1;
                }
            }
            let mut refs = Channel::create(refw, w).unwrap();
            paint(&mut refs.plane, 141);
            precompute_references(
                &image.channel[current],
                y,
                &image,
                current as u32,
                &mut refs,
            );
            let mut out = "R".to_owned();
            channel(&mut out, &refs, w, true);
            out
        }
        "pred-stream" => {
            let mode: u32 = number(args);
            let w = number(args);
            let h = number(args);
            let predictor = Predictor(number(args));
            let preset = number(args);
            let refw = number(args);
            let static_props = [number(args), number(args)];
            let mut image = Image::create(w, h, 16, 3).unwrap();
            fill(&mut image, args);
            let mut refs = Channel::create(refw, w).unwrap();
            paint(&mut refs.plane, 151);
            let mut wp_header = weighted::Header::default();
            weighted::predictor_mode(preset, &mut wp_header);
            let mut wp_state = weighted::State::new(&wp_header, w, h);
            let mut props: Vec<_> = (0..NUM_NONREF_PROPERTIES + refw)
                .map(|i| i as i32 * 17 - 100)
                .collect();
            let tree = fixture_tree();
            let lookup = MATreeLookup::new(&tree);
            let mut out = "S".to_owned();
            header(&mut out, &wp_header);
            let ch = &image.channel[2];
            let stride = ch.plane.pixels_per_row();
            for y in 0..h {
                precompute_references(ch, y, &image, 2, &mut refs);
                init_props_row(&mut props, &static_props, y as i32);
                for x in 0..w {
                    let mut predictions = vec![-999; 14];
                    let pp = PixelPosition::new(
                        ch.plane.pixels().unwrap(),
                        y * stride as usize + x,
                        stride,
                    );
                    let nec = y > 1 && x > 1 && x < w.saturating_sub(2);
                    let all = matches!(mode, 5 | 9 | 10);
                    let uses_wp = matches!(mode, 1 | 3 | 4 | 5 | 7 | 8 | 9);
                    let uses_tree = matches!(mode, 2 | 3 | 6 | 7);
                    let (xx, yy) = (x as i32, y as i32);
                    if all {
                        if mode == 10 {
                            predict_all_no_wp(w, pp, xx, yy, &mut predictions);
                        } else if mode == 9 && nec {
                            predict_learn_all_nec(
                                &mut props,
                                w,
                                pp,
                                xx,
                                yy,
                                &refs,
                                &mut wp_state,
                                &mut predictions,
                            );
                        } else {
                            predict_learn_all(
                                &mut props,
                                w,
                                pp,
                                xx,
                                yy,
                                &refs,
                                &mut wp_state,
                                &mut predictions,
                            );
                        }
                        write!(out, " A").unwrap();
                        values(&mut out, &predictions);
                    } else {
                        let result = match mode {
                            0 => predict_no_tree_no_wp(w, pp, xx, yy, predictor),
                            1 => predict_no_tree_wp(w, pp, xx, yy, predictor, &mut wp_state),
                            2 => predict_tree_no_wp(&mut props, w, pp, xx, yy, &lookup, &refs),
                            3 => predict_tree_wp(
                                &mut props,
                                w,
                                pp,
                                xx,
                                yy,
                                &lookup,
                                &refs,
                                &mut wp_state,
                            ),
                            4 => predict_learn(
                                &mut props,
                                w,
                                pp,
                                xx,
                                yy,
                                predictor,
                                &refs,
                                &mut wp_state,
                            ),
                            6 => {
                                if nec {
                                    predict_tree_no_wp_nec(
                                        &mut props, w, pp, xx, yy, &lookup, &refs,
                                    )
                                } else {
                                    predict_tree_no_wp(&mut props, w, pp, xx, yy, &lookup, &refs)
                                }
                            }
                            7 => {
                                if nec {
                                    predict_tree_wp_nec(
                                        &mut props,
                                        w,
                                        pp,
                                        xx,
                                        yy,
                                        &lookup,
                                        &refs,
                                        &mut wp_state,
                                    )
                                } else {
                                    predict_tree_wp(
                                        &mut props,
                                        w,
                                        pp,
                                        xx,
                                        yy,
                                        &lookup,
                                        &refs,
                                        &mut wp_state,
                                    )
                                }
                            }
                            _ => {
                                if nec {
                                    predict_learn_nec(
                                        &mut props,
                                        w,
                                        pp,
                                        xx,
                                        yy,
                                        predictor,
                                        &refs,
                                        &mut wp_state,
                                    )
                                } else {
                                    predict_learn(
                                        &mut props,
                                        w,
                                        pp,
                                        xx,
                                        yy,
                                        predictor,
                                        &refs,
                                        &mut wp_state,
                                    )
                                }
                            }
                        };
                        write!(
                            out,
                            " P {},{},{}",
                            result.context, result.guess, result.predictor.0
                        )
                        .unwrap();
                        if uses_tree {
                            write!(out, ",{}", result.multiplier).unwrap();
                        }
                    }
                    if uses_wp {
                        wp_state.update_errors(i64::from(ch.row(y)[x]), x, y, w);
                    }
                    values(&mut out, &props);
                    state(&mut out, &wp_state);
                }
            }
            out
        }
        _ => panic!("unknown predictor request"),
    }
}
fn add_samples(r: &mut String, n: usize, seed: &mut u64, wide: bool) {
    for _ in 0..n {
        let x = next(seed);
        let v = if wide {
            x as i32
        } else {
            (x % 65536) as i32 - 32768
        };
        write!(r, " {v}").unwrap();
    }
}
#[test]
fn weighted_headers_presets_and_truncated_locals_match_source() {
    let mut cases = Vec::new();
    let mut seed = 971573;
    for preset in [-1, 0, 1, 2, 3, 4, 99] {
        cases.push(format!("pred-preset {preset}"));
    }
    for offset in 0..8 {
        for value in [-1, 0, 7, 16, 31, 32, i32::MAX] {
            cases.push(format!(
                "pred-header {offset} {value} 10 7 7 7 0 0 13 12 12 12"
            ));
        }
        for weight in [0, 1, 12, 15, 16, u32::MAX] {
            cases.push(format!(
                "pred-header {offset} 16 10 7 7 7 0 0 13 {weight} 12 12"
            ));
        }
    }
    for len in 0..12 {
        for trial in 0..16 {
            let bytes: Vec<_> = (0..len).map(|_| next(&mut seed) as u8).collect();
            cases.push(format!("pred-header-read {} {}", trial % 8, hex(&bytes)));
        }
    }
    compare(cases);
}
#[test]
fn all_simple_predictors_and_weighted_integer_math_match_source() {
    let mut cases = Vec::new();
    let mut seed = 1483817;
    for p in (0..17).chain([u32::MAX]) {
        for _ in 0..128 {
            let mut r = format!("pred-one {p}");
            add_samples(&mut r, 8, &mut seed, true);
            cases.push(r);
        }
    }
    // Signed division truncates toward zero; Select ties choose its second input.
    cases.push("pred-one 3 -1 0 0 0 0 0 0 0".into());
    cases.push("pred-one 4 2 3 0 0 0 0 0 0".into());
    for x in [0, 1, 31, 32, 63, 64, 65535, u32::MAX as u64] {
        for max in [0, 1, 15, u32::MAX] {
            for weights in [[4, 4, 4, 4], [0, 0, 0, 16], [1 << 24; 4], [u32::MAX / 4; 4]] {
                let mut r = format!("pred-math {x} {max}");
                add_samples(&mut r, 4, &mut seed, true);
                for w in weights {
                    write!(r, " {w}").unwrap();
                }
                cases.push(r);
            }
        }
    }
    compare(cases);
}
#[test]
fn flat_tree_layout_union_accessors_and_two_level_lookup_match_source() {
    let mut cases = Vec::new();
    let mut seed = 2781509;
    for trial in 0..1024 {
        let n = 9;
        let mut r = format!("pred-lookup {n} 16");
        for i in 0..n {
            if i == 0 || i == 1 {
                write!(
                    r,
                    " {} {} {} {} {} {} {}",
                    i,
                    next(&mut seed) as i32,
                    next(&mut seed) as i32,
                    next(&mut seed) as i32,
                    if i == 0 { 1 } else { 5 },
                    (i + 4) % 16,
                    (i + 9) % 16
                )
                .unwrap();
            } else {
                write!(
                    r,
                    " -1 {} {} 0 {} {} 0",
                    trial % 17,
                    1 + (trial % 16),
                    next(&mut seed) as u32,
                    next(&mut seed) as i32
                )
                .unwrap();
            }
        }
        add_samples(&mut r, 16, &mut seed, true);
        cases.push(r);
    }
    compare(cases);
}
#[test]
fn previous_channel_reference_properties_match_filters_and_partial_rows() {
    let mut cases = Vec::new();
    let mut seed = 648391;
    for (w, h) in [(1, 1), (5, 3)] {
        for current in 0..5 {
            for refw in [0, 1, 4, 5, 8, 16] {
                for mode in [0, 1, 2, 4, 7] {
                    for y in 0..h {
                        let mut r =
                            format!("pred-references {w} {h} 5 {current} {y} {refw} {mode}");
                        add_samples(&mut r, w * h * 5, &mut seed, true);
                        cases.push(r);
                    }
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn complete_prediction_wrappers_match_properties_guesses_and_every_state_entry() {
    let mut cases = Vec::new();
    let mut seed = 485729;
    for mode in 0..11 {
        for preset in [-1, 0, 1, 2, 3, 4] {
            for refw in [0, 4] {
                for (w, h) in [(1, 1), (5, 3), (9, 4)] {
                    if (6..=9).contains(&mode) && refw != 0 {
                        continue;
                    }
                    let mut r = format!(
                        "pred-stream {mode} {w} {h} {} {preset} {refw} 7 -3",
                        (mode + preset + 16) % 16
                    );
                    add_samples(&mut r, w * h * 3, &mut seed, false);
                    cases.push(r);
                }
            }
        }
    }
    for mode in [1, 3, 4, 5, 7, 8, 9] {
        for preset in 0..5 {
            let mut r = format!("pred-stream {mode} 9 4 6 {preset} 0 7 -3");
            add_samples(&mut r, 9 * 4 * 3, &mut seed, true);
            cases.push(r);
        }
    }
    compare(cases);
}
