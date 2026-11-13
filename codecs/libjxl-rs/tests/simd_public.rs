// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Whole decodes through the public decoder with the SIMD target forced
//! against the original's public decoder built for the same single Highway
//! target (port ADR 0004, "Decoder path" condition (b)), and the scalar
//! gate's forced scalar path (condition (c)).
//!
//! Both sides run the same call sequence (`dps-decode` of
//! tools/oracle/decoder_public_simd.cc, mirrored token for token by
//! [`port_decode`]): the engine's sequence of `crates/ferrisma-jxl-decoder`
//! (whole input, closed input, no runner, no CMS, default coalescing and
//! orientation) plus frame headers and every extra channel buffer, for float,
//! 8-bit and 16-bit output. Status, image info, color profiles and frame
//! headers must be equal; output samples are compared word for word, any NaN
//! equal to any NaN (decision D8). Inputs: the files of the `decode_vardct`
//! whole-decode gate of `simd_parity.rs`, the public-decoder test inputs of
//! the scalar suite (encoder specs, retained files, captures, decode_test
//! codestreams) and, when `JXL_CONFORMANCE_CORPUS` names a corpus fetched by
//! `tools/conformance/corpus.py fetch` (docs/CONFORMANCE.md), its 40 inputs.
//!
//! [`STILL_DIFFERS`] is the punch list in executable form: every other file
//! must match exactly, every listed file must still differ (so the list
//! cannot rot), and a listed file that starts to match fails the test until
//! it is removed from the list.
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

use jxl::jxl::codestream_header::{JxlBasicInfo, JxlExtraChannelInfo, JxlFrameHeader};
use jxl::jxl::color_encoding::JxlColorEncoding;
use jxl::jxl::decode::{self as api, JxlColorProfileTarget, JxlDecoderStatus as Status};
use jxl::jxl::hwy::{
    DECODER_SELECTS_SIMD, decoder_target, decoder_target_for, scalar_forced_by_environment,
    set_decoder_simd_for_test,
};
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use simd_common::{is_nan, target};
use std::collections::{BTreeMap, HashSet};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;

/// The decoder target override is process-wide; the tests of this binary
/// that change or read it run one at a time.
static OVERRIDE: Mutex<()> = Mutex::new(());

/// Files whose whole decode through the public decoder with the SIMD target
/// forced still differs from the original's single-target SIMD build, with
/// the still-scalar stage or kernel that explains it (found from the frame
/// headers and the pipeline the decoder builds, printed by the test for each
/// differing file). Remove an entry when the work on its reason lands and the
/// file matches; the test fails until then.
const STILL_DIFFERS: &[(&str, &str)] = &[
    ("dpub w=64 h=48 c=3 lossless=0 opt=2:2", UPSAMPLING),
    (
        "dpub w=300 h=280 c=3 a=1 bits=2 opt=26:0 opt=3:4",
        UPSAMPLING,
    ),
    (
        "dpub w=300 h=280 c=3 a=1 effort=3 opt=26:0 opt=3:4",
        UPSAMPLING,
    ),
    (
        "data/decoder-geometry/sequence-extra-alpha-xyb-0-1-up-2-shift-1.jxl",
        UPSAMPLING,
    ),
    (
        "data/decoder-geometry/sequence-extra-xyb-1-up-2-4.jxl",
        UPSAMPLING,
    ),
    (
        "data/decoder-geometry/sequence-resampling-alpha-0-2-2.jxl",
        UPSAMPLING,
    ),
    (
        "data/decoder-geometry/sequence-resampling-alpha-1-4-4.jxl",
        UPSAMPLING,
    ),
    (
        "data/decoder-geometry/sequence-resampling-main-0-2.jxl",
        UPSAMPLING,
    ),
    (
        "data/decoder-geometry/sequence-resampling-main-1-4.jxl",
        UPSAMPLING,
    ),
    ("data/effects_gray_alpha16.jxl", NOISE_SPLINES),
    ("data/effects_rgb_alpha8.jxl", NOISE_SPLINES),
    ("data/extra-channel-alpha-0-up-2-shift-0.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-0-up-2-shift-1.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-0-up-4-shift-0.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-0-up-4-shift-1.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-0-up-4-shift-2.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-0-up-8-shift-0.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-0-up-8-shift-1.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-0-up-8-shift-2.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-0-up-8-shift-3.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-1-up-2-shift-0.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-1-up-2-shift-1.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-1-up-4-shift-0.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-1-up-4-shift-1.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-1-up-4-shift-2.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-1-up-8-shift-0.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-1-up-8-shift-1.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-1-up-8-shift-2.jxl", UPSAMPLING),
    ("data/extra-channel-alpha-1-up-8-shift-3.jxl", UPSAMPLING),
    (
        "data/extra-channel-alpha-xyb-0-0-up-2-shift-0.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-0-up-2-shift-1.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-0-up-4-shift-0.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-0-up-4-shift-1.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-0-up-4-shift-2.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-0-up-8-shift-0.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-0-up-8-shift-1.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-0-up-8-shift-2.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-0-up-8-shift-3.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-1-up-2-shift-0.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-1-up-4-shift-0.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-1-up-4-shift-1.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-1-up-4-shift-2.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-1-up-8-shift-0.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-1-up-8-shift-1.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-1-up-8-shift-2.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-0-1-up-8-shift-3.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-0-up-2-shift-0.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-0-up-2-shift-1.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-0-up-4-shift-0.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-0-up-4-shift-1.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-0-up-4-shift-2.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-0-up-8-shift-0.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-0-up-8-shift-1.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-0-up-8-shift-2.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-0-up-8-shift-3.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-1-up-2-shift-0.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-1-up-2-shift-1.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-1-up-4-shift-0.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-1-up-4-shift-1.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-1-up-4-shift-2.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-1-up-8-shift-0.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-1-up-8-shift-1.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-1-up-8-shift-2.jxl",
        UPSAMPLING,
    ),
    (
        "data/extra-channel-alpha-xyb-1-1-up-8-shift-3.jxl",
        UPSAMPLING,
    ),
    ("data/extra-channel-xyb-0-up-2-2.jxl", UPSAMPLING),
    ("data/extra-channel-xyb-0-up-2-4.jxl", UPSAMPLING),
    ("data/extra-channel-xyb-0-up-4-4.jxl", UPSAMPLING),
    ("data/extra-channel-xyb-0-up-4-8.jxl", UPSAMPLING),
    ("data/extra-channel-xyb-0-up-8-8.jxl", UPSAMPLING),
    ("data/extra-channel-xyb-1-up-2-2.jxl", UPSAMPLING),
    ("data/extra-channel-xyb-1-up-4-4.jxl", UPSAMPLING),
    ("data/extra-channel-xyb-1-up-4-8.jxl", UPSAMPLING),
    ("data/extra-channel-xyb-1-up-8-8.jxl", UPSAMPLING),
    ("conformance/animation_spline", SPLINES),
    ("conformance/noise", NOISE),
    ("conformance/upsampling", UPSAMPLING),
];

/// The frame upsamples color (`upsampling` > 1, resampling) or extra
/// channels (`ec_upsampling`): the pipeline has `Upsample` stages, whose
/// `stage_upsampling` export is still the scalar translation.
const UPSAMPLING: &str = "upsampling: Upsample stages (stage_upsampling) still scalar";
/// The frame has the noise flag: `ConvNoise`/`AddNoise` (`stage_noise` with
/// `dec_noise`'s random generator) are still the scalar translation.
const NOISE: &str = "noise: ConvNoise/AddNoise (stage_noise, dec_noise) still scalar";
/// The frame has the splines flag: the `Splines` stage (`stage_splines` with
/// `splines`' DrawSegment) is still the scalar translation.
const SPLINES: &str = "splines: Splines stage (stage_splines, splines) still scalar";
/// Both noise and splines (also Gaborish and EPF, which match).
const NOISE_SPLINES: &str = "noise and splines: ConvNoise/AddNoise and Splines (stage_noise, dec_noise, stage_splines, splines) still scalar";

fn hex(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "-".into();
    }
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(text: &str) -> Vec<u8> {
    if text == "-" {
        return Vec::new();
    }
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
        .collect()
}

fn f32_bits(v: f32) -> u32 {
    v.to_bits()
}

fn f64_bits(v: f64) -> u64 {
    v.to_bits()
}

/// Run requests through an oracle executable (one response line each).
fn run_oracle(path: &Path, requests: &[String]) -> Vec<String> {
    let mut child = Command::new(path)
        .env("ROSETTA_ADVERTISE_AVX", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| panic!("start {}: {e}", path.display()));
    let mut input = child.stdin.take().unwrap();
    let mut lines = requests.join("\n");
    lines.push('\n');
    let feed = std::thread::spawn(move || input.write_all(lines.as_bytes()));
    let output = child.wait_with_output().expect("wait for the oracle");
    let fed = feed.join().unwrap();
    assert!(
        output.status.success(),
        "oracle {} failed ({}); write: {:?}; stderr: {}",
        path.display(),
        output.status,
        fed.as_ref().err(),
        String::from_utf8_lossy(&output.stderr)
    );
    fed.expect("write the oracle requests");
    let text = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<_> = text.lines().map(str::to_owned).collect();
    assert_eq!(lines.len(), requests.len(), "oracle response count");
    lines
}

fn simd_oracle() -> PathBuf {
    std::env::var_os("JXL_CPP_ORACLE_SIMD")
        .expect("set JXL_CPP_ORACLE_SIMD to the NEON/AVX2 oracle (python3 tools/check.py)")
        .into()
}

/// The scalar oracle, which encodes the scalar suite's public-decoder inputs
/// exactly as that suite does.
fn scalar_oracle() -> PathBuf {
    std::env::var_os("JXL_CPP_ORACLE").map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("target/oracle/jxl_oracle"),
        PathBuf::from,
    )
}

// ---------------------------------------------------------------------------
// Inputs.

fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data")
}

/// The files of `simd_parity.rs`'s `decode_vardct` whole-decode gate,
/// generated the same way (copied from `whole_decode_inputs` there).
fn vardct_gate_files(files: &mut Vec<(String, Vec<u8>)>) {
    fn next(seed: &mut u64) -> u64 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }
    fn test_image(w: usize, h: usize, channels: usize, seed: u64) -> Vec<u8> {
        let mut seed = seed | 1;
        let mut pixels = Vec::with_capacity(w * h * channels);
        for y in 0..h {
            for x in 0..w {
                for c in 0..channels {
                    let base = (x * (3 + c) + y * (5 - c) + (x * y) / (7 + c)) % 256;
                    let noise = (next(&mut seed) % 24) as usize;
                    pixels.push(((base + noise) % 256) as u8);
                }
            }
        }
        pixels
    }
    let mut names = Vec::new();
    let mut requests = Vec::new();
    for (w, h) in [(65, 33), (258, 258), (513, 257)] {
        for gab in [false, true] {
            for epf in 0..=3 {
                let pixels = test_image(w, h, 3, (w * 31 + h + epf) as u64);
                names.push(format!("vardct {w}x{h} gab={} epf={epf}", u8::from(gab)));
                requests.push(format!(
                    "vd-encode-effects {w} {h} 0 0 8 7 {} 0 {epf} 0 {}",
                    u8::from(gab),
                    hex(&pixels)
                ));
            }
        }
    }
    let jpeg = std::fs::read(data_dir().join("decode-test/flower.png.im_q85_420.jpg")).unwrap();
    requests.push(format!("ycc-jpeg-encode {}", hex(&jpeg)));
    names.push("jpeg 4:2:0 ycbcr (flower q85)".into());
    for (name, line) in names.into_iter().zip(run_oracle(&simd_oracle(), &requests)) {
        let fields: Vec<_> = line.split_whitespace().collect();
        let file = if name.starts_with("jpeg") {
            let w = fields.iter().position(|f| *f == "W").unwrap();
            assert_eq!(fields[w + 1], "0", "original JPEG recompression");
            unhex(fields[w + 3])
        } else {
            assert_eq!(fields[0], "0", "original encoder: {name}");
            unhex(fields[1])
        };
        files.push((name, file));
    }
}

/// The encoder specs of the scalar suite's public-decoder tests
/// (tests/parity/decoder_public.rs: SPECS, COLOR_SPECS, FRAME_SPECS,
/// RUNNER_SPECS), encoded by the scalar oracle as there.
const DPUB_SPECS: &[&str] = &[
    "w=8 h=8 c=3",
    "w=13 h=7 c=1 bits=16",
    "w=8 h=8 c=3 icc=1",
    "w=9 h=5 c=1 icc=1",
    "w=8 h=8 c=3 container=1",
    "w=8 h=8 c=3 container=1 box=Exif:20:0 box=xml:30:1 box=jumb:12:0 icc=1",
    "w=16 h=12 c=3 a=1 ec=2 names=1 orient=6 intrinsic=1 container=1 box=Exif:40:1",
    "w=12 h=10 c=3 frames=3 duration=5 timecodes=1 container=1 index=1",
    "w=12 h=10 c=3 frames=2 duration=3 container=1 box=xml:64:0 late=Exif:24:1 index=1",
    "w=8 h=8 c=3 container=1 level=10",
    "w=40 h=30 c=3 lossless=0 container=1 box=Exif:16:0 late=jumb:20:1",
    "w=2305 h=8 c=3 lossless=0 container=1 mode=2 box=xml:10:0",
    "w=8 h=8 c=3 a=1 premul=1 orient=3",
    "w=8 h=8 c=1 bits=16",
    "w=8 h=8 c=3 lossless=0",
    "w=8 h=8 c=1 lossless=0",
    "w=8 h=8 c=3 icc=1 lossless=0",
    "w=40 h=30 c=3 lossless=0",
    "w=20 h=14 c=3 a=1 frames=4 layers=1 container=1",
    "w=20 h=14 c=3 frames=4 layers=1 duration=2",
    "w=300 h=280 c=3 lossless=0 opt=16:1",
    "w=300 h=280 c=3 lossless=0 opt=17:1 opt=18:1",
    "w=300 h=280 c=3 lossless=0 opt=19:1",
    "w=64 h=48 c=3 lossless=0 opt=2:2",
    "w=64 h=48 c=3 lossless=0 opt=6:1",
    "w=64 h=64 c=3 opt=8:1",
    "w=64 h=48 c=3 lossless=0 opt=9:3 opt=10:1",
    "w=300 h=280 c=3 opt=26:0",
    "w=300 h=280 c=3 a=1 ec=1 effort=3 opt=26:0 opt=25:6",
    "w=300 h=280 c=1 bits=16 effort=2 opt=26:0",
    "w=300 h=280 c=3 a=1 bits=2 opt=26:0 opt=3:4",
    "w=300 h=280 c=3 a=1 effort=3 opt=26:0 opt=3:4",
    "w=300 h=220 c=3 lossless=0 opt=11:1 opt=26:0",
    "w=290 h=40 c=3 lossless=0",
    "w=290 h=40 c=3 lossless=0 opt=16:1",
    "w=290 h=40 c=3 lossless=0 opt=17:1 opt=18:1",
    "w=290 h=40 c=3 lossless=0 opt=19:1",
    "w=290 h=40 c=3 lossless=0 opt=6:1 opt=9:3 opt=10:1",
    "w=280 h=40 c=3 a=1 lossless=0 frames=2 duration=1",
];

/// decode_test.cc codestreams of the scalar suite's public-decoder
/// inventory (tests/parity/decode_test_inventory.rs) with a fixed spec, and
/// its frame sequences.
const DTI_SPECS: &[&str] = &[
    "123 77 4 0 lossless=1 tier=8 icc=1",
    "123 77 3 0 icc=1",
    "123 77 3 0",
    "512 300 3 0 noise=1",
    "123 77 4 0 lossless=1 tier=8",
    "177 123 3 0",
    "77 120 3 0 preview=1",
    "55 257 4 0 lossless=1 tier=8",
    "508 470 3 0 pdc=1 preview=1",
    "508 470 4 0 lossless=1 tier=8 responsive=1",
];
const DTI_FRAMES: &[&str] = &[
    "animation",
    "spot",
    "skip-current",
    "skip-current-ec",
    "skip",
    "skip-blend",
    "skip-alpha-blend",
];

fn public_test_files(files: &mut Vec<(String, Vec<u8>)>) {
    let scalar = scalar_oracle();
    let requests: Vec<_> = DPUB_SPECS
        .iter()
        .map(|spec| format!("dpub-encode {spec}"))
        .collect();
    for (spec, line) in DPUB_SPECS.iter().zip(run_oracle(&scalar, &requests)) {
        files.push((format!("dpub {spec}"), unhex(line.trim())));
    }
    let icc = hex(&std::fs::read(data_dir().join("decode-test/scRGB-v2.icc")).unwrap());
    let requests: Vec<_> = DTI_SPECS
        .iter()
        .map(|spec| format!("dti-codestream {icc} {spec}"))
        .chain(
            DTI_FRAMES
                .iter()
                .map(|name| format!("dti-frames {icc} {name}")),
        )
        .collect();
    let names = DTI_SPECS
        .iter()
        .map(|s| format!("dti {s}"))
        .chain(DTI_FRAMES.iter().map(|s| format!("dti frames {s}")));
    for (name, line) in names.zip(run_oracle(&scalar, &requests)) {
        let file = line.split(' ').next().unwrap();
        files.push((name, unhex(file)));
    }
    // Original encoder outputs retained in tests/data, as the scalar suite's
    // retained_files() selects them, and the decode_test.cc data files.
    let root = data_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&root)
        .unwrap()
        .chain(std::fs::read_dir(root.join("decoder-geometry")).unwrap())
        .chain(std::fs::read_dir(root.join("decode-test")).unwrap())
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension().is_some_and(|e| e == "jxl")
                && path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| !n.starts_with("sampling-") || n.contains("13x11"))
        })
        .collect();
    paths.sort();
    for path in paths {
        let name = path.strip_prefix(&root).unwrap().display().to_string();
        files.push((format!("data/{name}"), std::fs::read(&path).unwrap()));
    }
    // The files of the retained public-decoder captures.
    let captures = include_str!("../tools/captures/decoder_public.tsv");
    for (index, row) in captures.lines().enumerate() {
        let request = row.split('\t').next().unwrap();
        let file = request.split_whitespace().nth(3).unwrap();
        files.push((format!("capture {index}"), unhex(file)));
    }
}

/// The pinned conformance corpus inputs, if `JXL_CONFORMANCE_CORPUS` names
/// the `corpus` directory that `tools/conformance/corpus.py fetch` wrote and
/// verified against tools/conformance/MANIFEST.sha256.
fn conformance_files(files: &mut Vec<(String, Vec<u8>)>) -> bool {
    let Some(root) = std::env::var_os("JXL_CONFORMANCE_CORPUS") else {
        return false;
    };
    let manifest = include_str!("../tools/conformance/MANIFEST.sha256");
    let mut count = 0;
    for line in manifest.lines() {
        let Some((_, name)) = line.split_once("  ") else {
            continue;
        };
        let Some(case) = name
            .strip_prefix("testcases/")
            .and_then(|n| n.strip_suffix("/input.jxl"))
        else {
            continue;
        };
        let path = Path::new(&root).join(name);
        let bytes = std::fs::read(&path)
            .unwrap_or_else(|e| panic!("conformance input {}: {e}", path.display()));
        files.push((format!("conformance/{case}"), bytes));
        count += 1;
    }
    assert!(count > 0, "no conformance input listed");
    true
}

// ---------------------------------------------------------------------------
// The decode, mirrored token for token from decoder_public_simd.cc.

#[derive(Debug, PartialEq)]
enum Token {
    Text(String),
    Out(Vec<u8>),
}

fn parse_original(line: &str) -> Vec<Token> {
    line.split(' ')
        .map(
            |t| match t.strip_prefix("o:").or_else(|| t.strip_prefix("x:")) {
                Some(h) => Token::Out(unhex(h)),
                None => Token::Text(t.to_owned()),
            },
        )
        .collect()
}

fn basic_info(dec: &api::JxlDecoder, out: &mut Vec<Token>) {
    let mut info = JxlBasicInfo::zeroed();
    let status = api::jxl_decoder_get_basic_info(dec, Some(&mut info));
    let mut text = format!("b:{}", status.0);
    if status == Status::SUCCESS {
        text += &format!(
            ",{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            info.have_container,
            info.xsize,
            info.ysize,
            info.bits_per_sample,
            info.exponent_bits_per_sample,
            f32_bits(info.intensity_target),
            f32_bits(info.min_nits),
            info.relative_to_max_display,
            f32_bits(info.linear_below),
            info.uses_original_profile,
            info.have_preview,
            info.have_animation,
            info.orientation.0,
            info.num_color_channels,
            info.num_extra_channels,
            info.alpha_bits,
            info.alpha_exponent_bits,
            info.alpha_premultiplied,
            info.preview.xsize,
            info.preview.ysize,
            info.animation.tps_numerator,
            info.animation.tps_denominator,
            info.animation.num_loops,
            info.animation.have_timecodes,
            info.intrinsic_xsize,
            info.intrinsic_ysize,
        );
    }
    out.push(Token::Text(text));
    if status != Status::SUCCESS {
        return;
    }
    for i in 0..info.num_extra_channels as usize {
        let mut ec = JxlExtraChannelInfo::default();
        let status = api::jxl_decoder_get_extra_channel_info(dec, i, Some(&mut ec));
        let mut text = format!("e:{}", status.0);
        if status == Status::SUCCESS {
            text += &format!(
                ",{},{},{},{},{},{}",
                ec.channel_type.0,
                ec.bits_per_sample,
                ec.exponent_bits_per_sample,
                ec.dim_shift,
                ec.name_length,
                ec.alpha_premultiplied
            );
            for v in ec.spot_color {
                text += &format!(",{}", f32_bits(v));
            }
            text += &format!(",{}", ec.cfa_channel);
        }
        out.push(Token::Text(text));
    }
}

fn color(dec: &api::JxlDecoder, out: &mut Vec<Token>) {
    for target in 0..2 {
        let target = JxlColorProfileTarget(target);
        let mut size = 0usize;
        let status = api::jxl_decoder_get_icc_profile_size(dec, target, Some(&mut size));
        let mut text = format!("c:{},{size}", status.0);
        if status == Status::SUCCESS {
            let mut icc = vec![0u8; size];
            // SAFETY: The buffer holds `size` writable bytes.
            let s = unsafe {
                api::jxl_decoder_get_color_as_icc_profile(dec, target, icc.as_mut_ptr(), size)
            };
            text += &format!(",{},{}", s.0, hex(&icc));
        }
        out.push(Token::Text(text));
        let mut c = JxlColorEncoding::default();
        let status = api::jxl_decoder_get_color_as_encoded_profile(dec, target, Some(&mut c));
        let mut text = format!("C:{}", status.0);
        if status == Status::SUCCESS {
            text += &format!(
                ",{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                c.color_space.0,
                c.white_point.0,
                f64_bits(c.white_point_xy[0]),
                f64_bits(c.white_point_xy[1]),
                c.primaries.0,
                f64_bits(c.primaries_red_xy[0]),
                f64_bits(c.primaries_red_xy[1]),
                f64_bits(c.primaries_green_xy[0]),
                f64_bits(c.primaries_green_xy[1]),
                f64_bits(c.primaries_blue_xy[0]),
                f64_bits(c.primaries_blue_xy[1]),
                c.transfer_function.0,
                f64_bits(c.gamma),
                c.rendering_intent.0
            );
        }
        out.push(Token::Text(text));
    }
}

fn frame_header(dec: &api::JxlDecoder, out: &mut Vec<Token>) {
    let mut header = JxlFrameHeader::default();
    let status = api::jxl_decoder_get_frame_header(dec, Some(&mut header));
    let mut text = format!("h:{}", status.0);
    if status == Status::SUCCESS {
        let l = header.layer_info;
        text += &format!(
            ",{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            header.duration,
            header.timecode,
            header.name_length,
            header.is_last,
            l.have_crop,
            l.crop_x0,
            l.crop_y0,
            l.xsize,
            l.ysize,
            l.blend_info.blendmode.0,
            l.blend_info.source,
            l.blend_info.alpha,
            l.blend_info.clamp,
            l.save_as_reference
        );
    }
    out.push(Token::Text(text));
}

/// The port's public decoder on the decoder target selected now.
fn port_decode(data_type: u32, file: &[u8]) -> Vec<Token> {
    let mut out = Vec::new();
    // SAFETY: A null manager selects the default allocator; the decoder is
    // destroyed below and `file` outlives it.
    let raw = unsafe { api::jxl_decoder_create(std::ptr::null()) };
    out.push(Token::Text(format!("create:{}", u8::from(!raw.is_null()))));
    if raw.is_null() {
        return out;
    }
    // SAFETY: `raw` is live and uniquely owned until destroyed below.
    let dec = unsafe { &mut *raw };
    let events =
        Status::BASIC_INFO.0 | Status::COLOR_ENCODING.0 | Status::FRAME.0 | Status::FULL_IMAGE.0;
    out.push(Token::Text(format!(
        "s:{}",
        api::jxl_decoder_subscribe_events(dec, events as i32).0
    )));
    // SAFETY: `file` is immutable and outlives the decoder.
    let s = unsafe { api::jxl_decoder_set_input(dec, file.as_ptr(), file.len()) };
    out.push(Token::Text(format!("I:{}", s.0)));
    api::jxl_decoder_close_input(dec);
    let mut format = JxlPixelFormat {
        num_channels: 0,
        data_type: JxlDataType(data_type),
        endianness: JxlEndianness::NATIVE,
        align: 0,
    };
    let ec_format = JxlPixelFormat {
        num_channels: 1,
        ..format
    };
    let mut extras = 0u32;
    let mut image: Vec<u8> = Vec::new();
    let mut extra: Vec<Vec<u8>> = Vec::new();
    loop {
        // SAFETY: The input and every registered output buffer stay live and
        // unmoved until the decoder is destroyed or they are replaced.
        let status = unsafe { api::jxl_decoder_process_input(dec) };
        out.push(Token::Text(format!("P{}", status.0)));
        if status == Status::BASIC_INFO {
            basic_info(dec, &mut out);
            let mut info = JxlBasicInfo::zeroed();
            if api::jxl_decoder_get_basic_info(dec, Some(&mut info)) == Status::SUCCESS {
                format.num_channels = info.num_color_channels + u32::from(info.alpha_bits != 0);
                extras = info.num_extra_channels;
            }
        } else if status == Status::COLOR_ENCODING {
            color(dec, &mut out);
        } else if status == Status::FRAME {
            frame_header(dec, &mut out);
        } else if status == Status::NEED_IMAGE_OUT_BUFFER {
            let mut size = 0usize;
            let s = api::jxl_decoder_image_out_buffer_size(dec, Some(&format), Some(&mut size));
            out.push(Token::Text(format!("q:{},{size}", s.0)));
            if s != Status::SUCCESS {
                break;
            }
            image = vec![0; size];
            // SAFETY: `image` holds `size` writable bytes and is neither
            // moved nor dropped while the decoder may write it.
            let s = unsafe {
                api::jxl_decoder_set_image_out_buffer(
                    dec,
                    Some(&format),
                    image.as_mut_ptr().cast(),
                    size,
                )
            };
            out.push(Token::Text(format!("Q:{}", s.0)));
            extra = vec![Vec::new(); extras as usize];
            for i in 0..extras {
                let s = api::jxl_decoder_extra_channel_buffer_size(
                    dec,
                    Some(&ec_format),
                    Some(&mut size),
                    i,
                );
                let mut text = format!("v:{},{size}", s.0);
                if s == Status::SUCCESS {
                    extra[i as usize] = vec![0; size];
                    // SAFETY: As for `image`.
                    let s = unsafe {
                        api::jxl_decoder_set_extra_channel_buffer(
                            dec,
                            Some(&ec_format),
                            extra[i as usize].as_mut_ptr().cast(),
                            size,
                            i,
                        )
                    };
                    text += &format!(",{}", s.0);
                }
                out.push(Token::Text(text));
            }
        } else if status == Status::FULL_IMAGE {
            out.push(Token::Out(image.clone()));
            for e in &extra {
                out.push(Token::Out(e.clone()));
            }
        } else {
            break;
        }
    }
    // SAFETY: The sole owner destroys the decoder once.
    unsafe { api::jxl_decoder_destroy(raw) };
    out
}

// ---------------------------------------------------------------------------
// Comparison.

const FORMATS: [(u32, &str, usize); 3] = [(0, "f32", 4), (2, "u8", 1), (3, "u16", 2)];

/// Differing output words of one decode, or the first difference in status,
/// image info or output layout.
fn compare(port: &[Token], original: &[Token], word: usize) -> Result<(usize, usize), String> {
    if port.len() != original.len() {
        let first = port.iter().zip(original).position(|(a, b)| match (a, b) {
            (Token::Text(a), Token::Text(b)) => a != b,
            (Token::Out(_), Token::Out(_)) => false,
            _ => true,
        });
        return Err(format!(
            "token count {} != {} (first differing token {first:?}: {:?} / {:?})",
            port.len(),
            original.len(),
            first.and_then(|i| port.get(i)).map(short),
            first.and_then(|i| original.get(i)).map(short),
        ));
    }
    let (mut differing, mut total) = (0, 0);
    for (i, (a, b)) in port.iter().zip(original).enumerate() {
        match (a, b) {
            (Token::Text(a), Token::Text(b)) if a == b => {}
            (Token::Out(a), Token::Out(b)) if a.len() == b.len() => {
                for (x, y) in a.chunks(word).zip(b.chunks(word)) {
                    total += 1;
                    if x == y {
                        continue;
                    }
                    if word == 4 {
                        let x = u32::from_ne_bytes(x.try_into().unwrap());
                        let y = u32::from_ne_bytes(y.try_into().unwrap());
                        if is_nan(x) && is_nan(y) {
                            continue;
                        }
                    }
                    differing += 1;
                }
            }
            _ => {
                return Err(format!(
                    "token {i}: port {:?} != original {:?}",
                    short(a),
                    short(b)
                ));
            }
        }
    }
    Ok((differing, total))
}

fn short(token: &Token) -> String {
    match token {
        Token::Text(t) if t.len() > 80 => format!("{}...", &t[..80]),
        Token::Text(t) => t.clone(),
        Token::Out(o) => format!("<{} output bytes>", o.len()),
    }
}

/// The frame headers, pipelines and modular inverse transforms of a decode
/// (jxl::jxl::decode_trace), deduplicated in order.
fn reach(file: &[u8]) -> String {
    jxl::jxl::decode_trace::start();
    let _ = port_decode(0, file);
    let events = jxl::jxl::decode_trace::stop();
    let mut seen = HashSet::new();
    events
        .into_iter()
        .filter(|e| seen.insert(e.clone()))
        .collect::<Vec<_>>()
        .join("; ")
}

/// Whole decodes through the public decoder with the SIMD target forced
/// (`set_decoder_simd_for_test`) against the original's public decoder built
/// for the same single Highway target, independent of the default selection
/// `hwy::DECODER_SELECTS_SIMD` (working decision 2026-11-13, delegated,
/// revisable by the owner). The punch list is [`STILL_DIFFERS`].
#[test]
fn public_decodes_with_the_simd_target_against_the_original_target() {
    let _guard = OVERRIDE.lock().unwrap_or_else(|e| e.into_inner());
    let t = target();
    let mut files = Vec::new();
    vardct_gate_files(&mut files);
    public_test_files(&mut files);
    let corpus = conformance_files(&mut files);
    // One entry per distinct input; the first name is kept.
    let mut seen = HashSet::new();
    files.retain(|(_, bytes)| seen.insert(bytes.clone()));
    let names: HashSet<_> = files.iter().map(|(n, _)| n.as_str()).collect();
    let listed: BTreeMap<&str, &str> = STILL_DIFFERS.iter().copied().collect();
    assert_eq!(
        listed.len(),
        STILL_DIFFERS.len(),
        "duplicate punch-list entry"
    );
    for name in listed.keys() {
        assert!(
            names.contains(name) || (!corpus && name.starts_with("conformance/")),
            "punch-list entry {name:?} names no input of this test"
        );
    }
    if !corpus {
        println!(
            "JXL_CONFORMANCE_CORPUS is not set: the conformance corpus is not compared (docs/CONFORMANCE.md)"
        );
    }
    let oracle = simd_oracle();
    println!(
        "public decode on {}: file | f32 | u8 | u16 differing words (of total) | punch list",
        t.name()
    );
    let mut unexpected = Vec::new();
    let mut now_matching = Vec::new();
    let mut still = 0;
    for (name, bytes) in &files {
        let requests: Vec<_> = FORMATS
            .iter()
            .map(|(ty, _, _)| format!("dps-decode {ty} {}", hex(bytes)))
            .collect();
        let originals = run_oracle(&oracle, &requests);
        set_decoder_simd_for_test(Some(true));
        assert_eq!(decoder_target().map(|d| d.name()), Some(t.name()));
        let mut cells = Vec::new();
        let mut differs = false;
        for ((ty, _, word), line) in FORMATS.iter().zip(&originals) {
            let port = port_decode(*ty, bytes);
            match compare(&port, &parse_original(line), *word) {
                Ok((0, total)) => cells.push(format!("0 ({total})")),
                Ok((n, total)) => {
                    differs = true;
                    cells.push(format!("{n} ({total})"));
                }
                Err(e) => {
                    differs = true;
                    cells.push(format!("structure: {e}"));
                }
            }
        }
        let reached = if differs { reach(bytes) } else { String::new() };
        set_decoder_simd_for_test(None);
        let reason = listed.get(name.as_str());
        println!(
            "  {name} | {} | {}",
            cells.join(" | "),
            reason
                .copied()
                .unwrap_or(if differs { "UNEXPECTED" } else { "-" })
        );
        if differs {
            println!("    reaches: {reached}");
        }
        match (differs, reason) {
            (true, None) => unexpected.push(name.clone()),
            (false, Some(_)) => now_matching.push(name.clone()),
            (true, Some(_)) => still += 1,
            (false, None) => {}
        }
    }
    println!(
        "{} files compared on {}: {} match, {still} still differ as listed, {} differ unexpectedly, {} listed now match",
        files.len(),
        t.name(),
        files.len() - still - unexpected.len() - now_matching.len(),
        unexpected.len(),
        now_matching.len()
    );
    assert!(
        now_matching.is_empty(),
        "these files now match the original's {} build: remove them from STILL_DIFFERS in tests/simd_public.rs (and from the punch list in ADR 0004): {now_matching:?}",
        t.name()
    );
    assert!(
        unexpected.is_empty(),
        "these whole public decodes with the {} target differ from the original's {} build and are not on the punch list: {unexpected:?}",
        t.name(),
        t.name()
    );
}

/// Condition (c): the scalar gate (`tools/check.py --stage test`, which sets
/// `JXL_CPP_ORACLE` and `JXL_DECODER_TARGET=scalar`) runs the scalar decoder
/// path even if `DECODER_SELECTS_SIMD` were on; only an explicit test
/// override selects a SIMD target.
#[test]
fn the_scalar_gate_forces_the_scalar_decoder_path() {
    let _guard = OVERRIDE.lock().unwrap_or_else(|e| e.into_inner());
    let gate = std::env::var_os("JXL_CPP_ORACLE").is_some();
    let forced = std::env::var_os("JXL_DECODER_TARGET").is_some_and(|v| v == "scalar");
    if gate {
        assert!(
            forced,
            "tools/check.py's test stage must set JXL_DECODER_TARGET=scalar"
        );
    }
    assert_eq!(scalar_forced_by_environment(), forced);
    set_decoder_simd_for_test(None);
    if forced {
        // The default selection, whatever DECODER_SELECTS_SIMD is.
        assert!(decoder_target_for(true).is_none());
        assert!(decoder_target_for(false).is_none());
        assert!(decoder_target().is_none());
        assert_eq!(jxl::jxl::hwy::max_vector_size(), 4);
    } else {
        assert_eq!(
            decoder_target_for(true).map(|d| d.name()),
            Some(target().name())
        );
        assert_eq!(
            decoder_target_for(DECODER_SELECTS_SIMD).is_some(),
            DECODER_SELECTS_SIMD
        );
    }
    // An explicit override still wins in both directions.
    set_decoder_simd_for_test(Some(true));
    assert_eq!(
        decoder_target_for(false).map(|d| d.name()),
        Some(target().name())
    );
    set_decoder_simd_for_test(Some(false));
    assert!(decoder_target_for(true).is_none());
    set_decoder_simd_for_test(None);
    println!("scalar gate: JXL_CPP_ORACLE set: {gate}, scalar forced by the environment: {forced}");
}
