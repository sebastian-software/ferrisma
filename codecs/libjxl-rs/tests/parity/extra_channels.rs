// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::dec_external_image::{
    convert_float_channels_to_buffer, convert_float_to_buffer, convert_uint_channels_to_buffer,
    convert_uint_to_buffer,
};
use jxl::jxl::dec_frame::SequenceDecoder;
use jxl::jxl::image_metadata::{CodecMetadata, Orientation};
use jxl::jxl::types::JxlEndianness;

pub(super) fn handles(kind: &str) -> bool {
    matches!(kind, "ec-export" | "ec-meta")
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let bytes = unhex(args.next().unwrap());
    if kind == "ec-meta" {
        let mut m = CodecMetadata::default();
        let mut reader = BitReader::new(&bytes);
        let parsed = (|| {
            if reader.read_fixed_bits::<16>() != 0x0aff {
                return Err(StatusCode::GenericError);
            }
            jxl::jxl::headers::read_size_header(&mut reader, &mut m.size)?;
            jxl::jxl::image_metadata::read_image_metadata(&mut reader, &mut m.m)?;
            m.transform_data.nonserialized_xyb_encoded = m.m.xyb_encoded;
            jxl::jxl::fields::bundle::read(&mut reader, &mut m.transform_data)?;
            if m.m.color_encoding.want_icc() {
                return Err(StatusCode::GenericError);
            }
            reader.jump_to_byte_boundary()?;
            reader.all_reads_within_bounds()
        })();
        let consumed = reader.total_bits_consumed();
        let close = reader.close();
        let mut out = format!(
            "{} {consumed} {} T {}",
            status_code(&parsed),
            status_code(&close),
            m.m.extra_channel_info.len()
        );
        for ec in &m.m.extra_channel_info {
            write!(out, " {}", ec.channel_type.0).unwrap();
        }
        metadata::metadata(&mut out, &m.m);
        return out;
    }
    let bits: usize = number(args);
    let floating = number::<u8>(args) != 0;
    let n = number(args);
    let unpremul = number::<u8>(args) != 0;
    let padding: usize = number(args);
    let mut m = CodecMetadata::default();
    let mut reader = BitReader::new(&bytes);
    assert_eq!(reader.read_fixed_bits::<16>(), 0x0aff);
    jxl::jxl::headers::read_size_header(&mut reader, &mut m.size).unwrap();
    jxl::jxl::image_metadata::read_image_metadata(&mut reader, &mut m.m).unwrap();
    m.transform_data.nonserialized_xyb_encoded = m.m.xyb_encoded;
    jxl::jxl::fields::bundle::read(&mut reader, &mut m.transform_data).unwrap();
    assert!(!m.m.color_encoding.want_icc());
    reader.jump_to_byte_boundary().unwrap();
    reader.all_reads_within_bounds().unwrap();
    let mut offset = reader.total_bits_consumed() / 8;
    reader.close().unwrap();
    let mut decoder = SequenceDecoder::new(&m).unwrap();
    loop {
        let frame = decoder.decode_frame(&bytes[offset..], false);
        if let Err(e) = frame.status {
            return format!("D {} {offset}", e as i32);
        }
        offset += decoder.output().decoded_bytes();
        if frame.header.unwrap().is_last {
            break;
        }
    }
    let image = decoder.output();
    let stride = image.xsize() * n * bits.div_ceil(8) + padding;
    let mut output = vec![0xa5; stride * image.ysize() + 7];
    let size = output.len();
    let status = if floating {
        convert_float_to_buffer(
            &image,
            bits,
            n,
            JxlEndianness::LITTLE,
            stride,
            &mut output,
            size,
            Orientation::IDENTITY,
            unpremul,
        )
    } else {
        convert_uint_to_buffer(
            &image,
            bits,
            n,
            JxlEndianness::LITTLE,
            stride,
            &mut output,
            size,
            Orientation::IDENTITY,
            unpremul,
        )
    };
    let mut out = format!(
        "{} {} {} {} X {}",
        status_code(&status),
        u8::from(image.has_alpha()),
        u8::from(image.alpha_is_premultiplied()),
        hex(&output),
        image.extra_channels().len()
    );
    for ec in image.extra_channels() {
        let stride = ec.xsize() * bits.div_ceil(8) + padding;
        let mut output = vec![0xa5; stride * ec.ysize() + 7];
        let size = output.len();
        let status = if floating {
            convert_float_channels_to_buffer(
                &[Some(ec)],
                bits,
                JxlEndianness::LITTLE,
                stride,
                &mut output,
                size,
                Orientation::IDENTITY,
            )
        } else {
            convert_uint_channels_to_buffer(
                &[Some(ec)],
                bits,
                JxlEndianness::LITTLE,
                stride,
                &mut output,
                size,
                Orientation::IDENTITY,
            )
        };
        write!(out, " {} {}", status_code(&status), hex(&output)).unwrap();
    }
    out
}
pub(super) fn fixtures() -> Vec<(String, Vec<u8>)> {
    include_str!("../data/extra-channels.requests")
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let (name, _) = line.split_once('\t').unwrap();
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/data")
                .join(format!("extra-channel-{name}.jxl"));
            (name.to_owned(), std::fs::read(path).unwrap())
        })
        .collect()
}
fn exact(cases: Vec<String>) {
    if let Some(path) = std::env::var_os("JXL_EXTRA_CHANNEL_CAPTURE") {
        static CAPTURE: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _lock = CAPTURE.lock().unwrap();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        file.write_all((cases.join("\n") + "\n").as_bytes())
            .unwrap();
    }
    for chunk in cases.chunks(4) {
        let originals = original_observations(chunk);
        for (request, original) in chunk.iter().zip(originals) {
            let rust = super::observe(request);
            if !super::parity_eq(request, &rust, &original) {
                let a: Vec<_> = rust.split_whitespace().collect();
                let b: Vec<_> = original.split_whitespace().collect();
                let differences: Vec<_> = a
                    .iter()
                    .zip(&b)
                    .enumerate()
                    .filter(|(_, (a, b))| a != b)
                    .take(16)
                    .collect();
                panic!(
                    "request {request}: lengths {}/{}; differences {differences:?}",
                    a.len(),
                    b.len()
                );
            }
        }
    }
    println!("{} C++/Rust differential cases matched", cases.len());
}
#[test]
fn genuine_extra_channel_complete_files_keep_all_initialized_pixels_and_metadata() {
    let fixtures = fixtures();
    let expected: Vec<i32> = include_str!("../data/extra-channels.decode-statuses")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    assert_eq!(fixtures.len(), expected.len());
    let mut cases = Vec::new();
    for ((name, bytes), status) in fixtures.iter().zip(expected) {
        let request = format!("fs-decode {}", hex(bytes));
        let source = original_observations(std::slice::from_ref(&request));
        let words: Vec<_> = source[0].split_whitespace().collect();
        let end = words.iter().rposition(|word| *word == "E").unwrap();
        assert_eq!(words[end + 1].parse::<i32>().unwrap(), status, "{name}");
        cases.push(request);
        cases.push(format!("ec-meta {}", hex(bytes)));
    }
    exact(cases);
}
#[test]
fn associated_and_independent_channels_export_source_words_with_and_without_unpremultiplication() {
    let mut cases = Vec::new();
    let undefined: std::collections::HashSet<_> = half_probes()
        .into_iter()
        .filter(|(_, undefined)| *undefined)
        .map(|(request, _)| request)
        .collect();
    for (_, bytes) in fixtures() {
        for (bits, floating) in [(8, 0), (16, 0), (16, 1), (32, 1)] {
            for channels in 1..=4 {
                for unpremul in 0..=1 {
                    let request = format!(
                        "ec-export {} {bits} {floating} {channels} {unpremul} 3",
                        hex(&bytes)
                    );
                    // Retain all original odd-half requests in the source-only
                    // census below. Only positively diagnosed source UB is
                    // outside the Rust comparison domain; single rows and
                    // decode failures retain their exact original requests.
                    if !undefined.contains(&request) {
                        cases.push(request);
                    }
                    if bits == 16 && floating == 1 {
                        cases.push(format!(
                            "ec-export {} 16 1 {channels} {unpremul} 4",
                            hex(&bytes)
                        ));
                    }
                }
            }
        }
    }
    exact(cases);
}
#[test]
fn extra_channel_truncation_corruption_and_retained_reference_errors_keep_source_partial_state() {
    let mut cases = Vec::new();
    for (_, bytes) in fixtures() {
        let mut cuts: Vec<_> = (0..=bytes.len().min(80)).collect();
        cuts.extend([bytes.len() / 4, bytes.len() / 2, bytes.len() * 3 / 4]);
        cuts.extend((1..=8).map(|n| bytes.len().saturating_sub(n)));
        let ends = original_observations(&[format!("fs-frame-ends {}", hex(&bytes))]);
        let ends: Vec<usize> = ends[0]
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        for &end in ends.iter().skip(1) {
            for advance in [0, 1, 2, 8, 32, 64] {
                cuts.push((end + advance).min(bytes.len()));
            }
        }
        cuts.sort_unstable();
        cuts.dedup();
        for cut in cuts {
            cases.push(format!("fs-decode {}", hex(&bytes[..cut])));
        }
        let mut trailing = bytes.clone();
        trailing.push(0);
        cases.push(format!("fs-decode {}", hex(&trailing)));
        if ends[0] > 1 {
            for frame in 0..ends[0] {
                for remove in [1, 2, 4] {
                    let changed = original_observations(&[format!(
                        "fs-section-size {} {frame} 0 {remove}",
                        hex(&bytes)
                    )]);
                    cases.push(format!("fs-decode {}", changed[0]));
                }
            }
        }
        for index in [bytes.len() / 2, bytes.len() - 2, bytes.len() - 1] {
            for mask in [1, 128] {
                let mut corrupt = bytes.clone();
                corrupt[index] ^= mask;
                cases.push(format!("fs-decode {}", hex(&corrupt)));
            }
        }
    }
    exact(cases);
}
#[test]
fn original_extra_channel_encoder_reproduces_each_genuine_fixture() {
    let requests: Vec<_> = include_str!("../data/extra-channels.requests")
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| line.split_once('\t').unwrap().1.to_owned())
        .collect();
    let fixtures = fixtures();
    for (chunk, fixtures) in requests.chunks(4).zip(fixtures.chunks(4)) {
        for ((_, expected), original) in fixtures.iter().zip(original_observations(chunk)) {
            assert_eq!(
                unhex(original.split_whitespace().next().unwrap()),
                *expected
            );
        }
    }
}

#[test]
fn original_rejected_extra_channel_types_keep_metadata_and_complete_file_statuses() {
    let mut cases = Vec::new();
    for row in include_str!("../data/extra-channels-rejections.requests").lines() {
        let (kind, request) = row.split_once('\t').unwrap();
        let source = original_observations(&[request.to_owned()]);
        let changed = source[0].split_whitespace().next().unwrap();
        let expected = std::fs::read(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/data")
                .join(format!("extra-channel-rejected-{kind}.jxl")),
        )
        .unwrap();
        assert_eq!(unhex(changed), expected);
        let metadata = format!("ec-meta {changed}");
        let observation = original_observations(std::slice::from_ref(&metadata));
        let words: Vec<_> = observation[0].split_whitespace().collect();
        assert_eq!(words[0], "1");
        assert_eq!(&words[3..6], &["T", "1", kind]);
        cases.push(format!("fs-decode {changed}"));
        cases.push(metadata);
    }
    exact(cases);
}

fn half_probes() -> Vec<(String, bool)> {
    let fixtures: std::collections::HashMap<_, _> = fixtures().into_iter().collect();
    let probes: Vec<_> = include_str!("../data/extra-channels-half-strides.tsv")
        .lines()
        .map(|row| {
            let fields: Vec<_> = row.split('\t').collect();
            assert_eq!(fields.len(), 4);
            let request = format!(
                "ec-export {} 16 1 {} {} 3",
                hex(&fixtures[fields[0]]),
                fields[1],
                fields[2]
            );
            assert!(matches!(fields[3], "source-undefined" | "defined"));
            (request, fields[3] == "source-undefined")
        })
        .collect();
    assert_eq!(probes.len(), 888);
    assert_eq!(
        probes.iter().filter(|(_, undefined)| *undefined).count(),
        528
    );
    probes
}
#[test]
fn original_odd_half_strides_retain_every_request_and_positive_undefined_store_diagnostic() {
    let probes = half_probes();
    if let Some(path) = std::env::var_os("JXL_EXTRA_CHANNEL_HALF_CAPTURE") {
        std::fs::write(
            path,
            probes
                .iter()
                .map(|(request, _)| request.as_str())
                .collect::<Vec<_>>()
                .join("\n")
                + "\n",
        )
        .unwrap();
    }
    let instrumented = original_observations(&["patch-sanitizers".to_owned()])[0] == "1";
    let oracle = std::env::var_os("JXL_CPP_ORACLE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/oracle/jxl_oracle")
        });
    let mut diagnosed = 0;
    for (request, undefined) in probes {
        let mut child = Command::new(&oracle)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        writeln!(child.stdin.take().unwrap(), "{request}").unwrap();
        let output = child.wait_with_output().unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        let misaligned = stderr.contains("span.h:59:")
            && stderr.contains("runtime error: reference binding to misaligned address ")
            && stderr.contains("for type 'hwy::float16_t', which requires 2 byte alignment")
            && stderr.contains("SUMMARY: UndefinedBehaviorSanitizer: undefined-behavior ")
            && stderr.trim_end().ends_with("span.h:59:12");
        if undefined && instrumented {
            assert!(
                misaligned && !output.status.success(),
                "missing exact nonrecovering original misalignment: {request}\n{stderr}"
            );
            diagnosed += 1;
        } else {
            assert!(
                output.status.success() && stderr.is_empty(),
                "unexpected original source boundary failure: {request}\n{stderr}"
            );
            assert_eq!(String::from_utf8_lossy(&output.stdout).lines().count(), 1);
        }
        // No Rust comparison or output semantics are assigned to an original
        // undefined store, including its successful ordinary execution.
    }
    println!(
        "888 original odd-half requests retained; {diagnosed} positive source diagnostics; 360 defined old observations and 888 aligned controls remain exact parity"
    );
}
