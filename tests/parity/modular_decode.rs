// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Complete original ModularGenericCompress fixtures feed the Rust image parser.
use super::*;
use jxl::jxl::dec_ans::{ANSCode, decode_histograms};
use jxl::jxl::modular::encoding::dec_ma::decode_tree;
use jxl::jxl::modular::encoding::encoding::*;
use jxl::jxl::modular::encoding::ma_common::MAX_TREE_SIZE;
use jxl::jxl::modular::modular_image::Image;
use jxl::jxl::modular::options::ModularOptions;
pub(super) fn handles(kind: &str) -> bool {
    matches!(kind, "md-read" | "md-dimensions")
}
struct Observation {
    status: Status,
    bits: usize,
    close: Status,
    header: GroupHeader,
    image: Image,
    samples: bool,
}
fn read_request(args: &mut std::str::SplitWhitespace<'_>) -> Observation {
    let generic = number::<u32>(args) != 0;
    let header_out = number::<u32>(args) != 0;
    let undo = number::<u32>(args) != 0;
    let allow = number::<u32>(args) != 0;
    let samples = number::<u32>(args) != 0;
    let group = number(args);
    let max_chan_size = number(args);
    let group_dim = number(args);
    let flags = number::<u32>(args);
    let prefix = number(args);
    let global_data = unhex(args.next().unwrap());
    let data = unhex(args.next().unwrap());
    let mut image = transforms::read_image(args);
    let mut header = ma::header_seed(3);
    let mut tree = Vec::new();
    let mut code = ANSCode::default();
    let mut map = Vec::new();
    if !global_data.is_empty() {
        let mut gb = BitReader::new(&global_data);
        decode_tree(&mut gb, &mut tree, MAX_TREE_SIZE).unwrap();
        decode_histograms(&mut gb, tree.len().div_ceil(2), &mut code, &mut map, false).unwrap();
        gb.close().unwrap();
    }
    if flags & 8 != 0 {
        tree.clear();
    }
    let mut options = ModularOptions {
        max_chan_size,
        group_dim,
        ..Default::default()
    };
    let mut br = BitReader::new(&data);
    br.skip_bits(prefix);
    let gt = if flags & 1 != 0 { Some(&tree) } else { None };
    let gc = if flags & 2 != 0 { Some(&code) } else { None };
    let gm = if flags & 4 != 0 {
        Some(map.as_slice())
    } else {
        None
    };
    let status = if generic {
        modular_generic_decompress(
            &mut br,
            &mut image,
            if header_out { Some(&mut header) } else { None },
            group,
            &mut options,
            undo,
            gt,
            gc,
            gm,
            allow,
        )
    } else {
        modular_decode(
            &mut br,
            &mut image,
            &mut header,
            group,
            &mut options,
            gt,
            gc,
            gm,
            allow,
        )
    };
    let bits = br.total_bits_consumed();
    let close = br.close();
    for t in &mut image.transform {
        t.max_delta_error = 77;
    }
    Observation {
        status,
        bits,
        close,
        header,
        image,
        samples,
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "md-dimensions" {
        let group_dim = number(args);
        let image = transforms::read_image(args);
        let s = validate_channel_dimensions(
            &image,
            &ModularOptions {
                group_dim,
                ..Default::default()
            },
        );
        let mut out = status_code(&s).to_string();
        transforms::metadata(&mut out, &image);
        return out;
    }
    let mut o = read_request(args);
    let mut out = format!(
        "{} {} {}",
        status_code(&o.status),
        o.bits,
        status_code(&o.close)
    );
    ma::header(&mut out, &mut o.header);
    if o.samples {
        image::modular(&mut out, &o.image);
    } else {
        transforms::metadata(&mut out, &o.image);
        write!(out, " {}", o.image.transform.len()).unwrap();
        for t in &o.image.transform {
            image::transform(&mut out, t);
        }
    }
    out
}

fn fixture(
    w: usize,
    h: usize,
    n: usize,
    depth: i32,
    pattern: usize,
    seed: &mut u64,
) -> (String, Vec<Vec<i32>>) {
    let mut out = format!("{w} {h} {depth} 0 0 {n}");
    let mut pixels = Vec::new();
    for c in 0..n {
        write!(out, " {w} {h} 0 0 {c}").unwrap();
        let mut channel = Vec::new();
        for y in 0..h {
            for x in 0..w {
                let value = match pattern {
                    0 => [0, 127, 65535][c % 3],
                    1 => ((c * 277 + y * 193 + x * 11) % 65536) as i32,
                    _ => (next(seed) % 65536) as i32,
                };
                let value = value & ((1i32 << depth) - 1);
                channel.push(value);
                write!(out, " {value}").unwrap();
            }
        }
        pixels.push(channel);
    }
    (out, pixels)
}
fn target(w: usize, h: usize, n: usize, depth: i32) -> String {
    let mut out = format!("{w} {h} {depth} 0 0 {n}");
    for c in 0..n {
        write!(out, " {w} {h} 0 0 {c}").unwrap();
        for _ in 0..w * h {
            out.push_str(" 12345");
        }
    }
    out
}
fn check_samples(request: &str, expected: &[Vec<i32>], w: usize, h: usize, bits: usize) {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("md-read"));
    let o = read_request(&mut args);
    assert_eq!(o.status, Ok(()), "{request}");
    assert_eq!(o.close, Ok(()));
    assert_eq!(o.bits, bits);
    assert!(!o.image.error);
    assert!(o.image.transform.is_empty());
    assert_eq!(o.image.channel.len(), expected.len());
    for (c, values) in expected.iter().enumerate() {
        assert_eq!((o.image.channel[c].w, o.image.channel[c].h), (w, h));
        for y in 0..h {
            assert_eq!(&o.image.channel[c].row(y)[..w], &values[y * w..(y + 1) * w]);
        }
    }
}
#[test]
fn modular_complete_source_streams_restore_gray_rgb_and_transform_chains() {
    let mut cases = Vec::new();
    let mut inputs = Vec::new();
    let mut seed = 993271;
    for i in 0..180 {
        let (w, h) = [(1, 1), (2, 3), (9, 5), (17, 9), (65, 17)][i % 5];
        let n = if (i / 6) % 2 == 0 { 1 } else { 3 };
        let shape = i % 6;
        let group = i % 11;
        let prefix = i % 8;
        let depth = [8, 16][(i / 12) % 2];
        let (image, pixels) = fixture(w, h, n, depth, (i / 6) % 3, &mut seed);
        let kind = [0, 1, 3, 4, 5, 6][(i / 3) % 6];
        let predictor = if i % 7 == 0 { 6 } else { 5 };
        inputs.push(format!(
            "md-source {kind} {predictor} {} {} {} {shape} {group} {prefix} {image}",
            i % 5,
            i % 2,
            (i / 2) % 2
        ));
        cases.push((w, h, n, group, prefix, depth, pixels, shape));
    }
    let output = original_observations(&inputs);
    let mut requests = Vec::new();
    for (i, encoded) in output.iter().enumerate() {
        let p: Vec<_> = encoded.split_whitespace().collect();
        assert_eq!(p[0], "0", "Source rejected {}", inputs[i]);
        let bits: usize = p[1].parse().unwrap();
        let (w, h, n, group, prefix, depth, ref pixels, _) = cases[i];
        let target = target(w, h, n, depth);
        let request = format!(
            "md-read 1 {} 1 0 1 {group} 16777215 536870911 0 {prefix} - {} {target}",
            i % 2,
            p[3]
        );
        check_samples(&request, pixels, w, h, bits);
        requests.push(request);
        requests.push(format!(
            "md-read 0 1 0 0 1 {group} 16777215 536870911 0 {prefix} - {} {target}",
            p[3]
        ));
    }
    compare(requests);
}
#[test]
fn modular_truncation_cleanup_and_partial_metadata_match_source() {
    let mut source = Vec::new();
    let mut configs = Vec::new();
    let mut seed = 881133;
    for i in 0..36 {
        let shape = i % 6;
        let (w, h) = [(1, 1), (9, 5), (17, 9)][(i / 6) % 3];
        let n = if (i / 6) % 2 == 0 { 1 } else { 3 };
        let prefix = i % 8;
        let (image, _) = fixture(w, h, n, 16, (i / 6) % 3, &mut seed);
        source.push(format!(
            "md-source {} 6 {} {} 1 {shape} 7 {prefix} {image}",
            [0, 1, 5, 6][i % 4],
            i % 5,
            i % 2
        ));
        configs.push((w, h, n, shape, prefix));
    }
    let encoded = original_observations(&source);
    let mut requests = Vec::new();
    for (i, encoded) in encoded.iter().enumerate() {
        let f: Vec<_> = encoded.split_whitespace().collect();
        let data = unhex(f[3]);
        let header_end: usize = f[2].parse().unwrap();
        let (w, h, n, shape, prefix) = configs[i];
        let image = target(w, h, n, 16);
        for len in [
            0,
            1,
            2,
            3,
            header_end.div_ceil(8),
            header_end.div_ceil(8) + 1,
            data.len() / 2,
            data.len().saturating_sub(1),
            data.len(),
        ] {
            let len = len.min(data.len());
            let bytes = hex(&data[..len]);
            for allow in 0..2 {
                for undo in 0..2 {
                    // After a valid complete header, the strict scope guard defines
                    // every newly allocated sample even on a later parse failure.
                    // Allowed truncation can leave source transform storage
                    // indeterminate; compare its metadata instead of those bytes.
                    let samples = u32::from(
                        shape == 0 || (allow == 0 && len * 8 >= header_end) || len == data.len(),
                    );
                    requests.push(format!("md-read 1 1 {undo} {allow} {samples} 7 16777215 536870911 0 {prefix} - {bytes} {image}"));
                }
            }
        }
        for byte in [0, data.len() / 2, data.len() - 1] {
            let mut broken = data.clone();
            broken[byte] ^= 0x40;
            for allow in 0..2 {
                requests.push(format!(
                    "md-read 1 1 1 {allow} {} 7 16777215 536870911 0 {prefix} - {} {image}",
                    u32::from(shape == 0),
                    hex(&broken)
                ));
            }
        }
        for max in [0, 1, 8] {
            requests.push(format!(
                "md-read 0 1 0 0 0 7 {max} 536870911 0 {prefix} - {} {image}",
                f[3]
            ));
        }
    }
    compare(requests);
}
#[test]
fn modular_global_tables_and_missing_pointer_errors_match_source() {
    let mut source = Vec::new();
    let mut configs = Vec::new();
    let mut seed = 876921;
    for i in 0..48 {
        let (w, h) = [(1, 1), (9, 5), (17, 9), (33, 17)][i % 4];
        let n = if (i / 6) % 2 == 0 { 1 } else { 3 };
        let prefix = i % 8;
        let (image, pixels) = fixture(w, h, n, 16, (i / 6) % 3, &mut seed);
        source.push(format!(
            "md-source-global {} 6 {} {} {} {} 13 {prefix} {image}",
            [0, 1, 5, 6][i % 4],
            i % 5,
            i % 2,
            (i / 2) % 2,
            i % 6
        ));
        configs.push((w, h, n, prefix, pixels));
    }
    let encoded = original_observations(&source);
    let mut requests = Vec::new();
    for (i, encoded) in encoded.iter().enumerate() {
        let f: Vec<_> = encoded.split_whitespace().collect();
        assert_eq!(f[0], "0");
        let (w, h, n, prefix, ref pixels) = configs[i];
        let image = target(w, h, n, 16);
        let request = format!(
            "md-read 1 1 1 0 1 13 16777215 536870911 7 {prefix} {} {} {image}",
            f[4], f[3]
        );
        check_samples(&request, pixels, w, h, f[1].parse().unwrap());
        requests.push(request);
        for flags in [0, 1, 2, 3, 4, 5, 6, 15] {
            requests.push(format!(
                "md-read 1 1 0 0 1 13 16777215 536870911 {flags} {prefix} {} {} {image}",
                f[4], f[3]
            ));
        }
    }
    compare(requests);
}
#[test]
fn modular_channel_dimension_validation_and_empty_images_keep_source_order() {
    let mut cases = Vec::new();
    let mut seed = 64321;
    for i in 0..500 {
        let n = 1 + i % 4;
        let meta = i % (n + 1);
        let group = [0, 1, 2, 8, 16, 256, 536870911, usize::MAX][i % 8];
        let mut image = format!("17 9 16 {meta} 0 {n}");
        for c in 0..n {
            let w = [0, 1, 8, 17, 33][(i + c) % 5];
            let h = [0, 1, 8, 9, 17][(i / 5 + c) % 5];
            let hs = [0, 1, 2, 3, 4, 30, 31, 63][(i + c) % 8];
            let vs = [0, 1, 2, 3, 4, 30, 31, 63][(i / 8 + c) % 8];
            write!(image, " {w} {h} {hs} {vs} {c}").unwrap();
            for _ in 0..w * h {
                write!(image, " {}", next(&mut seed) as i32).unwrap();
            }
        }
        cases.push(format!("md-dimensions {group} {image}"));
    }
    for n in [0, 1, 3] {
        for error in 0..2 {
            for allow in 0..2 {
                let mut image = format!("0 0 8 0 {error} {n}");
                for c in 0..n {
                    write!(image, " 0 0 0 0 {c}").unwrap();
                }
                for generic in 0..2 {
                    cases.push(format!(
                        "md-read {generic} 1 1 {allow} 1 0 16777215 536870911 0 0 - - {image}"
                    ));
                }
            }
        }
    }
    // A complete source GroupHeader with use_global_tree=true and no transforms.
    // Check the image-error/dimension/empty-work gates before missing globals.
    for n in [1, 3] {
        for error in 0..2 {
            for allow in 0..2 {
                for generic in 0..2 {
                    for header_out in 0..2 {
                        for max in [0, 1, 16777215] {
                            for group_dim in [0, 256] {
                                let mut fields: Vec<_> = target(3, 2, n, 8)
                                    .split_whitespace()
                                    .map(str::to_owned)
                                    .collect();
                                fields[4] = error.to_string();
                                let image = fields.join(" ");
                                cases.push(format!("md-read {generic} {header_out} 1 {allow} 1 0 {max} {group_dim} 15 0 - 03 {image}"));
                            }
                        }
                    }
                }
            }
        }
    }
    compare(cases);
}
