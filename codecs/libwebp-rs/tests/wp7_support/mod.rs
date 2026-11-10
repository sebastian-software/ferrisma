//! Shared helpers of the WP7 differential tests: the state printers that
//! reproduce `tools/oracle/probes/wp7/support/wp7_state.c` from the Rust
//! translation, the digest of `wp7_io.c`, the line comparison, a seeded
//! generator and the RIFF chunk lookup of the test vectors.
//!
//! Include it with `#[path = "wp7_support/mod.rs"] mod wp7_support;`.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use std::fmt::Write as _;
use std::path::Path;

use webp::dec::vp8_dec::VP8Io;
use webp::dec::vp8li_dec::VP8LDecoder;
use webp::dsp::lossless_common::vp8l_sub_sample_size;
use webp::utils::bit_reader_utils::VP8LBitReader;
use webp::utils::huffman_utils::{HTreeGroup, HuffmanCode, HuffmanTables};

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
}

/// The 64-bit FNV-1a digest of `wp7_io.c`.
pub fn digest(bytes: impl IntoIterator<Item = u8>) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

/// The digest of 32-bit words in little-endian byte order.
pub fn words_digest(words: &[u32]) -> u64 {
    digest(words.iter().flat_map(|word| word.to_le_bytes()))
}

/// Lowercase hex, as `ProbePrintHex` prints it.
pub fn hex(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        write!(text, "{byte:02x}").unwrap();
    }
    text
}

/// Prints a digest line as `Wp7PrintDigest` does.
pub fn print_digest(out: &mut String, key: &str, bytes: &[u8]) {
    writeln!(out, "{key}={:016x}", digest(bytes.iter().copied())).unwrap();
}

/// `Wp7PrintBitReader`.
pub fn print_bit_reader(out: &mut String, prefix: &str, br: &VP8LBitReader) {
    writeln!(out, "{prefix}_val={}", br.val).unwrap();
    writeln!(out, "{prefix}_len={}", br.len).unwrap();
    writeln!(out, "{prefix}_pos={}", br.pos).unwrap();
    writeln!(out, "{prefix}_bit_pos={}", br.bit_pos).unwrap();
    writeln!(out, "{prefix}_eos={}", i32::from(br.eos)).unwrap();
}

fn codes_digest(codes: &[HuffmanCode]) -> u64 {
    digest(
        codes
            .iter()
            .flat_map(|code| [code.bits, code.value as u8, (code.value >> 8) as u8]),
    )
}

/// `Wp7PrintTables`.
pub fn print_tables(out: &mut String, tables: &HuffmanTables) {
    writeln!(out, "segments={}", 1 + tables.next_segments.len()).unwrap();
    if !tables.root.start.is_empty() {
        writeln!(out, "current={}", tables.curr_segment).unwrap();
    }
    for segment in std::iter::once(&tables.root).chain(&tables.next_segments) {
        let has_start = !segment.start.is_empty();
        let used = if has_start { segment.curr_table } else { 0 };
        writeln!(
            out,
            "segment={},{},{}",
            segment.size,
            used,
            i32::from(has_start)
        )
        .unwrap();
        if has_start {
            writeln!(out, "codes={:016x}", codes_digest(&segment.start[..used])).unwrap();
        }
    }
}

/// `Wp7PrintGroups`; `None` is C's `NULL` array.
pub fn print_groups(out: &mut String, groups: Option<&[HTreeGroup]>, count: i32) {
    let Some(groups) = groups else {
        writeln!(out, "groups=-1").unwrap();
        return;
    };
    writeln!(out, "groups={count}").unwrap();
    for group in &groups[..count as usize] {
        for tree in &group.htrees {
            writeln!(out, "tree={},{}", tree.segment, tree.offset).unwrap();
        }
        writeln!(
            out,
            "trivial_literal={}",
            i32::from(group.is_trivial_literal)
        )
        .unwrap();
        if group.is_trivial_literal {
            writeln!(out, "literal_arb={}", group.literal_arb).unwrap();
        }
        writeln!(out, "trivial_code={}", i32::from(group.is_trivial_code)).unwrap();
        writeln!(out, "packed={}", i32::from(group.use_packed_table)).unwrap();
        if group.use_packed_table {
            let bytes = group.packed_table.iter().flat_map(|code| {
                let mut word = [0_u8; 8];
                word[..4].copy_from_slice(&(code.bits as u32).to_le_bytes());
                word[4..].copy_from_slice(&code.value.to_le_bytes());
                word
            });
            writeln!(out, "packed_table={:016x}", digest(bytes)).unwrap();
        }
    }
}

/// `Wp7PrintDecoder`.
pub fn print_decoder(out: &mut String, dec: &VP8LDecoder) {
    writeln!(out, "status={}", dec.status.0).unwrap();
    writeln!(out, "state={}", dec.state.0).unwrap();
    writeln!(out, "dec_width={}", dec.width).unwrap();
    writeln!(out, "dec_height={}", dec.height).unwrap();
    writeln!(out, "last_row={}", dec.last_row).unwrap();
    writeln!(out, "last_pixel={}", dec.last_pixel).unwrap();
    writeln!(out, "last_out_row={}", dec.last_out_row).unwrap();
    writeln!(out, "incremental={}", i32::from(dec.incremental)).unwrap();
    writeln!(out, "next_transform={}", dec.next_transform).unwrap();
    writeln!(out, "transforms_seen={}", dec.transforms_seen).unwrap();
    let has_pixels = !dec.pixels.is_empty() || !dec.pixels_8b.is_empty();
    writeln!(out, "has_pixels={}", i32::from(has_pixels)).unwrap();
    writeln!(out, "has_rescaler={}", i32::from(dec.rescaler.is_some())).unwrap();
    print_bit_reader(out, "br", &dec.br);
}

/// `Wp7PrintDecoderFull`.
pub fn print_decoder_full(out: &mut String, dec: &VP8LDecoder) {
    print_decoder(out, dec);
    for transform in &dec.transforms[..dec.next_transform as usize] {
        writeln!(
            out,
            "transform={},{},{},{}",
            transform.r#type.0, transform.bits, transform.xsize, transform.ysize
        )
        .unwrap();
        writeln!(out, "transform_size={}", transform.data.len()).unwrap();
        writeln!(out, "transform_data={:016x}", words_digest(&transform.data)).unwrap();
    }
    let hdr = &dec.hdr;
    writeln!(out, "color_cache_size={}", hdr.color_cache_size).unwrap();
    writeln!(out, "cache_bits={}", hdr.color_cache.hash_bits).unwrap();
    writeln!(out, "cache_shift={}", hdr.color_cache.hash_shift).unwrap();
    if !hdr.color_cache.colors.is_empty() {
        let count = 1_usize << hdr.color_cache.hash_bits;
        writeln!(
            out,
            "cache={:016x}",
            words_digest(&hdr.color_cache.colors[..count])
        )
        .unwrap();
    }
    writeln!(
        out,
        "saved_cache={}",
        i32::from(!hdr.saved_color_cache.colors.is_empty())
    )
    .unwrap();
    writeln!(out, "huffman_mask={}", hdr.huffman_mask).unwrap();
    writeln!(out, "huffman_bits={}", hdr.huffman_subsample_bits).unwrap();
    writeln!(out, "huffman_xsize={}", hdr.huffman_xsize).unwrap();
    if !hdr.huffman_image.is_empty() && hdr.huffman_subsample_bits > 0 {
        let rows = vp8l_sub_sample_size(dec.height as u32, hdr.huffman_subsample_bits as u32);
        let count = hdr.huffman_xsize as usize * rows as usize;
        writeln!(
            out,
            "huffman_image={:016x}",
            words_digest(&hdr.huffman_image[..count])
        )
        .unwrap();
    } else {
        writeln!(
            out,
            "huffman_image={}",
            i32::from(!hdr.huffman_image.is_empty())
        )
        .unwrap();
    }
    writeln!(out, "num_htree_groups={}", hdr.num_htree_groups).unwrap();
    let groups = (!hdr.htree_groups.is_empty()).then_some(hdr.htree_groups.as_slice());
    print_groups(out, groups, hdr.num_htree_groups);
    print_tables(out, &hdr.huffman_tables);
}

/// `Wp7PrintIo`.
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

/// The canvas width and height of the VP8X chunk of a WebP file.
pub fn canvas_size(file: &[u8]) -> Option<(i32, i32)> {
    let (offset, size) = find_chunk(file, b"VP8X")?;
    assert!(size >= 10);
    let read24 = |pos: usize| {
        i32::from(file[pos]) | (i32::from(file[pos + 1]) << 8) | (i32::from(file[pos + 2]) << 16)
    };
    Some((1 + read24(offset + 4), 1 + read24(offset + 7)))
}

/// Reads a test vector by file name.
pub fn read_vector(dir: &Path, name: &str) -> Vec<u8> {
    let path = dir.join(name);
    std::fs::read(&path).unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}
