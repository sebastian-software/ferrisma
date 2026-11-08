//! WP8 differential tests: `src/demux/demux.c` (`WebPDemuxInternal`, the iterators
//! and the chunk walks) and the wrappers `WebPDemux` and `WebPDemuxPartial` of
//! `src/webp/demux.h`, against the C probe `tools/oracle/probes/wp8/demux.c`.
//!
//! The probe prints the state, the features, every frame and chunk that the
//! iterators reach with all their fields, and the forward and backward walks.
//! Each test builds its inputs, runs them through one probe process and compares
//! the printed lines with `wp8_support::report` of the Rust translation.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp8_support/mod.rs"]
mod wp8_support;

use std::fmt::Write as _;
use std::path::Path;

use oracle::Oracle;
use wp8_support::{
    Rng, anim_payload, anmf, chunk, chunk_headers, hex_token, payload_of, report, riff,
    riff_with_size, truncation_points, version_report, vp8x_payload,
};

/// Commands and the report that each one must print, in order.
#[derive(Default)]
struct Batch {
    commands: String,
    expected: String,
    /// The first expected line of each command, with the command.
    starts: Vec<(usize, String)>,
    lines: usize,
}

impl Batch {
    fn add(&mut self, command: String, lines: String) {
        self.starts.push((self.lines, command.clone()));
        self.lines += lines.matches('\n').count();
        self.commands.push_str(&command);
        self.commands.push('\n');
        self.expected.push_str(&lines);
    }

    /// Runs `demux` on the bytes, with `allow_partial`.
    fn demux(&mut self, bytes: &[u8], allow_partial: bool) {
        let command = format!("demux {} {}", hex_token(bytes), i32::from(allow_partial));
        self.add(command, report(bytes, allow_partial));
    }

    /// Runs `file` on the first `size` bytes of `path`, whose contents are `bytes`.
    fn file(&mut self, path: &Path, bytes: &[u8], size: usize, allow_partial: bool) {
        let command = format!(
            "file {} 0 {} {}",
            path.display(),
            size,
            i32::from(allow_partial)
        );
        self.add(command, report(&bytes[..size], allow_partial));
    }

    /// Runs the probe with every command of the batch, and compares the output.
    fn check(&self, test: &str) {
        let Some(oracle) = Oracle::locate(test) else {
            return;
        };
        let output = oracle
            .probe("wp8", "demux")
            .run::<&str>(&[], self.commands.as_bytes());
        output.success();
        compare(test, &self.starts, &output.stdout, &self.expected);
        eprintln!("{test}: {} commands compared", self.starts.len());
    }
}

/// Compares the probe's output with the expected lines. A mismatch names the
/// command that printed the first differing line.
fn compare(test: &str, starts: &[(usize, String)], actual: &[u8], expected: &str) {
    let actual = std::str::from_utf8(actual).expect("the probe prints UTF-8");
    if actual == expected {
        return;
    }
    let actual_lines: Vec<&str> = actual.lines().collect();
    let expected_lines: Vec<&str> = expected.lines().collect();
    let index = actual_lines
        .iter()
        .zip(&expected_lines)
        .position(|(a, e)| a != e)
        .unwrap_or(actual_lines.len().min(expected_lines.len()));
    let command = starts
        .iter()
        .rev()
        .find(|(line, _)| *line <= index)
        .map_or("?", |(_, command)| command.as_str());
    let mut message = String::new();
    writeln!(
        message,
        "{test}: line {index} differs for command `{}`",
        command.chars().take(200).collect::<String>()
    )
    .unwrap();
    writeln!(message, "  C:    {:?}", actual_lines.get(index)).unwrap();
    writeln!(message, "  Rust: {:?}", expected_lines.get(index)).unwrap();
    for offset in 1..=3 {
        if let Some(line) = actual_lines.get(index.wrapping_sub(offset)) {
            writeln!(message, "  C -{offset}:  {line:?}").unwrap();
        }
    }
    panic!("{message}");
}

/// A base image set: the first payload of each tag in the test vectors.
struct Images {
    vp8: Vec<u8>,
    vp8l: Vec<u8>,
    alph: Vec<u8>,
}

fn images() -> Images {
    let data = oracle::test_data();
    let read = |name: &str| std::fs::read(data.join(name)).expect("test vector");
    let files = vec![
        read("vp80-00-comprehensive-001.webp"),
        read("lossless1.webp"),
        read("lossy_alpha1.webp"),
    ];
    Images {
        vp8: payload_of(&files, b"VP8 ").expect("a simple lossy vector"),
        vp8l: payload_of(&files, b"VP8L").expect("a simple lossless vector"),
        alph: payload_of(&files, b"ALPH").expect("a lossy vector with alpha"),
    }
}

/// A still image: `VP8X` with `flags`, metadata chunks and an image.
fn still_image(images: &Images, flags: u8, lossless: bool, metadata: bool) -> Vec<u8> {
    let mut body = chunk(b"VP8X", &vp8x_payload(flags, 96, 80));
    if metadata {
        body.extend(chunk(b"ICCP", b"icc-profile!"));
    }
    if lossless {
        body.extend(chunk(b"VP8L", &images.vp8l));
    } else {
        body.extend(chunk(b"ALPH", &images.alph));
        body.extend(chunk(b"VP8 ", &images.vp8));
    }
    if metadata {
        body.extend(chunk(b"EXIF", b"Exif\0\0mm"));
        body.extend(chunk(b"XMP ", b"<x:xmpmeta/>"));
    }
    riff(&body)
}

/// An animation: `VP8X` with the animation flag, `ANIM` and `frame_count` frames
/// with `VP8L` images, dispose and blend bits from `bits`.
fn animation(images: &Images, flags: u8, frame_count: u32, bits: u8, canvas: u32) -> Vec<u8> {
    let mut body = chunk(b"VP8X", &vp8x_payload(flags | 0x02, canvas, canvas));
    body.extend(chunk(b"ANIM", &anim_payload(0xff00_ff00, 0)));
    for index in 0..frame_count {
        let frame = chunk(b"VP8L", &images.vp8l);
        let x = (index * 2) % canvas.max(2);
        let frame_bits = bits.wrapping_add(index as u8);
        body.extend(anmf(
            x & !1,
            0,
            1 + index % 7,
            1 + index % 5,
            3 + 2 * index,
            frame_bits,
            &frame,
        ));
    }
    riff(&body)
}

/// Every full-length and truncated run of `file`, with both values of allow_partial.
fn add_file_cases(batch: &mut Batch, file: &[u8], limit: usize) {
    batch.demux(file, false);
    batch.demux(file, true);
    let points = truncation_points(file);
    let stride = (points.len() / limit).max(1);
    for &size in points.iter().step_by(stride) {
        batch.demux(&file[..size], false);
        batch.demux(&file[..size], true);
    }
}

#[test]
fn version_matches_the_c_probe() {
    let mut batch = Batch::default();
    batch.add("version".to_owned(), version_report());
    batch.check("version_matches_the_c_probe");
}

#[test]
fn test_vectors_match_the_c_demuxer() {
    let mut batch = Batch::default();
    let vectors = oracle::test_vectors();
    assert!(!vectors.is_empty(), "the test data is checked out");
    for path in &vectors {
        let bytes = std::fs::read(path).expect("test vector");
        for allow_partial in [false, true] {
            batch.file(path, &bytes, bytes.len(), allow_partial);
        }
        let points = truncation_points(&bytes);
        let stride = (points.len() / 14).max(1);
        for &size in points.iter().step_by(stride) {
            batch.file(path, &bytes, size, false);
            batch.file(path, &bytes, size, true);
        }
    }
    batch.check("test_vectors_match_the_c_demuxer");
}

#[test]
fn still_images_with_every_flag_combination_match_the_c_demuxer() {
    let images = images();
    let mut batch = Batch::default();
    for flags in 0..=255_u8 {
        for lossless in [false, true] {
            let file = still_image(&images, flags, lossless, flags % 3 != 0);
            batch.demux(&file, false);
            batch.demux(&file, true);
            let points = truncation_points(&file);
            let stride = (points.len() / 6).max(1);
            for &size in points.iter().step_by(stride) {
                batch.demux(&file[..size], true);
            }
        }
    }
    batch.check("still_images_with_every_flag_combination_match_the_c_demuxer");
}

#[test]
fn animations_match_the_c_demuxer() {
    let images = images();
    let mut batch = Batch::default();
    let mut rng = Rng(0x5eed_0801);
    for flags in [0x00_u8, 0x04, 0x08, 0x10, 0x20, 0x3e, 0x01, 0x40] {
        for frame_count in [0_u32, 1, 2, 3, 6] {
            for bits in 0..4_u8 {
                let canvas = 1 + rng.below(40) as u32;
                let file = animation(&images, flags, frame_count, bits, canvas);
                add_file_cases(&mut batch, &file, 6);
            }
        }
    }
    // An animation whose frames do not fit the canvas, and one with metadata.
    let file = animation(&images, 0x3e, 3, 0, 4);
    add_file_cases(&mut batch, &file, 8);
    let mut body = chunk(b"VP8X", &vp8x_payload(0x3e | 0x02, 90, 90));
    body.extend(chunk(b"ANIM", &anim_payload(0, 3)));
    body.extend(chunk(b"ICCP", b"icc"));
    body.extend(anmf(0, 0, 80, 60, 1, 0x00, &chunk(b"ALPH", &images.alph)));
    body.extend(anmf(
        0,
        0,
        80,
        60,
        0xFF_FFFF,
        0x03,
        &chunk(b"VP8 ", &images.vp8),
    ));
    body.extend(chunk(b"XMP ", b"xmp"));
    add_file_cases(&mut batch, &riff(&body), 12);
    batch.check("animations_match_the_c_demuxer");
}

#[test]
fn malformed_chunks_match_the_c_demuxer() {
    let images = images();
    let vp8x = chunk(b"VP8X", &vp8x_payload(0x12, 64, 48));
    let mut cases: Vec<(&str, Vec<u8>)> = Vec::new();

    // Odd chunk sizes with and without their pad byte.
    let mut odd = vp8x.clone();
    odd.extend_from_slice(b"ICCP\x03\x00\x00\x00abc");
    odd.extend(chunk(b"VP8L", &images.vp8l));
    cases.push(("odd_without_pad", riff(&odd)));
    let mut odd = vp8x.clone();
    odd.extend(chunk(b"ICCP", b"abc"));
    odd.extend(chunk(b"VP8L", &images.vp8l));
    cases.push(("odd_with_pad", riff(&odd)));

    // Chunk sizes beyond the RIFF size, and a RIFF size smaller than the data.
    let mut beyond = vp8x.clone();
    beyond.extend_from_slice(b"EXIF\xff\xff\x00\x00exif");
    cases.push(("beyond_riff", riff(&beyond)));
    let mut body = vp8x.clone();
    body.extend(chunk(b"VP8L", &images.vp8l));
    body.extend(chunk(b"XMP ", b"trailing"));
    let full = body.len() as u32 + 4;
    cases.push(("riff_smaller", riff_with_size(full - 20, &body)));
    cases.push(("riff_larger", riff_with_size(full + 40, &body)));
    cases.push(("riff_minimal", riff_with_size(8, &body)));
    cases.push(("riff_below_minimum", riff_with_size(4, &body)));

    // Zero-sized metadata and unknown chunks.
    let mut zero = vp8x.clone();
    zero.extend(chunk(b"ICCP", b""));
    zero.extend(chunk(b"EXIF", b""));
    zero.extend(chunk(b"ZZZZ", b""));
    zero.extend(chunk(b"VP8L", &images.vp8l));
    cases.push(("zero_sized", riff(&zero)));

    // Duplicated VP8X and ANIM, ANMF before ANIM, short ANIM and ANMF.
    let mut dup_vp8x = vp8x.clone();
    dup_vp8x.extend(chunk(b"VP8X", &vp8x_payload(0x12, 64, 48)));
    dup_vp8x.extend(chunk(b"VP8L", &images.vp8l));
    cases.push(("duplicated_vp8x", riff(&dup_vp8x)));
    let mut dup_anim = chunk(b"VP8X", &vp8x_payload(0x02, 64, 48));
    dup_anim.extend(chunk(b"ANIM", &anim_payload(1, 1)));
    dup_anim.extend(chunk(b"ANIM", &anim_payload(2, 2)));
    dup_anim.extend(anmf(0, 0, 16, 16, 10, 0, &chunk(b"VP8L", &images.vp8l)));
    cases.push(("duplicated_anim", riff(&dup_anim)));
    let mut anmf_first = chunk(b"VP8X", &vp8x_payload(0x02, 64, 48));
    anmf_first.extend(anmf(0, 0, 16, 16, 10, 0, &chunk(b"VP8L", &images.vp8l)));
    anmf_first.extend(chunk(b"ANIM", &anim_payload(1, 1)));
    cases.push(("anmf_before_anim", riff(&anmf_first)));
    let mut short_anim = chunk(b"VP8X", &vp8x_payload(0x02, 64, 48));
    short_anim.extend(chunk(b"ANIM", &[1, 2, 3, 4]));
    cases.push(("short_anim", riff(&short_anim)));
    let mut short_anmf = chunk(b"VP8X", &vp8x_payload(0x02, 64, 48));
    short_anmf.extend(chunk(b"ANIM", &anim_payload(1, 1)));
    short_anmf.extend(chunk(b"ANMF", &[0; 9]));
    cases.push(("short_anmf", riff(&short_anmf)));
    let mut long_subchunk = chunk(b"VP8X", &vp8x_payload(0x02, 64, 48));
    long_subchunk.extend(chunk(b"ANIM", &anim_payload(1, 1)));
    let mut anmf_payload = vec![0_u8; 16];
    anmf_payload[6] = 1; // width - 1 low byte, so the frame has a width
    anmf_payload[9] = 1;
    anmf_payload.extend(chunk(b"VP8L", &images.vp8l[..images.vp8l.len().min(10)]));
    let mut long = Vec::new();
    long.extend_from_slice(b"ANMF");
    long.extend_from_slice(&(anmf_payload.len() as u32 - 2).to_le_bytes());
    long.extend_from_slice(&anmf_payload);
    long_subchunk.extend(long);
    cases.push(("anmf_subchunk_overflow", riff(&long_subchunk)));

    // Frames with unknown chunks first, two ALPH chunks, ALPH before a VP8L, and
    // a VP8 chunk that is not a bitstream.
    let mut frame_unknown = chunk(b"VP8X", &vp8x_payload(0x02, 64, 48));
    frame_unknown.extend(chunk(b"ANIM", &anim_payload(1, 1)));
    frame_unknown.extend(anmf(0, 0, 16, 16, 10, 0, &chunk(b"ZZZZ", b"unknown")));
    frame_unknown.extend(anmf(0, 0, 16, 16, 10, 0, &chunk(b"ALPH", &images.alph)));
    frame_unknown.extend(anmf(
        0,
        0,
        16,
        16,
        10,
        0,
        &[chunk(b"ALPH", &images.alph), chunk(b"ALPH", &images.alph)].concat(),
    ));
    frame_unknown.extend(anmf(
        0,
        0,
        16,
        16,
        10,
        0,
        &[chunk(b"ALPH", &images.alph), chunk(b"VP8L", &images.vp8l)].concat(),
    ));
    frame_unknown.extend(anmf(
        0,
        0,
        16,
        16,
        10,
        0,
        &chunk(b"VP8 ", b"not a bitstream at all"),
    ));
    frame_unknown.extend(anmf(
        0,
        0,
        16,
        16,
        10,
        0,
        &[chunk(b"VP8 ", &images.vp8), chunk(b"VP8 ", &images.vp8)].concat(),
    ));
    cases.push(("frame_chunk_variants", riff(&frame_unknown)));

    // The VP8X payload: too short, larger than the RIFF, reserved bits, huge canvas.
    let mut short_vp8x = chunk(b"VP8X", &[0x10, 0, 0]);
    short_vp8x.extend(chunk(b"VP8L", &images.vp8l));
    cases.push(("vp8x_too_short", riff(&short_vp8x)));
    let mut huge = chunk(b"VP8X", &vp8x_payload(0x10, 1 << 20, 1 << 20));
    huge.extend(chunk(b"VP8L", &images.vp8l));
    cases.push(("vp8x_canvas_area", riff(&huge)));
    let mut big_size = b"VP8X\xf0\xff\xff\xff".to_vec();
    big_size.extend(vp8x_payload(0x10, 64, 48));
    cases.push(("vp8x_size_beyond_riff", riff(&big_size)));
    let mut no_image = chunk(b"VP8X", &vp8x_payload(0x10, 64, 48));
    no_image.extend(chunk(b"ALPH", &images.alph));
    cases.push(("alpha_without_image", riff(&no_image)));
    let mut alpha_then_lossless = chunk(b"VP8X", &vp8x_payload(0x10, 64, 48));
    alpha_then_lossless.extend(chunk(b"ALPH", &images.alph));
    alpha_then_lossless.extend(chunk(b"VP8L", &images.vp8l));
    cases.push(("alpha_then_lossless", riff(&alpha_then_lossless)));

    // Files that do not start with a known chunk, or are not RIFF at all.
    cases.push(("first_chunk_alph", riff(&chunk(b"ALPH", &images.alph))));
    cases.push(("first_chunk_unknown", riff(&chunk(b"ZZZZ", b"zzzz"))));
    cases.push(("riff_wrong_form", {
        let mut bad = riff(&chunk(b"VP8L", &images.vp8l));
        bad[8..12].copy_from_slice(b"WEBQ");
        bad
    }));
    cases.push(("riff_wrong_magic", {
        let mut bad = riff(&chunk(b"VP8L", &images.vp8l));
        bad[..4].copy_from_slice(b"RIFX");
        bad
    }));
    cases.push(("simple_vp8l", riff(&chunk(b"VP8L", &images.vp8l))));
    cases.push(("simple_vp8", riff(&chunk(b"VP8 ", &images.vp8))));
    cases.push(("empty", Vec::new()));
    cases.push(("one_byte", vec![b'R']));

    let mut batch = Batch::default();
    for (name, file) in &cases {
        let _ = name;
        add_file_cases(&mut batch, file, 20);
    }
    batch.check("malformed_chunks_match_the_c_demuxer");
}

#[test]
fn raw_bitstreams_match_the_c_demuxer() {
    let images = images();
    let mut batch = Batch::default();
    for payload in [&images.vp8, &images.vp8l, &images.alph] {
        add_file_cases(&mut batch, payload, 30);
    }
    // A raw bitstream in a short prefix, and one with bytes after the image.
    let mut trailing = images.vp8l.clone();
    trailing.extend_from_slice(b"trailing bytes");
    add_file_cases(&mut batch, &trailing, 10);
    batch.check("raw_bitstreams_match_the_c_demuxer");
}

#[test]
fn random_mutations_match_the_c_demuxer() {
    let images = images();
    let mut rng = Rng(0x5eed_0802);
    let bases = [
        still_image(&images, 0x12, false, true),
        still_image(&images, 0x18, true, true),
        animation(&images, 0x3e, 3, 1, 40),
        riff(&chunk(b"VP8L", &images.vp8l)),
    ];
    let mut batch = Batch::default();
    for _ in 0..600 {
        let base = &bases[rng.below(bases.len() as u64) as usize];
        let mut file = base.clone();
        for _ in 0..1 + rng.below(4) {
            match rng.below(4) {
                0 => {
                    // Flip a bit anywhere, the RIFF size included.
                    let position = rng.below(file.len() as u64) as usize;
                    file[position] ^= 1 << rng.below(8);
                }
                1 => {
                    // Change the size of a top-level chunk, when the file has one.
                    let headers = chunk_headers(&file);
                    if !headers.is_empty() {
                        let position = headers[rng.below(headers.len() as u64) as usize];
                        let size = match rng.below(4) {
                            0 => 0,
                            1 => 1,
                            2 => 0xffff_ffff,
                            _ => rng.next_u64() as u32 % 512,
                        };
                        file[position + 4..position + 8].copy_from_slice(&size.to_le_bytes());
                    }
                }
                2 => {
                    // Cut the file short.
                    let cut = rng.below(file.len() as u64) as usize;
                    file.truncate(cut.max(1));
                }
                _ => {
                    // Overwrite a byte with a value that frames often compare with.
                    let position = rng.below(file.len() as u64) as usize;
                    file[position] = [0, 1, 0xff, 0x2f][rng.below(4) as usize];
                }
            }
            if file.is_empty() {
                file.push(0);
            }
        }
        batch.demux(&file, false);
        batch.demux(&file, true);
    }
    batch.check("random_mutations_match_the_c_demuxer");
}

#[test]
fn synthetic_frames_match_the_c_demuxer_for_every_bit_combination() {
    let images = images();
    let mut batch = Batch::default();
    // Blend and dispose bits of every frame, odd durations and odd payload sizes.
    for bits in 0..16_u8 {
        for duration in [0_u32, 1, 3, 255, 0xFFFF, 0xFF_FFFF] {
            let mut body = chunk(b"VP8X", &vp8x_payload(0x02, 40, 40));
            body.extend(chunk(b"ANIM", &anim_payload(0, 0)));
            body.extend(anmf(
                4,
                6,
                20,
                14,
                duration,
                bits,
                &chunk(b"VP8L", &images.vp8l[..images.vp8l.len() - 1]),
            ));
            body.extend(anmf(
                0,
                0,
                40,
                40,
                duration,
                bits ^ 0x03,
                &chunk(b"VP8 ", &images.vp8),
            ));
            batch.demux(&riff(&body), false);
            batch.demux(&riff(&body), true);
        }
    }
    batch.check("synthetic_frames_match_the_c_demuxer_for_every_bit_combination");
}

#[test]
fn the_wrappers_match_the_c_demuxer() {
    let images = images();
    let mut batch = Batch::default();
    for file in [
        still_image(&images, 0x12, false, true),
        riff(&chunk(b"VP8L", &images.vp8l)),
        Vec::new(),
    ] {
        batch.demux(&file, false);
        batch.demux(&file, true);
    }
    // The public names report the same results as WebPDemuxInternal above.
    batch.check("the_wrappers_match_the_c_demuxer");
}
