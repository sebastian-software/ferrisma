// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Interpolation stages translate the original test_render_pipeline_stages.h.
// Probe is a fixture callback for failure and image-dimension transitions.
use super::*;
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::frame_header::FrameOrigin;
use jxl::jxl::image::Image3;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as X_OFFSET, RenderPipelineChannelMode as Mode, RenderPipelineStage,
    RowBuffers, RowInfo, Settings, WriteToImage3FStage, WriteToImageBundleStage,
};
use jxl::jxl::render_pipeline::simple_render_pipeline::{
    RenderPipelineInput, SimpleRenderPipeline, SimpleRenderPipelineBuilder,
};
use std::cell::Cell;
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("rp-")
}
struct Upsample {
    vertical: bool,
}
impl RenderPipelineStage for Upsample {
    fn settings(&self) -> Settings {
        if self.vertical {
            Settings::shift_y(1, 1)
        } else {
            Settings::shift_x(1, 1)
        }
    }
    fn get_channel_mode(&self, _c: usize) -> Mode {
        Mode::InOut
    }
    fn get_name(&self) -> &str {
        if self.vertical {
            "TEST::UpsampleYSlowStage"
        } else {
            "TEST::UpsampleXSlowStage"
        }
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        _x: usize,
        _y: usize,
        _thread: usize,
    ) -> Status {
        for c in 0..input.len() {
            for x in -(left as isize)..width.wrapping_add(right) as isize {
                let (xp, xc, xn) = if self.vertical {
                    (
                        buffers.row(self.get_input_row(input, c, -1).offset(x))[0],
                        buffers.row(self.get_input_row(input, c, 0).offset(x))[0],
                        buffers.row(self.get_input_row(input, c, 1).offset(x))[0],
                    )
                } else {
                    let row = self.get_input_row(input, c, 0);
                    (
                        buffers.row(row.offset(x - 1))[0],
                        buffers.row(row.offset(x))[0],
                        buffers.row(row.offset(x + 1))[0],
                    )
                };
                let out0 = xp * 0.25 + xc * 0.75;
                let out1 = xc * 0.75 + xn * 0.25;
                if self.vertical {
                    buffers.row_mut(self.get_output_row(output, c, 0).offset(x))[0] = out0;
                    buffers.row_mut(self.get_output_row(output, c, 1).offset(x))[0] = out1;
                } else {
                    let row = self.get_output_row(output, c, 0);
                    buffers.row_mut(row.offset(2 * x))[0] = out0;
                    buffers.row_mut(row.offset(2 * x + 1))[0] = out1;
                }
            }
        }
        Ok(())
    }
}
#[derive(Default)]
struct ProbeState {
    init: Cell<usize>,
    prepare: Cell<usize>,
    sizes: Cell<usize>,
    rows: Cell<usize>,
    padding: Cell<usize>,
}
fn bump(n: &Cell<usize>) {
    n.set(n.get() + 1);
}
struct Probe<'a> {
    state: &'a ProbeState,
    failures: u32,
    failrow: i32,
    change: bool,
    w: usize,
    h: usize,
    origin: FrameOrigin,
}
impl RenderPipelineStage for Probe<'_> {
    fn settings(&self) -> Settings {
        Settings {
            border_y: usize::from(self.failures & 16 != 0 && self.state.sizes.get() != 0),
            ..Settings::default()
        }
    }
    fn get_channel_mode(&self, _c: usize) -> Mode {
        Mode::InPlace
    }
    fn get_name(&self) -> &str {
        "fixture::probe"
    }
    fn is_initialized(&self) -> Status {
        bump(&self.state.init);
        if self.failures & 1 != 0 {
            Err(StatusCode::GenericError)
        } else {
            Ok(())
        }
    }
    fn prepare_for_threads(&mut self, _num: usize) -> Status {
        bump(&self.state.prepare);
        if self.failures & 2 != 0 {
            Err(StatusCode::GenericError)
        } else {
            Ok(())
        }
    }
    fn set_input_sizes(&mut self, _sizes: &[(usize, usize)]) -> Status {
        bump(&self.state.sizes);
        if self.failures & 4 != 0 {
            Err(StatusCode::GenericError)
        } else {
            Ok(())
        }
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        _left: usize,
        _right: usize,
        width: usize,
        _x: usize,
        y: usize,
        _thread: usize,
    ) -> Status {
        bump(&self.state.rows);
        if self.failures & 8 != 0 && y as i32 == self.failrow {
            return Err(StatusCode::GenericError);
        }
        for c in 0..input.len() {
            let row = self.get_input_row(input, c, 0);
            for value in &mut buffers.row_mut(row)[..width] {
                *value += 1.0;
            }
        }
        Ok(())
    }
    fn switch_to_image_dimensions(&self) -> bool {
        self.change
    }
    fn get_image_dimensions(&self, w: &mut usize, h: &mut usize, origin: &mut FrameOrigin) {
        *w = self.w;
        *h = self.h;
        *origin = self.origin;
    }
    fn process_padding_row(
        &mut self,
        output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        width: usize,
        _x: usize,
        y: usize,
    ) {
        bump(&self.state.padding);
        for (c, rows) in output.iter().enumerate() {
            let row = rows[0].offset(X_OFFSET as isize);
            for (x, v) in buffers.row_mut(row)[..width].iter_mut().enumerate() {
                *v = -((c * 19 + x * 3 + y * 7) as f32);
            }
        }
    }
}
fn value(c: usize, x: usize, y: usize, pass: usize, mode: u32) -> f32 {
    if mode == 0 {
        return 0.0;
    }
    if mode == 3 {
        let words = [
            0u32, 0x80000000, 0x7fa12345, 0x7fc98765, 0x7f800000, 0xff800000, 1, 0x3f800000,
        ];
        return f32::from_bits(words[(c + x + y + pass) % 8]);
    }
    (((c * 31 + x * 17 + y * 9 + pass * 11) % 257) as i32 - 128) as f32 / 16.0
}
pub(super) fn pipeline(out: &mut String, p: &SimpleRenderPipeline<'_>) {
    write!(
        out,
        " P {} {}",
        p.passes_with_all_input(),
        p.processed_passes()
    )
    .unwrap();
    frame::dimensions(out, p.frame_dimensions());
    write!(out, " {}", p.group_completed_passes().len()).unwrap();
    for v in p.group_completed_passes() {
        write!(out, " {v}").unwrap();
    }
    for table in [p.channel_shifts(), p.padding()] {
        write!(out, " {}", table.len()).unwrap();
        for stage in table {
            write!(out, " {}", stage.len()).unwrap();
            for (x, y) in stage {
                write!(out, " {x},{y}").unwrap();
            }
        }
    }
    write!(out, " {}", p.channel_data().len()).unwrap();
    for ch in p.channel_data() {
        write!(out, " {},{},{}", ch.xsize(), ch.ysize(), ch.bytes_per_row()).unwrap();
        if p.processed_passes() != 0 {
            for y in X_OFFSET..ch.ysize() - X_OFFSET {
                for x in X_OFFSET..ch.xsize() - X_OFFSET {
                    write!(out, " {}", ch.row(y)[x].to_bits()).unwrap();
                }
            }
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "rp-set-image" {
        return set_image(args);
    }
    if kind == "rp-default" {
        let mut a = RenderPipelineInput::default();
        let s = a.done();
        let mut b = RenderPipelineInput::move_from(&mut a);
        return format!(
            "{} {} {}",
            status_code(&s),
            status_code(&a.done()),
            status_code(&b.done())
        );
    }
    assert_eq!(kind, "rp-simple");
    let w = number(args);
    let h = number(args);
    let channels = number::<usize>(args);
    let shifts = number::<u32>(args);
    let passes = number::<usize>(args);
    let threads = number(args);
    let group_ids = number::<u32>(args) != 0;
    let reverse = number::<u32>(args) != 0;
    let move_input = number::<u32>(args) != 0;
    let mode = number::<u32>(args);
    let failures = number::<u32>(args);
    let failrow = number(args);
    let change = number::<u32>(args) != 0;
    let iw = number(args);
    let ih = number(args);
    let origin = FrameOrigin {
        x0: number(args),
        y0: number(args),
    };
    let build = number::<u32>(args);
    let mut dim = FrameDimensions::default();
    dim.set(w, h, 0, 0, 0, true, 1);
    let probe = ProbeState::default();
    let mut output = Image3::<f32>::default();
    let use_bundle = mode & 16 != 0;
    let gray = mode & 32 != 0;
    let mut metadata = jxl::jxl::image_metadata::ImageMetadata {
        num_extra_channels: channels.saturating_sub(3) as u32,
        extra_channel_info: vec![
            jxl::jxl::image_metadata::ExtraChannelInfo::default();
            channels.saturating_sub(3)
        ],
        ..Default::default()
    };
    metadata
        .color_encoding
        .set_srgb(
            if gray {
                jxl::jxl::color_encoding::ColorSpace::GRAY
            } else {
                jxl::jxl::color_encoding::ColorSpace::RGB
            },
            jxl::jxl::color_encoding::RenderingIntent::RELATIVE,
        )
        .unwrap();
    let mut bundle = jxl::jxl::image_bundle::ImageBundle::new(Some(&metadata));
    let mut out = String::new();
    {
        let mut builder = SimpleRenderPipelineBuilder::new(channels);
        if build & 1 != 0 {
            write!(out, " N {}", status_code(&builder.add_stage(None))).unwrap();
        }
        if shifts & 1 != 0 {
            builder
                .add_stage(Some(Box::new(Upsample { vertical: false })))
                .unwrap();
        }
        if shifts & 2 != 0 {
            builder
                .add_stage(Some(Box::new(Upsample { vertical: true })))
                .unwrap();
        }
        if change || failures != 0 {
            builder
                .add_stage(Some(Box::new(Probe {
                    state: &probe,
                    failures,
                    failrow,
                    change,
                    w: iw,
                    h: ih,
                    origin,
                })))
                .unwrap();
        }
        if build & 2 == 0 {
            if use_bundle {
                builder
                    .add_stage(Some(Box::new(WriteToImageBundleStage::new(
                        &mut bundle,
                        &metadata.color_encoding,
                    ))))
                    .unwrap();
            } else {
                builder
                    .add_stage(Some(Box::new(WriteToImage3FStage::new(&mut output))))
                    .unwrap();
            }
        }
        let made = builder.finalize(dim);
        write!(out, " B {}", made.as_ref().err().map_or(0, |s| *s as i32)).unwrap();
        if let Ok(mut p) = made {
            let init = p.is_initialized();
            write!(out, " I {}", status_code(&init)).unwrap();
            if init.is_ok() {
                let prep = p.prepare_for_threads(threads, group_ids);
                write!(out, " T {}", status_code(&prep)).unwrap();
                if prep.is_ok() {
                    for pass in 0..passes {
                        for i in 0..dim.num_groups {
                            let group = if reverse { dim.num_groups - i - 1 } else { i };
                            let mut input = p.get_input_buffers(group, 0);
                            write!(out, " R {group}").unwrap();
                            for c in 0..input.buffer_count() {
                                let (plane, r) = input.get_buffer_mut(c);
                                write!(out, " {},{},{},{}", r.x0(), r.y0(), r.xsize(), r.ysize())
                                    .unwrap();
                                for y in 0..r.ysize() {
                                    for x in 0..r.xsize() {
                                        plane.row_mut(r.y0() + y)[r.x0() + x] = value(
                                            if gray && c < 3 { 0 } else { c },
                                            r.x0() + x - X_OFFSET,
                                            r.y0() + y - X_OFFSET,
                                            pass,
                                            mode & 15,
                                        );
                                    }
                                }
                            }
                            let status = if move_input {
                                let mut moved = RenderPipelineInput::move_from(&mut input);
                                write!(out, " M {}", status_code(&input.done())).unwrap();
                                moved.done()
                            } else {
                                input.done()
                            };
                            drop(input);
                            write!(out, " D {}", status_code(&status)).unwrap();
                            pipeline(&mut out, &p);
                            p.clear_done(group);
                        }
                        write!(
                            out,
                            " A {}",
                            status_code(&p.prepare_for_threads(threads, group_ids))
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
    let output = if use_bundle { bundle.color() } else { &output };
    write!(
        out,
        " C {} {} {} {} {} O {} {}",
        probe.init.get(),
        probe.prepare.get(),
        probe.sizes.get(),
        probe.rows.get(),
        probe.padding.get(),
        output.xsize(),
        output.ysize()
    )
    .unwrap();
    for c in 0..3 {
        for y in 0..output.ysize() {
            for x in 0..output.xsize() {
                write!(out, " {}", output.plane_row(c, y)[x].to_bits()).unwrap();
            }
        }
    }
    if use_bundle {
        write!(out, " E {}", bundle.extra_channels().len()).unwrap();
        for ec in bundle.extra_channels() {
            write!(out, " {},{}", ec.xsize(), ec.ysize()).unwrap();
            for y in 0..ec.ysize() {
                for x in 0..ec.xsize() {
                    write!(out, " {}", ec.row(y)[x].to_bits()).unwrap();
                }
            }
        }
        metadata::color_bundle(&mut out, bundle.c_current());
    }
    out
}
fn set_image(args: &mut std::str::SplitWhitespace<'_>) -> String {
    use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
    use jxl::jxl::color_encoding_internal::ColorEncoding;
    use jxl::jxl::image::Plane;
    use jxl::jxl::image_bundle::ImageBundle;
    use jxl::jxl::image_metadata::ImageMetadata;
    use jxl::jxl::image_ops::{fill_image, fill_image3};
    let gray = number::<u32>(args) != 0;
    let profile_gray = number::<u32>(args) != 0;
    let w = number(args);
    let h = number(args);
    let extra = number::<u32>(args) != 0;
    let ew = number(args);
    let eh = number(args);
    let mut metadata = ImageMetadata::default();
    metadata
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
    let mut profile = ColorEncoding::default();
    profile
        .set_srgb(
            if profile_gray {
                ColorSpace::GRAY
            } else {
                ColorSpace::RGB
            },
            RenderingIntent::RELATIVE,
        )
        .unwrap();
    let mut bundle = ImageBundle::new(Some(&metadata));
    let mut old = Image3::create(1, 1).unwrap();
    fill_image3(1.0, &mut old);
    bundle
        .set_from_image(&mut Some(old), &metadata.color_encoding)
        .unwrap();
    if extra {
        let mut ec = Plane::create(ew, eh).unwrap();
        if ew != 0 && eh != 0 {
            fill_image(2.0, &mut ec);
        }
        bundle.extra_channels_mut().push(ec);
    }
    let mut input = Image3::create(w, h).unwrap();
    if w != 0 && h != 0 {
        fill_image3(f32::from_bits(0x7fa12345), &mut input);
    }
    let address = input.plane(0).bytes().map(<[u8]>::as_ptr);
    let mut input = Some(input);
    let status = bundle.set_from_image(&mut input, &profile);
    let transferred = input.is_none();
    if transferred {
        assert_eq!(bundle.color().plane(0).bytes().map(<[u8]>::as_ptr), address);
    }
    let mut out = format!("{} {}", status_code(&status), u32::from(transferred));
    if let Some(input) = input {
        write!(out, " {} {}", input.xsize(), input.ysize()).unwrap();
    }
    write!(
        out,
        " {} {} {}",
        status_code(&bundle.verify_sizes()),
        bundle.xsize(),
        bundle.ysize()
    )
    .unwrap();
    for c in 0..3 {
        for y in 0..bundle.color().ysize() {
            for x in 0..bundle.color().xsize() {
                write!(out, " {}", bundle.color().plane_row(c, y)[x].to_bits()).unwrap();
            }
        }
    }
    metadata::color_bundle(&mut out, bundle.c_current());
    out
}

#[test]
fn image_bundle_set_from_image_transfers_only_after_guards_and_keeps_mutations_before_size_errors()
{
    let mut cases = Vec::new();
    for gray in [0, 1] {
        for profile in [0, 1] {
            for (w, h) in [(0, 0), (0, 1), (1, 0), (1, 1), (3, 2)] {
                for (extra, ew, eh) in [(0, 0, 0), (1, 0, 1), (1, 1, 0), (1, 1, 1), (1, 3, 2)] {
                    cases.push(format!(
                        "rp-set-image {gray} {profile} {w} {h} {extra} {ew} {eh}"
                    ));
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn simple_pipeline_real_image_bundle_write_stage_keeps_color_profile_extra_planes_and_float_bits() {
    let mut cases = Vec::new();
    for (w, h) in [(1, 1), (17, 9), (129, 3)] {
        for channels in [1, 3, 4, 6] {
            for mode in [17, 49] {
                for shifts in [0, 3] {
                    for move_input in [0, 1] {
                        cases.push(format!("rp-simple {w} {h} {channels} {shifts} 1 1 0 1 {move_input} {mode} 0 0 0 {w} {h} 0 0 0"));
                    }
                }
            }
        }
    }
    for mode in [19, 51] {
        cases.push(format!(
            "rp-simple 17 9 4 0 3 1 0 1 1 {mode} 0 0 0 17 9 0 0 0"
        ));
    }
    compare(cases);
}
#[test]
fn simple_pipeline_original_test_upsampling_mirrors_groups_passes_and_write_stage_match_cpp() {
    let mut cases = vec!["rp-default".to_owned()];
    for (w, h) in [(1, 1), (3, 5), (17, 9), (129, 3), (3, 129), (257, 3)] {
        for channels in [1, 3, 4] {
            for shifts in 0..4 {
                for reverse in [0, 1] {
                    for move_input in [0, 1] {
                        cases.push(format!("rp-simple {w} {h} {channels} {shifts} 1 1 0 {reverse} {move_input} 1 0 0 0 {w} {h} 0 0 0"));
                    }
                }
            }
        }
    }
    for (w, h) in [(1, 1), (17, 9), (129, 3)] {
        for mode in [0, 1, 3] {
            for passes in [2, 3] {
                for group_ids in [0, 1] {
                    cases.push(format!("rp-simple {w} {h} 3 0 {passes} 4 {group_ids} 1 1 {mode} 0 0 0 {w} {h} 0 0 0"));
                }
            }
        }
    }
    cases.push("rp-simple 1 1 3 0 256 1 0 0 1 1 0 0 0 1 1 0 0 0".to_owned());
    compare(cases);
}
#[test]
fn simple_pipeline_builder_failures_stage_errors_and_image_dimension_padding_preserve_state() {
    let mut cases = Vec::new();
    for build in 0..4 {
        for shifts in 0..4 {
            for failures in [0, 1, 2, 4, 8, 15, 16] {
                cases.push(format!(
                    "rp-simple 3 5 3 {shifts} 1 1 0 0 1 1 {failures} 2 0 3 5 0 0 {build}"
                ));
            }
        }
    }
    for (w, h) in [(1, 1), (3, 5), (17, 9)] {
        for (iw, ih, x, y) in [
            (21, 19, 0, 0),
            (21, 19, 2, 3),
            (11, 7, -1, -1),
            (13, 11, -2, 2),
        ] {
            if w + x < 0 || h + y < 0 {
                continue;
            }
            for shifts in 0..4 {
                for group_ids in [0, 1] {
                    cases.push(format!("rp-simple {w} {h} 3 {shifts} 1 1 {group_ids} 0 0 1 0 0 1 {iw} {ih} {x} {y} 0"));
                }
            }
        }
    }
    compare(cases);
}
