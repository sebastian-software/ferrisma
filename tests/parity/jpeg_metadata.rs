// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::jpeg::jpeg_data::*;
use jxl::jxl::jpeg::jpeg_data_fields::set_jpeg_data_from_icc;

pub(super) fn handles(kind: &str) -> bool {
    kind == "jm-ops"
}
fn byte_vectors(args: &mut std::str::SplitWhitespace<'_>) -> Vec<Vec<u8>> {
    let n: usize = number(args);
    (0..n).map(|_| unhex(args.next().unwrap())).collect()
}
fn bytes(out: &mut String, v: &[u8]) {
    write!(out, " {}", hex(v)).unwrap();
}
fn vectors(out: &mut String, v: &[Vec<u8>]) {
    write!(out, " {}", v.len()).unwrap();
    for x in v {
        bytes(out, x);
    }
}
fn read_data(args: &mut std::str::SplitWhitespace<'_>) -> JpegData {
    let mut j = JpegData {
        width: number(args),
        height: number(args),
        restart_interval: number(args),
        app_data: byte_vectors(args),
        ..Default::default()
    };
    let n: usize = number(args);
    j.app_marker_type = (0..n).map(|_| AppMarkerType(number(args))).collect();
    j.com_data = byte_vectors(args);
    let n: usize = number(args);
    j.quant = (0..n)
        .map(|_| JpegQuantTable {
            precision: number(args),
            index: number(args),
            is_last: number::<u32>(args) != 0,
            values: std::array::from_fn(|_| number(args)),
        })
        .collect();
    let n: usize = number(args);
    j.huffman_code = (0..n)
        .map(|_| JpegHuffmanCode {
            slot_id: number(args),
            is_last: number::<u32>(args) != 0,
            counts: std::array::from_fn(|_| number(args)),
            values: std::array::from_fn(|_| number(args)),
        })
        .collect();
    let n: usize = number(args);
    j.components = (0..n)
        .map(|_| {
            let mut c = JpegComponent {
                id: number(args),
                h_samp_factor: number(args),
                v_samp_factor: number(args),
                quant_idx: number(args),
                width_in_blocks: number(args),
                height_in_blocks: number(args),
                ..Default::default()
            };
            let n: usize = number(args);
            c.coeffs = (0..n).map(|_| number(args)).collect();
            c
        })
        .collect();
    let n: usize = number(args);
    j.scan_info = (0..n)
        .map(|_| {
            let mut s = JpegScanInfo {
                ss: number(args),
                se: number(args),
                ah: number(args),
                al: number(args),
                num_components: number(args),
                last_needed_pass: number(args),
                components: std::array::from_fn(|_| JpegComponentScanInfo {
                    comp_idx: number(args),
                    dc_tbl_idx: number(args),
                    ac_tbl_idx: number(args),
                }),
                ..Default::default()
            };
            let n: usize = number(args);
            s.reset_points = (0..n).map(|_| number(args)).collect();
            let n: usize = number(args);
            s.extra_zero_runs = (0..n)
                .map(|_| ExtraZeroRunInfo {
                    block_idx: number(args),
                    num_extra_zero_runs: number(args),
                })
                .collect();
            s
        })
        .collect();
    j.marker_order = unhex(args.next().unwrap());
    j.inter_marker_data = byte_vectors(args);
    j.tail_data = unhex(args.next().unwrap());
    j.has_zero_padding_bit = number::<u32>(args) != 0;
    j.padding_bits = unhex(args.next().unwrap());
    j
}
fn data(out: &mut String, j: &JpegData) {
    write!(out, " {} {} {}", j.width, j.height, j.restart_interval).unwrap();
    vectors(out, &j.app_data);
    write!(out, " {}", j.app_marker_type.len()).unwrap();
    for x in &j.app_marker_type {
        write!(out, " {}", x.0).unwrap();
    }
    vectors(out, &j.com_data);
    write!(out, " {}", j.quant.len()).unwrap();
    for q in &j.quant {
        write!(out, " {} {} {}", q.precision, q.index, u32::from(q.is_last)).unwrap();
        for x in q.values {
            write!(out, " {x}").unwrap();
        }
    }
    write!(out, " {}", j.huffman_code.len()).unwrap();
    for h in &j.huffman_code {
        write!(out, " {} {}", h.slot_id, u32::from(h.is_last)).unwrap();
        for x in h.counts {
            write!(out, " {x}").unwrap();
        }
        for x in h.values {
            write!(out, " {x}").unwrap();
        }
    }
    write!(out, " {}", j.components.len()).unwrap();
    for c in &j.components {
        write!(
            out,
            " {} {} {} {} {} {} {}",
            c.id,
            c.h_samp_factor,
            c.v_samp_factor,
            c.quant_idx,
            c.width_in_blocks,
            c.height_in_blocks,
            c.coeffs.len()
        )
        .unwrap();
        for x in &c.coeffs {
            write!(out, " {x}").unwrap();
        }
    }
    write!(out, " {}", j.scan_info.len()).unwrap();
    for s in &j.scan_info {
        write!(
            out,
            " {} {} {} {} {} {}",
            s.ss, s.se, s.ah, s.al, s.num_components, s.last_needed_pass
        )
        .unwrap();
        for c in s.components {
            write!(out, " {} {} {}", c.comp_idx, c.dc_tbl_idx, c.ac_tbl_idx).unwrap();
        }
        write!(out, " {}", s.reset_points.len()).unwrap();
        for x in &s.reset_points {
            write!(out, " {x}").unwrap();
        }
        write!(out, " {}", s.extra_zero_runs.len()).unwrap();
        for x in &s.extra_zero_runs {
            write!(out, " {} {}", x.block_idx, x.num_extra_zero_runs).unwrap();
        }
    }
    bytes(out, &j.marker_order);
    vectors(out, &j.inter_marker_data);
    bytes(out, &j.tail_data);
    write!(out, " {}", u32::from(j.has_zero_padding_bit)).unwrap();
    bytes(out, &j.padding_bits);
}
pub(super) fn observe(_: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut j = read_data(args);
    let n: usize = number(args);
    let mut out = String::new();
    for _ in 0..n {
        let op = args.next().unwrap();
        match op {
            "W" => {
                let offset = number(args);
                let mut w = BitWriter::new();
                prefix(&mut w, offset);
                let s =
                    jxl::jxl::enc_fields::bundle::write(&mut j, &mut w, LayerType::Header, None);
                let bits = w.bits_written();
                w.zero_pad_to_byte();
                write!(out, " W {} {bits} {}", status_code(&s), hex(w.get_span())).unwrap();
            }
            "C" => {
                let s = bundle::can_encode(&mut j);
                let (ext, total) = s.as_ref().copied().unwrap_or((0, 0));
                write!(out, " C {} {ext} {total}", status_code(&s.map(|_| ()))).unwrap();
            }
            "D" => {
                let offset = number(args);
                let b = unhex(args.next().unwrap());
                let mut r = BitReader::new(&b);
                r.skip_bits(offset);
                let s = bundle::read(&mut r, &mut j);
                let bits = r.total_bits_consumed();
                write!(
                    out,
                    " D {} {bits} {}",
                    status_code(&s),
                    status_code(&r.close())
                )
                .unwrap();
            }
            "B" => {
                let encoded = unhex(args.next().unwrap());
                let result = jxl::jxl::jpeg::dec_jpeg_data::decode_jpeg_data(&encoded, &mut j);
                write!(out, " B {}", status_code(&result)).unwrap();
            }
            "I" => {
                let b = unhex(args.next().unwrap());
                write!(
                    out,
                    " I {}",
                    status_code(&set_jpeg_data_from_icc(&b, &mut j))
                )
                .unwrap();
            }
            _ => panic!("unknown JPEG metadata operation"),
        }
        out.push_str(" S");
        data(&mut out, &j);
    }
    out
}
fn request(j: &JpegData, ops: &str, n: usize) -> String {
    let mut s = "jm-ops".to_owned();
    data(&mut s, j);
    write!(s, " {n} {ops}").unwrap();
    s
}
fn fixture(name: &str) -> JpegData {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/jpeg-metadata")
        .join(name);
    let capture =
        original_observations(&[format!("jm-fixture {}", hex(&std::fs::read(path).unwrap()))]);
    read_data(&mut capture[0].split_whitespace())
}
fn encoded(j: &JpegData, offset: usize) -> Vec<u8> {
    let capture = original_observations(&[request(j, &format!("W {offset}"), 1)]);
    let mut words = capture[0].split_whitespace();
    assert_eq!(words.next(), Some("W"));
    assert_eq!(words.next(), Some("0"), "original writer rejected fixture");
    let _: usize = number(&mut words);
    unhex(words.next().unwrap())
}
fn empty() -> JpegData {
    JpegData::default()
}
#[test]
fn jpeg_metadata_source_written_genuine_fixtures_and_unaligned_reads() {
    let mut cases = Vec::new();
    for name in [
        "baseline.jpg",
        "gray.jpg",
        "progressive.jpg",
        "restarts.jpg",
    ] {
        let j = fixture(name);
        for offset in 0..8 {
            let b = encoded(&j, offset);
            cases.push(request(&j, &format!("C W {offset}"), 2));
            cases.push(request(&empty(), &format!("D {offset} {} C", hex(&b)), 2));
        }
    }
    compare(cases);
}
#[test]
fn jpeg_metadata_truncation_mutation_and_defined_partial_state() {
    let mut cases = Vec::new();
    for name in ["gray.jpg", "progressive.jpg"] {
        let j = fixture(name);
        for offset in [0, 3, 7] {
            let b = encoded(&j, offset);
            for len in 0..=b.len() {
                cases.push(request(
                    &empty(),
                    &format!("D {offset} {}", hex(&b[..len])),
                    1,
                ));
            }
            // Preserve partial existing owner state and unused quant-table comparisons.
            for len in [0, 1, 2, b.len() / 3, b.len() / 2, b.len().saturating_sub(1)] {
                let mut initial = j.clone();
                initial.marker_order.clear();
                initial.inter_marker_data = vec![vec![0xa5; 3]];
                initial.padding_bits = vec![1, 0, 1];
                cases.push(request(
                    &initial,
                    &format!("D {offset} {}", hex(&b[..len])),
                    1,
                ));
            }
            for i in 0..b.len() {
                for mask in [1, 0x80, 0xff] {
                    let mut changed = b.clone();
                    changed[i] ^= mask;
                    cases.push(request(
                        &empty(),
                        &format!("D {offset} {}", hex(&changed)),
                        1,
                    ));
                }
            }
        }
    }
    // Bounds work is large, but each request still compares every reached field.
    for batch in cases.chunks(100) {
        compare(batch.to_vec());
    }
}
#[test]
fn jpeg_metadata_structured_errors_preserve_mutations_and_validation_order() {
    let base = fixture("baseline.jpg");
    let mut cases = Vec::new();
    let mut add = |j: JpegData| {
        cases.push(request(&j, "C W 3", 2));
    };
    for marker in 0..=255 {
        let mut j = base.clone();
        j.marker_order[0] = marker;
        add(j);
    }
    for value in [0, 1, 2, 3, 4, 7, 8, u32::MAX] {
        let mut j = base.clone();
        j.app_marker_type[0] = AppMarkerType(value);
        add(j);
    }
    for len in [0, 1, 2, 3, 17, 65536, 65537] {
        let mut j = base.clone();
        j.app_data[0].resize(len, 0x51);
        add(j);
    }
    for len in [0, 1, 2, 3, 4] {
        let mut j = base.clone();
        j.quant.resize(len, JpegQuantTable::default());
        add(j);
    }
    for value in [0, 1, 2, u32::MAX] {
        let mut j = base.clone();
        j.quant[0].precision = value;
        add(j);
        let mut j = base.clone();
        j.components[0].quant_idx = value;
        add(j);
    }
    for value in [0, 1, 2, 3, 4, 11, 12, 255, 256, 257] {
        let mut j = base.clone();
        j.huffman_code[0].values[0] = value;
        add(j);
    }
    for value in [0, 1, 2, 3, 4] {
        let mut j = base.clone();
        j.scan_info[0].num_components = value;
        add(j);
    }
    for value in [0, 1, 2, 3, 4] {
        let mut j = base.clone();
        j.scan_info[0].components[0].comp_idx = value;
        add(j);
    }
    for value in [0, 1, 2, 3, 4, 5, 19, 20, 21, 255, 275] {
        let mut j = base.clone();
        j.scan_info[0].extra_zero_runs = vec![ExtraZeroRunInfo {
            block_idx: 0,
            num_extra_zero_runs: value,
        }];
        add(j);
    }
    for value in [
        0,
        1,
        7,
        8,
        9,
        40,
        41,
        (3 << 26) - 1,
        3 << 26,
        (3 << 26) + 1,
        u32::MAX,
    ] {
        let mut j = base.clone();
        j.scan_info[0].reset_points = vec![value];
        add(j);
        let mut j = base.clone();
        j.scan_info[0].extra_zero_runs = vec![ExtraZeroRunInfo {
            block_idx: value,
            num_extra_zero_runs: 1,
        }];
        add(j);
    }
    for value in [0, 1, 255, 256, 257, 65792, 65793, 4260096, 4260097] {
        let mut j = base.clone();
        j.tail_data.resize(value, 0x5a);
        add(j);
    }
    for value in [0, 1, 2, 255] {
        let mut j = base.clone();
        j.has_zero_padding_bit = true;
        j.padding_bits = vec![value; 19];
        add(j);
    }
    for n in [0, 1, 16383, 16384, 16385] {
        let mut j = base.clone();
        j.marker_order.splice(0..0, std::iter::repeat_n(0xc0, n));
        add(j);
    }
    for batch in cases.chunks(32) {
        compare(batch.to_vec());
    }
}
#[test]
fn jpeg_metadata_icc_injection_preserves_prior_writes_on_error() {
    let mut cases = Vec::new();
    for lens in [
        vec![],
        vec![0],
        vec![16],
        vec![17],
        vec![18],
        vec![18, 18],
        vec![20, 16],
        vec![18, 20],
    ] {
        for len in 0..=8 {
            for unknown in [false, true] {
                let mut j = empty();
                j.app_data = lens.iter().map(|n| vec![0xa5; *n]).collect();
                j.app_marker_type = vec![
                    if unknown {
                        AppMarkerType::UNKNOWN
                    } else {
                        AppMarkerType::ICC
                    };
                    lens.len()
                ];
                let b: Vec<u8> = (0..len).map(|i| i as u8).collect();
                cases.push(request(&j, &format!("I {} I -", hex(&b)), 2));
            }
        }
    }
    compare(cases);
}

#[test]
fn jpeg_metadata_all_serialized_fields_and_source_validation_branches() {
    let base = fixture("baseline.jpg");
    let mut variants = Vec::new();
    for ids in [[1, 2, 3], [82, 71, 66], [0, 128, 255]] {
        let mut j = base.clone();
        for (c, id) in j.components.iter_mut().zip(ids) {
            c.id = id;
        }
        variants.push(j);
    }
    for id in [0, 1, 82, 255, 256] {
        let mut j = fixture("gray.jpg");
        j.components[0].id = id;
        variants.push(j);
    }
    for n in [0, 1, 2, 3, 4] {
        let mut j = base.clone();
        j.components.resize(n, JpegComponent::default());
        variants.push(j);
    }
    for kind in 0..=3 {
        let mut j = base.clone();
        j.app_marker_type.fill(AppMarkerType(kind));
        variants.push(j);
    }
    for n in [0, 1, 2, 3, 65535, 65536, 65537] {
        let mut j = base.clone();
        j.com_data = vec![vec![0x5a; n]];
        j.marker_order.insert(0, 0xfe);
        variants.push(j);
    }
    for n in [0, 1, 2, 65535, 65536] {
        let mut j = base.clone();
        j.inter_marker_data = vec![vec![0xa5; n], vec![1, 2, 3]];
        j.marker_order.insert(0, 0xff);
        j.marker_order.insert(1, 0xff);
        variants.push(j);
    }
    for interval in [0, 1, 65535, 65536, u32::MAX] {
        let mut j = base.clone();
        j.marker_order.insert(0, 0xdd);
        j.restart_interval = interval;
        variants.push(j);
    }
    for unequal in [false, true] {
        let mut j = base.clone();
        let mut q = j.quant[1].clone();
        q.index = 2;
        if unequal {
            q.values[17] ^= 1;
        }
        j.quant.push(q);
        variants.push(j);
    }
    let mut j = base.clone();
    for c in &mut j.components {
        c.quant_idx = 1;
    }
    variants.push(j);
    for n in [0, 1, 2, 3, 4, 9, 10, 25, 26, 89, 90] {
        let mut j = base.clone();
        j.huffman_code.resize(n, JpegHuffmanCode::default());
        variants.push(j);
    }
    // Reached slots 4..15 are source out-of-bounds before CanEncode reports
    // its accumulated bit-range error; the native sanitizer receipt is recorded
    // in the metadata fixture provenance. Keep defined slot aliases.
    for slot in [0, 1, 3, 16, 17, 19] {
        let mut j = base.clone();
        j.huffman_code[0].slot_id = slot;
        variants.push(j);
    }
    for counts in [vec![], vec![1], vec![2], vec![257], vec![255, 3]] {
        let mut j = base.clone();
        let h = &mut j.huffman_code[0];
        h.counts.fill(0);
        for (i, v) in counts.iter().enumerate() {
            h.counts[i] = *v;
        }
        variants.push(j);
    }
    for values in [
        vec![256],
        vec![0, 256],
        vec![0, 0, 256],
        vec![12, 256],
        vec![255, 256],
        vec![256, 256],
        vec![0, 1],
        vec![257],
    ] {
        let mut j = base.clone();
        let h = &mut j.huffman_code[0];
        h.counts.fill(0);
        h.counts[1] = values.len() as u32;
        for (i, v) in values.iter().enumerate() {
            h.values[i] = *v;
        }
        variants.push(j);
    }
    for value in [0, 1, 3, 4, 15, 16, 63, 64, u32::MAX] {
        for field in 0..7 {
            if field >= 5 && value >= 4 {
                continue;
            }
            let mut j = base.clone();
            let s = &mut j.scan_info[0];
            match field {
                0 => s.ss = value,
                1 => s.se = value,
                2 => s.ah = value,
                3 => s.al = value,
                4 => s.last_needed_pass = value,
                5 => s.components[0].ac_tbl_idx = value,
                _ => s.components[0].dc_tbl_idx = value,
            };
            variants.push(j);
        }
    }
    for ids in [
        vec![],
        vec![0],
        vec![0, 0],
        vec![1, 0],
        vec![0, 1, 8, 40, 41, 1000],
        vec![(3 << 26) - 1, 3 << 26],
    ] {
        let mut j = base.clone();
        j.scan_info[0].reset_points = ids.clone();
        variants.push(j);
        let mut j = base.clone();
        j.scan_info[0].extra_zero_runs = ids
            .into_iter()
            .map(|block_idx| ExtraZeroRunInfo {
                block_idx,
                num_extra_zero_runs: 1,
            })
            .collect();
        variants.push(j);
    }
    for len in [16383, 16384, 16385] {
        let mut j = base.clone();
        j.marker_order
            .splice(0..0, std::iter::repeat_n(0xc0, len - j.marker_order.len()));
        variants.push(j);
    }
    for n in [0, 1, 7, 8, 1023, 1024, 1025] {
        let mut j = base.clone();
        j.has_zero_padding_bit = true;
        j.padding_bits = (0..n).map(|i| (i % 3) as u8).collect();
        variants.push(j);
    }
    let mut cases = Vec::new();
    for j in variants {
        cases.push(request(&j, "C W 7", 2));
        // Read source-written successes into a fresh owner, including deferred
        // tail/intermarker allocation and padding-bit append checkpoints.
        let observed = original_observations(&[request(&j, "W 7", 1)]);
        let mut args = observed[0].split_whitespace();
        assert_eq!(args.next(), Some("W"));
        let result: i32 = number(&mut args);
        let _: usize = number(&mut args);
        let b = unhex(args.next().unwrap());
        if result == 0 {
            cases.push(request(&empty(), &format!("D 7 {}", hex(&b)), 1));
            let mut initial = j.clone();
            initial.marker_order.clear();
            initial.inter_marker_data = vec![vec![0xa5; 3]];
            initial.tail_data = vec![0x5a; 7];
            initial.padding_bits = vec![1, 0, 1];
            cases.push(request(&initial, &format!("D 7 {}", hex(&b)), 1));
        }
    }
    for batch in cases.chunks(32) {
        compare(batch.to_vec());
    }
}

#[test]
fn jpeg_metadata_complete_brotli_payloads_keep_truncations_and_repeated_owner_state() {
    let mut cases = Vec::new();
    let mut undefined = Vec::new();
    for name in [
        "baseline.jpg",
        "gray.jpg",
        "progressive.jpg",
        "restarts.jpg",
    ] {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data/jpeg-metadata")
            .join(name);
        let raw = std::fs::read(path).unwrap();
        for effort in [0, 4, 11] {
            let source =
                original_observations(&[format!("jm-packed-fixture {} {effort}", hex(&raw))]);
            let mut words = source[0].split_whitespace();
            assert_eq!(words.next(), Some("P"));
            assert_eq!(words.next(), Some("0"));
            let encoded = unhex(words.next().unwrap());
            for length in 0..=encoded.len() {
                cases.push(request(
                    &empty(),
                    &format!("B {}", hex(&encoded[..length])),
                    1,
                ));
            }
            for trailing in [vec![0], vec![255], encoded.clone()] {
                let mut extra = encoded.clone();
                extra.extend(trailing);
                cases.push(request(&empty(), &format!("B {}", hex(&extra)), 1));
            }
            let lengths = [0, 1, encoded.len() / 2, encoded.len().saturating_sub(1)];
            let first: Vec<_> = lengths
                .iter()
                .map(|length| request(&empty(), &format!("B {}", hex(&encoded[..*length])), 1))
                .collect();
            for (length, observed) in lengths.iter().zip(original_observations(&first)) {
                let repeated = request(
                    &empty(),
                    &format!("B {} B {}", hex(&encoded[..*length]), hex(&encoded)),
                    2,
                );
                // VisitFields appends to the retained marker_order but resizes
                // scan_info to the new scan count. A retained SOS marker makes
                // the source table validation read scan_info[scan_info.size()].
                let stale_scans = stale_scan_markers(&observed);
                if stale_scans == 0 {
                    cases.push(repeated);
                } else {
                    undefined.push((repeated, stale_scans));
                }
            }
        }
    }
    if let Some(path) = std::env::var_os("JXL_JPEG_PAYLOAD_CAPTURE") {
        std::fs::write(path, cases.join("\n") + "\n").unwrap();
    }
    if let Some(path) = std::env::var_os("JXL_JPEG_PAYLOAD_UNDEFINED_CAPTURE") {
        let requests: Vec<_> = undefined
            .iter()
            .map(|(request, scans)| format!("{scans} {request}"))
            .collect();
        std::fs::write(path, requests.join("\n") + "\n").unwrap();
    }
    compare(cases);
    assert_eq!(undefined.len(), 24);
    let instrumented = original_observations(&["patch-sanitizers".into()])[0] == "1";
    let oracle = std::env::var_os("JXL_CPP_ORACLE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/oracle/jxl_oracle")
        });
    let mut diagnosed = 0;
    for (request, scans) in &undefined {
        let mut child = Command::new(&oracle)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("start original UB boundary probe");
        writeln!(child.stdin.take().unwrap(), "{request}").expect("write exact UB request");
        let output = child
            .wait_with_output()
            .expect("observe original UB boundary");
        let stderr = String::from_utf8_lossy(&output.stderr);
        // The READ lands 16 bytes after the retained scan_info allocation of
        // 120-byte JPEGScanInfo elements: num_components one past its end.
        // Frame names are checked whenever the report is symbolized.
        let frame = stderr
            .lines()
            .find(|line| line.trim_start().starts_with("#0 "))
            .unwrap_or_default();
        let stale_scan_read = stderr.contains("ERROR: AddressSanitizer: heap-buffer-overflow ")
            && stderr.contains("READ of size 4 ")
            && stderr.contains(&format!(
                "is located 16 bytes after {}-byte region",
                scans * 120
            ))
            && (!frame.contains(" in ")
                || (frame.contains("jxl::jpeg::JPEGData::VisitFields")
                    && stderr.contains("vector<jxl::jpeg::JPEGScanInfo")))
            && stderr.contains("SUMMARY: AddressSanitizer: heap-buffer-overflow");
        if instrumented {
            assert!(
                stale_scan_read && !output.status.success(),
                "instrumented source must diagnose the stale scan_info read: {request}\n{stderr}"
            );
            diagnosed += 1;
        } else {
            println!(
                "Original ordinary stale-scan request (undefined, not compared): {}",
                output.status
            );
        }
        // No C++/Rust comparison or codec status is assigned to this source
        // call. Safe Rust indexing panics instead of reading past scan_info.
        assert!(
            std::panic::catch_unwind(|| super::observe(request)).is_err(),
            "Rust must not read past the retained scan_info: {request}"
        );
    }
    println!(
        "{} original stale-scan owner requests exercised; {diagnosed} diagnosed",
        undefined.len()
    );
}

fn stale_scan_markers(observed: &str) -> usize {
    let mut words = observed.split_whitespace();
    assert_eq!(words.next(), Some("B"));
    let _: i32 = number(&mut words);
    assert_eq!(words.next(), Some("S"));
    let j = read_data(&mut words);
    j.marker_order
        .iter()
        .filter(|marker| **marker == 0xda)
        .count()
}
