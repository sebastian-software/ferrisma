// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
use super::*;
use jxl::skcms::transfer_function::TransferFunction as Tf;

pub(super) fn handles(kind: &str) -> bool {
    matches!(kind, "skcms-tf" | "skcms-tf-in-place" | "skcms-make")
}
fn read(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(number(args))
}
fn read_tf(args: &mut std::str::SplitWhitespace<'_>) -> Tf {
    Tf {
        g: read(args),
        a: read(args),
        b: read(args),
        c: read(args),
        d: read(args),
        e: read(args),
        f: read(args),
    }
}
fn write_tf(out: &mut String, tf: Tf) {
    for value in [tf.g, tf.a, tf.b, tf.c, tf.d, tf.e, tf.f] {
        write!(out, " {}", value.to_bits()).unwrap();
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut out;
    if kind == "skcms-tf" || kind == "skcms-tf-in-place" {
        let eval = number::<u8>(args) != 0;
        let mut tf = read_tf(args);
        let x = read(args);
        let mut dst = read_tf(args);
        out = format!("{}", tf.get_type() as i32);
        if eval {
            write!(out, " {}", tf.eval(x).to_bits()).unwrap();
        }
        if kind == "skcms-tf-in-place" {
            write!(out, " {}", u8::from(tf.invert_in_place())).unwrap();
            write_tf(&mut out, tf);
        } else {
            write!(out, " {}", u8::from(tf.invert_into(&mut dst))).unwrap();
            write_tf(&mut out, dst);
        }
        return out;
    }
    let which: u8 = number(args);
    let p = read_tf(args);
    let mut dst = read_tf(args);
    out = match which {
        0 => u8::from(dst.make_pqish(p.g, p.a, p.b, p.c, p.d, p.e)).to_string(),
        1 => u8::from(dst.make_scaled_hlgish(p.g, p.a, p.b, p.c, p.d, p.e)).to_string(),
        2 => u8::from(dst.make_hlgish(p.g, p.a, p.b, p.c, p.d)).to_string(),
        3 => {
            dst.make_pq(p.g);
            "V".to_owned()
        }
        4 => {
            dst.make_hlg(p.g, p.a, p.b);
            "V".to_owned()
        }
        _ => panic!("source constructor domain"),
    };
    write_tf(&mut out, dst);
    write!(
        out,
        " {} {} {} {} {} {}",
        dst.get_type() as i32,
        u8::from(dst.is_srgbish()),
        u8::from(dst.is_pqish()),
        u8::from(dst.is_hlgish()),
        u8::from(dst.is_pq()),
        u8::from(dst.is_hlg())
    )
    .unwrap();
    out
}

fn exact(requests: Vec<String>) {
    if let Some(path) = std::env::var_os("JXL_SKCMS_TRANSFER_CAPTURE") {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _lock = LOCK.lock().unwrap();
        let mut output = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        output
            .write_all((requests.join("\n") + "\n").as_bytes())
            .unwrap();
    }
    compare(requests);
}

const PROFILES: [[u32; 7]; 8] = [
    [1065353216, 1065353216, 0, 1065353216, 0, 0, 0],
    [
        1075419546, 1064478574, 1029015833, 1033798545, 1025879782, 0, 0,
    ],
    [1074580685, 1065353216, 0, 0, 0, 0, 0],
    [
        3221225472, 3210084352, 1065353216, 1011862956, 1100402688, 3247800320, 1086906475,
    ],
    [
        3225419776, 1073741824, 1073741824, 1085468713, 1049739296, 1057969743, 0,
    ],
    [
        3229614080, 1056964608, 1056964608, 1043800048, 1049739296, 1057969743, 0,
    ],
    [3231711232, 1128988672, 0, 0, 0, 0, 0],
    [3233808384, 1128988672, 1148846080, 1067030938, 0, 0, 0],
];
fn wire(tf: &[u32; 7], x: f32) -> String {
    let mut request = "skcms-tf 1".to_owned();
    for word in tf {
        write!(request, " {word}").unwrap();
    }
    write!(request, " {}", x.to_bits()).unwrap();
    for i in 0..7 {
        write!(request, " {}", 0x3f000001u32 + i).unwrap();
    }
    request
}
#[test]
fn srgb_pq_hlg_and_inverse_functions_preserve_every_float_word() {
    let mut requests = Vec::new();
    for tf in PROFILES {
        for i in -4096..=4096 {
            requests.push(wire(&tf, i as f32 / 1024.0));
        }
        for x in [0.0, -0.0, f32::MIN_POSITIVE, f32::from_bits(1)] {
            requests.push(wire(&tf, x));
        }
    }
    assert_eq!(requests.len(), 65576);
    exact(requests);
}
#[test]
fn original_classification_and_failed_inverse_keep_destination_partial_state() {
    let requests: Vec<_> = include_str!("../data/skcms-transfer-classification.requests")
        .lines()
        .map(|row| format!("skcms-tf {row}"))
        .collect();
    assert_eq!(requests.len(), 16000);
    exact(requests);
}
#[test]
fn all_original_constructors_and_type_predicates_keep_raw_parameters() {
    let mut requests = Vec::new();
    let mut seed = 190877;
    let special = [
        0, 0x80000000, 1, 0x80000001, 0x3f800000, 0xbf800000, 0x7f800000, 0xff800000, 0x7fc12345,
        0x7f812345,
    ];
    for i in 0..1024 {
        for which in 0..5 {
            let mut request = format!("skcms-make {which}");
            for field in 0..14 {
                let word = if i < special.len() {
                    special[(i + field) % special.len()]
                } else {
                    next(&mut seed) as u32
                };
                write!(request, " {word}").unwrap();
            }
            requests.push(request);
        }
    }
    exact(requests);
}

#[test]
fn exceptional_hdr_parameters_preserve_all_defined_source_words() {
    let requests: Vec<_> = include_str!("../data/skcms-transfer-exceptional.tsv")
        .lines()
        .filter_map(|row| row.strip_prefix("D\t").map(str::to_owned))
        .collect();
    assert_eq!(requests.len(), 8262);
    exact(requests);
}

#[test]
fn in_place_inversion_preserves_actual_original_aliasing_and_failed_state() {
    let mut requests: Vec<_> = include_str!("../data/skcms-transfer-classification.requests")
        .lines()
        .map(|row| format!("skcms-tf-in-place {row}"))
        .collect();
    for row in include_str!("../data/skcms-transfer-exceptional.tsv").lines() {
        if let Some(request) = row.strip_prefix("D\tskcms-tf ") {
            requests.push(format!("skcms-tf-in-place {request}"));
        }
    }
    for tf in PROFILES {
        for x in [-4.0, -0.0, 0.03125, 0.5, 1.0, 4.0] {
            requests.push(wire(&tf, x).replacen("skcms-tf ", "skcms-tf-in-place ", 1));
        }
    }
    assert_eq!(requests.len(), 24310);
    exact(requests);
}

#[test]
fn original_nan_to_integer_boundaries_retain_positive_source_diagnostics() {
    use std::io::Write as _;
    let oracle = std::env::var_os("JXL_CPP_ORACLE").expect("actual C++ source observer");
    let sanitized = oracle.to_string_lossy().contains("asan");
    let requests: Vec<_> = include_str!("../data/skcms-transfer-exceptional.tsv")
        .lines()
        .filter_map(|row| row.strip_prefix("U\t"))
        .collect();
    assert_eq!(requests.len(), 362);
    for (index, request) in requests.iter().enumerate() {
        let mut child = Command::new(&oracle)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        {
            let mut stdin = child.stdin.take().unwrap();
            writeln!(stdin, "{request}").unwrap();
        }
        let output = child.wait_with_output().unwrap();
        let diagnostic = String::from_utf8_lossy(&output.stderr);
        if sanitized {
            assert!(
                !output.status.success(),
                "source boundary {index}: {request}"
            );
            assert!(
                diagnostic.contains("runtime error:")
                    && diagnostic.contains("skcms_internals.h:")
                    && diagnostic.contains("outside the range of representable values"),
                "source boundary {index}: {diagnostic}"
            );
        }
        // These original-only observations have no defined Rust output to
        // compare. Keep every input and the actual diagnostic in audit runs.
        if let Some(path) = std::env::var_os("JXL_SKCMS_TRANSFER_PROBES") {
            let directory = PathBuf::from(path);
            std::fs::create_dir_all(&directory).unwrap();
            std::fs::write(
                directory.join(format!("{index:03}.request")),
                format!("{request}\n"),
            )
            .unwrap();
            std::fs::write(directory.join(format!("{index:03}.stdout")), &output.stdout).unwrap();
            std::fs::write(directory.join(format!("{index:03}.stderr")), &output.stderr).unwrap();
        }
    }
    println!("362 original-only skcms float-to-int boundaries recorded; sanitized={sanitized}");
}
