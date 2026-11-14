// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete-file differential evidence for LowMemoryRenderPipeline. The oracle
//! constructs the original FrameDecoder with use_slow_rendering_pipeline=false,
//! as decode.cc does, and the Rust sequence decoder makes the same selection.
//! Every file is a genuine output of the pinned original encoder: stored
//! fixtures were produced by it, the others are encoded during the test.
//! Observations compare every initialized output, reference and DC sample word.
use super::*;

/// The C++ and Rust observations of one request kind.
fn low_memory(bytes: &[u8]) -> String {
    format!("fs-decode-lm {}", hex(bytes))
}
fn simple(bytes: &[u8]) -> String {
    format!("fs-decode {}", hex(bytes))
}
/// The final sequence status; metadata failures return before " E ".
fn end_status(observation: &str) -> Option<&str> {
    let words: Vec<_> = observation.split_whitespace().collect();
    let end = words.iter().rposition(|&w| w == "E")?;
    words.get(end + 1).copied()
}
fn first_difference(rust: &str, original: &str) -> String {
    let a: Vec<_> = rust.split_whitespace().collect();
    let b: Vec<_> = original.split_whitespace().collect();
    let differences: Vec<_> = a
        .iter()
        .zip(&b)
        .enumerate()
        .filter(|(_, (a, b))| a != b)
        .take(16)
        .collect();
    format!(
        "lengths {}/{}; differences {differences:?}",
        a.len(),
        b.len()
    )
}

#[derive(Default)]
struct Totals {
    files: usize,
    complete: usize,
    agreeing: usize,
    // Inputs on which the original simple and low-memory pipelines differ;
    // each one is still compared exactly against its own original pipeline.
    differing: usize,
}

/// For each input: the Rust low-memory observation must equal the original
/// low-memory observation exactly. Where the original simple and low-memory
/// pipelines agree, the Rust simple and low-memory observations agree too.
fn exact(label: &str, inputs: &[Vec<u8>], require_complete: bool) -> Totals {
    exact_with(label, inputs, require_complete, true)
}

/// `with_simple` is false only for inputs on which the original simple
/// pipeline has no defined result, so only the low-memory pipeline is run.
fn exact_with(
    label: &str,
    inputs: &[Vec<u8>],
    require_complete: bool,
    with_simple: bool,
) -> Totals {
    let mut totals = Totals::default();
    for chunk in inputs.chunks(8) {
        let requests: Vec<_> = chunk
            .iter()
            .flat_map(|bytes| [low_memory(bytes), simple(bytes)])
            .collect();
        let originals = if with_simple {
            original_observations(&requests)
        } else {
            let low_memory: Vec<_> = requests.iter().step_by(2).cloned().collect();
            original_observations(&low_memory)
                .into_iter()
                .flat_map(|observation| [observation, String::new()])
                .collect()
        };
        for (pair, cpp) in requests.chunks(2).zip(originals.chunks(2)) {
            let (cpp_low_memory, cpp_simple) = (&cpp[0], &cpp[1]);
            let complete = end_status(cpp_low_memory) == Some("0");
            if require_complete {
                assert!(complete, "{label}: complete original file {}", pair[0]);
            }
            let rust_low_memory = super::observe(&pair[0]);
            assert!(
                rust_low_memory == *cpp_low_memory,
                "{label}: {}: {}",
                pair[0],
                first_difference(&rust_low_memory, cpp_low_memory)
            );
            if !with_simple {
                totals.differing += 1;
            } else if cpp_simple == cpp_low_memory {
                let rust_simple = super::observe(&pair[1]);
                assert!(
                    rust_simple == rust_low_memory,
                    "{label}: simple/low-memory agreement {}: {}",
                    pair[1],
                    first_difference(&rust_simple, &rust_low_memory)
                );
                totals.agreeing += 1;
            } else {
                totals.differing += 1;
            }
            totals.files += 1;
            totals.complete += usize::from(complete);
        }
    }
    println!(
        "{label}: {} C++/Rust low-memory cases matched ({} complete); {} also match both simple pipelines",
        totals.files, totals.complete, totals.agreeing
    );
    totals
}

fn truncations(bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut cases = Vec::new();
    let mut cuts = vec![
        bytes.len() / 4,
        bytes.len() / 2,
        bytes.len() * 3 / 4,
        bytes.len() - 1,
    ];
    cuts.sort_unstable();
    cuts.dedup();
    for cut in cuts {
        cases.push(bytes[..cut].to_vec());
    }
    for index in [bytes.len() / 2, bytes.len() - 1] {
        for mask in [1, 0x80] {
            let mut broken = bytes.to_vec();
            broken[index] ^= mask;
            cases.push(broken);
        }
    }
    let mut trailing = bytes.to_vec();
    trailing.push(0);
    cases.push(trailing);
    cases
}

fn sequence_inputs(encoded: &frame_sequences::EncodedSequence) -> Vec<Vec<u8>> {
    frame_sequences::sequence_variants(encoded)
        .into_iter()
        .map(|request| unhex(request.strip_prefix("fs-decode ").unwrap()))
        .collect()
}

#[test]
fn low_memory_reference_blending_crop_animation_and_preview_sequences_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let fixtures: Vec<_> = include_str!("../data/frame-sequences.requests")
        .lines()
        .map(|line| line.split_once('\t').unwrap())
        .collect();
    let encoded = original_observations(
        &fixtures
            .iter()
            .map(|(_, request)| request.to_string())
            .collect::<Vec<_>>(),
    );
    let mut complete = Vec::new();
    let mut variants = Vec::new();
    for ((name, _), result) in fixtures.iter().zip(encoded) {
        let mut fields = result.split_whitespace();
        let status: i32 = fields.next().unwrap().parse().unwrap();
        if status != 0 {
            assert!(name.starts_with("encoder-unsupported-dc-"));
            continue;
        }
        let bytes = unhex(fields.next().unwrap());
        let header: usize = fields.next().unwrap().parse().unwrap();
        let n: usize = fields.next().unwrap().parse().unwrap();
        let ends = (0..n)
            .map(|_| fields.next().unwrap().parse().unwrap())
            .collect();
        let sequence = frame_sequences::EncodedSequence {
            bytes,
            header,
            ends,
            status,
        };
        complete.push(sequence.bytes.clone());
        if name.starts_with("crop-blend-")
            || name.starts_with("reference-overwrite-")
            || name.starts_with("preview-main-1-")
        {
            variants.extend(sequence_inputs(&sequence));
        }
    }
    assert_eq!(complete.len(), 111);
    // Two invalid-reference files are genuine encodings whose decode fails at
    // the original reference check; their partial state is compared exactly.
    let totals = exact("sequences", &complete, false);
    assert_eq!((totals.complete, totals.agreeing), (109, 109));
    exact("sequence errors", &variants, false);
}

/// Request a genuine two-frame encoding whose blended second frame has the
/// given origin and size, derived from the stored crop-blend requests. Token
/// indices follow tools/oracle/frame_sequences.cc: nine sequence fields, then
/// nineteen fields per frame with the origin and size at offsets 10..14.
fn cropped_request(
    base: &str,
    image: (usize, usize),
    origin: (i32, i32),
    size: (usize, usize),
) -> String {
    let mut tokens: Vec<String> = base.split_whitespace().map(str::to_owned).collect();
    assert_eq!(tokens.len(), 1 + 9 + 2 * 19);
    tokens[1] = image.0.to_string();
    tokens[2] = image.1.to_string();
    tokens[22] = image.0.to_string();
    tokens[23] = image.1.to_string();
    tokens[39] = origin.0.to_string();
    tokens[40] = origin.1.to_string();
    tokens[41] = size.0.to_string();
    tokens[42] = size.1.to_string();
    tokens.join(" ")
}

#[test]
fn low_memory_frames_outside_and_across_image_bounds_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let bases: Vec<_> = include_str!("../data/frame-sequences.requests")
        .lines()
        .map(|line| line.split_once('\t').unwrap())
        .filter(|(name, _)| matches!(*name, "crop-blend-0-0-0" | "crop-blend-1-3-4"))
        .map(|(_, request)| request)
        .collect();
    assert_eq!(bases.len(), 2);
    let mut requests = Vec::new();
    let mut beyond = Vec::new();
    for base in &bases {
        // Entirely right of or below the image: RenderPadding then initializes
        // the whole image area from the first group. The original simple
        // pipeline's foreground copy has no defined result for these frames
        // (its sizes wrap and the reference build crashes), so they have no
        // simple-pipeline comparison.
        for origin in [(20, 2), (2, 9)] {
            beyond.push(cropped_request(base, (9, 7), origin, (5, 3)));
        }
        // Entirely left of or above the image, and partial overlaps that clip
        // either side, including negative origins.
        for origin in [(-10, 2), (3, -5), (6, -2), (-3, -2), (7, 5)] {
            requests.push(cropped_request(base, (9, 7), origin, (5, 3)));
        }
        // Several groups of the cropped frame and of the image, with padding
        // rendered on every side of the intersecting groups.
        for (origin, size) in [
            ((40, 15), (270, 10)),
            ((-30, -4), (290, 300)),
            ((-300, 3), (290, 9)),
        ] {
            requests.push(cropped_request(base, (300, 260), origin, size));
        }
    }
    let encode = |requests: &[String]| -> Vec<Vec<u8>> {
        original_observations(requests)
            .into_iter()
            .map(|result| {
                let mut fields = result.split_whitespace();
                assert_eq!(fields.next(), Some("0"), "genuine source encoder");
                unhex(fields.next().unwrap())
            })
            .collect()
    };
    let files = encode(&requests);
    let totals = exact("crops", &files, true);
    // For frames entirely left of or above the image, the original simple
    // pipeline's results differ from the low-memory pipeline's: it rejects the
    // multigroup frame and keeps different samples for the small ones.
    assert_eq!((totals.files, totals.differing), (16, 6));
    let outside = encode(&beyond);
    let totals = exact_with("frames beyond the image", &outside, true, false);
    assert_eq!(totals.files, 4);
    let mut variants = Vec::new();
    for bytes in files[5..8].iter().chain(&files[13..16]) {
        variants.extend(truncations(bytes));
    }
    exact("crop errors", &variants, false);
}

#[test]
fn low_memory_upsampling_multigroup_and_dc_ladder_files_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut complete = Vec::new();
    let mut variants = Vec::new();
    for prefix in ["resampling-", "multigroup-", "dc-ladder-base"] {
        for (name, _, encoded) in frame_sequences::extended_encodings(prefix) {
            assert_eq!(encoded.status, 0, "genuine source encoder: {name}");
            complete.push(encoded.bytes.clone());
            variants.extend(sequence_inputs(&encoded));
        }
    }
    assert_eq!(complete.len(), 59);
    let totals = exact("upsampling and multigroup", &complete, false);
    // The original simple pipeline rejects six odd-width modular files with
    // early extra-channel upsampling; the original low-memory pipeline renders
    // them, and Rust reproduces each original result.
    assert_eq!(
        (totals.complete, totals.agreeing, totals.differing),
        (59, 53, 6)
    );
    exact("upsampling and multigroup errors", &variants, false);
}

#[test]
fn low_memory_chroma_ycbcr_crop_and_extra_channel_files_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let geometry: Vec<_> = decoder_geometry::bases()
        .into_iter()
        .map(|(_, bytes)| bytes)
        .collect();
    assert_eq!(geometry.len(), 148);
    exact("chroma and geometry", &geometry, true);
    let chroma: Vec<_> = chroma_image::fixtures()
        .iter()
        .map(|(_, bytes)| bytes.to_vec())
        .collect();
    exact("JPEG chroma", &chroma, true);
    let extra = extra_channels::fixtures();
    let mut variants = Vec::new();
    for (name, bytes) in &extra {
        if name.contains("groups") || name.starts_with("reference-1-") || name == "xyb-filters" {
            variants.extend(truncations(bytes));
        }
    }
    let totals = exact(
        "extra channels",
        &extra
            .into_iter()
            .map(|(_, bytes)| bytes)
            .collect::<Vec<_>>(),
        false,
    );
    // The original simple pipeline rejects 37 early-upsampled alpha files,
    // which the low-memory pipeline renders; five reference-1 files fail in
    // both with different partial state.
    assert_eq!(
        (totals.files, totals.complete, totals.differing),
        (111, 106, 42)
    );
    exact("extra-channel errors", &variants, false);
}

#[test]
fn low_memory_vardct_epf_gaborish_noise_and_spline_files_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut files = Vec::new();
    for epf in 0..=3 {
        for (i, size) in [(1, 1), (3, 8), (17, 9), (65, 33)].into_iter().enumerate() {
            for (gray, alpha) in [(false, false), (true, true)] {
                let (bytes, _) = decoder_effects::encode_with_splines(
                    size,
                    (gray, alpha),
                    if i % 2 == 0 { 8 } else { 16 },
                    (epf, (i + epf) % 2 != 0, i % 2 != 0, i % 3 == 1),
                    i,
                    0,
                );
                files.push(bytes);
            }
        }
        // Several groups and DC groups: group borders, border storage and the
        // GroupBorderAssigner's corner/edge scheduling of every filter.
        for (j, size) in [(258, 258), (513, 257), (2049, 3), (3, 1030)]
            .into_iter()
            .enumerate()
        {
            for gab in [false, true] {
                let (bytes, _) = decoder_effects::encode_with_splines(
                    size,
                    (false, j % 2 == 1),
                    8,
                    (epf, gab, true, j == 1),
                    1,
                    0,
                );
                files.push(bytes);
            }
        }
    }
    for epf in [0, 3] {
        for (i, size) in [(17, 9), (65, 33), (258, 3), (258, 258), (513, 257)]
            .into_iter()
            .enumerate()
        {
            let (bytes, _) = decoder_effects::encode_with_splines(
                size,
                (i % 2 == 1, true),
                if i % 2 == 0 { 8 } else { 16 },
                (epf, i % 2 != 0, true, i % 2 == 0),
                i % 4,
                i % 2 + 1,
            );
            files.push(bytes);
        }
    }
    let totals = exact("VarDCT effects", &files, true);
    assert_eq!((totals.files, totals.agreeing), (74, 74));
    let mut variants = Vec::new();
    for (size, effects, splines) in [
        ((258, 258), (3, true, true, true), 2),
        ((513, 257), (1, false, true, true), 1),
    ] {
        let (bytes, payload) =
            decoder_effects::encode_with_splines(size, (false, true), 8, effects, 3, splines);
        variants.extend(truncations(&bytes));
        for index in [payload, payload + (bytes.len() - payload) / 3] {
            let mut broken = bytes.clone();
            broken[index] ^= 0x10;
            variants.push(broken);
        }
    }
    exact("VarDCT effect errors", &variants, false);
}

#[test]
fn low_memory_patch_rendering_matches_original_on_every_main_frame_failure() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let bytes = include_bytes!("../data/repeated-glyphs-modular-none.jxl").to_vec();
    exact("patches", std::slice::from_ref(&bytes), true);
    // The main frame occupies bytes 40..112; each truncation and bit flip
    // keeps the original partial output and the completed reference frame.
    let mut variants = Vec::new();
    for index in 40..bytes.len() {
        variants.push(bytes[..index].to_vec());
        let mut broken = bytes.clone();
        broken[index] ^= 0x80;
        variants.push(broken);
    }
    exact("patch errors", &variants, false);
}
