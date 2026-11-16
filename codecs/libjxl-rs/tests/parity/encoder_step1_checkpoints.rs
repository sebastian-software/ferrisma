// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Encoder step 1 (#118): checkpoint fixtures of LossyFrameHeuristics and
// the rest of ComputeVarDCTEncodingData. The oracle's `es1-ckpt` request
// (tools/oracle/encoder_step1_checkpoints.cc) runs the original frame
// encoder for one public encoder history and records the encoder state after
// the original LossyFrameHeuristics, InitializePassesEncoder and
// ComputeARHeuristics; the port runs the same history through its public
// encoder with a checkpoint observer (`jxl::jxl::enc_checkpoints`) at the
// same calls. Every float compares as its exact word (no NaN occurs in these
// finite inputs; ADR-0041's NaN rule would apply), every integer exactly. The
// oracle run's codestream must equal the original public encoder's output for
// the history, which ties the checkpoint run to the public path.
use super::*;
use jxl::jxl::ac_context::BlockCtxMap;
use jxl::jxl::ac_strategy::AcStrategyImage;
use jxl::jxl::dct_util::AcImage;
use jxl::jxl::enc_cache::PassesEncoderState;
use jxl::jxl::enc_checkpoints::{Checkpoint, set_checkpoint_observer};
use jxl::jxl::frame_header::FrameHeader;
use jxl::jxl::image::{Image3F, ImageF, Plane};
use jxl::jxl::quantizer::Quantizer;
use std::cell::RefCell;
use std::rc::Rc;

// The public encoder driver of encoder_public.rs, private to that module;
// a second instance keeps that shared file unchanged.
#[allow(
    clippy::duplicate_mod,
    reason = "A second instance of the private driver module of encoder_public.rs"
)]
#[path = "../support/encoder_public.rs"]
mod driver;

fn plane(out: &mut String, plane: &ImageF) {
    write!(out, " P {} {}", plane.xsize(), plane.ysize()).unwrap();
    for y in 0..plane.ysize() {
        for v in &plane.row(y)[..plane.xsize()] {
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
}

fn ints<T: jxl::jxl::image::Sample + Into<i64>>(out: &mut String, plane: &Plane<T>) {
    write!(out, " I {} {}", plane.xsize(), plane.ysize()).unwrap();
    for y in 0..plane.ysize() {
        for &v in &plane.row(y)[..plane.xsize()] {
            write!(out, " {}", v.into()).unwrap();
        }
    }
}

fn strategy(out: &mut String, acs: &AcStrategyImage) {
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

fn quantizer(out: &mut String, q: &Quantizer) {
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

fn block_ctx_map(out: &mut String, map: &BlockCtxMap) {
    write!(out, " K {} {}", map.num_ctxs, map.num_dc_ctxs).unwrap();
    for t in &map.dc_thresholds {
        write!(out, " {}", t.len()).unwrap();
        for v in t {
            write!(out, " {v}").unwrap();
        }
    }
    write!(out, " {}", map.qf_thresholds.len()).unwrap();
    for v in &map.qf_thresholds {
        write!(out, " {v}").unwrap();
    }
    write!(out, " {}", map.ctx_map.len()).unwrap();
    for v in &map.ctx_map {
        write!(out, " {v}").unwrap();
    }
}

/// The oracle's rendering of one checkpoint.
fn checkpoint(
    out: &mut String,
    checkpoint: Checkpoint,
    header: &FrameHeader<'_>,
    state: &PassesEncoderState<'_>,
    image: &Image3F,
) {
    let shared = &state.shared;
    match checkpoint {
        Checkpoint::LossyFrameHeuristics => {
            write!(
                out,
                " H 0 F {} {} {} {}",
                u8::from(header.loop_filter.gab),
                header.loop_filter.epf_iters,
                header.x_qm_scale,
                header.b_qm_scale
            )
            .unwrap();
            for c in 0..3 {
                plane(out, image.plane(c));
            }
            plane(out, &state.initial_quant_masking1x1);
            strategy(out, &shared.ac_strategy);
            ints(out, &shared.raw_quant_field);
            quantizer(out, &shared.quantizer);
            {
                let base = shared.cmap.base();
                write!(
                    out,
                    " B {} {} {} {} {} {}",
                    base.y_to_x_dc(),
                    base.y_to_b_dc(),
                    base.raw_color_factor(),
                    base.color_scale().to_bits(),
                    base.base_correlation_x().to_bits(),
                    base.base_correlation_b().to_bits()
                )
                .unwrap();
                for v in base.dc_factors() {
                    write!(out, " {}", v.to_bits()).unwrap();
                }
            }
            ints(out, &shared.cmap.ytox_map);
            ints(out, &shared.cmap.ytob_map);
            ints(out, &shared.epf_sharpness);
            block_ctx_map(out, &shared.block_ctx_map);
            out.push_str(" M");
            for e in shared.matrices.encodings() {
                write!(out, " {},{}", e.mode() as i32, e.predefined).unwrap();
            }
            for c in 0..3 {
                write!(out, " {}", shared.matrices.dc_quant(c).to_bits()).unwrap();
            }
            write!(
                out,
                " X {} {}",
                u8::from(shared.image_features.splines.borrow().has_any()),
                u8::from(shared.image_features.patches.borrow().has_any())
            )
            .unwrap();
        }
        Checkpoint::InitializePassesEncoder => {
            out.push_str(" I 0");
            for c in 0..3 {
                plane(out, shared.dc_storage.plane(c));
            }
            ints(out, &shared.quant_dc);
            // One row per group; the first 64 words per block are written.
            let frame_dim = &shared.frame_dim;
            write!(out, " C {}", state.coeffs.len()).unwrap();
            for coeffs in &state.coeffs {
                let Some(AcImage::I32(image)) = coeffs.as_deref() else {
                    panic!("the encoder's coefficients are 32-bit");
                };
                for g in 0..frame_dim.num_groups {
                    let r = frame_dim.block_group_rect(g);
                    let n = r.xsize() * r.ysize() * 64;
                    write!(out, " G {g} {n}").unwrap();
                    for c in 0..3 {
                        for v in &image.plane(c).row(g)[..n] {
                            write!(out, " {v}").unwrap();
                        }
                    }
                }
            }
        }
        Checkpoint::ArHeuristics => {
            out.push_str(" A 0");
            ints(out, &shared.epf_sharpness);
        }
    }
}

/// One checkpoint history: float32 samples of `content`, enum sRGB or linear
/// sRGB, one frame, CloseInput and enough output.
struct Case {
    xsize: usize,
    ysize: usize,
    gray: bool,
    linear: bool,
    content: u32,
    distance: f32,
    intensity: f32,
}

impl Case {
    fn request(&self) -> String {
        format!(
            "es1-ckpt {} {} {} {} {} {} {}",
            self.xsize,
            self.ysize,
            u8::from(self.gray),
            u8::from(self.linear),
            self.content,
            self.distance.to_bits(),
            self.intensity.to_bits()
        )
    }

    /// The same frame as a public encoder history (tests/parity/encoder_public.rs
    /// protocol): bits 32, exponent 8, the color encoding, the distance, an
    /// interleaved native float32 frame with the generated content.
    fn history(&self) -> String {
        let channels = if self.gray { 1 } else { 3 };
        let mut info = format!("Bw:{},h:{},bits:32,exp:8", self.xsize, self.ysize);
        if self.gray {
            info.push_str(",c:1");
        }
        if self.intensity != 0.0 {
            write!(info, ",it:{}", self.intensity.to_bits()).unwrap();
        }
        let color = if self.linear { 'g' } else { 'G' };
        let size = self.xsize * self.ysize * channels * 4;
        // ProcessOutput with 0, 1, 7 and 64 bytes, then doubling.
        let mut schedule = String::from("p0 p1 p7");
        let (mut sum, mut avail) = (8usize, 64usize);
        while sum <= size + 2048 {
            write!(schedule, " p{avail}").unwrap();
            sum += avail;
            avail *= 2;
        }
        format!(
            "epub-run 0 0 {info} {color}{} s- d0,{} a0,{channels},0,0,0,{size},{},{},{} Z {schedule}",
            u8::from(self.gray),
            self.distance.to_bits(),
            self.content,
            self.xsize,
            self.ysize
        )
    }
}

/// The codestream that one epub-run observation's ProcessOutput calls wrote.
fn public_bytes(observation: &str) -> String {
    let mut bytes = String::new();
    let mut last = "";
    for field in observation.split(' ') {
        let Some(result) = field.strip_prefix("p:") else {
            continue;
        };
        let parts: Vec<&str> = result.split(',').collect();
        let written: usize = parts[1].parse().unwrap();
        let hex = parts[3].split('/').next().unwrap();
        bytes.push_str(&hex[..2 * written]);
        last = parts[0];
    }
    assert_eq!(last, "0", "the encoder finishes");
    bytes
}

fn compare_checkpoints(family: &str, cases: &[Case]) {
    let requests: Vec<String> = cases.iter().map(Case::request).collect();
    let histories: Vec<String> = cases.iter().map(Case::history).collect();
    let originals = super::original_observations(&requests);
    let public = super::original_observations(&histories);
    let mut words = 0usize;
    for ((request, history), (cpp, public)) in requests
        .iter()
        .zip(&histories)
        .zip(originals.iter().zip(&public))
    {
        let (checkpoints, bytes) = cpp.rsplit_once(" W ").expect("checkpoint bytes");
        // The checkpoint run is the original public encoder's frame encode.
        assert_eq!(
            bytes,
            public_bytes(public),
            "{family}: oracle run equals the public encode: {request}"
        );
        let text = Rc::new(RefCell::new(String::from("E 0")));
        let sink = Rc::clone(&text);
        let previous = set_checkpoint_observer(Some(Box::new(move |c, h, s, i| {
            checkpoint(&mut sink.borrow_mut(), c, h, s, i);
        })));
        assert!(previous.is_none());
        let mut args = history.split_whitespace();
        assert_eq!(args.next(), Some("epub-run"));
        let rust = driver::observe(&mut args);
        set_checkpoint_observer(None);
        assert!(
            rust.admission.is_none(),
            "{family}: admitted history: {history}"
        );
        assert_eq!(&rust.text, public, "{family}: public history: {history}");
        let rust_checkpoints = text.borrow().clone();
        // Locate the first difference for the failure message.
        if rust_checkpoints != checkpoints {
            let r: Vec<&str> = rust_checkpoints.split(' ').collect();
            let c: Vec<&str> = checkpoints.split(' ').collect();
            let at = r
                .iter()
                .zip(&c)
                .position(|(a, b)| a != b)
                .unwrap_or(r.len().min(c.len()));
            panic!(
                "{family}: checkpoint word {at} differs (Rust {:?}, C++ {:?}); context {:?}: {request}",
                r.get(at),
                c.get(at),
                &c[at.saturating_sub(12)..at]
            );
        }
        words += checkpoints.split(' ').count();
    }
    println!(
        "{family}: {} C++/Rust checkpoint histories matched exactly ({words} words)",
        cases.len()
    );
}

/// The step 1 gate sizes in gray and RGB, every generated content, sRGB and
/// linear sRGB, the gate distances d1.0, d0.5, d1.5, d2.9 (AR heuristics from
/// d0.7) and both intensity targets.
#[test]
fn lossy_frame_heuristics_checkpoints_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let mut sizes = vec![(1, 1), (7, 7), (8, 8), (9, 17)];
    sizes.extend((41..=48).map(|x| (x, 15)));
    sizes.extend([(63, 129), (256, 256), (257, 255)]);
    let distances = [1.0f32, 0.5, 1.5, 2.9];
    let mut cases = Vec::new();
    let mut variant = 0usize;
    for &(xsize, ysize) in &sizes {
        for gray in [false, true] {
            cases.push(Case {
                xsize,
                ysize,
                gray,
                linear: variant % 3 == 1,
                content: 1 + (variant % 4) as u32,
                distance: distances[variant % 4],
                intensity: if variant % 5 == 2 { 100.0 } else { 0.0 },
            });
            variant += 1;
        }
    }
    // Eight groups at d1.0 and d2.9.
    for (gray, distance) in [(false, 1.0f32), (true, 2.9)] {
        cases.push(Case {
            xsize: 512,
            ysize: 1024,
            gray,
            linear: false,
            content: 4,
            distance,
            intensity: 0.0,
        });
    }
    compare_checkpoints("es1-checkpoints", &cases);
}
