// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::base::rect::Rect;
use jxl::jxl::dec_modular::{ModularFrameDecoder, ModularStreamId, ModularStreamKind};
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::image::Image3;
use jxl::jxl::passes_state::PassesSharedState;
use jxl::jxl::render_pipeline::render_pipeline_stage::WriteToImage3FStage;
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("mg-")
}
pub(super) fn output(out: &mut String, image: &Image3<f32>) {
    write!(out, " O {} {}", image.xsize(), image.ysize()).unwrap();
    for c in 0..3 {
        for y in 0..image.ysize() {
            for x in 0..image.xsize() {
                write!(out, " {}", image.plane_row(c, y)[x].to_bits()).unwrap();
            }
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "mg-pixels" {
        return super::modular_pixels::observe(args);
    }
    assert_eq!(kind, "mg-decode");
    let w = number(args);
    let h = number(args);
    let gray = number::<u32>(args) != 0;
    let depth = number::<u32>(args);
    let drop = number::<u32>(args) != 0;
    let inplace = number::<u32>(args) != 0;
    let repeats = number::<usize>(args);
    let zero = number::<u32>(args) != 0;
    let has_state = number::<u32>(args) != 0;
    let has_input = number::<u32>(args) != 0;
    let stream_kind = number::<u32>(args);
    let n = number::<usize>(args);
    let bytes: Vec<_> = (0..n).map(|_| unhex(args.next().unwrap())).collect();
    let mut readers: Vec<_> = bytes.iter().map(|b| BitReader::new(b)).collect();
    let index = |id: usize| if n == 1 { 0 } else { id };
    let config = LosslessThunderConfig::new(
        w,
        h,
        gray,
        if depth == 8 {
            LosslessSampleDepth::Uint8
        } else {
            LosslessSampleDepth::Uint16
        },
    )
    .unwrap();
    let frame = config.create_frame().unwrap();
    let header = frame.header();
    let dim = header.to_frame_dimensions();
    let mut shared = PassesSharedState::new();
    shared.metadata = Some(config.metadata());
    shared.frame_dim = dim;
    let mut decoder = ModularFrameDecoder::new();
    decoder.init(&dim);
    let mut image = Image3::<f32>::default();
    let mut out = String::new();
    let status = (|| -> Status {
        let q = shared.matrices.decode_dc(&mut readers[0]);
        write!(out, " Q {}", status_code(&q)).unwrap();
        q?;
        let global = decoder.decode_global_info(&mut readers[0], header, false);
        write!(out, " G {}", status_code(&global)).unwrap();
        global?;
        for g in 0..dim.num_dc_groups {
            let r = Rect::new(
                (g % dim.xsize_dc_groups) * dim.dc_group_dim,
                (g / dim.xsize_dc_groups) * dim.dc_group_dim,
                dim.dc_group_dim,
                dim.dc_group_dim,
            );
            let status = decoder.decode_group(
                header,
                r,
                Some(&mut readers[index(1 + g)]),
                3,
                1000,
                &ModularStreamId::modular_dc(g),
                false,
                None,
                None,
                false,
                None,
            );
            write!(out, " D {}", status_code(&status)).unwrap();
            status?;
        }
        if drop {
            decoder.maybe_drop_full_image();
        }
        let mut builder = SimpleRenderPipelineBuilder::new(3);
        builder
            .add_stage(Some(Box::new(WriteToImage3FStage::new(&mut image))))
            .unwrap();
        let mut pipe = builder.finalize(dim).unwrap();
        let inner = (|| -> Status {
            let prep = pipe.prepare_for_threads(1, false);
            write!(out, " P {}", status_code(&prep)).unwrap();
            prep?;
            for g in 0..dim.num_groups {
                let mut input = pipe.get_input_buffers(g, 0);
                let mut ready = true;
                let r = Rect::new(
                    (g % dim.xsize_groups) * dim.group_dim,
                    (g / dim.xsize_groups) * dim.group_dim,
                    dim.group_dim,
                    dim.group_dim,
                );
                let (min, max) = header.passes.get_downsampling_bracket(0);
                let mut stream = ModularStreamId::modular_ac(g, 0);
                stream.kind = ModularStreamKind(stream_kind as i32);
                let status = decoder.decode_group(
                    header,
                    r,
                    if zero {
                        None
                    } else {
                        Some(
                            &mut readers[index(jxl::jxl::toc::ac_group_index(
                                0,
                                g,
                                dim.num_groups,
                                dim.num_dc_groups,
                            ))],
                        )
                    },
                    min,
                    max,
                    &stream,
                    zero,
                    has_state.then_some(&shared),
                    if has_input { Some(&mut input) } else { None },
                    false,
                    Some(&mut ready),
                );
                write!(out, " A {} {}", status_code(&status), u32::from(ready)).unwrap();
                status?;
                if !decoder.uses_full_image() && ready && has_input {
                    let status = input.done();
                    write!(out, " R {}", status_code(&status)).unwrap();
                    status?;
                }
            }
            for _ in 0..repeats {
                let status = decoder.finalize_decoding(header, &shared, &mut *pipe, inplace);
                write!(out, " F {}", status_code(&status)).unwrap();
                status?;
            }
            Ok(())
        })();
        render_pipeline::pipeline(&mut out, &pipe);
        inner
    })();
    write!(out, " S {}", status_code(&status)).unwrap();
    modular_frame_decoder::decoder(&mut out, &decoder, false);
    output(&mut out, &image);
    write!(out, " B {}", readers.len()).unwrap();
    for r in &mut readers {
        let bits = r.total_bits_consumed();
        let closed = r.close();
        write!(out, " {bits},{}", status_code(&closed)).unwrap();
    }
    out
}
fn parts(output: &str) -> Vec<Vec<u8>> {
    let mut f = output.split_whitespace();
    let _shift: usize = f.next().unwrap().parse().unwrap();
    let n: usize = f.next().unwrap().parse().unwrap();
    (0..n).map(|_| unhex(f.next().unwrap())).collect()
}
#[expect(
    clippy::too_many_arguments,
    reason = "Keep source group fixture controls explicit."
)]
fn request(
    w: usize,
    h: usize,
    gray: u32,
    depth: u32,
    drop: u32,
    inplace: u32,
    zero: u32,
    context: u32,
    input: u32,
    kind: u32,
    data: &[Vec<u8>],
) -> String {
    let mut r = format!(
        "mg-decode {w} {h} {gray} {depth} {drop} {inplace} {} {zero} {context} {input} {kind} {}",
        if inplace == 1 { 1 } else { 2 },
        data.len()
    );
    for part in data {
        write!(r, " {}", hex(part)).unwrap();
    }
    r
}
#[test]
fn original_modular_group_sections_recover_float_pixels_in_full_and_dropped_image_paths() {
    let mut sources = Vec::new();
    let mut settings = Vec::new();
    for (w, h) in [
        (1, 1),
        (17, 9),
        (129, 3),
        (257, 3),
        (401, 3),
        (513, 3),
        (2049, 3),
        (65, 67),
    ] {
        for gray in [0, 1] {
            for depth in [8, 16] {
                for mode in 0..3 {
                    sources.push(format!("mfd-source-groups {w} {h} {gray} {depth} {mode}"));
                    settings.push((w, h, gray, depth));
                }
            }
        }
    }
    let mut cases = Vec::new();
    for ((w, h, gray, depth), source) in settings.into_iter().zip(original_observations(&sources)) {
        let data = parts(&source);
        for drop in [0, 1] {
            for inplace in [0, 1] {
                cases.push(request(w, h, gray, depth, drop, inplace, 0, 1, 1, 5, &data));
            }
        }
    }
    compare(cases);
}
#[test]
fn group_truncation_zerofill_nullable_context_and_stream_kind_errors_keep_original_state() {
    let settings = [
        (17, 9, 0, 8),
        (17, 9, 1, 16),
        (513, 3, 0, 16),
        (2049, 3, 1, 8),
    ];
    let sources: Vec<_> = settings
        .iter()
        .map(|(w, h, g, d)| format!("mfd-source-groups {w} {h} {g} {d} 2"))
        .collect();
    let mut cases = Vec::new();
    for ((w, h, gray, depth), source) in settings.into_iter().zip(original_observations(&sources)) {
        let data = parts(&source);
        for zero in [0, 1] {
            for context in [0, 1] {
                for input in [0, 1] {
                    for kind in [4, 5] {
                        cases.push(request(
                            w, h, gray, depth, 1, 1, zero, context, input, kind, &data,
                        ));
                    }
                }
            }
        }
        for target in 0..data.len() {
            if data[target].is_empty() {
                continue;
            }
            for len in [0, 1, data[target].len() / 2, data[target].len() - 1] {
                let mut damaged = data.clone();
                damaged[target].truncate(len);
                cases.push(request(w, h, gray, depth, 1, 1, 0, 1, 1, 5, &damaged));
            }
        }
    }
    compare(cases);
}
