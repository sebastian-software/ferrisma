// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::dec_external_image::{convert_float_to_buffer, convert_uint_to_buffer};
use jxl::jxl::dec_frame::SequenceDecoder;
use jxl::jxl::image_metadata::{CodecMetadata, Orientation};
use jxl::jxl::types::JxlEndianness;

pub(super) fn handles(kind: &str) -> bool {
    kind == "geo-export"
}
pub(super) fn observe(_kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let bytes = unhex(args.next().unwrap());
    let bits: usize = number(args);
    let floating = number::<u8>(args) != 0;
    let channels: usize = number(args);
    let keep = number::<u8>(args) != 0;
    let unpremul = number::<u8>(args) != 0;
    let padding: usize = number(args);
    let shortage: usize = number(args);
    let endianness = JxlEndianness(number(args));
    let mut metadata = CodecMetadata::default();
    let mut reader = BitReader::new(&bytes);
    assert_eq!(reader.read_fixed_bits::<16>(), 0x0aff);
    jxl::jxl::headers::read_size_header(&mut reader, &mut metadata.size).unwrap();
    jxl::jxl::image_metadata::read_image_metadata(&mut reader, &mut metadata.m).unwrap();
    metadata.transform_data.nonserialized_xyb_encoded = metadata.m.xyb_encoded;
    jxl::jxl::fields::bundle::read(&mut reader, &mut metadata.transform_data).unwrap();
    assert!(!metadata.m.color_encoding.want_icc());
    reader.jump_to_byte_boundary().unwrap();
    reader.all_reads_within_bounds().unwrap();
    let mut offset = reader.total_bits_consumed() / 8;
    reader.close().unwrap();
    assert!(!metadata.m.have_preview, "non-preview export fixture");
    let mut decoder = SequenceDecoder::new(&metadata).unwrap();
    let mut count = 0;
    loop {
        assert!(count < 64, "geometry fixture frame bound");
        count += 1;
        let decoded = decoder.decode_frame(&bytes[offset..], false);
        if let Err(status) = decoded.status {
            return format!("D {} {offset}", status as i32);
        }
        offset += decoder.output().decoded_bytes();
        if decoded.header.unwrap().is_last {
            break;
        }
    }
    let image = decoder.output();
    let orientation = if keep {
        Orientation::IDENTITY
    } else {
        metadata.m.get_orientation()
    };
    let (mut width, mut height) = (image.xsize(), image.ysize());
    if orientation.0 > 4 {
        std::mem::swap(&mut width, &mut height);
    }
    let stride = width * channels * bits.div_ceil(8) + padding;
    let mut output = vec![0xa5; stride * height + 7];
    let size = output.len().checked_sub(shortage).unwrap();
    let status = if floating {
        convert_float_to_buffer(
            &image,
            bits,
            channels,
            endianness,
            stride,
            &mut output,
            size,
            orientation,
            unpremul,
        )
    } else {
        convert_uint_to_buffer(
            &image,
            bits,
            channels,
            endianness,
            stride,
            &mut output,
            size,
            orientation,
            unpremul,
        )
    };
    format!(
        "{} M {} {} {} {} {} {} O {width} {height} {} {} {}",
        status_code(&status),
        metadata.m.orientation,
        u8::from(metadata.m.have_intrinsic_size),
        metadata.m.intrinsic_size.xsize(),
        metadata.m.intrinsic_size.ysize(),
        metadata.oriented_xsize(keep),
        metadata.oriented_ysize(keep),
        u8::from(image.has_alpha()),
        u8::from(image.alpha_is_premultiplied()),
        hex(&output),
    )
}

pub(super) fn bases() -> Vec<(String, Vec<u8>)> {
    include_str!("../data/decoder-geometry.requests")
        .lines()
        .map(|row| {
            let (name, path) = row.split_once('\t').unwrap();
            let bytes = std::fs::read(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("tests/data/decoder-geometry")
                    .join(path),
            )
            .unwrap();
            (name.to_owned(), bytes)
        })
        .collect()
}

fn oriented_cases(include_previews: bool) -> Vec<Vec<u8>> {
    let mut requests = Vec::new();
    for (index, (name, bytes)) in bases().iter().enumerate() {
        if !include_previews && name.starts_with("sequence-preview-") {
            continue;
        }
        for orientation in 1..=8 {
            let (iw, ih) = if (index + orientation) % 2 == 0 {
                (31, 17)
            } else {
                (0, 0)
            };
            requests.push(format!(
                "geo-rewrite {} {orientation} {iw} {ih}",
                hex(bytes)
            ));
        }
    }
    let mut output = Vec::new();
    for chunk in requests.chunks(8) {
        for observation in original_observations(chunk) {
            let words: Vec<_> = observation.split_whitespace().collect();
            assert_eq!(words[1], "G");
            assert_eq!(words[4], "P");
            let bytes = unhex(words[0]);
            let header: usize = words[3].parse().unwrap();
            assert_eq!(hex(&bytes[header..]), words[5]);
            output.push(bytes);
        }
    }
    output
}
fn exact(requests: Vec<String>, require_success: bool) {
    if let Some(path) = std::env::var_os("JXL_DECODER_GEOMETRY_CAPTURE") {
        static CAPTURE: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _lock = CAPTURE.lock().unwrap();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        file.write_all((requests.join("\n") + "\n").as_bytes())
            .unwrap();
    }
    for chunk in requests.chunks(8) {
        for (request, original) in chunk.iter().zip(original_observations(chunk)) {
            if require_success {
                if request.starts_with("fs-decode ") {
                    let words: Vec<_> = original.split_whitespace().collect();
                    let end = words.iter().rposition(|word| *word == "E").unwrap();
                    assert_eq!(words[end + 1], "0", "complete source fixture: {request}");
                } else {
                    assert_eq!(
                        original.split_whitespace().next(),
                        Some("0"),
                        "complete source export: {request}"
                    );
                }
            }
            let rust = super::observe(request);
            assert_eq!(rust, original, "request {request}");
        }
    }
    println!("{} C++/Rust differential cases matched", requests.len());
}

#[test]
fn all_source_chroma_modes_and_all_orientations_keep_complete_bundle_words() {
    exact(
        oriented_cases(true)
            .iter()
            .map(|bytes| format!("fs-decode {}", hex(bytes)))
            .collect(),
        true,
    );
}

#[test]
fn complete_oriented_exports_keep_integer_half_float_endian_and_padding_words() {
    let mut requests = Vec::new();
    for (index, bytes) in oriented_cases(false).iter().enumerate() {
        for (bits, floating) in [(8, 0), (16, 0), (16, 1), (32, 1)] {
            for channels in 1..=4 {
                for keep in 0..=1 {
                    let endian = (index + bits + channels) % 3;
                    requests.push(format!(
                        "geo-export {} {bits} {floating} {channels} {keep} 1 16 0 {endian}",
                        hex(bytes)
                    ));
                }
            }
        }
    }
    exact(requests, true);
}

#[test]
fn oriented_metadata_and_payload_errors_retain_every_initialized_partial_frame() {
    let mut requests = Vec::new();
    for bytes in oriented_cases(true) {
        let mut cuts = vec![
            0,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            12,
            16,
            bytes.len() / 2,
            bytes.len() - 1,
        ];
        cuts.retain(|&cut| cut < bytes.len());
        cuts.sort_unstable();
        cuts.dedup();
        for cut in cuts {
            requests.push(format!("fs-decode {}", hex(&bytes[..cut])));
        }
        for at in [bytes.len() / 2, bytes.len() - 1] {
            for mask in [1, 128] {
                let mut corrupted = bytes.clone();
                corrupted[at] ^= mask;
                requests.push(format!("fs-decode {}", hex(&corrupted)));
            }
        }
    }
    exact(requests, false);
}

#[test]
fn original_jpeg_api_reproduces_each_complete_geometry_base() {
    let bases: Vec<_> = bases()
        .into_iter()
        .filter(|(name, _)| name.starts_with("sampling-"))
        .collect();
    for chunk in bases.chunks(8) {
        let requests: Vec<_> = chunk
            .iter()
            .map(|(name, _)| {
                let jpeg = std::fs::read(
                    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("tests/data/decoder-geometry")
                        .join(format!("{name}.jpg")),
                )
                .unwrap();
                format!("ycc-jpeg-encode {}", hex(&jpeg))
            })
            .collect();
        for ((_, bytes), response) in chunk.iter().zip(original_observations(&requests)) {
            let words: Vec<_> = response.split_whitespace().collect();
            let at = words.iter().position(|&word| word == "W").unwrap();
            assert_eq!(words[at + 1], "0");
            assert_eq!(unhex(words[at + 3]), *bytes);
        }
    }
}
