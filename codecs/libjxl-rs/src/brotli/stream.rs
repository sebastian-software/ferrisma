// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Complete source decode.c:2415-2899 streaming state machine. The canonical
//! interface uses source-shaped raw cursors: actual reads/writes remain at
//! their original checkpoints, including defined aliases and callback reloads.
use super::bit_reader::Reg;
use super::bitfield::BitfieldWord;
use super::block_length::safe_read_block_length;
use super::compound_dictionary::CompoundDictionaryBindings;
use super::constants::*;
use super::distance::calculate_distance_lut;
use super::huffman::{BROTLI_HUFFMAN_MAX_SIZE_26, BROTLI_HUFFMAN_MAX_SIZE_258, HuffmanCode};
use super::instance::{DecoderInstance, MetadataStartFunc};
use super::layout::{
    BodyArenaStorage, CompoundDictionaryStorage, DecoderStateStorage, HeaderArenaStorage,
};
use super::metablock::{
    MetaBlockFields, decode_meta_block_length, decode_var_len_uint8, decode_window_bits,
};
use super::running::*;
use super::status::*;
use super::stream_input::SourceInput;
use super::stream_tables::{NativeTable, pointer_at};
use std::ffi::c_void;
use std::mem::{MaybeUninit, size_of};
macro_rules! read {
    ($p:expr,$f:ident) => {{
        #[allow(
            unused_unsafe,
            reason = "Selected source projections can nest inside raw reader operations"
        )]
        let value = unsafe { (*$p).$f.assume_init() };
        value
    }};
}
macro_rules! write {
    ($p:expr,$f:ident,$v:expr) => {{
        let value = $v;
        unsafe {
            (*$p).$f.write(value);
        }
    }};
}
macro_rules! array {
    ($p:expr,$f:ident,$i:expr,$ty:ty) => {{
        let i = $i;
        #[allow(
            unused_unsafe,
            reason = "Selected source projections can nest inside raw reader operations"
        )]
        let value = unsafe { (*$p).$f.as_ptr().cast::<$ty>().add(i).read() };
        value
    }};
}
macro_rules! array_write {
    ($p:expr,$f:ident,$i:expr,$ty:ty,$v:expr) => {{
        let value = $v;
        let i = $i;
        unsafe {
            (*$p).$f.as_mut_ptr().cast::<$ty>().add(i).write(value);
        }
    }};
}
impl<'a> DecoderInstance<'a> {
    // Selected initialized packed fields only; unknown neighboring flags stay
    // entirely inside the already independently accepted raw representation.
    unsafe fn stream_flag(&mut self, shift: u32, width: u32) -> u32 {
        let native = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        let bytes = unsafe {
            std::slice::from_raw_parts_mut(
                std::ptr::addr_of_mut!((*native).flags).cast::<MaybeUninit<u8>>(),
                4,
            )
        };
        unsafe { BitfieldWord::new(bytes.try_into().unwrap()).read(shift, width) }
    }
    unsafe fn stream_internal_input<'i>(&self, length: usize) -> SourceInput<'i> {
        let native = self.state.as_ptr().cast::<DecoderStateStorage>();
        assert!(length <= 8);
        // SAFETY: State buffer's reached prefix was copied from initialized
        // input. Only this raw fixed anchor is retained, not a state reference.
        unsafe { SourceInput::from_raw_parts(std::ptr::addr_of!((*native).buffer).cast(), length) }
    }
    /// BrotliDecoderDecompressStream on the same retained source allocation.
    /// `bindings` is the fixed borrowed-dictionary ownership sidecar used by
    /// source AttachDictionary; it contains no payload copies/allocations.
    ///
    /// # Safety
    /// Every reached pointer/cursor access and input/output extent must have
    /// the original C API's valid provenance, lifetime, alignment, initialized
    /// input and arithmetic domain. Unused nullable arguments are not read.
    /// Actual defined cursor/output aliases are permitted: no Rust references
    /// to caller cursor cells are created. Output copies have original memcpy
    /// nonoverlap requirements. Input/table/ring references exist only during
    /// individual source operations and must not overlap simultaneously mutable
    /// loans. No concurrent access or unsupported decoder reentry is allowed;
    /// metadata callbacks may replace their native callback fields after all
    /// source-storage loans have ended. Callbacks cannot unwind. Attached raw
    /// dictionaries retain their caller-proven lifetime/initialized read domain.
    pub unsafe fn decompress_stream_raw(
        &mut self,
        bindings: &CompoundDictionaryBindings<'a>,
        available_in: *mut usize,
        next_in: *mut *const u8,
        available_out: *mut usize,
        mut next_out: *mut *mut u8,
        total_out: *mut usize,
    ) -> u32 {
        let native = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        let mut result = BROTLI_DECODER_SUCCESS;
        let input_size = unsafe { available_in.read() };
        macro_rules! save {
            ($code:expr) => {{
                let code = $code;
                let consumed = input_size.wrapping_sub(unsafe { available_in.read() });
                unsafe { self.state.save_error_code(code, consumed) }
            }};
        }
        if !total_out.is_null() {
            unsafe {
                total_out.write(read!(native, partial_pos_out));
            }
        }
        if read!(native, error_code) < 0 {
            return 0;
        }
        if unsafe { available_out.read() } != 0
            && (next_out.is_null() || unsafe { next_out.read() }.is_null())
        {
            return save!(BROTLI_DECODER_ERROR_INVALID_ARGUMENTS);
        }
        if unsafe { available_out.read() } == 0 {
            next_out = std::ptr::null_mut();
        }
        let mut input;
        if read!(native, buffer_length) == 0 {
            let pointer = unsafe { next_in.read() };
            let length = unsafe { available_in.read() };
            input = unsafe { SourceInput::from_raw_parts(pointer, length) };
            input.set_input(unsafe { &mut *self.source_reader_pointer() }, 0, length);
        } else {
            result = BROTLI_DECODER_NEEDS_MORE_INPUT;
            let length = read!(native, buffer_length) as usize;
            input = unsafe { self.stream_internal_input(length) };
            input.set_input(unsafe { &mut *self.source_reader_pointer() }, 0, length);
        }
        macro_rules! reader {
            ($operation:expr) => {
                unsafe { input.with_reader(&mut *self.source_reader_pointer(), $operation) }
            };
        }
        loop {
            if result != BROTLI_DECODER_SUCCESS {
                if result == BROTLI_DECODER_NEEDS_MORE_INPUT {
                    if read!(native, ringbuffer) != 0 {
                        let intermediate = unsafe {
                            self.write_ringbuffer_raw(available_out, next_out, total_out, true)
                        };
                        if intermediate < 0 {
                            result = intermediate;
                            break;
                        }
                    }
                    if read!(native, buffer_length) != 0 {
                        let br = self.source_reader_pointer();
                        if read!(br, next_in) == read!(br, last_in) {
                            write!(native, buffer_length, 0);
                            result = BROTLI_DECODER_SUCCESS;
                            let pointer = unsafe { next_in.read() };
                            let length = unsafe { available_in.read() };
                            input = unsafe { SourceInput::from_raw_parts(pointer, length) };
                            input.set_input(unsafe { &mut *br }, 0, length);
                            continue;
                        } else if unsafe { available_in.read() } != 0 {
                            result = BROTLI_DECODER_SUCCESS;
                            let length = read!(native, buffer_length) as usize;
                            assert!(length < 8);
                            let byte = unsafe { next_in.read().read() };
                            array_write!(native, buffer, length, u8, byte);
                            write!(native, buffer_length, read!(native, buffer_length) + 1);
                            let length = read!(native, buffer_length) as usize;
                            input = unsafe { self.stream_internal_input(length) };
                            input.set_input(unsafe { &mut *br }, 0, length);
                            // Actual cursor cell reloads after every source
                            // write retain defined source alias observations.
                            unsafe {
                                next_in.write(next_in.read().add(1));
                                available_in.write(available_in.read().wrapping_sub(1));
                            }
                            continue;
                        }
                        break;
                    } else {
                        let pointer = unsafe { input.next_pointer(&*self.source_reader_pointer()) };
                        unsafe {
                            next_in.write(pointer);
                        }
                        let count = reader!(|br| br.available_in());
                        unsafe {
                            available_in.write(count);
                        }
                        while unsafe { available_in.read() } != 0 {
                            let byte = unsafe { next_in.read().read() };
                            let index = read!(native, buffer_length) as usize;
                            assert!(index < 8);
                            array_write!(native, buffer, index, u8, byte);
                            write!(native, buffer_length, read!(native, buffer_length) + 1);
                            unsafe {
                                next_in.write(next_in.read().add(1));
                                available_in.write(available_in.read().wrapping_sub(1));
                            }
                        }
                        break;
                    }
                }
                if read!(native, buffer_length) != 0 {
                    write!(native, buffer_length, 0);
                } else {
                    reader!(|br| br.unload());
                    let count = reader!(|br| br.available_in());
                    unsafe {
                        available_in.write(count);
                    }
                    let pointer = unsafe { input.next_pointer(&*self.source_reader_pointer()) };
                    unsafe {
                        next_in.write(pointer);
                    }
                }
                break;
            }
            match read!(native, state) {
                BROTLI_STATE_UNINITED => {
                    if !reader!(|br| br.warmup()) {
                        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                        continue;
                    }
                    result = reader!(|br| decode_window_bits(&mut (*native).flags, br));
                    if result != BROTLI_DECODER_SUCCESS {
                        continue;
                    }
                    if unsafe { self.stream_flag(5, 1) } != 0 {
                        write!(native, state, BROTLI_STATE_LARGE_WINDOW_BITS);
                        continue;
                    }
                    write!(native, state, BROTLI_STATE_INITIALIZE);
                }
                BROTLI_STATE_LARGE_WINDOW_BITS => {
                    let mut bits = 0;
                    if !reader!(|br| br.safe_read_bits(6, &mut bits)) {
                        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                        continue;
                    }
                    self.state.write_flag(6, 6, (bits & 63) as u32);
                    let bits = unsafe { self.stream_flag(6, 6) };
                    if !(10..=30).contains(&bits) {
                        result = BROTLI_DECODER_ERROR_FORMAT_WINDOW_BITS;
                        continue;
                    }
                    write!(native, state, BROTLI_STATE_INITIALIZE);
                }
                BROTLI_STATE_INITIALIZE => {
                    write!(
                        native,
                        max_backward_distance,
                        (1i32 << unsafe { self.stream_flag(6, 6) }) - BROTLI_WINDOW_GAP as i32
                    );
                    assert!(self.state.block_type_trees.is_none());
                    let owner = self.state.allocator.allocate(
                        size_of::<HuffmanCode>()
                            * 3
                            * (BROTLI_HUFFMAN_MAX_SIZE_258 + BROTLI_HUFFMAN_MAX_SIZE_26),
                    );
                    let token = owner.as_ref().map_or(0, |owner| owner.as_ptr() as usize);
                    self.state.block_type_trees = owner;
                    write!(native, block_type_trees, token);
                    if token == 0 {
                        result = BROTLI_DECODER_ERROR_ALLOC_BLOCK_TYPE_TREES;
                        continue;
                    }
                    let pointer = unsafe {
                        self.state
                            .block_type_trees
                            .as_ref()
                            .unwrap()
                            .as_ptr()
                            .add(3 * BROTLI_HUFFMAN_MAX_SIZE_258 * size_of::<HuffmanCode>())
                    };
                    write!(native, block_len_trees, pointer as usize);
                    write!(native, state, BROTLI_STATE_METABLOCK_BEGIN);
                }
                BROTLI_STATE_METABLOCK_BEGIN => {
                    self.state.metablock_begin();
                    write!(native, state, BROTLI_STATE_METABLOCK_HEADER);
                }
                BROTLI_STATE_METABLOCK_HEADER => {
                    result = reader!(|br| decode_meta_block_length(
                        MetaBlockFields {
                            substate: &mut (*native).substate_metablock_header,
                            flags: &mut (*native).flags,
                            remaining_len: &mut (*native).meta_block_remaining_len,
                            loop_counter: &mut (*native).loop_counter
                        },
                        br
                    ));
                    if result != BROTLI_DECODER_SUCCESS {
                        continue;
                    }
                    if (unsafe { self.stream_flag(2, 1) } != 0
                        || unsafe { self.stream_flag(1, 1) } != 0)
                        && !reader!(|br| br.jump_to_byte_boundary())
                    {
                        result = BROTLI_DECODER_ERROR_FORMAT_PADDING_1;
                        continue;
                    }
                    if unsafe { self.stream_flag(2, 1) } != 0 {
                        write!(native, state, BROTLI_STATE_METADATA);
                        let token = read!(native, metadata_start_func);
                        if token != 0 {
                            let opaque = read!(native, metadata_callback_opaque) as *mut c_void;
                            let count = read!(native, meta_block_remaining_len) as usize;
                            // SAFETY: Same actual stored callable field; all
                            // native/input/reader loans end before callback.
                            let callback: MetadataStartFunc = unsafe { std::mem::transmute(token) };
                            unsafe {
                                callback(opaque, count);
                            }
                        }
                        continue;
                    }
                    if read!(native, meta_block_remaining_len) == 0 {
                        write!(native, state, BROTLI_STATE_METABLOCK_DONE);
                        continue;
                    }
                    unsafe { self.state.calculate_ringbuffer_size() };
                    if unsafe { self.stream_flag(1, 1) } != 0 {
                        write!(native, state, BROTLI_STATE_UNCOMPRESSED);
                        continue;
                    }
                    write!(
                        native,
                        state,
                        BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER
                    );
                }
                BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_HEADER => {
                    write!(native, loop_counter, 0);
                    let h = unsafe {
                        std::ptr::addr_of_mut!((*native).arena).cast::<HeaderArenaStorage>()
                    };
                    write!(h, sub_loop_counter, 0);
                    let symbols = unsafe {
                        std::ptr::addr_of_mut!((*h).symbols_lists_array)
                            .cast::<u16>()
                            .add(16)
                    };
                    write!(h, symbol_lists, symbols as usize);
                    write!(h, substate_huffman, BROTLI_STATE_HUFFMAN_NONE);
                    write!(h, substate_tree_group, BROTLI_STATE_TREE_GROUP_NONE);
                    write!(h, substate_context_map, BROTLI_STATE_CONTEXT_MAP_NONE);
                    write!(native, state, BROTLI_STATE_HUFFMAN_CODE_0);
                }
                BROTLI_STATE_HUFFMAN_CODE_0 => {
                    if read!(native, loop_counter) >= 3 {
                        write!(native, state, BROTLI_STATE_METABLOCK_HEADER_2);
                        continue;
                    }
                    result = reader!(|br| decode_var_len_uint8(
                        &mut (*native).substate_decode_uint8,
                        br,
                        &mut *(*native)
                            .num_block_types
                            .as_mut_ptr()
                            .cast::<MaybeUninit<Reg>>()
                            .add(read!(native, loop_counter) as usize)
                    ));
                    if result != BROTLI_DECODER_SUCCESS {
                        continue;
                    }
                    let index = read!(native, loop_counter) as usize;
                    array_write!(
                        native,
                        num_block_types,
                        index,
                        Reg,
                        array!(native, num_block_types, index, Reg) + 1
                    );
                    if array!(native, num_block_types, index, Reg) < 2 {
                        write!(native, loop_counter, read!(native, loop_counter) + 1);
                        continue;
                    }
                    write!(native, state, BROTLI_STATE_HUFFMAN_CODE_1);
                }
                BROTLI_STATE_HUFFMAN_CODE_1 => {
                    let index = read!(native, loop_counter) as usize;
                    let alphabet = array!(native, num_block_types, index, Reg) + 2;
                    let token = read!(native, block_type_trees)
                        + index * BROTLI_HUFFMAN_MAX_SIZE_258 * size_of::<HuffmanCode>();
                    result =
                        unsafe { self.read_owned_block_tree(input, token, alphabet, alphabet) };
                    if result != BROTLI_DECODER_SUCCESS {
                        continue;
                    }
                    write!(native, state, BROTLI_STATE_HUFFMAN_CODE_2);
                }
                BROTLI_STATE_HUFFMAN_CODE_2 => {
                    let index = read!(native, loop_counter) as usize;
                    let token = read!(native, block_len_trees)
                        + index * BROTLI_HUFFMAN_MAX_SIZE_26 * size_of::<HuffmanCode>();
                    result = unsafe { self.read_owned_block_tree(input, token, 26, 26) };
                    if result != BROTLI_DECODER_SUCCESS {
                        continue;
                    }
                    write!(native, state, BROTLI_STATE_HUFFMAN_CODE_3);
                }
                BROTLI_STATE_HUFFMAN_CODE_3 => {
                    let index = read!(native, loop_counter) as usize;
                    let token = read!(native, block_len_trees)
                        + index * BROTLI_HUFFMAN_MAX_SIZE_26 * size_of::<HuffmanCode>();
                    let owner = self
                        .state
                        .block_type_trees
                        .as_ref()
                        .expect("source block trees");
                    let table = unsafe { NativeTable::new(owner, token) };
                    if !reader!(|br| safe_read_block_length(
                        &mut (*native).substate_read_block_length,
                        &mut (*native).block_length_index,
                        &mut *(*native)
                            .block_length
                            .as_mut_ptr()
                            .cast::<MaybeUninit<Reg>>()
                            .add(index),
                        &table,
                        br
                    )) {
                        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                        continue;
                    }
                    write!(native, loop_counter, read!(native, loop_counter) + 1);
                    write!(native, state, BROTLI_STATE_HUFFMAN_CODE_0);
                }
                BROTLI_STATE_UNCOMPRESSED => {
                    result = unsafe {
                        self.copy_uncompressed_block_to_output_raw(
                            input,
                            available_out,
                            next_out,
                            total_out,
                        )
                    };
                    if result != BROTLI_DECODER_SUCCESS {
                        continue;
                    }
                    write!(native, state, BROTLI_STATE_METABLOCK_DONE);
                }
                BROTLI_STATE_METADATA => {
                    result = unsafe { self.skip_metadata_block(input) };
                    if result != BROTLI_DECODER_SUCCESS {
                        continue;
                    }
                    write!(native, state, BROTLI_STATE_METABLOCK_DONE);
                }
                BROTLI_STATE_METABLOCK_HEADER_2 => {
                    let mut bits = 0;
                    if !reader!(|br| br.safe_read_bits(6, &mut bits)) {
                        result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                        continue;
                    }
                    write!(native, distance_postfix_bits, bits & 3);
                    bits >>= 2;
                    write!(
                        native,
                        num_direct_distance_codes,
                        bits << read!(native, distance_postfix_bits)
                    );
                    assert!(self.state.context_modes.is_none());
                    let owner =
                        self.state
                            .allocator
                            .allocate(array!(native, num_block_types, 0, Reg) as usize);
                    let token = owner.as_ref().map_or(0, |owner| owner.as_ptr() as usize);
                    self.state.context_modes = owner;
                    write!(native, context_modes, token);
                    if token == 0 {
                        result = BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MODES;
                        continue;
                    }
                    write!(native, loop_counter, 0);
                    write!(native, state, BROTLI_STATE_CONTEXT_MODES);
                }
                BROTLI_STATE_CONTEXT_MODES => {
                    result = unsafe { self.read_context_modes(input) };
                    if result != BROTLI_DECODER_SUCCESS {
                        continue;
                    }
                    write!(native, state, BROTLI_STATE_CONTEXT_MAP_1);
                }
                BROTLI_STATE_CONTEXT_MAP_1 => {
                    result = unsafe {
                        self.read_owned_context_map(
                            input,
                            array!(native, num_block_types, 0, Reg) << BROTLI_LITERAL_CONTEXT_BITS,
                            false,
                        )
                    };
                    if result != BROTLI_DECODER_SUCCESS {
                        continue;
                    }
                    unsafe { self.detect_trivial_literal_block_types() };
                    write!(native, state, BROTLI_STATE_CONTEXT_MAP_2);
                }
                BROTLI_STATE_CONTEXT_MAP_2 => {
                    let npostfix = read!(native, distance_postfix_bits);
                    let ndirect = read!(native, num_direct_distance_codes);
                    let mut maximum = brotli_distance_alphabet_size(
                        npostfix as u32,
                        ndirect as u32,
                        BROTLI_MAX_DISTANCE_BITS,
                    ) as Reg;
                    let mut limit = maximum;
                    if unsafe { self.stream_flag(5, 1) } != 0 {
                        let bound = brotli_calculate_distance_code_limit(
                            BROTLI_MAX_ALLOWED_DISTANCE,
                            npostfix as u32,
                            ndirect as u32,
                        );
                        maximum = brotli_distance_alphabet_size(
                            npostfix as u32,
                            ndirect as u32,
                            BROTLI_LARGE_MAX_DISTANCE_BITS,
                        ) as Reg;
                        limit = bound.max_alphabet_size as Reg;
                    }
                    result = unsafe {
                        self.read_owned_context_map(
                            input,
                            array!(native, num_block_types, 2, Reg) << BROTLI_DISTANCE_CONTEXT_BITS,
                            true,
                        )
                    };
                    if result != BROTLI_DECODER_SUCCESS {
                        continue;
                    }
                    let mut success = true;
                    success &= self.state.huffman_group_init(
                        0,
                        256,
                        256,
                        read!(native, num_literal_htrees),
                    );
                    success &= self.state.huffman_group_init(
                        1,
                        704,
                        704,
                        array!(native, num_block_types, 1, Reg),
                    );
                    success &= self.state.huffman_group_init(
                        2,
                        maximum,
                        limit,
                        read!(native, num_dist_htrees),
                    );
                    if !success {
                        return save!(BROTLI_DECODER_ERROR_ALLOC_TREE_GROUPS);
                    }
                    write!(native, loop_counter, 0);
                    write!(native, state, BROTLI_STATE_TREE_GROUP);
                }
                BROTLI_STATE_TREE_GROUP => {
                    let group = match read!(native, loop_counter) {
                        0 => 0,
                        1 => 1,
                        2 => 2,
                        _ => return save!(BROTLI_DECODER_ERROR_UNREACHABLE),
                    };
                    result = unsafe { self.decode_owned_tree_group(input, group) };
                    if result != BROTLI_DECODER_SUCCESS {
                        continue;
                    }
                    write!(native, loop_counter, read!(native, loop_counter) + 1);
                    if read!(native, loop_counter) < 3 {
                        continue;
                    }
                    write!(native, state, BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_BODY);
                }
                BROTLI_STATE_BEFORE_COMPRESSED_METABLOCK_BODY => {
                    unsafe { self.prepare_literal_decoding() };
                    write!(
                        native,
                        dist_context_map_slice,
                        read!(native, dist_context_map)
                    );
                    let token = unsafe {
                        (*(*native).insert_copy_hgroup.as_ptr())
                            .htrees
                            .assume_init()
                    };
                    let command = unsafe {
                        pointer_at(
                            self.state.hgroups[1]
                                .as_ref()
                                .expect("source command trees"),
                            token,
                            0,
                        )
                    };
                    write!(native, htree_command, command);
                    if !unsafe { self.state.ensure_ringbuffer() } {
                        result = BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_2;
                        continue;
                    }
                    unsafe {
                        calculate_distance_lut(
                            &mut *std::ptr::addr_of_mut!((*native).arena)
                                .cast::<BodyArenaStorage>(),
                            &(*native).distance_postfix_bits,
                            &(*native).num_direct_distance_codes,
                            &(*(*native).distance_hgroup.as_ptr()).alphabet_size_limit,
                        )
                    };
                    write!(native, state, BROTLI_STATE_COMMAND_BEGIN);
                }
                BROTLI_STATE_COMMAND_BEGIN
                | BROTLI_STATE_COMMAND_INNER
                | BROTLI_STATE_COMMAND_POST_DECODE_LITERALS
                | BROTLI_STATE_COMMAND_POST_WRAP_COPY => {
                    result = unsafe { self.process_commands(input, bindings) };
                    if result == BROTLI_DECODER_NEEDS_MORE_INPUT {
                        result = unsafe { self.safe_process_commands(input, bindings) };
                    }
                }
                BROTLI_STATE_COMMAND_INNER_WRITE
                | BROTLI_STATE_COMMAND_POST_WRITE_1
                | BROTLI_STATE_COMMAND_POST_WRITE_2 => {
                    result = unsafe {
                        self.write_ringbuffer_raw(available_out, next_out, total_out, false)
                    };
                    if result != BROTLI_DECODER_SUCCESS {
                        continue;
                    }
                    unsafe { self.state.wrap_ringbuffer() };
                    if read!(native, ringbuffer_size) == 1i32 << unsafe { self.stream_flag(6, 6) } {
                        write!(native, max_distance, read!(native, max_backward_distance));
                    }
                    if read!(native, state) == BROTLI_STATE_COMMAND_POST_WRITE_1 {
                        if read!(native, compound_dictionary) != 0 {
                            let owner = self
                                .state
                                .compound_dictionary
                                .as_ref()
                                .expect("source addon");
                            assert_eq!(owner.as_ptr() as usize, read!(native, compound_dictionary));
                            let addon = owner.as_ptr().cast::<CompoundDictionaryStorage>();
                            if read!(addon, br_length) != read!(addon, br_copied) {
                                let copied = unsafe {
                                    self.state
                                        .copy_from_compound_dictionary(bindings, read!(native, pos))
                                };
                                write!(native, pos, read!(native, pos) + copied);
                                if read!(native, pos) >= read!(native, ringbuffer_size) {
                                    continue;
                                }
                            }
                        }
                        write!(
                            native,
                            state,
                            if read!(native, meta_block_remaining_len) == 0 {
                                BROTLI_STATE_METABLOCK_DONE
                            } else {
                                BROTLI_STATE_COMMAND_BEGIN
                            }
                        );
                        continue;
                    } else if read!(native, state) == BROTLI_STATE_COMMAND_POST_WRITE_2 {
                        write!(native, state, BROTLI_STATE_COMMAND_POST_WRAP_COPY);
                    } else {
                        if read!(native, loop_counter) == 0 {
                            write!(
                                native,
                                state,
                                if read!(native, meta_block_remaining_len) == 0 {
                                    BROTLI_STATE_METABLOCK_DONE
                                } else {
                                    BROTLI_STATE_COMMAND_POST_DECODE_LITERALS
                                }
                            );
                            continue;
                        }
                        write!(native, state, BROTLI_STATE_COMMAND_INNER);
                    }
                }
                BROTLI_STATE_METABLOCK_DONE => {
                    if read!(native, meta_block_remaining_len) < 0 {
                        result = BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_2;
                        continue;
                    }
                    self.state.cleanup_after_metablock();
                    if unsafe { self.stream_flag(0, 1) } == 0 {
                        write!(native, state, BROTLI_STATE_METABLOCK_BEGIN);
                        continue;
                    }
                    if !reader!(|br| br.jump_to_byte_boundary()) {
                        result = BROTLI_DECODER_ERROR_FORMAT_PADDING_2;
                        continue;
                    }
                    if read!(native, buffer_length) == 0 {
                        reader!(|br| br.unload());
                        let count = reader!(|br| br.available_in());
                        unsafe {
                            available_in.write(count);
                        }
                        let pointer = unsafe { input.next_pointer(&*self.source_reader_pointer()) };
                        unsafe {
                            next_in.write(pointer);
                        }
                    }
                    write!(native, state, BROTLI_STATE_DONE);
                }
                BROTLI_STATE_DONE => {
                    if read!(native, ringbuffer) != 0 {
                        result = unsafe {
                            self.write_ringbuffer_raw(available_out, next_out, total_out, true)
                        };
                        if result != BROTLI_DECODER_SUCCESS {
                            continue;
                        }
                    }
                    return save!(result);
                }
                // The original switch deliberately has no default arm. Keep
                // the same no-transition loop for out-of-protocol raw states.
                _ => {}
            }
        }
        save!(result)
    }
}
