// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The lib/jxl/decode_test.cc inventory. Each test's call history runs on its
// genuine inputs, built in the oracle by the test's own pinned helpers
// (dti-* commands), on the original public API and on the Rust port.
// Statuses, events, getters, consumed and released byte counts, box and JPEG
// bytes and output digests compare exactly. Histories with a parallel runner
// run only on the original, which must then match its own runner-free
// history; the Rust decoder runs serially, as the original does with a null
// runner. The inventory table is in docs/PORT_STATUS.md.

const BASIC_INFO: i32 = 0x40;
const COLOR_ENCODING: i32 = 0x100;
const PREVIEW_IMAGE: i32 = 0x200;
const FRAME: i32 = 0x400;
const FULL_IMAGE: i32 = 0x1000;
const JPEG_RECONSTRUCTION: i32 = 0x2000;
const BOX: i32 = 0x4000;
const FRAME_PROGRESSION: i32 = 0x8000;

// CodeStreamBoxFormat of decode_test.cc.
const CSBF_NONE: usize = 0;
const CSBF_SINGLE: usize = 1;
const CSBF_MULTI: usize = 4;
const CSBF_MULTI_OTHER_TERMINATED: usize = 6;
const CSBF_MULTI_OTHER_ZERO_TERMINATED: usize = 7;
const CSBF_BROB_EXIF: usize = 10;
const CSBF_NUM_ENTRIES: usize = 11;

// Pixel format tokens: channels.type.endianness.align, with FLOAT=0, UINT8=2,
// UINT16=3, FLOAT16=5 and NATIVE=0, LITTLE=1, BIG=2.
const OUT_FORMATS: [(u32, u32); 6] = [(0, 2), (1, 3), (2, 3), (0, 5), (1, 0), (2, 0)];

fn hex(data: &[u8]) -> String {
    if data.is_empty() {
        return "-".into();
    }
    data.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(data: &str) -> Vec<u8> {
    if data == "-" {
        return Vec::new();
    }
    (0..data.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&data[i..i + 2], 16).unwrap())
        .collect()
}

fn test_data(name: &str) -> Vec<u8> {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    std::fs::read(root.join(name)).unwrap()
}

/// The pinned testdata profile returned by GetIccTestProfile.
fn icc() -> String {
    hex(&test_data("decode-test/scRGB-v2.icc"))
}

/// A genuine cjpeg file replacing the extras libjpeg encoder output.
fn jpeg_input() -> String {
    hex(&test_data("jpeg-writer/progressive.jpg"))
}

fn oracle(requests: &[String]) -> Vec<String> {
    super::original_observations(requests)
}

/// CreateTestJXLCodestream on GetSomeTestImage(w, h, channels, seed). Returns
/// the file and, for JPEG inputs, the JPEG bytes it reconstructs.
fn codestreams(specs: &[String]) -> Vec<(String, String)> {
    let icc = icc();
    let requests: Vec<String> = specs
        .iter()
        .map(|spec| format!("dti-codestream {icc} {spec}"))
        .collect();
    oracle(&requests)
        .into_iter()
        .map(|line| {
            let mut fields = line.split(' ');
            let file = fields.next().unwrap().to_owned();
            let jpeg = fields.next().unwrap_or("-").to_owned();
            (file, jpeg)
        })
        .collect()
}

fn codestream(spec: &str) -> String {
    codestreams(&[spec.to_owned()]).remove(0).0
}

/// The frame sequences built by the test bodies.
fn frames(name: &str) -> String {
    oracle(&[format!("dti-frames {} {name}", icc())]).remove(0)
}

/// GetTestHeader.
#[expect(clippy::too_many_arguments, reason = "mirrors the source helper")]
fn header(
    xsize: usize,
    ysize: usize,
    bits: usize,
    orientation: usize,
    alpha_bits: usize,
    xyb: bool,
    container: bool,
    insert_box: bool,
    with_icc: bool,
) -> String {
    oracle(&[format!(
        "dti-header {} {xsize} {ysize} {bits} {orientation} {alpha_bits} {} {} 0 {} {}",
        icc(),
        u8::from(xyb),
        u8::from(container),
        u8::from(insert_box),
        u8::from(with_icc)
    )])
    .remove(0)
}

#[derive(Debug)]
struct FramePositions {
    frame_start: usize,

    section_end: Vec<usize>,
}

#[derive(Debug)]
struct StreamPositions {
    jbrd_end: usize,
    frames: Vec<FramePositions>,
}

/// AnalyzeCodestream.
fn analyze(file: &str) -> StreamPositions {
    let line = oracle(&[format!("dti-analyze {file}")]).remove(0);
    let mut v = line.split(' ').map(|f| f.parse::<usize>().unwrap());
    let mut next = || v.next().unwrap();
    let _codestream_start = next();
    let _codestream_end = next();
    let _basic_info = next();
    let jbrd_end = next();
    let boxes = next();
    for _ in 0..boxes {
        next();
    }
    let count = next();
    let frames = (0..count)
        .map(|_| {
            let frame_start = next();
            let _header_end = next();
            let _toc_end = next();
            let sections = next();
            FramePositions {
                frame_start,

                section_end: (0..sections).map(|_| next()).collect(),
            }
        })
        .collect();
    StreamPositions { jbrd_end, frames }
}

fn run(file: &str, ops: &str) -> String {
    format!("dpub-run 0 0 {file} {ops}")
}

fn file_len(file: &str) -> usize {
    if file == "-" { 0 } else { file.len() / 2 }
}

/// Compares Rust with already observed original histories and retains them
/// for the public decoder audit.
fn compare_observed(family: &str, requests: &[String], originals: &[String]) {
    if let Some(path) = std::env::var_os("JXL_DECODER_PUBLIC_CAPTURE") {
        use std::io::Write;
        let path = std::path::PathBuf::from(path);
        std::fs::create_dir_all(&path).unwrap();
        let mut output = std::io::BufWriter::new(
            std::fs::File::create(path.join(format!("{family}.requests"))).unwrap(),
        );
        for request in requests {
            writeln!(output, "{request}").unwrap();
        }
    }
    assert_eq!(requests.len(), originals.len());
    for (request, cpp) in requests.iter().zip(originals) {
        assert_eq!(&super::observe(request), cpp, "request: {request}");
    }
    println!(
        "{family}: {} C++/Rust differential cases matched",
        requests.len()
    );
}

fn compare(family: &str, requests: Vec<String>) {
    compare_expecting(family, requests, |_| true);
}

/// The statuses returned by the history's ProcessInput operations.
fn statuses(observation: &str) -> Vec<&str> {
    observation
        .split(' ')
        .filter_map(|token| token.strip_prefix("p:"))
        .flat_map(|statuses| statuses.split(','))
        .collect()
}

/// The statuses of a streaming loop's ProcessInput calls.
fn loop_statuses(observation: &str) -> Vec<&str> {
    observation
        .split(' ')
        .filter(|token| token.len() > 1 && token.starts_with('A') && token != &"A:0")
        .map(|token| {
            let status = token.split_once(':').map_or(&token[1..], |(_, s)| s);
            status.split(',').next().unwrap()
        })
        .collect()
}

/// Whether the Rust port's history reaches the original's undefined raw-box
/// memcpy with a null input; such histories run original-only.
fn reaches_null_copy(request: &str) -> bool {
    jxl::jxl::box_content_decoder::take_null_input_copies();
    super::observe(request);
    jxl::jxl::box_content_decoder::take_null_input_copies() != 0
}

/// Also requires that the original follows the test's expectations, so each
/// history takes the path of the test it ports.
fn compare_expecting(family: &str, requests: Vec<String>, expected: impl Fn(&str) -> bool) {
    let (undefined, requests): (Vec<String>, Vec<String>) =
        requests.into_iter().partition(|r| reaches_null_copy(r));
    let originals = oracle(&requests);
    for (request, original) in requests.iter().zip(&originals) {
        assert!(
            expected(original),
            "unexpected original history: {request}\n{original}"
        );
    }
    compare_observed(family, &requests, &originals);
    super::decoder_public::null_copy_probes(&undefined);
}

fn is_runner_op(op: &str) -> bool {
    op.starts_with('@') || op.starts_with('#')
}

fn without_runner(request: &str) -> String {
    request
        .split(' ')
        .filter(|op| !is_runner_op(op))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Histories with parallel runners: the original with each runner must match
/// its own runner-free history, which Rust then reproduces serially.
fn compare_with_runners(family: &str, requests: Vec<String>, expected: impl Fn(&str) -> bool) {
    let (undefined, requests): (Vec<String>, Vec<String>) = requests
        .into_iter()
        .partition(|r| reaches_null_copy(&without_runner(r)));
    let undefined: Vec<String> = undefined.iter().map(|r| without_runner(r)).collect();
    let plain: Vec<String> = requests.iter().map(|r| without_runner(r)).collect();
    let serial = oracle(&plain);
    for (request, original) in plain.iter().zip(&serial) {
        assert!(
            expected(original),
            "unexpected original history: {request}\n{original}"
        );
    }
    let threaded = oracle(&requests);
    for ((request, serial), threaded) in requests.iter().zip(&serial).zip(&threaded) {
        let stripped = threaded
            .split(' ')
            .filter(|token| !(token.starts_with("@:") || token.starts_with("#:")))
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(
            &stripped, serial,
            "runner changes the original history: {request}"
        );
    }
    compare_observed(family, &plain, &serial);
    super::decoder_public::null_copy_probes(&undefined);
}

/// DecodeWithAPI's expectations: basic info, then optional color, preview and
/// frame events, the full image of every decode and a final status.
fn decodes_with_api(observation: &str) -> bool {
    // ProcessInputIgnoreBoxes passes over box events.
    let statuses: Vec<&str> = statuses(observation)
        .into_iter()
        .filter(|s| *s != "16384")
        .collect();
    statuses.first() == Some(&"64")
        && statuses.contains(&"4096")
        && statuses.iter().all(|s| {
            [
                "0", "2", "3", "5", "64", "256", "512", "1024", "4096", "16384",
            ]
            .contains(s)
        })
        && !observation.contains(",overflow")
}

/// No ProcessInput call of the history fails.
fn never_fails(observation: &str) -> bool {
    !statuses(observation).contains(&"1") && !loop_statuses(observation).contains(&"1")
}

/// DecodeWithAPI of decode_test.cc.
#[derive(Clone, Copy)]
struct Api {
    channels: u32,
    data_type: u32,
    endianness: u32,
    align: usize,
    callback: bool,
    early: bool,
    resizable: bool,
    boxes: bool,
    icc: bool,
    preview: bool,
}

impl Api {
    fn new(channels: u32, (endianness, data_type): (u32, u32)) -> Self {
        Self {
            channels,
            data_type,
            endianness,
            align: 0,
            callback: false,
            early: false,
            resizable: false,
            boxes: false,
            icc: false,
            preview: false,
        }
    }

    fn ops(&self, len: usize) -> String {
        let events = BASIC_INFO
            | if self.early { FRAME } else { 0 }
            | PREVIEW_IMAGE
            | FULL_IMAGE
            | if self.boxes { BOX } else { 0 }
            | if self.icc { COLOR_ENCODING } else { 0 };
        let p = if self.boxes { "p1" } else { "p" };
        let mut ops = vec![
            (if self.resizable { "@2" } else { "@1" }).to_owned(),
            format!("s{events}"),
            format!("I0,{len}"),
            p.into(),
            format!(
                "F{}.{}.{}.{}.{}",
                self.channels,
                self.data_type,
                self.endianness,
                self.align,
                if self.callback { 4 } else { 0 }
            ),
            "q".into(),
            "b".into(),
        ];
        if self.resizable {
            ops.push("#".into());
        }
        ops.push(p.into());
        if self.icc {
            ops.extend(["z1".into(), "Z1,?".into(), p.into()]);
        }
        if self.preview {
            ops.extend(["H".into(), p.into(), "X".into(), p.into()]);
        }
        ops.extend(["U".into(), p.into(), "X".into(), p.into()]);
        ops.join(" ")
    }
}

#[test]
fn decode_test_signature_and_allocation() {
    // JxlSignatureCheckTest.
    let signatures: [&[u8]; 10] = [
        b"a",
        b"abcdef",
        b"",
        &[0xff],
        &[0xff, 0xd8],
        &[0xff, 0x0a],
        &[0, 0, 0, 0xc, b'J', b'X', b'L', b' ', 0xd, 0xa, 0x87, 0xa],
        &[0, 0, 0, 0xc, b'J', b'X', b'L', b' ', 0xd, 0xa, 0x87, 0],
        &[0, 0, 0, 0xc, b'J', b'X', b'L', b' ', 0xd, 0xa, 0x87],
        &[0],
    ];
    let mut requests: Vec<String> = signatures
        .iter()
        .map(|s| run(&hex(s), &format!("!{}", s.len())))
        .collect();
    // DefaultAllocTest and CustomAllocTest, with the custom manager's log.
    requests.push("dpub-run 0 0 - ".into());
    requests.push("dpub-run 1 0 - ".into());
    compare("dt-signature", requests);
    // DefaultParallelRunnerTest: JxlDecoderSetParallelRunner(dec, nullptr,
    // nullptr) exists only in the original.
    compare_with_runners("dt-null-runner", vec!["dpub-run 0 0 - @0".into()], |o| {
        o == " create:1 E"
    });
}

#[test]
fn decode_test_basic_info_headers() {
    let mut requests = Vec::new();
    // BasicInfoTest: every prefix of two headers.
    for (file, alpha) in [
        (header(50, 50, 8, 3, 0, false, false, false, false), false),
        (header(33, 77, 23, 5, 8, false, true, false, false), true),
    ] {
        let len = file_len(&file);
        for size in 0..=len {
            let mut ops = format!("s{BASIC_INFO} I0,{size} p b");
            if size == len {
                if alpha {
                    ops.push_str(" e0 n0,11");
                }
                ops.push_str(" p");
            }
            ops.push_str(&format!(" s{BASIC_INFO} x s{BASIC_INFO}"));
            requests.push(run(&file, &ops));
        }
    }
    // BufferSizeTest.
    let file = header(33, 77, 8, 1, 8, false, false, false, false);
    requests.push(run(
        &file,
        &format!("s{BASIC_INFO} I0,{} p b F4.2.1.0.0 q", file_len(&file)),
    ));
    // BasicInfoSizeHintTest.
    let file = header(50, 50, 16, 1, 0, false, true, true, false);
    requests.push(run(&file, &format!("s{BASIC_INFO} h Ih p b0 i h Ih p b")));
    compare("dt-basic-info", requests);
}

#[test]
fn decode_test_icc_profiles() {
    let events = BASIC_INFO | COLOR_ENCODING;
    let icc_len = file_len(&icc());
    let srgb = oracle(&["dti-external srgb".into()]).remove(0);
    let linear = oracle(&["dti-external linear".into()]).remove(0);
    let original = header(50, 50, 16, 1, 0, false, false, false, true);
    let xyb = header(50, 50, 16, 1, 0, true, false, false, true);
    let requests = vec![
        // IccProfileTestOriginal.
        run(
            &original,
            &format!(
                "s{events} I0,{} p b p E0- z0 z0- Z0,{icc_len} E1- z1",
                file_len(&original)
            ),
        ),
        // IccProfileTestXybEncoded.
        run(
            &xyb,
            &format!(
                "s{events} I0,{} p b p E0- z0- z0 Z0,{icc_len} E1- E1 E1 P{srgb} E1 z1 Z1,? P{linear} E1 z1 Z1,?",
                file_len(&xyb)
            ),
        ),
        // ICCPartialTest: one more byte per call.
        run(&original, &format!("s{events} A1,1,-,0,100000")),
    ];
    compare("dt-icc", requests);
}

struct PixelConfig {
    grayscale: bool,
    alpha: bool,
    xsize: usize,
    ysize: usize,
    preview: usize,
    intrinsic: bool,
    format: (u32, u32),
    output_channels: u32,
    box_format: usize,
    callback: bool,
    early: bool,
    resizable: bool,
    orientation: usize,
    keep: bool,
    upsampling: usize,
}

/// GeneratePixelTests.
fn pixel_tests() -> Vec<PixelConfig> {
    let ch_info = [
        (false, true, 4),
        (true, false, 1),
        (true, true, 1),
        (true, true, 2),
        (false, false, 3),
        (false, true, 3),
        (false, false, 4),
    ];
    let mut all = Vec::new();
    let make = |all: &mut Vec<PixelConfig>,
                ch: (bool, bool, u32),
                xsize: usize,
                ysize: usize,
                preview: usize,
                intrinsic: bool,
                box_format: usize,
                orientation: usize,
                keep: bool,
                format: (u32, u32),
                callback: bool,
                early: bool,
                resizable: bool,
                upsampling: usize| {
        all.push(PixelConfig {
            grayscale: ch.0,
            alpha: ch.1,
            xsize,
            ysize,
            preview,
            intrinsic,
            format,
            output_channels: ch.2,
            box_format,
            callback,
            early,
            resizable,
            orientation,
            keep,
            upsampling,
        });
    };
    for ch in ch_info {
        for callback in [false, true] {
            for upsampling in [1, 2, 4, 8] {
                for format in OUT_FORMATS {
                    make(
                        &mut all, ch, 301, 33, 0, false, CSBF_NONE, 1, false, format, callback,
                        false, false, upsampling,
                    );
                }
            }
        }
    }
    for box_format in 1..CSBF_NUM_ENTRIES {
        make(
            &mut all,
            ch_info[0],
            77,
            33,
            0,
            false,
            box_format,
            1,
            false,
            OUT_FORMATS[0],
            false,
            false,
            false,
            1,
        );
    }
    for preview in 0..3 {
        make(
            &mut all,
            ch_info[0],
            77,
            33,
            preview,
            false,
            CSBF_NONE,
            1,
            false,
            OUT_FORMATS[0],
            false,
            false,
            false,
            1,
        );
    }
    for intrinsic in [false, true] {
        make(
            &mut all,
            ch_info[0],
            55,
            34,
            0,
            intrinsic,
            CSBF_NONE,
            1,
            false,
            OUT_FORMATS[0],
            false,
            false,
            false,
            1,
        );
    }
    make(
        &mut all,
        ch_info[0],
        300,
        33,
        0,
        false,
        CSBF_NONE,
        1,
        false,
        OUT_FORMATS[0],
        false,
        true,
        false,
        1,
    );
    for i in 0..4 {
        make(
            &mut all,
            ch_info[0],
            300 << i,
            33 << i,
            0,
            false,
            CSBF_NONE,
            1,
            false,
            OUT_FORMATS[0],
            false,
            false,
            true,
            1,
        );
    }
    for orientation in 2..=8 {
        for keep in [false, true] {
            for callback in [false, true] {
                for ch in ch_info {
                    for format in OUT_FORMATS {
                        make(
                            &mut all,
                            ch,
                            280,
                            12,
                            0,
                            false,
                            CSBF_NONE,
                            orientation,
                            keep,
                            format,
                            callback,
                            true,
                            false,
                            1,
                        );
                    }
                }
            }
        }
    }
    all
}

/// Encodes each distinct spec once.
fn unique_codestreams(specs: &[String]) -> std::collections::HashMap<String, String> {
    let mut unique: Vec<String> = specs.to_vec();
    unique.sort();
    unique.dedup();
    let files = codestreams(&unique);
    unique
        .into_iter()
        .zip(files)
        .map(|(spec, (file, _))| (spec, file))
        .collect()
}

fn pixel_requests(select: impl Fn(&PixelConfig) -> bool) -> Vec<String> {
    let configs: Vec<PixelConfig> = pixel_tests().into_iter().filter(select).collect();
    let specs: Vec<String> = configs
        .iter()
        .map(|c| {
            let channels = if c.grayscale { 1 } else { 3 } + u32::from(c.alpha);
            format!(
                "{} {} {channels} 0 lossless=1 tier=8 resampling={u} ec_resampling={u} box={} orient={} preview={} intrinsic={}",
                c.xsize,
                c.ysize,
                c.box_format,
                c.orientation,
                c.preview,
                u8::from(c.intrinsic),
                u = c.upsampling
            )
        })
        .collect();
    let files = unique_codestreams(&specs);
    let mut requests = Vec::new();
    for (c, spec) in configs.iter().zip(&specs) {
        let file = &files[spec];
        let mut api = Api::new(c.output_channels, c.format);
        api.callback = c.callback;
        api.early = c.early;
        api.resizable = c.resizable;
        api.preview = c.preview != 0;
        let keep = if c.keep { "k1 " } else { "" };
        let ops = api.ops(file_len(file));
        requests.push(run(file, &format!("{keep}{ops} x")));
        if c.callback {
            // Without a runner, every callback call also compares in order:
            // its position, length and pixels (output mode 1).
            let serial = without_runner(&ops).replacen(".4 q b", ".1 q b", 1);
            requests.push(run(file, &format!("{keep}{serial} x")));
        }
    }
    requests
}

#[test]
fn decode_test_pixel_test_formats() {
    // PixelTest: output formats, methods and upsampling.
    compare_with_runners(
        "dt-pixel-formats",
        pixel_requests(|c| c.xsize == 301),
        decodes_with_api,
    );
}

#[test]
fn decode_test_pixel_test_containers_previews_and_runners() {
    // PixelTest: codestream boxes, previews, intrinsic sizes, early buffers
    // and the resizable runner.
    compare_with_runners(
        "dt-pixel-boxes",
        pixel_requests(|c| c.xsize != 301 && c.xsize != 280),
        decodes_with_api,
    );
}

#[test]
fn decode_test_pixel_test_orientations() {
    // PixelTest: orientations, kept or applied.
    compare_with_runners(
        "dt-pixel-orientation",
        pixel_requests(|c| c.xsize == 280),
        decodes_with_api,
    );
}

#[test]
fn decode_test_pixel_icc_and_lossy() {
    let mut requests = Vec::new();
    // PixelTestWithICCProfileLossless: one decoder, reset between decodes.
    let file = codestream("123 77 4 0 lossless=1 tier=8 icc=1");
    let len = file_len(&file);
    let mut ops = Vec::new();
    for channels in 3..=4 {
        ops.push(format!("{} x", Api::new(channels, (1, 2)).ops(len)));
        let mut api = Api::new(channels, (1, 3));
        api.callback = true;
        api.early = true;
        ops.push(format!("{} x", api.ops(len)));
        ops.push(format!("{} x", Api::new(channels, (1, 0)).ops(len)));
    }
    requests.push(run(&file, &ops.join(" ")));
    // PixelTestWithICCProfileLossy.
    let file = codestream("123 77 3 0 icc=1");
    let mut api = Api::new(3, (1, 0));
    api.early = true;
    api.icc = true;
    requests.push(run(&file, &format!("{} x", api.ops(file_len(&file)))));
    // PixelTestOpaqueSrgbLossy.
    let file = codestream("123 77 3 0");
    for channels in 3..=4 {
        let mut api = Api::new(channels, (1, 2));
        api.callback = true;
        requests.push(run(&file, &format!("{} x", api.ops(file_len(&file)))));
    }
    // PixelTestOpaqueSrgbLossyNoise.
    let file = codestream("512 300 3 0 noise=1");
    for channels in 3..=4 {
        let mut api = Api::new(channels, (1, 2));
        api.early = true;
        requests.push(run(&file, &format!("{} x", api.ops(file_len(&file)))));
    }
    // AlignTest: a fresh decoder per method.
    let file = codestream("123 77 4 0 lossless=1 tier=8");
    for callback in [false, true] {
        let mut api = Api::new(3, (1, 2));
        api.align = 17;
        api.callback = callback;
        requests.push(run(&file, &api.ops(file_len(&file))));
    }
    // Callback decodes also compare every call in order without a runner.
    let ordered: Vec<String> = requests
        .iter()
        .filter(|r| r.contains(".4 q b"))
        .map(|r| without_runner(r).replace(".4 q b", ".1 q b"))
        .collect();
    requests.extend(ordered);
    compare_with_runners("dt-pixel-icc", requests, decodes_with_api);
}

/// One color encoding of dti-encodings.
struct Encoding {
    description: String,
    color_space: u32,
    white_point: u32,
    primaries: u32,
    pq: bool,
    hlg: bool,
    intent: u32,
    channels: u32,
    external: String,
    parsed: String,
    icc: String,
    parsed_icc: String,
}

/// AllEncodings, then the gray source of SetPreferredColorProfileTestFromGray.
fn encodings() -> Vec<Encoding> {
    oracle(&["dti-encodings".into()])
        .remove(0)
        .split('|')
        .map(|item| {
            let f: Vec<&str> = item.split(' ').collect();
            Encoding {
                description: f[0].into(),
                color_space: f[1].parse().unwrap(),
                white_point: f[2].parse().unwrap(),
                primaries: f[3].parse().unwrap(),
                pq: f[4] == "1",
                hlg: f[5] == "1",
                intent: f[6].parse().unwrap(),
                channels: f[7].parse().unwrap(),
                external: f[8].into(),
                parsed: f[9].into(),
                icc: f[10].into(),
                parsed_icc: f[11].into(),
            }
        })
        .collect()
}

// jxl::RenderingIntent, WhitePoint and Primaries values.
const RELATIVE: u32 = 1;
const D65: u32 = 1;
const PRIMARIES_SRGB: u32 = 1;
const PRIMARIES_2100: u32 = 9;
const PRIMARIES_P3: u32 = 11;
const XYB: u32 = 2;

#[test]
fn decode_test_preserve_original_profile() {
    let all = encodings();
    let sources: Vec<&Encoding> = all[..all.len() - 1]
        .iter()
        .filter(|c| c.intent == RELATIVE)
        .collect();
    let specs: Vec<String> = sources
        .iter()
        .map(|c| {
            format!(
                "123 77 3 0 cs={} intensity={}",
                c.description,
                if c.pq { 10000 } else { 255 }
            )
        })
        .collect();
    let files = codestreams(&specs);
    let events = BASIC_INFO | COLOR_ENCODING | FULL_IMAGE;
    let requests = files
        .iter()
        .map(|(file, _)| {
            run(
                file,
                &format!(
                    "s{events} I0,{} p b p E0 E1 p F3.3.2.0.0 U p X p",
                    file_len(file)
                ),
            )
        })
        .collect();
    compare_expecting("dt-preserve-profile", requests, |o| {
        statuses(o) == ["64", "256", "5", "4096", "0"]
    });
}

/// SetPreferredColorProfileTest's expectations: an ICC destination without a
/// CMS is rejected; every other destination is accepted and decodes.
fn sets_preferred_profile(observation: &str) -> bool {
    let statuses = statuses(observation);
    let rejected = observation.contains(" Q:1") && !observation.contains(" C:");
    if rejected {
        statuses == ["64", "256"]
    } else {
        statuses == ["64", "256", "5", "4096", "0"]
            && !observation.contains(" Q:1")
            && !observation.contains(" P:1")
    }
}

/// SetPreferredColorProfileTest for every source and destination variant.
fn preferred_profile_requests(sources: &[&Encoding], all: &[Encoding]) -> Vec<String> {
    let specs: Vec<String> = sources
        .iter()
        .map(|c| {
            format!(
                "123 77 {} 0 cs={} intensity={}",
                c.channels,
                c.description,
                if c.pq { 10000 } else { 255 }
            )
        })
        .collect();
    let files = codestreams(&specs);
    let events = BASIC_INFO | COLOR_ENCODING | FULL_IMAGE;
    let mut requests = Vec::new();
    for (c_in, (file, _)) in sources.iter().zip(&files) {
        let intensity = if c_in.pq { 10000 } else { 255 };
        for (icc_dst, use_cms) in [(true, true), (false, true), (true, false), (false, false)] {
            for c_out in &all[..all.len() - 1] {
                if c_out.color_space != XYB {
                    if c_out.intent != RELATIVE {
                        continue;
                    }
                    if (c_in.primaries == PRIMARIES_2100 && c_out.primaries != PRIMARIES_2100)
                        || (c_in.primaries == PRIMARIES_P3 && c_out.primaries == PRIMARIES_SRGB)
                    {
                        continue;
                    }
                }
                if c_out.hlg && intensity > 300 {
                    continue;
                }
                if c_in.description == c_out.description {
                    continue;
                }
                let mut ops = format!("s{events} I0,{} p b p E0", file_len(file));
                if use_cms {
                    ops.push_str(" C");
                }
                if icc_dst {
                    ops.push_str(&format!(" Q{}", c_out.parsed_icc));
                    if !use_cms {
                        requests.push(run(file, &ops));
                        continue;
                    }
                } else {
                    ops.push_str(&format!(" P{}", c_out.parsed));
                }
                ops.push_str(" E0");
                if !icc_dst {
                    ops.push_str(" E1");
                }
                ops.push_str(&format!(" p F{}.3.2.0.0 U p X p", c_out.channels));
                requests.push(run(file, &ops));
            }
        }
    }
    requests
}

#[test]
fn decode_test_preferred_profiles_from_gray() {
    // SetPreferredColorProfileTestFromGray.
    let all = encodings();
    let gray = &all[all.len() - 1];
    compare_expecting(
        "dt-preferred-gray",
        preferred_profile_requests(&[gray], &all),
        sets_preferred_profile,
    );
}

#[test]
fn decode_test_preferred_profiles_srgb_sources() {
    // DecodeAllEncodingsVariantsTest/SetPreferredColorProfileTest: sources
    // other than relative D65 return before any API call.
    let all = encodings();
    let sources: Vec<&Encoding> = all[..all.len() - 1]
        .iter()
        .filter(|c| c.intent == RELATIVE && c.white_point == D65 && c.primaries == PRIMARIES_SRGB)
        .collect();
    compare_expecting(
        "dt-preferred-srgb",
        preferred_profile_requests(&sources, &all),
        sets_preferred_profile,
    );
}

#[test]
fn decode_test_preferred_profiles_wide_gamut_sources() {
    let all = encodings();
    let sources: Vec<&Encoding> = all[..all.len() - 1]
        .iter()
        .filter(|c| c.intent == RELATIVE && c.white_point == D65 && c.primaries != PRIMARIES_SRGB)
        .collect();
    compare_expecting(
        "dt-preferred-wide",
        preferred_profile_requests(&sources, &all),
        sets_preferred_profile,
    );
}

#[test]
fn decode_test_decode_with_cms() {
    // DecodeAllEncodingsWithCMSTest/DecodeWithCMS.
    let all = encodings();
    let file = codestream("177 123 3 0");
    let len = file_len(&file);
    let events = BASIC_INFO | COLOR_ENCODING | FULL_IMAGE;
    let mut requests = Vec::new();
    for c in &all[..all.len() - 1] {
        let tail = format!("p F{}.3.2.0.0 U p X", c.channels);
        requests.push(run(
            &file,
            &format!("s{events} I0,{len} p b p C Q{} {tail}", c.icc),
        ));
        requests.push(run(
            &file,
            &format!("s{events} I0,{len} p b p R{},~ {tail}", c.external),
        ));
    }
    compare_expecting("dt-cms", requests, |o| {
        statuses(o) == ["64", "256", "5", "4096"]
    });
}

#[test]
fn decode_test_container_input_handling() {
    let specs: Vec<String> = (0..CSBF_NUM_ENTRIES)
        .map(|i| format!("123 77 3 0 box={i}"))
        .collect();
    let files: Vec<String> = codestreams(&specs).into_iter().map(|(f, _)| f).collect();
    let mut requests = Vec::new();
    // ProcessEmptyInputWithBoxes.
    let events = BASIC_INFO | FULL_IMAGE | COLOR_ENCODING;
    for file in &files {
        requests.push(run(
            file,
            &format!("s{events} p I0,{} p p F3.0.1.0.0 q b i p", file_len(file)),
        ));
    }
    // ExtraBytesAfterCompressedStream and ...RequireBoxes.
    for (i, file) in files.iter().enumerate() {
        if i == CSBF_MULTI_OTHER_ZERO_TERMINATED {
            continue;
        }
        let file = format!("{file}000102");
        for boxes in [false, true] {
            let mut api = Api::new(3, (1, 0));
            api.early = true;
            api.boxes = boxes;
            requests.push(run(&file, &format!("{} i", api.ops(file_len(&file)))));
        }
    }
    compare_with_runners("dt-extra-bytes", requests, never_fails);
}

#[test]
fn decode_test_concatenated_streams() {
    // ConcatenatedCompressedStreams: the second decoder starts where the
    // first one left the input.
    let specs: Vec<String> = (0..CSBF_NUM_ENTRIES)
        .map(|i| format!("123 77 3 0 box={i}"))
        .collect();
    let files: Vec<String> = codestreams(&specs).into_iter().map(|(f, _)| f).collect();
    let mut api = Api::new(3, (1, 0));
    api.early = true;
    api.boxes = true;
    let mut first = Vec::new();
    let mut concatenated = Vec::new();
    for (i, a) in files.iter().enumerate() {
        for (j, b) in files.iter().enumerate() {
            if i == CSBF_MULTI_OTHER_ZERO_TERMINATED || j == CSBF_MULTI_OTHER_ZERO_TERMINATED {
                continue;
            }
            let concat = format!("{a}{b}");
            first.push(run(&concat, &format!("{} i", api.ops(file_len(&concat)))));
            concatenated.push(concat);
        }
    }
    // The Rust history gives the remainder; it compares exactly below.
    let observed: Vec<String> = first
        .iter()
        .map(|r| super::observe(&without_runner(r)))
        .collect();
    let mut second = Vec::new();
    for (concat, observation) in concatenated.iter().zip(&observed) {
        let remaining: usize = observation
            .rsplit_once(" i:")
            .unwrap()
            .1
            .split(' ')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let bytes = unhex(concat);
        let rest = hex(&bytes[bytes.len() - remaining..]);
        second.push(run(&rest, &format!("{} i", api.ops(remaining))));
    }
    first.extend(second);
    compare_with_runners("dt-concatenated", first, |o| {
        decodes_with_api(o) && never_fails(o)
    });
}

/// TestPartialStream: the increment and the JPEG buffer persist across the
/// loops of the original test, so each history follows from the previous
/// original observation.
fn partial_stream(reconstructible_jpeg: bool) -> Vec<String> {
    let channels = if reconstructible_jpeg { 3 } else { 4 };
    let jpeg = jpeg_input();
    let specs: Vec<String> = (0..CSBF_NUM_ENTRIES)
        .map(|i| {
            if reconstructible_jpeg {
                format!("123 77 {channels} 0 ct=1 box={i} jpeg={jpeg}")
            } else {
                format!("123 77 {channels} 0 lossless=1 box={i}")
            }
        })
        .collect();
    let files = codestreams(&specs);
    let events = BASIC_INFO | FULL_IMAGE | JPEG_RECONSTRUCTION;
    let mut jpeg_size = 64usize;
    let mut requests = Vec::new();
    for increment in [1usize, 3, 17, 23, 120, 700, 1050] {
        let mut increment = increment;
        for (i, (file, _)) in files.iter().enumerate() {
            if reconstructible_jpeg && i == CSBF_NONE {
                continue;
            }
            let request = run(
                file,
                &format!("s{events} F{channels}.3.2.0.0 A{increment},1,-,{jpeg_size},100000000"),
            );
            // The Rust history gives the feedback; it compares exactly below.
            let observation = super::observe(&request);
            // The test reduces its increment when the last chunk is shorter,
            // and its JPEG buffer keeps its doubled size.
            let size = file_len(file);
            let needed = observation
                .split(' ')
                .filter(|t| t.starts_with('A') && t.contains(":2,"))
                .count();
            let full = size / increment;
            if needed > full && !size.is_multiple_of(increment) {
                increment = size % increment;
            }
            for token in observation.split(" a+:").skip(1) {
                jpeg_size = token
                    .split(' ')
                    .next()
                    .unwrap()
                    .rsplit(',')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
            }
            requests.push(request);
        }
    }
    requests
}

#[test]
fn decode_test_pixel_partial_stream() {
    // PixelPartialTest.
    compare_expecting("dt-partial-pixels", partial_stream(false), |o| {
        let s = loop_statuses(o);
        s.last() == Some(&"0") && s.contains(&"4096")
    });
}

#[test]
fn decode_test_jpeg_partial_stream() {
    // JPEGPartialTest.
    compare_expecting("dt-partial-jpeg", partial_stream(true), |o| {
        let s = loop_statuses(o);
        s.last() == Some(&"0") && s.contains(&"4096") && o.contains(" a-:")
    });
}

#[test]
fn decode_test_preview_dc_and_animation() {
    let mut requests = Vec::new();
    // DCNotGettableTest.
    // The 68-byte 1x1 codestream literal of the test.
    let dc = "ff0a0010b0130048802800dc00550f0000a8501965dce0e55ccf971f3a2ca66d5c6768ab6d0b4b1245c6b149aa4326704812eb20f30612087000809f1c99573264ad2401";
    assert_eq!(file_len(dc), 68);
    requests.push(run(dc, &format!("s{BASIC_INFO} I0,68 p p")));
    // PreviewTest.
    for mode in [1, 2] {
        let file = codestream(&format!("77 120 3 0 preview={mode}"));
        requests.push(run(
            &file,
            &format!(
                "s{} I0,{} p b F3.2.1.0.0 q1 p H p X",
                BASIC_INFO | PREVIEW_IMAGE,
                file_len(&file)
            ),
        ));
    }
    compare("dt-preview", requests);
    // AnimationTest and AnimationTestStreaming with the thread runner.
    let file = frames("animation");
    let events = BASIC_INFO | FRAME | FULL_IMAGE;
    let frame = " p N p U p X";
    compare_with_runners(
        "dt-animation",
        vec![
            run(
                &file,
                &format!(
                    "@3 s{events} I0,{} p F3.3.2.0.0 q b{frame}{frame} p",
                    file_len(&file)
                ),
            ),
            run(
                &file,
                &format!("@3 s{events} F3.3.2.0.0 A16,1,-,0,{}", file_len(&file) + 1),
            ),
        ],
        never_fails,
    );
}

#[test]
fn decode_test_extra_and_spot_channels() {
    let mut requests = Vec::new();
    // ExtraChannelTest.
    let file = codestream("55 257 4 0 lossless=1 tier=8");
    requests.push(run(
        &file,
        &format!(
            "s{} I0,{} p b e0 p F3.2.1.17.0 q q2,0 V1 U p X p",
            BASIC_INFO | FULL_IMAGE,
            file_len(&file)
        ),
    ));
    // SpotColorTest.
    let file = frames("spot");
    for render in [true, false] {
        requests.push(run(
            &file,
            &format!(
                "s{}{} I0,{} p b e0 p F3.2.1.0.0 q q2,0 V1 U p X p",
                BASIC_INFO | FULL_IMAGE,
                if render { "" } else { " r0" },
                file_len(&file)
            ),
        ));
    }
    compare_expecting("dt-extra-channels", requests, |o| {
        statuses(o) == ["64", "5", "4096", "0"]
    });
}

#[test]
fn decode_test_skip_current_frame() {
    let mut requests = Vec::new();
    // SkipCurrentFrameTest.
    let file = frames("skip-current");
    let mut ops = format!(
        "s{} D2 I0,{} p F3.3.2.0.0 q b",
        BASIC_INFO | FRAME | FRAME_PROGRESSION | FULL_IMAGE,
        file_len(&file)
    );
    for i in 0..7 {
        ops.push_str(" K p K N p U");
        if i == 2 {
            ops.push_str(" K");
            continue;
        }
        let mut skipped = false;
        for stop in 3..=5 {
            ops.push_str(" p m");
            if i == stop {
                ops.push_str(" K");
                skipped = true;
                break;
            }
        }
        if !skipped {
            ops.push_str(" p X K");
        }
    }
    ops.push_str(" p");
    requests.push(run(&file, &ops));
    // SkipCurrentFrameForgetsExtraChannelBuffers.
    let file = frames("skip-current-ec");
    requests.push(run(
        &file,
        &format!(
            "s{} I0,{} c p b F3.3.2.0.0 q q2,0 p p V1 U K p p V0 U p p V1 X",
            BASIC_INFO | FRAME | FULL_IMAGE,
            file_len(&file)
        ),
    ));
    compare_expecting("dt-skip-current", requests, never_fails);
}

/// Frame loop of the skip tests: frame i takes "p [q] N p U p X".
fn frame_ops(query: bool) -> &'static str {
    if query {
        " p q N p U p X"
    } else {
        " p N p U p X"
    }
}

/// A rewound pass that skips five frames at the fourth, as in SkipFrameTest
/// and its blending variants.
fn skip_five_pass(frames: usize, query: bool, len: usize) -> String {
    let mut ops = format!(" w s{} I0,{len}", FRAME | FULL_IMAGE);
    let mut i = 0;
    while i < frames {
        if i == 3 {
            ops.push_str(" S5");
            i += 5;
        }
        ops.push_str(frame_ops(query));
        i += 1;
    }
    ops
}

/// A rewound pass that skips three frames after the tenth frame header.
fn skip_three_pass(frames: usize, query: bool, len: usize) -> String {
    let mut ops = format!(" w s{} I0,{len}", FRAME | FULL_IMAGE);
    let mut i = 0;
    while i < frames {
        ops.push_str(if query { " p q" } else { " p" });
        if i == 9 {
            ops.push_str(" S3");
        }
        ops.push_str(" N p U p X");
        if i == 9 {
            i += 3;
        }
        i += 1;
    }
    ops
}

#[test]
fn decode_test_skip_frames() {
    let events = BASIC_INFO | FRAME | FULL_IMAGE;
    let mut requests = Vec::new();
    // SkipFrameTest.
    let file = frames("skip");
    let len = file_len(&file);
    let mut ops = format!("@3 s{events} I0,{len} p F3.3.2.0.0 q b");
    let mut i = 0;
    while i < 16 {
        if i == 3 {
            ops.push_str(" S5");
            i += 5;
        }
        ops.push_str(frame_ops(false));
        i += 1;
    }
    ops.push_str(" p");
    ops.push_str(&skip_three_pass(16, false, len));
    requests.push(run(&file, &ops));
    // SkipFrameWithBlendingTest.
    let file = frames("skip-blend");
    let len = file_len(&file);
    let mut reference = format!("@3 s{FULL_IMAGE} I0,{len} F3.3.2.0.0");
    for _ in 0..16 {
        reference.push_str(" p U p X");
    }
    reference.push_str(" p");
    requests.push(run(&file, &reference));
    let mut ops = format!("@3 s{events} I0,{len} p F3.3.2.0.0 q b");
    for _ in 0..=8 {
        ops.push_str(frame_ops(false));
    }
    ops.push_str(&skip_five_pass(16, false, len));
    ops.push_str(" p");
    ops.push_str(&skip_three_pass(16, false, len));
    requests.push(run(&file, &ops));
    // SkipFrameWithAlphaBlendingTest, coalesced and as layers.
    let file = frames("skip-alpha-blend");
    let len = file_len(&file);
    for coalescing in [1, 0] {
        let frames = 16 + if coalescing == 1 { 0 } else { 5 };
        let mut reference = format!("g{coalescing} @3 s{FULL_IMAGE} I0,{len} F4.3.2.0.0");
        for _ in 0..frames {
            reference.push_str(" p q U p X");
        }
        reference.push_str(" p");
        requests.push(run(&file, &reference));
        let mut ops = format!("g{coalescing} @3 s{events} I0,{len} p F4.3.2.0.0 b");
        for _ in 0..=8 {
            ops.push_str(frame_ops(true));
        }
        ops.push_str(&skip_five_pass(frames, true, len));
        ops.push_str(" p");
        ops.push_str(&skip_three_pass(frames, true, len));
        requests.push(run(&file, &ops));
    }
    compare_with_runners("dt-skip-frames", requests, never_fails);
}

#[test]
fn decode_test_oriented_cropped_frames() {
    // OrientedCroppedFrameTest.
    let icc = icc();
    let mut inputs = Vec::new();
    for orientation in 1..=8 {
        for resampling in [1, 2, 4, 8] {
            inputs.push(format!(
                "dti-frames {icc} oriented-crop {orientation} {resampling}"
            ));
        }
    }
    let files = oracle(&inputs);
    let mut requests = Vec::new();
    for keep in [1, 0] {
        for file in &files {
            for coalescing in [1, 0] {
                let mut ops = format!(
                    "g{coalescing} k{keep} @3 s{FULL_IMAGE} I0,{} F4.3.2.0.0",
                    file_len(file)
                );
                for _ in 0..if coalescing == 1 { 1 } else { 3 } {
                    ops.push_str(" p N q U p X");
                }
                ops.push_str(" p");
                requests.push(run(file, &ops));
            }
        }
    }
    compare_with_runners("dt-oriented-crop", requests, never_fails);
}

#[test]
fn decode_test_progression() {
    let mut requests = Vec::new();
    let events = BASIC_INFO | FRAME | FULL_IMAGE;
    // ProgressionTest.
    let file = codestream("508 470 3 0 pdc=1 preview=1");
    let s = analyze(&file);
    let fp = &s.frames;
    let n = file_len(&file);
    let breakpoints = [
        (fp[0].frame_start, 0),
        (fp[1].frame_start, 0),
        (fp[2].frame_start, 0),
        (fp[2].section_end[0], 0),
        (fp[2].section_end[1] - 1, 0),
        (fp[2].section_end[1], 2),
        (fp[2].section_end[2], 1),
        (fp[2].section_end[3], 2),
        (fp[2].section_end[4] - 1, 1),
        (fp[2].section_end[4], 2),
        (fp[2].section_end[5], 2),
        (n - 1, 1),
        (n, 2),
    ];
    let list: Vec<String> = breakpoints
        .iter()
        .map(|(p, f)| format!("{p}:{f}"))
        .collect();
    requests.push(run(
        &file,
        &format!("s{events} F3.3.2.0.0 B{}", list.join("/")),
    ));
    // ProgressionTestLosslessAlpha.
    let file = codestream("508 470 4 0 lossless=1 tier=8 responsive=1");
    let s = analyze(&file);
    let fp = &s.frames;
    let n = file_len(&file);
    let breakpoints = [
        (fp[0].frame_start, 0),
        (fp[0].section_end[0] - 1, 0),
        (fp[0].section_end[0], 2),
        (fp[0].section_end[1], 1),
        (fp[0].section_end[2], 1),
        (fp[0].section_end[3], 2),
        (fp[0].section_end[4] - 1, 1),
        (fp[0].section_end[4], 2),
        (fp[0].section_end[5], 2),
        (n - 1, 1),
        (n, 2),
    ];
    let list: Vec<String> = breakpoints
        .iter()
        .map(|(p, f)| format!("{p}:{f}"))
        .collect();
    requests.push(run(
        &file,
        &format!("s{events} F4.3.2.0.0 B{}", list.join("/")),
    ));
    // Every breakpoint is reached and the decode ends with SUCCESS.
    compare_expecting("dt-progression", requests, |o| {
        o.ends_with(" B0 E") && !o.contains(" B1 ")
    });
}

#[test]
fn decode_test_input_handling_one_shot() {
    // InputHandlingTestOneShot.
    let specs: Vec<String> = (0..CSBF_NUM_ENTRIES)
        .map(|i| format!("508 470 3 0 pdc=1 preview=1 box={i}"))
        .collect();
    let files = codestreams(&specs);
    let events = [
        BASIC_INFO,
        COLOR_ENCODING,
        PREVIEW_IMAGE,
        FRAME,
        FULL_IMAGE,
        FRAME_PROGRESSION,
    ];
    let mut requests = Vec::new();
    for (file, _) in &files {
        let mut wanted = 0;
        for (j, event) in events.iter().enumerate() {
            wanted |= event;
            let mut ops = format!("s{wanted} I0,{} F3.3.2.0.0 p l", file_len(file));
            if j >= 1 {
                ops.push_str(" p l");
            }
            if j >= 2 {
                ops.push_str(" p l H p l");
            }
            if j >= 3 {
                ops.push_str(" p l");
                if j >= 5 {
                    ops.push_str(" D1");
                }
            }
            if j >= 4 {
                ops.push_str(" p l U");
                if j >= 5 {
                    ops.push_str(" p l");
                }
                ops.push_str(" p l X");
            }
            ops.push_str(" p l");
            requests.push(run(file, &ops));
        }
    }
    // InputHandlingTestJPEGOneshot.
    let jpeg = jpeg_input();
    let specs: Vec<String> = (1..CSBF_NUM_ENTRIES)
        .map(|i| format!("123 77 3 0 ct=1 jpeg={jpeg} preview=1 box={i}"))
        .collect();
    let files = codestreams(&specs);
    let events = [
        BASIC_INFO,
        JPEG_RECONSTRUCTION,
        COLOR_ENCODING,
        PREVIEW_IMAGE,
        FRAME,
        FULL_IMAGE,
    ];
    for (file, _) in &files {
        assert!(analyze(file).jbrd_end > 0);
        let mut wanted = 0;
        for (j, event) in events.iter().enumerate() {
            wanted |= event;
            let mut ops = format!("s{wanted} I0,{} F3.3.2.0.0", file_len(file));
            if j >= 1 {
                ops.push_str(" p l");
            }
            ops.push_str(" p l");
            if j >= 2 {
                ops.push_str(" p l");
            }
            if j >= 3 {
                ops.push_str(" p l H p l");
            }
            if j >= 4 {
                ops.push_str(" p l");
            }
            if j >= 5 {
                ops.push_str(" p l U p l X");
            }
            ops.push_str(" p l");
            requests.push(run(file, &ops));
        }
    }
    compare_expecting("dt-input-one-shot", requests, |o| {
        let s = statuses(o);
        !s.contains(&"1") && !s.contains(&"2") && s.last() == Some(&"0")
    });
}

#[test]
fn decode_test_input_handling_streaming() {
    // InputHandlingTestStreaming.
    let specs: Vec<String> = (0..CSBF_NUM_ENTRIES)
        .map(|i| format!("508 470 3 0 pdc=1 box={i} preview=1"))
        .collect();
    let files = codestreams(&specs);
    let events =
        BASIC_INFO | COLOR_ENCODING | PREVIEW_IMAGE | FRAME | FULL_IMAGE | FRAME_PROGRESSION | BOX;
    let mut requests = Vec::new();
    for (file, _) in &files {
        for increment in [1, 7, 27, 1024] {
            requests.push(run(
                file,
                &format!("s{events} F3.3.2.0.0 A{increment},68,-,0,100000000"),
            ));
        }
    }
    compare_expecting("dt-input-streaming", requests, |o| {
        let s = loop_statuses(o);
        never_fails(o) && s.contains(&"4096") && matches!(s.last(), Some(&"0") | Some(&"2"))
    });
}

#[test]
fn decode_test_flush() {
    let events = BASIC_INFO | FRAME | FULL_IMAGE;
    let mut requests = Vec::new();
    // FlushTest, FlushTestImageOutCallback, FlushTestLossyProgressiveAlpha,
    // FlushTestLossyProgressiveAlphaUpsampling and
    // FlushTestLosslessProgressiveAlpha.
    for (spec, channels, mode, part) in [
        ("333 300 3 0 preview=1", 3, 0, 0),
        ("333 300 3 0 preview=1", 3, 4, 0),
        // The same callback history, with every call compared in order.
        ("333 300 3 0 preview=1", 3, 1, 0),
        ("333 300 4 0 preview=1", 4, 0, 0),
        (
            "533 401 4 0 resampling=2 ec_resampling=4 preview=1",
            4,
            0,
            1,
        ),
        (
            "333 300 4 0 lossless=1 tier=8 responsive=1 gshift=1 preview=1",
            4,
            0,
            2,
        ),
    ] {
        let file = codestream(spec);
        let n = file_len(&file);
        let first = match part {
            0 => n - 1,
            1 => n * 2 / 3,
            _ => n / 2,
        };
        requests.push(run(
            &file,
            &format!("s{events} I0,{first} p b p f F{channels}.3.2.0.{mode} U p f p l p X"),
        ));
    }
    compare_expecting("dt-flush", requests, |o| {
        statuses(o) == ["64", "1024", "2", "2", "4096"]
    });
}

#[test]
fn decode_test_progressive_events() {
    // DecodeProgressiveTest/ProgressiveEventTest.
    let mut specs = Vec::new();
    let mut params = Vec::new();
    for param in 0..8 {
        let single_group = param & 1 != 0;
        let lossless = (param >> 1) & 1 != 0;
        let channels = 3 + ((param >> 2) & 1);
        let (xsize, ysize) = if single_group { (99, 100) } else { (277, 280) };
        let quality = if lossless {
            "lossless=1"
        } else {
            "distance=0.5"
        };
        specs.push(format!(
            "{xsize} {ysize} {channels} 0 {quality} passes=event"
        ));
        params.push((lossless, channels));
    }
    let files = codestreams(&specs);
    let events = BASIC_INFO | FRAME | FULL_IMAGE | FRAME_PROGRESSION;
    let mut requests = Vec::new();
    for ((file, _), (lossless, channels)) in files.iter().zip(params) {
        let expect_flush = channels == 3 && !lossless;
        for detail in [1, 2, 3] {
            let next_pass = |pass: usize| match detail {
                1 => 5,
                2 => (pass + 2).min(5),
                _ => pass + 1,
            };
            for increment in [1, file_len(file)] {
                let step = format!("A{increment},129,-,0,100000000");
                let mut ops = format!(
                    "s{events} D0 D4 D5 D6 D{detail} F{channels}.3.2.0.0 {step} b {step} q U"
                );
                if expect_flush {
                    let mut pass = 0;
                    while pass < 5 {
                        ops.push_str(&format!(" {step} m f"));
                        pass = next_pass(pass);
                    }
                }
                ops.push_str(&format!(" {step} X {step}"));
                requests.push(run(file, &ops));
            }
        }
    }
    let expected: Vec<usize> = requests.iter().map(|r| r.matches(" m f").count()).collect();
    let checked = std::cell::Cell::new(0);
    compare_expecting("dt-progressive-events", requests, |o| {
        let s: Vec<&str> = loop_statuses(o).into_iter().filter(|s| *s != "2").collect();
        let progressions = s.iter().filter(|s| **s == "32768").count();
        let index = checked.get();
        checked.set(index + 1);
        progressions == expected[index]
            && s.ends_with(&["4096", "0"])
            && o.split(' ')
                .filter(|t| t.starts_with("f:"))
                .all(|t| t == "f:0")
    });
}

/// VerifyJPEGReconstruction.
fn verify_jpeg(file: &str) -> String {
    run(
        file,
        &format!(
            "s{} I0,{} p a128 a+ A0,8,-,0,100000",
            JPEG_RECONSTRUCTION | FULL_IMAGE,
            file_len(file)
        ),
    )
}

#[test]
fn decode_test_jpeg_reconstruction() {
    let jpeg = jpeg_input();
    let events = JPEG_RECONSTRUCTION | FULL_IMAGE;
    let mut requests = Vec::new();
    // JPEGReconstructTestCodestream.
    let (file, jpeg_bytes) = codestreams(&[format!(
        "123 77 3 0 ct=1 box={CSBF_SINGLE} jpeg={jpeg} preview=1"
    )])
    .remove(0);
    assert_eq!(jpeg_bytes, jpeg);
    requests.push(verify_jpeg(&file));
    // JPEGReconstructionBufferNotKeptAcrossReset.
    let (file, jpeg_bytes) =
        codestreams(&[format!("123 77 3 0 ct=1 box={CSBF_SINGLE} jpeg={jpeg}")]).remove(0);
    let len = file_len(&file);
    let jpeg_len = file_len(&jpeg_bytes);
    let pass = format!("s{events} I0,{len} c p J{jpeg_len} p x");
    requests.push(run(&file, &format!("{pass} {pass} ~")));
    // JPEGReconstructionRewindInsideBox.
    let bytes = unhex(&file);
    let jbrd = bytes.windows(4).position(|w| w == b"jbrd").unwrap();
    requests.push(run(
        &file,
        &format!(
            "s{events} I0,{} p i w I0,{len} c p J{jpeg_len} p j",
            jbrd + 4 + 8
        ),
    ));
    // JPEGReconstructionTest.
    let file = oracle(&[format!(
        "dti-frames {} jpeg-recon {}",
        icc(),
        hex(&test_data("decode-test/flower.png.im_q85_420.jpg"))
    )])
    .remove(0);
    requests.push(verify_jpeg(&file));
    // JPEGReconstructionMetadataTest.
    requests.push(verify_jpeg(&hex(&test_data(
        "decode-test/1x1_exif_xmp.jxl",
    ))));
    // Both reconstruct the pinned JPEG files exactly.
    for (request, original) in [
        (&requests[3], "decode-test/flower.png.im_q85_420.jpg"),
        (&requests[4], "decode-test/1x1_exif_xmp.jpg"),
    ] {
        let jpeg = format!(",{} ", hex(&test_data(original)));
        assert!(super::observe(request).contains(&jpeg));
    }
    compare("dt-jpeg", requests);
}

#[test]
fn decode_test_boxes() {
    let mut requests = Vec::new();
    // ContinueFinalNonEssentialBoxTest.
    let file = codestream(&format!(
        "80 90 4 0 box={CSBF_MULTI_OTHER_TERMINATED} icc=1"
    ));
    let len = file_len(&file);
    let last = len - 20;
    requests.push(run(
        &file,
        &format!("s{} I0,{last} p p p i I{last},20 p", BASIC_INFO | FRAME),
    ));
    // ExtendedBoxSizeTest.
    let file = hex(&test_data("decode-test/square-extended-size-container.jxl"));
    requests.push(run(
        &file,
        &format!("s{BOX} I0,{} p t0 y Y p t0 y Y p t0 y Y", file_len(&file)),
    ));
    // BoxTest.
    let file = codestream(&format!("1 1 4 0 box={CSBF_MULTI_OTHER_TERMINATED} icc=1"));
    let types = [
        "JXL ", "ftyp", "jxlp", "unk1", "unk2", "jxlp", "jxlp", "jxlp", "unk3",
    ];
    let sizes = [12, 20, 0, 34, 18, 0, 0, 0, 20];
    let mut ops = format!("s{BOX} t0 y");
    let mut release = false;
    for (box_type, size) in types.iter().zip(sizes) {
        ops.push_str(" I+ p t0 y");
        if size != 0 {
            ops.push_str(" Y");
        }
        if release {
            ops.push_str(" o");
            release = false;
        }
        if box_type.starts_with("unk") {
            ops.push_str(" O50");
            release = true;
        }
        ops.push_str(" i");
    }
    ops.push_str(" I+ p c p");
    requests.push(run(&file, &ops));
    // ExifBrobBoxTest: raw and decompressed, all at once and streaming.
    let file = codestream(&format!("1 1 4 0 lossless=1 box={CSBF_BROB_EXIF} icc=1"));
    let len = file_len(&file);
    requests.push(run(&file, &format!("s{BOX} I0,{len} c A0,0,brob,0,100000")));
    requests.push(run(&file, &format!("s{BOX} A64,2,brob,0,100000")));
    requests.push(run(
        &file,
        &format!("s{BOX} I0,{len} c d1 A0,16,Exif,0,100000"),
    ));
    requests.push(run(&file, &format!("s{BOX} d1 A64,18,Exif,0,100000")));
    // PartialCodestreamBoxTest, then the codestream assembled from its jxlp
    // box contents.
    let file = codestream(&format!(
        "23 81 4 0 lossless=1 tier=8 box={CSBF_MULTI} icc=1"
    ));
    let events = BASIC_INFO | FULL_IMAGE | BOX;
    requests.push(run(
        &file,
        &format!(
            "s{events} I0,{} c F4.3.2.0.0 A0,0,jxlp,0,100000",
            file_len(&file)
        ),
    ));
    let bytes = unhex(&file);
    let mut extracted = Vec::new();
    let mut pos = 0;
    while pos < bytes.len() {
        let mut size = u32::from_be_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
        if size == 0 {
            size = bytes.len() - pos;
        }
        if &bytes[pos + 4..pos + 8] == b"jxlp" {
            extracted.extend_from_slice(&bytes[pos + 12..pos + size]);
        }
        pos += size;
    }
    let extracted = hex(&extracted);
    requests.push(run(
        &extracted,
        &format!(
            "s{events} I0,{} c F4.3.2.0.0 A0,0,-,0,100000",
            file_len(&extracted)
        ),
    ));
    // OutOfOrderJxlpDuplicateIndexTest.
    let mut c = Vec::new();
    c.extend_from_slice(&12u32.to_be_bytes());
    c.extend_from_slice(b"JXL \r\n\x87\n");
    c.extend_from_slice(&20u32.to_be_bytes());
    c.extend_from_slice(b"ftypjxl ");
    c.extend_from_slice(&1u32.to_be_bytes());
    c.extend_from_slice(b"jxl ");
    for _ in 0..2 {
        c.extend_from_slice(&16u32.to_be_bytes());
        c.extend_from_slice(b"jxlp");
        c.extend_from_slice(&1u32.to_be_bytes());
        c.extend_from_slice(&[0xaa, 0xbb, 0xcc, 0xdd]);
    }
    requests.push(run(&hex(&c), &format!("s{BASIC_INFO} I0,{} p", c.len())));
    compare("dt-boxes", requests);
}

#[test]
fn decode_test_close_input_and_malformed() {
    let events = BASIC_INFO | FULL_IMAGE;
    // CloseInput.
    let mut requests = vec![run("ff", &format!("s{events} I0,1 p p c p"))];
    // Non444AdaptiveDCSmoothing.
    let data = [
        0xff, 0x0a, 0x20, 0x71, 0x00, 0xff, 0x0a, 0x00, 0x34, 0x58, 0xc6, 0x20, 0x10, 0x00, 0xb8,
        0x00, 0x2d, 0x20, 0xfa, 0xff, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0xff, 0xff, 0xff, 0xff,
        0xff, 0x32, 0x35, 0x31, 0x31, 0xb2, 0x20, 0x20, 0xff, 0xff, 0x20, 0x20, 0xbf, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0xd3, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20,
    ];
    requests.push(run(
        &hex(&data),
        &format!("s{events} I0,{} c p p F3.2.1.0.0 q U p", data.len()),
    ));
    compare("dt-close-input", requests);
}
