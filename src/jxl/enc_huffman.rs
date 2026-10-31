// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/enc_huffman.{h,cc}`.
//! The temporary arena is initialized so its borrowed slices are valid Rust.
//! Only the populated prefix is read, as in the source. See ADR 0003.

use super::base::status::{Status, StatusCode};
use super::enc_bit_writer::BitWriter;
use super::enc_huffman_tree::{
    convert_bit_depths_to_symbols, create_huffman_tree, write_huffman_tree,
};

const CODE_LENGTH_CODES: usize = 18;

fn store_huffman_tree_of_huffman_tree_to_bit_mask(
    num_codes: i32,
    code_length_bitdepth: &[u8; CODE_LENGTH_CODES],
    writer: &mut BitWriter,
) {
    const STORAGE_ORDER: [usize; CODE_LENGTH_CODES] =
        [1, 2, 3, 4, 0, 5, 17, 6, 16, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    const SYMBOLS: [u8; 6] = [0, 7, 3, 2, 1, 15];
    const BIT_LENGTHS: [usize; 6] = [2, 4, 3, 2, 2, 4];
    let mut codes_to_store = CODE_LENGTH_CODES;
    if num_codes > 1 {
        while codes_to_store > 0 {
            if code_length_bitdepth[STORAGE_ORDER[codes_to_store - 1]] != 0 {
                break;
            }
            codes_to_store -= 1;
        }
    }
    let mut skip_some = 0;
    if code_length_bitdepth[STORAGE_ORDER[0]] == 0 && code_length_bitdepth[STORAGE_ORDER[1]] == 0 {
        skip_some = 2;
        if code_length_bitdepth[STORAGE_ORDER[2]] == 0 {
            skip_some = 3;
        }
    }
    writer.write(2, skip_some as u64);
    for &index in &STORAGE_ORDER[skip_some..codes_to_store] {
        let l = usize::from(code_length_bitdepth[index]);
        writer.write(BIT_LENGTHS[l], u64::from(SYMBOLS[l]));
    }
}

fn store_huffman_tree_to_bit_mask(
    huffman_tree: &[u8],
    huffman_tree_extra_bits: &[u8],
    code_length_bitdepth: &[u8; CODE_LENGTH_CODES],
    code_length_bitdepth_symbols: &[u16; CODE_LENGTH_CODES],
    writer: &mut BitWriter,
) -> Status {
    for (i, &value) in huffman_tree.iter().enumerate() {
        let ix = usize::from(value);
        writer.write(
            usize::from(code_length_bitdepth[ix]),
            u64::from(code_length_bitdepth_symbols[ix]),
        );
        if ix > 17 {
            return Err(StatusCode::GenericError);
        }
        match ix {
            16 => writer.write(2, u64::from(huffman_tree_extra_bits[i])),
            17 => writer.write(3, u64::from(huffman_tree_extra_bits[i])),
            _ => {}
        }
    }
    Ok(())
}

fn store_simple_huffman_tree(
    depths: &[u8],
    symbols: &mut [usize; 4],
    num_symbols: usize,
    max_bits: usize,
    writer: &mut BitWriter,
) {
    writer.write(2, 1);
    writer.write(2, (num_symbols - 1) as u64);
    for i in 0..num_symbols {
        for j in i + 1..num_symbols {
            if depths[symbols[j]] < depths[symbols[i]] {
                symbols.swap(i, j);
            }
        }
    }
    if num_symbols == 2 {
        writer.write(max_bits, symbols[0] as u64);
        writer.write(max_bits, symbols[1] as u64);
    } else if num_symbols == 3 {
        writer.write(max_bits, symbols[0] as u64);
        writer.write(max_bits, symbols[1] as u64);
        writer.write(max_bits, symbols[2] as u64);
    } else {
        writer.write(max_bits, symbols[0] as u64);
        writer.write(max_bits, symbols[1] as u64);
        writer.write(max_bits, symbols[2] as u64);
        writer.write(max_bits, symbols[3] as u64);
        writer.write(1, u64::from(depths[symbols[0]] == 1));
    }
}

fn store_huffman_tree(depths: &[u8], writer: &mut BitWriter) -> Status {
    let mut arena = vec![0u8; 2 * depths.len()];
    let (huffman_tree, huffman_tree_extra_bits) = arena.split_at_mut(depths.len());
    let mut huffman_tree_size = 0;
    write_huffman_tree(
        depths,
        &mut huffman_tree_size,
        huffman_tree,
        huffman_tree_extra_bits,
    );
    let huffman_tree = &huffman_tree[..huffman_tree_size];
    let mut histogram = [0u32; CODE_LENGTH_CODES];
    for &value in huffman_tree {
        histogram[usize::from(value)] += 1;
    }
    let mut num_codes = 0;
    let mut code = 0;
    for (i, &count) in histogram.iter().enumerate() {
        if count != 0 {
            if num_codes == 0 {
                code = i;
                num_codes = 1;
            } else if num_codes == 1 {
                num_codes = 2;
                break;
            }
        }
    }
    let mut code_length_bitdepth = [0u8; CODE_LENGTH_CODES];
    let mut code_length_bitdepth_symbols = [0u16; CODE_LENGTH_CODES];
    create_huffman_tree(&histogram, 5, &mut code_length_bitdepth);
    convert_bit_depths_to_symbols(&code_length_bitdepth, &mut code_length_bitdepth_symbols);
    store_huffman_tree_of_huffman_tree_to_bit_mask(num_codes, &code_length_bitdepth, writer);
    if num_codes == 1 {
        code_length_bitdepth[code] = 0;
    }
    store_huffman_tree_to_bit_mask(
        huffman_tree,
        huffman_tree_extra_bits,
        &code_length_bitdepth,
        &code_length_bitdepth_symbols,
        writer,
    )
}

/// The histogram must be nonempty, and output arrays must be initialized with
/// zero. The caller must create a writer allotment before calling this helper.
pub fn build_and_store_huffman_tree(
    histogram: &[u32],
    depth: &mut [u8],
    bits: &mut [u16],
    writer: &mut BitWriter,
) -> Status {
    assert!(!histogram.is_empty());
    assert!(depth.len() >= histogram.len() && bits.len() >= histogram.len());
    let mut count = 0;
    let mut s4 = [0usize; 4];
    for (i, &value) in histogram.iter().enumerate() {
        if value != 0 {
            if count < 4 {
                s4[count] = i;
            } else if count > 4 {
                break;
            }
            count += 1;
        }
    }
    let mut max_bits_counter = histogram.len() - 1;
    let mut max_bits = 0;
    while max_bits_counter != 0 {
        max_bits_counter >>= 1;
        max_bits += 1;
    }
    if count <= 1 {
        writer.write(4, 1);
        writer.write(max_bits, s4[0] as u64);
        return Ok(());
    }
    create_huffman_tree(histogram, 15, depth);
    convert_bit_depths_to_symbols(&depth[..histogram.len()], bits);
    if count <= 4 {
        store_simple_huffman_tree(depth, &mut s4, count, max_bits, writer);
    } else {
        store_huffman_tree(&depth[..histogram.len()], writer)?;
    }
    Ok(())
}
