// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/huffman_table.{h,cc}`.
//! Slice offsets replace table pointers; callers supply enough initialized
//! storage and valid code lengths, as in the source's internal contract.

use super::ans_params::PREFIX_MAX_BITS;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HuffmanCode {
    pub bits: u8,
    pub value: u16,
}

fn get_next_key(key: i32, len: usize) -> i32 {
    let mut step = (1u32 << (len - 1)) as i32;
    while key & step != 0 {
        step >>= 1;
    }
    (key & (step - 1)) + step
}

fn replicate_value(table: &mut [HuffmanCode], step: i32, mut end: i32, code: HuffmanCode) {
    loop {
        end -= step;
        table[end as usize] = code;
        if end <= 0 {
            break;
        }
    }
}

fn next_table_bit_size(
    count: &[u16; PREFIX_MAX_BITS + 1],
    mut len: usize,
    root_bits: usize,
) -> usize {
    let mut left = 1usize << (len - root_bits);
    while len < PREFIX_MAX_BITS {
        if left <= usize::from(count[len]) {
            break;
        }
        left -= usize::from(count[len]);
        len += 1;
        left <<= 1;
    }
    len - root_bits
}

/// Builds the lookup table from code lengths in symbol order. `count` is
/// consumed in place. Invalid programmer preconditions panic instead of
/// accessing memory outside the caller's table.
pub fn build_huffman_table(
    root_table: &mut [HuffmanCode],
    root_bits: usize,
    code_lengths: &[u8],
    count: &mut [u16; PREFIX_MAX_BITS + 1],
) -> u32 {
    if code_lengths.len() > 1 << PREFIX_MAX_BITS {
        return 0;
    }
    // Source table sizes use a positive signed int after the unsigned shift.
    assert!(root_bits < i32::BITS as usize - 1);
    let mut sorted = vec![0u16; code_lengths.len()];
    let mut offset = [0u16; PREFIX_MAX_BITS + 1];
    let mut max_length = 1;
    let mut sum = 0u16;
    for len in 1..=PREFIX_MAX_BITS {
        offset[len] = sum;
        if count[len] != 0 {
            sum = sum.wrapping_add(count[len]);
            max_length = len;
        }
    }
    for (symbol, &length) in code_lengths.iter().enumerate() {
        if length != 0 {
            let length = usize::from(length);
            sorted[usize::from(offset[length])] = symbol as u16;
            offset[length] = offset[length].wrapping_add(1);
        }
    }

    let mut table = 0usize;
    let mut table_bits = root_bits;
    let mut table_size = (1u32 << table_bits) as i32;
    let mut total_size = table_size;
    if offset[PREFIX_MAX_BITS] == 1 {
        let code = HuffmanCode {
            bits: 0,
            value: sorted[0],
        };
        root_table[..total_size as usize].fill(code);
        return total_size as u32;
    }

    if table_bits > max_length {
        table_bits = max_length;
        table_size = (1u32 << table_bits) as i32;
    }
    let mut key = 0;
    let mut symbol = 0;
    let mut code = HuffmanCode { bits: 1, value: 0 };
    let mut step = 2;
    loop {
        while count[usize::from(code.bits)] != 0 {
            code.value = sorted[symbol];
            symbol += 1;
            replicate_value(&mut root_table[key as usize..], step, table_size, code);
            key = get_next_key(key, usize::from(code.bits));
            count[usize::from(code.bits)] -= 1;
        }
        step <<= 1;
        code.bits += 1;
        if usize::from(code.bits) > table_bits {
            break;
        }
    }
    while total_size != table_size {
        root_table.copy_within(..table_size as usize, table_size as usize);
        table_size <<= 1;
    }

    let mask = total_size - 1;
    let mut low = -1;
    step = 2;
    for len in root_bits + 1..=max_length {
        while count[len] != 0 {
            if key & mask != low {
                table += table_size as usize;
                table_bits = next_table_bit_size(count, len, root_bits);
                table_size = (1u32 << table_bits) as i32;
                total_size += table_size;
                low = key & mask;
                root_table[low as usize].bits = (table_bits + root_bits) as u8;
                root_table[low as usize].value = (table - low as usize) as u16;
            }
            code.bits = (len - root_bits) as u8;
            code.value = sorted[symbol];
            symbol += 1;
            replicate_value(
                &mut root_table[table + (key >> root_bits) as usize..],
                step,
                table_size,
                code,
            );
            key = get_next_key(key, len);
            count[len] -= 1;
        }
        step <<= 1;
    }
    total_size as u32
}
