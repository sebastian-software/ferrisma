// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Complete source decode.c:1179-1323 block switching/literal preparation.
use super::bit_reader::Reg;
use super::block_length::{read_block_length, safe_read_block_length};
use super::constants::{BROTLI_DISTANCE_CONTEXT_BITS, BROTLI_LITERAL_CONTEXT_BITS};
use super::huffman::{BROTLI_HUFFMAN_MAX_SIZE_26, BROTLI_HUFFMAN_MAX_SIZE_258, HuffmanCode};
use super::instance::DecoderInstance;
use super::layout::DecoderStateStorage;
use super::status::*;
use super::stream_input::SourceInput;
use super::stream_tables::{NativeTable, byte_at, pointer_at};
use super::symbols::{read_symbol, safe_read_symbol};
use super::tables::CONTEXT_LOOKUP;
use std::mem::size_of;
macro_rules! read {
    ($p:expr,$f:ident) => {
        unsafe { (*$p).$f.assume_init() }
    };
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
    ($p:expr,$f:ident,$i:expr,$ty:ty) => {
        unsafe { (*$p).$f.as_ptr().cast::<$ty>().add($i).read() }
    };
}
macro_rules! array_write {
    ($p:expr,$f:ident,$i:expr,$ty:ty,$v:expr) => {{
        let value = $v;
        unsafe {
            (*$p).$f.as_mut_ptr().cast::<$ty>().add($i).write(value);
        }
    }};
}
impl DecoderInstance<'_> {
    /// # Safety
    /// Exact original block-switch fields/tables/input and selected tree_type
    /// 0..3 have their initialized source domains; source loans are disjoint.
    #[inline(always)]
    pub(crate) unsafe fn decode_block_type_and_length(
        &mut self,
        safe: i32,
        input: SourceInput<'_>,
        tree_type: usize,
    ) -> i32 {
        assert!(tree_type < 3);
        let native = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        let max_block_type = array!(native, num_block_types, tree_type, Reg);
        let type_token = read!(native, block_type_trees)
            + tree_type * BROTLI_HUFFMAN_MAX_SIZE_258 * size_of::<HuffmanCode>();
        let len_token = read!(native, block_len_trees)
            + tree_type * BROTLI_HUFFMAN_MAX_SIZE_26 * size_of::<HuffmanCode>();
        let br = unsafe { &mut *self.source_reader_pointer() };
        let owner = self
            .state
            .block_type_trees
            .as_ref()
            .expect("actual source block trees");
        let type_tree = unsafe { NativeTable::new(owner, type_token) };
        let len_tree = unsafe { NativeTable::new(owner, len_token) };
        if max_block_type <= 1 {
            return BROTLI_DECODER_ERROR_FORMAT_BLOCK_SWITCH;
        }
        let mut block_type = 0;
        let result = unsafe {
            input.with_reader(br, |br| {
                if safe == 0 {
                    block_type = read_symbol(&type_tree, br);
                    let length = read_block_length(&len_tree, br);
                    (*native)
                        .block_length
                        .as_mut_ptr()
                        .cast::<Reg>()
                        .add(tree_type)
                        .write(length);
                } else {
                    let memento = br.save();
                    if !safe_read_symbol(&type_tree, br, &mut block_type) {
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    }
                    let length = &mut *(*native)
                        .block_length
                        .as_mut_ptr()
                        .cast::<std::mem::MaybeUninit<Reg>>()
                        .add(tree_type);
                    if !safe_read_block_length(
                        &mut (*native).substate_read_block_length,
                        &mut (*native).block_length_index,
                        length,
                        &len_tree,
                        br,
                    ) {
                        (*native)
                            .substate_read_block_length
                            .write(super::running::BROTLI_STATE_READ_BLOCK_LENGTH_NONE);
                        br.restore(memento);
                        return BROTLI_DECODER_NEEDS_MORE_INPUT;
                    }
                }
                BROTLI_DECODER_SUCCESS
            })
        };
        if result != BROTLI_DECODER_SUCCESS {
            return result;
        }
        if block_type == 1 {
            block_type = array!(native, block_type_rb, tree_type * 2 + 1, Reg).wrapping_add(1);
        } else if block_type == 0 {
            block_type = array!(native, block_type_rb, tree_type * 2, Reg);
        } else {
            block_type = block_type.wrapping_sub(2);
        }
        if block_type >= max_block_type {
            block_type = block_type.wrapping_sub(max_block_type);
        }
        array_write!(
            native,
            block_type_rb,
            tree_type * 2,
            Reg,
            array!(native, block_type_rb, tree_type * 2 + 1, Reg)
        );
        array_write!(native, block_type_rb, tree_type * 2 + 1, Reg, block_type);
        BROTLI_DECODER_SUCCESS
    }

    /// # Safety
    /// Source successful context-map decode initialized its complete payload;
    /// num_block_types[0] is in1..256 and source native fields are disjoint.
    #[inline(always)]
    pub(crate) unsafe fn detect_trivial_literal_block_types(&mut self) {
        let native = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        for i in 0..8 {
            array_write!(native, trivial_literal_contexts, i, u32, 0);
        }
        let mut i = 0;
        while i < array!(native, num_block_types, 0, Reg) as usize {
            let offset = i << BROTLI_LITERAL_CONTEXT_BITS;
            let mut error = 0usize;
            let token = read!(native, context_map);
            let owner = self
                .state
                .context_map
                .as_ref()
                .expect("actual literal context map");
            let sample = unsafe { byte_at(owner, token, offset) } as usize;
            let mut j = 0;
            macro_rules! step {
                () => {{
                    error |= (unsafe { byte_at(owner, token, offset + j) } as usize) ^ sample;
                    j += 1;
                }};
            }
            while j < (1 << BROTLI_LITERAL_CONTEXT_BITS) {
                step!();
                step!();
                step!();
                step!();
            }
            if error == 0 {
                array_write!(
                    native,
                    trivial_literal_contexts,
                    i >> 5,
                    u32,
                    array!(native, trivial_literal_contexts, i >> 5, u32) | (1u32 << (i & 31))
                );
            }
            i += 1;
        }
    }

    /// # Safety
    /// Source initialized context modes/maps, group pointers and block ring;
    /// each selected slot is within that same actual owner and initialized.
    #[inline(always)]
    pub(crate) unsafe fn prepare_literal_decoding(&mut self) {
        let native = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        let block_type = array!(native, block_type_rb, 1, Reg);
        let context_offset = block_type << BROTLI_LITERAL_CONTEXT_BITS;
        let token = read!(native, context_map);
        let map = self
            .state
            .context_map
            .as_ref()
            .expect("actual literal context map");
        assert_eq!(map.as_ptr() as usize, token);
        assert!(context_offset as usize <= map.len());
        let slice = unsafe { map.as_ptr().add(context_offset as usize) } as usize;
        write!(native, context_map_slice, slice);
        let trivial = array!(
            native,
            trivial_literal_contexts,
            (block_type >> 5) as usize,
            u32
        ) as usize;
        write!(
            native,
            trivial_literal_context,
            ((trivial >> (block_type & 31)) & 1) as i32
        );
        let group = unsafe { &*(*native).literal_hgroup.as_ptr() };
        let index = unsafe { byte_at(map, slice, 0) } as usize;
        let htrees = unsafe { group.htrees.assume_init() };
        let literal = unsafe {
            pointer_at(
                self.state.hgroups[0]
                    .as_ref()
                    .expect("actual literal trees"),
                htrees,
                index,
            )
        };
        write!(native, literal_htree, literal);
        let modes_token = read!(native, context_modes);
        let mode = unsafe {
            byte_at(
                self.state
                    .context_modes
                    .as_ref()
                    .expect("actual context modes"),
                modes_token,
                block_type as usize,
            )
        } & 3;
        write!(
            native,
            context_lookup,
            unsafe { CONTEXT_LOOKUP.as_ptr().add((mode as usize) << 9) } as usize
        );
    }

    #[inline(always)]
    unsafe fn decode_literal_block_switch_internal(
        &mut self,
        safe: i32,
        input: SourceInput<'_>,
    ) -> i32 {
        let result = unsafe { self.decode_block_type_and_length(safe, input, 0) };
        if result != BROTLI_DECODER_SUCCESS {
            return result;
        }
        unsafe { self.prepare_literal_decoding() };
        BROTLI_DECODER_SUCCESS
    }
    #[inline(always)]
    unsafe fn decode_command_block_switch_internal(
        &mut self,
        safe: i32,
        input: SourceInput<'_>,
    ) -> i32 {
        let result = unsafe { self.decode_block_type_and_length(safe, input, 1) };
        if result != BROTLI_DECODER_SUCCESS {
            return result;
        }
        let native = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        let token = unsafe {
            (*(*native).insert_copy_hgroup.as_ptr())
                .htrees
                .assume_init()
        };
        let index = array!(native, block_type_rb, 3, Reg) as usize;
        let value = unsafe {
            pointer_at(
                self.state.hgroups[1]
                    .as_ref()
                    .expect("actual command trees"),
                token,
                index,
            )
        };
        write!(native, htree_command, value);
        BROTLI_DECODER_SUCCESS
    }
    #[inline(always)]
    unsafe fn decode_distance_block_switch_internal(
        &mut self,
        safe: i32,
        input: SourceInput<'_>,
    ) -> i32 {
        let result = unsafe { self.decode_block_type_and_length(safe, input, 2) };
        if result != BROTLI_DECODER_SUCCESS {
            return result;
        }
        let native = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        let token = read!(native, dist_context_map);
        let owner = self
            .state
            .dist_context_map
            .as_ref()
            .expect("actual distance contexts");
        assert_eq!(owner.as_ptr() as usize, token);
        let offset =
            (array!(native, block_type_rb, 5, Reg) << BROTLI_DISTANCE_CONTEXT_BITS) as usize;
        assert!(offset <= owner.len());
        let slice = unsafe { owner.as_ptr().add(offset) } as usize;
        write!(native, dist_context_map_slice, slice);
        let context = read!(native, distance_context) as usize;
        let value = unsafe { byte_at(owner, slice, context) };
        write!(native, dist_htree_index, value);
        BROTLI_DECODER_SUCCESS
    }
    #[inline(never)]
    pub(crate) unsafe fn decode_literal_block_switch(&mut self, input: SourceInput<'_>) -> i32 {
        unsafe { self.decode_literal_block_switch_internal(0, input) }
    }
    #[inline(never)]
    pub(crate) unsafe fn safe_decode_literal_block_switch(
        &mut self,
        input: SourceInput<'_>,
    ) -> i32 {
        unsafe { self.decode_literal_block_switch_internal(1, input) }
    }

    #[inline(never)]
    pub(crate) unsafe fn decode_command_block_switch(&mut self, input: SourceInput<'_>) -> i32 {
        unsafe { self.decode_command_block_switch_internal(0, input) }
    }
    #[inline(never)]
    pub(crate) unsafe fn safe_decode_command_block_switch(
        &mut self,
        input: SourceInput<'_>,
    ) -> i32 {
        unsafe { self.decode_command_block_switch_internal(1, input) }
    }

    #[inline(never)]
    pub(crate) unsafe fn decode_distance_block_switch(&mut self, input: SourceInput<'_>) -> i32 {
        unsafe { self.decode_distance_block_switch_internal(0, input) }
    }
    #[inline(never)]
    pub(crate) unsafe fn safe_decode_distance_block_switch(
        &mut self,
        input: SourceInput<'_>,
    ) -> i32 {
        unsafe { self.decode_distance_block_switch_internal(1, input) }
    }
}
