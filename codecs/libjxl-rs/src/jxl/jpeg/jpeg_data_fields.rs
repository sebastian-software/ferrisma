// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Complete JPEG metadata bundle from `lib/jxl/jpeg/jpeg_data.cc`.
//! The separate Brotli byte payload, coefficient reconstruction, and JPEG parser
//! are not part of this serialized-fields boundary.

use super::jpeg_data::*;
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::field_encodings::{Fields, U32Enc, bits, bits_offset, val};
use crate::jxl::fields::{Visitor, check_has_enough_bits};

#[derive(Default)]
struct JpegInfo {
    num_app_markers: usize,
    num_com_markers: usize,
    num_scans: usize,
    num_intermarker: usize,
    has_dri: bool,
}
fn visit_marker(marker: &mut u8, visitor: &mut dyn Visitor, info: &mut JpegInfo) -> Status {
    // uint8_t promotes to int before subtraction, then converts to uint32_t.
    let mut marker32 = (i32::from(*marker) - 0xc0) as u32;
    visitor.bits(6, 0, &mut marker32)?;
    *marker = marker32.wrapping_add(0xc0) as u8;
    if *marker & 0xf0 == 0xe0 {
        info.num_app_markers += 1;
    }
    if *marker == 0xfe {
        info.num_com_markers += 1;
    }
    if *marker == 0xda {
        info.num_scans += 1;
    }
    if *marker == 0xff {
        info.num_intermarker += 1;
    }
    if *marker == 0xdd {
        info.has_dri = true;
    }
    Ok(())
}
fn invalid() -> Status {
    Err(StatusCode::GenericError)
}

impl Fields for JpegData {
    #[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        let mut is_gray = self.components.len() == 1;
        visitor.bool(false, &mut is_gray)?;
        if visitor.is_reading() {
            self.components
                .resize(if is_gray { 1 } else { 3 }, JpegComponent::default());
        }
        let mut info = JpegInfo::default();
        if visitor.is_reading() {
            let mut marker = 0xc0;
            loop {
                visit_marker(&mut marker, visitor, &mut info)?;
                self.marker_order.push(marker);
                if self.marker_order.len() > 16384 {
                    return invalid();
                }
                if marker == 0xd9 {
                    break;
                }
            }
        } else {
            if self.marker_order.len() > 16384 {
                return invalid();
            }
            for marker in &mut self.marker_order {
                visit_marker(marker, visitor, &mut info)?;
            }
            if !self.marker_order.is_empty() && self.marker_order.last() != Some(&0xd9) {
                return invalid();
            }
        }
        if info.num_scans == 0 {
            return invalid();
        }
        if visitor.is_reading() {
            self.app_data.resize(info.num_app_markers, Vec::new());
            self.app_marker_type
                .resize(info.num_app_markers, AppMarkerType::UNKNOWN);
            self.com_data.resize(info.num_com_markers, Vec::new());
            self.scan_info
                .resize(info.num_scans, JpegScanInfo::default());
        }
        if self.app_data.len() != info.num_app_markers
            || self.app_marker_type.len() != info.num_app_markers
            || self.com_data.len() != info.num_com_markers
            || self.scan_info.len() != info.num_scans
        {
            return invalid();
        }
        for (app, kind) in self.app_data.iter_mut().zip(&mut self.app_marker_type) {
            visitor.u32(
                U32Enc::new(val(0), val(1), bits_offset(1, 2), bits_offset(2, 4)),
                0,
                &mut kind.0,
            )?;
            if kind.0 > 3 {
                return invalid();
            }
            let mut len = app.len().wrapping_sub(1) as u32;
            visitor.bits(16, 0, &mut len)?;
            if visitor.is_reading() {
                app.resize((len + 1) as usize, 0);
            }
            if app.len() < 3 {
                return invalid();
            }
        }
        for com in &mut self.com_data {
            let mut len = com.len().wrapping_sub(1) as u32;
            visitor.bits(16, 0, &mut len)?;
            if visitor.is_reading() {
                com.resize((len + 1) as usize, 0);
            }
            if com.len() < 3 {
                return invalid();
            }
        }
        let mut num_quant_tables = self.quant.len() as u32;
        visitor.u32(
            U32Enc::new(val(1), val(2), val(3), val(4)),
            2,
            &mut num_quant_tables,
        )?;
        if num_quant_tables == 4 {
            return invalid();
        }
        if visitor.is_reading() {
            self.quant
                .resize(num_quant_tables as usize, JpegQuantTable::default());
        }
        for i in 0..num_quant_tables as usize {
            let quant = &mut self.quant[i];
            if quant.precision > 1 {
                return invalid();
            }
            visitor.bits(1, 0, &mut quant.precision)?;
            visitor.bits(2, i as u32, &mut quant.index)?;
            visitor.bool(true, &mut quant.is_last)?;
        }
        let mut component_type = if self.components.len() == 1 && self.components[0].id == 1 {
            0
        } else if self.components.len() == 3
            && self.components[0].id == 1
            && self.components[1].id == 2
            && self.components[2].id == 3
        {
            1
        } else if self.components.len() == 3
            && self.components[0].id == u32::from(b'R')
            && self.components[1].id == u32::from(b'G')
            && self.components[2].id == u32::from(b'B')
        {
            2
        } else {
            3
        };
        visitor.bits(2, 1, &mut component_type)?;
        let mut num_components;
        if component_type == 0 {
            num_components = 1;
        } else if component_type != 3 {
            num_components = 3;
        } else {
            num_components = self.components.len() as u32;
            visitor.u32(
                U32Enc::new(val(1), val(2), val(3), val(4)),
                3,
                &mut num_components,
            )?;
            if num_components != 1 && num_components != 3 {
                return invalid();
            }
        }
        if visitor.is_reading() {
            self.components
                .resize(num_components as usize, JpegComponent::default());
        }
        if component_type == 3 {
            for component in &mut self.components {
                visitor.bits(8, 0, &mut component.id)?;
            }
        } else if component_type == 0 {
            self.components[0].id = 1;
        } else if component_type == 2 {
            self.components[0].id = u32::from(b'R');
            self.components[1].id = u32::from(b'G');
            self.components[2].id = u32::from(b'B');
        } else {
            self.components[0].id = 1;
            self.components[1].id = 2;
            self.components[2].id = 3;
        }
        let mut used_tables = 0usize;
        for component in &mut self.components {
            visitor.bits(2, 0, &mut component.quant_idx)?;
            if component.quant_idx as usize >= self.quant.len() {
                return invalid();
            }
            used_tables |= 1usize << component.quant_idx;
        }
        for i in 0..self.quant.len() {
            if used_tables & (1usize << i) != 0 {
                continue;
            }
            if i == 0 {
                return invalid();
            }
            for j in 0..64 {
                if self.quant[i].values[j] != self.quant[i - 1].values[j] {
                    return invalid();
                }
            }
        }
        let mut num_huff = self.huffman_code.len() as u32;
        visitor.u32(
            U32Enc::new(
                val(4),
                bits_offset(3, 2),
                bits_offset(4, 10),
                bits_offset(6, 26),
            ),
            4,
            &mut num_huff,
        )?;
        if visitor.is_reading() {
            self.huffman_code
                .resize(num_huff as usize, JpegHuffmanCode::default());
        }
        for hc in &mut self.huffman_code {
            let mut is_ac = hc.slot_id >> 4 != 0;
            let mut id = (hc.slot_id & 0xf) as u32;
            visitor.bool(false, &mut is_ac)?;
            visitor.bits(2, 0, &mut id)?;
            hc.slot_id = ((u32::from(is_ac) << 4) | id) as i32;
            visitor.bool(true, &mut hc.is_last)?;
            let mut num_symbols = 0usize;
            for count in &mut hc.counts {
                visitor.u32(
                    U32Enc::new(val(0), val(1), bits_offset(3, 2), bits(8)),
                    0,
                    count,
                )?;
                num_symbols += *count as usize;
            }
            if num_symbols == 0 {
                continue;
            }
            if num_symbols > hc.values.len() {
                return invalid();
            }
            let mut value_slots = [0u64; 5];
            for value in &mut hc.values[..num_symbols] {
                visitor.u32(
                    U32Enc::new(
                        bits(2),
                        bits_offset(2, 4),
                        bits_offset(4, 8),
                        bits_offset(8, 1),
                    ),
                    0,
                    value,
                )?;
                value_slots[(*value >> 6) as usize] |= 1u64 << (*value & 0x3f);
            }
            if hc.values[num_symbols - 1] != JPEG_HUFFMAN_ALPHABET_SIZE as u32 {
                return invalid();
            }
            if value_slots[4] != 1 {
                return invalid();
            }
            let mut num_values = 1usize;
            for slot in &value_slots[..4] {
                num_values += slot.count_ones() as usize;
            }
            if num_values != num_symbols {
                return invalid();
            }
            if !is_ac
                && (value_slots[0] >> JPEG_DC_ALPHABET_SIZE
                    | value_slots[1]
                    | value_slots[2]
                    | value_slots[3])
                    != 0
            {
                return invalid();
            }
        }
        for scan in &mut self.scan_info {
            visitor.u32(
                U32Enc::new(val(1), val(2), val(3), val(4)),
                1,
                &mut scan.num_components,
            )?;
            if scan.num_components >= 4 {
                return invalid();
            }
            visitor.bits(6, 0, &mut scan.ss)?;
            visitor.bits(6, 63, &mut scan.se)?;
            visitor.bits(4, 0, &mut scan.al)?;
            visitor.bits(4, 0, &mut scan.ah)?;
            for c in &mut scan.components[..scan.num_components as usize] {
                visitor.bits(2, 0, &mut c.comp_idx)?;
                if c.comp_idx as usize >= self.components.len() {
                    return invalid();
                }
                visitor.bits(2, 0, &mut c.ac_tbl_idx)?;
                visitor.bits(2, 0, &mut c.dc_tbl_idx)?;
            }
            visitor.u32(
                U32Enc::new(val(0), val(1), val(2), bits_offset(3, 3)),
                (crate::jxl::common::MAX_NUM_PASSES - 1) as u32,
                &mut scan.last_needed_pass,
            )?;
        }
        if info.has_dri {
            visitor.bits(16, 0, &mut self.restart_interval)?;
        }
        for scan in &mut self.scan_info {
            let mut num_reset_points = scan.reset_points.len() as u32;
            visitor.u32(
                U32Enc::new(
                    val(0),
                    bits_offset(2, 1),
                    bits_offset(4, 4),
                    bits_offset(16, 20),
                ),
                0,
                &mut num_reset_points,
            )?;
            if visitor.is_reading() {
                scan.reset_points.resize(num_reset_points as usize, 0);
            }
            let mut last_block_idx = -1i32;
            for block_idx in &mut scan.reset_points {
                *block_idx = block_idx.wrapping_sub((last_block_idx + 1) as u32);
                visitor.u32(
                    U32Enc::new(
                        val(0),
                        bits_offset(3, 1),
                        bits_offset(5, 9),
                        bits_offset(28, 41),
                    ),
                    0,
                    block_idx,
                )?;
                *block_idx = block_idx.wrapping_add((last_block_idx + 1) as u32);
                if *block_idx >= 3u32 << 26 {
                    return invalid();
                }
                last_block_idx = *block_idx as i32;
            }
            let mut num_extra_zero_runs = scan.extra_zero_runs.len() as u32;
            visitor.u32(
                U32Enc::new(
                    val(0),
                    bits_offset(2, 1),
                    bits_offset(4, 4),
                    bits_offset(16, 20),
                ),
                0,
                &mut num_extra_zero_runs,
            )?;
            if visitor.is_reading() {
                scan.extra_zero_runs
                    .resize(num_extra_zero_runs as usize, ExtraZeroRunInfo::default());
            }
            last_block_idx = -1;
            for run in &mut scan.extra_zero_runs {
                visitor.u32(
                    U32Enc::new(
                        val(1),
                        bits_offset(2, 2),
                        bits_offset(4, 5),
                        bits_offset(8, 20),
                    ),
                    1,
                    &mut run.num_extra_zero_runs,
                )?;
                run.block_idx = run.block_idx.wrapping_sub((last_block_idx + 1) as u32);
                visitor.u32(
                    U32Enc::new(
                        val(0),
                        bits_offset(3, 1),
                        bits_offset(5, 9),
                        bits_offset(28, 41),
                    ),
                    0,
                    &mut run.block_idx,
                )?;
                run.block_idx = run.block_idx.wrapping_add((last_block_idx + 1) as u32);
                if run.num_extra_zero_runs > 4 {
                    return invalid();
                }
                if run.block_idx > 3u32 << 26 {
                    return invalid();
                }
                last_block_idx = run.block_idx as i32;
            }
        }
        let mut inter_marker_data_sizes = Vec::with_capacity(info.num_intermarker);
        for i in 0..info.num_intermarker {
            let mut len = if visitor.is_reading() {
                0
            } else {
                self.inter_marker_data[i].len() as u32
            };
            visitor.bits(16, 0, &mut len)?;
            if visitor.is_reading() {
                inter_marker_data_sizes.push(len);
            }
        }
        let mut tail_data_len = self.tail_data.len() as u32;
        if !visitor.is_reading() && tail_data_len > 4260096 {
            return invalid();
        }
        visitor.u32(
            U32Enc::new(
                val(0),
                bits_offset(8, 1),
                bits_offset(16, 257),
                bits_offset(22, 65793),
            ),
            0,
            &mut tail_data_len,
        )?;
        visitor.bool(false, &mut self.has_zero_padding_bit)?;
        if self.has_zero_padding_bit {
            let mut nbit = self.padding_bits.len() as u32;
            visitor.bits(24, 0, &mut nbit)?;
            if visitor.is_reading() {
                check_has_enough_bits(visitor, nbit as usize)?;
                // reserve() in C++ requests total capacity, not additional capacity.
                let capacity = nbit.min(1024) as usize;
                self.padding_bits
                    .reserve(capacity.saturating_sub(self.padding_bits.len()));
                for _ in 0..nbit {
                    let mut bit = false;
                    visitor.bool(false, &mut bit)?;
                    self.padding_bits.push(u8::from(bit));
                }
            } else {
                for bit in &mut self.padding_bits {
                    let mut bbit = *bit != 0;
                    visitor.bool(false, &mut bbit)?;
                    *bit = u8::from(bbit);
                }
            }
        }
        let mut dht_index = 0;
        let mut scan_index = 0;
        let mut is_progressive = false;
        let mut ac_ok = [false; MAX_HUFFMAN_TABLES];
        let mut dc_ok = [false; MAX_HUFFMAN_TABLES];
        for marker in &self.marker_order {
            if *marker == 0xc2 {
                is_progressive = true;
            } else if *marker == 0xc4 {
                while dht_index < self.huffman_code.len() {
                    let huff = &self.huffman_code[dht_index];
                    dht_index += 1;
                    let mut index = huff.slot_id as usize;
                    if index & 0x10 != 0 {
                        index -= 0x10;
                        ac_ok[index] = true;
                    } else {
                        dc_ok[index] = true;
                    }
                    if huff.is_last {
                        break;
                    }
                }
            } else if *marker == 0xda {
                let scan = &self.scan_info[scan_index];
                scan_index += 1;
                for c in &scan.components[..scan.num_components as usize] {
                    let want_dc = !is_progressive || scan.ss == 0;
                    if want_dc && !dc_ok[c.dc_tbl_idx as usize] {
                        return invalid();
                    }
                    let want_ac = !is_progressive || scan.ss != 0 || scan.se != 0;
                    if want_ac && !ac_ok[c.ac_tbl_idx as usize] {
                        return invalid();
                    }
                }
            }
        }
        if visitor.is_reading() {
            self.tail_data.resize(tail_data_len as usize, 0);
            if inter_marker_data_sizes.len() != info.num_intermarker {
                return invalid();
            }
            self.inter_marker_data.reserve(
                info.num_intermarker
                    .saturating_sub(self.inter_marker_data.len()),
            );
            for len in inter_marker_data_sizes {
                self.inter_marker_data.push(vec![0; len as usize]);
            }
        }
        Ok(())
    }
}

/// Port of the complete `SetJPEGDataFromICC` byte injection boundary.
pub fn set_jpeg_data_from_icc(icc: &[u8], jpeg_data: &mut JpegData) -> Status {
    let mut icc_pos = 0usize;
    for i in 0..jpeg_data.app_data.len() {
        let app = &mut jpeg_data.app_data[i];
        let kind = &jpeg_data.app_marker_type[i];
        if *kind != AppMarkerType::ICC {
            continue;
        }
        if app.len() < 17 {
            return invalid();
        }
        let len = app.len() - 17;
        if icc_pos + len > icc.len() {
            return invalid();
        }
        app[17..].copy_from_slice(&icc[icc_pos..icc_pos + len]);
        icc_pos += len;
    }
    if icc_pos != icc.len() && icc_pos != 0 {
        return invalid();
    }
    Ok(())
}
