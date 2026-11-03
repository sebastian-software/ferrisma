// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Data-only protocol adapter for the Rust writer and original JPEG fixtures.
use super::*;
use jxl::jxl::jpeg::dec_jpeg_data_writer::write_jpeg;
use jxl::jxl::jpeg::jpeg_data::*;

pub fn handles(command: &str) -> bool {
    matches!(command, "jw-write" | "jw-mcu")
}
pub(super) use super::jpeg_data_protocol::read_data;
fn mutate(j: &mut JpegData, mode: u32, arg: usize) {
    match mode {
        0 => (),
        1 => j.marker_order.clear(),
        2 => j.marker_order[arg] = 0,
        3 => j.components[0].quant_idx = 999,
        4 => j.scan_info[0].components[0].comp_idx = 999,
        5 => j.marker_order.retain(|&m| m != 0xc4),
        6 | 7 => {
            j.has_zero_padding_bit = true;
            j.padding_bits.clear();
            j.padding_bits.resize(arg, if mode == 6 { 2 } else { 0 });
        }
        8 => j.app_data.clear(),
        9 => {
            for q in &mut j.quant {
                q.precision = 1;
            }
        }
        10 => {
            for s in &mut j.scan_info {
                s.reset_points = vec![0, 1, 3];
            }
        }
        11 => {
            j.scan_info[0].extra_zero_runs = vec![ExtraZeroRunInfo {
                block_idx: 0,
                num_extra_zero_runs: arg as u32,
            }]
        }
        12 => {
            j.tail_data.reserve(1);
            j.tail_data.clear();
        }
        13 => {
            j.app_data.insert(0, vec![0xef, 0, 6, 0, 0xff, 0x80, 0x7f]);
            j.com_data.insert(0, vec![0xfe, 0, 4, 0, 0xff]);
            j.inter_marker_data.insert(0, vec![0x12, 0xff, 0, 0x34]);
            j.marker_order.splice(0..0, [0xef, 0xfe, 0xff]);
            j.tail_data = vec![0, 0xff, 1, 2, 0x80];
        }
        15 => j.marker_order = vec![arg as u8],
        16 | 17 => {
            for marker in &mut j.marker_order {
                if matches!(*marker, 0xc0 | 0xc1) {
                    *marker = 0xc2;
                }
            }
            j.scan_info[0].ss = 0;
            j.scan_info[0].se = 63;
            j.scan_info[0].ah = u32::from(mode == 17);
            j.scan_info[0].al = arg as u32;
        }
        _ => panic!("unknown JPEG mutation"),
    }
}
pub fn observe(command: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if command == "jw-mcu" {
        let j = read_data(args);
        let mut out = String::new();
        for scan in &j.scan_info {
            let (x, y) = j.calculate_mcu_size(scan);
            write!(out, " {x} {y}").unwrap();
        }
        return out;
    }
    let cap: usize = number(args);
    let fail: usize = number(args);
    let mode: u32 = number(args);
    let arg: usize = number(args);
    let mut j = read_data(args);
    mutate(&mut j, mode, arg);
    let mut calls = Vec::new();
    let mut output = Vec::new();
    let mut previous = Vec::new();
    let status = write_jpeg(&j, |data| {
        let null = data.is_none();
        let bytes = data.unwrap_or(&[]);
        let n = if calls.len() == fail {
            0
        } else {
            cap.min(bytes.len())
        };
        let offer = if previous.starts_with(bytes) {
            previous.truncate(bytes.len());
            "p".to_owned()
        } else {
            previous.clear();
            previous.extend_from_slice(bytes);
            format!("f{}", hex(bytes))
        };
        calls.push(format!("{} {n} {} {offer}", bytes.len(), u8::from(null)));
        output.extend_from_slice(&bytes[..n]);
        n
    });
    let mut result = format!("{} {}", status_code(&status), calls.len());
    for call in calls {
        write!(result, " {call}").unwrap();
    }
    write!(result, " {}", hex(&output)).unwrap();
    result
}

fn fixtures() -> Vec<String> {
    let requests = jpeg_files()
        .iter()
        .map(|bytes| format!("jw-fixture {}", hex(bytes)))
        .collect::<Vec<_>>();
    original_observations(&requests)
        .into_iter()
        .map(|line| {
            let (status, data) = line
                .split_once(' ')
                .expect("successful actual JPEG parsing");
            assert_eq!(status, "0");
            data.to_owned()
        })
        .collect()
}
fn jpeg_files() -> Vec<&'static [u8]> {
    vec![
        include_bytes!("../data/jpeg-writer/baseline.jpg"),
        include_bytes!("../data/jpeg-writer/progressive.jpg"),
        include_bytes!("../data/jpeg-writer/restarts.jpg"),
        include_bytes!("../data/jpeg-writer/gray.jpg"),
        include_bytes!("../data/jpeg-writer/large-progressive.jpg"),
        include_bytes!("../data/jpeg-writer/chunked-baseline.jpg"),
    ]
}
#[test]
fn jpeg_writer_reconstructs_genuine_original_parsed_files_and_mcu_geometry() {
    let data = fixtures();
    let mut requests = Vec::new();
    for (fixture, bytes) in data.iter().zip(jpeg_files()) {
        let request = format!("jw-write {} {} 0 0 {fixture}", usize::MAX, usize::MAX);
        let observed = original_observations(std::slice::from_ref(&request));
        assert!(observed[0].starts_with("0 "));
        assert!(observed[0].ends_with(&format!(" {}", hex(bytes))));
        if bytes.len() > 60000 {
            let fields = observed[0].split_whitespace().collect::<Vec<_>>();
            let calls = fields[1].parse::<usize>().unwrap();
            let large = (0..calls)
                .filter(|&i| fields[2 + 4 * i].parse::<usize>().unwrap() > 15000)
                .count();
            assert!(
                large >= 2,
                "genuine file must exercise bit-writer buffer swaps"
            );
            assert!(
                (0..calls).any(|i| fields[2 + 4 * i].parse::<usize>().unwrap() > 15000
                    && fields[5 + 4 * i].contains("ff00")),
                "actual entropy byte stuffing"
            );
        }
        requests.push(request);
        requests.push(format!("jw-mcu {fixture}"));
    }
    compare(requests);
}
#[test]
fn jpeg_writer_short_and_failing_callbacks_preserve_exact_offers_and_output() {
    let data = fixtures();
    let mut requests = Vec::new();
    for fixture in data.iter().take(4) {
        for cap in [0, 1, 2, 3, 7, 16, 63, 127, 1024, 16384, usize::MAX] {
            for fail in [0, 1, 2, 3, 5, 11, usize::MAX] {
                requests.push(format!("jw-write {cap} {fail} 0 0 {fixture}"));
            }
        }
    }
    compare(requests);
}
#[test]
fn jpeg_writer_marker_table_padding_and_scan_errors_retain_callback_prefixes() {
    let data = fixtures();
    let mut requests = Vec::new();
    for fixture in data.iter().take(4) {
        for (mode, arg) in [
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (6, 1),
            (6, 7),
            (6, 128),
            (7, 0),
            (7, 1),
            (7, 7),
            (7, 128),
            (8, 0),
            (9, 0),
            (10, 0),
        ] {
            for cap in [1, 7, usize::MAX] {
                requests.push(format!(
                    "jw-write {cap} {} {mode} {arg} {fixture}",
                    usize::MAX
                ));
            }
        }
    }
    // The unoptimized baseline's actual standard AC tables define F0.
    for arg in [0, 1, 2, 3, 2047, 2048, 2052, 4095] {
        requests.push(format!(
            "jw-write {} {} 11 {arg} {}",
            usize::MAX,
            usize::MAX,
            data[0]
        ));
    }
    compare(requests);
}

#[test]
fn jpeg_writer_all_markers_and_borrowed_metadata_empty_storage_match() {
    let data = fixtures();
    let mut requests = Vec::new();
    for marker in 0..256 {
        requests.push(format!(
            "jw-write {} {} 15 {marker} {}",
            usize::MAX,
            usize::MAX,
            data[0]
        ));
    }
    for fixture in &data {
        for mode in [12, 13] {
            for cap in [1, 7, usize::MAX] {
                requests.push(format!("jw-write {cap} {} {mode} 0 {fixture}", usize::MAX));
            }
        }
    }
    compare(requests);
}

#[test]
fn jpeg_writer_progressive_shifts_keep_cpp_int_promotion_through_bit31() {
    let data = fixtures();
    let mut requests = Vec::new();
    // These are source-defined JPEGData structures. Al16..31 is outside field
    // serialization but remains a defined promoted-int shift in WriteJpeg.
    // The actual baseline and gray standard tables define every reached symbol;
    // Ss0 forces each EOB run to flush before another block is accumulated.
    for index in [0, 3] {
        for mode in [16, 17] {
            for al in 0..32 {
                requests.push(format!(
                    "jw-write {} {} {mode} {al} {}",
                    usize::MAX,
                    usize::MAX,
                    data[index]
                ));
            }
        }
    }
    compare(requests);
}
