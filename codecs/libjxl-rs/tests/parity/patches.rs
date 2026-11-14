// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::blending::perform_blending;
use jxl::jxl::color_encoding_internal::ColorEncoding;
use jxl::jxl::dec_patch_dictionary::{
    PatchBlendMode, PatchBlending, PatchDictionary, ReferenceFrame,
};
use jxl::jxl::image::{Image3, Plane};
use jxl::jxl::image_bundle::ImageBundle;
use jxl::jxl::image_metadata::{ExtraChannel, ExtraChannelInfo, ImageMetadata};
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET, Row, RowBuffers, RowStorage,
};
use jxl::jxl::render_pipeline::stage_patches::{get_patches_stage, get_patches_stage_shared};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("patch-")
}
fn extras(n: usize, assoc: usize) -> Vec<ExtraChannelInfo> {
    (0..n)
        .map(|i| ExtraChannelInfo {
            channel_type: if i == 0 || i == 2 {
                ExtraChannel::ALPHA
            } else {
                ExtraChannel::DEPTH
            },
            alpha_associated: (assoc >> i) & 1 != 0,
            ..ExtraChannelInfo::default()
        })
        .collect()
}
fn pixel(r: usize, c: usize, x: usize, y: usize) -> f32 {
    (r.wrapping_mul(37)
        .wrapping_add(c.wrapping_mul(11))
        .wrapping_add(x.wrapping_mul(3))
        .wrapping_add(y.wrapping_mul(7))
        % 47) as f32
        / 16.0
        - 0.5
}
fn blend(args: &mut std::str::SplitWhitespace<'_>) -> PatchBlending {
    PatchBlending {
        mode: PatchBlendMode(number(args)),
        alpha_channel: number(args),
        clamp: number::<u8>(args) != 0,
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut out = String::new();
    if kind == "patch-needs" {
        use jxl::jxl::frame_header::{BlendMode, BlendingInfo, FrameHeader, FrameType};
        let mut header = FrameHeader {
            frame_type: FrameType(number(args)),
            custom_size_or_origin: number::<u8>(args) != 0,
            ..FrameHeader::new(None)
        };
        header.blending_info.mode = BlendMode(number(args));
        header.extra_channel_blending_info = (0..number(args))
            .map(|_| BlendingInfo {
                mode: BlendMode(number(args)),
                ..BlendingInfo::default()
            })
            .collect();
        return u8::from(jxl::jxl::blending::needs_blending(&header)).to_string();
    }
    if kind == "patch-rgba" {
        let premul = number::<u8>(args) != 0;
        let clamp = number::<u8>(args) != 0;
        let n = number(args);
        let input: [Vec<f32>; 8] =
            std::array::from_fn(|_| (0..n).map(|_| f32::from_bits(number(args))).collect());
        let mut result: [Vec<f32>; 4] = std::array::from_fn(|_| vec![19.0; n]);
        let [r, g, b, a] = &mut result;
        jxl::jxl::alpha::perform_alpha_blending_rgba(
            [&input[0], &input[1], &input[2], &input[3]],
            [&input[4], &input[5], &input[6], &input[7]],
            &mut [r, g, b, a],
            n,
            premul,
            clamp,
        );
        out.push('A');
        for row in result {
            for v in row {
                write!(out, " {}", v.to_bits()).unwrap();
            }
        }
        return out;
    }
    if kind == "patch-alpha" {
        use jxl::jxl::alpha::*;
        let op: usize = number(args);
        let premul = number::<u8>(args) != 0;
        let clamp = number::<u8>(args) != 0;
        let n = number(args);
        let bi: usize = number(args);
        let bai: usize = number(args);
        let fi: usize = number(args);
        let fai: usize = number(args);
        let input: [Vec<f32>; 5] =
            std::array::from_fn(|_| (0..n).map(|_| f32::from_bits(number(args))).collect());
        let mut result: [Vec<f32>; 4] = std::array::from_fn(|_| vec![19.0; n]);
        match op {
            0 => perform_alpha_blending(
                &input[bi],
                &input[bai],
                &input[fi],
                &input[fai],
                &mut result[0],
                n,
                premul,
                clamp,
            ),
            1 => {
                let [r, g, b, a] = &mut result;
                perform_alpha_blending_rgba(
                    [&input[0], &input[1], &input[2], &input[bai]],
                    [&input[2], &input[3], &input[4], &input[fai]],
                    &mut [r, g, b, a],
                    n,
                    premul,
                    clamp,
                );
            }
            2 => perform_alpha_weighted_add(
                &input[bi],
                &input[fi],
                &input[fai],
                &mut result[0],
                n,
                clamp,
            ),
            3 => perform_mul_blending(&input[bi], &input[fi], &mut result[0], n, clamp),
            _ => {
                for c in 0..3 {
                    result[c].copy_from_slice(&input[c]);
                }
                let [r, g, b, _] = &mut result;
                if op == 4 {
                    premultiply_alpha(&mut [r, g, b], &input[fai], n);
                } else {
                    unpremultiply_alpha(&mut [r, g, b], &input[fai], n);
                }
            }
        }
        out.push('A');
        for row in result {
            for v in row {
                write!(out, " {}", v.to_bits()).unwrap();
            }
        }
        return out;
    }
    if kind == "patch-blend" {
        let n: usize = number(args);
        let assoc = number(args);
        let width = number(args);
        let x0: usize = number(args);
        let color = blend(args);
        let ec: Vec<_> = (0..n).map(|_| blend(args)).collect();
        let bg: Vec<Vec<_>> = (0..3 + n)
            .map(|_| {
                (0..x0 + width + 2)
                    .map(|_| f32::from_bits(number(args)))
                    .collect()
            })
            .collect();
        let fg: Vec<Vec<_>> = (0..3 + n)
            .map(|_| {
                (0..x0 + width + 2)
                    .map(|_| f32::from_bits(number(args)))
                    .collect()
            })
            .collect();
        let b: Vec<_> = bg.iter().map(Vec::as_slice).collect();
        let f: Vec<_> = fg.iter().map(Vec::as_slice).collect();
        let tmp = perform_blending(&b, &f, x0, width, &color, &ec, &extras(n, assoc));
        write!(
            out,
            "{}",
            status_code(&tmp.as_ref().map(|_| ()).map_err(|&e| e))
        )
        .unwrap();
        let mut result = bg;
        if let Ok(tmp) = tmp {
            for (c, row) in result.iter_mut().enumerate() {
                if width != 0 {
                    row[x0..x0 + width].copy_from_slice(&tmp.row(c)[..width]);
                }
            }
        }
        for row in result {
            for v in row {
                write!(out, " {}", v.to_bits()).unwrap();
            }
        }
        return out;
    }
    assert_eq!(kind, "patch-read");
    let n: usize = number(args);
    let assoc = number(args);
    let w = number(args);
    let h: usize = number(args);
    let valid: usize = number(args);
    let xyb: usize = number(args);
    let render: usize = number(args);
    let x0: usize = number(args);
    let width: usize = number(args);
    let left: usize = number(args);
    let right: usize = number(args);
    let count = number(args);
    let extra = extras(n, assoc);
    let metadata = ImageMetadata {
        extra_channel_info: extra.clone(),
        ..ImageMetadata::default()
    };
    let refs: [ReferenceFrame; 4] = std::array::from_fn(|r| {
        let mut ib = ImageBundle::new(Some(&metadata));
        if (valid >> r) & 1 != 0 {
            let mut color = Image3::create(12, 11).unwrap();
            for c in 0..3 {
                for y in 0..11 {
                    for x in 0..12 {
                        color.plane_row_mut(c, y)[x] = pixel(r, c, x, y);
                    }
                }
            }
            ib.set_from_image(&mut Some(color), ColorEncoding::srgb(false))
                .unwrap();
            for c in 0..n {
                let mut plane = Plane::create(12, 11).unwrap();
                for y in 0..11 {
                    for x in 0..12 {
                        plane.row_mut(y)[x] = pixel(r, 3 + c, x, y);
                    }
                }
                ib.extra_channels_mut().push(plane);
            }
        }
        ReferenceFrame {
            frame: Some(Box::new(ib)),
            ib_is_in_xyb: (xyb >> r) & 1 != 0,
        }
    });
    let mut pd = PatchDictionary::new();
    pd.bind_own_reference_frames();
    let mut uses = false;
    let dictionary = std::sync::Arc::new(jxl::jxl::base::sync_cell::SyncRefCell::new(pd));
    let references = std::sync::Arc::new(jxl::jxl::base::sync_cell::SyncRefCell::new(refs));
    let persistent_stage = get_patches_stage_shared(
        std::sync::Arc::clone(&dictionary),
        std::sync::Arc::clone(&references),
        &extra,
    );
    for cycle in 0..count {
        if render == 3 {
            let mut refs = references.borrow_mut();
            for r in 0..4 {
                if (valid >> r) & 1 != 0 {
                    refs[r]
                        .frame
                        .as_mut()
                        .unwrap()
                        .color_mut()
                        .plane_row_mut(0, 1)[1] = pixel(r, 0, 1, 1) + cycle as f32 * 0.125;
                }
            }
        }
        let refs = references.borrow();
        let mut pd = dictionary.borrow_mut();
        let prefix = number(args);
        let bytes = unhex(args.next().unwrap());
        let mut reader = BitReader::new(&bytes);
        reader.skip_bits(prefix);
        let status = pd.decode(
            jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
            &mut reader,
            w,
            h,
            n,
            &mut uses,
            &refs,
        );
        drop(pd);
        let pd = dictionary.borrow();
        let consumed = reader.total_bits_consumed();
        let close = reader.close();
        write!(
            out,
            " D {} {} {} {}",
            status_code(&status),
            consumed,
            status_code(&close),
            u8::from(uses)
        )
        .unwrap();
        out.push_str(" S");
        for size in pd.storage_sizes() {
            write!(out, " {size}").unwrap();
        }
        write!(
            out,
            " {} {}",
            pd.blendings_stride().unwrap(),
            pd.get_references()
        )
        .unwrap();
        for p in pd.positions() {
            write!(out, " P {} {} {}", p.x, p.y, p.ref_pos_idx).unwrap();
        }
        for p in pd.ref_positions() {
            write!(
                out,
                " R {} {} {} {} {}",
                p.reference, p.x0, p.y0, p.xsize, p.ysize
            )
            .unwrap();
        }
        for p in pd.blendings() {
            write!(
                out,
                " B {} {} {}",
                p.mode.0,
                p.alpha_channel,
                u8::from(p.clamp)
            )
            .unwrap();
        }
        for node in pd.patch_tree() {
            write!(
                out,
                " N {} {} {} {} {}",
                node.left_child, node.right_child, node.y_center, node.start, node.num
            )
            .unwrap();
        }
        out.push_str(" C");
        for count in pd.num_patches() {
            write!(out, " {count}").unwrap();
        }
        out.push_str(" L");
        for patch in pd.sorted_patches_y0() {
            write!(out, " {} {}", patch.0, patch.1).unwrap();
        }
        out.push_str(" H");
        for patch in pd.sorted_patches_y1() {
            write!(out, " {} {}", patch.0, patch.1).unwrap();
        }
        if status.is_ok() {
            for y in 0..h + 2 {
                let patches = pd.get_patches_for_row(y);
                write!(out, " T {}", patches.len()).unwrap();
                for p in patches {
                    write!(out, " {p}").unwrap();
                }
            }
            if render != 0 {
                let span = width + left + right;
                for y in 0..h {
                    let mut storage: Vec<_> = (0..3 + n)
                        .map(|c| {
                            let mut p = Plane::create(span + 64, 1).unwrap();
                            for x in 0..span {
                                p.row_mut(0)[32 + x] =
                                    pixel(9, c, x0.wrapping_sub(left).wrapping_add(x), y);
                            }
                            p
                        })
                        .collect();
                    let rows: Vec<_> = (0..3 + n)
                        .map(|c| Row {
                            storage: RowStorage::Input,
                            channel: c,
                            y: 0,
                            x: 32,
                        })
                        .collect();
                    let mut buffers = RowBuffers {
                        input: &mut storage,
                        output: &mut [],
                    };
                    let status = if render == 1 {
                        pd.add_one_row(
                            &rows,
                            &mut buffers,
                            y,
                            x0.wrapping_sub(left),
                            span,
                            &extra,
                            &refs,
                        )
                    } else {
                        let stage = get_patches_stage(&pd, &refs, &extra);
                        let input = rows
                            .iter()
                            .map(|r| {
                                vec![r.offset(left as isize - RENDER_PIPELINE_X_OFFSET as isize)]
                            })
                            .collect();
                        if render == 3 {
                            persistent_stage.process_row(
                                &input,
                                &vec![],
                                &mut buffers,
                                left,
                                right,
                                width,
                                x0,
                                y,
                                0,
                            )
                        } else {
                            stage.process_row(
                                &input,
                                &vec![],
                                &mut buffers,
                                left,
                                right,
                                width,
                                x0,
                                y,
                                0,
                            )
                        }
                    };
                    write!(out, " O {}", status_code(&status)).unwrap();
                    for r in rows {
                        for v in &buffers.row(r)[..span] {
                            write!(out, " {}", v.to_bits()).unwrap();
                        }
                    }
                }
            }
        }
    }
    out.trim_start().to_owned()
}
fn fixture(tokens: &[(u32, u32)], prefix: usize) -> Vec<u8> {
    let mut request = format!("patch-fixture {prefix} {}", tokens.len());
    for &(ctx, value) in tokens {
        write!(request, " {ctx} {value}").unwrap();
    }
    let response = original_observations(&[request]);
    let words: Vec<_> = response[0].split_whitespace().collect();
    assert_eq!(words[0], "0");
    unhex(words[2])
}
fn tokens(n: usize, mode: u32) -> Vec<(u32, u32)> {
    let mut t = vec![(0, 2)];
    for r in 0..2 {
        t.extend([(1, r), (3, 1), (3, 1), (2, 3), (2, 2), (7, 2)]);
        for i in 0..3 {
            if i == 0 {
                t.extend([(4, 2 + r * 3), (4, 2 + r)]);
            } else {
                t.extend([(6, if i == 1 { 4 } else { 1 }), (6, 2)]);
            }
            for j in 0..n + 1 {
                let m = if j == 0 { mode } else { (mode + j as u32) % 8 };
                t.push((5, m));
                if m >= 4 && n > 1 {
                    t.push((8, if j % 2 == 0 { 0 } else { 2.min(n as u32 - 1) }));
                }
                if m >= 3 {
                    t.push((9, (i + j + r as usize) as u32 % 2));
                }
            }
        }
    }
    t
}
#[expect(
    clippy::too_many_arguments,
    reason = "Protocol fixture retains all source row and decoder parameters."
)]
fn read(
    n: usize,
    assoc: usize,
    render: usize,
    x0: usize,
    width: usize,
    left: usize,
    right: usize,
    prefix: usize,
    bytes: &[u8],
) -> String {
    format!(
        "patch-read {n} {assoc} 14 10 15 15 {render} {x0} {width} {left} {right} 1 {prefix} {}",
        hex(bytes)
    )
}
fn chunks(cases: Vec<String>) {
    let mut failures = Vec::new();
    for chunk in cases.chunks(16) {
        let original = original_observations(chunk);
        for (request, cpp) in chunk.iter().zip(original) {
            let rust = super::observe(request);
            if !super::parity_eq(request, &rust, &cpp) {
                let a: Vec<_> = rust.split_whitespace().collect();
                let b: Vec<_> = cpp.split_whitespace().collect();
                let diffs: Vec<_> = a
                    .iter()
                    .zip(&b)
                    .enumerate()
                    .filter(|(_, (x, y))| x != y)
                    .map(|(i, (x, y))| format!("{i}:{x}/{y}"))
                    .collect();
                failures.push(format!(
                    "{} => {}",
                    request
                        .split_whitespace()
                        .take(10)
                        .collect::<Vec<_>>()
                        .join(" "),
                    diffs.join(",")
                ));
            }
        }
    }
    assert!(
        failures.is_empty(),
        "{} mismatches:\n{}",
        failures.len(),
        failures.join("\n")
    );
    println!("{} C++/Rust differential cases matched", cases.len());
}

fn patch_tree_cases() -> Vec<(String, bool)> {
    let mut cases = Vec::new();
    for n in [0, 1, 3] {
        for mode in 0..8 {
            for prefix in [0, 3, 7] {
                let bytes = fixture(&tokens(n, mode), prefix);
                for assoc in [0, 5] {
                    for render in [0, 1, 2] {
                        for (x, width, l, r) in
                            [(0, 14, 0, 0), (3, 5, 0, 0), (3, 5, 2, 3), (8, 0, 0, 0)]
                        {
                            // Original decoded blendings repeat these modes and
                            // alpha selections for each generated position.
                            // A zero-width AddOneRow at x=8 reaches patches:
                            // its strict left-edge check retains the zero-size
                            // clipped call. Weighted-add on its selected alpha
                            // aliases fg/fga and calls memcpy(null,bg,0).
                            let aliased_weighted = (1..=n).any(|j| {
                                let ec_mode = (mode + j as u32) % 8;
                                let alpha = if n > 1 && j % 2 != 0 { 2.min(n - 1) } else { 0 };
                                ec_mode >= 6 && alpha == j - 1
                            });
                            let source_undefined =
                                render != 0 && (x, width, l, r) == (8, 0, 0, 0) && aliased_weighted;
                            cases.push((
                                read(n, assoc, render, x, width, l, r, prefix, &bytes),
                                source_undefined,
                            ));
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases.len(), 1728);
    assert_eq!(cases.iter().filter(|(_, undefined)| *undefined).count(), 48);
    cases
}

#[test]
fn patches_real_entropy_tree_rows_clipping_and_stage() {
    let cases = patch_tree_cases()
        .into_iter()
        .filter_map(|(request, undefined)| (!undefined).then_some(request))
        .collect();
    chunks(cases);
}
#[test]
fn patches_failures_truncation_checksums_partial_state_and_reuse() {
    let mut cases = Vec::new();
    for n in [0, 1, 3] {
        let t = tokens(n, 4);
        let bytes = fixture(&t, 3);
        for len in 0..=bytes.len() {
            cases.push(read(n, 0, 0, 0, 14, 0, 0, 3, &bytes[..len]));
        }
        for i in 0..bytes.len() {
            for bit in [0, 3, 7] {
                let mut b = bytes.clone();
                b[i] ^= 1 << bit;
                cases.push(read(n, 0, 0, 0, 14, 0, 0, 3, &b));
            }
        }
        let other = fixture(&tokens(n, 1), 0);
        cases.push(format!(
            "patch-read {n} 0 14 10 15 15 3 0 14 0 0 3 0 {} 3 {} 0 {}",
            hex(&other),
            hex(&bytes),
            hex(&other)
        ));
        for len in [0, bytes.len() / 2, bytes.len()] {
            cases.push(format!(
                "patch-read {n} 0 14 10 15 15 1 0 14 0 0 3 0 {} 3 {} 0 {}",
                hex(&other),
                hex(&bytes[..len]),
                hex(&other)
            ));
        }
        for (ctx, index, val) in [
            (0, 0, 2000),
            (1, 1, 4),
            (3, 2, 12),
            (3, 3, 11),
            (2, 4, 20),
            (2, 5, 20),
            (7, 6, 10000),
            (4, 7, 20),
            (4, 8, 20),
            (5, 9, 8),
        ] {
            let mut bad = t.clone();
            assert_eq!(bad[index].0, ctx);
            bad[index].1 = val;
            let b = fixture(&bad, 0);
            cases.push(read(n, 0, 0, 0, 14, 0, 0, 0, &b));
        }
        let empty = fixture(&[(0, 0)], 7);
        cases.push(read(n, 0, 0, 0, 14, 0, 0, 7, &empty));
        for (index, _) in t
            .iter()
            .enumerate()
            .filter(|(_, token)| token.0 == 6)
            .take(2)
        {
            let mut negative = t.clone();
            negative[index].1 = 199;
            let b = fixture(&negative, 0);
            cases.push(read(n, 0, 0, 0, 14, 0, 0, 0, &b));
        }
        if n > 1 {
            let index = t.iter().position(|token| token.0 == 8).unwrap();
            let mut bad = t.clone();
            bad[index].1 = n as u32;
            let b = fixture(&bad, 0);
            cases.push(read(n, 0, 0, 0, 14, 0, 0, 0, &b));
        }
        for (valid, xyb) in [(0, 15), (15, 0), (1, 15)] {
            cases.push(format!(
                "patch-read {n} 0 14 10 {valid} {xyb} 0 0 14 0 0 1 3 {}",
                hex(&bytes)
            ));
        }
    }
    chunks(cases);
}
fn patch_blend_cases() -> (Vec<String>, Vec<String>) {
    let mut cases = Vec::new();
    let mut upstream_undefined = Vec::new();
    let values = [
        -4.0f32,
        -0.0,
        0.0,
        0.25,
        0.75,
        1.0,
        2.0,
        f32::MIN_POSITIVE,
        f32::from_bits(1),
        f32::MAX,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];
    for n in [0, 1, 3] {
        for assoc in [0, 5] {
            for mode in 0..8 {
                for clamp in 0..2 {
                    for variant in 0..4 {
                        let width = if variant == 0 { 0 } else { 9 };
                        let x0 = variant % 3;
                        let alpha = if n > 2 { 2 } else { 0 };
                        let mut req =
                            format!("patch-blend {n} {assoc} {width} {x0} {mode} {alpha} {clamp}");
                        for i in 0..n {
                            write!(
                                req,
                                " {} {} {}",
                                (mode + i) % 8,
                                if n > 2 { i % 3 } else { 0 },
                                clamp
                            )
                            .unwrap();
                        }
                        for which in 0..2 {
                            for c in 0..3 + n {
                                for x in 0..x0 + width + 2 {
                                    let v =
                                        values[(x * 3 + c + which * 5 + variant) % values.len()];
                                    write!(req, " {}", v.to_bits()).unwrap();
                                }
                            }
                        }
                        // The original zero-width temporary has null rows.
                        // Its aliased weighted-add branch calls memcpy with a
                        // null destination even for count 0 (alpha.cc:70).
                        // Preserve these exact requests as explicit upstream
                        // UB probes, rather than claiming original pixel words.
                        if width == 0 && (0..n).any(|i| (mode + i) % 8 >= 6) {
                            upstream_undefined.push(req);
                        } else {
                            cases.push(req);
                        }
                    }
                }
            }
        }
    }
    assert_eq!((cases.len(), upstream_undefined.len()), (360, 24));
    assert_eq!(
        cases
            .iter()
            .filter(|s| s.split_whitespace().nth(3) == Some("0"))
            .count(),
        72
    );
    (cases, upstream_undefined)
}

#[test]
fn patches_all_blends_alpha_aliases_and_float_words() {
    let (cases, _) = patch_blend_cases();
    chunks(cases);
}

#[test]
fn patches_original_zero_width_alias_memcpy_undefined_boundary() {
    let (_, mut cases) = patch_blend_cases();
    cases.extend(
        patch_tree_cases()
            .into_iter()
            .filter_map(|(request, undefined)| undefined.then_some(request)),
    );
    assert_eq!(cases.len(), 72);
    let instrumented = original_observations(&["patch-sanitizers".into()])[0] == "1";
    let oracle = std::env::var_os("JXL_CPP_ORACLE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/oracle/jxl_oracle")
        });
    let mut diagnosed = 0;
    for request in &cases {
        let mut child = Command::new(&oracle)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start original UB boundary probe");
        writeln!(child.stdin.take().unwrap(), "{request}").expect("write exact UB request");
        let output = child
            .wait_with_output()
            .expect("observe original UB boundary");
        let stderr = String::from_utf8_lossy(&output.stderr);
        let diagnostic: Vec<_> = stderr.lines().filter(|line| !line.is_empty()).collect();
        let null_memcpy = diagnostic.len() == 3
            && diagnostic[0].contains("alpha.cc:70:")
            && diagnostic[0].ends_with(
                "runtime error: null pointer passed as argument 1, which is declared to never be null",
            )
            && diagnostic[1].ends_with("note: nonnull attribute specified here")
            && diagnostic[2].contains("SUMMARY: UndefinedBehaviorSanitizer: undefined-behavior ")
            && diagnostic[2].trim_end().ends_with("alpha.cc:70:12");
        if null_memcpy {
            assert!(
                !output.status.success(),
                "original non-recovering sanitizer returned success: {stderr}"
            );
            diagnosed += 1;
            println!("Original undefined zero-width alias request: {request}\n{stderr}");
        } else {
            assert!(
                output.status.success() && stderr.is_empty(),
                "unexpected original UB probe failure: {stderr}"
            );
        }
        if cfg!(target_os = "linux") && instrumented {
            assert!(
                null_memcpy && !output.status.success(),
                "native Linux instrumented source must diagnose alpha.cc:70: {request}\n{stderr}"
            );
        }
        // No comparison with Rust or assertion about source pixel semantics:
        // a successful execution on another graph does not define this call.
    }
    println!(
        "72 original undefined boundary probes exercised (24 blend, 48 tree); \
              {diagnosed} diagnosed; 360 defined blend and 1680 defined tree \
              requests remain exact parity"
    );
}

#[test]
fn patches_alpha_kernels_aliases_rounding_and_exceptional_values() {
    let mut cases = Vec::new();
    let words = [
        0u32, 0x80000000, 0x3e800000, 0x3f400000, 0x3f800000, 0x40000000, 0xc0800000, 0x00800000,
        1, 0x7f7fffff, 0x7f800000, 0xff800000, 0x7fc12345, 0xffc98765, 0x7fa12345, 0xffa98765,
    ];
    let finite_words = [
        0u32, 0x80000000, 0x3e800000, 0x3f400000, 0x3f800000, 0x40000000, 0xc0800000, 0x00800000,
        1, 0x7f7fffff, 0xff7fffff, 0x3eaaaaab, 0xbecccccd, 0x80800000, 0x00800001, 0x80000001,
    ];
    for op in 0..6 {
        for premul in 0..2 {
            for clamp in 0..2 {
                for alias in 0..4 {
                    for (words, variants, finite) in
                        [(&words[..], 4, false), (&finite_words[..], 16, true)]
                    {
                        for variant in 0..variants {
                            let (bi, bai, fi, fai) = match alias {
                                0 => (0, 1, 2, 3),
                                1 => (0, 0, 2, 2),
                                2 => (0, 1, 2, 2),
                                _ => (0, 0, 2, 3),
                            };
                            let n = if finite {
                                19
                            } else if variant == 0 {
                                0
                            } else {
                                17
                            };
                            let mut req = format!(
                                "patch-alpha {op} {premul} {clamp} {n} {bi} {bai} {fi} {fai}"
                            );
                            for c in 0..5 {
                                for x in 0..n {
                                    write!(
                                        req,
                                        " {}",
                                        words[(x * 3 + c * 5 + variant) % words.len()]
                                    )
                                    .unwrap();
                                }
                            }
                            cases.push(req);
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases.len(), 1920);
    chunks(cases);
}

#[test]
fn patches_independent_rgba_competing_words_and_stage_regression() {
    let words = [
        0u32, 0x80000000, 0x3f800000, 0xbf800000, 0x3f000000, 0x40000000, 0x7fc12345, 0xffc23456,
        0x7fa12345, 0xffa23456, 0x7f800000, 0xff800000, 1, 0x80000001, 0x3f7fffff, 0x3f800001,
    ];
    let mut cases = Vec::new();
    for premul in 0..2 {
        for clamp in 0..2 {
            // The independent original BlendingStage planes that exposed the
            // gap: bg RGB=Pixel(0,c,x,0), bg alpha=Pixel(1,3,x,0), and
            // fg=Pixel(8,c,OFFSET+x,0). The different alpha reference is real
            // BlendingStage behavior. Retain that split and a common reference,
            // with both original native OFFSET row layouts on every target.
            for offset in [24, 32] {
                for alpha_reference in [0, 1] {
                    let mut request = format!("patch-rgba {premul} {clamp} 17");
                    for which in 0..2 {
                        for c in 0..4 {
                            for x in 0..17 {
                                let index = which * (8 * 7 + offset)
                                    + usize::from(which == 0 && c == 3) * alpha_reference * 7
                                    + c * 5
                                    + x;
                                write!(request, " {}", words[index % words.len()]).unwrap();
                            }
                        }
                    }
                    cases.push(request);
                }
            }
            // Every pair of the four source operands (bg, ba, fg, fa) varies
            // independently, including both signs and quiet/signaling NaNs.
            let mut samples = Vec::new();
            for first in 0..4 {
                for second in first + 1..4 {
                    for &a in &words {
                        for &b in &words {
                            let mut sample = [0x3f400000, 0x3f000000, 0x3e800000, 0x3f400000];
                            sample[first] = a;
                            sample[second] = b;
                            samples.push(sample);
                        }
                    }
                }
            }
            let mut request = format!("patch-rgba {premul} {clamp} {}", samples.len());
            for c in 0..8 {
                let operand = match c {
                    0..=2 => 0,
                    3 => 1,
                    4..=6 => 2,
                    _ => 3,
                };
                for sample in &samples {
                    write!(request, " {}", sample[operand]).unwrap();
                }
            }
            cases.push(request);
        }
    }
    assert_eq!(cases.len(), 20);
    chunks(cases);
}

#[test]
fn patches_weighted_add_and_multiply_competing_operand_pairs() {
    let words = [
        0u32, 0x80000000, 0x3f800000, 0xbf800000, 0x3f000000, 0x40000000, 0x7fc12345, 0xffc23456,
        0x7fa12345, 0xffa23456, 0x7f800000, 0xff800000, 1, 0x80000001, 0x3f7fffff, 0x3f800001,
    ];
    let mut samples = Vec::new();
    for first in 0..4 {
        for second in first + 1..4 {
            for &a in &words {
                for &b in &words {
                    let mut sample = [0x3f400000, 0x3f000000, 0x3e800000, 0x3f400000, 0x3f800000];
                    sample[first] = a;
                    sample[second] = b;
                    samples.push(sample);
                }
            }
        }
    }
    let mut cases = Vec::new();
    for op in [2, 3] {
        for clamp in 0..2 {
            let mut request = format!("patch-alpha {op} 0 {clamp} {} 0 1 2 3", samples.len());
            for c in 0..5 {
                for sample in &samples {
                    write!(request, " {}", sample[c]).unwrap();
                }
            }
            cases.push(request);
        }
    }
    assert_eq!(cases.len(), 4);
    chunks(cases);
}

#[test]
fn patches_needs_blending_retains_frame_and_extra_channel_branches() {
    let mut cases = Vec::new();
    for frame_type in 0..4 {
        for custom in 0..2 {
            for mode in 0..5 {
                for n in 0..4 {
                    for ec_mode in 0..5 {
                        let mut req = format!("patch-needs {frame_type} {custom} {mode} {n}");
                        for i in 0..n {
                            write!(req, " {}", if i % 2 == 0 { ec_mode } else { 0 }).unwrap();
                        }
                        cases.push(req);
                    }
                }
            }
        }
    }
    chunks(cases);
}

#[test]
fn patches_interval_tree_nested_ties_and_disjoint_rows() {
    use jxl::jxl::pack_signed::pack_signed;
    let mut cases = Vec::new();
    let mut seed = 8371;
    for variant in 0..64 {
        let n = if variant % 2 == 0 { 0 } else { 3 };
        let mut t = vec![(0, 4)];
        for r in 0..4 {
            let pw = 1 + (next(&mut seed) % 5) as u32;
            let ph = 1 + (next(&mut seed) % 5) as u32;
            let count = 1 + (next(&mut seed) % 10) as u32;
            t.extend([
                (1, r),
                (3, 0),
                (3, 0),
                (2, pw - 1),
                (2, ph - 1),
                (7, count - 1),
            ]);
            let mut last = (0, 0);
            for i in 0..count {
                let x = (next(&mut seed) % (15 - u64::from(pw))) as i32;
                let y = (next(&mut seed) % (11 - u64::from(ph))) as i32;
                if i == 0 {
                    t.extend([(4, x as u32), (4, y as u32)]);
                } else {
                    t.extend([(6, pack_signed(x - last.0)), (6, pack_signed(y - last.1))]);
                }
                last = (x, y);
                for c in 0..n + 1 {
                    let mode = ((variant + c) % 8) as u32;
                    t.push((5, mode));
                    if mode >= 4 && n > 1 {
                        t.push((8, (c % 3) as u32));
                    }
                    if mode >= 3 {
                        t.push((9, (variant % 2) as u32));
                    }
                }
            }
        }
        let prefix = variant % 8;
        let bytes = fixture(&t, prefix);
        for render in [0, 1, 2] {
            cases.push(read(n, 5, render, 0, 14, 0, 0, prefix, &bytes));
        }
    }
    chunks(cases);
}

#[test]
fn patches_libcpp_tree_sort_preserves_large_equal_key_order() {
    use jxl::jxl::pack_signed::pack_signed;
    let mut cases = Vec::new();
    // Exercise small insertion, partition, and >128-element ninther paths of
    // the actual reference libc++ non-arithmetic interval specialization.
    for count in [
        23usize, 24, 25, 31, 32, 33, 63, 64, 65, 127, 128, 129, 257, 513,
    ] {
        for variant in 0..4 {
            let mut t = vec![(0, 4)];
            for reference in 0..4 {
                let ph = 1 + 2 * reference;
                let pw = 1 + reference;
                let occurrences = count / 4 + usize::from(reference < count % 4);
                t.extend([
                    (1, reference as u32),
                    (3, 0),
                    (3, 0),
                    (2, pw as u32 - 1),
                    (2, ph as u32 - 1),
                    (7, occurrences as u32 - 1),
                ]);
                let mut previous = (0i32, 0i32);
                for i in 0..occurrences {
                    let x = ((i + reference) % (15 - pw)) as i32;
                    let y = (match variant {
                        0 => i % (11 - ph),
                        1 => (i / 3) % (11 - ph),
                        2 => (occurrences - 1 - i) % (11 - ph),
                        _ => usize::from(i % 7 == 0) * (10 - ph),
                    }) as i32;
                    if i == 0 {
                        t.extend([(4, x as u32), (4, y as u32)]);
                    } else {
                        t.extend([
                            (6, pack_signed(x - previous.0)),
                            (6, pack_signed(y - previous.1)),
                        ]);
                    }
                    previous = (x, y);
                    t.push((5, 2));
                }
            }
            let prefix = variant * 2;
            let bytes = fixture(&t, prefix);
            cases.push(read(0, 0, 0, 0, 14, 0, 0, prefix, &bytes));
        }
    }
    chunks(cases);
}

#[test]
fn patches_padded_stage_coordinates_keep_size_t_wrapping_and_clipping() {
    let mut cases = Vec::new();
    for n in [0, 1, 3] {
        for mode in [1, 4] {
            let bytes = fixture(&tokens(n, mode), 3);
            for (x, width, left, right) in [
                (0, 14, 1, 0),
                (0, 14, 3, 2),
                (1, 5, 2, 1),
                (1, 0, 3, 0),
                (0, 0, 1, 1),
            ] {
                for render in [1, 2, 3] {
                    cases.push(read(n, 0, render, x, width, left, right, 3, &bytes));
                }
            }
        }
    }
    chunks(cases);
}
