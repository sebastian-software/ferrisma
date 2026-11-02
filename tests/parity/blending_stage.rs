// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
use jxl::jxl::dec_cache::PassesDecoderState;
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::frame_header::{BlendMode, BlendingInfo, FrameHeader, FrameOrigin};
use jxl::jxl::image::{Image3F, Plane};
use jxl::jxl::image_bundle::ImageBundle;
use jxl::jxl::image_metadata::{CodecMetadata, ExtraChannel, ExtraChannelInfo};
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineStage, Row, RowBuffers, RowStorage,
    WriteToImageBundleStage,
};
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
use jxl::jxl::render_pipeline::stage_blending::{
    get_blending_stage, get_blending_stage_borrowed, get_blending_stage_shared,
};
use std::rc::Rc;
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("blendstage-")
}
fn pixel(r: usize, c: usize, x: usize, y: usize, pattern: u32) -> f32 {
    if pattern != 0 {
        let words = [
            0, 0x80000000, 0x3f800000, 0xbf800000, 0x3f000000, 0x40000000, 0x7fc12345, 0xffc23456,
            0x7fa12345, 0xffa23456, 0x7f800000, 0xff800000, 1, 0x80000001, 0x3f7fffff, 0x3f800001,
        ];
        return f32::from_bits(words[(r * 7 + c * 5 + x + y * 3) % 16]);
    }
    ((r * 37 + c * 11 + x * 3 + y * 7) % 47) as f32 * 0.03125 - 0.25
}
fn blend(args: &mut std::str::SplitWhitespace<'_>) -> BlendingInfo {
    BlendingInfo {
        mode: BlendMode(number(args)),
        source: number(args),
        alpha_channel: number(args),
        clamp: number::<u32>(args) != 0,
        ..BlendingInfo::default()
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let iw = number(args);
    let ih = number(args);
    let ox = number(args);
    let oy = number(args);
    let n = number::<usize>(args);
    let assoc = number::<usize>(args);
    let xyb = number::<u32>(args) != 0;
    let original = number::<u32>(args) != 0;
    let pattern = number(args);
    let ownership = number(args);
    let mut metadata = CodecMetadata::default();
    metadata.size.set(iw, ih).unwrap();
    metadata.m.xyb_encoded = xyb;
    metadata.m.num_extra_channels = n as u32;
    metadata.m.extra_channel_info = (0..n)
        .map(|c| ExtraChannelInfo {
            channel_type: if c % 2 == 0 {
                ExtraChannel::ALPHA
            } else {
                ExtraChannel::DEPTH
            },
            alpha_associated: (assoc >> c) & 1 != 0,
            ..ExtraChannelInfo::default()
        })
        .collect();
    metadata
        .m
        .color_encoding
        .set_srgb(ColorSpace::RGB, RenderingIntent::RELATIVE)
        .unwrap();
    let mut header = FrameHeader::new(Some(&metadata));
    header.frame_origin = FrameOrigin { x0: ox, y0: oy };
    header.blending_info = blend(args);
    header.extra_channel_blending_info = (0..n).map(|_| blend(args)).collect();
    let header = Rc::new(header);
    let mut state = PassesDecoderState::new();
    state.shared_storage.metadata = Some(&metadata);
    state.output_encoding_info.color_encoding_is_original = original;
    {
        let mut refs = state.shared_storage.reference_frames.borrow_mut();
        for r in 0..4 {
            let w = number(args);
            let h = number(args);
            let x = number(args);
            let y = number(args);
            let inxyb = number::<u32>(args) != 0;
            let mut bg = ImageBundle::new(Some(&metadata.m));
            if w != 0 && h != 0 {
                let mut image = Image3F::create(w, h).unwrap();
                for c in 0..3 {
                    for yy in 0..h {
                        for xx in 0..w {
                            image.plane_row_mut(c, yy)[xx] = pixel(r, c, xx, yy, pattern);
                        }
                    }
                }
                bg.set_from_image(&mut Some(image), &metadata.m.color_encoding)
                    .unwrap();
                for c in 0..n {
                    let mut ec = Plane::create(w, h).unwrap();
                    for yy in 0..h {
                        for xx in 0..w {
                            ec.row_mut(yy)[xx] = pixel(r, c + 3, xx, yy, pattern);
                        }
                    }
                    bg.extra_channels_mut().push(ec);
                }
            }
            bg.origin = FrameOrigin { x0: x, y0: y };
            refs[r].frame = Some(Box::new(bg));
            refs[r].ib_is_in_xyb = inxyb;
        }
    }
    let borrowed = if ownership == 0 {
        Some(state.shared_storage.reference_frames.borrow())
    } else {
        None
    };
    let mut stage = match ownership {
        0 => get_blending_stage_borrowed(
            &header,
            borrowed.as_deref().unwrap(),
            xyb,
            original,
            &metadata.m.color_encoding,
        ),
        1 => get_blending_stage(&header, &state, &metadata.m.color_encoding),
        _ => get_blending_stage_shared(header.clone(), &state, &metadata.m.color_encoding),
    };
    let mut w = 999;
    let mut h = 999;
    let mut origin = FrameOrigin::default();
    stage.get_image_dimensions(&mut w, &mut h, &mut origin);
    let sizes = stage.constructor_sizes();
    let mut out = format!(
        "{} H {} {} {} {} {} {} {} {} {} {}",
        status_code(&stage.is_initialized()),
        sizes[0],
        sizes[1],
        sizes[2],
        sizes[3],
        stage.get_name(),
        u8::from(stage.switch_to_image_dimensions()),
        w,
        h,
        origin.x0,
        origin.y0
    );
    for c in [0, 1, 2, 3, 6, 99] {
        write!(out, " {}", stage.get_channel_mode(c) as i32).unwrap();
    }
    for b in stage.blendings() {
        write!(
            out,
            " B {} {} {}",
            b.mode.0,
            b.alpha_channel,
            u8::from(b.clamp)
        )
        .unwrap();
    }
    if kind == "blendstage-row" {
        let width = number(args);
        let x = number(args);
        let y = number(args);
        let left = number(args);
        let right = number(args);
        let mutate = number::<usize>(args);
        if mutate != 0 {
            assert!(ownership != 0);
            let mut refs = state.shared_storage.reference_frames.borrow_mut();
            for (r, reference) in refs.iter_mut().enumerate() {
                let bg = reference.frame.as_deref_mut().unwrap();
                if bg.has_color() {
                    for c in 0..3 {
                        for yy in 0..bg.ysize() {
                            for xx in 0..bg.xsize() {
                                bg.color_mut().plane_row_mut(c, yy)[xx] =
                                    pixel(r + mutate, c, xx, yy, pattern);
                            }
                        }
                    }
                    for c in 0..n {
                        for yy in 0..bg.ysize() {
                            for xx in 0..bg.xsize() {
                                bg.extra_channels_mut()[c].row_mut(yy)[xx] =
                                    pixel(r + mutate, c + 3, xx, yy, pattern);
                            }
                        }
                    }
                }
            }
        }
        let len = OFFSET + width + 8;
        let mut rows: Vec<_> = (0..3 + n)
            .map(|c| {
                let mut row = Plane::create(len, 1).unwrap();
                for xx in 0..len {
                    row.row_mut(0)[xx] = pixel(8, c, xx, 0, pattern);
                }
                row
            })
            .collect();
        let ri = (0..3 + n)
            .map(|c| {
                vec![Row {
                    storage: RowStorage::Input,
                    channel: c,
                    y: 0,
                    x: 0,
                }]
            })
            .collect();
        let result = stage.process_row(
            &ri,
            &Vec::new(),
            &mut RowBuffers {
                input: &mut rows,
                output: &mut [],
            },
            left,
            right,
            width,
            x,
            y,
            17,
        );
        write!(out, " R {}", status_code(&result)).unwrap();
        for row in &rows {
            for v in &row.row(0)[..len] {
                write!(out, " {}", v.to_bits()).unwrap();
            }
        }
        if stage.is_initialized().is_ok() {
            for row in &mut rows {
                row.row_mut(0)[..len].fill(0.125);
            }
            let pw = width.min(iw);
            stage.process_padding_row(
                &ri,
                &mut RowBuffers {
                    input: &mut rows,
                    output: &mut [],
                },
                pw,
                iw - pw,
                ih - 1,
            );
            out.push_str(" P");
            for row in &rows {
                for v in &row.row(0)[..len] {
                    write!(out, " {}", v.to_bits()).unwrap();
                }
            }
        }
    } else {
        assert_eq!(kind, "blendstage-simple");
        let fw = number(args);
        let fh = number(args);
        let mut dim = FrameDimensions::default();
        dim.set(fw, fh, 0, 0, 0, false, 1);
        let mut output = ImageBundle::new(Some(&metadata.m));
        {
            let mut builder = SimpleRenderPipelineBuilder::new(3 + n);
            let mut added = builder.add_stage(Some(stage));
            if added.is_ok() {
                added = builder.add_stage(Some(Box::new(WriteToImageBundleStage::new(
                    &mut output,
                    &metadata.m.color_encoding,
                ))));
            }
            write!(out, " A {}", status_code(&added)).unwrap();
            if added.is_err() {
                return out;
            }
            let built = builder.finalize(dim);
            write!(
                out,
                " Z {}",
                status_code(&built.as_ref().map(|_| ()).map_err(|e| *e))
            )
            .unwrap();
            let Ok(mut pipe) = built else {
                return out;
            };
            let mut status = pipe.is_initialized();
            write!(out, " I {}", status_code(&status)).unwrap();
            if status.is_ok() {
                status = pipe.prepare_for_threads(1, false);
            }
            if status.is_ok() {
                for g in 0..dim.num_groups {
                    let mut input = pipe.get_input_buffers(g, 0);
                    for c in 0..3 + n {
                        let (p, r) = input.get_buffer_mut(c);
                        for yy in 0..r.ysize() {
                            for xx in 0..r.xsize() {
                                p.row_mut(r.y0() + yy)[r.x0() + xx] = pixel(
                                    8,
                                    c,
                                    r.x0() - OFFSET + xx,
                                    r.y0() - OFFSET + yy,
                                    pattern,
                                );
                            }
                        }
                    }
                    status = input.done();
                    if status.is_err() {
                        break;
                    }
                }
            }
            write!(out, " D {}", status_code(&status)).unwrap();
        }
        write!(
            out,
            " {} {} {}",
            output.xsize(),
            output.ysize(),
            output.extra_channels().len()
        )
        .unwrap();
        if output.has_color() {
            for c in 0..3 + n {
                for yy in 0..output.ysize() {
                    for xx in 0..output.xsize() {
                        let v = if c < 3 {
                            output.color().plane_row(c, yy)[xx]
                        } else {
                            output.extra_channels()[c - 3].row(yy)[xx]
                        };
                        write!(out, " {}", v.to_bits()).unwrap();
                    }
                }
            }
        }
    }
    out
}
#[expect(
    clippy::too_many_arguments,
    reason = "Keep the independent source fixture axes explicit."
)]
fn config(
    iw: usize,
    ih: usize,
    origin: (i32, i32),
    n: usize,
    assoc: usize,
    xyb: u8,
    original: u8,
    pattern: u8,
    ownership: u8,
    color: [u32; 4],
    extras: &[[u32; 4]],
    refs: &[[i32; 5]; 4],
) -> String {
    let mut s = format!(
        "{iw} {ih} {} {} {n} {assoc} {xyb} {original} {pattern} {ownership}",
        origin.0, origin.1
    );
    for b in std::iter::once(&color).chain(extras.iter()) {
        for v in b {
            write!(s, " {v}").unwrap();
        }
    }
    for r in refs {
        for v in r {
            write!(s, " {v}").unwrap();
        }
    }
    s
}
#[test]
fn frame_blending_constructor_status_partial_state_and_guard_order_match_original() {
    let mut cases = Vec::new();
    for ownership in [0, 1, 2] {
        for n in [0, 1, 3] {
            for xyb in [0, 1] {
                for original in [0, 1] {
                    for mutation in 0..12 {
                        let mut refs = [[9, 7, 0, 0, 0]; 4];
                        match mutation {
                            1 => refs[0] = [0, 0, 0, 0, 1],
                            2 => refs[0][4] = 1,
                            3 => refs[1] = [0, 0, 0, 0, 1],
                            4 => refs[0][0] = 8,
                            5 => refs[1][1] = 6,
                            6 => refs[0][2] = 1,
                            7 => refs[1][3] = -1,
                            8 => refs[1][4] = 1,
                            9 => {
                                refs[0][0] = 8;
                                refs[1] = [0, 0, 0, 0, 0];
                            }
                            10 => {
                                refs[0][4] = 1;
                                refs[1] = [0, 0, 0, 0, 0];
                            }
                            11 => {
                                refs[0] = [0, 0, 0, 0, 0];
                                refs[1][2] = 1;
                            }
                            _ => {}
                        }
                        let extra = vec![[2, 1, 0, 1]; n];
                        let c = config(
                            9,
                            7,
                            (0, 0),
                            n,
                            0,
                            xyb,
                            original,
                            0,
                            ownership,
                            [2, 0, 0, 1],
                            &extra,
                            &refs,
                        );
                        cases.push(format!("blendstage-row {c} 5 0 0 0 0 0"));
                    }
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn frame_blending_rows_modes_independent_alpha_sources_signed_clipping_and_padding_match_original()
{
    let mut cases = Vec::new();
    for ownership in [0, 1, 2] {
        for n in [0, 1, 3] {
            for mode in 0..5 {
                for assoc in [0, 1, 5] {
                    for pattern in [0, 1] {
                        for (ox, oy, x, y, width, left, right) in [
                            (0, 0, 0, 0, 9, 0, 0),
                            (-3, -2, 0, 2, 9, 0, 0),
                            (3, 2, 0, 0, 9, 0, 0),
                            (-20, 0, 0, 0, 9, 0, 0),
                            (9, 0, 0, 0, 9, 0, 0),
                            (0, -20, 0, 0, 9, 0, 0),
                            (0, 7, 0, 0, 9, 0, 0),
                            (0, 0, 4, 6, 9, 0, 0),
                            // Zero span returns at the source signed-position guard,
                            // before an empty PerformBlending temporary is reached.
                            (0, 0, 0, 0, 0, 0, 0),
                            (0, 0, 0, 0, 9, 1, 0),
                            (0, 0, 0, 0, 9, 0, 1),
                        ] {
                            let extras: Vec<_> = (0..n)
                                .map(|i| {
                                    [
                                        (mode + i as u32) % 5,
                                        (i as u32 + 1) % 4,
                                        if n > 2 { 2 } else { 0 },
                                        1,
                                    ]
                                })
                                .collect();
                            let refs = if mode % 2 == 0 {
                                [[9, 7, 0, 0, 0]; 4]
                            } else {
                                [
                                    [0, 0, 0, 0, 0],
                                    [9, 7, 0, 0, 0],
                                    [0, 0, 0, 0, 0],
                                    [9, 7, 0, 0, 0],
                                ]
                            };
                            let c = config(
                                9,
                                7,
                                (ox, oy),
                                n,
                                assoc,
                                1,
                                1,
                                pattern,
                                ownership,
                                [mode, 0, if n > 2 { 2 } else { 0 }, 1],
                                &extras,
                                &refs,
                            );
                            cases.push(format!(
                                "blendstage-row {c} {width} {x} {y} {left} {right} {}",
                                u8::from(ownership != 0)
                            ));
                        }
                    }
                }
            }
        }
    }
    for chunk in cases.chunks(64) {
        compare(chunk.to_vec());
    }
}
#[test]
fn frame_blending_actual_simple_pipeline_switches_to_canvas_and_preserves_backgrounds() {
    let mut cases = Vec::new();
    for n in [0, 1, 3] {
        for ownership in [0, 1, 2] {
            for mode in 0..5 {
                for (ox, oy, fw, fh) in [
                    (0, 0, 9, 7),
                    (-3, -2, 9, 7),
                    (3, 2, 9, 7),
                    (2, 1, 3, 4),
                    (-1, 0, 10, 7),
                ] {
                    let extras: Vec<_> = (0..n).map(|i| [mode, (i as u32 + 1) % 4, 0, 1]).collect();
                    let c = config(
                        9,
                        7,
                        (ox, oy),
                        n,
                        0,
                        1,
                        1,
                        0,
                        ownership,
                        [mode, 0, 0, 1],
                        &extras,
                        &[[9, 7, 0, 0, 0]; 4],
                    );
                    cases.push(format!("blendstage-simple {c} {fw} {fh}"));
                }
            }
        }
    }
    for chunk in cases.chunks(32) {
        compare(chunk.to_vec());
    }
}

#[test]
fn frame_blending_all_main_slots_clamps_and_oversized_backgrounds_match_original() {
    let mut cases = Vec::new();
    for ownership in [0, 1, 2] {
        for source in 0..4 {
            for clamp in [0, 1] {
                for mode in 0..5 {
                    for pattern in [0, 1] {
                        let extras = vec![
                            [mode, (source + 1) % 4, 2, clamp],
                            [mode, (source + 2) % 4, 0, clamp],
                            [mode, (source + 3) % 4, 2, clamp],
                        ];
                        let c = config(
                            9,
                            7,
                            (-2, 1),
                            3,
                            5,
                            1,
                            1,
                            pattern,
                            ownership,
                            [mode, source, 2, clamp],
                            &extras,
                            &[[12, 10, 0, 0, 0]; 4],
                        );
                        cases.push(format!(
                            "blendstage-row {c} 12 0 0 0 0 {}",
                            u8::from(ownership != 0)
                        ));
                    }
                }
            }
        }
    }
    for chunk in cases.chunks(64) {
        compare(chunk.to_vec());
    }
}

#[test]
fn frame_blending_pipeline_error_checkpoints_and_multiple_groups_match_original() {
    let mut cases = Vec::new();
    for ownership in [0, 1, 2] {
        for invalid in 0..5 {
            let mut refs = [[9, 7, 0, 0, 0]; 4];
            let mut original = 1;
            match invalid {
                0 => refs[0][4] = 1,
                1 => refs[0][0] = 8,
                2 => refs[1][3] = 1,
                3 => original = 0,
                _ => {
                    refs[0][4] = 1;
                    refs[1] = [0, 0, 0, 0, 0];
                }
            }
            let c = config(
                9,
                7,
                (2, 1),
                1,
                0,
                1,
                original,
                0,
                ownership,
                [2, 0, 0, 1],
                &[[2, 1, 0, 1]],
                &refs,
            );
            cases.push(format!("blendstage-simple {c} 3 4"));
        }
        for (ox, oy, fw, fh) in [(-250, 0, 259, 7), (0, -250, 9, 257), (-250, -250, 259, 257)] {
            let c = config(
                9,
                7,
                (ox, oy),
                3,
                5,
                1,
                1,
                0,
                ownership,
                [1, 3, 2, 0],
                &[[1, 2, 2, 0], [1, 1, 0, 1], [1, 0, 2, 0]],
                &[[9, 7, 0, 0, 0]; 4],
            );
            cases.push(format!("blendstage-simple {c} {fw} {fh}"));
        }
    }
    compare(cases);
}
