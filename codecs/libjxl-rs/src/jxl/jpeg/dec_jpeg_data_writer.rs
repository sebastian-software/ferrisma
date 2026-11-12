// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Mechanical WriteJpeg port from lib/jxl/jpeg/dec_jpeg_data_writer.cc.
//! Explicit queue reborrows replace its bit-writer output pointer. Huffman and
//! padding pointers use owner-relative indices; buffers and JPEG payloads retain
//! their original owned/borrowed roles. Internal source programmer preconditions
//! remain required. No JPEG parser or Brotli decoder is substituted here.

use super::dec_jpeg_output_chunk::OutputChunk;
use super::dec_jpeg_serialization_state::*;
use super::jpeg_data::*;
use crate::jxl::base::status::{Status, StatusCode};
use std::collections::VecDeque;

const CHUNK_SIZE: usize = 16384;
type Queue<'a> = VecDeque<OutputChunk<'a>>;
#[derive(PartialEq)]
pub enum SerializationStatus {
    NeedsMoreInput,
    NeedsMoreOutput,
    Error,
    Done,
}

fn has_zero_byte(x: u64) -> u64 {
    x.wrapping_sub(0x0101010101010101) & !x & 0x8080808080808080
}
fn bit_writer_init(bw: &mut JpegBitWriter<'_>) {
    bw.chunk = Some(OutputChunk::owned(CHUNK_SIZE));
    bw.pos = 0;
    bw.put_buffer = 0;
    bw.put_bits = 64;
    bw.healthy = true;
}
#[inline(never)]
fn swap_buffer<'a>(bw: &mut JpegBitWriter<'a>, queue: &mut Queue<'a>) {
    let mut chunk = bw.chunk.take().unwrap();
    chunk.len = bw.pos;
    queue.push_back(chunk);
    bw.chunk = Some(OutputChunk::owned(CHUNK_SIZE));
    bw.pos = 0;
}
fn reserve<'a>(bw: &mut JpegBitWriter<'a>, queue: &mut Queue<'a>, n: usize) {
    if bw.pos + n > CHUNK_SIZE {
        swap_buffer(bw, queue);
    }
}
fn emit_byte(bw: &mut JpegBitWriter<'_>, byte: i32) {
    let data = bw.chunk.as_mut().unwrap().buffer_mut();
    data[bw.pos] = byte as u8;
    data[bw.pos + 1] = 0;
    bw.pos += if byte != 0xff { 1 } else { 2 };
}
fn discharge_bit_buffer(bw: &mut JpegBitWriter<'_>, bits: u64) {
    bw.put_buffer |= bits >> -bw.put_bits;
    if has_zero_byte(!bw.put_buffer) != 0 {
        for shift in [56, 48, 40, 32, 24, 16, 8, 0] {
            emit_byte(bw, ((bw.put_buffer >> shift) & 0xff) as i32);
        }
    } else {
        bw.chunk.as_mut().unwrap().buffer_mut()[bw.pos..bw.pos + 8]
            .copy_from_slice(&bw.put_buffer.to_be_bytes());
        bw.pos += 8;
    }
    bw.put_bits += 64;
    bw.put_buffer = bits << bw.put_bits;
}
fn write_bits(bw: &mut JpegBitWriter<'_>, nbits: i32, bits: u64) {
    debug_assert!(nbits > 0);
    bw.put_bits -= nbits;
    if bw.put_bits < 0 {
        if nbits > 64 {
            bw.put_bits += nbits;
            bw.healthy = false;
        } else {
            discharge_bit_buffer(bw, bits);
        }
    } else {
        bw.put_buffer |= bits << bw.put_bits;
    }
}
fn emit_marker<'a>(bw: &mut JpegBitWriter<'a>, queue: &mut Queue<'a>, marker: i32) {
    reserve(bw, queue, 2);
    debug_assert_ne!(marker, 0xff);
    let data = bw.chunk.as_mut().unwrap().buffer_mut();
    data[bw.pos] = 0xff;
    data[bw.pos + 1] = marker as u8;
    bw.pos += 2;
}
fn jump_to_byte_boundary<'a>(
    bw: &mut JpegBitWriter<'a>,
    queue: &mut Queue<'a>,
    pad_bits: &mut Option<usize>,
    padding: &[u8],
) -> bool {
    let n_bits = (bw.put_bits & 7) as usize;
    let mut dangling = 0;
    let mut pattern;
    if let Some(start) = *pad_bits {
        pattern = 0;
        let mut src = start;
        for _ in 0..n_bits {
            pattern <<= 1;
            if src >= padding.len() {
                return false;
            }
            let bit = padding[src];
            src += 1;
            dangling |= bit;
            pattern |= bit;
        }
        *pad_bits = Some(src);
    } else {
        pattern = ((1u32 << n_bits) - 1) as u8;
    }
    if dangling & !1 != 0 {
        return false;
    }
    reserve(bw, queue, 16);
    while bw.put_bits <= 56 {
        emit_byte(bw, ((bw.put_buffer >> 56) & 0xff) as i32);
        bw.put_buffer <<= 8;
        bw.put_bits += 8;
    }
    if bw.put_bits < 64 {
        let mask = 0xff >> (64 - bw.put_bits);
        let c = (((bw.put_buffer >> 56) as i32) & !mask) | pattern as i32;
        emit_byte(bw, c);
    }
    bw.put_buffer = 0;
    bw.put_bits = 64;
    true
}
fn bit_writer_finish<'a>(bw: &mut JpegBitWriter<'a>, queue: &mut Queue<'a>) {
    if bw.pos == 0 {
        return;
    }
    let mut chunk = bw.chunk.take().unwrap();
    chunk.len = bw.pos;
    queue.push_back(chunk);
    bw.pos = 0;
}
fn coding_state_init(s: &mut DctCodingState) {
    s.eob_run = 0;
    s.cur_ac_huff = None;
    s.refinement_bits.clear();
    // std::vector::reserve specifies total capacity, rather than additional.
    if s.refinement_bits.capacity() < 64 {
        s.refinement_bits.reserve(64);
    }
}
fn write_symbol(symbol: i32, table: &HuffmanCodeTable, bw: &mut JpegBitWriter<'_>) {
    write_bits(
        bw,
        table.depth[symbol as usize] as i32,
        table.code[symbol as usize] as u64,
    );
}
fn write_symbol_bits(
    symbol: i32,
    table: &HuffmanCodeTable,
    bw: &mut JpegBitWriter<'_>,
    nbits: i32,
    bits: u64,
) {
    write_bits(
        bw,
        nbits + table.depth[symbol as usize] as i32,
        bits | ((table.code[symbol as usize] as i32) << nbits) as u64,
    );
}
fn log2(x: u32) -> i32 {
    debug_assert_ne!(x, 0);
    (31 - x.leading_zeros()) as i32
}
fn flush<'a>(
    s: &mut DctCodingState,
    bw: &mut JpegBitWriter<'a>,
    queue: &mut Queue<'a>,
    ac: &[HuffmanCodeTable],
) {
    if s.eob_run > 0 {
        reserve(bw, queue, 16);
        let nbits = log2(s.eob_run as u32);
        write_symbol(nbits << 4, &ac[s.cur_ac_huff.unwrap()], bw);
        if nbits > 0 {
            write_bits(bw, nbits, (s.eob_run & ((1 << nbits) - 1)) as u64);
        }
        s.eob_run = 0;
    }
    let num_words = s.refinement_bits_count >> 4;
    let mut i = 0;
    while i < num_words {
        let limit = (i + 124).min(num_words);
        reserve(bw, queue, 512);
        while i < limit {
            write_bits(bw, 16, s.refinement_bits[i] as u64);
            i += 1;
        }
    }
    reserve(bw, queue, 16);
    let tail = s.refinement_bits_count & 15;
    if tail != 0 {
        write_bits(bw, tail as i32, *s.refinement_bits.last().unwrap() as u64);
    }
    s.refinement_bits.clear();
    s.refinement_bits_count = 0;
}
fn buffer_end_of_band<'a>(
    s: &mut DctCodingState,
    ac_index: usize,
    bits: &[i32],
    bw: &mut JpegBitWriter<'a>,
    queue: &mut Queue<'a>,
    ac: &[HuffmanCodeTable],
) {
    if s.eob_run == 0 {
        s.cur_ac_huff = Some(ac_index);
    }
    s.eob_run += 1;
    let mut count = bits.len();
    if count != 0 {
        let mut new_bits = 0u64;
        for &bit in bits {
            new_bits = (new_bits << 1) | bit as u64;
        }
        let tail = s.refinement_bits_count & 15;
        if tail != 0 {
            let stuff = (16 - tail).min(count);
            let value = ((new_bits >> (count - stuff)) as u16) & ((1u32 << stuff) - 1) as u16;
            let back = s.refinement_bits.last_mut().unwrap();
            *back = ((*back as u32) << stuff | value as u32) as u16;
            count -= stuff;
            s.refinement_bits_count += stuff;
        }
        while count >= 16 {
            s.refinement_bits.push((new_bits >> (count - 16)) as u16);
            count -= 16;
            s.refinement_bits_count += 16;
        }
        if count != 0 {
            s.refinement_bits
                .push((new_bits & ((1u64 << count) - 1)) as u16);
            s.refinement_bits_count += count;
        }
    }
    if s.eob_run == 0x7fff {
        flush(s, bw, queue, ac);
    }
}
#[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
fn build_huffman_code_table(huff: &JpegHuffmanCode, table: &mut HuffmanCodeTable) -> bool {
    let mut codes = [0i32; 256];
    let mut sizes = [0u32; 257];
    let mut p = 0;
    for l in 1..=16 {
        let count = huff.counts[l] as usize;
        if p + count > 257 {
            return false;
        }
        for _ in 0..count {
            sizes[p] = l as u32;
            p += 1;
        }
    }
    if p == 0 {
        return true;
    }
    let last = p - 1;
    sizes[last] = 0;
    let mut code = 0;
    let mut si = sizes[0];
    p = 0;
    while sizes[p] != 0 {
        while sizes[p] == si {
            codes[p] = code;
            p += 1;
            code += 1;
        }
        code <<= 1;
        si += 1;
    }
    for p in 0..last {
        let i = huff.values[p] as usize;
        table.depth[i] = sizes[p] as i8;
        table.code[i] = codes[p] as u16;
    }
    true
}
fn encode_sof(jpg: &JpegData, marker: u8, state: &mut SerializationState<'_>) -> bool {
    if marker <= 0xc2 {
        state.is_progressive = marker == 0xc2;
    }
    let n = jpg.components.len();
    let len = 8 + 3 * n;
    state.output_queue.push_back(OutputChunk::owned(len + 2));
    let data = state.output_queue.back_mut().unwrap().buffer_mut();
    data[..10].copy_from_slice(&[
        0xff,
        marker,
        (len >> 8) as u8,
        len as u8,
        8,
        (jpg.height >> 8) as u8,
        jpg.height as u8,
        (jpg.width >> 8) as u8,
        jpg.width as u8,
        n as u8,
    ]);
    let mut pos = 10;
    for c in &jpg.components {
        data[pos] = c.id as u8;
        pos += 1;
        data[pos] = ((c.h_samp_factor << 4) | c.v_samp_factor) as u8;
        pos += 1;
        if c.quant_idx as usize >= jpg.quant.len() {
            return false;
        }
        data[pos] = jpg.quant[c.quant_idx as usize].index as u8;
        pos += 1;
    }
    true
}
fn encode_sos(jpg: &JpegData, scan: &JpegScanInfo, state: &mut SerializationState<'_>) -> bool {
    let n = scan.num_components as usize;
    let len = 6 + 2 * n;
    state.output_queue.push_back(OutputChunk::owned(len + 2));
    let data = state.output_queue.back_mut().unwrap().buffer_mut();
    data[..5].copy_from_slice(&[0xff, 0xda, (len >> 8) as u8, len as u8, n as u8]);
    let mut pos = 5;
    for si in &scan.components[..n] {
        if si.comp_idx as usize >= jpg.components.len() {
            return false;
        }
        data[pos] = jpg.components[si.comp_idx as usize].id as u8;
        pos += 1;
        data[pos] = ((si.dc_tbl_idx << 4) + si.ac_tbl_idx) as u8;
        pos += 1;
    }
    data[pos] = scan.ss as u8;
    data[pos + 1] = scan.se as u8;
    data[pos + 2] = ((scan.ah << 4) | scan.al) as u8;
    true
}
#[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
fn encode_dht(jpg: &JpegData, state: &mut SerializationState<'_>) -> bool {
    let mut len = 2;
    for huff in &jpg.huffman_code[state.dht_index as usize..] {
        for count in huff.counts {
            len += count as usize;
        }
        if len == 2 {
            break;
        }
        len += 16;
        if huff.is_last {
            break;
        }
    }
    state.output_queue.push_back(OutputChunk::owned(len + 2));
    let data = state.output_queue.back_mut().unwrap().buffer_mut();
    data[..4].copy_from_slice(&[0xff, 0xc4, (len >> 8) as u8, len as u8]);
    let mut pos = 4;
    loop {
        let index = state.dht_index as usize;
        state.dht_index += 1;
        if index >= jpg.huffman_code.len() {
            return false;
        }
        let huff = &jpg.huffman_code[index];
        let mut slot = huff.slot_id as usize;
        let mut total = 0;
        let mut max_length = 0;
        for (i, &count) in huff.counts.iter().enumerate() {
            if count != 0 {
                max_length = i;
            }
            total += count as usize;
        }
        if total == 0 {
            break;
        }
        let table = if slot & 0x10 != 0 {
            slot -= 0x10;
            &mut state.ac_huff_table[slot]
        } else {
            &mut state.dc_huff_table[slot]
        };
        table.depth.fill(127);
        if !build_huffman_code_table(huff, table) {
            return false;
        }
        table.initialized = true;
        total -= 1;
        data[pos] = huff.slot_id as u8;
        pos += 1;
        for i in 1..=16 {
            data[pos] = (if i == max_length {
                huff.counts[i] - 1
            } else {
                huff.counts[i]
            }) as u8;
            pos += 1;
        }
        for &value in &huff.values[..total] {
            data[pos] = value as u8;
            pos += 1;
        }
        if huff.is_last {
            break;
        }
    }
    true
}
fn encode_dqt(jpg: &JpegData, state: &mut SerializationState<'_>) -> bool {
    let mut len = 2i32;
    for table in &jpg.quant[state.dqt_index as usize..] {
        len += 1 + (if table.precision != 0 { 2 } else { 1 }) * 64;
        if table.is_last {
            break;
        }
    }
    state
        .output_queue
        .push_back(OutputChunk::owned(len as usize + 2));
    let data = state.output_queue.back_mut().unwrap().buffer_mut();
    data[..4].copy_from_slice(&[0xff, 0xdb, (len >> 8) as u8, len as u8]);
    let mut pos = 4;
    loop {
        let index = state.dqt_index as usize;
        state.dqt_index += 1;
        if index >= jpg.quant.len() {
            return false;
        }
        let table = &jpg.quant[index];
        data[pos] = ((table.precision << 4) + table.index) as u8;
        pos += 1;
        for &i in &JPEG_NATURAL_ORDER[..64] {
            let value = table.values[i];
            if table.precision != 0 {
                data[pos] = (value >> 8) as u8;
                pos += 1;
            }
            data[pos] = value as u8;
            pos += 1;
        }
        if table.is_last {
            break;
        }
    }
    true
}
fn encode_sequential(
    coeffs: &[i16],
    dc: &HuffmanCodeTable,
    ac: &HuffmanCodeTable,
    num_zero_runs: i32,
    last_dc: &mut i16,
    bw: &mut JpegBitWriter<'_>,
) -> bool {
    let mut temp2 = coeffs[0];
    let mut temp = temp2.wrapping_sub(*last_dc);
    *last_dc = temp2;
    temp2 = temp >> 15;
    temp = temp.wrapping_add(temp2);
    temp2 ^= temp;
    let nbits = if temp2 == 0 {
        0
    } else {
        log2(temp2 as u32) + 1
    };
    write_symbol(nbits, dc, bw);
    if nbits != 0 {
        write_bits(bw, nbits, (temp as u32 & ((1u32 << nbits) - 1)) as u64);
    }
    let mut r = 0i16;
    let mut litmus = 0i16;
    for &i in &JPEG_NATURAL_ORDER[1..64] {
        temp = coeffs[i];
        if temp == 0 {
            r += 1;
        } else {
            temp2 = temp >> 15;
            temp = temp.wrapping_add(temp2);
            temp2 ^= temp;
            if r > 15 {
                write_symbol(0xf0, ac, bw);
                r -= 16;
                if r > 15 {
                    write_symbol(0xf0, ac, bw);
                    r -= 16;
                }
                if r > 15 {
                    write_symbol(0xf0, ac, bw);
                    r -= 16;
                }
            }
            litmus |= temp2;
            let nbits = log2(temp2 as u16 as u32) + 1;
            write_symbol_bits(
                ((r as i32) << 4) + nbits,
                ac,
                bw,
                nbits,
                ((temp as i32) & ((1 << nbits) - 1)) as u64,
            );
            r = 0;
        }
    }
    for _ in 0..num_zero_runs {
        write_symbol(0xf0, ac, bw);
        // C++ promotes the compound subtraction to int, then narrows to i16.
        r = r.wrapping_sub(16);
    }
    if r > 0 {
        write_symbol(0, ac, bw);
    }
    litmus >= 0
}
#[allow(clippy::too_many_arguments)]
fn encode_progressive<'a>(
    coeffs: &[i16],
    dc: &HuffmanCodeTable,
    ac_index: usize,
    ac: &[HuffmanCodeTable],
    mut ss: i32,
    se: i32,
    al: i32,
    num_zero_runs: i32,
    coding: &mut DctCodingState,
    last_dc: &mut i16,
    bw: &mut JpegBitWriter<'a>,
    queue: &mut Queue<'a>,
) -> bool {
    let eob_allowed = ss > 0;
    if ss == 0 {
        let value = ((coeffs[0] as i32) >> al) as i16;
        let mut temp = value.wrapping_sub(*last_dc);
        *last_dc = value;
        let mut temp2 = temp;
        if temp < 0 {
            temp = temp.wrapping_neg();
            if temp < 0 {
                return false;
            }
            temp2 = temp2.wrapping_sub(1);
        }
        let nbits = if temp == 0 { 0 } else { log2(temp as u32) + 1 };
        write_symbol(nbits, dc, bw);
        if nbits != 0 {
            write_bits(bw, nbits, ((temp2 as i32) & ((1 << nbits) - 1)) as u64);
        }
        ss += 1;
    }
    if ss > se {
        return true;
    }
    let mut r = 0;
    for k in ss..=se {
        let mut temp = coeffs[JPEG_NATURAL_ORDER[k as usize]];
        if temp == 0 {
            r += 1;
            continue;
        }
        let temp2 = if temp < 0 {
            temp = temp.wrapping_neg();
            if temp < 0 {
                return false;
            }
            temp = ((temp as i32) >> al) as i16;
            !temp
        } else {
            temp = ((temp as i32) >> al) as i16;
            temp
        };
        if temp == 0 {
            r += 1;
            continue;
        }
        flush(coding, bw, queue, ac);
        while r > 15 {
            write_symbol(0xf0, &ac[ac_index], bw);
            r -= 16;
        }
        let nbits = log2(temp as u32) + 1;
        write_symbol((r << 4) + nbits, &ac[ac_index], bw);
        write_bits(bw, nbits, ((temp2 as i32) & ((1 << nbits) - 1)) as u64);
        r = 0;
    }
    if num_zero_runs > 0 {
        flush(coding, bw, queue, ac);
        for _ in 0..num_zero_runs {
            write_symbol(0xf0, &ac[ac_index], bw);
            r -= 16;
        }
    }
    if r > 0 {
        buffer_end_of_band(coding, ac_index, &[], bw, queue, ac);
        if !eob_allowed {
            flush(coding, bw, queue, ac);
        }
    }
    true
}
#[allow(clippy::too_many_arguments)]
fn encode_refinement<'a>(
    coeffs: &[i16],
    ac_index: usize,
    ac: &[HuffmanCodeTable],
    mut ss: i32,
    se: i32,
    al: i32,
    coding: &mut DctCodingState,
    bw: &mut JpegBitWriter<'a>,
    queue: &mut Queue<'a>,
) -> bool {
    let eob_allowed = ss > 0;
    if ss == 0 {
        write_bits(bw, 1, (((coeffs[0] as i32) >> al) & 1) as u64);
        ss += 1;
    }
    if ss > se {
        return true;
    }
    let mut abs_values = [0i32; 64];
    let mut eob = 0;
    for k in ss..=se {
        let abs_val = (coeffs[JPEG_NATURAL_ORDER[k as usize]] as i32).abs() as i16;
        abs_values[k as usize] = (abs_val as i32) >> al;
        if abs_values[k as usize] == 1 {
            eob = k;
        }
    }
    let mut r = 0;
    let mut bits = [0i32; 64];
    let mut count = 0;
    for k in ss..=se {
        if abs_values[k as usize] == 0 {
            r += 1;
            continue;
        }
        while r > 15 && k <= eob {
            flush(coding, bw, queue, ac);
            write_symbol(0xf0, &ac[ac_index], bw);
            r -= 16;
            for &bit in &bits[..count] {
                write_bits(bw, 1, bit as u64);
            }
            count = 0;
        }
        if abs_values[k as usize] > 1 {
            bits[count] = abs_values[k as usize] & 1;
            count += 1;
            continue;
        }
        flush(coding, bw, queue, ac);
        write_symbol((r << 4) + 1, &ac[ac_index], bw);
        write_bits(
            bw,
            1,
            u64::from(coeffs[JPEG_NATURAL_ORDER[k as usize]] >= 0),
        );
        for &bit in &bits[..count] {
            write_bits(bw, 1, bit as u64);
        }
        count = 0;
        r = 0;
    }
    if r > 0 || count != 0 {
        buffer_end_of_band(coding, ac_index, &bits[..count], bw, queue, ac);
        if !eob_allowed {
            flush(coding, bw, queue, ac);
        }
    }
    true
}
fn next_extra_zero_run(scan: &JpegScanInfo, ss: &EncodeScanState<'_>) -> i32 {
    if ss.extra_zero_runs_pos < scan.extra_zero_runs.len() {
        scan.extra_zero_runs[ss.extra_zero_runs_pos].block_idx as i32
    } else {
        -1
    }
}
fn next_reset_point(scan: &JpegScanInfo, ss: &mut EncodeScanState<'_>) -> i32 {
    if ss.next_reset_point_pos < scan.reset_points.len() {
        let value = scan.reset_points[ss.next_reset_point_pos] as i32;
        ss.next_reset_point_pos += 1;
        value
    } else {
        -1
    }
}
#[inline(never)]
fn do_encode_scan<const MODE: u32>(
    jpg: &JpegData,
    state: &mut SerializationState<'_>,
) -> SerializationStatus {
    let scan = &jpg.scan_info[state.scan_index as usize];
    let restart_interval = if state.seen_dri_marker {
        jpg.restart_interval as i32
    } else {
        0
    };
    if !state.scan_state.body {
        if !encode_sos(jpg, scan, state) {
            return SerializationStatus::Error;
        }
        let ss = &mut state.scan_state;
        bit_writer_init(&mut ss.bw);
        coding_state_init(&mut ss.coding_state);
        ss.restarts_to_go = restart_interval;
        ss.next_restart_marker = 0;
        ss.block_scan_index = 0;
        ss.extra_zero_runs_pos = 0;
        ss.next_extra_zero_run_index = next_extra_zero_run(scan, ss);
        ss.next_reset_point_pos = 0;
        ss.next_reset_point = next_reset_point(scan, ss);
        ss.mcu_y = 0;
        ss.last_dc_coeff.fill(0);
        ss.body = true;
    }
    let ss = &mut state.scan_state;
    if !ss.body {
        return SerializationStatus::Error;
    }
    let interleaved = scan.num_components > 1;
    let (mcus_per_row, mcu_rows) = jpg.calculate_mcu_size(scan);
    let al = if state.is_progressive {
        scan.al as i32
    } else {
        0
    };
    let start = if state.is_progressive {
        scan.ss as i32
    } else {
        0
    };
    let end = if state.is_progressive {
        scan.se as i32
    } else {
        63
    };
    let want_ac = start != 0 || end != 0;
    let want_dc = start == 0;
    // Match the source's current nonstreaming completeness state and guards.
    let complete_ac = true;
    let has_ac = true;
    if want_ac && !has_ac {
        return SerializationStatus::NeedsMoreInput;
    }
    let complete_dc = has_ac;
    let complete = if want_ac { complete_ac } else { complete_dc };
    let last_mcu_y = if complete { mcu_rows } else { 0 };
    while ss.mcu_y < last_mcu_y {
        for mcu_x in 0..mcus_per_row {
            if restart_interval > 0 && ss.restarts_to_go == 0 {
                flush(
                    &mut ss.coding_state,
                    &mut ss.bw,
                    &mut state.output_queue,
                    &state.ac_huff_table,
                );
                if !jump_to_byte_boundary(
                    &mut ss.bw,
                    &mut state.output_queue,
                    &mut state.pad_bits,
                    &jpg.padding_bits,
                ) {
                    return SerializationStatus::Error;
                }
                emit_marker(
                    &mut ss.bw,
                    &mut state.output_queue,
                    0xd0 + ss.next_restart_marker,
                );
                ss.next_restart_marker = (ss.next_restart_marker + 1) & 7;
                ss.restarts_to_go = restart_interval;
                ss.last_dc_coeff.fill(0);
            }
            for si in &scan.components[..scan.num_components as usize] {
                let c = &jpg.components[si.comp_idx as usize];
                let dc = &state.dc_huff_table[si.dc_tbl_idx as usize];
                let ac_index = si.ac_tbl_idx as usize;
                let ac = &state.ac_huff_table[ac_index];
                if want_dc && !dc.initialized || want_ac && !ac.initialized {
                    return SerializationStatus::Error;
                }
                let blocks_y = if interleaved { c.v_samp_factor } else { 1 };
                let blocks_x = if interleaved { c.h_samp_factor } else { 1 };
                for iy in 0..blocks_y {
                    for ix in 0..blocks_x {
                        let block_y = ss.mcu_y * blocks_y + iy;
                        let block_x = mcu_x * blocks_x + ix;
                        let block_index =
                            block_y as usize * c.width_in_blocks as usize + block_x as usize;
                        if ss.block_scan_index == ss.next_reset_point {
                            flush(
                                &mut ss.coding_state,
                                &mut ss.bw,
                                &mut state.output_queue,
                                &state.ac_huff_table,
                            );
                            ss.next_reset_point = next_reset_point(scan, ss);
                        }
                        let mut num_zero_runs = 0;
                        if ss.block_scan_index == ss.next_extra_zero_run_index {
                            num_zero_runs = scan.extra_zero_runs[ss.extra_zero_runs_pos]
                                .num_extra_zero_runs
                                as i32;
                            ss.extra_zero_runs_pos += 1;
                            ss.next_extra_zero_run_index = next_extra_zero_run(scan, ss);
                        }
                        let coeffs = &c.coeffs[block_index << 6..];
                        reserve(&mut ss.bw, &mut state.output_queue, 512);
                        let last_dc = &mut ss.last_dc_coeff[si.comp_idx as usize];
                        let ok = if MODE == 0 {
                            encode_sequential(coeffs, dc, ac, num_zero_runs, last_dc, &mut ss.bw)
                        } else if MODE == 1 {
                            encode_progressive(
                                coeffs,
                                dc,
                                ac_index,
                                &state.ac_huff_table,
                                start,
                                end,
                                al,
                                num_zero_runs,
                                &mut ss.coding_state,
                                last_dc,
                                &mut ss.bw,
                                &mut state.output_queue,
                            )
                        } else {
                            encode_refinement(
                                coeffs,
                                ac_index,
                                &state.ac_huff_table,
                                start,
                                end,
                                al,
                                &mut ss.coding_state,
                                &mut ss.bw,
                                &mut state.output_queue,
                            )
                        };
                        if !ok {
                            return SerializationStatus::Error;
                        }
                        ss.block_scan_index += 1;
                    }
                }
            }
            ss.restarts_to_go -= 1;
        }
        ss.mcu_y += 1;
    }
    if ss.mcu_y < mcu_rows {
        if !ss.bw.healthy {
            return SerializationStatus::Error;
        }
        return SerializationStatus::NeedsMoreInput;
    }
    flush(
        &mut ss.coding_state,
        &mut ss.bw,
        &mut state.output_queue,
        &state.ac_huff_table,
    );
    if !jump_to_byte_boundary(
        &mut ss.bw,
        &mut state.output_queue,
        &mut state.pad_bits,
        &jpg.padding_bits,
    ) {
        return SerializationStatus::Error;
    }
    bit_writer_finish(&mut ss.bw, &mut state.output_queue);
    ss.body = false;
    state.scan_index += 1;
    if !ss.bw.healthy {
        return SerializationStatus::Error;
    }
    SerializationStatus::Done
}
fn encode_scan(jpg: &JpegData, state: &mut SerializationState<'_>) -> SerializationStatus {
    let scan = &jpg.scan_info[state.scan_index as usize];
    let progressive = state.is_progressive;
    let al = if progressive { scan.al } else { 0 };
    let ah = if progressive { scan.ah } else { 0 };
    let ss = if progressive { scan.ss } else { 0 };
    let se = if progressive { scan.se } else { 63 };
    if !progressive || ah == 0 && al == 0 && ss == 0 && se == 63 {
        do_encode_scan::<0>(jpg, state)
    } else if ah == 0 {
        do_encode_scan::<1>(jpg, state)
    } else {
        do_encode_scan::<2>(jpg, state)
    }
}
fn serialize_section<'a>(
    marker: u8,
    state: &mut SerializationState<'a>,
    jpg: &'a JpegData,
) -> SerializationStatus {
    let ok = match marker {
        0xc0 | 0xc1 | 0xc2 | 0xc9 | 0xca => encode_sof(jpg, marker, state),
        0xc4 => encode_dht(jpg, state),
        0xd0..=0xd7 => {
            state
                .output_queue
                .push_back(OutputChunk::from_bytes(&[0xff, marker]));
            true
        }
        0xd9 => {
            state
                .output_queue
                .push_back(OutputChunk::from_bytes(&[0xff, 0xd9]));
            state
                .output_queue
                .push_back(OutputChunk::borrowed(&jpg.tail_data));
            true
        }
        0xda => return encode_scan(jpg, state),
        0xdb => encode_dqt(jpg, state),
        0xdd => {
            state.seen_dri_marker = true;
            state.output_queue.push_back(OutputChunk::from_bytes(&[
                0xff,
                0xdd,
                0,
                4,
                (jpg.restart_interval >> 8) as u8,
                jpg.restart_interval as u8,
            ]));
            true
        }
        0xe0..=0xef => {
            let i = state.app_index as usize;
            state.app_index += 1;
            if i >= jpg.app_data.len() {
                false
            } else {
                state
                    .output_queue
                    .push_back(OutputChunk::from_bytes(&[0xff]));
                state
                    .output_queue
                    .push_back(OutputChunk::borrowed(&jpg.app_data[i]));
                true
            }
        }
        0xfe => {
            let i = state.com_index as usize;
            state.com_index += 1;
            if i >= jpg.com_data.len() {
                false
            } else {
                state
                    .output_queue
                    .push_back(OutputChunk::from_bytes(&[0xff]));
                state
                    .output_queue
                    .push_back(OutputChunk::borrowed(&jpg.com_data[i]));
                true
            }
        }
        0xff => {
            let i = state.data_index as usize;
            state.data_index += 1;
            if i >= jpg.inter_marker_data.len() {
                false
            } else {
                state
                    .output_queue
                    .push_back(OutputChunk::borrowed(&jpg.inter_marker_data[i]));
                true
            }
        }
        _ => false,
    };
    if ok {
        SerializationStatus::Done
    } else {
        SerializationStatus::Error
    }
}
fn maybe_push_output(
    state: &mut SerializationState<'_>,
    out: &mut impl FnMut(Option<&[u8]>) -> usize,
) -> Status {
    if !matches!(state.stage, Stage::Error) {
        while let Some(chunk) = state.output_queue.front_mut() {
            let written = out(if chunk.null {
                None
            } else {
                Some(chunk.bytes())
            });
            assert!(
                written <= chunk.len,
                "source output callback cannot overreport writes"
            );
            if written == 0 && chunk.len > 0 {
                return Err(StatusCode::NotEnoughBytes);
            }
            // The pinned code decreases len without advancing next. Preserve
            // that observable behavior on a short successful callback.
            chunk.len -= written;
            if chunk.len == 0 {
                state.output_queue.pop_front();
            }
        }
    }
    Ok(())
}
#[expect(
    clippy::collapsible_if,
    reason = "Mirror the source nested conditions."
)]
fn write_jpeg_internal<'a>(
    jpg: &'a JpegData,
    out: &mut impl FnMut(Option<&[u8]>) -> usize,
    state: &mut SerializationState<'a>,
) -> Status {
    loop {
        match state.stage {
            Stage::Init => {
                if jpg.marker_order.is_empty() {
                    state.stage = Stage::Error;
                    continue;
                }
                state
                    .dc_huff_table
                    .resize_with(MAX_HUFFMAN_TABLES, HuffmanCodeTable::default);
                state
                    .ac_huff_table
                    .resize_with(MAX_HUFFMAN_TABLES, HuffmanCodeTable::default);
                if jpg.has_zero_padding_bit && jpg.padding_bits.capacity() != 0 {
                    // An empty source vector with no allocation has a null data
                    // pointer; its null padding pointer selects all-one padding.
                    state.pad_bits = Some(0);
                }
                state
                    .output_queue
                    .push_back(OutputChunk::from_bytes(&[0xff, 0xd8]));
                maybe_push_output(state, out)?;
                state.stage = Stage::SerializeSection;
            }
            Stage::SerializeSection => {
                if state.section_index >= jpg.marker_order.len() {
                    state.stage = Stage::Done;
                    continue;
                }
                let status = serialize_section(jpg.marker_order[state.section_index], state, jpg);
                if status == SerializationStatus::Error {
                    state.stage = Stage::Error;
                    continue;
                }
                maybe_push_output(state, out)?;
                if status == SerializationStatus::NeedsMoreInput {
                    return Err(StatusCode::GenericError);
                }
                if status != SerializationStatus::Done {
                    state.stage = Stage::Error;
                    return Err(StatusCode::GenericError);
                }
                state.section_index += 1;
            }
            Stage::Done => {
                if !state.output_queue.is_empty() {
                    return Err(StatusCode::GenericError);
                }
                if let Some(pad) = state.pad_bits {
                    if pad != jpg.padding_bits.len() {
                        return Err(StatusCode::GenericError);
                    }
                }
                return Ok(());
            }
            Stage::Error => return Err(StatusCode::GenericError),
        }
    }
}
/// Write the original JPEG representation through the source callback lifecycle.
/// A null source pointer is `None` (and always has zero length). Empty allocated
/// storage stays `Some(&[])`. Callback returns must not exceed their offered
/// byte count, as in JPEGOutput.
pub fn write_jpeg(jpg: &JpegData, mut out: impl FnMut(Option<&[u8]>) -> usize) -> Status {
    let mut state = Box::new(SerializationState::default());
    write_jpeg_internal(jpg, &mut out, &mut state)
}
