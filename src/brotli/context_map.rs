// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
//! Mechanical translation of c/dec/decode.c:1050-1175. Field loans end only
//! at the source allocation checkpoint; the actual owner supplies that callback.

use super::bit_reader::{BitReader, Reg};
use super::huffman_reader::{
    HuffmanHeaderFields, context_map_output, read_huffman_code, split_header,
};
use super::layout::{HeaderArenaStorage, HuffmanCodeStorage};
use super::metablock::decode_var_len_uint8;
use super::mtf::inverse_move_to_front_transform;
use super::running::*;
use super::status::*;
use super::symbols::{HuffmanTable, safe_read_symbol};
use std::mem::MaybeUninit;

pub struct ContextMapHeaderFields<'a> {
    pub substate: &'a mut MaybeUninit<u32>,
    pub index: &'a mut MaybeUninit<Reg>,
    pub max_run_length_prefix: &'a mut MaybeUninit<Reg>,
    pub code: &'a mut MaybeUninit<Reg>,
    pub table: &'a mut MaybeUninit<[HuffmanCodeStorage; 646]>,
    pub huffman: HuffmanHeaderFields<'a>,
}

/// Eighteen disjoint actual header fields, including the existing sealed
/// thirteen-field Huffman loan. No whole-header reference survives this split.
pub fn split_context_map(header: &mut HeaderArenaStorage) -> ContextMapHeaderFields<'_> {
    let actual = std::ptr::from_mut(header);
    let (huffman, table) = split_header(header);
    // SAFETY: frozen split_header retains exactly substate_huffman,
    // sub_loop_counter, repeat_code_len, prev_code_len, symbol, repeat, space,
    // table, symbol_lists, symbols_lists_array, next_symbol,
    // code_length_code_lengths, code_length_histo, and context_map_table.
    // The four addr_of_mut projections below are disjoint from those fourteen
    // fields and each other. The original exclusive owner pointer is retained
    // only for these projections; no whole-header reference is recreated.
    unsafe {
        ContextMapHeaderFields {
            substate: &mut *std::ptr::addr_of_mut!((*actual).substate_context_map),
            index: &mut *std::ptr::addr_of_mut!((*actual).context_index),
            max_run_length_prefix: &mut *std::ptr::addr_of_mut!((*actual).max_run_length_prefix),
            code: &mut *std::ptr::addr_of_mut!((*actual).code),
            table,
            huffman,
        }
    }
}

pub struct ContextMapFields<'a> {
    pub header: ContextMapHeaderFields<'a>,
    pub num_htrees: &'a mut MaybeUninit<Reg>,
    pub map_slot: &'a mut MaybeUninit<usize>,
    pub uint8_substate: &'a mut MaybeUninit<u32>,
    pub mtf_upper_bound: &'a mut MaybeUninit<Reg>,
    pub mtf: &'a mut MaybeUninit<[u32; 65]>,
}

/// A monomorphized capability over actual owner fields and allocation storage.
/// Creating a region reads no payload or pointer slot. Optional map backing is
/// borrowed from its real owner, without resolving the native map token.
///
/// Every region flushes its actual BitReader fields and ends ALL input, reader,
/// header, outer-field and map loans before returning. allocate_map is called
/// only between regions: invoke the real allocation callback while the old
/// native map slot remains, then store its actual result address (including
/// NULL). A NONE allocation requires no retained previous map owner; this is
/// the source caller's reached release/lifecycle checkpoint, not reallocation.
pub trait ContextMapOwner {
    fn with_fields<R>(
        &mut self,
        operation: impl FnOnce(
            ContextMapFields<'_>,
            &mut BitReader<'_>,
            Option<&mut [MaybeUninit<u8>]>,
        ) -> R,
    ) -> R;
    fn allocate_map(&mut self, size: usize) -> bool;
}

struct Table<'a>(&'a MaybeUninit<[HuffmanCodeStorage; 646]>);
impl HuffmanTable for Table<'_> {
    fn bits(&self, index: usize) -> u8 {
        assert!(index < 646);
        // SAFETY: only the original reached table field is read. Successful
        // header construction/resume initialized that exact bits field.
        unsafe {
            (*self.0.as_ptr().cast::<HuffmanCodeStorage>().add(index))
                .bits
                .assume_init_read()
        }
    }
    fn value(&self, index: usize) -> u16 {
        assert!(index < 646);
        // SAFETY: source table traversal initialized this exact value field;
        // no adjacent bits/padding or unused table entry is exposed.
        unsafe {
            (*self.0.as_ptr().cast::<HuffmanCodeStorage>().add(index))
                .value
                .assume_init_read()
        }
    }
}

fn backing(
    map: Option<&mut [MaybeUninit<u8>]>,
    token: usize,
    size: usize,
) -> &mut [MaybeUninit<u8>] {
    let map = map.expect("reached source map access requires its actual owner");
    assert_eq!(token, map.as_ptr() as usize);
    assert!(size <= map.len());
    &mut map[..size]
}
fn write_map(map: &mut Option<&mut [MaybeUninit<u8>]>, token: usize, index: Reg, value: u8) {
    let map = map
        .as_deref_mut()
        .expect("reached source map write requires its actual owner");
    assert_eq!(token, map.as_ptr() as usize);
    map[usize::try_from(index).unwrap()].write(value);
}

enum Step {
    Return(BrotliDecoderErrorCode),
    Allocate,
}

/// # Safety
/// Every reached scalar/header/reader/map operand is initialized and every
/// reached backing/shift/table domain is source-defined, with stable
/// symbol_lists owner identity. Direct synthetic raw states are permitted when
/// those concrete read/write domains hold; complete-codec reachability is a
/// separate invariant census, not an extra filter on defined source behavior.
/// Output, input, all eighteen header fields, outer fields and MTF are disjoint.
/// Only reached table fields and map bytes need be initialized. TRANSFORM/MTF
/// requires the complete map prefix, regardless of stale header.context_index;
/// the retained MTF bound/list domain matches the frozen transform contract.
/// Every source unsigned shift/bit-reader width and actual map backing extent
/// is defined. The owner obeys the scoped loan/allocation contract above.
pub unsafe fn decode_context_map<O: ContextMapOwner>(
    context_map_size: Reg,
    owner: &mut O,
) -> BrotliDecoderErrorCode {
    let step = owner.with_fields(|mut fields, br, map| {
        // SAFETY: source initialization/resume supplies this discriminant.
        let state = unsafe { fields.header.substate.assume_init_read() };
        if state == BROTLI_STATE_CONTEXT_MAP_NONE {
            // SAFETY: disjoint actual uint8 state/result/reader fields; only
            // LONG needs prior initialized output, exactly as at source entry.
            let result =
                unsafe { decode_var_len_uint8(fields.uint8_substate, br, fields.num_htrees) };
            if result != BROTLI_DECODER_SUCCESS {
                return Step::Return(result);
            }
            // SAFETY: successful variable-byte decode initialized the count.
            let value = unsafe { fields.num_htrees.assume_init_read() };
            fields.num_htrees.write(value.wrapping_add(1));
            fields.header.index.write(0);
            // All actual field/input/map loans end before the callback.
            Step::Allocate
        } else {
            // SAFETY: unchanged source state enters its original fallthrough
            // region, with only reached initialization requirements.
            Step::Return(unsafe { phases(context_map_size, state, &mut fields, br, map) })
        }
    });
    match step {
        Step::Return(result) => result,
        Step::Allocate => {
            if !owner.allocate_map(context_map_size as usize) {
                return BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP;
            }
            owner.with_fields(|mut fields, br, map| {
                // SAFETY: count written before the actual allocation callback;
                // caller-compatible callbacks retain the reached owner fields.
                if unsafe { fields.num_htrees.assume_init_read() } <= 1 {
                    // SAFETY: successful allocate_map initialized this slot.
                    let token = unsafe { fields.map_slot.assume_init_read() };
                    let map = backing(map, token, context_map_size as usize);
                    // SAFETY: actual reached prefix is writable, no prior map
                    // byte is read; this is exactly the source memset extent.
                    unsafe { map.as_mut_ptr().cast::<u8>().write_bytes(0, map.len()) };
                    return BROTLI_DECODER_SUCCESS;
                }
                fields
                    .header
                    .substate
                    .write(BROTLI_STATE_CONTEXT_MAP_READ_PREFIX);
                // SAFETY: original successful allocation fallthrough.
                unsafe {
                    phases(
                        context_map_size,
                        BROTLI_STATE_CONTEXT_MAP_READ_PREFIX,
                        &mut fields,
                        br,
                        map,
                    )
                }
            })
        }
    }
}

unsafe fn phases(
    context_map_size: Reg,
    mut state: u32,
    fields: &mut ContextMapFields<'_>,
    br: &mut BitReader<'_>,
    mut map: Option<&mut [MaybeUninit<u8>]>,
) -> BrotliDecoderErrorCode {
    if state == BROTLI_STATE_CONTEXT_MAP_READ_PREFIX {
        let mut bits = 0;
        if !br.safe_get_bits(5, &mut bits) {
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        if bits & 1 != 0 {
            fields.header.max_run_length_prefix.write((bits >> 1) + 1);
            br.drop_bits(5);
        } else {
            fields.header.max_run_length_prefix.write(0);
            br.drop_bits(1);
        }
        fields
            .header
            .substate
            .write(BROTLI_STATE_CONTEXT_MAP_HUFFMAN);
        state = BROTLI_STATE_CONTEXT_MAP_HUFFMAN;
    }
    if state == BROTLI_STATE_CONTEXT_MAP_HUFFMAN {
        // SAFETY: NONE/READ_PREFIX or the original resume initialized these
        // exact operands; preserve unsigned register arithmetic.
        let alphabet_size = unsafe { fields.num_htrees.assume_init_read() }
            .wrapping_add(unsafe { fields.header.max_run_length_prefix.assume_init_read() });
        // SAFETY: actual separate embedded context-map output and sealed
        // header fields are disjoint; initialized table prefix follows source.
        let result = unsafe {
            read_huffman_code(
                alphabet_size,
                alphabet_size,
                context_map_output(fields.header.table),
                None,
                br,
                &mut fields.header.huffman,
            )
        };
        if result != BROTLI_DECODER_SUCCESS {
            return result;
        }
        fields.header.code.write(0xffff);
        fields
            .header
            .substate
            .write(BROTLI_STATE_CONTEXT_MAP_DECODE);
        state = BROTLI_STATE_CONTEXT_MAP_DECODE;
    }
    if state == BROTLI_STATE_CONTEXT_MAP_DECODE {
        // SAFETY: original successful Huffman or partial DECODE checkpoint
        // initialized each exact scalar/token, in this source read order.
        let mut context_index = unsafe { fields.header.index.assume_init_read() };
        // SAFETY: same reached DECODE checkpoint.
        let max_run_length_prefix =
            unsafe { fields.header.max_run_length_prefix.assume_init_read() };
        // SAFETY: actual allocation or defined direct resume initialized slot;
        // token is not resolved until a map access is actually reached.
        let context_map = unsafe { fields.map_slot.assume_init_read() };
        // SAFETY: Huffman success or partial DECODE initialized this scalar.
        let mut code = unsafe { fields.header.code.assume_init_read() };
        let mut skip_preamble = code != 0xffff;
        while context_index < context_map_size || skip_preamble {
            if !skip_preamble {
                if !safe_read_symbol(&Table(fields.header.table), br, &mut code) {
                    fields.header.code.write(0xffff);
                    fields.header.index.write(context_index);
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                if code == 0 {
                    write_map(&mut map, context_map, context_index, 0);
                    context_index = context_index.wrapping_add(1);
                    continue;
                }
                if code > max_run_length_prefix {
                    write_map(
                        &mut map,
                        context_map,
                        context_index,
                        code.wrapping_sub(max_run_length_prefix) as u8,
                    );
                    context_index = context_index.wrapping_add(1);
                    continue;
                }
            } else {
                skip_preamble = false;
            }
            let mut reps = 0;
            if !br.safe_read_bits(code, &mut reps) {
                fields.header.code.write(code);
                fields.header.index.write(context_index);
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            reps = reps.wrapping_add((1 as Reg) << code);
            if context_index.wrapping_add(reps) > context_map_size {
                return BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT;
            }
            loop {
                write_map(&mut map, context_map, context_index, 0);
                context_index = context_index.wrapping_add(1);
                reps -= 1;
                if reps == 0 {
                    break;
                }
            }
        }
        state = BROTLI_STATE_CONTEXT_MAP_TRANSFORM;
    }
    if state == BROTLI_STATE_CONTEXT_MAP_TRANSFORM {
        let mut bits = 0;
        if !br.safe_read_bits(1, &mut bits) {
            fields
                .header
                .substate
                .write(BROTLI_STATE_CONTEXT_MAP_TRANSFORM);
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        if bits != 0 {
            // SAFETY: only reached MTF reads the native token; completed decode
            // or a valid TRANSFORM resume proves all map-prefix bytes initialized.
            let token = unsafe { fields.map_slot.assume_init_read() };
            let size = context_map_size as usize;
            let empty = &mut [];
            let map = if size == 0 && map.is_none() {
                empty
            } else {
                backing(map, token, size)
            };
            // SAFETY: exactly the initialized prefix is borrowed, no unknown
            // tail or map byte is read merely by resolving actual ownership.
            let initialized =
                unsafe { std::slice::from_raw_parts_mut(map.as_mut_ptr().cast::<u8>(), size) };
            // SAFETY: caller proves retained actual MTF bound/list history;
            // map prefix is initialized and disjoint from both actual fields.
            unsafe {
                inverse_move_to_front_transform(
                    initialized,
                    context_map_size,
                    fields.mtf_upper_bound,
                    fields.mtf,
                )
            };
        }
        fields.header.substate.write(BROTLI_STATE_CONTEXT_MAP_NONE);
        return BROTLI_DECODER_SUCCESS;
    }
    BROTLI_DECODER_ERROR_UNREACHABLE
}
