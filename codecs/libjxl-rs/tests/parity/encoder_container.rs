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
