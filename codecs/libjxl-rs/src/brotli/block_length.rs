// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
//! Mechanical translation of c/dec/decode.c:932-969.

use super::bit_reader::{BitReader, Reg};
use super::constants::BROTLI_PREFIX_CODE_RANGES;
use super::running::{BROTLI_STATE_READ_BLOCK_LENGTH_NONE, BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX};
use super::symbols::{HuffmanTable, read_symbol, safe_read_symbol};
use std::mem::MaybeUninit;

/// The source table decodes only indices 0..26. Fast reader backing/slack
/// satisfies the original symbol and dynamic ReadBits24 refill domains.
#[inline(always)]
pub fn read_block_length<T: HuffmanTable + ?Sized>(table: &T, br: &mut BitReader<'_>) -> Reg {
    let code = read_symbol(table, br);
    let nbits = Reg::from(BROTLI_PREFIX_CODE_RANGES[code as usize].nbits);
    Reg::from(BROTLI_PREFIX_CODE_RANGES[code as usize].offset).wrapping_add(br.read_bits24(nbits))
}

/// # Safety
/// `substate` is an initialized source raw state. For every non-NONE value,
/// `block_length_index` is source-initialized and in 0..26. NONE may leave that
/// index and result wholly uninitialized. These actual fields, input/table and
/// result are disjoint. A partial suffix must resume here, never with the fast
/// read_block_length helper, as in the source. Table getters retain the
/// source's reached field/index initialization preconditions.
#[inline(always)]
pub unsafe fn safe_read_block_length<T: HuffmanTable + ?Sized>(
    substate: &mut MaybeUninit<u32>,
    block_length_index: &mut MaybeUninit<Reg>,
    result: &mut MaybeUninit<Reg>,
    table: &T,
    br: &mut BitReader<'_>,
) -> bool {
    // SAFETY: source StateInit or the previous suffix checkpoint initialized
    // this raw state; no enum discriminant with a restricted domain is formed.
    let state = unsafe { substate.assume_init_read() };
    let index = if state == BROTLI_STATE_READ_BLOCK_LENGTH_NONE {
        // Existing symbol helper accepts an initialized scalar output. This
        // local dummy value is never observed/copied on failure and initializes
        // neither owner output; it does not justify an unknown &mut Reg borrow.
        let mut index = 0;
        if !safe_read_symbol(table, br, &mut index) {
            return false;
        }
        index
    } else {
        // SAFETY: source non-NONE entry retained the index at its suffix
        // checkpoint; no table field is accessed on this path.
        unsafe { block_length_index.assume_init_read() }
    };
    let nbits = Reg::from(BROTLI_PREFIX_CODE_RANGES[index as usize].nbits);
    let offset = Reg::from(BROTLI_PREFIX_CODE_RANGES[index as usize].offset);
    let mut bits = 0;
    if !br.safe_read_bits(nbits, &mut bits) {
        block_length_index.write(index);
        substate.write(BROTLI_STATE_READ_BLOCK_LENGTH_SUFFIX);
        return false;
    }
    result.write(offset.wrapping_add(bits));
    substate.write(BROTLI_STATE_READ_BLOCK_LENGTH_NONE);
    true
}
