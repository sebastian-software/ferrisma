// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! c/dec/decode.c window, variable-byte and metablock-header readers.
//! Field loans reference the actual native owner. Unknown source fields stay
//! MaybeUninit until their reached assignment; no state payload is copied.

use super::bit_reader::{BitReader, Reg};
use super::bitfield::BitfieldWord;
use super::running::*;
use super::status::*;
use std::mem::MaybeUninit;

fn flags(storage: &mut MaybeUninit<u32>) -> BitfieldWord<'_> {
    // SAFETY: MaybeUninit<u32> is an aligned live four-byte extent. This
    // exclusive byte view reads no u32, including its unknown padding bits.
    let bytes = unsafe {
        std::slice::from_raw_parts_mut(storage.as_mut_ptr().cast::<MaybeUninit<u8>>(), 4)
    };
    BitfieldWord::new(bytes.try_into().unwrap())
}

/// DecodeWindowBits. The original precondition requires at least eight bits
/// in the reader accumulator, and the large_window bit was initialized by
/// StateInit or SetParameter. Other packed bits may remain uninitialized.
///
/// # Safety
/// Those source initialization and accumulator preconditions must hold.
pub unsafe fn decode_window_bits(
    word: &mut MaybeUninit<u32>,
    br: &mut BitReader<'_>,
) -> BrotliDecoderErrorCode {
    let mut word = flags(word);
    // SAFETY: The caller proves initialization of exactly large_window.
    let large_window = unsafe { word.read(5, 1) };
    word.write(5, 1, 0);
    let mut n = br.take_bits(1);
    if n == 0 {
        word.write(6, 6, 16);
        return BROTLI_DECODER_SUCCESS;
    }
    n = br.take_bits(3);
    if n != 0 {
        word.write(6, 6, (17 + n) as u32 & 63);
        return BROTLI_DECODER_SUCCESS;
    }
    n = br.take_bits(3);
    if n == 1 {
        if large_window != 0 {
            n = br.take_bits(1);
            if n == 1 {
                return BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS;
            }
            word.write(5, 1, 1);
            return BROTLI_DECODER_SUCCESS;
        }
        return BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS;
    }
    if n != 0 {
        word.write(6, 6, (8 + n) as u32 & 63);
        return BROTLI_DECODER_SUCCESS;
    }
    word.write(6, 6, 17);
    BROTLI_DECODER_SUCCESS
}

/// DecodeVarLenUint8. LONG alone needs the prior output value initialized;
/// NONE/SHORT can start with genuinely uninitialized output storage.
///
/// # Safety
/// The substate and any prior LONG temporary have the source's initialization
/// history. The reader, substate and value are disjoint actual owner fields.
#[inline(never)]
pub unsafe fn decode_var_len_uint8(
    substate: &mut MaybeUninit<u32>,
    br: &mut BitReader<'_>,
    value: &mut MaybeUninit<Reg>,
) -> BrotliDecoderErrorCode {
    // SAFETY: Entry requires the initialized source discriminant.
    let mut state = unsafe { substate.assume_init_read() };
    let mut bits = 0;
    if state == BROTLI_STATE_DECODE_UINT8_NONE {
        if !br.safe_read_bits(1, &mut bits) {
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        if bits == 0 {
            value.write(0);
            return BROTLI_DECODER_SUCCESS;
        }
        state = BROTLI_STATE_DECODE_UINT8_SHORT;
    }
    if state == BROTLI_STATE_DECODE_UINT8_SHORT {
        if !br.safe_read_bits(3, &mut bits) {
            substate.write(BROTLI_STATE_DECODE_UINT8_SHORT);
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        if bits == 0 {
            value.write(1);
            substate.write(BROTLI_STATE_DECODE_UINT8_NONE);
            return BROTLI_DECODER_SUCCESS;
        }
        value.write(bits);
        state = BROTLI_STATE_DECODE_UINT8_LONG;
    }
    if state == BROTLI_STATE_DECODE_UINT8_LONG {
        // SAFETY: LONG is entered only after SHORT wrote this temporary or
        // after a valid LONG partial-input checkpoint retained that write.
        let previous = unsafe { value.assume_init_read() };
        if !br.safe_read_bits(previous, &mut bits) {
            substate.write(BROTLI_STATE_DECODE_UINT8_LONG);
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        // SAFETY: SafeReadBits does not alter the owner's initialized width;
        // preserve the original second *value access in the successful RHS.
        let previous = unsafe { value.assume_init_read() };
        value.write(((1 as Reg) << previous) + bits);
        substate.write(BROTLI_STATE_DECODE_UINT8_NONE);
        return BROTLI_DECODER_SUCCESS;
    }
    BROTLI_DECODER_ERROR_UNREACHABLE
}

/// Disjoint loans of only the source fields reached by DecodeMetaBlockLength.
pub struct MetaBlockFields<'a> {
    pub substate: &'a mut MaybeUninit<u32>,
    pub flags: &'a mut MaybeUninit<u32>,
    pub remaining_len: &'a mut MaybeUninit<i32>,
    pub loop_counter: &'a mut MaybeUninit<i32>,
}

/// # Safety
/// The initialized substate and the fields it reaches retain the original
/// StateInit/header resume history. NONE requires no initialized size_nibbles
/// or remaining_len. SIZE/METADATA retain the preceding initialized length,
/// loop counter and nibble field. All field loans and input are disjoint.
#[inline(never)]
pub unsafe fn decode_meta_block_length(
    fields: MetaBlockFields<'_>,
    br: &mut BitReader<'_>,
) -> BrotliDecoderErrorCode {
    let mut word = flags(fields.flags);
    let mut bits = 0;
    loop {
        // SAFETY: Entry or the preceding source transition initializes this
        // discriminant. Every read below follows the same source checkpoint.
        let mut state = unsafe { fields.substate.assume_init_read() };
        if state == BROTLI_STATE_METABLOCK_HEADER_NONE {
            if !br.safe_read_bits(1, &mut bits) {
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            word.write(0, 1, u32::from(bits != 0));
            fields.remaining_len.write(0);
            word.write(1, 1, 0);
            word.write(2, 1, 0);
            // SAFETY: The source just initialized is_last_metablock.
            if unsafe { word.read(0, 1) } == 0 {
                fields.substate.write(BROTLI_STATE_METABLOCK_HEADER_NIBBLES);
                continue;
            }
            fields.substate.write(BROTLI_STATE_METABLOCK_HEADER_EMPTY);
            state = BROTLI_STATE_METABLOCK_HEADER_EMPTY;
        }
        if state == BROTLI_STATE_METABLOCK_HEADER_EMPTY {
            if !br.safe_read_bits(1, &mut bits) {
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            if bits != 0 {
                fields.substate.write(BROTLI_STATE_METABLOCK_HEADER_NONE);
                return BROTLI_DECODER_SUCCESS;
            }
            fields.substate.write(BROTLI_STATE_METABLOCK_HEADER_NIBBLES);
            state = BROTLI_STATE_METABLOCK_HEADER_NIBBLES;
        }
        if state == BROTLI_STATE_METABLOCK_HEADER_NIBBLES {
            if !br.safe_read_bits(2, &mut bits) {
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            word.write(12, 8, (bits + 4) as u8 as u32);
            fields.loop_counter.write(0);
            if bits == 3 {
                word.write(2, 1, 1);
                fields
                    .substate
                    .write(BROTLI_STATE_METABLOCK_HEADER_RESERVED);
                continue;
            }
            fields.substate.write(BROTLI_STATE_METABLOCK_HEADER_SIZE);
            state = BROTLI_STATE_METABLOCK_HEADER_SIZE;
        }
        if state == BROTLI_STATE_METABLOCK_HEADER_SIZE {
            // SAFETY: SIZE follows NIBBLES or its own saved-input checkpoint.
            let mut i = unsafe { fields.loop_counter.assume_init_read() };
            // SAFETY: NIBBLES initialized size_nibbles before selecting SIZE.
            while i < unsafe { word.read(12, 8) } as i32 {
                if !br.safe_read_bits(4, &mut bits) {
                    fields.loop_counter.write(i);
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                // SAFETY: Same initialized nibble field in source order.
                if i + 1 == unsafe { word.read(12, 8) } as i32
                    && unsafe { word.read(12, 8) } > 4
                    && bits == 0
                {
                    return BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_NIBBLE;
                }
                // SAFETY: NONE zeroed length and every SIZE resume retains it.
                let length = unsafe { fields.remaining_len.assume_init_read() };
                fields
                    .remaining_len
                    .write(length | ((bits << (i * 4)) as i32));
                i += 1;
            }
            fields
                .substate
                .write(BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED);
            state = BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED;
        }
        if state == BROTLI_STATE_METABLOCK_HEADER_UNCOMPRESSED {
            // SAFETY: NONE initialized the selected last-block field.
            if unsafe { word.read(0, 1) } == 0 {
                if !br.safe_read_bits(1, &mut bits) {
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                word.write(1, 1, u32::from(bits != 0));
            }
            // SAFETY: NONE/SIZE initialized the complete valid source length.
            let length = unsafe { fields.remaining_len.assume_init_read() };
            fields.remaining_len.write(length + 1);
            fields.substate.write(BROTLI_STATE_METABLOCK_HEADER_NONE);
            return BROTLI_DECODER_SUCCESS;
        }
        if state == BROTLI_STATE_METABLOCK_HEADER_RESERVED {
            if !br.safe_read_bits(1, &mut bits) {
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            if bits != 0 {
                return BROTLI_DECODER_ERROR_FORMAT_RESERVED;
            }
            fields.substate.write(BROTLI_STATE_METABLOCK_HEADER_BYTES);
            state = BROTLI_STATE_METABLOCK_HEADER_BYTES;
        }
        if state == BROTLI_STATE_METABLOCK_HEADER_BYTES {
            if !br.safe_read_bits(2, &mut bits) {
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            if bits == 0 {
                fields.substate.write(BROTLI_STATE_METABLOCK_HEADER_NONE);
                return BROTLI_DECODER_SUCCESS;
            }
            word.write(12, 8, bits as u8 as u32);
            fields
                .substate
                .write(BROTLI_STATE_METABLOCK_HEADER_METADATA);
            state = BROTLI_STATE_METABLOCK_HEADER_METADATA;
        }
        if state == BROTLI_STATE_METABLOCK_HEADER_METADATA {
            // SAFETY: NIBBLES initialized the loop and BYTES initialized size.
            let mut i = unsafe { fields.loop_counter.assume_init_read() };
            while i < unsafe { word.read(12, 8) } as i32 {
                if !br.safe_read_bits(8, &mut bits) {
                    fields.loop_counter.write(i);
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                // SAFETY: Same initialized size field, read at source tests.
                if i + 1 == unsafe { word.read(12, 8) } as i32
                    && unsafe { word.read(12, 8) } > 1
                    && bits == 0
                {
                    return BROTLI_DECODER_ERROR_FORMAT_EXUBERANT_META_NIBBLE;
                }
                // SAFETY: NONE zeroed length before this metadata path.
                let length = unsafe { fields.remaining_len.assume_init_read() };
                fields
                    .remaining_len
                    .write(length | ((bits << (i * 8)) as i32));
                i += 1;
            }
            // SAFETY: Metadata reads preserve the initialized source length.
            let length = unsafe { fields.remaining_len.assume_init_read() };
            fields.remaining_len.write(length + 1);
            fields.substate.write(BROTLI_STATE_METABLOCK_HEADER_NONE);
            return BROTLI_DECODER_SUCCESS;
        }
        return BROTLI_DECODER_ERROR_UNREACHABLE;
    }
}
