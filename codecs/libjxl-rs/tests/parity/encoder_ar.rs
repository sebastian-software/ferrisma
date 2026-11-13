// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The `ear-chain` request: the adaptive-reconstruction (AR) heuristics with
// the decoder in the loop, compared with the original archive functions
// (tools/oracle/encoder_ar.cc) as exact words and integers. From sRGB samples
// both sides run the `ecf-chain` heuristics and InitializePassesEncoder,
// select the loop filter with LoopFilterFromParams (or explicit values that
// isolate the Gaborish and EPF stages), then report ReconstructImage and the
// ComputeBlockL2Distance image for every EPF sharpness candidate, the
// epf_sharpness plane ComputeARHeuristics chooses, and the AC-metadata
// modular images ComputeACMetadata builds from it.
use super::encoder_heuristics::{CONTENTS, CORPUS_SIZES, Content, push_floats, srgb_image};
use super::*;
use jxl::jxl::ac_strategy::AcStrategyImage;
use jxl::jxl::base::rect::Rect;
use jxl::jxl::chroma_from_luma::ColorCorrelationMap;
use jxl::jxl::coeff_order::COEFF_ORDER_MAX_SIZE;
use jxl::jxl::color_encoding_internal::ColorEncoding;
use jxl::jxl::dec_modular::ModularStreamId;
use jxl::jxl::enc_ac_strategy::{AcStrategyHeuristics, AcsPlanes};
use jxl::jxl::enc_adaptive_quantization::{
    adjust_quant_field, initial_quant_dc, initial_quant_field,
};
use jxl::jxl::enc_cache::{PassesEncoderState, compute_ac_metadata, initialize_passes_encoder};
use jxl::jxl::enc_chroma_from_luma::CfLHeuristics;
use jxl::jxl::enc_frame::loop_filter_from_params;
use jxl::jxl::enc_gaborish::gaborish_inverse;
use jxl::jxl::enc_heuristics::find_best_block_entropy_model;
use jxl::jxl::enc_heuristics_ar::{
    compute_ar_heuristics, compute_block_l2_distance, reconstruct_image,
};
use jxl::jxl::enc_modular::ModularFrameEncoder;
use jxl::jxl::enc_params::CompressParams;
use jxl::jxl::enc_xyb::{XybInputEncoding, to_xyb};
use jxl::jxl::frame_header::{ColorTransform, FrameEncoding, FrameHeader};
use jxl::jxl::image::{Image3, Image3F, ImageF, Plane};
use jxl::jxl::image_metadata::CodecMetadata;
use jxl::jxl::image_ops::{copy_image_to, fill_plane, pad_image_to_block_multiple_in_place};
use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
use jxl::jxl::modular::modular_image::Image as ModularImage;

pub(super) fn handles(kind: &str) -> bool {
    kind == "ear-chain"
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
fn full(plane: &Plane<u8>) -> Rect {
    Rect::new(0, 0, plane.xsize(), plane.ysize())
}

pub(super) fn observe(_kind: &str, args: Args<'_, '_>) -> String {
    let mut out = String::new();
    let distance = float(args);
    let xsize: usize = number(args);
    let ysize: usize = number(args);
    let gray: usize = number(args);
    let x_qm_scale: u32 = number(args);
    let b_qm_scale: u32 = number(args);
    let lf_explicit: usize = number(args);
    let gab: usize = number(args);
    let epf_iters: u32 = number(args);
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
    if lf_explicit != 0 {
        frame_header.loop_filter.gab = gab != 0;
        frame_header.loop_filter.epf_iters = epf_iters;
    } else {
        status = loop_filter_from_params(&cparams, false, &mut frame_header);
    }
    write!(
        out,
        " L {} {} {}",
        status_code(&status),
        u8::from(frame_header.loop_filter.gab),
        frame_header.loop_filter.epf_iters
    )
    .unwrap();
    if status.is_err() {
        return out;
    }

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

    // ComputeVarDCTEncodingData: the pre-Gaborish copy for the AR heuristics.
    let r = Rect::new(0, 0, opsin.xsize(), opsin.ysize());
    let mut orig_opsin = Image3F::create(r.xsize(), r.ysize()).unwrap();
    for c in 0..3 {
        copy_image_to(opsin.plane(c), orig_opsin.plane_mut(c)).unwrap();
    }
    orig_opsin
        .shrink_to(frame_dim.xsize, frame_dim.ysize)
        .unwrap();

    // LossyFrameHeuristics at kSquirrel without splines and patches.
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
        let rect = full(&shared.epf_sharpness);
        fill_plane(4u8, &mut shared.epf_sharpness, rect);
        find_best_block_entropy_model(
            &cparams,
            &shared.raw_quant_field,
            &shared.ac_strategy,
            &mut shared.block_ctx_map,
        );
    }
    status = initialize_passes_encoder(&frame_header, &opsin, r, &mut enc_state, &mut modular);
    write!(out, " I {}", status_code(&status)).unwrap();
    if status.is_err() {
        return out;
    }

    // The candidates ComputeARHeuristics reconstructs at this distance.
    let epf_steps: &[u8] = if cparams.butteraugli_distance > 4.5 {
        &[0, 4]
    } else {
        &[0, 2, 7]
    };
    for &val in epf_steps {
        let rect = full(&enc_state.shared.epf_sharpness);
        fill_plane(val, &mut enc_state.shared.epf_sharpness, rect);
        let decoded = reconstruct_image(&frame_header, &mut enc_state.shared, &enc_state.coeffs);
        let decoded_status = decoded.as_ref().map(|_| ()).map_err(|e| *e);
        write!(out, " V {val} {}", status_code(&decoded_status)).unwrap();
        let Ok(decoded) = decoded else {
            return out;
        };
        for c in 0..3 {
            print_plane(&mut out, decoded.plane(c));
        }
        let mut error = ImageF::create(frame_dim.xsize_blocks, frame_dim.ysize_blocks).unwrap();
        for by in 0..frame_dim.ysize_blocks {
            for bx in 0..frame_dim.xsize_blocks {
                error.row_mut(by)[bx] = compute_block_l2_distance(
                    &orig_opsin,
                    &decoded,
                    &enc_state.initial_quant_masking1x1,
                    by,
                    bx,
                );
            }
        }
        out.push_str(" e");
        print_plane(&mut out, &error);
    }

    {
        let rect = full(&enc_state.shared.epf_sharpness);
        fill_plane(4u8, &mut enc_state.shared.epf_sharpness, rect);
    }
    status = compute_ar_heuristics(&frame_header, &mut enc_state, &orig_opsin, r);
    write!(out, " A {}", status_code(&status)).unwrap();
    if status.is_err() {
        return out;
    }
    {
        let epf = &enc_state.shared.epf_sharpness;
        write!(out, " F {} {}", epf.xsize(), epf.ysize()).unwrap();
        for y in 0..epf.ysize() {
            for x in 0..epf.xsize() {
                write!(out, " {}", epf.row(y)[x]).unwrap();
            }
        }
    }
    status = compute_ac_metadata(&mut enc_state, &mut modular);
    write!(out, " C {}", status_code(&status)).unwrap();
    if status.is_err() {
        return out;
    }
    for g in 0..frame_dim.num_dc_groups {
        write!(out, " E {}", modular.ac_metadata_size[g]).unwrap();
        print_modular_image(
            &mut out,
            &modular.stream_images[ModularStreamId::ac_metadata(g).id(&frame_dim)],
        );
    }
    out
}

/// One `ear-chain` request. `lf` is None for LoopFilterFromParams, or the
/// explicit (Gaborish, EPF iterations) pair.
#[allow(clippy::too_many_arguments)] // One parameter per request field.
fn request(
    xsize: usize,
    ysize: usize,
    content: Content,
    gray: bool,
    seed: u64,
    d: f32,
    qm: (u32, u32),
    lf: Option<(bool, u32)>,
) -> String {
    let srgb = srgb_image(xsize, ysize, content, gray, seed);
    let (explicit, gab, epf) = lf.map_or((0, 0, 0), |(g, e)| (1, u8::from(g), e));
    let mut s = format!(
        "ear-chain {} {xsize} {ysize} {} {} {} {explicit} {gab} {epf}",
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

/// Compares each request and reports the first differing field and the
/// section tag before it.
fn compare_located(requests: Vec<String>) -> usize {
    let observations = original_observations(&requests);
    let mut ar_runs = 0;
    for (request, cpp) in requests.iter().zip(observations) {
        let rust = super::observe(request);
        if !super::parity_eq(request, &rust, &cpp) {
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
                    .map(|i| format!("{} {} at field {i}", v[i], v.get(i + 1).unwrap_or(&"")))
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
        // ComputeARHeuristics ran its search (not the fill of value 4):
        // distance of at least 0.5 and EPF enabled (`L <status> <gab> <iters>`).
        let fields: Vec<&str> = cpp.split(' ').collect();
        let distance = f32::from_bits(request.split(' ').nth(1).unwrap().parse().unwrap());
        if distance >= 0.5 && fields.get(5).is_some_and(|&iters| iters != "0") {
            ar_runs += 1;
        }
        if std::env::var_os("EAR_DEBUG").is_some() {
            let f = cpp
                .find(" F ")
                .map_or("", |i| &cpp[i..(i + 200).min(cpp.len())]);
            println!("{} | {} | {f}", &cpp[..cpp.len().min(60)], cpp.len());
        }
    }
    println!(
        "{} C++/Rust ear-chain cases matched, {ar_runs} with the AR search",
        requests.len()
    );
    ar_runs
}

/// The step 1 corpus (sizes up to 70,000 pixels, every content, gray and
/// RGB) at the default distance 1.0 with LoopFilterFromParams.
#[test]
fn encoder_ar_chain_matches_small_corpus() {
    let mut cases = Vec::new();
    for (i, &(xsize, ysize)) in CORPUS_SIZES.iter().enumerate() {
        if xsize * ysize > 70000 {
            continue;
        }
        for (j, &content) in CONTENTS.iter().enumerate() {
            for gray in [false, true] {
                let seed = (i * 10 + j) as u64;
                cases.push(request(
                    xsize,
                    ysize,
                    content,
                    gray,
                    seed,
                    1.0,
                    (3, 2),
                    None,
                ));
            }
        }
    }
    assert!(compare_located(cases) > 0);
}

/// Distances from 0.7 to 2.9 (one, two and, from 4.0, three EPF iterations;
/// the 4.5 boundary of the two-candidate branch), the 0.5/0.69 boundaries
/// where the AR heuristics fill the default, quantization-matrix scale
/// pairs, and the large corpus sizes.
#[test]
fn encoder_ar_chain_matches_distances_and_large_images() {
    let mut cases = Vec::new();
    let distances = [0.7f32, 1.5, 2.0, 2.9];
    for (i, &(xsize, ysize)) in CORPUS_SIZES.iter().enumerate() {
        for (j, &content) in CONTENTS.iter().enumerate() {
            let large = xsize * ysize > 70000;
            if large && !matches!(content, Content::Photo | Content::SomeTestImage) {
                continue;
            }
            let seed = (i * 10 + j) as u64;
            if large {
                let d = [1.0f32, 2.9][j % 2];
                cases.push(request(
                    xsize,
                    ysize,
                    content,
                    j % 2 == 0,
                    seed,
                    d,
                    (3, 2),
                    None,
                ));
                continue;
            }
            for (k, &d) in distances.iter().enumerate() {
                let gray = (i + j + k) % 2 == 0;
                let qm = [(2, 2), (3, 2), (4, 3)][(i + k) % 3];
                cases.push(request(xsize, ysize, content, gray, seed, d, qm, None));
            }
        }
    }
    let boundary = [0.5f32, 0.69, 4.0, 4.5, 4.6, 6.0];
    for (k, &d) in boundary.iter().enumerate() {
        for (j, &content) in CONTENTS.iter().enumerate() {
            let (xsize, ysize) = [(63, 129), (45, 15), (9, 17)][(j + k) % 3];
            let seed = (100 + k * 10 + j) as u64;
            cases.push(request(
                xsize,
                ysize,
                content,
                (j + k) % 2 == 1,
                seed,
                d,
                (3, 2),
                None,
            ));
        }
    }
    assert!(compare_located(cases) > 0);
}

/// Unit isolation of the decoder in the loop: explicit loop filters without
/// Gaborish and EPF (DecodeGroupForRoundtrip and the output writer alone),
/// Gaborish alone, and each EPF iteration count with and without Gaborish.
#[test]
fn encoder_ar_reconstruction_matches_isolated_loop_filters() {
    let mut cases = Vec::new();
    let filters = [
        (false, 0),
        (true, 0),
        (false, 1),
        (false, 2),
        (false, 3),
        (true, 1),
        (true, 2),
        (true, 3),
    ];
    let sizes = [(1, 1), (8, 8), (9, 17), (47, 15), (63, 129), (256, 256)];
    for (f, &lf) in filters.iter().enumerate() {
        for (s, &(xsize, ysize)) in sizes.iter().enumerate() {
            let content = CONTENTS[(f + s) % CONTENTS.len()];
            let d = [1.0f32, 2.9, 0.7][(f + s) % 3];
            let seed = (200 + f * 10 + s) as u64;
            cases.push(request(
                xsize,
                ysize,
                content,
                (f + s) % 2 == 0,
                seed,
                d,
                (3, 2),
                Some(lf),
            ));
        }
    }
    assert!(compare_located(cases) > 0);
}
