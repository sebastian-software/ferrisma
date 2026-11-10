// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Mechanical translation of c/dec/decode.c:361-519. The native source's
//! separate bits/value loads remain separate borrowed table operations.

use super::bit_reader::{BitReader, Reg, bit_mask};
use super::huffman::HuffmanCode;

const HUFFMAN_TABLE_BITS: Reg = 8;
const HUFFMAN_TABLE_MASK: Reg = 0xff;

/// A view of the actual table owner. Each method may read only its reached
/// field/index; the unused entries and the other field need not be initialized.
/// Implementations retain the source table origin and do not copy its payload.
pub trait HuffmanTable {
    fn bits(&self, index: usize) -> u8;
    fn value(&self, index: usize) -> u16;
}

impl HuffmanTable for [HuffmanCode] {
    #[inline(always)]
    fn bits(&self, index: usize) -> u8 {
        self[index].bits
    }
    #[inline(always)]
    fn value(&self, index: usize) -> u16 {
        self[index].value
    }
}

/// `bits` contains at least 15 valid bits; the reader has the decoded code's
/// number of bits available. The table follows the source's root-eight layout.
#[inline(always)]
pub fn decode_symbol<T: HuffmanTable + ?Sized>(
    bits: Reg,
    table: &T,
    br: &mut BitReader<'_>,
) -> Reg {
    let mut index = (bits & HUFFMAN_TABLE_MASK) as usize;
    if Reg::from(table.bits(index)) > HUFFMAN_TABLE_BITS {
        let nbits = Reg::from(table.bits(index)) - HUFFMAN_TABLE_BITS;
        br.drop_bits(HUFFMAN_TABLE_BITS);
        index += (Reg::from(table.value(index)) + ((bits >> HUFFMAN_TABLE_BITS) & bit_mask(nbits)))
            as usize;
    }
    br.drop_bits(Reg::from(table.bits(index)));
    Reg::from(table.value(index))
}

/// The caller supplies the source fast-reader backing/slack precondition.
#[inline(always)]
pub fn read_symbol<T: HuffmanTable + ?Sized>(table: &T, br: &mut BitReader<'_>) -> Reg {
    decode_symbol(br.get16_bits_unmasked(), table, br)
}

/// The source caller has fewer than 15 bits available. Failure leaves `result`
/// and reader bits unchanged; no speculative table field is read eagerly.
#[inline(never)]
pub fn safe_decode_symbol<T: HuffmanTable + ?Sized>(
    table: &T,
    br: &mut BitReader<'_>,
    result: &mut Reg,
) -> bool {
    let mut available_bits = br.available_bits();
    let mut index = 0usize;
    if available_bits == 0 {
        if table.bits(index) == 0 {
            *result = Reg::from(table.value(index));
            return true;
        }
        return false;
    }
    let mut val = br.bits_unmasked();
    index += (val & HUFFMAN_TABLE_MASK) as usize;
    if Reg::from(table.bits(index)) <= HUFFMAN_TABLE_BITS {
        if Reg::from(table.bits(index)) <= available_bits {
            br.drop_bits(Reg::from(table.bits(index)));
            *result = Reg::from(table.value(index));
            return true;
        } else {
            return false;
        }
    }
    if available_bits <= HUFFMAN_TABLE_BITS {
        return false;
    }
    val = (val & bit_mask(Reg::from(table.bits(index)))) >> HUFFMAN_TABLE_BITS;
    available_bits -= HUFFMAN_TABLE_BITS;
    index += (Reg::from(table.value(index)) + val) as usize;
    if available_bits < Reg::from(table.bits(index)) {
        return false;
    }
    br.drop_bits(HUFFMAN_TABLE_BITS + Reg::from(table.bits(index)));
    *result = Reg::from(table.value(index));
    true
}

#[inline(always)]
pub fn safe_read_symbol<T: HuffmanTable + ?Sized>(
    table: &T,
    br: &mut BitReader<'_>,
    result: &mut Reg,
) -> bool {
    let mut val = 0;
    if br.safe_get_bits(15, &mut val) {
        *result = decode_symbol(val, table, br);
        return true;
    }
    safe_decode_symbol(table, br, result)
}

#[inline(always)]
pub fn preload_symbol<T: HuffmanTable + ?Sized>(
    safe: i32,
    table: &T,
    br: &mut BitReader<'_>,
    bits: &mut Reg,
    value: &mut Reg,
) {
    if safe != 0 {
        return;
    }
    let index = br.get_bits_const::<8>() as usize;
    *bits = Reg::from(table.bits(index));
    *value = Reg::from(table.value(index));
}

/// `bits` and `value` are the source's actual preloaded first-level fields.
/// The caller supplies backing for both the current read and following peek.
#[inline(always)]
pub fn read_preloaded_symbol<T: HuffmanTable + ?Sized>(
    table: &T,
    br: &mut BitReader<'_>,
    bits: &mut Reg,
    value: &mut Reg,
) -> Reg {
    let mut result = *value;
    if *bits > HUFFMAN_TABLE_BITS {
        let val = br.get16_bits_unmasked();
        let mut ext = ((val & HUFFMAN_TABLE_MASK) + *value) as usize;
        let mask = bit_mask(*bits - HUFFMAN_TABLE_BITS);
        br.drop_bits(HUFFMAN_TABLE_BITS);
        ext += ((val >> HUFFMAN_TABLE_BITS) & mask) as usize;
        br.drop_bits(Reg::from(table.bits(ext)));
        result = Reg::from(table.value(ext));
    } else {
        br.drop_bits(*bits);
    }
    preload_symbol(0, table, br, bits, value);
    result
}

/// Writes directly into the caller's actual destination at the source signed
/// indices. Each reached index must be writable, and source signed arithmetic
/// must be representable. The writer must not mutate the borrowed table/input.
#[inline(always)]
pub fn copy_preloaded_symbols_to_u8_with_writer<T: HuffmanTable + ?Sized>(
    table: &T,
    br: &mut BitReader<'_>,
    bits: &mut Reg,
    value: &mut Reg,
    mut write: impl FnMut(i32, u8),
    mut pos: i32,
    limit: i32,
) -> i32 {
    let mut new_lim =
        i64::try_from(br.guard_position()).unwrap() - i64::try_from(br.position()).unwrap();
    new_lim *= 8;
    new_lim /= 15;
    let k_maximal_overread = 4;
    let mut pos_limit = limit;
    if new_lim - k_maximal_overread <= i64::from(limit) {
        pos_limit = (new_lim - k_maximal_overread) as i32;
    }
    if pos_limit < 0 {
        pos_limit = 0;
    }
    let mut copies = pos_limit;
    pos_limit += pos;
    while pos < pos_limit {
        debug_assert!(br.check_input_amount());
        write(pos, read_preloaded_symbol(table, br, bits, value) as u8);
        pos += 1;
    }
    while br.check_input_amount() && copies < limit {
        write(pos, read_preloaded_symbol(table, br, bits, value) as u8);
        pos += 1;
        copies += 1;
    }
    copies
}

#[inline(always)]
pub fn copy_preloaded_symbols_to_u8<T: HuffmanTable + ?Sized>(
    table: &T,
    br: &mut BitReader<'_>,
    bits: &mut Reg,
    value: &mut Reg,
    destination: &mut [u8],
    pos: i32,
    limit: i32,
) -> i32 {
    copy_preloaded_symbols_to_u8_with_writer(
        table,
        br,
        bits,
        value,
        |index, byte| destination[usize::try_from(index).unwrap()] = byte,
        pos,
        limit,
    )
}
