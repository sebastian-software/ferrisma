// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Complete tree streams and source tree/filter/LUT/header state observations.

use super::*;
use jxl::jxl::dec_ans::{ANSCode, ANSSymbolReader, decode_histograms};
use jxl::jxl::enc_ans::{
    EntropyEncodingData, Token, build_and_encode_histograms, write_tokens_with_allotment,
};
use jxl::jxl::enc_ans_params::{HistogramParams, HybridUintMethod, LZ77Method};
use jxl::jxl::modular::encoding::context_predict::{FlatTree, weighted};
use jxl::jxl::modular::encoding::dec_ma::{PropertyDecisionNode, Tree, decode_tree};
use jxl::jxl::modular::encoding::enc_encoding::{modular_compress, predefined_tree};
use jxl::jxl::modular::encoding::enc_ma::tokenize_tree;
use jxl::jxl::modular::encoding::encoding::*;
use jxl::jxl::modular::encoding::ma_common::*;
use jxl::jxl::modular::modular_image::Image;
use jxl::jxl::modular::options::Predictor;
use jxl::jxl::modular::options::{ModularOptions, TreeKind};
use jxl::jxl::modular::transform::transform::{Transform, TransformId};
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("ma-")
}
fn read_tree(args: &mut std::str::SplitWhitespace<'_>) -> Tree {
    let n = number(args);
    (0..n)
        .map(|_| {
            PropertyDecisionNode::new(
                number(args),
                number(args),
                number(args),
                number(args),
                Predictor(number(args)),
                number(args),
                number(args),
            )
        })
        .collect()
}
fn tree(out: &mut String, tree: &Tree) {
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
fn tokens(out: &mut String, tokens: &[Token]) {
    write!(out, " {}", tokens.len()).unwrap();
    for t in tokens {
        write!(
            out,
            " {},{},{}",
            t.context(),
            t.value,
            u32::from(t.is_lz77_length())
        )
        .unwrap();
    }
}
fn seed_tree() -> Tree {
    vec![PropertyDecisionNode::leaf(Predictor::LEFT, 77, 3)]
}
fn decode(out: &mut String, br: &mut BitReader<'_>, limit: usize) {
    let mut t = seed_tree();
    let s = decode_tree(br, &mut t, limit);
    write!(
        out,
        " {} {} {}",
        status_code(&s),
        br.total_bits_consumed(),
        status_code(&br.close())
    )
    .unwrap();
    tree(out, &t);
}
fn lut<T: TreeLutWord + std::fmt::Display + PartialEq, const O: bool, const M: bool>(
    out: &mut String,
    flat: &FlatTree,
) {
    let mut l = TreeLut::<T, O, M>::default();
    l.context_lookup.fill(T::from_context(197));
    l.offsets.fill(73);
    l.multipliers.fill(47);
    let ok = tree_to_lookup_table(flat, &mut l);
    write!(out, " {}", u32::from(ok)).unwrap();
    for i in 0..l.context_lookup.len() {
        let off = if O { l.offsets[i] } else { 0 };
        let mul = if M { l.multipliers[i] } else { 0 };
        if i == 0
            || l.context_lookup[i] != l.context_lookup[i - 1]
            || (O && off != l.offsets[i - 1])
            || (M && mul != l.multipliers[i - 1])
        {
            write!(out, " {i},{},{off},{mul}", l.context_lookup[i]).unwrap();
        }
    }
    out.push_str(" E");
}
fn luts<T: TreeLutWord + std::fmt::Display + PartialEq>(
    out: &mut String,
    flags: u32,
    flat: &FlatTree,
) {
    match flags {
        0 => lut::<T, false, false>(out, flat),
        1 => lut::<T, true, false>(out, flat),
        2 => lut::<T, false, true>(out, flat),
        3 => lut::<T, true, true>(out, flat),
        _ => unreachable!(),
    }
}
fn header_seed(count: usize) -> GroupHeader {
    let mut h = GroupHeader {
        use_global_tree: true,
        ..GroupHeader::default()
    };
    weighted::predictor_mode(2, &mut h.wp_header);
    for i in 0..count {
        let mut t = Transform::new(TransformId((i % 3) as u32));
        t.begin_c = (i % 32) as u32;
        t.rct_type = (i % 42) as u32;
        t.num_c = (1 + i % 4) as u32;
        t.nb_colors = (i % 257) as u32;
        t.nb_deltas = (i % 17) as u32;
        t.predictor = Predictor((i % 14) as u32);
        t.max_delta_error = 77;
        t.ordered_palette = false;
        t.lossy_palette = true;
        if i % 3 == 2 {
            t.squeezes = (0..i % 4)
                .map(
                    |_| jxl::jxl::modular::transform::squeeze_params::SqueezeParams {
                        horizontal: i % 2 != 0,
                        in_place: i % 2 == 0,
                        begin_c: (i % 16) as u32,
                        num_c: (1 + i % 16) as u32,
                    },
                )
                .collect();
        }
        h.transforms.push(t);
    }
    h
}
fn header(out: &mut String, h: &mut GroupHeader) {
    write!(out, " {}", u32::from(h.use_global_tree)).unwrap();
    predict::header(out, &h.wp_header);
    write!(out, " {}", h.transforms.len()).unwrap();
    for t in &mut h.transforms {
        t.max_delta_error = 77;
        image::transform(out, t);
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Keep the original reader/image/tree fixture inputs explicit"
)]
fn raster_decode(
    out: &mut String,
    br: &mut BitReader<'_>,
    w: usize,
    h: usize,
    n: i32,
    group: usize,
    preset: i32,
    t: &Tree,
) {
    let mut image = Image::create(w, h, 16, n).unwrap();
    for c in &mut image.channel {
        image::paint(&mut c.plane, 81 + c.component as u64);
    }
    let mut code = ANSCode::default();
    let mut map = Vec::new();
    let mut header = weighted::Header::default();
    weighted::predictor_mode(preset, &mut header);
    let s = decode_histograms(br, t.len().div_ceil(2), &mut code, &mut map, false);
    write!(out, " {} {}", status_code(&s), br.total_bits_consumed()).unwrap();
    if s.is_ok() {
        let mut reader = ANSSymbolReader::create(&code, br, w).unwrap();
        write!(
            out,
            " {} {}",
            u32::from(reader.uses_lz77()),
            u32::from(reader.is_huff_rle_only())
        )
        .unwrap();
        let mut lut = TreeLut::<u8, false, false>::default();
        let mut run = 0;
        let mut value = 0;
        for c in 0..n {
            let s = decode_modular_channel_maans(
                br,
                &mut reader,
                &map,
                t,
                &header,
                c,
                group,
                &mut lut,
                &mut image,
                &mut run,
                &mut value,
            );
            write!(
                out,
                " {} {} {run} {value} {}",
                status_code(&s),
                br.total_bits_consumed(),
                status_code(&br.all_reads_within_bounds())
            )
            .unwrap();
            if s.is_err() {
                break;
            }
        }
        write!(out, " {}", u32::from(reader.check_ans_final_state())).unwrap();
    }
    write!(
        out,
        " {} {}",
        br.total_bits_consumed(),
        status_code(&br.close())
    )
    .unwrap();
    image::modular(out, &image);
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "ma-raster" | "ma-raster-read" => {
            let w = number(args);
            let h = number(args);
            let n = number(args);
            let group = number(args);
            let preset = number(args);
            if kind == "ma-raster" {
                let force = number::<u32>(args) != 0;
                let lz = number::<u32>(args);
                let skip = number::<u32>(args) != 0;
                let offset = number(args);
                let mut image = Image::create(w, h, 16, n).unwrap();
                rct::fill(&mut image, args);
                let input = read_tree(args);
                let mut t = Vec::new();
                tokenize_tree(&input, &mut Vec::new(), &mut t).unwrap();
                let options = ModularOptions {
                    predictor: Predictor::WEIGHTED,
                    wp_mode: preset,
                    skip_encoder_fast_path: skip,
                    ..Default::default()
                };
                let mut h = GroupHeader::default();
                let mut ts = vec![Vec::new()];
                let mut width = 0;
                let s =
                    modular_compress(&image, &options, group, &t, &mut h, &mut ts[0], &mut width);
                let mut out = status_code(&s).to_string();
                if s.is_err() {
                    return out;
                }
                let p = HistogramParams {
                    force_huffman: force,
                    image_widths: vec![width],
                    lz77_method: if lz != 0 {
                        LZ77Method::RLE
                    } else {
                        LZ77Method::None
                    },
                    uint_method: if lz == 2 {
                        HybridUintMethod::Uint000
                    } else {
                        HybridUintMethod::Best
                    },
                    ..Default::default()
                };
                let mut writer = BitWriter::new();
                prefix(&mut writer, offset);
                let mut code = EntropyEncodingData::default();
                let mut s = build_and_encode_histograms(
                    &p,
                    t.len().div_ceil(2),
                    &mut ts,
                    &mut code,
                    Some(&mut writer),
                    LayerType::ModularGlobal,
                    None,
                )
                .map(|_| ());
                if s.is_ok() {
                    s = write_tokens_with_allotment(
                        &ts[0],
                        &code,
                        0,
                        &mut writer,
                        LayerType::ModularGlobal,
                        None,
                    );
                }
                write!(out, " {} {}", status_code(&s), writer.bits_written()).unwrap();
                pad(&mut writer);
                write!(out, " {}", hex(writer.get_span())).unwrap();
                let mut br = BitReader::new(writer.get_span());
                br.skip_bits(offset);
                raster_decode(&mut out, &mut br, w, image.h, n, group, preset, &t);
                out
            } else {
                let offset = number(args);
                let input = read_tree(args);
                let mut t = Vec::new();
                tokenize_tree(&input, &mut Vec::new(), &mut t).unwrap();
                let data = unhex(args.next().unwrap());
                let mut br = BitReader::new(&data);
                br.skip_bits(offset);
                let mut out = "R".to_owned();
                raster_decode(&mut out, &mut br, w, h, n, group, preset, &t);
                out
            }
        }
        "ma-predefined" => {
            let kind = [
                TreeKind::TrivialTreeNoPredictor,
                TreeKind::Learn,
                TreeKind::JpegTranscodeACMeta,
                TreeKind::FalconACMeta,
                TreeKind::ACMeta,
                TreeKind::WPFixedDC,
                TreeKind::GradientFixedDC,
            ][number::<usize>(args)];
            let t = predefined_tree(kind, number(args), number(args), number(args));
            let mut out = "T".to_owned();
            tree(&mut out, &t);
            out
        }
        "ma-compress" => {
            let w = number(args);
            let h = number(args);
            let n = number(args);
            let depth = number(args);
            let error = number::<u32>(args) != 0;
            let group = number(args);
            let wp_mode = number(args);
            let predictor = Predictor(number(args));
            let skip_encoder_fast_path = number::<u32>(args) != 0;
            let zero_tokens = number::<u32>(args) != 0;
            let max_chan_size = number(args);
            let meta = number(args);
            let variant = number::<u32>(args);
            let mut image = Image::create(w, h, depth, n).unwrap();
            rct::fill(&mut image, args);
            image.error = error;
            image.nb_meta_channels = meta;
            if image.channel.len() > 1 {
                match variant {
                    1 => image.channel[1].w = 0,
                    2 => image.channel[1].hshift = 1,
                    3 => image.channel[1].h = 0,
                    _ => (),
                }
            }
            image.transform.push(Transform {
                max_delta_error: 77,
                ..Transform::new(TransformId::RCT)
            });
            let t = read_tree(args);
            let opts = ModularOptions {
                wp_mode,
                predictor,
                skip_encoder_fast_path,
                zero_tokens,
                max_chan_size,
                ..Default::default()
            };
            let mut h = header_seed(3);
            let mut width = 314159;
            let mut ts = vec![Token::new(5, 0xdeadbeef)];
            let s = modular_compress(&image, &opts, group, &t, &mut h, &mut ts, &mut width);
            let mut out = format!("{} {width}", status_code(&s));
            header(&mut out, &mut h);
            tokens(&mut out, &ts);
            out
        }
        "ma-token" => {
            let t = read_tree(args);
            let mut ts = vec![Token::new(5, 0xdeadbeef)];
            let mut dt = seed_tree();
            let s = tokenize_tree(&t, &mut ts, &mut dt);
            let mut out = status_code(&s).to_string();
            tokens(&mut out, &ts);
            tree(&mut out, &dt);
            out
        }
        "ma-write" | "ma-stream" => {
            let offset = number(args);
            let limit = number(args);
            let force = number::<u32>(args) != 0;
            let lz = number::<u32>(args) != 0;
            let mut ts = vec![Vec::new()];
            let mut dt = Vec::new();
            let s = if kind == "ma-write" {
                let t = read_tree(args);
                tokenize_tree(&t, &mut ts[0], &mut dt)
            } else {
                let n = number(args);
                for _ in 0..n {
                    ts[0].push(Token::new(number(args), number(args)));
                }
                Ok(())
            };
            let mut out = status_code(&s).to_string();
            tokens(&mut out, &ts[0]);
            tree(&mut out, &dt);
            if s.is_err() {
                return out;
            }
            let mut writer = BitWriter::new();
            prefix(&mut writer, offset);
            let p = HistogramParams {
                force_huffman: force,
                lz77_method: if lz {
                    LZ77Method::RLE
                } else {
                    LZ77Method::None
                },
                ..Default::default()
            };
            let mut code = EntropyEncodingData::default();
            let mut s = build_and_encode_histograms(
                &p,
                NUM_TREE_CONTEXTS,
                &mut ts,
                &mut code,
                Some(&mut writer),
                LayerType::ModularTree,
                None,
            )
            .map(|_| ());
            if s.is_ok() {
                s = write_tokens_with_allotment(
                    &ts[0],
                    &code,
                    0,
                    &mut writer,
                    LayerType::ModularTree,
                    None,
                );
            }
            write!(out, " {} {}", status_code(&s), writer.bits_written()).unwrap();
            pad(&mut writer);
            write!(out, " {}", hex(writer.get_span())).unwrap();
            let mut br = BitReader::new(writer.get_span());
            br.skip_bits(offset);
            decode(&mut out, &mut br, limit);
            out
        }
        "ma-read" => {
            let offset = number(args);
            let limit = number(args);
            let data = unhex(args.next().unwrap());
            let mut br = BitReader::new(&data);
            br.skip_bits(offset);
            let mut out = "R".to_owned();
            decode(&mut out, &mut br, limit);
            out
        }
        "ma-filter" => {
            let mut props = [number(args), number(args)];
            let width = number::<u32>(args);
            let flags = number(args);
            let t = read_tree(args);
            let mut n = 0;
            let mut wp = false;
            let mut only = false;
            let mut grad = false;
            let flat = filter_tree(&t, &mut props, &mut n, &mut wp, &mut only, &mut grad);
            let mut out = format!(
                "{n} {} {} {} {} {} {}",
                u32::from(wp),
                u32::from(only),
                u32::from(grad),
                props[0],
                props[1],
                flat.len()
            );
            for v in &flat {
                write!(out, " {},{}", v.property0, v.child_id).unwrap();
                if v.property0 == -1 {
                    write!(
                        out,
                        ",{},{},{}",
                        v.predictor().0,
                        v.predictor_offset(),
                        v.multiplier()
                    )
                    .unwrap();
                } else {
                    write!(
                        out,
                        ",{},{},{},{},{}",
                        v.splitval0(),
                        v.properties()[0],
                        v.properties()[1],
                        v.splitvals()[0],
                        v.splitvals()[1]
                    )
                    .unwrap();
                }
            }
            match width {
                8 => luts::<u8>(&mut out, flags, &flat),
                16 => luts::<u16>(&mut out, flags, &flat),
                32 => luts::<u32>(&mut out, flags, &flat),
                _ => (),
            }
            out
        }
        "ma-group" | "ma-group-read" => {
            let count = number(args);
            let offset = number(args);
            let mut h = header_seed(count);
            if kind == "ma-group-read" {
                let data = unhex(args.next().unwrap());
                let mut br = BitReader::new(&data);
                br.skip_bits(offset);
                let r = bundle::read(&mut br, &mut h);
                let mut out = format!(
                    "{} {} {}",
                    status_code(&r),
                    br.total_bits_consumed(),
                    status_code(&br.close())
                );
                header(&mut out, &mut h);
                out
            } else {
                let can = bundle::can_encode(&mut h);
                let (ext, bits) = can.unwrap_or((0, 0));
                let mut out = format!("{} {ext} {bits}", status_code(&can.map(|_| ())));
                if can.is_ok() {
                    let mut w = BitWriter::new();
                    prefix(&mut w, offset);
                    let s = bundle::write(&mut h, &mut w, LayerType::Header, None);
                    write!(out, " {} {}", status_code(&s), w.bits_written()).unwrap();
                    pad(&mut w);
                    write!(out, " {}", hex(w.get_span())).unwrap();
                    let mut decoded = header_seed(3);
                    let mut br = BitReader::new(w.get_span());
                    br.skip_bits(offset);
                    let r = bundle::read(&mut br, &mut decoded);
                    write!(
                        out,
                        " {} {} {}",
                        status_code(&r),
                        br.total_bits_consumed(),
                        status_code(&br.close())
                    )
                    .unwrap();
                    header(&mut out, &mut decoded);
                }
                header(&mut out, &mut h);
                out
            }
        }
        _ => unreachable!(),
    }
}

#[test]
fn ma_predefined_trees_keep_source_cutoffs_and_size_reduction() {
    let mut cases = Vec::new();
    for kind in 0..7 {
        for pixels in [
            1,
            2,
            511,
            512,
            1023,
            1024,
            1025,
            2048,
            4096,
            8192,
            8193,
            16383,
            16384,
            16385,
            1 << 20,
            usize::MAX,
        ] {
            for depth in [-1, 1, 8, 11, 12, 14, 15, 16, 32, i32::MAX] {
                for prev in [-1, 0, 1] {
                    cases.push(format!("ma-predefined {kind} {pixels} {depth} {prev}"));
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn ma_residual_encoder_preserves_fast_slow_and_error_token_state() {
    let mut cases = Vec::new();
    let mut seed = 778822;
    for i in 0..1400 {
        let w = [1, 2, 3, 7, 8, 9, 17, 33][i % 8];
        let h = 1 + i % 5;
        let n = 1 + i % 4;
        let p = (i / 8) % 14;
        let skip = (i / 112) % 2;
        let variant = (i / 224) % 4;
        let mut t = if i % 3 == 0 {
            generated_tree(2 + i % 3, &mut seed)
        } else {
            vec![PropertyDecisionNode::leaf(
                Predictor(p as u32),
                if i % 13 == 0 { 17 } else { 0 },
                if i % 17 == 0 { 4 } else { 1 },
            )]
        };
        let mut id = 0;
        let multiple_nodes = t.len() > 1;
        for v in &mut t {
            if v.property == -1 {
                v.lchild = id;
                id += 1;
                if multiple_nodes {
                    v.multiplier = 1;
                }
            }
        }
        let mut out = format!(
            "ma-compress {w} {h} {n} 16 0 {} {} {} {skip} {} {} {} {variant}",
            i % 11,
            i % 5,
            if i % 7 == 0 { u32::MAX } else { p as u32 },
            u32::from(i % 29 == 0),
            if i % 31 == 0 { w - 1 } else { 1000 },
            i % n
        );
        for c in 0..n {
            for y in 0..h {
                for x in 0..w {
                    let value = if i % 11 == 0 {
                        (next(&mut seed) as i32) >> 1
                    } else {
                        ((x * 3 + y * 13 + c * 17) % 65536) as i32 - 32768
                    };
                    write!(out, " {value}").unwrap();
                }
            }
        }
        write!(out, " {}", request_tree(&t)).unwrap();
        cases.push(out);
    }
    // Every basic predictor, all raster widths, both dispatch modes, and full
    // signed samples. Optimized overflow failures keep already written tokens.
    for p in 0..14 {
        for skip in 0..2 {
            for w in [1, 2, 9, 17] {
                let t = vec![PropertyDecisionNode::leaf(Predictor(p), 0, 1)];
                let mut out = format!("ma-compress {w} 3 2 32 0 0 0 {p} {skip} 0 1000 0 0");
                for i in 0..w * 6 {
                    write!(out, " {}", [i32::MIN, i32::MAX, 0, -1, 1][i % 5]).unwrap();
                }
                write!(out, " {}", request_tree(&t)).unwrap();
                cases.push(out);
            }
        }
    }
    for &(w, h, n) in &[(0, 3, 3), (3, 0, 3), (3, 3, 0), (3, 3, -1), (3, 3, 3)] {
        for error in 0..2 {
            for zero in 0..2 {
                let mut out = format!("ma-compress {w} {h} {n} 8 {error} 0 0 5 0 {zero} 2 1 0");
                for _ in 0..w * h * n.max(0) {
                    out.push_str(" 17");
                }
                write!(
                    out,
                    " {}",
                    request_tree(&[PropertyDecisionNode::leaf(Predictor::GRADIENT, 0, 1)])
                )
                .unwrap();
                cases.push(out);
            }
        }
    }
    compare(cases);
}
#[test]
fn ma_channel_rasters_cross_decode_real_entropy_bytes_and_damage() {
    let mut cases = Vec::new();
    let mut configs = Vec::new();
    let mut seed = 732442;
    let mut trees: Vec<Tree> = (0..14)
        .map(|p| vec![PropertyDecisionNode::leaf(Predictor(p), 0, 1)])
        .collect();
    for prop in [0, 1, 9, 15, 18, 22] {
        trees.push(vec![
            PropertyDecisionNode::split(prop, 0, 1, 2),
            PropertyDecisionNode::leaf(
                if prop == 15 {
                    Predictor::WEIGHTED
                } else {
                    Predictor::GRADIENT
                },
                0,
                1,
            ),
            PropertyDecisionNode::leaf(
                if prop == 15 {
                    Predictor::WEIGHTED
                } else {
                    Predictor::GRADIENT
                },
                0,
                1,
            ),
        ]);
    }
    // A static mixture must not enable the global gradient-only Huffman/RLE
    // specialization. Its shared run state spans successive channel calls.
    trees.push(vec![
        PropertyDecisionNode::split(0, 0, 1, 2),
        PropertyDecisionNode::leaf(Predictor::LEFT, 0, 1),
        PropertyDecisionNode::leaf(Predictor::GRADIENT, 0, 1),
    ]);
    for prop in [9, 15, 18] {
        trees.push(vec![
            PropertyDecisionNode::split(prop, 31, 1, 2),
            PropertyDecisionNode::leaf(Predictor::AVERAGE4, -7, 1),
            PropertyDecisionNode::leaf(Predictor::WEIGHTED, 13, 1),
        ]);
    }
    for (ti, t) in trees.iter().enumerate() {
        for w in [1, 2, 9, 17] {
            for mode in 0..8 {
                let h = 5;
                let n = if mode % 3 == 0 { 3 } else { 1 };
                let group = ti % 7;
                let preset = mode % 5;
                let offset = (ti + mode) % 8;
                let mut request = format!(
                    "ma-raster {w} {h} {n} {group} {preset} {} {} {} {offset}",
                    mode % 2,
                    (mode / 2) % 2,
                    mode / 4
                );
                for c in 0..n {
                    for y in 0..h {
                        for x in 0..w {
                            let v = match mode % 3 {
                                0 => 0,
                                1 => c * 17 + y * 3 + x * 2 - 19,
                                _ => (next(&mut seed) % 65536) as i32 - 32768,
                            };
                            write!(request, " {v}").unwrap();
                        }
                    }
                }
                write!(request, " {}", request_tree(t)).unwrap();
                cases.push(request);
                configs.push((w, h, n, group, preset, offset, t));
            }
        }
    }
    for ti in [5, 14, 20] {
        for h in [1, 3] {
            let t = &trees[ti];
            let w = 256;
            let n = 3;
            let mut request = format!("ma-raster {w} {h} {n} 0 0 1 2 0 7");
            for c in 0..n {
                for y in 0..h {
                    let mut value = if c == 0 { 0 } else { ((w - 1) / 8) * 16 };
                    for x in 0..w {
                        value += 8192 + (x / 8) * 16;
                        write!(request, " {}", value + y * 16384 + c * y * 4096).unwrap();
                    }
                }
            }
            write!(request, " {}", request_tree(t)).unwrap();
            cases.push(request);
            configs.push((w, h, n, 0, 0, 7, t));
        }
    }
    compare(cases.clone());
    let outputs = original_observations(&cases);
    assert!(
        outputs
            .iter()
            .any(|out| out.split_whitespace().nth(7) == Some("1")),
        "Exercise the original Huffman/RLE-only specialization"
    );
    let mut reads = Vec::new();
    for (i, output) in outputs.iter().enumerate().step_by(7) {
        let fields: Vec<_> = output.split_whitespace().collect();
        assert!(
            fields.len() >= 4,
            "Original encoder rejected raster {i}: {output}"
        );
        let data = unhex(fields[3]);
        let (w, h, n, group, preset, offset, t) = configs[i];
        let start = format!(
            "ma-raster-read {w} {h} {n} {group} {preset} {offset} {}",
            request_tree(t)
        );
        reads.push(format!("{start} {}", hex(&data)));
        for len in [
            0,
            1,
            2,
            3,
            4,
            5,
            data.len() / 2,
            data.len().saturating_sub(1),
        ] {
            reads.push(format!("{start} {}", hex(&data[..len.min(data.len())])));
        }
        for byte in [0, data.len() / 2, data.len() - 1] {
            let mut broken = data.clone();
            broken[byte] ^= 0x80;
            reads.push(format!("{start} {}", hex(&broken)));
        }
    }
    compare(reads);
}

fn request_tree(t: &[PropertyDecisionNode]) -> String {
    let mut out = t.len().to_string();
    for n in t {
        write!(
            out,
            " {} {} {} {} {} {} {}",
            n.property,
            n.splitval,
            n.lchild as i32,
            n.rchild as i32,
            n.predictor.0,
            n.predictor_offset,
            n.multiplier
        )
        .unwrap();
    }
    out
}
fn generated_tree(depth: usize, seed: &mut u64) -> Tree {
    fn add(t: &mut Tree, depth: usize, seed: &mut u64) -> usize {
        let idx = t.len();
        t.push(PropertyDecisionNode::default());
        let r = next(seed);
        if depth == 0 || r % 5 == 0 {
            t[idx] = PropertyDecisionNode::leaf(
                Predictor((r % 14) as u32),
                ((r >> 5) % 1024) as i64 - 512,
                1 << ((r >> 16) % 16),
            );
        } else {
            let l = add(t, depth - 1, seed);
            let rr = add(t, depth - 1, seed);
            let p = [0, 1, 2, 4, 8, 9, 15, 16, 17, 22, 255][(r >> 20) as usize % 11];
            t[idx] = PropertyDecisionNode::split(
                p,
                ((r >> 32) % 16000) as i32 - 8000,
                l as i32,
                rr as i32,
            );
        }
        idx
    }
    let mut t = Vec::new();
    add(&mut t, depth, seed);
    t
}
fn stream(prefix: usize, limit: usize, force: bool, ts: &[(u32, u32)]) -> String {
    let mut out = format!(
        "ma-stream {prefix} {limit} {} 0 {}",
        u32::from(force),
        ts.len()
    );
    for (c, v) in ts {
        write!(out, " {c} {v}").unwrap();
    }
    out
}
fn encoded_hex(out: &str) -> String {
    // Writer bytes immediately follow its status and bit count. Earlier tokens
    // and tree nodes carry commas; locate the plain hexadecimal field by shape.
    out.split_whitespace()
        .find(|s| s.len() > 8 && !s.contains(',') && s.bytes().all(|b| b.is_ascii_hexdigit()))
        .expect("encoded stream")
        .to_owned()
}
#[test]
fn ma_tokenization_preserves_bfs_widths_and_partial_failure() {
    let mut cases = Vec::new();
    let mut seed = 775;
    for i in 0..500 {
        cases.push(format!(
            "ma-token {}",
            request_tree(&generated_tree(i % 7, &mut seed))
        ));
    }
    for p in [-32768, -2, -1, 0, 255, 256, 32767] {
        for pred in [0, 13, 14, 15, u32::MAX] {
            let t = if p >= 0 {
                vec![
                    PropertyDecisionNode::split(p, 17, 1, 2),
                    PropertyDecisionNode::leaf(Predictor(pred), i64::MAX, 1 << 31),
                    PropertyDecisionNode::leaf(Predictor::TOP, i64::MIN, u32::MAX),
                ]
            } else {
                vec![PropertyDecisionNode::new(
                    p,
                    0,
                    0,
                    0,
                    Predictor(pred),
                    i64::MAX,
                    1,
                )]
            };
            cases.push(format!("ma-token {}", request_tree(&t)));
        }
    }
    compare(cases);
}
#[test]
fn ma_streams_cross_decode_and_reject_invalid_nodes() {
    let mut cases = Vec::new();
    let mut seed = 7715;
    for i in 0..240 {
        let t = generated_tree(i % 7, &mut seed);
        cases.push(format!(
            "ma-write {} {} {} {} {}",
            i % 8,
            if i % 5 == 0 {
                t.len() / 2
            } else {
                MAX_TREE_SIZE
            },
            i % 2,
            (i / 2) % 2,
            request_tree(&t)
        ));
    }
    for force in [false, true] {
        // Valid leaf, invalid predictor, multiplier boundaries, invalid property,
        // and the decoder's early infinite-tree rejection.
        for ts in [
            vec![(1, 0), (2, 5), (3, u32::MAX), (4, 0), (5, 0)],
            vec![(1, 0), (2, 14), (3, 0), (4, 0), (5, 0)],
            vec![(1, 0), (2, 0), (3, 0), (4, 31), (5, 0)],
            vec![(1, 0), (2, 0), (3, 0), (4, 30), (5, 1)],
            vec![(1, 0), (2, 0), (3, 0), (4, 30), (5, 0)],
            vec![(1, 0), (2, 0), (3, 0), (4, 0), (5, 2147483647)],
            vec![(1, 257)],
            vec![(1, 257), (1, 0)],
            vec![
                (1, 256),
                (0, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
            ],
            vec![(1, 1), (0, 0)],
            vec![
                (1, 2),
                (0, 0xffff_fffe),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
            ],
        ] {
            cases.push(stream(7, MAX_TREE_SIZE, force, &ts));
        }
        for depth in [2045, 2046, 2047, 2048] {
            let mut t = vec![PropertyDecisionNode::default(); 2 * depth + 1];
            for (i, node) in t.iter_mut().take(depth).enumerate() {
                *node = PropertyDecisionNode::split(
                    9,
                    i as i32,
                    (i + 1) as i32,
                    (depth + 1 + i) as i32,
                );
            }
            cases.push(format!(
                "ma-write 3 {MAX_TREE_SIZE} {} 0 {}",
                u32::from(force),
                request_tree(&t)
            ));
        }
    }
    compare(cases.clone());
    let source = original_observations(&cases[..80]);
    let mut reads = Vec::new();
    for (i, cpp) in source.iter().enumerate() {
        let data = unhex(&encoded_hex(cpp));
        let offset = i % 8;
        // Read original bytes in both implementations, not just Rust round trips.
        reads.push(format!("ma-read {offset} {MAX_TREE_SIZE} {}", hex(&data)));
        for len in [0, 1, 2, 3, 4, data.len() / 2, data.len().saturating_sub(1)] {
            reads.push(format!(
                "ma-read {offset} {MAX_TREE_SIZE} {}",
                hex(&data[..len.min(data.len())])
            ));
        }
    }
    let mut random_seed = 991;
    for len in 0..80 {
        for offset in 0..8 {
            let data: Vec<_> = (0..len).map(|_| next(&mut random_seed) as u8).collect();
            reads.push(format!("ma-read {offset} 257 {}", hex(&data)));
        }
    }
    compare(reads);
}
#[test]
fn ma_filtering_and_lookup_tables_preserve_every_partial_entry() {
    let mut cases = Vec::new();
    let mut seed = 90225;
    for i in 0..800 {
        let mut t = generated_tree(i % 7, &mut seed);
        // Assign leaf IDs as DecodeTree does; input order need not be BFS here.
        let mut id = 0;
        for n in &mut t {
            if n.property == -1 {
                n.lchild = id;
                id += 1;
            }
        }
        cases.push(format!(
            "ma-filter {} {} 0 0 {}",
            (i % 4) as i32 - 1,
            (i % 13) as i32 - 5,
            request_tree(&t)
        ));
    }
    for width in [8, 16, 32] {
        for flags in 0..4 {
            for split in [-8194, -8193, -8192, -1, 0, 8190, 8191, 8192] {
                for (offset, mul) in [
                    (-129, 1),
                    (-128, 1),
                    (0, 1),
                    (127, 127),
                    (128, 1),
                    (0, 128),
                    (0, 0),
                    (0, u32::MAX),
                ] {
                    let t = vec![
                        PropertyDecisionNode::split(9, split, 1, 2),
                        PropertyDecisionNode::new(-1, 0, 257, 0, Predictor::GRADIENT, offset, mul),
                        PropertyDecisionNode::new(-1, 0, 65537, 0, Predictor::WEIGHTED, 0, 1),
                    ];
                    cases.push(format!(
                        "ma-filter 0 0 {width} {flags} {}",
                        request_tree(&t)
                    ));
                }
            }
        }
    }
    // Exercise both real second-level decisions, deeper range-stack visits,
    // and failures after some ranges have already been written.
    for i in 0..400 {
        let mut t = generated_tree(2 + i % 5, &mut seed);
        let mut id = 0;
        for n in &mut t {
            if n.property == -1 {
                n.lchild = id * 257;
                id += 1;
                n.predictor_offset = if i % 11 == 0 {
                    128
                } else {
                    (id % 7) as i64 - 3
                };
                n.multiplier = if i % 13 == 0 { 128 } else { 1 + id % 5 };
            } else {
                n.property = 9;
            }
        }
        cases.push(format!(
            "ma-filter 0 0 {} {} {}",
            [8, 16, 32][i % 3],
            i % 4,
            request_tree(&t)
        ));
    }
    compare(cases);
}
#[test]
fn ma_group_bundles_retain_nested_and_reused_read_state() {
    let mut cases = Vec::new();
    for count in [0, 1, 2, 3, 7, 17, 18, 31, 64, 273, 274] {
        for offset in 0..8 {
            cases.push(format!("ma-group {count} {offset}"));
        }
    }
    compare(cases);
    let mut reads = Vec::new();
    for count in [0, 1, 2, 3, 7, 17, 18, 64] {
        for offset in 0..8 {
            let cpp = original_observations(&[format!("ma-group {count} {offset}")]).remove(0);
            let fields: Vec<_> = cpp.split_whitespace().collect();
            let data = unhex(fields[5]);
            for len in 0..=data.len().min(24) {
                reads.push(format!("ma-group-read 3 {offset} {}", hex(&data[..len])));
            }
            reads.push(format!("ma-group-read 0 {offset} {}", hex(&data)));
        }
    }
    compare(reads);
}
