// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/dec_ans.{h,cc}`. Borrowed code tables replace retained
//! pointers; aligned, initialized storage owns the alias tables and LZ77 window.
//! Unsigned state/counter wrapping remains explicit. See ADR 0003.

use super::ans_common::{
    AliasTable, Entry, create_flat_histogram, get_population_count_precision, init_alias_table,
};
use super::ans_params::{
    ANS_LOG_TAB_SIZE, ANS_MAX_ALPHABET_SIZE, ANS_SIGNATURE, ANS_TAB_SIZE, PREFIX_MAX_BITS,
};
use super::base::bits::{ceil_log2_nonzero, floor_log2_nonzero};
use super::base::status::{Status, StatusCode, StatusOr};
use super::dec_bit_reader::BitReader;
use super::dec_huffman::{HUFFMAN_TABLE_BITS, HuffmanDecodingData};
use super::field_encodings::{Fields, U32Enc, bits_offset, val};
use super::fields::{Visitor, bundle};
use super::huffman_table::HuffmanCode;
use super::memory_manager_internal::AlignedMemory;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HybridUintConfig {
    pub split_exponent: u32,
    pub split_token: u32,
    pub msb_in_token: u32,
    pub lsb_in_token: u32,
}

impl Default for HybridUintConfig {
    fn default() -> Self {
        Self::new(4, 2, 0)
    }
}

impl HybridUintConfig {
    pub const fn new(split_exponent: u32, msb_in_token: u32, lsb_in_token: u32) -> Self {
        assert!(split_exponent < 32 && msb_in_token + lsb_in_token <= split_exponent);
        Self {
            split_exponent,
            split_token: 1u32 << split_exponent,
            msb_in_token,
            lsb_in_token,
        }
    }

    pub fn encode(self, value: u32) -> (u32, u32, u32) {
        if value < self.split_token {
            return (value, 0, 0);
        }
        let n = floor_log2_nonzero(value) as u32;
        let m = value - (1u32 << n);
        let token = self
            .split_token
            .wrapping_add((n - self.split_exponent) << (self.msb_in_token + self.lsb_in_token))
            .wrapping_add((m >> (n - self.msb_in_token)) << self.lsb_in_token)
            .wrapping_add(m & ((1u32 << self.lsb_in_token) - 1));
        let nbits = n - self.msb_in_token - self.lsb_in_token;
        let bits = ((u64::from(value) >> self.lsb_in_token) & ((1u64 << nbits) - 1)) as u32;
        (token, nbits, bits)
    }

    pub const fn lsb_mask(self) -> u32 {
        (1u32 << self.lsb_in_token) - 1
    }
}

#[derive(Clone, Debug)]
pub struct LZ77Params {
    pub enabled: bool,
    pub min_symbol: u32,
    pub min_length: u32,
    pub length_uint_config: HybridUintConfig,
    pub nonserialized_distance_context: usize,
}

impl Default for LZ77Params {
    fn default() -> Self {
        let mut result = Self {
            enabled: false,
            min_symbol: 0,
            min_length: 0,
            length_uint_config: HybridUintConfig::new(0, 0, 0),
            nonserialized_distance_context: 0,
        };
        bundle::init(&mut result);
        result
    }
}

impl Fields for LZ77Params {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.bool(false, &mut self.enabled)?;
        if !visitor.conditional(self.enabled) {
            return Ok(());
        }
        visitor.u32(
            U32Enc::new(val(224), val(512), val(4096), bits_offset(15, 8)),
            224,
            &mut self.min_symbol,
        )?;
        visitor.u32(
            U32Enc::new(val(3), val(4), bits_offset(2, 5), bits_offset(8, 9)),
            3,
            &mut self.min_length,
        )?;
        Ok(())
    }
}

pub const WINDOW_SIZE: usize = 1 << 20;
pub const NUM_SPECIAL_DISTANCES: usize = 120;
const WINDOW_MASK: usize = WINDOW_SIZE - 1;
const SPECIAL_DISTANCES: [[i8; 2]; NUM_SPECIAL_DISTANCES] = [
    [0, 1],
    [1, 0],
    [1, 1],
    [-1, 1],
    [0, 2],
    [2, 0],
    [1, 2],
    [-1, 2],
    [2, 1],
    [-2, 1],
    [2, 2],
    [-2, 2],
    [0, 3],
    [3, 0],
    [1, 3],
    [-1, 3],
    [3, 1],
    [-3, 1],
    [2, 3],
    [-2, 3],
    [3, 2],
    [-3, 2],
    [0, 4],
    [4, 0],
    [1, 4],
    [-1, 4],
    [4, 1],
    [-4, 1],
    [3, 3],
    [-3, 3],
    [2, 4],
    [-2, 4],
    [4, 2],
    [-4, 2],
    [0, 5],
    [3, 4],
    [-3, 4],
    [4, 3],
    [-4, 3],
    [5, 0],
    [1, 5],
    [-1, 5],
    [5, 1],
    [-5, 1],
    [2, 5],
    [-2, 5],
    [5, 2],
    [-5, 2],
    [4, 4],
    [-4, 4],
    [3, 5],
    [-3, 5],
    [5, 3],
    [-5, 3],
    [0, 6],
    [6, 0],
    [1, 6],
    [-1, 6],
    [6, 1],
    [-6, 1],
    [2, 6],
    [-2, 6],
    [6, 2],
    [-6, 2],
    [4, 5],
    [-4, 5],
    [5, 4],
    [-5, 4],
    [3, 6],
    [-3, 6],
    [6, 3],
    [-6, 3],
    [0, 7],
    [7, 0],
    [1, 7],
    [-1, 7],
    [5, 5],
    [-5, 5],
    [7, 1],
    [-7, 1],
    [4, 6],
    [-4, 6],
    [6, 4],
    [-6, 4],
    [2, 7],
    [-2, 7],
    [7, 2],
    [-7, 2],
    [3, 7],
    [-3, 7],
    [7, 3],
    [-7, 3],
    [5, 6],
    [-5, 6],
    [6, 5],
    [-6, 5],
    [8, 0],
    [4, 7],
    [-4, 7],
    [7, 4],
    [-7, 4],
    [8, 1],
    [8, 2],
    [6, 6],
    [-6, 6],
    [8, 3],
    [5, 7],
    [-5, 7],
    [7, 5],
    [-7, 5],
    [8, 4],
    [6, 7],
    [-6, 7],
    [7, 6],
    [-7, 6],
    [8, 5],
    [7, 7],
    [-7, 7],
    [8, 6],
    [8, 7],
];

pub fn special_distance(index: usize, multiplier: i32) -> i32 {
    let [x, y] = SPECIAL_DISTANCES[index];
    let dist = i32::from(x).wrapping_add(multiplier.wrapping_mul(i32::from(y)));
    if dist > 1 { dist } else { 1 }
}

#[derive(Default)]
pub struct ANSCode {
    alias_storage: Option<AlignedMemory>,
    pub huffman_data: Vec<HuffmanDecodingData>,
    pub uint_config: Vec<HybridUintConfig>,
    pub degenerate_symbols: Vec<i32>,
    pub use_prefix_code: bool,
    pub log_alpha_size: u8,
    pub lz77: LZ77Params,
    pub max_num_bits: usize,
}

impl ANSCode {
    pub fn alias_tables(&self) -> &[Entry] {
        self.alias_storage
            .as_ref()
            .map_or(&[], AlignedMemory::alias_entries)
    }

    pub fn update_max_num_bits(&mut self, ctx: usize, mut symbol: usize) {
        let cfg = if self.lz77.enabled
            && self.lz77.nonserialized_distance_context != ctx
            && symbol >= self.lz77.min_symbol as usize
        {
            symbol -= self.lz77.min_symbol as usize;
            self.lz77.length_uint_config
        } else {
            self.uint_config[ctx]
        };
        let HybridUintConfig {
            split_token,
            msb_in_token,
            lsb_in_token,
            split_exponent,
        } = cfg;
        if symbol < split_token as usize {
            self.max_num_bits = self.max_num_bits.max(split_exponent as usize);
            return;
        }
        let n_extra_bits = (split_exponent as usize - (msb_in_token + lsb_in_token) as usize
            + ((symbol - split_token as usize) >> (msb_in_token + lsb_in_token)))
            as u32;
        let total_bits = (msb_in_token + lsb_in_token) as usize + n_extra_bits as usize + 1;
        self.max_num_bits = self.max_num_bits.max(total_bits);
    }
}

pub fn decode_uint_config(
    log_alpha_size: usize,
    uint_config: &mut HybridUintConfig,
    br: &mut BitReader<'_>,
) -> Status {
    br.refill();
    let split_exponent = br.read_bits(ceil_log2_nonzero(log_alpha_size + 1)) as usize;
    let mut msb_in_token = 0;
    let mut lsb_in_token = 0;
    if split_exponent != log_alpha_size {
        let nbits = ceil_log2_nonzero(split_exponent + 1);
        msb_in_token = br.read_bits(nbits) as usize;
        if msb_in_token > split_exponent {
            return Err(StatusCode::GenericError);
        }
        let nbits = ceil_log2_nonzero(split_exponent - msb_in_token + 1);
        lsb_in_token = br.read_bits(nbits) as usize;
    }
    if lsb_in_token + msb_in_token > split_exponent {
        return Err(StatusCode::GenericError);
    }
    *uint_config = HybridUintConfig::new(
        split_exponent as u32,
        msb_in_token as u32,
        lsb_in_token as u32,
    );
    Ok(())
}

pub fn decode_uint_configs(
    log_alpha_size: usize,
    uint_config: &mut [HybridUintConfig],
    br: &mut BitReader<'_>,
) -> Status {
    for cfg in uint_config {
        decode_uint_config(log_alpha_size, cfg, br)?;
    }
    Ok(())
}

fn decode_var_len_uint8(input: &mut BitReader<'_>) -> i32 {
    if input.read_fixed_bits::<1>() != 0 {
        let nbits = input.read_fixed_bits::<3>() as usize;
        if nbits == 0 {
            1
        } else {
            input.read_bits(nbits) as i32 + (1 << nbits)
        }
    } else {
        0
    }
}

fn decode_var_len_uint16(input: &mut BitReader<'_>) -> i32 {
    if input.read_fixed_bits::<1>() != 0 {
        let nbits = input.read_fixed_bits::<4>() as usize;
        if nbits == 0 {
            1
        } else {
            input.read_bits(nbits) as i32 + (1 << nbits)
        }
    } else {
        0
    }
}

fn read_histogram(
    precision_bits: usize,
    counts: &mut Vec<i32>,
    input: &mut BitReader<'_>,
) -> Status {
    let range = 1i32 << precision_bits;
    let simple_code = input.read_bits(1);
    if simple_code == 1 {
        let mut symbols = [0i32; 2];
        let mut max_symbol = 0;
        let num_symbols = input.read_bits(1) as usize + 1;
        for symbol in &mut symbols[..num_symbols] {
            *symbol = decode_var_len_uint8(input);
            max_symbol = max_symbol.max(*symbol);
        }
        counts.resize(max_symbol as usize + 1, 0);
        if num_symbols == 1 {
            counts[symbols[0] as usize] = range;
        } else {
            if symbols[0] == symbols[1] {
                return Err(StatusCode::GenericError);
            }
            counts[symbols[0] as usize] = input.read_bits(precision_bits) as i32;
            counts[symbols[1] as usize] = range - counts[symbols[0] as usize];
        }
    } else {
        if input.read_bits(1) == 1 {
            let alphabet_size = decode_var_len_uint8(input) + 1;
            if alphabet_size > range {
                return Err(StatusCode::GenericError);
            }
            *counts = create_flat_histogram(alphabet_size, range);
            return Ok(());
        }
        let upper_bound_log = floor_log2_nonzero(ANS_LOG_TAB_SIZE + 1);
        let mut log = 0;
        while log < upper_bound_log {
            if input.read_fixed_bits::<1>() == 0 {
                break;
            }
            log += 1;
        }
        let shift = (input.read_bits(log) as u32 | (1 << log)) - 1;
        if shift > ANS_LOG_TAB_SIZE + 1 {
            return Err(StatusCode::GenericError);
        }
        let length = decode_var_len_uint8(input) as usize + 3;
        counts.resize(length, 0);
        let mut total_count = 0;
        const HUFF: [[u8; 2]; 128] = [
            [3, 10],
            [7, 12],
            [3, 7],
            [4, 3],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 5],
            [3, 10],
            [4, 4],
            [3, 7],
            [4, 1],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 2],
            [3, 10],
            [5, 0],
            [3, 7],
            [4, 3],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 5],
            [3, 10],
            [4, 4],
            [3, 7],
            [4, 1],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 2],
            [3, 10],
            [6, 11],
            [3, 7],
            [4, 3],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 5],
            [3, 10],
            [4, 4],
            [3, 7],
            [4, 1],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 2],
            [3, 10],
            [5, 0],
            [3, 7],
            [4, 3],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 5],
            [3, 10],
            [4, 4],
            [3, 7],
            [4, 1],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 2],
            [3, 10],
            [7, 13],
            [3, 7],
            [4, 3],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 5],
            [3, 10],
            [4, 4],
            [3, 7],
            [4, 1],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 2],
            [3, 10],
            [5, 0],
            [3, 7],
            [4, 3],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 5],
            [3, 10],
            [4, 4],
            [3, 7],
            [4, 1],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 2],
            [3, 10],
            [6, 11],
            [3, 7],
            [4, 3],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 5],
            [3, 10],
            [4, 4],
            [3, 7],
            [4, 1],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 2],
            [3, 10],
            [5, 0],
            [3, 7],
            [4, 3],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 5],
            [3, 10],
            [4, 4],
            [3, 7],
            [4, 1],
            [3, 6],
            [3, 8],
            [3, 9],
            [4, 2],
        ];
        let mut logcounts = vec![0i32; length];
        let mut omit_log = -1;
        let mut omit_pos = -1i32;
        let mut same = vec![0i32; length];
        let mut i = 0;
        while i < length {
            input.refill();
            let idx = input.peek_fixed_bits::<7>() as usize;
            input.consume(HUFF[idx][0] as usize);
            logcounts[i] = i32::from(HUFF[idx][1]) - 1;
            if logcounts[i] == ANS_LOG_TAB_SIZE as i32 {
                let rle_length = decode_var_len_uint8(input);
                same[i] = rle_length + 5;
                i += rle_length as usize + 4;
                continue;
            }
            if logcounts[i] > omit_log {
                omit_log = logcounts[i];
                omit_pos = i as i32;
            }
            i += 1;
        }
        if omit_pos < 0 {
            return Err(StatusCode::GenericError);
        }
        let omit_pos = omit_pos as usize;
        if omit_pos + 1 < length && logcounts[omit_pos + 1] == ANS_LOG_TAB_SIZE as i32 {
            return Err(StatusCode::GenericError);
        }
        let mut prev = 0;
        let mut numsame = 0;
        for i in 0..length {
            if same[i] != 0 {
                numsame = same[i] - 1;
                prev = if i > 0 { counts[i - 1] } else { 0 };
            }
            if numsame > 0 {
                counts[i] = prev;
                numsame -= 1;
            } else {
                let code = logcounts[i];
                if i == omit_pos || code < 0 {
                    continue;
                }
                if shift == 0 || code == 0 {
                    counts[i] = 1 << code;
                } else {
                    let bitcount = get_population_count_precision(code as u32, shift);
                    counts[i] = (1i32 << code)
                        + (input.read_bits(bitcount as usize) << (code as u32 - bitcount)) as i32;
                }
            }
            total_count += counts[i];
        }
        counts[omit_pos] = range - total_count;
        if counts[omit_pos] <= 0 {
            return Err(StatusCode::GenericError);
        }
    }
    Ok(())
}

pub fn decode_ans_codes(
    num_histograms: usize,
    max_alphabet_size: usize,
    input: &mut BitReader<'_>,
    result: &mut ANSCode,
) -> Status {
    result.degenerate_symbols.resize(num_histograms, -1);
    if result.use_prefix_code {
        if max_alphabet_size > 1 << PREFIX_MAX_BITS {
            return Err(StatusCode::GenericError);
        }
        result
            .huffman_data
            .resize_with(num_histograms, HuffmanDecodingData::default);
        let mut alphabet_sizes = vec![0u16; num_histograms];
        for size in &mut alphabet_sizes {
            *size = (decode_var_len_uint16(input) + 1) as u16;
            if usize::from(*size) > max_alphabet_size {
                return Err(StatusCode::GenericError);
            }
        }
        for (c, &size) in alphabet_sizes.iter().enumerate() {
            if size > 1 {
                if !result.huffman_data[c].read_from_bit_stream(usize::from(size), input) {
                    if input.all_reads_within_bounds().is_err() {
                        return Err(StatusCode::NotEnoughBytes);
                    }
                    return Err(StatusCode::GenericError);
                }
            } else {
                result.huffman_data[c].table.clear();
                result.huffman_data[c]
                    .table
                    .resize(1 << HUFFMAN_TABLE_BITS, HuffmanCode::default());
            }
            for i in 0..result.huffman_data[c].table.len() {
                let h = result.huffman_data[c].table[i];
                if usize::from(h.bits) <= HUFFMAN_TABLE_BITS {
                    result.update_max_num_bits(c, usize::from(h.value));
                }
            }
        }
    } else {
        if max_alphabet_size > ANS_MAX_ALPHABET_SIZE {
            return Err(StatusCode::GenericError);
        }
        let alloc_size =
            num_histograms * (1usize << result.log_alpha_size) * std::mem::size_of::<Entry>();
        result.alias_storage = Some(AlignedMemory::create(alloc_size, 0)?);
        for c in 0..num_histograms {
            let mut counts = Vec::new();
            if read_histogram(ANS_LOG_TAB_SIZE as usize, &mut counts, input).is_err() {
                return Err(StatusCode::GenericError);
            }
            if counts.len() > max_alphabet_size {
                return Err(StatusCode::GenericError);
            }
            while counts.last() == Some(&0) {
                counts.pop();
            }
            for (s, &count) in counts.iter().enumerate() {
                if count != 0 {
                    result.update_max_num_bits(c, s);
                }
            }
            let mut degenerate_symbol = if counts.is_empty() {
                0
            } else {
                counts.len() as i32 - 1
            };
            for s in 0..degenerate_symbol {
                if counts[s as usize] != 0 {
                    degenerate_symbol = -1;
                    break;
                }
            }
            result.degenerate_symbols[c] = degenerate_symbol;
            let offset = c * (1usize << result.log_alpha_size);
            let tables = result.alias_storage.as_mut().unwrap().alias_entries_mut();
            init_alias_table(
                counts,
                ANS_LOG_TAB_SIZE,
                result.log_alpha_size as usize,
                &mut tables[offset..],
            )?;
        }
    }
    Ok(())
}

pub fn decode_histograms(
    br: &mut BitReader<'_>,
    mut num_contexts: usize,
    code: &mut ANSCode,
    context_map: &mut Vec<u8>,
    disallow_lz77: bool,
) -> Status {
    assert!(num_contexts > 0);
    bundle::read(br, &mut code.lz77)?;
    if code.lz77.enabled {
        num_contexts += 1;
        decode_uint_config(8, &mut code.lz77.length_uint_config, br)?;
    }
    if code.lz77.enabled && disallow_lz77 {
        return Err(StatusCode::GenericError);
    }
    let mut num_histograms = 1;
    context_map.resize(num_contexts, 0);
    if num_contexts > 1 {
        super::dec_context_map::decode_context_map(context_map, &mut num_histograms, br)?;
    }
    code.lz77.nonserialized_distance_context = usize::from(*context_map.last().unwrap());
    code.use_prefix_code = br.read_fixed_bits::<1>() != 0;
    code.log_alpha_size = if code.use_prefix_code {
        PREFIX_MAX_BITS as u8
    } else {
        br.read_fixed_bits::<2>() as u8 + 5
    };
    code.uint_config
        .resize(num_histograms, HybridUintConfig::default());
    decode_uint_configs(code.log_alpha_size as usize, &mut code.uint_config, br)?;
    decode_ans_codes(num_histograms, 1 << code.log_alpha_size, br, code)
}

pub const MAX_CHECKPOINT_INTERVAL: usize = 512;

pub struct Checkpoint {
    pub state: u32,
    pub num_to_copy: u32,
    pub copy_pos: u32,
    pub num_decoded: u32,
    pub lz77_window: [u32; MAX_CHECKPOINT_INTERVAL],
}

impl Default for Checkpoint {
    fn default() -> Self {
        Self {
            state: 0,
            num_to_copy: 0,
            copy_pos: 0,
            num_decoded: 0,
            lz77_window: [0; MAX_CHECKPOINT_INTERVAL],
        }
    }
}

/// Code-independent owned fields from the original ANSSymbolReader. Moving
/// this carrier retains the same LZ77 allocation and all checkpoint state.
pub struct ANSSymbolReaderState {
    use_prefix_code: bool,
    state: u32,
    log_alpha_size: u32,
    log_entry_size: u32,
    entry_size_minus_1: u32,
    lz77_window: Option<AlignedMemory>,
    num_decoded: u32,
    num_to_copy: u32,
    copy_pos: u32,
    lz77_ctx: u32,
    lz77_min_length: u32,
    lz77_threshold: u32,
    lz77_length_uint: HybridUintConfig,
    special_distances: [u32; NUM_SPECIAL_DISTANCES],
    num_special_distances: u32,
}

impl Default for ANSSymbolReaderState {
    fn default() -> Self {
        Self {
            use_prefix_code: false,
            state: ANS_SIGNATURE << 16,
            log_alpha_size: 0,
            log_entry_size: 0,
            entry_size_minus_1: 0,
            lz77_window: None,
            num_decoded: 0,
            num_to_copy: 0,
            copy_pos: 0,
            lz77_ctx: 0,
            lz77_min_length: 0,
            lz77_threshold: 1 << 20,
            lz77_length_uint: HybridUintConfig::default(),
            special_distances: [0; NUM_SPECIAL_DISTANCES],
            num_special_distances: 0,
        }
    }
}

#[cfg(feature = "cpp-oracle")]
impl ANSSymbolReaderState {
    pub fn observer_use_prefix_code(&self) -> bool {
        self.use_prefix_code
    }

    pub fn observer_scalars(&self) -> [u32; 15] {
        [
            self.state,
            self.log_alpha_size,
            self.log_entry_size,
            self.entry_size_minus_1,
            self.num_decoded,
            self.num_to_copy,
            self.copy_pos,
            self.lz77_ctx,
            self.lz77_min_length,
            self.lz77_threshold,
            self.lz77_length_uint.split_exponent,
            self.lz77_length_uint.split_token,
            self.lz77_length_uint.msb_in_token,
            self.lz77_length_uint.lsb_in_token,
            self.num_special_distances,
        ]
    }

    pub fn observer_window(&self) -> Option<&[u32]> {
        self.lz77_window
            .as_ref()
            .map(|window| &window.u32s()[..(self.num_decoded as usize).min(WINDOW_SIZE)])
    }
}

pub struct ANSSymbolReader<'a> {
    code: &'a ANSCode,
    use_prefix_code: bool,
    state: u32,
    log_alpha_size: u32,
    log_entry_size: u32,
    entry_size_minus_1: u32,
    lz77_window: Option<AlignedMemory>,
    num_decoded: u32,
    num_to_copy: u32,
    copy_pos: u32,
    lz77_ctx: u32,
    lz77_min_length: u32,
    lz77_threshold: u32,
    lz77_length_uint: HybridUintConfig,
    special_distances: [u32; NUM_SPECIAL_DISTANCES],
    num_special_distances: u32,
}

impl<'a> ANSSymbolReader<'a> {
    /// Resolve surviving cached table/config views in the same code owner.
    /// In-place source updates remain visible; the mode scalar stays cached.
    /// Reached source views must not have been invalidated by owner changes.
    pub fn from_state(code: &'a ANSCode, state: ANSSymbolReaderState) -> Self {
        Self {
            code,
            use_prefix_code: state.use_prefix_code,
            state: state.state,
            log_alpha_size: state.log_alpha_size,
            log_entry_size: state.log_entry_size,
            entry_size_minus_1: state.entry_size_minus_1,
            lz77_window: state.lz77_window,
            num_decoded: state.num_decoded,
            num_to_copy: state.num_to_copy,
            copy_pos: state.copy_pos,
            lz77_ctx: state.lz77_ctx,
            lz77_min_length: state.lz77_min_length,
            lz77_threshold: state.lz77_threshold,
            lz77_length_uint: state.lz77_length_uint,
            special_distances: state.special_distances,
            num_special_distances: state.num_special_distances,
        }
    }

    pub fn into_state(self) -> ANSSymbolReaderState {
        ANSSymbolReaderState {
            use_prefix_code: self.use_prefix_code,
            state: self.state,
            log_alpha_size: self.log_alpha_size,
            log_entry_size: self.log_entry_size,
            entry_size_minus_1: self.entry_size_minus_1,
            lz77_window: self.lz77_window,
            num_decoded: self.num_decoded,
            num_to_copy: self.num_to_copy,
            copy_pos: self.copy_pos,
            lz77_ctx: self.lz77_ctx,
            lz77_min_length: self.lz77_min_length,
            lz77_threshold: self.lz77_threshold,
            lz77_length_uint: self.lz77_length_uint,
            special_distances: self.special_distances,
            num_special_distances: self.num_special_distances,
        }
    }

    pub fn create(
        code: &'a ANSCode,
        br: &mut BitReader<'_>,
        distance_multiplier: usize,
    ) -> StatusOr<Self> {
        let lz77_window = if code.lz77.enabled {
            Some(AlignedMemory::create(WINDOW_SIZE * 4, 0)?)
        } else {
            None
        };
        let mut result = Self {
            code,
            use_prefix_code: code.use_prefix_code,
            state: ANS_SIGNATURE << 16,
            log_alpha_size: 0,
            log_entry_size: 0,
            entry_size_minus_1: 0,
            lz77_window,
            num_decoded: 0,
            num_to_copy: 0,
            copy_pos: 0,
            lz77_ctx: 0,
            lz77_min_length: 0,
            lz77_threshold: 1 << 20,
            lz77_length_uint: HybridUintConfig::default(),
            special_distances: [0; NUM_SPECIAL_DISTANCES],
            num_special_distances: 0,
        };
        if !code.use_prefix_code {
            result.state = br.read_fixed_bits::<32>() as u32;
            result.log_alpha_size = u32::from(code.log_alpha_size);
            result.log_entry_size = ANS_LOG_TAB_SIZE - u32::from(code.log_alpha_size);
            result.entry_size_minus_1 = (1 << result.log_entry_size) - 1;
        } else {
            result.state = ANS_SIGNATURE << 16;
        }
        if !code.lz77.enabled {
            return Ok(result);
        }
        result.lz77_ctx = code.lz77.nonserialized_distance_context as u32;
        result.lz77_length_uint = code.lz77.length_uint_config;
        result.lz77_threshold = code.lz77.min_symbol;
        result.lz77_min_length = code.lz77.min_length;
        result.num_special_distances = if distance_multiplier == 0 {
            0
        } else {
            NUM_SPECIAL_DISTANCES as u32
        };
        for i in 0..result.num_special_distances as usize {
            result.special_distances[i] = special_distance(i, distance_multiplier as i32) as u32;
        }
        Ok(result)
    }

    pub fn read_symbol_ans_without_refill(
        &mut self,
        histo_idx: usize,
        br: &mut BitReader<'_>,
    ) -> usize {
        let res = self.state & (ANS_TAB_SIZE - 1);
        let table = &self.code.alias_tables()[histo_idx << self.log_alpha_size..];
        let symbol = AliasTable::lookup(
            table,
            res as usize,
            self.log_entry_size as usize,
            self.entry_size_minus_1 as usize,
        );
        self.state =
            (symbol.freq * (self.state >> ANS_LOG_TAB_SIZE) as usize + symbol.offset) as u32;
        let new_state = (self.state << 16) | br.peek_fixed_bits::<16>() as u32;
        let normalize = self.state < 1 << 16;
        if normalize {
            self.state = new_state;
        }
        br.consume(if normalize { 16 } else { 0 });
        let next_res = self.state & (ANS_TAB_SIZE - 1);
        AliasTable::prefetch(table, next_res as usize, self.log_entry_size as usize);
        symbol.value
    }

    pub fn read_symbol_huff_without_refill(
        &self,
        histo_idx: usize,
        br: &mut BitReader<'_>,
    ) -> usize {
        self.code.huffman_data[histo_idx].read_symbol(br) as usize
    }

    pub fn read_symbol_without_refill(
        &mut self,
        histo_idx: usize,
        br: &mut BitReader<'_>,
    ) -> usize {
        if self.use_prefix_code {
            self.read_symbol_huff_without_refill(histo_idx, br)
        } else {
            self.read_symbol_ans_without_refill(histo_idx, br)
        }
    }

    pub fn read_symbol(&mut self, histo_idx: usize, br: &mut BitReader<'_>) -> usize {
        br.refill();
        self.read_symbol_without_refill(histo_idx, br)
    }

    pub fn check_ans_final_state(&self) -> bool {
        self.state == ANS_SIGNATURE << 16
    }

    /// The caller must refill first. The source masks the extra-bit count
    /// instead of rejecting invalid shifts at this hot-path boundary.
    pub fn read_hybrid_uint_config(
        config: HybridUintConfig,
        mut token: usize,
        br: &mut BitReader<'_>,
    ) -> u32 {
        let HybridUintConfig {
            split_token,
            msb_in_token,
            lsb_in_token,
            split_exponent,
        } = config;
        let (split_token, msb_in_token, lsb_in_token, split_exponent) = (
            split_token as usize,
            msb_in_token as usize,
            lsb_in_token as usize,
            split_exponent as usize,
        );
        if token < split_token {
            return token as u32;
        }
        let mut nbits = (split_exponent - (msb_in_token + lsb_in_token)
            + ((token - split_token) >> (msb_in_token + lsb_in_token)))
            as u32;
        nbits &= 31;
        let low = token & ((1 << lsb_in_token) - 1);
        token >>= lsb_in_token;
        let bits = br.peek_bits(nbits as usize) as usize;
        br.consume(nbits as usize);
        let ret = (((((1usize << msb_in_token) | (token & ((1 << msb_in_token) - 1))) << nbits)
            | bits)
            << lsb_in_token)
            | low;
        ret as u32
    }

    pub fn read_hybrid_uint_clustered_huff_rle_only(
        &mut self,
        ctx: usize,
        br: &mut BitReader<'_>,
        value: &mut u32,
        run: &mut u32,
    ) {
        debug_assert!(self.is_huff_rle_only());
        br.refill();
        let token = self.read_symbol_huff_without_refill(ctx, br);
        if token >= self.lz77_threshold as usize {
            *run = Self::read_hybrid_uint_config(
                self.lz77_length_uint,
                token - self.lz77_threshold as usize,
                br,
            )
            .wrapping_add(self.lz77_min_length)
            .wrapping_sub(1);
            return;
        }
        *value = Self::read_hybrid_uint_config(self.code.uint_config[ctx], token, br);
    }

    pub fn is_huff_rle_only(&self) -> bool {
        if self.lz77_window.is_none() || !self.use_prefix_code {
            return false;
        }
        for i in 0..HUFFMAN_TABLE_BITS {
            let h = self.code.huffman_data[self.lz77_ctx as usize].table[i];
            if h.bits != 0 || h.value != 1 {
                return false;
            }
        }
        self.code.uint_config[self.lz77_ctx as usize].split_token <= 1
    }

    pub fn uses_lz77(&self) -> bool {
        self.lz77_window.is_some()
    }

    pub fn read_hybrid_uint_clustered_inlined<const USES_LZ77: bool>(
        &mut self,
        ctx: usize,
        br: &mut BitReader<'_>,
    ) -> usize {
        if USES_LZ77 && self.num_to_copy > 0 {
            return self.copy_one();
        }
        br.refill();
        let token = self.read_symbol_without_refill(ctx, br);
        if USES_LZ77 && token >= self.lz77_threshold as usize {
            self.num_to_copy = Self::read_hybrid_uint_config(
                self.lz77_length_uint,
                token - self.lz77_threshold as usize,
                br,
            )
            .wrapping_add(self.lz77_min_length);
            br.refill();
            let d_token = self.read_symbol_without_refill(self.lz77_ctx as usize, br);
            let mut distance = Self::read_hybrid_uint_config(
                self.code.uint_config[self.lz77_ctx as usize],
                d_token,
                br,
            ) as usize;
            if distance < self.num_special_distances as usize {
                distance = self.special_distances[distance] as usize;
            } else {
                distance = distance
                    .wrapping_add(1)
                    .wrapping_sub(self.num_special_distances as usize);
            }
            if distance > self.num_decoded as usize {
                distance = self.num_decoded as usize;
            }
            if distance > WINDOW_SIZE {
                distance = WINDOW_SIZE;
            }
            self.copy_pos = (self.num_decoded as usize - distance) as u32;
            if distance == 0 {
                let to_fill = (self.num_to_copy as usize).min(WINDOW_SIZE);
                self.lz77_window.as_mut().expect("LZ77 window").u32s_mut()[..to_fill].fill(0);
            }
            if self.num_to_copy < self.lz77_min_length {
                self.num_to_copy = 0;
                br.mark_unhealthy();
                return 0;
            }
            return self.copy_one();
        }
        let ret = Self::read_hybrid_uint_config(self.code.uint_config[ctx], token, br) as usize;
        if USES_LZ77 {
            if let Some(window) = self.lz77_window.as_mut() {
                window.u32s_mut()[self.num_decoded as usize & WINDOW_MASK] = ret as u32;
                self.num_decoded = self.num_decoded.wrapping_add(1);
            }
        }
        ret
    }

    fn copy_one(&mut self) -> usize {
        let window = self.lz77_window.as_mut().expect("LZ77 window").u32s_mut();
        let ret = window[self.copy_pos as usize & WINDOW_MASK];
        self.copy_pos = self.copy_pos.wrapping_add(1);
        self.num_to_copy = self.num_to_copy.wrapping_sub(1);
        window[self.num_decoded as usize & WINDOW_MASK] = ret;
        self.num_decoded = self.num_decoded.wrapping_add(1);
        ret as usize
    }

    pub fn read_hybrid_uint_clustered<const USES_LZ77: bool>(
        &mut self,
        ctx: usize,
        br: &mut BitReader<'_>,
    ) -> usize {
        self.read_hybrid_uint_clustered_inlined::<USES_LZ77>(ctx, br)
    }

    pub fn read_hybrid_uint_clustered_maybe_inlined<const USES_LZ77: bool>(
        &mut self,
        ctx: usize,
        br: &mut BitReader<'_>,
    ) -> usize {
        self.read_hybrid_uint_clustered_inlined::<USES_LZ77>(ctx, br)
    }

    pub fn read_hybrid_uint_inlined<const USES_LZ77: bool>(
        &mut self,
        ctx: usize,
        br: &mut BitReader<'_>,
        context_map: &[u8],
    ) -> usize {
        self.read_hybrid_uint_clustered::<USES_LZ77>(usize::from(context_map[ctx]), br)
    }

    pub fn read_hybrid_uint(
        &mut self,
        ctx: usize,
        br: &mut BitReader<'_>,
        context_map: &[u8],
    ) -> usize {
        self.read_hybrid_uint_clustered::<true>(usize::from(context_map[ctx]), br)
    }

    pub fn is_single_value_and_advance(
        &mut self,
        ctx: usize,
        value: &mut u32,
        count: usize,
    ) -> bool {
        if self.use_prefix_code || self.num_to_copy != 0 {
            return false;
        }
        let res = self.state & (ANS_TAB_SIZE - 1);
        let table = &self.code.alias_tables()[ctx << self.log_alpha_size..];
        let symbol = AliasTable::lookup(
            table,
            res as usize,
            self.log_entry_size as usize,
            self.entry_size_minus_1 as usize,
        );
        if symbol.freq != ANS_TAB_SIZE as usize
            || self.code.uint_config[ctx].split_token as usize <= symbol.value
            || symbol.value >= self.lz77_threshold as usize
        {
            return false;
        }
        *value = symbol.value as u32;
        if let Some(window) = self.lz77_window.as_mut() {
            for _ in 0..count {
                window.u32s_mut()[self.num_decoded as usize & WINDOW_MASK] = symbol.value as u32;
                self.num_decoded = self.num_decoded.wrapping_add(1);
            }
        }
        true
    }

    pub fn save(&self, checkpoint: &mut Checkpoint) {
        checkpoint.state = self.state;
        checkpoint.num_decoded = self.num_decoded;
        checkpoint.num_to_copy = self.num_to_copy;
        checkpoint.copy_pos = self.copy_pos;
        if let Some(window) = self.lz77_window.as_ref() {
            let win_start = self.num_decoded as usize & WINDOW_MASK;
            let win_end = (self.num_decoded as usize + MAX_CHECKPOINT_INTERVAL) & WINDOW_MASK;
            let window = window.u32s();
            if win_end > win_start {
                checkpoint
                    .lz77_window
                    .copy_from_slice(&window[win_start..win_end]);
            } else {
                let first = WINDOW_SIZE - win_start;
                checkpoint.lz77_window[..first].copy_from_slice(&window[win_start..]);
                checkpoint.lz77_window[first..].copy_from_slice(&window[..win_end]);
            }
        }
    }

    pub fn restore(&mut self, checkpoint: &Checkpoint) {
        self.state = checkpoint.state;
        debug_assert!(
            self.num_decoded as usize <= checkpoint.num_decoded as usize + MAX_CHECKPOINT_INTERVAL
        );
        self.num_decoded = checkpoint.num_decoded;
        self.num_to_copy = checkpoint.num_to_copy;
        self.copy_pos = checkpoint.copy_pos;
        if let Some(window) = self.lz77_window.as_mut() {
            let win_start = self.num_decoded as usize & WINDOW_MASK;
            let win_end = (self.num_decoded as usize + MAX_CHECKPOINT_INTERVAL) & WINDOW_MASK;
            let window = window.u32s_mut();
            if win_end > win_start {
                window[win_start..win_end].copy_from_slice(&checkpoint.lz77_window);
            } else {
                let first = WINDOW_SIZE - win_start;
                window[win_start..].copy_from_slice(&checkpoint.lz77_window[..first]);
                window[..win_end].copy_from_slice(&checkpoint.lz77_window[first..]);
            }
        }
    }
}
