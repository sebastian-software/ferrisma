// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compare the encoder's quantizer field selection, adaptive quantization,
// chroma-from-luma and AC strategy heuristics with the original library
// functions as exact float words, integers and strategy layers, on the step 1
// image corpus at kSquirrel and on boundary inputs of each unit.
//
// The compared domain is the original's defined domain: finite inputs. NaN
// quant fields violate std::nth_element's strict weak ordering, and NaN or
// out-of-range values reach undefined float-to-int conversions (ClampVal,
// ComputeGlobalScaleAndQuant) in the original.
use super::*;
use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyImage, AcStrategyType};
use jxl::jxl::base::rect::Rect;
use jxl::jxl::chroma_from_luma::ColorCorrelationMap;
use jxl::jxl::common::SpeedTier;
use jxl::jxl::enc_ac_strategy::{
    AcStrategyHeuristics, AcsConfig, AcsPlanes, estimate_entropy, find_best_8x8_transform,
};
use jxl::jxl::enc_adaptive_quantization::{
    adjust_quant_field, initial_quant_dc, initial_quant_field,
};
use jxl::jxl::enc_aux_out::AuxOut as HeuristicsAuxOut;
use jxl::jxl::enc_chroma_from_luma::{CfLHeuristics, find_best_multiplier};
use jxl::jxl::enc_gaborish::gaborish_inverse;
use jxl::jxl::enc_params::CompressParams;
use jxl::jxl::enc_xyb::{XybInputEncoding, to_xyb};
use jxl::jxl::frame_dimensions::{DCT_BLOCK_SIZE, FrameDimensions};
use jxl::jxl::image::{Image3F, ImageF, ImageI, ImageSB};
use jxl::jxl::image_ops::pad_image_to_block_multiple_in_place;
use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
use jxl::jxl::quant_weights::DequantMatrices;
use jxl::jxl::quantizer::Quantizer;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("eqh-")
}

type Args<'a, 'b> = &'a mut std::str::SplitWhitespace<'b>;

fn float(args: Args<'_, '_>) -> f32 {
    f32::from_bits(number(args))
}
fn rect(args: Args<'_, '_>) -> Rect {
    Rect::new(number(args), number(args), number(args), number(args))
}
fn read_plane(args: Args<'_, '_>, plane: &mut ImageF) {
    for y in 0..plane.ysize() {
        for x in 0..plane.xsize() {
            plane.row_mut(y)[x] = float(args);
        }
    }
}
fn read_image(args: Args<'_, '_>, xsize: usize, ysize: usize) -> Image3F {
    let mut image = Image3F::create(xsize, ysize).unwrap();
    for c in 0..3 {
        read_plane(args, image.plane_mut(c));
    }
    image
}
fn print_plane(out: &mut String, plane: &ImageF) {
    write!(out, " P {} {}", plane.xsize(), plane.ysize()).unwrap();
    for y in 0..plane.ysize() {
        for x in 0..plane.xsize() {
            write!(out, " {}", plane.row(y)[x].to_bits()).unwrap();
        }
    }
}
fn print_ints(out: &mut String, plane: &ImageI) {
    out.push_str(" I");
    for y in 0..plane.ysize() {
        for x in 0..plane.xsize() {
            write!(out, " {}", plane.row(y)[x]).unwrap();
        }
    }
}
fn print_map(out: &mut String, plane: &ImageSB) {
    out.push_str(" C");
    for y in 0..plane.ysize() {
        for x in 0..plane.xsize() {
            write!(out, " {}", plane.row(y)[x]).unwrap();
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
fn ints(xsize: usize, ysize: usize) -> ImageI {
    let mut plane = ImageI::create(xsize, ysize).unwrap();
    for y in 0..ysize {
        for x in 0..xsize {
            plane.row_mut(y)[x] = -7000 - (y * 64 + x) as i32;
        }
    }
    plane
}
fn read_strategy(args: Args<'_, '_>, xsize: usize, ysize: usize) -> AcStrategyImage {
    let mut acs = AcStrategyImage::create(xsize, ysize).unwrap();
    acs.fill_dct8_full();
    let n: usize = number(args);
    for _ in 0..n {
        let x: usize = number(args);
        let y: usize = number(args);
        let raw: u32 = number(args);
        acs.set(x, y, AcStrategyType(raw)).unwrap();
    }
    acs
}
fn read_map(args: Args<'_, '_>, map: &mut ImageSB) {
    for y in 0..map.ysize() {
        for x in 0..map.xsize() {
            map.row_mut(y)[x] = number::<i32>(args) as i8;
        }
    }
}
fn read_params(args: Args<'_, '_>) -> CompressParams<'static> {
    let butteraugli_distance = float(args);
    let tier: i32 = number(args);
    CompressParams {
        butteraugli_distance,
        speed_tier: speed_tier(tier),
        decoding_speed_tier: number(args),
        ..CompressParams::default()
    }
}
fn speed_tier(tier: i32) -> SpeedTier {
    match tier {
        -1 => SpeedTier::TectonicPlate,
        0 => SpeedTier::Glacier,
        1 => SpeedTier::Tortoise,
        2 => SpeedTier::Kitten,
        3 => SpeedTier::Squirrel,
        4 => SpeedTier::Wombat,
        5 => SpeedTier::Hare,
        6 => SpeedTier::Cheetah,
        7 => SpeedTier::Falcon,
        8 => SpeedTier::Thunder,
        _ => SpeedTier::Lightning,
    }
}
fn print_config(out: &mut String, h: &AcStrategyHeuristics<'_, '_>) {
    write!(
        out,
        " K {} {} {} {}",
        h.config.info_loss_multiplier.to_bits(),
        h.config.cost_delta.to_bits(),
        h.config.zeros_mul.to_bits(),
        h.config.mask1x1_xsize
    )
    .unwrap();
}
// The serial RunOnPool order of the 64x64 encoder tiles.
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
fn matrices() -> DequantMatrices {
    let mut matrices = DequantMatrices::default();
    let mut acs_mask = 0u32;
    for i in 0..AcStrategyType::DCT128X128.0 {
        acs_mask |= 1 << i;
    }
    matrices.ensure_computed(acs_mask).unwrap();
    matrices
}

pub(super) fn observe(kind: &str, args: Args<'_, '_>) -> String {
    let mut out = String::new();
    match kind {
        "eqh-gsq" | "eqh-sq" => {
            let dequant = DequantMatrices::default();
            let mut quantizer = Quantizer::new(&dequant);
            let quant_dc = float(args);
            let a = float(args);
            if kind == "eqh-gsq" {
                let absd = float(args);
                quantizer.compute_global_scale_and_quant(&dequant, quant_dc, a, absd);
                out.push('G');
                print_quantizer(&mut out, &quantizer);
            } else {
                let xsize: usize = number(args);
                let ysize: usize = number(args);
                let mut raw = ints(xsize, ysize);
                quantizer.set_quant(&dequant, quant_dc, a, &mut raw);
                out.push('U');
                print_quantizer(&mut out, &quantizer);
                print_ints(&mut out, &raw);
            }
        }
        "eqh-sqf" | "eqh-sqfr" => {
            let dequant = DequantMatrices::default();
            let mut quantizer = Quantizer::new(&dequant);
            let quant_dc = float(args);
            let (with_raw, xsize, ysize, r, median);
            if kind == "eqh-sqf" {
                with_raw = number::<u32>(args) != 0;
                xsize = number(args);
                ysize = number(args);
                r = Rect::new(0, 0, 0, 0);
                median = 0.0;
            } else {
                with_raw = true;
                median = float(args);
                xsize = number(args);
                ysize = number(args);
                r = rect(args);
            }
            let mut qf = ImageF::create(xsize, ysize).unwrap();
            read_plane(args, &mut qf);
            let mut raw = ints(xsize, ysize);
            if kind == "eqh-sqf" {
                let status = quantizer.set_quant_field(
                    &dequant,
                    quant_dc,
                    &qf,
                    if with_raw { Some(&mut raw) } else { None },
                );
                write!(out, "F {}", status_code(&status)).unwrap();
            } else {
                quantizer.compute_global_scale_and_quant(&dequant, quant_dc, median, 0.0);
                quantizer.set_quant_field_rect(&qf, r, &mut raw);
                out.push('R');
            }
            print_quantizer(&mut out, &quantizer);
            print_ints(&mut out, &raw);
        }
        "eqh-iqdc" => {
            write!(out, "D {}", initial_quant_dc(float(args)).to_bits()).unwrap();
        }
        "eqh-iqf" => {
            let distance = float(args);
            let rescale = float(args);
            let xsize: usize = number(args);
            let ysize: usize = number(args);
            let r = rect(args);
            let opsin = read_image(args, xsize, ysize);
            let mut mask = ImageF::default();
            let mut mask1x1 = ImageF::default();
            let qf =
                initial_quant_field(distance, &opsin, r, None, rescale, &mut mask, &mut mask1x1);
            let status = qf.as_ref().map(|_| ()).map_err(|e| *e);
            write!(out, "Q {}", status_code(&status)).unwrap();
            if let Ok(field) = qf {
                print_plane(&mut out, &field);
                print_plane(&mut out, &mask);
                print_plane(&mut out, &mask1x1);
            }
        }
        "eqh-adj" => {
            let distance = float(args);
            let xsize: usize = number(args);
            let ysize: usize = number(args);
            let r = rect(args);
            let acs = read_strategy(args, xsize, ysize);
            let mut qf = ImageF::create(xsize, ysize).unwrap();
            read_plane(args, &mut qf);
            let status = adjust_quant_field(&acs, r, distance, &mut qf);
            write!(out, "A {}", status_code(&status)).unwrap();
            print_plane(&mut out, &qf);
        }
        "eqh-mult" => {
            let num: usize = number(args);
            let base = float(args);
            let distance_mul = float(args);
            let fast = number::<u32>(args) != 0;
            let mut m = vec![0.0f32; num + 1];
            let mut s = vec![0.0f32; num + 1];
            for v in m.iter_mut().take(num) {
                *v = float(args);
            }
            for v in s.iter_mut().take(num) {
                *v = float(args);
            }
            let result = find_best_multiplier(&m, &s, num, base, distance_mul, fast);
            write!(out, "M {result}").unwrap();
        }
        "eqh-cfl" => {
            let mode: u32 = number(args);
            let fast = number::<u32>(args) != 0;
            let xsize: usize = number(args);
            let ysize: usize = number(args);
            let opsin = read_image(args, xsize, ysize);
            let r = Rect::new(0, 0, xsize, ysize);
            let (bxs, bys) = (xsize / 8, ysize / 8);
            let matrices = matrices();
            let mut acs = AcStrategyImage::default();
            let mut raw = ImageI::default();
            let mut quantizer = Quantizer::new(&matrices);
            if mode == 1 {
                acs = read_strategy(args, bxs, bys);
                raw = ints(bxs, bys);
                for y in 0..bys {
                    for x in 0..bxs {
                        raw.row_mut(y)[x] = number(args);
                    }
                }
                let quant_dc = float(args);
                let median = float(args);
                quantizer.compute_global_scale_and_quant(&matrices, quant_dc, median, 0.0);
            }
            let mut cmap = ColorCorrelationMap::create(xsize, ysize, true).unwrap();
            let mut cfl = CfLHeuristics::new(MemoryManagerPtr::NATIVE);
            let mut status = cfl.init(r);
            if status.is_ok() {
                status = cfl.prepare_for_threads(1);
            }
            for t in tiles(bxs, bys) {
                if status.is_err() {
                    break;
                }
                status = cfl.compute_tile(
                    t,
                    &opsin,
                    r,
                    &matrices,
                    (mode == 1).then_some(&acs),
                    (mode == 1).then_some(&raw),
                    (mode == 1).then_some(&quantizer),
                    fast,
                    0,
                    &mut cmap,
                );
            }
            write!(out, "L {}", status_code(&status)).unwrap();
            print_map(&mut out, &cmap.ytox_map);
            print_map(&mut out, &cmap.ytob_map);
            print_plane(&mut out, &cfl.dc_values);
        }
        "eqh-acs" | "eqh-entropy" | "eqh-best8" => {
            let cparams = read_params(args);
            let xsize: usize = number(args);
            let ysize: usize = number(args);
            let (bxs, bys) = (xsize / 8, ysize / 8);
            let opsin = read_image(args, xsize, ysize);
            let r = Rect::new(0, 0, xsize, ysize);
            let mut qf = ImageF::create(bxs, bys).unwrap();
            let mut mask = ImageF::create(bxs, bys).unwrap();
            let mut mask1x1 = ImageF::create(xsize, ysize).unwrap();
            read_plane(args, &mut qf);
            read_plane(args, &mut mask);
            read_plane(args, &mut mask1x1);
            let mut cmap = ColorCorrelationMap::create(xsize, ysize, true).unwrap();
            read_map(args, &mut cmap.ytox_map);
            read_map(args, &mut cmap.ytob_map);
            let mut acs = read_strategy(args, bxs, bys);
            let mut matrices = DequantMatrices::default();
            let mut heuristics = AcStrategyHeuristics::new(MemoryManagerPtr::NATIVE, &cparams);
            let mut status = heuristics.init(&opsin, r, &qf, &mask, &mask1x1, &mut matrices);
            if status.is_ok() {
                status = heuristics.prepare_for_threads(1);
            }
            write!(out, "H {}", status_code(&status)).unwrap();
            print_config(&mut out, &heuristics);
            if status.is_err() {
                return out;
            }
            let planes = AcsPlanes {
                src: &opsin,
                rect_in: r,
                quant_field: &qf,
                mask: &mask,
                mask1x1: &mask1x1,
                dequant: &matrices,
            };
            if kind == "eqh-acs" {
                for t in tiles(bxs, bys) {
                    if status.is_err() {
                        break;
                    }
                    status = heuristics.process_rect(t, &cmap, &mut acs, 0, &planes);
                }
                let mut aux = HeuristicsAuxOut::default();
                let frame_dim = FrameDimensions::default();
                if status.is_ok() {
                    status = heuristics.finalize(&frame_dim, &acs, Some(&mut aux));
                }
                write!(out, " {}", status_code(&status)).unwrap();
                print_strategy(&mut out, &acs);
                write!(
                    out,
                    " N {} {} {} {} {} {} {} {} {} {} {}",
                    aux.num_small_blocks,
                    aux.num_dct4x8_blocks,
                    aux.num_afv_blocks,
                    aux.num_dct8_blocks,
                    aux.num_dct8x16_blocks,
                    aux.num_dct8x32_blocks,
                    aux.num_dct16_blocks,
                    aux.num_dct16x32_blocks,
                    aux.num_dct32_blocks,
                    aux.num_dct32x64_blocks,
                    aux.num_dct64_blocks
                )
                .unwrap();
                return out;
            }
            let x: usize = number(args);
            let y: usize = number(args);
            let cmap_factors = [float(args), float(args), float(args)];
            let config = AcsConfig::new(&heuristics.config, &planes);
            let words = heuristics.mem_per_thread;
            let mut memory: Vec<f32> = (0..words)
                .map(|i| f32::from_bits(0x7fb00000u32 + (i % 4096) as u32))
                .collect();
            let mut quantized = vec![0u32; heuristics.qmem_per_thread];
            let (block, scratch) = memory.split_at_mut(3 * jxl::jxl::ac_strategy::MAX_COEFF_AREA);
            if kind == "eqh-entropy" {
                let raw: u32 = number(args);
                let entropy_mul = float(args);
                let strategy = AcStrategy::from_raw_strategy(AcStrategyType(raw));
                let mut entropy = -1.0f32;
                let status = estimate_entropy(
                    &strategy,
                    entropy_mul,
                    x,
                    y,
                    &config,
                    &cmap_factors,
                    block,
                    scratch,
                    &mut quantized,
                    &mut entropy,
                );
                write!(out, " E {} {}", status_code(&status), entropy.to_bits()).unwrap();
                let size = (1usize << strategy.log2_covered_blocks()) * DCT_BLOCK_SIZE;
                out.push_str(" B");
                for v in &block[..3 * size] {
                    write!(out, " {}", v.to_bits()).unwrap();
                }
                out.push_str(" W");
                for v in &scratch[..size] {
                    write!(out, " {}", v.to_bits()).unwrap();
                }
            } else {
                let mut entropy = -1.0f32;
                let mut best = AcStrategyType::DCT128X256;
                let status = find_best_8x8_transform(
                    x,
                    y,
                    cparams.speed_tier as i32,
                    cparams.butteraugli_distance,
                    &config,
                    &cmap_factors,
                    &mut acs,
                    block,
                    scratch,
                    &mut quantized,
                    &mut entropy,
                    &mut best,
                );
                write!(
                    out,
                    " T {} {} {}",
                    status_code(&status),
                    entropy.to_bits(),
                    best.0
                )
                .unwrap();
            }
        }
        "eqh-chain" => return chain(args),
        _ => panic!("unknown encoder heuristics request {kind}"),
    }
    out
}

// LossyFrameHeuristics' kSquirrel order for one serial thread, as the oracle
// sequences the original functions.
fn chain(args: Args<'_, '_>) -> String {
    let mut out = String::new();
    let distance = float(args);
    let xsize: usize = number(args);
    let ysize: usize = number(args);
    let pxs = xsize.div_ceil(8) * 8;
    let pys = ysize.div_ceil(8) * 8;
    let mut opsin = Image3F::create(pxs, pys).unwrap();
    opsin.shrink_to(xsize, ysize).unwrap();
    for c in 0..3 {
        read_plane(args, opsin.plane_mut(c));
    }
    let mut status = to_xyb(XybInputEncoding::Srgb, 255.0, None, None, &mut opsin, None);
    if status.is_ok() {
        status = pad_image_to_block_multiple_in_place(&mut opsin, 8);
    }
    write!(out, "X {}", status_code(&status)).unwrap();
    if status.is_err() {
        return out;
    }
    for c in 0..3 {
        print_plane(&mut out, opsin.plane(c));
    }
    let r = Rect::new(0, 0, opsin.xsize(), opsin.ysize());
    let (bxs, bys) = (pxs / 8, pys / 8);
    let cparams = CompressParams {
        butteraugli_distance: distance,
        ..CompressParams::default()
    };
    let mut matrices = DequantMatrices::default();
    let mut quantizer = Quantizer::new(&matrices);
    let mut raw_quant_field = ImageI::create(bxs, bys).unwrap();
    let mut cmap = ColorCorrelationMap::create(pxs, pys, true).unwrap();
    let mut ac_strategy = AcStrategyImage::create(bxs, bys).unwrap();
    let quant_dc = initial_quant_dc(cparams.butteraugli_distance);
    let mut cfl_heuristics = CfLHeuristics::new(MemoryManagerPtr::NATIVE);
    let mut initial_quant_masking = ImageF::default();
    let mut initial_quant_masking1x1 = ImageF::default();
    let iqf = initial_quant_field(
        cparams.butteraugli_distance,
        &opsin,
        r,
        None,
        1.0,
        &mut initial_quant_masking,
        &mut initial_quant_masking1x1,
    );
    let iqf_status = iqf.as_ref().map(|_| ()).map_err(|e| *e);
    write!(out, " {}", status_code(&iqf_status)).unwrap();
    let Ok(mut initial_quant_field) = iqf else {
        return out;
    };
    print_plane(&mut out, &initial_quant_field);
    print_plane(&mut out, &initial_quant_masking);
    print_plane(&mut out, &initial_quant_masking1x1);
    let q = (0.39 / f64::from(cparams.butteraugli_distance)) as f32;
    quantizer.compute_global_scale_and_quant(&matrices, quant_dc, q, 0.0);
    print_quantizer(&mut out, &quantizer);
    let weight = [1.0f32; 3];
    status = gaborish_inverse(&mut opsin, r, &weight, None);
    if status.is_ok() {
        status = cfl_heuristics.init(r);
    }
    let mut acs_heuristics = AcStrategyHeuristics::new(MemoryManagerPtr::NATIVE, &cparams);
    if status.is_ok() {
        status = acs_heuristics.init(
            &opsin,
            r,
            &initial_quant_field,
            &initial_quant_masking,
            &initial_quant_masking1x1,
            &mut matrices,
        );
    }
    if status.is_ok() {
        status = acs_heuristics.prepare_for_threads(1);
    }
    if status.is_ok() {
        status = cfl_heuristics.prepare_for_threads(1);
    }
    for t in tiles(bxs, bys) {
        if status.is_err() {
            break;
        }
        status = cfl_heuristics.compute_tile(
            t, &opsin, r, &matrices, None, None, None, false, 0, &mut cmap,
        );
        if status.is_ok() {
            let planes = AcsPlanes {
                src: &opsin,
                rect_in: r,
                quant_field: &initial_quant_field,
                mask: &initial_quant_masking,
                mask1x1: &initial_quant_masking1x1,
                dequant: &matrices,
            };
            status = acs_heuristics.process_rect(t, &cmap, &mut ac_strategy, 0, &planes);
        }
        if status.is_ok() {
            status = adjust_quant_field(
                &ac_strategy,
                t,
                cparams.butteraugli_distance,
                &mut initial_quant_field,
            );
        }
        if status.is_err() {
            break;
        }
        quantizer.set_quant_field_rect(&initial_quant_field, t, &mut raw_quant_field);
        status = cfl_heuristics.compute_tile(
            t,
            &opsin,
            r,
            &matrices,
            Some(&ac_strategy),
            Some(&raw_quant_field),
            Some(&quantizer),
            false,
            0,
            &mut cmap,
        );
    }
    write!(out, " T {}", status_code(&status)).unwrap();
    print_strategy(&mut out, &ac_strategy);
    print_plane(&mut out, &initial_quant_field);
    print_ints(&mut out, &raw_quant_field);
    print_map(&mut out, &cmap.ytox_map);
    print_map(&mut out, &cmap.ytob_map);
    print_plane(&mut out, &cfl_heuristics.dc_values);
    out
}

// ---------------------------------------------------------------------------
// Inputs.

fn bits(v: f32) -> u32 {
    v.to_bits()
}
fn push_floats(s: &mut String, values: &[f32]) {
    for v in values {
        write!(s, " {}", v.to_bits()).unwrap();
    }
}
fn unit(seed: &mut u64) -> f32 {
    (next(seed) >> 40) as f32 / (1u64 << 24) as f32
}

/// Step 1 corpus contents.
#[derive(Clone, Copy, Debug)]
enum Content {
    Flat,
    Gradient,
    Noise,
    Photo,
    SomeTestImage,
}
const CONTENTS: [Content; 5] = [
    Content::Flat,
    Content::Gradient,
    Content::Noise,
    Content::Photo,
    Content::SomeTestImage,
];

/// Step 1 corpus sizes: 1x1, 7x7, 8x8, 9x17, every `xsize mod 8` (41-48 x
/// 15), 63x129, 256x256, 257x255 and 512x1024.
const CORPUS_SIZES: [(usize, usize); 17] = [
    (1, 1),
    (7, 7),
    (8, 8),
    (9, 17),
    (41, 15),
    (42, 15),
    (43, 15),
    (44, 15),
    (45, 15),
    (46, 15),
    (47, 15),
    (48, 15),
    (63, 129),
    (256, 256),
    (257, 255),
    (512, 1024),
    (128, 72),
];

/// Nonlinear sRGB samples in [0, 1] for each of three planes. Gray images
/// repeat the first plane.
fn srgb_image(
    xsize: usize,
    ysize: usize,
    content: Content,
    gray: bool,
    seed: u64,
) -> [Vec<f32>; 3] {
    let n = xsize * ysize;
    let mut planes = [vec![0.0f32; n], vec![0.0f32; n], vec![0.0f32; n]];
    let mut s = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
    match content {
        Content::Flat => {
            let color = [unit(&mut s), unit(&mut s), unit(&mut s)];
            for c in 0..3 {
                planes[c].fill(color[c]);
            }
        }
        Content::Gradient => {
            for y in 0..ysize {
                for x in 0..xsize {
                    let fx = x as f32 / xsize.max(2) as f32;
                    let fy = y as f32 / ysize.max(2) as f32;
                    planes[0][y * xsize + x] = fx;
                    planes[1][y * xsize + x] = fy;
                    planes[2][y * xsize + x] = 0.5 * (fx + fy);
                }
            }
        }
        Content::Noise => {
            for plane in &mut planes {
                for v in plane.iter_mut() {
                    *v = unit(&mut s);
                }
            }
        }
        Content::Photo => {
            // Smooth sky, a textured ground, a sharp-edged disk and a fine
            // high-contrast patch, with mild sensor noise.
            let cx = xsize as f32 * 0.6;
            let cy = ysize as f32 * 0.4;
            let radius = (xsize.min(ysize) as f32 * 0.25).max(1.5);
            for y in 0..ysize {
                for x in 0..xsize {
                    let fx = x as f32 / xsize.max(2) as f32;
                    let fy = y as f32 / ysize.max(2) as f32;
                    let mut rgb = if fy < 0.45 {
                        [0.35 + 0.2 * fy, 0.55 + 0.25 * fy, 0.85 - 0.1 * fx]
                    } else {
                        let t = (x as f32 * 0.37).sin() * (y as f32 * 0.23).cos();
                        [0.32 + 0.1 * t, 0.42 + 0.12 * t, 0.18 + 0.05 * t]
                    };
                    let dx = x as f32 - cx;
                    let dy = y as f32 - cy;
                    if dx * dx + dy * dy < radius * radius {
                        rgb = [0.9, 0.75, 0.2];
                    }
                    if x % 23 < 6 && y % 19 < 5 && fy > 0.5 {
                        let checker = ((x + y) % 2) as f32;
                        rgb = [checker, checker, checker];
                    }
                    for c in 0..3 {
                        let noise = (unit(&mut s) - 0.5) * 0.02;
                        planes[c][y * xsize + x] = (rgb[c] + noise).clamp(0.0, 1.0);
                    }
                }
            }
        }
        Content::SomeTestImage => {
            let channels = if gray { 1 } else { 3 };
            let request = format!("eqh-some {xsize} {ysize} {channels} {}", seed % 4);
            let line = original_observations(&[request]).remove(0);
            let samples: Vec<u32> = line
                .split_whitespace()
                .skip(1)
                .map(|v| v.parse().unwrap())
                .collect();
            for i in 0..n {
                for (c, plane) in planes.iter_mut().enumerate() {
                    let channel = if gray { 0 } else { c };
                    plane[i] = samples[i * channels + channel] as f32 / 65535.0;
                }
            }
        }
    }
    if gray {
        let first = planes[0].clone();
        planes[1].clone_from(&first);
        planes[2] = first;
    }
    planes
}

/// The encoder's padded XYB image of a corpus image: ToXYB on the image, then
/// PadImageToBlockMultipleInPlace, as enc_frame.cc prepares the color planes.
fn corpus_opsin(xsize: usize, ysize: usize, content: Content, gray: bool, seed: u64) -> Image3F {
    let srgb = srgb_image(xsize, ysize, content, gray, seed);
    let mut opsin = Image3F::create(xsize.div_ceil(8) * 8, ysize.div_ceil(8) * 8).unwrap();
    opsin.shrink_to(xsize, ysize).unwrap();
    for (c, plane) in srgb.iter().enumerate() {
        for y in 0..ysize {
            opsin.plane_mut(c).row_mut(y)[..xsize].copy_from_slice(&plane[y * xsize..][..xsize]);
        }
    }
    to_xyb(XybInputEncoding::Srgb, 255.0, None, None, &mut opsin, None).unwrap();
    pad_image_to_block_multiple_in_place(&mut opsin, 8).unwrap();
    opsin
}
fn push_image(s: &mut String, image: &Image3F) {
    for c in 0..3 {
        push_plane(s, image.plane(c));
    }
}
fn push_plane(s: &mut String, plane: &ImageF) {
    for y in 0..plane.ysize() {
        push_floats(s, &plane.row(y)[..plane.xsize()]);
    }
}

// ---------------------------------------------------------------------------
// Quantizer field selection.

#[test]
fn encoder_compute_global_scale_and_quant_matches_clamps_and_rounding() {
    let quant_dcs = [
        0.0f32, 1e-4, 0.01, 0.1, 0.5, 0.9, 1.0, 1.0959, 1.6, 2.0, 2.5, 5.0, 10.0, 50.0, 100.0,
        1000.0,
    ];
    let medians = [
        -1.0f32, 0.0, 1e-5, 7.6e-5, 7.7e-5, 0.01, 0.1, 0.13, 0.39, 0.5, 0.765, 1.0, 1.53, 2.5, 2.6,
        3.0, 5.0, 10.0, 100.0,
    ];
    let mut cases = Vec::new();
    for &quant_dc in &quant_dcs {
        for &median in &medians {
            for absd in [0.0f32, 1e-3, 0.1, 0.5 * median, median] {
                cases.push(format!(
                    "eqh-gsq {} {} {}",
                    bits(quant_dc),
                    bits(median),
                    bits(absd)
                ));
            }
        }
    }
    let mut seed = 0x5eed;
    for _ in 0..2000 {
        let quant_dc = unit(&mut seed) * 60.0;
        let median = unit(&mut seed) * 4.0;
        let absd = unit(&mut seed) * median;
        cases.push(format!(
            "eqh-gsq {} {} {}",
            bits(quant_dc),
            bits(median),
            bits(absd)
        ));
    }
    for &quant_dc in &quant_dcs {
        for &quant_ac in &medians {
            for (xsize, ysize) in [(1, 1), (3, 2), (9, 4)] {
                cases.push(format!(
                    "eqh-sq {} {} {xsize} {ysize}",
                    bits(quant_dc),
                    bits(quant_ac)
                ));
            }
        }
    }
    compare(cases);
}

fn field_values(count: usize, kind: usize, seed: u64) -> Vec<f32> {
    let mut s = seed | 1;
    (0..count)
        .map(|i| match kind {
            // Realistic adjusted fields.
            0 => 0.2 + unit(&mut s) * 2.5,
            // Few distinct values: many ties for the selection.
            1 => [0.5f32, 0.75, 1.0][(next(&mut s) % 3) as usize],
            // All equal.
            2 => 0.6,
            // Signed zeros and small values around the median.
            3 => [0.0f32, -0.0, 0.25, -0.25][(next(&mut s) % 4) as usize],
            // Sorted and reverse-sorted runs.
            4 => i as f32 * 0.01 + 0.1,
            5 => (count - i) as f32 * 0.01 + 0.1,
            // Wide range and clamp boundaries of ClampVal.
            6 => [1e-6f32, 1e-3, 0.03, 0.5, 4.0, 40.0, 400.0][(next(&mut s) % 7) as usize],
            _ => (next(&mut s) % 1000) as f32 / 256.0,
        })
        .collect()
}

#[test]
fn encoder_set_quant_field_matches_median_selection_and_raw_fields() {
    let sizes = [
        (1, 1),
        (2, 1),
        (1, 3),
        (3, 3),
        (2, 4),
        (7, 5),
        (8, 8),
        (13, 11),
        (32, 32),
        (64, 128),
    ];
    let mut cases = Vec::new();
    for (i, &(xsize, ysize)) in sizes.iter().enumerate() {
        for kind in 0..8 {
            for quant_dc in [0.7f32, 1.0959, 2.25, 45.0] {
                for with_raw in [0, 1] {
                    let mut s = format!("eqh-sqf {} {with_raw} {xsize} {ysize}", bits(quant_dc));
                    push_floats(
                        &mut s,
                        &field_values(xsize * ysize, kind, (i * 8 + kind) as u64 + 77),
                    );
                    cases.push(s);
                }
            }
        }
    }
    // Every placement of a selected median among few elements.
    for len in 1..=9usize {
        for perm in 0..40u64 {
            let mut s = format!("eqh-sqf {} 1 {len} 1", bits(1.0));
            let mut seed = perm * 31 + len as u64;
            let values: Vec<f32> = (0..len)
                .map(|_| (next(&mut seed) % 5) as f32 * 0.25 + 0.25)
                .collect();
            push_floats(&mut s, &values);
            cases.push(s);
        }
    }
    for (i, &(xsize, ysize)) in sizes.iter().enumerate() {
        for kind in [0usize, 6, 7] {
            for (x0, y0) in [(0, 0), (xsize / 2, ysize / 3)] {
                let rect = [x0, y0, xsize - x0, ysize - y0];
                for median in [0.13f32, 0.39, 1.0] {
                    let mut s = format!(
                        "eqh-sqfr {} {} {xsize} {ysize} {} {} {} {}",
                        bits(1.0959),
                        bits(median),
                        rect[0],
                        rect[1],
                        rect[2],
                        rect[3]
                    );
                    push_floats(&mut s, &field_values(xsize * ysize, kind, i as u64 + 5));
                    cases.push(s);
                }
            }
        }
    }
    compare(cases);
}

// ---------------------------------------------------------------------------
// Adaptive quantization.

#[test]
fn encoder_initial_quant_dc_matches_distances() {
    let mut cases = Vec::new();
    for i in 1..=600 {
        let d = i as f32 * 0.05;
        cases.push(format!("eqh-iqdc {}", bits(d)));
    }
    for d in [
        1e-3f32,
        0.1,
        0.3,
        f32::from_bits(0x3e99_9999),
        f32::from_bits(0x3e99_999b),
        0.5,
        0.62,
        1.0,
        1.5,
        2.9,
        3.0,
        25.0,
    ] {
        cases.push(format!("eqh-iqdc {}", bits(d)));
        cases.push(format!("eqh-iqdc {}", bits(d * 0.62)));
    }
    compare(cases);
}

fn iqf_request(distance: f32, rescale: f32, opsin: &Image3F, rect: [usize; 4]) -> String {
    let mut s = format!(
        "eqh-iqf {} {} {} {} {} {} {} {}",
        bits(distance),
        bits(rescale),
        opsin.xsize(),
        opsin.ysize(),
        rect[0],
        rect[1],
        rect[2],
        rect[3]
    );
    push_image(&mut s, opsin);
    s
}

#[test]
fn encoder_initial_quant_field_matches_step1_corpus_and_distances() {
    let mut cases = Vec::new();
    for (i, &(xsize, ysize)) in CORPUS_SIZES.iter().enumerate() {
        for (j, &content) in CONTENTS.iter().enumerate() {
            for gray in [false, true] {
                let opsin = corpus_opsin(xsize, ysize, content, gray, (i * 10 + j) as u64);
                let full = [0, 0, opsin.xsize(), opsin.ysize()];
                let distances: &[f32] = if xsize * ysize > 70000 {
                    &[1.0]
                } else {
                    &[0.5, 1.0, 1.5, 2.9, 0.62]
                };
                for &d in distances {
                    cases.push(iqf_request(d, 1.0, &opsin, full));
                }
            }
        }
    }
    // Distances above the dampening ramp and the FuzzyErosion branch, and
    // the rescale argument.
    let opsin = corpus_opsin(63, 129, Content::Photo, false, 3);
    let full = [0, 0, opsin.xsize(), opsin.ysize()];
    for d in [0.1f32, 1.9999, 2.0, 2.0001, 4.5, 8.0, 14.0, 20.0] {
        cases.push(iqf_request(d, 1.0, &opsin, full));
    }
    for rescale in [0.5f32, 0.62, 2.0] {
        cases.push(iqf_request(1.0, rescale, &opsin, full));
    }
    compare(cases);
}

#[test]
fn encoder_initial_quant_field_matches_sub_rects_and_tile_borders() {
    // Rects away from the image edges select the source's +-2 and +-4 pixel
    // extensions of the 1x1 and pre-erosion passes.
    let mut cases = Vec::new();
    for (k, &(xsize, ysize)) in [(256usize, 256usize), (257, 255), (128, 72)]
        .iter()
        .enumerate()
    {
        let opsin = corpus_opsin(xsize, ysize, Content::Photo, false, 40 + k as u64);
        let (pxs, pys) = (opsin.xsize(), opsin.ysize());
        for rect in [
            [8, 8, pxs - 16, pys - 16],
            [64, 0, pxs - 64, pys],
            [0, 64.min(pys - 8), pxs, pys - 64.min(pys - 8)],
            [16, 24, 64, 40],
            [0, 0, 72, 64],
        ] {
            if rect[0] + rect[2] <= pxs && rect[1] + rect[3] <= pys && rect[2] > 0 && rect[3] > 0 {
                cases.push(iqf_request(1.0, 1.0, &opsin, rect));
            }
        }
    }
    compare(cases);
}

/// A random valid strategy layout: no transform leaves the image or its
/// 64x64 tile, as AcStrategyHeuristics guarantees.
fn placements(bxs: usize, bys: usize, seed: u64) -> Vec<(usize, usize, u32)> {
    let mut covered = vec![false; bxs * bys];
    let mut s = seed | 1;
    let mut result = Vec::new();
    for y in 0..bys {
        for x in 0..bxs {
            if covered[y * bxs + x] {
                continue;
            }
            let mut choice = 0u32;
            for _ in 0..6 {
                let raw = (next(&mut s) % 21) as u32;
                let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
                let (w, h) = (acs.covered_blocks_x(), acs.covered_blocks_y());
                let fits = x + w <= bxs
                    && y + h <= bys
                    && x / 8 == (x + w - 1) / 8
                    && y / 8 == (y + h - 1) / 8
                    && (0..h).all(|iy| (0..w).all(|ix| !covered[(y + iy) * bxs + x + ix]));
                if fits {
                    choice = raw;
                    break;
                }
            }
            let acs = AcStrategy::from_raw_strategy(AcStrategyType(choice));
            for iy in 0..acs.covered_blocks_y() {
                for ix in 0..acs.covered_blocks_x() {
                    covered[(y + iy) * bxs + x + ix] = true;
                }
            }
            if choice != 0 {
                result.push((x, y, choice));
            }
        }
    }
    result
}
fn push_placements(s: &mut String, placements: &[(usize, usize, u32)]) {
    write!(s, " {}", placements.len()).unwrap();
    for (x, y, raw) in placements {
        write!(s, " {x} {y} {raw}").unwrap();
    }
}

#[test]
fn encoder_adjust_quant_field_matches_layouts_rects_and_mixer_distances() {
    let mut cases = Vec::new();
    for (i, &(bxs, bys)) in [
        (1usize, 1usize),
        (2, 2),
        (4, 4),
        (8, 8),
        (9, 7),
        (16, 16),
        (21, 13),
        (32, 64),
    ]
    .iter()
    .enumerate()
    {
        for layout in 0..4u64 {
            let p = placements(bxs, bys, i as u64 * 97 + layout);
            for d in [0.5f32, 1.0, 1.54138, 1.5414, 1.6, 2.9, 3.31, 3.5, 10.0] {
                for (x0, y0) in [(0usize, 0usize), (bxs / 2, bys / 2)] {
                    let rect = [x0, y0, bxs - x0, bys - y0];
                    let mut s = format!(
                        "eqh-adj {} {bxs} {bys} {} {} {} {}",
                        bits(d),
                        rect[0],
                        rect[1],
                        rect[2],
                        rect[3]
                    );
                    push_placements(&mut s, &p);
                    push_floats(
                        &mut s,
                        &field_values(bxs * bys, (layout % 2) as usize * 7, layout + 3),
                    );
                    cases.push(s);
                }
            }
        }
    }
    compare(cases);
}

// ---------------------------------------------------------------------------
// Chroma from luma.

#[test]
fn encoder_find_best_multiplier_matches_newton_and_fast_searches() {
    let mut cases = Vec::new();
    for (k, &num) in [0usize, 1, 2, 5, 64, 640, 4096].iter().enumerate() {
        for base in [0.0f32, 1.0] {
            for distance_mul in [1e-9f32, 1e-3, 0.5] {
                for fast in [0, 1] {
                    for kind in 0..4u64 {
                        let mut seed = (k as u64) * 131 + kind * 7 + 1;
                        let mut m = Vec::with_capacity(num);
                        let mut s = Vec::with_capacity(num);
                        for _ in 0..num {
                            let luma = (unit(&mut seed) - 0.5)
                                * [2.0f32, 200.0, 0.02, 500.0][kind as usize];
                            let ratio = (unit(&mut seed) - 0.5) * 3.0;
                            m.push(luma);
                            s.push(luma * ratio / 84.0 + (unit(&mut seed) - 0.5) * 0.1);
                        }
                        let mut r = format!(
                            "eqh-mult {num} {} {} {fast}",
                            bits(base),
                            bits(distance_mul)
                        );
                        push_floats(&mut r, &m);
                        push_floats(&mut r, &s);
                        cases.push(r);
                    }
                }
            }
        }
    }
    compare(cases);
}

fn corpus_subset() -> Vec<((usize, usize), Content, bool, u64)> {
    let mut result = Vec::new();
    for (i, &size) in CORPUS_SIZES.iter().enumerate() {
        for (j, &content) in CONTENTS.iter().enumerate() {
            for gray in [false, true] {
                // The largest image runs with photographic and test content.
                if size.0 * size.1 > 70000
                    && !matches!(content, Content::Photo | Content::SomeTestImage)
                {
                    continue;
                }
                result.push((size, content, gray, (i * 10 + j) as u64));
            }
        }
    }
    result
}

#[test]
fn encoder_cfl_heuristics_match_dct8_and_strategy_passes() {
    let mut cases = Vec::new();
    for (index, ((xsize, ysize), content, gray, seed)) in corpus_subset().into_iter().enumerate() {
        let opsin = corpus_opsin(xsize, ysize, content, gray, seed);
        let (pxs, pys) = (opsin.xsize(), opsin.ysize());
        for fast in [0, 1] {
            let mut s = format!("eqh-cfl 0 {fast} {pxs} {pys}");
            push_image(&mut s, &opsin);
            cases.push(s);
        }
        let (bxs, bys) = (pxs / 8, pys / 8);
        let p = placements(bxs, bys, seed + 1000);
        let mut raw_seed = seed + 17;
        for fast in [0, 1] {
            let mut s = format!("eqh-cfl 1 {fast} {pxs} {pys}");
            push_image(&mut s, &opsin);
            push_placements(&mut s, &p);
            for _ in 0..bxs * bys {
                write!(s, " {}", 1 + next(&mut raw_seed) % 40).unwrap();
            }
            let quant_dc = [1.0959f32, 2.5, 0.7][index % 3];
            let median = [0.39f32, 0.26, 0.78][index % 3];
            push_floats(&mut s, &[quant_dc, median]);
            cases.push(s);
        }
    }
    compare(cases);
}

// ---------------------------------------------------------------------------
// AC strategy.

/// The ACS inputs: InitialQuantField's field and masks of the opsin image
/// (compared separately above), its Gaborish inverse, and a DCT8 CfL map.
struct AcsInputs {
    opsin: Image3F,
    qf: ImageF,
    mask: ImageF,
    mask1x1: ImageF,
}
fn acs_inputs(opsin: &Image3F, distance: f32) -> AcsInputs {
    let r = Rect::new(0, 0, opsin.xsize(), opsin.ysize());
    let mut mask = ImageF::default();
    let mut mask1x1 = ImageF::default();
    let qf = initial_quant_field(distance, opsin, r, None, 1.0, &mut mask, &mut mask1x1).unwrap();
    let mut gab = Image3F::create(opsin.xsize(), opsin.ysize()).unwrap();
    for c in 0..3 {
        for y in 0..opsin.ysize() {
            gab.plane_mut(c).row_mut(y)[..opsin.xsize()]
                .copy_from_slice(&opsin.plane(c).row(y)[..opsin.xsize()]);
        }
    }
    gaborish_inverse(&mut gab, r, &[1.0; 3], None).unwrap();
    AcsInputs {
        opsin: gab,
        qf,
        mask,
        mask1x1,
    }
}
fn acs_prefix(command: &str, cparams: (f32, i32, usize), inputs: &AcsInputs, seed: u64) -> String {
    let (pxs, pys) = (inputs.opsin.xsize(), inputs.opsin.ysize());
    let mut s = format!(
        "{command} {} {} {} {pxs} {pys}",
        bits(cparams.0),
        cparams.1,
        cparams.2
    );
    push_image(&mut s, &inputs.opsin);
    push_plane(&mut s, &inputs.qf);
    push_plane(&mut s, &inputs.mask);
    push_plane(&mut s, &inputs.mask1x1);
    let tiles = pxs.div_ceil(64) * pys.div_ceil(64);
    let mut map_seed = seed | 1;
    for _ in 0..2 * tiles {
        write!(s, " {}", (next(&mut map_seed) % 41) as i32 - 20).unwrap();
    }
    s
}

#[test]
fn encoder_estimate_entropy_and_best_8x8_match_every_strategy() {
    let mut cases = Vec::new();
    for (k, &(xsize, ysize, content)) in [
        (64usize, 64usize, Content::Photo),
        (63, 129, Content::SomeTestImage),
        (48, 15, Content::Noise),
        (128, 72, Content::Gradient),
    ]
    .iter()
    .enumerate()
    {
        let opsin = corpus_opsin(xsize, ysize, content, false, 60 + k as u64);
        for d in [0.5f32, 1.0, 2.9, 4.5, 13.0] {
            let inputs = acs_inputs(&opsin, d.min(3.0));
            let (bxs, bys) = (inputs.opsin.xsize() / 8, inputs.opsin.ysize() / 8);
            for raw in 0..21u32 {
                let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
                let (w, h) = (acs.covered_blocks_x(), acs.covered_blocks_y());
                if w > bxs || h > bys {
                    continue;
                }
                for (bx, by) in [(0, 0), (bxs - w, bys - h)] {
                    let mut s = acs_prefix("eqh-entropy", (d, 3, 0), &inputs, raw as u64);
                    push_placements(&mut s, &[]);
                    write!(s, " {} {}", bx * 8, by * 8).unwrap();
                    push_floats(&mut s, &[0.031, 0.0, 0.97]);
                    write!(s, " {raw}").unwrap();
                    push_floats(&mut s, &[[1.0f32, 0.8, 1.21][raw as usize % 3]]);
                    cases.push(s);
                }
            }
            for tier in [0i32, 3, 4, 5, 9] {
                for (bx, by) in [(0, 0), (bxs / 2, bys / 2), (bxs - 1, bys - 1)] {
                    let mut s = acs_prefix("eqh-best8", (d, tier, 0), &inputs, 5);
                    push_placements(&mut s, &[]);
                    write!(s, " {} {}", bx * 8, by * 8).unwrap();
                    push_floats(&mut s, &[-0.02, 0.0, 1.1]);
                    cases.push(s);
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn encoder_ac_strategy_heuristics_match_step1_corpus() {
    let mut cases = Vec::new();
    for ((xsize, ysize), content, gray, seed) in corpus_subset() {
        let opsin = corpus_opsin(xsize, ysize, content, gray, seed);
        let distances: &[f32] = if xsize * ysize > 70000 {
            &[1.0]
        } else {
            &[0.5, 1.0, 1.5, 2.9]
        };
        for &d in distances {
            let inputs = acs_inputs(&opsin, d);
            let mut s = acs_prefix("eqh-acs", (d, 3, 0), &inputs, seed);
            push_placements(&mut s, &[]);
            cases.push(s);
        }
    }
    compare(cases);
}

#[test]
fn encoder_ac_strategy_heuristics_match_speed_and_decoding_tiers() {
    let mut cases = Vec::new();
    for (k, &(xsize, ysize, content)) in [
        (256usize, 256usize, Content::Photo),
        (63, 129, Content::SomeTestImage),
        (128, 72, Content::Photo),
    ]
    .iter()
    .enumerate()
    {
        let opsin = corpus_opsin(xsize, ysize, content, false, 80 + k as u64);
        for d in [1.0f32, 4.5, 13.0] {
            let inputs = acs_inputs(&opsin, d.min(3.0));
            for tier in [0i32, 1, 2, 3, 4, 5, 6] {
                for decoding in [0usize, 1, 2, 4] {
                    if tier != 3 && decoding != 0 && d != 1.0 {
                        continue;
                    }
                    let mut s = acs_prefix("eqh-acs", (d, tier, decoding), &inputs, k as u64);
                    push_placements(&mut s, &[]);
                    cases.push(s);
                }
            }
        }
    }
    compare(cases);
}

// ---------------------------------------------------------------------------
// The kSquirrel checkpoint chain.

fn chain_request(
    xsize: usize,
    ysize: usize,
    content: Content,
    gray: bool,
    seed: u64,
    d: f32,
) -> String {
    let srgb = srgb_image(xsize, ysize, content, gray, seed);
    let mut s = format!("eqh-chain {} {xsize} {ysize}", bits(d));
    for plane in &srgb {
        push_floats(&mut s, plane);
    }
    s
}

#[test]
fn encoder_heuristics_chain_matches_step1_corpus_at_squirrel() {
    let mut cases = Vec::new();
    for ((xsize, ysize), content, gray, seed) in corpus_subset() {
        let distances: &[f32] = if xsize * ysize > 70000 {
            &[1.0]
        } else {
            &[0.5, 1.0, 1.5, 2.9]
        };
        for &d in distances {
            cases.push(chain_request(xsize, ysize, content, gray, seed, d));
        }
    }
    compare(cases);
}

/// FindBest8x8Transform's `entropy_mul -= kFavor2X2AtHighQuality * weight`
/// for DCT2X2 and IDENTITY, at distances where its fused and separate
/// roundings differ, on content where those transforms win. Its other
/// adjustment, `kAvoidEntropyOfTransforms * mul`, multiplies by 0.5 exactly.
#[test]
fn encoder_best_8x8_matches_favored_small_transform_multipliers() {
    let differs = |d: f32| {
        let favor = 0.4f64 as f32;
        let base = (5.0f32 - d) / 5.0f32;
        let weight = base * base;
        [0.95f64, 1.0427542510634957].iter().any(|&m| {
            let em = (m / 0.8) as f32;
            (-favor).mul_add(weight, em) != -favor * weight + em
        })
    };
    let distances: Vec<f32> = (1..490)
        .map(|i| i as f32 * 0.01)
        .filter(|&d| differs(d))
        .take(6)
        .collect();
    assert_eq!(distances.len(), 6);
    let mut cases = Vec::new();
    for (k, content) in [Content::Noise, Content::Photo].into_iter().enumerate() {
        let opsin = corpus_opsin(48, 40, content, false, 90 + k as u64);
        for &d in &distances {
            let inputs = acs_inputs(&opsin, d.min(3.0));
            let (bxs, bys) = (inputs.opsin.xsize() / 8, inputs.opsin.ysize() / 8);
            for by in 0..bys {
                for bx in 0..bxs {
                    let mut s = acs_prefix("eqh-best8", (d, 3, 0), &inputs, 7);
                    push_placements(&mut s, &[]);
                    write!(s, " {} {}", bx * 8, by * 8).unwrap();
                    push_floats(&mut s, &[0.0, 0.0, 1.0]);
                    cases.push(s);
                }
            }
        }
    }
    compare(cases);
}
