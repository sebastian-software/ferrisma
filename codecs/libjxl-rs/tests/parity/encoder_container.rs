// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Encoder step 3 (encoder container): containers and boxes of the public
// encoder against the original public C API. A child module of
// `encoder_public`, whose epub-run drivers, history builders and comparisons
// it reuses: every history compares every status, error, output chunk and
// output-processor call with the original's. The `ectr-*` requests observe
// the original's box header writers directly (tools/oracle/encoder_container.cc).

use super::driver;
use super::{bits, compare_admitted, lossy_history, output_schedule, run};
use jxl::jxl::encode_internal::{append_box_header, make_container_header, write_box_header};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("ectr-")
}

fn box_type(hex: &str) -> [u8; 4] {
    driver::unhex(hex).try_into().unwrap()
}

pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut next = || args.next().unwrap();
    match kind {
        "ectr-box" => {
            let box_type = box_type(next());
            let size: u64 = next().parse().unwrap();
            let unbounded = next() != "0";
            let force = next() != "0";
            let mut output = [0u8; 16];
            let n = write_box_header(&box_type, size as usize, unbounded, force, &mut output);
            format!("{n} {}", driver::hex(&output[..n]))
        }
        "ectr-append" => {
            let prefix = next();
            let mut out = if prefix == "-" {
                Vec::new()
            } else {
                driver::unhex(prefix)
            };
            let box_type = box_type(next());
            let size: u64 = next().parse().unwrap();
            let unbounded = next() != "0";
            append_box_header(&box_type, size as usize, unbounded, &mut out);
            driver::hex(&out)
        }
        "ectr-ftyp" => {
            let version: i32 = next().parse().unwrap();
            driver::hex(&make_container_header(version))
        }
        _ => panic!("unknown encoder container request {kind}"),
    }
}

/// The box header writers at every size class: small, at and around the
/// 64-bit threshold (kLargeBoxContentSizeThreshold), sizes whose header
/// arithmetic wraps, unbounded and forced large headers. The public
/// histories cannot reach the 64-bit size field (a box or codestream of
/// 4 GiB), so these requests cover it.
#[test]
fn public_encoder_box_header_writers_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let threshold: u64 = (1 << 32) - 8;
    let sizes = [
        0,
        1,
        7,
        8,
        255,
        256,
        65535,
        1 << 24,
        threshold - 1,
        threshold,
        threshold + 1,
        threshold + 77,
        (1 << 32) - 1,
        1 << 32,
        (1 << 32) + 5,
        1 << 40,
        1 << 63,
        u64::MAX - 16,
        u64::MAX - 15,
        u64::MAX - 8,
        u64::MAX - 7,
        u64::MAX,
    ];
    let types = ["6a786c63", "6a786c70", "45786966", "00000000", "ffffffff"];
    let mut requests = Vec::new();
    for box_type in types {
        for size in sizes {
            for unbounded in 0..2 {
                for force in 0..2 {
                    requests.push(format!("ectr-box {box_type} {size} {unbounded} {force}"));
                }
                for prefix in ["-", "00", "0102030405"] {
                    requests.push(format!(
                        "ectr-append {prefix} {box_type} {size} {unbounded}"
                    ));
                }
            }
        }
    }
    for version in [-1, 0, 1, 2, 127, 255, 256, i32::MAX, i32::MIN] {
        requests.push(format!("ectr-ftyp {version}"));
    }
    let count = requests.len();
    super::super::compare(requests);
    println!("container-box-headers: {count} requests");
}

/// Encoder container, unit 1: one frame in a container in buffer mode
/// (JxlEncoderProcessOutput). The container comes from
/// JxlEncoderUseContainer, from JxlEncoderSetCodestreamLevel(10) (a jxll box
/// after the ftyp box), from JxlEncoderStoreJPEGMetadata (the codestream
/// header in an early jxlp box) or from a required level 10; the frame is a
/// jxlc box. Sizes from 1x1 to 512x1024 (one-shot) and above 8 groups
/// (streaming output modes 0 and 1), gray and RGB, every sample type, and
/// ProcessOutput schedules that split every box header. Every history must be
/// exact, and the original must encode it.
#[test]
fn public_encoder_container_single_frame_matches_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let prefixes = [
        "U1 ", "L10 ", "U1 L10 ", "U1 L5 ", "U1 L-1 ", "J1 ", "J1 L10 ", "U0 L10 ",
    ];
    let sizes = [
        (1, 1),
        (7, 7),
        (8, 8),
        (9, 17),
        (43, 15),
        (71, 23),
        (63, 129),
        (256, 256),
        (257, 255),
        (512, 1024),
    ];
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize) in &sizes {
        for gray in [false, true] {
            for _ in 0..2 {
                let prefix = prefixes[variant % prefixes.len()];
                let distance = [1.0, 0.5, 2.9, 1.5, 0.69][variant % 5];
                let history = lossy_history(
                    xsize,
                    ysize,
                    gray,
                    variant % 4,
                    1 + variant % 4,
                    distance,
                    variant,
                    "",
                    "",
                );
                requests.push(run(0, 0, &format!("{prefix}{history}")));
                variant += 1;
            }
        }
    }
    // Above 8 groups: EncodeFrameStreaming inside a jxlc box, output modes 0
    // (buffering -1 to 2) and 1 (buffering 3, the TOC written through seeks
    // into the reserved box).
    for (index, &(xsize, ysize)) in [(768, 768), (1024, 1024), (2049, 300)].iter().enumerate() {
        for (buffering, mode) in [(-1, -1), (2, 0), (3, -1), (1, 1)] {
            let prefix = prefixes[(index + 2 * buffering.max(0) as usize) % 3];
            let settings = format!(" o0,34,{buffering} o0,40,{mode}");
            let history = lossy_history(
                xsize,
                ysize,
                index == 1,
                index % 4,
                4,
                1.0,
                index,
                "",
                &settings,
            );
            requests.push(run(0, 0, &format!("{prefix}{history}")));
        }
    }
    // A required level 10 (an image dimension above level 5's limit is not
    // encodable here; five extra channels exceed level 5's four) with the
    // automatic level: the original writes the jxll box.
    for (xsize, ysize) in [(17, 19), (64, 64)] {
        let info = ",ec:5";
        let mut ecs = String::new();
        for i in 0..5 {
            ecs.push_str(&format!(" K{i},6,8,0,0,0,0,0,0,0,0"));
        }
        let row = xsize * 3;
        let size = row * ysize;
        let mut buffers = String::new();
        for ec in 0..5 {
            buffers.push_str(&format!(
                " E0,1,2,0,0,{},{ec},3,{xsize},{ysize}",
                xsize * ysize
            ));
        }
        requests.push(run(
            0,
            0,
            &format!(
                "Bw:{xsize},h:{ysize}{info} G0{ecs} s- d0,{} a0,3,2,0,0,{size},3,{xsize},{ysize}{buffers} Z {}",
                bits(1.0),
                output_schedule(xsize * ysize * 32 + 4096)
            ),
        ));
    }
    compare_admitted("container-single-frame", requests);
}

/// Decode-back of the container histories: the Rust encoder's containers,
/// which equal the original's byte for byte, decoded by the Rust public
/// decoder and by the original decoder (boxes subscribed) give identical
/// events, boxes and pixels.
#[test]
fn public_encoder_container_decode_back_matches_original_decoder() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let mut requests = Vec::new();
    for (index, prefix) in ["U1 ", "L10 ", "J1 "].iter().enumerate() {
        for &(xsize, ysize) in &[(8, 8), (43, 15), (257, 255)] {
            let history = run(
                0,
                0,
                &format!(
                    "{prefix}{}",
                    lossy_history(xsize, ysize, index == 1, index, 2, 1.0, index, "", "")
                ),
            );
            let file = driver::hex(&super::encoded_bytes(&history));
            requests.push(format!(
                "dpub-run 0 0 {file} s5184 F3.0.0.0.0 L1,1000000,0,1000,1"
            ));
            requests.push(format!(
                "dpub-run 0 0 {file} s5120 F4.2.0.0.0 L2,64,0,1000,1"
            ));
            // Basic info, frame and box events with a box buffer.
            requests.push(format!("dpub-run 0 0 {file} s17472 L3,256,64,1000,1"));
        }
    }
    super::super::decoder_public::compare("container-decode-back", requests);
}

/// Encoder container, unit 2: containers through an output processor
/// (JxlEncoderSetOutputProcessor and JxlEncoderFlushInput). The jxlc and
/// jxlp box headers are reserved by seeks and written after the frame, with
/// and without the processor's seek callback (the wrapper then keeps the
/// reserved bytes), and every buffer schedule; one-shot and streaming frames
/// in output modes 0 and 1. The processor call sequence (get buffer,
/// release, seek, finalized position) and the file are compared exactly.
#[test]
fn public_encoder_container_output_processor_matches_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let prefixes = ["U1 ", "L10 ", "U1 L10 ", "J1 ", "U1 L5 "];
    let cases = [
        (1, 1, -1, -1),
        (43, 15, 0, -1),
        (63, 129, -1, -1),
        (512, 1024, 3, -1),
        (768, 768, -1, -1),
        (768, 768, -1, 1),
        (1031, 517, 3, -1),
        (1031, 517, 2, 0),
        (2049, 300, 1, 1),
        (1024, 1024, -1, 0),
    ];
    let mut requests = Vec::new();
    for (index, &(xsize, ysize, buffering, output_mode)) in cases.iter().enumerate() {
        for gray in [false, true] {
            let variant = index * 2 + usize::from(gray);
            let mut settings = String::new();
            if buffering != -1 {
                settings.push_str(&format!(" o0,34,{buffering}"));
            }
            if output_mode != -1 {
                settings.push_str(&format!(" o0,40,{output_mode}"));
            }
            let mask = if variant % 3 == 1 { 4 } else { 0 };
            let schedule = variant % 4;
            let schedule = if schedule == 1 && xsize * ysize > 600_000 {
                3
            } else {
                schedule
            };
            let history = super::processor_history(
                xsize,
                ysize,
                gray,
                variant % 4,
                1 + variant % 4,
                [1.0, 2.0, 0.5, 1.5][variant % 4],
                variant,
                &settings,
                &format!("w{mask},{schedule},0,0"),
                variant % 5 == 2,
            );
            requests.push(run(
                0,
                0,
                &format!("{}{history}", prefixes[variant % prefixes.len()]),
            ));
        }
    }
    super::compare_admitted_processor("container-output-processor", requests);
}

/// One output-mode-2 history: `o0,40,2` with the given buffering, in buffer
/// mode (a ProcessOutput schedule) or through an output processor.
#[expect(clippy::too_many_arguments, reason = "One history's fields")]
fn mode2_history(
    prefix: &str,
    xsize: usize,
    ysize: usize,
    gray: bool,
    buffering: i32,
    distance: f32,
    variant: usize,
    processor: Option<&str>,
) -> String {
    let mut settings = String::new();
    if buffering != -1 {
        settings.push_str(&format!(" o0,34,{buffering}"));
    }
    settings.push_str(" o0,40,2");
    let history = match processor {
        Some(processor) => super::processor_history(
            xsize,
            ysize,
            gray,
            variant % 4,
            1 + variant % 4,
            distance,
            variant,
            &settings,
            processor,
            variant % 5 == 2,
        ),
        None => lossy_history(
            xsize,
            ysize,
            gray,
            variant % 4,
            1 + variant % 4,
            distance,
            variant,
            "",
            &settings,
        ),
    };
    run(0, 0, &format!("{prefix}{history}"))
}

/// Encoder container, unit 2: JXL_ENC_FRAME_SETTING_OUTPUT_MODE 2. Mode 2
/// forces a container with ftyp version 1 and writes the codestream header
/// as jxlp box 0. A frame that takes the one-shot path leaves the 12-byte gap
/// that ProcessOneEnqueuedInput fills with its jxlp header; a streaming frame
/// (more than 8 groups) writes out-of-order jxlp boxes: the frame header,
/// the DC global section and every group section with its codestream
/// position as counter, and the TOC as counter base + 1. Buffer mode and
/// output processors with and without seek, every buffering value.
#[test]
fn public_encoder_output_mode_2_matches_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let prefixes = ["", "U1 ", "L10 ", "J1 "];
    let cases = [
        (1, 1, -1),
        (8, 8, 0),
        (43, 15, 2),
        (63, 129, -1),
        (257, 255, 3),
        (512, 1024, -1),
        (768, 768, -1),
        (768, 768, 0),
        (768, 769, 2),
        (1031, 517, 3),
        (1024, 1024, 1),
        (2049, 300, 1),
        (2049, 300, -1),
    ];
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize, buffering) in &cases {
        for gray in [false, true] {
            for processor in [false, true] {
                let mask = if variant % 3 == 1 { 4 } else { 0 };
                let schedule = if xsize * ysize > 600_000 {
                    [0, 2, 3][variant % 3]
                } else {
                    variant % 4
                };
                let callbacks = format!("w{mask},{schedule},0,0");
                requests.push(mode2_history(
                    prefixes[variant % prefixes.len()],
                    xsize,
                    ysize,
                    gray,
                    buffering,
                    [1.0, 2.0, 0.5, 1.5][variant % 4],
                    variant,
                    processor.then_some(callbacks.as_str()),
                ));
                variant += 1;
            }
        }
    }
    let (buffer, processor): (Vec<String>, Vec<String>) = requests
        .into_iter()
        .partition(|request| !request.contains(" i r"));
    compare_admitted("output-mode-2", buffer);
    super::compare_admitted_processor("output-mode-2-processor", processor);
}

/// Encoder step 2's gate sizes in output mode 2 (#117): 768x768, 1024x1024,
/// 2048x1536, 2049x2049 and 4000x3000 at d1.0 and d2.0, gray and RGB, buffer
/// mode and an output processor with and without seek, rotating the
/// buffering values -1 to 3. The histories of one size; the counter `k` runs
/// over all sizes, so each size's histories are those of one list over all
/// sizes. One test per size lets the check's partitions spread them.
fn output_mode_2_gate_histories(size: (usize, usize)) -> Vec<String> {
    let mut requests = Vec::new();
    let mut k = 0;
    for (xsize, ysize) in [
        (768, 768),
        (1024, 1024),
        (2048, 1536),
        (2049, 2049),
        (4000, 3000),
    ] {
        for distance in [1.0f32, 2.0] {
            for gray in [false, true] {
                for processor in [false, true] {
                    // 4000x3000: one history per distance and processor kind.
                    if xsize == 4000 && gray == processor {
                        continue;
                    }
                    if (xsize, ysize) == size {
                        let buffering = [-1, 0, 1, 2, 3][k % 5];
                        let mut settings = if buffering == -1 {
                            String::new()
                        } else {
                            format!(" o0,34,{buffering}")
                        };
                        settings.push_str(" o0,40,2");
                        let history =
                            lossy_history(xsize, ysize, gray, k % 4, 4, distance, k, "", &settings);
                        let (input, _) = history.split_once(" Z ").unwrap();
                        let chunk = 4 << 20;
                        let count = (xsize * ysize * if gray { 1 } else { 3 }).div_ceil(chunk) + 1;
                        let output = if processor {
                            format!("w{},0,0,0 i r", if k.is_multiple_of(2) { 0 } else { 4 })
                        } else {
                            vec![format!("p{chunk}"); count].join(" ")
                        };
                        requests.push(run(0, 0, &format!("{input} Z {output}")));
                    }
                    k += 1;
                }
            }
        }
    }
    requests
}

/// One part of the output-mode-2 gate of one size: the histories `part` of
/// [`output_mode_2_gate_histories`] (eight per size, four at 4000x3000).
/// One test per size at 768 and 1024, per pair at 2048x1536 and 2049x2049,
/// and per history at 4000x3000: together the same histories as one test
/// over all sizes, in parts that the check's partitions spread over shards.
fn output_mode_2_gate_part(family: &str, size: (usize, usize), part: std::ops::Range<usize>) {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let histories = output_mode_2_gate_histories(size);
    assert_eq!(
        histories.len(),
        if size.0 == 4000 { 4 } else { 8 },
        "the gate's histories per size"
    );
    super::compare_gate(family, histories[part].to_vec());
}

macro_rules! output_mode_2_gate_parts {
    ($($name:ident: $family:literal, $size:expr, $part:expr;)*) => {
        $(
            #[test]
            fn $name() {
                output_mode_2_gate_part($family, $size, $part);
            }
        )*
    };
}

output_mode_2_gate_parts! {
    public_encoder_output_mode_2_gate_768_matches_original: "output-mode-2-gate-768x768", (768, 768), 0..8;
    public_encoder_output_mode_2_gate_1024_matches_original: "output-mode-2-gate-1024x1024", (1024, 1024), 0..8;
    public_encoder_output_mode_2_gate_2048x1536_h0_h1_matches_original: "output-mode-2-gate-2048x1536-h0-h1", (2048, 1536), 0..2;
    public_encoder_output_mode_2_gate_2048x1536_h2_h3_matches_original: "output-mode-2-gate-2048x1536-h2-h3", (2048, 1536), 2..4;
    public_encoder_output_mode_2_gate_2048x1536_h4_h5_matches_original: "output-mode-2-gate-2048x1536-h4-h5", (2048, 1536), 4..6;
    public_encoder_output_mode_2_gate_2048x1536_h6_h7_matches_original: "output-mode-2-gate-2048x1536-h6-h7", (2048, 1536), 6..8;
    public_encoder_output_mode_2_gate_2049_h0_h1_matches_original: "output-mode-2-gate-2049x2049-h0-h1", (2049, 2049), 0..2;
    public_encoder_output_mode_2_gate_2049_h2_h3_matches_original: "output-mode-2-gate-2049x2049-h2-h3", (2049, 2049), 2..4;
    public_encoder_output_mode_2_gate_2049_h4_h5_matches_original: "output-mode-2-gate-2049x2049-h4-h5", (2049, 2049), 4..6;
    public_encoder_output_mode_2_gate_2049_h6_h7_matches_original: "output-mode-2-gate-2049x2049-h6-h7", (2049, 2049), 6..8;
    public_encoder_output_mode_2_gate_4000x3000_h0_matches_original: "output-mode-2-gate-4000x3000-h0", (4000, 3000), 0..1;
    public_encoder_output_mode_2_gate_4000x3000_h1_matches_original: "output-mode-2-gate-4000x3000-h1", (4000, 3000), 1..2;
    public_encoder_output_mode_2_gate_4000x3000_h2_matches_original: "output-mode-2-gate-4000x3000-h2", (4000, 3000), 2..3;
    public_encoder_output_mode_2_gate_4000x3000_h3_matches_original: "output-mode-2-gate-4000x3000-h3", (4000, 3000), 3..4;
}

/// The EncoderStreamingTest instances that need the container: every
/// instance with use_container (JxlEncoderSetCodestreamLevel(10)) in output
/// modes 0, 1 and 2, and the instances without it in output mode 2. Multiple
/// frames and fast lossless stay outside the admitted profile.
#[test]
fn public_encoder_streaming_suite_container_instances_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let mut requests = super::streaming_suite_requests(&[0, 1, 2], true);
    requests.extend(super::streaming_suite_requests(&[2], false));
    let count = requests.len();
    let exact = super::compare_counted("streaming-suite-container", requests);
    assert_eq!(exact, count, "every admitted instance is exact");
}

/// A history's setup (basic info, color, settings) and its frame operation,
/// from [`lossy_history`].
fn setup_and_frame(
    xsize: usize,
    ysize: usize,
    gray: bool,
    variant: usize,
    extra_settings: &str,
) -> (String, String) {
    let history = lossy_history(
        xsize,
        ysize,
        gray,
        variant % 4,
        1 + variant % 4,
        [1.0, 2.0, 0.5, 1.5][variant % 4],
        variant,
        "",
        extra_settings,
    );
    let (input, _) = history.split_once(" Z ").unwrap();
    let (setup, frame) = input.rsplit_once(" a0,").unwrap();
    (setup.to_string(), format!("a0,{frame}"))
}

/// Encoder container, unit 3: uncompressed metadata boxes through
/// JxlEncoderUseBoxes, JxlEncoderAddBox and JxlEncoderCloseBoxes. Box types
/// Exif, `xml `, jumb, an unregistered type, a zero type and the types the
/// original only rejects for compressed boxes (jxlc, jxli, brob, jbrd), at
/// sizes 0 to 70001 bytes; boxes before the frame (the codestream header
/// then goes into jxlp box 0 and the frame into the last jxlp box), after the
/// frame, before and after, several boxes in a row, boxes without a frame,
/// UseBoxes without a box; ProcessOutput between the inputs, schedules that
/// split every box header, the ProcessEncoder loop of encode_test (`P`), and
/// an output processor with and without seek (FlushInput).
#[test]
fn public_encoder_container_metadata_boxes_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let types = [
        "45786966", "786d6c20", "6a756d62", "61626364", "00000000", "6a786c63", "6a786c69",
        "62726f62", "6a627264",
    ];
    let sizes = [0, 1, 7, 8, 255, 4096, 70001];
    let mut requests = Vec::new();
    let mut k = 0;
    let mut next_box = || {
        let b = format!(
            "O{},{},0",
            types[k % types.len()],
            sizes[(k / 2) % sizes.len()]
        );
        k += 1;
        b
    };
    for (index, &(xsize, ysize)) in [(8, 8), (43, 15), (63, 129), (257, 255)].iter().enumerate() {
        for gray in [false, true] {
            let variant = 2 * index + usize::from(gray);
            let prefix = ["X ", "X U1 ", "X L10 ", "X J1 "][variant % 4];
            let (setup, frame) = setup_and_frame(xsize, ysize, gray, variant, "");
            let bound = output_schedule(xsize * ysize * 12 + 80_000);
            let (b1, b2, b3) = (next_box(), next_box(), next_box());
            let scripts = [
                // Before the frame.
                format!("{prefix}{setup} {b1} {b2} {frame} Z {bound}"),
                // After the frame.
                format!("{prefix}{setup} {frame} F p0 p1 {b1} {b3} Y {bound}"),
                // EncodeBoxTest's order with the ProcessEncoder loop.
                format!("{prefix}{setup} {b2} P64 {frame} F P64 {b3} Y P64 S"),
                // Before and after, output after each input.
                format!("{prefix}{setup} {b3} p9 p40 {frame} F {bound} {b1} {b2} Y {bound}"),
                // Boxes and no frame.
                format!("{prefix}{setup} {b1} {b2} Y {bound} S"),
                // UseBoxes without a box.
                format!("{prefix}{setup} {frame} Z {bound}"),
                // Boxes only, before basic info.
                format!("X {b1} {b3} Z {bound}"),
            ];
            for script in scripts {
                requests.push(run(0, 0, &script));
            }
            // Through an output processor (FlushInput after each input).
            let mask = if variant % 2 == 0 { 0 } else { 4 };
            let processor = format!("w{mask},{},0,0", variant % 4);
            requests.push(run(
                0,
                0,
                &format!("{prefix}{setup} {processor} {b1} i {frame} F i {b2} {b3} Y i r S"),
            ));
            requests.push(run(
                0,
                0,
                &format!("{prefix}{setup} {b2} {frame} Z {processor} i r S"),
            ));
        }
    }
    // Boxes around a streaming frame (more than 8 groups), output modes 0, 1
    // and 2, buffer and processor mode.
    for (index, mode) in [0, 1, 2].into_iter().enumerate() {
        let settings = format!(" o0,34,{} o0,40,{mode}", [2, 3, -1][index]);
        let (setup, frame) = setup_and_frame(768, 768, index == 1, index, &settings);
        let (b1, b2) = (next_box(), next_box());
        requests.push(run(
            0,
            0,
            &format!(
                "X {setup} {b1} {frame} F {b2} Y {}",
                output_schedule(768 * 768 * 12)
            ),
        ));
        requests.push(run(
            0,
            0,
            &format!(
                "X {setup} {b1} {frame} F {b2} Y w{},0,0,0 i r",
                4 * (index % 2)
            ),
        ));
    }
    let count = requests.len();
    let exact = super::compare_counted("container-boxes", requests);
    assert_eq!(exact, count, "container-boxes: every history is exact");
}

/// Encoder container, unit 3: the frame index (JXL_ENC_FRAME_INDEX_BOX). For
/// one frame, AddFrame records it as indexed and the original appends a jxli
/// box after the frame, also after a raw codestream (the setter stores the
/// request for the values 0 and 1 alike; other values are rejected); with
/// boxes, a container, output mode 2 and an output processor.
#[test]
fn public_encoder_container_frame_index_matches_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let mut requests = Vec::new();
    for (index, &(xsize, ysize)) in [(1, 1), (8, 8), (43, 15), (257, 255), (768, 768)]
        .iter()
        .enumerate()
    {
        for value in [1, 0, 2] {
            let variant = 3 * index + value as usize;
            let mode = if variant % 4 == 3 { " o0,40,2" } else { "" };
            let (setup, frame) = setup_and_frame(
                xsize,
                ysize,
                variant % 2 == 1,
                variant,
                &format!(" o0,31,{value}{mode}"),
            );
            let prefix = ["", "U1 ", "X ", "L10 "][variant % 4];
            let bound = output_schedule(xsize * ysize * 12 + 4096);
            requests.push(run(0, 0, &format!("{prefix}{setup} {frame} Z {bound} S")));
            if variant.is_multiple_of(3) {
                requests.push(run(
                    0,
                    0,
                    &format!("{prefix}{setup} {frame} Z w0,0,0,0 i r S"),
                ));
            }
        }
    }
    let count = requests.len();
    let exact = super::compare_counted("container-frame-index", requests);
    assert_eq!(
        exact, count,
        "container-frame-index: every history is exact"
    );
}

/// Encoder container, unit 4: rejections. Boxes after JxlEncoderCloseBoxes
/// and CloseInput and without UseBoxes; compressed boxes of the reserved
/// types (jxl*, jbrd, brob); the container, box, level and JPEG-metadata
/// settings after output started; invalid levels; level 5 violations (five
/// extra channels, a 16-bit alpha channel, dimensions above level 5, which
/// SetBasicInfo or ProcessOutput rejects) and a level 10 violation that only
/// ProcessOutput detects; output mode 2 after a container with ftyp version
/// 0 was written. The `brob` compression of a box (step 11) is an admission
/// result after an exact prefix with the original's branch evidence.
#[test]
fn public_encoder_container_rejections_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let (setup, frame) = setup_and_frame(8, 8, false, 1, "");
    let (setup2, frame2) = setup_and_frame(43, 15, false, 2, " o0,40,2");
    let bound = output_schedule(8 * 8 * 12 + 4096);
    let mut scripts = vec![
        format!("X {setup} Y O45786966,4,0 {frame} Z {bound} S"),
        format!("X {setup} Z O45786966,4,0 {bound} S"),
        format!("X {setup} O45786966,4,0 {frame} Z {bound} O786d6c20,3,0 S"),
        format!("{setup} O45786966,4,0 {frame} Z {bound} S"),
        format!("{setup} {frame} Z p64 X U1 U0 L10 L5 L-1 J1 J0 {bound} S"),
        format!("X {setup} O45786966,4,0 p64 X U1 L10 Y {bound} S"),
        format!("L0 L4 L6 L11 L-2 L2147483647 {setup} {frame} Z {bound} S"),
        // Level 5 violations.
        "L5 Bw:1073741824,h:8 S".to_string(),
        "L5 Bw:8,h:262145 S".to_string(),
        "L5 Bw:8,h:8,ec:5 S".to_string(),
        "L-1 Bw:1073741825,h:8 Bw:2147483648,h:8 S".to_string(),
        "X Bw:1073741824,h:2048 l O45786966,4,0 Z p64 S".to_string(),
        "X L10 Bw:1073741824,h:2048 l O45786966,4,0 Z p64 S".to_string(),
        // Mode 2 after a container with ftyp version 0.
        format!("X {setup2} O45786966,4,0 p100 {frame2} Z {bound} S"),
        format!("X {setup2} {frame2} O45786966,4,0 Z {bound} S"),
    ];
    // Five extra channels or a 16-bit alpha channel at level 5.
    let mut ecs = String::new();
    let mut buffers = String::new();
    for i in 0..5 {
        ecs.push_str(&format!(" K{i},6,8,0,0,0,0,0,0,0,0"));
        buffers.push_str(&format!(" E0,1,2,0,0,64,{i},3,8,8"));
    }
    scripts.push(format!(
        "L5 Bw:8,h:8,ec:5 G0{ecs} s- a0,3,2,0,0,192,3,8,8{buffers} Z {bound} S"
    ));
    scripts.push(format!(
        "L5 Bw:8,h:8,bits:16,ec:1,ab:16 G0 s- a0,4,3,0,0,512,19,8,8 Z {bound} S"
    ));
    for compress in ["6a786c63", "6a786c70", "6a786c6c", "6a627264", "62726f62"] {
        scripts.push(format!("X {setup} O{compress},4,1 {frame} Z {bound} S"));
    }
    // brob compression (admission result at ProcessOutput).
    scripts.push(format!("X {setup} O45786966,40,1 {frame} Z {bound} S"));
    scripts.push(format!(
        "X {setup} {frame} F {bound} O786d6c20,40,1 Y {bound} S"
    ));
    let requests = scripts.iter().map(|script| run(0, 0, script)).collect();
    super::compare("container-rejections", requests);
}

/// The pixels of an upstream encode_test case: GetSomeTestImage from the
/// original's test helpers, as an `a` operation's explicit content.
fn some_test_image(xsize: usize, ysize: usize, channels: usize, seed: usize) -> String {
    let request = format!("ectr-pixels {xsize} {ysize} {channels} {seed}");
    let pixels = super::super::original_observations(&[request]).remove(0);
    assert_eq!(pixels.len(), xsize * ysize * channels * 2 * 2);
    format!("x{pixels}")
}

/// The upstream encode_test cases of the gate in differential form, with the
/// tests' inputs (GetSomeTestImage, RGBA uint16 big endian), settings and
/// ProcessOutput loops (ProcessEncoder): SingleFrameBoundedJXLCTest,
/// CodestreamLevelTest, CodestreamLevelVerificationTest and the uncompressed
/// instances of EncodeBoxTest (both of its xml_box_size values, which the
/// test does not use). Every call's status and the complete output are
/// compared instead of the tests' container parse; the decoder half of the
/// box test decodes the output through both public decoders with boxes
/// subscribed. The compressed EncodeBoxTest instances are not admitted
/// (brob compression, step 11): their history is an admission result after
/// an exact prefix.
#[test]
fn upstream_encode_test_container_cases_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    // JxlBasicInfoSetFromPixelFormat for RGBA uint16 and uint8.
    let rgba16 = |x: usize, y: usize| format!("Bw:{x},h:{y},bits:16,ec:1,ab:16");
    let rgba8 = |x: usize, y: usize| format!("Bw:{x},h:{y},bits:8,ec:1,ab:8");
    let frame = |x: usize, y: usize, pixels: &str| format!("a0,4,3,2,0,{},{pixels}", x * y * 8);
    let mut requests = Vec::new();
    // SingleFrameBoundedJXLCTest.
    let pixels = some_test_image(71, 23, 4, 0);
    requests.push(run(
        0,
        0,
        &format!(
            "U1 s- L10 {} G0 {} Z P64 S",
            rgba16(71, 23),
            frame(71, 23, &pixels)
        ),
    ));
    // CodestreamLevelTest.
    let pixels = some_test_image(64, 64, 4, 0);
    requests.push(run(
        0,
        0,
        &format!(
            "L10 s- {} G0 {} Z P64 S",
            rgba16(64, 64),
            frame(64, 64, &pixels)
        ),
    ));
    // CodestreamLevelVerificationTest.
    requests.push(run(
        0,
        0,
        &format!(
            "{} l L5 {} L10 {} l {} S",
            rgba8(64, 64),
            rgba8(1 << 30, 64),
            rgba8(1 << 30, 64),
            rgba8(1 << 31, 64)
        ),
    ));
    // EncodeBoxTest, uncompressed (the two xml_box_size instances).
    let exif = format!("{}{}", "00000000", driver::hex(b"exif test data"));
    let xml = driver::hex(b"<some random xml data>");
    let pixels = some_test_image(50, 17, 4, 0);
    let box_test = |compress: u8| {
        format!(
            "X s- L10 {} G0 O45786966,{},{compress},{exif} P64 {} F P64 O584d4c20,{},{compress},{xml} Y P64 S",
            rgba16(50, 17),
            exif.len() / 2,
            frame(50, 17, &pixels),
            xml.len() / 2
        )
    };
    for _xml_box_size in [256usize, (1 << 32) - 8 + 77] {
        requests.push(run(0, 0, &box_test(0)));
    }
    let originals = super::original_runs_and_probes(&requests);
    let mut decodes = Vec::new();
    for (request, cpp) in requests.iter().zip(&originals.runs) {
        for field in cpp.split(' ').filter_map(|f| f.strip_prefix("P:")) {
            let (statuses, bytes) = field.split_once(';').unwrap();
            let bytes = bytes.split('/').next().unwrap();
            assert!(
                statuses.ends_with(&format!(".{}", bytes.len() / 2))
                    && statuses.rsplit(':').next().unwrap().starts_with("0."),
                "the original's ProcessEncoder loop succeeds: {request}"
            );
            if request.contains(" Y P64") && !field.contains(";/") {
                decodes.push(bytes.to_string());
            }
        }
    }
    let count = requests.len();
    let exact = super::compare_counted_with("upstream-encode-container", requests, Some(originals));
    assert_eq!(exact, count, "every admitted upstream case is exact");
    // The decoder half of EncodeBoxTest: the last ProcessEncoder output holds
    // the whole file; frame and box events (JXL_DEC_FRAME | JXL_DEC_BOX).
    let file = decodes.last().unwrap();
    super::super::decoder_public::compare(
        "upstream-encode-container-decode",
        vec![
            format!("dpub-run 0 0 {file} s17472 L1,1000000,64,1000,1"),
            format!("dpub-run 0 0 {file} s5184 F4.3.2.0.0 L2,64,0,1000,1"),
        ],
    );
    // Not admitted: the compressed EncodeBoxTest instances (brob, step 11).
    // The probe reports its branch evidence at ProcessOutput calls ('p'), so
    // these histories write through fixed ProcessOutput calls.
    let compressed = vec![run(0, 0, &box_test(1).replace("P64", "p100000"))];
    super::compare("upstream-encode-container-brob", compressed);
    println!(
        "upstream-encode-container: not admitted (brob compression, step 11): \
         EncodeBoxTest Compressed_BoxSize_256 and Compressed_BoxSize_4294967365"
    );
}

/// Encoder step 4's 16-bit and float alpha (#115), which needed level 10 and
/// so a container: alpha deeper than 12 bits clears
/// modular_16_bit_buffer_sufficient. uint16 input with 13- and 16-bit alpha,
/// float16 input with half-float alpha, float input with 32-bit float alpha;
/// interleaved and separate alpha planes, translucent and premultiplied
/// alpha, gray and RGB, one-shot and streaming sizes, at the automatic level
/// and with JxlEncoderSetCodestreamLevel(10). Every history must be exact.
#[test]
fn public_encoder_alpha_16_bit_and_float_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let depths = [
        (1, ",ab:16"),
        (2, ",ab:16,aexp:5"),
        (3, ",ab:32,aexp:8"),
        (1, ",ab:13"),
    ];
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize) in &[(1, 1), (9, 17), (45, 15), (63, 129), (256, 256), (768, 768)] {
        for &(type_index, depth) in &depths {
            let gray = variant % 2 == 1;
            let distance = [1.0, 0.5, 2.9, 1.5][variant % 4];
            let prefix = if variant % 3 == 2 { "L10 " } else { "" };
            let premultiplied = if variant % 5 == 4 { ",apre:1" } else { "" };
            let history = super::alpha_history(
                xsize,
                ysize,
                gray,
                type_index,
                1 + variant % 4,
                2 + variant % 2,
                distance,
                variant,
                variant % 4 == 1,
                &format!("{depth}{premultiplied}"),
                "",
            );
            requests.push(run(0, 0, &format!("{prefix}{history}")));
            variant += 1;
        }
    }
    compare_admitted("alpha-16-bit-and-float", requests);
}

/// roundtrip_test.cc GetTestImage<float> for four channels (the test's
/// input, as in tests/parity/encoder_step1.rs, whose es1-rt requests check
/// it against the original's helper), native-endian bytes.
fn roundtrip_float_rgba(xsize: usize, ysize: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    for y in 0..ysize {
        for x in 0..xsize {
            for chan in 0..4 {
                let val = match chan {
                    0 => y as f32 / ysize as f32,
                    1 => x as f32 / xsize as f32,
                    2 => (x + y) as f32 / (xsize + ysize) as f32,
                    _ => (x * y) as f32 / (xsize * ysize) as f32,
                };
                bytes.extend_from_slice(&val.to_ne_bytes());
            }
        }
    }
    bytes
}

/// roundtrip_test.cc ExtraBoxesTest in differential form: float RGBA (float
/// alpha, level 10) at 61x71 with JxlEncoderUseContainer, linear sRGB, the
/// test's EncodeWithEncoder loop (`P64`); then the test appends a 1023-byte
/// `crud` box with the original's AppendBoxHeader, and both public decoders
/// decode the result with the test's events and format.
#[test]
fn upstream_roundtrip_test_extra_boxes_matches_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    jxl::jxl::hwy::select_scalar_encoder_for_test();
    let (xsize, ysize) = (61, 71);
    let pixels = driver::hex(&roundtrip_float_rgba(xsize, ysize));
    let request = run(
        0,
        0,
        &format!(
            "U1 L10 Bw:{xsize},h:{ysize},bits:32,exp:8,ec:1,ab:32,aexp:8 g0 s- y0,0 a0,4,0,0,0,{},x{pixels} Z P64",
            xsize * ysize * 16
        ),
    );
    let originals = super::original_runs_and_probes(std::slice::from_ref(&request));
    let cpp = originals.runs[0].clone();
    let exact = super::compare_counted_with("upstream-extra-boxes", vec![request], Some(originals));
    assert_eq!(exact, 1, "ExtraBoxesTest is exact");
    let field = cpp
        .split(' ')
        .find_map(|f| f.strip_prefix("P:"))
        .expect("the ProcessEncoder loop");
    let (statuses, bytes) = field.split_once(';').unwrap();
    assert!(statuses.rsplit(':').next().unwrap().starts_with("0."));
    let bytes = bytes.split('/').next().unwrap();
    // The test's crud box: its header from the original, then zero bytes.
    let header =
        super::super::original_observations(&["ectr-append - 63727564 1023 0".to_string()])
            .remove(0);
    let file = format!("{bytes}{header}{}", "00".repeat(1023));
    super::super::decoder_public::compare(
        "upstream-extra-boxes-decode",
        vec![
            format!("dpub-run 0 0 {file} s4416 F4.0.0.0.0 L1,1000000,0,1000,1"),
            format!("dpub-run 0 0 {file} s4416 F4.0.0.0.0 L2,64,0,1000,1"),
        ],
    );
}
