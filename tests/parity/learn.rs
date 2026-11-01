// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Independent observations of source RNG, quantization, and sample state.
use super::*;
use jxl::jxl::base::random::Rng;
use jxl::jxl::enc_ans_params::{
    ANSHistogramStrategy, ClusteringType, HybridUintMethod, LZ77Method,
};
use jxl::jxl::modular::encoding::context_predict::weighted;
use jxl::jxl::modular::encoding::enc_encoding::{learn_tree, modular_generic_compress};
use jxl::jxl::modular::encoding::enc_ma::{
    TreeSamples, collect_pixel_samples, compute_best_tree, estimate_bits,
};
use jxl::jxl::modular::encoding::encoding::modular_generic_decompress;
use jxl::jxl::modular::modular_image::Image;
use jxl::jxl::modular::options::{ModularMultiplierInfo, ModularOptions, Predictor, TreeMode};
use jxl::jxl::modular::transform::enc_rct::fwd_rct;
use jxl::jxl::modular::transform::enc_squeeze::fwd_squeeze;
use jxl::jxl::modular::transform::squeeze::default_squeeze_parameters;
use jxl::jxl::modular::transform::transform::{Transform, TransformId};
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("learn-")
}
fn mode(v: usize) -> TreeMode {
    [
        TreeMode::GradientOnly,
        TreeMode::WPOnly,
        TreeMode::NoWP,
        TreeMode::Default,
    ][v]
}
fn values<T: FromStr>(args: &mut std::str::SplitWhitespace<'_>) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    let n = number(args);
    (0..n).map(|_| number(args)).collect()
}
fn output<T: std::fmt::Display>(out: &mut String, v: &[T]) {
    write!(out, " {}", v.len()).unwrap();
    for x in v {
        write!(out, " {x}").unwrap();
    }
}
fn config(out: &mut String, s: &TreeSamples) {
    write!(out, " {}", s.num_predictors()).unwrap();
    for i in 0..s.num_predictors() {
        write!(out, " {}", s.predictor_from_index(i).0).unwrap();
    }
    write!(out, " {}", s.num_properties()).unwrap();
    for i in 0..s.num_properties() {
        write!(out, " {}", s.property_from_index(i)).unwrap();
    }
    write!(
        out,
        " {} {} {} {}",
        s.num_static_props(),
        s.num_samples(),
        s.num_distinct_samples(),
        u32::from(s.has_samples())
    )
    .unwrap();
}
fn samples(out: &mut String, s: &TreeSamples) {
    config(out, s);
    for p in 0..s.num_properties() {
        write!(out, " {}", s.num_property_values(p)).unwrap();
        for q in 0..s.num_property_values(p) - 1 {
            write!(out, " {}", s.unquantize_property(p, q as u32)).unwrap();
        }
        let mut previous = 0;
        for v in -520..=520 {
            let q = if p < s.num_static_props() {
                s.quantize_static_property(p as u32, v)
            } else {
                s.quantize_property(p as u32, v)
            };
            if v == -520 || q != previous {
                write!(out, " {v},{q}").unwrap();
            }
            previous = q;
        }
        out.push_str(" E");
    }
    for i in 0..s.num_distinct_samples() {
        write!(out, " {}", s.count(i)).unwrap();
        for p in 0..s.num_predictors() {
            let t = s.rtoken(p, i);
            write!(out, " {},{}", t.tok, t.nbits).unwrap();
        }
        for p in 0..s.num_static_props() {
            write!(out, " {}", s.property::<true>(p, i)).unwrap();
        }
        for p in 0..s.num_properties() - s.num_static_props() {
            write!(out, " {}", s.property::<false>(p, i)).unwrap();
        }
    }
}
fn options(args: &mut std::str::SplitWhitespace<'_>) -> ModularOptions {
    let tree_kind = [
        jxl::jxl::modular::options::TreeKind::TrivialTreeNoPredictor,
        jxl::jxl::modular::options::TreeKind::Learn,
        jxl::jxl::modular::options::TreeKind::JpegTranscodeACMeta,
        jxl::jxl::modular::options::TreeKind::FalconACMeta,
        jxl::jxl::modular::options::TreeKind::ACMeta,
        jxl::jxl::modular::options::TreeKind::WPFixedDC,
        jxl::jxl::modular::options::TreeKind::GradientFixedDC,
    ][number::<usize>(args)];
    let mut o = ModularOptions {
        tree_kind,
        predictor: Predictor(number(args)),
        wp_mode: number(args),
        wp_tree_mode: mode(number(args)),
        max_properties: number(args),
        max_property_values: number(args),
        nb_repeats: f32::from_bits(number(args)),
        splitting_heuristics_node_threshold: f32::from_bits(number(args)),
        fast_decode_multiplier: f32::from_bits(number(args)),
        skip_encoder_fast_path: number::<u32>(args) != 0,
        zero_tokens: number::<u32>(args) != 0,
        max_chan_size: number(args),
        group_dim: number(args),
        ..Default::default()
    };
    o.histogram_params.force_huffman = number::<u32>(args) != 0;
    o.histogram_params.lz77_method = [
        LZ77Method::None,
        LZ77Method::RLE,
        LZ77Method::LZ77b1w3f,
        LZ77Method::LZ77b3w3f,
        LZ77Method::LZ77b7w3f,
        LZ77Method::LZ77b15w3f,
        LZ77Method::LZ77b31w3f,
        LZ77Method::LZ77b1w3t,
        LZ77Method::LZ77b3w3t,
        LZ77Method::LZ77b7w3t,
        LZ77Method::LZ77b15w3t,
        LZ77Method::LZ77b31w3t,
        LZ77Method::Optc1,
        LZ77Method::Optc3,
        LZ77Method::Optc8,
        LZ77Method::Optc256,
    ][number::<usize>(args)];
    o.histogram_params.clustering = [
        ClusteringType::Fastest,
        ClusteringType::Fast,
        ClusteringType::Best,
    ][number::<usize>(args)];
    o.histogram_params.uint_method = [
        HybridUintMethod::None,
        HybridUintMethod::Uint000,
        HybridUintMethod::Fast,
        HybridUintMethod::ContextMap,
        HybridUintMethod::Best,
    ][number::<usize>(args)];
    o.histogram_params.ans_histogram_strategy = [
        ANSHistogramStrategy::Fast,
        ANSHistogramStrategy::Approximate,
        ANSHistogramStrategy::Precise,
    ][number::<usize>(args)];
    o.splitting_heuristics_properties = values(args);
    o
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "learn-tree" => {
            let start = number(args);
            let stop = number(args);
            let n = number(args);
            let mut images = Vec::new();
            let mut opts = Vec::new();
            for _ in 0..n {
                opts.push(options(args));
                images.push(transforms::read_image(args));
            }
            let count = number(args);
            let mult: Vec<_> = (0..count)
                .map(|_| ModularMultiplierInfo {
                    range: std::array::from_fn(|_| [number(args), number(args)]),
                    multiplier: number(args),
                })
                .collect();
            let tree = learn_tree(&images, &opts, start, stop, &mult);
            let mut out = status_code(&tree.as_ref().map(|_| ()).map_err(|e| *e)).to_string();
            if let Ok(tree) = tree {
                tree_observation(&mut out, &tree);
            }
            out
        }
        "learn-generic" => {
            let prefix = number(args);
            let group = number(args);
            let transform = number::<u32>(args);
            let roundtrip = number::<u32>(args) != 0;
            let mut opts = options(args);
            let mut image = transforms::read_image(args);
            if (transform == 1 || transform == 3) && image.channel.len() >= 3 {
                let t = Transform {
                    max_delta_error: 77,
                    rct_type: 6,
                    ..Transform::new(TransformId::RCT)
                };
                if fwd_rct(&mut image, 0, 6).is_ok() {
                    image.transform.push(t);
                }
            }
            if transform == 2 || transform == 3 {
                let mut t = Transform {
                    max_delta_error: 77,
                    ..Transform::new(TransformId::SQUEEZE)
                };
                default_squeeze_parameters(&mut t.squeezes, &image);
                if fwd_squeeze(&mut image, t.squeezes.clone()).is_ok() {
                    image.transform.push(t);
                }
            }
            let mut writer = BitWriter::new();
            super::prefix(&mut writer, prefix);
            let mut aux = AuxOut::default();
            let status = modular_generic_compress(
                &image,
                &opts,
                &mut writer,
                Some(&mut aux),
                LayerType::ModularGlobal,
                group,
            );
            let mut out = format!("{} {}", status_code(&status), writer.bits_written());
            pad(&mut writer);
            write!(out, " {}", hex(writer.get_span())).unwrap();
            for layer in [LayerType::ModularGlobal, LayerType::ModularTree] {
                let t = aux.layer(layer);
                write!(
                    out,
                    " {} {} {} {} {}",
                    t.num_clustered_histograms,
                    t.extra_bits,
                    t.histogram_bits,
                    t.total_bits,
                    t.clustered_entropy.to_bits()
                )
                .unwrap();
            }
            if status.is_ok() && roundtrip {
                image.undo_transforms(&weighted::Header::default());
                assert!(!image.error);
                let mut decoded =
                    Image::create(image.w, image.h, image.bitdepth, image.channel.len() as i32)
                        .unwrap();
                let mut br = BitReader::new(writer.get_span());
                br.skip_bits(prefix);
                let s = modular_generic_decompress(
                    &mut br,
                    &mut decoded,
                    None,
                    group,
                    &mut opts,
                    true,
                    None,
                    None,
                    None,
                    false,
                );
                write!(
                    out,
                    " {} {} {}",
                    status_code(&s),
                    br.total_bits_consumed(),
                    status_code(&br.close())
                )
                .unwrap();
                for t in &mut decoded.transform {
                    t.max_delta_error = 77;
                }
                image::modular(&mut out, &decoded);
            }
            out
        }
        "learn-estimate" => {
            let counts: Vec<i32> = values(args);
            estimate_bits(&counts, counts.len()).to_bits().to_string()
        }
        "learn-rng" => {
            let seed = number(args);
            let bi = number(args);
            let ei = number(args);
            let bu = number(args);
            let eu = number(args);
            let bf = f32::from_bits(number(args));
            let ef = f32::from_bits(number(args));
            let p = f32::from_bits(number(args));
            let n = number(args);
            let mut rng = Rng::new(seed);
            let dist = Rng::make_geometric(p);
            let mut out = dist.to_bits().to_string();
            for _ in 0..n {
                write!(
                    out,
                    " {} {} {} {} {} {}",
                    rng.next_u64(),
                    rng.uniform_i(bi, ei),
                    rng.uniform_u(bu, eu),
                    rng.uniform_f(bf, ef).to_bits(),
                    u32::from(rng.bernoulli(p)),
                    rng.geometric(&dist)
                )
                .unwrap();
            }
            out
        }
        "learn-shuffle" => {
            let seed = number(args);
            let n = number(args);
            let mut v: Vec<u32> = (0..n as u32 + 3).collect();
            let mut rng = Rng::new(seed);
            rng.shuffle(&mut v, n);
            let mut out = rng.next_u64().to_string();
            output(&mut out, &v);
            out
        }
        "learn-config" => {
            let n = number(args);
            let mut s = TreeSamples::default();
            s.set_predictor(Predictor::ZERO, TreeMode::Default).unwrap();
            s.set_properties(&[0, 1, 9], TreeMode::Default).unwrap();
            let mut out = "C".to_owned();
            for _ in 0..n {
                let p = Predictor(number(args));
                let m = mode(number(args));
                let props = values(args);
                let a = s.set_predictor(p, m);
                let b = s.set_properties(&props, m);
                write!(out, " {} {}", status_code(&a), status_code(&b)).unwrap();
                config(&mut out, &s);
            }
            out
        }
        "learn-samples" | "learn-split" => {
            let (threshold, fast) = if kind == "learn-split" {
                (f32::from_bits(number(args)), f32::from_bits(number(args)))
            } else {
                (0.0, 0.0)
            };
            let predictor = Predictor(number(args));
            let m = mode(number(args));
            let max = number(args);
            let props = values(args);
            let range = std::array::from_fn(|_| [number(args), number(args)]);
            let n = number(args);
            let mult: Vec<_> = (0..n)
                .map(|_| ModularMultiplierInfo {
                    range: std::array::from_fn(|_| [number(args), number(args)]),
                    multiplier: number(args),
                })
                .collect();
            let group = values(args);
            let channel = values(args);
            let mut pixel = values(args);
            let mut diff = values(args);
            let mut s = TreeSamples::default();
            s.set_predictor(predictor, m).unwrap();
            s.set_properties(&props, m).unwrap();
            s.pre_quantize_properties(&range, &mult, &group, &channel, &mut pixel, &mut diff, max);
            let mut out = "S".to_owned();
            output(&mut out, &pixel);
            output(&mut out, &diff);
            samples(&mut out, &s);
            let batches = number(args);
            for _ in 0..batches {
                let extra = number(args);
                let n = number(args);
                let repeat = number(args);
                s.prepare_for_samples(extra);
                let data: Vec<_> = (0..n)
                    .map(|_| (number::<i64>(args), values(args), values::<i64>(args)))
                    .collect();
                for _ in 0..repeat {
                    for (pixel, props, preds) in &data {
                        s.add_sample(*pixel, props, preds);
                    }
                }
                samples(&mut out, &s);
            }
            let swaps = number(args);
            for _ in 0..swaps {
                if s.num_distinct_samples() != 0 {
                    s.swap(0, s.num_distinct_samples() - 1);
                }
                samples(&mut out, &s);
            }
            if kind == "learn-split" {
                s.all_samples_done();
                let mut tree = Vec::new();
                let status = compute_best_tree(&mut s, threshold, &mult, range, fast, &mut tree);
                write!(out, " {}", status_code(&status)).unwrap();
                write!(out, " {}", tree.len()).unwrap();
                for n in tree {
                    write!(
                        out,
                        " {},{},{},{},{},{},{}",
                        n.property,
                        n.splitval,
                        n.lchild,
                        n.rchild,
                        n.predictor.0,
                        n.predictor_offset,
                        n.multiplier
                    )
                    .unwrap();
                }
                samples(&mut out, &s);
            }
            out
        }
        "learn-collect" => {
            let repeats = f32::from_bits(number(args));
            let group = number(args);
            let max_chan_size = number(args);
            let image = transforms::read_image(args);
            let mut gc = values(args);
            let mut cc = values(args);
            let mut px = values(args);
            let mut diff = values(args);
            collect_pixel_samples(
                &image,
                &ModularOptions {
                    nb_repeats: repeats,
                    max_chan_size,
                    ..Default::default()
                },
                group,
                &mut gc,
                &mut cc,
                &mut px,
                &mut diff,
            );
            let mut out = "P".to_owned();
            output(&mut out, &gc);
            output(&mut out, &cc);
            output(&mut out, &px);
            output(&mut out, &diff);
            out
        }
        _ => unreachable!(),
    }
}
fn tree_observation(
    out: &mut String,
    tree: &[jxl::jxl::modular::encoding::dec_ma::PropertyDecisionNode],
) {
    write!(out, " {}", tree.len()).unwrap();
    for n in tree {
        write!(
            out,
            " {},{},{},{},{},{},{}",
            n.property,
            n.splitval,
            n.lchild,
            n.rchild,
            n.predictor.0,
            n.predictor_offset,
            n.multiplier
        )
        .unwrap();
    }
}

#[test]
fn learning_rng_distributions_and_shuffle_match_every_state_transition() {
    let mut cases = Vec::new();
    for seed in [0, 1, 19, u64::MAX, 0xdeadbeef01234567] {
        for p in [0.00001f32, 0.01, 0.05, 0.5, 0.99, 1.0] {
            for (lo, hi) in [(-1.0f32, 1.0f32), (-127.25, 65535.75), (0.0, 1.0)] {
                cases.push(format!(
                    "learn-rng {seed} -10000000000 10000000000 7 {} {} {} {} 200",
                    u64::MAX,
                    lo.to_bits(),
                    hi.to_bits(),
                    p.to_bits()
                ));
            }
        }
        for n in [0, 1, 2, 3, 17, 257, 1024] {
            cases.push(format!("learn-shuffle {seed} {n}"));
        }
    }
    compare(cases);
}
#[test]
fn learning_configuration_preserves_append_order_and_failed_setter_state() {
    let mut cases = Vec::new();
    for p in [0, 5, 6, 13, 14, 15, u32::MAX] {
        for m in 0..4 {
            for props in [
                "0",
                "1 15",
                "3 0 1 9",
                "3 1 0 15",
                "4 0 0 1 1",
                "1 4294967295",
            ] {
                cases.push(format!(
                    "learn-config 3 {p} {m} {props} 15 3 3 0 1 15 15 3 0"
                ));
            }
        }
    }
    compare(cases);
}
fn sample_request(
    p: u32,
    m: usize,
    max: usize,
    variant: usize,
    include_multipliers: bool,
) -> String {
    let props = match variant % 3 {
        0 => "20 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19",
        1 => "5 4 6 9 15 18",
        _ => "3 0 1 9",
    };
    let mult = if variant % 2 == 0 && include_multipliers {
        "2 0 2 0 8 1 2 4 0 8 3"
    } else {
        "0"
    };
    let mut out = format!(
        "learn-samples {p} {m} {max} {props} 0 4 0 8 {mult} 8 7 0 19 3 1 0 11 5 4 3 17 0 8 10 -31 -7 -1 0 1 3 7 19 31 511 10 -33 -11 -3 -1 0 2 4 17 255 512 3"
    );
    for b in 0..3 {
        let repeat = if variant == 199 {
            if b == 0 { 65536 } else { 3 }
        } else {
            1 + (variant + b) % 7
        };
        let count = if variant == 199 { 1 } else { 4 };
        let extra = count * repeat + 64;
        write!(out, " {extra} {count} {repeat}").unwrap();
        for i in 0..count {
            let pixel = if variant == 199 {
                17
            } else {
                [i32::MIN as i64, 0, 127, 2147483647][i]
            };
            write!(out, " {pixel} 20").unwrap();
            for q in 0..20 {
                let v = if q == 0 {
                    i % 4
                } else if q == 1 {
                    i % 8
                } else {
                    (i * 17 + q * 31) % 1025
                };
                write!(out, " {}", if q < 2 { v as i32 } else { v as i32 - 512 }).unwrap();
            }
            out.push_str(" 14");
            for pred in 0..14 {
                write!(
                    out,
                    " {}",
                    if variant == 199 {
                        0
                    } else {
                        (i as i64 - pred) * 111
                    }
                )
                .unwrap();
            }
        }
    }
    out.push_str(" 2");
    out
}
#[test]
fn learning_samples_quantization_dedup_growth_saturation_and_swaps_match_source() {
    let mut cases = Vec::new();
    for i in 0..200 {
        let m = i % 4;
        let p = if m == 2 {
            5
        } else {
            [0, 5, 6, 14, 15][(i / 4) % 5]
        };
        cases.push(sample_request(
            p,
            m,
            [1, 2, 16, 31, 32, 63, 64, 256][i % 8],
            i,
            true,
        ));
    }
    compare(cases);
}
#[test]
fn learning_scalar_entropy_and_best_split_match_cost_bits_and_sample_order() {
    let mut cases = Vec::new();
    let mut seed = 88313;
    for n in [0, 1, 2, 3, 7, 16, 33, 128, 256] {
        for i in 0..80 {
            let mut out = format!("learn-estimate {n}");
            for _ in 0..n {
                let count = if i % 11 == 0 {
                    0
                } else {
                    (next(&mut seed) % 1000000) as i32
                };
                write!(out, " {count}").unwrap();
            }
            cases.push(out);
        }
    }
    compare(cases);
    let mut cases = Vec::new();
    for i in 0..200 {
        let mode = i % 4;
        let pred = if mode == 2 {
            5
        } else {
            [0, 5, 6, 14, 15][(i / 4) % 5]
        };
        let s = sample_request(
            pred,
            mode,
            [2, 16, 32, 64, 256][i % 5],
            i,
            mode > 1 && i % 3 != 1,
        );
        let threshold = [0.0f32, 0.5, 17.0, 96.0, 1000.0][i % 5];
        let fast = [1.0f32, 1.01, 1.25][i % 3];
        cases.push(s.replacen(
            "learn-samples",
            &format!("learn-split {} {}", threshold.to_bits(), fast.to_bits()),
            1,
        ));
    }
    compare(cases);
}
#[test]
fn learning_pixel_sampling_keeps_counts_rng_draws_and_existing_prefixes() {
    let mut cases = Vec::new();
    let mut seed = 882731;
    for i in 0..500 {
        let w = [0, 1, 2, 3, 9, 17, 65][i % 7];
        let h = [0, 1, 3, 9, 17][(i / 7) % 5];
        let n = 1 + i % 4;
        let meta = i % (n + 1);
        let mut image = format!("{w} {h} 16 {meta} 0 {n}");
        for c in 0..n {
            let cw = if i % 13 == 0 && c == n - 1 { w / 2 } else { w };
            write!(image, " {cw} {h} 0 0 {c}").unwrap();
            for _ in 0..cw * h {
                write!(image, " {}", next(&mut seed) as i32).unwrap();
            }
        }
        cases.push(format!(
            "learn-collect {} {} {} {image} 2 17 19 1 23 2 -7 11 1 -13",
            [0.0f32, 0.01, 0.5, 1.0, 4.0][i % 5].to_bits(),
            i % 9,
            if i % 11 == 0 { w / 2 } else { 1000 }
        ));
    }
    compare(cases);
}

struct Parameters {
    kind: u32,
    pred: u32,
    wp: i32,
    mode: usize,
    refs: i32,
    max_values: usize,
    repeats: f32,
    threshold: f32,
    fast: f32,
    skip: bool,
    zero: bool,
    max_channel: usize,
    huffman: bool,
    lz: usize,
    cluster: usize,
    uint: usize,
    strategy: usize,
    props: Vec<u32>,
}
impl Default for Parameters {
    fn default() -> Self {
        Self {
            kind: 1,
            pred: 15,
            wp: 0,
            mode: 3,
            refs: 0,
            max_values: 32,
            repeats: 0.5,
            threshold: 96.0,
            fast: 1.01,
            skip: false,
            zero: false,
            max_channel: 16777215,
            huffman: false,
            lz: 1,
            cluster: 2,
            uint: 4,
            strategy: 2,
            props: vec![0, 1, 15, 9, 10, 11, 12, 13],
        }
    }
}
fn parameters(p: &Parameters) -> String {
    let mut out = format!(
        "{} {} {} {} {} {} {} {} {} {} {} {} 536870911 {} {} {} {} {}",
        p.kind,
        p.pred,
        p.wp,
        p.mode,
        p.refs,
        p.max_values,
        p.repeats.to_bits(),
        p.threshold.to_bits(),
        p.fast.to_bits(),
        u32::from(p.skip),
        u32::from(p.zero),
        p.max_channel,
        u32::from(p.huffman),
        p.lz,
        p.cluster,
        p.uint,
        p.strategy
    );
    output(&mut out, &p.props);
    out
}
fn raster(w: usize, h: usize, n: usize, depth: i32, pattern: usize, seed: &mut u64) -> String {
    let mut out = format!("{w} {h} {depth} 0 0 {n}");
    for c in 0..n {
        write!(out, " {w} {h} 0 0 {c}").unwrap();
        for y in 0..h {
            for x in 0..w {
                let v = match pattern {
                    0 => (c * 73) as i32,
                    1 => (c * 737 + y * 127 + x * 3) as i32,
                    _ => next(seed) as i32,
                };
                write!(out, " {}", v & ((1i32 << depth) - 1)).unwrap();
            }
        }
    }
    out
}
#[test]
fn learning_image_batches_match_final_trees_with_sampling_refs_and_multiplier_ranges() {
    let mut cases = Vec::new();
    let mut seed = 798877;
    for i in 0..220 {
        let count = 1 + i % 3;
        let start = if count > 1 && i % 7 == 0 { 1 } else { 0 };
        let stop = if i % 19 == 0 { start } else { count };
        let mut out = format!("learn-tree {start} {stop} {count}");
        for group in 0..count {
            let mut p = Parameters {
                pred: [0, 5, 6, 14, 15][i % 5],
                wp: ((i + group) % 5) as i32,
                mode: (i / 5) % 4,
                refs: ((i / 20) % 3) as i32,
                max_values: [1, 16, 32, 64, 256][i % 5],
                repeats: [0.0, 0.01, 0.5, 1.0][(i + group) % 4],
                threshold: [0.0, 17.0, 96.0, 400.0][i % 4],
                ..Default::default()
            };
            if p.refs > 0 {
                p.props = vec![0, 1, 4, 6, 9, 15, 16, 17, 18, 19];
            }
            if i % 31 == 0 {
                p.refs = -1;
            }
            if i % 37 == 0 {
                p.props = vec![15];
                p.mode = 2;
            }
            let (w, h) = [(1, 1), (3, 5), (17, 9), (65, 33)][(i + group) % 4];
            let n = if i % 2 == 0 { 1 } else { 3 };
            write!(
                out,
                " {} {}",
                parameters(&p),
                raster(w, h, n, 16, i % 3, &mut seed)
            )
            .unwrap();
        }
        let force = stop > start && (i / 5) % 4 == 3 && i % 2 == 0 && i % 37 != 0;
        if force {
            write!(out, " 2 0 1 {start} {stop} 1 1 3 {start} {stop} 2").unwrap();
        } else {
            out.push_str(" 0");
        }
        cases.push(out);
    }
    compare(cases);
}
#[test]
fn learning_generic_encoder_matches_complete_bytes_accounting_and_decoded_rasters() {
    let mut cases = Vec::new();
    let mut seed = 885533;
    for i in 0..400 {
        let (w, h) = [(1, 1), (2, 3), (9, 5), (17, 9), (65, 17)][i % 5];
        let n = if (i / 5) % 2 == 0 { 1 } else { 3 };
        let mut p = Parameters {
            kind: (i % 7) as u32,
            pred: [0, 5, 6, 14, 15, u32::MAX][(i / 7) % 6],
            wp: (i % 5) as i32,
            mode: (i / 42) % 4,
            refs: ((i / 84) % 2) as i32,
            max_values: [16, 32, 64][i % 3],
            repeats: [0.0, 0.01, 0.5, 1.0][i % 4],
            threshold: [0.0, 17.0, 96.0][i % 3],
            skip: i % 13 == 0,
            huffman: i % 2 == 0,
            lz: if i % 3 == 0 { 0 } else { 1 },
            cluster: [1, 2][i % 2],
            uint: [0, 1, 2, 4][i % 4],
            strategy: i % 3,
            ..Default::default()
        };
        if p.refs > 0 {
            p.props = vec![0, 1, 4, 6, 9, 15, 16, 17, 18, 19];
        }
        cases.push(format!(
            "learn-generic {} {} {} 1 {} {}",
            i % 8,
            i % 11,
            (i / 10) % 4,
            parameters(&p),
            raster(w, h, n, [8, 16][i % 2], i % 3, &mut seed)
        ));
    }
    for i in 0..60 {
        let mut p = Parameters {
            kind: 1,
            pred: 5,
            refs: if i % 3 == 0 { -1 } else { 0 },
            zero: i % 7 == 0,
            max_channel: if i % 5 == 0 { 0 } else { 16777215 },
            ..Default::default()
        };
        if i % 11 == 0 {
            p.props.clear();
        }
        let mut image = raster(
            if i % 13 == 0 { 0 } else { 3 },
            2,
            if i % 17 == 0 { 0 } else { 3 },
            8,
            1,
            &mut seed,
        );
        if i % 19 == 0 {
            let mut fields: Vec<_> = image.split_whitespace().map(str::to_owned).collect();
            fields[4] = "1".to_owned();
            image = fields.join(" ");
        }
        cases.push(format!(
            "learn-generic {} 0 0 0 {} {image}",
            i % 8,
            parameters(&p)
        ));
    }
    compare(cases);
}
