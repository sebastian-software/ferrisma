// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Caller CMS probe: trace the real adapter/stage without a CMS substitution.
use super::*;
use jxl::jxl::cms_interface::{JxlCmsInterface, JxlColorProfile};
use jxl::jxl::color_encoding::{ColorSpace, JxlColorEncoding, TransferFunction};
use jxl::jxl::color_encoding_internal::{ColorEncoding, ColorSpaceTransform};
use jxl::jxl::dec_xyb::OutputEncodingInfo;
use jxl::jxl::image::Plane;
use jxl::jxl::render_pipeline::render_pipeline_stage::{Row, RowBuffers, RowStorage};
use jxl::jxl::render_pipeline::stage_cms::get_cms_stage;
use std::cell::{Cell, RefCell};
use std::ffi::{c_int, c_void};
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("cst-")
}

struct Buffer {
    data: Vec<f32>,
    pointer: *mut f32,
}
impl Buffer {
    fn new(len: usize, value: f32) -> Self {
        let mut data = vec![value; len.max(1)];
        let pointer = data.as_mut_ptr();
        Self { data, pointer }
    }
    // No slice references to the allocation are formed after as_mut_ptr. Its
    // Vec owner never resizes; callbacks/snapshots use bounded raw accesses.
    fn words(&self) -> Vec<u32> {
        (0..self.data.len())
            .map(|i| {
                // SAFETY: This initialized allocation is still owned here, never
                // resized, and no callback is running while snapshots are collected.
                unsafe { self.pointer.add(i).read().to_bits() }
            })
            .collect()
    }
}
struct Context {
    owner: *const Probe,
    id: usize,
    threads: usize,
    width: usize,
    src_channels: usize,
    dst_channels: usize,
    src_profile: JxlColorProfile,
    dst_profile: JxlColorProfile,
    destroyed: Cell<bool>,
    src: Vec<Buffer>,
    dst: Vec<Buffer>,
}
struct Probe {
    trace: RefCell<String>,
    #[expect(
        clippy::vec_box,
        reason = "Foreign context handles must stay at stable addresses when the registry grows."
    )]
    contexts: RefCell<Vec<Box<Context>>>,
    parse_external: RefCell<JxlColorEncoding>,
    parse_result: Cell<c_int>,
    parse_cmyk: Cell<c_int>,
    fail_init: c_int,
    fail_run: c_int,
    alias: bool,
    calls: Cell<usize>,
}
impl Probe {
    fn new(fail_init: c_int, fail_run: c_int, alias: bool) -> Box<Self> {
        Box::new(Self {
            trace: RefCell::new(String::new()),
            contexts: RefCell::new(Vec::new()),
            parse_external: RefCell::new(JxlColorEncoding::default()),
            parse_result: Cell::new(1),
            parse_cmyk: Cell::new(0),
            fail_init,
            fail_run,
            alias,
            calls: Cell::new(0),
        })
    }
    fn interface(&self) -> JxlCmsInterface {
        let data = (self as *const Self).cast_mut().cast();
        JxlCmsInterface {
            set_fields_data: data,
            set_fields_from_icc: Some(fields),
            init_data: data,
            init: Some(init),
            get_src_buf: Some(src),
            get_dst_buf: Some(dst),
            run: Some(run),
            destroy: Some(destroy),
        }
    }
}
fn external(c: &JxlColorEncoding) -> String {
    cms::external_args(c).replace(' ', ",")
}
unsafe fn icc(profile: &JxlColorProfile) -> String {
    if profile.icc.size == 0 {
        return "-".into();
    }
    // SAFETY: Every probe caller passes a profile borrowed from live adapter
    // caches, either during init or before destroy releases those caches.
    hex(unsafe { std::slice::from_raw_parts(profile.icc.data, profile.icc.size) })
}
unsafe fn profile(p: &JxlColorProfile) -> String {
    // SAFETY: Forward the live-profile-cache contract documented by icc.
    format!(
        " {} {} {} {}",
        p.num_channels,
        u32::from(!p.icc.data.is_null()),
        unsafe { icc(p) },
        external(&p.color_encoding)
    )
}
unsafe extern "C" fn fields(
    data: *mut c_void,
    bytes: *const u8,
    size: usize,
    output: *mut JxlColorEncoding,
    cmyk: *mut c_int,
) -> c_int {
    // SAFETY: The interface is created from a stable boxed Probe which outlives
    // every synchronous call. Interior cells keep callback mutations exclusive.
    let p = unsafe { &*data.cast::<Probe>() };
    let contents = if size == 0 {
        "-".into()
    } else {
        // SAFETY: The actual ICC setter provides this initialized byte range
        // for the callback's duration; the probe never retains its pointer.
        hex(unsafe { std::slice::from_raw_parts(bytes, size) })
    };
    write!(
        p.trace.borrow_mut(),
        " P {} {} {contents}",
        p.parse_result.get(),
        p.parse_cmyk.get()
    )
    .unwrap();
    if p.parse_result.get() != 0 {
        // SAFETY: Both output pointers are writable locals owned by the actual
        // setter. A successful callback initializes both complete C values.
        unsafe {
            output.write(p.parse_external.borrow().clone());
            cmyk.write(p.parse_cmyk.get());
        }
    }
    p.parse_result.get()
}
unsafe extern "C" fn init(
    data: *mut c_void,
    threads: usize,
    width: usize,
    input: *const JxlColorProfile,
    output: *const JxlColorProfile,
    intensity: f32,
) -> *mut c_void {
    // SAFETY: The actual adapter passes live initialized profile structs and a
    // stable Probe pointer. Only the ICC spans, not stack profile addresses,
    // are retained; adapter caches keep those spans alive until destroy.
    let (p, input, output) = unsafe { (&*data.cast::<Probe>(), &*input, &*output) };
    p.calls.set(p.calls.get() + 1);
    // SAFETY: Input/output ICC caches are live during this init call.
    write!(
        p.trace.borrow_mut(),
        " I {threads} {width} {}{}{}",
        intensity.to_bits(),
        unsafe { profile(input) },
        unsafe { profile(output) }
    )
    .unwrap();
    if p.fail_init == 1
        || (p.fail_init == 2 && p.calls.get() == 1)
        || (p.fail_init == 3 && p.calls.get() == 2)
    {
        p.trace.borrow_mut().push_str(" F");
        return std::ptr::null_mut();
    }
    let id = p.contexts.borrow().len();
    let context = Box::new(Context {
        owner: p,
        id,
        threads,
        width,
        src_channels: input.num_channels,
        dst_channels: output.num_channels,
        src_profile: input.clone(),
        dst_profile: output.clone(),
        destroyed: Cell::new(false),
        src: (0..threads)
            .map(|_| Buffer::new(width * input.num_channels, -11.0))
            .collect(),
        dst: (0..threads)
            .map(|_| Buffer::new(width * output.num_channels, -13.0))
            .collect(),
    });
    let result = (&raw const *context).cast_mut().cast();
    write!(p.trace.borrow_mut(), " C {id}").unwrap();
    p.contexts.borrow_mut().push(context);
    result
}
unsafe extern "C" fn src(data: *mut c_void, thread: usize) -> *mut f32 {
    // SAFETY: Data is a live registered Context and callers use the init bound.
    let c = unsafe { &*data.cast::<Context>() };
    // SAFETY: The stable owner outlives every registered context.
    let p = unsafe { &*c.owner };
    write!(p.trace.borrow_mut(), " S {} {thread}", c.id).unwrap();
    c.src[thread].pointer
}
unsafe extern "C" fn dst(data: *mut c_void, thread: usize) -> *mut f32 {
    // SAFETY: Data is a live registered Context and callers use the init bound.
    let c = unsafe { &*data.cast::<Context>() };
    // SAFETY: The stable owner outlives every registered context.
    let p = unsafe { &*c.owner };
    write!(p.trace.borrow_mut(), " D {} {thread}", c.id).unwrap();
    if p.alias && c.src_channels > c.dst_channels {
        c.src[thread].pointer
    } else {
        c.dst[thread].pointer
    }
}
unsafe extern "C" fn run(
    data: *mut c_void,
    thread: usize,
    input: *const f32,
    output: *mut f32,
    width: usize,
) -> c_int {
    // SAFETY: Successful probe init installed this context; the real adapter or
    // direct fixture supplies initialized, bounded buffers and a valid thread.
    let c = unsafe { &*data.cast::<Context>() };
    // SAFETY: The stable owner outlives every registered context.
    let p = unsafe { &*c.owner };
    write!(
        p.trace.borrow_mut(),
        " R {} {thread} {width} {}",
        c.id,
        u32::from(input == output)
    )
    .unwrap();
    for x in 0..width {
        for ch in 0..c.dst_channels {
            // SAFETY: Both offsets are in the initialized callback buffers. When
            // pointers alias, channel reduction writes behind future input pixels.
            let value = unsafe {
                input
                    .add(x * c.src_channels + ch % c.src_channels)
                    .read()
                    .to_bits()
            } ^ 0x80000000;
            // SAFETY: This output offset is in the owned destination range; the
            // same-pointer reduction case is intentionally handled without slices.
            unsafe {
                output
                    .add(x * c.dst_channels + ch)
                    .write(f32::from_bits(value));
            }
        }
    }
    if p.fail_run != 0 { 0 } else { 1 }
}
unsafe extern "C" fn destroy(data: *mut c_void) {
    // SAFETY: The adapter releases the current live registered context once,
    // before its ICC caches are freed. Registry storage outlives this call.
    let c = unsafe { &*data.cast::<Context>() };
    c.destroyed.set(true);
    // SAFETY: The stable Probe owner and both ICC caches are still alive.
    let p = unsafe { &*c.owner };
    write!(
        p.trace.borrow_mut(),
        " K {} {} {}",
        c.id,
        unsafe { icc(&c.src_profile) },
        unsafe { icc(&c.dst_profile) }
    )
    .unwrap();
}
fn color(p: &Probe, kind: u32) -> ColorEncoding {
    if kind == 9 {
        return ColorEncoding::default();
    }
    let gray = matches!(kind, 2 | 3 | 6);
    let mut c = if matches!(kind, 1 | 3) {
        ColorEncoding::linear_srgb(gray)
    } else {
        ColorEncoding::srgb(gray)
    }
    .clone();
    if kind == 4 {
        p.parse_external.replace(c.to_external());
        p.parse_cmyk.set(1);
        let mut bytes = vec![0xa5, 0xc4, 7];
        // SAFETY: The stable Probe and valid complete-output callback live for
        // this synchronous ICC assignment and retain no input references.
        unsafe { c.set_icc(&mut bytes, Some(&p.interface())) }.unwrap();
    }
    if matches!(kind, 5 | 6) {
        c.set_icc_raw(vec![9, 8, 7]);
    }
    if kind == 7 {
        c.set_color_space(ColorSpace::UNKNOWN);
    }
    if kind == 8 {
        c.tf_mut().set_gamma(0.45).unwrap();
    }
    c
}
fn dump(out: &mut String, p: &Probe) {
    write!(out, " X {}", p.contexts.borrow().len()).unwrap();
    for c in p.contexts.borrow().iter() {
        write!(
            out,
            " {} {} {} {} {} {}",
            c.id,
            c.threads,
            c.width,
            c.src_channels,
            c.dst_channels,
            u32::from(c.destroyed.get())
        )
        .unwrap();
        for all in [&c.src, &c.dst] {
            for row in all {
                write!(out, " {}", row.data.len()).unwrap();
                for v in row.words() {
                    write!(out, " {v}").unwrap();
                }
            }
        }
    }
    write!(out, " L{} ;", p.trace.borrow()).unwrap();
}
#[expect(
    clippy::field_reassign_with_default,
    reason = "Retain the source fixture's aggregate seeds and field-assignment checkpoints."
)]
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "cst-icc" {
        let k = number(args);
        let direct = number::<u32>(args) != 0;
        let has = number::<u32>(args) != 0;
        let result = number(args);
        let cmyk = number(args);
        let mut bytes = unhex(args.next().unwrap());
        let external = cms::color_encoding(args);
        let p = Probe::new(0, 0, false);
        let mut c = color(&p, k);
        p.parse_external.replace(external);
        p.parse_result.set(result);
        p.parse_cmyk.set(cmyk);
        let interface = p.interface();
        let original = bytes.as_ptr();
        let mut out;
        if direct {
            let mut storage = c.view().clone();
            // SAFETY: The live Probe supplies the complete ICC callback
            // contract; it retains no input bytes or output-local pointers.
            let status = unsafe { storage.set_fields_from_icc(&mut bytes, &interface) };
            let moved =
                bytes.is_empty() && !storage.icc.is_empty() && storage.icc.as_ptr() == original;
            out = format!(
                "{} {} {}",
                status_code(&status),
                u32::from(moved),
                hex(&bytes)
            );
            metadata::color_storage(&mut out, &storage);
        } else {
            // SAFETY: The same synchronous callback contract holds when Some;
            // None selects the original guard before any foreign invocation.
            let status = unsafe { c.set_icc(&mut bytes, has.then_some(&interface)) };
            let moved = bytes.is_empty() && !c.icc().is_empty() && c.icc().as_ptr() == original;
            out = format!(
                "{} {} {}",
                status_code(&status),
                u32::from(moved),
                hex(&bytes)
            );
            metadata::color_bundle(&mut out, &c);
        }
        dump(&mut out, &p);
        return out;
    }
    if kind == "cst-flow" {
        let sk = number(args);
        let dk = number(args);
        let width = number::<usize>(args);
        let threads = number::<usize>(args);
        let repeats = number::<usize>(args);
        let fi = number(args);
        let fr = number(args);
        let alias = number::<u32>(args) != 0;
        let p = Probe::new(fi, fr, alias);
        let source = color(&p, sk);
        let destination = color(&p, dk);
        let mut transform = ColorSpaceTransform::new(&p.interface());
        let mut out = String::new();
        for i in 0..repeats {
            // SAFETY: Probe/callbacks outlive the transform, profiles are valid,
            // and every call obeys init/run bounds. A repeated init matches
            // source context overwrite; abandoned contexts are never accessed.
            let status = unsafe { transform.init(&source, &destination, 255.0, width, threads) };
            write!(
                out,
                " I {} {} {} {}",
                status_code(&status),
                u32::from(transform.has_context()),
                hex(transform.icc_src()),
                hex(transform.icc_dst())
            )
            .unwrap();
            let moved = Box::new(transform);
            transform = *moved;
            if status.is_ok() {
                for thread in 0..threads {
                    for count in [width, width.saturating_sub(1), 0] {
                        // SAFETY: Successful init installed a live context and this
                        // thread is below its bound. Probe buffers remain allocated.
                        let (src, dst) =
                            unsafe { (transform.buf_src(thread), transform.buf_dst(thread)) };
                        let channels = if source.is_cmyk() {
                            4
                        } else {
                            source.channels()
                        };
                        for x in 0..count * channels {
                            // SAFETY: Count is within the initialized width and the
                            // selected source-channel count matches the CMS profile.
                            unsafe {
                                src.add(x).write(render_pipeline::value(0, x, thread, i, 3));
                            }
                        }
                        // SAFETY: Input is initialized, output capacity and permitted
                        // same-pointer reduction are supplied by this probe context.
                        let run = unsafe { transform.run(thread, src, dst, count) };
                        write!(out, " R {}", status_code(&run)).unwrap();
                    }
                }
            }
        }
        drop(transform);
        dump(&mut out, &p);
        return out;
    }
    assert_eq!(kind, "cst-stage");
    let sk = number(args);
    let dk = number(args);
    let enabled = number::<u32>(args) != 0;
    let linear = number::<u32>(args) != 0;
    let width = number::<usize>(args);
    let threads = number::<usize>(args);
    let repeats = number::<usize>(args);
    let mutation = number::<u32>(args);
    let guard = number::<u32>(args);
    let mode = number(args);
    let fi = number(args);
    let fr = number(args);
    let alias = number::<u32>(args) != 0;
    let p = Probe::new(fi, fr, alias);
    let source = color(&p, sk);
    let destination = color(&p, dk);
    let mut info = OutputEncodingInfo::default();
    info.orig_color_encoding = source.clone();
    info.linear_color_encoding = source;
    info.linear_color_encoding
        .tf_mut()
        .set_transfer_function(TransferFunction::LINEAR);
    info.color_encoding = destination;
    info.desired_intensity_target = 255.0;
    info.cms_set = enabled;
    info.color_management_system = p.interface();
    // SAFETY: The boxed Probe is stable and outlives stage destruction. Its
    // callbacks satisfy buffer, initialization, alias, and non-unwinding rules;
    // the fixture sets input sizes before preparing and uses valid thread ids.
    let mut stage = unsafe { get_cms_stage(&info, linear) };
    let mut out = format!(" H {}", u32::from(stage.is_some()));
    if let Some(stage) = &mut stage {
        write!(out, " {}", stage.get_name()).unwrap();
        for c in 0..5 {
            write!(out, " {}", stage.get_channel_mode(c) as i32).unwrap();
        }
        let mut sizes = vec![(width, 1); 4];
        write!(out, " Z {}", status_code(&stage.set_input_sizes(&sizes))).unwrap();
        if mutation != 0 {
            match mutation {
                1 => sizes.truncate(2),
                2 => sizes[1].0 += 1,
                3 => sizes[1].1 += 1,
                4 => {
                    for s in &mut sizes {
                        s.0 = width.saturating_sub(1);
                    }
                }
                _ => {}
            }
            write!(out, " Z {}", status_code(&stage.set_input_sizes(&sizes))).unwrap();
        }
        let mut planes: Vec<_> = (0..4)
            .map(|c| {
                let mut plane = Plane::create(64, 1).unwrap();
                for x in 0..64 {
                    plane.row_mut(0)[x] = render_pipeline::value(c, x, 0, 0, mode);
                }
                plane
            })
            .collect();
        let rows: Vec<_> = (0..4)
            .map(|channel| {
                vec![Row {
                    storage: RowStorage::Input,
                    channel,
                    y: 0,
                    x: 0,
                }]
            })
            .collect();
        for _ in 0..repeats {
            let prep = stage.prepare_for_threads(threads);
            write!(out, " T {}", status_code(&prep)).unwrap();
            if prep.is_ok() {
                for thread in 0..threads {
                    let count = match guard {
                        3 => width + 1,
                        4 => 0,
                        _ => width,
                    };
                    let mut buffers = RowBuffers {
                        input: &mut planes,
                        output: &mut [],
                    };
                    let status = stage.process_row(
                        &rows,
                        &Vec::new(),
                        &mut buffers,
                        usize::from(guard == 1),
                        usize::from(guard == 2),
                        count,
                        0,
                        0,
                        thread,
                    );
                    write!(out, " R {}", status_code(&status)).unwrap();
                }
            }
        }
        for plane in &planes {
            out.push_str(" P");
            for x in 0..64 {
                write!(out, " {}", plane.row(0)[x].to_bits()).unwrap();
            }
        }
    }
    drop(stage);
    dump(&mut out, &p);
    out
}

#[test]
fn cms_icc_callbacks_keep_input_ownership_guards_and_partial_color_fields() {
    let mut profiles = vec![cms::srgb()];
    let mut bad = cms::srgb();
    bad.gamma = -1.0;
    bad.transfer_function = TransferFunction::GAMMA;
    profiles.push(bad);
    let mut bad = cms::srgb();
    bad.white_point = jxl::jxl::color_encoding::WhitePoint::CUSTOM;
    bad.white_point_xy = [1.0, 0.0];
    profiles.push(bad);
    let mut cases = Vec::new();
    for k in [0, 4, 5, 8, 9] {
        for direct in [0, 1] {
            for has in [0, 1] {
                for result in [0, 1, 2, -1] {
                    for cmyk in [0, 1, -3] {
                        for bytes in ["-", "010203"] {
                            for external in &profiles {
                                cases.push(format!(
                                    "cst-icc {k} {direct} {has} {result} {cmyk} {bytes} {}",
                                    cms::external_args(external)
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn original_cms_transform_profiles_buffers_moves_failures_and_destroy_match_callback_trace() {
    let mut cases = Vec::new();
    for sk in 0..10 {
        for dk in [0, 1, 2, 4, 5, 9] {
            for (width, threads) in [(0, 0), (0, 1), (3, 2), (17, 1)] {
                for alias in [0, 1] {
                    cases.push(format!(
                        "cst-flow {sk} {dk} {width} {threads} 1 0 0 {alias}"
                    ));
                }
            }
        }
    }
    for fi in 0..4 {
        for fr in [0, 1] {
            cases.push(format!("cst-flow 4 1 3 2 2 {fi} {fr} 1"));
        }
    }
    cases.push("cst-flow 9 9 0 0 0 0 0 0".into());
    compare(cases);
}
#[test]
fn original_cms_stage_selection_interleaving_size_guards_and_callback_lifecycle_match() {
    let mut cases = Vec::new();
    for sk in 0..10 {
        for dk in 0..10 {
            for enabled in [0, 1] {
                for linear in [0, 1] {
                    for alias in [0, 1] {
                        cases.push(format!(
                            "cst-stage {sk} {dk} {enabled} {linear} 3 2 1 0 0 3 0 0 {alias}"
                        ));
                    }
                }
            }
        }
    }
    for (sk, dk) in [(0, 1), (2, 3), (4, 1)] {
        for width in [0, 1, 17] {
            for mutation in 0..5 {
                for guard in 0..5 {
                    cases.push(format!(
                        "cst-stage {sk} {dk} 1 0 {width} 1 1 {mutation} {guard} 1 0 0 1"
                    ));
                }
            }
        }
    }
    for fi in 0..4 {
        for fr in [0, 1] {
            cases.push(format!("cst-stage 0 1 1 0 3 2 2 0 0 1 {fi} {fr} 0"));
        }
    }
    compare(cases);
}
