// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/dec_huffman.{h,cc}`.
//! Table offsets replace pointers. Preserve partial table state, deferred
//! reader bounds checks, and the caller's refill obligation for symbol reads.

use super::ans_params::PREFIX_MAX_BITS;
use super::base::bits::floor_log2_nonzero;
use super::dec_bit_reader::BitReader;
use super::huffman_table::{HuffmanCode, build_huffman_table};

pub const HUFFMAN_TABLE_BITS: usize = 8;
const CODE_LENGTH_CODES: usize = 18;
const CODE_LENGTH_CODE_ORDER: [usize; CODE_LENGTH_CODES] =
    [1, 2, 3, 4, 0, 5, 17, 6, 16, 7, 8, 9, 10, 11, 12, 13, 14, 15];
const DEFAULT_CODE_LENGTH: u8 = 8;
const CODE_LENGTH_REPEAT_CODE: u8 = 16;

/// The code-length alphabet must describe a complete tree or a single symbol,
/// with depths at most five, as checked by `read_from_bit_stream`.
pub fn read_huffman_code_lengths(
    code_length_code_lengths: &[u8; CODE_LENGTH_CODES],
    code_lengths: &mut [u8],
    br: &mut BitReader<'_>,
) -> bool {
    let num_symbols = code_lengths.len() as i32;
    let mut symbol = 0i32;
    let mut prev_code_len = DEFAULT_CODE_LENGTH;
    let mut repeat = 0i32;
    let mut repeat_code_len = 0u8;
    let mut space = 32768i32;
    let mut table = [HuffmanCode::default(); 32];
    let mut counts = [0u16; 16];
    for &length in code_length_code_lengths {
        counts[usize::from(length)] += 1;
    }
    if build_huffman_table(&mut table, 5, code_length_code_lengths, &mut counts) == 0 {
        return false;
    }
    while symbol < num_symbols && space > 0 {
        br.refill();
        let p = table[br.peek_fixed_bits::<5>() as usize];
        br.consume(usize::from(p.bits));
        let code_len = p.value as u8;
        if code_len < CODE_LENGTH_REPEAT_CODE {
            repeat = 0;
            code_lengths[symbol as usize] = code_len;
            symbol += 1;
            if code_len != 0 {
                prev_code_len = code_len;
                space = (space as u32).wrapping_sub(32768u32 >> code_len) as i32;
            }
        } else {
            let extra_bits = code_len - 14;
            let new_len = if code_len == CODE_LENGTH_REPEAT_CODE {
                prev_code_len
            } else {
                0
            };
            if repeat_code_len != new_len {
                repeat = 0;
                repeat_code_len = new_len;
            }
            let old_repeat = repeat;
            if repeat > 0 {
                repeat -= 2;
                repeat <<= extra_bits;
            }
            repeat += (br.read_bits(usize::from(extra_bits)) + 3) as i32;
            let repeat_delta = repeat - old_repeat;
            if symbol + repeat_delta > num_symbols {
                return false;
            }
            code_lengths[symbol as usize..(symbol + repeat_delta) as usize].fill(repeat_code_len);
            symbol += repeat_delta;
            if repeat_code_len != 0 {
                space -= repeat_delta << (15 - repeat_code_len);
            }
        }
    }
    if space != 0 {
        return false;
    }
    code_lengths[symbol as usize..].fill(0);
    true
}

fn read_simple_code(
    alphabet_size: usize,
    br: &mut BitReader<'_>,
    table: &mut [HuffmanCode],
) -> bool {
    let max_bits = if alphabet_size > 1 {
        floor_log2_nonzero(alphabet_size - 1) + 1
    } else {
        0
    };
    let mut num_symbols = br.read_fixed_bits::<2>() as usize + 1;
    let mut symbols = [0u16; 4];
    for output in &mut symbols[..num_symbols] {
        let symbol = br.read_bits(max_bits) as u16;
        if usize::from(symbol) >= alphabet_size {
            return false;
        }
        *output = symbol;
    }
    for i in 0..num_symbols - 1 {
        for j in i + 1..num_symbols {
            if symbols[i] == symbols[j] {
                return false;
            }
        }
    }
    if num_symbols == 4 {
        num_symbols += br.read_fixed_bits::<1>() as usize;
    }
    let mut table_size = 1;
    match num_symbols {
        1 => {
            table[0] = HuffmanCode {
                bits: 0,
                value: symbols[0],
            }
        }
        2 => {
            if symbols[0] > symbols[1] {
                symbols.swap(0, 1);
            }
            table[0] = HuffmanCode {
                bits: 1,
                value: symbols[0],
            };
            table[1] = HuffmanCode {
                bits: 1,
                value: symbols[1],
            };
            table_size = 2;
        }
        3 => {
            if symbols[1] > symbols[2] {
                symbols.swap(1, 2);
            }
            table[0] = HuffmanCode {
                bits: 1,
                value: symbols[0],
            };
            table[2] = table[0];
            table[1] = HuffmanCode {
                bits: 2,
                value: symbols[1],
            };
            table[3] = HuffmanCode {
                bits: 2,
                value: symbols[2],
            };
            table_size = 4;
        }
        4 => {
            for i in 0..3 {
                for j in i + 1..4 {
                    if symbols[i] > symbols[j] {
                        symbols.swap(i, j);
                    }
                }
            }
            table[0] = HuffmanCode {
                bits: 2,
                value: symbols[0],
            };
            table[2] = HuffmanCode {
                bits: 2,
                value: symbols[1],
            };
            table[1] = HuffmanCode {
                bits: 2,
                value: symbols[2],
            };
            table[3] = HuffmanCode {
                bits: 2,
                value: symbols[3],
            };
            table_size = 4;
        }
        5 => {
            if symbols[2] > symbols[3] {
                symbols.swap(2, 3);
            }
            table[0] = HuffmanCode {
                bits: 1,
                value: symbols[0],
            };
            table[1] = HuffmanCode {
                bits: 2,
                value: symbols[1],
            };
            table[2] = table[0];
            table[3] = HuffmanCode {
                bits: 3,
                value: symbols[2],
            };
            table[4] = table[0];
            table[5] = table[1];
            table[6] = table[0];
            table[7] = HuffmanCode {
                bits: 3,
                value: symbols[3],
            };
            table_size = 8;
        }
        _ => return false,
    }
    while table_size != 1 << HUFFMAN_TABLE_BITS {
        table.copy_within(..table_size, table_size);
        table_size <<= 1;
    }
    true
}

#[derive(Default)]
pub struct HuffmanDecodingData {
    pub table: Vec<HuffmanCode>,
}

impl HuffmanDecodingData {
    /// The caller must refill the reader first and have a valid decoding table.
    pub fn read_symbol(&self, br: &mut BitReader<'_>) -> u16 {
        let mut index = br.peek_bits(HUFFMAN_TABLE_BITS) as usize;
        let mut n_bits = usize::from(self.table[index].bits);
        if n_bits > HUFFMAN_TABLE_BITS {
            br.consume(HUFFMAN_TABLE_BITS);
            n_bits -= HUFFMAN_TABLE_BITS;
            index += usize::from(self.table[index].value) + br.peek_bits(n_bits) as usize;
        }
        br.consume(usize::from(self.table[index].bits));
        self.table[index].value
    }

    pub fn read_from_bit_stream(&mut self, alphabet_size: usize, br: &mut BitReader<'_>) -> bool {
        if alphabet_size > 1 << PREFIX_MAX_BITS {
            return false;
        }
        let simple_code_or_skip = br.read_fixed_bits::<2>() as usize;
        if simple_code_or_skip == 1 {
            self.table
                .resize(1 << HUFFMAN_TABLE_BITS, HuffmanCode::default());
            return read_simple_code(alphabet_size, br, &mut self.table);
        }
        let mut code_lengths = vec![0u8; alphabet_size];
        let mut code_length_code_lengths = [0u8; CODE_LENGTH_CODES];
        let mut space = 32i32;
        let mut num_codes = 0;
        const HUFF: [HuffmanCode; 16] = [
            HuffmanCode { bits: 2, value: 0 },
            HuffmanCode { bits: 2, value: 4 },
            HuffmanCode { bits: 2, value: 3 },
            HuffmanCode { bits: 3, value: 2 },
            HuffmanCode { bits: 2, value: 0 },
            HuffmanCode { bits: 2, value: 4 },
            HuffmanCode { bits: 2, value: 3 },
            HuffmanCode { bits: 4, value: 1 },
            HuffmanCode { bits: 2, value: 0 },
            HuffmanCode { bits: 2, value: 4 },
            HuffmanCode { bits: 2, value: 3 },
            HuffmanCode { bits: 3, value: 2 },
            HuffmanCode { bits: 2, value: 0 },
            HuffmanCode { bits: 2, value: 4 },
            HuffmanCode { bits: 2, value: 3 },
            HuffmanCode { bits: 4, value: 5 },
        ];
        for &index in &CODE_LENGTH_CODE_ORDER[simple_code_or_skip..] {
            if space <= 0 {
                break;
            }
            br.refill();
            let p = HUFF[br.peek_fixed_bits::<4>() as usize];
            br.consume(usize::from(p.bits));
            let v = p.value as u8;
            code_length_code_lengths[index] = v;
            if v != 0 {
                space = (space as u32).wrapping_sub(32u32 >> v) as i32;
                num_codes += 1;
            }
        }
        let ok = (num_codes == 1 || space == 0)
            && read_huffman_code_lengths(&code_length_code_lengths, &mut code_lengths, br);
        if !ok {
            return false;
        }
        let mut counts = [0u16; 16];
        for &length in &code_lengths {
            counts[usize::from(length)] += 1;
        }
        self.table
            .resize(alphabet_size + 376, HuffmanCode::default());
        let table_size = build_huffman_table(
            &mut self.table,
            HUFFMAN_TABLE_BITS,
            &code_lengths,
            &mut counts,
        );
        self.table
            .resize(table_size as usize, HuffmanCode::default());
        table_size > 0
    }
}
