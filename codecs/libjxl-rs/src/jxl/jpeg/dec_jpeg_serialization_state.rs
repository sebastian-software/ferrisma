// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Storage from dec_jpeg_serialization_state.h. Queue and bit-writer are
//! reborrowed together instead of storing an owner-relative output pointer.
//! The current AC Huffman table and pad pointers are stable owner-relative
//! indices; source-uninitialized scalars get safe initial values only.

use super::dec_jpeg_output_chunk::OutputChunk;
use super::jpeg_data::MAX_COMPONENTS;
use std::collections::VecDeque;

pub struct HuffmanCodeTable {
    pub depth: [i8; 256],
    pub code: [u16; 256],
    pub initialized: bool,
}
impl Default for HuffmanCodeTable {
    fn default() -> Self {
        Self {
            depth: [0; 256],
            code: [0; 256],
            initialized: false,
        }
    }
}
#[derive(Default)]
pub struct JpegBitWriter<'a> {
    pub healthy: bool,
    pub chunk: Option<OutputChunk<'a>>,
    pub pos: usize,
    pub put_buffer: u64,
    pub put_bits: i32,
}
#[derive(Default)]
pub struct DctCodingState {
    pub eob_run: i32,
    pub cur_ac_huff: Option<usize>,
    pub refinement_bits: Vec<u16>,
    pub refinement_bits_count: usize,
}
#[derive(Default)]
pub struct EncodeScanState<'a> {
    pub body: bool,
    pub mcu_y: i32,
    pub bw: JpegBitWriter<'a>,
    pub last_dc_coeff: [i16; MAX_COMPONENTS],
    pub restarts_to_go: i32,
    pub next_restart_marker: i32,
    pub block_scan_index: i32,
    pub coding_state: DctCodingState,
    pub extra_zero_runs_pos: usize,
    pub next_extra_zero_run_index: i32,
    pub next_reset_point_pos: usize,
    pub next_reset_point: i32,
}
#[derive(Default)]
pub enum Stage {
    #[default]
    Init,
    SerializeSection,
    Done,
    Error,
}
#[derive(Default)]
pub struct SerializationState<'a> {
    pub stage: Stage,
    pub output_queue: VecDeque<OutputChunk<'a>>,
    pub section_index: usize,
    pub dht_index: i32,
    pub dqt_index: i32,
    pub app_index: i32,
    pub com_index: i32,
    pub data_index: i32,
    pub scan_index: i32,
    pub dc_huff_table: Vec<HuffmanCodeTable>,
    pub ac_huff_table: Vec<HuffmanCodeTable>,
    pub pad_bits: Option<usize>,
    pub seen_dri_marker: bool,
    pub is_progressive: bool,
    pub scan_state: EncodeScanState<'a>,
}
