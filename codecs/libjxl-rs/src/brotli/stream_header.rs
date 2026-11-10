// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Decoder header operations on the same actual native state and allocations.
//! The source algorithm helpers retain their own checkpoint/state bodies;
//! these adapters project only disjoint reached native fields and actual
//! allocator-owned output regions. Every loan ends before an allocation.
use super::bit_reader::Reg;
use super::context_map::{
    ContextMapFields, ContextMapOwner, decode_context_map, split_context_map,
};
use super::huffman::HuffmanCode;
use super::huffman_reader::{read_huffman_code, split_header};
use super::instance::DecoderInstance;
use super::layout::{DecoderStateStorage, HeaderArenaStorage, HuffmanTreeGroupStorage};
use super::memory::Allocation;
use super::status::*;
use super::stream_input::SourceInput;
use super::tree_group::{TreeGroupOutput, huffman_tree_group_decode, split_tree_group};
use std::mem::{MaybeUninit, align_of, size_of};

struct MapOwner<'s, 'd, 'i> {
    decoder: &'s mut DecoderInstance<'d>,
    input: SourceInput<'i>,
    distance: bool,
}
impl ContextMapOwner for MapOwner<'_, '_, '_> {
    fn with_fields<R>(
        &mut self,
        operation: impl FnOnce(
            ContextMapFields<'_>,
            &mut super::bit_reader::BitReader<'_>,
            Option<&mut [MaybeUninit<u8>]>,
        ) -> R,
    ) -> R {
        let native = self
            .decoder
            .state
            .as_ptr()
            .cast_mut()
            .cast::<DecoderStateStorage>();
        // SAFETY: Actual distinct native fields, all MaybeUninit storage.
        // No whole-state/header reference survives projection into these
        // sealed field loans. The reader/input/map fields are disjoint.
        let fields = unsafe {
            ContextMapFields {
                header: split_context_map(
                    &mut *std::ptr::addr_of_mut!((*native).arena).cast::<HeaderArenaStorage>(),
                ),
                num_htrees: if self.distance {
                    &mut (*native).num_dist_htrees
                } else {
                    &mut (*native).num_literal_htrees
                },
                map_slot: if self.distance {
                    &mut (*native).dist_context_map
                } else {
                    &mut (*native).context_map
                },
                uint8_substate: &mut (*native).substate_decode_uint8,
                mtf_upper_bound: &mut (*native).mtf_upper_bound,
                mtf: &mut (*native).mtf,
            }
        };
        let br = unsafe { &mut *self.decoder.source_reader_pointer() };
        let map = if self.distance {
            self.decoder.state.dist_context_map.as_mut()
        } else {
            self.decoder.state.context_map.as_mut()
        }
        .map(Allocation::storage_mut);
        // SAFETY: The actual stream caller initialized this reader/input
        // binding. The operation's field/map loans end within this region;
        // source allocation runs only after the complete reader flush/return.
        unsafe {
            self.input
                .with_reader(br, |reader| operation(fields, reader, map))
        }
    }
    fn allocate_map(&mut self, size: usize) -> bool {
        let slot = if self.distance {
            &self.decoder.state.dist_context_map
        } else {
            &self.decoder.state.context_map
        };
        assert!(
            slot.is_none(),
            "source fresh context map after prior cleanup"
        );
        // No native field/input/reader/map reference is live here. The old
        // actual native pointer remains unchanged during the callback RHS.
        let allocation = self.decoder.state.allocator.allocate(size);
        let token = allocation
            .as_ref()
            .map_or(0, |owner| owner.as_ptr() as usize);
        if self.distance {
            self.decoder.state.dist_context_map = allocation;
            self.decoder
                .state
                .storage_mut()
                .dist_context_map
                .write(token);
        } else {
            self.decoder.state.context_map = allocation;
            self.decoder.state.storage_mut().context_map.write(token);
        }
        token != 0
    }
}

impl DecoderInstance<'_> {
    /// ReadContextModes from decode.c1682-1696, retaining the original stale
    /// loop_counter on success and saved counter only on input shortage.
    ///
    /// # Safety
    /// Compressed-header caller initialized the reached reader/state fields
    /// and actual context_modes allocation. Input/owner loans are disjoint.
    pub unsafe fn read_context_modes(&mut self, input: SourceInput<'_>) -> i32 {
        let mut i = unsafe { self.state.storage().loop_counter.assume_init() };
        while i < unsafe {
            self.state
                .storage()
                .num_block_types
                .as_ptr()
                .cast::<Reg>()
                .read()
        } as i32
        {
            let mut bits = 0;
            let br = unsafe { &mut *self.source_reader_pointer() };
            if !unsafe { input.with_reader(br, |reader| reader.safe_read_bits(2, &mut bits)) } {
                self.state.storage_mut().loop_counter.write(i);
                return BROTLI_DECODER_NEEDS_MORE_INPUT;
            }
            // Resolve the actual source pointer only when its write is reached.
            let token = unsafe { self.state.storage().context_modes.assume_init() };
            let owner = self
                .state
                .context_modes
                .as_mut()
                .expect("actual context_modes owner");
            assert_eq!(owner.as_ptr() as usize, token);
            owner.storage_mut()[i as usize].write(bits as u8);
            i += 1;
        }
        BROTLI_DECODER_SUCCESS
    }

    /// Same source DecodeContextMap, bound to literal or distance sibling
    /// fields and the corresponding actual retained owner, without copies.
    ///
    /// # Safety
    /// Actual compressed-header entry/resume history establishes the frozen
    /// helper's initialized fields/output prefix and allocation lifecycle.
    pub unsafe fn read_owned_context_map(
        &mut self,
        input: SourceInput<'_>,
        size: Reg,
        distance: bool,
    ) -> i32 {
        unsafe {
            decode_context_map(
                size,
                &mut MapOwner {
                    decoder: self,
                    input,
                    distance,
                },
            )
        }
    }

    /// Source ReadHuffmanCode with its block-type/length output token resolved
    /// against the original single allocation. No integer address is read as
    /// a pointer and no header/table payload is cloned or reparsed.
    ///
    /// # Safety
    /// Reached compressed-header history supplies initialized source fields
    /// and an aligned output token into the actual block-tree allocation.
    pub unsafe fn read_owned_block_tree(
        &mut self,
        input: SourceInput<'_>,
        token: usize,
        maximum: Reg,
        limit: Reg,
    ) -> i32 {
        let native = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        let br = unsafe { &mut *self.source_reader_pointer() };
        let (mut fields, _) = unsafe {
            split_header(&mut *std::ptr::addr_of_mut!((*native).arena).cast::<HeaderArenaStorage>())
        };
        let allocation = self
            .state
            .block_type_trees
            .as_mut()
            .expect("actual block tree owner");
        let offset = token.checked_sub(allocation.as_ptr() as usize).unwrap();
        assert!(offset <= allocation.len());
        assert_eq!(offset % size_of::<HuffmanCode>(), 0);
        let bytes = &mut allocation.storage_mut()[offset..];
        assert_eq!(bytes.len() % size_of::<HuffmanCode>(), 0);
        assert_eq!(bytes.as_ptr() as usize % align_of::<HuffmanCode>(), 0);
        let output = unsafe {
            std::slice::from_raw_parts_mut(
                bytes.as_mut_ptr().cast::<MaybeUninit<HuffmanCode>>(),
                bytes.len() / size_of::<HuffmanCode>(),
            )
        };
        unsafe {
            input.with_reader(br, |reader| {
                read_huffman_code(maximum, limit, output, None, reader, &mut fields)
            })
        }
    }

    /// Source HuffmanTreeGroupDecode borrowing its actual combined pointer/code
    /// allocation. The original stream counts are in1..256 and fit the native
    /// descriptor field without narrowing the allocation geometry.
    ///
    /// # Safety
    /// Same source GroupInit/entry/checkpoint contracts as the frozen helper;
    /// its actual source caller released the prior group before allocation.
    pub unsafe fn decode_owned_tree_group(&mut self, input: SourceInput<'_>, group: usize) -> i32 {
        let native = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        let br = unsafe { &mut *self.source_reader_pointer() };
        let group_ptr = unsafe {
            match group {
                0 => std::ptr::addr_of!((*native).literal_hgroup),
                1 => std::ptr::addr_of!((*native).insert_copy_hgroup),
                2 => std::ptr::addr_of!((*native).distance_hgroup),
                _ => panic!("source tree group index"),
            }
        }
        .cast::<HuffmanTreeGroupStorage>();
        let count = unsafe { (*group_ptr).num_htrees.assume_init() } as usize;
        let (mut fields, mut huffman) = unsafe {
            split_tree_group(
                &mut *std::ptr::addr_of_mut!((*native).arena).cast::<HeaderArenaStorage>(),
            )
        };
        let allocation = self.state.hgroups[group]
            .as_mut()
            .expect("actual combined group owner");
        let mut output = TreeGroupOutput::new(allocation.storage_mut(), count);
        unsafe {
            input.with_reader(br, |reader| {
                huffman_tree_group_decode(
                    &*group_ptr,
                    &mut output,
                    &mut fields,
                    &mut huffman,
                    reader,
                )
            })
        }
    }
}
