// libwebp v1.6.0, src/dec/vp8_dec.c:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// main entry for the decoder
//
// Author: Skal (pascal.massimino@gmail.com)

// libwebp v1.6.0, src/dec/vp8_dec.h:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
//  Low-level API for VP8 decoder
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dec/vp8_dec.c` and `src/dec/vp8_dec.h` from libwebp v1.6.0.
//!
//! Work package: WP6 (VP8 decoder core).
//!
//! Exception: WP7 translates `VP8Io`, `VP8IoPutHook`, `VP8IoSetupHook` and `VP8IoTeardownHook` from `src/dec/vp8_dec.h`, because `VP8LDecoder` and `ALPHDecoder` hold a `VP8Io`.
//!
//! # Representation
//!
//! 1. **Alpha input.** `dec->alpha_data` points into the caller's input outside
//!    the VP8 payload, so it is a per-call argument `alpha_data: Option<&[u8]>`
//!    (C's `alpha_data[..alpha_data_size]`, `None` for `NULL`) of `vp8_decode`,
//!    `vp8_init_frame`, `vp8_process_row`, `vp8_exit_critical` and
//!    `vp8_decompress_alpha_rows`. The decoder stores neither field.
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Pointers into blocks and planes (ADR 0003).** A pointer that the C code
//!    only moves forward is a slice from it; one from which it also reads
//!    backwards (the predictors' `dst[-1]` and `dst[-BPS]`, the filters'
//!    `p[-2 * step]`) is a slice and a position. Positions outside the slice
//!    panic, where C reads or writes outside the buffer (item 2 of
//!    `# Deviations from upstream` in `src/dsp/dsp.rs`). The bit readers of the
//!    first partition and of the token partitions hold positions in `data`, the
//!    VP8 payload: `VP8InitBitReader(br, buf + start, size)` is the init of
//!    `src/utils/bit_reader_utils.rs` on the bytes followed by
//!    `VP8RemapBitReader` by `start`.
//! 2. **`GetCoeffs`** is a `LazyLock` of the function that `InitGetCoeffs`
//!    selects from the CPU query (C: the `volatile` pointer that `VP8New`
//!    sets). The coefficient probabilities are the bands of one type and the
//!    indexes of `bands_ptr` into them, where C passes an array of pointers;
//!    `bands_ptr` starts at 0 where C has `NULL`, and nothing reads it before
//!    `VP8ParseProba` sets it (the probes print it only when the decoder is
//!    ready).
//! 3. **Per-call context (ADR 0003).** `VP8GetHeaders`, `VP8Decode`,
//!    `VP8ParseProba`, `VP8ParseIntraModeRow`, `VP8ParseQuant` and
//!    `VP8DecodeMB` take `data`, C's `io->data[..io->data_size]`; the functions
//!    that emit rows take `alpha_data` and `Option<&mut (dyn VP8IoHooks + '_)>`
//!    (item 1 of `# Representation`). C's `VP8ParseProba(br, dec)` and
//!    `VP8ParseIntraModeRow(br, dec)` receive `&dec->br` from every caller and
//!    read `dec.br` here; `VP8DecodeMB` takes the index of the token partition,
//!    which every caller computes as `mb_y & num_parts_minus_one`.
//! 4. **Errors (ADR 0003).** `VP8SetError` keeps its precedence and returns the
//!    status that the decoder holds afterwards. `VP8GetHeaders` and
//!    `VP8EnterCritical` return `Result<(), VP8StatusCode>` with the recorded
//!    status. `VP8Decode`, `VP8InitFrame`, `VP8ProcessRow`, `VP8ExitCritical`,
//!    `ParseFrame` and `VP8DecodeMB` return `bool`, because C can fail there
//!    without recording a status (a failed `put()` of the last threaded row, a
//!    size test that does not fit), so the status field is the only record, as
//!    in C. `VP8GetInfo` returns `Option<(width, height)>`, since C writes them
//!    only on success. `VP8DecompressAlphaRows` returns `Option<usize>`, the
//!    position of the row in the alpha plane, `None` where C returns `NULL`.
//!    `error_msg` is `Option<&'static str>`.
//! 5. **Hooks.** C tests `io->put != NULL` and the others; the Rust calls test
//!    `Option` and `has_put`, `has_setup` and `has_teardown` (the `VP8IoHooks`
//!    trait). The trait objects take their own lifetime
//!    (`dyn VP8IoHooks + '_`), so that the calls can reborrow them.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. **Parse failures after a successful `VP8Decode` without new headers**,
//!    and calls of `VP8ProcessRow` and `VP8ExitCritical` outside the pairing
//!    that `VP8EnterCritical` documents.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **`GetCoeffsAlt`** is selected only on x86 CPUs with slow SSSE3; the
//!    development host and the CI hosts run `GetCoeffsFast`. Both decode the
//!    same values.

// The items below are WP7's exception in this module (`tools/units.json`):
// `VP8Io` and its three hook types. ADR 0003 ("Decoding context per call")
// fixes their form: `VP8Io` keeps its scalar fields, and the input, the sample
// rows, `opaque` and the hooks are passed to each call.

// upstream: src/dec/vp8_dec.h VP8Io
/// Input and output parameters of a decoding call (C: `VP8Io`).
///
/// The C pointers are not fields (ADR 0003, "Decoding context per call"): the
/// input `data` and `data_size` is a `data: &[u8]` argument of every function that
/// reads it, the rows `y`, `u`, `v` and `a` reach the put hook as [`VP8IoRows`],
/// and `opaque` with `put`, `setup` and `teardown` is the [`VP8IoHooks`] argument
/// of the functions that call hooks. The `int` flags that C uses only as
/// predicates are `bool`. `Default` is the zeroed structure of `VP8InitIo`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "the int flags of the C structure, which it uses as predicates"
)]
pub struct VP8Io {
    /// Picture width in pixels, set by the header parsers (invariable). These are
    /// the original, uncropped dimensions; the area passed to `put()` is
    /// `mb_w` by `mb_h`.
    pub width: i32,
    /// Picture height in pixels, set by the header parsers (invariable).
    pub height: i32,
    /// Position of the current rows, in pixels; set before calling `put()`.
    pub mb_y: i32,
    /// Number of columns in the sample.
    pub mb_w: i32,
    /// Number of rows in the sample.
    pub mb_h: i32,
    /// Row stride for luma.
    pub y_stride: i32,
    /// Row stride for chroma.
    pub uv_stride: i32,
    /// Recommendation for the user-side YUV to RGB converter. Set when calling
    /// the setup hook, which can overwrite it; `put()` can take it into account.
    pub fancy_upsampling: bool,
    /// If true, in-loop filtering is not performed even if present in the
    /// bitstream. The output is then not compliant with the VP8 specification.
    pub bypass_filtering: bool,
    /// Whether cropping applies.
    pub use_cropping: bool,
    /// Left edge of the crop window.
    pub crop_left: i32,
    /// Right edge (exclusive) of the crop window.
    pub crop_right: i32,
    /// Top edge of the crop window.
    pub crop_top: i32,
    /// Bottom edge (exclusive) of the crop window.
    pub crop_bottom: i32,
    /// Whether scaling applies.
    pub use_scaling: bool,
    /// Width after scaling.
    pub scaled_width: i32,
    /// Height after scaling.
    pub scaled_height: i32,
}

/// The rows that C passes to `put()` through `io->y`, `io->u`, `io->v` and
/// `io->a` (ADR 0003, "Decoding context per call"). The luma and chroma slices
/// start where the C pointers point, and the rows follow at `y_stride`,
/// `uv_stride` and `width`. `y` is mutable because `EmitRescaledYUV`
/// premultiplies the luma through the `const` pointer. The alpha data is
/// persistent, and `EmitAlphaRGB` steps back one row from `io->a`, so `a` holds
/// the alpha plane from its first row and `a_offset` is the index of C's `io->a`
/// in it (WP6). `a` is `None` where C has `io->a == NULL`. This type has no C
/// counterpart.
#[derive(Debug)]
pub struct VP8IoRows<'r> {
    /// Luma rows (C: `io->y`).
    pub y: &'r mut [u8],
    /// Chroma U rows (C: `io->u`).
    pub u: &'r [u8],
    /// Chroma V rows (C: `io->v`).
    pub v: &'r [u8],
    /// The alpha plane, from its first row; `None` where C has `io->a == NULL`.
    pub a: Option<&'r [u8]>,
    /// Index in `a` of C's `io->a`: the alpha rows, pre-offset by `mb_y` and the
    /// crop.
    pub a_offset: usize,
}

/// The hooks of a `VP8Io` and their user data (C: `put`, `setup`, `teardown` and
/// `opaque`), passed per call as `Option<&mut dyn VP8IoHooks>` (ADR 0003,
/// "Decoding context per call"). The implementing value is the C `opaque`. The
/// `has_*` methods answer C's test of each hook against `NULL`; a caller tests
/// them before it calls the hook, as C does.
pub trait VP8IoHooks {
    // upstream: src/dec/vp8_dec.h VP8IoSetupHook
    /// Called just before starting to decode the blocks. Must return false in
    /// case of setup error, true otherwise. If false is returned, `teardown()` is
    /// not called; if true is returned, `teardown()` is always called afterward.
    fn setup(&mut self, io: &mut VP8Io) -> bool;

    // upstream: src/dec/vp8_dec.h VP8IoPutHook
    /// Called when fresh samples are available. The samples are in YUV420 format
    /// and can be up to `width` by 24 in size. Returns false in case of error or
    /// abort request. The area to update is `mb_w` by `mb_h`, taking cropping into
    /// account.
    fn put(&mut self, io: &VP8Io, rows: &mut VP8IoRows<'_>) -> bool;

    // upstream: src/dec/vp8_dec.h VP8IoTeardownHook
    /// Called just after block decoding is finished, or when an error occurred
    /// during `put()`. Not called if `setup()` failed.
    fn teardown(&mut self, io: &VP8Io);

    /// Whether the setup hook is set (C: `io->setup != NULL`).
    fn has_setup(&self) -> bool {
        true
    }

    /// Whether the put hook is set (C: `io->put != NULL`).
    fn has_put(&self) -> bool {
        true
    }

    /// Whether the teardown hook is set (C: `io->teardown != NULL`).
    fn has_teardown(&self) -> bool {
        true
    }
}

// The items below translate the rest of `src/dec/vp8_dec.h` and
// `src/dec/vp8_dec.c` (WP6).

use std::sync::LazyLock;

use crate::dec::alpha_dec::webp_deallocate_alpha_memory;
use crate::dec::common_dec::{
    B_DC_PRED, MB_FEATURE_TREE_PROBS, NUM_BANDS, NUM_MB_SEGMENTS, NUM_MODE_LF_DELTAS,
    NUM_REF_LF_DELTAS,
};
use crate::dec::frame_dec::{
    FINISH_ROW_HOOK, row_job, vp8_enter_critical, vp8_exit_critical, vp8_init_frame,
    vp8_process_row, vp8_sync_rows,
};
use crate::dec::quant_dec::vp8_parse_quant;
use crate::dec::tree_dec::{vp8_parse_intra_mode_row, vp8_parse_proba, vp8_reset_proba};
use crate::dec::vp8i_dec::{
    DEC_MAJ_VERSION, DEC_MIN_VERSION, DEC_REV_VERSION, VP8BandProbas, VP8Decoder, VP8RowJob,
    VP8SegmentHeader, quant_t,
};
use crate::dsp::cpu::{CPUFeature, vp8_get_cpu_info};
use crate::dsp::dec::vp8_transform_wht;
use crate::utils::bit_reader_inl_utils::{vp8_get_bit, vp8_get_bit_alt, vp8_get_signed};
use crate::utils::bit_reader_utils::{
    VP8BitReader, vp8_get, vp8_get_signed_value, vp8_get_value, vp8_init_bit_reader,
    vp8_remap_bit_reader,
};
use crate::webp::decode::{VP8StatusCode, WEBP_DECODER_ABI_VERSION};
use crate::webp::format_constants::VP8_FRAME_HEADER_SIZE;
use crate::webp::types::webp_abi_is_incompatible;

//------------------------------------------------------------------------------

// upstream: src/dec/vp8_dec.c WebPGetDecoderVersion
/// Returns the version number of the decoder, packed in hexadecimal using 8bits
/// for each of major/minor/revision. E.g: v2.5.7 is 0x020507.
#[must_use]
pub fn webp_get_decoder_version() -> i32 {
    (DEC_MAJ_VERSION << 16) | (DEC_MIN_VERSION << 8) | DEC_REV_VERSION
}

//------------------------------------------------------------------------------
// Signature and pointer-to-function for GetCoeffs() variants below.

// upstream: src/dec/vp8_dec.c GetCoeffsFunc
/// Parses the coefficients of one block (C: `GetCoeffsFunc`). C's
/// `const VP8BandProbas* const prob[]` is the bands of one type and the indexes
/// of `bands_ptr` into them (`VP8Proba`).
type GetCoeffsFunc = fn(
    &mut VP8BitReader,
    &[u8],
    &[VP8BandProbas; NUM_BANDS as usize],
    &[usize; 16 + 1],
    i32,
    quant_t,
    i32,
    &mut [i16],
) -> i32;

// upstream: src/dec/vp8_dec.c GetCoeffs
/// The variant that `InitGetCoeffs` selects, once (C: the `volatile` pointer
/// that `VP8New` sets through `InitGetCoeffs`).
static GET_COEFFS: LazyLock<GetCoeffsFunc> = LazyLock::new(init_get_coeffs);

//------------------------------------------------------------------------------
// VP8Decoder

// upstream: src/dec/vp8_dec.c SetOk
fn set_ok(dec: &mut VP8Decoder) {
    dec.status = VP8StatusCode::VP8_STATUS_OK;
    dec.error_msg = Some("OK");
}

// upstream: src/dec/vp8_dec.c VP8InitIoInternal
/// Resets `io` to zero unless the ABI versions are incompatible; returns false
/// in case of version mismatch. C also accepts a `NULL` io, which exists only at
/// the C ABI.
pub fn vp8_init_io_internal(io: &mut VP8Io, version: i32) -> bool {
    if webp_abi_is_incompatible(version, WEBP_DECODER_ABI_VERSION) {
        return false; // mismatch error
    }
    *io = VP8Io::default();
    true
}

// upstream: src/dec/vp8_dec.h VP8InitIo
/// Must be called to make sure `io` is initialized properly. Returns false in
/// case of version mismatch. Upon such failure, no other decoding function
/// should be called (`VP8Decode`, `VP8GetHeaders`, ...).
#[must_use]
pub fn vp8_init_io(io: &mut VP8Io) -> bool {
    vp8_init_io_internal(io, WEBP_DECODER_ABI_VERSION)
}

// upstream: src/dec/vp8_dec.c VP8New
/// Creates a new decoder object. C returns `NULL` when the fixed-size
/// allocation fails; here a refused host allocation aborts, and the size test of
/// `WebPSafeCalloc` always passes for one object.
#[must_use]
pub fn vp8_new() -> Box<VP8Decoder> {
    let mut dec = Box::<VP8Decoder>::default();
    set_ok(&mut dec);
    // C: WebPGetWorkerInterface()->Init(&dec->worker); `Default` is that state.
    // The worker is also the home of the row job (`frame_dec.rs`), whose hook
    // C sets in InitThreadContext.
    dec.worker
        .set_job(FINISH_ROW_HOOK, VP8RowJob::default(), ());
    dec.ready = false;
    dec.num_parts_minus_one = 0;
    // C: InitGetCoeffs(); the selection is made at the first use of `GET_COEFFS`.
    dec
}

// upstream: src/dec/vp8_dec.c VP8Status
/// Returns the current status of the decoder. C answers
/// `VP8_STATUS_INVALID_PARAM` for a `NULL` decoder, which exists only at the C
/// ABI.
pub fn vp8_status(dec: &VP8Decoder) -> VP8StatusCode {
    dec.status
}

// upstream: src/dec/vp8_dec.c VP8StatusMessage
/// Returns a readable string corresponding to the last status. C answers
/// "no object" for a `NULL` decoder, which exists only at the C ABI.
#[must_use]
pub fn vp8_status_message(dec: &VP8Decoder) -> &'static str {
    dec.error_msg.unwrap_or("OK")
}

// upstream: src/dec/vp8_dec.c VP8SetError
/// Records `error` and `msg` unless an error is already recorded: the oldest
/// error reported takes precedence over the new one. Returns the status that
/// the decoder holds afterwards (C returns 0, ADR 0003).
pub fn vp8_set_error(
    dec: &mut VP8Decoder,
    error: VP8StatusCode,
    msg: &'static str,
) -> VP8StatusCode {
    // VP8_STATUS_SUSPENDED is only meaningful in incremental decoding.
    debug_assert!(dec.incremental || error != VP8StatusCode::VP8_STATUS_SUSPENDED);
    // The oldest error reported takes precedence over the new one.
    if dec.status == VP8StatusCode::VP8_STATUS_OK {
        dec.status = error;
        dec.error_msg = Some(msg);
        dec.ready = false;
    }
    dec.status
}

//------------------------------------------------------------------------------

// upstream: src/dec/vp8_dec.c VP8CheckSignature
/// Returns true if the next 3 bytes in `data` contain the VP8 signature.
#[must_use]
pub fn vp8_check_signature(data: &[u8]) -> bool {
    data.len() >= 3 && data[0] == 0x9d && data[1] == 0x01 && data[2] == 0x2a
}

// upstream: src/dec/vp8_dec.c VP8GetInfo
/// Validates the VP8 data-header and retrieves the width and height; `None` in
/// case of formatting error. `data` is the data available so far and
/// `chunk_size` the total data size expected in the chunk. C writes the width
/// and height only on success, and accepts `NULL` for them; a `NULL` `data`
/// exists only at the C ABI.
#[must_use]
pub fn vp8_get_info(data: &[u8], chunk_size: usize) -> Option<(i32, i32)> {
    if data.len() < VP8_FRAME_HEADER_SIZE.unsigned_abs() as usize {
        return None; // not enough data
    }
    // check signature
    if !vp8_check_signature(&data[3..]) {
        return None; // Wrong signature.
    }
    let bits = u32::from(data[0]) | (u32::from(data[1]) << 8) | (u32::from(data[2]) << 16);
    let key_frame = (bits & 1) == 0;
    let w = ((i32::from(data[7]) << 8) | i32::from(data[6])) & 0x3fff;
    let h = ((i32::from(data[9]) << 8) | i32::from(data[8])) & 0x3fff;

    if !key_frame {
        // Not a keyframe.
        return None;
    }

    if ((bits >> 1) & 7) > 3 {
        return None; // unknown profile
    }
    if ((bits >> 4) & 1) == 0 {
        return None; // first frame is invisible!
    }
    if (bits >> 5) as usize >= chunk_size {
        // partition_length
        return None; // inconsistent size information.
    }
    if w == 0 || h == 0 {
        return None; // We don't support both width and height to be zero.
    }
    Some((w, h))
}

//------------------------------------------------------------------------------
// Header parsing

/// `VP8InitBitReader(br, data + start, size)`: the reader is set up over the
/// `size` bytes at `start`, and its positions are then moved by `start`, so that
/// they are positions in `data`, the input that the owner passes to every read
/// (ADR 0003, "Readers over input"). This helper has no C counterpart.
fn init_bit_reader_at(br: &mut VP8BitReader, data: &[u8], start: usize, size: usize) {
    vp8_init_bit_reader(br, &data[start..start + size]);
    vp8_remap_bit_reader(
        br,
        isize::try_from(start).expect("a position in the input fits in isize"),
    );
}

// upstream: src/dec/vp8_dec.c ResetSegmentHeader
fn reset_segment_header(hdr: &mut VP8SegmentHeader) {
    hdr.use_segment = false;
    hdr.update_map = false;
    hdr.absolute_delta = true;
    hdr.quantizer = [0; NUM_MB_SEGMENTS as usize];
    hdr.filter_strength = [0; NUM_MB_SEGMENTS as usize];
}

/// The `int8_t` that C stores from a signed value of at most 7 bits.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C assigns the int value of at most 7 bits to an int8_t"
)]
fn value_i8(v: i32) -> i8 {
    v as i8
}

// upstream: src/dec/vp8_dec.c ParseSegmentHeader
/// Paragraph 9.3. C takes `br`, `hdr` and `proba` as separate pointers into
/// the decoder; they are the fields of `dec` here.
fn parse_segment_header(dec: &mut VP8Decoder, data: &[u8]) -> bool {
    let VP8Decoder {
        br,
        segment_hdr: hdr,
        proba,
        ..
    } = dec;
    hdr.use_segment = vp8_get(br, data) != 0;
    if hdr.use_segment {
        hdr.update_map = vp8_get(br, data) != 0;
        if vp8_get(br, data) != 0 {
            // update data
            hdr.absolute_delta = vp8_get(br, data) != 0;
            for s in 0..NUM_MB_SEGMENTS as usize {
                hdr.quantizer[s] = if vp8_get(br, data) != 0 {
                    value_i8(vp8_get_signed_value(br, data, 7))
                } else {
                    0
                };
            }
            for s in 0..NUM_MB_SEGMENTS as usize {
                hdr.filter_strength[s] = if vp8_get(br, data) != 0 {
                    value_i8(vp8_get_signed_value(br, data, 6))
                } else {
                    0
                };
            }
        }
        if hdr.update_map {
            for s in 0..MB_FEATURE_TREE_PROBS as usize {
                proba.segments[s] = if vp8_get(br, data) != 0 {
                    #[expect(
                        clippy::cast_possible_truncation,
                        reason = "C assigns the 8-bit value to a uint8_t"
                    )]
                    let value = vp8_get_value(br, data, 8) as u8;
                    value
                } else {
                    255
                };
            }
        }
    } else {
        hdr.update_map = false;
    }
    !br.eof
}

// upstream: src/dec/vp8_dec.c ParsePartitions
/// Paragraph 9.5.
/// If we don't have all the necessary data in `data[buf..buf + size]`, this
/// function returns `VP8_STATUS_SUSPENDED` in incremental decoding,
/// `VP8_STATUS_NOT_ENOUGH_DATA` otherwise.
/// In incremental decoding, this case is not necessarily an error. Still, no
/// bitreader is ever initialized to make it possible to read unavailable memory.
/// If we don't even have the partitions' sizes, then `VP8_STATUS_NOT_ENOUGH_DATA`
/// is returned, and this is an unrecoverable error.
/// If the partitions were positioned ok, `VP8_STATUS_OK` is returned (`Ok`).
fn parse_partitions(
    dec: &mut VP8Decoder,
    data: &[u8],
    buf: usize,
    size: usize,
) -> Result<(), VP8StatusCode> {
    let br = &mut dec.br;
    let mut sz = buf;
    let buf_end = buf + size;
    let mut size_left = size;

    dec.num_parts_minus_one = (1 << vp8_get_value(br, data, 2)) - 1;
    let last_part = dec.num_parts_minus_one as usize;
    if size < 3 * last_part {
        // we can't even read the sizes with sz[]! That's a failure.
        return Err(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA);
    }
    let mut part_start = buf + last_part * 3;
    size_left -= last_part * 3;
    for p in 0..last_part {
        let mut psize = usize::from(data[sz])
            | (usize::from(data[sz + 1]) << 8)
            | (usize::from(data[sz + 2]) << 16);
        if psize > size_left {
            psize = size_left;
        }
        init_bit_reader_at(&mut dec.parts[p], data, part_start, psize);
        part_start += psize;
        size_left -= psize;
        sz += 3;
    }
    init_bit_reader_at(&mut dec.parts[last_part], data, part_start, size_left);
    if part_start < buf_end {
        return Ok(());
    }
    Err(if dec.incremental {
        VP8StatusCode::VP8_STATUS_SUSPENDED // Init is ok, but there's not enough data
    } else {
        VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA
    })
}

// upstream: src/dec/vp8_dec.c ParseFilterHeader
/// Paragraph 9.4. C takes `br` next to `dec`, and passes `&dec->br`.
#[expect(
    clippy::cast_possible_wrap,
    reason = "C assigns the unsigned values of at most 6 bits to ints"
)]
fn parse_filter_header(dec: &mut VP8Decoder, data: &[u8]) -> bool {
    let VP8Decoder {
        br,
        filter_hdr: hdr,
        ..
    } = dec;
    hdr.simple = vp8_get(br, data) != 0;
    hdr.level = vp8_get_value(br, data, 6) as i32;
    hdr.sharpness = vp8_get_value(br, data, 3) as i32;
    hdr.use_lf_delta = vp8_get(br, data) != 0;
    if hdr.use_lf_delta && vp8_get(br, data) != 0 {
        // update lf-delta?
        for i in 0..NUM_REF_LF_DELTAS as usize {
            if vp8_get(br, data) != 0 {
                hdr.ref_lf_delta[i] = vp8_get_signed_value(br, data, 6);
            }
        }
        for i in 0..NUM_MODE_LF_DELTAS as usize {
            if vp8_get(br, data) != 0 {
                hdr.mode_lf_delta[i] = vp8_get_signed_value(br, data, 6);
            }
        }
    }
    dec.filter_type = if hdr.level == 0 {
        0
    } else if hdr.simple {
        1
    } else {
        2
    };
    !br.eof
}

// upstream: src/dec/vp8_dec.c VP8GetHeaders
/// Decodes the VP8 frame header. `data` is C's `io->data[..io->data_size]`, which
/// must start at the VP8 frame header (ADR 0003). Returns `Err` with the
/// recorded status after an error. C also accepts a `NULL` decoder or io, which
/// exist only at the C ABI.
///
/// # Errors
///
/// Returns the status that `VP8SetError` recorded.
pub fn vp8_get_headers(
    dec: &mut VP8Decoder,
    io: &mut VP8Io,
    data: &[u8],
) -> Result<(), VP8StatusCode> {
    vp8_get_headers_at(dec, io, data, 0)
}

/// `VP8GetHeaders` for a C `io->data` that points `start` bytes into `data`, the
/// whole input that the owner passes to every later read. The readers then hold
/// positions in `data`, as the C readers hold pointers into the owner's buffer.
/// The incremental decoder (WP12) needs this: its input is its whole memory
/// buffer, the frame starts at `mem.start`, and `DoRemap` moves the readers by
/// index deltas of that buffer. This function has no C counterpart of its own.
///
/// # Errors
///
/// Returns the status that `VP8SetError` recorded.
///
/// # Panics
///
/// Panics when `start` is past the end of `data`.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C assigns the bit fields of the header to their uint8_t fields"
)]
#[expect(clippy::too_many_lines, reason = "the C function")]
pub(crate) fn vp8_get_headers_at(
    dec: &mut VP8Decoder,
    io: &mut VP8Io,
    data: &[u8],
    start: usize,
) -> Result<(), VP8StatusCode> {
    set_ok(dec);
    let mut buf = start;
    let mut buf_size = data.len() - start;
    if buf_size < 4 {
        return Err(vp8_set_error(
            dec,
            VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA,
            "Truncated header.",
        ));
    }

    // Paragraph 9.1
    {
        let bits = u32::from(data[buf])
            | (u32::from(data[buf + 1]) << 8)
            | (u32::from(data[buf + 2]) << 16);
        let frm_hdr = &mut dec.frm_hdr;
        frm_hdr.key_frame = u8::from((bits & 1) == 0);
        frm_hdr.profile = ((bits >> 1) & 7) as u8;
        frm_hdr.show = ((bits >> 4) & 1) as u8;
        frm_hdr.partition_length = bits >> 5;
        if frm_hdr.profile > 3 {
            return Err(vp8_set_error(
                dec,
                VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR,
                "Incorrect keyframe parameters.",
            ));
        }
        if frm_hdr.show == 0 {
            return Err(vp8_set_error(
                dec,
                VP8StatusCode::VP8_STATUS_UNSUPPORTED_FEATURE,
                "Frame not displayable.",
            ));
        }
        buf += 3;
        buf_size -= 3;
    }

    if dec.frm_hdr.key_frame != 0 {
        // Paragraph 9.2
        if buf_size < 7 {
            return Err(vp8_set_error(
                dec,
                VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA,
                "cannot parse picture header",
            ));
        }
        if !vp8_check_signature(&data[buf..buf + buf_size]) {
            return Err(vp8_set_error(
                dec,
                VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR,
                "Bad code word",
            ));
        }
        let pic_hdr = &mut dec.pic_hdr;
        pic_hdr.width = ((u16::from(data[buf + 4]) << 8) | u16::from(data[buf + 3])) & 0x3fff;
        pic_hdr.xscale = data[buf + 4] >> 6; // ratio: 1, 5/4 5/3 or 2
        pic_hdr.height = ((u16::from(data[buf + 6]) << 8) | u16::from(data[buf + 5])) & 0x3fff;
        pic_hdr.yscale = data[buf + 6] >> 6;
        buf += 7;
        buf_size -= 7;

        dec.mb_w = (i32::from(pic_hdr.width) + 15) >> 4;
        dec.mb_h = (i32::from(pic_hdr.height) + 15) >> 4;

        // Setup default output area (can be later modified during io->setup())
        io.width = i32::from(pic_hdr.width);
        io.height = i32::from(pic_hdr.height);
        // IMPORTANT! use some sane dimensions in crop* and scaled* fields.
        // So they can be used interchangeably without always testing for
        // 'use_cropping'.
        io.use_cropping = false;
        io.crop_top = 0;
        io.crop_left = 0;
        io.crop_right = io.width;
        io.crop_bottom = io.height;
        io.use_scaling = false;
        io.scaled_width = io.width;
        io.scaled_height = io.height;

        io.mb_w = io.width; // for soundness
        io.mb_h = io.height; // ditto

        vp8_reset_proba(&mut dec.proba);
        reset_segment_header(&mut dec.segment_hdr);
    }

    // Check if we have all the partition #0 available, and initialize dec->br
    // to read this partition (and this partition only).
    if dec.frm_hdr.partition_length as usize > buf_size {
        return Err(vp8_set_error(
            dec,
            VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA,
            "bad partition length",
        ));
    }

    let partition_length = dec.frm_hdr.partition_length as usize;
    init_bit_reader_at(&mut dec.br, data, buf, partition_length);
    buf += partition_length;
    buf_size -= partition_length;

    if dec.frm_hdr.key_frame != 0 {
        dec.pic_hdr.colorspace = vp8_get(&mut dec.br, data) as u8;
        dec.pic_hdr.clamp_type = vp8_get(&mut dec.br, data) as u8;
    }
    if !parse_segment_header(dec, data) {
        return Err(vp8_set_error(
            dec,
            VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR,
            "cannot parse segment header",
        ));
    }
    // Filter specs
    if !parse_filter_header(dec, data) {
        return Err(vp8_set_error(
            dec,
            VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR,
            "cannot parse filter header",
        ));
    }
    if let Err(status) = parse_partitions(dec, data, buf, buf_size) {
        return Err(vp8_set_error(dec, status, "cannot parse partitions"));
    }

    // quantizer change
    vp8_parse_quant(dec, data);

    // Frame buffer marking
    if dec.frm_hdr.key_frame == 0 {
        return Err(vp8_set_error(
            dec,
            VP8StatusCode::VP8_STATUS_UNSUPPORTED_FEATURE,
            "Not a key frame.",
        ));
    }

    vp8_get(&mut dec.br, data); // ignore the value of 'update_proba'

    vp8_parse_proba(dec, data);

    // sanitized state
    dec.ready = true;
    Ok(())
}

//------------------------------------------------------------------------------
// Residual decoding (Paragraph 13.2 / 13.3)

// upstream: src/dec/vp8_dec.c kCat3
const CAT3: [u8; 4] = [173, 148, 140, 0];
// upstream: src/dec/vp8_dec.c kCat4
const CAT4: [u8; 5] = [176, 155, 140, 135, 0];
// upstream: src/dec/vp8_dec.c kCat5
const CAT5: [u8; 6] = [180, 157, 141, 134, 130, 0];
// upstream: src/dec/vp8_dec.c kCat6
const CAT6: [u8; 12] = [254, 254, 243, 230, 196, 177, 153, 140, 133, 130, 129, 0];
// upstream: src/dec/vp8_dec.c kCat3456
const CAT3456: [&[u8]; 4] = [&CAT3, &CAT4, &CAT5, &CAT6];
// upstream: src/dec/vp8_dec.c kZigzag
const ZIGZAG: [u8; 16] = [0, 1, 4, 8, 5, 2, 3, 6, 9, 12, 13, 10, 7, 11, 14, 15];

// upstream: src/dec/vp8_dec.c GetLargeValue
/// See section 13-2: <https://datatracker.ietf.org/doc/html/rfc6386#section-13.2>
#[expect(
    clippy::misrefactored_assign_op,
    reason = "the C expression v += v + bit"
)]
fn get_large_value(br: &mut VP8BitReader, data: &[u8], p: &[u8; 11]) -> i32 {
    let prob = |i: usize| i32::from(p[i]);
    let v;
    if vp8_get_bit(br, data, prob(3)) == 0 {
        if vp8_get_bit(br, data, prob(4)) == 0 {
            v = 2;
        } else {
            v = 3 + vp8_get_bit(br, data, prob(5));
        }
    } else if vp8_get_bit(br, data, prob(6)) == 0 {
        if vp8_get_bit(br, data, prob(7)) == 0 {
            v = 5 + vp8_get_bit(br, data, 159);
        } else {
            let mut w = 7 + 2 * vp8_get_bit(br, data, 165);
            w += vp8_get_bit(br, data, 145);
            v = w;
        }
    } else {
        let bit1 = vp8_get_bit(br, data, prob(8));
        let bit0 = vp8_get_bit(br, data, prob(9 + usize::from(bit1 != 0)));
        let cat = 2 * bit1 + bit0;
        let mut w = 0;
        for &tab in CAT3456[usize::try_from(cat).expect("two bits")] {
            if tab == 0 {
                break;
            }
            w += w + vp8_get_bit(br, data, i32::from(tab));
        }
        w += 3 + (8 << cat);
        v = w;
    }
    v
}

/// The `int16_t` that C stores from the dequantized coefficient.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C stores the int product in an int16_t"
)]
fn coeff(v: i32) -> i16 {
    v as i16
}

/// The `int` coefficient position that C returns.
fn position(n: usize) -> i32 {
    i32::try_from(n).expect("a position in [0, 16]")
}

// upstream: src/dec/vp8_dec.c GetCoeffsFast
/// Returns the position of the last non-zero coeff plus one.
#[expect(clippy::too_many_arguments, reason = "the C parameters with the input")]
fn get_coeffs_fast(
    br: &mut VP8BitReader,
    data: &[u8],
    bands: &[VP8BandProbas; NUM_BANDS as usize],
    prob: &[usize; 16 + 1],
    ctx: i32,
    dq: quant_t,
    n: i32,
    out: &mut [i16],
) -> i32 {
    let ctx = usize::try_from(ctx).expect("a context in [0, 2]");
    let mut n = usize::try_from(n).expect("a position in [0, 15]");
    let mut p = &bands[prob[n]].probas[ctx];
    while n < 16 {
        if vp8_get_bit(br, data, i32::from(p[0])) == 0 {
            return position(n); // previous coeff was last non-zero coeff
        }
        while vp8_get_bit(br, data, i32::from(p[1])) == 0 {
            // sequence of zero coeffs
            n += 1;
            p = &bands[prob[n]].probas[0];
            if n == 16 {
                return 16;
            }
        }
        {
            // non zero coeff
            let p_ctx = &bands[prob[n + 1]].probas;
            let v;
            if vp8_get_bit(br, data, i32::from(p[2])) == 0 {
                v = 1;
                p = &p_ctx[1];
            } else {
                v = get_large_value(br, data, p);
                p = &p_ctx[2];
            }
            out[usize::from(ZIGZAG[n])] =
                coeff(vp8_get_signed(br, data, v) * dq[usize::from(n > 0)]);
        }
        n += 1;
    }
    16
}

// upstream: src/dec/vp8_dec.c GetCoeffsAlt
/// This version of `GetCoeffs()` uses `VP8GetBitAlt()` which is an alternate
/// version of `VP8GetBitAlt()` targeting specific platforms.
#[expect(clippy::too_many_arguments, reason = "the C parameters with the input")]
fn get_coeffs_alt(
    br: &mut VP8BitReader,
    data: &[u8],
    bands: &[VP8BandProbas; NUM_BANDS as usize],
    prob: &[usize; 16 + 1],
    ctx: i32,
    dq: quant_t,
    n: i32,
    out: &mut [i16],
) -> i32 {
    let ctx = usize::try_from(ctx).expect("a context in [0, 2]");
    let mut n = usize::try_from(n).expect("a position in [0, 15]");
    let mut p = &bands[prob[n]].probas[ctx];
    while n < 16 {
        if vp8_get_bit_alt(br, data, i32::from(p[0])) == 0 {
            return position(n); // previous coeff was last non-zero coeff
        }
        while vp8_get_bit_alt(br, data, i32::from(p[1])) == 0 {
            // sequence of zero coeffs
            n += 1;
            p = &bands[prob[n]].probas[0];
            if n == 16 {
                return 16;
            }
        }
        {
            // non zero coeff
            let p_ctx = &bands[prob[n + 1]].probas;
            let v;
            if vp8_get_bit_alt(br, data, i32::from(p[2])) == 0 {
                v = 1;
                p = &p_ctx[1];
            } else {
                v = get_large_value(br, data, p);
                p = &p_ctx[2];
            }
            out[usize::from(ZIGZAG[n])] =
                coeff(vp8_get_signed(br, data, v) * dq[usize::from(n > 0)]);
        }
        n += 1;
    }
    16
}

// upstream: src/dec/vp8_dec.c InitGetCoeffs
/// Selects `GetCoeffsAlt` on CPUs with slow SSSE3 and `GetCoeffsFast`
/// otherwise. In the reference configuration `VP8GetCPUInfo` is `NULL` except on
/// x86, where it answers for the running CPU (ADR 0003).
fn init_get_coeffs() -> GetCoeffsFunc {
    if vp8_get_cpu_info(CPUFeature::kSlowSSSE3) {
        get_coeffs_alt
    } else {
        get_coeffs_fast
    }
}

// upstream: src/dec/vp8_dec.c NzCodeBits
fn nz_code_bits(nz_coeffs: u32, nz: i32, dc_nz: bool) -> u32 {
    let mut nz_coeffs = nz_coeffs << 2;
    nz_coeffs |= if nz > 3 {
        3
    } else if nz > 1 {
        2
    } else {
        u32::from(dc_nz)
    };
    nz_coeffs
}

/// The `uint8_t` that C stores from an `int` or `uint32_t` context value.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C stores the int or uint32_t value in a uint8_t"
)]
fn low_u8(v: i64) -> u8 {
    v as u8
}

// upstream: src/dec/vp8_dec.c ParseResiduals
/// Parses the residuals of the macroblock at `dec.mb_x`. `mb` is the index of
/// C's `mb` in `dec.mb_info` (index 0 is C's `mb_info[-1]`, the left context),
/// and `token_br` the index of the partition reader in `dec.parts`.
#[expect(clippy::too_many_lines, reason = "the C function")]
#[expect(clippy::similar_names, reason = "the C names out_t_nz and out_l_nz")]
fn parse_residuals(dec: &mut VP8Decoder, mb: usize, token_br: usize, data: &[u8]) -> bool {
    let VP8Decoder {
        parts,
        proba,
        mb_data,
        mb_x,
        dqm,
        mb_info,
        ..
    } = dec;
    let get_coeffs = *GET_COEFFS;
    let token_br = &mut parts[token_br];
    let bands = &proba.bands;
    let bands_ptr = &proba.bands_ptr;
    let block = &mut mb_data[usize::try_from(*mb_x).expect("a non-negative position")];
    let q = &dqm[usize::from(block.segment)];
    let dst = &mut block.coeffs;
    let mut dst_pos = 0;
    let mut non_zero_y: u32 = 0;
    let mut non_zero_uv: u32 = 0;

    *dst = [0; 384];
    let (first, ac_proba) = if block.is_i4x4 == 0 {
        // parse DC
        let mut dc = [0i16; 16];
        let ctx = i32::from(mb_info[mb].nz_dc) + i32::from(mb_info[0].nz_dc);
        let nz = get_coeffs(
            token_br,
            data,
            &bands[1],
            &bands_ptr[1],
            ctx,
            q.y2_mat,
            0,
            &mut dc,
        );
        let nz_dc = u8::from(nz > 0);
        mb_info[mb].nz_dc = nz_dc;
        mb_info[0].nz_dc = nz_dc;
        if nz > 1 {
            // more than just the DC -> perform the full transform
            vp8_transform_wht(&dc, dst);
        } else {
            // only DC is non-zero -> inlined simplified transform
            let dc0 = coeff((i32::from(dc[0]) + 3) >> 3);
            for i in (0..16 * 16).step_by(16) {
                dst[i] = dc0;
            }
        }
        (1, 0)
    } else {
        (0, 3)
    };

    let mut tnz: u8 = mb_info[mb].nz & 0x0f;
    let mut lnz: u8 = mb_info[0].nz & 0x0f;
    for _y in 0..4 {
        let mut l = i32::from(lnz & 1);
        let mut nz_coeffs: u32 = 0;
        for _x in 0..4 {
            let ctx = l + i32::from(tnz & 1);
            let nz = get_coeffs(
                token_br,
                data,
                &bands[ac_proba],
                &bands_ptr[ac_proba],
                ctx,
                q.y1_mat,
                first,
                &mut dst[dst_pos..],
            );
            l = i32::from(nz > first);
            tnz = low_u8(i64::from((i32::from(tnz) >> 1) | (l << 7)));
            nz_coeffs = nz_code_bits(nz_coeffs, nz, dst[dst_pos] != 0);
            dst_pos += 16;
        }
        tnz >>= 4;
        lnz = low_u8(i64::from((i32::from(lnz) >> 1) | (l << 7)));
        non_zero_y = (non_zero_y << 8) | nz_coeffs;
    }
    let mut out_t_nz = u32::from(tnz);
    let mut out_l_nz = u32::from(lnz >> 4);

    for ch in (0..4).step_by(2) {
        let mut nz_coeffs: u32 = 0;
        tnz = mb_info[mb].nz >> (4 + ch);
        lnz = mb_info[0].nz >> (4 + ch);
        for _y in 0..2 {
            let mut l = i32::from(lnz & 1);
            for _x in 0..2 {
                let ctx = l + i32::from(tnz & 1);
                let nz = get_coeffs(
                    token_br,
                    data,
                    &bands[2],
                    &bands_ptr[2],
                    ctx,
                    q.uv_mat,
                    0,
                    &mut dst[dst_pos..],
                );
                l = i32::from(nz > 0);
                tnz = low_u8(i64::from((i32::from(tnz) >> 1) | (l << 3)));
                nz_coeffs = nz_code_bits(nz_coeffs, nz, dst[dst_pos] != 0);
                dst_pos += 16;
            }
            tnz >>= 2;
            lnz = low_u8(i64::from((i32::from(lnz) >> 1) | (l << 5)));
        }
        // Note: we don't really need the per-4x4 details for U/V blocks.
        non_zero_uv |= nz_coeffs << (4 * ch);
        out_t_nz |= (u32::from(tnz) << 4) << ch;
        out_l_nz |= (u32::from(lnz) & 0xf0) << ch;
    }
    mb_info[mb].nz = low_u8(i64::from(out_t_nz));
    mb_info[0].nz = low_u8(i64::from(out_l_nz));

    block.non_zero_y = non_zero_y;
    block.non_zero_uv = non_zero_uv;

    // We look at the mode-code of each block and check if some blocks have less
    // than three non-zero coeffs (code < 2). This is to avoid dithering flat and
    // empty blocks.
    block.dither = if (non_zero_uv & 0xaaaa) != 0 {
        0
    } else {
        low_u8(i64::from(q.dither))
    };

    (non_zero_y | non_zero_uv) == 0 // will be used for further optimization
}

//------------------------------------------------------------------------------
// Main loop

// upstream: src/dec/vp8_dec.c VP8DecodeMB
/// Decodes one macroblock. Returns false if there is not enough data. C takes
/// the token reader as a pointer, and every caller passes
/// `&dec->parts[dec->mb_y & dec->num_parts_minus_one]`; here it is the index of
/// that reader in `dec.parts`.
///
/// # Panics
///
/// Panics when `dec.mb_x` is outside the macroblock row that `VP8InitFrame`
/// allocated, or `token_br` outside `dec.parts`.
pub fn vp8_decode_mb(dec: &mut VP8Decoder, token_br: usize, data: &[u8]) -> bool {
    let mb_x = usize::try_from(dec.mb_x).expect("a non-negative position");
    let mb = mb_x + 1; // C: dec->mb_info + dec->mb_x
    let block = &dec.mb_data[mb_x];
    let mut skip = if dec.use_skip_proba {
        block.skip != 0
    } else {
        false
    };

    if skip {
        let is_i4x4 = block.is_i4x4 != 0;
        dec.mb_info[0].nz = 0;
        dec.mb_info[mb].nz = 0;
        if !is_i4x4 {
            dec.mb_info[0].nz_dc = 0;
            dec.mb_info[mb].nz_dc = 0;
        }
        let block = &mut dec.mb_data[mb_x];
        block.non_zero_y = 0;
        block.non_zero_uv = 0;
        block.dither = 0;
    } else {
        skip = parse_residuals(dec, mb, token_br, data);
    }

    if dec.filter_type > 0 {
        // store filter info
        let block = &dec.mb_data[mb_x];
        let mut finfo = dec.fstrengths[usize::from(block.segment)][usize::from(block.is_i4x4)];
        finfo.f_inner |= u8::from(!skip);
        dec.f_info[mb_x] = finfo;
    }

    !dec.parts[token_br].eof
}

// upstream: src/dec/vp8_dec.c VP8InitScanline
/// To be called at the start of a new scanline, to initialize predictors.
pub fn vp8_init_scanline(dec: &mut VP8Decoder) {
    let left = &mut dec.mb_info[0];
    left.nz = 0;
    left.nz_dc = 0;
    #[expect(
        clippy::cast_possible_truncation,
        reason = "memset with the mode B_DC_PRED"
    )]
    let mode = B_DC_PRED as u8;
    dec.intra_l = [mode; 4];
    dec.mb_x = 0;
}

// upstream: src/dec/vp8_dec.c ParseFrame
/// Parses, reconstructs and emits every macroblock row. Returns false on error,
/// which C does not record when the last threaded row fails in `put()`.
fn parse_frame(
    dec: &mut VP8Decoder,
    io: &mut VP8Io,
    data: &[u8],
    alpha_data: Option<&[u8]>,
    hooks: Option<&mut (dyn VP8IoHooks + '_)>,
) -> bool {
    let mut hooks = hooks;
    dec.mb_y = 0;
    while dec.mb_y < dec.br_mb_y {
        // Parse bitstream for this row.
        let token_br = (dec.mb_y.cast_unsigned() & dec.num_parts_minus_one) as usize;
        if !vp8_parse_intra_mode_row(dec, data) {
            let _ = vp8_set_error(
                dec,
                VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA,
                "Premature end-of-partition0 encountered.",
            );
            return false;
        }
        while dec.mb_x < dec.mb_w {
            if !vp8_decode_mb(dec, token_br, data) {
                let _ = vp8_set_error(
                    dec,
                    VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA,
                    "Premature end-of-file encountered.",
                );
                return false;
            }
            dec.mb_x += 1;
        }
        vp8_init_scanline(dec); // Prepare for next scanline

        // Reconstruct, filter and emit the row.
        if !vp8_process_row(dec, io, alpha_data, hooks.as_deref_mut()) {
            let _ = vp8_set_error(dec, VP8StatusCode::VP8_STATUS_USER_ABORT, "Output aborted.");
            return false;
        }
        dec.mb_y += 1;
    }
    if dec.mt_method > 0 && !vp8_sync_rows(dec, alpha_data, hooks) {
        return false;
    }

    true
}

// upstream: src/dec/vp8_dec.c VP8Decode
/// Decodes a picture. Will call `VP8GetHeaders()` if it wasn't done already.
/// Returns false in case of error; the status is in `dec.status`, as in C,
/// which does not record one when the `put()` of the last threaded row fails.
///
/// `data` is C's `io->data[..io->data_size]`, `alpha_data` is C's
/// `dec->alpha_data[..alpha_data_size]` (`None` for `NULL`), and `hooks` are the
/// io's hooks with their user data (`None`: all three `NULL`). The owner passes
/// the same input to every call of one decoding (ADR 0003, "Decoding context
/// per call"). C also accepts a `NULL` decoder or io, which exist only at the C
/// ABI.
pub fn vp8_decode(
    dec: &mut VP8Decoder,
    io: &mut VP8Io,
    data: &[u8],
    alpha_data: Option<&[u8]>,
    hooks: Option<&mut (dyn VP8IoHooks + '_)>,
) -> bool {
    let mut hooks = hooks;
    if !dec.ready && vp8_get_headers(dec, io, data).is_err() {
        return false;
    }
    debug_assert!(dec.ready);

    // Finish setting up the decoding parameter. Will call io->setup().
    let mut ok = vp8_enter_critical(dec, io, hooks.as_deref_mut()).is_ok();
    if ok {
        // good to go.
        // Will allocate memory and prepare everything.
        ok = vp8_init_frame(dec, io, alpha_data);

        // Main decoding loop
        if ok {
            ok = parse_frame(dec, io, data, alpha_data, hooks.as_deref_mut());
        }

        // Exit.
        ok &= vp8_exit_critical(dec, io, alpha_data, hooks);
    }

    if !ok {
        vp8_clear(dec);
        return false;
    }

    dec.ready = false;
    ok
}

// upstream: src/dec/vp8_dec.c VP8Clear
/// Resets the decoder in its initial state, reclaiming memory. Not a mandatory
/// call between calls to `VP8Decode()`. The row job stays in the worker with
/// empty buffers; a `put()` part that was not synced is dropped (no caller
/// clears a decoder with a pending row: they call `VP8ExitCritical` first).
pub fn vp8_clear(dec: &mut VP8Decoder) {
    dec.worker.end();
    webp_deallocate_alpha_memory(dec);
    // C: WebPSafeFree(dec->mem).
    dec.intra_t = Vec::new();
    dec.mb_info = Vec::new();
    dec.f_info = Vec::new();
    dec.mb_data = Vec::new();
    dec.frame_alpha = Vec::new();
    {
        let job = row_job(&mut dec.worker);
        job.yuv_t = Vec::new();
        job.yuv_b = Vec::new();
        job.cache_y = Vec::new();
        job.cache_u = Vec::new();
        job.cache_v = Vec::new();
        job.thread_ctx.f_info = Vec::new();
        job.thread_ctx.mb_data = Vec::new();
    }
    dec.mem_size = 0;
    dec.br = VP8BitReader::default();
    dec.ready = false;
    dec.put_pending = false;
}

// upstream: src/dec/vp8_dec.c VP8Delete
// VP8Delete clears and frees the decoder: the owner drops the box (C:
// VP8Clear, then WebPSafeFree), and dropping the worker ends its thread.
