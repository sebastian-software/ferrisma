// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Independent observations of original parameter/header preparation entry points.

use super::*;
use jxl::jxl::base::override_option::{Override, apply_override, override_from_bool};
use jxl::jxl::cms_interface::{JxlCmsInterface, JxlColorProfile};
use jxl::jxl::color_encoding::JxlColorEncoding;
use jxl::jxl::common::{MAX_NUM_PASSES, SpeedTier};
use jxl::jxl::dec_modular::{ModularStreamId, ModularStreamKind};
use jxl::jxl::enc_ans_params::HistogramParams;
use jxl::jxl::enc_frame::*;
use jxl::jxl::enc_params::CompressParams;
use jxl::jxl::enc_progressive_split::{PassDefinition, ProgressiveMode, ProgressiveSplitter};
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::frame_header::*;
use jxl::jxl::image_metadata::ExtraChannel;
use jxl::jxl::jpeg::enc_jpeg_data::{
    set_chroma_subsampling_from_jpeg_data, set_color_transform_from_jpeg_data,
};
use jxl::jxl::jpeg::jpeg_data::{JpegComponent, JpegData};
use jxl::jxl::modular::options::Predictor;
use jxl::jxl::quant_weights::NUM_QUANT_TABLES;
use jxl::jxl::splines::SplinePoint;
use jxl::jxl::types::JxlBitDepth;
use std::mem::{align_of, offset_of, size_of};

macro_rules! vals { ($out:expr; $($value:expr),* $(,)?) => {{ $(write!($out, " {}", $value).unwrap();)* }}; }
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("ef-")
}
fn floats(args: &mut std::str::SplitWhitespace<'_>) -> Vec<f32> {
    let n: usize = number(args);
    (0..n).map(|_| f32::from_bits(number(args))).collect()
}
fn output_floats(out: &mut String, values: &[f32]) {
    vals!(out;values.len());
    for v in values {
        vals!(out;v.to_bits());
    }
}
pub(super) fn tier(value: i32) -> SpeedTier {
    [
        SpeedTier::TectonicPlate,
        SpeedTier::Glacier,
        SpeedTier::Tortoise,
        SpeedTier::Kitten,
        SpeedTier::Squirrel,
        SpeedTier::Wombat,
        SpeedTier::Hare,
        SpeedTier::Cheetah,
        SpeedTier::Falcon,
        SpeedTier::Thunder,
        SpeedTier::Lightning,
    ][(value + 1) as usize]
}
pub(super) fn read_params(args: &mut std::str::SplitWhitespace<'_>) -> CompressParams<'static> {
    let mut p = CompressParams {
        butteraugli_distance: f32::from_bits(number(args)),
        modular_mode: number::<u32>(args) != 0,
        color_transform: ColorTransform(number(args)),
        speed_tier: tier(number(args)),
        decoding_speed_tier: number(args),
        modular_group_size_shift: number(args),
        resampling: number(args),
        ec_resampling: number(args),
        already_downsampled: number::<u32>(args) != 0,
        original_butteraugli_distance: f32::from_bits(number(args)),
        progressive_mode: Override(number(args)),
        qprogressive_mode: Override(number(args)),
        responsive: number(args),
        epf: number(args),
        gaborish: Override(number(args)),
        noise: Override(number(args)),
        disable_perceptual_optimizations: number::<u32>(args) != 0,
        photon_noise_iso: f32::from_bits(number(args)),
        progressive_dc: number(args),
        lossy_palette: number::<u32>(args) != 0,
        force_lfs_jpeg_recompression: number(args),
        ..Default::default()
    };
    p.options.predictor = Predictor(number(args));
    p.manual_noise = floats(args);
    p.manual_xyb_factors = floats(args);
    p.ec_distance = floats(args);
    p
}
pub(super) fn histogram(out: &mut String, p: &HistogramParams) {
    vals!(out;p.clustering as u32,p.uint_method as u32,p.lz77_method as u32,p.ans_histogram_strategy as u32,p.max_histograms,u32::from(p.force_huffman),u32::from(p.initialize_global_state),u32::from(p.streaming_mode),u32::from(p.add_missing_symbols),u32::from(p.add_fixed_histograms),p.image_widths.len());
    for v in &p.image_widths {
        vals!(out;v);
    }
}
pub(super) fn params(out: &mut String, p: &CompressParams<'_>) {
    vals!(out;p.butteraugli_distance.to_bits());
    output_floats(out, &p.ec_distance);
    vals!(out;u32::from(p.max_error_mode));
    for v in p.max_error {
        vals!(out;v.to_bits());
    }
    vals!(out;u32::from(p.disable_perceptual_optimizations),p.speed_tier as i32,p.brotli_effort,p.decoding_speed_tier,p.color_transform.0,u32::from(p.modular_mode),p.modular_group_size_shift,
        p.preview.0,p.noise.0,p.dots.0,p.patches.0,p.gaborish.0,p.epf,p.progressive_mode.0,p.qprogressive_mode.0,u32::from(p.centerfirst),p.center_x,p.center_y,p.progressive_dc,p.keep_invisible.0,u32::from(p.cms_set),
        u32::from(p.force_cfl_jpeg_recompression),p.force_lfs_jpeg_recompression,u32::from(p.jpeg_compress_boxes),u32::from(p.jpeg_keep_exif),u32::from(p.jpeg_keep_xmp),u32::from(p.jpeg_keep_jumbf),p.photon_noise_iso.to_bits());
    let o = &p.options;
    vals!(out;o.max_chan_size,o.group_dim,o.nb_repeats.to_bits(),o.max_properties,o.splitting_heuristics_properties.len());
    for v in &o.splitting_heuristics_properties {
        vals!(out;v);
    }
    vals!(out;o.splitting_heuristics_node_threshold.to_bits(),o.max_property_values,o.predictor.0,o.wp_mode,o.fast_decode_multiplier.to_bits(),o.wp_tree_mode as i32,u32::from(o.skip_encoder_fast_path),o.tree_kind as u32,u32::from(o.zero_tokens));
    histogram(out, &o.histogram_params);
    vals!(out;p.responsive,p.colorspace,p.move_to_front_from_channel,p.channel_colors_pre_transform_percent.to_bits(),p.channel_colors_percent.to_bits(),p.palette_colors,u32::from(p.lossy_palette),
        p.resampling,p.ec_resampling,u32::from(p.already_downsampled),p.original_butteraugli_distance.to_bits(),p.quant_ac_rescale.to_bits(),p.level,p.buffering,p.output_mode,u32::from(p.use_full_image_heuristics));
    output_floats(out, &p.manual_noise);
    output_floats(out, &p.manual_xyb_factors);
    vals!(out;p.custom_fixed_tree.len(),u32::from(p.custom_splines.has_any()),p.custom_splines.splines.len(),p.custom_splines.starting_points.len(),u32::from(p.custom_progressive_mode.is_some()),u32::from(p.debug_image.is_some()));
}
fn read_jpeg(args: &mut std::str::SplitWhitespace<'_>) -> JpegData {
    let n: usize = number(args);
    let components = (0..n)
        .map(|_| JpegComponent {
            id: number(args),
            h_samp_factor: number(args),
            v_samp_factor: number(args),
            ..Default::default()
        })
        .collect();
    let marker_order = unhex(args.next().unwrap());
    let n: usize = number(args);
    let app_data = (0..n).map(|_| unhex(args.next().unwrap())).collect();
    JpegData {
        components,
        marker_order,
        app_data,
        ..Default::default()
    }
}
fn read_mode(args: &mut std::str::SplitWhitespace<'_>) -> ProgressiveMode {
    let n = number(args);
    let passes = std::array::from_fn(|_| PassDefinition {
        num_coefficients: number(args),
        shift: number(args),
        suitable_for_downsampling_of_at_least: number(args),
    });
    ProgressiveMode {
        num_passes: n,
        passes,
    }
}
fn seed_passes(seed: u32) -> Passes {
    Passes {
        num_passes: 3,
        num_downsample: 2,
        downsample: std::array::from_fn(|i| seed.wrapping_mul(1000).wrapping_add(100 + i as u32)),
        last_pass: std::array::from_fn(|i| seed.wrapping_mul(1000).wrapping_add(200 + i as u32)),
        shift: std::array::from_fn(|i| seed.wrapping_mul(1000).wrapping_add(300 + i as u32)),
    }
}
fn read_info(args: &mut std::str::SplitWhitespace<'_>) -> FrameInfo {
    let mut f = FrameInfo {
        save_before_color_transform: number::<u32>(args) != 0,
        ib_needs_color_transform: number::<u32>(args) != 0,
        frame_type: FrameType(number(args)),
        dc_level: number(args),
        is_last: number::<u32>(args) != 0,
        is_preview: number::<u32>(args) != 0,
        save_as_reference: number(args),
        source: number(args),
        clamp: number::<u32>(args) != 0,
        alpha_channel: number(args),
        origin: FrameOrigin {
            x0: number(args),
            y0: number(args),
        },
        blend: number::<u32>(args) != 0,
        blendmode: BlendMode(number(args)),
        duration: number(args),
        timecode: number(args),
        name: unhex(args.next().unwrap()),
        ..Default::default()
    };
    let n: usize = number(args);
    f.extra_channel_blending_info = (0..n)
        .map(|_| BlendingInfo {
            mode: BlendMode(number(args)),
            alpha_channel: number(args),
            clamp: number::<u32>(args) != 0,
            source: number(args),
            ..Default::default()
        })
        .collect();
    f
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "ef-point" => {
            let a = SplinePoint::new(f32::from_bits(number(args)), f32::from_bits(number(args)));
            let b = SplinePoint::new(f32::from_bits(number(args)), f32::from_bits(number(args)));
            u32::from(a == b).to_string()
        }
        "ef-layout" => {
            let values = [
                size_of::<JxlColorEncoding>(),
                align_of::<JxlColorEncoding>(),
                offset_of!(JxlColorEncoding, color_space),
                offset_of!(JxlColorEncoding, white_point),
                offset_of!(JxlColorEncoding, white_point_xy),
                offset_of!(JxlColorEncoding, primaries),
                offset_of!(JxlColorEncoding, primaries_red_xy),
                offset_of!(JxlColorEncoding, primaries_green_xy),
                offset_of!(JxlColorEncoding, primaries_blue_xy),
                offset_of!(JxlColorEncoding, transfer_function),
                offset_of!(JxlColorEncoding, gamma),
                offset_of!(JxlColorEncoding, rendering_intent),
                size_of::<JxlColorProfile>(),
                align_of::<JxlColorProfile>(),
                offset_of!(JxlColorProfile, icc),
                offset_of!(JxlColorProfile, color_encoding),
                offset_of!(JxlColorProfile, num_channels),
                size_of::<JxlCmsInterface>(),
                align_of::<JxlCmsInterface>(),
                offset_of!(JxlCmsInterface, set_fields_data),
                offset_of!(JxlCmsInterface, set_fields_from_icc),
                offset_of!(JxlCmsInterface, init_data),
                offset_of!(JxlCmsInterface, init),
                offset_of!(JxlCmsInterface, get_src_buf),
                offset_of!(JxlCmsInterface, get_dst_buf),
                offset_of!(JxlCmsInterface, run),
                offset_of!(JxlCmsInterface, destroy),
                size_of::<JxlBitDepth>(),
                align_of::<JxlBitDepth>(),
                offset_of!(JxlBitDepth, depth_type),
                offset_of!(JxlBitDepth, bits_per_sample),
                offset_of!(JxlBitDepth, exponent_bits_per_sample),
            ];
            values
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        }
        "ef-override" => {
            let v = Override(number(args));
            let d = number::<u32>(args) != 0;
            format!(
                "{} {}",
                u32::from(apply_override(v, d)),
                override_from_bool(d).0
            )
        }
        "ef-default" => {
            let mut out = "P".to_owned();
            params(&mut out, &CompressParams::default());
            out
        }
        "ef-params" => {
            let lossless = number::<u32>(args) != 0;
            let post = number::<u32>(args) != 0;
            let mut p = read_params(args);
            let mut out = format!(
                "{} {}",
                u32::from(p.is_lossless()),
                u32::from(p.modular_part_is_lossless())
            );
            if lossless {
                p.set_lossless();
            }
            let status = if post {
                params_post_init(&mut p)
            } else {
                Ok(())
            };
            vals!(out;status_code(&status),u32::from(p.is_lossless()),u32::from(p.modular_part_is_lossless()));
            params(&mut out, &p);
            out
        }
        "ef-policy" => {
            let p = read_params(args);
            let streaming = number::<u32>(args) != 0;
            let n: usize = number(args);
            let extra: Vec<u8> = (0..n).map(|_| number(args)).collect();
            let mut out = "H".to_owned();
            histogram(
                &mut out,
                &HistogramParams::for_modular(&p, &extra, streaming),
            );
            out
        }
        "ef-groups" => {
            let p = read_params(args);
            let x = number(args);
            let y = number(args);
            let count = number::<u32>(args) != 0;
            let mut out = get_group_size_shift(x, y, &p).to_string();
            if count {
                vals!(out;num_groups_for_frame(x,y,&p));
            }
            out
        }
        "ef-progressive" => {
            let select = number::<u32>(args) != 0;
            let custom = number::<u32>(args) != 0;
            let progressive = Override(number(args));
            let quantized = Override(number(args));
            let seed = number(args);
            let mode = read_mode(args);
            let mut splitter = ProgressiveSplitter::default();
            splitter.set_progressive_mode(mode);
            let p = CompressParams {
                progressive_mode: progressive,
                qprogressive_mode: quantized,
                custom_progressive_mode: custom.then_some(&mode),
                ..Default::default()
            };
            if select {
                set_progressive_mode(&p, &mut splitter);
            }
            let mut passes = seed_passes(seed);
            let s = splitter.init_passes(&mut passes);
            let mut out = format!("{} {}", splitter.get_num_passes(), status_code(&s));
            frame::passes(&mut out, &passes);
            out
        }
        "ef-stream" => {
            let dim = FrameDimensions {
                num_groups: number(args),
                num_dc_groups: number(args),
                ..Default::default()
            };
            let kind = ModularStreamKind(number(args));
            let qt = number(args);
            let group = number(args);
            let pass = number(args);
            let passes = number(args);
            let id = ModularStreamId {
                kind,
                quant_table_id: qt,
                group_id: group,
                pass_id: pass,
            };
            let mut out = id.id(&dim).to_string();
            for s in [
                ModularStreamId::global(),
                ModularStreamId::var_dct_dc(group),
                ModularStreamId::modular_dc(group),
                ModularStreamId::ac_metadata(group),
                ModularStreamId::modular_ac(group, pass),
            ] {
                vals!(out;s.id(&dim));
            }
            match ModularStreamId::quant_table(qt) {
                Ok(q) => vals!(out;0,q.id(&dim)),
                Err(e) => vals!(out;e as i32),
            }
            vals!(out;ModularStreamId::num(&dim,passes),NUM_QUANT_TABLES);
            out
        }
        "ef-jpeg" => {
            let jpg = read_jpeg(args);
            let mut cs = YCbCrChromaSubsampling::default();
            cs.set(&[2, 1, 1], &[2, 1, 1]).unwrap();
            let mut t = ColorTransform::XYB;
            let a = set_chroma_subsampling_from_jpeg_data(&jpg, &mut cs);
            let b = set_color_transform_from_jpeg_data(&jpg, &mut t);
            let mut out = format!("{} {} {}", status_code(&a), status_code(&b), t.0);
            frame::subsampling(&mut out, &cs);
            out
        }
        "ef-filter" => {
            let p = read_params(args);
            let encoding = FrameEncoding(number(args));
            let variant = number(args);
            let streaming = number::<u32>(args) != 0;
            let mut header = FrameHeader::new(None);
            header.encoding = encoding;
            frame::configure_filter(&mut header.loop_filter, variant);
            let s = loop_filter_from_params(&p, streaming, &mut header);
            let mut out = format!("{} {}", status_code(&s), frame_flags_from_params(&p));
            frame::filter(&mut out, &header.loop_filter);
            out
        }
        "ef-header" => {
            let p = read_params(args);
            let mut metadata = frame::configure_metadata(args);
            let x = number(args);
            let y = number(args);
            let has_jpeg = number::<u32>(args) != 0;
            let streaming = number::<u32>(args) != 0;
            let encode = number::<u32>(args) != 0;
            let prefix_bits = number(args);
            let seed: u32 = number(args);
            for e in &mut metadata.m.extra_channel_info {
                e.channel_type = ExtraChannel(number(args));
            }
            let info = read_info(args);
            let jpg = has_jpeg.then(|| read_jpeg(args));
            let mut header = FrameHeader::new(Some(&metadata));
            frame::configure_filter(&mut header.loop_filter, seed % 3);
            if seed & 1 != 0 {
                header.custom_size_or_origin = true;
                header.frame_size = FrameSize {
                    xsize: 400,
                    ysize: 200,
                };
                header.frame_origin = FrameOrigin { x0: -2, y0: 3 };
                header.flags = 128;
            }
            if seed & 2 != 0 {
                header
                    .chroma_subsampling
                    .set(&[2, 1, 1], &[2, 1, 1])
                    .unwrap();
            }
            let mut splitter = ProgressiveSplitter::default();
            set_progressive_mode(&p, &mut splitter);
            let s = make_frame_header(
                x,
                y,
                &p,
                &splitter,
                &info,
                jpg.as_ref(),
                streaming,
                &mut header,
            );
            let mut out = status_code(&s).to_string();
            frame::frame(&mut out, &header, false);
            if s.is_ok() && encode {
                let mut writer = BitWriter::new();
                prefix(&mut writer, prefix_bits);
                let can = bundle::can_encode(&mut header);
                let (ext, bits) = can.as_ref().copied().unwrap_or((0, 0));
                vals!(out;status_code(&can.as_ref().map(|_|()).map_err(|e|*e)),ext,bits);
                if can.is_ok() {
                    let written =
                        jxl::jxl::enc_fields::write_frame_header(&mut header, &mut writer, None);
                    vals!(out;status_code(&written),writer.bits_written());
                    pad(&mut writer);
                    vals!(out;hex(writer.get_span()));
                }
                frame::frame(&mut out, &header, false);
            }
            out
        }
        _ => panic!("unknown encoder-frame command"),
    }
}

pub(super) fn parameter_input(p: &CompressParams<'_>) -> String {
    let mut s = String::new();
    vals!(s;p.butteraugli_distance.to_bits(),u32::from(p.modular_mode),p.color_transform.0,p.speed_tier as i32,p.decoding_speed_tier,p.modular_group_size_shift,p.resampling,p.ec_resampling,u32::from(p.already_downsampled),p.original_butteraugli_distance.to_bits(),
        p.progressive_mode.0,p.qprogressive_mode.0,p.responsive,p.epf,p.gaborish.0,p.noise.0,u32::from(p.disable_perceptual_optimizations),p.photon_noise_iso.to_bits(),p.progressive_dc,u32::from(p.lossy_palette),p.force_lfs_jpeg_recompression,p.options.predictor.0);
    output_floats(&mut s, &p.manual_noise);
    output_floats(&mut s, &p.manual_xyb_factors);
    output_floats(&mut s, &p.ec_distance);
    s
}
fn fixture_params(i: usize) -> CompressParams<'static> {
    let distances = [
        f32::NEG_INFINITY,
        -1.0,
        -0.0,
        0.0,
        f32::from_bits(0.05f32.to_bits() - 1),
        0.05,
        0.5,
        f32::from_bits(0.5f32.to_bits() + 1),
        f32::from_bits(0.7f32.to_bits() - 1),
        0.7,
        1.5,
        4.0,
        f32::from_bits(10.0f32.to_bits() - 1),
        10.0,
        99.0,
        f32::INFINITY,
        f32::from_bits(0x7fc01234),
    ];
    let override_values = [-128, -1, 0, 1, 2, 127];
    let mut p = CompressParams {
        butteraugli_distance: distances[i % distances.len()],
        modular_mode: i % 2 != 0,
        color_transform: ColorTransform((i / 2 % 3) as u32),
        speed_tier: tier((i % 11) as i32 - 1),
        decoding_speed_tier: [0, 1, 2, 3, 4, 8, usize::MAX][i % 7],
        modular_group_size_shift: [-1, 0, 1, 2, 3][i % 5],
        resampling: [-3, -1, 0, 1, 2, 3, 4, 8][i % 8],
        ec_resampling: [-1, 0, 1, 2, 4, 8, 3][i % 7],
        already_downsampled: i % 3 == 0,
        original_butteraugli_distance: if i % 3 == 0 { -1.0 } else { 3.0 },
        progressive_mode: Override(override_values[i % 6]),
        qprogressive_mode: Override(override_values[i / 6 % 6]),
        responsive: 3,
        epf: [-3, -1, 0, 1, 2, 3, 7][i % 7],
        gaborish: Override(override_values[i / 5 % 6]),
        noise: Override(override_values[i / 7 % 6]),
        disable_perceptual_optimizations: i % 5 == 0,
        photon_noise_iso: [-1.0, 0.0, 100.0, f32::from_bits(0x7fc01234)][i % 4],
        progressive_dc: [-1, 0, 1, 2][i % 4],
        lossy_palette: i % 7 == 0,
        force_lfs_jpeg_recompression: [-1, 0, 1, 2][i % 4],
        ..Default::default()
    };
    p.options.predictor = Predictor([u32::MAX, 0, 5, 6, 14, 15][i % 6]);
    p.manual_noise = (0..[0, 1, 7, 8, 9][i / 3 % 5])
        .map(|j| j as f32 / 4.0)
        .collect();
    p.manual_xyb_factors = (0..[0, 1, 2, 3, 4][i / 7 % 5])
        .map(|j| j as f32 + 0.5)
        .collect();
    p.ec_distance = match i % 7 {
        0 => vec![],
        1 => vec![0.0, -0.0],
        2 => vec![-1.0],
        3 => vec![0.25],
        4 => vec![f32::from_bits(0x7fc01234)],
        5 => vec![-1.0, 0.0, 0.1],
        _ => vec![f32::INFINITY, f32::NEG_INFINITY],
    };
    p
}
#[test]
fn encoder_frame_parameter_defaults_lossless_setters_and_post_init_match_source() {
    let mut cases = vec!["ef-default".into(), "ef-layout".into()];
    for value in -128..=127 {
        for d in 0..2 {
            cases.push(format!("ef-override {value} {d}"));
        }
    }
    for i in 0..1600 {
        let p = fixture_params(i);
        cases.push(format!(
            "ef-params {} {} {}",
            u32::from(i % 4 == 0),
            u32::from(i % 3 != 0),
            parameter_input(&p)
        ));
    }
    compare(cases);
}
#[test]
fn encoder_frame_entropy_policy_covers_all_speed_predictor_and_streaming_choices() {
    let mut cases = Vec::new();
    for speed in -1..=9 {
        for decode in [0, 1, 2, 3, 4, 8, usize::MAX] {
            for modular in [false, true] {
                for predictor in [0, 5, 6, 14, 15, u32::MAX] {
                    for streaming in 0..2 {
                        for extra in ["0", "1 0", "2 1 255"] {
                            let mut p = CompressParams {
                                speed_tier: tier(speed),
                                decoding_speed_tier: decode,
                                modular_mode: modular,
                                ..Default::default()
                            };
                            p.options.predictor = Predictor(predictor);
                            cases.push(format!(
                                "ef-policy {} {streaming} {extra}",
                                parameter_input(&p)
                            ));
                        }
                    }
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn encoder_frame_group_geometry_and_modular_stream_ids_keep_source_word_arithmetic() {
    let mut cases = Vec::new();
    let mut seed = 442231;
    for i in 0..650 {
        let mut p = fixture_params(i);
        p.modular_group_size_shift = [-1, 0, 1, 2, 3, 30, 63, i32::MAX][i % 8];
        let (x, y) = [
            (0, 0),
            (1, 1),
            (128, 128),
            (129, 128),
            (256, 256),
            (257, 255),
            (400, 400),
            (401, 399),
            (4097, 1033),
            (usize::MAX, usize::MAX - 1),
        ][i % 10];
        let shift = get_group_size_shift(x, y, &p);
        let count = shift < 57;
        cases.push(format!(
            "ef-groups {} {x} {y} {}",
            parameter_input(&p),
            u32::from(count)
        ));
        // Count/no-count is only a fixture precondition for defined C++ shifts.
        // ID inputs are independent raw group counts and exercise size_t wrap.
        let groups = next(&mut seed) as usize;
        let dc = next(&mut seed) as usize;
        let group = next(&mut seed) as usize;
        let pass = next(&mut seed) as usize;
        let kind = [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, i32::MAX][i % 10];
        let qt = [0, 1, 16, 17, usize::MAX][i % 5];
        cases.push(format!(
            "ef-stream {groups} {dc} {kind} {qt} {group} {pass} {}",
            [0, 1, 11, usize::MAX][i % 4]
        ));
    }
    compare(cases);
}
#[test]
fn encoder_frame_progressive_configuration_keeps_reused_arrays_and_narrowed_counts() {
    let mut cases = Vec::new();
    for i in 0..720 {
        let n = if i % 29 == 0 {
            (1u64 << 32) as usize
        } else if i % 31 == 0 {
            ((1u64 << 32) as usize) + 1
        } else {
            i % 12
        };
        let mut mode = String::new();
        vals!(mode;n);
        for j in 0..MAX_NUM_PASSES {
            vals!(mode;[0,2,3,8,usize::MAX][(i+j)%5],[0,1,2,3,u32::MAX as usize+1][(i+j)%5],[0,1,2,4,8,usize::MAX,u32::MAX as usize+5][(i+j)%7]);
        }
        cases.push(format!(
            "ef-progressive {} {} {} {} {} {mode}",
            i % 2,
            i / 2 % 2,
            [-1, 0, 1, 2][i % 4],
            [-1, 0, 1, 2][i / 4 % 4],
            i % 19
        ));
    }
    compare(cases);
}
fn jpeg_input(i: usize) -> String {
    let n = i % 5;
    let mut out = n.to_string();
    for c in 0..n {
        vals!(out;if i%3==0 {[82,71,66,0][c]}else{c+1},[1,2,3,0,257,-254][(i+c)%6],[1,2,1,0,2,257][(i/3+c)%6]);
    }
    let markers: Vec<u8> = match i % 7 {
        0 => vec![],
        1 => vec![0xE0],
        2 => vec![0xEE],
        3 => vec![0xDB, 0xE1, 0xEE, 0xDA],
        4 => vec![0xEE, 0xE0],
        5 => vec![0xE2, 0xE3],
        _ => vec![0xEE, 0xEE],
    };
    vals!(out;hex(&markers));
    let count = match i % 7 {
        0 | 1 => 0,
        2 | 4 => usize::from(i % 11 != 0),
        _ => 2,
    };
    vals!(out;count);
    for j in 0..count {
        let mut data = vec![0u8; [14, 15, 16][(i + j) % 3]];
        if data.len() >= 15 {
            data[3..8].copy_from_slice(b"Adobe");
            data[14] = (i % 3) as u8;
            if i % 13 == 0 {
                data[5] = 0;
            }
        }
        vals!(out;hex(&data));
    }
    out
}
#[test]
fn encoder_frame_jpeg_selection_preserves_marker_precedence_and_partial_subsampling() {
    compare(
        (0..600)
            .map(|i| format!("ef-jpeg {}", jpeg_input(i)))
            .collect(),
    );
}
#[test]
fn encoder_frame_filter_policy_preserves_nan_bits_early_returns_and_override_defaults() {
    let cases = (0..1100)
        .map(|i| {
            format!(
                "ef-filter {} {} {} {}",
                parameter_input(&fixture_params(i)),
                i % 3,
                i / 3 % 3,
                i % 2
            )
        })
        .collect();
    compare(cases);
}
fn info_input(i: usize, baseline: bool, extras: usize) -> String {
    let info = if baseline {
        FrameInfo::default()
    } else {
        FrameInfo {
            save_before_color_transform: i % 2 != 0,
            ib_needs_color_transform: i % 3 != 0,
            frame_type: FrameType((i % 4) as u32),
            dc_level: if i % 53 == 0 {
                ((1u64 << 32) as usize) + 1
            } else {
                i % 5
            },
            is_last: i % 2 == 0,
            is_preview: i % 7 == 0,
            save_as_reference: if i % 11 == 0 { usize::MAX } else { i % 4 },
            source: i % 4,
            clamp: i % 2 == 0,
            alpha_channel: [-1, 0, 1, 5, -2][i % 5],
            origin: FrameOrigin {
                x0: [0, -2, 3][i % 3],
                y0: [0, 0, 5][i % 3],
            },
            blend: i % 3 == 0,
            blendmode: BlendMode((i % 5) as u32),
            duration: (i * 17) as u32,
            timecode: i as u32 ^ 0x12345678,
            name: vec![b'f', i as u8, 0, 255],
            ..Default::default()
        }
    };
    let mut out = String::new();
    vals!(out;u32::from(info.save_before_color_transform),u32::from(info.ib_needs_color_transform),info.frame_type.0,info.dc_level,u32::from(info.is_last),u32::from(info.is_preview),info.save_as_reference,info.source,u32::from(info.clamp),info.alpha_channel,info.origin.x0,info.origin.y0,u32::from(info.blend),info.blendmode.0,info.duration,info.timecode,hex(&info.name));
    let n = if baseline { 0 } else { i % (extras + 1) };
    vals!(out;n);
    for j in 0..n {
        vals!(out;j%5,j%3,j%2,(i+j)%4);
    }
    out
}
#[test]
fn encoder_frame_preparation_matches_complete_header_state_errors_and_encoded_bytes() {
    let mut cases = Vec::new();
    for i in 0..950 {
        let baseline = i < 120;
        let extras = i % 4;
        let mut p = if baseline {
            let mut p = CompressParams::default();
            p.set_lossless();
            p.resampling = 1;
            p.ec_resampling = 1;
            p
        } else {
            fixture_params(i)
        };
        if !baseline && i % 3 != 0 {
            p.resampling = [1, 2, 4, 8][i % 4];
            p.ec_resampling = [1, 2, 4, 8][i / 4 % 4];
        }
        let (x, y) = [(513, 257), (32, 16), (1, 1), (400, 200), (17, 9)][i % 5];
        let has_jpeg = !baseline && i % 5 == 0;
        let types = (0..extras)
            .map(|j| [0, 1, 2, 4][(i + j) % 4].to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let info = info_input(i, baseline, extras);
        let jpeg = if has_jpeg {
            jpeg_input(i / 5)
        } else {
            String::new()
        };
        cases.push(format!(
            "ef-header {} {} {} {extras} {x} {y} {} {} 1 {} {} {types} {info} {jpeg}",
            parameter_input(&p),
            u32::from(!baseline && i % 2 != 0),
            u32::from(!baseline && i % 3 == 0),
            u32::from(has_jpeg),
            i % 2,
            i % 8,
            if baseline { 0 } else { i % 4 }
        ));
    }
    compare(cases);
}

#[test]
fn encoder_frame_spline_point_equality_keeps_strict_float_thresholds() {
    let mut cases = Vec::new();
    let mut seed = 755632;
    for i in 0..800 {
        let base = if i % 19 == 0 {
            0.0
        } else if i % 17 == 0 {
            f32::from_bits(0x7fc01234)
        } else {
            (next(&mut seed) as i32 % 100000) as f32 / 16.0
        };
        let delta = [
            0.0,
            -0.0,
            f32::from_bits(1e-3f32.to_bits() - 1),
            1e-3,
            -1e-3,
            1.0,
            f32::INFINITY,
            f32::NEG_INFINITY,
        ][i % 8];
        let a = [base, base];
        let b = if i % 2 == 0 {
            [base + delta, base]
        } else {
            [base, base + delta]
        };
        cases.push(format!(
            "ef-point {} {} {} {}",
            a[0].to_bits(),
            a[1].to_bits(),
            b[0].to_bits(),
            b[1].to_bits()
        ));
    }
    compare(cases);
}
