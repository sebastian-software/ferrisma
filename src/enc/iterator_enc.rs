// libwebp v1.6.0, src/enc/iterator_enc.c:
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
// VP8Iterator: block iterator
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/iterator_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).

use crate::dsp::dsp::BPS;
use crate::enc::picture_enc::{offset_by, yuva_planes_mut, yuva_samples};
use crate::enc::quant_enc::VP8_SCAN;
use crate::enc::vp8i_enc::{ModeRows, U_OFF_ENC, V_OFF_ENC, VP8EncIterator, VP8Encoder, Y_OFF_ENC};
use crate::enc::webp_enc::webp_report_progress;
use crate::webp::encode::{WebPConfig, WebPEncodingError, WebPPicture};

/// `BPS` as an index stride.
const BPS_U: usize = BPS as usize;

/// Converts a non-negative C `int` to an index.
#[expect(clippy::cast_sign_loss, reason = "the value is non-negative")]
const fn idx(v: i32) -> usize {
    v as usize
}

//------------------------------------------------------------------------------
// VP8Iterator
//------------------------------------------------------------------------------

// upstream: src/enc/iterator_enc.c InitLeft
fn init_left(it: &mut VP8EncIterator) {
    let v = if it.y > 0 { 129 } else { 127 };
    it.yuv_left_mem[it.y_left - 1] = v;
    it.yuv_left_mem[it.u_left - 1] = v;
    it.yuv_left_mem[it.v_left - 1] = v;
    it.yuv_left_mem[it.y_left..it.y_left + 16].fill(129);
    it.yuv_left_mem[it.u_left..it.u_left + 8].fill(129);
    it.yuv_left_mem[it.v_left..it.v_left + 8].fill(129);
    it.left_nz[8] = 0;
    if it.top_derr {
        it.left_derr = [[0; 2]; 2];
    }
}

// upstream: src/enc/iterator_enc.c InitTop
fn init_top(enc: &mut VP8Encoder) {
    let top_size = idx(enc.mb_w) * 16;
    enc.y_top[..2 * top_size].fill(127);
    let mb_w = idx(enc.mb_w);
    enc.nz[1..=mb_w].fill(0);
    if let Some(top_derr) = enc.top_derr.as_mut() {
        top_derr[..mb_w].fill([[0; 2]; 2]);
    }
}

// upstream: src/enc/iterator_enc.c VP8IteratorSetRow
/// Reset iterator position to row 'y'.
pub fn vp8_iterator_set_row(it: &mut VP8EncIterator, enc: &VP8Encoder, y: i32) {
    it.x = 0;
    it.y = y;
    it.bw = idx(y & (enc.num_parts - 1));
    it.preds = enc.preds_base() + idx(y * 4 * enc.preds_w);
    it.nz = 1;
    it.mb = idx(y * enc.mb_w);
    it.y_top = 0;
    it.uv_top = enc.uv_top;
    it.top_in_tmp_32 = false;
    init_left(it);
}

// upstream: src/enc/iterator_enc.c VP8IteratorReset
/// restart a scan
fn vp8_iterator_reset(it: &mut VP8EncIterator, enc: &mut VP8Encoder) {
    vp8_iterator_set_row(it, enc, 0);
    vp8_iterator_set_count_down(it, enc.mb_w * enc.mb_h); // default
    init_top(enc);
    it.bit_count = [[0; 3]; 4];
    it.do_trellis = false;
}

// upstream: src/enc/iterator_enc.c VP8IteratorSetCountDown
/// Set count down (=number of iterations to go).
pub fn vp8_iterator_set_count_down(it: &mut VP8EncIterator, count_down: i32) {
    it.count_down = count_down;
    it.count_down0 = count_down;
}

// upstream: src/enc/iterator_enc.c VP8IteratorIsDone
/// Return true if iteration is finished.
#[must_use]
pub fn vp8_iterator_is_done(it: &VP8EncIterator) -> bool {
    it.count_down <= 0
}

// upstream: src/enc/iterator_enc.c VP8IteratorInit
/// Must be called first. C fills the caller's iterator; the buffers of `it` are
/// reused, as C reuses the iterator's own memory.
pub fn vp8_iterator_init(enc: &mut VP8Encoder, it: &mut VP8EncIterator) {
    // C: yuv_in, yuv_out, yuv_out2 and yuv_p are carved from yuv_mem; they are
    // the iterator's own buffers here.
    it.lf_stats = enc.lf_stats.is_some();
    it.percent0 = enc.percent;
    // C: y_left = WEBP_ALIGN(yuv_left_mem + 1), u_left = y_left + 16 + 16,
    // v_left = u_left + 16.
    it.y_left = 1;
    it.u_left = it.y_left + 16 + 16;
    it.v_left = it.u_left + 16;
    it.top_derr = enc.top_derr.is_some();
    vp8_iterator_reset(it, enc);
}

// upstream: src/enc/iterator_enc.c VP8IteratorProgress
/// Report progression based on macroblock rows. C reports through
/// `WebPReportProgress(enc->pic, percent, &enc->percent)`; `percent_store` is the
/// encoder's `percent`.
///
/// # Errors
///
/// Returns the code that the picture records when the hook asks for an abort.
pub fn vp8_iterator_progress(
    it: &VP8EncIterator,
    pic: &mut WebPPicture<'_>,
    percent_store: &mut i32,
    delta: i32,
) -> Result<(), WebPEncodingError> {
    if delta != 0 && pic.progress_hook.is_some() {
        let done = it.count_down0 - it.count_down;
        let percent = if it.count_down0 <= 0 {
            it.percent0
        } else {
            it.percent0 + delta * done / it.count_down0
        };
        return webp_report_progress(pic, percent, Some(percent_store));
    }
    Ok(())
}

//------------------------------------------------------------------------------
// Import the source samples into the cache. Takes care of replicating
// boundary pixels if necessary.

// upstream: src/enc/iterator_enc.c MinSize
fn min_size(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

// upstream: src/enc/iterator_enc.c ImportBlock
fn import_block(
    src: &[u8],
    src_pos: usize,
    src_stride: i32,
    dst: &mut [u8],
    w: i32,
    h: i32,
    size: i32,
) {
    let (w, h, size) = (idx(w), idx(h), idx(size));
    let mut src_pos = src_pos;
    let mut d = 0;
    for _ in 0..h {
        dst[d..d + w].copy_from_slice(&src[src_pos..src_pos + w]);
        if w < size {
            let v = dst[d + w - 1];
            dst[d + w..d + size].fill(v);
        }
        d += BPS_U;
        src_pos = offset_by(src_pos, i64::from(src_stride));
    }
    for _ in h..size {
        dst.copy_within(d - BPS_U..d - BPS_U + size, d);
        d += BPS_U;
    }
}

// upstream: src/enc/iterator_enc.c ImportLine
fn import_line(
    src: &[u8],
    src_pos: usize,
    src_stride: i32,
    dst: &mut [u8],
    len: i32,
    total_len: i32,
) {
    let (len, total_len) = (idx(len), idx(total_len));
    let mut src_pos = src_pos;
    let mut i = 0;
    while i < len {
        dst[i] = src[src_pos];
        i += 1;
        src_pos = offset_by(src_pos, i64::from(src_stride));
    }
    while i < total_len {
        dst[i] = dst[len - 1];
        i += 1;
    }
}

// upstream: src/enc/iterator_enc.c VP8IteratorImport
/// Import uncompressed samples from source. If `tmp_32` is set, import boundary
/// samples too, into the iterator's 32-byte scratch (C: a non-`NULL` `tmp_32`).
///
/// # Panics
///
/// Panics when the picture has no Y, U or V plane, which C dereferences.
#[expect(clippy::too_many_lines, reason = "the C function translated whole")]
#[expect(clippy::similar_names, reason = "the C names usrc and vsrc")]
pub fn vp8_iterator_import(it: &mut VP8EncIterator, pic: &WebPPicture<'_>, tmp_32: bool) {
    let x = it.x;
    let y = it.y;
    let y_plane = yuva_samples(&pic.yuva, pic.y.expect("the picture has a Y plane"));
    let u_plane = yuva_samples(&pic.yuva, pic.u.expect("the picture has a U plane"));
    let v_plane = yuva_samples(&pic.yuva, pic.v.expect("the picture has a V plane"));
    let ysrc = offset_by(
        0,
        (i64::from(y) * i64::from(pic.y_stride) + i64::from(x)) * 16,
    );
    let usrc = offset_by(
        0,
        (i64::from(y) * i64::from(pic.uv_stride) + i64::from(x)) * 8,
    );
    let vsrc = usrc;
    let w = min_size(pic.width - x * 16, 16);
    let h = min_size(pic.height - y * 16, 16);
    let uv_w = (w + 1) >> 1;
    let uv_h = (h + 1) >> 1;

    import_block(
        y_plane,
        ysrc,
        pic.y_stride,
        &mut it.yuv_in[idx(Y_OFF_ENC)..],
        w,
        h,
        16,
    );
    import_block(
        u_plane,
        usrc,
        pic.uv_stride,
        &mut it.yuv_in[idx(U_OFF_ENC)..],
        uv_w,
        uv_h,
        8,
    );
    import_block(
        v_plane,
        vsrc,
        pic.uv_stride,
        &mut it.yuv_in[idx(V_OFF_ENC)..],
        uv_w,
        uv_h,
        8,
    );

    if !tmp_32 {
        return;
    }

    // Import source (uncompressed) samples into boundary.
    if x == 0 {
        init_left(it);
    } else {
        if y == 0 {
            it.yuv_left_mem[it.y_left - 1] = 127;
            it.yuv_left_mem[it.u_left - 1] = 127;
            it.yuv_left_mem[it.v_left - 1] = 127;
        } else {
            it.yuv_left_mem[it.y_left - 1] = y_plane[offset_by(ysrc, -1 - i64::from(pic.y_stride))];
            it.yuv_left_mem[it.u_left - 1] =
                u_plane[offset_by(usrc, -1 - i64::from(pic.uv_stride))];
            it.yuv_left_mem[it.v_left - 1] =
                v_plane[offset_by(vsrc, -1 - i64::from(pic.uv_stride))];
        }
        let (y_left, u_left, v_left) = (it.y_left, it.u_left, it.v_left);
        import_line(
            y_plane,
            ysrc - 1,
            pic.y_stride,
            &mut it.yuv_left_mem[y_left..],
            h,
            16,
        );
        import_line(
            u_plane,
            usrc - 1,
            pic.uv_stride,
            &mut it.yuv_left_mem[u_left..],
            uv_h,
            8,
        );
        import_line(
            v_plane,
            vsrc - 1,
            pic.uv_stride,
            &mut it.yuv_left_mem[v_left..],
            uv_h,
            8,
        );
    }

    it.y_top = 0;
    it.uv_top = 16;
    it.top_in_tmp_32 = true;
    if y == 0 {
        it.tmp_32.fill(127);
    } else {
        import_line(
            y_plane,
            offset_by(ysrc, -i64::from(pic.y_stride)),
            1,
            &mut it.tmp_32[..],
            w,
            16,
        );
        import_line(
            u_plane,
            offset_by(usrc, -i64::from(pic.uv_stride)),
            1,
            &mut it.tmp_32[16..],
            uv_w,
            8,
        );
        import_line(
            v_plane,
            offset_by(vsrc, -i64::from(pic.uv_stride)),
            1,
            &mut it.tmp_32[16 + 8..],
            uv_w,
            8,
        );
    }
}

//------------------------------------------------------------------------------
// Copy back the compressed samples into user space if requested.

// upstream: src/enc/iterator_enc.c ExportBlock
fn export_block(src: &[u8], dst: &mut [u8], dst_pos: usize, dst_stride: i32, w: i32, h: i32) {
    let w = idx(w);
    let mut dst_pos = dst_pos;
    for row in 0..idx(h) {
        dst[dst_pos..dst_pos + w].copy_from_slice(&src[row * BPS_U..row * BPS_U + w]);
        if row + 1 < idx(h) {
            dst_pos = offset_by(dst_pos, i64::from(dst_stride));
        }
    }
}

// upstream: src/enc/iterator_enc.c VP8IteratorExport
/// Export decimated samples into the picture when `show_compressed` is set.
///
/// # Panics
///
/// Panics when the picture has no Y, U or V plane, or when its planes are a
/// shared view, which C writes through its cast-away `const`.
#[expect(clippy::similar_names, reason = "the C names of the planes")]
pub fn vp8_iterator_export(it: &VP8EncIterator, config: &WebPConfig, pic: &mut WebPPicture<'_>) {
    if config.show_compressed != 0 {
        let x = it.x;
        let y = it.y;
        let ysrc = &it.yuv_out[idx(Y_OFF_ENC)..];
        let usrc = &it.yuv_out[idx(U_OFF_ENC)..];
        let vsrc = &it.yuv_out[idx(V_OFF_ENC)..];
        let ydst = offset_by(
            0,
            (i64::from(y) * i64::from(pic.y_stride) + i64::from(x)) * 16,
        );
        let udst = offset_by(
            0,
            (i64::from(y) * i64::from(pic.uv_stride) + i64::from(x)) * 8,
        );
        let vdst = udst;
        let mut w = pic.width - x * 16;
        let mut h = pic.height - y * 16;
        let (y_stride, uv_stride) = (pic.y_stride, pic.uv_stride);

        if w > 16 {
            w = 16;
        }
        if h > 16 {
            h = 16;
        }

        let [y_plane, u_plane, v_plane, _] =
            yuva_planes_mut(&mut pic.yuva, [pic.y, pic.u, pic.v, None]);
        let y_plane = y_plane.expect("the picture has a Y plane");
        let u_plane = u_plane.expect("the picture has a U plane");
        let v_plane = v_plane.expect("the picture has a V plane");

        // Luma plane
        export_block(ysrc, y_plane, ydst, y_stride, w, h);

        {
            // U/V planes
            let uv_w = (w + 1) >> 1;
            let uv_h = (h + 1) >> 1;
            export_block(usrc, u_plane, udst, uv_stride, uv_w, uv_h);
            export_block(vsrc, v_plane, vdst, uv_stride, uv_w, uv_h);
        }
    }
}

//------------------------------------------------------------------------------
// Non-zero contexts setup/teardown

// Nz bits:
//  0  1  2  3  Y
//  4  5  6  7
//  8  9 10 11
// 12 13 14 15
// 16 17        U
// 18 19
// 20 21        V
// 22 23
// 24           DC-intra16

// upstream: src/enc/iterator_enc.c BIT
/// Convert packed context to byte array.
fn bit(nz: u32, n: u32) -> i32 {
    i32::from(nz & (1 << n) != 0)
}

// upstream: src/enc/iterator_enc.c VP8IteratorNzToBytes
/// Non-zero context setup.
pub fn vp8_iterator_nz_to_bytes(it: &mut VP8EncIterator, enc: &VP8Encoder) {
    let tnz = enc.nz[it.nz];
    let lnz = enc.nz[it.nz - 1];
    let top_nz = &mut it.top_nz;
    let left_nz = &mut it.left_nz;

    // Top-Y
    top_nz[0] = bit(tnz, 12);
    top_nz[1] = bit(tnz, 13);
    top_nz[2] = bit(tnz, 14);
    top_nz[3] = bit(tnz, 15);
    // Top-U
    top_nz[4] = bit(tnz, 18);
    top_nz[5] = bit(tnz, 19);
    // Top-V
    top_nz[6] = bit(tnz, 22);
    top_nz[7] = bit(tnz, 23);
    // DC
    top_nz[8] = bit(tnz, 24);

    // left-Y
    left_nz[0] = bit(lnz, 3);
    left_nz[1] = bit(lnz, 7);
    left_nz[2] = bit(lnz, 11);
    left_nz[3] = bit(lnz, 15);
    // left-U
    left_nz[4] = bit(lnz, 17);
    left_nz[5] = bit(lnz, 19);
    // left-V
    left_nz[6] = bit(lnz, 21);
    left_nz[7] = bit(lnz, 23);
    // left-DC is special, iterated separately
}

// upstream: src/enc/iterator_enc.c VP8IteratorBytesToNz
/// Non-zero context teardown.
#[expect(clippy::cast_sign_loss, reason = "the contexts are 0 or 1")]
pub fn vp8_iterator_bytes_to_nz(it: &VP8EncIterator, enc: &mut VP8Encoder) {
    let mut nz: u32 = 0;
    let top_nz = &it.top_nz;
    let left_nz = &it.left_nz;
    // top
    nz |= ((top_nz[0] << 12) | (top_nz[1] << 13)) as u32;
    nz |= ((top_nz[2] << 14) | (top_nz[3] << 15)) as u32;
    nz |= ((top_nz[4] << 18) | (top_nz[5] << 19)) as u32;
    nz |= ((top_nz[6] << 22) | (top_nz[7] << 23)) as u32;
    nz |= (top_nz[8] << 24) as u32; // we propagate the _top_ bit, esp. for intra4
    // left
    nz |= ((left_nz[0] << 3) | (left_nz[1] << 7)) as u32;
    nz |= (left_nz[2] << 11) as u32;
    nz |= ((left_nz[4] << 17) | (left_nz[6] << 21)) as u32;

    enc.nz[it.nz] = nz;
}

//------------------------------------------------------------------------------
// Advance to the next position, doing the bookkeeping.

/// The top samples of the iterator from position `pos`: the encoder's, or the
/// scratch of the analysis (C: the target of `it->y_top` and `it->uv_top`).
pub(crate) fn top_samples<'a>(it: &'a VP8EncIterator, enc: &'a VP8Encoder, pos: usize) -> &'a [u8] {
    if it.top_in_tmp_32 {
        &it.tmp_32[pos..]
    } else {
        &enc.y_top[pos..]
    }
}

// upstream: src/enc/iterator_enc.c VP8IteratorSaveBoundary
/// Save the `yuv_out` boundary values to 'top'/'left' arrays for next
/// iterations.
pub fn vp8_iterator_save_boundary(it: &mut VP8EncIterator, enc: &mut VP8Encoder) {
    let x = it.x;
    let y = it.y;
    let ysrc = idx(Y_OFF_ENC);
    let uvsrc = idx(U_OFF_ENC);
    if x < enc.mb_w - 1 {
        // left
        for i in 0..16 {
            it.yuv_left_mem[it.y_left + i] = it.yuv_out[ysrc + 15 + i * BPS_U];
        }
        for i in 0..8 {
            it.yuv_left_mem[it.u_left + i] = it.yuv_out[uvsrc + 7 + i * BPS_U];
            it.yuv_left_mem[it.v_left + i] = it.yuv_out[uvsrc + 15 + i * BPS_U];
        }
        // top-left (before 'top'!)
        let y_top = top_samples(it, enc, it.y_top)[15];
        let u_top = top_samples(it, enc, it.uv_top)[7];
        let v_top = top_samples(it, enc, it.uv_top)[8 + 7];
        it.yuv_left_mem[it.y_left - 1] = y_top;
        it.yuv_left_mem[it.u_left - 1] = u_top;
        it.yuv_left_mem[it.v_left - 1] = v_top;
    }
    if y < enc.mb_h - 1 {
        // top
        let top: &mut [u8] = if it.top_in_tmp_32 {
            &mut it.tmp_32
        } else {
            &mut enc.y_top
        };
        top[it.y_top..it.y_top + 16]
            .copy_from_slice(&it.yuv_out[ysrc + 15 * BPS_U..ysrc + 15 * BPS_U + 16]);
        top[it.uv_top..it.uv_top + 8 + 8]
            .copy_from_slice(&it.yuv_out[uvsrc + 7 * BPS_U..uvsrc + 7 * BPS_U + 8 + 8]);
    }
}

// upstream: src/enc/iterator_enc.c VP8IteratorNext
/// Go to next macroblock. Returns false if not finished.
pub fn vp8_iterator_next(it: &mut VP8EncIterator, enc: &VP8Encoder) -> bool {
    it.x += 1;
    if it.x == enc.mb_w {
        it.y += 1;
        let y = it.y;
        vp8_iterator_set_row(it, enc, y);
    } else {
        it.preds += 4;
        it.mb += 1;
        it.nz += 1;
        it.y_top += 16;
        it.uv_top += 16;
    }
    it.count_down -= 1;
    0 < it.count_down
}

//------------------------------------------------------------------------------
// Helper function to set mode properties

// upstream: src/enc/iterator_enc.c VP8SetIntra16Mode
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C memsets the low byte of the mode"
)]
pub fn vp8_set_intra16_mode(it: &VP8EncIterator, rows: &mut ModeRows<'_>, mode: i32) {
    let preds_w = idx(rows.preds_w);
    let preds = rows.preds_at(it.preds);
    for y in 0..4 {
        preds[y * preds_w..y * preds_w + 4].fill(mode as u8);
    }
    rows.mb(it.mb).r#type = 1;
}

// upstream: src/enc/iterator_enc.c VP8SetIntra4Mode
pub fn vp8_set_intra4_mode(it: &VP8EncIterator, rows: &mut ModeRows<'_>, modes: &[u8; 16]) {
    let preds_w = idx(rows.preds_w);
    let preds = rows.preds_at(it.preds);
    for y in 0..4 {
        preds[y * preds_w..y * preds_w + 4].copy_from_slice(&modes[4 * y..4 * y + 4]);
    }
    rows.mb(it.mb).r#type = 0;
}

// upstream: src/enc/iterator_enc.c VP8SetIntraUVMode
#[expect(clippy::cast_sign_loss, reason = "the 2-bit field keeps the low bits")]
pub fn vp8_set_intra_uv_mode(it: &VP8EncIterator, rows: &mut ModeRows<'_>, mode: i32) {
    rows.mb(it.mb).uv_mode = (mode & 3) as u8;
}

// upstream: src/enc/iterator_enc.c VP8SetSkip
#[expect(clippy::cast_sign_loss, reason = "the 1-bit field keeps the low bit")]
pub fn vp8_set_skip(it: &VP8EncIterator, rows: &mut ModeRows<'_>, skip: i32) {
    rows.mb(it.mb).skip = (skip & 1) as u8;
}

// upstream: src/enc/iterator_enc.c VP8SetSegment
#[expect(clippy::cast_sign_loss, reason = "the 2-bit field keeps the low bits")]
pub fn vp8_set_segment(it: &VP8EncIterator, rows: &mut ModeRows<'_>, segment: i32) {
    rows.mb(it.mb).segment = (segment & 3) as u8;
}

//------------------------------------------------------------------------------
// Intra4x4 sub-blocks iteration
//
//  We store and update the boundary samples into an array of 37 pixels. They
//  are updated as we iterate and reconstructs each intra4x4 blocks in turn.
//  The position of the samples has the following snake pattern:
//
// 16|17 18 19 20|21 22 23 24|25 26 27 28|29 30 31 32|33 34 35 36  <- Top-right
// --+-----------+-----------+-----------+-----------+
// 15|         19|         23|         27|         31|
// 14|         18|         22|         26|         30|
// 13|         17|         21|         25|         29|
// 12|13 14 15 16|17 18 19 20|21 22 23 24|25 26 27 28|
// --+-----------+-----------+-----------+-----------+
// 11|         15|         19|         23|         27|
// 10|         14|         18|         22|         26|
//  9|         13|         17|         21|         25|
//  8| 9 10 11 12|13 14 15 16|17 18 19 20|21 22 23 24|
// --+-----------+-----------+-----------+-----------+
//  7|         11|         15|         19|         23|
//  6|         10|         14|         18|         22|
//  5|          9|         13|         17|         21|
//  4| 5  6  7  8| 9 10 11 12|13 14 15 16|17 18 19 20|
// --+-----------+-----------+-----------+-----------+
//  3|          7|         11|         15|         19|
//  2|          6|         10|         14|         18|
//  1|          5|          9|         13|         17|
//  0| 1  2  3  4| 5  6  7  8| 9 10 11 12|13 14 15 16|
// --+-----------+-----------+-----------+-----------+

// upstream: src/enc/iterator_enc.c VP8TopLeftI4
/// Array to record the position of the top sample to pass to the prediction
/// functions in dsp.c.
static VP8_TOP_LEFT_I4: [u8; 16] = [17, 21, 25, 29, 13, 17, 21, 25, 9, 13, 17, 21, 5, 9, 13, 17];

// upstream: src/enc/iterator_enc.c VP8IteratorStartI4
/// Intra4x4 iterations.
pub fn vp8_iterator_start_i4(it: &mut VP8EncIterator, enc: &VP8Encoder) {
    it.i4 = 0; // first 4x4 sub-block
    it.i4_top = usize::from(VP8_TOP_LEFT_I4[0]);

    // Import the boundary samples
    for i in 0..17 {
        // left
        it.i4_boundary[i] = it.yuv_left_mem[it.y_left + 15 - i];
    }
    let y_top = top_samples(it, enc, it.y_top);
    let mut top = [0_u8; 20];
    for (i, sample) in top.iter_mut().enumerate().take(16) {
        // top
        *sample = y_top[i];
    }
    // top-right samples have a special case on the far right of the picture
    let far_right = it.x >= enc.mb_w - 1;
    if !far_right {
        top[16..16 + 4].copy_from_slice(&y_top[16..16 + 4]);
    }
    it.i4_boundary[17..17 + 16].copy_from_slice(&top[..16]);
    if far_right {
        // else, replicate the last valid pixel four times
        for i in 16..16 + 4 {
            it.i4_boundary[17 + i] = it.i4_boundary[17 + 15];
        }
    } else {
        it.i4_boundary[17 + 16..17 + 20].copy_from_slice(&top[16..20]);
    }
    vp8_iterator_nz_to_bytes(it, enc); // import the non-zero context
}

// upstream: src/enc/iterator_enc.c VP8IteratorRotateI4
/// Returns true if not done.
#[expect(clippy::if_not_else, reason = "the C condition form")]
pub fn vp8_iterator_rotate_i4(it: &mut VP8EncIterator, yuv_out: &[u8]) -> bool {
    let blk = &yuv_out[usize::from(VP8_SCAN[idx(it.i4)])..];
    let top = it.i4_top;

    // Update the cache with 7 fresh samples
    for i in 0..=3 {
        it.i4_boundary[top - 4 + i] = blk[i + 3 * BPS_U]; // store future top samples
    }
    if (it.i4 & 3) != 3 {
        // if not on the right sub-blocks #3, #7, #11, #15
        for i in 0..=2 {
            // store future left samples
            it.i4_boundary[top + i] = blk[3 + (2 - i) * BPS_U];
        }
    } else {
        // else replicate top-right samples, as says the specs.
        for i in 0..=3 {
            it.i4_boundary[top + i] = it.i4_boundary[top + i + 4];
        }
    }
    // move pointers to next sub-block
    it.i4 += 1;
    if it.i4 == 16 {
        // we're done
        return false;
    }

    it.i4_top = usize::from(VP8_TOP_LEFT_I4[idx(it.i4)]);
    true
}
