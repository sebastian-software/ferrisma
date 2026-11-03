// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under MIT license. See licenses/BROTLI-MIT.txt or
// https://opensource.org/licenses/MIT.
// Mechanical translation of pinned Brotli c/dec/huffman.{c,h}.
// Table storage is borrowed directly from its owner. MaybeUninit retains the
// source's untouched slots without manufacturing initialized table payloads.

use super::constants::BROTLI_CODE_LENGTH_CODES;
use std::mem::MaybeUninit;

pub const BROTLI_HUFFMAN_MAX_CODE_LENGTH: usize = 15;
pub const BROTLI_HUFFMAN_MAX_SIZE_26: usize = 396;
pub const BROTLI_HUFFMAN_MAX_SIZE_258: usize = 632;
pub const BROTLI_HUFFMAN_MAX_SIZE_272: usize = 646;
pub const BROTLI_HUFFMAN_MAX_CODE_LENGTH_CODE_LENGTH: usize = 5;
const BROTLI_REVERSE_BITS_MAX: usize = 8;

// The pinned native AArch64 build defines BROTLI_RBIT. The native x86_64
// configuration uses the original 256-byte reverse table instead.
#[cfg(target_arch = "aarch64")]
const BROTLI_REVERSE_BITS_BASE: usize = usize::BITS as usize - BROTLI_REVERSE_BITS_MAX;
#[cfg(not(target_arch = "aarch64"))]
const BROTLI_REVERSE_BITS_BASE: usize = 0;
const BROTLI_REVERSE_BITS_LOWEST: usize =
    1usize << (BROTLI_REVERSE_BITS_MAX - 1 + BROTLI_REVERSE_BITS_BASE);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct HuffmanCode {
    pub bits: u8,
    pub value: u16,
}

impl HuffmanCode {
    pub const fn new(bits: u8, value: u16) -> Self {
        Self { bits, value }
    }
}

#[cfg(target_arch = "aarch64")]
#[inline]
fn brotli_reverse_bits(num: usize) -> usize {
    num.reverse_bits()
}

#[cfg(not(target_arch = "aarch64"))]
#[inline]
fn brotli_reverse_bits(num: usize) -> usize {
    usize::from(REVERSE_BITS[num])
}

macro_rules! repeat_5 {
    ($body:block) => {{
        $body;
        $body;
        $body;
        $body;
        $body;
    }};
}
macro_rules! repeat_6 {
    ($body:block) => {{
        $body;
        $body;
        $body;
        $body;
        $body;
        $body;
    }};
}

#[inline]
fn replicate_value(
    table: &mut [MaybeUninit<HuffmanCode>],
    step: usize,
    mut end: usize,
    code: HuffmanCode,
) {
    loop {
        end -= step;
        table[end].write(code);
        if end == 0 {
            break;
        }
    }
}

#[inline]
fn next_table_bit_size(count: &[u16], mut len: usize, root_bits: usize) -> usize {
    let mut left = 1i32 << (len - root_bits);
    while len < BROTLI_HUFFMAN_MAX_CODE_LENGTH {
        left -= i32::from(count[len]);
        if left <= 0 {
            break;
        }
        len += 1;
        left <<= 1;
    }
    len - root_bits
}

/// Builds the original 32-entry code-length lookup table.
///
/// The caller supplies 18 initialized code lengths in 0..=5 and the initialized
/// histogram prefix count[0..6]. A complete tree or singleton initializes
/// table[0..32]; incomplete trees retain untouched holes in that extent.
/// Reached sort indices and replicated table writes must be in bounds, as in C.
pub fn brotli_build_code_lengths_huffman_table(
    table: &mut [MaybeUninit<HuffmanCode>],
    code_lengths: &[u8],
    count: &mut [u16],
) {
    let mut sorted = [0i32; BROTLI_CODE_LENGTH_CODES as usize];
    let mut offset = [0i32; BROTLI_HUFFMAN_MAX_CODE_LENGTH_CODE_LENGTH + 1];
    let mut symbol = -1i32;
    let mut bits = 1usize;
    repeat_5!({
        symbol += i32::from(count[bits]);
        offset[bits] = symbol;
        bits += 1;
    });
    let _ = bits; // Preserve the final source macro increment.
    offset[0] = BROTLI_CODE_LENGTH_CODES as i32 - 1;
    symbol = BROTLI_CODE_LENGTH_CODES as i32;
    loop {
        repeat_6!({
            symbol -= 1;
            let length = usize::from(code_lengths[symbol as usize]);
            sorted[offset[length] as usize] = symbol;
            offset[length] -= 1;
        });
        if symbol == 0 {
            break;
        }
    }
    let table_size = 1 << BROTLI_HUFFMAN_MAX_CODE_LENGTH_CODE_LENGTH;
    if offset[0] == 0 {
        let code = HuffmanCode::new(0, sorted[0] as u16);
        for slot in &mut table[..table_size] {
            slot.write(code);
        }
        return;
    }
    let mut key = 0usize;
    let mut key_step = BROTLI_REVERSE_BITS_LOWEST;
    symbol = 0;
    bits = 1;
    let mut step = 2;
    loop {
        let mut bits_count = count[bits];
        while bits_count != 0 {
            let code = HuffmanCode::new(bits as u8, sorted[symbol as usize] as u16);
            symbol += 1;
            replicate_value(
                &mut table[brotli_reverse_bits(key)..],
                step,
                table_size,
                code,
            );
            key = key.wrapping_add(key_step);
            bits_count -= 1;
        }
        step <<= 1;
        key_step >>= 1;
        bits += 1;
        if bits > BROTLI_HUFFMAN_MAX_CODE_LENGTH_CODE_LENGTH {
            break;
        }
    }
}

/// Builds the original root and second-level lookup tables.
///
/// `symbol_lists` is the original backing allocation and `origin` is its symbol
/// zero. The 15 preceding slots hold list heads for lengths 1..=15. All reached
/// links, root bits (7..=8), counts and output spans retain the C preconditions.
/// All 16 count words must be initialized. Only counts above `root_bits` are
/// decremented. A complete tree initializes table[0..returned_size]; incomplete
/// trees retain untouched holes. Returning an extent does not initialize them.
pub fn brotli_build_huffman_table(
    root_table: &mut [MaybeUninit<HuffmanCode>],
    root_bits: usize,
    symbol_lists: &[u16],
    origin: usize,
    count: &mut [u16],
) -> u32 {
    brotli_build_huffman_table_with_symbols(
        root_table,
        root_bits,
        |index| symbol_lists[origin.checked_add_signed(index).unwrap()],
        count,
    )
}

/// Builds the same table from a borrowed, indexed view of symbol storage.
///
/// `symbol_at` reads the original signed index relative to symbol zero. Only
/// reached length heads and forward links are requested, in source order; a
/// final symbol's unused link and zero-length symbols are never read. Every
/// returned word must be initialized. This permits a decoder arena to borrow
/// only the source-initialized slots without initializing the whole backing
/// array. The getter is monomorphized and requires no closure allocation.
///
/// The count/table/root preconditions and output extent are identical to
/// `brotli_build_huffman_table`. An incomplete tree can leave holes within
/// that extent untouched; returning an extent does not initialize those holes.
pub fn brotli_build_huffman_table_with_symbols(
    root_table: &mut [MaybeUninit<HuffmanCode>],
    root_bits: usize,
    mut symbol_at: impl FnMut(isize) -> u16,
    count: &mut [u16],
) -> u32 {
    debug_assert!(root_bits <= BROTLI_REVERSE_BITS_MAX);
    debug_assert!(BROTLI_HUFFMAN_MAX_CODE_LENGTH - root_bits <= BROTLI_REVERSE_BITS_MAX);
    let mut max_length = -1isize;
    while symbol_at(max_length) == 0xffff {
        max_length -= 1;
    }
    let max_length = (max_length + BROTLI_HUFFMAN_MAX_CODE_LENGTH as isize + 1) as usize;
    let mut table_offset = 0usize;
    let mut table_bits = root_bits;
    let mut table_size = 1usize << table_bits;
    let mut total_size = table_size;
    if table_bits > max_length {
        table_bits = max_length;
        table_size = 1usize << table_bits;
    }
    let mut key = 0usize;
    let mut key_step = BROTLI_REVERSE_BITS_LOWEST;
    let mut bits = 1usize;
    let mut step = 2usize;
    loop {
        let mut symbol = bits as isize - (BROTLI_HUFFMAN_MAX_CODE_LENGTH + 1) as isize;
        let mut bits_count = count[bits];
        while bits_count != 0 {
            symbol = symbol_at(symbol) as isize;
            let code = HuffmanCode::new(bits as u8, symbol as u16);
            replicate_value(
                &mut root_table[brotli_reverse_bits(key)..],
                step,
                table_size,
                code,
            );
            key = key.wrapping_add(key_step);
            bits_count -= 1;
        }
        step <<= 1;
        key_step >>= 1;
        bits += 1;
        if bits > table_bits {
            break;
        }
    }
    while total_size != table_size {
        root_table.copy_within(0..table_size, table_size);
        table_size <<= 1;
    }
    key_step = BROTLI_REVERSE_BITS_LOWEST >> (root_bits - 1);
    let sub_key_limit = BROTLI_REVERSE_BITS_LOWEST.wrapping_shl(1);
    let mut sub_key = sub_key_limit;
    let mut sub_key_step = BROTLI_REVERSE_BITS_LOWEST;
    step = 2;
    for len in root_bits + 1..=max_length {
        let mut symbol = len as isize - (BROTLI_HUFFMAN_MAX_CODE_LENGTH + 1) as isize;
        while count[len] != 0 {
            if sub_key == sub_key_limit {
                table_offset += table_size;
                table_bits = next_table_bit_size(count, len, root_bits);
                table_size = 1usize << table_bits;
                total_size += table_size;
                sub_key = brotli_reverse_bits(key);
                key = key.wrapping_add(key_step);
                root_table[sub_key].write(HuffmanCode::new(
                    (table_bits + root_bits) as u8,
                    table_offset.wrapping_sub(sub_key) as u16,
                ));
                sub_key = 0;
            }
            symbol = symbol_at(symbol) as isize;
            let code = HuffmanCode::new((len - root_bits) as u8, symbol as u16);
            replicate_value(
                &mut root_table[table_offset + brotli_reverse_bits(sub_key)..],
                step,
                table_size,
                code,
            );
            sub_key = sub_key.wrapping_add(sub_key_step);
            count[len] -= 1;
        }
        step <<= 1;
        sub_key_step >>= 1;
    }
    total_size as u32
}

/// Builds the original simple table, including in-place symbol sorting.
///
/// The root must contain the selected pattern (0/1/2/2/3 minimum bits for
/// cases 0..=4), and the borrowed table must cover 1 << root_bits entries.
/// Only the reached val prefix (1/2/3/4/4 words) must be initialized. Known
/// cases initialize table[0..returned_size] and leave the remaining storage
/// untouched. Unknown cases retain the source switch behavior: the prior first
/// slot is replicated, with its prior initialization state.
pub fn brotli_build_simple_huffman_table(
    table: &mut [MaybeUninit<HuffmanCode>],
    root_bits: usize,
    val: &mut [u16],
    num_symbols: u32,
) -> u32 {
    let mut table_size = 1usize;
    let goal_size = 1usize << root_bits;
    match num_symbols {
        0 => {
            table[0].write(HuffmanCode::new(0, val[0]));
        }
        1 => {
            if val[1] > val[0] {
                table[0].write(HuffmanCode::new(1, val[0]));
                table[1].write(HuffmanCode::new(1, val[1]));
            } else {
                table[0].write(HuffmanCode::new(1, val[1]));
                table[1].write(HuffmanCode::new(1, val[0]));
            }
            table_size = 2;
        }
        2 => {
            table[0].write(HuffmanCode::new(1, val[0]));
            table[2].write(HuffmanCode::new(1, val[0]));
            if val[2] > val[1] {
                table[1].write(HuffmanCode::new(2, val[1]));
                table[3].write(HuffmanCode::new(2, val[2]));
            } else {
                table[1].write(HuffmanCode::new(2, val[2]));
                table[3].write(HuffmanCode::new(2, val[1]));
            }
            table_size = 4;
        }
        3 => {
            for i in 0..3 {
                for k in i + 1..4 {
                    if val[k] < val[i] {
                        val.swap(k, i);
                    }
                }
            }
            table[0].write(HuffmanCode::new(2, val[0]));
            table[2].write(HuffmanCode::new(2, val[1]));
            table[1].write(HuffmanCode::new(2, val[2]));
            table[3].write(HuffmanCode::new(2, val[3]));
            table_size = 4;
        }
        4 => {
            if val[3] < val[2] {
                val.swap(3, 2);
            }
            table[0].write(HuffmanCode::new(1, val[0]));
            table[1].write(HuffmanCode::new(2, val[1]));
            table[2].write(HuffmanCode::new(1, val[0]));
            table[3].write(HuffmanCode::new(3, val[2]));
            table[4].write(HuffmanCode::new(1, val[0]));
            table[5].write(HuffmanCode::new(2, val[1]));
            table[6].write(HuffmanCode::new(1, val[0]));
            table[7].write(HuffmanCode::new(3, val[3]));
            table_size = 8;
        }
        _ => {
            // The source switch has no default: retain the caller's first slot.
        }
    }
    while table_size != goal_size {
        table.copy_within(0..table_size, table_size);
        table_size <<= 1;
    }
    goal_size as u32
}

#[cfg(not(target_arch = "aarch64"))]
const REVERSE_BITS: [u8; 256] = [
    0x00, 0x80, 0x40, 0xC0, 0x20, 0xA0, 0x60, 0xE0, 0x10, 0x90, 0x50, 0xD0, 0x30, 0xB0, 0x70, 0xF0,
    0x08, 0x88, 0x48, 0xC8, 0x28, 0xA8, 0x68, 0xE8, 0x18, 0x98, 0x58, 0xD8, 0x38, 0xB8, 0x78, 0xF8,
    0x04, 0x84, 0x44, 0xC4, 0x24, 0xA4, 0x64, 0xE4, 0x14, 0x94, 0x54, 0xD4, 0x34, 0xB4, 0x74, 0xF4,
    0x0C, 0x8C, 0x4C, 0xCC, 0x2C, 0xAC, 0x6C, 0xEC, 0x1C, 0x9C, 0x5C, 0xDC, 0x3C, 0xBC, 0x7C, 0xFC,
    0x02, 0x82, 0x42, 0xC2, 0x22, 0xA2, 0x62, 0xE2, 0x12, 0x92, 0x52, 0xD2, 0x32, 0xB2, 0x72, 0xF2,
    0x0A, 0x8A, 0x4A, 0xCA, 0x2A, 0xAA, 0x6A, 0xEA, 0x1A, 0x9A, 0x5A, 0xDA, 0x3A, 0xBA, 0x7A, 0xFA,
    0x06, 0x86, 0x46, 0xC6, 0x26, 0xA6, 0x66, 0xE6, 0x16, 0x96, 0x56, 0xD6, 0x36, 0xB6, 0x76, 0xF6,
    0x0E, 0x8E, 0x4E, 0xCE, 0x2E, 0xAE, 0x6E, 0xEE, 0x1E, 0x9E, 0x5E, 0xDE, 0x3E, 0xBE, 0x7E, 0xFE,
    0x01, 0x81, 0x41, 0xC1, 0x21, 0xA1, 0x61, 0xE1, 0x11, 0x91, 0x51, 0xD1, 0x31, 0xB1, 0x71, 0xF1,
    0x09, 0x89, 0x49, 0xC9, 0x29, 0xA9, 0x69, 0xE9, 0x19, 0x99, 0x59, 0xD9, 0x39, 0xB9, 0x79, 0xF9,
    0x05, 0x85, 0x45, 0xC5, 0x25, 0xA5, 0x65, 0xE5, 0x15, 0x95, 0x55, 0xD5, 0x35, 0xB5, 0x75, 0xF5,
    0x0D, 0x8D, 0x4D, 0xCD, 0x2D, 0xAD, 0x6D, 0xED, 0x1D, 0x9D, 0x5D, 0xDD, 0x3D, 0xBD, 0x7D, 0xFD,
    0x03, 0x83, 0x43, 0xC3, 0x23, 0xA3, 0x63, 0xE3, 0x13, 0x93, 0x53, 0xD3, 0x33, 0xB3, 0x73, 0xF3,
    0x0B, 0x8B, 0x4B, 0xCB, 0x2B, 0xAB, 0x6B, 0xEB, 0x1B, 0x9B, 0x5B, 0xDB, 0x3B, 0xBB, 0x7B, 0xFB,
    0x07, 0x87, 0x47, 0xC7, 0x27, 0xA7, 0x67, 0xE7, 0x17, 0x97, 0x57, 0xD7, 0x37, 0xB7, 0x77, 0xF7,
    0x0F, 0x8F, 0x4F, 0xCF, 0x2F, 0xAF, 0x6F, 0xEF, 0x1F, 0x9F, 0x5F, 0xDF, 0x3F, 0xBF, 0x7F, 0xFF,
];
