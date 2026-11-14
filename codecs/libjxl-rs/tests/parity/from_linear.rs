// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::cms::transfer_functions_scalar::{Tf709, TfHlg};
use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent, TransferFunction};
use jxl::jxl::dec_xyb::OutputEncodingInfo;
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::image::Plane;
use jxl::jxl::image_bundle::ImageBundle;
use jxl::jxl::image_metadata::{CodecMetadata, ExtraChannelInfo};
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, Row, RowBuffers, RowStorage, WriteToImageBundleStage,
};
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
use jxl::jxl::render_pipeline::stage_from_linear::get_from_linear_stage;
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("fl-")
}
fn f(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(number(args))
}
fn pixel(c: usize, x: usize, y: usize, pattern: u32) -> f32 {
    if pattern == 1 {
        let w = [
            0, 0x80000000, 0x3f800000, 0xbf800000, 0x3f000000, 0x40000000, 0x7fc12345, 0xffc23456,
            0x7fa12345, 0xffa23456, 0x7f800000, 0xff800000, 1, 0x80000001, 0x3f7fffff, 0x3f800001,
        ];
        return f32::from_bits(w[(c * 5 + x + y * 7) % 16]);
    }
    if pattern == 2 || pattern == 4 {
        let w = [
            0,
            0x80000000,
            1,
            0x007fffff,
            0x322bcc77,
            0x3727c5ab,
            0x3727c5ac,
            0x3727c5ad,
            0x3c9374bb,
            0x3c9374bc,
            0x3c9374bd,
            0x3da5e353,
            0x3da5e354,
            0x3da5e355,
            0x3daaaaaa,
            0x3daaaaab,
            0x3daaaaac,
            0x3effffff,
            0x3f000000,
            0x3f000001,
            0x3f7fffff,
            0x3f800000,
            0x3f800001,
            0x40000000,
            0x447a0000,
            if pattern == 4 { 0x461c4000 } else { 0x7f800000 },
        ];
        return f32::from_bits(w[(c * 5 + x + y * 7) % 26]);
    }
    if pattern == 3 && c < 3 {
        return if (x + y) % 3 == c {
            -0.125
        } else {
            [0.25, 0.5, 1.0][c]
        };
    }
    ((c * 7 + x * 3 + y * 11) % 33) as f32 * 0.03125
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "fl-tf" {
        let op: u32 = number(args);
        if op == 4 {
            return Tf709::encoded_from_display_f64(f64::from_bits(number(args)))
                .to_bits()
                .to_string();
        }
        let x = f(args);
        return match op {
            0 => TfHlg::encoded_from_display(x),
            1 => TfHlg::display_from_encoded(x),
            2 => Tf709::encoded_from_display(x),
            _ => Tf709::display_from_encoded(x),
        }
        .to_bits()
        .to_string();
    }
    let tf = TransferFunction(number(args));
    let have_gamma = number::<u32>(args) != 0;
    let gamma = number(args);
    let mut info = OutputEncodingInfo {
        orig_intensity_target: f(args),
        desired_intensity_target: f(args),
        inverse_gamma: f(args),
        luminances: [f(args), f(args), f(args)],
        ..OutputEncodingInfo::default()
    };
    info.color_encoding.tf_mut().set_transfer_function(tf);
    info.color_encoding.tf_mut().have_gamma = have_gamma;
    info.color_encoding.tf_mut().gamma = gamma;
    let stage = get_from_linear_stage(&info);
    info.orig_intensity_target = 7.0;
    info.desired_intensity_target = 3.0;
    info.inverse_gamma = 2.0;
    info.luminances = [0.0; 3];
    info.color_encoding
        .tf_mut()
        .set_transfer_function(TransferFunction::UNKNOWN);
    let Some(stage) = stage else {
        return "0".into();
    };
    let s = stage.settings();
    let mut out = format!(
        "1 {} {} {} {} {} {}",
        stage.get_name(),
        status_code(&stage.is_initialized()),
        s.shift_x,
        s.shift_y,
        s.border_x,
        s.border_y
    );
    for c in [0, 1, 2, 3, 4, 99] {
        write!(out, " {}", stage.get_channel_mode(c) as i32).unwrap();
    }
    if kind == "fl-row" {
        let width = number(args);
        let left = number(args);
        let right = number(args);
        let offset = number(args);
        let extra = number::<usize>(args);
        let pattern = number(args);
        let passes = number(args);
        let len = OFFSET + offset + width + 8;
        let mut rows: Vec<_> = (0..3 + extra)
            .map(|_| Plane::create(len, 1).unwrap())
            .collect();
        let ri = (0..3 + extra)
            .map(|c| {
                vec![Row {
                    storage: RowStorage::Input,
                    channel: c,
                    y: 0,
                    x: offset,
                }]
            })
            .collect();
        for pass in 0..passes {
            for (c, row) in rows.iter_mut().enumerate() {
                for x in 0..len {
                    row.row_mut(0)[x] = pixel(c, x, pass, pattern);
                }
            }
            let status = stage.process_row(
                &ri,
                &vec![],
                &mut RowBuffers {
                    input: &mut rows,
                    output: &mut [],
                },
                left,
                right,
                width,
                19 + pass,
                23 + pass,
                pass,
            );
            write!(out, " R {}", status_code(&status)).unwrap();
            for row in &rows {
                for x in &row.row(0)[..len] {
                    write!(out, " {}", x.to_bits()).unwrap();
                }
            }
        }
    } else {
        assert_eq!(kind, "fl-simple");
        let width = number(args);
        let height = number(args);
        let extra = number::<usize>(args);
        let pattern = number(args);
        let mut dim = FrameDimensions::default();
        dim.set(width, height, 0, 0, 0, false, 1);
        let mut metadata = CodecMetadata::default();
        metadata.size.set(width, height).unwrap();
        metadata.m.num_extra_channels = extra as u32;
        metadata.m.extra_channel_info = vec![ExtraChannelInfo::default(); extra];
        metadata
            .m
            .color_encoding
            .set_srgb(ColorSpace::RGB, RenderingIntent::RELATIVE)
            .unwrap();
        let mut output = ImageBundle::new(Some(&metadata.m));
        {
            let mut builder = SimpleRenderPipelineBuilder::new(3 + extra);
            let mut status = builder.add_stage(Some(stage));
            if status.is_ok() {
                status = builder.add_stage(Some(Box::new(WriteToImageBundleStage::new(
                    &mut output,
                    &metadata.m.color_encoding,
                ))));
            }
            write!(out, " A {}", status_code(&status)).unwrap();
            if status.is_err() {
                return out;
            }
            let made = builder.finalize(dim);
            write!(
                out,
                " Z {}",
                status_code(&made.as_ref().map(|_| ()).map_err(|e| *e))
            )
            .unwrap();
            let Ok(mut pipeline) = made else {
                return out;
            };
            status = pipeline.is_initialized();
            write!(out, " I {}", status_code(&status)).unwrap();
            if status.is_ok() {
                status = pipeline.prepare_for_threads(2, true);
            }
            if status.is_ok() {
                for i in 0..dim.num_groups {
                    let mut input = pipeline.get_input_buffers(dim.num_groups - 1 - i, i % 2);
                    for c in 0..3 + extra {
                        let (p, r) = input.get_buffer_mut(c);
                        for y in 0..r.ysize() {
                            for x in 0..r.xsize() {
                                p.row_mut(r.y0() + y)[r.x0() + x] =
                                    pixel(c, r.x0() + x - OFFSET, r.y0() + y - OFFSET, pattern);
                            }
                        }
                    }
                    status = input.done();
                    write!(out, " D {}", status_code(&status)).unwrap();
                    if status.is_err() {
                        break;
                    }
                }
            }
        }
        write!(
            out,
            " O {} {} {}",
            output.xsize(),
            output.ysize(),
            output.extra_channels().len()
        )
        .unwrap();
        if output.has_color() {
            for c in 0..3 + extra {
                for y in 0..output.ysize() {
                    for x in 0..output.xsize() {
                        let v = if c < 3 {
                            output.color().plane_row(c, y)[x]
                        } else {
                            output.extra_channels()[c - 3].row(y)[x]
                        };
                        write!(out, " {}", v.to_bits()).unwrap();
                    }
                }
            }
        }
    }
    out
}
fn config(
    tf: TransferFunction,
    custom: bool,
    gamma: u32,
    original: f32,
    desired: f32,
    inverse: f32,
    lum: [f32; 3],
) -> String {
    let mut s = format!(
        "{} {} {gamma} {} {} {}",
        tf.0,
        u8::from(custom),
        original.to_bits(),
        desired.to_bits(),
        inverse.to_bits()
    );
    for x in lum {
        write!(s, " {}", x.to_bits()).unwrap();
    }
    s
}
#[test]
fn hlg_and_bt709_original_scalar_input_output_thresholds_and_words_match() {
    let mut values = vec![
        0.0,
        -0.0,
        f32::from_bits(1),
        f32::from_bits(0x80000001),
        f32::MIN_POSITIVE,
        f32::MAX,
        -f32::MAX,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];
    for threshold in [1.0f32 / 12.0, 0.5, 0.018, 0.081, 1e-5, 1.0] {
        for delta in [-1i32, 0, 1] {
            let x = f32::from_bits(threshold.to_bits().wrapping_add_signed(delta));
            values.extend([x, -x]);
        }
    }
    let mut seed = 13u64;
    for _ in 0..256 {
        let x = (next(&mut seed) % 100000) as f32 / 1031.0;
        values.extend([x, -x]);
    }
    let mut cases = Vec::new();
    for op in 0..4 {
        for &x in &values {
            cases.push(format!("fl-tf {op} {}", x.to_bits()));
        }
    }
    // HLG's inverse reaches FastPow2f, which has defined scalar conversion for
    // these words; no selected FastLog2f negative/NaN approximation is requested.
    for bits in [0x7fc12345u32, 0xffc23456, 0x7fa12345, 0xffa23456] {
        cases.push(format!("fl-tf 1 {bits}"));
    }
    for chunk in cases.chunks(128) {
        compare(chunk.to_vec());
    }
}
#[test]
fn bt709_original_double_overload_retains_strict_threshold_and_precision() {
    let mut cases = Vec::new();
    for x in [
        0.0f64,
        -0.0,
        -1.0,
        0.018,
        f64::from_bits(0.018f64.to_bits() - 1),
        f64::from_bits(0.018f64.to_bits() + 1),
        1.0,
        1e-20,
        1e20,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ] {
        cases.push(format!("fl-tf 4 {}", x.to_bits()));
    }
    compare(cases);
}
#[test]
fn from_linear_actual_factory_copies_operations_reuses_rows_and_preserves_guards_and_extras() {
    let mut cases = Vec::new();
    for (tf, custom) in [
        (TransferFunction::LINEAR, false),
        (TransferFunction::SRGB, false),
        (TransferFunction::PQ, false),
        (TransferFunction::HLG, false),
        (TransferFunction::BT709, false),
        (TransferFunction::DCI, false),
        (TransferFunction::LINEAR, true),
        (TransferFunction::HLG, true),
    ] {
        for state in 0..4 {
            let lum = if state % 2 == 0 {
                [0.2627, 0.6780, 0.0593]
            } else {
                [0.2126, 0.7152, 0.0722]
            };
            let c = config(
                tf,
                custom,
                10000000 - state,
                [0.0, 203.0, 1000.0, 10000.0][state as usize],
                [203.0, 334.0, 1000.0, 4000.0][state as usize],
                [0.45, 1.0 / 2.6, 1.0, 2.0][state as usize],
                lum,
            );
            let patterns = if custom
                || tf == TransferFunction::DCI
                || tf == TransferFunction::BT709
                || tf == TransferFunction::HLG
            {
                [0, 2]
            } else {
                [0, 1]
            };
            for pattern in patterns {
                for width in [0, 1, 7, 31] {
                    for (left, right) in [(0, 0), (1, 0), (0, 1)] {
                        for offset in [0, 3] {
                            for extra in [0, 2] {
                                cases.push(format!(
                                    "fl-row {c} {width} {left} {right} {offset} {extra} {pattern} 2"
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    for chunk in cases.chunks(32) {
        compare(chunk.to_vec());
    }
}
#[test]
fn from_linear_hlg_copies_luminances_and_keeps_negative_channels_with_positive_luma() {
    let mut cases = Vec::new();
    for lum in [
        [0.2627, 0.6780, 0.0593],
        [0.2126, 0.7152, 0.0722],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ] {
        for desired in [80.0, 203.0, 320.0, 334.0, 350.0, 1000.0, 10000.0] {
            let c = config(TransferFunction::HLG, false, 0, 1000.0, desired, 0.45, lum);
            for pattern in [0, 4] {
                cases.push(format!("fl-row {c} 31 0 0 1 2 {pattern} 2"));
            }
            if lum[0] > 0.0 && lum[1] > 0.0 && lum[2] > 0.0 {
                cases.push(format!("fl-row {c} 31 0 0 1 2 3 2"));
            }
        }
    }
    compare(cases);
}
#[test]
fn from_linear_actual_simple_pipeline_handles_groups_copied_state_and_extra_planes() {
    let mut cases = Vec::new();
    for (tf, custom) in [
        (TransferFunction::LINEAR, false),
        (TransferFunction::SRGB, false),
        (TransferFunction::PQ, false),
        (TransferFunction::HLG, false),
        (TransferFunction::BT709, false),
        (TransferFunction::DCI, false),
        (TransferFunction::PQ, true),
    ] {
        for desired in [203.0, 334.0, 4000.0] {
            let c = config(
                tf,
                custom,
                2300000,
                1000.0,
                desired,
                0.45,
                [0.2627, 0.6780, 0.0593],
            );
            for (w, h) in [(1, 1), (7, 3), (65, 9), (257, 3)] {
                for extra in [0, 2] {
                    cases.push(format!("fl-simple {c} {w} {h} {extra} 0"));
                }
            }
        }
    }
    for chunk in cases.chunks(8) {
        compare(chunk.to_vec());
    }
}

#[test]
fn masked_negative_gamma_and_bt709_rows_retain_source_vector_selection() {
    let mut cases = Vec::new();
    for (tf, custom) in [
        (TransferFunction::DCI, false),
        (TransferFunction::BT709, false),
        (TransferFunction::LINEAR, true),
        (TransferFunction::UNKNOWN, true),
    ] {
        for inverse in [0.1, 0.45, 1.0, 4.0] {
            let c = config(
                tf,
                custom,
                1,
                1000.0,
                203.0,
                inverse,
                [0.2627, 0.6780, 0.0593],
            );
            for offset in [0, 3] {
                for extra in [0, 2] {
                    cases.push(format!("fl-row {c} 31 0 0 {offset} {extra} 3 2"));
                }
            }
        }
    }
    compare(cases);
}
