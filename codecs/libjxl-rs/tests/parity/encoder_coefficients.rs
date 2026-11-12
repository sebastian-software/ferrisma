// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The `ecf-chain` request: the VarDCT encoder from sRGB samples through the
// kSquirrel heuristics, InitializePassesEncoder (ComputeCoefficients,
// AddVarDCTDC, AdaptiveDCSmoothing), ComputeACMetadata, the coefficient
// orders, TokenizeCoefficients and the serialized DC-info, global modular,
// DC-group, AC-global and AC-group sections, compared with the original
// archive functions (tools/oracle/encoder_coefficients.cc) as exact words,
// integers and bytes. ComputeARHeuristics is not run on either side; the
// oracle states the sequence it repeats.
use super::encoder_heuristics::{CONTENTS, CORPUS_SIZES, Content, push_floats, srgb_image};
use super::*;
use jxl::jxl::ac_strategy::AcStrategyImage;
use jxl::jxl::base::rect::Rect;
use jxl::jxl::chroma_from_luma::ColorCorrelationMap;
use jxl::jxl::coeff_order::{COEFF_ORDER_MAX_SIZE, coeff_order_offset};
use jxl::jxl::coeff_order_fwd::NUM_ORDERS;
use jxl::jxl::color_encoding_internal::ColorEncoding;
use jxl::jxl::common::SpeedTier;
use jxl::jxl::dct_util::ConstAcPtr;
use jxl::jxl::dec_modular::ModularStreamId;
use jxl::jxl::enc_ac_strategy::{AcStrategyHeuristics, AcsPlanes};
use jxl::jxl::enc_adaptive_quantization::{
    adjust_quant_field, initial_quant_dc, initial_quant_field,
};
use jxl::jxl::enc_ans::Token;
use jxl::jxl::enc_ans_params::{HistogramParams, LZ77Method};
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_cache::{PassesEncoderState, compute_ac_metadata, initialize_passes_encoder};
use jxl::jxl::enc_chroma_from_luma::{CfLHeuristics, color_correlation_encode_dc};
use jxl::jxl::enc_coeff_order::{compute_coeff_order, compute_used_orders, encode_coeff_orders};
use jxl::jxl::enc_context_map::encode_block_ctx_map;
use jxl::jxl::enc_entropy_coder::tokenize_coefficients;
use jxl::jxl::enc_fields::write_quantizer_params;
use jxl::jxl::enc_gaborish::gaborish_inverse;
use jxl::jxl::enc_group::encode_group_tokenized_coefficients;
use jxl::jxl::enc_heuristics::find_best_block_entropy_model;
use jxl::jxl::enc_modular::ModularFrameEncoder;
use jxl::jxl::enc_params::CompressParams;
use jxl::jxl::enc_quant_weights::dequant_matrices_encode_default;
use jxl::jxl::enc_xyb::{XybInputEncoding, to_xyb};
use jxl::jxl::frame_dimensions::{DCT_BLOCK_SIZE, GROUP_DIM_IN_BLOCKS};
use jxl::jxl::frame_header::{ColorTransform, FrameEncoding, FrameHeader, ORDER_ENC};
use jxl::jxl::image::{Image3, Image3F, ImageF, Plane};
use jxl::jxl::image_metadata::CodecMetadata;
use jxl::jxl::image_ops::{fill_plane, pad_image_to_block_multiple_in_place};
use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
use jxl::jxl::modular::modular_image::Image as ModularImage;
use jxl::jxl::quantizer::Quantizer;

pub(super) fn handles(kind: &str) -> bool {
    kind == "ecf-chain"
}

type Args<'a, 'b> = &'a mut std::str::SplitWhitespace<'b>;

fn float(args: Args<'_, '_>) -> f32 {
    f32::from_bits(number(args))
}
fn print_plane(out: &mut String, plane: &ImageF) {
    write!(out, " P {} {}", plane.xsize(), plane.ysize()).unwrap();
    for y in 0..plane.ysize() {
        for x in 0..plane.xsize() {
            write!(out, " {}", plane.row(y)[x].to_bits()).unwrap();
        }
    }
}
fn print_ints<T: jxl::jxl::image::Sample + Copy + Into<i64>>(
    out: &mut String,
    tag: &str,
    plane: &Plane<T>,
) {
    write!(out, " {tag} {} {}", plane.xsize(), plane.ysize()).unwrap();
    for y in 0..plane.ysize() {
        for x in 0..plane.xsize() {
            write!(out, " {}", plane.row(y)[x].into()).unwrap();
        }
    }
}
fn print_strategy(out: &mut String, acs: &AcStrategyImage) {
    out.push_str(" S");
    for y in 0..acs.ysize() {
        let row = acs.const_row(y, 0);
        for x in 0..acs.xsize() {
            let a = row.get(x);
            write!(
                out,
                " {}:{}",
                a.raw_strategy(),
                u8::from(a.is_first_block())
            )
            .unwrap();
        }
    }
}
fn print_quantizer(out: &mut String, q: &Quantizer) {
    let params = q.get_params();
    write!(
        out,
        " Q {} {} {} {} {}",
        params.global_scale,
        params.quant_dc,
        q.scale().to_bits(),
        q.inv_global_scale().to_bits(),
        q.inv_quant_dc().to_bits()
    )
    .unwrap();
    for c in 0..3 {
        write!(out, " {}", q.mul_dc()[c].to_bits()).unwrap();
    }
    for c in 0..3 {
        write!(out, " {}", q.inv_mul_dc()[c].to_bits()).unwrap();
    }
}
fn print_bytes(out: &mut String, tag: &str, mut writer: BitWriter) {
    write!(out, " {tag} {}", writer.bits_written()).unwrap();
    writer.zero_pad_to_byte();
    for b in writer.get_span() {
        write!(out, " {b}").unwrap();
    }
}
fn print_tokens(out: &mut String, tokens: &[Token]) {
    write!(out, " K {}", tokens.len()).unwrap();
    for t in tokens {
        write!(out, " {}:{}", t.context(), t.value).unwrap();
    }
}
fn print_modular_image(out: &mut String, image: &ModularImage) {
    write!(out, " M {}", image.channel.len()).unwrap();
    for ch in &image.channel {
        write!(out, " {} {} {} {}", ch.w, ch.h, ch.hshift, ch.vshift).unwrap();
        for y in 0..ch.h {
            for x in 0..ch.w {
                write!(out, " {}", ch.plane.row(y)[x]).unwrap();
            }
        }
    }
}
fn tiles(xsize_blocks: usize, ysize_blocks: usize) -> Vec<Rect> {
    let n = xsize_blocks.div_ceil(8);
    let count = n * ysize_blocks.div_ceil(8);
    (0..count)
        .map(|tid| {
            let (tx, ty) = (tid % n, tid / n);
            let by0 = ty * 8;
            let by1 = ((ty + 1) * 8).min(ysize_blocks);
            let bx0 = tx * 8;
            let bx1 = ((tx + 1) * 8).min(xsize_blocks);
            Rect::new(bx0, by0, bx1 - bx0, by1 - by0)
        })
        .collect()
}

pub(super) fn observe(_kind: &str, args: Args<'_, '_>) -> String {
    let mut out = String::new();
    let distance = float(args);
    let xsize: usize = number(args);
    let ysize: usize = number(args);
    let gray: usize = number(args);
    let x_qm_scale: u32 = number(args);
    let b_qm_scale: u32 = number(args);
    let pxs = xsize.div_ceil(8) * 8;
    let pys = ysize.div_ceil(8) * 8;
    let mut opsin = Image3F::create(pxs, pys).unwrap();
    opsin.shrink_to(xsize, ysize).unwrap();
    for c in 0..3 {
        let plane = opsin.plane_mut(c);
        for y in 0..ysize {
            for x in 0..xsize {
                plane.row_mut(y)[x] = float(args);
            }
        }
    }
    let encoding = XybInputEncoding::new(ColorEncoding::srgb(gray != 0)).unwrap();
    let mut status = to_xyb(encoding, 255.0, None, None, &mut opsin, None);
    if status.is_ok() {
        status = pad_image_to_block_multiple_in_place(&mut opsin, 8);
    }
    write!(out, "X {}", status_code(&status)).unwrap();
    if status.is_err() {
        return out;
    }

    let mut metadata = CodecMetadata::default();
    metadata.size.set(xsize, ysize).unwrap();
    metadata.m.color_encoding = ColorEncoding::srgb(gray != 0).clone();
    let mut frame_header = FrameHeader::new(Some(&metadata));
    frame_header.encoding = FrameEncoding::VAR_DCT;
    frame_header.color_transform = ColorTransform::XYB;
    frame_header.x_qm_scale = x_qm_scale;
    frame_header.b_qm_scale = b_qm_scale;

    let cparams = CompressParams {
        butteraugli_distance: distance,
        ..CompressParams::default()
    };
    let mut enc_state = PassesEncoderState::new();
    enc_state.cparams = cparams.clone();
    enc_state.shared.metadata = Some(&metadata);
    enc_state.shared.frame_dim = frame_header.to_frame_dimensions();
    let frame_dim = enc_state.shared.frame_dim;
    let mut modular = ModularFrameEncoder::create(&frame_header, &cparams, false).unwrap();
    {
        let shared = &mut enc_state.shared;
        shared.ac_strategy =
            AcStrategyImage::create(frame_dim.xsize_blocks, frame_dim.ysize_blocks).unwrap();
        shared.raw_quant_field =
            Plane::create(frame_dim.xsize_blocks, frame_dim.ysize_blocks).unwrap();
        shared.epf_sharpness =
            Plane::create(frame_dim.xsize_blocks, frame_dim.ysize_blocks).unwrap();
        shared.cmap = ColorCorrelationMap::create(frame_dim.xsize, frame_dim.ysize, true).unwrap();
        shared.coeff_order_size = COEFF_ORDER_MAX_SIZE;
        shared.coeff_orders.resize(
            frame_header.passes.num_passes as usize * COEFF_ORDER_MAX_SIZE,
            0,
        );
        shared.quant_dc = Plane::create(frame_dim.xsize_blocks, frame_dim.ysize_blocks).unwrap();
        shared.dc_storage = Image3::create(frame_dim.xsize_blocks, frame_dim.ysize_blocks).unwrap();
    }
    let num_passes = enc_state.progressive_splitter.get_num_passes();
    enc_state.passes.resize_with(num_passes, Default::default);
    for pass in &mut enc_state.passes {
        pass.ac_tokens.resize_with(frame_dim.num_groups, Vec::new);
    }

    // LossyFrameHeuristics at kSquirrel without splines and patches.
    let r = Rect::new(0, 0, opsin.xsize(), opsin.ysize());
    let quant_dc = initial_quant_dc(cparams.butteraugli_distance);
    let mut acs_heuristics = AcStrategyHeuristics::new(MemoryManagerPtr::NATIVE, &cparams);
    let mut cfl_heuristics = CfLHeuristics::new(MemoryManagerPtr::NATIVE);
    let mut initial_quant_masking = ImageF::default();
    let iqf = initial_quant_field(
        cparams.butteraugli_distance,
        &opsin,
        r,
        None,
        1.0,
        &mut initial_quant_masking,
        &mut enc_state.initial_quant_masking1x1,
    );
    let iqf_status = iqf.as_ref().map(|_| ()).map_err(|e| *e);
    write!(out, " {}", status_code(&iqf_status)).unwrap();
    let Ok(mut initial_quant_field) = iqf else {
        return out;
    };
    let q = (0.39 / f64::from(cparams.butteraugli_distance)) as f32;
    {
        let shared = &mut enc_state.shared;
        shared
            .quantizer
            .compute_global_scale_and_quant(&shared.matrices, quant_dc, q, 0.0);
    }
    let weight = [1.0f32; 3];
    status = gaborish_inverse(&mut opsin, r, &weight, None);
    if status.is_ok() {
        status = cfl_heuristics.init(r);
    }
    if status.is_ok() {
        status = acs_heuristics.init(
            &opsin,
            r,
            &initial_quant_field,
            &initial_quant_masking,
            &enc_state.initial_quant_masking1x1,
            &mut enc_state.shared.matrices,
        );
    }
    if status.is_ok() {
        status = acs_heuristics.prepare_for_threads(1);
    }
    if status.is_ok() {
        status = cfl_heuristics.prepare_for_threads(1);
    }
    for t in tiles(frame_dim.xsize_blocks, frame_dim.ysize_blocks) {
        if status.is_err() {
            break;
        }
        let shared = &mut enc_state.shared;
        status = cfl_heuristics.compute_tile(
            t,
            &opsin,
            r,
            &shared.matrices,
            None,
            None,
            None,
            false,
            0,
            &mut shared.cmap,
        );
        if status.is_ok() {
            let planes = AcsPlanes {
                src: &opsin,
                rect_in: r,
                quant_field: &initial_quant_field,
                mask: &initial_quant_masking,
                mask1x1: &enc_state.initial_quant_masking1x1,
                dequant: &shared.matrices,
            };
            status =
                acs_heuristics.process_rect(t, &shared.cmap, &mut shared.ac_strategy, 0, &planes);
        }
        if status.is_ok() {
            status = adjust_quant_field(
                &shared.ac_strategy,
                t,
                cparams.butteraugli_distance,
                &mut initial_quant_field,
            );
        }
        if status.is_err() {
            break;
        }
        shared
            .quantizer
            .set_quant_field_rect(&initial_quant_field, t, &mut shared.raw_quant_field);
        status = cfl_heuristics.compute_tile(
            t,
            &opsin,
            r,
            &shared.matrices,
            Some(&shared.ac_strategy),
            Some(&shared.raw_quant_field),
            Some(&shared.quantizer),
            false,
            0,
            &mut shared.cmap,
        );
    }
    if status.is_ok() {
        status = acs_heuristics.finalize(&frame_dim, &enc_state.shared.ac_strategy, None);
    }
    write!(out, " H {}", status_code(&status)).unwrap();
    if status.is_err() {
        return out;
    }
    {
        let shared = &mut enc_state.shared;
        let full = Rect::new(
            0,
            0,
            shared.epf_sharpness.xsize(),
            shared.epf_sharpness.ysize(),
        );
        fill_plane(4u8, &mut shared.epf_sharpness, full);
        find_best_block_entropy_model(
            &cparams,
            &shared.raw_quant_field,
            &shared.ac_strategy,
            &mut shared.block_ctx_map,
        );
        let bcm = &shared.block_ctx_map;
        write!(out, " B {} {}", bcm.num_ctxs, bcm.qf_thresholds.len()).unwrap();
        for t in &bcm.qf_thresholds {
            write!(out, " {t}").unwrap();
        }
        write!(out, " {}", bcm.ctx_map.len()).unwrap();
        for v in &bcm.ctx_map {
            write!(out, " {v}").unwrap();
        }
    }

    // ComputeVarDCTEncodingData after the heuristics, without AR.
    status = initialize_passes_encoder(&frame_header, &opsin, r, &mut enc_state, &mut modular);
    write!(out, " I {}", status_code(&status)).unwrap();
    if status.is_err() {
        return out;
    }
    status = compute_ac_metadata(&mut enc_state, &mut modular);
    write!(out, " {}", status_code(&status)).unwrap();
    if status.is_err() {
        return out;
    }
    {
        let shared = &enc_state.shared;
        print_quantizer(&mut out, &shared.quantizer);
        out.push_str(" D");
        for c in 0..3 {
            write!(out, " {}", shared.matrices.dc_quants()[c].to_bits()).unwrap();
        }
        print_strategy(&mut out, &shared.ac_strategy);
        print_ints(&mut out, "R", &shared.raw_quant_field);
        print_ints(&mut out, "C", &shared.cmap.ytox_map);
        print_ints(&mut out, "C", &shared.cmap.ytob_map);
        write!(
            out,
            " {} {}",
            enc_state.x_qm_multiplier.to_bits(),
            enc_state.b_qm_multiplier.to_bits()
        )
        .unwrap();
        for c in 0..3 {
            print_plane(&mut out, shared.dc_storage.plane(c));
        }
        print_ints(&mut out, "U", &shared.quant_dc);
        for g in 0..frame_dim.num_groups {
            let br = frame_dim.block_group_rect(g);
            let n = br.xsize() * br.ysize() * DCT_BLOCK_SIZE;
            write!(out, " G {g}").unwrap();
            for c in 0..3 {
                let ConstAcPtr::I32(row) = enc_state.coeffs[0].as_ref().unwrap().plane_row(c, g, 0)
                else {
                    unreachable!()
                };
                for v in &row[..n] {
                    write!(out, " {v}").unwrap();
                }
            }
        }
        for g in 0..frame_dim.num_dc_groups {
            write!(
                out,
                " E {} {}",
                modular.extra_dc_precision[g], modular.ac_metadata_size[g]
            )
            .unwrap();
            print_modular_image(
                &mut out,
                &modular.stream_images[ModularStreamId::var_dct_dc(g).id(&frame_dim)],
            );
            print_modular_image(
                &mut out,
                &modular.stream_images[ModularStreamId::ac_metadata(g).id(&frame_dim)],
            );
        }
    }

    // ComputeAllCoeffOrders.
    let used_orders_info = compute_used_orders(
        cparams.speed_tier,
        &enc_state.shared.ac_strategy,
        Rect::new(
            0,
            0,
            enc_state.shared.raw_quant_field.xsize(),
            enc_state.shared.raw_quant_field.ysize(),
        ),
    );
    enc_state.used_orders.resize(num_passes, 0);
    {
        let shared = &mut enc_state.shared;
        status = compute_coeff_order(
            cparams.speed_tier,
            enc_state.coeffs[0].as_ref().unwrap(),
            &shared.ac_strategy,
            &frame_dim,
            &mut enc_state.used_orders[0],
            enc_state.used_acs,
            used_orders_info.0,
            used_orders_info.1,
            &mut shared.coeff_orders,
        );
    }
    enc_state.used_acs |= used_orders_info.0;
    write!(
        out,
        " O {} {} {} {} {}",
        status_code(&status),
        used_orders_info.0,
        used_orders_info.1,
        enc_state.used_orders[0],
        enc_state.used_acs
    )
    .unwrap();
    if status.is_err() {
        return out;
    }
    for ord in 0..usize::from(NUM_ORDERS) {
        if enc_state.used_acs & (1 << ord) == 0 {
            continue;
        }
        for i in coeff_order_offset(ord, 0)..coeff_order_offset(ord + 1, 0) {
            write!(out, " {}", enc_state.shared.coeff_orders[i]).unwrap();
        }
    }
    enc_state.shared.num_histograms = 1;
    enc_state.histogram_idx.resize(frame_dim.num_groups, 0);

    // TokenizeAllCoefficients.
    let mut num_nzeroes = Image3::<i32>::create(GROUP_DIM_IN_BLOCKS, GROUP_DIM_IN_BLOCKS).unwrap();
    for g in 0..frame_dim.num_groups {
        let mut tokens = Vec::new();
        {
            let coeffs = enc_state.coeffs[0].as_ref().unwrap();
            let rows: [&[i32]; 3] = std::array::from_fn(|c| match coeffs.plane_row(c, g, 0) {
                ConstAcPtr::I32(row) => row,
                ConstAcPtr::I16(_) => unreachable!(),
            });
            let shared = &enc_state.shared;
            status = tokenize_coefficients(
                &shared.coeff_orders,
                frame_dim.block_group_rect(g),
                rows,
                &shared.ac_strategy,
                &frame_header.chroma_subsampling,
                &mut num_nzeroes,
                &mut tokens,
                &shared.quant_dc,
                &shared.raw_quant_field,
                &shared.block_ctx_map,
            );
        }
        write!(out, " T {}", status_code(&status)).unwrap();
        if status.is_err() {
            return out;
        }
        print_tokens(&mut out, &tokens);
        enc_state.passes[0].ac_tokens[g] = tokens;
    }

    status = modular.compute_tree();
    if status.is_ok() {
        status = modular.compute_tokens();
    }
    write!(out, " Z {}", status_code(&status)).unwrap();
    if status.is_err() {
        return out;
    }

    {
        let mut writer = BitWriter::new();
        let shared = &enc_state.shared;
        let mut params = shared.quantizer.get_params();
        status = write_quantizer_params(&mut params, &mut writer, LayerType::Quant, None);
        if status.is_ok() {
            status = encode_block_ctx_map(&shared.block_ctx_map, &mut writer, None);
        }
        if status.is_ok() {
            status =
                color_correlation_encode_dc(&shared.cmap.base(), &mut writer, LayerType::Dc, None);
        }
        write!(out, " W {}", status_code(&status)).unwrap();
        if status.is_err() {
            return out;
        }
        print_bytes(&mut out, "dcinfo", writer);
    }
    {
        let mut writer = BitWriter::new();
        status = modular.encode_global_info(false, &mut writer, None);
        if status.is_ok() {
            status = modular.encode_stream(
                &mut writer,
                None,
                LayerType::ModularGlobal,
                &ModularStreamId::global(),
            );
        }
        write!(out, " W {}", status_code(&status)).unwrap();
        if status.is_err() {
            return out;
        }
        print_bytes(&mut out, "global", writer);
    }
    for g in 0..frame_dim.num_dc_groups {
        let mut writer = BitWriter::new();
        let precision = modular.extra_dc_precision[g];
        status = writer.with_max_bits(
            2,
            LayerType::Dc,
            None,
            |w, _| {
                w.write(2, u64::from(precision));
                Ok(())
            },
            false,
        );
        if status.is_ok() {
            status = modular.encode_stream(
                &mut writer,
                None,
                LayerType::Dc,
                &ModularStreamId::var_dct_dc(g),
            );
        }
        if status.is_ok() {
            status = modular.encode_stream(
                &mut writer,
                None,
                LayerType::ModularDcGroup,
                &ModularStreamId::modular_dc(g),
            );
        }
        let r = frame_dim.dc_group_rect(g);
        let nb_bits = jxl::jxl::base::bits::ceil_log2_nonzero(r.xsize() * r.ysize());
        let size = modular.ac_metadata_size[g];
        if status.is_ok() && nb_bits != 0 {
            status = writer.with_max_bits(
                nb_bits,
                LayerType::ControlFields,
                None,
                |w, _| {
                    w.write(nb_bits, size as u64 - 1);
                    Ok(())
                },
                false,
            );
        }
        if status.is_ok() {
            status = modular.encode_stream(
                &mut writer,
                None,
                LayerType::ControlFields,
                &ModularStreamId::ac_metadata(g),
            );
        }
        write!(out, " W {}", status_code(&status)).unwrap();
        if status.is_err() {
            return out;
        }
        print_bytes(&mut out, "dcgroup", writer);
    }
    {
        let mut writer = BitWriter::new();
        status = dequant_matrices_encode_default(
            &enc_state.shared.matrices,
            &mut writer,
            LayerType::Quant,
            None,
        );
        let num_histo_bits = jxl::jxl::base::bits::ceil_log2_nonzero(frame_dim.num_groups);
        let num_histograms = enc_state.shared.num_histograms;
        if status.is_ok() && num_histo_bits != 0 {
            status = writer.with_max_bits(
                num_histo_bits,
                LayerType::Ac,
                None,
                |w, _| {
                    w.write(num_histo_bits, num_histograms as u64 - 1);
                    Ok(())
                },
                false,
            );
        }
        let used_orders = enc_state.used_orders[0];
        let mut order_bits = 0;
        if status.is_ok() {
            status =
                jxl::jxl::fields::u32_coder::can_encode(ORDER_ENC, used_orders, &mut order_bits);
        }
        if status.is_ok() {
            status = writer.with_max_bits(
                order_bits,
                LayerType::Order,
                None,
                |w, _| jxl::jxl::enc_fields::u32_coder::write(ORDER_ENC, used_orders, w),
                false,
            );
        }
        if status.is_ok() {
            status = encode_coeff_orders(
                used_orders as u16,
                &enc_state.shared.coeff_orders,
                &mut writer,
                LayerType::Order,
                None,
            );
        }
        let num_ac_contexts = enc_state.shared.block_ctx_map.num_ac_contexts() as usize;
        let mut hist_params = HistogramParams::with_tier(cparams.speed_tier, num_ac_contexts);
        if cparams.speed_tier > SpeedTier::Tortoise {
            hist_params.lz77_method = LZ77Method::None;
        }
        if cparams.decoding_speed_tier >= 1 {
            hist_params.max_histograms = 6;
        }
        hist_params.streaming_mode = false;
        hist_params.initialize_global_state = true;
        if status.is_ok() {
            let pass = &mut enc_state.passes[0];
            status = jxl::jxl::enc_entropy::build_and_encode_histograms(
                &hist_params,
                num_histograms * num_ac_contexts,
                &mut pass.ac_tokens,
                &mut pass.codes,
                Some(&mut writer),
                LayerType::Ac,
                None,
            )
            .map(|_| ());
        }
        write!(out, " W {}", status_code(&status)).unwrap();
        if status.is_err() {
            return out;
        }
        print_bytes(&mut out, "acglobal", writer);
    }
    for g in 0..frame_dim.num_groups {
        let mut writer = BitWriter::new();
        status = encode_group_tokenized_coefficients(
            g,
            0,
            enc_state.histogram_idx[g],
            &enc_state,
            &mut writer,
            None,
        );
        if status.is_ok() {
            status = modular.encode_stream(
                &mut writer,
                None,
                LayerType::ModularAcGroup,
                &ModularStreamId::modular_ac(g, 0),
            );
        }
        write!(out, " W {}", status_code(&status)).unwrap();
        if status.is_err() {
            return out;
        }
        print_bytes(&mut out, "acgroup", writer);
    }
    out
}

fn request(
    xsize: usize,
    ysize: usize,
    content: Content,
    gray: bool,
    seed: u64,
    d: f32,
    qm: (u32, u32),
) -> String {
    let srgb = srgb_image(xsize, ysize, content, gray, seed);
    let mut s = format!(
        "ecf-chain {} {xsize} {ysize} {} {} {}",
        d.to_bits(),
        u8::from(gray),
        qm.0,
        qm.1
    );
    for plane in &srgb {
        push_floats(&mut s, plane);
    }
    s
}

/// Compares each request and reports the first differing field.
fn compare_located(requests: Vec<String>) {
    let observations = original_observations(&requests);
    for (request, cpp) in requests.iter().zip(observations) {
        let rust = super::observe(request);
        if rust != cpp {
            let r: Vec<&str> = rust.split(' ').collect();
            let c: Vec<&str> = cpp.split(' ').collect();
            let first = r
                .iter()
                .zip(&c)
                .position(|(a, b)| a != b)
                .unwrap_or(r.len().min(c.len()));
            let tag = |v: &[&str]| {
                v[..first.min(v.len())]
                    .iter()
                    .rposition(|f| f.chars().next().is_some_and(char::is_alphabetic))
                    .map(|i| format!("{} at field {i}", v[i]))
                    .unwrap_or_default()
            };
            let lo = first.saturating_sub(4);
            panic!(
                "first difference at field {first} (section {}), rust {:?} vs C++ {:?}; request prefix {}",
                tag(&r),
                &r[lo..(first + 6).min(r.len())],
                &c[lo..(first + 6).min(c.len())],
                &request[..request.len().min(80)]
            );
        }
    }
    println!("{} C++/Rust ecf-chain cases matched", requests.len());
}

#[test]
fn encoder_coefficients_chain_matches_small_corpus() {
    let mut cases = Vec::new();
    for (i, &(xsize, ysize)) in CORPUS_SIZES.iter().enumerate() {
        if xsize * ysize > 70000 {
            continue;
        }
        for (j, &content) in CONTENTS.iter().enumerate() {
            for gray in [false, true] {
                cases.push(request(
                    xsize,
                    ysize,
                    content,
                    gray,
                    (i * 10 + j) as u64,
                    1.0,
                    (3, 2),
                ));
            }
        }
    }
    compare_located(cases);
}

/// The step 1 distances and the x/b quantization-matrix scales that
/// ComputeChromacityAdjustments selects, on the remaining corpus sizes.
#[test]
fn encoder_coefficients_chain_matches_distances_and_large_images() {
    let mut cases = Vec::new();
    for (i, &(xsize, ysize)) in CORPUS_SIZES.iter().enumerate() {
        for (j, &content) in CONTENTS.iter().enumerate() {
            let large = xsize * ysize > 70000;
            if large && !matches!(content, Content::Photo | Content::SomeTestImage) {
                continue;
            }
            let seed = (i * 10 + j) as u64;
            if large {
                cases.push(request(xsize, ysize, content, false, seed, 1.0, (3, 2)));
                continue;
            }
            for (k, &d) in [0.5f32, 1.5, 2.9].iter().enumerate() {
                let gray = (i + j + k) % 2 == 0;
                let qm = [(2, 2), (3, 2), (4, 3)][(i + k) % 3];
                cases.push(request(xsize, ysize, content, gray, seed, d, qm));
            }
        }
    }
    compare_located(cases);
}
