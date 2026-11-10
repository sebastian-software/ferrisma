//! Shared helpers of the WP8 differential tests: the report that reproduces the
//! output of `tools/oracle/probes/wp8/demux.c` from the Rust translation, the
//! FNV-1a digest of that probe, the RIFF builders for synthetic files, and the
//! chunk walk that chooses truncation points in the test vectors.
//!
//! Include it with `#[path = "wp8_support/mod.rs"] mod wp8_support;`.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use std::borrow::Cow;
use std::fmt::Write as _;

use webp::demux::demux::{
    WebPDemuxer, webp_demux_get_chunk, webp_demux_get_frame, webp_demux_get_i, webp_demux_internal,
    webp_demux_next_chunk, webp_demux_next_frame, webp_demux_prev_chunk, webp_demux_prev_frame,
    webp_get_demux_version,
};
use webp::webp::demux::{
    WEBP_DEMUX_ABI_VERSION, WebPChunkIterator, WebPFormatFeature, WebPIterator, webp_demux,
    webp_demux_partial,
};
use webp::webp::mux_types::WebPData;

/// The tags that the probe walks, in its order (`kTags`).
pub const TAGS: [&[u8; 4]; 11] = [
    b"ICCP", b"EXIF", b"XMP ", b"ALPH", b"VP8 ", b"VP8L", b"VP8X", b"ANIM", b"ANMF", b"UNKN",
    b"ZZZZ",
];

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

    pub fn byte(&mut self) -> u8 {
        self.next_u64() as u8
    }
}

/// The 64-bit FNV-1a digest of the probe's `Digest`.
pub fn digest(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

/// Lowercase hex, as `ProbePrintHex` prints it.
pub fn hex(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        write!(text, "{byte:02x}").unwrap();
    }
    text
}

/// The hex token of a command: `-` for empty input.
pub fn hex_token(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "-".to_owned()
    } else {
        hex(bytes)
    }
}

/// The report of `PrintBytes(key, data, size)`.
fn print_bytes(out: &mut String, key: &str, bytes: &[u8]) {
    writeln!(out, "{key}.size={}", bytes.len()).unwrap();
    writeln!(out, "{key}.digest={:016x}", digest(bytes)).unwrap();
    let head = &bytes[..bytes.len().min(16)];
    writeln!(out, "{key}.head={}", hex(head)).unwrap();
}

/// The report of `PrintIterator`.
fn print_iterator(out: &mut String, prefix: &str, it: &WebPIterator<'_>) {
    writeln!(out, "{prefix}.frame_num={}", it.frame_num).unwrap();
    writeln!(out, "{prefix}.num_frames={}", it.num_frames).unwrap();
    writeln!(out, "{prefix}.x_offset={}", it.x_offset).unwrap();
    writeln!(out, "{prefix}.y_offset={}", it.y_offset).unwrap();
    writeln!(out, "{prefix}.width={}", it.width).unwrap();
    writeln!(out, "{prefix}.height={}", it.height).unwrap();
    writeln!(out, "{prefix}.duration={}", it.duration).unwrap();
    writeln!(out, "{prefix}.dispose={}", it.dispose_method.0 as i32).unwrap();
    writeln!(out, "{prefix}.complete={}", it.complete).unwrap();
    writeln!(out, "{prefix}.has_alpha={}", it.has_alpha).unwrap();
    writeln!(out, "{prefix}.blend={}", it.blend_method.0 as i32).unwrap();
    writeln!(out, "{prefix}.pad={},{}", it.pad[0], it.pad[1]).unwrap();
    print_bytes(out, &format!("{prefix}.fragment"), &it.fragment.bytes);
}

/// The report of `PrintChunk`.
fn print_chunk(out: &mut String, prefix: &str, it: &WebPChunkIterator<'_>) {
    writeln!(out, "{prefix}.chunk_num={}", it.chunk_num).unwrap();
    writeln!(out, "{prefix}.num_chunks={}", it.num_chunks).unwrap();
    print_bytes(out, &format!("{prefix}.chunk"), &it.chunk.bytes);
}

/// The report of `WalkFrames`.
fn walk_frames(out: &mut String, dmux: &WebPDemuxer<'_>, frame_count: u32) {
    let last = frame_count as i32 + 1;
    for k in 0..=last {
        match webp_demux_get_frame(dmux, k) {
            Some(it) => {
                writeln!(out, "get.{k}=1").unwrap();
                print_iterator(out, &format!("get.{k}"), &it);
            }
            None => writeln!(out, "get.{k}=0").unwrap(),
        }
    }

    let mut count = 0;
    if let Some(mut it) = webp_demux_get_frame(dmux, 1) {
        loop {
            print_iterator(out, &format!("fwd.{count}"), &it);
            count += 1;
            if !webp_demux_next_frame(&mut it) {
                break;
            }
        }
    }
    writeln!(out, "fwd.count={count}").unwrap();

    let mut count = 0;
    if let Some(mut it) = webp_demux_get_frame(dmux, 0) {
        loop {
            print_iterator(out, &format!("back.{count}"), &it);
            count += 1;
            if !webp_demux_prev_frame(&mut it) {
                break;
            }
        }
    }
    writeln!(out, "back.count={count}").unwrap();
}

/// The report of `WalkChunks`.
fn walk_chunks(out: &mut String, dmux: &WebPDemuxer<'_>) {
    for (tag_index, tag) in TAGS.iter().enumerate() {
        for k in 0..=5 {
            match webp_demux_get_chunk(dmux, **tag, k) {
                Some(it) => {
                    writeln!(out, "chunk.{tag_index}.get.{k}=1").unwrap();
                    print_chunk(out, &format!("chunk.{tag_index}.get.{k}"), &it);
                }
                None => writeln!(out, "chunk.{tag_index}.get.{k}=0").unwrap(),
            }
        }

        let mut count = 0;
        if let Some(mut it) = webp_demux_get_chunk(dmux, **tag, 1) {
            loop {
                print_chunk(out, &format!("chunk.{tag_index}.fwd.{count}"), &it);
                count += 1;
                if !webp_demux_next_chunk(&mut it) {
                    break;
                }
            }
        }
        writeln!(out, "chunk.{tag_index}.fwd.count={count}").unwrap();

        let mut count = 0;
        if let Some(mut it) = webp_demux_get_chunk(dmux, **tag, 0) {
            loop {
                print_chunk(out, &format!("chunk.{tag_index}.back.{count}"), &it);
                count += 1;
                if !webp_demux_prev_chunk(&mut it) {
                    break;
                }
            }
        }
        writeln!(out, "chunk.{tag_index}.back.count={count}").unwrap();
    }
}

/// The lines that the probe prints for one `demux` command.
pub fn report(bytes: &[u8], allow_partial: bool) -> String {
    let mut out = String::new();
    let data = WebPData {
        bytes: Cow::Borrowed(bytes),
    };

    let (dmux, state) = webp_demux_internal(&data, allow_partial, WEBP_DEMUX_ABI_VERSION);
    writeln!(out, "state={}", state.0).unwrap();
    if allow_partial {
        let (public, public_state) = webp_demux_partial(&data);
        writeln!(out, "public_state={}", public_state.0).unwrap();
        writeln!(out, "public={}", i32::from(public.is_some())).unwrap();
    } else {
        let public = webp_demux(&data);
        writeln!(out, "public={}", i32::from(public.is_some())).unwrap();
    }

    let Some(dmux) = dmux else {
        writeln!(out, "dmux=0").unwrap();
        return out;
    };
    writeln!(out, "dmux=1").unwrap();
    let feature = |feature: WebPFormatFeature| webp_demux_get_i(&dmux, feature);
    writeln!(
        out,
        "flags={}",
        feature(WebPFormatFeature::WEBP_FF_FORMAT_FLAGS)
    )
    .unwrap();
    writeln!(
        out,
        "canvas_width={}",
        feature(WebPFormatFeature::WEBP_FF_CANVAS_WIDTH)
    )
    .unwrap();
    writeln!(
        out,
        "canvas_height={}",
        feature(WebPFormatFeature::WEBP_FF_CANVAS_HEIGHT)
    )
    .unwrap();
    writeln!(
        out,
        "loop_count={}",
        feature(WebPFormatFeature::WEBP_FF_LOOP_COUNT)
    )
    .unwrap();
    writeln!(
        out,
        "bgcolor={}",
        feature(WebPFormatFeature::WEBP_FF_BACKGROUND_COLOR)
    )
    .unwrap();
    let frame_count = feature(WebPFormatFeature::WEBP_FF_FRAME_COUNT);
    writeln!(out, "frame_count={frame_count}").unwrap();
    writeln!(out, "feature6={}", feature(WebPFormatFeature(6))).unwrap();
    writeln!(out, "feature99={}", feature(WebPFormatFeature(99))).unwrap();
    walk_frames(&mut out, &dmux, frame_count);
    walk_chunks(&mut out, &dmux);
    out
}

/// The line that the probe prints for `version`.
pub fn version_report() -> String {
    format!("version={}\n", webp_get_demux_version())
}

/// Encodes a 24-bit little-endian value.
pub fn le24(value: u32) -> [u8; 3] {
    [value as u8, (value >> 8) as u8, (value >> 16) as u8]
}

/// One RIFF chunk: tag, size, payload and the pad byte of an odd payload.
pub fn chunk(tag: &[u8; 4], payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + payload.len() + 1);
    out.extend_from_slice(tag);
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(payload);
    if payload.len() % 2 == 1 {
        out.push(0);
    }
    out
}

/// A RIFF file over `body` whose size field is `riff_size`.
pub fn riff_with_size(riff_size: u32, body: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(12 + body.len());
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&riff_size.to_le_bytes());
    out.extend_from_slice(b"WEBP");
    out.extend_from_slice(body);
    out
}

/// A RIFF file over `body` whose size field is exact.
pub fn riff(body: &[u8]) -> Vec<u8> {
    riff_with_size(4 + body.len() as u32, body)
}

/// The payload of a `VP8X` chunk.
pub fn vp8x_payload(flags: u8, canvas_width: u32, canvas_height: u32) -> Vec<u8> {
    let mut out = vec![flags, 0, 0, 0];
    out.extend_from_slice(&le24(canvas_width - 1));
    out.extend_from_slice(&le24(canvas_height - 1));
    out
}

/// The payload of an `ANIM` chunk.
pub fn anim_payload(background: u32, loops: u16) -> Vec<u8> {
    let mut out = background.to_le_bytes().to_vec();
    out.extend_from_slice(&loops.to_le_bytes());
    out
}

/// The payload of an `ANMF` chunk followed by its frame chunks.
pub fn anmf(
    x_offset: u32,
    y_offset: u32,
    width: u32,
    height: u32,
    duration: u32,
    flags: u8,
    frame_chunks: &[u8],
) -> Vec<u8> {
    let mut payload = Vec::with_capacity(16 + frame_chunks.len());
    payload.extend_from_slice(&le24(x_offset / 2));
    payload.extend_from_slice(&le24(y_offset / 2));
    payload.extend_from_slice(&le24(width - 1));
    payload.extend_from_slice(&le24(height - 1));
    payload.extend_from_slice(&le24(duration));
    payload.push(flags);
    payload.extend_from_slice(frame_chunks);
    chunk(b"ANMF", &payload)
}

/// The offsets of the top-level chunk headers of a file whose RIFF header is
/// intact, walked as the sizes say. Empty for any other file.
pub fn chunk_headers(file: &[u8]) -> Vec<usize> {
    let mut headers = Vec::new();
    if file.len() < 12 || &file[..4] != b"RIFF" || &file[8..12] != b"WEBP" {
        return headers;
    }
    let mut position = 12;
    while position + 8 <= file.len() {
        headers.push(position);
        let size =
            u32::from_le_bytes(file[position + 4..position + 8].try_into().unwrap()) as usize;
        position += 8 + size + (size & 1);
    }
    headers
}

/// The chunks at the top level of a RIFF file, as `(tag, payload)` pairs; the
/// payload is cut at the end of the file. Empty for any other file.
pub fn top_level_chunks(file: &[u8]) -> Vec<([u8; 4], Vec<u8>)> {
    chunk_headers(file)
        .into_iter()
        .map(|position| {
            let tag: [u8; 4] = file[position..position + 4].try_into().unwrap();
            let size =
                u32::from_le_bytes(file[position + 4..position + 8].try_into().unwrap()) as usize;
            let end = (position + 8 + size).min(file.len());
            (tag, file[position + 8..end].to_vec())
        })
        .collect()
}

/// Positions at which a file is cut for the partial states: the chunk
/// boundaries of its top level, with their neighbours, and the middle of each
/// payload. Sorted and without duplicates; every position is below `file.len()`.
pub fn truncation_points(file: &[u8]) -> Vec<usize> {
    let mut points = vec![0, 1, 4, 8, 11, 12, 16, 19, 20, 21, 27, 28, 29];
    let mut position = 12;
    while position + 8 <= file.len() {
        let size =
            u32::from_le_bytes(file[position + 4..position + 8].try_into().unwrap()) as usize;
        points.extend([
            position,
            position + 1,
            position + 4,
            position + 7,
            position + 8,
        ]);
        if size > 1 {
            points.push(position + 8 + size / 2);
        }
        points.extend([position + 8 + size, position + 8 + size + 1]);
        let next = position + 8 + size + (size & 1);
        if next <= position {
            break;
        }
        position = next;
    }
    points.push(file.len().saturating_sub(1));
    points.retain(|&point| point < file.len());
    points.sort_unstable();
    points.dedup();
    points
}

/// The first chunk payload with `tag` in the test vectors, from the first file
/// in `files` that has one at the top level, or `None` when none does.
pub fn payload_of(files: &[Vec<u8>], tag: &[u8; 4]) -> Option<Vec<u8>> {
    files.iter().find_map(|file| {
        top_level_chunks(file)
            .into_iter()
            .find(|(found, _)| found == tag)
            .map(|(_, payload)| payload)
    })
}
