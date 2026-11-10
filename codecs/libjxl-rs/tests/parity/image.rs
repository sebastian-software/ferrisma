// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observe original/translated storage and field APIs with defined caller data.

use super::*;
use jxl::jxl::base::rect::Rect;
use jxl::jxl::image::{Image3, Plane, Sample};
use jxl::jxl::image_ops::*;
use jxl::jxl::memory_manager_internal::bytes_per_row;
use jxl::jxl::modular::modular_image::{Channel, Image};
use jxl::jxl::modular::options::{ModularOptions, Predictor};
use jxl::jxl::modular::transform::squeeze_params::SqueezeParams;
use jxl::jxl::modular::transform::transform::{
    Transform, TransformId, check_equal_channels, pixel_add,
};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("image-")
}
fn code<T>(result: &Result<T, StatusCode>) -> i32 {
    status_code(&result.as_ref().map(|_| ()).map_err(|&e| e))
}
trait TestSample: Sample {
    fn from_bits(bits: u64) -> Self;
}
macro_rules! samples {
    ($($t:ty),*) => {$(
        impl TestSample for $t {
            fn from_bits(bits:u64)->Self {Self::from_ne_bytes(bits.to_ne_bytes()[..std::mem::size_of::<Self>()].try_into().unwrap())}
        }
    )*};
}
samples!(u8, i16, i32, f32, u64, u16, f64);
pub(super) fn paint<T: Sample>(p: &mut Plane<T>, seed: u64) {
    if let Some(bytes) = p.bytes_mut() {
        for (i, v) in bytes.iter_mut().enumerate() {
            let i = i as u64;
            *v = (i.wrapping_mul(17).wrapping_add(seed.wrapping_mul(13)) ^ (i >> 5)) as u8;
        }
    }
}
fn plane<T: Sample>(out: &mut String, p: &Plane<T>, capacity_rows: usize, full: bool) {
    write!(
        out,
        " {},{},{},{},{},{}",
        p.xsize(),
        p.ysize(),
        p.bytes_per_row(),
        p.pixels_per_row(),
        u32::from(p.bytes().is_some()),
        p.bytes().map_or(0, |b| b.as_ptr() as usize % 128)
    )
    .unwrap();
    let mut samples = Vec::new();
    if let Some(bytes) = p.bytes() {
        if full {
            samples.extend_from_slice(&bytes[..capacity_rows * p.bytes_per_row()]);
        } else {
            for y in 0..p.ysize() {
                let start = y * p.bytes_per_row();
                samples
                    .extend_from_slice(&bytes[start..start + p.xsize() * std::mem::size_of::<T>()]);
            }
        }
    }
    write!(out, " {}", hex(&samples)).unwrap();
}
fn plane_command<T: Sample>(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let x = number(args);
    let y = number(args);
    let pre = number(args);
    let sx = number(args);
    let sy = number(args);
    let ux = number(args);
    let uy = number(args);
    let result = Plane::<T>::create_with_pre_padding(x, y, pre);
    let mut out = code(&result).to_string();
    if let Ok(mut p) = result {
        paint(&mut p, 7);
        plane(&mut out, &p, y, true);
        let s = p.shrink_to(sx, sy);
        write!(out, " {}", code(&s)).unwrap();
        plane(&mut out, &p, y, true);
        let s = p.shrink_to(ux, uy);
        write!(out, " {}", code(&s)).unwrap();
        plane(&mut out, &p, y, true);
        let mut empty = Plane::default();
        p.swap(&mut empty);
        plane(&mut out, &p, 0, true);
        let moved = empty;
        plane(&mut out, &moved, y, true);
    }
    out
}
fn three_command<T: Sample>(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let x = number(args);
    let y = number(args);
    let sx = number(args);
    let sy = number(args);
    let ux = number(args);
    let uy = number(args);
    let result = Image3::<T>::create(x, y);
    let mut out = code(&result).to_string();
    if let Ok(mut p) = result {
        for c in 0..3 {
            paint(p.plane_mut(c), 10 + c as u64);
        }
        let s = p.shrink_to(sx, sy);
        write!(out, " {}", code(&s)).unwrap();
        let s = p.shrink_to(ux, uy);
        write!(out, " {}", code(&s)).unwrap();
        let mut empty = Image3::default();
        p.swap(&mut empty);
        let moved = empty;
        for c in 0..3 {
            plane(&mut out, p.plane(c), 0, true);
            plane(&mut out, moved.plane(c), y, true);
        }
    }
    out
}
fn copy_command<T: TestSample>(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mode: u32 = number(args);
    let w = number(args);
    let h = number(args);
    let tw = number(args);
    let th = number(args);
    let x = number(args);
    let y = number(args);
    let rw = number(args);
    let rh = number(args);
    let tx = number(args);
    let ty = number(args);
    let padding = number(args);
    let value = number(args);
    let a = Plane::<T>::create(w, h);
    let b = Plane::<T>::create(tw, th);
    let mut out = format!("{} {}", code(&a), code(&b));
    if let (Ok(mut from), Ok(mut to)) = (a, b) {
        paint(&mut from, 11);
        paint(&mut to, 19);
        let rf = Rect::new(x, y, rw, rh);
        let rt = Rect::new(tx, ty, rw, rh);
        let status = match mode {
            0 => copy_image_to(&from, &mut to),
            1 => copy_rect_to(rf, &from, rt, &mut to),
            2 => copy_image_to_with_padding(rf, &from, padding, rt, &mut to),
            3 => {
                fill_image(T::from_bits(value), &mut to);
                Ok(())
            }
            4 => {
                zero_fill_image(&mut to);
                Ok(())
            }
            5 => {
                fill_plane(T::from_bits(value), &mut to, rt);
                Ok(())
            }
            _ => panic!("unknown copy request"),
        };
        write!(out, " {}", code(&status)).unwrap();
        plane(&mut out, &from, h, true);
        plane(&mut out, &to, th, true);
    }
    out
}
pub(super) fn transform(out: &mut String, t: &Transform) {
    write!(
        out,
        " {},{},{},{},{},{},{},{},{},{}",
        t.id.0,
        t.begin_c,
        t.rct_type,
        t.num_c,
        t.nb_colors,
        t.nb_deltas,
        t.max_delta_error,
        t.predictor.0,
        u32::from(t.ordered_palette),
        u32::from(t.lossy_palette)
    )
    .unwrap();
    write!(out, " {}", t.squeezes.len()).unwrap();
    for s in &t.squeezes {
        write!(
            out,
            " {},{},{},{}",
            u32::from(s.horizontal),
            u32::from(s.in_place),
            s.begin_c,
            s.num_c
        )
        .unwrap();
    }
}
pub(super) fn channel(out: &mut String, c: &Channel, rows: usize, full: bool) {
    write!(
        out,
        " {},{},{},{},{}",
        c.w, c.h, c.hshift, c.vshift, c.component
    )
    .unwrap();
    plane(out, &c.plane, rows, full);
}
pub(super) fn modular(out: &mut String, image: &Image) {
    write!(
        out,
        " {},{},{},{},{},{}",
        image.w,
        image.h,
        image.bitdepth,
        image.nb_meta_channels,
        u32::from(image.error),
        u32::from(image.empty())
    )
    .unwrap();
    write!(out, " {}", image.channel.len()).unwrap();
    for c in &image.channel {
        channel(out, c, c.plane.ysize(), false);
    }
    write!(out, " {}", image.transform.len()).unwrap();
    for t in &image.transform {
        transform(out, t);
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "image-pad" | "image-pad-partial" => {
            let cw = number(args);
            let ch = number(args);
            let w = number(args);
            let h = number(args);
            let block = number(args);
            let mut image = Image3::<f32>::create(cw, ch).unwrap();
            let partial = kind == "image-pad-partial";
            if partial {
                let mut replacement = Plane::create(20, 12).unwrap();
                image.plane_mut(1).swap(&mut replacement);
            }
            for c in 0..3 {
                paint(image.plane_mut(c), 51 + c as u64);
            }
            let initial = image.shrink_to(w, h);
            let mut out = code(&initial).to_string();
            if initial.is_ok() {
                for _ in 0..2 {
                    let status = pad_image_to_block_multiple_in_place(&mut image, block);
                    write!(out, " {}", code(&status)).unwrap();
                    for c in 0..3 {
                        plane(
                            &mut out,
                            image.plane(c),
                            if partial && c == 1 { 12 } else { ch },
                            true,
                        );
                    }
                    if image.xsize() == 0 || image.ysize() == 0 {
                        break;
                    }
                }
            }
            out
        }
        "image-downsample" => {
            let three = number::<u32>(args) != 0;
            let w = number(args);
            let h = number(args);
            let factor = number(args);
            let mode = number::<u32>(args);
            let mut image = Image3::<f32>::create(
                w + if mode >= 2 { 8 } else { 0 },
                h + if mode >= 2 { 8 } else { 0 },
            )
            .unwrap();
            if mode >= 2 {
                image.shrink_to(w, h).unwrap();
            }
            let values = [
                0, 0x80000000, 1, 0x80000001, 0x3f800000, 0xbf800000, 0x7f7fffff, 0xff7fffff,
                0x7f800000, 0xff800000, 0x7fc12345, 0x7fa23456,
            ];
            for c in 0..3 {
                for y in 0..h {
                    for x in 0..w {
                        image.plane_row_mut(c, y)[x] = if mode.is_multiple_of(2) {
                            ((x * 13 + y * 67 + c * 997) % 4093) as f32 - 2051.0
                        } else {
                            f32::from_bits(values[(x + y * 7 + c * 3) % 12])
                        };
                    }
                }
            }
            let mut out;
            if three {
                let down = downsample_image3(&image, factor);
                out = code(&down).to_string();
                if let Ok(output) = down {
                    for c in 0..3 {
                        plane(&mut out, output.plane(c), output.ysize(), false);
                    }
                }
            } else {
                let down = downsample_image(image.plane(0), factor);
                out = code(&down).to_string();
                if let Ok(output) = down {
                    plane(&mut out, &output, output.ysize(), false);
                }
            }
            for c in 0..3 {
                plane(&mut out, image.plane(c), h, false);
            }
            out
        }
        "image-row-size" => {
            let r = bytes_per_row(number(args), number(args));
            format!("{} {}", code(&r), r.unwrap_or(0))
        }
        "image-plane" => match number::<u32>(args) {
            0 => plane_command::<u8>(args),
            1 => plane_command::<i16>(args),
            2 => plane_command::<i32>(args),
            3 => plane_command::<f32>(args),
            4 => plane_command::<u64>(args),
            5 => plane_command::<u16>(args),
            _ => plane_command::<f64>(args),
        },
        "image-three" => match number::<u32>(args) {
            0 => three_command::<u8>(args),
            1 => three_command::<i16>(args),
            2 => three_command::<i32>(args),
            3 => three_command::<f32>(args),
            4 => three_command::<u64>(args),
            5 => three_command::<u16>(args),
            _ => three_command::<f64>(args),
        },
        "image-copy" => match number::<u32>(args) {
            0 => copy_command::<u8>(args),
            1 => copy_command::<i16>(args),
            2 => copy_command::<i32>(args),
            3 => copy_command::<f32>(args),
            4 => copy_command::<u64>(args),
            5 => copy_command::<u16>(args),
            _ => copy_command::<f64>(args),
        },
        "image-three-partial" | "image-three-stride" => {
            let partial = kind == "image-three-partial";
            let mut p =
                Image3::<i32>::create(if partial { 17 } else { 1 }, if partial { 9 } else { 3 })
                    .unwrap();
            let mut replacement =
                Plane::create(if partial { 3 } else { 256 }, if partial { 2 } else { 3 }).unwrap();
            p.plane_mut(1).swap(&mut replacement);
            for c in 0..3 {
                paint(p.plane_mut(c), 21 + c as u64);
            }
            let s = if partial { p.shrink_to(7, 4) } else { Ok(()) };
            let mut out = code(&s).to_string();
            for c in 0..3 {
                plane(
                    &mut out,
                    p.plane(c),
                    if partial {
                        if c == 1 { 2 } else { 9 }
                    } else {
                        3
                    },
                    true,
                );
            }
            if !partial {
                for c in 0..3 {
                    for y in 0..3 {
                        write!(out, " {}", p.plane_row(c, y)[0]).unwrap();
                    }
                }
            }
            out
        }
        "image-three-ops" => {
            let mut from = Image3::<i32>::create(9, 4).unwrap();
            let mut to = Image3::create(9, 4).unwrap();
            for c in 0..3 {
                paint(from.plane_mut(c), 61 + c as u64);
                paint(to.plane_mut(c), 67 + c as u64);
            }
            let mut out = "P".to_owned();
            for phase in 0..5 {
                let status = match phase {
                    0 => copy_image3_to(&from, &mut to),
                    1 => {
                        fill_image3(1234, &mut to);
                        Ok(())
                    }
                    2 => {
                        zero_fill_image3(&mut to);
                        Ok(())
                    }
                    3 => {
                        copy_rect3_to(Rect::new(1, 1, 3, 2), &from, Rect::new(2, 1, 3, 2), &mut to)
                    }
                    _ => {
                        to.plane_mut(1).shrink_to(1, 1).unwrap();
                        copy_rect3_to(Rect::new(0, 0, 9, 4), &from, Rect::new(0, 0, 9, 4), &mut to)
                    }
                };
                write!(out, " {}", code(&status)).unwrap();
                for c in 0..3 {
                    plane(&mut out, to.plane(c), 4, true);
                }
            }
            out
        }
        "image-channel" => {
            let w = number(args);
            let h = number(args);
            let hs = number(args);
            let vs = number(args);
            let nw = number(args);
            let nh = number(args);
            let r = Channel::create_with_shifts(w, h, hs, vs);
            let mut out = code(&r).to_string();
            if let Ok(mut c) = r {
                paint(&mut c.plane, 31);
                channel(&mut out, &c, h, true);
                let changed = c.plane.xsize() != nw as usize || c.plane.ysize() != nh as usize;
                let status = c.shrink_to(nw, nh);
                let mut rows = h;
                if status.is_ok() && changed {
                    rows = c.plane.ysize();
                    paint(&mut c.plane, 37);
                }
                write!(out, " {}", code(&status)).unwrap();
                channel(&mut out, &c, rows, true);
            }
            out
        }
        "image-modular" => {
            let w = number(args);
            let h = number(args);
            let depth = number(args);
            let n = number(args);
            let meta = number(args);
            let variant: u32 = number(args);
            let r = Image::create(w, h, depth, n);
            let mut out = code(&r).to_string();
            if let Ok(mut image) = r {
                image.nb_meta_channels = meta;
                for c in &mut image.channel {
                    paint(&mut c.plane, 41 + c.component as u64);
                }
                if image.channel.len() > 1 {
                    if variant == 1 {
                        image.channel[1].w = 0;
                    }
                    if variant == 2 {
                        image.channel[1].hshift = 3;
                    }
                }
                let mut t = Transform::new(TransformId::RCT);
                t.max_delta_error = 77;
                t.squeezes = vec![SqueezeParams {
                    horizontal: true,
                    ..SqueezeParams::default()
                }];
                image.transform.push(t);
                modular(&mut out, &image);
                for (c1, c2) in [(0, 0), (0, 1), (1, 2), (0, 3), (2, 1), (u32::MAX, u32::MAX)] {
                    write!(out, " {}", code(&check_equal_channels(&image, c1, c2))).unwrap();
                }
                let clone = Image::clone_image(&image);
                write!(out, " {}", code(&clone)).unwrap();
                if let Ok(clone) = clone {
                    modular(&mut out, &clone);
                }
            }
            out
        }
        "image-transform" | "image-transform-read" => {
            let mut t = Transform::new(TransformId::RCT);
            t.max_delta_error = 77;
            t.ordered_palette = false;
            t.lossy_palette = true;
            if kind == "image-transform-read" {
                t.begin_c = 21;
                t.rct_type = 17;
                t.num_c = 7;
                t.nb_colors = 99;
                t.nb_deltas = 8;
                t.predictor = Predictor::SELECT;
                t.squeezes = vec![SqueezeParams::default(); 2];
                let offset = number(args);
                let bytes = unhex(args.next().unwrap());
                let mut br = BitReader::new(&bytes);
                br.skip_bits(offset);
                let read = bundle::read(&mut br, &mut t);
                let mut out = format!(
                    "{} {} {}",
                    code(&read),
                    br.total_bits_consumed(),
                    code(&br.close())
                );
                transform(&mut out, &t);
                out
            } else {
                t.id = TransformId(number(args));
                t.begin_c = number(args);
                t.rct_type = number(args);
                t.num_c = number(args);
                t.nb_colors = number(args);
                t.nb_deltas = number(args);
                t.predictor = Predictor(number(args));
                let count = number(args);
                let offset = number(args);
                t.squeezes = vec![SqueezeParams::default(); count];
                for (i, s) in t.squeezes.iter_mut().enumerate() {
                    s.horizontal = i % 2 != 0;
                    s.in_place = (i / 2) % 2 != 0;
                    s.begin_c = (i % 3) as u32;
                    s.num_c = 2;
                }
                let can = bundle::can_encode(&mut t);
                let (ext, bits) = can.unwrap_or((0, 0));
                let mut out = format!("{} {ext} {bits}", code(&can));
                if can.is_ok() {
                    let mut writer = BitWriter::new();
                    prefix(&mut writer, offset);
                    let written = bundle::write(&mut t, &mut writer, LayerType::Header, None);
                    write!(out, " {} {}", code(&written), writer.bits_written()).unwrap();
                    pad(&mut writer);
                    write!(out, " {}", hex(writer.get_span())).unwrap();
                    let mut decoded = Transform {
                        max_delta_error: 77,
                        ..Transform::default()
                    };
                    let mut br = BitReader::new(writer.get_span());
                    br.skip_bits(offset);
                    let read = bundle::read(&mut br, &mut decoded);
                    write!(
                        out,
                        " {} {} {}",
                        code(&read),
                        br.total_bits_consumed(),
                        code(&br.close())
                    )
                    .unwrap();
                    transform(&mut out, &decoded);
                }
                transform(&mut out, &t);
                out
            }
        }
        "image-options" => {
            let a = number(args);
            let b = number(args);
            let mut out = pixel_add(a, b).to_string();
            let o = ModularOptions::default();
            write!(
                out,
                " {},{},{},{},{},{},{},{},{},{},{},{},{}",
                o.max_chan_size,
                o.group_dim,
                o.nb_repeats.to_bits(),
                o.max_properties,
                o.splitting_heuristics_node_threshold.to_bits(),
                o.max_property_values,
                o.predictor.0,
                o.wp_mode,
                o.fast_decode_multiplier.to_bits(),
                o.wp_tree_mode as u32,
                u32::from(o.skip_encoder_fast_path),
                o.tree_kind as u32,
                u32::from(o.zero_tokens)
            )
            .unwrap();
            write!(out, " {}", o.splitting_heuristics_properties.len()).unwrap();
            for v in &o.splitting_heuristics_properties {
                write!(out, " {v}").unwrap();
            }
            let p = &o.histogram_params;
            write!(
                out,
                " {},{},{},{},{},{},{},{},{},{}",
                p.clustering as u32,
                p.uint_method as u32,
                p.lz77_method as u32,
                p.ans_histogram_strategy as u32,
                p.max_histograms,
                u32::from(p.force_huffman),
                u32::from(p.initialize_global_state),
                u32::from(p.streaming_mode),
                u32::from(p.add_missing_symbols),
                u32::from(p.add_fixed_histograms)
            )
            .unwrap();
            write!(out, " {}", p.image_widths.len()).unwrap();
            for v in &p.image_widths {
                write!(out, " {v}").unwrap();
            }
            out
        }
        "image-mirror" => mirror(number(args), number(args)).to_string(),
        _ => panic!("unknown image request"),
    }
}

#[test]
fn block_padding_preserves_float_bits_capacity_bytes_and_partial_plane_sizes() {
    let mut cases = Vec::new();
    for (w, h) in [(1, 1), (2, 3), (7, 9), (8, 8), (9, 17), (17, 9), (31, 3)] {
        for block in [1, 2, 3, 8, 16, 32, usize::MAX] {
            for spare in [false, true] {
                let (cw, ch) = if spare { (64, 64) } else { (w, h) };
                cases.push(format!("image-pad {cw} {ch} {w} {h} {block}"));
            }
        }
    }
    cases.push("image-pad-partial 32 16 17 9 8".to_owned());
    compare(cases);
}

#[test]
fn downsampling_matches_float_accumulation_edges_special_values_and_unsigned_rounding() {
    let mut cases = Vec::new();
    for three in [0, 1] {
        for (w, h) in [
            (1, 1),
            (2, 3),
            (7, 9),
            (8, 8),
            (9, 17),
            (17, 9),
            (31, 3),
            (0, 0),
            (17, 0),
            (0, 5),
        ] {
            for factor in [1, 2, 3, 4, 8, 16, usize::MAX - 1, usize::MAX] {
                for mode in [0, 1, 2, 3] {
                    cases.push(format!("image-downsample {three} {w} {h} {factor} {mode}"));
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn row_sizing_matches_scalar_vector_padding_alias_avoidance_and_overflow() {
    let mut cases = Vec::new();
    for width in [
        0,
        1,
        2,
        3,
        15,
        16,
        31,
        32,
        63,
        64,
        127,
        128,
        255,
        256,
        511,
        512,
        1023,
        1024,
        2047,
        2048,
        4096,
        u32::MAX as usize,
        usize::MAX / 4,
        usize::MAX - 128,
        usize::MAX,
    ] {
        for size in [0, 1, 2, 3, 4, 8, 16, usize::MAX] {
            cases.push(format!("image-row-size {width} {size}"));
        }
    }
    compare(cases);
}
#[test]
fn plane_and_three_plane_storage_match_shrink_restore_swap_move_and_limits() {
    let mut cases = Vec::new();
    for kind in 0..7 {
        for x in [0, 1, 15, 31, 32, 127, 128, 255, 511, 512, 513] {
            for y in [0, 1, 3] {
                for pre in [0, 1, 2048, 2049, usize::MAX] {
                    cases.push(format!(
                        "image-plane {kind} {x} {y} {pre} {} 0 {x} {y}",
                        x / 2
                    ));
                    cases.push(format!(
                        "image-plane {kind} {x} {y} {pre} {} {y} {x} {}",
                        x + 1,
                        y + 1
                    ));
                }
                cases.push(format!("image-three {kind} {x} {y} {} 0 {x} {y}", x / 2));
            }
        }
    }
    for kind in 0..7 {
        for (x, y) in [
            (u32::MAX as usize, 0),
            (0, u32::MAX as usize),
            (u32::MAX as usize + 1, 0),
            (0, u32::MAX as usize + 1),
            (u32::MAX as usize, u32::MAX as usize),
        ] {
            cases.push(format!("image-plane {kind} {x} {y} 0 0 0 {x} {y}"));
        }
    }
    cases.push("image-three-partial".into());
    cases.push("image-three-stride".into());
    cases.push("image-three-ops".into());
    compare(cases);
}
#[test]
fn copying_and_filling_preserve_valid_samples_padding_and_border_mirroring() {
    let mut cases = Vec::new();
    let mut seed = 295513;
    for kind in [0, 1, 2, 3, 5] {
        for trial in 0..128 {
            let w = next(&mut seed) % 24 + 1;
            let h = next(&mut seed) % 8 + 1;
            let tw = next(&mut seed) % 24 + 1;
            let th = next(&mut seed) % 8 + 1;
            let x = next(&mut seed) % 28;
            let y = next(&mut seed) % 10;
            let rw = next(&mut seed) % 12;
            let rh = next(&mut seed) % 7;
            let tx = next(&mut seed) % 28;
            let ty = next(&mut seed) % 10;
            for mode in 0..5 {
                cases.push(format!(
                    "image-copy {kind} {mode} {w} {h} {tw} {th} {x} {y} {rw} {rh} {tx} {ty} {} {}",
                    trial % 4,
                    next(&mut seed)
                ));
            }
            cases.push(format!(
                "image-copy {kind} 0 {w} {h} {w} {h} 0 0 {w} {h} 0 0 0 0"
            ));
            cases.push(format!(
                "image-copy {kind} 5 {w} {h} {tw} {th} 0 0 {} {} 0 0 0 {}",
                tw / 2,
                th / 2,
                next(&mut seed)
            ));
        }
    }
    for size in 1..32 {
        for x in -128..128 {
            cases.push(format!("image-mirror {x} {size}"));
        }
    }
    compare(cases);
}
#[test]
fn modular_channels_clone_defaults_and_failed_reallocation_match_original() {
    let mut cases = Vec::new();
    for (w, h) in [(0, 0), (0, 3), (17, 0), (1, 1), (17, 3)] {
        for (nw, nh) in [(0, 0), (0, 3), (1, 1), (17, 3), (7, 2), (-1, 3), (17, -1)] {
            cases.push(format!("image-channel {w} {h} -1 2 {nw} {nh}"));
        }
        for n in [-1, 0, 1, 3] {
            for meta in [0, 1, 3] {
                for variant in 0..3 {
                    cases.push(format!("image-modular {w} {h} 16 {n} {meta} {variant}"));
                }
            }
        }
    }
    cases.push(format!("image-modular {} 0 -1 0 0 0", usize::MAX));
    for a in [i32::MIN, -1, 0, 1, i32::MAX] {
        for b in [i32::MIN, -1, 0, 1, i32::MAX] {
            cases.push(format!("image-options {a} {b}"));
        }
    }
    compare(cases);
}
#[test]
fn modular_transform_fields_match_all_ids_counts_limits_and_reused_state() {
    let mut cases = Vec::new();
    let mut seed = 453619;
    for id in 0..4 {
        for rct in [0, 1, 6, 17, 41, 42, 73] {
            for pred in [0, 4, 6, 13, 14, 15] {
                for count in [0, 1, 17, 296, 297] {
                    cases.push(format!(
                        "image-transform {id} 9 {rct} 3 256 0 {pred} {count} {}",
                        (rct + pred) % 8
                    ));
                }
            }
        }
    }
    for begin in [0, 7, 8, 71, 72, 1095, 1096, 9287, 9288, u32::MAX] {
        for (num, colors, deltas) in [
            (1, 0, 0),
            (4, 255, 256),
            (8192, 70911, 66816),
            (8193, 70912, 66817),
        ] {
            cases.push(format!(
                "image-transform 1 {begin} 6 {num} {colors} {deltas} 13 0 7"
            ));
        }
    }
    for len in 0..48 {
        for trial in 0..16 {
            let bytes: Vec<_> = (0..len).map(|_| next(&mut seed) as u8).collect();
            cases.push(format!(
                "image-transform-read {} {}",
                trial % 8,
                hex(&bytes)
            ));
        }
    }
    compare(cases);
}
