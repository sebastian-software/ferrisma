// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
#[path = "../src/brotli/constants.rs"]
pub mod constants;
#[path = "../src/brotli/huffman.rs"]
pub mod huffman;
#[path = "../src/brotli/huffman_reader.rs"]
pub mod huffman_reader;
#[path = "../src/brotli/layout.rs"]
pub mod layout;
#[path = "../src/brotli/memory.rs"]
pub mod memory;
#[path = "../src/brotli/running.rs"]
pub mod running;
#[path = "../src/brotli/status.rs"]
pub mod status;
#[path = "../src/brotli/tree_group.rs"]
pub mod tree_group;

use bit_reader::{BitReader, Reg};
use huffman::HuffmanCode;
use layout::{
    DecoderStateStorage, HeaderArenaStorage, HuffmanCodeStorage, HuffmanTreeGroupStorage,
};
use memory::{Allocation, Allocator};
use running::*;
use status::*;
use std::ffi::c_void;
use std::mem::{align_of, offset_of, size_of};
use tree_group::*;

#[derive(Default)]
struct Context {
    seed: Option<u8>,
    blocks: Vec<(*mut c_void, usize)>,
    frees: usize,
}
unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: context stays at this address through every matching owner drop;
    // callbacks never reenter or inspect borrowed actual storage.
    let context = unsafe { &mut *opaque.cast::<Context>() };
    let layout = std::alloc::Layout::from_size_align(size.max(1), 16).unwrap();
    // SAFETY: valid nonzero reservation; the exposed source extent stays size.
    let pointer = unsafe { std::alloc::alloc(layout) };
    if !pointer.is_null() {
        if let Some(seed) = context.seed {
            // SAFETY: actual fresh allocation spans at least the source size.
            unsafe { pointer.write_bytes(seed, size) };
        }
        context.blocks.push((pointer.cast(), size));
    }
    pointer.cast()
}
unsafe extern "C" fn free(opaque: *mut c_void, pointer: *mut c_void) {
    if pointer.is_null() {
        return;
    }
    // SAFETY: original stable context; every field/output loan has ended.
    let context = unsafe { &mut *opaque.cast::<Context>() };
    let index = context
        .blocks
        .iter()
        .position(|&(ptr, _)| ptr == pointer)
        .unwrap();
    let (_, size) = context.blocks.remove(index);
    let layout = std::alloc::Layout::from_size_align(size.max(1), 16).unwrap();
    // SAFETY: exact original request/layout, once, after exclusive owner drop.
    unsafe { std::alloc::dealloc(pointer.cast(), layout) };
    context.frees += 1;
}
fn allocation(context: &mut Context, size: usize) -> Allocation {
    // SAFETY: context remains stable/live until all returned owners drop;
    // matched callbacks provide exclusive source-compatible actual storage.
    unsafe {
        Allocator::from_callbacks(
            Some(allocate),
            Some(free),
            std::ptr::from_mut(context).cast(),
        )
    }
    .unwrap()
    .allocate(size)
    .unwrap()
}
fn state_fields(owner: &mut Allocation) -> (&mut HeaderArenaStorage, &mut HuffmanTreeGroupStorage) {
    assert_eq!(owner.len(), size_of::<DecoderStateStorage>());
    // SAFETY: verified aligned native state layout contains only MaybeUninit
    // fields. Arena and literal_hgroup are distinct disjoint source fields;
    // neither view reads unknown payload or retains a whole-state reference.
    unsafe {
        let state = &mut *owner
            .storage_mut()
            .as_mut_ptr()
            .cast::<DecoderStateStorage>();
        (
            &mut *state.arena.as_mut_ptr().cast::<HeaderArenaStorage>(),
            &mut *state.literal_hgroup.as_mut_ptr(),
        )
    }
}
fn checkpoint(h: &mut HeaderArenaStorage) {
    // Actual decode.c2614 source checkpoint fields only; source observer runs
    // that genuine checkpoint through original DecompressStream(empty input).
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
}
fn group_size(limit: u16, ntrees: usize) -> usize {
    ntrees * (size_of::<usize>() + (usize::from(limit) + 376) * size_of::<HuffmanCode>())
}
fn group_checkpoint(
    group: &mut HuffmanTreeGroupStorage,
    owner: &Allocation,
    max: u16,
    limit: u16,
    ntrees: usize,
) {
    // Five source GroupInit descriptor assignments; this fixture owns the
    // actual callback block separately. No allocation occurs in the helper.
    group.alphabet_size_max.write(max);
    group.alphabet_size_limit.write(limit);
    group.num_htrees.write(ntrees as u16);
    group.htrees.write(owner.as_ptr() as usize);
    group
        .codes
        .write(owner.as_ptr().wrapping_add(ntrees * size_of::<usize>()) as usize);
}
unsafe fn read(
    group: &HuffmanTreeGroupStorage,
    h: &mut HeaderArenaStorage,
    owner: &mut Allocation,
    ntrees: usize,
    br: &mut BitReader<'_>,
) -> BrotliDecoderErrorCode {
    let mut output = TreeGroupOutput::new(owner.storage_mut(), ntrees);
    let (mut fields, mut huffman) = split_tree_group(h);
    // SAFETY: individual tests/corpus prove original reached checkpoint and
    // actual allocation geometry/nonalias domains before invoking this adapter.
    unsafe { huffman_tree_group_decode(group, &mut output, &mut fields, &mut huffman, br) }
}

#[test]
fn loop_complete_does_not_read_unknown_next_descriptor_or_output_slots() {
    for (ntrees, index) in [(0, 0), (0, i32::MAX), (2, 2), (2, i32::MAX)] {
        let mut context = Context::default();
        let mut state = allocation(&mut context, size_of::<DecoderStateStorage>());
        let mut output = allocation(&mut context, group_size(16, ntrees));
        let (h, g) = state_fields(&mut state);
        h.substate_tree_group.write(BROTLI_STATE_TREE_GROUP_LOOP);
        h.htree_index.write(index);
        g.num_htrees.write(ntrees as u16);
        let mut br = BitReader::new();
        // SAFETY: LOOP's condition is false. Only state/index/num are initialized;
        // next, alphabet, codes/htrees tokens and every output slot remain unknown.
        assert_eq!(
            unsafe { read(g, h, &mut output, ntrees, &mut br) },
            BROTLI_DECODER_SUCCESS
        );
        // SAFETY: final source assignment initialized this discriminant.
        assert_eq!(
            unsafe { h.substate_tree_group.assume_init() },
            BROTLI_STATE_TREE_GROUP_NONE
        );
    }
}

#[test]
fn nonloop_zero_tree_retains_actual_zero_size_callback_address() {
    let mut context = Context::default();
    let mut state = allocation(&mut context, size_of::<DecoderStateStorage>());
    let mut output = allocation(&mut context, 0);
    let actual = output.as_ptr() as usize;
    let (h, g) = state_fields(&mut state);
    h.substate_tree_group.write(u32::MAX);
    g.codes.write(actual);
    g.num_htrees.write(0);
    let mut br = BitReader::new();
    // SAFETY: non-LOOP initializes next/index, then num0 skips every unknown
    // alphabet/table/output field. Actual zero-size allocation identity is live.
    assert_eq!(
        unsafe { read(g, h, &mut output, 0, &mut br) },
        BROTLI_DECODER_SUCCESS
    );
    // SAFETY: source entry initialized next/index before the zero-tree exit.
    unsafe {
        assert_eq!(h.next.assume_init(), actual);
        assert_eq!(h.htree_index.assume_init(), 0);
    }
    assert_ne!(actual, 0);
}

#[test]
fn header_error_does_not_read_unknown_pointer_table_or_publish_a_tree() {
    let mut context = Context::default();
    let mut state = allocation(&mut context, size_of::<DecoderStateStorage>());
    let mut output = allocation(&mut context, group_size(16, 1));
    let code = output.as_ptr().wrapping_add(size_of::<usize>()) as usize;
    let (h, g) = state_fields(&mut state);
    h.substate_tree_group.write(BROTLI_STATE_TREE_GROUP_NONE);
    h.substate_huffman.write(u32::MAX);
    g.codes.write(code);
    g.num_htrees.write(1);
    g.alphabet_size_max.write(16);
    g.alphabet_size_limit.write(16);
    let mut br = BitReader::new();
    // SAFETY: original Huffman default state returns UNREACHABLE before any
    // code write. group.htrees, symbol-list fields and all output stay unknown.
    assert_eq!(
        unsafe { read(g, h, &mut output, 1, &mut br) },
        BROTLI_DECODER_ERROR_UNREACHABLE
    );
    // SAFETY: non-LOOP initialization wrote these exact control fields.
    unsafe {
        assert_eq!(h.htree_index.assume_init(), 0);
        assert_eq!(h.next.assume_init(), code);
        assert_eq!(
            h.substate_tree_group.assume_init(),
            BROTLI_STATE_TREE_GROUP_LOOP
        );
    }
}

#[test]
fn successful_tree_pointers_are_written_only_after_each_complete_header() {
    let mut context = Context::default();
    let mut state = allocation(&mut context, size_of::<DecoderStateStorage>());
    let mut output = allocation(&mut context, group_size(16, 2));
    let pointer_base = output.as_ptr() as usize;
    let code_base = pointer_base + 2 * size_of::<usize>();
    let (h, g) = state_fields(&mut state);
    checkpoint(h);
    group_checkpoint(g, &output, 16, 16, 2);
    let mut br = BitReader::new();
    br.set_input(&[0x11]);
    // SAFETY: exact genuine encoder singleton header0 from the frozen source
    // header packet. First tree finishes; second stops at actual partial input.
    assert_eq!(
        unsafe { read(g, h, &mut output, 2, &mut br) },
        BROTLI_DECODER_NEEDS_MORE_INPUT
    );
    // SAFETY: successful first iteration initialized pointer slot0 and control
    // fields. Pointer slot1 and the unused code suffix remain unknown/unread.
    unsafe {
        assert_eq!(h.htree_index.assume_init(), 1);
        assert_eq!(h.next.assume_init(), code_base + 256 * 4);
        assert_eq!(
            output.initialized_bytes(0..size_of::<usize>()),
            &code_base.to_ne_bytes()
        );
    }
    br.set_input(&[0x11]);
    // SAFETY: retain exact source partial state and stable owner allocation;
    // only the second actual header is appended, no pointer/token reconstruction.
    assert_eq!(
        unsafe { read(g, h, &mut output, 2, &mut br) },
        BROTLI_DECODER_SUCCESS
    );
    // SAFETY: both successes initialized both pointer slots and first512 codes.
    unsafe {
        assert_eq!(h.htree_index.assume_init(), 2);
        assert_eq!(h.next.assume_init(), code_base + 512 * 4);
        assert_eq!(
            output.initialized_bytes(size_of::<usize>()..2 * size_of::<usize>()),
            &(code_base + 256 * 4).to_ne_bytes()
        );
        let codes = output
            .as_ptr()
            .add(2 * size_of::<usize>())
            .cast::<HuffmanCode>();
        for i in 0..512 {
            assert_eq!((*codes.add(i)).bits, 0);
            assert_eq!((*codes.add(i)).value, 1);
        }
    }
}

#[test]
fn failed_negative_index_iteration_does_not_eagerly_index_pointer_table() {
    let mut context = Context::default();
    let mut state = allocation(&mut context, size_of::<DecoderStateStorage>());
    let mut output = allocation(&mut context, group_size(16, 1));
    let (h, g) = state_fields(&mut state);
    h.substate_tree_group.write(BROTLI_STATE_TREE_GROUP_LOOP);
    h.htree_index.write(-1);
    h.next
        .write(output.as_ptr().wrapping_add(size_of::<usize>()) as usize);
    h.substate_huffman.write(u32::MAX);
    g.num_htrees.write(1);
    g.alphabet_size_max.write(16);
    g.alphabet_size_limit.write(16);
    let mut br = BitReader::new();
    // SAFETY: this defined malformed state returns header error before the
    // pointer index is used or incremented. Unknown group.codes/htrees stay unread.
    assert_eq!(
        unsafe { read(g, h, &mut output, 1, &mut br) },
        BROTLI_DECODER_ERROR_UNREACHABLE
    );
    // SAFETY: initialized signed index is retained on the immediate error return.
    assert_eq!(unsafe { h.htree_index.assume_init() }, -1);
}

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

#[test]
fn observer_field_ranges_exclude_only_exact_structural_padding_bytes() {
    assert_eq!(align_of::<HeaderArenaStorage>(), 8);
    let mut covered = vec![false; size_of::<HeaderArenaStorage>()];
    for (offset, size) in fields() {
        for byte in &mut covered[offset..offset + size] {
            assert!(!*byte);
            *byte = true;
        }
    }
    assert_eq!(covered.iter().filter(|&&b| b).count(), 3756);
    for i in 0..32 {
        assert!(!covered[offset_of!(HeaderArenaStorage, table) + i * 4 + 1]);
    }
    for i in 0..646 {
        assert!(!covered[offset_of!(HeaderArenaStorage, context_map_table) + i * 4 + 1]);
    }
}

#[cfg(feature = "cpp-oracle")]
mod parity {
    use super::*;
    use std::io::{BufReader, BufWriter, Read, Write};
    use std::process::{Command, Stdio};

    #[derive(Clone, Debug)]
    struct Fixture {
        max: u16,
        limit: u16,
        ntrees: usize,
        bits: usize,
        data: Vec<u8>,
    }
    #[derive(Clone, Debug)]
    struct Case {
        max: u16,
        limit: u16,
        ntrees: usize,
        mode: u32,
        initial_bits: Reg,
        initial_val: Reg,
        chunks: Vec<usize>,
        data: Vec<u8>,
    }
    fn word(bytes: &[u8], cursor: &mut usize) -> u64 {
        let value = u64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap());
        *cursor += 8;
        value
    }
    fn fixtures(oracle: &std::ffi::OsStr) -> (Vec<Fixture>, Vec<u8>) {
        let source = Command::new(oracle).arg("fixtures").output().unwrap();
        assert!(
            source.status.success(),
            "{}",
            String::from_utf8_lossy(&source.stderr)
        );
        assert!(source.stderr.is_empty());
        let mut cursor = 0;
        let count = word(&source.stdout, &mut cursor);
        assert_eq!(count, 133);
        let mut fixtures = Vec::new();
        for _ in 0..count {
            let max = word(&source.stdout, &mut cursor) as u16;
            let limit = word(&source.stdout, &mut cursor) as u16;
            let ntrees = word(&source.stdout, &mut cursor) as usize;
            let bits = word(&source.stdout, &mut cursor) as usize;
            let size = word(&source.stdout, &mut cursor) as usize;
            let data = source.stdout[cursor..cursor + size].to_vec();
            cursor += size;
            assert_eq!(size, bits.div_ceil(8));
            fixtures.push(Fixture {
                max,
                limit,
                ntrees,
                bits,
                data,
            });
        }
        assert_eq!(cursor, source.stdout.len());
        (fixtures, source.stdout)
    }
    fn cases(fixtures: &[Fixture]) -> Vec<Case> {
        let mut out = Vec::new();
        for fixture in fixtures {
            let base = Case {
                max: fixture.max,
                limit: fixture.limit,
                ntrees: fixture.ntrees,
                mode: 0,
                initial_bits: 0,
                initial_val: 0,
                chunks: vec![fixture.data.len()],
                data: fixture.data.clone(),
            };
            out.push(base.clone());
            // Genuine complete streams with every initial bit alignment; the
            // adapter only splits existing encoded bits, never invents a header.
            if fixture.ntrees > 0 {
                for initial_bits in 1..8 {
                    let mut data = Vec::new();
                    for i in 0..fixture.data.len() {
                        let pair = u16::from(fixture.data[i])
                            | (u16::from(*fixture.data.get(i + 1).unwrap_or(&0)) << 8);
                        data.push((pair >> initial_bits) as u8);
                    }
                    out.push(Case {
                        initial_bits,
                        initial_val: Reg::from(fixture.data[0]) & ((1 << initial_bits) - 1),
                        chunks: vec![data.len()],
                        data,
                        ..base.clone()
                    });
                }
            }
            let mut ends = vec![
                0,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                fixture.data.len() / 2,
                fixture.data.len().saturating_sub(1),
                fixture.data.len(),
            ];
            ends.retain(|&n| n <= fixture.data.len());
            ends.sort_unstable();
            ends.dedup();
            for end in ends {
                out.push(Case {
                    chunks: vec![end],
                    ..base.clone()
                });
            }
            // Source resumes from actual statuses on chunk boundaries. Larger
            // complex streams use bounded chunks to retain complete observations.
            for step in [1, 3, 7] {
                if fixture.data.len() > 128 && step == 1 {
                    continue;
                }
                let mut chunks = vec![0, 0];
                let mut left = fixture.data.len();
                while left > 0 {
                    let n = left.min(step);
                    chunks.push(n);
                    left -= n;
                }
                chunks.push(0);
                out.push(Case {
                    chunks,
                    ..base.clone()
                });
            }
            for mode in [1, 3, 6] {
                out.push(Case {
                    mode,
                    ..base.clone()
                });
            }
            if fixture.ntrees > 0 {
                out.push(Case {
                    mode: 2,
                    ..base.clone()
                });
                out.push(Case {
                    mode: 4,
                    ..base.clone()
                });
                for limit in [0, 1, base.limit.saturating_sub(1)] {
                    out.push(Case {
                        limit,
                        ..base.clone()
                    });
                }
                // Defined malformed/truncated source inputs, preserving all
                // code storage capacity required by the supplied logical limit.
                for bit in 0..fixture.bits.min(24) {
                    let mut data = base.data.clone();
                    data[bit / 8] ^= 1 << (bit % 8);
                    out.push(Case {
                        data,
                        ..base.clone()
                    });
                }
            } else {
                out.push(Case {
                    mode: 5,
                    ..base.clone()
                });
            }
        }
        // Early LOOP exits need none of the alphabet/next/code fields, even
        // with zero alphabets; all original callback allocation bytes retained.
        for ntrees in [0, 1, 2, 3, 8] {
            for mode in [1, 6] {
                out.push(Case {
                    max: 0,
                    limit: 0,
                    ntrees,
                    mode,
                    initial_bits: 0,
                    initial_val: 0,
                    chunks: vec![0],
                    data: vec![],
                });
            }
        }
        out
    }
    unsafe fn header_bytes(
        h: &HeaderArenaStorage,
        spans: &[(usize, usize)],
        next_known: bool,
        code_base: usize,
        capacity: usize,
    ) -> Vec<u8> {
        let mut out = Vec::with_capacity(3756);
        let base = std::ptr::from_ref(h).cast::<u8>();
        for &(offset, size) in spans {
            if offset == offset_of!(HeaderArenaStorage, symbol_lists) {
                // SAFETY: genuine source checkpoint initialized only this token.
                let token = unsafe { h.symbol_lists.assume_init_read() };
                assert_eq!(
                    token,
                    h.symbols_lists_array
                        .as_ptr()
                        .cast::<u16>()
                        .wrapping_add(16) as usize
                );
                out.extend_from_slice(&16u64.to_le_bytes());
            } else if offset == offset_of!(HeaderArenaStorage, next) && next_known {
                // SAFETY: source control flow, not seed equality, proves next
                // initialized. Normalize only the actual owner-relative token.
                let token = unsafe { h.next.assume_init_read() };
                let normalized = if token == 0 {
                    u64::MAX
                } else {
                    let delta = token.checked_sub(code_base).unwrap();
                    assert_eq!(delta % 4, 0);
                    assert!(delta / 4 <= capacity);
                    (delta / 4) as u64
                };
                out.extend_from_slice(&normalized.to_le_bytes());
            } else {
                // SAFETY: every field byte is physically fixture-seeded and
                // the independent schema excludes every structural padding byte.
                // No matching-byte heuristic establishes logical initialization.
                out.extend_from_slice(unsafe {
                    std::slice::from_raw_parts(base.add(offset), size)
                });
            }
        }
        out
    }
    fn observe(
        case: &Case,
        seed: u8,
        spans: &[(usize, usize)],
        sink: &mut impl FnMut(&[u8]),
    ) -> usize {
        let mut context = Context {
            seed: Some(seed),
            ..Context::default()
        };
        let mut state = allocation(&mut context, size_of::<DecoderStateStorage>());
        let mut output = allocation(&mut context, group_size(case.limit, case.ntrees));
        let allocation_size = output.len();
        let code_offset = case.ntrees * size_of::<usize>();
        let code_base = output.as_ptr().wrapping_add(code_offset) as usize;
        let capacity = (usize::from(case.limit) + 376) * case.ntrees;
        let (h, g) = state_fields(&mut state);
        checkpoint(h);
        group_checkpoint(g, &output, case.max, case.limit, case.ntrees);
        let next_known = !matches!(case.mode, 1 | 6);
        match case.mode {
            1 | 6 => {
                h.substate_tree_group.write(BROTLI_STATE_TREE_GROUP_LOOP);
                h.htree_index.write(if case.mode == 1 {
                    case.ntrees as i32
                } else {
                    i32::MAX
                });
            }
            2 => {
                let mut empty = BitReader::new();
                // SAFETY: actual empty-input source checkpoint before first
                // tree; retains real initialized next/index, no published slot.
                assert_eq!(
                    unsafe { read(g, h, &mut output, case.ntrees, &mut empty) },
                    BROTLI_DECODER_NEEDS_MORE_INPUT
                );
            }
            3 => {
                h.substate_tree_group.write(u32::MAX);
            }
            4 => {
                h.substate_tree_group.write(BROTLI_STATE_TREE_GROUP_LOOP);
                h.htree_index.write(-1);
                h.next.write(code_base);
                h.substate_huffman.write(u32::MAX);
            }
            5 => {
                assert_eq!(case.ntrees, 0);
                g.codes.write(0);
            }
            _ => {}
        }
        let mut br = BitReader::new();
        br.val = case.initial_val;
        br.bit_pos = case.initial_bits;
        let mut initialized = vec![false; case.ntrees];
        let mut offset = 0;
        let mut records = 0;
        for &chunk in &case.chunks {
            // SAFETY: checkpoint/bootstrap/explicit malformed discriminants
            // initialize only these scalar controls before the source call.
            let before = unsafe {
                if h.substate_tree_group.assume_init_read() == BROTLI_STATE_TREE_GROUP_LOOP {
                    h.htree_index.assume_init_read()
                } else {
                    0
                }
            };
            br.set_input(&case.data[offset..offset + chunk]);
            // SAFETY: actual stable source state/group allocations, all reached
            // alphabet/backing domains are generated as defined source cases;
            // only exact statuses returned by the actual helper are resumed.
            let result = unsafe { read(g, h, &mut output, case.ntrees, &mut br) };
            // SAFETY: every entry path initialized index; success increments
            // only after the actual pointer-slot write and next advance.
            let after = unsafe { h.htree_index.assume_init_read() };
            for i in before..after {
                initialized[usize::try_from(i).unwrap()] = true;
            }
            let mut record = Vec::new();
            for value in [
                result as i64 as u64,
                br.val,
                br.bit_pos,
                (offset + br.position()) as u64,
                (offset + br.guard_position()) as u64,
                br.available_in() as u64,
                u64::from(next_known),
                allocation_size as u64,
                code_offset as u64,
                capacity as u64,
            ] {
                record.extend_from_slice(&value.to_le_bytes());
            }
            // SAFETY: fixture physical initialization and exact field schema;
            // next normalization relies only on the stated source checkpoints.
            record.extend(unsafe { header_bytes(h, spans, next_known, code_base, capacity) });
            for (i, &known) in initialized.iter().enumerate() {
                record.extend_from_slice(&u64::from(known).to_le_bytes());
                if known {
                    // SAFETY: independent source-control-flow metadata proves
                    // this actual slot was written after a successful header.
                    let token = unsafe {
                        usize::from_ne_bytes(
                            output
                                .initialized_bytes(i * 8..(i + 1) * 8)
                                .try_into()
                                .unwrap(),
                        )
                    };
                    let delta = token.checked_sub(code_base).unwrap();
                    assert_eq!(delta % 4, 0);
                    assert!(delta / 4 <= capacity);
                    record.extend_from_slice(&((delta / 4) as u64).to_le_bytes());
                } else {
                    // SAFETY: physically seeded pointer bytes, not interpreted
                    // as initialized pointer/token values or used for a read.
                    record
                        .extend_from_slice(unsafe { output.initialized_bytes(i * 8..(i + 1) * 8) });
                }
            }
            // SAFETY: this fixture seeds all code fields before source writes;
            // explicit field observations never inspect HuffmanCode padding.
            unsafe {
                let codes = output.as_ptr().add(code_offset).cast::<HuffmanCode>();
                for i in 0..capacity {
                    record.push((*codes.add(i)).bits);
                    record.extend_from_slice(&(*codes.add(i)).value.to_le_bytes());
                }
            }
            sink(&record);
            records += 1;
            if result != BROTLI_DECODER_NEEDS_MORE_INPUT {
                break;
            }
            assert_eq!(br.available_in(), 0);
            offset += chunk;
        }
        records
    }

    #[test]
    fn actual_original_tree_groups_partial_fields_and_pointer_identity_match() {
        let oracle = std::env::var_os("BROTLI_TREE_GROUP_ORACLE").unwrap();
        let directory =
            std::path::PathBuf::from(std::env::var_os("BROTLI_TREE_GROUP_CAPTURE").unwrap());
        std::fs::create_dir_all(&directory).unwrap();
        let schema = Command::new(&oracle).arg("schema").output().unwrap();
        assert!(schema.status.success() && schema.stderr.is_empty());
        let mut cursor = 0;
        for value in [
            size_of::<HeaderArenaStorage>(),
            align_of::<HeaderArenaStorage>(),
            size_of::<DecoderStateStorage>(),
            offset_of!(DecoderStateStorage, arena),
            3756,
            1375,
        ] {
            assert_eq!(word(&schema.stdout, &mut cursor), value as u64);
        }
        let fields = fields();
        for &(offset, size) in &fields {
            assert_eq!(word(&schema.stdout, &mut cursor), offset as u64);
            assert_eq!(word(&schema.stdout, &mut cursor), size as u64);
        }
        for value in [
            size_of::<HuffmanTreeGroupStorage>(),
            align_of::<HuffmanTreeGroupStorage>(),
            offset_of!(HuffmanTreeGroupStorage, htrees),
            offset_of!(HuffmanTreeGroupStorage, codes),
            offset_of!(HuffmanTreeGroupStorage, alphabet_size_max),
            offset_of!(HuffmanTreeGroupStorage, alphabet_size_limit),
            offset_of!(HuffmanTreeGroupStorage, num_htrees),
            size_of::<HuffmanCode>(),
            size_of::<usize>(),
        ] {
            assert_eq!(word(&schema.stdout, &mut cursor), value as u64);
        }
        assert_eq!(cursor, schema.stdout.len());
        std::fs::write(directory.join("schema.bin"), schema.stdout).unwrap();
        let (fixtures, fixture_bytes) = fixtures(&oracle);
        std::fs::write(directory.join("fixtures.bin"), fixture_bytes).unwrap();
        let cases = cases(&fixtures);
        let mut requests =
            BufWriter::new(std::fs::File::create(directory.join("requests.txt")).unwrap());
        let mut expected =
            BufWriter::new(std::fs::File::create(directory.join("rust.bin")).unwrap());
        let mut records = 0usize;
        for case in &cases {
            write!(
                requests,
                "{} {} {} {} {} {} {} {}",
                case.max,
                case.limit,
                case.ntrees,
                case.mode,
                case.initial_bits,
                case.initial_val,
                case.chunks.len(),
                case.data.len()
            )
            .unwrap();
            for chunk in &case.chunks {
                write!(requests, " {chunk}").unwrap();
            }
            write!(requests, " ").unwrap();
            for byte in &case.data {
                write!(requests, "{byte:02x}").unwrap();
            }
            writeln!(requests).unwrap();
            for seed in [165, 90] {
                records += observe(case, seed, &fields, &mut |record| {
                    expected.write_all(record).unwrap()
                });
            }
        }
        requests.flush().unwrap();
        expected.flush().unwrap();
        drop(requests);
        drop(expected);
        let source = Command::new(&oracle)
            .arg("run")
            .stdin(Stdio::from(
                std::fs::File::open(directory.join("requests.txt")).unwrap(),
            ))
            .stdout(Stdio::from(
                std::fs::File::create(directory.join("source.bin")).unwrap(),
            ))
            .output()
            .unwrap();
        assert!(
            source.status.success(),
            "{}",
            String::from_utf8_lossy(&source.stderr)
        );
        assert!(source.stderr.is_empty());
        let mut source = BufReader::new(std::fs::File::open(directory.join("source.bin")).unwrap());
        let mut rust = BufReader::new(std::fs::File::open(directory.join("rust.bin")).unwrap());
        let mut a = [0u8; 65536];
        let mut b = [0u8; 65536];
        let mut offset = 0usize;
        loop {
            let n = source.read(&mut a).unwrap();
            let mut filled = 0;
            while filled < n {
                let m = rust.read(&mut b[filled..n]).unwrap();
                assert!(m > 0, "Rust capture truncated at{offset}");
                filled += m;
            }
            assert_eq!(&a[..n], &b[..n], "observation offset{offset}");
            offset += n;
            if n == 0 {
                assert_eq!(rust.read(&mut b[..1]).unwrap(), 0);
                break;
            }
        }
        std::fs::write(
            directory.join("counts.txt"),
            format!("{} {} {}\n", cases.len(), records, offset),
        )
        .unwrap();
    }
}
