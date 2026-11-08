// libwebp v1.6.0, src/demux/demux.c:
//
// Copyright 2012 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
//  WebP container demux.
//

//! Translation of `src/demux/demux.c` from libwebp v1.6.0.
//!
//! Work package: WP8 (demux).
//!
//! Representation (decided for WP8, see `docs/status/wp8.md`): the demuxer
//! borrows the caller's bytes for its lifetime, as C keeps `mem.buf` without a
//! copy. C's offsets stay offsets. The frame and chunk lists are `Vec`s in C's
//! order. The reader positions hold no bytes: each read takes the input that
//! the demuxer holds (ADR 0003, "Readers over input"). The iterators are
//! defined in `src/webp/demux.rs` (the exceptions in `tools/units.json`) and
//! hold `&WebPDemuxer` in place of C's `private_`.

use std::borrow::Cow;

use crate::utils::utils::{get_le16, get_le24, get_le32};
use crate::webp::decode::{VP8StatusCode, WebPBitstreamFeatures, webp_get_features};
use crate::webp::demux::{
    WEBP_DEMUX_ABI_VERSION, WebPChunkIterator, WebPDemuxState, WebPFormatFeature, WebPIterator,
};
use crate::webp::format_constants;
use crate::webp::mux_types::{WebPData, WebPFeatureFlags, WebPMuxAnimBlend, WebPMuxAnimDispose};
use crate::webp::types::webp_abi_is_incompatible;

// upstream: src/webp/format_constants.h TAG_SIZE
const TAG_SIZE: usize = format_constants::TAG_SIZE.unsigned_abs() as usize;
// upstream: src/webp/format_constants.h CHUNK_HEADER_SIZE
const CHUNK_HEADER_SIZE: usize = format_constants::CHUNK_HEADER_SIZE.unsigned_abs() as usize;
// upstream: src/webp/format_constants.h RIFF_HEADER_SIZE
const RIFF_HEADER_SIZE: usize = format_constants::RIFF_HEADER_SIZE.unsigned_abs() as usize;
// upstream: src/webp/format_constants.h ANMF_CHUNK_SIZE
const ANMF_CHUNK_SIZE: u32 = format_constants::ANMF_CHUNK_SIZE.unsigned_abs();
// upstream: src/webp/format_constants.h ANIM_CHUNK_SIZE
const ANIM_CHUNK_SIZE: u32 = format_constants::ANIM_CHUNK_SIZE.unsigned_abs();
// upstream: src/webp/format_constants.h VP8X_CHUNK_SIZE
const VP8X_CHUNK_SIZE: u32 = format_constants::VP8X_CHUNK_SIZE.unsigned_abs();

// upstream: src/demux/demux.c DMUX_MAJ_VERSION
const DMUX_MAJ_VERSION: i32 = 1;
// upstream: src/demux/demux.c DMUX_MIN_VERSION
const DMUX_MIN_VERSION: i32 = 6;
// upstream: src/demux/demux.c DMUX_REV_VERSION
const DMUX_REV_VERSION: i32 = 0;

// upstream: src/demux/demux.c WebPGetDemuxVersion
/// Returns the version number of the demux library, packed in hexadecimal using
/// 8 bits for each of major, minor and revision (v1.6.0 is `0x010600`).
#[must_use]
pub fn webp_get_demux_version() -> i32 {
    (DMUX_MAJ_VERSION << 16) | (DMUX_MIN_VERSION << 8) | DMUX_REV_VERSION
}

// upstream: src/demux/demux.c ParseStatus
/// Result of one parsing step (C: `ParseStatus`, a file-local enum). Its values
/// are C's `unsigned int` enumerators (ADR 0003, "Enums").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ParseStatus(u32);

impl ParseStatus {
    // upstream: src/demux/demux.c PARSE_OK
    const PARSE_OK: Self = Self(0);
    // upstream: src/demux/demux.c PARSE_NEED_MORE_DATA
    const PARSE_NEED_MORE_DATA: Self = Self(1);
    // upstream: src/demux/demux.c PARSE_ERROR
    const PARSE_ERROR: Self = Self(2);
}

// upstream: src/demux/demux.c MemBuffer
/// Parser positions over the input (C: `MemBuffer`). The bytes are not held
/// here; the demuxer holds them and passes them to each read.
#[derive(Clone, Copy, Debug, Default)]
struct MemBuffer {
    /// Start location of the data (C: `start`).
    start: usize,
    /// End location (C: `end`).
    end: usize,
    /// RIFF chunk end location; it can be past `end` (C: `riff_end`).
    riff_end: usize,
    /// Size of the buffer, trimmed to the RIFF chunk (C: `buf_size`).
    buf_size: usize,
}

// upstream: src/demux/demux.c ChunkData
/// A byte range of the input (C: `ChunkData`).
#[derive(Clone, Copy, Debug, Default)]
struct ChunkData {
    /// Offset of the chunk header in the input (C: `offset`).
    offset: usize,
    /// Size of the chunk, header included (C: `size`).
    size: usize,
}

// upstream: src/demux/demux.c Frame
/// One frame of the animation, or the single image (C: `Frame`).
#[derive(Clone, Copy, Debug, Default)]
#[expect(
    clippy::struct_field_names,
    reason = "frame_num is the C field name, which ADR 0003 keeps"
)]
struct Frame {
    x_offset: i32,
    y_offset: i32,
    width: i32,
    height: i32,
    has_alpha: i32,
    duration: i32,
    dispose_method: WebPMuxAnimDispose,
    blend_method: WebPMuxAnimBlend,
    frame_num: i32,
    /// `img_components` contains a full image (C: `complete`).
    complete: i32,
    /// `[0]` is the VP8 or VP8L chunk, `[1]` the ALPH chunk.
    img_components: [ChunkData; 2],
}

// upstream: src/demux/demux.c Chunk
/// A non-image chunk, kept for `WebPDemuxGetChunk` (C: `Chunk`).
#[derive(Clone, Copy, Debug, Default)]
struct Chunk {
    data: ChunkData,
}

// upstream: src/demux/demux.c WebPDemuxer
/// A parsed WebP file (C: `struct WebPDemuxer`, opaque in `webp/demux.h`). It
/// borrows the bytes it parsed for its lifetime, and it is immutable after
/// construction.
#[derive(Debug)]
pub struct WebPDemuxer<'a> {
    /// The input (C: `mem.buf`), borrowed and not copied.
    buf: &'a [u8],
    /// The parser positions (C: `mem`).
    mem: MemBuffer,
    state: WebPDemuxState,
    is_ext_format: bool,
    feature_flags: u32,
    canvas_width: i32,
    canvas_height: i32,
    loop_count: i32,
    bgcolor: u32,
    num_frames: i32,
    /// Frames in file order (C: `frames`, linked by `next`).
    frames: Vec<Frame>,
    /// Non-image chunks in file order (C: `chunks`, linked by `next`).
    chunks: Vec<Chunk>,
}

// The area test of C, `width * (uint64_t)height >= MAX_IMAGE_AREA`: the `int`
// operand is converted to `uint64_t` modulo 2^64, and the product wraps.
fn image_area(width: i32, height: i32) -> u64 {
    i64::from(width)
        .cast_unsigned()
        .wrapping_mul(i64::from(height).cast_unsigned())
}

impl MemBuffer {
    // upstream: src/demux/demux.c MemDataSize
    /// Returns the remaining data size available in the buffer.
    fn data_size(&self) -> usize {
        self.end - self.start
    }

    // upstream: src/demux/demux.c SizeIsInvalid
    /// Returns true if `size` exceeds the end of the RIFF chunk.
    fn size_is_invalid(&self, size: usize) -> bool {
        size > self.riff_end - self.start
    }

    // upstream: src/demux/demux.c Skip
    fn skip(&mut self, size: usize) {
        self.start += size;
    }

    // upstream: src/demux/demux.c Rewind
    fn rewind(&mut self, size: usize) {
        self.start -= size;
    }

    // upstream: src/demux/demux.c GetBuffer
    fn get_buffer<'b>(&self, buf: &'b [u8]) -> &'b [u8] {
        &buf[self.start..]
    }

    // upstream: src/demux/demux.c ReadByte
    fn read_byte(&mut self, buf: &[u8]) -> u8 {
        let byte = buf[self.start];
        self.skip(1);
        byte
    }

    // upstream: src/demux/demux.c ReadLE16s
    fn read_le16s(&mut self, buf: &[u8]) -> i32 {
        let val = get_le16(&buf[self.start..]);
        self.skip(2);
        val
    }

    // upstream: src/demux/demux.c ReadLE24s
    fn read_le24s(&mut self, buf: &[u8]) -> i32 {
        let val = get_le24(&buf[self.start..]);
        self.skip(3);
        val
    }

    // upstream: src/demux/demux.c ReadLE32
    fn read_le32(&mut self, buf: &[u8]) -> u32 {
        let val = get_le32(&buf[self.start..]);
        self.skip(4);
        val
    }

    /// Reads a chunk tag. C reads it with `ReadLE32` and compares the value with
    /// `MKFOURCC`, which is the same as comparing the four bytes in order.
    fn read_fourcc(&mut self, buf: &[u8]) -> [u8; TAG_SIZE] {
        let mut fourcc = [0; TAG_SIZE];
        fourcc.copy_from_slice(&buf[self.start..self.start + TAG_SIZE]);
        self.skip(TAG_SIZE);
        fourcc
    }
}

// upstream: src/demux/demux.c RemapMemBuffer
/// Points the reader at `size` bytes. It refuses a size below the buffer size
/// it already has.
fn remap_mem_buffer(mem: &mut MemBuffer, size: usize) -> bool {
    // can't remap to a shorter buffer!
    if size < mem.buf_size {
        return false;
    }
    mem.end = size;
    mem.buf_size = size;
    true
}

// upstream: src/demux/demux.c InitMemBuffer
fn init_mem_buffer(size: usize) -> Option<MemBuffer> {
    let mut mem = MemBuffer::default();
    remap_mem_buffer(&mut mem, size).then_some(mem)
}

// upstream: src/demux/demux.c SetFrameInfo
fn set_frame_info(
    start_offset: usize,
    size: usize,
    frame_num: i32,
    complete: bool,
    features: &WebPBitstreamFeatures,
    frame: &mut Frame,
) {
    frame.img_components[0].offset = start_offset;
    frame.img_components[0].size = size;
    frame.width = features.width;
    frame.height = features.height;
    frame.has_alpha |= features.has_alpha;
    frame.frame_num = frame_num;
    frame.complete = i32::from(complete);
}

// upstream: src/demux/demux.c StoreFrame
/// Stores the image-bearing chunks into `frame`. `min_size` is an optional size
/// requirement; it may be zero.
fn store_frame(
    frame_num: i32,
    min_size: u32,
    buf: &[u8],
    mem: &mut MemBuffer,
    frame: &mut Frame,
) -> ParseStatus {
    let mut alpha_chunks = 0;
    let mut image_chunks = 0;
    let mut done = mem.data_size() < CHUNK_HEADER_SIZE || mem.data_size() < min_size as usize;
    let mut status = ParseStatus::PARSE_OK;

    if done {
        return ParseStatus::PARSE_NEED_MORE_DATA;
    }

    loop {
        let chunk_start_offset = mem.start;
        let fourcc = mem.read_fourcc(buf);
        let payload_size = mem.read_le32(buf);
        let mut rewind_and_stop = false;

        if payload_size > format_constants::MAX_CHUNK_PAYLOAD {
            return ParseStatus::PARSE_ERROR;
        }

        let payload_size_padded = payload_size + (payload_size & 1);
        let payload_available = if payload_size_padded as usize > mem.data_size() {
            mem.data_size()
        } else {
            payload_size_padded as usize
        };
        let chunk_size = CHUNK_HEADER_SIZE + payload_available;
        if mem.size_is_invalid(payload_size_padded as usize) {
            return ParseStatus::PARSE_ERROR;
        }
        if payload_size_padded as usize > mem.data_size() {
            status = ParseStatus::PARSE_NEED_MORE_DATA;
        }

        match &fourcc {
            b"ALPH" => {
                if alpha_chunks == 0 {
                    alpha_chunks += 1;
                    frame.img_components[1] = ChunkData {
                        offset: chunk_start_offset,
                        size: chunk_size,
                    };
                    frame.has_alpha = 1;
                    frame.frame_num = frame_num;
                    mem.skip(payload_available);
                } else {
                    // goto Done
                    rewind_and_stop = true;
                }
            }
            // The VP8L case falls through to the VP8 case in C.
            b"VP8L" | b"VP8 " => {
                if fourcc == *b"VP8L" && alpha_chunks > 0 {
                    // VP8L has its own alpha
                    return ParseStatus::PARSE_ERROR;
                }
                if image_chunks == 0 {
                    // Extract the bitstream features, tolerating failures when the data
                    // is incomplete.
                    let mut features = WebPBitstreamFeatures::default();
                    let vp8_status = webp_get_features(
                        &buf[chunk_start_offset..chunk_start_offset + chunk_size],
                        &mut features,
                    );
                    if status == ParseStatus::PARSE_NEED_MORE_DATA
                        && vp8_status == Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA)
                    {
                        return ParseStatus::PARSE_NEED_MORE_DATA;
                    } else if vp8_status.is_err() {
                        // We have enough data, and yet WebPGetFeatures() failed.
                        return ParseStatus::PARSE_ERROR;
                    }
                    image_chunks += 1;
                    set_frame_info(
                        chunk_start_offset,
                        chunk_size,
                        frame_num,
                        status == ParseStatus::PARSE_OK,
                        &features,
                        frame,
                    );
                    mem.skip(payload_available);
                } else {
                    // goto Done
                    rewind_and_stop = true;
                }
            }
            // Done: restore fourcc/size when moving up one level in parsing.
            _ => rewind_and_stop = true,
        }

        if rewind_and_stop {
            mem.rewind(CHUNK_HEADER_SIZE);
            done = true;
        }

        if mem.start == mem.riff_end {
            done = true;
        } else if mem.data_size() < CHUNK_HEADER_SIZE {
            status = ParseStatus::PARSE_NEED_MORE_DATA;
        }

        if done || status != ParseStatus::PARSE_OK {
            break;
        }
    }

    status
}

// upstream: src/demux/demux.c ReadHeader
/// Reads the RIFF header and trims the buffer to the RIFF chunk.
fn read_header(buf: &[u8], mem: &mut MemBuffer) -> ParseStatus {
    let min_size = RIFF_HEADER_SIZE + CHUNK_HEADER_SIZE;

    // Basic file level validation.
    if mem.data_size() < min_size {
        return ParseStatus::PARSE_NEED_MORE_DATA;
    }
    let header = mem.get_buffer(buf);
    if &header[..TAG_SIZE] != b"RIFF"
        || &header[CHUNK_HEADER_SIZE..CHUNK_HEADER_SIZE + TAG_SIZE] != b"WEBP"
    {
        return ParseStatus::PARSE_ERROR;
    }

    let riff_size = get_le32(&header[TAG_SIZE..]);
    if riff_size < format_constants::CHUNK_HEADER_SIZE.unsigned_abs() {
        return ParseStatus::PARSE_ERROR;
    }
    if riff_size > format_constants::MAX_CHUNK_PAYLOAD {
        return ParseStatus::PARSE_ERROR;
    }

    // There's no point in reading past the end of the RIFF chunk
    mem.riff_end = riff_size as usize + CHUNK_HEADER_SIZE;
    if mem.buf_size > mem.riff_end {
        mem.buf_size = mem.riff_end;
        mem.end = mem.riff_end;
    }

    mem.skip(RIFF_HEADER_SIZE);
    ParseStatus::PARSE_OK
}

// upstream: src/demux/demux.c GetFramePayload
/// Returns the bytes of a frame: the ALPH chunk, the chunks between it and the
/// image, and the image chunk.
fn get_frame_payload<'a>(buf: &'a [u8], frame: &Frame) -> &'a [u8] {
    let image = frame.img_components[0];
    let alpha = frame.img_components[1];
    let mut start_offset = image.offset;
    let mut data_size = image.size;

    // if alpha exists it precedes image, update the size allowing for
    // intervening chunks.
    if alpha.size > 0 {
        let inter_size = if image.offset > 0 {
            image.offset - (alpha.offset + alpha.size)
        } else {
            0
        };
        start_offset = alpha.offset;
        data_size += alpha.size + inter_size;
    }
    &buf[start_offset..start_offset + data_size]
}

// upstream: src/demux/demux.c CheckFrameBounds
/// If `exact` is true, checks that the frame is the size of the canvas at the
/// origin. If `exact` is false, checks that the frame's offsets and extent stay
/// within the canvas.
fn check_frame_bounds(frame: &Frame, exact: bool, canvas_width: i32, canvas_height: i32) -> bool {
    if exact {
        if frame.x_offset != 0 || frame.y_offset != 0 {
            return false;
        }
        if frame.width != canvas_width || frame.height != canvas_height {
            return false;
        }
    } else {
        if frame.x_offset < 0 || frame.y_offset < 0 {
            return false;
        }
        if frame.width + frame.x_offset > canvas_width {
            return false;
        }
        if frame.height + frame.y_offset > canvas_height {
            return false;
        }
    }
    true
}

// upstream: src/demux/demux.c InitDemux
fn init_demux(buf: &[u8], mem: MemBuffer) -> WebPDemuxer<'_> {
    WebPDemuxer {
        buf,
        mem,
        state: WebPDemuxState::WEBP_DEMUX_PARSING_HEADER,
        is_ext_format: false,
        feature_flags: 0,
        canvas_width: -1,
        canvas_height: -1,
        loop_count: 1,
        bgcolor: 0xFFFF_FFFF, // White background by default.
        num_frames: 0,
        frames: Vec::new(),
        chunks: Vec::new(),
    }
}

// upstream: src/demux/demux.c CreateRawImageDemuxer
/// Creates a demuxer for a raw VP8 or VP8L bitstream that has no RIFF header.
fn create_raw_image_demuxer(buf: &[u8], mem: MemBuffer) -> (ParseStatus, Option<WebPDemuxer<'_>>) {
    let mut features = WebPBitstreamFeatures::default();
    if let Err(status) = webp_get_features(&buf[..mem.buf_size], &mut features) {
        let parse_status = if status == VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA {
            ParseStatus::PARSE_NEED_MORE_DATA
        } else {
            ParseStatus::PARSE_ERROR
        };
        return (parse_status, None);
    }

    let mut dmux = init_demux(buf, mem);
    if !dmux.reserve_frame() {
        return (ParseStatus::PARSE_ERROR, None);
    }
    let mut frame = Frame::default();
    set_frame_info(0, mem.buf_size, 1, true, &features, &mut frame);
    let (width, height, has_alpha) = (frame.width, frame.height, frame.has_alpha);
    dmux.add_frame(frame);
    dmux.state = WebPDemuxState::WEBP_DEMUX_DONE;
    dmux.canvas_width = width;
    dmux.canvas_height = height;
    if has_alpha != 0 {
        dmux.feature_flags |= WebPFeatureFlags::ALPHA_FLAG.0;
    }
    dmux.num_frames = 1;
    debug_assert!(dmux.is_valid_simple_format());
    (ParseStatus::PARSE_OK, Some(dmux))
}

// upstream: src/demux/demux.c ChunkParser, kMasterChunks
/// The chunks that open a RIFF file (C: `kMasterChunks`, a table of the parse and
/// validation functions per tag).
#[derive(Clone, Copy)]
enum MasterChunk {
    /// `VP8 ` and `VP8L` (C: `ParseSingleImage` with `IsValidSimpleFormat`).
    SingleImage,
    /// `VP8X` (C: `ParseVP8X` with `IsValidExtendedFormat`).
    ExtendedFormat,
}

impl MasterChunk {
    /// Looks up the `kMasterChunks` entry whose `id` is `tag` (C: the `memcmp` loop).
    fn from_tag(tag: &[u8]) -> Option<Self> {
        match tag {
            b"VP8 " | b"VP8L" => Some(Self::SingleImage),
            b"VP8X" => Some(Self::ExtendedFormat),
            _ => None,
        }
    }

    /// Runs the `parse` function pointer of the `ChunkParser` entry (C: `parser->parse`).
    fn parse(self, dmux: &mut WebPDemuxer<'_>) -> ParseStatus {
        match self {
            Self::SingleImage => dmux.parse_single_image(),
            Self::ExtendedFormat => dmux.parse_vp8x(),
        }
    }

    /// Runs the `valid` function pointer of the `ChunkParser` entry (C: `parser->valid`).
    fn is_valid(self, dmux: &WebPDemuxer<'_>) -> bool {
        match self {
            Self::SingleImage => dmux.is_valid_simple_format(),
            Self::ExtendedFormat => dmux.is_valid_extended_format(),
        }
    }
}

impl WebPDemuxer<'_> {
    // upstream: src/demux/demux.c AddChunk
    fn add_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    // upstream: src/demux/demux.c StoreChunk
    /// Stores the header and the unpadded payload size of a non-image chunk.
    /// Returns false when the host refuses the allocation, as C's `WebPSafeCalloc`
    /// does with NULL.
    fn store_chunk(&mut self, start_offset: usize, size: usize) -> bool {
        if self.chunks.try_reserve(1).is_err() {
            return false;
        }
        self.add_chunk(Chunk {
            data: ChunkData {
                offset: start_offset,
                size,
            },
        });
        true
    }

    /// Reserves room for one frame, as C's `WebPSafeCalloc` of a `Frame` does
    /// before the frame is parsed. A refused host allocation fails the parse, as
    /// C's NULL does, and the following push then cannot reallocate.
    fn reserve_frame(&mut self) -> bool {
        self.frames.try_reserve(1).is_ok()
    }

    // upstream: src/demux/demux.c AddFrame
    /// Adds a frame to the end of the list.
    ///
    /// C's guard tests `*dmux->frames_tail`, which is the `next` link of the last
    /// frame; `AddFrame` sets that link to NULL, so the guard against an incomplete
    /// last frame never refuses a frame. The port keeps that behavior: the frame is
    /// always appended, and an incomplete frame followed by another one is
    /// rejected by `is_valid_extended_format`, as C does. Recorded in the status file.
    fn add_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    // upstream: src/demux/demux.c NewFrame
    /// Checks the sizes of an `ANMF` payload and reserves its frame.
    fn new_frame(&mut self, min_size: usize, actual_size: usize) -> ParseStatus {
        if self.mem.size_is_invalid(min_size) {
            return ParseStatus::PARSE_ERROR;
        }
        if actual_size < min_size {
            return ParseStatus::PARSE_ERROR;
        }
        if self.mem.data_size() < min_size {
            return ParseStatus::PARSE_NEED_MORE_DATA;
        }
        if !self.reserve_frame() {
            return ParseStatus::PARSE_ERROR;
        }
        ParseStatus::PARSE_OK
    }

    // upstream: src/demux/demux.c ParseAnimationFrame
    /// Parses an `ANMF` chunk and the image-bearing chunks that immediately
    /// follow it. `frame_chunk_size` is the previously validated, padded size.
    fn parse_animation_frame(&mut self, frame_chunk_size: u32) -> ParseStatus {
        let buf = self.buf;
        let is_animation = self.feature_flags & WebPFeatureFlags::ANIMATION_FLAG.0 != 0;
        // C's unsigned difference; it is used only after `new_frame` has
        // accepted the chunk size.
        let anmf_payload_size = frame_chunk_size.wrapping_sub(ANMF_CHUNK_SIZE);
        let status = self.new_frame(ANMF_CHUNK_SIZE as usize, frame_chunk_size as usize);
        if status != ParseStatus::PARSE_OK {
            return status;
        }

        let mut frame = Frame {
            x_offset: 2 * self.mem.read_le24s(buf),
            y_offset: 2 * self.mem.read_le24s(buf),
            width: 1 + self.mem.read_le24s(buf),
            height: 1 + self.mem.read_le24s(buf),
            duration: self.mem.read_le24s(buf),
            ..Frame::default()
        };
        let bits = i32::from(self.mem.read_byte(buf));
        frame.dispose_method = if bits & 1 != 0 {
            WebPMuxAnimDispose::WEBP_MUX_DISPOSE_BACKGROUND
        } else {
            WebPMuxAnimDispose::WEBP_MUX_DISPOSE_NONE
        };
        frame.blend_method = if bits & 2 != 0 {
            WebPMuxAnimBlend::WEBP_MUX_NO_BLEND
        } else {
            WebPMuxAnimBlend::WEBP_MUX_BLEND
        };
        if image_area(frame.width, frame.height) >= format_constants::MAX_IMAGE_AREA {
            return ParseStatus::PARSE_ERROR;
        }

        // Store a frame only if the animation flag is set there is some data for
        // this frame is available.
        let start_offset = self.mem.start;
        let mut status = store_frame(
            self.num_frames + 1,
            anmf_payload_size,
            buf,
            &mut self.mem,
            &mut frame,
        );
        if status != ParseStatus::PARSE_ERROR
            && self.mem.start - start_offset > anmf_payload_size as usize
        {
            status = ParseStatus::PARSE_ERROR;
        }
        if status != ParseStatus::PARSE_ERROR && is_animation && frame.frame_num > 0 {
            self.add_frame(frame);
            self.num_frames += 1;
        }
        status
    }

    // upstream: src/demux/demux.c ParseSingleImage
    /// Parses a single `VP8 `, `VP8L` or `ALPH`-plus-image chunk sequence.
    fn parse_single_image(&mut self) -> ParseStatus {
        let buf = self.buf;
        let min_size = CHUNK_HEADER_SIZE;

        if !self.frames.is_empty() {
            return ParseStatus::PARSE_ERROR;
        }
        if self.mem.size_is_invalid(min_size) {
            return ParseStatus::PARSE_ERROR;
        }
        if self.mem.data_size() < min_size {
            return ParseStatus::PARSE_NEED_MORE_DATA;
        }
        if !self.reserve_frame() {
            return ParseStatus::PARSE_ERROR;
        }

        let mut frame = Frame::default();
        // For the single image case we allow parsing of a partial frame, so no
        // minimum size is imposed here.
        let status = store_frame(1, 0, buf, &mut self.mem, &mut frame);
        if status != ParseStatus::PARSE_ERROR {
            let has_alpha = self.feature_flags & WebPFeatureFlags::ALPHA_FLAG.0 != 0;
            // Clear any alpha when the alpha flag is missing.
            if !has_alpha && frame.img_components[1].size > 0 {
                frame.img_components[1] = ChunkData::default();
                frame.has_alpha = 0;
            }

            // Use the frame width/height as the canvas values for non-vp8x files.
            // Also, set ALPHA_FLAG if this is a lossless image with alpha.
            if !self.is_ext_format && frame.width > 0 && frame.height > 0 {
                self.state = WebPDemuxState::WEBP_DEMUX_PARSED_HEADER;
                self.canvas_width = frame.width;
                self.canvas_height = frame.height;
                if frame.has_alpha != 0 {
                    self.feature_flags |= WebPFeatureFlags::ALPHA_FLAG.0;
                }
            }
            self.add_frame(frame);
            self.num_frames = 1;
        }
        status
    }

    // upstream: src/demux/demux.c ParseVP8XChunks
    /// Parses the chunks that follow the `VP8X` chunk.
    fn parse_vp8x_chunks(&mut self) -> ParseStatus {
        let buf = self.buf;
        let is_animation = self.feature_flags & WebPFeatureFlags::ANIMATION_FLAG.0 != 0;
        let mut anim_chunks = 0;
        let mut status = ParseStatus::PARSE_OK;

        loop {
            let mut store = true;
            let chunk_start_offset = self.mem.start;
            let fourcc = self.mem.read_fourcc(buf);
            let chunk_size = self.mem.read_le32(buf);

            if chunk_size > format_constants::MAX_CHUNK_PAYLOAD {
                return ParseStatus::PARSE_ERROR;
            }

            let chunk_size_padded = chunk_size + (chunk_size & 1);
            if self.mem.size_is_invalid(chunk_size_padded as usize) {
                return ParseStatus::PARSE_ERROR;
            }

            // `skip` is true where C jumps to its `Skip` label or runs the default
            // case, and false where the case ends with `break`.
            let skip = match &fourcc {
                b"VP8X" => return ParseStatus::PARSE_ERROR,
                b"ALPH" | b"VP8 " | b"VP8L" => {
                    // check that this isn't an animation (all frames should be in an ANMF).
                    if anim_chunks > 0 || is_animation {
                        return ParseStatus::PARSE_ERROR;
                    }
                    self.mem.rewind(CHUNK_HEADER_SIZE);
                    status = self.parse_single_image();
                    false
                }
                b"ANIM" => {
                    if chunk_size_padded < ANIM_CHUNK_SIZE {
                        return ParseStatus::PARSE_ERROR;
                    }
                    if self.mem.data_size() < chunk_size_padded as usize {
                        status = ParseStatus::PARSE_NEED_MORE_DATA;
                        false
                    } else if anim_chunks == 0 {
                        anim_chunks += 1;
                        self.bgcolor = self.mem.read_le32(buf);
                        self.loop_count = self.mem.read_le16s(buf);
                        self.mem
                            .skip((chunk_size_padded - ANIM_CHUNK_SIZE) as usize);
                        false
                    } else {
                        store = false;
                        true
                    }
                }
                b"ANMF" => {
                    if anim_chunks == 0 {
                        return ParseStatus::PARSE_ERROR; // 'ANIM' precedes frames.
                    }
                    status = self.parse_animation_frame(chunk_size_padded);
                    false
                }
                b"ICCP" => {
                    store = self.feature_flags & WebPFeatureFlags::ICCP_FLAG.0 != 0;
                    true
                }
                b"EXIF" => {
                    store = self.feature_flags & WebPFeatureFlags::EXIF_FLAG.0 != 0;
                    true
                }
                b"XMP " => {
                    store = self.feature_flags & WebPFeatureFlags::XMP_FLAG.0 != 0;
                    true
                }
                _ => true,
            };

            if skip {
                if chunk_size_padded as usize <= self.mem.data_size() {
                    if store
                        && !self.store_chunk(
                            chunk_start_offset,
                            CHUNK_HEADER_SIZE + chunk_size as usize,
                        )
                    {
                        return ParseStatus::PARSE_ERROR;
                    }
                    self.mem.skip(chunk_size_padded as usize);
                } else {
                    status = ParseStatus::PARSE_NEED_MORE_DATA;
                }
            }

            if self.mem.start == self.mem.riff_end {
                break;
            } else if self.mem.data_size() < CHUNK_HEADER_SIZE {
                status = ParseStatus::PARSE_NEED_MORE_DATA;
            }
            if status != ParseStatus::PARSE_OK {
                break;
            }
        }

        status
    }

    // upstream: src/demux/demux.c ParseVP8X
    /// Parses the `VP8X` chunk and then the chunks that follow it.
    fn parse_vp8x(&mut self) -> ParseStatus {
        let buf = self.buf;

        if self.mem.data_size() < CHUNK_HEADER_SIZE {
            return ParseStatus::PARSE_NEED_MORE_DATA;
        }

        self.is_ext_format = true;
        self.mem.skip(TAG_SIZE); // VP8X
        let mut vp8x_size = self.mem.read_le32(buf);
        if vp8x_size > format_constants::MAX_CHUNK_PAYLOAD {
            return ParseStatus::PARSE_ERROR;
        }
        if vp8x_size < VP8X_CHUNK_SIZE {
            return ParseStatus::PARSE_ERROR;
        }
        vp8x_size += vp8x_size & 1;
        if self.mem.size_is_invalid(vp8x_size as usize) {
            return ParseStatus::PARSE_ERROR;
        }
        if self.mem.data_size() < vp8x_size as usize {
            return ParseStatus::PARSE_NEED_MORE_DATA;
        }

        self.feature_flags = u32::from(self.mem.read_byte(buf));
        self.mem.skip(3); // Reserved.
        self.canvas_width = 1 + self.mem.read_le24s(buf);
        self.canvas_height = 1 + self.mem.read_le24s(buf);
        if image_area(self.canvas_width, self.canvas_height) >= format_constants::MAX_IMAGE_AREA {
            return ParseStatus::PARSE_ERROR; // image final dimension is too large
        }
        self.mem.skip((vp8x_size - VP8X_CHUNK_SIZE) as usize); // skip any trailing data.
        self.state = WebPDemuxState::WEBP_DEMUX_PARSED_HEADER;

        if self.mem.size_is_invalid(CHUNK_HEADER_SIZE) {
            return ParseStatus::PARSE_ERROR;
        }
        if self.mem.data_size() < CHUNK_HEADER_SIZE {
            return ParseStatus::PARSE_NEED_MORE_DATA;
        }

        self.parse_vp8x_chunks()
    }

    // upstream: src/demux/demux.c IsValidSimpleFormat
    fn is_valid_simple_format(&self) -> bool {
        if self.state == WebPDemuxState::WEBP_DEMUX_PARSING_HEADER {
            return true;
        }

        if self.canvas_width <= 0 || self.canvas_height <= 0 {
            return false;
        }
        let frame = self.frames.first();
        if self.state == WebPDemuxState::WEBP_DEMUX_DONE && frame.is_none() {
            return false;
        }

        // C reads the first frame here. A parsed header always has its frame:
        // `parse_single_image` sets the state only after it stores the frame.
        let Some(frame) = frame else {
            return false;
        };
        frame.width > 0 && frame.height > 0
    }

    // upstream: src/demux/demux.c IsValidExtendedFormat
    fn is_valid_extended_format(&self) -> bool {
        let is_animation = self.feature_flags & WebPFeatureFlags::ANIMATION_FLAG.0 != 0;

        if self.state == WebPDemuxState::WEBP_DEMUX_PARSING_HEADER {
            return true;
        }

        if self.canvas_width <= 0 || self.canvas_height <= 0 {
            return false;
        }
        if self.loop_count < 0 {
            return false;
        }
        if self.state == WebPDemuxState::WEBP_DEMUX_DONE && self.frames.is_empty() {
            return false;
        }
        if self.feature_flags & !WebPFeatureFlags::ALL_VALID_FLAGS.0 != 0 {
            return false; // invalid bitstream
        }

        for (index, frame) in self.frames.iter().enumerate() {
            let image = frame.img_components[0];
            let alpha = frame.img_components[1];

            if !is_animation && frame.frame_num > 1 {
                return false;
            }

            if frame.complete != 0 {
                if alpha.size == 0 && image.size == 0 {
                    return false;
                }
                // Ensure alpha precedes image bitstream.
                if alpha.size > 0 && alpha.offset > image.offset {
                    return false;
                }

                if frame.width <= 0 || frame.height <= 0 {
                    return false;
                }
            } else {
                // There shouldn't be a partial frame in a complete file.
                if self.state == WebPDemuxState::WEBP_DEMUX_DONE {
                    return false;
                }

                // Ensure alpha precedes image bitstream.
                if alpha.size > 0 && image.size > 0 && alpha.offset > image.offset {
                    return false;
                }
                // There shouldn't be any frames after an incomplete one.
                if index + 1 < self.frames.len() {
                    return false;
                }
            }

            if frame.width > 0
                && frame.height > 0
                && !check_frame_bounds(frame, !is_animation, self.canvas_width, self.canvas_height)
            {
                return false;
            }
        }
        true
    }

    // upstream: src/demux/demux.c GetFrame
    fn get_frame(&self, frame_num: i32) -> Option<&Frame> {
        self.frames
            .iter()
            .find(|frame| frame.frame_num == frame_num)
    }

    /// The first four bytes of a chunk, its tag.
    fn chunk_header(&self, chunk: &Chunk) -> &[u8] {
        &self.buf[chunk.data.offset..chunk.data.offset + TAG_SIZE]
    }

    // upstream: src/demux/demux.c ChunkCount
    fn chunk_count(&self, fourcc: [u8; TAG_SIZE]) -> i32 {
        let mut count = 0;
        for chunk in &self.chunks {
            if self.chunk_header(chunk) == fourcc {
                count += 1;
            }
        }
        count
    }

    // upstream: src/demux/demux.c GetChunk
    fn get_chunk(&self, fourcc: [u8; TAG_SIZE], chunk_num: i32) -> Option<&Chunk> {
        let mut count = 0;
        for chunk in &self.chunks {
            if self.chunk_header(chunk) == fourcc {
                count += 1;
            }
            if count == chunk_num {
                return Some(chunk);
            }
        }
        None
    }
}

// upstream: src/demux/demux.c WebPDemuxInternal
/// Parses the WebP file in `data` and returns the demuxer together with the
/// parser state, which C writes to `*state`. `allow_partial` accepts a file whose
/// RIFF chunk is not complete yet. `version` is the ABI version the caller was
/// built with; a different major number fails before any parsing.
///
/// Without a demuxer the state is `WEBP_DEMUX_PARSING_HEADER` when more data
/// could complete the header, and `WEBP_DEMUX_PARSE_ERROR` otherwise. A file
/// whose first chunk is not `VP8 `, `VP8L` or `VP8X` also reports
/// `WEBP_DEMUX_PARSING_HEADER`, because C returns the parser's state before any
/// error is recorded in it. With `allow_partial`, an incomplete file gives a
/// demuxer in the state that its parse reached.
#[must_use]
pub fn webp_demux_internal<'a>(
    data: &'a WebPData<'_>,
    allow_partial: bool,
    version: i32,
) -> (Option<WebPDemuxer<'a>>, WebPDemuxState) {
    if webp_abi_is_incompatible(version, WEBP_DEMUX_ABI_VERSION) {
        return (None, WebPDemuxState::WEBP_DEMUX_PARSE_ERROR);
    }
    let buf: &'a [u8] = &data.bytes;
    if buf.is_empty() {
        return (None, WebPDemuxState::WEBP_DEMUX_PARSE_ERROR);
    }

    let Some(mut mem) = init_mem_buffer(buf.len()) else {
        return (None, WebPDemuxState::WEBP_DEMUX_PARSE_ERROR);
    };
    let mut status = read_header(buf, &mut mem);
    if status != ParseStatus::PARSE_OK {
        // If parsing of the webp file header fails attempt to handle a raw
        // VP8/VP8L frame. Note 'allow_partial' is ignored in this case.
        if status == ParseStatus::PARSE_ERROR {
            let (raw_status, dmux) = create_raw_image_demuxer(buf, mem);
            if raw_status == ParseStatus::PARSE_OK {
                return (dmux, WebPDemuxState::WEBP_DEMUX_DONE);
            }
            status = raw_status;
        }
        let state = if status == ParseStatus::PARSE_NEED_MORE_DATA {
            WebPDemuxState::WEBP_DEMUX_PARSING_HEADER
        } else {
            WebPDemuxState::WEBP_DEMUX_PARSE_ERROR
        };
        return (None, state);
    }

    let partial = mem.buf_size < mem.riff_end;
    if !allow_partial && partial {
        return (None, WebPDemuxState::WEBP_DEMUX_PARSE_ERROR);
    }

    let mut dmux = init_demux(buf, mem);

    status = ParseStatus::PARSE_ERROR;
    let tag = dmux.mem.get_buffer(buf);
    if let Some(master) = MasterChunk::from_tag(&tag[..TAG_SIZE]) {
        status = master.parse(&mut dmux);
        if status == ParseStatus::PARSE_OK {
            dmux.state = WebPDemuxState::WEBP_DEMUX_DONE;
        }
        if status == ParseStatus::PARSE_NEED_MORE_DATA && !partial {
            status = ParseStatus::PARSE_ERROR;
        }
        if status != ParseStatus::PARSE_ERROR && !master.is_valid(&dmux) {
            status = ParseStatus::PARSE_ERROR;
        }
        if status == ParseStatus::PARSE_ERROR {
            dmux.state = WebPDemuxState::WEBP_DEMUX_PARSE_ERROR;
        }
    }
    let state = dmux.state;

    if status == ParseStatus::PARSE_ERROR {
        return (None, state);
    }
    (Some(dmux), state)
}

// upstream: src/demux/demux.c WebPDemuxDelete
/// Frees the memory associated with the demuxer. The C function accepts NULL;
/// in Rust the caller holds no `Option`, and dropping the value does the work.
pub fn webp_demux_delete(dmux: WebPDemuxer<'_>) {
    drop(dmux);
}

// upstream: src/demux/demux.c WebPDemuxGetI
/// Returns the value of `feature`. The values are valid when the demuxer was
/// built by `webp_demux` or `webp_demux_partial` in the state
/// `WEBP_DEMUX_PARSED_HEADER` or later. An unknown feature gives 0, as in C.
#[must_use]
pub fn webp_demux_get_i(dmux: &WebPDemuxer<'_>, feature: WebPFormatFeature) -> u32 {
    match feature {
        WebPFormatFeature::WEBP_FF_FORMAT_FLAGS => dmux.feature_flags,
        WebPFormatFeature::WEBP_FF_CANVAS_WIDTH => dmux.canvas_width.cast_unsigned(),
        WebPFormatFeature::WEBP_FF_CANVAS_HEIGHT => dmux.canvas_height.cast_unsigned(),
        WebPFormatFeature::WEBP_FF_LOOP_COUNT => dmux.loop_count.cast_unsigned(),
        WebPFormatFeature::WEBP_FF_BACKGROUND_COLOR => dmux.bgcolor,
        WebPFormatFeature::WEBP_FF_FRAME_COUNT => dmux.num_frames.cast_unsigned(),
        _ => 0,
    }
}

// upstream: src/demux/demux.c SynthesizeFrame
/// Fills the iterator from `frame`, with the frame's fragment.
fn synthesize_frame<'d>(
    dmux: &'d WebPDemuxer<'d>,
    frame: &Frame,
    iter: &mut WebPIterator<'d>,
) -> bool {
    let payload = get_frame_payload(dmux.buf, frame);

    iter.frame_num = frame.frame_num;
    iter.num_frames = dmux.num_frames;
    iter.x_offset = frame.x_offset;
    iter.y_offset = frame.y_offset;
    iter.width = frame.width;
    iter.height = frame.height;
    iter.has_alpha = frame.has_alpha;
    iter.duration = frame.duration;
    iter.dispose_method = frame.dispose_method;
    iter.blend_method = frame.blend_method;
    iter.complete = frame.complete;
    iter.fragment = WebPData {
        bytes: Cow::Borrowed(payload),
    };
    true
}

// upstream: src/demux/demux.c SetFrame
/// Sets the iterator to frame `frame_num`. Zero selects the last frame. Leaves
/// the iterator unchanged on failure.
fn set_frame(frame_num: i32, iter: &mut WebPIterator<'_>) -> bool {
    let dmux = iter.dmux;
    if frame_num < 0 {
        return false;
    }
    if frame_num > dmux.num_frames {
        return false;
    }
    let frame_num = if frame_num == 0 {
        dmux.num_frames
    } else {
        frame_num
    };

    let Some(frame) = dmux.get_frame(frame_num) else {
        return false;
    };
    synthesize_frame(dmux, frame, iter)
}

// upstream: src/demux/demux.c WebPDemuxGetFrame
/// Retrieves frame `frame` from `dmux`; zero gives the last frame. Returns `None`
/// when the frame is not present, where C returns 0 with a zeroed iterator.
#[must_use]
pub fn webp_demux_get_frame<'d>(dmux: &'d WebPDemuxer<'d>, frame: i32) -> Option<WebPIterator<'d>> {
    let mut iter = WebPIterator {
        frame_num: 0,
        num_frames: 0,
        x_offset: 0,
        y_offset: 0,
        width: 0,
        height: 0,
        duration: 0,
        dispose_method: WebPMuxAnimDispose::default(),
        complete: 0,
        fragment: WebPData::default(),
        has_alpha: 0,
        blend_method: WebPMuxAnimBlend::default(),
        pad: [0; 2],
        dmux,
    };
    set_frame(frame, &mut iter).then_some(iter)
}

// upstream: src/demux/demux.c WebPDemuxNextFrame
/// Sets the iterator to the next frame. Returns false when there is none.
pub fn webp_demux_next_frame(iter: &mut WebPIterator<'_>) -> bool {
    let next = iter.frame_num + 1;
    set_frame(next, iter)
}

// upstream: src/demux/demux.c WebPDemuxPrevFrame
/// Sets the iterator to the previous frame. Returns false when there is none.
pub fn webp_demux_prev_frame(iter: &mut WebPIterator<'_>) -> bool {
    if iter.frame_num <= 1 {
        return false;
    }
    let previous = iter.frame_num - 1;
    set_frame(previous, iter)
}

// upstream: src/demux/demux.c WebPDemuxReleaseIterator
/// Releases the iterator. C frees nothing here: the fragment borrows the input.
pub fn webp_demux_release_iterator(_iter: &mut WebPIterator<'_>) {}

// upstream: src/demux/demux.c SetChunk
/// Sets the iterator to the chunk `chunk_num` of `fourcc`. Zero selects the last
/// chunk. Leaves the iterator unchanged on failure.
fn set_chunk(fourcc: [u8; TAG_SIZE], chunk_num: i32, iter: &mut WebPChunkIterator<'_>) -> bool {
    let dmux = iter.dmux;
    if chunk_num < 0 {
        return false;
    }
    let count = dmux.chunk_count(fourcc);
    if count == 0 {
        return false;
    }
    let chunk_num = if chunk_num == 0 { count } else { chunk_num };

    if chunk_num <= count {
        let Some(chunk) = dmux.get_chunk(fourcc, chunk_num) else {
            return false;
        };
        let offset = chunk.data.offset;
        iter.chunk = WebPData {
            bytes: Cow::Borrowed(&dmux.buf[offset + CHUNK_HEADER_SIZE..offset + chunk.data.size]),
        };
        iter.num_chunks = count;
        iter.chunk_num = chunk_num;
        return true;
    }
    false
}

// upstream: src/demux/demux.c WebPDemuxGetChunk
/// Retrieves the `chunk_number`-th chunk with the tag `fourcc`; zero gives the
/// last one. Returns `None` when there is no such chunk, where C returns 0 with a
/// zeroed iterator.
#[must_use]
pub fn webp_demux_get_chunk<'d>(
    dmux: &'d WebPDemuxer<'d>,
    fourcc: [u8; TAG_SIZE],
    chunk_number: i32,
) -> Option<WebPChunkIterator<'d>> {
    let mut iter = WebPChunkIterator {
        chunk_num: 0,
        num_chunks: 0,
        chunk: WebPData::default(),
        pad: [0; 6],
        dmux,
        fourcc,
    };
    set_chunk(fourcc, chunk_number, &mut iter).then_some(iter)
}

// upstream: src/demux/demux.c WebPDemuxNextChunk
/// Sets the iterator to the next chunk with the same tag. Returns false when
/// there is none.
pub fn webp_demux_next_chunk(iter: &mut WebPChunkIterator<'_>) -> bool {
    let fourcc = iter.fourcc;
    let next = iter.chunk_num + 1;
    set_chunk(fourcc, next, iter)
}

// upstream: src/demux/demux.c WebPDemuxPrevChunk
/// Sets the iterator to the previous chunk with the same tag. Returns false when
/// there is none.
pub fn webp_demux_prev_chunk(iter: &mut WebPChunkIterator<'_>) -> bool {
    if iter.chunk_num <= 1 {
        return false;
    }
    let fourcc = iter.fourcc;
    let previous = iter.chunk_num - 1;
    set_chunk(fourcc, previous, iter)
}

// upstream: src/demux/demux.c WebPDemuxReleaseChunkIterator
/// Releases the iterator. C frees nothing here: the payload borrows the input.
pub fn webp_demux_release_chunk_iterator(_iter: &mut WebPChunkIterator<'_>) {}
