// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Complete source decode.c:1984-2380 command kernel. Local labels represent
//! the original gotos; actual state fields remain in their callback-owned block.
use super::bit_reader::{Reg, bit_mask};
use super::command::{CommandFields, check_input_amount, read_command_internal};
use super::compound_dictionary::CompoundDictionaryBindings;
use super::distance::{DistanceFields, DistanceRingFields, read_distance_internal};
use super::instance::DecoderInstance;
use super::layout::{BodyArenaStorage, DecoderStateStorage, SharedDictionaryStorage};
use super::raw_transform::RawByteDestination;
use super::running::*;
use super::static_descriptors::{
    DICTIONARY, DictionaryDescriptor, TRANSFORMS, TransformDescriptor,
};
use super::status::*;
use super::stream_input::SourceInput;
use super::stream_tables::{NativeTable, byte_at, pointer_at};
use super::symbols::{
    copy_preloaded_symbols_to_u8_with_writer, preload_symbol, read_symbol, safe_read_symbol,
};
use super::tables::CONTEXT_LOOKUP;
use super::transform::{brotli_get_transforms, brotli_transform_dictionary_word_to};
use std::mem::MaybeUninit;
macro_rules! read {
    ($p:expr,$f:ident) => {{
        #[allow(
            unused_unsafe,
            reason = "Field projection macros also nest inside source operation loans"
        )]
        let value = unsafe { (*$p).$f.assume_init() };
        value
    }};
}
macro_rules! write {
    ($p:expr,$f:ident,$v:expr) => {{
        let v = $v;
        unsafe {
            (*$p).$f.write(v);
        }
    }};
}
macro_rules! array {
    ($p:expr,$f:ident,$i:expr,$ty:ty) => {{
        let index = $i;
        #[allow(
            unused_unsafe,
            reason = "Field projection macros also nest inside source operation loans"
        )]
        let value = unsafe { (*$p).$f.as_ptr().cast::<$ty>().add(index).read() };
        value
    }};
}
macro_rules! array_write {
    ($p:expr,$f:ident,$i:expr,$ty:ty,$v:expr) => {{
        let v = $v;
        let index = $i;
        unsafe {
            (*$p).$f.as_mut_ptr().cast::<$ty>().add(index).write(v);
        }
    }};
}
#[derive(Clone, Copy)]
enum Label {
    Begin,
    Inner,
    PostLiterals,
    PostWrap,
    NextLiteral,
    Save,
}
impl<'a> DecoderInstance<'a> {
    // Only source-default static descriptors can be installed: serialized
    // attachment is not compiled without BROTLI_EXPERIMENTAL. Resolve tokens
    // against their real named static owners, never dereference an integer.
    unsafe fn command_dictionary(
        &self,
        index: usize,
    ) -> (&'static DictionaryDescriptor, &'static TransformDescriptor) {
        let native = self.state.as_ptr().cast::<DecoderStateStorage>();
        let owner = self
            .state
            .dictionary
            .as_ref()
            .expect("source shared dictionary");
        assert_eq!(read!(native, dictionary), owner.as_ptr() as usize);
        let shared = owner.as_ptr().cast::<SharedDictionaryStorage>();
        assert!(index < read!(shared, num_dictionaries) as usize);
        assert_eq!(
            array!(shared, words, index, usize),
            &DICTIONARY as *const _ as usize
        );
        assert_eq!(
            array!(shared, transforms, index, usize),
            &TRANSFORMS as *const _ as usize
        );
        (&DICTIONARY, &TRANSFORMS)
    }
    unsafe fn command_context(&self, p1: u8, p2: u8) -> u8 {
        let native = self.state.as_ptr().cast::<DecoderStateStorage>();
        let offset = read!(native, context_lookup)
            .checked_sub(CONTEXT_LOOKUP.as_ptr() as usize)
            .unwrap();
        assert!(offset + 512 <= CONTEXT_LOOKUP.len());
        CONTEXT_LOOKUP[offset + p1 as usize] | CONTEXT_LOOKUP[offset + 256 + p2 as usize]
    }
    unsafe fn command_ring_read(&self, index: i32) -> u8 {
        let native = self.state.as_ptr().cast::<DecoderStateStorage>();
        let ring = self.state.ringbuffer.as_ref().expect("source ring");
        assert_eq!(read!(native, ringbuffer), ring.as_ptr() as usize);
        assert!(index >= 0 && (index as usize) < ring.len());
        // SAFETY: Reached source history is initialized by initial two zero
        // bytes or previous logical literal/copy/transform writes.
        unsafe { ring.as_ptr().add(index as usize).read() }
    }
    unsafe fn command_ring_write(&mut self, index: i32, value: u8) {
        let native = self.state.as_ptr().cast::<DecoderStateStorage>();
        let ring = self.state.ringbuffer.as_mut().expect("source ring");
        assert_eq!(read!(native, ringbuffer), ring.as_ptr() as usize);
        ring.storage_mut()[index as usize].write(value);
    }
    unsafe fn command_ring_move(&mut self, dst: i32, src: i32, count: usize, overlap: bool) {
        let ring = self.state.ringbuffer.as_mut().expect("source ring");
        let dst = usize::try_from(dst).unwrap();
        let src = usize::try_from(src).unwrap();
        assert!(src <= ring.len() && count <= ring.len() - src);
        assert!(dst <= ring.len() && count <= ring.len() - dst);
        let p = ring.storage_mut().as_mut_ptr();
        // SAFETY: Source's speculative fixed-size copy includes slack bytes
        // that may be unknown; copy MaybeUninit representations, never u8s.
        // ptr::copy retains memmove16's temporary-object overlap behavior.
        unsafe {
            if overlap {
                std::ptr::copy(p.add(src), p.add(dst), count);
            } else {
                std::ptr::copy_nonoverlapping(p.add(src), p.add(dst), count);
            }
        }
    }
    unsafe fn command_read(&mut self, safe: i32, input: SourceInput<'_>, i: &mut i32) -> bool {
        let native = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        let owner = self.state.hgroups[1]
            .as_ref()
            .expect("source command trees");
        let map = self
            .state
            .dist_context_map
            .as_ref()
            .expect("source distance map");
        let mut length = MaybeUninit::new(*i);
        // SAFETY: Project only disjoint selected native fields. No allocator
        // or external callback runs during this one reader/table loan.
        let ok = unsafe {
            input.with_reader(&mut *self.source_reader_pointer(), |br| {
                read_command_internal(
                    safe,
                    &mut CommandFields {
                        distance_code: &mut (*native).distance_code,
                        distance_context: &mut (*native).distance_context,
                        dist_htree_index: &mut (*native).dist_htree_index,
                        copy_length: &mut (*native).copy_length,
                        block_length: &mut *(*native)
                            .block_length
                            .as_mut_ptr()
                            .cast::<MaybeUninit<Reg>>()
                            .add(1),
                    },
                    br,
                    &mut length,
                    || NativeTable::new(owner, read!(native, htree_command)),
                    |index| byte_at(map, read!(native, dist_context_map_slice), index),
                )
            })
        };
        // ReadCommand stores the initial insert offset even when suffix read
        // fails. It is a direct caller-local write, not a success-only output.
        *i = unsafe { length.assume_init() };
        ok
    }
    unsafe fn command_distance(&mut self, safe: i32, input: SourceInput<'_>) -> bool {
        let native = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        let owner = self.state.hgroups[2]
            .as_ref()
            .expect("source distance trees");
        unsafe {
            input.with_reader(&mut *self.source_reader_pointer(), |br| {
                read_distance_internal(
                    safe,
                    &mut DistanceFields {
                        ring: DistanceRingFields {
                            distance_code: &mut (*native).distance_code,
                            distance_context: &mut (*native).distance_context,
                            dist_rb_idx: &mut (*native).dist_rb_idx,
                            dist_rb: &(*native).dist_rb,
                        },
                        block_length: &mut *(*native)
                            .block_length
                            .as_mut_ptr()
                            .cast::<MaybeUninit<Reg>>()
                            .add(2),
                        distance_postfix_bits: &(*native).distance_postfix_bits,
                        dist_htree_index: &(*native).dist_htree_index,
                        body: &*std::ptr::addr_of!((*native).arena).cast::<BodyArenaStorage>(),
                    },
                    br,
                    |index| {
                        let token = (*(*native).distance_hgroup.as_ptr()).htrees.assume_init();
                        let table = pointer_at(owner, token, index as usize);
                        NativeTable::new(owner, table)
                    },
                )
            })
        }
    }
    /// # Safety
    /// The exact original command-entry state/header/body/history checkpoints
    /// hold in this same live decoder; reached initialized tables/input/ring
    /// extents and source arithmetic are valid. No source loan survives calls.
    #[inline(always)]
    unsafe fn process_commands_internal(
        &mut self,
        safe: i32,
        input: SourceInput<'_>,
        bindings: &CompoundDictionaryBindings<'a>,
    ) -> i32 {
        let native = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        let mut pos = read!(native, pos);
        let mut i = read!(native, loop_counter);
        let mut result = BROTLI_DECODER_SUCCESS;
        let compound_size = unsafe { self.state.compound_dictionary_size() };
        let enough = unsafe {
            input.with_reader(&mut *self.source_reader_pointer(), |br| {
                check_input_amount(safe, br)
            })
        };
        if !enough {
            write!(native, pos, pos);
            write!(native, loop_counter, i);
            return BROTLI_DECODER_NEEDS_MORE_INPUT;
        }
        if safe == 0 {
            unsafe {
                input.with_reader(&mut *self.source_reader_pointer(), |br| {
                    br.warmup();
                });
            }
        }
        let mut label = match read!(native, state) {
            BROTLI_STATE_COMMAND_BEGIN => Label::Begin,
            BROTLI_STATE_COMMAND_INNER => Label::Inner,
            BROTLI_STATE_COMMAND_POST_DECODE_LITERALS => Label::PostLiterals,
            BROTLI_STATE_COMMAND_POST_WRAP_COPY => Label::PostWrap,
            _ => return BROTLI_DECODER_ERROR_UNREACHABLE,
        };
        macro_rules! check {
            () => {
                unsafe {
                    input.with_reader(&mut *self.source_reader_pointer(), |br| {
                        check_input_amount(safe, br)
                    })
                }
            };
        }
        macro_rules! status {
            ($expr:expr) => {{
                let status = $expr;
                if status != BROTLI_DECODER_SUCCESS {
                    result = status;
                    label = Label::Save;
                    continue;
                }
            }};
        }
        macro_rules! need_input {
            () => {{
                result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                label = Label::Save;
                continue;
            }};
        }
        loop {
            match label {
                Label::Begin => {
                    if safe != 0 {
                        write!(native, state, BROTLI_STATE_COMMAND_BEGIN);
                    }
                    if !check!() {
                        write!(native, state, BROTLI_STATE_COMMAND_BEGIN);
                        need_input!();
                    }
                    if array!(native, block_length, 1, Reg) == 0 {
                        status!(unsafe {
                            if safe != 0 {
                                self.safe_decode_command_block_switch(input)
                            } else {
                                self.decode_command_block_switch(input)
                            }
                        });
                        continue;
                    }
                    if !unsafe { self.command_read(safe, input, &mut i) } {
                        need_input!();
                    }
                    if i == 0 {
                        label = Label::PostLiterals;
                        continue;
                    }
                    write!(
                        native,
                        meta_block_remaining_len,
                        read!(native, meta_block_remaining_len) - i
                    );
                    label = Label::Inner;
                }
                Label::Inner => {
                    if safe != 0 {
                        write!(native, state, BROTLI_STATE_COMMAND_INNER);
                    }
                    if read!(native, trivial_literal_context) != 0 {
                        let mut bits = 0;
                        let mut value = 0;
                        {
                            let owner = self.state.hgroups[0]
                                .as_ref()
                                .expect("source literal trees");
                            let table =
                                unsafe { NativeTable::new(owner, read!(native, literal_htree)) };
                            unsafe {
                                input.with_reader(&mut *self.source_reader_pointer(), |br| {
                                    preload_symbol(safe, &table, br, &mut bits, &mut value)
                                });
                            }
                        }
                        if safe == 0 {
                            let mut steps = i - 1;
                            if steps > 0 && steps as Reg > array!(native, block_length, 0, Reg) {
                                steps = array!(native, block_length, 0, Reg) as i32;
                            }
                            if read!(native, ringbuffer_size) >= pos
                                && read!(native, ringbuffer_size) - pos <= steps
                            {
                                steps = read!(native, ringbuffer_size) - pos - 1;
                            }
                            if steps < 0 {
                                steps = 0;
                            }
                            let owner = self.state.hgroups[0]
                                .as_ref()
                                .expect("source literal trees");
                            let table =
                                unsafe { NativeTable::new(owner, read!(native, literal_htree)) };
                            let ring = self.state.ringbuffer.as_mut().expect("source ring");
                            let storage = ring.storage_mut();
                            steps = unsafe {
                                input.with_reader(
                                    &mut *std::ptr::addr_of_mut!((*native).br).cast(),
                                    |br| {
                                        copy_preloaded_symbols_to_u8_with_writer(
                                            &table,
                                            br,
                                            &mut bits,
                                            &mut value,
                                            |index, byte| {
                                                storage[index as usize].write(byte);
                                            },
                                            pos,
                                            steps,
                                        )
                                    },
                                )
                            };
                            pos += steps;
                            array_write!(
                                native,
                                block_length,
                                0,
                                Reg,
                                array!(native, block_length, 0, Reg).wrapping_sub(steps as Reg)
                            );
                            i -= steps;
                        }
                        loop {
                            if safe == 0 && !check!() {
                                write!(native, state, BROTLI_STATE_COMMAND_INNER);
                                result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                label = Label::Save;
                                break;
                            }
                            if array!(native, block_length, 0, Reg) == 0 {
                                label = Label::NextLiteral;
                                break;
                            }
                            let owner = self.state.hgroups[0]
                                .as_ref()
                                .expect("source literal trees");
                            let table =
                                unsafe { NativeTable::new(owner, read!(native, literal_htree)) };
                            let mut literal = 0;
                            let ok = unsafe {
                                input.with_reader(&mut *self.source_reader_pointer(), |br| {
                                    if safe != 0 {
                                        safe_read_symbol(&table, br, &mut literal)
                                    } else {
                                        let ring =
                                            self.state.ringbuffer.as_mut().expect("source ring");
                                        let storage = ring.storage_mut();
                                        copy_preloaded_symbols_to_u8_with_writer(
                                            &table,
                                            br,
                                            &mut bits,
                                            &mut value,
                                            |index, byte| {
                                                storage[index as usize].write(byte);
                                            },
                                            pos,
                                            1,
                                        );
                                        true
                                    }
                                })
                            };
                            if !ok {
                                result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                label = Label::Save;
                                break;
                            }
                            if safe != 0 {
                                unsafe { self.command_ring_write(pos, literal as u8) };
                            }
                            array_write!(
                                native,
                                block_length,
                                0,
                                Reg,
                                array!(native, block_length, 0, Reg).wrapping_sub(1)
                            );
                            pos += 1;
                            if pos == read!(native, ringbuffer_size) {
                                write!(native, state, BROTLI_STATE_COMMAND_INNER_WRITE);
                                i -= 1;
                                label = Label::Save;
                                break;
                            }
                            i -= 1;
                            if i == 0 {
                                label = Label::PostLiterals;
                                break;
                            }
                        }
                    } else {
                        let mut p1 = unsafe {
                            self.command_ring_read((pos - 1) & read!(native, ringbuffer_mask))
                        };
                        let mut p2 = unsafe {
                            self.command_ring_read((pos - 2) & read!(native, ringbuffer_mask))
                        };
                        loop {
                            if !check!() {
                                write!(native, state, BROTLI_STATE_COMMAND_INNER);
                                result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                label = Label::Save;
                                break;
                            }
                            if array!(native, block_length, 0, Reg) == 0 {
                                label = Label::NextLiteral;
                                break;
                            }
                            let context = unsafe { self.command_context(p1, p2) };
                            let map = self.state.context_map.as_ref().expect("source literal map");
                            let index = unsafe {
                                byte_at(map, read!(native, context_map_slice), context as usize)
                            };
                            let owner = self.state.hgroups[0]
                                .as_ref()
                                .expect("source literal trees");
                            let token = unsafe {
                                (*(*native).literal_hgroup.as_ptr()).htrees.assume_init()
                            };
                            let table = unsafe {
                                NativeTable::new(owner, pointer_at(owner, token, index as usize))
                            };
                            p2 = p1;
                            let mut literal = 0;
                            let ok = unsafe {
                                input.with_reader(&mut *self.source_reader_pointer(), |br| {
                                    if safe == 0 {
                                        literal = read_symbol(&table, br);
                                        true
                                    } else {
                                        safe_read_symbol(&table, br, &mut literal)
                                    }
                                })
                            };
                            if !ok {
                                result = BROTLI_DECODER_NEEDS_MORE_INPUT;
                                label = Label::Save;
                                break;
                            }
                            p1 = literal as u8;
                            unsafe { self.command_ring_write(pos, p1) };
                            array_write!(
                                native,
                                block_length,
                                0,
                                Reg,
                                array!(native, block_length, 0, Reg).wrapping_sub(1)
                            );
                            pos += 1;
                            if pos == read!(native, ringbuffer_size) {
                                write!(native, state, BROTLI_STATE_COMMAND_INNER_WRITE);
                                i -= 1;
                                label = Label::Save;
                                break;
                            }
                            i -= 1;
                            if i == 0 {
                                label = Label::PostLiterals;
                                break;
                            }
                        }
                    }
                    if matches!(label, Label::PostLiterals)
                        && read!(native, meta_block_remaining_len) <= 0
                    {
                        write!(native, state, BROTLI_STATE_METABLOCK_DONE);
                        label = Label::Save;
                    }
                }
                Label::PostLiterals => {
                    if safe != 0 {
                        write!(native, state, BROTLI_STATE_COMMAND_POST_DECODE_LITERALS);
                    }
                    if read!(native, distance_code) >= 0 {
                        write!(
                            native,
                            distance_context,
                            if read!(native, distance_code) != 0 {
                                0
                            } else {
                                1
                            }
                        );
                        write!(native, dist_rb_idx, read!(native, dist_rb_idx) - 1);
                        write!(
                            native,
                            distance_code,
                            array!(
                                native,
                                dist_rb,
                                (read!(native, dist_rb_idx) & 3) as usize,
                                i32
                            )
                        );
                    } else {
                        if array!(native, block_length, 2, Reg) == 0 {
                            status!(unsafe {
                                if safe != 0 {
                                    self.safe_decode_distance_block_switch(input)
                                } else {
                                    self.decode_distance_block_switch(input)
                                }
                            });
                        }
                        if !unsafe { self.command_distance(safe, input) } {
                            need_input!();
                        }
                    }
                    if read!(native, max_distance) != read!(native, max_backward_distance) {
                        write!(
                            native,
                            max_distance,
                            pos.min(read!(native, max_backward_distance))
                        );
                    }
                    i = read!(native, copy_length);
                    if read!(native, distance_code) > read!(native, max_distance) {
                        if read!(native, distance_code) > 0x7ffffffc {
                            return BROTLI_DECODER_ERROR_FORMAT_DISTANCE;
                        }
                        if read!(native, distance_code) - read!(native, max_distance) - 1
                            < compound_size
                        {
                            let address = compound_size
                                - (read!(native, distance_code) - read!(native, max_distance));
                            if !unsafe {
                                self.state.initialize_compound_dictionary_copy(address, i)
                            } {
                                return BROTLI_DECODER_ERROR_COMPOUND_DICTIONARY;
                            }
                            pos +=
                                unsafe { self.state.copy_from_compound_dictionary(bindings, pos) };
                            if pos >= read!(native, ringbuffer_size) {
                                write!(native, state, BROTLI_STATE_COMMAND_POST_WRITE_1);
                                label = Label::Save;
                                continue;
                            }
                        } else if (4..=24).contains(&i) {
                            let p1 = unsafe {
                                self.command_ring_read((pos - 1) & read!(native, ringbuffer_mask))
                            };
                            let p2 = unsafe {
                                self.command_ring_read((pos - 2) & read!(native, ringbuffer_mask))
                            };
                            let shared = self
                                .state
                                .dictionary
                                .as_ref()
                                .expect("source dictionary")
                                .as_ptr()
                                .cast::<SharedDictionaryStorage>();
                            let mut dict_id = if read!(shared, context_based) != 0 {
                                array!(
                                    shared,
                                    context_map,
                                    unsafe { self.command_context(p1, p2) } as usize,
                                    u8
                                )
                            } else {
                                0
                            };
                            let (mut words, mut transforms) =
                                unsafe { self.command_dictionary(dict_id as usize) };
                            let mut offset = words.offsets_by_length[i as usize] as i32;
                            let mut shift = Reg::from(words.size_bits_by_length[i as usize]);
                            let mut address = read!(native, distance_code)
                                - read!(native, max_distance)
                                - 1
                                - compound_size;
                            let mut mask = bit_mask(shift) as i32;
                            let mut word_idx = address & mask;
                            let mut transform_idx = address >> shift;
                            write!(
                                native,
                                dist_rb_idx,
                                read!(native, dist_rb_idx) + read!(native, distance_context)
                            );
                            offset += word_idx * i;
                            if (transform_idx >= transforms.num_transforms as i32
                                || words.size_bits_by_length[i as usize] == 0)
                                && read!(shared, num_dictionaries) > 1
                            {
                                let mut remaining = address
                                    - (((1u32 << shift) & !1) as i32)
                                        * transforms.num_transforms as i32;
                                let mut id = 0u8;
                                while id < read!(shared, num_dictionaries) {
                                    let (words2, transforms2) =
                                        unsafe { self.command_dictionary(id as usize) };
                                    if id != dict_id && words2.size_bits_by_length[i as usize] != 0
                                    {
                                        let shift2 =
                                            Reg::from(words2.size_bits_by_length[i as usize]);
                                        let num = (((1u32 << shift2) & !1) as i32)
                                            * transforms2.num_transforms as i32;
                                        if remaining < num {
                                            dict_id = id;
                                            words = words2;
                                            transforms = transforms2;
                                            address = remaining;
                                            shift = shift2;
                                            mask = bit_mask(shift) as i32;
                                            word_idx = address & mask;
                                            transform_idx = address >> shift;
                                            offset = words.offsets_by_length[i as usize] as i32
                                                + word_idx * i;
                                            break;
                                        }
                                        remaining -= num;
                                    }
                                    id += 1;
                                }
                            }
                            let _ = dict_id;
                            if words.size_bits_by_length[i as usize] == 0 {
                                return BROTLI_DECODER_ERROR_FORMAT_DICTIONARY;
                            }
                            if words.data.is_null() {
                                return BROTLI_DECODER_ERROR_DICTIONARY_NOT_SET;
                            }
                            if transform_idx < transforms.num_transforms as i32 {
                                let mut len = i;
                                assert_eq!(words.data, super::tables::DICTIONARY_DATA.as_ptr());
                                let offset = usize::try_from(offset).unwrap();
                                if transform_idx == i32::from(transforms.cut_off_transforms[0]) {
                                    let ring = self.state.ringbuffer.as_mut().expect("source ring");
                                    let storage = ring.storage_mut();
                                    let word = &super::tables::DICTIONARY_DATA
                                        [offset..offset + len as usize];
                                    for (dst, byte) in storage
                                        [pos as usize..pos as usize + len as usize]
                                        .iter_mut()
                                        .zip(word)
                                    {
                                        dst.write(*byte);
                                    }
                                } else {
                                    let ring = self.state.ringbuffer.as_mut().expect("source ring");
                                    let mut dst = RawByteDestination::new(
                                        &mut ring.storage_mut()[pos as usize..],
                                    );
                                    len = brotli_transform_dictionary_word_to(
                                        &mut dst,
                                        &super::tables::DICTIONARY_DATA[offset..],
                                        len,
                                        brotli_get_transforms(),
                                        transform_idx,
                                    );
                                    if len == 0 && read!(native, distance_code) <= 120 {
                                        return BROTLI_DECODER_ERROR_FORMAT_TRANSFORM;
                                    }
                                }
                                pos += len;
                                write!(
                                    native,
                                    meta_block_remaining_len,
                                    read!(native, meta_block_remaining_len) - len
                                );
                                if pos >= read!(native, ringbuffer_size) {
                                    write!(native, state, BROTLI_STATE_COMMAND_POST_WRITE_1);
                                    label = Label::Save;
                                    continue;
                                }
                            } else {
                                return BROTLI_DECODER_ERROR_FORMAT_TRANSFORM;
                            }
                        } else {
                            return BROTLI_DECODER_ERROR_FORMAT_DICTIONARY;
                        }
                    } else {
                        let src =
                            (pos - read!(native, distance_code)) & read!(native, ringbuffer_mask);
                        let dst_end = pos + i;
                        let src_end = src + i;
                        array_write!(
                            native,
                            dist_rb,
                            (read!(native, dist_rb_idx) & 3) as usize,
                            i32,
                            read!(native, distance_code)
                        );
                        write!(native, dist_rb_idx, read!(native, dist_rb_idx) + 1);
                        write!(
                            native,
                            meta_block_remaining_len,
                            read!(native, meta_block_remaining_len) - i
                        );
                        unsafe { self.command_ring_move(pos, src, 16, true) };
                        if (src_end > pos && dst_end > src)
                            || dst_end >= read!(native, ringbuffer_size)
                            || src_end >= read!(native, ringbuffer_size)
                        {
                            label = Label::PostWrap;
                            continue;
                        }
                        pos += i;
                        if i > 16 {
                            if i > 32 {
                                unsafe {
                                    self.command_ring_move(
                                        pos - i + 16,
                                        src + 16,
                                        (i - 16) as usize,
                                        false,
                                    )
                                };
                            } else {
                                unsafe { self.command_ring_move(pos - i + 16, src + 16, 16, true) };
                            }
                        }
                    }
                    if read!(native, meta_block_remaining_len) <= 0 {
                        write!(native, state, BROTLI_STATE_METABLOCK_DONE);
                        label = Label::Save;
                    } else {
                        label = Label::Begin;
                    }
                }
                Label::PostWrap => {
                    let mut guard = read!(native, ringbuffer_size) - pos;
                    loop {
                        i -= 1;
                        if i < 0 {
                            break;
                        }
                        let byte = unsafe {
                            self.command_ring_read(
                                (pos - read!(native, distance_code))
                                    & read!(native, ringbuffer_mask),
                            )
                        };
                        unsafe { self.command_ring_write(pos, byte) };
                        pos += 1;
                        guard -= 1;
                        if guard == 0 {
                            write!(native, state, BROTLI_STATE_COMMAND_POST_WRITE_2);
                            label = Label::Save;
                            break;
                        }
                    }
                    if !matches!(label, Label::Save) {
                        if read!(native, meta_block_remaining_len) <= 0 {
                            write!(native, state, BROTLI_STATE_METABLOCK_DONE);
                            label = Label::Save;
                        } else {
                            label = Label::Begin;
                        }
                    }
                }
                Label::NextLiteral => {
                    status!(unsafe {
                        if safe != 0 {
                            self.safe_decode_literal_block_switch(input)
                        } else {
                            self.decode_literal_block_switch(input)
                        }
                    });
                    label = Label::Inner;
                }
                Label::Save => {
                    write!(native, pos, pos);
                    write!(native, loop_counter, i);
                    return result;
                }
            }
        }
    }
    /// # Safety
    /// Same source history/initialized-domain contract as the original fast kernel.
    #[inline(never)]
    pub(crate) unsafe fn process_commands(
        &mut self,
        input: SourceInput<'_>,
        bindings: &CompoundDictionaryBindings<'a>,
    ) -> i32 {
        unsafe { self.process_commands_internal(0, input, bindings) }
    }
    /// # Safety
    /// Same source history/initialized-domain contract as the original safe kernel.
    #[inline(never)]
    pub(crate) unsafe fn safe_process_commands(
        &mut self,
        input: SourceInput<'_>,
        bindings: &CompoundDictionaryBindings<'a>,
    ) -> i32 {
        unsafe { self.process_commands_internal(1, input, bindings) }
    }
}
