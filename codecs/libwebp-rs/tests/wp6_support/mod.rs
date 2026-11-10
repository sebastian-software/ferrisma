//! Shared helpers of the WP6 differential tests: a seeded generator, the hex and
//! digest formats of `tools/oracle/probes/wp6/support/wp6_io.c`, the line
//! comparison and the RIFF chunk lookup of the test vectors.
//!
//! Include it with `#[path = "wp6_support/mod.rs"] mod wp6_support;`.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use std::fmt::Write as _;
use std::path::Path;

/// SplitMix64, a small deterministic generator for the scripts.
pub struct Rng(pub u64);

impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// A value in `0..bound`.
    pub fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }

    /// A value in `low..=high`.
    pub fn range(&mut self, low: i64, high: i64) -> i64 {
        low + (self.next_u64() % ((high - low + 1) as u64)) as i64
    }

    pub fn byte(&mut self) -> u8 {
        self.next_u64() as u8
    }

    pub fn bytes(&mut self, count: usize) -> Vec<u8> {
        (0..count).map(|_| self.byte()).collect()
    }

    pub fn chance(&mut self, numerator: u64, denominator: u64) -> bool {
        self.below(denominator) < numerator
    }
}

/// The 64-bit FNV-1a digest of `wp6_io.c`.
pub fn digest(bytes: impl IntoIterator<Item = u8>) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

/// Lowercase hex, as `ProbePrintHex` prints it; `-` for no bytes in commands.
pub fn hex(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        write!(text, "{byte:02x}").unwrap();
    }
    text
}

/// A byte array as a command token: hex, or `-` when empty.
pub fn token(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "-".to_owned()
    } else {
        hex(bytes)
    }
}

/// Little-endian `int16` values as bytes.
pub fn int16_bytes(values: &[i16]) -> Vec<u8> {
    values
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect()
}

/// Prints a digest line as `Wp6PrintDigest` does.
pub fn print_digest(out: &mut String, key: &str, bytes: &[u8]) {
    writeln!(out, "{key}={:016x}", digest(bytes.iter().copied())).unwrap();
}

/// Compares the probe's output with the expected lines and panics at the
/// first difference, with the preceding lines for context.
pub fn compare(test: &str, actual: &[u8], expected: &str) {
    let actual = std::str::from_utf8(actual).expect("the probe prints UTF-8");
    let actual_lines: Vec<&str> = actual.lines().collect();
    let expected_lines: Vec<&str> = expected.lines().collect();
    for (index, (a, e)) in actual_lines.iter().zip(&expected_lines).enumerate() {
        if a != e {
            let start = index.saturating_sub(40);
            let context = actual_lines[start..index].join("\n");
            panic!(
                "{test}: line {index} differs\n--- context (C) ---\n{context}\n--- C ---\n{}\n--- Rust ---\n{}",
                truncate(a),
                truncate(e)
            );
        }
    }
    assert_eq!(
        actual_lines.len(),
        expected_lines.len(),
        "{test}: the C probe printed {} lines and the Rust translation {}",
        actual_lines.len(),
        expected_lines.len()
    );
}

fn truncate(line: &str) -> String {
    if line.len() > 300 {
        format!("{}... ({} bytes)", &line[..300], line.len())
    } else {
        line.to_owned()
    }
}

/// The payload offset and size of the first chunk named `fourcc` in a RIFF
/// WebP file.
pub fn find_chunk(file: &[u8], fourcc: &[u8; 4]) -> Option<(usize, usize)> {
    let mut pos = 12;
    while pos + 8 <= file.len() {
        let size = u32::from_le_bytes(file[pos + 4..pos + 8].try_into().unwrap()) as usize;
        if &file[pos..pos + 4] == fourcc {
            return Some((pos + 8, size));
        }
        pos += 8 + size + (size & 1);
    }
    None
}

/// Reads a test vector by file name.
pub fn read_vector(dir: &Path, name: &str) -> Vec<u8> {
    let path = dir.join(name);
    std::fs::read(&path).unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

use webp::dec::vp8_dec::VP8Io;
use webp::dec::vp8i_dec::VP8Decoder;
use webp::utils::bit_reader_utils::VP8BitReader;

/// `Wp6PrintBitReader`.
pub fn print_bit_reader(out: &mut String, prefix: &str, br: &VP8BitReader) {
    write!(
        out,
        "{prefix}={},{},{},{},{}",
        br.value,
        br.range,
        br.bits,
        i32::from(br.eof),
        i32::from(br.has_buf)
    )
    .unwrap();
    if br.has_buf {
        writeln!(out, ",{},{},{}", br.buf, br.buf_end, br.buf_max).unwrap();
    } else {
        writeln!(out, ",0,0,0").unwrap();
    }
}

/// `Wp6PrintIo`.
pub fn print_io(out: &mut String, io: &VP8Io) {
    writeln!(
        out,
        "io={},{},{},{},{},{},{},{},{}",
        io.width,
        io.height,
        io.mb_y,
        io.mb_w,
        io.mb_h,
        io.y_stride,
        io.uv_stride,
        i32::from(io.fancy_upsampling),
        i32::from(io.bypass_filtering)
    )
    .unwrap();
    writeln!(
        out,
        "io_crop={},{},{},{},{}",
        i32::from(io.use_cropping),
        io.crop_left,
        io.crop_right,
        io.crop_top,
        io.crop_bottom
    )
    .unwrap();
    writeln!(
        out,
        "io_scale={},{},{}",
        i32::from(io.use_scaling),
        io.scaled_width,
        io.scaled_height
    )
    .unwrap();
}

/// `Wp6PrintHeaders`.
pub fn print_headers(out: &mut String, dec: &VP8Decoder) {
    writeln!(out, "status={}", dec.status.0).unwrap();
    writeln!(out, "error={}", dec.error_msg.unwrap_or("(null)")).unwrap();
    writeln!(
        out,
        "ready={} incremental={}",
        i32::from(dec.ready),
        i32::from(dec.incremental)
    )
    .unwrap();
    let frm = &dec.frm_hdr;
    writeln!(
        out,
        "frm={},{},{},{}",
        frm.key_frame, frm.profile, frm.show, frm.partition_length
    )
    .unwrap();
    let pic = &dec.pic_hdr;
    writeln!(
        out,
        "pic={},{},{},{},{},{}",
        pic.width, pic.height, pic.xscale, pic.yscale, pic.colorspace, pic.clamp_type
    )
    .unwrap();
    let seg = &dec.segment_hdr;
    write!(
        out,
        "seg={},{},{}",
        i32::from(seg.use_segment),
        i32::from(seg.update_map),
        i32::from(seg.absolute_delta)
    )
    .unwrap();
    for s in 0..4 {
        write!(out, ",{},{}", seg.quantizer[s], seg.filter_strength[s]).unwrap();
    }
    writeln!(out).unwrap();
    let filter = &dec.filter_hdr;
    write!(
        out,
        "filter={},{},{},{}",
        i32::from(filter.simple),
        filter.level,
        filter.sharpness,
        i32::from(filter.use_lf_delta)
    )
    .unwrap();
    for delta in filter.ref_lf_delta.iter().chain(&filter.mode_lf_delta) {
        write!(out, ",{delta}").unwrap();
    }
    writeln!(out, " type={}", dec.filter_type).unwrap();
    writeln!(
        out,
        "mb={},{} parts={}",
        dec.mb_w, dec.mb_h, dec.num_parts_minus_one
    )
    .unwrap();
    for (s, m) in dec.dqm.iter().enumerate() {
        writeln!(
            out,
            "dqm{s}={},{},{},{},{},{},{},{}",
            m.y1_mat[0],
            m.y1_mat[1],
            m.y2_mat[0],
            m.y2_mat[1],
            m.uv_mat[0],
            m.uv_mat[1],
            m.uv_quant,
            m.dither
        )
        .unwrap();
    }
    writeln!(
        out,
        "segments={},{},{} skip={},{}",
        dec.proba.segments[0],
        dec.proba.segments[1],
        dec.proba.segments[2],
        i32::from(dec.use_skip_proba),
        dec.skip_p
    )
    .unwrap();
    let bands: Vec<u8> = dec
        .proba
        .bands
        .iter()
        .flatten()
        .flat_map(|band| band.probas.iter().flatten().copied())
        .collect();
    print_digest(out, "bands", &bands);
    if dec.ready {
        write!(out, "bands_ptr=").unwrap();
        for t in 0..4 {
            for b in 0..17 {
                write!(out, "{},", dec.proba.bands_ptr[t][b]).unwrap();
            }
        }
        writeln!(out).unwrap();
    }
    print_bit_reader(out, "br", &dec.br);
    for (s, part) in dec.parts.iter().enumerate() {
        print_bit_reader(out, &format!("part{s}"), part);
    }
}

/// The names of the lossy test vectors: every vector with a `VP8 ` chunk.
pub fn lossy_vectors(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .expect("the test data directory")
        .map(|entry| {
            entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .filter(|name| name.ends_with(".webp"))
        .filter(|name| {
            let file = read_vector(dir, name);
            find_chunk(&file, b"VP8 ").is_some()
        })
        .collect();
    names.sort();
    names
}
