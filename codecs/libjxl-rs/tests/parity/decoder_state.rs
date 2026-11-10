// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::dec_ans::ANSCode;
use jxl::jxl::dec_cache::{DecoderShared, ImageOutput, PassesDecoderState, PixelCallback};
use jxl::jxl::dec_patch_dictionary::ReferenceFrameBinding;
use jxl::jxl::frame_header::{FLAG_USE_DC_FRAME, FrameEncoding, FrameHeader};
use jxl::jxl::image::{Image3, Plane};
use jxl::jxl::image_metadata::{CodecMetadata, Orientation};
use jxl::jxl::passes_state::{DcSource, PassesSharedState, initialize_passes_shared_state};
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use std::ffi::c_void;
use std::sync::atomic::Ordering;
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("dcs-")
}
fn orders(out: &mut String, v: &[u32]) {
    write!(out, " V {}", v.len()).unwrap();
    let mut i = 0;
    while i < v.len() {
        let mut end = i + 1;
        while end < v.len() && v[end] == v[i] {
            end += 1;
        }
        write!(out, " {},{}", v[i], end - i).unwrap();
        i = end;
    }
    out.push_str(" ;");
}
fn shared(out: &mut String, s: &PassesSharedState<'_>, quant_pixels: bool) {
    write!(
        out,
        " S {} {} {} {}",
        u32::from(s.metadata.is_some()),
        u32::from(matches!(
            s.image_features.patches.borrow().reference_frame_binding(),
            Some(ReferenceFrameBinding::SharedState)
        )),
        s.coeff_order_size,
        s.num_histograms
    )
    .unwrap();
    frame::dimensions(out, &s.frame_dim);
    shared_primitives::plane(out, s.ac_strategy.layers());
    shared_primitives::plane(out, &s.raw_quant_field);
    shared_primitives::plane(out, &s.epf_sharpness);
    shared_primitives::plane(out, &s.cmap.ytox_map);
    shared_primitives::plane(out, &s.cmap.ytob_map);
    if s.cmap.ytox_map.xsize() != 0 {
        shared_primitives::corr(out, &s.cmap.base());
    }
    shared_primitives::plane(out, &s.quant_dc);
    if quant_pixels {
        for y in 0..s.quant_dc.ysize() {
            for x in 0..s.quant_dc.xsize() {
                write!(out, " {}", s.quant_dc.row(y)[x]).unwrap();
            }
        }
    }
    orders(out, &s.coeff_orders);
    let dc = match s.dc_source {
        DcSource::Storage => -1,
        DcSource::Frame(i) => i as i32,
    };
    write!(out, " DC {dc}").unwrap();
    for c in 0..3 {
        shared_primitives::plane(out, s.dc_storage.plane(c));
    }
    for f in s.dc_frames.borrow().iter() {
        for c in 0..3 {
            shared_primitives::plane(out, f.plane(c));
        }
    }
}
#[derive(Default)]
struct CallbackState {
    calls: usize,
    threads: usize,
    pixels: usize,
}
unsafe extern "C" fn init(p: *mut c_void, threads: usize, pixels: usize) -> *mut c_void {
    if p.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: Every fixture caller passes an exclusive live CallbackState or
    // null, and the callback is invoked synchronously before it is observed.
    let state = unsafe { &mut *p.cast::<CallbackState>() };
    state.calls += 1;
    state.threads = threads;
    state.pixels = pixels;
    p
}
unsafe extern "C" fn run(_: *mut c_void, _: usize, _: usize, _: usize, _: usize, _: *const c_void) {
}
unsafe extern "C" fn destroy(_: *mut c_void) {}
fn output(out: &mut String, o: &ImageOutput) {
    write!(
        out,
        " O {} {} {} {} {} {} {} {} {} {} {} {}",
        o.format.num_channels,
        o.format.data_type.0,
        o.format.endianness.0,
        o.format.align,
        o.bits_per_sample,
        u32::from(o.callback.init.is_some()),
        u32::from(o.callback.run.is_some()),
        u32::from(o.callback.destroy.is_some()),
        u32::from(!o.callback.init_opaque.is_null()),
        u32::from(!o.buffer.is_null()),
        o.buffer_size,
        o.stride
    )
    .unwrap();
}
fn state(out: &mut String, s: &PassesDecoderState<'_, '_>) {
    write!(
        out,
        " D {} {} {} {} {}",
        u32::from(s.have_memory_manager()),
        u32::from(matches!(s.shared, DecoderShared::Own)),
        u32::from(s.upsampler8x.is_some()),
        s.code.len(),
        s.context_map.len()
    )
    .unwrap();
    for map in &s.context_map {
        write!(out, " {}", map.len()).unwrap();
        for v in map {
            write!(out, " {v}").unwrap();
        }
    }
    write!(
        out,
        " {} {}",
        s.x_dm_multiplier.to_bits(),
        s.b_dm_multiplier.to_bits()
    )
    .unwrap();
    shared_primitives::plane(out, &s.sigma.borrow());
    write!(out, " {} {}", s.width, s.height).unwrap();
    output(out, &s.main_output);
    write!(out, " {}", s.extra_output.len()).unwrap();
    for o in &s.extra_output {
        output(out, o);
    }
    write!(
        out,
        " {} {} {} {} {} {} {}",
        u32::from(s.fast_xyb_srgb8_conversion),
        u32::from(s.unpremul_alpha),
        s.undo_orientation.0,
        s.visible_frame_index,
        s.nonvisible_frame_index,
        s.used_acs.load(Ordering::SeqCst),
        u32::from(s.coefficients.is_some())
    )
    .unwrap();
    if let Some(c) = &s.coefficients {
        write!(
            out,
            " {} {} {}",
            c.image_type() as i32,
            u32::from(c.is_empty()),
            c.pixels_per_row()
        )
        .unwrap();
    }
    write!(out, " {}", u32::from(s.render_pipeline.is_some())).unwrap();
    shared_primitives::observe_bundle(out, &s.frame_storage_for_referencing.borrow());
    output_encoding::observation(out, &s.output_encoding_info);
    write!(out, " C {}", s.shared_storage.coeff_order_size).unwrap();
    orders(out, &s.shared_storage.coeff_orders);
    if let Some(stage) = &s.upsampler8x {
        let t = stage.settings();
        write!(
            out,
            " U {} {} {} {} {}",
            t.shift_x,
            t.shift_y,
            t.border_x,
            t.border_y,
            stage.get_name()
        )
        .unwrap();
        for c in 0..4 {
            write!(out, " {}", stage.get_channel_mode(c) as i32).unwrap();
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "dcs-callback" {
        let mask = number::<u32>(args);
        let opaque = number::<u32>(args) != 0;
        let threads = number(args);
        let pixels = number(args);
        let mut s = CallbackState::default();
        let p = if opaque {
            (&raw mut s).cast()
        } else {
            std::ptr::null_mut()
        };
        let cb = PixelCallback::new(
            if mask & 1 != 0 { Some(init) } else { None },
            if mask & 2 != 0 { Some(run) } else { None },
            if mask & 4 != 0 { Some(destroy) } else { None },
            p,
        );
        let mut result = std::ptr::null_mut();
        if mask & 1 != 0 {
            // SAFETY: init is present and receives the exclusive live fixture
            // state or its supported null value for this synchronous call.
            result = unsafe { cb.init(threads, pixels) };
        }
        return format!(
            "{} {} {} {} {} {} {} {} {}",
            u32::from(cb.is_present()),
            u32::from(cb.init.is_some()),
            u32::from(cb.run.is_some()),
            u32::from(cb.destroy.is_some()),
            u32::from(!cb.init_opaque.is_null()),
            u32::from(result == p),
            s.calls,
            s.threads,
            s.pixels
        );
    }
    if kind == "dcs-shared" {
        let n = number::<usize>(args);
        let mask = number::<u32>(args);
        let seed = number::<usize>(args);
        let actions: Vec<_> = (0..n)
            .map(|_| {
                (
                    number::<usize>(args),
                    number::<usize>(args),
                    number::<u32>(args) != 0,
                    number::<u32>(args) != 0,
                    number::<u64>(args),
                    number::<u32>(args),
                    number::<u32>(args),
                    number::<u32>(args) != 0,
                )
            })
            .collect();
        let metadata: Vec<_> = actions
            .iter()
            .map(|a| {
                let mut m = CodecMetadata::default();
                m.size.set(a.0, a.1).unwrap();
                m.m.xyb_encoded = false;
                m
            })
            .collect();
        let mut s = PassesSharedState::new();
        s.frame_dim.set(1, 1, 0, 0, 0, true, 1);
        s.coeff_orders = (0..seed).map(|i| (i * 17 + 3) as u32).collect();
        for i in 0..4 {
            if mask & (1 << i) != 0 {
                s.dc_frames.borrow_mut()[i] = Image3::create(i + 1, 2).unwrap();
            }
        }
        let mut out = String::new();
        for (i, &(_, _, modular, encoder, flags, level, passes, present)) in
            actions.iter().enumerate()
        {
            let mut header = FrameHeader::new(Some(&metadata[i]));
            header.encoding = if modular {
                FrameEncoding::MODULAR
            } else {
                FrameEncoding::VAR_DCT
            };
            header.flags = flags;
            header.dc_level = level;
            header.passes.num_passes = passes;
            if !present {
                header.nonserialized_metadata = None;
            }
            let status = initialize_passes_shared_state(&header, &mut s, encoder);
            write!(out, " R {}", status_code(&status)).unwrap();
            shared(
                &mut out,
                &s,
                present && !encoder && flags & FLAG_USE_DC_FRAME != 0 && status.is_ok(),
            );
        }
        return out;
    }
    assert_eq!(kind, "dcs-state");
    let w = number(args);
    let h = number(args);
    let external = number::<u32>(args) != 0;
    let xqm = number(args);
    let bqm = number(args);
    let epf = number(args);
    let repeats = number::<usize>(args);
    let n = number::<usize>(args);
    let mut metadata = CodecMetadata::default();
    metadata.size.set(w, h).unwrap();
    let mut ext_metadata = CodecMetadata::default();
    ext_metadata.size.set(65, 33).unwrap();
    let mut header = FrameHeader::new(Some(&metadata));
    let ext_header = FrameHeader::new(Some(&ext_metadata));
    header.x_qm_scale = xqm;
    header.b_qm_scale = bqm;
    header.loop_filter.epf_iters = epf;
    let mut ext = PassesSharedState::new();
    ext.metadata = Some(&ext_metadata);
    ext.frame_dim = ext_header.to_frame_dimensions();
    let mut s = PassesDecoderState::new();
    s.shared_storage.metadata = Some(&metadata);
    s.shared_storage.frame_dim = header.to_frame_dimensions();
    if external {
        s.shared = DecoderShared::External(&ext);
    }
    let mut callback = CallbackState::default();
    let p = (&raw mut callback).cast();
    s.width = 21;
    s.height = 13;
    s.main_output = ImageOutput {
        format: JxlPixelFormat {
            num_channels: 4,
            data_type: JxlDataType::UINT16,
            endianness: JxlEndianness::BIG,
            align: 16,
        },
        bits_per_sample: 12,
        callback: PixelCallback::new(Some(init), Some(run), Some(destroy), p),
        buffer: p,
        buffer_size: 999,
        stride: 77,
    };
    s.extra_output = vec![s.main_output; 2];
    s.fast_xyb_srgb8_conversion = true;
    s.unpremul_alpha = true;
    s.undo_orientation = Orientation::ROTATE_90;
    s.visible_frame_index = 9;
    s.nonvisible_frame_index = 13;
    s.used_acs.store(0x7ffffff, Ordering::SeqCst);
    s.shared_storage.coeff_order_size = 123;
    s.shared_storage.coeff_orders = vec![3, 5, 7];
    s.code = (0..2).map(|_| ANSCode::default()).collect();
    s.context_map = vec![vec![1, 2], vec![3]];
    *s.sigma.borrow_mut() = Plane::create(3, 2).unwrap();
    let mut out = String::new();
    state(&mut out, &s);
    for _ in 0..repeats {
        let status = s.init(&header);
        write!(out, " R {}", status_code(&status)).unwrap();
        state(&mut out, &s);
    }
    for _ in 0..n {
        let mask = number(args);
        let passes = number(args);
        s.used_acs.store(mask, Ordering::SeqCst);
        let status = s.init_for_ac(passes);
        write!(out, " A {}", status_code(&status)).unwrap();
        state(&mut out, &s);
    }
    out
}
#[test]
fn shared_initialization_retains_dc_selection_allocation_prefixes_and_coefficient_orders() {
    let mut cases = Vec::new();
    for (w, h) in [(1, 1), (17, 9), (257, 3), (65, 67)] {
        for modular in [0, 1] {
            for encoder in [0, 1] {
                for mask in [0, 5, 15] {
                    for level in 0..=4 {
                        let action = format!(
                            "{w} {h} {modular} {encoder} {} {level} 3 1",
                            FLAG_USE_DC_FRAME
                        );
                        cases.push(format!("dcs-shared 4 {mask} 7 17 9 1 0 0 0 1 1 {action} 3 2 0 1 0 0 2 0 33 17 0 1 0 0 2 1"));
                    }
                }
            }
        }
    }
    cases.push("dcs-shared 1 0 0 17 9 0 1 0 0 1 0".into());
    compare(cases);
}
#[test]
fn decoder_initialization_resets_only_original_output_fields_and_sizes_coefficients_from_atomic_mask()
 {
    let mut cases = Vec::new();
    for (w, h) in [(1, 1), (17, 9), (257, 3)] {
        for external in [0, 1] {
            for epf in [0, 1, 3] {
                for repeats in [0, 1, 2] {
                    cases.push(format!("dcs-state {w} {h} {external} 2 3 {epf} {repeats} 6 0 1 1 1 64 2 67108864 1 134217727 2 0 0"));
                }
            }
        }
    }
    for x in [0, 1, 2, 3, 7, 15, 32, 255, u32::MAX] {
        for b in [0, 2, 7, 255, u32::MAX] {
            cases.push(format!("dcs-state 3 2 0 {x} {b} 0 1 0"));
        }
    }
    for strategy in 0..27 {
        for passes in [1, 2] {
            cases.push(format!(
                "dcs-state 3 2 0 2 2 0 1 1 {} {passes}",
                1u32 << strategy
            ));
        }
    }
    for (mask, size) in [
        (1, 192),
        (1 << 26, jxl::jxl::coeff_order::COEFF_ORDER_MAX_SIZE),
    ] {
        let passes = usize::MAX / size + 1;
        cases.push(format!("dcs-state 3 2 0 2 2 0 1 1 {mask} {passes}"));
    }
    compare(cases);
}
#[test]
fn pixel_callback_carriers_and_initialization_forward_exact_original_presence_and_arguments() {
    let mut cases = Vec::new();
    for mask in 0..8 {
        for opaque in [0, 1] {
            for (threads, pixels) in [(0, 0), (1, 17), (7, 1024), (usize::MAX, usize::MAX)] {
                cases.push(format!("dcs-callback {mask} {opaque} {threads} {pixels}"));
            }
        }
    }
    compare(cases);
}
