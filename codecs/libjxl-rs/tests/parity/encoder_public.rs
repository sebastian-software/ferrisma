// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Public encoder histories against the original public C API of the complete
// archive: object lifecycle, option and metadata setters, frame and box
// queueing, ProcessOutput and the buffer-mode output wrapper. A history whose
// Rust run returns an admission result compares exactly up to that call, and
// the original's own state must show that it enters the excluded branch there
// (ADR 0001); such histories are reported separately and are not parity.
#[path = "../support/encoder_public.rs"]
mod driver;

use driver::JxlEncoderAdmission;

pub(super) fn handles(kind: &str) -> bool {
    kind == "epub-wrap"
}

pub(super) fn observe(_kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    driver::observe_wrapper(args)
}

/// Retains a family's requests for review when JXL_ENCODER_PUBLIC_CAPTURE
/// names a directory.
fn capture(family: &str, requests: &[String]) {
    if let Some(path) = std::env::var_os("JXL_ENCODER_PUBLIC_CAPTURE") {
        let path = std::path::PathBuf::from(path);
        std::fs::create_dir_all(&path).unwrap();
        std::fs::write(
            path.join(format!("{family}.requests")),
            requests.join("\n") + "\n",
        )
        .unwrap();
    }
}

/// Compares one family of epub-run histories with the original.
fn compare(family: &str, requests: Vec<String>) {
    compare_counted(family, requests);
}

/// [`compare`], returning the number of exactly matched histories.
fn compare_counted(family: &str, requests: Vec<String>) -> usize {
    capture(family, &requests);
    let originals = super::original_observations(&requests);
    let probes = super::original_observations(
        &requests
            .iter()
            .map(|request| request.replacen("epub-run", "epub-probe", 1))
            .collect::<Vec<_>>(),
    );
    let (mut exact, mut fast_lossless, mut enqueued) = (0, 0, 0);
    for ((request, cpp), probe) in requests.iter().zip(&originals).zip(&probes) {
        let mut args = request.split_whitespace();
        assert_eq!(args.next(), Some("epub-run"));
        let rust = driver::observe(&mut args);
        let ops: Vec<&str> = request.split_whitespace().skip(3).collect();
        let probe: Vec<&str> = probe.split(' ').collect();
        // Every compared AddImageFrame of the original queued a regular frame
        // or failed, as the Rust encoder did: no admission is missing.
        let compared = rust.admission.map_or(ops.len(), |(index, _)| index);
        for (index, op) in ops.iter().enumerate().take(compared) {
            assert!(
                !(op.starts_with('a') && probe.get(index + 1).is_some_and(|p| p.ends_with(",L"))),
                "the original took its fast-lossless branch at {op}: {request}"
            );
        }
        let Some((index, checkpoint)) = rust.admission else {
            assert_eq!(&rust.text, cpp, "request: {request}");
            exact += 1;
            continue;
        };
        let r: Vec<&str> = rust.text.split(' ').collect();
        let c: Vec<&str> = cpp.split(' ').collect();
        assert_eq!(
            r[..=index],
            c[..=index],
            "prefix before the admission: {request}"
        );
        let op = ops[index];
        let kind = op.chars().next().unwrap();
        // The status is the result's first field; the error follows the slash.
        let (result, error) = r[index + 1].rsplit_once('/').unwrap();
        assert!(
            result.starts_with(&format!("{kind}:1")) && error == "128",
            "admission result: {} {request}",
            r[index + 1]
        );
        let evidence = probe[index + 1];
        if std::env::var_os("JXL_ENCODER_PUBLIC_DEBUG").is_some() {
            eprintln!("admission {checkpoint:?} at {op}: {request}");
        }
        match checkpoint {
            JxlEncoderAdmission::FastLosslessFrame => {
                assert!(
                    kind == 'a' && evidence.ends_with(",L"),
                    "the original must queue a fast-lossless frame at {op}: {evidence} {request}"
                );
                fast_lossless += 1;
            }
            JxlEncoderAdmission::EnqueuedInput
            | JxlEncoderAdmission::StreamingFrame
            | JxlEncoderAdmission::FrameProfile
            | JxlEncoderAdmission::PatchDictionary => {
                let (queue, frame) = evidence[2..]
                    .split_once(';')
                    .unwrap_or((&evidence[2..], ""));
                let fields: Vec<usize> = queue.split(',').map(|v| v.parse().unwrap()).collect();
                frame_branch_evidence(checkpoint, frame, request);
                assert!(
                    kind == 'p' && fields[0] > 0 && fields[1] > 0 && fields[2] == 0,
                    "the original must call ProcessOneEnqueuedInput at {op}: {evidence} {request}"
                );
                enqueued += 1;
            }
        }
    }
    println!(
        "{family}: {exact} C++/Rust histories matched exactly; admission histories matched up to \
         the admission call with original branch evidence (not parity): {fast_lossless} \
         fast-lossless, {enqueued} ProcessOneEnqueuedInput"
    );
    exact
}

/// The original's branch evidence for a frame-encoder admission result,
/// from the front queued frame's state that `epub-probe` reports after the
/// original's own setting resolution (ParamsPostInit). The conditions are the
/// source's: CanDoStreamingEncoding's checks reached by these histories.
fn frame_branch_evidence(checkpoint: JxlEncoderAdmission, frame: &str, request: &str) {
    if checkpoint == JxlEncoderAdmission::EnqueuedInput {
        return;
    }
    assert!(
        !frame.is_empty(),
        "the original has a queued frame: {request}"
    );
    let f: Vec<&str> = frame.split(',').collect();
    assert_eq!(f[0], "1", "the original resolves the settings: {request}");
    let distance = f32::from_bits(f[1].parse().unwrap());
    let speed_tier: i32 = f[2].parse().unwrap();
    let buffering: i32 = f[3].parse().unwrap();
    let _ = (distance, speed_tier);
    let modular = f[6] == "1";
    let (xsize, ysize): (usize, usize) = (f[7].parse().unwrap(), f[8].parse().unwrap());
    if checkpoint == JxlEncoderAdmission::StreamingFrame {
        let groups = xsize.div_ceil(256) * ysize.div_ceil(256);
        assert!(
            !modular && groups > 8 && buffering != 0,
            "the original encodes a streaming frame: {frame} {request}"
        );
    }
}

fn run(mm: u32, fail: usize, script: &str) -> String {
    format!("epub-run {mm} {fail} {script}")
}

fn bits(value: f32) -> u32 {
    value.to_bits()
}

const RGB8: &str = "Bw:8,h:8";

#[test]
fn public_encoder_lifecycle_and_allocation_failures_match_original() {
    let scripts = [
        "V e x e S",
        "s- s- s0 S x S s- S",
        "X O45786966,10,0 O786d6c20,0,1 S x S",
        "Bw:8,h:8 G0 s- a0,3,2,0,0,192 S x S Bw:4,h:4 G0 s- a0,3,2,0,0,48 S",
        "s- Bw:8,h:8 s0 o1,0,3 S x e S",
        "Bw:8,h:0 e x e S",
        "I0 I1 I2 I4 I15 I16 I31 T0 T1",
        "M S x S",
    ];
    let mut requests = Vec::new();
    for mm in 0..3 {
        for script in scripts {
            requests.push(run(mm, 0, script));
        }
    }
    // Each manager allocation of one history fails in turn: the encoder, two
    // frame settings, boxes, queued frames and settings after Reset.
    let failing = "s- s- X O45786966,4,0 Bw:8,h:8 G0 a0,3,2,0,0,192 O786d6c20,3,0 \
                   a0,3,2,0,0,192 S x S s- S";
    for fail in 1..=10 {
        requests.push(run(1, fail, failing));
        requests.push(run(2, fail, failing));
    }
    compare("lifecycle", requests);
}

#[test]
fn public_encoder_basic_info_validation_matches_original() {
    let nan = f32::NAN.to_bits();
    let infos = [
        String::new(),
        "w:1,h:1".into(),
        "w:0".into(),
        "h:0".into(),
        "w:262144,h:1".into(),
        "w:262145,h:1".into(),
        "w:16384,h:16385".into(),
        "w:1073741824,h:1".into(),
        "w:1073741825,h:1".into(),
        "w:1048576,h:1048577".into(),
        "w:4294967295,h:1".into(),
        "bits:0".into(),
        "bits:1".into(),
        "bits:12".into(),
        "bits:13".into(),
        "bits:24".into(),
        "bits:25".into(),
        "bits:16,exp:5".into(),
        "bits:32,exp:8".into(),
        "bits:33,exp:8".into(),
        "bits:7,exp:5".into(),
        "bits:8,exp:5".into(),
        "bits:12,exp:9".into(),
        "bits:3,exp:1".into(),
        "bits:2,exp:1".into(),
        "ix:8,iy:8".into(),
        "ix:9,iy:8".into(),
        "ix:0,iy:5".into(),
        "ix:1073741824,iy:1".into(),
        "ix:1073741825,iy:4".into(),
        "ab:8".into(),
        "ec:1,ab:8".into(),
        "ec:1,ab:16,orig:1".into(),
        "ec:1,ab:13".into(),
        "ec:1,ab:8,aexp:5".into(),
        "ec:1,ab:16,aexp:5,apre:1".into(),
        "ec:1,ab:25".into(),
        "ec:2,ab:8,apre:1".into(),
        "or:0".into(),
        "or:5".into(),
        "or:8".into(),
        "or:9".into(),
        "c:0".into(),
        "c:1".into(),
        "c:2".into(),
        "c:4".into(),
        format!("it:{}", bits(100.0)),
        format!("it:{}", bits(10000.0)),
        format!("it:{nan}"),
        format!("it:{}", bits(-1.0)),
        format!("mn:{},rel:1,lb:{}", bits(0.5), bits(2.0)),
        "anim:1".into(),
        "anim:1,td:0".into(),
        "anim:1,tn:0".into(),
        "anim:1,tn:30,td:1001,nl:3,tc:1".into(),
        "orig:1,bits:16".into(),
        "orig:1,bits:12".into(),
        "ec:4".into(),
        "ec:5".into(),
        "ec:256".into(),
        "ec:257".into(),
        "prev:1,px:4,py:4".into(),
        "cont:1,pad:1793,pad:65347".into(),
    ];
    let mut requests = Vec::new();
    for info in &infos {
        let b = if info.is_empty() {
            RGB8.to_string()
        } else {
            format!("{RGB8},{info}")
        };
        for script in [
            format!("{b} l S"),
            format!("L5 {b} l S e"),
            format!("L10 {b} l S"),
            format!("{b} G0 S"),
            format!("{b} G1 S"),
            format!("{RGB8} G0 {b} l S"),
            format!("{b} Bw:3,h:3 S"),
        ] {
            requests.push(run(0, 0, &script));
        }
    }
    for level in [-2, -1, 0, 4, 5, 6, 10, 11] {
        requests.push(run(0, 0, &format!("L{level} l {RGB8} l S")));
    }
    compare("basic-info", requests);
}

#[test]
fn public_encoder_extra_channel_setters_match_original() {
    let one = bits(1.0);
    let half = bits(0.5);
    let mut requests = Vec::new();
    let base = "Bw:8,h:8,ec:3";
    for index in 0..4 {
        for (channel_type, bits_per_sample, exponent) in [
            (0, 8, 0),
            (1, 16, 0),
            (2, 12, 0),
            (3, 13, 0),
            (4, 8, 0),
            (5, 8, 0),
            (6, 24, 0),
            (16, 8, 0),
            (17, 8, 0),
            (0, 0, 0),
            (0, 25, 0),
            (0, 16, 5),
            (0, 32, 8),
            (0, 12, 9),
        ] {
            let k = format!(
                "K{index},{channel_type},{bits_per_sample},{exponent},{},{},{},{one},{half},0,{one}",
                index % 3,
                index % 2,
                index
            );
            requests.push(run(0, 0, &format!("{base} {k} l S")));
            requests.push(run(0, 0, &format!("L5 {base} {k} l S")));
        }
        for name in ["-", "6e616d65", "00", "616200", &"7a".repeat(300)] {
            requests.push(run(0, 0, &format!("{base} n{index},{name} S")));
        }
    }
    for script in [
        format!("K0,0,8,0,0,0,0,{one},{one},{one},{one} n0,61 S"),
        format!("Bw:8,h:8,ec:1,ab:8 K0,2,10,0,1,1,0,{one},{half},{one},{half} S"),
        "Bw:8,h:8,ec:2 n1,6162 Bw:8,h:8,ec:1 S Bw:8,h:8,ec:3 S".into(),
        "Bw:8,h:8,ec:1,ab:8,orig:1 K0,0,16,0,0,0,0,0,0,0,0 l S".into(),
    ] {
        requests.push(run(0, 0, &script));
    }
    compare("extra-channels", requests);
}

fn icc_profiles() -> (String, String) {
    let profiles = super::original_observations(&["epub-icc 0".into(), "epub-icc 1".into()]);
    (profiles[0].clone(), profiles[1].clone())
}

#[test]
fn public_encoder_color_setters_match_original() {
    let (rgb, gray) = icc_profiles();
    let gamma = 0.45f64.to_bits();
    let encodings = [
        "0.1.1.13.0.0".to_string(),
        "0.1.1.13.0.3".into(),
        "0.1.1.16.0.1".into(),
        "0.1.1.18.0.2".into(),
        "0.1.1.8.0.0".into(),
        "0.11.11.17.0.0".into(),
        "0.10.9.1.0.0".into(),
        "1.1.1.13.0.0".into(),
        "1.2.1.8.0.0".into(),
        "0.2.2.8.0.0".into(),
        format!("0.1.1.65535.{gamma}.0"),
        "0.1.1.65535.0.0".into(),
        "2.1.1.13.0.0".into(),
        "3.1.1.13.0.0".into(),
        "0.15.1.13.0.0".into(),
        "0.1.15.13.0.0".into(),
        "0.1.1.99.0.0".into(),
        "x:0:2:4600877379321698714:4600877379321698714:2:4604930618986332160:\
         4599075939470750515:4598895795485661266:4604480259023595110:\
         4594572339843380019:4589168020290535424:13:0:1"
            .into(),
    ];
    let mut ops: Vec<String> = ["G0", "G1", "g0", "g1"].map(String::from).to_vec();
    ops.extend(encodings.iter().map(|e| format!("C{e}")));
    ops.extend([
        format!("Q{rgb}"),
        format!("Q{gray}"),
        "Q00010203".into(),
        "Q-".into(),
        format!("Q{}", &rgb[..200]),
    ]);
    let bases = [
        "".to_string(),
        RGB8.into(),
        format!("{RGB8},c:1"),
        format!("{RGB8},orig:1"),
        format!("{RGB8},c:1,orig:1"),
        format!("{RGB8},it:{}", bits(400.0)),
    ];
    let mut requests = Vec::new();
    for base in &bases {
        for op in &ops {
            requests.push(run(0, 0, &format!("{base} {op} l S")));
            requests.push(run(0, 0, &format!("{base} {op} G0 S")));
        }
        requests.push(run(0, 0, &format!("{base} M Q{rgb} S")));
        requests.push(run(0, 0, &format!("{base} C0.1.1.16.0.1 {RGB8} S")));
    }
    compare("color", requests);
}

#[test]
fn public_encoder_frame_setting_options_match_original() {
    let mut ids: Vec<u32> = (0..=43).collect();
    ids.extend([100, 65534, 65535]);
    let values: [i64; 24] = [
        -3,
        -2,
        -1,
        0,
        1,
        2,
        3,
        4,
        5,
        7,
        8,
        9,
        10,
        11,
        12,
        15,
        16,
        41,
        42,
        70913,
        70914,
        i64::MIN,
        i64::MAX,
        4294967296,
    ];
    let floats = [
        -2.0f32,
        -1.0,
        -0.75,
        -0.5,
        -0.25,
        0.0,
        -0.0,
        0.5,
        33.3,
        100.0,
        100.5,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        1e-40,
    ];
    let mut requests = Vec::new();
    for id in &ids {
        let mut script = String::from("s-");
        for value in values {
            script.push_str(&format!(" o0,{id},{value}"));
        }
        requests.push(run(0, 0, &format!("{script} S")));
        for value in values {
            requests.push(run(0, 0, &format!("s- o0,{id},{value} S")));
        }
        for value in floats {
            requests.push(run(0, 0, &format!("s- f0,{id},{} S", bits(value))));
        }
    }
    for value in [0, 1, 10, 11, 12] {
        requests.push(run(0, 0, &format!("A s- o0,0,{value} S")));
        requests.push(run(0, 0, &format!("s- A o0,0,{value} S")));
    }
    for script in [
        format!("{RGB8} s- o0,39,1 S"),
        format!("{RGB8},orig:1 s- o0,39,1 S"),
        format!("s- o0,39,1 {RGB8} S"),
        "s- s- o1,32,7 S x S".into(),
        "s- o0,0,3 o0,12,1 s0 o1,0,9 S".into(),
    ] {
        requests.push(run(0, 0, &script));
    }
    compare("options", requests);
}

#[test]
fn public_encoder_distances_match_original() {
    let distances = [
        -1.0f32,
        -0.0,
        0.0,
        0.001,
        0.0099,
        0.0499,
        0.05,
        0.06,
        1.0,
        2.9,
        25.0,
        25.0001,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];
    let bases = [
        String::new(),
        RGB8.to_string(),
        format!("{RGB8},orig:1"),
        format!("{RGB8},ec:2"),
    ];
    let mut requests = Vec::new();
    for base in &bases {
        for distance in distances {
            requests.push(run(0, 0, &format!("{base} s- d0,{} S", bits(distance))));
        }
        requests.push(run(0, 0, &format!("{base} s- y0,1 S y0,0 S")));
    }
    for index in 0..3 {
        for distance in distances {
            let d = bits(distance);
            requests.push(run(0, 0, &format!("{RGB8},ec:2 s- D0,{index},{d} S")));
            requests.push(run(0, 0, &format!("s- {RGB8},ec:2 D0,{index},{d} S")));
        }
    }
    // JxlEncoderDistanceFromQuality over its branches, boundaries and
    // exceptional values; 50 calls per history.
    let mut qualities: Vec<f32> = (-200..=220).map(|q| q as f32 * 0.5).collect();
    qualities.extend([
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        -0.0,
        29.999998,
        30.000002,
        99.99999,
        100.00001,
        f32::MIN_POSITIVE,
        f32::MAX,
        f32::MIN,
    ]);
    let mut seed = 0x51u64;
    for _ in 0..400 {
        qualities.push(f32::from_bits(super::next(&mut seed) as u32 % 0x42dc_0000));
    }
    for chunk in qualities.chunks(50) {
        let ops: Vec<String> = chunk.iter().map(|q| format!("q{}", bits(*q))).collect();
        requests.push(run(0, 0, &ops.join(" ")));
    }
    compare("distance", requests);
}

#[test]
fn public_encoder_frame_headers_and_names_match_original() {
    let headers = [
        "0,0,0,1,0,0,0,0,0,0,0,0,0,0",
        "5,7,3,0,0,0,0,0,0,2,1,0,1,1",
        "0,0,0,1,1,-5,3,4,2,1,3,1,0,2",
        "0,0,0,1,0,0,0,0,0,0,4,0,0,0",
        "0,0,0,1,0,0,0,0,0,0,0,1,0,0",
        "0,0,0,1,0,0,0,0,0,0,0,2,0,0",
        "0,0,0,1,0,0,0,0,0,0,0,3,0,0",
        "0,0,0,1,0,0,0,0,0,4,3,0,0,3",
        "1,2,3,4,1,2147483647,-2147483648,4294967295,1,7,0,0,7,4294967295",
    ];
    let mut requests = Vec::new();
    for header in headers {
        for base in [RGB8.to_string(), format!("{RGB8},ec:2"), String::new()] {
            requests.push(run(0, 0, &format!("{base} s- h0,{header} S")));
        }
    }
    let long = "61".repeat(1071);
    let longer = "62".repeat(1072);
    for name in ["~", "-", "6672616d65", "61006263", &long, &longer] {
        requests.push(run(0, 0, &format!("s- N0,{name} S")));
        requests.push(run(0, 0, &format!("s- N0,{name} h0,{} S", headers[1])));
        requests.push(run(0, 0, &format!("s- h0,{} N0,{name} S", headers[1])));
    }
    for index in 0..3 {
        for blend in ["0,0,0,0", "2,1,1,1", "4,3,2,0", "7,7,5,3"] {
            requests.push(run(0, 0, &format!("{RGB8},ec:2 s- t0,{index},{blend} S")));
        }
    }
    requests.push(run(0, 0, "s- t0,0,1,0,0,0 S"));
    requests.push(run(
        0,
        0,
        &format!("s- {RGB8},ec:3 t0,2,1,0,0,0 {RGB8},ec:1 t0,0,2,0,0,0 S"),
    ));
    for depth in ["0,0,0", "1,8,0", "2,8,0", "2,16,5", "3,1,1", "3,0,0"] {
        requests.push(run(0, 0, &format!("s- W0,{depth} S")));
    }
    requests.push(run(
        0,
        0,
        &format!(
            "{RGB8},ec:1 s- o0,0,3 d0,{} N0,616263 h0,{} t0,0,1,0,0,0 W0,1,0,0 s0 S x S",
            bits(2.0),
            headers[2]
        ),
    ));
    compare("frame-settings", requests);
}

/// The minimum buffer size of SetFromBuffer for one channel.
fn buffer_size(xsize: usize, ysize: usize, channels: usize, data_type: u32, align: usize) -> usize {
    let bits = match data_type {
        2 => 8,
        3 | 5 => 16,
        0 => 32,
        _ => 0,
    };
    let row = xsize * channels * bits / 8;
    let stride = if align < 2 || row.is_multiple_of(align) {
        row
    } else {
        row + align - row % align
    };
    stride * (ysize - 1) + row
}

#[test]
fn public_encoder_frame_input_and_rejection_match_original() {
    let images = [
        ("Bw:8,h:8", 8, 8, 3, 0),
        ("Bw:7,h:5", 7, 5, 3, 0),
        ("Bw:7,h:5,c:1", 7, 5, 1, 0),
        ("Bw:7,h:5,ec:1,ab:8", 7, 5, 3, 1),
        ("Bw:7,h:5,c:1,ec:1,ab:16,bits:16,orig:1", 7, 5, 1, 1),
        ("Bw:9,h:3,ec:2", 9, 3, 3, 2),
    ];
    let formats = [
        (1, 2),
        (2, 2),
        (3, 2),
        (4, 2),
        (3, 3),
        (3, 0),
        (3, 5),
        (4, 5),
        (3, 1),
        (3, 4),
        (3, 6),
    ];
    let mut requests = Vec::new();
    for (index, &(image, xsize, ysize, color, ec)) in images.iter().enumerate() {
        for (f, &(channels, data_type)) in formats.iter().enumerate() {
            let endianness = (index + f) % 3;
            let align = [0, 1, 3, 4, 16][(index * 3 + f) % 5];
            let min = buffer_size(xsize, ysize, channels, data_type, align);
            let color_op = match (index + f) % 3 {
                0 => String::new(),
                1 => format!(" G{}", u8::from(color == 1)),
                _ => format!(" g{}", u8::from(color == 1)),
            };
            for size in [min.saturating_sub(1), min, min + 5] {
                let fmt = format!("{channels},{data_type},{endianness},{align}");
                requests.push(run(
                    0,
                    0,
                    &format!("{image}{color_op} s- a0,{fmt},{size} S p0 S p64"),
                ));
            }
            if ec > 0 {
                let ec_min = buffer_size(xsize, ysize, 1, data_type, align);
                let fmt = format!("{channels},{data_type},{endianness},{align}");
                let ec_fmt = format!("1,{data_type},{endianness},{align}");
                requests.push(run(
                    0,
                    0,
                    &format!(
                        "{image} G{} s- a0,{fmt},{min} E0,{ec_fmt},{ec_min},0 \
                         E0,{ec_fmt},{},{} E0,{ec_fmt},{ec_min},{ec} S p0 p64",
                        u8::from(color == 1),
                        ec_min.saturating_sub(1),
                        ec - 1
                    ),
                ));
            }
        }
    }
    let frame = "a0,3,2,0,0,192";
    for script in [
        // Order and configuration rejections.
        format!("s- {frame} S"),
        format!("{RGB8} s- F {frame} S"),
        format!("{RGB8} s- Z {frame} S"),
        format!("s- y0,1 {RGB8} {frame} S"),
        format!("s- o0,39,1 {RGB8} {frame} S"),
        format!("{RGB8},c:1 s- {frame} S"),
        format!("{RGB8} s- a0,1,2,0,0,64 S"),
        format!("{RGB8} s- a0,4,2,0,0,256 S"),
        format!("{RGB8} s- o0,41,1 a0,4,2,0,0,256 S p64"),
        format!("{RGB8} s- o0,41,2 a0,4,2,0,0,256 S"),
        // Crops, resampling and already-downsampled dimensions.
        format!("{RGB8} s- h0,0,0,0,1,1,2,3,4,3,0,0,0,0,0 a0,3,2,0,0,36 S p64"),
        format!("{RGB8} s- h0,0,0,0,1,1,0,0,0,3,0,0,0,0,0 a0,3,2,0,0,36 S"),
        format!("{RGB8} s- o0,4,1 {frame} S"),
        format!("{RGB8} s- o0,2,2 o0,4,1 a0,3,2,0,0,48 S p64"),
        format!("{RGB8} s- o0,2,8 o0,4,1 a0,3,2,0,0,3 S"),
        format!("{RGB8} s- u2,0 u4,1 u8,1 u8,-1 u3,0 u2,-2 u2,2 S"),
        // Null buffers and pixel formats without bytes per pixel.
        format!("{RGB8} s- a0,3,2,0,0,~ S"),
        format!("{RGB8} s- a0,3,1,0,0,~ S p0 p1"),
        format!("{RGB8} s- a0,3,4,0,0,0 S"),
        // Extra-channel buffers in the wrong order or state.
        "Bw:7,h:5,ec:1 s- E0,1,2,0,0,35,0 S".into(),
        "Bw:7,h:5,ec:1 G0 s- E0,1,2,0,0,35,0 S".into(),
        "Bw:7,h:5,ec:1 G0 s- a0,3,2,0,0,105 F E0,1,2,0,0,35,0 S".into(),
        "Bw:7,h:5,ec:1 G0 s- a0,3,2,0,0,105 E0,1,2,0,0,~,0 E0,1,1,0,0,~,0 S".into(),
        "Bw:7,h:5,ec:1 s- a0,3,2,0,0,105 E0,1,2,0,0,35,0 S".into(),
        // Several queued frames and settings.
        format!("{RGB8} s- s- o1,0,9 {frame} a1,3,2,0,0,192 a0,3,3,0,0,384 S p0 S"),
        format!("{RGB8} s- {frame} x {RGB8} s- {frame} S"),
        // Fast lossless at effort 1 and its near misses.
        format!("{RGB8},orig:1 s- o0,0,1 y0,1 {frame} S"),
        format!("{RGB8},orig:1,c:1 s- o0,0,1 y0,1 a0,1,2,0,0,64 S"),
        format!("{RGB8},orig:1,bits:16 s- o0,0,1 y0,1 a0,3,3,0,0,384 S"),
        format!("{RGB8},orig:1,ec:1,ab:8 s- o0,0,1 y0,1 a0,4,2,0,0,256 S"),
        format!("{RGB8},orig:1,ec:1,ab:8 s- o0,0,1 y0,1 o0,41,1 a0,4,2,0,0,256 S p8"),
        format!("{RGB8},orig:1 s- o0,0,2 y0,1 {frame} S p8"),
        format!("{RGB8},orig:1 s- o0,0,1 y0,1 N0,61 {frame} S p8"),
        format!("{RGB8},orig:1 s- o0,0,1 y0,1 o0,31,1 {frame} S p8"),
        format!("{RGB8},orig:1,bits:10 s- o0,0,1 y0,1 a0,3,3,0,0,384 S p8"),
        format!("{RGB8},orig:1,bits:10 s- o0,0,1 y0,1 W0,1,0,0 a0,3,3,0,0,384 S p8"),
        format!("{RGB8},orig:1 s- o0,0,1 y0,1 a0,3,0,0,0,768 S p8"),
        format!("{RGB8},orig:1,anim:1 s- o0,0,1 y0,1 {frame} S p8"),
        format!("{RGB8},orig:1 s- o0,0,1 d0,0 {frame} S"),
    ] {
        requests.push(run(0, 0, &script));
        requests.push(run(1, 0, &script));
    }
    compare("frames", requests);
}

#[test]
fn public_encoder_boxes_and_output_match_original() {
    let types = [
        "45786966", "786d6c20", "6a756d62", "6a786c63", "6a786c70", "6a627264", "62726f62",
        "6a786c69", "00000000",
    ];
    let mut requests = Vec::new();
    for box_type in types {
        for compress in 0..2 {
            requests.push(run(0, 0, &format!("X O{box_type},12,{compress} S p0 S p1")));
            requests.push(run(1, 0, &format!("O{box_type},3,{compress} S")));
        }
    }
    for script in [
        "X Y O45786966,4,0 S",
        "X Z O45786966,4,0 S",
        "X O45786966,0,0 O45786966,4096,1 S x O45786966,4,0 S",
        "U1 J1 X L10 U0 J0 L5 S",
        "p0 p1 p64 S",
        "Bw:8,h:8 p8 Z p8 x p0 S",
        "X O45786966,4,0 p0 S Z p0 p1",
        "Bw:8,h:8 G0 s- a0,3,2,0,0,192 Z p0 p0 S p7",
    ] {
        requests.push(run(0, 0, script));
        requests.push(run(1, 0, script));
    }
    compare("boxes-output", requests);
}

#[test]
fn public_encoder_output_wrapper_buffer_mode_matches_original() {
    let mut requests = vec![
        "epub-wrap P F P".to_string(),
        "epub-wrap A0 G1,4,3,1 F P".into(),
        "epub-wrap G0,4,3,1 G1,0,0,1 R1,8,5,2 F P".into(),
        "epub-wrap A10 G1,4,3,1 F P D20,2 F A30 P".into(),
        "epub-wrap D9,1 k4 D3,2 F A8 F P K2 K12 F".into(),
        "epub-wrap A64 H8,20,3 F H16,0,4 F P".into(),
    ];
    let mut seed = 0xE11Cu64;
    let mut next = |bound: u64| super::next(&mut seed) % bound;
    for _ in 0..600 {
        let count = 6 + next(13) as usize;
        let avail_at = match next(10) {
            0..=3 => Some(0),
            4..=6 => Some(next(count as u64) as usize),
            _ => None,
        };
        let mut ops = Vec::new();
        for i in 0..count {
            if avail_at == Some(i) {
                ops.push(format!("A{}", next(81)));
            }
            ops.push(match next(9) {
                0 | 1 => format!("G{},{},{},{}", next(7), next(25), next(31), next(9)),
                2 => format!("R{},{},{},{}", 1 + next(6), next(25), next(31), next(9)),
                3 | 4 => format!("D{},{}", next(31), next(9)),
                5 => format!("k{}", next(11)),
                6 => format!("H{},{},{}", 1 + next(12), next(21), next(9)),
                7 => "F".into(),
                _ => "P".into(),
            });
        }
        ops.push("F".into());
        ops.push("P".into());
        requests.push(format!("epub-wrap {}", ops.join(" ")));
    }
    super::compare(requests);
}

/// A ProcessOutput schedule: 0, 1, 7 and 64 bytes, then doubling, until the
/// sum exceeds `total`.
fn output_schedule(total: usize) -> String {
    let mut ops = vec!["p0".to_string(), "p1".into(), "p7".into()];
    let mut sum = 8;
    let mut avail = 64;
    while sum <= total {
        ops.push(format!("p{avail}"));
        sum += avail;
        avail *= 2;
    }
    ops.join(" ")
}

/// Data types of the lossy gate: (JxlDataType, bytes per sample, basic-info
/// bit depth fields).
const LOSSY_TYPES: [(u32, usize, &str); 4] = [
    (2, 1, ""),
    (3, 2, ",bits:16"),
    (5, 2, ",bits:16,exp:5"),
    (0, 4, ",bits:32,exp:8"),
];

/// Distances of the admitted profile (below kMinButteraugliForDots): below
/// 0.7 the original skips the AR heuristics (epf_iters == 0 at kSquirrel);
/// from 0.7 it runs them with the decoder in the loop.
const LOSSY_DISTANCES: [f32; 11] = [0.5, 1.0, 0.1, 1.5, 2.9, 0.69, 0.7, 2.0, 0.05, 0.3, 0.001];

/// One lossy history: basic info, optional color encoding and settings, one
/// frame of generated content, CloseInput and a ProcessOutput schedule.
#[expect(clippy::too_many_arguments, reason = "One history's fields")]
fn lossy_history(
    xsize: usize,
    ysize: usize,
    gray: bool,
    type_index: usize,
    content: usize,
    distance: f32,
    variant: usize,
    extra_info: &str,
    extra_settings: &str,
) -> String {
    let (data_type, bytes, depth) = LOSSY_TYPES[type_index];
    let channels = if gray { 1 } else { 3 };
    let endianness = variant % 3;
    let align = [0, 1, 4, 16, 3][variant % 5];
    let row = xsize * channels * bytes;
    let stride = if align < 2 || row.is_multiple_of(align) {
        row
    } else {
        row + align - row % align
    };
    let size = stride * (ysize - 1) + row + usize::from(variant % 4 == 3) * 5;
    let mut info = format!("Bw:{xsize},h:{ysize}{depth}{extra_info}");
    if gray {
        info.push_str(",c:1");
    }
    if variant % 7 == 2 {
        info.push_str(&format!(",it:{}", bits(100.0)));
    }
    let color = match variant % 3 {
        0 => String::new(),
        1 => format!(" G{}", u8::from(gray)),
        _ => format!(" g{}", u8::from(gray)),
    };
    let encoded_bound = xsize * ysize * channels * 4 + 2048;
    format!(
        "{info}{color} s- d0,{}{extra_settings} \
         a0,{channels},{data_type},{endianness},{align},{size},{content},{xsize},{ysize} Z {}",
        bits(distance),
        output_schedule(encoded_bound)
    )
}

/// Every history of an admitted lossy family must match without admission.
fn compare_admitted(family: &str, requests: Vec<String>) {
    let count = requests.len();
    let exact = compare_counted(family, requests);
    assert_eq!(exact, count, "{family}: every admitted history is exact");
}

#[test]
fn public_encoder_lossy_vardct_small_sizes_match_original() {
    let mut sizes = vec![(1, 1), (7, 7), (8, 8), (9, 17)];
    sizes.extend((41..=48).map(|x| (x, 15)));
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize) in &sizes {
        for gray in [false, true] {
            for type_index in 0..4 {
                for content in 1..=4 {
                    let distance = LOSSY_DISTANCES[variant % LOSSY_DISTANCES.len()];
                    requests.push(run(
                        0,
                        0,
                        &lossy_history(
                            xsize, ysize, gray, type_index, content, distance, variant, "", "",
                        ),
                    ));
                    variant += 1;
                }
            }
        }
    }
    compare_admitted("lossy-small", requests);
}

#[test]
fn public_encoder_lossy_vardct_large_sizes_match_original() {
    let sizes = [(63, 129), (256, 256), (257, 255), (512, 1024)];
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize) in &sizes {
        for gray in [false, true] {
            for content in 1..=4 {
                let distance = [1.0, 2.9, 0.5, 1.5][variant % 4];
                requests.push(run(
                    0,
                    0,
                    &lossy_history(
                        xsize,
                        ysize,
                        gray,
                        variant % 4,
                        content,
                        distance,
                        variant,
                        "",
                        "",
                    ),
                ));
                variant += 1;
            }
        }
    }
    compare_admitted("lossy-large", requests);
}

#[test]
fn public_encoder_lossy_vardct_d05_and_settings_match_original() {
    let sizes = [
        (1, 1),
        (7, 7),
        (8, 8),
        (9, 17),
        (45, 15),
        (63, 129),
        (256, 256),
        (257, 255),
        (512, 1024),
    ];
    let mut requests = Vec::new();
    for (index, &(xsize, ysize)) in sizes.iter().enumerate() {
        // d1.0 and d0.5 at both intensity targets (variant % 7 == 2 sets 100
        // nits).
        for distance in [1.0, 0.5] {
            for base in [0, 2] {
                let variant = base + index * 7;
                requests.push(run(
                    0,
                    0,
                    &lossy_history(xsize, ysize, false, 0, 4, distance, variant, "", ""),
                ));
                requests.push(run(
                    0,
                    0,
                    &lossy_history(xsize, ysize, true, 3, 4, distance, variant, "", ""),
                ));
            }
        }
    }
    // Buffering values that keep the one-shot path, explicit effort 7 and
    // 16 groups with buffering 0.
    for buffering in [-1, 0, 1, 2, 3] {
        requests.push(run(
            0,
            0,
            &lossy_history(
                512,
                1024,
                false,
                0,
                4,
                0.3,
                5,
                "",
                &format!(" o0,34,{buffering}"),
            ),
        ));
    }
    requests.push(run(
        0,
        0,
        &lossy_history(64, 64, false, 1, 2, 0.5, 6, "", " o0,0,7"),
    ));
    requests.push(run(
        0,
        0,
        &lossy_history(1024, 1024, false, 0, 4, 0.5, 0, "", " o0,34,0"),
    ));
    // Bit depth from the codestream and a custom bit depth for uint16 input.
    for (info_bits, depth) in [(",bits:12", "1,0,0"), (",bits:16", "2,10,0")] {
        requests.push(run(
            0,
            0,
            &lossy_history(
                19,
                23,
                false,
                1,
                4,
                0.5,
                4,
                info_bits,
                &format!(" W0,{depth}"),
            ),
        ));
    }
    // Basic-info metadata: orientation.
    for orientation in [1, 5, 8] {
        requests.push(run(
            0,
            0,
            &lossy_history(
                21,
                13,
                false,
                0,
                2,
                0.5,
                0,
                &format!(",or:{orientation}"),
                "",
            ),
        ));
    }
    compare_admitted("lossy-settings", requests);
}

#[test]
fn public_encoder_lossy_vardct_admissions_and_rejections_match_original() {
    let mut requests = Vec::new();
    // Distances from 0.7 run the AR heuristics (exact since the AR
    // translation), at the gate's d1.0, d1.5 and d2.9 and at 0.7.
    for (index, distance) in [0.7f32, 1.0, 1.5, 2.9].into_iter().enumerate() {
        for (xsize, ysize) in [(1, 1), (9, 17), (256, 256)] {
            requests.push(run(
                0,
                0,
                &lossy_history(
                    xsize,
                    ysize,
                    index % 2 == 1,
                    index,
                    4,
                    distance,
                    index,
                    "",
                    "",
                ),
            ));
        }
    }
    // An explicit EPF setting is outside the profile (FrameProfile).
    requests.push(run(
        0,
        0,
        &lossy_history(16, 16, false, 0, 4, 0.5, 0, "", " o0,9,1"),
    ));
    // More than 8 groups take the streaming path (exact since step 2).
    for (xsize, ysize, buffering) in [(768, 768, -1), (2049, 300, 2), (768, 769, 3)] {
        requests.push(run(
            0,
            0,
            &lossy_history(
                xsize,
                ysize,
                false,
                0,
                2,
                0.5,
                0,
                "",
                &format!(" o0,34,{buffering}"),
            ),
        ));
    }
    // Outside the one-shot profile (FrameProfile): another effort, modular,
    // lossless, a non-default option, an unclosed (non-last) frame, a crop,
    // a custom memory manager, d >= 3.
    for settings in [
        " o0,0,6",
        " o0,0,9",
        " o0,11,1",
        " y0,1",
        " o0,10,0",
        " o0,6,0",
        " o0,1,1",
        " o0,13,1",
        " h0,0,0,0,1,1,0,0,8,8,0,0,0,0,0",
    ] {
        requests.push(run(
            0,
            0,
            &lossy_history(16, 16, false, 0, 4, 0.5, 0, "", settings),
        ));
    }
    requests.push(run(
        1,
        0,
        &lossy_history(16, 16, false, 0, 4, 0.5, 0, "", ""),
    ));
    requests.push(run(
        0,
        0,
        &lossy_history(16, 16, false, 0, 4, 3.0, 0, "", ""),
    ));
    requests.push(run(
        0,
        0,
        &lossy_history(16, 16, false, 0, 4, 0.5, 0, "", "").replacen(" Z ", " ", 1),
    ));
    // Rejections of the step 1 gate before any output: a frame before basic
    // info, after CloseInput, with a short buffer, invalid distances, and a
    // gray color encoding for an RGB image.
    let frame = "a0,3,2,0,0,192,4,8,8";
    for script in [
        format!("s- {frame} p64"),
        format!("Bw:8,h:8 s- Z {frame} p64"),
        "Bw:8,h:8 s- a0,3,2,0,0,191,4,8,8 Z p64".to_string(),
        format!("Bw:8,h:8 s- d0,{} {frame} Z p64", bits(-1.0)),
        format!("Bw:8,h:8 s- d0,{} {frame} Z p64", bits(25.5)),
        format!("Bw:8,h:8 G1 s- {frame} Z p64"),
        format!("Bw:8,h:8 s- o0,0,10 {frame} Z p64"),
    ] {
        requests.push(run(0, 0, &script));
    }
    compare("lossy-admissions", requests);
}

/// The codestream bytes that one Rust history wrote through ProcessOutput.
fn encoded_bytes(request: &str) -> Vec<u8> {
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("epub-run"));
    let observation = driver::observe(&mut args);
    assert!(
        observation.admission.is_none(),
        "admitted history: {request}"
    );
    let mut bytes = Vec::new();
    for field in observation.text.split(' ') {
        let Some(result) = field.strip_prefix("p:") else {
            continue;
        };
        let parts: Vec<&str> = result.split(',').collect();
        let written: usize = parts[1].parse().unwrap();
        let hex = parts[3].split('/').next().unwrap();
        for i in 0..written {
            bytes.push(u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap());
        }
    }
    bytes
}

/// Decode-back: the Rust encoder's codestreams, which equal the original's
/// byte for byte (see the families above), decoded by the Rust public decoder
/// and by the original decoder give identical events, metadata and pixels
/// (float and uint8 output, whole-file and small input chunks).
#[test]
fn public_encoder_lossy_vardct_decode_back_matches_original_decoder() {
    let mut sizes = vec![
        (1, 1),
        (7, 7),
        (8, 8),
        (9, 17),
        (43, 15),
        (63, 129),
        (257, 255),
    ];
    sizes.push((512, 1024));
    let mut requests = Vec::new();
    for (index, &(xsize, ysize)) in sizes.iter().enumerate() {
        for gray in [false, true] {
            let variant = index * 3 + usize::from(gray);
            let distance = LOSSY_DISTANCES[variant % LOSSY_DISTANCES.len()];
            let history = run(
                0,
                0,
                &lossy_history(
                    xsize,
                    ysize,
                    gray,
                    variant % 4,
                    1 + variant % 4,
                    distance,
                    variant,
                    "",
                    "",
                ),
            );
            let file = driver::hex(&encoded_bytes(&history));
            requests.push(format!(
                "dpub-run 0 0 {file} s5184 F3.0.0.0.0 L1,1000000,0,1000,1"
            ));
            requests.push(format!(
                "dpub-run 0 0 {file} s5120 F4.2.0.0.0 L2,64,0,1000,1"
            ));
        }
    }
    super::decoder_public::compare("lossy-decode-back", requests);
}

/// Encoder step 2: more than 8 groups take EncodeFrameStreaming, in output
/// mode 0 (collected sections, buffering -1 to 2) and 1 (buffering 3, TOC
/// written after the groups through output seeks).
#[test]
fn public_encoder_lossy_vardct_streaming_matches_original() {
    let cases = [
        (768, 768, -1),
        (768, 769, 0),
        (1000, 700, -1),
        (2049, 300, 1),
        (2049, 300, 2),
        (768, 768, 3),
        (1031, 517, 3),
        (1024, 1024, -1),
    ];
    let mut requests = Vec::new();
    for (index, &(xsize, ysize, buffering)) in cases.iter().enumerate() {
        for gray in [false, true] {
            let variant = index * 2 + usize::from(gray);
            let settings = if buffering == -1 {
                String::new()
            } else {
                format!(" o0,34,{buffering}")
            };
            requests.push(run(
                0,
                0,
                &lossy_history(
                    xsize,
                    ysize,
                    gray,
                    variant % 4,
                    1 + variant % 4,
                    [1.0, 0.5, 2.9, 1.5][variant % 4],
                    variant,
                    "",
                    &settings,
                ),
            ));
        }
    }
    compare_admitted("lossy-streaming", requests);
}

#[test]
fn public_encoder_lossy_vardct_smoke_matches_original() {
    let mut requests = Vec::new();
    for (w, h) in [(8usize, 8usize), (1, 1), (17, 9)] {
        for d in [0.5f32, 0.1, 1.0] {
            let size = w * h * 3;
            requests.push(run(
                0,
                0,
                &format!(
                    "Bw:{w},h:{h} s- d0,{} a0,3,2,0,0,{size} Z {}",
                    bits(d),
                    output_schedule(4 * size + 1024)
                ),
            ));
        }
    }
    compare("lossy-smoke", requests);
}

/// Alpha bit-depth fields of the basic info for each lossy data type. Alpha
/// deeper than 12 bits clears modular_16_bit_buffer_sufficient, so
/// VerifyLevelSettings requires level 10 and the original writes a container
/// (step 3); the raw-codestream profile keeps alpha at 12 bits or less.
const ALPHA_DEPTHS: [&str; 4] = [",ab:8", ",ab:12", ",ab:8", ",ab:10"];

/// One lossy history with an alpha channel: interleaved (gray-alpha or RGBA)
/// or, with `separate`, color plus a JxlEncoderSetExtraChannelBuffer alpha
/// plane. `alpha_mode` is the content generator's: 1 opaque, 2 a ramp with
/// transparent stripes, 3 a ramp without zeros.
#[expect(clippy::too_many_arguments, reason = "One history's fields")]
fn alpha_history(
    xsize: usize,
    ysize: usize,
    gray: bool,
    type_index: usize,
    content: usize,
    alpha_mode: usize,
    distance: f32,
    variant: usize,
    separate: bool,
    extra_info: &str,
    extra_settings: &str,
) -> String {
    let (data_type, bytes, depth) = LOSSY_TYPES[type_index];
    let color_channels = if gray { 1 } else { 3 };
    let channels = if separate {
        color_channels
    } else {
        color_channels + 1
    };
    let endianness = variant % 3;
    let align = [0, 1, 4, 16, 3][variant % 5];
    let stride = |channels: usize| {
        let row = xsize * channels * bytes;
        let stride = if align < 2 || row.is_multiple_of(align) {
            row
        } else {
            row + align - row % align
        };
        stride * (ysize - 1) + row
    };
    let size = stride(channels) + usize::from(variant % 4 == 3) * 5;
    let mut info = format!(
        "Bw:{xsize},h:{ysize}{depth},ec:1{}{extra_info}",
        ALPHA_DEPTHS[type_index]
    );
    if gray {
        info.push_str(",c:1");
    }
    if variant % 7 == 2 {
        info.push_str(&format!(",it:{}", bits(100.0)));
    }
    let color = match variant % 3 {
        0 => String::new(),
        1 => format!(" G{}", u8::from(gray)),
        _ => format!(" g{}", u8::from(gray)),
    };
    let content = content + 16 * alpha_mode;
    let mut frame =
        format!("a0,{channels},{data_type},{endianness},{align},{size},{content},{xsize},{ysize}");
    if separate {
        frame.push_str(&format!(
            " E0,1,{data_type},{endianness},{align},{},0,{content},{xsize},{ysize}",
            stride(1)
        ));
    }
    let encoded_bound = xsize * ysize * (channels + 1) * 4 + 2048;
    format!(
        "{info}{color} s- d0,{}{extra_settings} {frame} Z {}",
        bits(distance),
        output_schedule(encoded_bound)
    )
}

/// Encoder step 4: lossy VarDCT with one lossless alpha channel at the
/// default settings, 8- and 16-bit integer input, interleaved RGBA and
/// gray-alpha rows, transparent and translucent alpha (SimplifyInvisible),
/// sizes from 1x1 to 512x1024 through the one-shot path.
#[test]
fn public_encoder_lossy_vardct_alpha_small_sizes_match_original() {
    let mut sizes = vec![(1, 1), (7, 7), (8, 8), (9, 17)];
    sizes.extend((41..=48).map(|x| (x, 15)));
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize) in &sizes {
        for gray in [false, true] {
            for type_index in 0..2 {
                for alpha_mode in [2, 3] {
                    let distance = LOSSY_DISTANCES[variant % LOSSY_DISTANCES.len()];
                    requests.push(run(
                        0,
                        0,
                        &alpha_history(
                            xsize,
                            ysize,
                            gray,
                            type_index,
                            1 + variant % 4,
                            alpha_mode,
                            distance,
                            variant,
                            false,
                            "",
                            "",
                        ),
                    ));
                    variant += 1;
                }
            }
        }
    }
    compare_admitted("alpha-small", requests);
}

#[test]
fn public_encoder_lossy_vardct_alpha_large_sizes_match_original() {
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize) in &[(63, 129), (256, 256), (257, 255), (512, 1024)] {
        for gray in [false, true] {
            for alpha_mode in [2, 3] {
                let distance = [1.0, 2.9, 0.5, 1.5, 0.7][variant % 5];
                requests.push(run(
                    0,
                    0,
                    &alpha_history(
                        xsize,
                        ysize,
                        gray,
                        variant % 2,
                        4,
                        alpha_mode,
                        distance,
                        variant,
                        false,
                        "",
                        "",
                    ),
                ));
                variant += 1;
            }
        }
    }
    compare_admitted("alpha-large", requests);
}

/// More than 8 groups with lossless alpha: ModularPartIsLossless holds, so
/// CanDoStreamingEncoding keeps the streaming path (the extra-channel check
/// excludes only lossy or responsive modular parts), and the alpha streams
/// are coded per DC group with local trees. Output modes 0 and 1, and
/// buffering 0 for the one-shot path at the same sizes.
#[test]
fn public_encoder_lossy_vardct_alpha_streaming_matches_original() {
    let cases = [
        (768, 768, -1),
        (768, 769, 0),
        (1000, 700, -1),
        (2049, 300, 1),
        (2049, 300, 2),
        (768, 768, 3),
        (1031, 517, 3),
        (1024, 1024, -1),
    ];
    let mut requests = Vec::new();
    for (index, &(xsize, ysize, buffering)) in cases.iter().enumerate() {
        for gray in [false, true] {
            let variant = index * 2 + usize::from(gray);
            let settings = if buffering == -1 {
                String::new()
            } else {
                format!(" o0,34,{buffering}")
            };
            requests.push(run(
                0,
                0,
                &alpha_history(
                    xsize,
                    ysize,
                    gray,
                    variant % 2,
                    1 + variant % 4,
                    2 + variant % 2,
                    [1.0, 0.5, 2.9, 1.5][variant % 4],
                    variant,
                    false,
                    "",
                    &settings,
                ),
            ));
        }
    }
    compare_admitted("alpha-streaming", requests);
}

/// Alpha variants of the admitted profile: fully opaque alpha (stripped by
/// the opacity scan, every type), the strip-alpha option, a separate alpha
/// buffer (JxlEncoderSetExtraChannelBuffer), premultiplied alpha (no
/// SimplifyInvisible), float16 and float32 color input with integer alpha,
/// low alpha bit depths, and text-like content with patches off.
#[test]
fn public_encoder_lossy_vardct_alpha_variants_match_original() {
    let mut requests = Vec::new();
    let mut push = |history: String| requests.push(run(0, 0, &history));
    for (index, &(xsize, ysize)) in [(1, 1), (9, 17), (45, 15), (256, 256), (768, 768)]
        .iter()
        .enumerate()
    {
        for type_index in 0..4 {
            let variant = index * 4 + type_index;
            let gray = variant % 2 == 1;
            let d = [1.0, 0.5, 2.9, 1.5][variant % 4];
            // Opaque: stripped at the default (auto) and if-opaque settings.
            for settings in ["", " o0,41,2"] {
                push(alpha_history(
                    xsize, ysize, gray, type_index, 4, 1, d, variant, false, "", settings,
                ));
            }
            // Translucent alpha with every input type.
            push(alpha_history(
                xsize, ysize, gray, type_index, 2, 2, d, variant, false, "", "",
            ));
            // A separate alpha plane.
            push(alpha_history(
                xsize,
                ysize,
                gray,
                type_index,
                4,
                2 + variant % 2,
                d,
                variant,
                true,
                "",
                "",
            ));
            // Premultiplied alpha.
            push(alpha_history(
                xsize, ysize, gray, type_index, 4, 2, d, variant, false, ",apre:1", "",
            ));
        }
    }
    for (xsize, ysize) in [(7, 7), (63, 129), (257, 255)] {
        for (variant, ab) in [1usize, 4, 7, 12].into_iter().enumerate() {
            let gray = variant % 2 == 0;
            // Low bit depths: ab overrides the type's default alpha depth.
            push(alpha_history(
                xsize,
                ysize,
                gray,
                0,
                4,
                2 + variant % 2,
                1.0,
                variant,
                false,
                &format!(",ab:{ab}"),
                "",
            ));
        }
        // Strip alpha forced, and never stripped.
        for settings in [" o0,41,1", " o0,41,0"] {
            push(alpha_history(
                xsize, ysize, false, 0, 4, 1, 1.0, 1, false, "", settings,
            ));
        }
        // Text-like content with alpha, patches off.
        push(alpha_history(
            xsize, ysize, false, 0, 5, 3, 1.0, 2, false, "", " o0,8,0",
        ));
    }
    compare_admitted("alpha-variants", requests);
}

/// Alpha settings: keep_invisible on and off, and lossy alpha through
/// JxlEncoderSetExtraChannelDistance (a lossy modular part: Squeeze and
/// quantization in the modular encoder, and no streaming path).
#[test]
fn public_encoder_lossy_vardct_alpha_settings_match_original() {
    let mut requests = Vec::new();
    for (index, &(xsize, ysize)) in [(1, 1), (9, 17), (45, 15), (63, 129), (257, 255), (768, 768)]
        .iter()
        .enumerate()
    {
        for (k, settings) in [
            " o0,12,1".to_string(),
            " o0,12,0".to_string(),
            format!(" D0,0,{}", bits(1.0)),
            format!(" D0,0,{}", bits(0.5)),
            format!(" D0,0,{}", bits(3.0)),
        ]
        .iter()
        .enumerate()
        {
            let variant = index * 5 + k;
            requests.push(run(
                0,
                0,
                &alpha_history(
                    xsize,
                    ysize,
                    variant % 2 == 1,
                    variant % 2,
                    1 + variant % 4,
                    2 + variant % 2,
                    [1.0, 0.5, 2.9, 1.5][variant % 4],
                    variant,
                    false,
                    "",
                    settings,
                ),
            ));
        }
    }
    compare_admitted("alpha-settings", requests);
}

/// The codestream bytes of a history's ProcessOutput results (an observation
/// line of either implementation).
fn output_bytes(text: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for field in text.split(' ') {
        let Some(result) = field.strip_prefix("p:") else {
            continue;
        };
        let parts: Vec<&str> = result.split(',').collect();
        let Ok(written) = parts[1].parse::<usize>() else {
            continue;
        };
        let hex = parts[3].split('/').next().unwrap();
        for i in 0..written.min(hex.len() / 2) {
            bytes.push(u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap());
        }
    }
    bytes
}

/// Diagnostics: for each history in the file that JXL_ENCODER_PUBLIC_REQUESTS
/// names, the first divergence between the Rust and original observations:
/// the call, and for output the codestream byte offset.
#[test]
#[ignore = "diagnostics for a request file"]
fn public_encoder_first_divergence() {
    let Some(path) = std::env::var_os("JXL_ENCODER_PUBLIC_REQUESTS") else {
        return;
    };
    let requests: Vec<String> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect();
    let originals = super::original_observations(&requests);
    for (request, cpp) in requests.iter().zip(&originals) {
        let mut args = request.split_whitespace();
        args.next();
        let rust = driver::observe(&mut args);
        let r: Vec<&str> = rust.text.split(' ').collect();
        let c: Vec<&str> = cpp.split(' ').collect();
        let call = r.iter().zip(&c).position(|(a, b)| a != b);
        let (rb, cb) = (output_bytes(&rust.text), output_bytes(cpp));
        let byte = rb.iter().zip(&cb).position(|(a, b)| a != b);
        println!(
            "{}: admission {:?} first differing call {:?} ({}) byte {:?} sizes {}/{}",
            &request[..request.len().min(160)],
            rust.admission,
            call,
            call.map_or("", |i| &r[i][..r[i].len().min(12)]),
            byte,
            rb.len(),
            cb.len()
        );
    }
}

/// The patches option at 0 (JXL_ENC_FRAME_SETTING_PATCHES, Override::kOff):
/// LossyFrameHeuristics skips FindBestPatchDictionary, so every image of the
/// lossy profile encodes, including text-like content (content 5) that
/// FindTextLikePatches turns into patches at the default.
#[test]
fn public_encoder_lossy_vardct_patches_off_matches_original() {
    const PATCHES_OFF: &str = " o0,8,0";
    let mut requests = Vec::new();
    let mut variant = 0;
    let mut sizes = vec![(1, 1), (7, 7), (8, 8), (9, 17)];
    sizes.extend((41..=48).step_by(3).map(|x| (x, 15)));
    for &(xsize, ysize) in &sizes {
        for gray in [false, true] {
            for type_index in 0..4 {
                for content in [1 + variant % 4, 5] {
                    let distance = LOSSY_DISTANCES[variant % LOSSY_DISTANCES.len()];
                    requests.push(run(
                        0,
                        0,
                        &lossy_history(
                            xsize,
                            ysize,
                            gray,
                            type_index,
                            content,
                            distance,
                            variant,
                            "",
                            PATCHES_OFF,
                        ),
                    ));
                    variant += 1;
                }
            }
        }
    }
    for &(xsize, ysize) in &[(63, 129), (96, 64), (256, 256), (257, 255), (512, 1024)] {
        for gray in [false, true] {
            for content in [4, 5] {
                let distance = [1.0, 2.9, 0.5, 1.5, 0.7][variant % 5];
                requests.push(run(
                    0,
                    0,
                    &lossy_history(
                        xsize,
                        ysize,
                        gray,
                        variant % 4,
                        content,
                        distance,
                        variant,
                        "",
                        PATCHES_OFF,
                    ),
                ));
                variant += 1;
            }
        }
    }
    // More than 8 groups: the streaming path (patches are never searched
    // there), output modes 0 and 1.
    for (xsize, ysize, buffering) in [(768, 768, -1), (1031, 517, 3), (2049, 300, 2)] {
        let settings = if buffering == -1 {
            PATCHES_OFF.to_string()
        } else {
            format!("{PATCHES_OFF} o0,34,{buffering}")
        };
        requests.push(run(
            0,
            0,
            &lossy_history(xsize, ysize, false, 0, 5, 1.0, variant, "", &settings),
        ));
        variant += 1;
    }
    compare_admitted("lossy-patches-off", requests);
}

/// Text-like content at the default patches setting: FindTextLikePatches
/// (compared with the original by `ecf-patches`) finds patches, and the
/// encode returns the PatchDictionary admission result after an exact prefix.
/// The same content with the patches option at 0 is exact (see
/// [`public_encoder_lossy_vardct_patches_off_matches_original`]).
#[test]
fn public_encoder_lossy_vardct_text_like_default_admission_matches_original() {
    let mut requests = Vec::new();
    for (index, &(xsize, ysize)) in [(64, 64), (96, 64), (63, 129), (256, 256)]
        .iter()
        .enumerate()
    {
        for distance in [0.5f32, 1.0, 2.9] {
            requests.push(run(
                0,
                0,
                &lossy_history(xsize, ysize, index % 2 == 1, 0, 5, distance, index, "", ""),
            ));
        }
    }
    let patches = requests
        .iter()
        .filter(|request| {
            let mut args = request.split_whitespace();
            args.next();
            driver::observe(&mut args)
                .admission
                .is_some_and(|(_, checkpoint)| checkpoint == JxlEncoderAdmission::PatchDictionary)
        })
        .count();
    println!(
        "text-like default: {patches} of {} histories found patches",
        requests.len()
    );
    assert!(
        patches > 0,
        "text-like content must reach the patch dictionary"
    );
    // The original's evidence: with patches found, its default codestream
    // (with the patch reference frame) differs from its codestream with the
    // patches option at 0, which the Rust encoder matches exactly.
    let off: Vec<String> = requests
        .iter()
        .map(|request| request.replacen(" s- ", " s- o0,8,0 ", 1))
        .collect();
    let (default_cpp, off_cpp) = (
        super::original_observations(&requests),
        super::original_observations(&off),
    );
    for ((request, default_cpp), off_cpp) in requests.iter().zip(&default_cpp).zip(&off_cpp) {
        assert_ne!(
            output_bytes(default_cpp),
            output_bytes(off_cpp),
            "the original codes patches: {request}"
        );
    }
    compare_admitted("lossy-text-patches-off", off);
    compare("lossy-text-default", requests);
}
