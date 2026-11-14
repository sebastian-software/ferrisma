// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyImage, AcStrategyType};
use jxl::jxl::chroma_from_luma::ColorCorrelationMap;
use jxl::jxl::dct_util::{AcImage, AcPtrMut, AcType, ConstAcPtr};
use jxl::jxl::dec_cache::GroupDecCache;
use jxl::jxl::dec_group::{DrawMode, GetBlock};
use jxl::jxl::dec_xyb::OpsinParams;
use jxl::jxl::frame_header::FrameHeader;
use jxl::jxl::image::{Image3, Plane};
use jxl::jxl::passes_state::PassesSharedState;
use jxl::jxl::render_pipeline::render_pipeline_stage::WriteToImage3FStage;
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
pub(super) fn handles(kind: &str) -> bool {
    matches!(kind, "dq-group" | "dq-jpeg")
}
struct FixtureGetBlock {
    calls: usize,
    rows: Vec<usize>,
    fail: i32,
    jpeg_seed: Option<i32>,
}
impl GetBlock for FixtureGetBlock {
    fn start_row(&mut self, by: usize) {
        self.rows.push(by);
    }
    fn load_block(
        &mut self,
        bx: usize,
        by: usize,
        _acs: AcStrategy,
        size: usize,
        _log: usize,
        block: &mut [AcPtrMut<'_>; 3],
    ) -> Status {
        for (c, block) in block.iter_mut().enumerate() {
            let k = (self.calls * 13 + c * 7 + 1) % size;
            let v = ((bx * 11 + by * 19 + c * 23 + self.calls * 5) % 193) as i32 - 96;
            match block {
                AcPtrMut::I16(b) => b[k] += v as i16,
                AcPtrMut::I32(b) => b[k] += v,
            }
        }
        if let Some(seed) = self.jpeg_seed {
            for (c, block) in block.iter_mut().enumerate() {
                let AcPtrMut::I32(values) = block else {
                    unreachable!()
                };
                for (k, value) in values.iter_mut().take(size).enumerate() {
                    *value = seed + ((k * 17 + c * 29 + bx * 7 + by * 13) % 257) as i32 - 128;
                }
            }
        }
        let valid = self.calls as i32 != self.fail;
        self.calls += 1;
        if valid {
            Ok(())
        } else {
            Err(StatusCode::GenericError)
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let raw = number(args);
    let ty: u32 = number(args);
    let mode: u32 = number(args);
    let accumulate = number::<u32>(args) != 0;
    let fail = number(args);
    let jpeg_output = kind == "dq-jpeg";
    let mut config = [0i32; 8];
    if jpeg_output {
        for v in &mut config {
            *v = number(args);
        }
    }
    let [transform, sampling, gray, cflx, cflb, qmode, seed, dcmode] = config;
    let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
    let (cx, cy) = (acs.covered_blocks_x(), acs.covered_blocks_y());
    let gx = if mode & 1 != 0 { 32 } else { 0 };
    let nx = if cx <= 4 { 2 } else { 1 };
    let ny = if cy <= 4 { 2 } else { 1 };
    let (w, h) = ((gx + nx * cx) * 8, ny * cy * 8);
    let group_idx = usize::from(gx != 0);
    let mut header = FrameHeader::new(None);
    if jpeg_output {
        header.color_transform = jxl::jxl::frame_header::ColorTransform(transform as u32);
        header
            .chroma_subsampling
            .set(
                &[if sampling & 1 != 0 { 2 } else { 1 }, 1, 1],
                &[if sampling & 2 != 0 { 2 } else { 1 }, 1, 1],
            )
            .unwrap();
    }
    let mut shared = PassesSharedState::new();
    shared.frame_dim.set(w, h, 1, 0, 0, false, 1);
    let (bw, bh) = (shared.frame_dim.xsize_blocks, shared.frame_dim.ysize_blocks);
    shared.ac_strategy = AcStrategyImage::create(bw, bh).unwrap();
    shared.ac_strategy.fill_dct8_full();
    for y in (0..ny * cy).step_by(cy) {
        for x in (gx..gx + nx * cx).step_by(cx) {
            shared.ac_strategy.set(x, y, acs.strategy()).unwrap();
        }
    }
    shared.raw_quant_field = Plane::create(bw, bh).unwrap();
    for y in 0..bh {
        for x in 0..bw {
            shared.raw_quant_field.row_mut(y)[x] = (1 + (x * 13 + y * 7) % 256) as i32;
        }
    }
    shared.dc_storage = Image3::create(bw, bh).unwrap();
    for c in 0..3 {
        for y in 0..bh {
            for x in 0..bw {
                shared.dc_storage.plane_row_mut(c, y)[x] =
                    ((c * 19 + x * 7 + y * 13) % 97) as f32 / 64.0 - 48.0 / 64.0;
            }
        }
    }
    shared.cmap = ColorCorrelationMap::create(w, h, true).unwrap();
    for y in 0..shared.cmap.ytox_map.ysize() {
        for x in 0..shared.cmap.ytox_map.xsize() {
            shared.cmap.ytox_map.row_mut(y)[x] = ((x * 13 + y * 7) % 71) as i8 - 35;
            shared.cmap.ytob_map.row_mut(y)[x] = ((x * 3 + y * 17) % 73) as i8 - 36;
        }
    }
    let mut jpeg = jxl::jxl::jpeg::jpeg_data::JpegData::default();
    if jpeg_output {
        shared.cmap = ColorCorrelationMap::create(w, h, false).unwrap();
        for y in 0..shared.cmap.ytox_map.ysize() {
            shared.cmap.ytox_map.row_mut(y).fill(cflx as i8);
            shared.cmap.ytob_map.row_mut(y).fill(cflb as i8);
        }
        let mut qt: Vec<_> = (0..192).map(|i| 1 + ((i * 31 + 7) % 255)).collect();
        match qmode {
            1 => qt[65] = 0,
            2 => qt[65] = 65536,
            3 => qt[130] = -1,
            7 => qt[0] = 0,
            8 => qt[0] = i32::MIN,
            9 => qt.truncate(191),
            _ => (),
        }
        let mut encodings = shared.matrices.encodings().clone();
        encodings[0] = jxl::jxl::quant_weights::QuantEncoding::raw(&qt, i32::from(qmode == 4));
        if qmode == 6 {
            encodings[0] = jxl::jxl::quant_weights::QuantEncoding::library::<0>();
        }
        shared.matrices.set_encodings(&encodings);
        if qmode == 5 {
            shared.cmap = ColorCorrelationMap::create(w, h, true).unwrap();
        }
        jpeg.components
            .resize_with(if gray != 0 { 1 } else { 3 }, Default::default);
        let map = jxl::jxl::frame_header::jpeg_order(header.color_transform, gray != 0);
        for (c, &index) in map.iter().enumerate() {
            if gray != 0 && c != 1 {
                continue;
            }
            let component = &mut jpeg.components[index as usize];
            component.width_in_blocks = (bw >> header.chroma_subsampling.h_shift(c)) as u32;
            component.height_in_blocks = (bh >> header.chroma_subsampling.v_shift(c)) as u32;
            component.coeffs =
                vec![
                    1234;
                    component.width_in_blocks as usize * component.height_in_blocks as usize * 64
                ];
        }
        for c in 0..3 {
            for y in 0..bh {
                for x in 0..bw {
                    shared.dc_storage.plane_row_mut(c, y)[x] = match dcmode {
                        0 => ((c * 19 + x * 7 + y * 13) % 97) as f32 - 48.0,
                        1 => 4096.75,
                        2 => -4096.75,
                        4 => f32::from_bits(0x7fc12345),
                        5 => f32::from_bits(0x7fa12345),
                        6 => f32::INFINITY,
                        7 => f32::NEG_INFINITY,
                        _ => 0.75,
                    };
                }
            }
        }
    } else {
        shared.matrices.ensure_computed(1 << raw).unwrap();
    }
    let mut opsin = OpsinParams::default();
    opsin.init(255.0);
    let kind = if ty == 0 { AcType::I16 } else { AcType::I32 };
    let count = nx * ny * cx * cy * 64;
    let mut coefficients = if accumulate {
        *AcImage::make(kind, count, shared.frame_dim.num_groups).unwrap()
    } else {
        AcImage::empty(kind)
    };
    if accumulate {
        for c in 0..3 {
            for g in 0..shared.frame_dim.num_groups {
                let mut ptr = coefficients.plane_row_mut(c, g, 0);
                for k in 0..count {
                    let v = ((k * 7 + c * 13 + g * 5) % 29) as i32 - 14;
                    match &mut ptr {
                        AcPtrMut::I16(vs) => vs[k] = v as i16,
                        AcPtrMut::I32(vs) => vs[k] = v,
                    }
                }
            }
        }
    }
    let mut cache = GroupDecCache::default();
    cache.init_once(1, 1 << raw).unwrap();
    let (buffers, _) = cache.split_buffers();
    buffers.block.fill(-123.25);
    buffers.scratch.fill(-123.25);
    buffers.qblock.fill(1234);
    buffers.qblock16.fill(1234);
    let mut output = Image3::default();
    let mut builder = SimpleRenderPipelineBuilder::new(3);
    builder
        .add_stage(Some(Box::new(WriteToImage3FStage::new(&mut output))))
        .unwrap();
    let mut pipeline = builder.finalize(shared.frame_dim).unwrap();
    pipeline.prepare_for_threads(1, false).unwrap();
    let mut input = pipeline.get_input_buffers(group_idx, 0);
    for c in 0..3 {
        let (plane, _) = input.get_buffer_mut(c);
        for y in 0..plane.ysize() {
            plane.row_mut(y).fill(-123.25);
        }
    }
    let mut get = FixtureGetBlock {
        calls: 0,
        rows: Vec::new(),
        fail,
        jpeg_seed: jpeg_output.then_some(seed),
    };
    let draw = if mode & 2 != 0 {
        DrawMode::DontDraw
    } else {
        DrawMode::Draw
    };
    let status = jxl::jxl::dec_group_render::decode_group_impl(
        &header,
        &mut get,
        buffers,
        &shared,
        &mut coefficients,
        0.8,
        1.25,
        &opsin.quant_biases,
        group_idx,
        &mut input,
        if jpeg_output { Some(&mut jpeg) } else { None },
        draw,
    );
    let mut out = format!("{} {} R", status_code(&status), get.calls);
    for y in get.rows {
        write!(out, " {y}").unwrap();
    }
    out.push_str(" C");
    if accumulate {
        for c in 0..3 {
            match coefficients.plane_row(c, group_idx, 0) {
                ConstAcPtr::I16(v) => {
                    for v in &v[..count] {
                        write!(out, " {v}").unwrap();
                    }
                }
                ConstAcPtr::I32(v) => {
                    for v in &v[..count] {
                        write!(out, " {v}").unwrap();
                    }
                }
            }
        }
    }
    out.push_str(" P");
    for c in 0..3 {
        let (plane, r) = input.get_buffer(c);
        for y in 0..r.ysize() {
            for x in 0..r.xsize() {
                write!(out, " {}", plane.row(r.y0() + y)[r.x0() + x].to_bits()).unwrap();
            }
        }
    }
    if jpeg_output {
        out.push_str(" J");
        for component in &jpeg.components {
            write!(out, " {}", component.coeffs.len()).unwrap();
            for v in &component.coeffs {
                write!(out, " {v}").unwrap();
            }
        }
        out.push_str(" B");
        let (buffers, _) = cache.split_buffers();
        for v in &buffers.qblock[..cx * cy * 64 * 3] {
            write!(out, " {v}").unwrap();
        }
    }
    out
}
#[test]
fn group_pixels_match_original_all_strategies_widths_and_absolute_offsets() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    for raw in 0..27 {
        let requests = (0..2)
            .flat_map(|ty| (0..2).map(move |mode| format!("dq-group {raw} {ty} {mode} 1 -1")))
            .collect();
        compare(requests);
    }
}
#[test]
fn group_pixels_keep_cache_zeroing_draw_modes_and_partial_load_failures() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    for raw in [0, 4, 7, 14, 18, 24] {
        for ty in 0..2 {
            let mut requests = Vec::new();
            for mode in 0..4 {
                for accumulate in 0..2 {
                    for fail in [-1, 0, 1] {
                        requests.push(format!("dq-group {raw} {ty} {mode} {accumulate} {fail}"));
                    }
                }
            }
            for chunk in requests.chunks(4) {
                compare(chunk.to_vec());
            }
        }
    }
}

#[test]
fn jpeg_groups_keep_transpose_cfl_sampling_dc_and_failure_order() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut requests = Vec::new();
    for transform in [1, 2] {
        for sampling in 0..4 {
            for gray in 0..2 {
                for accumulate in 0..2 {
                    for (cx, cb) in [(0, 0), (-128, 127), (1, -1), (84, -73)] {
                        for seed in [0, 3968, -3967, 32768, -32768] {
                            requests.push(format!("dq-jpeg 0 1 0 {accumulate} -1 {transform} {sampling} {gray} {cx} {cb} 0 {seed} 0"));
                        }
                    }
                }
            }
        }
    }
    for raw in [0, 4, 7, 14] {
        for mode in 0..4 {
            for accumulate in 0..2 {
                for fail in [-1, 0, 1] {
                    for qmode in 0..7 {
                        for dcmode in 0..4 {
                            requests.push(format!("dq-jpeg {raw} 1 {mode} {accumulate} {fail} 2 0 0 4 -9 {qmode} 0 {dcmode}"));
                        }
                    }
                }
            }
        }
    }
    for transform in [1, 2] {
        for qmode in [8, 9] {
            for dcmode in [0, 6, 7] {
                requests.push(format!(
                    "dq-jpeg 0 1 0 1 -1 {transform} 0 0 4 -9 {qmode} 0 {dcmode}"
                ));
            }
        }
        for dcmode in [6, 7] {
            requests.push(format!(
                "dq-jpeg 0 1 0 1 -1 {transform} 0 0 4 -9 0 0 {dcmode}"
            ));
        }
    }
    for chunk in requests.chunks(16) {
        super::jpeg_reconstruction::compare_jpeg(chunk.to_vec());
    }
}
