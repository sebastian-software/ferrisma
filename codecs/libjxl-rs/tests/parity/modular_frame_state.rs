// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe original frame owners and methods, without replacing their algorithms.

use super::*;
use jxl::jxl::base::rect::Rect;
use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
use jxl::jxl::common::SpeedTier;
use jxl::jxl::dec_modular::ModularStreamId;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::enc_modular::{GroupParams, ModularFrameEncoder};
use jxl::jxl::enc_params::CompressParams;
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::frame_header::{FrameEncoding, FrameHeader};
use jxl::jxl::image::Image3;
use jxl::jxl::image_metadata::{CodecMetadata, ExtraChannelInfo};
use jxl::jxl::modular::modular_image::Image;
use jxl::jxl::modular::options::{
    ModularOptions, Predictor, TreeKind, TreeMode, UNDEFINED_PREDICTOR,
};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("mfs-")
}
fn values<T: std::fmt::Display>(out: &mut String, values: &[T]) {
    write!(out, " {}", values.len()).unwrap();
    for v in values {
        write!(out, " {v}").unwrap();
    }
}
fn tree_kind(raw: usize) -> TreeKind {
    [
        TreeKind::TrivialTreeNoPredictor,
        TreeKind::Learn,
        TreeKind::JpegTranscodeACMeta,
        TreeKind::FalconACMeta,
        TreeKind::ACMeta,
        TreeKind::WPFixedDC,
        TreeKind::GradientFixedDC,
    ][raw]
}
fn tree_mode(raw: usize) -> TreeMode {
    [
        TreeMode::GradientOnly,
        TreeMode::WPOnly,
        TreeMode::NoWP,
        TreeMode::Default,
    ][raw]
}
fn options(out: &mut String, o: &ModularOptions) {
    write!(
        out,
        " {} {} {} {}",
        o.max_chan_size,
        o.group_dim,
        o.nb_repeats.to_bits(),
        o.max_properties
    )
    .unwrap();
    values(out, &o.splitting_heuristics_properties);
    write!(
        out,
        " {} {} {} {} {} {} {} {} {}",
        o.splitting_heuristics_node_threshold.to_bits(),
        o.max_property_values,
        o.predictor.0,
        o.wp_mode,
        o.fast_decode_multiplier.to_bits(),
        o.wp_tree_mode as i32,
        u32::from(o.skip_encoder_fast_path),
        o.tree_kind as u32,
        u32::from(o.zero_tokens)
    )
    .unwrap();
    enc_frame::histogram(out, &o.histogram_params);
}
fn read_params(args: &mut std::str::SplitWhitespace<'_>) -> CompressParams<'static> {
    let mut p = enc_frame::read_params(args);
    p.colorspace = number(args);
    p.palette_colors = number(args);
    p.channel_colors_pre_transform_percent = f32::from_bits(number(args));
    p.channel_colors_percent = f32::from_bits(number(args));
    p.options.nb_repeats = f32::from_bits(number(args));
    p.options.max_properties = number(args);
    p.options.wp_mode = number(args);
    p.options.wp_tree_mode = tree_mode(number(args));
    p.options.tree_kind = tree_kind(number(args));
    p.options.fast_decode_multiplier = f32::from_bits(number(args));
    p.options.zero_tokens = number::<u32>(args) != 0;
    p.custom_fixed_tree = ma::read_tree(args);
    p
}
fn state(out: &mut String, e: &ModularFrameEncoder<'_>) {
    out.push_str(" E");
    frame::dimensions(out, &e.frame_dim);
    enc_frame::params(out, &e.cparams);
    ma::tree(out, &e.cparams.custom_fixed_tree);
    values(out, &e.ac_metadata_size);
    values(out, &e.extra_dc_precision);
    write!(out, " {}", e.stream_options.len()).unwrap();
    for o in &e.stream_options {
        options(out, o);
    }
    write!(out, " {}", e.stream_images.len()).unwrap();
    for im in &e.stream_images {
        modular_frame::image(out, im, true);
    }
    values(out, &e.quants);
    ma::tree(out, &e.tree);
    write!(out, " {}", e.tree_tokens.len()).unwrap();
    for t in &e.tree_tokens {
        ma::tokens(out, t);
    }
    write!(out, " {}", e.stream_headers.len()).unwrap();
    for h in &e.stream_headers {
        write!(out, " {}", u32::from(h.use_global_tree)).unwrap();
        predict::header(out, &h.wp_header);
        let mut view = Image::default();
        view.transform = h.transforms.clone();
        modular_frame::image(out, &view, false);
    }
    write!(out, " {}", e.tokens.len()).unwrap();
    for t in &e.tokens {
        ma::tokens(out, t);
    }
    values(out, &e.context_map);
    values(out, &e.tree_splits);
    write!(out, " {}", e.gi_channel.len()).unwrap();
    for c in &e.gi_channel {
        values(out, c);
    }
    values(out, &e.image_widths);
    write!(out, " {}", e.stream_params.len()).unwrap();
    for g in &e.stream_params {
        write!(
            out,
            " {},{},{},{},{},{},{}",
            g.rect.x0(),
            g.rect.y0(),
            g.rect.xsize(),
            g.rect.ysize(),
            g.min_shift,
            g.max_shift,
            g.id.id(&e.frame_dim)
        )
        .unwrap();
    }
    write!(
        out,
        " {} {} {}",
        e.code.encoding_info.len(),
        e.code.uint_config.len(),
        e.code.encoded_histograms.len()
    )
    .unwrap();
    values(out, &e.code.context_map);
}
fn read_id(args: &mut std::str::SplitWhitespace<'_>) -> ModularStreamId {
    let kind: u32 = number(args);
    let group = number(args);
    let pass = number(args);
    match kind {
        0 => ModularStreamId::global(),
        1 => ModularStreamId::var_dct_dc(group),
        2 => ModularStreamId::modular_dc(group),
        3 => ModularStreamId::ac_metadata(group),
        4 => ModularStreamId::quant_table(group).unwrap(),
        5 => ModularStreamId::modular_ac(group, pass),
        _ => panic!("frame fixture stream ID"),
    }
}
pub(super) fn layer(raw: usize) -> LayerType {
    [
        LayerType::Header,
        LayerType::Toc,
        LayerType::Dictionary,
        LayerType::Splines,
        LayerType::Noise,
        LayerType::Quant,
        LayerType::ModularTree,
        LayerType::ModularGlobal,
        LayerType::Dc,
        LayerType::ModularDcGroup,
        LayerType::ControlFields,
        LayerType::Order,
        LayerType::Ac,
        LayerType::AcTokens,
        LayerType::ModularAcGroup,
    ][raw]
}
pub(super) fn writer_output(out: &mut String, writer: &BitWriter, aux: &AuxOut) {
    let mut copy = BitWriter::new();
    copy.append_unaligned(writer).unwrap();
    pad(&mut copy);
    write!(out, " W {} {}", writer.bits_written(), hex(copy.get_span())).unwrap();
    for a in &aux.layers {
        write!(
            out,
            " {},{},{},{},{}",
            a.total_bits,
            a.histogram_bits,
            a.extra_bits,
            a.num_clustered_histograms,
            a.clustered_entropy.to_bits()
        )
        .unwrap();
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "mfs-thunder" || kind == "mfs-thunder-groups" {
        return thunder(args, kind == "mfs-thunder-groups");
    }
    assert!(matches!(kind, "mfs-init" | "mfs-chain"));
    let streaming = number::<u32>(args) != 0;
    let p = read_params(args);
    let w = number(args);
    let h = number(args);
    let shift = number(args);
    let passes = number(args);
    let gray = number::<u32>(args) != 0;
    let extras = number(args);
    let modular = number::<u32>(args) != 0;
    let mut metadata = CodecMetadata::default();
    metadata.size.set(w, h).unwrap();
    metadata.m.num_extra_channels = extras;
    metadata
        .m
        .extra_channel_info
        .resize(extras as usize, ExtraChannelInfo::default());
    metadata
        .m
        .color_encoding
        .set_srgb(
            if gray {
                ColorSpace::GRAY
            } else {
                ColorSpace::RGB
            },
            RenderingIntent::RELATIVE,
        )
        .unwrap();
    let mut header = FrameHeader::new(Some(&metadata));
    header.group_size_shift = shift;
    header.passes.num_passes = passes;
    header.encoding = if modular {
        FrameEncoding::MODULAR
    } else {
        FrameEncoding::VAR_DCT
    };
    let e = ModularFrameEncoder::create(&header, &p, streaming);
    let mut out = status_code(&e.as_ref().map(|_| ()).map_err(|e| *e)).to_string();
    if let Ok(mut e) = e {
        if kind == "mfs-chain" {
            e.stream_images[0] = transforms::read_image(args);
            e.gi_channel.resize_with(e.stream_images.len(), Vec::new);
            let n: usize = number(args);
            e.quants = (0..n).map(|_| number(args)).collect();
            let prefix_bits = number(args);
            let actions: usize = number(args);
            let mut writer = BitWriter::new();
            let mut aux = AuxOut::default();
            prefix(&mut writer, prefix_bits);
            state(&mut out, &e);
            writer_output(&mut out, &writer, &aux);
            for _ in 0..actions {
                let op: u32 = number(args);
                write!(out, " O {op}").unwrap();
                let status = match op {
                    0 => {
                        let id = read_id(args);
                        let rect =
                            Rect::new(number(args), number(args), number(args), number(args));
                        let min = number(args);
                        let max = number(args);
                        let color = number::<u32>(args) != 0;
                        let groupwise = number::<u32>(args) != 0;
                        let record = number::<u32>(args) != 0;
                        if record {
                            e.stream_params.push(GroupParams {
                                rect,
                                min_shift: min,
                                max_shift: max,
                                id,
                            });
                        }
                        let s = id.id(&e.frame_dim);
                        if s != 0 {
                            e.stream_options[s] = e.stream_options[0].clone();
                        }
                        e.prepare_stream_params(&rect, min, max, &id, color, groupwise)
                    }
                    1 => e.compute_tree(),
                    2 => e.compute_tokens(),
                    3 => {
                        let streaming = number::<u32>(args) != 0;
                        let use_aux = number::<u32>(args) != 0;
                        e.encode_global_info(streaming, &mut writer, use_aux.then_some(&mut aux))
                    }
                    4 => {
                        let id = read_id(args);
                        let layer = layer(number(args));
                        let use_aux = number::<u32>(args) != 0;
                        e.encode_stream(&mut writer, use_aux.then_some(&mut aux), layer, &id)
                    }
                    5 => {
                        let id = read_id(args);
                        e.clear_stream_data(&id);
                        Ok(())
                    }
                    6 => {
                        e.clear_modular_stream_data();
                        Ok(())
                    }
                    7 => {
                        let dc = number(args);
                        let ac = number(args);
                        let patch = FrameDimensions {
                            xsize_groups: number(args),
                            ysize_groups: number(args),
                            ..Default::default()
                        };
                        write!(
                            out,
                            " I {}",
                            e.compute_streaming_absolute_ac_group_id(dc, ac, &patch)
                        )
                        .unwrap();
                        Ok(())
                    }
                    8 => {
                        let id = read_id(args);
                        let s = id.id(&e.frame_dim);
                        let o = &mut e.stream_options[s];
                        o.tree_kind = tree_kind(number(args));
                        o.predictor = Predictor(number(args));
                        o.nb_repeats = f32::from_bits(number(args));
                        o.max_properties = number(args);
                        o.max_chan_size = number(args);
                        Ok(())
                    }
                    9 => {
                        let id = read_id(args);
                        let s = id.id(&e.frame_dim);
                        e.stream_images[s].error = number::<u32>(args) != 0;
                        Ok(())
                    }
                    10 => {
                        let id = read_id(args);
                        let s = id.id(&e.frame_dim);
                        e.stream_images[s] = transforms::read_image(args);
                        Ok(())
                    }
                    _ => panic!("unknown frame fixture action"),
                };
                write!(out, " {}", status_code(&status)).unwrap();
                state(&mut out, &e);
                writer_output(&mut out, &writer, &aux);
            }
        } else {
            state(&mut out, &e);
        }
    }
    out.push_str(" P");
    enc_frame::params(&mut out, &p);
    out
}

fn thunder(args: &mut std::str::SplitWhitespace<'_>, groups: bool) -> String {
    let w = number(args);
    let h = number(args);
    let gray = number::<u32>(args) != 0;
    let depth = match number::<u32>(args) {
        8 => LosslessSampleDepth::Uint8,
        16 => LosslessSampleDepth::Uint16,
        _ => panic!("fixture depth"),
    };
    let cw = number(args);
    let ch = number(args);
    let config = match LosslessThunderConfig::new(w, h, gray, depth) {
        Ok(c) => c,
        Err(e) => return format!("S {}", e as i32),
    };
    let mut frame = config.create_frame().unwrap();
    let mut color = Image3::<f32>::create(cw, ch).unwrap();
    for c in 0..3 {
        for y in 0..ch {
            for x in 0..cw {
                color.plane_row_mut(c, y)[x] = f32::from_bits(number(args));
            }
        }
    }
    let prepared = frame.prepare_modular(&color);
    let mut out = status_code(&prepared).to_string();
    if groups {
        let has_aux = number::<u32>(args) != 0;
        let prepopulate = number::<u32>(args) != 0;
        let repeat = number::<u32>(args) != 0;
        let mut aux = aux_stats::seed(number(args));
        let mut codes = Vec::new();
        if prepopulate {
            let mut w = BitWriter::new();
            prefix(&mut w, 5);
            codes.push(w);
        }
        if prepared.is_ok() {
            let result = frame.compute_modular_tokens();
            write!(out, " T {}", status_code(&result)).unwrap();
            if result.is_ok() {
                let result =
                    frame.encode_groups(&mut codes, if has_aux { Some(&mut aux) } else { None });
                write!(out, " C {}", status_code(&result)).unwrap();
                group_codes(&mut out, &codes);
                aux_stats::output(&mut out, &aux);
                if repeat {
                    let result = frame
                        .encode_groups(&mut codes, if has_aux { Some(&mut aux) } else { None });
                    write!(out, " C {}", status_code(&result)).unwrap();
                    group_codes(&mut out, &codes);
                    aux_stats::output(&mut out, &aux);
                }
            }
        }
        state(&mut out, frame.modular());
        return out;
    }
    state(&mut out, frame.modular());
    for c in 0..3 {
        for y in 0..ch {
            for x in 0..cw {
                write!(out, " {}", color.plane_row(c, y)[x].to_bits()).unwrap();
            }
        }
    }
    out
}
fn group_codes(out: &mut String, codes: &[BitWriter]) {
    write!(out, " G {}", codes.len()).unwrap();
    for code in codes {
        let mut copy = BitWriter::new();
        copy.append_unaligned(code).unwrap();
        pad(&mut copy);
        write!(out, " {} {}", code.bits_written(), hex(copy.get_span())).unwrap();
    }
}

#[test]
fn lossless_thunder_group_encoding_matches_original_sections_shared_tokens_auxiliary_state_and_guard_errors()
 {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let mut seed = 0x510a_3679_9cde_28bf;
    for (w, h) in [
        (1, 1),
        (17, 9),
        (129, 3),
        (257, 3),
        (401, 3),
        (513, 5),
        (9, 513),
        (2049, 3),
        (7, 2049),
    ] {
        for gray in [false, true] {
            for depth in [8, 16] {
                for mode in [0, 3, 5] {
                    for has_aux in [0, 1] {
                        let request = thunder_request(w, h, gray, depth, mode, false, &mut seed)
                            .replacen("mfs-thunder", "mfs-thunder-groups", 1);
                        cases.push(format!("{request} {has_aux} 0 1 {}", mode + has_aux));
                    }
                }
            }
        }
    }
    for mode in 0..7 {
        let request = thunder_request(17, 9, false, 16, mode, false, &mut seed).replacen(
            "mfs-thunder",
            "mfs-thunder-groups",
            1,
        );
        cases.push(format!("{request} 1 1 1 {mode}"));
    }
    compare(cases);
}

fn thunder_request(
    w: usize,
    h: usize,
    gray: bool,
    depth: u32,
    mode: usize,
    padded: bool,
    seed: &mut u64,
) -> String {
    let cw = w + if padded { 5 } else { 0 };
    let ch = h + if padded { 3 } else { 0 };
    let mut request = format!("mfs-thunder {w} {h} {} {depth} {cw} {ch}", u32::from(gray));
    let maxval = (1u32 << depth) - 1;
    let factor = 1.0f32 / maxval as f32;
    for c in 0..3 {
        for y in 0..ch {
            for x in 0..cw {
                let v = if x >= w || y >= h {
                    f32::from_bits(0x7fc1_2345)
                } else {
                    let sample = match mode {
                        0 => 0,
                        1 => maxval,
                        2 => ((c + 1) * 73) as u32 & maxval,
                        3 => ((x + y * 11 + c * 31) as u32).wrapping_mul(997) & maxval,
                        4 => (((x / 3 + y / 2 + c) % 4) * 47) as u32,
                        5 => next(seed) as u32 & maxval,
                        6 => [0, maxval, maxval / 2, 1][(x + y) % 4],
                        _ => unreachable!(),
                    };
                    sample as f32 * factor
                };
                write!(request, " {}", v.to_bits()).unwrap();
            }
        }
    }
    request
}

#[test]
fn lossless_thunder_pixel_preparation_matches_original_palettes_rct_groups_and_released_planes() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut seed = 0x6d28_a954_72f1_0b63;
    let mut cases = Vec::new();
    for (w, h) in [
        (1, 1),
        (2, 3),
        (17, 9),
        (127, 3),
        (128, 1),
        (129, 3),
        (256, 1),
        (257, 3),
        (399, 1),
        (400, 3),
    ] {
        for gray in [false, true] {
            for depth in [8, 16] {
                for mode in 0..7 {
                    cases.push(thunder_request(
                        w,
                        h,
                        gray,
                        depth,
                        mode,
                        mode % 2 == 0,
                        &mut seed,
                    ));
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn lossless_thunder_pixel_preparation_retains_group_boundaries_dc_groups_and_default_compaction() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut seed = 0x307f_9652_4bad_8e11;
    let mut cases = Vec::new();
    for (w, h) in [
        (401, 3),
        (513, 5),
        (9, 513),
        (2049, 3),
        (7, 2049),
        (257, 129),
    ] {
        for gray in [false, true] {
            for depth in [8, 16] {
                for mode in [0, 3, 5] {
                    cases.push(thunder_request(w, h, gray, depth, mode, false, &mut seed));
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn lossless_thunder_configuration_rejects_original_invalid_dimensions_before_pixel_access() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    compare(
        [
            "mfs-thunder 0 1 0 8 0 1",
            "mfs-thunder 1 0 1 16 1 0",
            "mfs-thunder 4294967296 1 0 8 0 0",
            "mfs-thunder 1 4294967296 1 16 0 0",
        ]
        .map(str::to_owned)
        .to_vec(),
    );
}
fn parameter_input(p: &CompressParams<'_>) -> String {
    let mut out = enc_frame::parameter_input(p);
    write!(
        out,
        " {} {} {} {} {} {} {} {} {} {} {} {}",
        p.colorspace,
        p.palette_colors,
        p.channel_colors_pre_transform_percent.to_bits(),
        p.channel_colors_percent.to_bits(),
        p.options.nb_repeats.to_bits(),
        p.options.max_properties,
        p.options.wp_mode,
        p.options.wp_tree_mode as i32,
        p.options.tree_kind as u32,
        p.options.fast_decode_multiplier.to_bits(),
        u32::from(p.options.zero_tokens),
        p.custom_fixed_tree.len()
    )
    .unwrap();
    for n in &p.custom_fixed_tree {
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
#[test]
fn modular_frame_state_initialization_preserves_all_tiers_properties_and_original_parameters() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    for i in 0..1100 {
        let mut p = CompressParams {
            speed_tier: enc_frame::tier((i % 11) as i32 - 1),
            modular_mode: i % 3 != 0,
            responsive: [-1, 0, 1, 2][i % 4],
            colorspace: [-1, 0, 6][i % 3],
            lossy_palette: i % 13 == 0,
            ..Default::default()
        };
        if i % 4 != 0 {
            p.set_lossless();
        }
        p.decoding_speed_tier = [0usize, 1, 2, 3, 4, 7, usize::MAX][(i / 3) % 7];
        p.options.predictor = [
            UNDEFINED_PREDICTOR,
            Predictor::ZERO,
            Predictor::LEFT,
            Predictor::WEIGHTED,
            Predictor::BEST,
            Predictor::AVERAGE0,
            Predictor::AVERAGE4,
            Predictor::VARIABLE,
        ][(i / 5) % 8];
        p.options.nb_repeats = [-0.5f32, -0.0, 0.0, 0.5, 1.0, 2.0, f32::NAN][(i / 7) % 7];
        p.options.fast_decode_multiplier = [0.8f32, 1.09, f32::NAN][i % 3];
        p.options.wp_tree_mode = tree_mode(i % 4);
        p.options.tree_kind = tree_kind(i % 7);
        p.options.wp_mode = (i % 6) as i32;
        p.options.max_properties = [-2, 0, 1, 2, 8][(i / 11) % 5];
        p.options.zero_tokens = i % 19 == 0;
        let (w, h) = [(1, 1), (17, 9), (129, 65), (257, 129), (513, 257)][i % 5];
        cases.push(format!(
            "mfs-init {} {} {w} {h} {} {} {} {} {}",
            i % 2,
            parameter_input(&p),
            i % 3,
            1 + i % 3,
            (i / 2) % 2,
            (i / 7) % 4,
            u32::from(i % 3 != 0)
        ));
    }
    compare(cases);
}

fn input_image(w: usize, h: usize, depth: i32, n: usize, mode: usize, seed: &mut u64) -> String {
    let mut out = format!("{w} {h} {depth} 0 0 {n}");
    for c in 0..n {
        write!(out, " {w} {h} 0 0 {c}").unwrap();
        for y in 0..h {
            for x in 0..w {
                let value = match mode {
                    0 => 0,
                    1 => 11 + c as i32 * 7,
                    2 => ((x + 3 * y) % 4 * (c + 1) * 7) as i32,
                    3 => (x * (c + 1) + y * 3) as i32,
                    4 => (next(seed) % 256) as i32,
                    5 => (next(seed) % 2048) as i32 * 4,
                    _ => {
                        if x % 2 == 0 {
                            i32::MIN
                        } else {
                            i32::MAX
                        }
                    }
                };
                write!(out, " {value}").unwrap();
            }
        }
    }
    out
}
#[expect(
    clippy::too_many_arguments,
    reason = "Keep fixture metadata, sample input, options, and action controls explicit."
)]
fn chain(
    p: &CompressParams<'_>,
    w: usize,
    h: usize,
    passes: usize,
    n: usize,
    streaming: bool,
    prefix: usize,
    image: &str,
    quants: &[u32],
    actions: &[String],
) -> String {
    let mut out = format!(
        "mfs-chain {} {} {w} {h} 0 {passes} {} 0 {} {image}",
        u32::from(streaming),
        parameter_input(p),
        u32::from(n == 1),
        u32::from(p.modular_mode)
    );
    values(&mut out, quants);
    write!(out, " {prefix} {}", actions.len()).unwrap();
    for a in actions {
        write!(out, " {a}").unwrap();
    }
    out
}
fn lossless(tier: i32) -> CompressParams<'static> {
    let mut p = CompressParams {
        speed_tier: enc_frame::tier(tier),
        ..Default::default()
    };
    p.set_lossless();
    p
}
#[test]
fn modular_frame_state_streams_match_group_selection_shared_entropy_repeated_calls_and_cleanup() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let mut seed = 0x914c_836d_4b25_a7fe;
    for i in 0..180 {
        let mut p = lossless((i % 11) as i32 - 1);
        p.options.max_properties = (i % 3) as i32;
        p.decoding_speed_tier = (i / 11) % 4;
        let (w, h) = [(1, 1), (17, 9), (129, 3), (257, 7), (65, 11)][i % 5];
        let n = if i % 4 == 0 { 1 } else { 3 };
        let passes = 1 + i % 2;
        let image = input_image(w, h, 8, n, i % 5, &mut seed);
        let mut actions = vec![format!("0 0 0 0 0 0 {w} {h} -1 30 1 0 0")];
        let groups = w.div_ceil(128);
        for g in 0..groups {
            let x = g * 128;
            let gw = (w - x).min(128);
            actions.push(format!("0 5 {g} 0 {x} 0 {gw} {h} -1 30 1 {} 1", i % 2));
        }
        if i % 9 == 0 && passes > 1 {
            actions.push(format!("0 5 0 1 0 0 {} {h} -1 30 1 1 1", w.min(128)));
        }
        actions.extend(["1".to_owned(), "2".to_owned()]);
        if i % 7 == 0 {
            actions.extend(["1".to_owned(), "2".to_owned()]);
        }
        actions.extend(["3 0 1".to_owned(), "4 0 0 0 7 1".to_owned()]);
        for g in 0..groups {
            actions.push(format!("4 5 {g} 0 14 1"));
        }
        if i % 9 == 0 && passes > 1 {
            actions.push("4 5 0 1 14 1".to_owned());
        }
        actions.extend([
            format!("7 {} {} 2 3", usize::MAX - i, i * 7),
            "5 5 0 0".to_owned(),
            "4 5 0 0 14 1".to_owned(),
            "6".to_owned(),
        ]);
        cases.push(chain(
            &p,
            w,
            h,
            passes,
            n,
            false,
            i % 8,
            &image,
            &[],
            &actions,
        ));
    }
    compare(cases);
}

#[test]
fn modular_frame_state_streaming_tiles_use_original_local_stream_lifecycle() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let mut seed = 0x8f3a_05c1_92dd_e764;
    for i in 0..16 {
        let mut p = lossless([5, 6, 7, 8, 9][i % 5]);
        p.colorspace = 0;
        p.modular_group_size_shift = 0;
        p.resampling = 1;
        p.ec_resampling = 1;
        let n = if i % 2 == 0 { 1 } else { 3 };
        let image = input_image(1024, 1, 8, n, i % 5, &mut seed);
        let mut actions = vec!["0 0 0 0 0 0 1024 1 -1 30 1 1 0".to_owned()];
        for g in 0..8 {
            actions.push(format!("0 5 {g} 0 {} 0 128 1 -1 30 1 1 1", g * 128));
        }
        // enc_frame.cc does not ComputeTree/ComputeTokens in streaming mode.
        actions.extend(["3 1 1".to_owned(), "4 0 0 0 7 1".to_owned()]);
        for g in 0..8 {
            actions.push(format!("4 5 {g} 0 14 1"));
        }
        actions.push("6".to_owned());
        cases.push(chain(&p, 1153, 1, 1, n, true, i % 8, &image, &[], &actions));
    }
    compare(cases);
}

#[test]
fn modular_frame_state_generic_streams_and_empty_global_information_keep_writer_and_accounting() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let mut seed = 0xb54f_31e7_09ac_6802;
    for i in 0..80 {
        let p = lossless((i % 11) as i32 - 1);
        let (w, h) = (1 + i % 23, 1 + i % 7);
        let n = 1 + 2 * (i % 2);
        let input = input_image(w, h, 16, n, i % 6, &mut seed);
        let actions = vec![
            format!("3 {} {}", i % 2, (i / 3) % 2),
            format!("4 0 0 0 7 {}", i % 2),
            "5 0 0 0".to_owned(),
            "4 0 0 0 7 1".to_owned(),
            "3 0 1".to_owned(),
        ];
        cases.push(chain(
            &p,
            w,
            h,
            1,
            n,
            i % 2 != 0,
            i % 8,
            &input,
            &[],
            &actions,
        ));
    }
    compare(cases);
}

#[test]
fn modular_frame_state_tree_choices_quantized_ranges_and_error_prefixes_match_source() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let mut seed = 0x2a79_1fb4_ce83_606d;
    for i in 0..140 {
        let mut p = lossless((i % 8) as i32 - 1);
        p.palette_colors = 0;
        p.channel_colors_percent = 0.0;
        p.options.max_properties = 0;
        let (w, h) = (17, 3);
        let n = 3;
        let image = input_image(w, h, 16, n, 5, &mut seed);
        let predictor = 5;
        let mut actions = vec![
            format!("8 0 0 0 {} {predictor} {} 0 128", i % 7, 0.5f32.to_bits()),
            "1".to_owned(),
        ];
        actions.extend(["2".to_owned(), "3 0 1".to_owned(), "4 0 0 0 7 1".to_owned()]);
        cases.push(chain(
            &p,
            w,
            h,
            1,
            n,
            false,
            i % 8,
            &image,
            &[4, 4, 4],
            &actions,
        ));
    }
    // Token overflow and pre-existing image errors are terminal fixture calls.
    for error in [false, true] {
        let p = lossless(SpeedTier::Lightning as i32);
        let image = input_image(3, 1, 32, 1, 6, &mut seed);
        let mut actions = vec!["1".to_owned()];
        if error {
            actions.push("9 0 0 0 1".to_owned());
        }
        actions.push("2".to_owned());
        cases.push(chain(&p, 3, 1, 1, 1, false, 5, &image, &[], &actions));
    }
    for later in [false, true] {
        use jxl::jxl::modular::encoding::dec_ma::PropertyDecisionNode as N;
        let mut p = lossless(SpeedTier::Thunder as i32);
        let bad = N::new(-2, 0, 0, 0, Predictor::ZERO, 0, 1);
        p.custom_fixed_tree = if later {
            vec![
                N::split(0, 0, 1, 2),
                N::leaf(Predictor::GRADIENT, 0, 1),
                bad,
            ]
        } else {
            vec![bad]
        };
        let image = input_image(3, 2, 8, 1, 0, &mut seed);
        cases.push(chain(
            &p,
            3,
            2,
            1,
            1,
            false,
            3,
            &image,
            &[],
            &["1".to_owned()],
        ));
    }
    let p = lossless(SpeedTier::Lightning as i32);
    let image = input_image(3, 1, 8, 1, 0, &mut seed);
    cases.push(chain(
        &p,
        3,
        1,
        1,
        1,
        false,
        0,
        &image,
        &[],
        &[
            "1".to_owned(),
            "2".to_owned(),
            "9 0 0 0 1".to_owned(),
            "2".to_owned(),
        ],
    ));
    compare(cases);
}

#[test]
fn modular_frame_state_multiple_useful_chunks_and_empty_chunks_keep_tree_merging() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let mut seed = 0x74b8_231a_e6c9_d5f0;
    for i in 0..44 {
        let mut p = lossless((i % 11) as i32 - 1);
        p.modular_mode = false;
        p.options.max_properties = 0;
        let global = input_image(129, 3, 8, 0, 0, &mut seed);
        let streams = [(1, 0, 3), (2, 0, 1), (3, 0, 5), (4, 0, 1), (5, 0, 3)];
        let mut actions = Vec::new();
        for (j, (kind, group, n)) in streams.into_iter().enumerate() {
            if (i + j) % 4 == 0 {
                continue;
            }
            let im = input_image(3 + j, 2 + i % 3, 8, n, (i + j) % 5, &mut seed);
            actions.push(format!("10 {kind} {group} 0 {im}"));
            actions.push(format!(
                "8 {kind} {group} 0 {} 5 {} 0 128",
                [0, 1, 3, 4, 5, 6][(i + j) % 6],
                0.5f32.to_bits()
            ));
        }
        actions.extend(["1".to_owned(), "2".to_owned(), "3 0 1".to_owned()]);
        for (j, (kind, group, _)) in streams.into_iter().enumerate() {
            if (i + j) % 4 != 0 {
                actions.push(format!("4 {kind} {group} 0 8 1"));
            }
        }
        cases.push(chain(
            &p,
            129,
            3,
            1,
            0,
            false,
            i % 8,
            &global,
            &[],
            &actions,
        ));
    }
    // A non-modular frame with no modular samples keeps its empty tree/token
    // owners; the global stream then contains just the original no-tree bit.
    for tier in -1..=9 {
        let mut p = lossless(tier);
        p.modular_mode = false;
        let image = input_image(1, 1, 8, 0, 0, &mut seed);
        cases.push(chain(
            &p,
            1,
            1,
            1,
            0,
            false,
            0,
            &image,
            &[],
            &[
                "1".to_owned(),
                "2".to_owned(),
                "3 0 1".to_owned(),
                "4 0 0 0 7 1".to_owned(),
            ],
        ));
    }
    compare(cases);
}

#[test]
fn modular_frame_state_crops_heterogeneous_channels_and_retains_reused_mappings() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let mut image = "129 33 16 1 0 4".to_owned();
    for (c, (w, h, hs, vs)) in [(2, 1, 0, 0), (32, 9, 2, 2), (129, 33, 0, 0), (65, 17, 1, 1)]
        .into_iter()
        .enumerate()
    {
        write!(image, " {w} {h} {hs} {vs} {c}").unwrap();
        for y in 0..h {
            for x in 0..w {
                write!(image, " {}", c * 1000 + y * 100 + x).unwrap();
            }
        }
    }
    let mut p = lossless(SpeedTier::Thunder as i32);
    p.colorspace = 0;
    for groupwise in 0..2 {
        for (min, max) in [(-1, 30), (0, 0), (1, 1), (2, 3), (5, 4)] {
            for (x, y, w, h) in [
                (128, 0, 1, 33),
                (0, 0, 128, 33),
                (64, 4, 64, 16),
                (129, 33, 0, 0),
            ] {
                let actions = vec![
                    format!("0 5 1 0 {x} {y} {w} {h} {min} {max} 1 {groupwise} 1"),
                    "0 5 1 0 128 0 1 33 0 1 1 0 1".to_owned(),
                    "6".to_owned(),
                ];
                cases.push(chain(&p, 129, 33, 1, 3, false, 0, &image, &[], &actions));
            }
        }
    }
    compare(cases);
}
