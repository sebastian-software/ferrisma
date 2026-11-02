// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineStage, Row, RowBuffers, RowStorage,
    WriteToImage3FStage, WriteToImageBundleStage,
};
use jxl::jxl::{
    color_encoding_internal::ColorEncoding,
    frame_header::{BlendMode, FrameOrigin},
    image::{Image3, Plane},
    image_bundle::ImageBundle,
    image_metadata::ImageMetadata,
    jpeg::jpeg_data::JpegData,
    passes_state::{PassesSharedState, SharedDcFrames, SharedReferenceFrames},
};
use std::{cell::RefCell, rc::Rc};
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("ow-")
}
fn value(c: usize, x: usize, y: usize, pass: usize) -> f32 {
    let words = [
        0, 0x80000000, 1, 0x80000001, 0x7f7fffff, 0xff7fffff, 0x7f800000, 0xff800000, 0x7fa12345,
        0xffa54321, 0x7fc98765, 0xffc76543, 0x3f800000,
    ];
    f32::from_bits(words[(c * 7 + x * 3 + y * 11 + pass) % 13])
}
fn fill(image: &mut Image3<f32>, pass: usize) {
    for c in 0..3 {
        for y in 0..image.ysize() {
            for x in 0..image.xsize() {
                image.plane_row_mut(c, y)[x] = value(c, x, y, pass);
            }
        }
    }
}
fn pixels(out: &mut String, image: &Image3<f32>) {
    write!(out, " I {} {}", image.xsize(), image.ysize()).unwrap();
    for c in 0..3 {
        for y in 0..image.ysize() {
            for x in 0..image.xsize() {
                write!(out, " {}", image.plane_row(c, y)[x].to_bits()).unwrap();
            }
        }
    }
}
fn extras(out: &mut String, image: &ImageBundle<'_>) {
    write!(out, " X {}", image.extra_channels().len()).unwrap();
    for ec in image.extra_channels() {
        write!(out, " {} {}", ec.xsize(), ec.ysize()).unwrap();
        for y in 0..ec.ysize() {
            for x in 0..ec.xsize() {
                write!(out, " {}", ec.row(y)[x].to_bits()).unwrap();
            }
        }
    }
}
#[allow(clippy::too_many_arguments)]
fn init(
    b: &mut ImageBundle<'_>,
    metadata: &ImageMetadata,
    w: usize,
    h: usize,
    ec: usize,
    profile: usize,
    name: usize,
    seed: usize,
    jpeg: bool,
) {
    let mut color = Image3::create(w + 2, h + 1).unwrap();
    fill(&mut color, seed);
    color.shrink_to(w, h).unwrap();
    let mut profile_value = if profile % 2 != 0 {
        ColorEncoding::linear_srgb(metadata.color_encoding.is_gray())
    } else {
        ColorEncoding::srgb(metadata.color_encoding.is_gray())
    }
    .clone();
    if profile == 2 {
        profile_value.set_icc_raw(vec![0, 1, 127, 128, 255, 31]);
    }
    b.set_from_image(&mut Some(color), &profile_value).unwrap();
    for e in 0..ec {
        let mut plane = Plane::create(w, h).unwrap();
        for y in 0..h {
            for x in 0..w {
                plane.row_mut(y)[x] = value(e + 3, x, y, seed);
            }
        }
        b.extra_channels_mut().push(plane);
    }
    b.color_transform = jxl::jxl::frame_header::ColorTransform((profile % 3) as u32);
    b.chroma_subsampling
        .set(
            &[1 + (profile % 2) as u8, 1, 1],
            &[1 + u8::from(profile == 2), 1, 1],
        )
        .unwrap();
    b.origin = FrameOrigin {
        x0: -(seed as i32 + 3),
        y0: seed as i32 + 7,
    };
    b.duration = (seed * 313) as u32;
    b.timecode = (seed * 117) as u32;
    b.use_for_next_frame = seed % 2 != 0;
    b.blend = seed % 2 == 0;
    b.blendmode = BlendMode::ADD;
    b.name = (0..name).map(|i| ((i * 37 + seed) % 256) as u8).collect();
    b.set_decoded_bytes(seed * 1931);
    if jpeg {
        b.jpeg_data = Some(Box::new(JpegData {
            width: (w + 11) as i32,
            height: (h + 13) as i32,
            ..JpegData::default()
        }));
    }
}
enum Target<'a> {
    Image(Rc<RefCell<Image3<f32>>>),
    Dc(SharedDcFrames, usize),
    Bundle(Rc<RefCell<ImageBundle<'a>>>),
    Reference(SharedReferenceFrames<'a>, usize),
}
impl<'a> Target<'a> {
    fn image_mut(&self, f: impl FnOnce(&mut Image3<f32>)) {
        match self {
            Self::Image(v) => f(&mut v.borrow_mut()),
            Self::Dc(v, i) => f(&mut v.borrow_mut()[*i]),
            Self::Bundle(v) => f(v.borrow_mut().color_mut()),
            Self::Reference(v, i) => {
                f(v.borrow_mut()[*i].frame.as_deref_mut().unwrap().color_mut())
            }
        }
    }
    fn bundle_mut(&self, f: impl FnOnce(&mut ImageBundle<'a>)) {
        match self {
            Self::Bundle(v) => f(&mut v.borrow_mut()),
            Self::Reference(v, i) => f(v.borrow_mut()[*i].frame.as_deref_mut().unwrap()),
            _ => unreachable!(),
        }
    }
    fn snapshot(&self, out: &mut String, full: bool) {
        match self {
            Self::Image(v) => {
                let v = v.borrow();
                if full {
                    pixels(out, &v)
                } else {
                    write!(out, " I {} {}", v.xsize(), v.ysize()).unwrap();
                }
            }
            Self::Dc(v, i) => {
                let v = v.borrow();
                if full {
                    pixels(out, &v[*i])
                } else {
                    write!(out, " I {} {}", v[*i].xsize(), v[*i].ysize()).unwrap();
                }
            }
            Self::Bundle(v) => {
                let v = v.borrow();
                if full {
                    pixels(out, v.color());
                }
                shared_primitives::observe_bundle(out, &v);
                if full {
                    extras(out, &v);
                }
            }
            Self::Reference(v, i) => {
                let v = v.borrow();
                let b = v[*i].frame.as_deref().unwrap();
                if full {
                    pixels(out, b.color());
                }
                shared_primitives::observe_bundle(out, b);
                if full {
                    extras(out, b);
                }
            }
        }
    }
    fn address(&self) -> usize {
        match self {
            Self::Image(v) => (&*v.borrow() as *const Image3<f32>) as usize,
            Self::Dc(v, i) => (&v.borrow()[*i] as *const Image3<f32>) as usize,
            Self::Bundle(v) => (&*v.borrow() as *const ImageBundle<'_>) as usize,
            Self::Reference(v, i) => {
                (v.borrow()[*i].frame.as_deref().unwrap() as *const ImageBundle<'_>) as usize
            }
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "ow-move" {
        let w = number(args);
        let h = number(args);
        let ec = number(args);
        let profile = number(args);
        let name = number(args);
        let gray = number::<u32>(args) != 0;
        let jpeg = number::<u32>(args) != 0;
        let construct = number::<u32>(args) != 0;
        let metadata = ImageMetadata {
            color_encoding: ColorEncoding::srgb(gray).clone(),
            ..ImageMetadata::default()
        };
        let old = ImageMetadata::default();
        let mut source = ImageBundle::new(Some(&metadata));
        let mut destination = ImageBundle::new(Some(&old));
        init(&mut source, &metadata, w, h, ec, profile, name, 7, jpeg);
        init(&mut destination, &old, 2, 3, 1, 0, 5, 2, false);
        let identity = &destination as *const _;
        let allocation = source.color().plane(0).bytes().unwrap().as_ptr();
        let mut intermediate = ImageBundle::new(None);
        let alias = if construct {
            intermediate.move_from(&mut source);
            destination.move_from(&mut intermediate)
        } else {
            destination.move_from(&mut source)
        };
        let mut alias_pixels = String::new();
        pixels(&mut alias_pixels, alias.color());
        let transferred = alias.color().plane(0).bytes().unwrap().as_ptr() == allocation;
        let mut out = format!(
            "{} {} {} {}",
            u32::from(std::ptr::eq(&destination, identity)),
            u32::from(std::ptr::eq(destination.metadata().unwrap(), &metadata)),
            u32::from(transferred),
            u32::from(transferred)
        );
        shared_primitives::observe_bundle(&mut out, &destination);
        pixels(&mut out, destination.color());
        extras(&mut out, &destination);
        shared_primitives::observe_bundle(&mut out, &source);
        out.push_str(&alias_pixels);
        let ds = destination.color_mut().shrink_to(w + 2, h + 1);
        let ss = source.color_mut().shrink_to(w + 2, h + 1);
        write!(out, " G {} {}", status_code(&ds), status_code(&ss)).unwrap();
        shared_primitives::observe_bundle(&mut out, &destination);
        pixels(&mut out, destination.color());
        shared_primitives::observe_bundle(&mut out, &source);
        destination.remove_color();
        shared_primitives::observe_bundle(&mut out, &destination);
        extras(&mut out, &destination);
        write!(
            out,
            " S {} {} {}",
            destination.xsize(),
            destination.ysize(),
            u32::from(destination.has_color())
        )
        .unwrap();
        return out;
    }
    assert_eq!(kind, "ow-write");
    let kind: usize = number(args);
    let slot = number(args);
    let w = number(args);
    let h = number(args);
    let channels = number(args);
    let guard: usize = number(args);
    let mut metadata = ImageMetadata {
        color_encoding: ColorEncoding::srgb(false).clone(),
        num_extra_channels: (channels - 3) as u32,
        ..ImageMetadata::default()
    };
    metadata
        .extra_channel_info
        .resize_with(channels - 3, Default::default);
    let shared = PassesSharedState::new();
    for r in shared.reference_frames.borrow_mut().iter_mut() {
        *r.frame.as_deref_mut().unwrap() = ImageBundle::new(Some(&metadata));
    }
    let target = match kind {
        0 => Target::Image(Rc::new(RefCell::new(Image3::default()))),
        1 => Target::Dc(shared.dc_frames.clone(), slot),
        2 => Target::Bundle(Rc::new(RefCell::new(ImageBundle::new(Some(&metadata))))),
        _ => Target::Reference(shared.reference_frames.clone(), slot),
    };
    let write_profile = if guard == 6 {
        ColorEncoding::srgb(true)
    } else {
        &metadata.color_encoding
    };
    if kind >= 2 && (guard == 5 || guard == 10) {
        target.bundle_mut(|b| {
            b.extra_channels_mut()
                .push(Plane::create(if guard == 5 { w + 1 } else { w }, h).unwrap())
        });
    }
    let mut stage: Box<dyn RenderPipelineStage> = match &target {
        Target::Image(v) => Box::new(WriteToImage3FStage::from_owned(v.clone())),
        Target::Dc(v, i) => Box::new(WriteToImage3FStage::from_dc_frame(v.clone(), *i)),
        Target::Bundle(v) => Box::new(WriteToImageBundleStage::from_owned(
            v.clone(),
            write_profile,
        )),
        Target::Reference(v, i) => Box::new(WriteToImageBundleStage::from_reference(
            v.clone(),
            *i,
            write_profile,
        )),
    };
    let identity = target.address();
    let mut sizes = vec![(w, h); channels];
    if guard == 3 {
        sizes[1].0 += 1;
    }
    if guard == 4 && channels > 3 {
        sizes.last_mut().unwrap().1 += 1;
    }
    if guard == 9 {
        sizes.truncate(2);
    }
    let status = stage.set_input_sizes(&sizes);
    let mut out = format!("{} {}", status_code(&status), stage.get_name());
    for c in 0..channels + 1 {
        write!(out, " {}", stage.get_channel_mode(c) as i32).unwrap();
    }
    if status.is_err() {
        target.snapshot(&mut out, false);
        return out;
    }
    if w == 0 || h == 0 {
        target.snapshot(&mut out, true);
        return out;
    }
    let target = std::convert::identity(target);
    let _shared = std::convert::identity(shared);
    write!(out, " {}", u32::from(identity == target.address())).unwrap();
    for pass in 0..2 {
        if pass != 0 {
            if kind < 2 {
                target.image_mut(|v| *v = Image3::create(w, h).unwrap());
            } else {
                target.bundle_mut(|b| {
                    let mut replacement = ImageBundle::new(Some(&metadata));
                    init(
                        &mut replacement,
                        &metadata,
                        w,
                        h,
                        channels - 3,
                        1,
                        23,
                        3,
                        false,
                    );
                    b.move_from(&mut replacement);
                });
            }
        }
        if kind >= 2 && guard == 7 && channels > 3 {
            target.bundle_mut(|b| {
                *b.extra_channels_mut().last_mut().unwrap() = Plane::create(w - 1, h).unwrap()
            });
        }
        target.image_mut(|v| fill(v, pass + 5));
        if kind >= 2 {
            target.bundle_mut(|b| {
                for (e, plane) in b.extra_channels_mut().iter_mut().enumerate() {
                    for y in 0..h {
                        for x in 0..plane.xsize() {
                            plane.row_mut(y)[x] = value(e + 3, x, y, pass + 5);
                        }
                    }
                }
            });
        }
        let mut input: Vec<_> = (0..channels)
            .map(|c| {
                let mut plane = Plane::create(OFFSET + w, 1).unwrap();
                for x in 0..w {
                    plane.row_mut(0)[OFFSET + x] = value(c, x, pass, 1);
                }
                plane
            })
            .collect();
        let rows = (0..channels)
            .map(|channel| {
                vec![Row {
                    storage: RowStorage::Input,
                    channel,
                    y: 0,
                    x: 0,
                }]
            })
            .collect();
        let mut x = usize::from(w > 2);
        let mut width = if w > 2 { w - 2 } else { w };
        if guard == 7 {
            x = 0;
            width = w;
        }
        if guard == 8 {
            x = w;
            width = 0;
        }
        let status = stage.process_row(
            &rows,
            &vec![],
            &mut RowBuffers {
                input: &mut input,
                output: &mut [],
            },
            usize::from(guard == 1),
            usize::from(guard == 2),
            width,
            x,
            pass % h,
            0,
        );
        write!(out, " R {}", status_code(&status)).unwrap();
        target.snapshot(&mut out, true);
    }
    out
}
#[test]
fn retained_bundle_moves_remove_color_and_defined_aliases() {
    let mut requests = vec![];
    for (w, h) in [(1, 1), (2, 3), (9, 2), (33, 1)] {
        for ec in [0, 1, 3] {
            for profile in 0..3 {
                for name in [0, 3, 22, 23, 31, 64, 1024] {
                    for gray in [0, 1] {
                        for jpeg in [0, 1] {
                            for construct in [0, 1] {
                                requests.push(format!(
                                "ow-move {w} {h} {ec} {profile} {name} {gray} {jpeg} {construct}"
                            ));
                            }
                        }
                    }
                }
            }
        }
    }
    compare(requests);
}
#[test]
fn retained_write_targets_keep_live_dc_and_reference_identity() {
    let mut requests = vec![];
    for kind in 0..4 {
        for slot in 0..4 {
            for (w, h) in [
                (0, 0),
                (0, 1),
                (1, 0),
                (1, 1),
                (2, 3),
                (3, 2),
                (9, 3),
                (33, 1),
            ] {
                for channels in [3, 4, 6] {
                    for guard in 0..11 {
                        requests.push(format!("ow-write {kind} {slot} {w} {h} {channels} {guard}"));
                    }
                }
            }
        }
    }
    compare(requests);
}
