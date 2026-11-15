// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Encoder lossless streaming (issue #223): the lossless Modular profile of
// the public encoder at effort 2 (kThunder) against the original public C
// API, one-shot and through EncodeFrameStreaming, which the original selects
// by default (buffering -1) for more than 8 groups and for buffering 1 above
// 2048 pixels per side, 2 and 3. A child module of `encoder_public`, whose
// epub-run drivers and comparisons it reuses: every history compares every
// status, error, output chunk and output-processor call with the original's.
// The expected bytes are always the original's, from the oracle on the same
// host; nothing is recorded from the port.

use super::driver;
use super::{compare_admitted, compare_admitted_processor, compare_gate, output_schedule, run};

/// One lossless history: basic info with `uses_original_profile`, an
/// optional sRGB color encoding, frame settings with effort 2 and
/// JxlEncoderSetFrameLossless plus `settings`, one frame of generated
/// content and an output stage. `depth` is 8 or 16 bits; `alpha` adds one
/// interleaved unassociated alpha channel at the color depth with the
/// content generator's alpha mode 2 (transparent stripes, whose invisible
/// colors lossless keeps) or 3. `output` is `None` for buffer mode, which
/// takes chunks of at most 4 MiB, or an output processor (`w` operation)
/// set after CloseInput and flushed with FlushInput. `prefix` holds
/// encoder-level operations (container, level).
#[expect(clippy::too_many_arguments, reason = "One history's fields")]
fn lossless_history(
    prefix: &str,
    xsize: usize,
    ysize: usize,
    gray: bool,
    depth: usize,
    alpha: bool,
    content: usize,
    variant: usize,
    settings: &str,
    processor: Option<&str>,
) -> String {
    let (data_type, bytes) = if depth == 16 { (3, 2) } else { (2, 1) };
    let channels = if gray { 1 } else { 3 } + usize::from(alpha);
    let endianness = variant % 3;
    let align = [0, 1, 4, 16, 3][variant % 5];
    let row = xsize * channels * bytes;
    let stride = if align < 2 || row.is_multiple_of(align) {
        row
    } else {
        row + align - row % align
    };
    let size = stride * (ysize - 1) + row;
    let mut info = format!("Bw:{xsize},h:{ysize},orig:1");
    if depth == 16 {
        info.push_str(",bits:16");
    }
    if alpha {
        info.push_str(&format!(",ec:1,ab:{depth}"));
    }
    if gray {
        info.push_str(",c:1");
    }
    let color = match variant % 2 {
        0 => String::new(),
        _ => format!(" G{}", u8::from(gray)),
    };
    let content = content + if alpha { 16 * (2 + variant % 2) } else { 0 };
    let frame =
        format!("a0,{channels},{data_type},{endianness},{align},{size},{content},{xsize},{ysize}");
    let output = match processor {
        Some(processor) => format!("{processor} i r"),
        None => {
            let chunk = 4 << 20;
            let count = (size + size / 4 + 4096).div_ceil(chunk) + 1;
            if size < chunk {
                output_schedule(size + size / 4 + 4096)
            } else {
                vec![format!("p{chunk}"); count].join(" ")
            }
        }
    };
    run(
        0,
        0,
        &format!("{prefix}{info}{color} s- o0,0,2 y0,1{settings} {frame} Z {output}"),
    )
}

/// The buffering and output-mode settings of one history; -1 leaves the
/// option at its default.
fn buffering_settings(buffering: i32, output_mode: i32) -> String {
    let mut settings = String::new();
    if buffering != -1 {
        settings.push_str(&format!(" o0,34,{buffering}"));
    }
    if output_mode != -1 {
        settings.push_str(&format!(" o0,40,{output_mode}"));
    }
    settings
}

/// Splits buffer-mode from processor-mode histories and requires every one
/// to be exact and encoded by the original.
fn compare_lossless(family: &str, requests: Vec<String>) {
    let (buffer, processor): (Vec<String>, Vec<String>) = requests
        .into_iter()
        .partition(|request| !request.contains(" i r"));
    if !buffer.is_empty() {
        compare_admitted(family, buffer);
    }
    if !processor.is_empty() {
        compare_admitted_processor(&format!("{family}-processor"), processor);
    }
}

/// The one-shot path (EncodeFrameOneShot): at most 8 groups, or buffering
/// 0, or buffering 1 up to 2048 pixels per side; with buffering 3 the
/// original learns no global tree and codes each group with its own
/// (ModularGenericCompress). Gray and RGB, 8 and 16 bits (16-bit samples
/// need level 10, which the original signals in a container), with and
/// without alpha, every content of the generator, from one pixel to 8
/// groups, and the group-size heuristic's 128, 256 and 512 group sizes.
#[test]
fn public_encoder_lossless_one_shot_matches_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let sizes = [
        (1, 1),
        (7, 7),
        (9, 17),
        (128, 128),
        (129, 64),
        (256, 256),
        (257, 255),
        (400, 400),
        (401, 300),
        (512, 1024),
        (768, 512),
        (1031, 517),
    ];
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize) in &sizes {
        for gray in [false, true] {
            for depth in [8, 16] {
                for alpha in [false, true] {
                    // 1031x517 has 10 groups: one-shot only with buffering 0
                    // or 1 (both sides at most 2048).
                    let buffering = if xsize * ysize > 768 * 512 {
                        [0, 1][variant % 2]
                    } else {
                        [-1, 0, 1, 2, 3][variant % 5]
                    };
                    let processor = (variant % 4 == 3).then_some("w0,0,0,0");
                    requests.push(lossless_history(
                        "",
                        xsize,
                        ysize,
                        gray,
                        depth,
                        alpha,
                        1 + variant % 4,
                        variant,
                        &buffering_settings(buffering, -1),
                        processor,
                    ));
                    variant += 1;
                }
            }
        }
    }
    compare_lossless("lossless-one-shot", requests);
}

/// The streaming path (EncodeFrameStreaming, local trees and histograms per
/// DC group): more than 8 groups at buffering -1, 2 and 3, and 1 above 2048
/// pixels per side; output modes 0 (sections collected) and 1 (TOC written
/// after the groups through seeks), buffer mode and output processors with
/// and without seek support. Gray and RGB, 8 and 16 bits, with and without
/// alpha.
#[test]
fn public_encoder_lossless_streaming_matches_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let cases = [
        (768, 768, -1, -1),
        (768, 769, 2, -1),
        (1000, 700, -1, 1),
        (1031, 517, 3, -1),
        (1031, 517, 2, 0),
        (2049, 300, 1, -1),
        (2049, 300, 2, 1),
        (300, 2049, 1, 0),
        (1024, 1024, 3, 0),
        (1024, 1024, -1, 1),
    ];
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize, buffering, output_mode) in &cases {
        for gray in [false, true] {
            for (depth, alpha) in [(8, false), (16, true), (8, true), (16, false)] {
                let processor = match variant % 3 {
                    0 => None,
                    1 => Some("w0,0,0,0"),
                    _ => Some("w4,0,0,0"),
                };
                requests.push(lossless_history(
                    "",
                    xsize,
                    ysize,
                    gray,
                    depth,
                    alpha,
                    1 + variant % 4,
                    variant,
                    &buffering_settings(buffering, output_mode),
                    processor,
                ));
                variant += 1;
            }
        }
    }
    compare_lossless("lossless-streaming", requests);
}

/// The gate histories of encoder step 2 for one size, at the lossless
/// profile: gray and RGB, 8 and 16 bits with and without alpha, buffer mode
/// and an output processor, rotating the buffering values -1 to 3 and the
/// output modes -1, 1 and 0, so that each size covers every buffering value
/// and both output modes.
fn lossless_gate_histories(xsize: usize, ysize: usize, seed: usize) -> Vec<String> {
    let mut requests = Vec::new();
    let mut k = seed;
    for (depth, alpha) in [(8, false), (8, true), (16, false), (16, true)] {
        for gray in [false, true] {
            for processor in [false, true] {
                let buffering = [-1, 0, 1, 2, 3][k % 5];
                let output_mode = [-1, 1, 0][k % 3];
                let callbacks = format!("w{},0,0,0", if k.is_multiple_of(2) { 0 } else { 4 });
                requests.push(lossless_history(
                    "",
                    xsize,
                    ysize,
                    gray,
                    depth,
                    alpha,
                    // Photo-like content (4) and the gradient (2).
                    [4, 2][k % 2],
                    k,
                    &buffering_settings(buffering, output_mode),
                    processor.then_some(callbacks.as_str()),
                ));
                k += 1;
            }
        }
    }
    requests
}

/// Encoder step 2's gate sizes at the lossless profile: 768x768, 1024x1024
/// and 2048x1536.
#[test]
fn public_encoder_lossless_gate_sizes_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut requests = Vec::new();
    for (index, (xsize, ysize)) in [(768, 768), (1024, 1024), (2048, 1536)]
        .into_iter()
        .enumerate()
    {
        requests.extend(lossless_gate_histories(xsize, ysize, 3 * index));
    }
    compare_gate("lossless-gate", requests);
}

/// Encoder step 2's gate at 2049x2049, lossless.
#[test]
fn public_encoder_lossless_gate_2049_matches_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    compare_gate("lossless-gate-2049", lossless_gate_histories(2049, 2049, 1));
}

/// Encoder step 2's gate at 4000x3000, lossless.
#[test]
fn public_encoder_lossless_gate_4000x3000_matches_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    compare_gate(
        "lossless-gate-4000x3000",
        lossless_gate_histories(4000, 3000, 2),
    );
}

/// JxlEncoderAddChunkedFrame with input callbacks at the lossless profile:
/// buffer mode (the frame is copied in AddChunkedFrame) and, with an output
/// processor, encoding straight from the callbacks per DC group of the
/// streaming encode. Every callback of the source and of the processor is
/// compared in order. Gray and RGB, interleaved alpha, 8 and 16 bits, row
/// padding, every buffering value.
#[test]
fn public_encoder_lossless_chunked_frame_matches_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let cases: [(usize, usize, i32, bool); 8] = [
        (17, 19, -1, true),
        (257, 259, 2, false),
        (512, 1024, 3, true),
        (768, 768, -1, false),
        (768, 768, -1, true),
        (1031, 517, 3, false),
        (2049, 300, 1, true),
        (1024, 1024, 2, false),
    ];
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize, buffering, alpha) in &cases {
        for gray in [false, true] {
            for processor in [false, true] {
                let depth = [8, 16][variant % 2];
                let data_type = if depth == 16 { 3 } else { 2 };
                let channels = if gray { 1 } else { 3 } + usize::from(alpha);
                let mut info = format!("Bw:{xsize},h:{ysize},orig:1");
                if depth == 16 {
                    info.push_str(",bits:16");
                }
                if alpha {
                    info.push_str(&format!(",ec:1,ab:{depth}"));
                }
                if gray {
                    info.push_str(",c:1");
                }
                let endianness = variant % 3;
                let align = [0, 1, 4, 16, 128][variant % 5];
                let pad = [0, 3, 0, 8, 1][variant % 5];
                let content = 1 + variant % 4 + if alpha { 16 * (2 + variant % 2) } else { 0 };
                let mut script = format!(
                    "{info} G{} s- o0,0,2 y0,1{}",
                    u8::from(gray),
                    buffering_settings(buffering, [-1, 1, 0][variant % 3])
                );
                if processor {
                    script.push_str(&format!(" w{},0,0,0", [0, 4][variant % 2]));
                }
                script.push_str(&format!(
                    " c0,1,{channels},{data_type},{endianness},{align},{content},{xsize},{ysize},{pad},0,{data_type},{}",
                    usize::from(alpha)
                ));
                let bound = xsize * ysize * channels * 2 * 2 + 4096;
                requests.push(run(
                    0,
                    0,
                    &if processor {
                        format!("{script} Z r S")
                    } else {
                        format!("{script} Z {} S", output_schedule(bound))
                    },
                ));
                variant += 1;
            }
        }
    }
    for (request, cpp) in requests
        .iter()
        .zip(super::super::original_observations(&requests))
    {
        assert!(
            cpp.contains(" c:0;"),
            "the original accepts the chunked frame: {request}"
        );
    }
    let count = requests.len();
    let exact = super::compare_counted("lossless-chunked", requests);
    assert_eq!(exact, count, "lossless-chunked: every history is exact");
}

/// The lossless profile with the container (encoder step 3): an explicit
/// container (JxlEncoderUseContainer), level 10 (jxll box), and output mode
/// 2, whose streaming frames write out-of-order jxlp boxes, one-shot and
/// streaming, buffer mode and output processors.
#[test]
fn public_encoder_lossless_container_matches_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let prefixes = ["U1 ", "L10 ", "", "U1 "];
    let cases = [
        (9, 17, -1, -1),
        (257, 255, 0, 2),
        (512, 1024, -1, 2),
        (768, 768, -1, -1),
        (768, 768, 2, 2),
        (1031, 517, 3, 2),
        (2049, 300, 1, 2),
        (1024, 1024, -1, 1),
    ];
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize, buffering, output_mode) in &cases {
        for gray in [false, true] {
            for processor in [false, true] {
                let callbacks = format!("w{},0,0,0", [0, 4][variant % 2]);
                requests.push(lossless_history(
                    prefixes[variant % prefixes.len()],
                    xsize,
                    ysize,
                    gray,
                    [8, 16][(variant / 2) % 2],
                    variant % 3 == 1,
                    1 + variant % 4,
                    variant,
                    &buffering_settings(buffering, output_mode),
                    processor.then_some(callbacks.as_str()),
                ));
                variant += 1;
            }
        }
    }
    compare_lossless("lossless-container", requests);
}

/// Decode-back: the Rust encoder's lossless codestreams, which equal the
/// original's byte for byte (see the families above), decoded by the Rust
/// public decoder and by the original decoder give identical events,
/// metadata and pixels, one-shot and streaming.
#[test]
fn public_encoder_lossless_decode_back_matches_original_decoder() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let cases = [
        (1, 1, -1),
        (9, 17, -1),
        (257, 255, -1),
        (768, 768, -1),
        (1031, 517, 3),
    ];
    let mut requests = Vec::new();
    for (index, &(xsize, ysize, buffering)) in cases.iter().enumerate() {
        for gray in [false, true] {
            let variant = index * 2 + usize::from(gray);
            let history = lossless_history(
                "",
                xsize,
                ysize,
                gray,
                8,
                variant % 2 == 1,
                1 + variant % 4,
                variant,
                &buffering_settings(buffering, 0),
                None,
            );
            let file = driver::hex(&super::encoded_bytes(&history));
            requests.push(format!(
                "dpub-run 0 0 {file} s5184 F3.0.0.0.0 L1,1000000,0,1000,1"
            ));
            requests.push(format!(
                "dpub-run 0 0 {file} s5120 F4.2.0.0.0 L2,64,0,1000,1"
            ));
        }
    }
    super::super::decoder_public::compare("lossless-decode-back", requests);
}

/// Branch evidence for the streaming families above: for the same image the
/// original writes other bytes with its default buffering (and with
/// buffering 1 above 2048 pixels per side, 2 and 3) than with buffering 0,
/// its one-shot path, above 8 groups; at most 8 groups the default buffering
/// writes the one-shot bytes. Both sides of every pair are compared with the
/// port as well.
#[test]
fn public_encoder_lossless_streaming_takes_the_original_streaming_path() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let cases = [
        (768, 768, -1, true),
        (2049, 300, 1, true),
        (1031, 517, 2, true),
        (1024, 1024, 3, true),
        (2048, 300, 1, false),
        (768, 512, -1, false),
    ];
    let mut requests = Vec::new();
    for (index, &(xsize, ysize, buffering, _)) in cases.iter().enumerate() {
        for setting in [0, buffering] {
            requests.push(lossless_history(
                "",
                xsize,
                ysize,
                index % 2 == 1,
                8,
                false,
                4,
                0,
                &buffering_settings(setting, 0),
                None,
            ));
        }
    }
    let originals = super::super::original_observations(&requests);
    for (index, &(xsize, ysize, buffering, streams)) in cases.iter().enumerate() {
        let one_shot = super::output_bytes(&originals[2 * index]);
        let other = super::output_bytes(&originals[2 * index + 1]);
        assert!(!one_shot.is_empty() && !other.is_empty());
        assert_eq!(
            one_shot != other,
            streams,
            "{xsize}x{ysize} buffering {buffering}: the original's bytes differ from its \
             one-shot bytes exactly when it streams"
        );
    }
    compare_lossless("lossless-streaming-evidence", requests);
}
