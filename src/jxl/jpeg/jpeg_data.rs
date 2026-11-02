// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Parsed JPEG data storage from lib/jxl/jpeg/jpeg_data.h. Coefficient/marker
//! storage owns its vectors. Source-indeterminate scalar/array storage receives
//! safe initialization with no extra source meaning. JPEG parsing,
//! MCU calculation and reconstruction remain unported. Metadata bundle fields
//! are implemented in the adjacent jpeg_data_fields module.

use crate::jxl::frame_dimensions::DCT_BLOCK_SIZE;
pub const MAX_COMPONENTS: usize = 4;
pub const MAX_QUANT_TABLES: usize = 4;
pub const MAX_HUFFMAN_TABLES: usize = 4;
pub const JPEG_HUFFMAN_MAX_BIT_LENGTH: usize = 16;
pub const JPEG_HUFFMAN_ALPHABET_SIZE: usize = 256;
pub const JPEG_DC_ALPHABET_SIZE: usize = 12;
pub const MAX_DHT_MARKERS: usize = 512;
pub const MAX_DIM_PIXELS: usize = 65535;
#[derive(Clone)]
pub struct JpegQuantTable {
    pub values: [i32; DCT_BLOCK_SIZE],
    pub precision: u32,
    pub index: u32,
    pub is_last: bool,
}
impl Default for JpegQuantTable {
    fn default() -> Self {
        Self {
            values: [0; DCT_BLOCK_SIZE],
            precision: 0,
            index: 0,
            is_last: true,
        }
    }
}
#[derive(Clone)]
pub struct JpegHuffmanCode {
    pub counts: [u32; JPEG_HUFFMAN_MAX_BIT_LENGTH + 1],
    pub values: [u32; JPEG_HUFFMAN_ALPHABET_SIZE + 1],
    pub slot_id: i32,
    pub is_last: bool,
}
impl Default for JpegHuffmanCode {
    fn default() -> Self {
        Self {
            counts: [0; JPEG_HUFFMAN_MAX_BIT_LENGTH + 1],
            values: [0; JPEG_HUFFMAN_ALPHABET_SIZE + 1],
            slot_id: 0,
            is_last: true,
        }
    }
}
#[derive(Clone, Copy, Default)]
pub struct JpegComponentScanInfo {
    pub comp_idx: u32,
    pub dc_tbl_idx: u32,
    pub ac_tbl_idx: u32,
}
#[derive(Clone, Copy, Default)]
pub struct ExtraZeroRunInfo {
    pub block_idx: u32,
    pub num_extra_zero_runs: u32,
}
#[derive(Clone, Default)]
pub struct JpegScanInfo {
    pub ss: u32,
    pub se: u32,
    pub ah: u32,
    pub al: u32,
    pub num_components: u32,
    pub components: [JpegComponentScanInfo; 4],
    pub last_needed_pass: u32,
    pub reset_points: Vec<u32>,
    pub extra_zero_runs: Vec<ExtraZeroRunInfo>,
}
#[derive(Clone)]
pub struct JpegComponent {
    pub id: u32,
    pub h_samp_factor: i32,
    pub v_samp_factor: i32,
    pub quant_idx: u32,
    pub width_in_blocks: u32,
    pub height_in_blocks: u32,
    pub coeffs: Vec<i16>,
}
impl Default for JpegComponent {
    fn default() -> Self {
        Self {
            id: 0,
            h_samp_factor: 1,
            v_samp_factor: 1,
            quant_idx: 0,
            width_in_blocks: 0,
            height_in_blocks: 0,
            coeffs: Vec::new(),
        }
    }
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct AppMarkerType(pub u32);
impl AppMarkerType {
    pub const UNKNOWN: Self = Self(0);
    pub const ICC: Self = Self(1);
    pub const EXIF: Self = Self(2);
    pub const XMP: Self = Self(3);
}
#[derive(Clone, Default)]
pub struct JpegData {
    pub width: i32,
    pub height: i32,
    pub restart_interval: u32,
    pub app_data: Vec<Vec<u8>>,
    pub app_marker_type: Vec<AppMarkerType>,
    pub com_data: Vec<Vec<u8>>,
    pub quant: Vec<JpegQuantTable>,
    pub huffman_code: Vec<JpegHuffmanCode>,
    pub components: Vec<JpegComponent>,
    pub scan_info: Vec<JpegScanInfo>,
    pub marker_order: Vec<u8>,
    pub inter_marker_data: Vec<Vec<u8>>,
    pub tail_data: Vec<u8>,
    pub has_zero_padding_bit: bool,
    pub padding_bits: Vec<u8>,
}
