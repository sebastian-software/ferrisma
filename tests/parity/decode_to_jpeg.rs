// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Differential scripts for JxlToJpegDecoder from lib/jxl/decode_to_jpeg.{h,cc}.
// Every jbrd, Exif, xml and codestream input comes from the original public
// JPEG encoder (or, for ICC payloads, the original EncodeJPEGData).
use super::*;
use jxl::jxl::base::status::{StatusCode, status_code};
use jxl::jxl::dec_frame::decode_jpeg_frame;
use jxl::jxl::decode::JxlDecoderStatus;
use jxl::jxl::decode_to_jpeg::JxlToJpegDecoder;
use jxl::jxl::image_bundle::ImageBundle;
use jxl::jxl::image_metadata::{CodecMetadata, ImageMetadata};
use jxl::jxl::jpeg::jpeg_data::{AppMarkerType, JpegData};
use std::sync::OnceLock;

pub(super) fn handles(kind: &str) -> bool {
    kind == "jtj-ops"
}

/// Caller-owned output storage with one byte beyond its logical size, as in
/// the original adapter. Every access uses the pointer the decoder stores.
struct RawBuffer {
    ptr: *mut u8,
    size: usize,
}
impl RawBuffer {
    fn new(size: usize) -> Self {
        let storage = vec![0xa5u8; size + 1].into_boxed_slice();
        Self {
            ptr: Box::into_raw(storage).cast::<u8>(),
            size,
        }
    }
    fn bytes(&self) -> &[u8] {
        // SAFETY: ptr owns size + 1 initialized bytes until drop. No decoder
        // write is in progress while this shared view exists.
        unsafe { std::slice::from_raw_parts(self.ptr, self.size) }
    }
}
impl Drop for RawBuffer {
    fn drop(&mut self) {
        // SAFETY: This reconstructs exactly the boxed slice leaked by new.
        drop(unsafe { Box::from_raw(std::ptr::slice_from_raw_parts_mut(self.ptr, self.size + 1)) });
    }
}

fn target<'a>(
    t: u32,
    d: &'a mut JxlToJpegDecoder,
    moved: &'a mut Option<Box<JpegData>>,
) -> &'a mut JpegData {
    let j = if t == 0 {
        d.get_jpeg_data()
    } else {
        moved.as_deref_mut()
    };
    j.expect("JPEG decoder script requires a live JPEGData target")
}

fn frame(
    args: &mut std::str::SplitWhitespace<'_>,
    d: &mut JxlToJpegDecoder,
    moved: &mut Option<Box<JpegData>>,
    out: &mut String,
) {
    let bytes = unhex(args.next().unwrap());
    let mut metadata = CodecMetadata::default();
    let mut reader = BitReader::new(&bytes);
    let status = (|| {
        if reader.read_fixed_bits::<16>() != 0x0aff {
            return Err(StatusCode::GenericError);
        }
        jxl::jxl::headers::read_size_header(&mut reader, &mut metadata.size)?;
        jxl::jxl::image_metadata::read_image_metadata(&mut reader, &mut metadata.m)?;
        metadata.transform_data.nonserialized_xyb_encoded = metadata.m.xyb_encoded;
        bundle::read(&mut reader, &mut metadata.transform_data)?;
        assert!(
            !metadata.m.color_encoding.want_icc(),
            "JPEG decoder script needs a non-ICC codestream"
        );
        reader.jump_to_byte_boundary()
    })();
    let offset = reader.total_bits_consumed() / 8;
    let close = reader.close();
    write!(
        out,
        " F {} {offset} {}",
        status_code(&status),
        status_code(&close)
    )
    .unwrap();
    if status.is_err() || close.is_err() {
        return;
    }
    let mut ib = ImageBundle::new(Some(&metadata.m));
    let set = d.set_image_bundle_jpeg_data(&mut ib);
    write!(out, " {}", status_code(&set)).unwrap();
    let Some(jpeg) = ib.jpeg_data.take() else {
        out.push_str(" -");
        return;
    };
    let decoded = decode_jpeg_frame(&metadata, &bytes[offset..], jpeg);
    write!(out, " {}", status_code(&decoded.status)).unwrap();
    *moved = decoded.image.jpeg_data;
}

pub(super) fn observe(_: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    // Buffers outlive every decoder use of their stored pointers.
    let mut buffers: Vec<RawBuffer> = Vec::new();
    let mut current: Option<usize> = None;
    let mut moved: Option<Box<JpegData>> = None;
    let mut d = JxlToJpegDecoder::new();
    let mut out = String::new();
    while let Some(op) = args.next() {
        match op {
            "R" => {
                d.reset();
                current = None;
                out.push_str(" R");
            }
            "S" => {
                let eof: u32 = number(args);
                let size: usize = number(args);
                d.start_box(eof != 0, size);
                out.push_str(" S");
            }
            "P" => {
                let v = unhex(args.next().unwrap());
                let mut next_in: &[u8] = &v;
                let s = d.process(&mut next_in);
                write!(
                    out,
                    " P {} {} {}",
                    s.0,
                    v.len() - next_in.len(),
                    next_in.len()
                )
                .unwrap();
            }
            "O" => {
                let size: usize = number(args);
                buffers.push(RawBuffer::new(size));
                let s = d.set_output_buffer(buffers.last().unwrap().ptr, size);
                if s == JxlDecoderStatus::SUCCESS {
                    current = Some(buffers.len() - 1);
                }
                write!(out, " O {}", s.0).unwrap();
            }
            "Z" => {
                let size: usize = number(args);
                let s = d.set_output_buffer(std::ptr::null_mut(), size);
                if s == JxlDecoderStatus::SUCCESS {
                    current = None;
                }
                write!(out, " Z {}", s.0).unwrap();
            }
            "L" => {
                let r = d.release_output_buffer();
                write!(out, " L {r}").unwrap();
                match current {
                    Some(i) => write!(out, " {}", hex(buffers[i].bytes())).unwrap(),
                    None => out.push_str(" -"),
                }
                current = None;
            }
            "B" => match current {
                Some(i) => write!(out, " B {}", hex(buffers[i].bytes())).unwrap(),
                None => out.push_str(" B -"),
            },
            "F" => frame(args, &mut d, &mut moved, &mut out),
            "I" => {
                let icc = unhex(args.next().unwrap());
                let mut m = ImageMetadata::default();
                if !icc.is_empty() {
                    m.color_encoding.set_icc_raw(icc);
                }
                let mut ib = ImageBundle::new(Some(&m));
                let s = d.set_image_bundle_jpeg_data(&mut ib);
                write!(
                    out,
                    " I {} {}",
                    status_code(&s),
                    u32::from(ib.jpeg_data.is_some())
                )
                .unwrap();
                if let Some(j) = ib.jpeg_data.take() {
                    moved = Some(j);
                }
            }
            "N" => {
                let t: u32 = number(args);
                let j = target(t, &mut d, &mut moved);
                let (mut es, mut xs) = (777usize, 777usize);
                let e = JxlToJpegDecoder::exif_box_content_size(j, &mut es);
                let x = JxlToJpegDecoder::xml_box_content_size(j, &mut xs);
                write!(
                    out,
                    " N {} {} {} {es} {} {xs}",
                    JxlToJpegDecoder::num_exif_markers(j),
                    JxlToJpegDecoder::num_xmp_markers(j),
                    e.0,
                    x.0
                )
                .unwrap();
            }
            "X" | "M" => {
                let t: u32 = number(args);
                let v = unhex(args.next().unwrap());
                let j = target(t, &mut d, &mut moved);
                let s = if op == "X" {
                    JxlToJpegDecoder::set_exif(&v, j)
                } else {
                    JxlToJpegDecoder::set_xmp(&v, j)
                };
                write!(out, " {op} {}", s.0).unwrap();
            }
            "A" => {
                let t: u32 = number(args);
                let j = target(t, &mut d, &mut moved);
                write!(out, " A {}", j.app_data.len()).unwrap();
                for app in &j.app_data {
                    write!(out, " {}", hex(app)).unwrap();
                }
                write!(out, " {}", j.app_marker_type.len()).unwrap();
                for kind in &j.app_marker_type {
                    write!(out, " {}", kind.0).unwrap();
                }
            }
            "J" => {
                out.push_str(" J");
                match d.get_jpeg_data() {
                    Some(j) => super::jpeg_metadata::data(&mut out, j),
                    None => out.push_str(" -"),
                }
            }
            "W" => {
                let j = moved
                    .as_deref()
                    .expect("JPEG decoder script requires a decoded frame");
                // SAFETY: The stored output pointer is null only with zero
                // size (scripts never write after a nonzero null buffer) or
                // a live RawBuffer of the stored size. Buffers outlive d, the
                // JPEGData owner is distinct and no other access overlaps.
                let s = unsafe { d.write_output(j) };
                write!(out, " W {}", s.0).unwrap();
            }
            _ => panic!("unknown JPEG decoder operation {op}"),
        }
        write!(
            out,
            " | {} {} {}",
            u32::from(d.is_output_set()),
            u32::from(d.is_parsing_box()),
            u32::from(d.get_jpeg_data().is_some())
        )
        .unwrap();
    }
    drop(d);
    out
}

struct Fixture {
    name: String,
    jpeg: Vec<u8>,
    jbrd: Vec<u8>,
    exif: Option<Vec<u8>>,
    xml: Option<Vec<u8>>,
    codestream: Vec<u8>,
}

fn segment(marker: u8, payload: &[u8]) -> Vec<u8> {
    let length = payload.len() + 2;
    let mut s = vec![0xff, marker, (length >> 8) as u8, length as u8];
    s.extend_from_slice(payload);
    s
}

/// Exif APP1 payload: the tag, a little-endian TIFF header and one IFD with a
/// SHORT orientation entry, followed by `extra` padding bytes.
fn exif_payload(orientation: u8, extra: usize) -> Vec<u8> {
    let mut p = b"Exif\0\0".to_vec();
    p.extend_from_slice(&[0x49, 0x49, 0x2a, 0, 8, 0, 0, 0, 1, 0]);
    p.extend_from_slice(&[0x12, 0x01, 3, 0, 1, 0, 0, 0, orientation, 0, 0, 0]);
    p.extend_from_slice(&[0, 0, 0, 0]);
    p.extend((0..extra).map(|i| (i * 7 + 3) as u8));
    p
}

fn xmp_payload(text: &str) -> Vec<u8> {
    let mut p = b"http://ns.adobe.com/xap/1.0/\0".to_vec();
    p.extend_from_slice(b"<?xpacket begin='' id='W5M0MpCehiHzreSzNTczkc9d'?>");
    p.extend_from_slice(b"<x:xmpmeta xmlns:x='adobe:ns:meta/'><rdf:RDF ");
    p.extend_from_slice(b"xmlns:rdf='http://www.w3.org/1999/02/22-rdf-syntax-ns#'>");
    p.extend_from_slice(
        b"<rdf:Description rdf:about='' xmlns:dc='http://purl.org/dc/elements/1.1/'>",
    );
    p.extend_from_slice(text.as_bytes());
    p.extend_from_slice(b"</rdf:Description></rdf:RDF></x:xmpmeta><?xpacket end='w'?>");
    p
}

/// Insert whole marker segments after SOI and an initial APP0 segment.
fn with_segments(jpeg: &[u8], segments: &[Vec<u8>]) -> Vec<u8> {
    let mut pos = 2;
    if jpeg[2..4] == [0xff, 0xe0] {
        pos += 2 + usize::from(u16::from_be_bytes([jpeg[4], jpeg[5]]));
    }
    let mut out = jpeg[..pos].to_vec();
    for s in segments {
        out.extend_from_slice(s);
    }
    out.extend_from_slice(&jpeg[pos..]);
    out
}

fn boxes(file: &[u8]) -> Vec<([u8; 4], &[u8])> {
    let mut result = Vec::new();
    let mut pos = 0;
    while pos < file.len() {
        let size = u32::from_be_bytes(file[pos..pos + 4].try_into().unwrap()) as usize;
        let kind: [u8; 4] = file[pos + 4..pos + 8].try_into().unwrap();
        assert!(size >= 8, "encoder boxes use 32-bit sizes");
        result.push((kind, &file[pos + 8..pos + size]));
        pos += size;
    }
    result
}

fn base_jpegs() -> Vec<(&'static str, Vec<u8>)> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/data");
    [
        "jpeg-writer/baseline.jpg",
        "jpeg-writer/gray.jpg",
        "jpeg-writer/progressive.jpg",
        "jpeg-writer/restarts.jpg",
        "jpeg-reconstruction/rgb-baseline.jpg",
        "jpeg-reconstruction/rgb-progressive.jpg",
        "jpeg-reconstruction/rgb-restarts.jpg",
        "jpeg-writer/large-progressive.jpg",
        "jpeg-writer/chunked-baseline.jpg",
    ]
    .into_iter()
    .map(|path| (path, std::fs::read(root.join(path)).unwrap()))
    .collect()
}

fn fixtures() -> &'static [Fixture] {
    static FIXTURES: OnceLock<Vec<Fixture>> = OnceLock::new();
    FIXTURES.get_or_init(|| {
        let comment = segment(0xfe, b"JxlToJpegDecoder differential fixture");
        let mut inputs = Vec::new();
        for (path, jpeg) in base_jpegs() {
            let large = jpeg.len() > 16384;
            let mut variants = vec![("plain", vec![])];
            variants.push((
                "exif-xmp-com",
                vec![
                    segment(0xe1, &exif_payload(1, 0)),
                    segment(0xe1, &xmp_payload("<dc:format>image/jpeg</dc:format>")),
                    comment.clone(),
                ],
            ));
            if !large {
                variants.push(("exif", vec![segment(0xe1, &exif_payload(1, 37))]));
                variants.push(("xmp", vec![segment(0xe1, &xmp_payload("x"))]));
                // DetectBlobs marks only the first Exif marker; the second one
                // stays an unknown, Brotli-compressed APP1 marker.
                variants.push((
                    "two-exif-xmp",
                    vec![
                        segment(0xe1, &exif_payload(1, 5)),
                        segment(0xe1, &xmp_payload("")),
                        segment(0xe1, &exif_payload(1, 200)),
                    ],
                ));
            }
            for (suffix, segments) in variants {
                inputs.push((format!("{path}:{suffix}"), with_segments(&jpeg, &segments)));
            }
        }
        let requests: Vec<_> = inputs
            .iter()
            .map(|(_, jpeg)| format!("jtj-encode {} 7 0", hex(jpeg)))
            .collect();
        let encoded = original_observations(&requests);
        inputs
            .into_iter()
            .zip(encoded)
            .map(|((name, jpeg), observed)| {
                let (prefix, wire) = observed.rsplit_once(' ').unwrap();
                assert_eq!(prefix, "API 0 0 0 0 0 W 0 0", "{name}");
                let file = unhex(wire);
                let mut f = Fixture {
                    name,
                    jpeg,
                    jbrd: Vec::new(),
                    exif: None,
                    xml: None,
                    codestream: Vec::new(),
                };
                for (kind, contents) in boxes(&file) {
                    match &kind {
                        b"jbrd" => f.jbrd = contents.to_vec(),
                        b"Exif" => f.exif = Some(contents.to_vec()),
                        b"xml " => f.xml = Some(contents.to_vec()),
                        b"jxlp" => f.codestream.extend_from_slice(&contents[4..]),
                        _ => (),
                    }
                }
                assert!(!f.jbrd.is_empty() && !f.codestream.is_empty());
                assert_eq!(f.exif.is_some(), f.name.contains("exif"), "{}", f.name);
                assert_eq!(f.xml.is_some(), f.name.contains("xmp"), "{}", f.name);
                f
            })
            .collect()
    })
}

/// The decode.cc order: jbrd box, frame, Exif/XMP boxes, JPEG output.
fn metadata_ops(f: &Fixture) -> String {
    let mut ops = String::new();
    if let Some(exif) = &f.exif {
        write!(ops, " X 1 {}", hex(exif)).unwrap();
    }
    if let Some(xml) = &f.xml {
        write!(ops, " M 1 {}", hex(xml)).unwrap();
    }
    ops
}

fn decoded_prefix(f: &Fixture, output: usize) -> String {
    format!(
        "jtj-ops S 0 {} P {} O {output} F {}{}",
        f.jbrd.len(),
        hex(&f.jbrd),
        hex(&f.codestream),
        metadata_ops(f)
    )
}

fn released_bytes(observed: &str, index: usize) -> Vec<u8> {
    let part = observed.split(" L ").nth(index + 1).unwrap();
    unhex(part.split_whitespace().nth(1).unwrap())
}

#[test]
fn jxl_to_jpeg_reconstructs_original_files_with_exif_and_xmp() {
    let mut requests = Vec::new();
    for f in fixtures() {
        let n = f.jpeg.len();
        requests.push(format!(
            "jtj-ops S 0 {} P {} N 0 J O {n} F {}{} N 1 A 1 W L",
            f.jbrd.len(),
            hex(&f.jbrd),
            hex(&f.codestream),
            metadata_ops(f)
        ));
        // A larger buffer keeps its unwritten suffix; a second write appends
        // another copy at the advanced position until the buffer is full.
        requests.push(format!("{} W B W L", decoded_prefix(f, 2 * n + 7)));
        // Exif and XMP sizes are checked before any marker write; the
        // decoder-owned copy is filled before the bundle takes it.
        let mut ops = format!("jtj-ops S 0 {} P {}", f.jbrd.len(), hex(&f.jbrd));
        for (op, data) in [("X", &f.exif), ("M", &f.xml)] {
            let data = data.clone().unwrap_or_else(|| vec![0; 40]);
            let mut longer = data.clone();
            longer.push(0);
            write!(
                ops,
                " {op} 0 {} {op} 0 {} {op} 0 {} A 0",
                hex(&data[..data.len() - 1]),
                hex(&longer),
                hex(&data)
            )
            .unwrap();
        }
        write!(
            ops,
            " N 0 O {n} F {}{} A 1 W L",
            hex(&f.codestream),
            metadata_ops(f)
        )
        .unwrap();
        requests.push(ops);
    }
    let observed = original_observations(&requests);
    for (f, chunk) in fixtures().iter().zip(observed.chunks(3)) {
        assert_eq!(released_bytes(&chunk[0], 0), f.jpeg, "{}", f.name);
        let mut twice = f.jpeg.clone();
        twice.extend_from_slice(&f.jpeg);
        twice.extend_from_slice(&[0xa5; 7]);
        assert_eq!(released_bytes(&chunk[1], 0), twice, "{}", f.name);
        assert_eq!(released_bytes(&chunk[2], 0), f.jpeg, "{}", f.name);
    }
    for (request, cpp) in requests.iter().zip(observed) {
        assert_eq!(super::observe(request), cpp, "request: {request}");
    }
    println!("{} C++/Rust differential cases matched", requests.len());
}

fn chunked(data: &[u8], sizes: &mut dyn FnMut() -> usize) -> String {
    let mut ops = String::new();
    let mut pos = 0;
    while pos < data.len() {
        let end = (pos + sizes().max(1)).min(data.len());
        write!(ops, " P {}", hex(&data[pos..end])).unwrap();
        pos = end;
    }
    ops
}

#[test]
fn jxl_to_jpeg_box_contents_in_arbitrary_chunks_and_repeated_owners() {
    let mut cases = Vec::new();
    let mut seed = 0x5eed_u64;
    let all = fixtures();
    for (index, f) in all.iter().enumerate() {
        let n = f.jbrd.len();
        let mut trailing = f.jbrd.clone();
        trailing.extend_from_slice(&[0x11, 0x22, 0x33, 0x44, 0x55]);
        let mut patterns: Vec<Box<dyn FnMut() -> usize>> = Vec::new();
        for size in [1, 2, 3, 7, 64, 255, n - 1, n, n + 5] {
            patterns.push(Box::new(move || size));
        }
        for modulus in [5, 97] {
            let mut local = seed ^ modulus;
            patterns.push(Box::new(move || (next(&mut local) % modulus) as usize));
        }
        seed = next(&mut seed);
        for mut sizes in patterns {
            let bounded = chunked(&trailing, &mut *sizes);
            cases.push(format!("jtj-ops S 0 {n}{bounded} P - J N 0"));
            let unbounded = chunked(&f.jbrd, &mut *sizes);
            cases.push(format!("jtj-ops S 1 0{unbounded} J"));
            let unbounded = chunked(&trailing, &mut *sizes);
            cases.push(format!("jtj-ops S 1 0{unbounded} J"));
        }
        let half = &f.jbrd[..n / 2];
        let rest = &f.jbrd[n / 2..];
        let other = &all[(index + 1) % all.len()].jbrd;
        let (jbrd, half, rest, other) = (hex(&f.jbrd), hex(half), hex(rest), hex(other));
        let m = other.len() / 2;
        for ops in [
            format!("P {jbrd} S 0 {n} P {jbrd} P {jbrd} J"),
            format!("S 0 {n} P {jbrd} S 0 {n} P {half} J P {rest} J"),
            format!("S 0 {n} P {half} R P {rest} S 0 {n} P {jbrd} J"),
            format!("S 0 {n} P {half} S 0 {n} P {half} P {rest} J"),
            format!("S 0 {n} P {jbrd} S 0 {m} P {other} J N 0"),
            // An unbounded box keeps box_until_eof until Reset, so the next
            // bounded StartBox still consumes all input as unbounded.
            format!("S 1 0 P {jbrd} S 0 {n} P {half} P {rest}{} J", " P 00"),
            format!("S 1 0 P {half} R S 0 {n} P {half} P {rest} J"),
            format!("S 0 0 P - J S 0 0 P {jbrd} J"),
            format!("S 0 {n} P - P - J R J"),
            format!("S 0 {} P {jbrd} J", n + 1),
            format!("S 0 {} P {half} P {rest} P 00 J", n + 1),
        ] {
            cases.push(format!("jtj-ops {ops}"));
        }
    }
    for batch in cases.chunks(64) {
        compare(batch.to_vec());
    }
}

fn output_sizes(n: usize) -> Vec<usize> {
    if n <= 1100 {
        return (0..=n + 2).collect();
    }
    let mut sizes: Vec<usize> = (0..=300).collect();
    sizes.extend((301..n).step_by(97));
    for boundary in (16384..n).step_by(16384) {
        sizes.extend(boundary - 3..=boundary + 3);
    }
    sizes.extend(n - 3..=n + 2);
    sizes.sort_unstable();
    sizes.dedup();
    sizes
}

#[test]
fn jxl_to_jpeg_output_buffers_of_every_small_size_need_more_output() {
    let mut cases = Vec::new();
    let mut checks = Vec::new();
    for f in fixtures() {
        let n = f.jpeg.len();
        for sizes in output_sizes(n).chunks(if n <= 1100 { 400 } else { 40 }) {
            let mut ops = format!("{} L", decoded_prefix(f, 0));
            for size in sizes {
                write!(ops, " O {size} W L").unwrap();
            }
            checks.push((cases.len(), sizes.to_vec(), n));
            cases.push(ops);
        }
        let prefix = decoded_prefix(f, 1);
        cases.push(format!(
            "{prefix} W L O {} W W W L O 5 O 6 L Z 9 O 4 L Z 0 W L",
            2 * n + 3
        ));
        // Without an output buffer the decoder keeps its JPEGData and the
        // bundle stays without one. A null buffer is not an output buffer.
        let jbrd = format!("S 0 {} P {}", f.jbrd.len(), hex(&f.jbrd));
        let cs = hex(&f.codestream);
        cases.push(format!(
            "jtj-ops {jbrd} F {cs} J Z 3 F {cs} L O {n} F {cs} J W L"
        ));
        cases.push(format!("jtj-ops O 4 {jbrd} R J O {n} {jbrd} F {cs} W L"));
    }
    for (index, sizes, n) in checks {
        let observed = original_observations(std::slice::from_ref(&cases[index]));
        for (i, size) in sizes.iter().enumerate() {
            let part = observed[0].split(" O ").nth(i + 2).unwrap();
            let status = part.split(" W ").nth(1).unwrap().split(' ').next().unwrap();
            let expected = if *size >= n { "0" } else { "6" };
            assert_eq!(status, expected, "size {size} of {n}");
        }
    }
    for batch in cases.chunks(8) {
        compare(batch.to_vec());
    }
}

#[test]
fn jxl_to_jpeg_truncated_and_mutated_reconstruction_boxes() {
    let mut cases = Vec::new();
    for f in fixtures()
        .iter()
        .filter(|f| f.name.ends_with(":exif-xmp-com") && f.jpeg.len() < 16384)
    {
        let n = f.jbrd.len();
        for length in 0..=n {
            let prefix = hex(&f.jbrd[..length]);
            cases.push(format!(
                "jtj-ops S 0 {n} P {prefix} J S 0 {length} P {prefix} J N 0"
            ));
            cases.push(format!("jtj-ops S 1 0 P {prefix} J N 0"));
        }
        for position in (0..n).filter(|p| *p < 160 || p % 5 == 0) {
            for mask in [0x01, 0x10, 0x80] {
                let mut mutated = f.jbrd.clone();
                mutated[position] ^= mask;
                cases.push(format!("jtj-ops S 0 {n} P {} J N 0", hex(&mutated)));
            }
        }
    }
    for batch in cases.chunks(256) {
        compare(batch.to_vec());
    }
}

#[test]
fn jxl_to_jpeg_icc_injection_keeps_partial_writes_on_failure() {
    let mut seed = 0x1cc_u64;
    let icc: Vec<u8> = (0..700).map(|_| next(&mut seed) as u8).collect();
    let chunk = |id: u8, count: u8, data: &[u8]| {
        let mut payload = b"ICC_PROFILE\0".to_vec();
        payload.extend_from_slice(&[id, count]);
        payload.extend_from_slice(data);
        segment(0xe2, &payload)
    };
    let mut cases = Vec::new();
    for (_, jpeg) in base_jpegs().into_iter().take(3) {
        for segments in [
            vec![chunk(1, 1, &icc)],
            vec![
                chunk(1, 3, &icc[..100]),
                chunk(2, 3, &icc[100..450]),
                chunk(3, 3, &icc[450..]),
                segment(0xe1, &exif_payload(1, 3)),
            ],
        ] {
            let input = with_segments(&jpeg, &segments);
            let observed = original_observations(&[format!("jm-packed-fixture {} 4", hex(&input))]);
            let mut words = observed[0].split_whitespace();
            assert_eq!(words.next(), Some("P"));
            assert_eq!(words.next(), Some("0"));
            let jbrd = unhex(words.next().unwrap());
            let box_ops = format!("S 0 {} P {}", jbrd.len(), hex(&jbrd));
            let mut longer = icc.clone();
            longer.push(9);
            for (exact, profile) in [
                (true, hex(&icc)),
                (false, hex(&icc[..icc.len() - 1])),
                (false, hex(&longer)),
                (false, hex(&icc[..1])),
                (false, "-".to_owned()),
            ] {
                // Only the exact profile moves JPEGData into the bundle. A
                // failed injection keeps its partial writes in the decoder.
                let after = if exact { "J A 1" } else { "A 0 J" };
                cases.push(format!(
                    "jtj-ops {box_ops} N 0 I {profile} A 0 O 3 I {profile} {after} I {profile}"
                ));
            }
        }
    }
    // A bundle never receives JPEGData when the decoder has none.
    cases.push(format!("jtj-ops O 1 I {} R O 1 I -", hex(&icc)));
    compare(cases);
}

/// SetExif with a matching Exif marker shorter than its nine header bytes
/// (Exif data shorter than four bytes) makes both original memcpy calls
/// overflow. decode.cc never reaches it: DecodeJPEGData rejects such a marker
/// and the API calls SetExif only after a successful jbrd box. A failed box
/// still leaves the short marker in the decoder's owner, so a direct call can.
/// The defined prefix compares exactly; the undefined call runs original-only
/// and the Rust boundary stops at a bounds check instead of inventing a status.
#[test]
fn jxl_to_jpeg_short_exif_marker_is_an_undefined_source_call() {
    let fixture = super::jpeg_metadata::fixture("gray.jpg");
    assert_eq!(fixture.app_data[0][0], 0xe0, "gray.jpg starts with APP0");
    let mut defined = Vec::new();
    let mut undefined = Vec::new();
    for length in 5..9usize {
        let mut j = fixture.clone();
        j.app_marker_type = vec![AppMarkerType::UNKNOWN; j.app_data.len()];
        j.app_marker_type[0] = AppMarkerType::EXIF;
        j.app_data[0].truncate(length);
        let written =
            original_observations(&[super::jpeg_metadata::request(&j, "Z 4 -", 1)]).remove(0);
        let mut words = written.split_whitespace();
        assert_eq!(words.next(), Some("Z"));
        assert_eq!(words.next(), Some("0"));
        let jbrd = unhex(words.next().unwrap());
        let prefix = format!("jtj-ops S 0 {} P {} N 0 A 0 J", jbrd.len(), hex(&jbrd));
        defined.push(prefix.clone());
        // The matching size check passes: length == (length - 5) + 9 - 4.
        undefined.push(format!("{prefix} X 0 {}", hex(&vec![0x42; length - 5])));
    }
    compare(defined);
    let instrumented = original_observations(&["patch-sanitizers".into()])[0] == "1";
    let oracle = std::env::var_os("JXL_CPP_ORACLE")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/oracle/jxl_oracle")
        });
    let mut diagnosed = 0;
    for request in &undefined {
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
        let frame = stderr
            .lines()
            .find(|line| {
                line.trim_start().starts_with("#1 ") || line.trim_start().starts_with("#0 ")
            })
            .unwrap_or_default();
        let overflow = stderr.contains("ERROR: AddressSanitizer: heap-buffer-overflow ")
            && stderr.contains("WRITE of size ")
            && (!frame.contains(" in ") || stderr.contains("JxlToJpegDecoder::SetExif"))
            && stderr.contains("SUMMARY: AddressSanitizer: heap-buffer-overflow");
        if instrumented {
            assert!(
                overflow && !output.status.success(),
                "instrumented source must diagnose the short Exif marker write: {request}\n{stderr}"
            );
            diagnosed += 1;
        } else {
            println!(
                "Original ordinary short Exif request (undefined, not compared): {}",
                output.status
            );
        }
        assert!(
            std::panic::catch_unwind(|| super::observe(request)).is_err(),
            "Rust must not write past the short Exif marker: {request}"
        );
    }
    println!(
        "{} original short Exif requests exercised; {diagnosed} diagnosed",
        undefined.len()
    );
}
