// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Mechanical translation of c/dec/decode.c:521-930. Header state is borrowed
//! from its actual partial-initialization owner; only original reached fields
//! become typed initialized references. Output tables retain untouched storage.

use super::bit_reader::{BitReader, Reg, bit_mask};
use super::huffman::{
    HuffmanCode, brotli_build_code_lengths_huffman_table, brotli_build_huffman_table_with_symbols,
    brotli_build_simple_huffman_table,
};
use super::layout::{HeaderArenaStorage, HuffmanCodeStorage};
use super::running::*;
use super::status::*;
use std::mem::{MaybeUninit, align_of, size_of};

const CODE_LENGTH_ORDER: [usize; 18] =
    [1, 2, 3, 4, 0, 5, 17, 6, 16, 7, 8, 9, 10, 11, 12, 13, 14, 15];
const PREFIX_LENGTH: [Reg; 16] = [2, 2, 2, 3, 2, 2, 2, 4, 2, 2, 2, 3, 2, 2, 2, 4];
const PREFIX_VALUE: [Reg; 16] = [0, 4, 3, 2, 0, 4, 3, 1, 0, 4, 3, 2, 0, 4, 3, 5];
const SYMBOL_ORIGIN: usize = 16;

/// Disjoint borrows of exactly the source fields used by the header readers.
/// No payload is copied or allocated. The separate context-map output is a
/// sibling in the same physical arena, as at the original decode.c1101 caller.
pub struct HuffmanHeaderFields<'a> {
    substate_huffman: &'a mut MaybeUninit<u32>,
    sub_loop_counter: &'a mut MaybeUninit<Reg>,
    repeat_code_len: &'a mut MaybeUninit<Reg>,
    prev_code_len: &'a mut MaybeUninit<Reg>,
    symbol: &'a mut MaybeUninit<Reg>,
    repeat: &'a mut MaybeUninit<Reg>,
    space: &'a mut MaybeUninit<Reg>,
    table: &'a mut MaybeUninit<[HuffmanCodeStorage; 32]>,
    symbol_lists: &'a mut MaybeUninit<usize>,
    symbols_lists_array: &'a mut MaybeUninit<[u16; 720]>,
    next_symbol: &'a mut MaybeUninit<[i32; 32]>,
    code_length_code_lengths: &'a mut MaybeUninit<[u8; 18]>,
    code_length_histo: &'a mut MaybeUninit<[u16; 16]>,
}

pub fn split_header(
    header: &mut HeaderArenaStorage,
) -> (
    HuffmanHeaderFields<'_>,
    &mut MaybeUninit<[HuffmanCodeStorage; 646]>,
) {
    (
        HuffmanHeaderFields {
            substate_huffman: &mut header.substate_huffman,
            sub_loop_counter: &mut header.sub_loop_counter,
            repeat_code_len: &mut header.repeat_code_len,
            prev_code_len: &mut header.prev_code_len,
            symbol: &mut header.symbol,
            repeat: &mut header.repeat,
            space: &mut header.space,
            table: &mut header.table,
            symbol_lists: &mut header.symbol_lists,
            symbols_lists_array: &mut header.symbols_lists_array,
            next_symbol: &mut header.next_symbol,
            code_length_code_lengths: &mut header.code_length_code_lengths,
            code_length_histo: &mut header.code_length_histo,
        },
        &mut header.context_map_table,
    )
}

/// Borrow the actual embedded context-map table, without initializing its
/// fields. The splitter keeps it disjoint from every header-reader field.
pub fn context_map_output(
    table: &mut MaybeUninit<[HuffmanCodeStorage; 646]>,
) -> &mut [MaybeUninit<HuffmanCode>] {
    code_output(table)
}

fn elements<T, const N: usize>(array: &mut MaybeUninit<[T; N]>) -> &mut [MaybeUninit<T>] {
    // SAFETY: an array has N contiguous T slots; MaybeUninit<T> has T's exact
    // size/alignment and permits each slot to remain uninitialized. This view
    // reads no T and exclusively reborrows the actual array allocation.
    unsafe { std::slice::from_raw_parts_mut(array.as_mut_ptr().cast(), N) }
}

unsafe fn prefix<T, const N: usize>(array: &MaybeUninit<[T; N]>, n: usize) -> &[T] {
    assert!(n <= N);
    // SAFETY: the caller proves these exact first n elements initialized;
    // array storage is contiguous and remains borrowed for the returned view.
    unsafe { std::slice::from_raw_parts(array.as_ptr().cast(), n) }
}

unsafe fn prefix_mut<T, const N: usize>(array: &mut MaybeUninit<[T; N]>, n: usize) -> &mut [T] {
    assert!(n <= N);
    // SAFETY: caller proves initialization only for this reached prefix. The
    // exclusive actual-owner borrow prevents concurrent writes/aliases.
    unsafe { std::slice::from_raw_parts_mut(array.as_mut_ptr().cast(), n) }
}

fn code_output<const N: usize>(
    array: &mut MaybeUninit<[HuffmanCodeStorage; N]>,
) -> &mut [MaybeUninit<HuffmanCode>] {
    assert_eq!(size_of::<HuffmanCodeStorage>(), size_of::<HuffmanCode>());
    assert_eq!(align_of::<HuffmanCodeStorage>(), align_of::<HuffmanCode>());
    // SAFETY: both native repr(C) layouts have identical bits/value fields;
    // this MaybeUninit output view requires no prior field initialization.
    // Builders write only complete codes in the actual embedded table. No
    // padding is read, and the exclusive owner borrow lasts for the view.
    unsafe { std::slice::from_raw_parts_mut(array.as_mut_ptr().cast(), N) }
}

unsafe fn table_bits(array: &MaybeUninit<[HuffmanCodeStorage; 32]>, index: usize) -> Reg {
    assert!(index < 32);
    // SAFETY: source LENGTH_SYMBOLS entry follows the completed code-length
    // builder, which initialized this reached bits field. Other fields and
    // padding are not read by this projection.
    unsafe {
        Reg::from(
            (*array.as_ptr().cast::<HuffmanCodeStorage>().add(index))
                .bits
                .assume_init_read(),
        )
    }
}

unsafe fn table_value(array: &MaybeUninit<[HuffmanCodeStorage; 32]>, index: usize) -> Reg {
    assert!(index < 32);
    // SAFETY: the same completed source builder initialized this exact value
    // field. The caller's reached entry satisfies the native table domain.
    unsafe {
        Reg::from(
            (*array.as_ptr().cast::<HuffmanCodeStorage>().add(index))
                .value
                .assume_init_read(),
        )
    }
}

unsafe fn check_symbol_origin(header: &HuffmanHeaderFields<'_>) {
    // SAFETY: the caller retained the source metablock-header initialization
    // checkpoint, which stored this pointer token inside the stable owner.
    let token = unsafe { header.symbol_lists.assume_init_read() };
    let actual = header
        .symbols_lists_array
        .as_ptr()
        .cast::<u16>()
        .wrapping_add(SYMBOL_ORIGIN);
    assert_eq!(token, actual as usize);
    // The token is never dereferenced or used to reconstruct an owner.
}

unsafe fn check_symbol_origin_for_lists(token: &MaybeUninit<usize>, lists: &[MaybeUninit<u16>]) {
    // SAFETY: the source reaches the stored pointer argument only when calling
    // a length-processing helper; the caller retained that initialization.
    let token = unsafe { token.assume_init_read() };
    assert_eq!(token, lists.as_ptr().wrapping_add(SYMBOL_ORIGIN) as usize);
}

#[inline(always)]
pub fn log2_floor(mut x: Reg) -> Reg {
    let mut result = 0;
    while x != 0 {
        x >>= 1;
        result += 1;
    }
    result
}

/// # Safety
/// `header` is the source's stable actual arena. SIMPLE_READ entry has
/// initialized symbol/sub_loop_counter and previously completed simple words;
/// alphabet widths and every reached output index retain source preconditions.
pub unsafe fn read_simple_huffman_symbols(
    alphabet_size_max: Reg,
    alphabet_size_limit: Reg,
    br: &mut BitReader<'_>,
    header: &mut HuffmanHeaderFields<'_>,
) -> BrotliDecoderErrorCode {
    let max_bits = log2_floor(alphabet_size_max.wrapping_sub(1));
    // SAFETY: required SIMPLE_READ source fields were initialized by the size
    // branch or its previous partial-read checkpoint.
    let (mut i, num_symbols) = unsafe {
        (
            header.sub_loop_counter.assume_init_read(),
            header.symbol.assume_init_read(),
        )
    };
    let words = elements(header.symbols_lists_array);
    while i <= num_symbols {
        let mut v = 0;
        if !br.safe_read_bits(max_bits, &mut v) {
            header.sub_loop_counter.write(i);
            header
                .substate_huffman
                .write(BROTLI_STATE_HUFFMAN_SIMPLE_READ);
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        if v >= alphabet_size_limit {
            return BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET;
        }
        words[i as usize].write(v as u16);
        i += 1;
    }
    i = 0;
    while i < num_symbols {
        let mut k = i + 1;
        while k <= num_symbols {
            // SAFETY: successful loop above completed every word through
            // num_symbols, including the prefix from earlier resume calls.
            if unsafe {
                words[i as usize].assume_init_read() == words[k as usize].assume_init_read()
            } {
                return BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_SAME;
            }
            k += 1;
        }
        i += 1;
    }
    BROTLI_DECODER_SUCCESS
}

/// The borrowed list has its source symbol-zero at physical index 16. The
/// reached histogram and next-symbol entries are initialized source prefixes.
#[inline(always)]
#[allow(clippy::too_many_arguments)] // Preserve the original pointer arguments.
pub fn process_single_code_length(
    code_len: Reg,
    symbol: &mut Reg,
    repeat: &mut Reg,
    space: &mut Reg,
    prev_code_len: &mut Reg,
    symbol_lists: &mut [MaybeUninit<u16>],
    code_length_histo: &mut [u16],
    next_symbol: &mut [i32],
) {
    *repeat = 0;
    if code_len != 0 {
        let len = code_len as usize;
        let index = SYMBOL_ORIGIN
            .checked_add_signed(next_symbol[len] as isize)
            .unwrap();
        symbol_lists[index].write(*symbol as u16);
        next_symbol[len] = *symbol as i32;
        *prev_code_len = code_len;
        *space = space.wrapping_sub(32768 >> code_len);
        code_length_histo[len] = code_length_histo[len].wrapping_add(1);
    }
    *symbol = symbol.wrapping_add(1);
}

/// `code_len` is 16 or 17; source signed indices and shifts retain their
/// original domains. The physical list origin and initialized prefixes are
/// identical to process_single_code_length.
#[inline(always)]
#[allow(clippy::too_many_arguments)] // Preserve the original pointer arguments.
pub fn process_repeated_code_length(
    code_len: Reg,
    mut repeat_delta: Reg,
    alphabet_size: Reg,
    symbol: &mut Reg,
    repeat: &mut Reg,
    space: &mut Reg,
    prev_code_len: &mut Reg,
    repeat_code_len: &mut Reg,
    symbol_lists: &mut [MaybeUninit<u16>],
    code_length_histo: &mut [u16],
    next_symbol: &mut [i32],
) {
    let mut extra_bits = 3;
    let mut new_len = 0;
    if code_len == 16 {
        new_len = *prev_code_len;
        extra_bits = 2;
    }
    if *repeat_code_len != new_len {
        *repeat = 0;
        *repeat_code_len = new_len;
    }
    let old_repeat = *repeat;
    if *repeat > 0 {
        *repeat = repeat.wrapping_sub(2);
        *repeat <<= extra_bits;
    }
    *repeat = repeat.wrapping_add(repeat_delta.wrapping_add(3));
    repeat_delta = repeat.wrapping_sub(old_repeat);
    if symbol.wrapping_add(repeat_delta) > alphabet_size {
        *symbol = alphabet_size;
        *space = 0xfffff;
        return;
    }
    if *repeat_code_len != 0 {
        let last = symbol.wrapping_add(repeat_delta);
        let len = *repeat_code_len as usize;
        let mut next = next_symbol[len];
        loop {
            let index = SYMBOL_ORIGIN.checked_add_signed(next as isize).unwrap();
            symbol_lists[index].write(*symbol as u16);
            next = *symbol as i32;
            *symbol = symbol.wrapping_add(1);
            if *symbol == last {
                break;
            }
        }
        next_symbol[len] = next;
        *space = space.wrapping_sub(repeat_delta << (15 - *repeat_code_len));
        code_length_histo[len] =
            (Reg::from(code_length_histo[len]).wrapping_add(repeat_delta)) as u16;
    } else {
        *symbol = symbol.wrapping_add(repeat_delta);
    }
}

/// # Safety
/// Source LENGTH_SYMBOLS initialization has completed: all scalar counters,
/// histogram[0..16], next_symbol[0..16], and the code-length table fields are
/// initialized. `symbol_lists` denotes this stable owner's embedded array+16.
/// Only already-written heads/links need be initialized in that array.
pub unsafe fn read_symbol_code_lengths(
    alphabet_size: Reg,
    br: &mut BitReader<'_>,
    header: &mut HuffmanHeaderFields<'_>,
) -> BrotliDecoderErrorCode {
    // SAFETY: exact source entry/checkpoint initialized these cached locals.
    let (mut symbol, mut repeat, mut space, mut prev_code_len, mut repeat_code_len) = unsafe {
        (
            header.symbol.assume_init_read(),
            header.repeat.assume_init_read(),
            header.space.assume_init_read(),
            header.prev_code_len.assume_init_read(),
            header.repeat_code_len.assume_init_read(),
        )
    };
    // SAFETY: caller retains the actual source symbol_lists pointer checkpoint.
    unsafe { check_symbol_origin(header) };
    let lists = elements(header.symbols_lists_array);
    // SAFETY: source LENGTH_SYMBOLS initialized exactly these prefixes.
    let (histo, next) = unsafe {
        (
            prefix_mut(header.code_length_histo, 16),
            prefix_mut(header.next_symbol, 16),
        )
    };
    if !br.warmup() {
        return BROTLI_DECODER_NEEDS_MORE_INPUT;
    }
    while symbol < alphabet_size && space > 0 {
        if !br.check_input_amount() {
            header.symbol.write(symbol);
            header.repeat.write(repeat);
            header.prev_code_len.write(prev_code_len);
            header.repeat_code_len.write(repeat_code_len);
            header.space.write(space);
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        br.fill_bit_window16();
        let index = (br.bits_unmasked() & bit_mask(5)) as usize;
        // SAFETY: the source code-length builder initialized each reached
        // bits/value field before entering LENGTH_SYMBOLS.
        br.drop_bits(unsafe { table_bits(header.table, index) });
        // SAFETY: same source-initialized reached table value, read after drop.
        let code_len = unsafe { table_value(header.table, index) };
        if code_len < 16 {
            process_single_code_length(
                code_len,
                &mut symbol,
                &mut repeat,
                &mut space,
                &mut prev_code_len,
                lists,
                histo,
                next,
            );
        } else {
            let extra_bits = if code_len == 16 { 2 } else { 3 };
            let repeat_delta = br.bits_unmasked() & bit_mask(extra_bits);
            br.drop_bits(extra_bits);
            process_repeated_code_length(
                code_len,
                repeat_delta,
                alphabet_size,
                &mut symbol,
                &mut repeat,
                &mut space,
                &mut prev_code_len,
                &mut repeat_code_len,
                lists,
                histo,
                next,
            );
        }
    }
    header.space.write(space);
    BROTLI_DECODER_SUCCESS
}

/// # Safety
/// Same initialized-field, pointer-origin, nonalias and stable-owner contract
/// as read_symbol_code_lengths; counters here are updated in the actual arena.
pub unsafe fn safe_read_symbol_code_lengths(
    alphabet_size: Reg,
    br: &mut BitReader<'_>,
    header: &mut HuffmanHeaderFields<'_>,
) -> BrotliDecoderErrorCode {
    // SAFETY: these exact scalar/prefix fields are initialized at LENGTH_SYMBOLS;
    // disjoint field borrows retain the actual arena for immediate writes.
    let (symbol, repeat, space, prev, repeat_len, histo, next) = unsafe {
        (
            header.symbol.assume_init_mut(),
            header.repeat.assume_init_mut(),
            header.space.assume_init_mut(),
            header.prev_code_len.assume_init_mut(),
            header.repeat_code_len.assume_init_mut(),
            prefix_mut(header.code_length_histo, 16),
            prefix_mut(header.next_symbol, 16),
        )
    };
    let lists = elements(header.symbols_lists_array);
    let mut get_byte = false;
    while *symbol < alphabet_size && *space > 0 {
        let mut bits = 0;
        if get_byte && !br.pull_byte() {
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        get_byte = false;
        let available_bits = br.available_bits();
        if available_bits != 0 {
            bits = Reg::from(br.bits_unmasked() as u32);
        }
        let index = (bits & bit_mask(5)) as usize;
        // SAFETY: completed source code-length builder initialized this field.
        if unsafe { table_bits(header.table, index) } > available_bits {
            get_byte = true;
            continue;
        }
        // SAFETY: same reached entry; bits check preceded this value load.
        let code_len = unsafe { table_value(header.table, index) };
        if code_len < 16 {
            // SAFETY: exact same source-initialized table bits field.
            br.drop_bits(unsafe { table_bits(header.table, index) });
            // SAFETY: this is the source-reached pointer-argument checkpoint;
            // zero-iteration and insufficient-bit paths never read the token.
            unsafe { check_symbol_origin_for_lists(header.symbol_lists, lists) };
            process_single_code_length(code_len, symbol, repeat, space, prev, lists, histo, next);
        } else {
            let extra_bits = code_len - 14;
            // SAFETY: exact same source-initialized table bits field.
            let repeat_delta =
                (bits >> unsafe { table_bits(header.table, index) }) & bit_mask(extra_bits);
            // SAFETY: exact source re-load of the initialized bits field.
            if available_bits < unsafe { table_bits(header.table, index) } + extra_bits {
                get_byte = true;
                continue;
            }
            // SAFETY: exact source re-load after the successful availability test.
            br.drop_bits(unsafe { table_bits(header.table, index) } + extra_bits);
            // SAFETY: pointer argument is reached only after both bit checks.
            unsafe { check_symbol_origin_for_lists(header.symbol_lists, lists) };
            process_repeated_code_length(
                code_len,
                repeat_delta,
                alphabet_size,
                symbol,
                repeat,
                space,
                prev,
                repeat_len,
                lists,
                histo,
                next,
            );
        }
    }
    BROTLI_DECODER_SUCCESS
}

/// # Safety
/// COMPLEX entry initialized sub_loop_counter/repeat/space, all 18 length
/// bytes and histogram[0..6]. Resumes preserve every previously written entry.
pub unsafe fn read_code_length_code_lengths(
    br: &mut BitReader<'_>,
    header: &mut HuffmanHeaderFields<'_>,
) -> BrotliDecoderErrorCode {
    // SAFETY: required COMPLEX fields originate in NONE or its partial checkpoint.
    let (mut num_codes, mut space, mut i) = unsafe {
        (
            header.repeat.assume_init_read(),
            header.space.assume_init_read(),
            header.sub_loop_counter.assume_init_read(),
        )
    };
    while i < 18 {
        let code_len_idx = CODE_LENGTH_ORDER[i as usize];
        let mut ix = 0;
        if !br.safe_get_bits(4, &mut ix) {
            let available_bits = br.available_bits();
            ix = if available_bits != 0 {
                br.bits_unmasked() & 0xf
            } else {
                0
            };
            if PREFIX_LENGTH[ix as usize] > available_bits {
                header.sub_loop_counter.write(i);
                header.repeat.write(num_codes);
                header.space.write(space);
                header.substate_huffman.write(BROTLI_STATE_HUFFMAN_COMPLEX);
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
        }
        let v = PREFIX_VALUE[ix as usize];
        br.drop_bits(PREFIX_LENGTH[ix as usize]);
        elements(header.code_length_code_lengths)[code_len_idx].write(v as u8);
        if v != 0 {
            space = space.wrapping_sub(32 >> v);
            num_codes += 1;
            // SAFETY: COMPLEX source initialization zeroed exactly histogram
            // entries0..6; v is in1..5 from the actual fixed prefix table.
            let histo = unsafe { prefix_mut(header.code_length_histo, 6) };
            histo[v as usize] = histo[v as usize].wrapping_add(1);
            if space.wrapping_sub(1) >= 32 {
                break;
            }
        }
        i += 1;
    }
    if !(num_codes == 1 || space == 0) {
        return BROTLI_DECODER_ERROR_FORMAT_CL_SPACE;
    }
    BROTLI_DECODER_SUCCESS
}

/// # Safety
/// `header` belongs to the actual, stable callback-backed arena and must not
/// move across resumptions while its symbol_lists token points inside it.
/// Its substate and each field reached in that substate must have the source's
/// initialization history: NONE needs only substate; SIMPLE_SIZE retains the
/// consumed selector; SIMPLE_READ has size/counter/previous word prefix;
/// SIMPLE_BUILD has all selected words; COMPLEX has initialized code-length
/// bytes/histogram prefix6 and counters; LENGTH_SYMBOLS has the completed
/// code-length table, full histogram, next prefix16 and source scalar counters.
/// Reached symbol-list heads/links alone must be initialized. The stored list
/// pointer token must equal this owner's embedded array+16 whenever reached.
/// Reader input, output table and optional size are disjoint from these
/// borrowed source fields and each other, and retain source backing/index/
/// shift/alphabet domains. The output may be the splitter's context-map sibling.
pub unsafe fn read_huffman_code(
    alphabet_size_max: Reg,
    alphabet_size_limit: Reg,
    table: &mut [MaybeUninit<HuffmanCode>],
    mut opt_table_size: Option<&mut MaybeUninit<Reg>>,
    br: &mut BitReader<'_>,
    header: &mut HuffmanHeaderFields<'_>,
) -> BrotliDecoderErrorCode {
    loop {
        // SAFETY: the entry contract requires initialized source substate;
        // every continuation below writes it at the original checkpoint.
        let mut state = unsafe { header.substate_huffman.assume_init_read() };
        if state == BROTLI_STATE_HUFFMAN_NONE {
            let mut value = 0;
            if !br.safe_read_bits(2, &mut value) {
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            header.sub_loop_counter.write(value);
            if value != 1 {
                header.space.write(32);
                header.repeat.write(0);
                for slot in &mut elements(header.code_length_histo)[..6] {
                    slot.write(0);
                }
                for slot in elements(header.code_length_code_lengths) {
                    slot.write(0);
                }
                header.substate_huffman.write(BROTLI_STATE_HUFFMAN_COMPLEX);
                continue;
            }
            state = BROTLI_STATE_HUFFMAN_SIMPLE_SIZE;
        }
        if state == BROTLI_STATE_HUFFMAN_SIMPLE_SIZE {
            let mut value = 0;
            if !br.safe_read_bits(2, &mut value) {
                header
                    .substate_huffman
                    .write(BROTLI_STATE_HUFFMAN_SIMPLE_SIZE);
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            header.symbol.write(value);
            header.sub_loop_counter.write(0);
            state = BROTLI_STATE_HUFFMAN_SIMPLE_READ;
        }
        if state == BROTLI_STATE_HUFFMAN_SIMPLE_READ {
            // SAFETY: preceding size branch or valid source resume initialized
            // exactly the simple reader's fields and prior output prefix.
            let result = unsafe {
                read_simple_huffman_symbols(alphabet_size_max, alphabet_size_limit, br, header)
            };
            if result != BROTLI_DECODER_SUCCESS {
                return result;
            }
            state = BROTLI_STATE_HUFFMAN_SIMPLE_BUILD;
        }
        if state == BROTLI_STATE_HUFFMAN_SIMPLE_BUILD {
            // SAFETY: source size branch and successful symbol reader initialized
            // this scalar and the selected1..4 word prefix.
            let mut symbol = unsafe { header.symbol.assume_init_read() };
            if symbol == 3 {
                let mut bits = 0;
                if !br.safe_read_bits(1, &mut bits) {
                    header
                        .substate_huffman
                        .write(BROTLI_STATE_HUFFMAN_SIMPLE_BUILD);
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                symbol += bits;
                header.symbol.write(symbol);
            }
            let n = [1, 2, 3, 4, 4][symbol as usize];
            // SAFETY: the completed simple symbol reader initialized precisely
            // this selected prefix; the builder sorts the actual arena words.
            let values = unsafe { prefix_mut(header.symbols_lists_array, n) };
            let size = brotli_build_simple_huffman_table(table, 8, values, symbol as u32);
            if let Some(out) = opt_table_size.as_mut() {
                out.write(Reg::from(size));
            }
            header.substate_huffman.write(BROTLI_STATE_HUFFMAN_NONE);
            return BROTLI_DECODER_SUCCESS;
        }
        if state == BROTLI_STATE_HUFFMAN_COMPLEX {
            // SAFETY: source NONE or the valid COMPLEX resume checkpoint
            // initialized exactly the fields required by this helper.
            let result = unsafe { read_code_length_code_lengths(br, header) };
            if result != BROTLI_DECODER_SUCCESS {
                return result;
            }
            // SAFETY: NONE zeroed all18 length bytes and the first6 histogram
            // entries; COMPLEX changed only those initialized entries.
            let (lengths, histo) = unsafe {
                (
                    prefix(header.code_length_code_lengths, 18),
                    prefix_mut(header.code_length_histo, 6),
                )
            };
            brotli_build_code_lengths_huffman_table(code_output(header.table), lengths, histo);
            for slot in elements(header.code_length_histo) {
                slot.write(0);
            }
            // SAFETY: source header-entry checkpoint stored this stable owner's
            // array+16 pointer. Validation never dereferences its integer token.
            unsafe { check_symbol_origin(header) };
            for i in 0..16 {
                let next = i as i32 - 16;
                elements(header.next_symbol)[i].write(next);
                elements(header.symbols_lists_array)
                    [SYMBOL_ORIGIN.checked_add_signed(next as isize).unwrap()]
                .write(0xffff);
            }
            header.symbol.write(0);
            header.prev_code_len.write(8);
            header.repeat.write(0);
            header.repeat_code_len.write(0);
            header.space.write(32768);
            header
                .substate_huffman
                .write(BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS);
            state = BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS;
        }
        if state == BROTLI_STATE_HUFFMAN_LENGTH_SYMBOLS {
            // SAFETY: source COMPLEX completed the table/counter/array-prefix
            // initialization above, or caller retained its valid resume state.
            let mut result = unsafe { read_symbol_code_lengths(alphabet_size_limit, br, header) };
            if result == BROTLI_DECODER_NEEDS_MORE_INPUT {
                // SAFETY: fast helper's exact partial checkpoint retained all
                // initialized LENGTH_SYMBOLS fields for immediate safe resume.
                result = unsafe { safe_read_symbol_code_lengths(alphabet_size_limit, br, header) };
            }
            if result != BROTLI_DECODER_SUCCESS {
                return result;
            }
            // SAFETY: fast/safe source length reader initialized/preserved space.
            if unsafe { header.space.assume_init_read() } != 0 {
                return BROTLI_DECODER_ERROR_FORMAT_HUFFMAN_SPACE;
            }
            // SAFETY: caller/source length entry retained stable pointer origin.
            unsafe { check_symbol_origin(header) };
            let words = elements(header.symbols_lists_array);
            // SAFETY: all16 histogram entries were initialized before LENGTH_SYMBOLS.
            let histo = unsafe { prefix_mut(header.code_length_histo, 16) };
            let size = brotli_build_huffman_table_with_symbols(
                table,
                8,
                |index| {
                    let physical = SYMBOL_ORIGIN.checked_add_signed(index).unwrap();
                    // SAFETY: the source initializes all16 negative heads and each
                    // reached forward link before building. The indexed builder
                    // never requests an unwritten terminal or zero-symbol link.
                    unsafe { words[physical].assume_init_read() }
                },
                histo,
            );
            if let Some(out) = opt_table_size.as_mut() {
                out.write(Reg::from(size));
            }
            header.substate_huffman.write(BROTLI_STATE_HUFFMAN_NONE);
            return BROTLI_DECODER_SUCCESS;
        }
        return BROTLI_DECODER_ERROR_UNREACHABLE;
    }
}
