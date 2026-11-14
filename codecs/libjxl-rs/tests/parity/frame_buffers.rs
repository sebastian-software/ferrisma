// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::base::rect::Rect;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::encode_internal::{JxlEncoderChunkedFrameAdapter, bits_per_channel, bytes_per_pixel};
use jxl::jxl::image::Image3;
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("fb-")
}
fn format(args: &mut std::str::SplitWhitespace<'_>) -> JxlPixelFormat {
    JxlPixelFormat {
        num_channels: number(args),
        data_type: JxlDataType(number(args)),
        endianness: JxlEndianness(number(args)),
        align: number(args),
    }
}
fn state(out: &mut String, a: &JxlEncoderChunkedFrameAdapter<'_>) {
    write!(out, " S {} {} {}", a.xsize(), a.ysize(), a.channels().len()).unwrap();
    for (i, c) in a.channels().iter().enumerate() {
        let f = if i == 0 {
            a.get_color_channels_pixel_format()
        } else {
            a.get_extra_channel_pixel_format(i - 1)
        };
        write!(
            out,
            " {} {} {} {} {} {} {} {} {} {} {} {}",
            f.num_channels,
            f.data_type.0,
            f.endianness.0,
            f.align,
            c.xsize(),
            c.ysize(),
            c.bytes_per_pixel(),
            c.stride(),
            c.buffer_size(),
            u32::from(c.borrowed_buffer().is_some()),
            hex(c.copied_buffer()),
            hex(c
                .borrowed_buffer()
                .map_or(&[], |p| &p[..c.buffer_size().min(8)]))
        )
        .unwrap();
        let mut row = 123;
        let b = if i == 0 {
            a.get_color_channel_data_at(0, 0, 0, 0, &mut row)
        } else {
            a.get_extra_channel_data_at(i - 1, 0, 0, 0, 0, &mut row)
        };
        let available = if c.copied_buffer().is_empty() {
            c.buffer_size()
        } else {
            c.copied_buffer().len()
        };
        write!(
            out,
            " {} {} {}",
            u32::from(b.is_some()),
            row,
            hex(b.map_or(&[], |p| &p[..available.min(8)]))
        )
        .unwrap();
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "fb-bind" {
        let w = number(args);
        let h = number(args);
        let extra = number(args);
        let index = number(args);
        let null = number::<u32>(args) != 0;
        let size = number(args);
        let f = format(args);
        let data = unhex(args.next().unwrap());
        let mut a = JxlEncoderChunkedFrameAdapter::new(w, h, extra);
        let status = a.set_from_buffer(index, if null { None } else { Some(&data) }, size, f);
        return format!(
            "{} {} {} {}",
            u32::from(status),
            a.channels().len(),
            bits_per_channel(f.data_type),
            bytes_per_pixel(f)
        );
    }
    if kind == "fb-chain" {
        let w = number(args);
        let h = number(args);
        let extra = number(args);
        let n = number(args);
        struct Op {
            index: usize,
            size: usize,
            null: bool,
            copy: bool,
            f: JxlPixelFormat,
            data: Vec<u8>,
        }
        let ops: Vec<_> = (0..n)
            .map(|_| {
                let index = number(args);
                let null = number::<u32>(args) != 0;
                let size = number(args);
                let f = format(args);
                let copy = number::<u32>(args) != 0;
                let data = unhex(args.next().unwrap());
                Op {
                    index,
                    size,
                    null,
                    copy,
                    f,
                    data,
                }
            })
            .collect();
        let initial: Vec<_> = (0..512).map(|i| (i * 137 + 29) as u8).collect();
        let mut a = JxlEncoderChunkedFrameAdapter::new(w, h, extra);
        let f = JxlPixelFormat {
            num_channels: 1,
            data_type: JxlDataType::UINT8,
            endianness: JxlEndianness::NATIVE,
            align: 0,
        };
        for i in 0..a.channels().len() {
            a.set_from_buffer(i, Some(&initial), initial.len(), f);
        }
        let mut out = "I".to_owned();
        state(&mut out, &a);
        for o in &ops {
            let result = a.set_from_buffer(
                o.index,
                if o.null { None } else { Some(&o.data) },
                o.size,
                o.f,
            );
            write!(out, " O {}", u32::from(result)).unwrap();
            if o.copy {
                write!(out, " C {}", u32::from(a.copy_buffers())).unwrap();
            }
            state(&mut out, &a);
        }
        return out;
    }
    assert_eq!(kind, "fb-color");
    let w = number(args);
    let h = number(args);
    let gray = number::<u32>(args) != 0;
    let depth = number::<u32>(args);
    let f = format(args);
    let null = number::<u32>(args) != 0;
    let copy = number::<u32>(args) != 0;
    let size = number(args);
    let rx = number(args);
    let ry = number(args);
    let rw = number(args);
    let rh = number(args);
    let cw = number(args);
    let ch = number(args);
    let shrink = number::<i32>(args);
    let sw = number(args);
    let sh = number(args);
    let mut alpha = number::<u32>(args) != 0;
    let data = unhex(args.next().unwrap());
    let mut input = JxlEncoderChunkedFrameAdapter::new(w, h, 0);
    let bound = input.set_from_buffer(0, if null { None } else { Some(&data) }, size, f);
    if copy {
        input.copy_buffers();
    }
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
    let mut color = Image3::<f32>::create(cw, ch).unwrap();
    for c in 0..3 {
        for y in 0..ch {
            color
                .plane_mut(c)
                .row_mut(y)
                .fill(f32::from_bits(0x7fa12345));
        }
    }
    if shrink >= 0 {
        color.plane_mut(shrink as usize).shrink_to(sw, sh).unwrap();
    }
    let status =
        config.copy_color_channels(&input, Rect::new(rx, ry, rw, rh), &mut color, &mut alpha);
    let mut out = format!(
        "{} {} {}",
        u32::from(bound),
        status_code(&status),
        u32::from(alpha)
    );
    for c in 0..3 {
        let p = color.plane(c);
        write!(out, " {} {} {}", p.xsize(), p.ysize(), p.bytes_per_row()).unwrap();
        for value in p
            .pixels()
            .unwrap_or(&[])
            .iter()
            .take(ch * p.bytes_per_row() / 4)
        {
            write!(out, " {}", value.to_bits()).unwrap();
        }
    }
    out
}
fn payload(seed: usize) -> Vec<u8> {
    (0..512).map(|i| (i * 173 + seed * 37 + 61) as u8).collect()
}
fn next(state: &mut u64) -> usize {
    *state = state
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);
    (*state >> 24) as usize
}

#[test]
fn frame_buffer_binding_preserves_original_size_overflow_null_format_and_channel_guards() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let data = hex(&payload(0));
    for (w, h) in [
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (3, 2),
        (usize::MAX, 1),
        (usize::MAX / 2 + 1, 3),
    ] {
        for dtype in 0..8 {
            for channels in [0u32, 1, 3, u32::MAX] {
                for align in [0usize, 3, usize::MAX] {
                    for (extras, index) in [(0, 0), (2, 1), (usize::MAX, 0)] {
                        for null in [0, 1] {
                            let size = if (dtype + null) % 2 == 0 { 0 } else { 512 };
                            cases.push(format!("fb-bind {w} {h} {extras} {index} {null} {size} {channels} {dtype} 0 {align} {data}"));
                        }
                    }
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn frame_buffer_rebinding_and_copying_preserve_stale_copy_precedence_partial_fields_and_null_views()
{
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let mut seed = 0x61c948d053eda27f;
    for (w, h) in [
        (1, 1),
        (3, 2),
        (17, 3),
        (0, 1),
        (usize::MAX, 1),
        (usize::MAX / 2 + 1, 1),
    ] {
        for extra in [0usize, 2] {
            for trial in 0..32 {
                let mut request = format!("fb-chain {w} {h} {extra} 12");
                for step in 0..12 {
                    let index = next(&mut seed) % (extra + 3);
                    let null = next(&mut seed).is_multiple_of(3);
                    let size = [0, 1, 2, 31, 64, 511, 512][next(&mut seed) % 7];
                    let channels = [0u32, 1, 2, 3, 4, u32::MAX][next(&mut seed) % 6];
                    let dtype = next(&mut seed) % 8;
                    let endian = next(&mut seed) % 4;
                    let align =
                        [0usize, 1, 3, 16, usize::MAX / 2 + 1, usize::MAX][next(&mut seed) % 6];
                    let copy = next(&mut seed) % 2;
                    write!(
                        request,
                        " {index} {} {size} {channels} {dtype} {endian} {align} {copy} {}",
                        u32::from(null),
                        hex(&payload(trial + step))
                    )
                    .unwrap();
                }
                cases.push(request);
            }
        }
    }
    compare(cases);
}

#[test]
fn fixed_profile_color_copy_matches_original_buffer_views_conversion_gray_replication_and_partial_errors()
 {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    for (w, h) in [(1, 1), (5, 3)] {
        for gray in [0, 1] {
            for depth in [8, 16] {
                for channels in 0..=4 {
                    for dtype in [0, 2, 3, 5, 7] {
                        for endian in 0..4 {
                            for align in [0, 3, 16] {
                                for (null, copy) in [(0, 0), (0, 1), (1, 0)] {
                                    cases.push(format!("fb-color {w} {h} {gray} {depth} {channels} {dtype} {endian} {align} {null} {copy} 512 0 0 {w} {h} {w} {h} -1 0 0 {} {}",(channels+dtype)%2,hex(&payload(channels+dtype))));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for gray in [0, 1] {
        for dtype in [0, 2, 3, 5] {
            for c in 0..3 {
                for (sw, sh) in [(2, 2), (3, 1), (0, 1), (0, 0)] {
                    cases.push(format!("fb-color 3 2 {gray} 16 3 {dtype} 1 0 0 0 512 0 0 3 2 3 2 {c} {sw} {sh} 1 {}",hex(&payload(c))));
                }
            }
            for (rx, ry, rw, rh) in [
                (1, 1, 3, 2),
                (6, 0, 1, 1),
                (0, 5, 1, 1),
                (0, 0, usize::MAX, 1),
                (usize::MAX, 0, 1, 1),
            ] {
                cases.push(format!("fb-color 5 4 {gray} 8 4 {dtype} 2 16 0 0 512 {rx} {ry} {rw} {rh} 3 2 -1 0 0 0 {}",hex(&payload(dtype))));
            }
        }
        for size in [0, 17, 18, 512, 513] {
            cases.push(format!(
                "fb-color 3 2 {gray} 8 3 2 0 0 0 0 {size} 0 0 3 2 3 2 -1 0 0 1 {}",
                hex(&payload(size))
            ));
        }
    }
    compare(cases);
}
