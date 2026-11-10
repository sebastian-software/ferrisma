// libwebp v1.6.0, src/dec/idec_dec.c:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Incremental decoding
//
// Author: somnath@google.com (Somnath Banerjee)

//! Translation of `src/dec/idec_dec.c` from libwebp v1.6.0.
//!
//! Work package: WP12 (decoder integration).
//!
//! The decoder keeps `io`, `params`, the VP8 or VP8L decoder, `mem` and its
//! output buffers as plain fields and passes disjoint borrows of them to each
//! call (ADR 0003, "Decoding context per call"). The input of a decoding step is
//! the memory buffer: in append mode the decoder's own `mem.buf`, in map mode the
//! caller's buffer, which `WebPIUpdate` passes and which the decoder does not
//! keep between calls. Every C pointer into that input is an index of it: the
//! readers of the VP8 decoder (`VP8GetHeaders` sets them up at `mem.start`), the
//! alpha chunk and the token partitions; `DoRemap` moves them by the index
//! delta that C adds to its pointers, which is 0 in map mode, where the bytes
//! keep their positions in the caller's growing buffer. Partition #0 is read
//! from `mem.part0_buf` once `CopyParts0Data` copied it in append mode. The VP8L
//! reader reads the input from `mem.start` on.
//!
//! The io hooks are the custom ones of `WebPInitCustomIo` over `params` and the
//! output buffer, or the caller's from `WebPISetIOHooks`; they are made for
//! each call. C's `params.output` is the decoder's own `output` or the caller's
//! buffer (`user_output`). For slow external memory the caller's buffer is
//! `final_output` until `FinishDecoding` copies the samples into it; C then
//! copies the caller's buffer structure into its own `output`, which here
//! becomes `params.output = user_output` (see `# Deviations from upstream`,
//! item 1).
//!
//! # Representation
//!
//! 1. **Incremental decoder.** `WebPIDecoder<'a, 'b>` keeps `io`, `params`, the
//!    decoder (`enum` of a boxed `VP8Decoder` and `VP8LDecoder`), `mem` and its
//!    own output buffer as plain fields. `'a` is what the caller lends for the
//!    decoder's life (the options and output buffer of a `WebPDecoderConfig`,
//!    the output buffer of `WebPINewDecoder`, the hooks of `WebPISetIOHooks`);
//!    `'b` is the caller memory that an output buffer borrows. C's
//!    `params.output` is the caller's buffer (`user_output`) or the decoder's
//!    own. Two lifetimes are needed because a mutable borrow is invariant: one
//!    lifetime would keep the configuration borrowed for its whole existence.
//!    Dropping the decoder is `WebPIDelete`.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **The slow copy of the incremental decoder.** After `FinishDecoding`
//!    copies into the caller's buffer, C copies the caller's buffer structure
//!    into its own `output` (`*output = *idec->final_output`), which
//!    `params.output` points at. Safe Rust cannot hold a second view of the
//!    caller's memory, so `params.output` becomes the caller's buffer, which
//!    has the same contents; the C ABI returns C's copy of the structure from
//!    `WebPIDecodedArea`, as C does. In the C ABI the decoder's own block stays
//!    allocated until `WebPIDelete` (C frees it at the copy).
//! 2. **Allocation sizes.** `CopyParts0Data` tests `WebPSafeMalloc(1, size)` as
//!    `size` elements of one byte (the same outcome). `NewDecoder`, `VP8New`
//!    and `VP8LNew` cannot fail; a refused host allocation of a fixed-size
//!    object aborts.
//!
//! # Calling the VP8 decoder
//!
//! How `WebPIDecoder` drives the VP8 decoder:
//!
//! 1. `WebPIDecoder` keeps `io`, `params`, `dec`, `mem` and the output side by
//!    side and passes disjoint borrows: `vp8_enter_critical(dec, io, hooks)`,
//!    `vp8_init_frame(dec, io, alpha_data)`, then per row
//!    `vp8_parse_intra_mode_row`, `vp8_decode_mb(dec, token_br, data)` for each
//!    macroblock, `vp8_init_scanline` and `vp8_process_row(dec, io, alpha_data,
//!    hooks)`, and `vp8_exit_critical`.
//! 2. The same `data` and `alpha_data` must reach every call of one decoding;
//!    `alpha_data` is the bytes in `mem`, so `DoRemap` needs no alpha update (C
//!    moves `dec->alpha_data`), and `NeedCompressedAlpha` tests
//!    `alpha_data.is_some()` and `!dec.is_alpha_decoded`.
//! 3. `DoRemap` remaps `dec.br` and `dec.parts` with index deltas (the bit
//!    readers store positions). `RestoreContext` copies `dec.br` and
//!    `dec.parts[token_br]` field by field (the bit reader has no `Clone`).
//! 4. `VP8Clear` before an `End` with a pending row is never needed: call
//!    `vp8_exit_critical` first, as `IDecError` does.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. **`WebPIAppend` of zero bytes before any data**: C asserts
//!    `mem->buf != NULL` and calls `memcpy` with a `NULL` destination.
//! 2. **`WebPISetIOHooks` with a lossless image**: C's `VP8LDecodeImage` reads
//!    the caller's user data as `WebPDecParams`; Rust uses the decoder's
//!    parameters (ADR 0003).

use crate::dec::buffer_dec::{
    is_null_plane, webp_allocate_dec_buffer, webp_avoid_slow_memory, webp_copy_dec_buffer_pixels,
    webp_flip_buffer, webp_free_dec_buffer,
};
use crate::dec::frame_dec::{
    vp8_enter_critical, vp8_exit_critical, vp8_get_thread_method, vp8_init_dithering,
    vp8_init_frame, vp8_process_row, vp8_sync_rows,
};
use crate::dec::io_dec::{CustomIo, webp_init_custom_io};
use crate::dec::tree_dec::vp8_parse_intra_mode_row;
use crate::dec::vp8_dec::{
    VP8Io, VP8IoHooks, vp8_decode_mb, vp8_get_headers_at, vp8_get_info, vp8_init_io,
    vp8_init_scanline, vp8_new,
};
use crate::dec::vp8i_dec::{VP8Decoder, VP8MB};
use crate::dec::vp8l_dec::{vp8l_decode_header, vp8l_decode_image, vp8l_new};
use crate::dec::vp8li_dec::VP8LDecoder;
use crate::dec::webp_dec::{webp_parse_headers, webp_reset_dec_params};
use crate::dec::webpi_dec::{WebPDecParams, WebPHeaderStructure};
use crate::utils::bit_reader_utils::{
    VP8BitReader, vp8_bit_reader_set_buffer, vp8_remap_bit_reader, vp8l_bit_reader_set_buffer,
};
use crate::utils::utils::webp_safe_malloc;
use crate::webp::decode::{
    VP8StatusCode, WEBP_CSP_MODE, WebPBitstreamFeatures, WebPDecBuffer, WebPDecBufferPlanes,
    WebPDecoderConfig, WebPDecoderOptions, WebPRGBABuffer, WebPYUVABuffer, webp_get_features,
    webp_init_dec_buffer,
};
use crate::webp::format_constants::{
    ALPHA_HEADER_LEN, ALPHA_LOSSLESS_COMPRESSION, MAX_CHUNK_PAYLOAD, VP8_FRAME_HEADER_SIZE,
};
use crate::webp::types::{PlaneOffset, PlaneStorage};

// upstream: src/dec/idec_dec.c CHUNK_SIZE
/// In append mode, buffer allocations increase as multiples of this value.
/// Needs to be a power of 2.
const CHUNK_SIZE: u64 = 4096;
// upstream: src/dec/idec_dec.c MAX_MB_SIZE
const MAX_MB_SIZE: usize = 4096;

//------------------------------------------------------------------------------
// Data structures for memory and states

// upstream: src/dec/idec_dec.c DecState
/// Decoding states. State normally flows as:
/// `WEBP_HEADER->VP8_HEADER->VP8_PARTS0->VP8_DATA->DONE` for a lossy image, and
/// `WEBP_HEADER->VP8L_HEADER->VP8L_DATA->DONE` for a lossless image.
/// If there is any error the decoder goes into state `ERROR`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DecState(u32);

impl DecState {
    // upstream: src/dec/idec_dec.c STATE_WEBP_HEADER
    /// All the data before that of the VP8/VP8L chunk.
    const STATE_WEBP_HEADER: Self = Self(0);
    // upstream: src/dec/idec_dec.c STATE_VP8_HEADER
    /// The VP8 Frame header (within the VP8 chunk).
    const STATE_VP8_HEADER: Self = Self(1);
    // upstream: src/dec/idec_dec.c STATE_VP8_PARTS0
    const STATE_VP8_PARTS0: Self = Self(2);
    // upstream: src/dec/idec_dec.c STATE_VP8_DATA
    const STATE_VP8_DATA: Self = Self(3);
    // upstream: src/dec/idec_dec.c STATE_VP8L_HEADER
    const STATE_VP8L_HEADER: Self = Self(4);
    // upstream: src/dec/idec_dec.c STATE_VP8L_DATA
    const STATE_VP8L_DATA: Self = Self(5);
    // upstream: src/dec/idec_dec.c STATE_DONE
    const STATE_DONE: Self = Self(6);
    // upstream: src/dec/idec_dec.c STATE_ERROR
    const STATE_ERROR: Self = Self(7);
}

// upstream: src/dec/idec_dec.c MemBufferMode
/// Operating state for the `MemBuffer`.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct MemBufferMode(u32);

impl MemBufferMode {
    // upstream: src/dec/idec_dec.c MEM_MODE_NONE
    const MEM_MODE_NONE: Self = Self(0);
    // upstream: src/dec/idec_dec.c MEM_MODE_APPEND
    const MEM_MODE_APPEND: Self = Self(1);
    // upstream: src/dec/idec_dec.c MEM_MODE_MAP
    const MEM_MODE_MAP: Self = Self(2);
}

// upstream: src/dec/idec_dec.c MemBuffer
/// Storage for partition #0 and partial data (in a rolling fashion).
///
/// `buf` is the decoder's own memory in append mode (`buf_size` bytes, empty
/// where C has `NULL`). In map mode C points `buf` at the caller's buffer; here
/// the caller passes it to each `WebPIUpdate`, and `buf` stays empty.
/// `part0_buf` is empty where C has `NULL`.
#[derive(Debug, Default)]
struct MemBuffer {
    /// Operation mode.
    mode: MemBufferMode,
    /// Start location of the data to be decoded.
    start: usize,
    /// End location.
    end: usize,
    /// Size of the allocated buffer.
    buf_size: usize,
    /// The data in append mode.
    buf: Vec<u8>,
    /// Size of partition #0.
    part0_size: usize,
    /// Buffer to store partition #0.
    part0_buf: Vec<u8>,
}

/// C's `void* dec`, which `is_lossless` down-casts: either a `VP8Decoder` or a
/// `VP8LDecoder` instance. This type has no C counterpart.
enum IDecDecoder {
    /// The lossy decoder.
    VP8(Box<VP8Decoder>),
    /// The lossless decoder.
    VP8L(Box<VP8LDecoder>),
}

// upstream: src/dec/idec_dec.c WebPIDecoder
/// The incremental decoder.
///
/// `'a` is the lifetime of what the caller lends to the decoder for its whole
/// life: the decoding options and output buffer of a `WebPDecoderConfig`, or
/// the output buffer of `WebPINewDecoder`, and the hooks of `WebPISetIOHooks`.
/// `'b` is the lifetime of the caller's memory that an output buffer borrows.
/// C's `params.output` is `user_output` when the caller supplied a buffer and
/// `output` otherwise. C keeps the alpha chunk in the VP8 decoder
/// (`dec->alpha_data`); here it is a position in the input, kept by its owner.
pub struct WebPIDecoder<'a, 'b> {
    /// Current decoding state.
    state: DecState,
    /// Params to store output info.
    params: WebPDecParams<'a>,
    /// For down-casting `dec`.
    is_lossless: bool,
    /// Either a `VP8Decoder` or a `VP8LDecoder` instance; `None` is `NULL`.
    dec: Option<IDecDecoder>,
    io: VP8Io,

    /// Input memory buffer.
    mem: MemBuffer,
    /// Output buffer (when no external one is supplied, or if the external
    /// one has slow-memory).
    output: WebPDecBuffer<'b>,
    /// The caller's output buffer, which C's `params.output` points at when
    /// one was supplied without slow memory, or after the slow copy.
    user_output: Option<&'a mut WebPDecBuffer<'b>>,
    /// Slow-memory output to copy to eventually.
    final_output: Option<&'a mut WebPDecBuffer<'b>>,
    /// Set when `NewDecoder` chose a `final_output` (no C counterpart: C's
    /// `params.output` then stays `&idec->output`, which the C ABI reproduces).
    slow_copy: bool,
    /// Compressed VP8/VP8L size extracted from Header.
    chunk_size: usize,

    /// Last row reached for intra-mode decoding.
    last_mb_y: i32,

    /// The alpha chunk of the VP8 decoder, as a position in the input (C:
    /// `dec->alpha_data`, `NULL` is `None`).
    alpha_data: Option<usize>,
    /// Its size (C: `dec->alpha_data_size`).
    alpha_data_size: usize,
    /// The hooks of `WebPISetIOHooks` with their user data; `None` stands for
    /// the custom hooks of `WebPInitCustomIo`.
    hooks: Option<Box<dyn VP8IoHooks + 'a>>,
}

// upstream: src/dec/idec_dec.c MBContext
/// MB context to restore in case `VP8DecodeMB()` fails.
struct MBContext {
    left: VP8MB,
    info: VP8MB,
    token_br: VP8BitReader,
}

/// The lossy decoder of `dec`.
///
/// # Panics
///
/// Panics when the decoder is not a VP8 decoder, where C would down-cast the
/// wrong object.
fn vp8_dec(dec: &mut Option<IDecDecoder>) -> &mut VP8Decoder {
    match dec {
        Some(IDecDecoder::VP8(dec)) => dec,
        _ => panic!("the decoder is a VP8 decoder"),
    }
}

/// The lossless decoder of `dec`; see [`vp8_dec`].
fn vp8l_dec(dec: &mut Option<IDecDecoder>) -> &mut VP8LDecoder {
    match dec {
        Some(IDecDecoder::VP8L(dec)) => dec,
        _ => panic!("the decoder is a VP8L decoder"),
    }
}

/// The status code of a `Result`, as C stores it in `dec->status`.
fn status_code(result: Result<(), VP8StatusCode>) -> VP8StatusCode {
    result.err().unwrap_or(VP8StatusCode::VP8_STATUS_OK)
}

/// C's `params.output`: the caller's buffer, or the decoder's own.
fn params_output<'s, 'b>(
    output: &'s mut WebPDecBuffer<'b>,
    user_output: &'s mut Option<&'_ mut WebPDecBuffer<'b>>,
) -> &'s mut WebPDecBuffer<'b> {
    match user_output {
        Some(user_output) => user_output,
        None => output,
    }
}

/// The io hooks of one call: the caller's, or the custom hooks of
/// `WebPInitCustomIo` over the parameters and the output buffer (C:
/// `io->put`, `io->setup`, `io->teardown` and `io->opaque`). This type has no C
/// counterpart.
enum IoHooks<'s, 'a, 'b> {
    /// The hooks of `WebPISetIOHooks`.
    User(&'s mut (dyn VP8IoHooks + 'a)),
    /// The hooks of `WebPInitCustomIo`.
    Custom(CustomIo<'s, 'a, 'b>),
}

impl IoHooks<'_, '_, '_> {
    /// The hooks as the decoding functions take them.
    fn get(&mut self) -> &mut dyn VP8IoHooks {
        match self {
            Self::User(hooks) => &mut **hooks,
            Self::Custom(hooks) => hooks,
        }
    }
}

/// The io hooks of one call; see [`IoHooks`].
fn io_hooks<'s, 'a, 'b>(
    hooks: &'s mut Option<Box<dyn VP8IoHooks + 'a>>,
    params: &'s mut WebPDecParams<'a>,
    output: &'s mut WebPDecBuffer<'b>,
    user_output: &'s mut Option<&'a mut WebPDecBuffer<'b>>,
    io: &mut VP8Io,
) -> IoHooks<'s, 'a, 'b> {
    match hooks {
        Some(hooks) => IoHooks::User(hooks.as_mut()),
        None => IoHooks::Custom(webp_init_custom_io(
            params,
            params_output(output, user_output),
            io,
        )),
    }
}

/// The alpha chunk in the input (C: `dec->alpha_data[..alpha_data_size]`).
/// Once the alpha plane is decoded, C no longer moves `alpha_data` with the
/// input and never reads it again; the stale position then gives the bytes that
/// happen to be there, or none.
fn alpha_slice(input: &[u8], alpha_data: Option<usize>, alpha_data_size: usize) -> Option<&[u8]> {
    alpha_data.map(|alpha| {
        input
            .get(alpha..alpha + alpha_data_size)
            .unwrap_or_default()
    })
}

/// A copy of a reader (WP2's reader has no `Clone`): C copies the structure.
fn copy_bit_reader(br: &VP8BitReader) -> VP8BitReader {
    VP8BitReader {
        value: br.value,
        range: br.range,
        bits: br.bits,
        has_buf: br.has_buf,
        buf: br.buf,
        buf_end: br.buf_end,
        buf_max: br.buf_max,
        eof: br.eof,
    }
}

/// `pointer += offset` for an index.
///
/// # Panics
///
/// Panics when the index would become negative, which means that `offset` does
/// not describe a move of the input's bytes.
fn add_offset(index: usize, offset: isize) -> usize {
    index
        .checked_add_signed(offset)
        .expect("the moved position stays in the input")
}

/// An index as the signed difference C computes between pointers.
fn signed(index: usize) -> isize {
    isize::try_from(index).expect("a position in the input fits in isize")
}

//------------------------------------------------------------------------------
// MemBuffer: incoming data handling

// upstream: src/dec/idec_dec.c MemDataSize
fn mem_data_size(mem: &MemBuffer) -> usize {
    mem.end - mem.start
}

// upstream: src/dec/idec_dec.c NeedCompressedAlpha
/// Checks if we need to preserve the compressed alpha data, as it may not have
/// been decoded yet.
fn need_compressed_alpha(idec: &WebPIDecoder<'_, '_>) -> bool {
    if idec.state == DecState::STATE_WEBP_HEADER {
        // We haven't parsed the headers yet, so we don't know whether the image is
        // lossy or lossless. This also means that we haven't parsed the ALPH chunk.
        return false;
    }
    if idec.is_lossless {
        false // ALPH chunk is not present for lossless images.
    } else {
        // Must be a decoder as idec->state != STATE_WEBP_HEADER.
        let Some(IDecDecoder::VP8(dec)) = &idec.dec else {
            panic!("a lossy decoding past the headers has a VP8 decoder")
        };
        idec.alpha_data.is_some() && !dec.is_alpha_decoded
    }
}

// upstream: src/dec/idec_dec.c DoRemap
/// Moves the positions into the input by `offset`, the index delta of the
/// input's bytes (see the module documentation), and gives the last partition
/// and the lossless reader the input up to `mem.end`. C also points `io.data`
/// at the new input; here the input is passed per call.
fn do_remap(idec: &mut WebPIDecoder<'_, '_>, offset: isize) {
    let need_compressed_alpha = need_compressed_alpha(idec);
    let mem = &idec.mem;
    // note: for VP8, setting up idec->io is only really needed at the beginning
    // of the decoding, till partition #0 is complete.

    match &mut idec.dec {
        None => {}
        Some(IDecDecoder::VP8(dec)) => {
            let last_part = dec.num_parts_minus_one as usize;
            if offset != 0 {
                for p in 0..last_part {
                    vp8_remap_bit_reader(&mut dec.parts[p], offset);
                }
                // C moves the three pointers of the last partition too. Its end
                // and limit can then lie before the new buffer: with one
                // partition, `mem.start` follows the reader, which can be past its
                // limit near the end of the input. The buffer reset below
                // replaces both, so only the position moves here, which keeps
                // the positions non-negative.
                let last = &mut dec.parts[last_part];
                if last.has_buf {
                    last.buf = add_offset(last.buf, offset);
                }
                // Remap partition #0 data pointer to new offset, but only in MAP
                // mode (in APPEND mode, partition #0 is copied into a fixed memory).
                if mem.mode == MemBufferMode::MEM_MODE_MAP {
                    vp8_remap_bit_reader(&mut dec.br, offset);
                }
            }
            {
                let last = &mut dec.parts[last_part];
                // 'last_start' will be NULL when 'idec->state' is < STATE_VP8_PARTS0
                // and through a portion of that state (when there isn't enough data to
                // parse the partitions). The bitreader is only used meaningfully when
                // there is enough data to begin parsing partition 0.
                if last.has_buf {
                    let last_start = last.buf;
                    // C: VP8BitReaderSetBuffer(last, last_start, end - last_start).
                    vp8_bit_reader_set_buffer(last, mem.end - last_start);
                    vp8_remap_bit_reader(last, signed(last_start));
                }
            }
            if need_compressed_alpha {
                let alpha_data = add_offset(
                    idec.alpha_data
                        .expect("compressed alpha needs the alpha chunk"),
                    offset,
                );
                idec.alpha_data = Some(alpha_data);
                if let Some(alph_dec) = dec.alph_dec.as_deref_mut()
                    && let Some(alph_vp8l_dec) = alph_dec.vp8l_dec.as_deref_mut()
                {
                    if alph_dec.method == ALPHA_LOSSLESS_COMPRESSION {
                        let header_len = ALPHA_HEADER_LEN.unsigned_abs() as usize;
                        debug_assert!(idec.alpha_data_size >= header_len);
                        // C: VP8LBitReaderSetBuffer(&alph_vp8l_dec->br,
                        //   dec->alpha_data + ALPHA_HEADER_LEN, size - ALPHA_HEADER_LEN);
                        // the reader's positions are in the moved chunk.
                        vp8l_bit_reader_set_buffer(
                            &mut alph_vp8l_dec.br,
                            idec.alpha_data_size - header_len,
                        );
                    } else {
                        // alph_dec->method == ALPHA_NO_COMPRESSION
                        // Nothing special to do in this case.
                    }
                }
            }
        }
        Some(IDecDecoder::VP8L(dec)) => {
            // Resize lossless bitreader
            vp8l_bit_reader_set_buffer(&mut dec.br, mem_data_size(mem));
        }
    }
}

// upstream: src/dec/idec_dec.c AppendToMemBuffer
/// Appends data to the end of `MemBuffer->buf`. It expands the allocated memory
/// size if required and also updates the readers if new memory is allocated.
/// Returns false when the data is too large or the allocation fails.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C converts the uint64_t size to size_t, which holds it after WebPSafeMalloc"
)]
fn append_to_mem_buffer(idec: &mut WebPIDecoder<'_, '_>, data: &[u8]) -> bool {
    let need_compressed_alpha = need_compressed_alpha(idec);
    let mem = &mut idec.mem;
    // C: old_start is NULL without a buffer, where start is 0 (asserted below).
    let old_start = mem.start;
    let old_base = if need_compressed_alpha {
        idec.alpha_data
            .expect("compressed alpha needs the alpha chunk")
    } else {
        old_start
    };
    debug_assert!(!mem.buf.is_empty() || mem.start == 0);
    debug_assert_eq!(mem.mode, MemBufferMode::MEM_MODE_APPEND);
    if data.len() as u64 > u64::from(MAX_CHUNK_PAYLOAD) {
        // security safeguard: trying to allocate more than what the format
        // allows for a chunk should be considered a smoke smell.
        return false;
    }

    if mem.end + data.len() > mem.buf_size {
        // Need some free memory
        let new_mem_start = old_start - old_base;
        let current_size = mem_data_size(mem) + new_mem_start;
        let new_size = current_size as u64 + data.len() as u64;
        let extra_size = (new_size + CHUNK_SIZE - 1) & !(CHUNK_SIZE - 1);
        let Some(mut new_buf) = webp_safe_malloc::<u8>(extra_size, 1) else {
            return false;
        };
        // Rust initializes the allocation (ADR 0003, allocation limits).
        new_buf.resize(extra_size as usize, 0);
        if !mem.buf.is_empty() {
            // C: if (old_base != NULL)
            new_buf[..current_size].copy_from_slice(&mem.buf[old_base..old_base + current_size]);
        }
        // C: WebPSafeFree(mem->buf).
        mem.buf = new_buf;
        mem.buf_size = extra_size as usize;
        mem.start = new_mem_start;
        mem.end = current_size;
    }

    debug_assert_ne!(mem.buf.len(), 0);
    mem.buf[mem.end..mem.end + data.len()].copy_from_slice(data);
    mem.end += data.len();
    debug_assert!(mem.end <= mem.buf_size);

    let offset = signed(mem.start) - signed(old_start);
    do_remap(idec, offset);
    true
}

// upstream: src/dec/idec_dec.c RemapMemBuffer
/// Makes the memory buffer the caller's `data`, which holds the data so far
/// at the same positions. C points `mem->buf` at `data` and moves its pointers
/// by the distance between the old and the new buffer; the positions here are
/// indexes of the caller's buffer, which do not change. Returns false for a
/// shorter buffer.
fn remap_mem_buffer(idec: &mut WebPIDecoder<'_, '_>, data: &[u8]) -> bool {
    let mem = &mut idec.mem;
    let old_start = mem.start;
    debug_assert_eq!(mem.mode, MemBufferMode::MEM_MODE_MAP);

    if data.len() < mem.buf_size {
        return false; // can't remap to a shorter buffer!
    }

    // C: mem->buf = (uint8_t*)data; the caller passes it to each call.
    mem.end = data.len();
    mem.buf_size = data.len();

    let offset = signed(mem.start) - signed(old_start);
    do_remap(idec, offset);
    true
}

// upstream: src/dec/idec_dec.c InitMemBuffer
fn init_mem_buffer(mem: &mut MemBuffer) {
    mem.mode = MemBufferMode::MEM_MODE_NONE;
    mem.buf = Vec::new();
    mem.buf_size = 0;
    mem.part0_buf = Vec::new();
    mem.part0_size = 0;
}

// upstream: src/dec/idec_dec.c ClearMemBuffer
fn clear_mem_buffer(mem: &mut MemBuffer) {
    if mem.mode == MemBufferMode::MEM_MODE_APPEND {
        mem.buf = Vec::new();
        mem.part0_buf = Vec::new();
    }
}

// upstream: src/dec/idec_dec.c CheckMemBufferMode
/// Returns false when the calls mix the modes.
fn check_mem_buffer_mode(mem: &mut MemBuffer, expected: MemBufferMode) -> bool {
    if mem.mode == MemBufferMode::MEM_MODE_NONE {
        mem.mode = expected; // switch to the expected mode
    } else if mem.mode != expected {
        return false; // we mixed the modes => error
    }
    debug_assert_eq!(mem.mode, expected); // mode is ok
    true
}

// upstream: src/dec/idec_dec.c FinishDecoding
/// To be called last. After the slow copy, C copies the structure of
/// `final_output` into its own `output`, which `params.output` points at; here
/// `params.output` becomes the caller's buffer itself.
fn finish_decoding(idec: &mut WebPIDecoder<'_, '_>) -> Result<(), VP8StatusCode> {
    let options = idec.params.options;
    let output = params_output(&mut idec.output, &mut idec.user_output);

    idec.state = DecState::STATE_DONE;
    if options.is_some_and(|options| options.flip != 0) {
        webp_flip_buffer(output)?;
    }
    if let Some(final_output) = idec.final_output.as_deref_mut() {
        // do the slow-copy
        let status = webp_copy_dec_buffer_pixels(&idec.output, final_output);
        webp_free_dec_buffer(&mut idec.output);
        status?;
        // C: *output = *idec->final_output; idec->final_output = NULL.
        idec.user_output = idec.final_output.take();
    }
    Ok(())
}

//------------------------------------------------------------------------------
// Macroblock-decoding contexts

// upstream: src/dec/idec_dec.c SaveContext
/// C takes the token reader as a pointer; here it is its index in `dec.parts`.
fn save_context(dec: &VP8Decoder, token_br: usize) -> MBContext {
    let mb_x = usize::try_from(dec.mb_x).expect("a non-negative position");
    MBContext {
        left: dec.mb_info[0],        // C: dec->mb_info[-1]
        info: dec.mb_info[mb_x + 1], // C: dec->mb_info[dec->mb_x]
        token_br: copy_bit_reader(&dec.parts[token_br]),
    }
}

// upstream: src/dec/idec_dec.c RestoreContext
fn restore_context(context: &MBContext, dec: &mut VP8Decoder, token_br: usize) {
    let mb_x = usize::try_from(dec.mb_x).expect("a non-negative position");
    dec.mb_info[0] = context.left;
    dec.mb_info[mb_x + 1] = context.info;
    dec.parts[token_br] = copy_bit_reader(&context.token_br);
}

//------------------------------------------------------------------------------

// upstream: src/dec/idec_dec.c IDecError
/// Enters the error state and returns `error`. `input` is the input of the
/// call, which the synchronization reads for a row's alpha.
fn i_dec_error(
    idec: &mut WebPIDecoder<'_, '_>,
    input: &[u8],
    error: VP8StatusCode,
) -> VP8StatusCode {
    if idec.state == DecState::STATE_VP8_DATA {
        // Synchronize the thread, clean-up and check for errors.
        let WebPIDecoder {
            dec,
            io,
            params,
            output,
            user_output,
            hooks,
            alpha_data,
            alpha_data_size,
            ..
        } = idec;
        let alpha = alpha_slice(input, *alpha_data, *alpha_data_size);
        let mut hooks = io_hooks(hooks, params, output, user_output, io);
        let _ = vp8_exit_critical(vp8_dec(dec), io, alpha, Some(hooks.get()));
    }
    idec.state = DecState::STATE_ERROR;
    error
}

// upstream: src/dec/idec_dec.c ChangeState
/// C also points `io.data` at the remaining input; here it is passed per call.
fn change_state(idec: &mut WebPIDecoder<'_, '_>, new_state: DecState, consumed_bytes: usize) {
    let mem = &mut idec.mem;
    idec.state = new_state;
    mem.start += consumed_bytes;
    debug_assert!(mem.start <= mem.end);
}

// upstream: src/dec/idec_dec.c DecodeWebPHeaders
/// Headers.
fn decode_webp_headers(idec: &mut WebPIDecoder<'_, '_>, input: &[u8]) -> Result<(), VP8StatusCode> {
    let mem = &idec.mem;
    let data = &input[mem.start..mem.end];
    let start = mem.start;
    let mut headers = WebPHeaderStructure {
        data,
        have_all_data: false,
        ..WebPHeaderStructure::default()
    };

    match webp_parse_headers(&mut headers) {
        Ok(()) => {}
        Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA) => {
            return Err(VP8StatusCode::VP8_STATUS_SUSPENDED); // We haven't found a VP8 chunk yet.
        }
        Err(status) => return Err(i_dec_error(idec, input, status)),
    }

    idec.chunk_size = headers.compressed_size;
    idec.is_lossless = headers.is_lossless;
    if idec.is_lossless {
        // C: VP8LNew() returns NULL when the allocation fails.
        let dec = vp8l_new();
        idec.dec = Some(IDecDecoder::VP8L(dec));
        change_state(idec, DecState::STATE_VP8L_HEADER, headers.offset);
    } else {
        // C: VP8New() returns NULL when the allocation fails.
        let mut dec = vp8_new();
        dec.incremental = true;
        idec.dec = Some(IDecDecoder::VP8(dec));
        idec.alpha_data = headers.alpha_data.map(|alpha| start + alpha);
        idec.alpha_data_size = headers.alpha_data_size;
        change_state(idec, DecState::STATE_VP8_HEADER, headers.offset);
    }
    Ok(())
}

// upstream: src/dec/idec_dec.c DecodeVP8FrameHeader
fn decode_vp8_frame_header(
    idec: &mut WebPIDecoder<'_, '_>,
    input: &[u8],
) -> Result<(), VP8StatusCode> {
    let data = &input[idec.mem.start..idec.mem.end];
    let curr_size = mem_data_size(&idec.mem);

    if curr_size < VP8_FRAME_HEADER_SIZE.unsigned_abs() as usize {
        // Not enough data bytes to extract VP8 Frame Header.
        return Err(VP8StatusCode::VP8_STATUS_SUSPENDED);
    }
    if vp8_get_info(data, idec.chunk_size).is_none() {
        return Err(i_dec_error(
            idec,
            input,
            VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR,
        ));
    }

    let bits = u32::from(data[0]) | (u32::from(data[1]) << 8) | (u32::from(data[2]) << 16);
    idec.mem.part0_size = (bits >> 5) as usize + VP8_FRAME_HEADER_SIZE.unsigned_abs() as usize;

    // C: idec->io.data = data; idec->io.data_size = curr_size.
    idec.state = DecState::STATE_VP8_PARTS0;
    Ok(())
}

// upstream: src/dec/idec_dec.c CopyParts0Data
/// Partition #0.
fn copy_parts0_data(idec: &mut WebPIDecoder<'_, '_>, input: &[u8]) -> Result<(), VP8StatusCode> {
    debug_assert!(!idec.is_lossless);
    let dec = vp8_dec(&mut idec.dec);
    let br = &mut dec.br;
    let part_size = br.buf_end - br.buf;
    let mem = &mut idec.mem;
    debug_assert_eq!(mem.part0_buf.len(), 0);
    // the following is a format limitation, no need for runtime check:
    debug_assert!(part_size <= mem.part0_size);
    if part_size == 0 {
        // can't have zero-size partition #0
        return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
    }
    if mem.mode == MemBufferMode::MEM_MODE_APPEND {
        // We copy and grab ownership of the partition #0 data. C tests
        // WebPSafeMalloc(1ULL, part_size); the test of part_size elements of
        // one byte makes the same decision.
        let Some(mut part0_buf) = webp_safe_malloc::<u8>(part_size as u64, 1) else {
            return Err(VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY);
        };
        part0_buf.extend_from_slice(&input[br.buf..br.buf + part_size]);
        mem.part0_buf = part0_buf;
        vp8_bit_reader_set_buffer(br, part_size);
    } else {
        // Else: just keep pointers to the partition #0's data in dec->br.
    }
    mem.start += part_size;
    Ok(())
}

// upstream: src/dec/idec_dec.c DecodePartition0
fn decode_partition0(idec: &mut WebPIDecoder<'_, '_>, input: &[u8]) -> Result<(), VP8StatusCode> {
    // Wait till we have enough data for the whole partition #0
    if mem_data_size(&idec.mem) < idec.mem.part0_size {
        return Err(VP8StatusCode::VP8_STATUS_SUSPENDED);
    }

    {
        let dec = vp8_dec(&mut idec.dec);
        // C: io->data, which points mem.start bytes into the input.
        if vp8_get_headers_at(dec, &mut idec.io, &input[..idec.mem.end], idec.mem.start).is_err() {
            let status = dec.status;
            if status == VP8StatusCode::VP8_STATUS_SUSPENDED
                || status == VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA
            {
                // treating NOT_ENOUGH_DATA as SUSPENDED state
                return Err(VP8StatusCode::VP8_STATUS_SUSPENDED);
            }
            return Err(i_dec_error(idec, input, status));
        }
    }

    // Allocate/Verify output buffer now
    let status = {
        let WebPIDecoder {
            dec,
            io,
            params,
            output,
            user_output,
            ..
        } = idec;
        let dec = vp8_dec(dec);
        dec.status = status_code(webp_allocate_dec_buffer(
            io.width,
            io.height,
            params.options,
            params_output(output, user_output),
        ));
        dec.status
    };
    if status != VP8StatusCode::VP8_STATUS_OK {
        return Err(i_dec_error(idec, input, status));
    }
    {
        let WebPIDecoder {
            dec, io, params, ..
        } = idec;
        let dec = vp8_dec(dec);
        // This change must be done before calling VP8InitFrame(). C passes no
        // headers to VP8GetThreadMethod.
        dec.mt_method = vp8_get_thread_method(params.options, io.width, io.height);
        vp8_init_dithering(params.options, dec);
    }

    let status = status_code(copy_parts0_data(idec, input));
    vp8_dec(&mut idec.dec).status = status;
    if status != VP8StatusCode::VP8_STATUS_OK {
        return Err(i_dec_error(idec, input, status));
    }

    // Finish setting up the decoding parameters. Will call io->setup().
    let entered = {
        let WebPIDecoder {
            dec,
            io,
            params,
            output,
            user_output,
            hooks,
            ..
        } = idec;
        let dec = vp8_dec(dec);
        let mut hooks = io_hooks(hooks, params, output, user_output, io);
        vp8_enter_critical(dec, io, Some(hooks.get())).map_err(|_| dec.status)
    };
    if let Err(status) = entered {
        return Err(i_dec_error(idec, input, status));
    }

    // Note: past this point, teardown() must always be called
    // in case of error.
    idec.state = DecState::STATE_VP8_DATA;
    // Allocate memory and prepare everything.
    let alpha = alpha_slice(input, idec.alpha_data, idec.alpha_data_size);
    let dec = vp8_dec(&mut idec.dec);
    if !vp8_init_frame(dec, &mut idec.io, alpha) {
        let status = dec.status;
        return Err(i_dec_error(idec, input, status));
    }
    Ok(())
}

/// How the row loop of `DecodeRemaining` ends. This type has no C counterpart.
enum RowsEnd {
    /// The loop ran through every row.
    Done,
    /// A macroblock needs more data: return `VP8_STATUS_SUSPENDED`.
    Suspended,
    /// `IDecError` with this status.
    Error(VP8StatusCode),
}

// upstream: src/dec/idec_dec.c DecodeRemaining
/// Remaining partitions.
fn decode_remaining(idec: &mut WebPIDecoder<'_, '_>, input: &[u8]) -> Result<(), VP8StatusCode> {
    // Make sure partition #0 has been read before, to set dec to ready.
    if !vp8_dec(&mut idec.dec).ready {
        return Err(i_dec_error(
            idec,
            input,
            VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR,
        ));
    }
    let rows_end = {
        let WebPIDecoder {
            dec,
            io,
            params,
            mem,
            output,
            user_output,
            last_mb_y,
            alpha_data,
            alpha_data_size,
            hooks,
            ..
        } = idec;
        let dec = vp8_dec(dec);
        let data = &input[..mem.end];
        let alpha = alpha_slice(input, *alpha_data, *alpha_data_size);
        let mut hooks = io_hooks(hooks, params, output, user_output, io);
        'rows: {
            while dec.mb_y < dec.mb_h {
                if *last_mb_y != dec.mb_y {
                    // Partition #0: its copy in append mode, the input in map mode.
                    let part0 = if mem.mode == MemBufferMode::MEM_MODE_APPEND {
                        mem.part0_buf.as_slice()
                    } else {
                        data
                    };
                    if !vp8_parse_intra_mode_row(dec, part0) {
                        // note: normally, error shouldn't occur since we already have the whole
                        // partition0 available here in DecodeRemaining(). Reaching EOF while
                        // reading intra modes really means a BITSTREAM_ERROR.
                        break 'rows RowsEnd::Error(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
                    }
                    *last_mb_y = dec.mb_y;
                }
                while dec.mb_x < dec.mb_w {
                    let token_br = (dec.mb_y.cast_unsigned() & dec.num_parts_minus_one) as usize;
                    let context = save_context(dec, token_br);
                    if !vp8_decode_mb(dec, token_br, data) {
                        // We shouldn't fail when MAX_MB data was available
                        if dec.num_parts_minus_one == 0 && mem_data_size(mem) > MAX_MB_SIZE {
                            break 'rows RowsEnd::Error(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
                        }
                        // Synchronize the threads.
                        if dec.mt_method > 0 && !vp8_sync_rows(dec, alpha, Some(hooks.get())) {
                            break 'rows RowsEnd::Error(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
                        }
                        restore_context(&context, dec, token_br);
                        break 'rows RowsEnd::Suspended;
                    }
                    // Release buffer only if there is only one partition
                    if dec.num_parts_minus_one == 0 {
                        mem.start = dec.parts[token_br].buf;
                        debug_assert!(mem.start <= mem.end);
                    }
                    dec.mb_x += 1;
                }
                vp8_init_scanline(dec); // Prepare for next scanline

                // Reconstruct, filter and emit the row.
                if !vp8_process_row(dec, io, alpha, Some(hooks.get())) {
                    break 'rows RowsEnd::Error(VP8StatusCode::VP8_STATUS_USER_ABORT);
                }
                dec.mb_y += 1;
            }
            RowsEnd::Done
        }
    };
    match rows_end {
        RowsEnd::Done => {}
        RowsEnd::Suspended => return Err(VP8StatusCode::VP8_STATUS_SUSPENDED),
        RowsEnd::Error(status) => return Err(i_dec_error(idec, input, status)),
    }
    // Synchronize the thread and check for errors.
    let exited = {
        let WebPIDecoder {
            dec,
            io,
            params,
            output,
            user_output,
            alpha_data,
            alpha_data_size,
            hooks,
            ..
        } = idec;
        let alpha = alpha_slice(input, *alpha_data, *alpha_data_size);
        let mut hooks = io_hooks(hooks, params, output, user_output, io);
        vp8_exit_critical(vp8_dec(dec), io, alpha, Some(hooks.get()))
    };
    if !exited {
        idec.state = DecState::STATE_ERROR; // prevent re-entry in IDecError
        return Err(i_dec_error(
            idec,
            input,
            VP8StatusCode::VP8_STATUS_USER_ABORT,
        ));
    }
    vp8_dec(&mut idec.dec).ready = false;
    finish_decoding(idec)
}

// upstream: src/dec/idec_dec.c ErrorStatusLossless
fn error_status_lossless(
    idec: &mut WebPIDecoder<'_, '_>,
    input: &[u8],
    status: VP8StatusCode,
) -> VP8StatusCode {
    if status == VP8StatusCode::VP8_STATUS_SUSPENDED
        || status == VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA
    {
        return VP8StatusCode::VP8_STATUS_SUSPENDED;
    }
    i_dec_error(idec, input, status)
}

// upstream: src/dec/idec_dec.c DecodeVP8LHeader
fn decode_vp8l_header(idec: &mut WebPIDecoder<'_, '_>, input: &[u8]) -> Result<(), VP8StatusCode> {
    let curr_size = mem_data_size(&idec.mem);
    debug_assert!(idec.is_lossless);

    // Wait until there's enough data for decoding header.
    if curr_size < (idec.chunk_size >> 3) {
        let dec = vp8l_dec(&mut idec.dec);
        dec.status = VP8StatusCode::VP8_STATUS_SUSPENDED;
        let status = dec.status;
        return Err(error_status_lossless(idec, input, status));
    }

    {
        let dec = vp8l_dec(&mut idec.dec);
        let data = &input[idec.mem.start..idec.mem.end];
        if vp8l_decode_header(dec, &mut idec.io, data).is_err() {
            if dec.status == VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR
                && curr_size < idec.chunk_size
            {
                dec.status = VP8StatusCode::VP8_STATUS_SUSPENDED;
            }
            let status = dec.status;
            return Err(error_status_lossless(idec, input, status));
        }
    }
    // Allocate/verify output buffer now.
    let status = {
        let WebPIDecoder {
            dec,
            io,
            params,
            output,
            user_output,
            ..
        } = idec;
        let dec = vp8l_dec(dec);
        dec.status = status_code(webp_allocate_dec_buffer(
            io.width,
            io.height,
            params.options,
            params_output(output, user_output),
        ));
        dec.status
    };
    if status != VP8StatusCode::VP8_STATUS_OK {
        return Err(i_dec_error(idec, input, status));
    }

    idec.state = DecState::STATE_VP8L_DATA;
    Ok(())
}

// upstream: src/dec/idec_dec.c DecodeVP8LData
/// `VP8LDecodeImage` takes the decoder's parameters and output buffer. C reads
/// the parameters through `io->opaque`, which is the caller's user data after
/// `WebPISetIOHooks`: undefined, and excluded (`# Excluded undefined behavior`,
/// item 2, in the module documentation).
fn decode_vp8l_data(idec: &mut WebPIDecoder<'_, '_>, input: &[u8]) -> Result<(), VP8StatusCode> {
    let curr_size = mem_data_size(&idec.mem);
    debug_assert!(idec.is_lossless);

    let status = {
        let WebPIDecoder {
            dec,
            io,
            params,
            mem,
            output,
            user_output,
            chunk_size,
            ..
        } = idec;
        let dec = vp8l_dec(dec);
        // Switch to incremental decoding if we don't have all the bytes available.
        dec.incremental = curr_size < *chunk_size;

        let data = &input[mem.start..mem.end];
        if vp8l_decode_image(dec, io, data, params, params_output(output, user_output)).is_err() {
            Err(dec.status)
        } else {
            debug_assert!(
                dec.status == VP8StatusCode::VP8_STATUS_OK
                    || dec.status == VP8StatusCode::VP8_STATUS_SUSPENDED
            );
            Ok(dec.status)
        }
    };
    match status {
        Err(status) => Err(error_status_lossless(idec, input, status)),
        Ok(VP8StatusCode::VP8_STATUS_SUSPENDED) => Err(VP8StatusCode::VP8_STATUS_SUSPENDED),
        Ok(_) => finish_decoding(idec),
    }
}

// upstream: src/dec/idec_dec.c IDecode
/// Main decoding loop. `input` is the memory buffer (see the module
/// documentation).
fn i_decode(idec: &mut WebPIDecoder<'_, '_>, input: &[u8]) -> Result<(), VP8StatusCode> {
    let mut status = Err(VP8StatusCode::VP8_STATUS_SUSPENDED);

    if idec.state == DecState::STATE_WEBP_HEADER {
        status = decode_webp_headers(idec, input);
    } else if idec.dec.is_none() {
        return Err(VP8StatusCode::VP8_STATUS_SUSPENDED); // can't continue if we have no decoder.
    }
    if idec.state == DecState::STATE_VP8_HEADER {
        status = decode_vp8_frame_header(idec, input);
    }
    if idec.state == DecState::STATE_VP8_PARTS0 {
        status = decode_partition0(idec, input);
    }
    if idec.state == DecState::STATE_VP8_DATA {
        if idec.dec.is_none() {
            return Err(VP8StatusCode::VP8_STATUS_SUSPENDED); // can't continue if we have no decoder.
        }
        status = decode_remaining(idec, input);
    }
    if idec.state == DecState::STATE_VP8L_HEADER {
        status = decode_vp8l_header(idec, input);
    }
    if idec.state == DecState::STATE_VP8L_DATA {
        status = decode_vp8l_data(idec, input);
    }
    status
}

//------------------------------------------------------------------------------
// Internal constructor

// upstream: src/dec/idec_dec.c NewDecoder
/// Creates the decoder over the caller's `output_buffer`, or over its own
/// output buffer for `None` or for slow memory, which receives the samples at
/// the end. C returns `NULL` when its allocation fails; here a refused host
/// allocation aborts.
fn new_decoder<'a, 'b>(
    output_buffer: Option<&'a mut WebPDecBuffer<'b>>,
    features: Option<&WebPBitstreamFeatures>,
) -> Option<WebPIDecoder<'a, 'b>> {
    let mut idec = WebPIDecoder {
        state: DecState::STATE_WEBP_HEADER,
        params: WebPDecParams::default(),
        is_lossless: false,
        dec: None,
        io: VP8Io::default(),
        mem: MemBuffer::default(),
        output: WebPDecBuffer::default(),
        user_output: None,
        final_output: None,
        slow_copy: false,
        chunk_size: 0,
        last_mb_y: -1,
        alpha_data: None,
        alpha_data_size: 0,
        hooks: None,
    };

    init_mem_buffer(&mut idec.mem);
    if !webp_init_dec_buffer(&mut idec.output) || !vp8_init_io(&mut idec.io) {
        return None;
    }

    webp_reset_dec_params(&mut idec.params);
    match output_buffer {
        Some(output_buffer) if !webp_avoid_slow_memory(output_buffer, features) => {
            // C: idec->params.output = output_buffer; final_output = NULL.
            idec.user_output = Some(output_buffer);
        }
        output_buffer => {
            // C: idec->params.output = &idec->output.
            if let Some(output_buffer) = &output_buffer {
                idec.output.colorspace = output_buffer.colorspace;
            }
            idec.slow_copy = output_buffer.is_some();
            idec.final_output = output_buffer;
        }
    }
    // C: WebPInitCustomIo(&idec->params, &idec->io); the hooks are made for each
    // call (`io_hooks`).

    Some(idec)
}

//------------------------------------------------------------------------------
// Public functions

// upstream: src/dec/idec_dec.c WebPINewDecoder
/// Creates a new incremental decoder with the supplied buffer parameter. This
/// `output_buffer` can be passed `None`, in which case a default output buffer
/// is used (with `MODE_RGB`). Otherwise, an internal reference to
/// `output_buffer` is kept, which means that the lifetime of `output_buffer`
/// must be larger than that of the returned decoder. The decoder is a value;
/// dropping it is `WebPIDelete()`.
#[must_use]
pub fn webp_i_new_decoder<'a, 'b>(
    output_buffer: Option<&'a mut WebPDecBuffer<'b>>,
) -> Option<WebPIDecoder<'a, 'b>> {
    new_decoder(output_buffer, None)
}

// upstream: src/dec/idec_dec.c WebPIDecode
/// Instantiates a new incremental decoder object with the requested
/// configuration. The bitstream can be passed in `data` to retrieve the
/// features into `config.input` (or into a temporary when `config` is `None`);
/// `data` is not consumed, only parsed. Returns `None` in case of error, for
/// instance when the features cannot be retrieved from a non-empty `data`. The
/// options and the output buffer of `config` stay borrowed by the decoder. C
/// tests `data != NULL && data_size > 0`; a `NULL` `data` is an empty slice
/// here.
#[must_use]
pub fn webp_i_decode<'a, 'b>(
    data: &[u8],
    config: Option<&'a mut WebPDecoderConfig<'b>>,
) -> Option<WebPIDecoder<'a, 'b>> {
    let mut tmp_features = WebPBitstreamFeatures::default();
    let (features, output, options) = match config {
        Some(config) => {
            let WebPDecoderConfig {
                input,
                output,
                options,
            } = config;
            (input, Some(output), Some(&*options))
        }
        None => (&mut tmp_features, None, None),
    };

    // Parse the bitstream's features, if requested:
    if !data.is_empty() && webp_get_features(data, features).is_err() {
        return None;
    }

    // Create an instance of the incremental decoder
    let mut idec = new_decoder(output, Some(features))?;
    // Finish initialization
    if options.is_some() {
        idec.params.options = options;
    }
    Some(idec)
}

// upstream: src/dec/idec_dec.c WebPIDelete
/// Deletes the decoder: dropping it does what `WebPIDelete()` does (see
/// [`Drop`] for [`WebPIDecoder`]).
pub fn webp_i_delete(idec: WebPIDecoder<'_, '_>) {
    drop(idec);
}

impl Drop for WebPIDecoder<'_, '_> {
    // upstream: src/dec/idec_dec.c WebPIDelete
    /// Synchronizes the thread and tears the io down for a lossy decoding in
    /// progress, then releases the decoder, the memory buffer and the
    /// decoder's own output buffer. No row is pending at this point, because
    /// every call ends with the worker synchronized; in map mode the caller's
    /// buffer is gone, so the alpha chunk is not passed.
    fn drop(&mut self) {
        if let Some(dec) = &mut self.dec {
            match dec {
                IDecDecoder::VP8(dec) => {
                    if self.state == DecState::STATE_VP8_DATA {
                        debug_assert!(!dec.put_pending);
                        let alpha = if self.mem.mode == MemBufferMode::MEM_MODE_APPEND {
                            alpha_slice(&self.mem.buf, self.alpha_data, self.alpha_data_size)
                        } else {
                            None
                        };
                        let mut hooks = io_hooks(
                            &mut self.hooks,
                            &mut self.params,
                            &mut self.output,
                            &mut self.user_output,
                            &mut self.io,
                        );
                        // Synchronize the thread, clean-up and check for errors.
                        // C does not use the result either.
                        let _ = vp8_exit_critical(dec, &mut self.io, alpha, Some(hooks.get()));
                    }
                    // C: VP8Delete((VP8Decoder*)idec->dec).
                }
                IDecDecoder::VP8L(_) => {
                    // C: VP8LDelete((VP8LDecoder*)idec->dec).
                }
            }
        }
        self.dec = None;
        clear_mem_buffer(&mut self.mem);
        webp_free_dec_buffer(&mut self.output);
    }
}

//------------------------------------------------------------------------------
// Wrapper toward WebPINewDecoder

// upstream: src/dec/idec_dec.c WebPINewRGB
/// Creates a decoder that outputs RGB samples in `csp`: into the caller's
/// `output_buffer` with `output_stride` (C: the pointer and its size), or, for
/// `None`, into memory that the decoder allocates, in which case the stride is
/// ignored. Returns `None` for a YUV colorspace, or for a zero stride or an
/// empty buffer.
#[must_use]
pub fn webp_i_new_rgb<'a>(
    csp: WEBP_CSP_MODE,
    output_buffer: Option<&mut [u8]>,
    output_stride: i32,
) -> Option<WebPIDecoder<'a, '_>> {
    let is_external_memory = output_buffer.is_some();
    let mut output_buffer_size = output_buffer.as_ref().map_or(0, |buffer| buffer.len());
    let mut output_stride = output_stride;

    if csp >= WEBP_CSP_MODE::MODE_YUV {
        return None;
    }
    if is_external_memory {
        // A buffer was passed. Validate the other params.
        if output_stride == 0 || output_buffer_size == 0 {
            return None; // invalid parameter.
        }
    } else {
        // Overwrite parameters to sane values.
        output_buffer_size = 0;
        output_stride = 0;
    }
    let mut idec = webp_i_new_decoder(None)?;
    idec.output.colorspace = csp;
    // C: idec->output.is_external_memory = is_external_memory.
    idec.output.storage = match output_buffer {
        Some(buffer) => PlaneStorage::External(vec![buffer]),
        None => PlaneStorage::Internal(Vec::new()),
    };
    idec.output.u = WebPDecBufferPlanes::RGBA(WebPRGBABuffer {
        rgba: PlaneOffset::default(),
        stride: output_stride,
        size: output_buffer_size,
    });
    Some(idec)
}

// upstream: src/dec/idec_dec.c WebPINewYUVA
/// Creates a decoder that outputs YUV samples, with alpha when `a` is given:
/// into the caller's `luma`, `u`, `v` and `a` with their strides (C: each
/// pointer with its size), or, for a `None` luma, into memory that the decoder
/// allocates (`MODE_YUVA`), in which case the other arguments are ignored.
/// Returns `None` when a luma buffer is passed without U or V, or with an empty
/// buffer or a zero stride.
#[expect(
    clippy::too_many_arguments,
    reason = "the four planes and strides of the C function"
)]
#[must_use]
pub fn webp_i_new_yuva<'a, 'b>(
    luma: Option<&'b mut [u8]>,
    luma_stride: i32,
    u: Option<&'b mut [u8]>,
    u_stride: i32,
    v: Option<&'b mut [u8]>,
    v_stride: i32,
    a: Option<&'b mut [u8]>,
    a_stride: i32,
) -> Option<WebPIDecoder<'a, 'b>> {
    let size = |plane: &Option<&'b mut [u8]>| plane.as_ref().map_or(0, |plane| plane.len());
    let (mut luma_size, mut u_size, mut v_size, mut a_size) =
        (size(&luma), size(&u), size(&v), size(&a));
    let (mut luma_stride, mut u_stride, mut v_stride, mut a_stride) =
        (luma_stride, u_stride, v_stride, a_stride);
    let colorspace;
    let storage;

    if let Some(luma) = luma {
        // A luma buffer was passed. Validate the other parameters.
        let (Some(u), Some(v)) = (u, v) else {
            return None;
        };
        if luma_size == 0 || u_size == 0 || v_size == 0 {
            return None;
        }
        if luma_stride == 0 || u_stride == 0 || v_stride == 0 {
            return None;
        }
        if a.is_some() && (a_size == 0 || a_stride == 0) {
            return None;
        }
        colorspace = if a.is_none() {
            WEBP_CSP_MODE::MODE_YUV
        } else {
            WEBP_CSP_MODE::MODE_YUVA
        };
        // C: is_external_memory = 1.
        let mut slices = vec![luma, u, v];
        slices.extend(a);
        storage = PlaneStorage::External(slices);
    } else {
        // Overwrite parameters to sane values.
        luma_size = 0;
        u_size = 0;
        v_size = 0;
        a_size = 0;
        luma_stride = 0;
        u_stride = 0;
        v_stride = 0;
        a_stride = 0;
        colorspace = WEBP_CSP_MODE::MODE_YUVA;
        storage = PlaneStorage::Internal(Vec::new());
    }
    let has_a = matches!(&storage, PlaneStorage::External(slices) if slices.len() == 4);

    let mut idec = webp_i_new_decoder(None)?;

    idec.output.colorspace = colorspace;
    idec.output.storage = storage;
    let plane = |region| PlaneOffset { region, offset: 0 };
    idec.output.u = WebPDecBufferPlanes::YUVA(WebPYUVABuffer {
        y: plane(0),
        y_stride: luma_stride,
        y_size: luma_size,
        u: plane(1),
        u_stride,
        u_size,
        v: plane(2),
        v_stride,
        v_size,
        a: has_a.then(|| plane(3)),
        a_stride,
        a_size,
    });
    Some(idec)
}

// upstream: src/dec/idec_dec.c WebPINewYUV
/// Deprecated version of [`webp_i_new_yuva`], without the alpha plane.
#[must_use]
pub fn webp_i_new_yuv<'a, 'b>(
    luma: Option<&'b mut [u8]>,
    luma_stride: i32,
    u: Option<&'b mut [u8]>,
    u_stride: i32,
    v: Option<&'b mut [u8]>,
    v_stride: i32,
) -> Option<WebPIDecoder<'a, 'b>> {
    webp_i_new_yuva(luma, luma_stride, u, u_stride, v, v_stride, None, 0)
}

//------------------------------------------------------------------------------

// upstream: src/dec/idec_dec.c IDecCheckStatus
fn i_dec_check_status(idec: &WebPIDecoder<'_, '_>) -> Result<(), VP8StatusCode> {
    if idec.state == DecState::STATE_ERROR {
        return Err(VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR);
    }
    if idec.state == DecState::STATE_DONE {
        return Ok(());
    }
    Err(VP8StatusCode::VP8_STATUS_SUSPENDED)
}

// upstream: src/dec/idec_dec.c WebPIAppend
/// Copies and decodes the next available data.
///
/// # Errors
///
/// `VP8_STATUS_SUSPENDED` when more data is expected, the decoding error
/// otherwise; `Ok` when the image is successfully decoded.
/// `VP8_STATUS_INVALID_PARAM` after calls of `WebPIUpdate`, and
/// `VP8_STATUS_OUT_OF_MEMORY` when the data cannot be stored. C returns
/// `VP8_STATUS_INVALID_PARAM` for a `NULL` decoder or data, which exist only at
/// the C ABI.
pub fn webp_i_append(idec: &mut WebPIDecoder<'_, '_>, data: &[u8]) -> Result<(), VP8StatusCode> {
    let status = i_dec_check_status(idec);
    if status != Err(VP8StatusCode::VP8_STATUS_SUSPENDED) {
        return status;
    }
    // Check mixed calls between RemapMemBuffer and AppendToMemBuffer.
    if !check_mem_buffer_mode(&mut idec.mem, MemBufferMode::MEM_MODE_APPEND) {
        return Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM);
    }
    // Append data to memory buffer
    if !append_to_mem_buffer(idec, data) {
        return Err(VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY);
    }
    // The memory buffer is the input of the decoding step.
    let input = std::mem::take(&mut idec.mem.buf);
    let status = i_decode(idec, &input);
    idec.mem.buf = input;
    status
}

// upstream: src/dec/idec_dec.c WebPIUpdate
/// A variant of [`webp_i_append`] for a `data` buffer that contains partial
/// data from the beginning. The data is not copied; the caller passes the
/// whole buffer so far to each call, which may move between calls.
///
/// # Errors
///
/// As [`webp_i_append`]; `VP8_STATUS_INVALID_PARAM` also for a buffer shorter
/// than the previous one.
pub fn webp_i_update(idec: &mut WebPIDecoder<'_, '_>, data: &[u8]) -> Result<(), VP8StatusCode> {
    let status = i_dec_check_status(idec);
    if status != Err(VP8StatusCode::VP8_STATUS_SUSPENDED) {
        return status;
    }
    // Check mixed calls between RemapMemBuffer and AppendToMemBuffer.
    if !check_mem_buffer_mode(&mut idec.mem, MemBufferMode::MEM_MODE_MAP) {
        return Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM);
    }
    // Make the memory buffer point to the new buffer
    if !remap_mem_buffer(idec, data) {
        return Err(VP8StatusCode::VP8_STATUS_INVALID_PARAM);
    }
    i_decode(idec, data)
}

//------------------------------------------------------------------------------

// upstream: src/dec/idec_dec.c GetOutputBuffer
/// C returns `NULL` for a `NULL` decoder, which exists only at the C ABI.
fn get_output_buffer<'s, 'b>(idec: &'s WebPIDecoder<'_, 'b>) -> Option<&'s WebPDecBuffer<'b>> {
    idec.dec.as_ref()?;
    if idec.state <= DecState::STATE_VP8_PARTS0 {
        return None;
    }
    if idec.final_output.is_some() {
        return None; // not yet slow-copied
    }
    Some(match &idec.user_output {
        Some(user_output) => user_output,
        None => &idec.output,
    })
}

/// The region and offset of a plane, or `None` where C's pointer is `NULL`
/// (`src/dec/buffer_dec.rs` describes when it is).
fn plane_rows<'s>(
    storage: &'s PlaneStorage<'_, u8>,
    plane: PlaneOffset,
) -> Option<(&'s [u8], usize)> {
    if is_null_plane(storage, plane) {
        return None;
    }
    storage
        .region(plane.region)
        .map(|region| (region, plane.offset))
}

// upstream: src/dec/idec_dec.c WebPIDecodedArea
/// Generic call to retrieve information about the displayable area. If
/// non-`None`, `left`, `top`, `width` and `height` receive the visible
/// rectangular area so far (`left` and `top` are always 0). Returns `None` in
/// case the incremental decoder object is in an invalid state, otherwise the
/// output buffer, which is still owned by the decoder and lives as long as it.
#[must_use]
pub fn webp_i_decoded_area<'s, 'b>(
    idec: &'s WebPIDecoder<'_, 'b>,
    left: Option<&mut i32>,
    top: Option<&mut i32>,
    width: Option<&mut i32>,
    height: Option<&mut i32>,
) -> Option<&'s WebPDecBuffer<'b>> {
    let src = get_output_buffer(idec);
    if let Some(left) = left {
        *left = 0;
    }
    if let Some(top) = top {
        *top = 0;
    }
    if let Some(src) = src {
        if let Some(width) = width {
            *width = src.width;
        }
        if let Some(height) = height {
            *height = idec.params.last_y;
        }
    } else {
        if let Some(width) = width {
            *width = 0;
        }
        if let Some(height) = height {
            *height = 0;
        }
    }
    src
}

// upstream: src/dec/idec_dec.c WebPIDecGetRGB
/// Returns the RGB/A image decoded so far: the region that holds the samples
/// and the position of the first row in it, where C returns the pointer (the
/// rows follow at `stride`, which is negative for a flipped output). Returns
/// `None` if the output is not RGB or not allocated yet. `last_y` receives the
/// index of the row after the last decoded one; `width`, `height` and `stride`
/// describe the output.
#[must_use]
pub fn webp_i_dec_get_rgb<'s>(
    idec: &'s WebPIDecoder<'_, '_>,
    last_y: Option<&mut i32>,
    width: Option<&mut i32>,
    height: Option<&mut i32>,
    stride: Option<&mut i32>,
) -> Option<(&'s [u8], usize)> {
    let src = get_output_buffer(idec)?;
    if src.colorspace >= WEBP_CSP_MODE::MODE_YUV {
        return None;
    }
    // The RGBA member, which the colorspace selects. A buffer that was not
    // allocated yet may hold the other member, zeroed, which C reads as a
    // zeroed RGBA member (see `src/dec/buffer_dec.rs`).
    let (buf, rows) = match &src.u {
        WebPDecBufferPlanes::RGBA(buf) => (*buf, plane_rows(&src.storage, buf.rgba)),
        WebPDecBufferPlanes::YUVA(_) => (WebPRGBABuffer::default(), None),
    };

    if let Some(last_y) = last_y {
        *last_y = idec.params.last_y;
    }
    if let Some(width) = width {
        *width = src.width;
    }
    if let Some(height) = height {
        *height = src.height;
    }
    if let Some(stride) = stride {
        *stride = buf.stride;
    }

    rows
}

/// The rows of one plane, as [`webp_i_dec_get_yuva`] returns them: the region
/// and the position of the first row in it, or `None` for C's `NULL`.
pub type PlaneRows<'s> = Option<(&'s [u8], usize)>;

// upstream: src/dec/idec_dec.c WebPIDecGetYUVA
/// Same as [`webp_i_dec_get_rgb`] but to retrieve the Y, U, V and alpha planes:
/// returns the luma plane and writes the U, V and alpha planes (`None` for no
/// alpha plane), with `stride` for luma, `uv_stride` for chroma and `a_stride`
/// for alpha. Returns `None` if the output is not YUV or not allocated yet.
#[expect(
    clippy::too_many_arguments,
    reason = "the output parameters of the C function"
)]
#[must_use]
pub fn webp_i_dec_get_yuva<'s>(
    idec: &'s WebPIDecoder<'_, '_>,
    last_y: Option<&mut i32>,
    u: Option<&mut PlaneRows<'s>>,
    v: Option<&mut PlaneRows<'s>>,
    a: Option<&mut PlaneRows<'s>>,
    width: Option<&mut i32>,
    height: Option<&mut i32>,
    stride: Option<&mut i32>,
    uv_stride: Option<&mut i32>,
    a_stride: Option<&mut i32>,
) -> PlaneRows<'s> {
    let src = get_output_buffer(idec)?;
    if src.colorspace < WEBP_CSP_MODE::MODE_YUV {
        return None;
    }
    // The YUVA member, which the colorspace selects. A buffer that was not
    // allocated yet may hold the other member, zeroed, which C reads as a
    // zeroed YUVA member with no planes (see `src/dec/buffer_dec.rs`).
    let (buf, planes) = match &src.u {
        WebPDecBufferPlanes::YUVA(buf) => (*buf, true),
        WebPDecBufferPlanes::RGBA(_) => (WebPYUVABuffer::default(), false),
    };
    let rows = |plane: Option<PlaneOffset>| {
        plane
            .filter(|_| planes)
            .and_then(|plane| plane_rows(&src.storage, plane))
    };

    if let Some(last_y) = last_y {
        *last_y = idec.params.last_y;
    }
    if let Some(u) = u {
        *u = rows(Some(buf.u));
    }
    if let Some(v) = v {
        *v = rows(Some(buf.v));
    }
    if let Some(a) = a {
        *a = rows(buf.a);
    }
    if let Some(width) = width {
        *width = src.width;
    }
    if let Some(height) = height {
        *height = src.height;
    }
    if let Some(stride) = stride {
        *stride = buf.y_stride;
    }
    if let Some(uv_stride) = uv_stride {
        *uv_stride = buf.u_stride;
    }
    if let Some(a_stride) = a_stride {
        *a_stride = buf.a_stride;
    }

    rows(Some(buf.y))
}

// upstream: src/dec/idec_dec.c WebPISetIOHooks
/// Sets the custom IO functions and their user data (C: `put`, `setup`,
/// `teardown` and `user_data`, ADR 0003). Returns false when the decoding has
/// gone past the headers. In C, a lossless image then makes `VP8LDecodeImage`
/// read the user data as `WebPDecParams`, which is undefined; here it uses the
/// decoder's parameters (`# Excluded undefined behavior`, item 2, in the module
/// documentation). C returns 0 for a `NULL` decoder, which exists only at the C
/// ABI.
#[must_use]
pub fn webp_i_set_io_hooks<'a>(
    idec: &mut WebPIDecoder<'a, '_>,
    hooks: Box<dyn VP8IoHooks + 'a>,
) -> bool {
    if idec.state > DecState::STATE_WEBP_HEADER {
        return false;
    }

    idec.hooks = Some(hooks);

    true
}

//------------------------------------------------------------------------------
// Access for the C ABI (no C counterpart). The C ABI (`src/abi/decode.rs`)
// lends caller memory to the output buffers for the duration of each call and
// reads their planes after each call.

impl<'b> WebPIDecoder<'_, 'b> {
    /// Whether C's `params.output` is the caller's buffer.
    pub(crate) fn has_user_output(&self) -> bool {
        self.user_output.is_some()
    }

    /// The decoder's own output buffer (C: `idec->output`).
    pub(crate) fn own_output_mut(&mut self) -> &mut WebPDecBuffer<'b> {
        &mut self.output
    }

    /// Whether the decoder copies into the caller's buffer at the end (C: a
    /// `final_output` was set by `NewDecoder`, slow memory).
    pub(crate) fn was_slow(&self) -> bool {
        self.slow_copy
    }

    /// The caller's output buffer: C's `params.output` or `final_output`.
    pub(crate) fn caller_output_mut(&mut self) -> Option<&mut WebPDecBuffer<'b>> {
        match (&mut self.user_output, &mut self.final_output) {
            (Some(output), _) | (None, Some(output)) => Some(&mut **output),
            (None, None) => None,
        }
    }
}

//------------------------------------------------------------------------------
// Access for the safe API (no C counterpart). `src/api/decode.rs` (WP16)
// creates a decoder over its own output buffer and sets the options as
// `WebPIDecode` does, so that it borrows the options without the output buffer
// of a `WebPDecoderConfig`.

impl<'a> WebPIDecoder<'a, '_> {
    /// Sets the decoding options, as `WebPIDecode` does with
    /// `idec->params.options = &config->options`, for a decoder that
    /// `webp_i_new_decoder`, `webp_i_new_rgb` or `webp_i_new_yuva` created. Call
    /// it before the first `webp_i_append` or `webp_i_update`, as C sets the
    /// options before the decoder sees any data.
    pub(crate) fn set_options(&mut self, options: &'a WebPDecoderOptions) {
        self.params.options = Some(options);
    }
}
