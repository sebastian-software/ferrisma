//! Shared helpers of the WP14 differential tests: the animated inputs and the
//! report that reproduces the `anim` command of
//! `tools/oracle/probes/wp14/anim.c` with the Rust translation.
//!
//! The inputs come from two sources, because `upstream/libwebp-test-data` has
//! no animated file:
//!
//! - hand-assembled containers (the RIFF builders of `tests/wp8_support`)
//!   around frame bitstreams from the translation's `webp_encode_rgba` and
//!   `webp_encode_lossless_rgba`, which WP13 showed byte-identical to C, so the
//!   frames need no oracle;
//! - the oracle's own `img2webp` on generated PAM frame sequences, so that the
//!   animation encoder's sub-rectangle, blend and dispose choices are covered.
//!
//! Include it after `wp8_support` and `oracle`:
//! `#[path = "wp8_support/mod.rs"] mod wp8_support;`,
//! `#[path = "support/oracle.rs"] mod oracle;` and
//! `#[path = "wp14_support/mod.rs"] mod wp14_support;`.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use std::borrow::Cow;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

use webp::demux::anim_decode::{
    webp_anim_decoder_get_demuxer, webp_anim_decoder_get_info, webp_anim_decoder_get_next,
    webp_anim_decoder_has_more_frames, webp_anim_decoder_reset,
};
use webp::demux::demux::{
    WebPDemuxer, webp_demux_get_chunk, webp_demux_get_i, webp_demux_next_chunk,
};
use webp::enc::picture_enc::{webp_encode_lossless_rgba, webp_encode_rgba};
use webp::webp::decode::WEBP_CSP_MODE;
use webp::webp::demux::{
    WebPAnimDecoderOptions, WebPFormatFeature, webp_anim_decoder_new,
    webp_anim_decoder_options_init,
};
use webp::webp::mux_types::WebPData;

use crate::oracle::Oracle;
use crate::wp8_support::{
    Rng, anim_payload, anmf, chunk, digest, hex, riff, top_level_chunks, vp8x_payload,
};

/// The valid color modes, in the order the tests cycle through them.
pub const MODES: [i32; 4] = [1, 3, 7, 8]; // MODE_RGBA, MODE_BGRA, MODE_rgbA, MODE_bgrA

/// Color modes that `WebPAnimDecoderNew` refuses, and -1 for `NULL` options.
pub const OTHER_MODES: [i32; 13] = [-1, 0, 2, 4, 5, 6, 9, 10, 11, 12, 13, 99, 1000];

/// The directory for the files of one test, per process, so that runs at the
/// same time do not share it. It is removed when the test passes and kept for
/// inspection when it fails.
pub struct WorkDir(PathBuf);

impl WorkDir {
    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl std::ops::Deref for WorkDir {
    type Target = Path;

    fn deref(&self) -> &Path {
        &self.0
    }
}

impl Drop for WorkDir {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
}

/// A fresh [`WorkDir`] named after the test.
pub fn work_dir(name: &str) -> WorkDir {
    let directory =
        PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    WorkDir(directory)
}

//------------------------------------------------------------------------------
// Hand-assembled animations

/// The alpha channel of a generated frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alpha {
    /// Every pixel opaque: lossy frames have no `ALPH` chunk.
    Opaque,
    /// Opaque and fully transparent pixels.
    Binary,
    /// Alpha values over the whole range.
    Gradient,
}

/// How a frame is compressed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Codec {
    Lossy(f32),
    Lossless,
}

/// One frame of a hand-assembled animation.
#[derive(Clone, Copy, Debug)]
pub struct FrameSpec {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub duration: u32,
    pub dispose_background: bool,
    pub no_blend: bool,
    pub codec: Codec,
    pub alpha: Alpha,
    pub seed: u64,
}

impl FrameSpec {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        FrameSpec {
            x,
            y,
            width,
            height,
            duration: 40,
            dispose_background: false,
            no_blend: false,
            codec: Codec::Lossless,
            alpha: Alpha::Gradient,
            seed: u64::from(x * 131 + y * 17 + width * 7 + height),
        }
    }
}

/// A hand-assembled animation.
#[derive(Clone, Debug)]
pub struct AnimSpec {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub background: u32,
    pub loops: u16,
    pub frames: Vec<FrameSpec>,
    /// Adds an `ICCP` chunk before `ANIM`, and two `EXIF` and one `XMP ` chunk
    /// after the frames.
    pub metadata: bool,
    /// The VP8X animation flag; without it the frames are not animation frames.
    pub animation_flag: bool,
}

impl AnimSpec {
    pub fn new(canvas_width: u32, canvas_height: u32, frames: Vec<FrameSpec>) -> Self {
        AnimSpec {
            canvas_width,
            canvas_height,
            background: 0xff20_4080,
            loops: 0,
            frames,
            metadata: false,
            animation_flag: true,
        }
    }
}

/// RGBA samples of a `width` by `height` frame: gradients, a checkerboard and
/// a disc, varied by `seed`, with the alpha channel `alpha`.
pub fn pattern(width: u32, height: u32, seed: u64, alpha: Alpha) -> Vec<u8> {
    let mut rng = Rng(seed);
    let base = [rng.byte(), rng.byte(), rng.byte()];
    let cx = rng.below(u64::from(width)) as i64;
    let cy = rng.below(u64::from(height)) as i64;
    let radius = 1 + rng.below(u64::from(width.max(height))) as i64;
    let mut out = Vec::with_capacity((width * height * 4) as usize);
    for y in 0..i64::from(height) {
        for x in 0..i64::from(width) {
            let inside = (x - cx).pow(2) + (y - cy).pow(2) <= radius * radius;
            let checker = ((x / 3 + y / 3) % 2) as u8;
            let r = base[0]
                .wrapping_add((x * 9) as u8)
                .wrapping_add(checker * 40);
            let g = base[1].wrapping_add((y * 11) as u8);
            let b = if inside { 255 - base[2] } else { base[2] };
            let a = match alpha {
                Alpha::Opaque => 255,
                Alpha::Binary => {
                    if inside {
                        255
                    } else {
                        0
                    }
                }
                Alpha::Gradient => (x * 255 / i64::from(width.max(2) - 1)) as u8 ^ (checker * 0x30),
            };
            out.extend_from_slice(&[r, g, b, a]);
        }
    }
    out
}

/// The frame chunks (`ALPH` and `VP8 `, or `VP8L`) of an encoded frame.
pub fn frame_chunks(frame: &FrameSpec) -> Vec<u8> {
    let rgba = pattern(frame.width, frame.height, frame.seed, frame.alpha);
    let width = frame.width as i32;
    let height = frame.height as i32;
    let file = match frame.codec {
        Codec::Lossy(quality) => webp_encode_rgba(&rgba, width, height, width * 4, quality),
        Codec::Lossless => webp_encode_lossless_rgba(&rgba, width, height, width * 4),
    }
    .expect("the translation encodes the frame");
    let mut out = Vec::new();
    for (tag, payload) in top_level_chunks(&file) {
        if &tag == b"ALPH" || &tag == b"VP8 " || &tag == b"VP8L" {
            out.extend_from_slice(&chunk(&tag, &payload));
        }
    }
    out
}

/// The `ANMF` chunk of a frame around the given frame chunks.
pub fn anmf_chunk(frame: &FrameSpec, frame_chunks: &[u8]) -> Vec<u8> {
    let flags = u8::from(frame.dispose_background) | (u8::from(frame.no_blend) << 1);
    anmf(
        frame.x,
        frame.y,
        frame.width,
        frame.height,
        frame.duration,
        flags,
        frame_chunks,
    )
}

/// The metadata chunks of `AnimSpec::metadata`.
fn metadata_chunks() -> (Vec<u8>, Vec<u8>) {
    let iccp = chunk(b"ICCP", b"profile bytes, not parsed");
    let mut tail = chunk(b"EXIF", b"Exif\0\0first");
    tail.extend_from_slice(&chunk(b"EXIF", b"second EXIF chunk!"));
    tail.extend_from_slice(&chunk(b"XMP ", b"<x:xmpmeta/>"));
    (iccp, tail)
}

/// Assembles the animation with the frames' chunks as given.
pub fn assemble(spec: &AnimSpec, frames: &[Vec<u8>]) -> Vec<u8> {
    let has_alpha = spec.frames.iter().any(|frame| frame.alpha != Alpha::Opaque);
    let mut flags = 0u8;
    if spec.animation_flag {
        flags |= 0x02;
    }
    if has_alpha {
        flags |= 0x10;
    }
    let (iccp, tail) = metadata_chunks();
    if spec.metadata {
        flags |= 0x20 | 0x08 | 0x04;
    }
    let mut body = chunk(
        b"VP8X",
        &vp8x_payload(flags, spec.canvas_width, spec.canvas_height),
    );
    if spec.metadata {
        body.extend_from_slice(&iccp);
    }
    body.extend_from_slice(&chunk(b"ANIM", &anim_payload(spec.background, spec.loops)));
    for (frame, chunks) in spec.frames.iter().zip(frames) {
        body.extend_from_slice(&anmf_chunk(frame, chunks));
    }
    if spec.metadata {
        body.extend_from_slice(&tail);
    }
    riff(&body)
}

/// Encodes the frames and assembles the animation.
pub fn build(spec: &AnimSpec) -> Vec<u8> {
    let frames: Vec<Vec<u8>> = spec.frames.iter().map(frame_chunks).collect();
    assemble(spec, &frames)
}

/// The frame chunks with the image payload cut to `keep` of its bytes (and the
/// chunk sizes adjusted), so that the container stays valid and the decoding of
/// the frame fails or ends early.
pub fn truncate_image(chunks: &[u8], keep: fn(usize) -> usize) -> Vec<u8> {
    let mut out = Vec::new();
    let mut position = 0;
    while position + 8 <= chunks.len() {
        let tag: [u8; 4] = chunks[position..position + 4].try_into().unwrap();
        let size =
            u32::from_le_bytes(chunks[position + 4..position + 8].try_into().unwrap()) as usize;
        let payload = &chunks[position + 8..position + 8 + size];
        if &tag == b"VP8 " || &tag == b"VP8L" {
            out.extend_from_slice(&chunk(&tag, &payload[..keep(size)]));
        } else {
            out.extend_from_slice(&chunk(&tag, payload));
        }
        position += 8 + size + (size & 1);
    }
    out
}

/// The frame chunks with every byte of the `ALPH` payload after its header byte
/// replaced, so that the alpha plane does not decode.
pub fn corrupt_alpha(chunks: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut position = 0;
    while position + 8 <= chunks.len() {
        let tag: [u8; 4] = chunks[position..position + 4].try_into().unwrap();
        let size =
            u32::from_le_bytes(chunks[position + 4..position + 8].try_into().unwrap()) as usize;
        let mut payload = chunks[position + 8..position + 8 + size].to_vec();
        if &tag == b"ALPH" {
            // Compression method 1 (lossless) with a VP8L stream of 0xff bytes.
            payload[0] = 1;
            for byte in &mut payload[1..] {
                *byte = 0xff;
            }
        }
        out.extend_from_slice(&chunk(&tag, &payload));
        position += 8 + size + (size & 1);
    }
    out
}

/// A frame rectangle: x, y, width and height.
type Rect = (u32, u32, u32, u32);

/// The hand-assembled animations, with names.
#[expect(clippy::too_many_lines, reason = "one list of cases")]
pub fn synthetic_animations() -> Vec<(String, Vec<u8>)> {
    let mut out: Vec<(String, Vec<u8>)> = Vec::new();

    // Every combination of the previous frame's dispose method and size, the
    // current frame's blend method, size and alpha, over four placements of the
    // current frame relative to the previous one: wider on both sides, inside,
    // disjoint, and overlapping on the left only. Frame 1 fills the canvas.
    let placements: [(&str, Rect, Rect); 4] = [
        ("wider", (6, 4, 10, 8), (2, 2, 19, 11)),
        ("inside", (2, 2, 20, 12), (6, 4, 7, 5)),
        ("disjoint", (0, 0, 9, 7), (12, 8, 11, 7)),
        ("left", (10, 2, 13, 10), (4, 6, 9, 9)),
    ];
    for (place, prev_rect, cur_rect) in placements {
        for prev_dispose in [false, true] {
            for prev_full in [false, true] {
                for cur_no_blend in [false, true] {
                    for cur_full in [false, true] {
                        for cur_alpha in [Alpha::Opaque, Alpha::Gradient] {
                            let (canvas_width, canvas_height) = (24, 16);
                            let mut first = FrameSpec::new(0, 0, canvas_width, canvas_height);
                            first.alpha = Alpha::Binary;
                            first.codec = Codec::Lossy(70.0);
                            let (x, y, w, h) = if prev_full {
                                (0, 0, canvas_width, canvas_height)
                            } else {
                                prev_rect
                            };
                            let mut prev = FrameSpec::new(x, y, w, h);
                            prev.dispose_background = prev_dispose;
                            prev.alpha = Alpha::Gradient;
                            let (x, y, w, h) = if cur_full {
                                (0, 0, canvas_width, canvas_height)
                            } else {
                                cur_rect
                            };
                            let mut cur = FrameSpec::new(x, y, w, h);
                            cur.no_blend = cur_no_blend;
                            cur.alpha = cur_alpha;
                            cur.codec = if cur_alpha == Alpha::Opaque {
                                Codec::Lossy(80.0)
                            } else {
                                Codec::Lossless
                            };
                            let mut last = FrameSpec::new(2, 2, 5, 3);
                            last.alpha = Alpha::Binary;
                            let spec = AnimSpec::new(
                                canvas_width,
                                canvas_height,
                                vec![first, prev, cur, last],
                            );
                            out.push((
                                format!(
                                    "combo_{place}_d{}_f{}_nb{}_cf{}_{:?}",
                                    u8::from(prev_dispose),
                                    u8::from(prev_full),
                                    u8::from(cur_no_blend),
                                    u8::from(cur_full),
                                    cur_alpha
                                ),
                                build(&spec),
                            ));
                        }
                    }
                }
            }
        }
    }

    // A keyframe chain: a disposed keyframe sub-rectangle makes the next frame a
    // keyframe (prev_frame_was_keyframe), and full frames with NO_BLEND are
    // keyframes even with alpha.
    {
        let mut a = FrameSpec::new(4, 2, 10, 8);
        a.dispose_background = true;
        let mut b = FrameSpec::new(8, 6, 12, 8);
        b.dispose_background = true;
        let mut c = FrameSpec::new(0, 0, 24, 16);
        c.no_blend = true;
        let mut d = FrameSpec::new(2, 4, 6, 6);
        d.dispose_background = true;
        let e = FrameSpec::new(0, 0, 24, 16);
        let spec = AnimSpec::new(24, 16, vec![a, b, c, d, e]);
        out.push(("keyframe_chain".to_owned(), build(&spec)));
    }

    // Frames touching the canvas edges, 1-pixel frames, odd sizes and offsets
    // that are odd multiples of two.
    {
        let (cw, ch) = (31, 17);
        let mut frames = vec![
            FrameSpec::new(0, 0, 31, 17),
            FrameSpec::new(30, 16, 1, 1),
            FrameSpec::new(0, 16, 31, 1),
            FrameSpec::new(30, 0, 1, 17),
            FrameSpec::new(2, 6, 29, 11),
            FrameSpec::new(6, 10, 3, 5),
            FrameSpec::new(0, 0, 1, 1),
        ];
        for (index, frame) in frames.iter_mut().enumerate() {
            frame.dispose_background = index % 3 == 1;
            frame.no_blend = index % 4 == 2;
            frame.codec = if index % 2 == 0 {
                Codec::Lossless
            } else {
                Codec::Lossy(60.0)
            };
            frame.alpha = [Alpha::Gradient, Alpha::Binary, Alpha::Opaque][index % 3];
        }
        out.push(("edges".to_owned(), build(&AnimSpec::new(cw, ch, frames))));
    }

    // Durations 0, 1 and the largest, loop counts and background colors.
    for (index, (loops, background)) in [(0u16, 0u32), (1, 0xffff_ffff), (65535, 0x8040_2010)]
        .into_iter()
        .enumerate()
    {
        let mut frames = vec![
            FrameSpec::new(0, 0, 16, 12),
            FrameSpec::new(2, 2, 8, 6),
            FrameSpec::new(4, 4, 12, 8),
        ];
        frames[0].duration = 0;
        frames[1].duration = 1;
        frames[2].duration = 0x00ff_ffff;
        frames[index].alpha = Alpha::Opaque;
        frames[index].codec = Codec::Lossy(90.0);
        let mut spec = AnimSpec::new(16, 12, frames);
        spec.loops = loops;
        spec.background = background;
        out.push((format!("durations_{index}"), build(&spec)));
    }

    // A single frame, smaller than the canvas, and one that fills it.
    out.push((
        "single_small".to_owned(),
        build(&AnimSpec::new(20, 14, vec![FrameSpec::new(4, 2, 9, 7)])),
    ));
    {
        let mut frame = FrameSpec::new(0, 0, 20, 14);
        frame.codec = Codec::Lossy(50.0);
        frame.alpha = Alpha::Opaque;
        out.push((
            "single_full".to_owned(),
            build(&AnimSpec::new(20, 14, vec![frame])),
        ));
    }

    // Metadata around the frames: ICCP, two EXIF and one XMP chunk.
    {
        let mut spec = AnimSpec::new(
            18,
            12,
            vec![FrameSpec::new(0, 0, 18, 12), FrameSpec::new(4, 2, 8, 8)],
        );
        spec.metadata = true;
        out.push(("metadata".to_owned(), build(&spec)));
    }

    // Seeded random sequences: rectangles, dispose and blend methods, codecs
    // and alpha.
    for seed in 0..16u64 {
        let mut rng = Rng(0x5eed_0000 + seed);
        let canvas_width = 8 + 2 * rng.below(16) as u32 + rng.below(2) as u32;
        let canvas_height = 6 + 2 * rng.below(12) as u32 + rng.below(2) as u32;
        let count = 2 + rng.below(9) as usize;
        let mut frames = Vec::new();
        for index in 0..count {
            let x = 2 * rng.below(u64::from(canvas_width / 2)) as u32;
            let y = 2 * rng.below(u64::from(canvas_height / 2)) as u32;
            let width = 1 + rng.below(u64::from(canvas_width - x)) as u32;
            let height = 1 + rng.below(u64::from(canvas_height - y)) as u32;
            let mut frame = if index == 0 && rng.below(2) == 0 {
                FrameSpec::new(0, 0, canvas_width, canvas_height)
            } else {
                FrameSpec::new(x, y, width, height)
            };
            frame.duration = rng.below(300) as u32;
            frame.dispose_background = rng.below(2) == 1;
            frame.no_blend = rng.below(3) == 0;
            frame.codec = if rng.below(2) == 0 {
                Codec::Lossless
            } else {
                Codec::Lossy(30.0 + rng.below(70) as f32)
            };
            frame.alpha = [Alpha::Opaque, Alpha::Binary, Alpha::Gradient][rng.below(3) as usize];
            frame.seed = rng.next_u64();
            frames.push(frame);
        }
        let mut spec = AnimSpec::new(canvas_width, canvas_height, frames);
        spec.loops = rng.below(4) as u16;
        spec.background = rng.next_u64() as u32;
        out.push((format!("random_{seed:02}"), build(&spec)));
    }

    // A still image in an extended container without the animation flag: one
    // frame, not an animation (the demuxer ignores ANMF chunks then, so the file
    // is invalid), and a VP8X still image with ALPH.
    {
        let mut spec = AnimSpec::new(16, 12, vec![FrameSpec::new(0, 0, 16, 12)]);
        spec.animation_flag = false;
        out.push(("anmf_without_animation_flag".to_owned(), build(&spec)));
        let rgba = pattern(16, 12, 3, Alpha::Gradient);
        let still = webp_encode_rgba(&rgba, 16, 12, 64, 75.0).unwrap();
        out.push(("still_alpha".to_owned(), still));
    }

    // Failures: a frame outside the canvas, a file cut inside its last frame,
    // a last frame whose lossless image is cut, a middle frame whose lossy
    // image is cut, a corrupt alpha plane, and the animation flag without
    // frames.
    {
        let base = AnimSpec::new(
            20,
            14,
            vec![
                FrameSpec::new(0, 0, 20, 14),
                {
                    let mut frame = FrameSpec::new(4, 4, 10, 8);
                    frame.codec = Codec::Lossy(75.0);
                    frame
                },
                FrameSpec::new(6, 2, 12, 10),
            ],
        );
        let frames: Vec<Vec<u8>> = base.frames.iter().map(frame_chunks).collect();

        let mut outside = base.clone();
        outside.frames[2].x = 10; // 10 + 12 > 20
        out.push((
            "frame_outside_canvas".to_owned(),
            assemble(&outside, &frames),
        ));

        let complete = assemble(&base, &frames);
        out.push((
            "file_cut_in_last_frame".to_owned(),
            complete[..complete.len() - 9].to_vec(),
        ));

        let mut cut = frames.clone();
        cut[2] = truncate_image(&frames[2], |size| size / 2);
        out.push(("last_lossless_frame_cut".to_owned(), assemble(&base, &cut)));

        let mut cut = frames.clone();
        cut[1] = truncate_image(&frames[1], |size| size * 2 / 3);
        out.push(("middle_lossy_frame_cut".to_owned(), assemble(&base, &cut)));

        let mut alpha = base.clone();
        alpha.frames[2].codec = Codec::Lossy(75.0);
        let mut alpha_frames: Vec<Vec<u8>> = alpha.frames.iter().map(frame_chunks).collect();
        alpha_frames[2] = corrupt_alpha(&alpha_frames[2]);
        out.push(("corrupt_alpha".to_owned(), assemble(&alpha, &alpha_frames)));

        let empty = AnimSpec::new(20, 14, Vec::new());
        out.push(("no_frames".to_owned(), assemble(&empty, &[])));
    }

    // A tall lossy frame cut so that its decoding stops after some macroblock
    // rows have reached the canvas: the canvas after the failure holds them.
    {
        let mut second = FrameSpec::new(2, 4, 60, 56);
        second.codec = Codec::Lossy(85.0);
        second.alpha = Alpha::Gradient;
        let spec = AnimSpec::new(64, 64, vec![FrameSpec::new(0, 0, 64, 64), second]);
        let mut frames: Vec<Vec<u8>> = spec.frames.iter().map(frame_chunks).collect();
        frames[1] = truncate_image(&frames[1], |size| size * 3 / 5);
        out.push(("tall_lossy_frame_cut".to_owned(), assemble(&spec, &frames)));
    }
    out
}

/// Writes each animation to `directory` and returns the file paths with names.
pub fn write_all(directory: &Path, animations: &[(String, Vec<u8>)]) -> Vec<(String, PathBuf)> {
    animations
        .iter()
        .map(|(name, bytes)| {
            let path = directory.join(format!("{name}.webp"));
            std::fs::write(&path, bytes).unwrap();
            (name.clone(), path)
        })
        .collect()
}

//------------------------------------------------------------------------------
// img2webp animations

/// Writes a PAM image (`TUPLTYPE RGB_ALPHA`).
fn write_pam(path: &Path, width: u32, height: u32, rgba: &[u8]) {
    let mut bytes = format!(
        "P7\nWIDTH {width}\nHEIGHT {height}\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n"
    )
    .into_bytes();
    bytes.extend_from_slice(rgba);
    std::fs::write(path, bytes).unwrap();
}

/// A frame sequence: a moving disc and square over a background, which is
/// opaque, transparent or half transparent.
fn sequence_frame(width: u32, height: u32, index: u32, background_alpha: u8) -> Vec<u8> {
    let mut out = Vec::with_capacity((width * height * 4) as usize);
    let cx = (index * 7 % width) as i64;
    let cy = (index * 3 % height) as i64;
    let sx = (width - 1 - index * 5 % width) as i64;
    for y in 0..i64::from(height) {
        for x in 0..i64::from(width) {
            let disc = (x - cx).pow(2) + (y - cy).pow(2) <= 49;
            let square = (x - sx).abs() <= 4 && (y - i64::from(height) / 2).abs() <= 4;
            let pixel = if disc {
                [250, 40, 30, 255]
            } else if square {
                [30, 200, 60, 160]
            } else {
                [(x * 4) as u8, (y * 5) as u8, 120, background_alpha]
            };
            out.extend_from_slice(&pixel);
        }
    }
    out
}

/// Animations that the oracle's `img2webp` assembles from generated PAM
/// sequences, with the encoder choosing sub-rectangles, blending and disposal.
pub fn img2webp_animations(oracle: &Oracle, directory: &Path) -> Vec<(String, PathBuf)> {
    let img2webp = oracle.tool("img2webp");
    let (width, height) = (48, 32);
    let mut out = Vec::new();
    for (background_name, background_alpha) in [("opaque", 255u8), ("clear", 0), ("half", 128)] {
        let mut frames = Vec::new();
        for index in 0..8 {
            let path = directory.join(format!("{background_name}_{index}.pam"));
            write_pam(
                &path,
                width,
                height,
                &sequence_frame(width, height, index, background_alpha),
            );
            frames.push(path);
        }
        // (name, file-level options, per-frame options for frames 1 and 5)
        let option_sets: [(&str, &[&str], &[&str]); 7] = [
            ("lossless", &[], &["-lossless"]),
            ("lossy", &[], &["-lossy", "-q", "60"]),
            ("mixed", &["-mixed"], &["-q", "70"]),
            (
                "loop_d",
                &["-loop", "3"],
                &["-d", "70", "-lossy", "-q", "40"],
            ),
            ("kmax", &["-kmin", "2", "-kmax", "3"], &["-lossless"]),
            ("min_size", &["-min_size"], &["-lossy", "-q", "80"]),
            ("exact", &["-kmax", "0"], &["-exact", "-lossy", "-m", "2"]),
        ];
        for (options_name, global, frame_options) in option_sets {
            let name = format!("img2webp_{background_name}_{options_name}");
            let output = directory.join(format!("{name}.webp"));
            let mut command = Command::new(&img2webp);
            command.args(global);
            for (index, frame) in frames.iter().enumerate() {
                if index == 0 || index == 4 {
                    command.args(frame_options);
                }
                if index == 5 {
                    command.args(["-d", "1"]);
                }
                command.arg(frame);
            }
            command.arg("-o").arg(&output);
            let result = command.output().unwrap();
            assert!(
                result.status.success(),
                "img2webp {name} failed: {}",
                String::from_utf8_lossy(&result.stderr)
            );
            out.push((name, output));
        }
    }
    out
}

//------------------------------------------------------------------------------
// The report of the probe's anim command

/// The features that the probe asks the demuxer for.
const FEATURES: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 99];

/// The report of `PrintDemuxer`.
fn print_demuxer(out: &mut String, dmux: &WebPDemuxer<'_>) {
    writeln!(out, "demux=1").unwrap();
    for feature in FEATURES {
        writeln!(
            out,
            "demux.feature{feature}={}",
            webp_demux_get_i(dmux, WebPFormatFeature(feature))
        )
        .unwrap();
    }
    for (index, tag) in [b"ICCP", b"EXIF", b"XMP "].into_iter().enumerate() {
        let mut count = 0;
        if let Some(mut it) = webp_demux_get_chunk(dmux, *tag, 1) {
            loop {
                writeln!(
                    out,
                    "demux.meta{index}.{count}={},{},{},{:016x}",
                    it.chunk_num,
                    it.num_chunks,
                    it.chunk.bytes.len(),
                    digest(&it.chunk.bytes)
                )
                .unwrap();
                count += 1;
                if !webp_demux_next_chunk(&mut it) {
                    break;
                }
            }
        }
        writeln!(out, "demux.meta{index}.count={count}").unwrap();
    }
}

/// One `anim` command of the probe.
#[derive(Clone, Debug)]
pub struct AnimCommand {
    pub mode: i32,
    pub threads: i32,
    pub reset: i32,
    pub dump: i32,
    pub path: PathBuf,
}

impl AnimCommand {
    /// The command line, with `partial` as given.
    pub fn line(&self, partial: bool) -> String {
        format!(
            "anim {} {} {} {} {} {}\n",
            self.mode,
            self.threads,
            self.reset,
            self.dump,
            i32::from(partial),
            self.path.display()
        )
    }
}

/// The lines that the probe prints for an `anim` command without `PARTIAL`,
/// from the Rust translation.
pub fn anim_report(command: &AnimCommand) -> String {
    let bytes = std::fs::read(&command.path).unwrap();
    let mut out = String::new();
    let data = WebPData {
        bytes: Cow::Borrowed(&bytes),
    };
    let mut options = WebPAnimDecoderOptions::default();
    if command.mode >= 0 {
        assert!(webp_anim_decoder_options_init(&mut options));
        options.color_mode = WEBP_CSP_MODE(command.mode as u32);
        options.use_threads = command.threads;
    }
    let dec = webp_anim_decoder_new(&data, (command.mode >= 0).then_some(&options));
    writeln!(out, "new={}", i32::from(dec.is_some())).unwrap();
    let Some(mut dec) = dec else {
        return out;
    };
    let info = webp_anim_decoder_get_info(&dec);
    writeln!(
        out,
        "info={},{},{},{},{},{},{},{},{}",
        info.canvas_width,
        info.canvas_height,
        info.loop_count,
        info.bgcolor,
        info.frame_count,
        info.pad[0],
        info.pad[1],
        info.pad[2],
        info.pad[3]
    )
    .unwrap();
    print_demuxer(&mut out, webp_anim_decoder_get_demuxer(&dec));
    let canvas_size = info.canvas_width as usize * 4 * info.canvas_height as usize;

    let (mut pass, mut frame, mut failed) = (0, 0, false);
    loop {
        let more = webp_anim_decoder_has_more_frames(&dec);
        writeln!(out, "pass.{pass}.{}.more={}", frame + 1, i32::from(more)).unwrap();
        if !more {
            break;
        }
        frame += 1;
        let Some((canvas, timestamp)) = webp_anim_decoder_get_next(&mut dec) else {
            writeln!(out, "pass.{pass}.{frame}.ok=0").unwrap();
            failed = true;
            break;
        };
        assert_eq!(canvas.len(), canvas_size);
        writeln!(out, "pass.{pass}.{frame}.ok=1").unwrap();
        writeln!(out, "pass.{pass}.{frame}.timestamp={timestamp}").unwrap();
        writeln!(out, "pass.{pass}.{frame}.digest={:016x}", digest(canvas)).unwrap();
        if pass == 0 && frame == command.dump {
            writeln!(out, "pass.{pass}.{frame}.canvas={}", hex(canvas)).unwrap();
        }
        if pass == 0 && frame == command.reset {
            webp_anim_decoder_reset(&mut dec);
            writeln!(
                out,
                "reset={}",
                i32::from(webp_anim_decoder_has_more_frames(&dec))
            )
            .unwrap();
            pass = 1;
            frame = 0;
        }
    }
    writeln!(out, "failed={}", i32::from(failed)).unwrap();
    let more = webp_anim_decoder_has_more_frames(&dec);
    let next = webp_anim_decoder_get_next(&mut dec);
    writeln!(out, "end.more={}", i32::from(more)).unwrap();
    writeln!(out, "end.next={}", i32::from(next.is_some())).unwrap();
    if let Some((canvas, timestamp)) = next {
        writeln!(out, "end.timestamp={timestamp}").unwrap();
        writeln!(out, "end.digest={:016x}", digest(canvas)).unwrap();
    }
    out
}

/// The commands for every file: each file runs once per valid color mode, with
/// threads, a reset and a dumped canvas varied between the runs.
pub fn anim_commands(files: &[(String, PathBuf)]) -> Vec<AnimCommand> {
    let mut commands = Vec::new();
    for (index, (_, path)) in files.iter().enumerate() {
        for (run, mode) in MODES.into_iter().enumerate() {
            let shift = index + run;
            commands.push(AnimCommand {
                mode,
                threads: (shift % 2) as i32,
                reset: [0, 1, 2, 3][shift % 4],
                dump: [2, 0, 1, 0][shift % 4],
                path: path.clone(),
            });
        }
    }
    commands
}

/// Splits a probe's output into its commands, by the `command=N` lines.
pub fn split_commands(output: &str) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    for line in output.lines() {
        if line.starts_with("command=") {
            commands.push(String::new());
        } else if let Some(current) = commands.last_mut() {
            current.push_str(line);
            current.push('\n');
        }
    }
    commands
}
