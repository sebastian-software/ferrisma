// libwebp v1.6.0, src/enc/syntax_enc.c:
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
// Header syntax writing
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/syntax_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).
//!
//! # Upstream behavior preserved
//!
//! Behavior of the C code that a reader might take for a defect, which the
//! translation keeps:
//!
//! 1. **`GeneratePartition0` measures its first position on the zeroed writer**
//!    before `VP8BitWriterInit`, which counts 8 bits.

use crate::dec::common_dec::{MAX_NUM_PARTITIONS, NUM_MB_SEGMENTS};
use crate::enc::tree_enc::{vp8_code_intra_modes, vp8_write_probas};
use crate::enc::vp8i_enc::{VP8EncFilterHeader, VP8Encoder};
use crate::enc::webp_enc::{webp_encoding_set_error, webp_report_progress};
use crate::utils::bit_writer_utils::{
    VP8BitWriter, vp8_bit_writer_finish, vp8_bit_writer_init, vp8_bit_writer_pos,
    vp8_bit_writer_size, vp8_bit_writer_wipe_out, vp8_put_bit_uniform, vp8_put_bits,
    vp8_put_signed_bits,
};
use crate::utils::utils::{put_le24, put_le32};
use crate::webp::encode::{WebPEncodingError, WebPPicture};
use crate::webp::format_constants::{
    CHUNK_HEADER_SIZE, MAX_CANVAS_SIZE, RIFF_HEADER_SIZE, TAG_SIZE, VP8_FRAME_HEADER_SIZE,
    VP8_MAX_PARTITION_SIZE, VP8_MAX_PARTITION0_SIZE, VP8_SIGNATURE, VP8X_CHUNK_SIZE,
};
use crate::webp::mux_types::WebPFeatureFlags;

/// Converts a non-negative C `int` to an index.
#[expect(clippy::cast_sign_loss, reason = "the value is non-negative")]
const fn idx(v: i32) -> usize {
    v as usize
}

/// Calls the picture's writer (C: `pic->writer(data, size, pic)`).
///
/// # Panics
///
/// Panics when the picture has no writer, which C calls through `NULL`.
fn write(pic: &mut WebPPicture<'_>, data: &[u8]) -> bool {
    let writer = pic
        .writer
        .as_mut()
        .expect("the picture has a writer (C calls NULL otherwise)");
    writer(data)
}

//------------------------------------------------------------------------------
// Helper functions

// upstream: src/enc/syntax_enc.c IsVP8XNeeded
fn is_vp8x_needed(enc: &VP8Encoder) -> bool {
    enc.has_alpha // Currently the only case when VP8X is needed.
    // This could change in the future.
}

// upstream: src/enc/syntax_enc.c PutPaddingByte
fn put_padding_byte(pic: &mut WebPPicture<'_>) -> bool {
    let pad_byte: [u8; 1] = [0];
    write(pic, &pad_byte)
}

//------------------------------------------------------------------------------
// Writers for header's various pieces (in order of appearance)

// upstream: src/enc/syntax_enc.c PutRIFFHeader
#[expect(
    clippy::cast_possible_truncation,
    reason = "the RIFF size fits 32 bits"
)]
fn put_riff_header(pic: &mut WebPPicture<'_>, riff_size: usize) -> WebPEncodingError {
    let mut riff: [u8; RIFF_HEADER_SIZE as usize] =
        [b'R', b'I', b'F', b'F', 0, 0, 0, 0, b'W', b'E', b'B', b'P'];
    debug_assert!(u32::try_from(riff_size).is_ok());
    put_le32(&mut riff[idx(TAG_SIZE)..], riff_size as u32);
    if !write(pic, &riff) {
        return WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE;
    }
    WebPEncodingError::VP8_ENC_OK
}

// upstream: src/enc/syntax_enc.c PutVP8XHeader
fn put_vp8x_header(enc: &VP8Encoder, pic: &mut WebPPicture<'_>) -> WebPEncodingError {
    let mut vp8x = [0_u8; (CHUNK_HEADER_SIZE + VP8X_CHUNK_SIZE) as usize];
    vp8x[..4].copy_from_slice(b"VP8X");
    let mut flags: u32 = 0;

    debug_assert!(is_vp8x_needed(enc));
    debug_assert!(pic.width >= 1 && pic.height >= 1);
    debug_assert!(pic.width <= MAX_CANVAS_SIZE && pic.height <= MAX_CANVAS_SIZE);

    if enc.has_alpha {
        flags |= WebPFeatureFlags::ALPHA_FLAG.0;
    }

    put_le32(&mut vp8x[idx(TAG_SIZE)..], VP8X_CHUNK_SIZE as u32);
    put_le32(&mut vp8x[idx(CHUNK_HEADER_SIZE)..], flags);
    put_le24(&mut vp8x[idx(CHUNK_HEADER_SIZE) + 4..], pic.width - 1);
    put_le24(&mut vp8x[idx(CHUNK_HEADER_SIZE) + 7..], pic.height - 1);
    if !write(pic, &vp8x) {
        return WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE;
    }
    WebPEncodingError::VP8_ENC_OK
}

// upstream: src/enc/syntax_enc.c PutAlphaChunk
fn put_alpha_chunk(enc: &VP8Encoder, pic: &mut WebPPicture<'_>) -> WebPEncodingError {
    let mut alpha_chunk_hdr = [0_u8; CHUNK_HEADER_SIZE as usize];
    alpha_chunk_hdr[..4].copy_from_slice(b"ALPH");

    debug_assert!(enc.has_alpha);

    // Alpha chunk header.
    put_le32(&mut alpha_chunk_hdr[idx(TAG_SIZE)..], enc.alpha_data_size);
    if !write(pic, &alpha_chunk_hdr) {
        return WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE;
    }

    // Alpha chunk data.
    if !write(pic, &enc.alpha_data[..enc.alpha_data_size as usize]) {
        return WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE;
    }

    // Padding.
    if (enc.alpha_data_size & 1) != 0 && !put_padding_byte(pic) {
        return WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE;
    }
    WebPEncodingError::VP8_ENC_OK
}

// upstream: src/enc/syntax_enc.c PutVP8Header
#[expect(clippy::cast_possible_truncation, reason = "the VP8 size fits 32 bits")]
fn put_vp8_header(pic: &mut WebPPicture<'_>, vp8_size: usize) -> WebPEncodingError {
    let mut vp8_chunk_hdr = [0_u8; CHUNK_HEADER_SIZE as usize];
    vp8_chunk_hdr[..4].copy_from_slice(b"VP8 ");
    debug_assert!(u32::try_from(vp8_size).is_ok());
    put_le32(&mut vp8_chunk_hdr[idx(TAG_SIZE)..], vp8_size as u32);
    if !write(pic, &vp8_chunk_hdr) {
        return WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE;
    }
    WebPEncodingError::VP8_ENC_OK
}

// upstream: src/enc/syntax_enc.c PutVP8FrameHeader
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C stores the low bytes of the header fields"
)]
#[expect(clippy::identity_op, reason = "the C bit layout")]
fn put_vp8_frame_header(
    pic: &mut WebPPicture<'_>,
    profile: i32,
    size0: usize,
) -> WebPEncodingError {
    let mut vp8_frm_hdr = [0_u8; VP8_FRAME_HEADER_SIZE as usize];

    if size0 >= idx(VP8_MAX_PARTITION0_SIZE) {
        // partition #0 is too big to fit
        return WebPEncodingError::VP8_ENC_ERROR_PARTITION0_OVERFLOW;
    }

    // Paragraph 9.1.
    let bits: u32 = 0                         // keyframe (1b)
        | ((profile as u32) << 1)            // profile (3b)
        | (1 << 4)                           // visible (1b)
        | ((size0 as u32) << 5); // partition length (19b)
    vp8_frm_hdr[0] = (bits & 0xff) as u8;
    vp8_frm_hdr[1] = ((bits >> 8) & 0xff) as u8;
    vp8_frm_hdr[2] = ((bits >> 16) & 0xff) as u8;
    // signature
    vp8_frm_hdr[3] = ((VP8_SIGNATURE >> 16) & 0xff) as u8;
    vp8_frm_hdr[4] = ((VP8_SIGNATURE >> 8) & 0xff) as u8;
    vp8_frm_hdr[5] = (VP8_SIGNATURE & 0xff) as u8;
    // dimensions
    vp8_frm_hdr[6] = (pic.width & 0xff) as u8;
    vp8_frm_hdr[7] = (pic.width >> 8) as u8;
    vp8_frm_hdr[8] = (pic.height & 0xff) as u8;
    vp8_frm_hdr[9] = (pic.height >> 8) as u8;

    if !write(pic, &vp8_frm_hdr) {
        return WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE;
    }
    WebPEncodingError::VP8_ENC_OK
}

// upstream: src/enc/syntax_enc.c PutWebPHeaders
/// WebP Headers.
fn put_webp_headers(
    enc: &VP8Encoder,
    pic: &mut WebPPicture<'_>,
    size0: usize,
    vp8_size: usize,
    riff_size: usize,
) -> Result<(), WebPEncodingError> {
    let result = (|| {
        // RIFF header.
        let err = put_riff_header(pic, riff_size);
        if err != WebPEncodingError::VP8_ENC_OK {
            return err;
        }

        // VP8X.
        if is_vp8x_needed(enc) {
            let err = put_vp8x_header(enc, pic);
            if err != WebPEncodingError::VP8_ENC_OK {
                return err;
            }
        }

        // Alpha.
        if enc.has_alpha {
            let err = put_alpha_chunk(enc, pic);
            if err != WebPEncodingError::VP8_ENC_OK {
                return err;
            }
        }

        // VP8 header.
        let err = put_vp8_header(pic, vp8_size);
        if err != WebPEncodingError::VP8_ENC_OK {
            return err;
        }

        // VP8 frame header.
        put_vp8_frame_header(pic, enc.profile, size0)
    })();
    if result == WebPEncodingError::VP8_ENC_OK {
        // All OK.
        return Ok(());
    }
    // Error.
    Err(webp_encoding_set_error(pic, result))
}

// upstream: src/enc/syntax_enc.c PutSegmentHeader
/// Segmentation header.
#[allow(clippy::needless_range_loop, reason = "the C index loops")]
fn put_segment_header(bw: &mut VP8BitWriter, enc: &VP8Encoder) {
    let hdr = &enc.segment_hdr;
    let proba = &enc.proba;
    if vp8_put_bit_uniform(bw, hdr.num_segments > 1) {
        // We always 'update' the quant and filter strength values
        let update_data = true;
        vp8_put_bit_uniform(bw, hdr.update_map);
        if vp8_put_bit_uniform(bw, update_data) {
            // we always use absolute values, not relative ones
            vp8_put_bit_uniform(bw, true); // (segment_feature_mode = 1. Paragraph 9.3.)
            for s in 0..idx(NUM_MB_SEGMENTS) {
                vp8_put_signed_bits(bw, enc.dqm[s].quant, 7);
            }
            for s in 0..idx(NUM_MB_SEGMENTS) {
                vp8_put_signed_bits(bw, enc.dqm[s].fstrength, 6);
            }
        }
        if hdr.update_map {
            for s in 0..3 {
                if vp8_put_bit_uniform(bw, proba.segments[s] != 255) {
                    vp8_put_bits(bw, u32::from(proba.segments[s]), 8);
                }
            }
        }
    }
}

// upstream: src/enc/syntax_enc.c PutFilterHeader
/// Filtering parameters header.
#[expect(clippy::cast_sign_loss, reason = "C passes the int fields as uint32_t")]
fn put_filter_header(bw: &mut VP8BitWriter, hdr: &VP8EncFilterHeader) {
    let use_lf_delta = hdr.i4x4_lf_delta != 0;
    vp8_put_bit_uniform(bw, hdr.simple != 0);
    vp8_put_bits(bw, hdr.level as u32, 6);
    vp8_put_bits(bw, hdr.sharpness as u32, 3);
    if vp8_put_bit_uniform(bw, use_lf_delta) {
        // '0' is the default value for i4x4_lf_delta at frame #0.
        let need_update = hdr.i4x4_lf_delta != 0;
        if vp8_put_bit_uniform(bw, need_update) {
            // we don't use ref_lf_delta => emit four 0 bits
            vp8_put_bits(bw, 0, 4);
            // we use mode_lf_delta for i4x4
            vp8_put_signed_bits(bw, hdr.i4x4_lf_delta, 6);
            vp8_put_bits(bw, 0, 3); // all others unused
        }
    }
}

// upstream: src/enc/syntax_enc.c PutQuant
/// Nominal quantization parameters.
#[expect(clippy::cast_sign_loss, reason = "C passes base_quant as uint32_t")]
fn put_quant(bw: &mut VP8BitWriter, enc: &VP8Encoder) {
    vp8_put_bits(bw, enc.base_quant as u32, 7);
    vp8_put_signed_bits(bw, enc.dq_y1_dc, 4);
    vp8_put_signed_bits(bw, enc.dq_y2_dc, 4);
    vp8_put_signed_bits(bw, enc.dq_y2_ac, 4);
    vp8_put_signed_bits(bw, enc.dq_uv_dc, 4);
    vp8_put_signed_bits(bw, enc.dq_uv_ac, 4);
}

// upstream: src/enc/syntax_enc.c EmitPartitionsSize
/// Partition sizes.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C stores the low bytes of the partition sizes"
)]
fn emit_partitions_size(
    enc: &VP8Encoder,
    pic: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    let mut buf = [0_u8; 3 * (MAX_NUM_PARTITIONS as usize - 1)];
    let mut p = 0;
    while p < idx(enc.num_parts) - 1 {
        let part_size = vp8_bit_writer_size(&enc.parts[p]);
        if part_size >= idx(VP8_MAX_PARTITION_SIZE) {
            return Err(webp_encoding_set_error(
                pic,
                WebPEncodingError::VP8_ENC_ERROR_PARTITION_OVERFLOW,
            ));
        }
        buf[3 * p] = (part_size & 0xff) as u8;
        buf[3 * p + 1] = ((part_size >> 8) & 0xff) as u8;
        buf[3 * p + 2] = ((part_size >> 16) & 0xff) as u8;
        p += 1;
    }
    if p != 0 && !write(pic, &buf[..3 * p]) {
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE,
        ));
    }
    Ok(())
}

//------------------------------------------------------------------------------

// upstream: src/enc/syntax_enc.c GeneratePartition0
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C converts the bit positions and sizes to int and size_t"
)]
fn generate_partition0(
    enc: &mut VP8Encoder,
    pic: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    let mb_size = enc.mb_w * enc.mb_h;

    let pos1 = vp8_bit_writer_pos(&enc.bw);
    if !vp8_bit_writer_init(&mut enc.bw, (mb_size * 7 / 8) as usize) {
        // ~7 bits per macroblock
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    }
    let mut bw = std::mem::take(&mut enc.bw);
    vp8_put_bit_uniform(&mut bw, false); // colorspace
    vp8_put_bit_uniform(&mut bw, false); // clamp type

    put_segment_header(&mut bw, enc);
    put_filter_header(&mut bw, &enc.filter_hdr);
    vp8_put_bits(
        &mut bw,
        match enc.num_parts {
            8 => 3,
            4 => 2,
            2 => 1,
            _ => 0,
        },
        2,
    );
    put_quant(&mut bw, enc);
    vp8_put_bit_uniform(&mut bw, false); // no proba update
    vp8_write_probas(&mut bw, &enc.proba);
    let pos2 = vp8_bit_writer_pos(&bw);
    enc.bw = bw;
    vp8_code_intra_modes(enc);
    vp8_bit_writer_finish(&mut enc.bw);

    let pos3 = vp8_bit_writer_pos(&enc.bw);

    if let Some(stats) = pic.stats.as_deref_mut() {
        stats.header_bytes[0] = ((pos2 - pos1 + 7) >> 3) as i32;
        stats.header_bytes[1] = ((pos3 - pos2 + 7) >> 3) as i32;
        stats.alpha_data_size = enc.alpha_data_size as i32;
    }
    if enc.bw.error {
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    }
    Ok(())
}

// upstream: src/enc/syntax_enc.c VP8EncFreeBitWriters
/// Release memory allocated for bit-writing in `VP8EncLoop` & seq.
pub fn vp8_enc_free_bit_writers(enc: &mut VP8Encoder) {
    vp8_bit_writer_wipe_out(&mut enc.bw);
    for p in 0..idx(enc.num_parts) {
        vp8_bit_writer_wipe_out(&mut enc.parts[p]);
    }
}

// upstream: src/enc/syntax_enc.c VP8EncWrite
/// Generates the final bitstream by coding the partition0 and headers, and
/// appending an assembly of all the pre-coded token partitions.
///
/// # Errors
///
/// Returns the code that the picture records.
#[allow(clippy::needless_range_loop, reason = "the C index loops")]
pub fn vp8_enc_write(
    enc: &mut VP8Encoder,
    pic: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    let task_percent = 19;
    let percent_per_part = task_percent / enc.num_parts;
    let final_percent = enc.percent + task_percent;

    // Partition #0 with header and partition sizes
    generate_partition0(enc, pic)?;
    let mut ok = true;

    // Compute VP8 size
    let mut vp8_size =
        idx(VP8_FRAME_HEADER_SIZE) + vp8_bit_writer_size(&enc.bw) + 3 * (idx(enc.num_parts) - 1);
    for p in 0..idx(enc.num_parts) {
        vp8_size += vp8_bit_writer_size(&enc.parts[p]);
    }
    let pad = vp8_size & 1;
    vp8_size += pad;

    // Compute RIFF size
    // At the minimum it is: "WEBPVP8 nnnn" + VP8 data size.
    let mut riff_size = idx(TAG_SIZE) + idx(CHUNK_HEADER_SIZE) + vp8_size;
    if is_vp8x_needed(enc) {
        // Add size for: VP8X header + data.
        riff_size += idx(CHUNK_HEADER_SIZE) + idx(VP8X_CHUNK_SIZE);
    }
    if enc.has_alpha {
        // Add size for: ALPH header + data.
        let padded_alpha_size = enc.alpha_data_size + (enc.alpha_data_size & 1);
        riff_size += idx(CHUNK_HEADER_SIZE) + padded_alpha_size as usize;
    }
    // RIFF size should fit in 32-bits.
    if riff_size > 0xffff_fffe {
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_FILE_TOO_BIG,
        ));
    }

    // Emit headers and partition #0
    {
        let size0 = vp8_bit_writer_size(&enc.bw);
        ok = ok
            && put_webp_headers(enc, pic, size0, vp8_size, riff_size).is_ok()
            && write(pic, &enc.bw.buf[..size0])
            && emit_partitions_size(enc, pic).is_ok();
        vp8_bit_writer_wipe_out(&mut enc.bw); // will free the internal buffer.
    }

    // Token partitions
    for p in 0..idx(enc.num_parts) {
        let size = vp8_bit_writer_size(&enc.parts[p]);
        if size != 0 {
            ok = ok && write(pic, &enc.parts[p].buf[..size]);
        }
        vp8_bit_writer_wipe_out(&mut enc.parts[p]); // will free the internal buffer.
        ok = ok
            && webp_report_progress(pic, enc.percent + percent_per_part, Some(&mut enc.percent))
                .is_ok();
    }

    // Padding byte
    if ok && pad != 0 {
        ok = put_padding_byte(pic);
    }

    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        reason = "C converts the size to int"
    )]
    {
        enc.coded_size = (idx(CHUNK_HEADER_SIZE) + riff_size) as i32;
    }
    ok = ok && webp_report_progress(pic, final_percent, Some(&mut enc.percent)).is_ok();
    if !ok {
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE,
        ));
    }
    Ok(())
}
