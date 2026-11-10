// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::cms_interface::{IccProfileSpan, JxlCmsInterface, JxlColorProfile};
use jxl::jxl::color_encoding_internal::{ColorEncoding, ColorSpaceTransform};
use jxl::jxl::dec_xyb::OutputEncodingInfo;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::render_pipeline::stage_cms::get_cms_stage;
use std::ffi::c_void;

#[derive(Default)]
struct Trace {
    destroyed: usize,
    profiles: Vec<(Vec<u8>, Vec<u8>)>,
}
struct Context {
    trace: *mut Trace,
    src: IccProfileSpan,
    dst: IccProfileSpan,
}
unsafe extern "C" fn init(
    data: *mut c_void,
    _threads: usize,
    _width: usize,
    src: *const JxlColorProfile,
    dst: *const JxlColorProfile,
    _intensity: f32,
) -> *mut c_void {
    // SAFETY: The real adapter passes live fully initialized profile structs.
    // Retain only their ICC spans, whose owners outlive the destroy callback.
    let (src, dst) = unsafe { (&*src, &*dst) };
    Box::into_raw(Box::new(Context {
        trace: data.cast(),
        src: src.icc,
        dst: dst.icc,
    }))
    .cast()
}
unsafe extern "C" fn destroy(data: *mut c_void) {
    // SAFETY: init allocated this unique context and the actual owner transfers
    // it to destroy exactly once. The boxed Trace outlives transform drop.
    let context = unsafe { Box::from_raw(data.cast::<Context>()) };
    // SAFETY: ICC spans still refer to initialized adapter-owned bytes here;
    // both lengths are positive in this test. No competing trace access exists.
    let (src, dst, trace) = unsafe {
        (
            std::slice::from_raw_parts(context.src.data, context.src.size).to_vec(),
            std::slice::from_raw_parts(context.dst.data, context.dst.size).to_vec(),
            &mut *context.trace,
        )
    };
    trace.destroyed += 1;
    trace.profiles.push((src, dst));
}

#[test]
fn owned_icc_caches_survive_moves_and_are_live_during_single_context_destroy() {
    let mut trace = Box::<Trace>::default();
    let interface = JxlCmsInterface {
        init_data: (&raw mut *trace).cast(),
        init: Some(init),
        destroy: Some(destroy),
        ..JxlCmsInterface::default()
    };
    let mut src = ColorEncoding::srgb(false).clone();
    src.set_icc_raw(vec![9, 8, 7]);
    let mut dst = ColorEncoding::linear_srgb(false).clone();
    dst.set_icc_raw(vec![4, 5]);
    let mut transform = ColorSpaceTransform::new(&interface);
    // SAFETY: The callbacks/context obey the interface and Trace remains alive
    // until transform destruction. This test never invokes buffer/run methods.
    unsafe { transform.init(&src, &dst, 255.0, 17, 1) }.unwrap();
    drop(src);
    drop(dst);
    let moved = Box::new(transform);
    let moved = *moved;
    drop(moved);
    assert_eq!(trace.destroyed, 1);
    assert_eq!(trace.profiles, [(vec![9, 8, 7], vec![4, 5])]);
}

#[test]
fn fixed_lossless_profiles_skip_cms_without_requiring_callback_implementations() {
    for gray in [false, true] {
        for depth in [LosslessSampleDepth::Uint8, LosslessSampleDepth::Uint16] {
            let config = LosslessThunderConfig::new(17, 9, gray, depth).unwrap();
            let mut info = OutputEncodingInfo::default();
            info.set_from_metadata(config.metadata()).unwrap();
            assert!(!info.cms_set);
            for linear in [false, true] {
                // SAFETY: The private lossless profile has cms_set=false. The
                // original IsNeeded return releases an unprepared stage without
                // calling any interface pointer, so no foreign lifetime is needed.
                assert!(unsafe { get_cms_stage(&info, linear) }.is_none());
            }
        }
    }
}
