// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
use super::*;
use jxl::skcms::{matrix, types::Matrix3x3};
const COMMANDS: [&str; 4] = [
    "skcms-concat",
    "skcms-invert",
    "skcms-adapt",
    "skcms-primaries",
];
fn read(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(number(args))
}
fn read_matrix(args: &mut std::str::SplitWhitespace<'_>) -> Matrix3x3 {
    let mut matrix = Matrix3x3::default();
    for row in &mut matrix.vals {
        for value in row {
            *value = read(args);
        }
    }
    matrix
}
pub(super) fn handles(kind: &str) -> bool {
    COMMANDS.contains(&kind)
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let flag = number::<u8>(args) != 0;
    let (status, matrix) = match kind {
        "skcms-concat" => {
            let a = read_matrix(args);
            let b = read_matrix(args);
            ("V".to_owned(), a.concat(if flag { &a } else { &b }))
        }
        "skcms-invert" => {
            let mut source = read_matrix(args);
            let mut destination = read_matrix(args);
            if flag {
                let ok = source.invert_in_place();
                (u8::from(ok).to_string(), source)
            } else {
                let ok = source.invert_into(&mut destination);
                (u8::from(ok).to_string(), destination)
            }
        }
        "skcms-adapt" => {
            let wx = read(args);
            let wy = read(args);
            let mut destination = read_matrix(args);
            let ok =
                matrix::adapt_to_xyz_d50(wx, wy, if flag { None } else { Some(&mut destination) });
            (u8::from(ok).to_string(), destination)
        }
        "skcms-primaries" => {
            let mut xy = [0.0; 8];
            for value in &mut xy {
                *value = read(args);
            }
            let mut destination = read_matrix(args);
            let ok = matrix::primaries_to_xyz_d50(
                xy[0],
                xy[1],
                xy[2],
                xy[3],
                xy[4],
                xy[5],
                xy[6],
                xy[7],
                if flag { None } else { Some(&mut destination) },
            );
            (u8::from(ok).to_string(), destination)
        }
        _ => panic!("original matrix command"),
    };
    let mut output = status;
    for row in matrix.vals {
        for value in row {
            write!(output, " {}", value.to_bits()).unwrap();
        }
    }
    output
}
fn retained_requests(command: u8) -> Vec<String> {
    let mut bytes: &[u8] = include_bytes!("../data/skcms-matrix-requests.bin");
    let mut requests = Vec::new();
    let mut total = 0;
    while !bytes.is_empty() {
        let tag = bytes[0];
        let flag = bytes[1];
        let words = [18, 18, 11, 17][usize::from(tag)];
        let (record, tail) = bytes.split_at(2 + 4 * words);
        bytes = tail;
        total += 1;
        if tag != command {
            continue;
        }
        let mut request = format!("{} {flag}", COMMANDS[usize::from(tag)]);
        for word in record[2..].chunks_exact(4) {
            write!(request, " {}", u32::from_le_bytes(word.try_into().unwrap())).unwrap();
        }
        requests.push(request);
    }
    assert_eq!(total, 71226);
    requests
}
fn exact(requests: Vec<String>) {
    if let Some(path) = std::env::var_os("JXL_SKCMS_MATRIX_CAPTURE") {
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
#[test]
fn concatenation_keeps_every_source_float_word_and_shared_input_alias() {
    exact(retained_requests(0));
}
#[test]
fn inverse_keeps_partial_failure_destinations_aliases_and_nan_signs() {
    exact(retained_requests(1));
}
#[test]
fn bradford_adaptation_keeps_zero_subnormal_whitepoints_and_null_output() {
    exact(retained_requests(2));
}
#[test]
fn chromatic_primaries_keep_guards_singular_matrices_and_destination_state() {
    exact(retained_requests(3));
}

#[test]
fn paired_exceptional_entries_keep_every_defined_partial_inverse_word() {
    let words: [u32; 15] = [
        0, 0x80000000, 1, 0x80000001, 0x3f000000, 0x3f800000, 0xbf800000, 0x7f7fffff, 0xff7fffff,
        0x7f800000, 0xff800000, 0x7fc12345, 0xffc23456, 0x7f812345, 0xff823456,
    ];
    let mut requests = Vec::new();
    for first in 0..9 {
        for second in first + 1..9 {
            for a in words {
                for b in words {
                    let mut source = [0; 9];
                    source[0] = 0x3f800000;
                    source[4] = 0x3f800000;
                    source[8] = 0x3f800000;
                    source[first] = a;
                    source[second] = b;
                    for alias in 0..2 {
                        let mut request = format!("skcms-invert {alias}");
                        for word in source {
                            write!(request, " {word}").unwrap();
                        }
                        for i in 0..9 {
                            write!(request, " {}", 0x3f000001 + i).unwrap();
                        }
                        requests.push(request);
                    }
                }
            }
        }
    }
    assert_eq!(requests.len(), 16200);
    exact(requests);
}

#[test]
fn pinned_standard_primaries_and_original_upstream_srgb_examples_remain_exact() {
    // skcms tests.c:1366-1414 and JXL cms/color_encoding_cms.h primaries/whitepoints.
    let primaries = [
        [0.64_f32, 0.33, 0.30, 0.60, 0.15, 0.06],
        [0.708, 0.292, 0.170, 0.797, 0.131, 0.046],
        [0.680, 0.320, 0.265, 0.690, 0.150, 0.060],
    ];
    let whitepoints = [[0.3127_f32, 0.3290], [1.0 / 3.0, 1.0 / 3.0], [0.314, 0.351]];
    let mut requests = Vec::new();
    for rgb in primaries {
        for xy in whitepoints {
            for null in 0..2 {
                let mut request = format!("skcms-primaries {null}");
                for value in rgb.into_iter().chain(xy) {
                    write!(request, " {}", value.to_bits()).unwrap();
                }
                for i in 0..9 {
                    write!(request, " {}", 0x3f000001 + i).unwrap();
                }
                requests.push(request);
            }
        }
    }
    exact(requests);
}
