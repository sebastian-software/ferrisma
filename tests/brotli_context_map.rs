// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
#[path = "../src/brotli/bitfield.rs"]
pub mod bitfield;
#[path = "../src/brotli/constants.rs"]
pub mod constants;
#[path = "../src/brotli/context_map.rs"]
pub mod context_map;
#[path = "../src/brotli/huffman.rs"]
pub mod huffman;
#[path = "../src/brotli/huffman_reader.rs"]
pub mod huffman_reader;
#[path = "../src/brotli/layout.rs"]
pub mod layout;
#[path = "../src/brotli/memory.rs"]
pub mod memory;
#[path = "../src/brotli/metablock.rs"]
pub mod metablock;
#[path = "../src/brotli/mtf.rs"]
pub mod mtf;
#[path = "../src/brotli/running.rs"]
pub mod running;
#[path = "../src/brotli/status.rs"]
pub mod status;
#[path = "../src/brotli/symbols.rs"]
pub mod symbols;

use bit_reader::{BitReader, Reg};
use context_map::*;
use layout::{DecoderStateStorage, HeaderArenaStorage, HuffmanCodeStorage};
use memory::{Allocation, Allocator};
use running::*;
use status::*;
use std::ffi::c_void;
use std::mem::{MaybeUninit, size_of};

struct Context {
    seed: Option<u8>,
    blocks: Vec<(*mut c_void, usize)>,
    state: *mut DecoderStateStorage,
    active_region: bool,
    recording: bool,
    fail_map: bool,
    callbacks: Vec<Vec<u8>>,
    which: bool,
    input_base: usize,
}
impl Default for Context {
    fn default() -> Self {
        Self {
            seed: None,
            blocks: Vec::new(),
            state: std::ptr::null_mut(),
            active_region: false,
            recording: false,
            fail_map: false,
            callbacks: Vec::new(),
            which: false,
            input_base: 0,
        }
    }
}
unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: stable matching callback context, no active region exists during
    // a recorded source allocation; payload observations are reached scalars.
    let context = unsafe { &mut *opaque.cast::<Context>() };
    if context.recording {
        assert!(!context.active_region);
        let mut record = Vec::new();
        // SAFETY: callback follows successful variable-byte decode and region
        // flush, which initialized precisely these observed source fields.
        unsafe {
            let s = &*context.state;
            let h = &*s.arena.as_ptr().cast::<HeaderArenaStorage>();
            for value in [
                size as u64,
                if context.which {
                    s.num_dist_htrees.assume_init()
                } else {
                    s.num_literal_htrees.assume_init()
                },
                if context.which {
                    s.dist_context_map.assume_init()
                } else {
                    s.context_map.assume_init()
                } as u64,
                h.context_index.assume_init(),
                u64::from(h.substate_context_map.assume_init()),
                u64::from(s.substate_decode_uint8.assume_init()),
                s.br.assume_init_ref().val.assume_init(),
                s.br.assume_init_ref().bit_pos.assume_init(),
            ] {
                record.extend_from_slice(&value.to_le_bytes());
            }
        }
        #[cfg(feature = "cpp-oracle")]
        if context.seed.is_some() {
            // SAFETY: paired-seed fixtures physically initialized all observed
            // nonpadding fields. No seed comparison establishes typed validity.
            unsafe {
                parity::callback_tail(context, &mut record);
            }
        }
        context.callbacks.push(record);
        if context.fail_map {
            return std::ptr::null_mut();
        }
    }
    let layout = std::alloc::Layout::from_size_align(size.max(1), 16).unwrap();
    // SAFETY: valid nonzero physical reservation; logical extent remains size.
    let p = unsafe { std::alloc::alloc(layout) };
    if !p.is_null() {
        if let Some(seed) = context.seed {
            // SAFETY: fixture initializes only the actual allocation extent.
            unsafe { p.write_bytes(seed, size) };
        }
        context.blocks.push((p.cast(), size));
    }
    p.cast()
}
unsafe extern "C" fn free(opaque: *mut c_void, p: *mut c_void) {
    if p.is_null() {
        return;
    }
    // SAFETY: stable context; all region/output loans ended before owner drop.
    let c = unsafe { &mut *opaque.cast::<Context>() };
    assert!(!c.active_region);
    let i = c.blocks.iter().position(|&(q, _)| q == p).unwrap();
    let (_, n) = c.blocks.remove(i);
    // SAFETY: original matching allocation and exact physical layout, once.
    unsafe {
        std::alloc::dealloc(
            p.cast(),
            std::alloc::Layout::from_size_align(n.max(1), 16).unwrap(),
        )
    };
}
fn allocator(c: &mut Context) -> Allocator {
    // SAFETY: context remains stable until every actual owner is dropped.
    unsafe { Allocator::from_callbacks(Some(allocate), Some(free), std::ptr::from_mut(c).cast()) }
        .unwrap()
}
struct Owner<'a> {
    state: Allocation,
    map: Option<Allocation>,
    context: &'a mut Context,
    input: Vec<u8>,
    chunk_start: usize,
    chunk_end: usize,
    consumed: usize,
}
impl Drop for Owner<'_> {
    fn drop(&mut self) {
        drop(self.map.take());
    }
}
impl<'a> Owner<'a> {
    fn new(c: &'a mut Context, input: Vec<u8>) -> Self {
        let mut state = allocator(c)
            .allocate(size_of::<DecoderStateStorage>())
            .unwrap();
        let p = state
            .storage_mut()
            .as_mut_ptr()
            .cast::<DecoderStateStorage>();
        c.state = p;
        // SAFETY: native state storage has only MaybeUninit fields. The reader
        // Init fields and native old-map NULL are source initialization only.
        unsafe {
            let s = &mut *p;
            let br = &mut *s.br.as_mut_ptr();
            br.val.write(0);
            br.bit_pos.write(0);
            s.context_map.write(0);
            s.dist_context_map.write(0);
        }
        let size = input.len();
        c.input_base = input.as_ptr() as usize;
        let mut out = Self {
            state,
            map: None,
            context: c,
            input,
            chunk_start: 0,
            chunk_end: size,
            consumed: 0,
        };
        out.set_chunk(0, size);
        out
    }
    fn state(&mut self) -> &mut DecoderStateStorage {
        // SAFETY: exact native aligned actual callback allocation, exclusively
        // borrowed; only MaybeUninit storage fields are exposed.
        unsafe {
            &mut *self
                .state
                .storage_mut()
                .as_mut_ptr()
                .cast::<DecoderStateStorage>()
        }
    }
    fn header(&mut self) -> &mut HeaderArenaStorage {
        // SAFETY: actual header interpretation is selected for this fixture;
        // no union payload is read by forming its MaybeUninit field view.
        unsafe { &mut *self.state().arena.as_mut_ptr().cast::<HeaderArenaStorage>() }
    }
    fn set_chunk(&mut self, start: usize, end: usize) {
        assert!(start <= end && end <= self.input.len());
        self.chunk_start = start;
        self.chunk_end = end;
        self.consumed = 0;
        let base = self.input.as_ptr().wrapping_add(start) as usize;
        let len = end - start;
        // SAFETY: SetInput initializes only native pointer slots, retaining
        // actual reader accumulators written by Init or the preceding region.
        unsafe {
            let b = &mut *self.state().br.as_mut_ptr();
            b.next_in.write(base);
            b.last_in.write(base + len);
            b.guard_in.write(base + len.saturating_sub(27));
        }
    }
    fn checkpoint(&mut self) {
        let h = self.header();
        h.sub_loop_counter.write(0);
        h.symbol_lists.write(
            h.symbols_lists_array
                .as_ptr()
                .cast::<u16>()
                .wrapping_add(16) as usize,
        );
        h.substate_huffman.write(BROTLI_STATE_HUFFMAN_NONE);
        h.substate_tree_group.write(BROTLI_STATE_TREE_GROUP_NONE);
        h.substate_context_map.write(BROTLI_STATE_CONTEXT_MAP_NONE);
        self.state()
            .substate_decode_uint8
            .write(BROTLI_STATE_DECODE_UINT8_NONE);
        self.state().mtf_upper_bound.write(63);
    }
}
impl ContextMapOwner for Owner<'_> {
    fn with_fields<R>(
        &mut self,
        operation: impl FnOnce(
            ContextMapFields<'_>,
            &mut BitReader<'_>,
            Option<&mut [MaybeUninit<u8>]>,
        ) -> R,
    ) -> R {
        assert!(!self.context.active_region);
        assert_eq!(self.context.input_base, self.input.as_ptr() as usize);
        self.context.active_region = true;
        let mut br = BitReader::new();
        br.set_input(&self.input[self.chunk_start..self.chunk_end]);
        // Reconstruct the ephemeral source view using only position metadata;
        // drop_bytes reads no payload and retains original chunk guard identity.
        br.drop_bytes(self.consumed);
        // SAFETY: actual Init/preceding region initialized accumulators; raw
        // projection keeps native br disjoint from all borrowed actual fields.
        let raw = self
            .state
            .storage_mut()
            .as_mut_ptr()
            .cast::<DecoderStateStorage>();
        unsafe {
            br.val = (*(*raw).br.as_ptr()).val.assume_init_read();
            br.bit_pos = (*(*raw).br.as_ptr()).bit_pos.assume_init_read();
        }
        let result = {
            // SAFETY: all projections are distinct actual fields; no complete
            // state value is read. Eighteen header loans remain separately split.
            let fields = unsafe {
                ContextMapFields {
                    header: split_context_map(
                        &mut *(*raw).arena.as_mut_ptr().cast::<HeaderArenaStorage>(),
                    ),
                    num_htrees: if self.context.which {
                        &mut (*raw).num_dist_htrees
                    } else {
                        &mut (*raw).num_literal_htrees
                    },
                    map_slot: if self.context.which {
                        &mut (*raw).dist_context_map
                    } else {
                        &mut (*raw).context_map
                    },
                    uint8_substate: &mut (*raw).substate_decode_uint8,
                    mtf_upper_bound: &mut (*raw).mtf_upper_bound,
                    mtf: &mut (*raw).mtf,
                }
            };
            operation(
                fields,
                &mut br,
                self.map.as_mut().map(Allocation::storage_mut),
            )
        };
        // All source field/map loans ended; flush only real native reader
        // fields while its borrowed input is still valid, then drop that view.
        self.consumed = br.position();
        let base = self.input.as_ptr().wrapping_add(self.chunk_start) as usize;
        // SAFETY: native br is a separate actual field and no field loan remains.
        unsafe {
            let b = &mut *(*raw).br.as_mut_ptr();
            b.val.write(br.val);
            b.bit_pos.write(br.bit_pos);
            b.next_in.write(base + br.position());
            b.guard_in.write(base + br.guard_position());
            b.last_in.write(base + br.position() + br.available_in());
        }
        self.context.active_region = false;
        result
    }
    fn allocate_map(&mut self, size: usize) -> bool {
        assert!(!self.context.active_region);
        assert!(self.map.is_none());
        self.context.recording = true;
        let result = allocator(self.context).allocate(size);
        self.context.recording = false;
        let token = result.as_ref().map_or(0, |m| m.as_ptr() as usize);
        self.map = result;
        if self.context.which {
            self.state().dist_context_map.write(token);
        } else {
            self.state().context_map.write(token);
        }
        token != 0
    }
}
fn call(o: &mut Owner<'_>, size: Reg) -> BrotliDecoderErrorCode {
    // SAFETY: each native fixture proves reached fields and output prefix;
    // corpus only resumes actual original returned NEEDS_MORE_INPUT checkpoints.
    unsafe { decode_context_map(size, o) }
}

#[test]
fn unknown_default_state_reads_no_other_owner_fields() {
    let mut c = Context::default();
    let mut o = Owner::new(&mut c, vec![]);
    o.header().substate_context_map.write(u32::MAX);
    assert_eq!(call(&mut o, 8), BROTLI_DECODER_ERROR_UNREACHABLE);
    assert!(o.context.callbacks.is_empty());
}
#[test]
fn none_empty_input_keeps_unknown_count_and_output() {
    let mut c = Context::default();
    let mut o = Owner::new(&mut c, vec![]);
    o.header()
        .substate_context_map
        .write(BROTLI_STATE_CONTEXT_MAP_NONE);
    o.state()
        .substate_decode_uint8
        .write(BROTLI_STATE_DECODE_UINT8_NONE);
    assert_eq!(call(&mut o, 8), BROTLI_DECODER_NEEDS_MORE_INPUT);
    assert!(o.map.is_none() && o.context.callbacks.is_empty());
}
#[test]
fn failed_allocation_observes_flushed_actual_fields_and_old_slot() {
    let mut c = Context {
        fail_map: true,
        ..Context::default()
    };
    let mut o = Owner::new(&mut c, vec![0]);
    o.checkpoint();
    assert_eq!(call(&mut o, 4), BROTLI_DECODER_ERROR_ALLOC_CONTEXT_MAP);
    assert_eq!(o.context.callbacks.len(), 1);
    let expected = [
        4u64,
        1,
        0,
        0,
        BROTLI_STATE_CONTEXT_MAP_NONE as u64,
        BROTLI_STATE_DECODE_UINT8_NONE as u64,
        0,
        7,
    ];
    assert_eq!(
        o.context.callbacks[0],
        expected
            .into_iter()
            .flat_map(u64::to_le_bytes)
            .collect::<Vec<_>>()
    );
    assert!(o.map.is_none());
    unsafe {
        assert_eq!(o.state().context_map.assume_init(), 0);
    }
}
#[test]
fn trivial_map_initializes_raw_actual_output_without_table_or_mtf_reads() {
    let mut c = Context::default();
    let mut o = Owner::new(&mut c, vec![0]);
    o.checkpoint();
    assert_eq!(call(&mut o, 4), BROTLI_DECODER_SUCCESS);
    unsafe {
        assert_eq!(
            o.map.as_ref().unwrap().initialized_bytes(0..4),
            [0, 0, 0, 0]
        );
    }
    assert_eq!(o.context.callbacks.len(), 1);
}
#[test]
fn symbol_shortage_reads_only_reached_bits_field_and_retains_absent_map() {
    let mut c = Context::default();
    let mut o = Owner::new(&mut c, vec![]);
    let h = o.header();
    h.substate_context_map
        .write(BROTLI_STATE_CONTEXT_MAP_DECODE);
    h.context_index.write(0);
    h.max_run_length_prefix.write(0);
    h.code.write(0xffff);
    unsafe {
        (*h.context_map_table
            .as_mut_ptr()
            .cast::<HuffmanCodeStorage>())
        .bits
        .write(1);
    };
    assert_eq!(call(&mut o, 1), BROTLI_DECODER_NEEDS_MORE_INPUT);
    unsafe {
        assert_eq!(o.header().code.assume_init(), 0xffff);
        assert_eq!(o.header().context_index.assume_init(), 0);
    }
}
#[test]
fn retained_repeat_shortage_never_reads_table_or_map() {
    let mut c = Context::default();
    let mut o = Owner::new(&mut c, vec![]);
    let h = o.header();
    h.substate_context_map
        .write(BROTLI_STATE_CONTEXT_MAP_DECODE);
    h.context_index.write(0);
    h.max_run_length_prefix.write(3);
    h.code.write(3);
    assert_eq!(call(&mut o, 1), BROTLI_DECODER_NEEDS_MORE_INPUT);
    unsafe {
        assert_eq!(o.header().code.assume_init(), 3);
    }
}
#[test]
fn repeat_overflow_error_leaves_output_and_global_partial_fields_untouched() {
    let mut c = Context::default();
    let mut o = Owner::new(&mut c, vec![0]);
    let h = o.header();
    h.substate_context_map
        .write(BROTLI_STATE_CONTEXT_MAP_DECODE);
    h.context_index.write(0);
    h.max_run_length_prefix.write(1);
    h.code.write(1);
    assert_eq!(
        call(&mut o, 1),
        BROTLI_DECODER_ERROR_FORMAT_CONTEXT_MAP_REPEAT
    );
    unsafe {
        assert_eq!(o.header().context_index.assume_init(), 0);
        assert_eq!(o.header().code.assume_init(), 1);
    }
}
#[test]
fn transform_zero_does_not_read_unknown_map_slot_or_mtf() {
    let mut c = Context::default();
    let mut o = Owner::new(&mut c, vec![0]);
    o.header()
        .substate_context_map
        .write(BROTLI_STATE_CONTEXT_MAP_TRANSFORM);
    o.state().context_map = MaybeUninit::uninit();
    assert_eq!(call(&mut o, 8), BROTLI_DECODER_SUCCESS);
}
#[test]
fn empty_transform_initializes_only_native_mtf_word_prefix() {
    let mut c = Context::default();
    let mut o = Owner::new(&mut c, vec![1]);
    o.header()
        .substate_context_map
        .write(BROTLI_STATE_CONTEXT_MAP_TRANSFORM);
    o.state().mtf_upper_bound.write(0);
    assert_eq!(call(&mut o, 0), BROTLI_DECODER_SUCCESS);
    unsafe {
        let p = o.state().mtf.as_ptr().cast::<u32>();
        assert_eq!(p.add(1).read(), u32::from_ne_bytes([0, 1, 2, 3]));
        assert_eq!(p.add(2).read(), u32::from_ne_bytes([4, 5, 6, 7]));
    }
}
#[test]
fn transform_uses_complete_initialized_map_with_unknown_stale_index() {
    let mut c = Context::default();
    let mut o = Owner::new(&mut c, vec![1]);
    o.map = Some(allocator(o.context).allocate(4).unwrap());
    for (i, b) in o.map.as_mut().unwrap().storage_mut().iter_mut().enumerate() {
        b.write(i as u8);
    }
    let token = o.map.as_ref().unwrap().as_ptr() as usize;
    o.state().context_map.write(token);
    o.header()
        .substate_context_map
        .write(BROTLI_STATE_CONTEXT_MAP_TRANSFORM);
    o.state().mtf_upper_bound.write(63);
    assert_eq!(call(&mut o, 4), BROTLI_DECODER_SUCCESS);
    unsafe {
        assert_eq!(
            o.map.as_ref().unwrap().initialized_bytes(0..4),
            [0, 1, 2, 3]
        );
    }
}

#[cfg(feature = "cpp-oracle")]
mod parity {
    use super::*;
    use std::io::{BufReader, BufWriter, Read, Write};
    use std::mem::offset_of;
    use std::process::{Command, Stdio};
    fn fields() -> Vec<(usize, usize)> {
        let mut out = Vec::new();
        macro_rules! field {
            ($name:ident,$ty:ty) => {
                out.push((offset_of!(HeaderArenaStorage, $name), size_of::<$ty>()));
            };
        }
        field!(substate_tree_group, u32);
        field!(substate_context_map, u32);
        field!(substate_huffman, u32);
        field!(sub_loop_counter, Reg);
        field!(repeat_code_len, Reg);
        field!(prev_code_len, Reg);
        field!(symbol, Reg);
        field!(repeat, Reg);
        field!(space, Reg);
        for i in 0..32 {
            out.push((
                offset_of!(HeaderArenaStorage, table)
                    + i * size_of::<HuffmanCodeStorage>()
                    + offset_of!(HuffmanCodeStorage, bits),
                1,
            ));
            out.push((
                offset_of!(HeaderArenaStorage, table)
                    + i * size_of::<HuffmanCodeStorage>()
                    + offset_of!(HuffmanCodeStorage, value),
                2,
            ));
        }
        field!(symbol_lists, usize);
        field!(symbols_lists_array, [u16; 720]);
        field!(next_symbol, [i32; 32]);
        field!(code_length_code_lengths, [u8; 18]);
        field!(code_length_histo, [u16; 16]);
        field!(htree_index, i32);
        field!(next, usize);
        field!(context_index, Reg);
        field!(max_run_length_prefix, Reg);
        field!(code, Reg);
        for i in 0..646 {
            out.push((
                offset_of!(HeaderArenaStorage, context_map_table)
                    + i * size_of::<HuffmanCodeStorage>()
                    + offset_of!(HuffmanCodeStorage, bits),
                1,
            ));
            out.push((
                offset_of!(HeaderArenaStorage, context_map_table)
                    + i * size_of::<HuffmanCodeStorage>()
                    + offset_of!(HuffmanCodeStorage, value),
                2,
            ));
        }
        out
    }
    unsafe fn header_bytes(h: &HeaderArenaStorage) -> Vec<u8> {
        let mut out = Vec::with_capacity(3756);
        let base = std::ptr::from_ref(h).cast::<u8>();
        for (offset, size) in fields() {
            if offset == offset_of!(HeaderArenaStorage, symbol_lists) {
                // SAFETY: actual source checkpoint initialized only this native token.
                let token = unsafe { h.symbol_lists.assume_init_read() };
                assert_eq!(
                    token,
                    h.symbols_lists_array
                        .as_ptr()
                        .cast::<u16>()
                        .wrapping_add(16) as usize
                );
                out.extend_from_slice(&16u64.to_le_bytes());
            } else {
                // SAFETY: physical fixture seeds initialize all observed field bytes; exact
                // schema excludes source padding. No mask infers logical initialization.
                out.extend_from_slice(unsafe {
                    std::slice::from_raw_parts(base.add(offset), size)
                });
            }
        }
        out
    }
    pub(super) unsafe fn callback_tail(c: &Context, out: &mut Vec<u8>) {
        // SAFETY: recorded callbacks have no active source field/reader/input/map loan;
        // seeded bytes are independent direct observations, not initialization proof.
        unsafe {
            let s = &*c.state;
            let b = &*s.br.as_ptr();
            for value in [
                b.next_in.assume_init_read() - c.input_base,
                b.guard_in.assume_init_read() - c.input_base,
                b.last_in.assume_init_read() - c.input_base,
                usize::from(c.fail_map),
            ] {
                out.extend_from_slice(&(value as u64).to_le_bytes());
            }
            out.extend(header_bytes(
                &*s.arena.as_ptr().cast::<HeaderArenaStorage>(),
            ));
            out.extend_from_slice(&s.mtf_upper_bound.assume_init_read().to_le_bytes());
            out.extend_from_slice(std::slice::from_raw_parts(s.mtf.as_ptr().cast::<u8>(), 260));
        }
    }
    struct Case {
        size: usize,
        which: bool,
        mode: u32,
        num: Reg,
        prefix: Reg,
        code: Reg,
        index: Reg,
        bound: Reg,
        symbol: u16,
        initial_bits: Reg,
        initial_val: Reg,
        fail: bool,
        chunks: Vec<usize>,
        data: Vec<u8>,
    }
    fn parse(line: &str) -> Case {
        let mut words = line.split_whitespace();
        let mut n = || words.next().unwrap().parse::<u64>().unwrap();
        let size = n() as usize;
        let which = n() != 0;
        let mode = n() as u32;
        let num = n();
        let prefix = n();
        let code = n();
        let index = n();
        let bound = n();
        let _uint8_state = n();
        let symbol = n() as u16;
        let initial_bits = n();
        let initial_val = n();
        let fail = n() != 0;
        let steps = n();
        let bytes = n() as usize;
        let chunks = (0..steps).map(|_| n() as usize).collect();
        let hex = words.next().unwrap_or("");
        assert_eq!(hex.len(), 2 * bytes);
        let data = (0..bytes)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
            .collect();
        assert!(words.next().is_none());
        Case {
            size,
            which,
            mode,
            num,
            prefix,
            code,
            index,
            bound,
            symbol,
            initial_bits,
            initial_val,
            fail,
            chunks,
            data,
        }
    }
    fn observe(case: &Case, seed: u8, sink: &mut impl FnMut(&[u8])) -> (usize, usize) {
        let mut c = Context {
            seed: Some(seed),
            which: case.which,
            fail_map: case.fail,
            ..Context::default()
        };
        let mut o = Owner::new(&mut c, case.data.clone());
        o.checkpoint();
        let mut known = 0;
        if matches!(case.mode, 1..=5 | 7) {
            o.map = Some(allocator(o.context).allocate(case.size).unwrap());
            let token = o.map.as_ref().unwrap().as_ptr() as usize;
            if case.which {
                o.state().num_dist_htrees.write(case.num);
                o.state().dist_context_map.write(token);
            } else {
                o.state().num_literal_htrees.write(case.num);
                o.state().context_map.write(token);
            }
            o.header().context_index.write(case.index);
            known = if case.mode == 5 {
                case.size
            } else {
                case.index as usize
            };
            for (i, b) in o.map.as_mut().unwrap().storage_mut()[..known]
                .iter_mut()
                .enumerate()
            {
                b.write(i as u8);
            }
        }
        match case.mode {
            1 => {
                o.header()
                    .substate_context_map
                    .write(BROTLI_STATE_CONTEXT_MAP_READ_PREFIX);
            }
            2 => {
                o.header()
                    .substate_context_map
                    .write(BROTLI_STATE_CONTEXT_MAP_HUFFMAN);
                o.header().max_run_length_prefix.write(case.prefix);
            }
            3 | 4 | 7 => {
                let h = o.header();
                h.substate_context_map
                    .write(BROTLI_STATE_CONTEXT_MAP_DECODE);
                h.max_run_length_prefix.write(case.prefix);
                h.code
                    .write(if case.mode == 4 { case.code } else { 0xffff });
                let mut values = if case.mode == 3 {
                    [case.symbol, 0]
                } else {
                    [0, case.symbol]
                };
                huffman::brotli_build_simple_huffman_table(
                    huffman_reader::context_map_output(&mut h.context_map_table),
                    8,
                    &mut values,
                    if case.mode == 7 { 1 } else { 0 },
                );
            }
            5 => {
                o.header()
                    .substate_context_map
                    .write(BROTLI_STATE_CONTEXT_MAP_TRANSFORM);
                // SAFETY: source empty transform initializes the entire identity tail in
                // actual native MTF storage before explicit retained-bound resume cases.
                unsafe {
                    let s = o.state();
                    mtf::inverse_move_to_front_transform(
                        &mut [],
                        0,
                        &mut s.mtf_upper_bound,
                        &mut s.mtf,
                    );
                }
                o.state().mtf_upper_bound.write(case.bound);
            }
            6 => {
                o.header().substate_context_map.write(u32::MAX);
            }
            8 => {
                o.state()
                    .substate_decode_uint8
                    .write(BROTLI_STATE_DECODE_UINT8_LONG);
                if case.which {
                    o.state().num_dist_htrees.write(case.num);
                } else {
                    o.state().num_literal_htrees.write(case.num);
                }
            }
            9 => {
                o.state().substate_decode_uint8.write(u32::MAX);
            }
            _ => {}
        }
        // SAFETY: initialized actual native reader Init fields, no region active.
        unsafe {
            let b = &mut *o.state().br.as_mut_ptr();
            b.val.write(case.initial_val);
            b.bit_pos.write(case.initial_bits);
        }
        let mut off = 0;
        let mut records = 0;
        let mut bytes = 0;
        for &chunk in &case.chunks {
            o.set_chunk(off, off + chunk);
            let result = call(&mut o, case.size as Reg);
            // SAFETY: every entry supplies discriminator; only partial DECODE return
            // reads the saved index written at the actual source shortage checkpoint.
            let state = unsafe { o.header().substate_context_map.assume_init_read() };
            if o.map.is_some() {
                if result == BROTLI_DECODER_SUCCESS
                    || result == BROTLI_DECODER_NEEDS_MORE_INPUT
                        && state == BROTLI_STATE_CONTEXT_MAP_TRANSFORM
                {
                    known = case.size;
                } else if result == BROTLI_DECODER_NEEDS_MORE_INPUT
                    && state == BROTLI_STATE_CONTEXT_MAP_DECODE
                {
                    known =
                        known.max(unsafe { o.header().context_index.assume_init_read() } as usize);
                }
            }
            let mut record = Vec::new();
            // SAFETY: all bytes are fixture-seeded or written, schema excludes padding.
            // Map pointer normalization uses actual owner identity, never a guessed token.
            unsafe {
                let s = &*o.state.as_ptr().cast::<DecoderStateStorage>();
                let b = &*s.br.as_ptr();
                for value in [
                    result as i64 as u64,
                    b.val.assume_init_read(),
                    b.bit_pos.assume_init_read(),
                    (b.next_in.assume_init_read() - o.context.input_base) as u64,
                    (b.guard_in.assume_init_read() - o.context.input_base) as u64,
                    (b.last_in.assume_init_read() - b.next_in.assume_init_read()) as u64,
                ] {
                    record.extend_from_slice(&value.to_le_bytes());
                }
                let num = if case.which {
                    &s.num_dist_htrees
                } else {
                    &s.num_literal_htrees
                };
                record.extend_from_slice(std::slice::from_raw_parts(num.as_ptr().cast::<u8>(), 8));
                let slot = if case.which {
                    &s.dist_context_map
                } else {
                    &s.context_map
                };
                let token = slot.assume_init_read();
                assert_eq!(token, o.map.as_ref().map_or(0, |m| m.as_ptr() as usize));
                let trace = o.context.callbacks.first().map_or(&[][..], Vec::as_slice);
                for value in [
                    u64::from(token != 0),
                    o.map.as_ref().map_or(0, Allocation::len) as u64,
                    known as u64,
                    o.context.callbacks.len() as u64,
                    trace.len() as u64,
                ] {
                    record.extend_from_slice(&value.to_le_bytes());
                }
                record.extend_from_slice(trace);
                record.extend(header_bytes(
                    &*s.arena.as_ptr().cast::<HeaderArenaStorage>(),
                ));
                record.extend_from_slice(std::slice::from_raw_parts(
                    s.substate_decode_uint8.as_ptr().cast::<u8>(),
                    4,
                ));
                record.extend_from_slice(&s.mtf_upper_bound.assume_init_read().to_le_bytes());
                record.extend_from_slice(std::slice::from_raw_parts(
                    s.mtf.as_ptr().cast::<u8>(),
                    260,
                ));
                if let Some(m) = &o.map {
                    record.extend_from_slice(m.initialized_bytes(0..m.len()));
                }
            }
            bytes += record.len();
            records += 1;
            sink(&record);
            if result != BROTLI_DECODER_NEEDS_MORE_INPUT {
                break;
            }
            assert_eq!(o.consumed, chunk);
            off += chunk;
        }
        (records, bytes)
    }
    fn word(bytes: &[u8], pos: &mut usize) -> u64 {
        let out = u64::from_le_bytes(bytes[*pos..*pos + 8].try_into().unwrap());
        *pos += 8;
        out
    }
    #[test]
    fn genuine_source_fixture_maps_are_decoded_exactly() {
        let oracle = std::env::var_os("BROTLI_CONTEXT_MAP_ORACLE").unwrap();
        let p = Command::new(oracle).arg("fixtures").output().unwrap();
        assert!(p.status.success() && p.stderr.is_empty());
        let mut at = 0;
        assert_eq!(word(&p.stdout, &mut at), 265);
        for _ in 0..265 {
            let size = word(&p.stdout, &mut at) as usize;
            let clusters = word(&p.stdout, &mut at);
            let bits = word(&p.stdout, &mut at) as usize;
            let len = word(&p.stdout, &mut at) as usize;
            assert_eq!(len, bits.div_ceil(8));
            let data = p.stdout[at..at + len].to_vec();
            at += len;
            let expected = &p.stdout[at..at + size];
            at += size;
            let mut c = Context::default();
            let mut o = Owner::new(&mut c, data);
            o.checkpoint();
            assert_eq!(call(&mut o, size as Reg), BROTLI_DECODER_SUCCESS);
            // SAFETY: source-success initializes complete map/count, independently of
            // stale context_index or any seed-matching heuristic.
            unsafe {
                assert_eq!(o.map.as_ref().unwrap().initialized_bytes(0..size), expected);
                assert_eq!(o.state().num_literal_htrees.assume_init_read(), clusters);
            }
        }
        assert_eq!(at, p.stdout.len());
    }
    #[test]
    fn original_context_map_fields_callbacks_maps_and_partial_states_match() {
        let oracle = std::env::var_os("BROTLI_CONTEXT_MAP_ORACLE").unwrap();
        let dir = std::path::PathBuf::from(std::env::var_os("BROTLI_CONTEXT_MAP_CAPTURE").unwrap());
        std::fs::create_dir_all(&dir).unwrap();
        let schema = Command::new(&oracle).arg("schema").output().unwrap();
        assert!(schema.status.success() && schema.stderr.is_empty());
        let mut at = 0;
        for v in [size_of::<HeaderArenaStorage>(), 1375, 3756] {
            assert_eq!(word(&schema.stdout, &mut at), v as u64);
        }
        let spans = fields();
        assert_eq!(spans.len(), 1375);
        assert_eq!(spans.iter().map(|(_, s)| s).sum::<usize>(), 3756);
        for (o, s) in spans {
            assert_eq!(word(&schema.stdout, &mut at), o as u64);
            assert_eq!(word(&schema.stdout, &mut at), s as u64);
        }
        for v in [
            size_of::<DecoderStateStorage>(),
            offset_of!(DecoderStateStorage, num_literal_htrees),
            offset_of!(DecoderStateStorage, context_map),
            offset_of!(DecoderStateStorage, num_dist_htrees),
            offset_of!(DecoderStateStorage, dist_context_map),
            offset_of!(DecoderStateStorage, mtf_upper_bound),
            offset_of!(DecoderStateStorage, mtf),
            offset_of!(DecoderStateStorage, substate_decode_uint8),
        ] {
            assert_eq!(word(&schema.stdout, &mut at), v as u64);
        }
        assert_eq!(at, schema.stdout.len());
        std::fs::write(dir.join("schema.bin"), schema.stdout).unwrap();
        let input =
            std::fs::read_to_string(std::env::var_os("BROTLI_CONTEXT_MAP_REQUESTS").unwrap())
                .unwrap();
        std::fs::write(dir.join("requests.txt"), &input).unwrap();
        let mut rust = BufWriter::new(std::fs::File::create(dir.join("rust.bin")).unwrap());
        let (mut cases, mut records, mut bytes) = (0, 0, 0);
        for line in input.lines() {
            cases += 1;
            let case = parse(line);
            for seed in [165, 90] {
                let (n, b) = observe(&case, seed, &mut |record| rust.write_all(record).unwrap());
                records += n;
                bytes += b;
            }
        }
        rust.flush().unwrap();
        drop(rust);
        let source = Command::new(&oracle)
            .arg("run")
            .stdin(Stdio::from(
                std::fs::File::open(dir.join("requests.txt")).unwrap(),
            ))
            .stdout(Stdio::from(
                std::fs::File::create(dir.join("source.bin")).unwrap(),
            ))
            .output()
            .unwrap();
        assert!(
            source.status.success(),
            "{}",
            String::from_utf8_lossy(&source.stderr)
        );
        assert!(source.stderr.is_empty());
        let mut source = BufReader::new(std::fs::File::open(dir.join("source.bin")).unwrap());
        let mut rust = BufReader::new(std::fs::File::open(dir.join("rust.bin")).unwrap());
        let (mut a, mut b) = ([0u8; 65536], [0u8; 65536]);
        let mut offset = 0;
        loop {
            let n = source.read(&mut a).unwrap();
            let mut filled = 0;
            while filled < n {
                let m = rust.read(&mut b[filled..n]).unwrap();
                assert!(m > 0);
                filled += m;
            }
            assert_eq!(&a[..n], &b[..n], "observation offset{offset}");
            offset += n;
            if n == 0 {
                assert_eq!(rust.read(&mut b[..1]).unwrap(), 0);
                break;
            }
        }
        assert_eq!(offset, bytes);
        std::fs::write(
            dir.join("counts.txt"),
            format!("{cases} {records} {bytes}\n"),
        )
        .unwrap();
    }
}
