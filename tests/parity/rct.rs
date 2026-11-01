// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Compare original scalar/null-pool RCT APIs, including status and channel tags.

use super::image::{modular, paint};
use super::*;
use jxl::jxl::modular::modular_image::Image;
use jxl::jxl::modular::transform::enc_rct::{fwd_rct, fwd_rct_channels};
use jxl::jxl::modular::transform::rct::inv_rct;
pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("rct-")
}
pub(super) fn fill(image: &mut Image, args: &mut std::str::SplitWhitespace<'_>) {
    for c in &mut image.channel {
        paint(&mut c.plane, 81 + c.component as u64);
        for y in 0..c.h {
            for x in 0..c.w {
                c.row_mut(y)[x] = number(args);
            }
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "rct-image" => {
            let op: u32 = number(args);
            let w = number(args);
            let h = number(args);
            let n = number(args);
            let begin: usize = number(args);
            let kind = number(args);
            let meta = number(args);
            let mismatch: u32 = number(args);
            let mut image = Image::create(w, h, 16, n).unwrap();
            fill(&mut image, args);
            image.nb_meta_channels = meta;
            for (c, ch) in image.channel.iter_mut().enumerate() {
                ch.hshift = (c / 3) as i32;
            }
            if begin + 1 < image.channel.len() {
                if mismatch == 1 {
                    image.channel[begin + 1].vshift = 1;
                }
                if mismatch == 2 {
                    image.channel[begin + 1].w = 0;
                }
            }
            let status = if op == 1 {
                inv_rct(&mut image, begin, kind)
            } else {
                fwd_rct(&mut image, begin, kind)
            };
            let mut out = status_code(&status).to_string();
            modular(&mut out, &image);
            if op == 2 {
                let status = inv_rct(&mut image, begin, kind);
                write!(out, " {}", status_code(&status)).unwrap();
                modular(&mut out, &image);
            }
            out
        }
        "rct-channels" => {
            let w = number(args);
            let h = number(args);
            let kind = number(args);
            let mut from = Image::create(w, h, 16, 3).unwrap();
            let mut to = Image::create(w, h, 16, 3).unwrap();
            fill(&mut from, args);
            for c in 0..3 {
                from.channel[c].hshift = c as i32;
                to.channel[c].hshift = (3 - c) as i32;
                to.channel[c].component = (c + 3) as i32;
                paint(&mut to.channel[c].plane, 91 + c as u64);
            }
            let [a, b, c] = &mut to.channel[..] else {
                unreachable!()
            };
            let status = fwd_rct_channels(
                [&from.channel[0], &from.channel[1], &from.channel[2]],
                [a, b, c],
                kind,
            );
            let mut out = status_code(&status).to_string();
            modular(&mut out, &from);
            modular(&mut out, &to);
            out
        }
        "rct-range-wrap" => {
            let op: u32 = number(args);
            let kind = number(args);
            let mut image = Image::create(3, 1, 16, 3).unwrap();
            for c in 0..3 {
                paint(&mut image.channel[c].plane, 101 + c as u64);
                for x in 0..3 {
                    image.channel[c].row_mut(0)[x] = (c * 3 + x) as i32;
                }
                image.channel[c].h = 1usize << 32;
            }
            let status = if op != 0 {
                inv_rct(&mut image, 0, kind)
            } else {
                fwd_rct(&mut image, 0, kind)
            };
            let mut out = status_code(&status).to_string();
            modular(&mut out, &image);
            out
        }
        _ => panic!("unknown RCT request"),
    }
}
fn samples(request: &mut String, count: usize, wide: bool, seed: &mut u64) {
    let extremes = [
        i32::MIN,
        i32::MIN + 1,
        -1073741824,
        -1,
        0,
        1,
        1073741823,
        i32::MAX - 1,
        i32::MAX,
    ];
    for _ in 0..count {
        let v = if wide {
            extremes[next(seed) as usize % extremes.len()]
        } else {
            (next(seed) % 131071) as i32 - 65535
        };
        write!(request, " {v}").unwrap();
    }
}
#[test]
fn forward_rct_all_types_in_place_and_separate_channels_match_original() {
    let mut cases = Vec::new();
    let mut seed = 518737;
    for kind in 0..42 {
        for (w, h) in [(1, 1), (7, 2), (32, 3), (129, 1), (3, 0)] {
            for (n, begin) in [(3, 0), (6, 3)] {
                let mut r = format!("rct-image 2 {w} {h} {n} {begin} {kind} 0 0");
                samples(&mut r, w * h * n, false, &mut seed);
                cases.push(r);
            }
            let mut r = format!("rct-channels {w} {h} {kind}");
            samples(&mut r, w * h * 3, false, &mut seed);
            cases.push(r);
        }
    }
    // The standalone forward overload has no inverse permutation-range guard.
    // Its index formulas and aliases remain defined for these source calls.
    for kind in [42, 43, 48, 79, usize::MAX] {
        let mut r = format!("rct-image 0 3 2 3 0 {kind} 0 0");
        samples(&mut r, 18, false, &mut seed);
        cases.push(r);
        let mut r = format!("rct-channels 3 2 {kind}");
        samples(&mut r, 18, false, &mut seed);
        cases.push(r);
    }
    compare(cases);
}
#[test]
fn inverse_rct_preserves_highway_wrapping_and_channel_move_metadata() {
    let mut cases = Vec::new();
    let mut seed = 954137;
    for kind in 0..42 {
        for (w, h) in [(1, 1), (17, 2), (33, 1)] {
            let mut r = format!("rct-image 1 {w} {h} 3 0 {kind} 0 0");
            samples(&mut r, w * h * 3, true, &mut seed);
            cases.push(r);
        }
    }
    for kind in [42, 43, 48] {
        let mut r = format!("rct-image 1 3 1 3 0 {kind} 0 0");
        samples(&mut r, 9, true, &mut seed);
        cases.push(r);
    }
    compare(cases);
}
#[test]
fn rct_channel_validation_noop_status_and_uint32_task_ranges_match_original() {
    let mut cases = Vec::new();
    let mut seed = 694529;
    for n in [0, 1, 2, 3, 6] {
        for begin in [0, 1, 3, 6] {
            for meta in [0, 1, 3, 4, 7] {
                for mismatch in 0..3 {
                    for kind in [0, 6, 7, 41] {
                        let mut r = format!("rct-image 2 3 1 {n} {begin} {kind} {meta} {mismatch}");
                        samples(&mut r, n * 3, false, &mut seed);
                        cases.push(r);
                    }
                }
            }
        }
    }
    for op in 0..2 {
        for kind in [0, 6, 7, 41] {
            cases.push(format!("rct-range-wrap {op} {kind}"));
        }
    }
    compare(cases);
}
