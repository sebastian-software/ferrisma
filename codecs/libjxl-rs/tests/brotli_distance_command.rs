// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
#[path = "../src/brotli/command.rs"]
pub mod command;
#[path = "../src/brotli/constants.rs"]
pub mod constants;
#[path = "../src/brotli/distance.rs"]
pub mod distance;
#[path = "../src/brotli/huffman.rs"]
pub mod huffman;
#[path = "../src/brotli/layout.rs"]
pub mod layout;
#[path = "../src/brotli/memory.rs"]
pub mod memory;
#[path = "../src/brotli/symbols.rs"]
pub mod symbols;
#[path = "../src/brotli/tables.rs"]
pub mod tables;

use bit_reader::{BitReader, Reg};
use command::*;
use distance::*;
use huffman::HuffmanCode;
use layout::{ArenaStorage, BodyArenaStorage, DecoderStateStorage};
use std::mem::{MaybeUninit, size_of};
use symbols::HuffmanTable;
unsafe extern "C" fn allocate(_: *mut std::ffi::c_void, size: usize) -> *mut std::ffi::c_void {
    assert_eq!(size, size_of::<DecoderStateStorage>());
    let layout = std::alloc::Layout::from_size_align(size, 16).unwrap();
    // SAFETY: fixed nonzero source state extent and native-compatible alignment.
    unsafe { std::alloc::alloc(layout).cast() }
}
unsafe extern "C" fn free(_: *mut std::ffi::c_void, ptr: *mut std::ffi::c_void) {
    if ptr.is_null() {
        return;
    }
    let layout = std::alloc::Layout::from_size_align(size_of::<DecoderStateStorage>(), 16).unwrap();
    // SAFETY: Allocation invokes this once with the exact allocate return;
    // all source field borrows have ended before this callback.
    unsafe { std::alloc::dealloc(ptr.cast(), layout) };
}
fn owner(seed: Option<u8>) -> memory::Allocation {
    // SAFETY: static matched callbacks need no opaque context, return aligned
    // exclusive blocks and never inspect/reenter live borrowed storage.
    let allocator = unsafe {
        memory::Allocator::from_callbacks(Some(allocate), Some(free), std::ptr::null_mut())
    }
    .unwrap();
    let mut allocation = allocator
        .allocate(size_of::<DecoderStateStorage>())
        .unwrap();
    if let Some(seed) = seed {
        allocation.fill(seed);
    }
    allocation
}
fn state(owner: &mut memory::Allocation) -> &mut DecoderStateStorage {
    // SAFETY: actual source-sized/aligned owner, exclusive live borrow; the
    // repr(C) storage contains only MaybeUninit fields, not typed unknown data.
    unsafe {
        &mut *owner
            .storage_mut()
            .as_mut_ptr()
            .cast::<DecoderStateStorage>()
    }
}

fn body(storage: &mut MaybeUninit<ArenaStorage>) -> &mut BodyArenaStorage {
    // SAFETY: verified native union extent/alignment contains actual body
    // storage. Every body field is MaybeUninit; no unknown scalar is read.
    unsafe { &mut *storage.as_mut_ptr().cast::<BodyArenaStorage>() }
}
fn block(storage: &mut MaybeUninit<[Reg; 3]>, index: usize) -> &mut MaybeUninit<Reg> {
    assert!(index < 3);
    // SAFETY: actual three-element array, reached in-bounds MaybeUninit slot;
    // this exclusive view neither reads nor initializes any sibling counter.
    unsafe { &mut *storage.as_mut_ptr().cast::<MaybeUninit<Reg>>().add(index) }
}
fn ring(s: &mut DecoderStateStorage) -> DistanceRingFields<'_> {
    DistanceRingFields {
        distance_code: &mut s.distance_code,
        distance_context: &mut s.distance_context,
        dist_rb_idx: &mut s.dist_rb_idx,
        dist_rb: &s.dist_rb,
    }
}
fn distance(s: &mut DecoderStateStorage) -> DistanceFields<'_> {
    DistanceFields {
        ring: DistanceRingFields {
            distance_code: &mut s.distance_code,
            distance_context: &mut s.distance_context,
            dist_rb_idx: &mut s.dist_rb_idx,
            dist_rb: &s.dist_rb,
        },
        block_length: block(&mut s.block_length, 2),
        distance_postfix_bits: &s.distance_postfix_bits,
        dist_htree_index: &s.dist_htree_index,
        body: body(&mut s.arena),
    }
}
fn command(s: &mut DecoderStateStorage) -> CommandFields<'_> {
    CommandFields {
        distance_code: &mut s.distance_code,
        distance_context: &mut s.distance_context,
        dist_htree_index: &mut s.dist_htree_index,
        copy_length: &mut s.copy_length,
        block_length: block(&mut s.block_length, 1),
    }
}
struct TableRef<'a>(&'a [HuffmanCode]);
impl HuffmanTable for TableRef<'_> {
    fn bits(&self, index: usize) -> u8 {
        self.0[index].bits
    }
    fn value(&self, index: usize) -> u16 {
        self.0[index].value
    }
}
struct OnlyBits;
impl HuffmanTable for OnlyBits {
    fn bits(&self, index: usize) -> u8 {
        assert_eq!(index, 255);
        15
    }
    fn value(&self, _: usize) -> u16 {
        panic!("unknown Huffman value must not be read")
    }
}
struct Singleton(u16);
impl HuffmanTable for Singleton {
    fn bits(&self, index: usize) -> u8 {
        assert!(index < 256);
        0
    }
    fn value(&self, index: usize) -> u16 {
        assert!(index < 256);
        self.0
    }
}

#[test]
fn zero_width_safe_wrappers_initialize_output_without_touching_reader() {
    let mut br = BitReader::new();
    br.val = 0x123456789abcdef0;
    br.bit_pos = 0;
    for wide in [false, true] {
        let mut out = MaybeUninit::uninit();
        assert!(if wide {
            safe_read_bits32(&mut br, 0, &mut out)
        } else {
            safe_read_bits(&mut br, 0, &mut out)
        });
        // SAFETY: zero-width source branch just initialized the output.
        assert_eq!(unsafe { out.assume_init() }, 0);
        assert_eq!(
            (br.val, br.bit_pos, br.position()),
            (0x123456789abcdef0, 0, 0)
        );
    }
    for safe in [i32::MIN, -1, 1, i32::MAX] {
        assert!(check_input_amount(safe, &br));
    }
    assert!(!check_input_amount(0, &br));
}

#[test]
fn ring_reads_only_its_selected_initialized_slot_and_preserves_unknown_context() {
    for code in 0..18 {
        let mut allocation = owner(None);
        let s = state(&mut allocation);
        s.distance_code.write(code);
        s.dist_rb_idx.write(0);
        let slot = if code <= 3 {
            (3 - code) as usize
        } else if code < 10 {
            3
        } else {
            2
        };
        // SAFETY: actual history has four aligned words; only the selected
        // source slot is initialized. Every other payload word stays unknown.
        unsafe { s.dist_rb.as_mut_ptr().cast::<i32>().add(slot).write(777) };
        // SAFETY: selected slot and signed arithmetic are defined for0..17.
        // Context is genuinely unknown for code>=4 and must remain unread.
        unsafe { take_distance_from_ring_buffer(&mut ring(s)) };
        // SAFETY: every source path initializes code and retains initialized index.
        unsafe {
            assert!(s.distance_code.assume_init() > 0);
            assert_eq!(s.dist_rb_idx.assume_init(), if code == 0 { -1 } else { 0 });
            if code <= 3 {
                assert_eq!(s.distance_context.assume_init(), i32::from(code == 0));
            }
        }
    }
}

#[test]
fn lut_initializes_complete_group_beyond_limit_and_no_unreached_bytes() {
    let mut allocation = owner(None);
    let s = state(&mut allocation);
    s.distance_postfix_bits.write(3);
    s.num_direct_distance_codes.write(0);
    let limit = MaybeUninit::new(17);
    let b = body(&mut s.arena);
    // SAFETY: shift counts and full eight-entry group fit actual544 backing.
    unsafe {
        calculate_distance_lut(
            b,
            &s.distance_postfix_bits,
            &s.num_direct_distance_codes,
            &limit,
        )
    };
    for i in 16..24 {
        // SAFETY: original complete group wrote all8 entries, including17..23
        // beyond the logical limit17. Short-code slots/tail are never observed.
        unsafe {
            assert_eq!(b.dist_extra_bits.as_ptr().cast::<u8>().add(i).read(), 1);
            assert_eq!(
                b.dist_offset.as_ptr().cast::<Reg>().add(i).read(),
                (i - 15) as Reg
            );
        }
    }
    // Unconditional shift63 is defined even when no regular group is reached.
    let mut allocation = owner(None);
    let s = state(&mut allocation);
    s.distance_postfix_bits.write(63);
    s.num_direct_distance_codes.write(0);
    // SAFETY: no group/direct store is reached, so the body remains unknown.
    unsafe {
        calculate_distance_lut(
            body(&mut s.arena),
            &s.distance_postfix_bits,
            &s.num_direct_distance_codes,
            &MaybeUninit::new(16),
        )
    };
}

#[test]
fn distance_symbol_shortage_retains_pulled_bytes_without_touching_unknown_outputs() {
    let mut allocation = owner(None);
    let s = state(&mut allocation);
    s.distance_code.write(-1);
    s.dist_htree_index.write(9);
    let mut br = BitReader::new();
    br.set_input(&[255]);
    // SAFETY: source negative-code entry and selected tree index; symbol
    // shortage reaches no counter/context/LUT/history read, all still unknown.
    assert!(!unsafe {
        safe_read_distance(&mut distance(s), &mut br, |index| {
            assert_eq!(index, 9);
            OnlyBits
        })
    });
    assert_eq!((br.val, br.bit_pos, br.position()), (255, 8, 1));
}

#[test]
fn distance_suffix_shortage_restores_reader_and_counter_but_keeps_context_zero() {
    let mut allocation = owner(None);
    let s = state(&mut allocation);
    s.distance_code.write(-1);
    s.dist_htree_index.write(9);
    block(&mut s.block_length, 2).write(0);
    // SAFETY: only reached extra-width16 slot is initialized. Offset/postfix,
    // history and every neighboring LUT entry remain genuinely unknown.
    unsafe {
        body(&mut s.arena)
            .dist_extra_bits
            .as_mut_ptr()
            .cast::<u8>()
            .add(16)
            .write(32)
    };
    let mut br = BitReader::new();
    br.set_input(&[19, 92]);
    br.bit_pos = 1;
    // SAFETY: singleton16 and initialized width32 reach suffix shortage before
    // offset/postfix reads; source unsigned decrement/increment wrap at0.
    assert!(!unsafe {
        safe_read_distance(&mut distance(s), &mut br, |index| {
            assert_eq!(index, 9);
            Singleton(16)
        })
    });
    assert_eq!(
        (br.val, br.bit_pos, br.position(), br.available_in()),
        (0, 1, 0, 2)
    );
    // SAFETY: source context write and counter rollback initialized these slots.
    unsafe {
        assert_eq!(s.distance_context.assume_init(), 0);
        assert_eq!(block(&mut s.block_length, 2).assume_init(), 0);
        assert_eq!(s.distance_code.assume_init(), -1);
    }
}

#[test]
fn short_distance_does_not_read_unknown_lut_or_postfix_fields() {
    let mut allocation = owner(None);
    let s = state(&mut allocation);
    s.distance_code.write(-1);
    s.dist_htree_index.write(9);
    s.dist_rb_idx.write(0);
    block(&mut s.block_length, 2).write(0);
    // SAFETY: only short-code0's actual history slot3 is reached.
    unsafe { s.dist_rb.as_mut_ptr().cast::<i32>().add(3).write(777) };
    let mut br = BitReader::new();
    // SAFETY: initialized singleton0 table/index/history. No LUT/postfix input
    // is required; unrelated arena and ring slots remain unknown.
    assert!(unsafe { safe_read_distance(&mut distance(s), &mut br, |_| Singleton(0)) });
    // SAFETY: this successful short path initialized all fields observed here.
    unsafe {
        assert_eq!(s.distance_code.assume_init(), 777);
        assert_eq!(s.distance_context.assume_init(), 1);
        assert_eq!(s.dist_rb_idx.assume_init(), -1);
        assert_eq!(block(&mut s.block_length, 2).assume_init(), Reg::MAX);
    }
}

#[test]
fn command_symbol_shortage_retains_bytes_and_never_resolves_context_map() {
    let mut allocation = owner(None);
    let s = state(&mut allocation);
    let mut out = MaybeUninit::uninit();
    let mut br = BitReader::new();
    br.set_input(&[255]);
    // SAFETY: symbol shortage reaches no unknown output/counter or context-map
    // byte. Only the reached root bits field exists in the borrowed table.
    assert!(!unsafe {
        safe_read_command(
            &mut command(s),
            &mut br,
            &mut out,
            || OnlyBits,
            |_| panic!("map must not be resolved on symbol shortage"),
        )
    });
    assert_eq!((br.val, br.bit_pos, br.position()), (255, 8, 1));
}

#[test]
fn command_suffix_shortage_retains_early_writes_and_unknown_copy_counter() {
    let mut allocation = owner(None);
    let s = state(&mut allocation);
    let mut out = MaybeUninit::uninit();
    let mut br = BitReader::new();
    br.val = 0xffffff;
    br.bit_pos = 24;
    // SAFETY: singleton703 selects initialized static LUT. First24 suffix bits
    // succeed, second24 fail; copy/counter are unknown and must not be read.
    assert!(!unsafe {
        safe_read_command(
            &mut command(s),
            &mut br,
            &mut out,
            || Singleton(703),
            |context| {
                assert_eq!(context, 3);
                255
            },
        )
    });
    assert_eq!((br.val, br.bit_pos, br.position()), (0xffffff, 24, 0));
    let v = tables::CMD_LUT[703];
    // SAFETY: each selected output was written before the failing suffix;
    // insert still holds the source offset, not the already-read local extra.
    unsafe {
        assert_eq!(s.distance_code.assume_init(), i32::from(v.distance_code));
        assert_eq!(s.distance_context.assume_init(), i32::from(v.context));
        assert_eq!(s.dist_htree_index.assume_init(), 255);
        assert_eq!(out.assume_init(), i32::from(v.insert_len_offset));
    }
}

fn make_table(n: usize, family: usize, code: usize) -> Vec<HuffmanCode> {
    let size = if family < 2 { 256 } else { 4096 };
    let mut out = vec![MaybeUninit::new(HuffmanCode::new(165, 48879)); size];
    if family < 2 {
        huffman::brotli_build_simple_huffman_table(
            &mut out,
            8,
            &mut [code as u16, ((code + 1) % n) as u16, 0, 0],
            family as u32,
        );
    } else {
        let mut lengths = if family == 2 {
            (0..n)
                .map(|i| if i < 1024 - n { 9 } else { 10 })
                .collect::<Vec<_>>()
        } else {
            (1..=14).chain([15, 15]).collect::<Vec<_>>()
        };
        while lengths.len() < n {
            let index = (0..lengths.len()).min_by_key(|&i| lengths[i]).unwrap();
            lengths[index] += 1;
            lengths.push(lengths[index]);
        }
        let mut links = vec![65535; n + 16];
        let mut count = [0; 16];
        let mut next: [usize; 16] = std::array::from_fn(|i| i);
        for (i, &len) in lengths.iter().enumerate() {
            count[len] += 1;
            links[next[len]] = i as u16;
            next[len] = i + 16;
        }
        huffman::brotli_build_huffman_table(&mut out, 8, &links, 16, &mut count);
    }
    out.into_iter()
        .map(|slot| {
            // SAFETY: fixture seeded every code field before the actual builder;
            // no code padding is observed or interpreted as initialized data.
            unsafe { slot.assume_init() }
        })
        .collect()
}
fn tables() -> Vec<Vec<HuffmanCode>> {
    let mut out = Vec::new();
    for n in [544, 704] {
        for family in 0..2 {
            for code in 0..n {
                out.push(make_table(n, family, code));
            }
        }
        for family in 2..4 {
            out.push(make_table(n, family, 0));
        }
    }
    out
}

#[derive(Clone, Debug)]
struct Case {
    op: u32,
    family: usize,
    code: usize,
    safe: i32,
    postfix: Reg,
    direct: Reg,
    limit: u16,
    available: Reg,
    val: Reg,
    input: usize,
    block: Reg,
    rb_index: i32,
    context: i32,
    history: [i32; 4],
    seed: u8,
    pattern: u32,
}
impl Default for Case {
    fn default() -> Self {
        Self {
            op: 0,
            family: 0,
            code: 0,
            safe: 1,
            postfix: 0,
            direct: 0,
            limit: 16,
            available: 0,
            val: 0,
            input: 0,
            block: 1,
            rb_index: 0,
            context: -123,
            history: [16, 15, 11, 4],
            seed: 165,
            pattern: 19,
        }
    }
}
fn lut_defined(postfix: Reg, direct: Reg, limit: u16) -> bool {
    if postfix >= 64 || direct > 528 {
        return false;
    }
    let i = 16 + direct;
    if Reg::from(limit) <= i {
        return true;
    }
    let group = 1u128 << postfix;
    let groups = (u128::from(limit) - u128::from(i)).div_ceil(group);
    u128::from(i) + groups * group <= 544 && groups.div_ceil(2) <= 63
}
fn cases() -> Vec<Case> {
    let mut out = Vec::new();
    let base = Case::default();
    for code in 0..18 {
        for rb_index in [-1000, -4, -1, 0, 1, 3, 4, 1000] {
            for history in [
                [16, 15, 11, 4],
                [0, 1, 2, 3],
                [-10, -3, -1, 0],
                [i32::MAX - 3; 4],
                [i32::MIN + 3; 4],
            ] {
                out.push(Case {
                    code,
                    rb_index,
                    history,
                    ..base.clone()
                });
            }
        }
    }
    for op in [1, 2] {
        for code in 0..=if op == 1 { 24 } else { 32 } {
            for available in 0..=63 {
                for val in [0, Reg::MAX, 0xaaaa5555ffff0000] {
                    for input in 0..=4 {
                        out.push(Case {
                            op,
                            code,
                            available,
                            val,
                            input,
                            ..base.clone()
                        });
                    }
                }
            }
        }
    }
    for safe in [i32::MIN, -1, 0, 1, i32::MAX] {
        for input in 0..=128 {
            out.push(Case {
                op: 6,
                safe,
                input,
                ..base.clone()
            });
        }
    }
    for postfix in 0..64 {
        for direct in [0, 1, 15, 16, 31, 63, 120, 127, 255, 511, 527, 528] {
            let start = 16 + direct;
            let group = 1u128 << postfix;
            let mut limits = vec![0, 1, 15, 16, start - 1, start, start + 1, 543, 544];
            if postfix < 10 {
                for groups in [1u128, 2, 3, 4, 63, 64, 125, 126, 127] {
                    for delta in [-1i128, 0, 1] {
                        let limit = i128::from(start) + (group * groups) as i128 + delta;
                        if (0..=65535).contains(&limit) {
                            limits.push(limit as Reg);
                        }
                    }
                }
            }
            limits.sort_unstable();
            limits.dedup();
            for limit in limits {
                if lut_defined(postfix, direct, limit as u16) {
                    for seed in [165, 90] {
                        out.push(Case {
                            op: 3,
                            postfix,
                            direct,
                            limit: limit as u16,
                            seed,
                            ..base.clone()
                        });
                    }
                }
            }
        }
    }
    // Every actual normal-codec NPOSTFIX/NDIRECT combination, plus complete
    // group tails when a source-valid logical limit ends inside the group.
    for postfix in 0..4 {
        for n in 0..16 {
            let direct = n << postfix;
            let end = 16 + direct + (48 << postfix);
            for limit in [16 + direct, 16 + direct + 1, end - 1, end] {
                for seed in [0, 165] {
                    out.push(Case {
                        op: 3,
                        postfix,
                        direct,
                        limit: limit as u16,
                        seed,
                        ..base.clone()
                    });
                }
            }
        }
    }
    for command in [false, true] {
        let n = if command { 704 } else { 544 };
        let op = if command { 5 } else { 4 };
        let c = Case {
            op,
            postfix: 3,
            direct: 16,
            limit: 544,
            ..base.clone()
        };
        for code in 0..n {
            for available in 0..=if command { 49 } else { 33 } {
                for val in [0, Reg::MAX] {
                    out.push(Case {
                        code,
                        available,
                        val,
                        ..c.clone()
                    });
                }
            }
            for available in [0, 7, 15, 23, 31] {
                for input in 1..=6 {
                    out.push(Case {
                        code,
                        available,
                        input,
                        val: 0x123456789abcdef0,
                        ..c.clone()
                    });
                }
            }
            for available in [0, 15, 16, 32, 48, 63] {
                for val in [0, Reg::MAX] {
                    out.push(Case {
                        code,
                        safe: 0,
                        input: 128,
                        available,
                        val,
                        ..c.clone()
                    });
                }
            }
            for safe in [-1, 2] {
                out.push(Case {
                    code,
                    safe,
                    available: 63,
                    val: Reg::MAX,
                    ..c.clone()
                });
            }
            for wrapper in if command { [9, 10] } else { [7, 8] } {
                out.push(Case {
                    op: wrapper,
                    code,
                    input: 128,
                    available: 0,
                    ..c.clone()
                });
            }
        }
        for family in [2, 3] {
            // Every15-bit prefix reaches original general builder entries,
            // including native two-level dispatch; caller backing stays valid.
            for val in 0..32768 {
                out.push(Case {
                    family,
                    val,
                    available: 63,
                    safe: 0,
                    input: 128,
                    ..c.clone()
                });
            }
            for available in 0..=if command { 63 } else { 47 } {
                for val in [0, Reg::MAX, 0x5555, 0xaaaa, 0x123456789abcdef0] {
                    for input in 0..=4 {
                        out.push(Case {
                            family,
                            available,
                            val,
                            input,
                            ..c.clone()
                        });
                    }
                }
            }
        }
    }
    // Source counter operations are unsigned, including zero/MAX. Wider LUT
    // shifts are separate from readers, whose actual suffix width is<=32.
    for op in [4, 5] {
        for code in [0, 1, 15, 16, 25, 31, 63, 127, 255, 543] {
            for count in [0, Reg::MAX] {
                for available in [0, 1, 31, 63] {
                    out.push(Case {
                        op,
                        code,
                        block: count,
                        available,
                        postfix: 3,
                        direct: 16,
                        limit: 544,
                        ..base.clone()
                    });
                }
            }
        }
    }
    // Last defined signed index expressions for each source branch. These
    // cases stop before int overflow; no wrapping is invented for source UB.
    for code in 0..18 {
        let low = if code == 0 { i32::MIN + 1 } else { i32::MIN };
        let high = match code {
            0 => i32::MAX - 3,
            1 => i32::MAX - 2,
            2 => i32::MAX - 1,
            3 => i32::MAX,
            4..=9 => i32::MAX - 3,
            _ => i32::MAX - 2,
        };
        let histories = if code <= 3 {
            [[i32::MIN; 4], [i32::MAX; 4]]
        } else {
            [[i32::MIN + 3; 4], [i32::MAX - 3; 4]]
        };
        for rb_index in [low, high] {
            for history in histories {
                out.push(Case {
                    code,
                    rb_index,
                    history,
                    ..base.clone()
                });
            }
        }
    }
    out
}

fn observe(c: &Case, tables: &[Vec<HuffmanCode>]) -> Vec<u8> {
    let mut allocation = owner(Some(c.seed));
    let s = state(&mut allocation);
    s.distance_postfix_bits.write(c.postfix);
    s.num_direct_distance_codes.write(c.direct);
    s.dist_rb_idx.write(c.rb_index);
    s.dist_rb.write(c.history);
    s.distance_code
        .write(if c.op == 0 { c.code as i32 } else { -1 });
    s.distance_context.write(c.context);
    block(&mut s.block_length, 1).write(c.block);
    block(&mut s.block_length, 2).write(c.block);
    s.dist_htree_index.write(9);
    s.copy_length.write(0x5a789abc);
    let input: [u8; 128] = std::array::from_fn(|i| (i * 73 + c.pattern as usize) as u8);
    let mut br = BitReader::new();
    br.set_input(&input[..c.input]);
    br.val = c.val;
    br.bit_pos = c.available;
    let mut output = MaybeUninit::new(3735928559);
    let mut insert = MaybeUninit::new(0x5a123456);
    let mut ok = true;
    if matches!(c.op, 3 | 4 | 7 | 8) {
        assert!(lut_defined(c.postfix, c.direct, c.limit));
        // SAFETY: fixture independently checks full-group backing and all
        // unsigned shift counts. Only fieldwise initialized output is needed.
        unsafe {
            calculate_distance_lut(
                body(&mut s.arena),
                &s.distance_postfix_bits,
                &s.num_direct_distance_codes,
                &MaybeUninit::new(c.limit),
            )
        };
    }
    let is_command = matches!(c.op, 5 | 9 | 10);
    let n = if is_command { 704 } else { 544 };
    let index = (if is_command { 1090 } else { 0 })
        + if c.family < 2 {
            c.family * n + c.code
        } else {
            2 * n + c.family - 2
        };
    match c.op {
        0 => {
            // SAFETY: corpus ring codes0..17, initialized history and signed
            // arithmetic is kept within the original int domain by construction.
            unsafe { take_distance_from_ring_buffer(&mut ring(s)) };
        }
        1 => ok = safe_read_bits(&mut br, c.code as Reg, &mut output),
        2 => ok = safe_read_bits32(&mut br, c.code as Reg, &mut output),
        3 => {
            let b = body(&mut s.arena);
            let mut out = Vec::new();
            // SAFETY: fixture physically initializes every byte before source
            // writes. Full arrays have no padding; no logical-init inference.
            unsafe {
                out.extend_from_slice(std::slice::from_raw_parts(
                    b.dist_extra_bits.as_ptr().cast::<u8>(),
                    544,
                ));
                out.extend_from_slice(std::slice::from_raw_parts(
                    b.dist_offset.as_ptr().cast::<u8>(),
                    544 * size_of::<Reg>(),
                ));
            }
            return out;
        }
        4 | 7 | 8 => {
            let tree_at = |i| {
                assert_eq!(i, 9);
                TableRef(&tables[index])
            };
            // SAFETY: complete fixture tables decode0..543; LUT prefix544
            // and widths<=32 are initialized, ring/shift domains are defined;
            // fast modes retain actual128-byte input backing.
            unsafe {
                match c.op {
                    4 => ok = read_distance_internal(c.safe, &mut distance(s), &mut br, tree_at),
                    7 => read_distance(&mut distance(s), &mut br, tree_at),
                    _ => ok = safe_read_distance(&mut distance(s), &mut br, tree_at),
                }
            }
        }
        5 | 9 | 10 => {
            let map = [0, 7, 255, 3];
            let map_at = |context| map[context];
            // SAFETY: actual complete command table and selected map bytes
            // initialized, fields disjoint and signed extra-bit additions fit.
            unsafe {
                match c.op {
                    5 => {
                        ok = read_command_internal(
                            c.safe,
                            &mut command(s),
                            &mut br,
                            &mut insert,
                            || TableRef(&tables[index]),
                            map_at,
                        )
                    }
                    9 => read_command(
                        &mut command(s),
                        &mut br,
                        &mut insert,
                        || TableRef(&tables[index]),
                        map_at,
                    ),
                    _ => {
                        ok = safe_read_command(
                            &mut command(s),
                            &mut br,
                            &mut insert,
                            || TableRef(&tables[index]),
                            map_at,
                        )
                    }
                }
            }
        }
        6 => ok = check_input_amount(c.safe, &br),
        _ => unreachable!(),
    }
    // SAFETY: every observed scalar is explicitly fixture-initialized above;
    // source writes retain valid representation. No padding bytes are exposed.
    let fields = unsafe {
        [
            u64::from(ok),
            output.assume_init(),
            br.val,
            br.bit_pos,
            br.position() as u64,
            br.guard_position() as u64,
            br.available_in() as u64,
            s.distance_code.assume_init() as i64 as u64,
            s.distance_context.assume_init() as i64 as u64,
            s.dist_rb_idx.assume_init() as i64 as u64,
            block(&mut s.block_length, 1).assume_init(),
            block(&mut s.block_length, 2).assume_init(),
            s.copy_length.assume_init() as i64 as u64,
            u64::from(s.dist_htree_index.assume_init()),
            insert.assume_init() as i64 as u64,
            s.dist_rb.as_ptr().cast::<i32>().read() as i64 as u64,
            s.dist_rb.as_ptr().cast::<i32>().add(1).read() as i64 as u64,
            s.dist_rb.as_ptr().cast::<i32>().add(2).read() as i64 as u64,
            s.dist_rb.as_ptr().cast::<i32>().add(3).read() as i64 as u64,
            u64::from(check_input_amount(0, &br)),
        ]
    };
    fields.into_iter().flat_map(u64::to_le_bytes).collect()
}

#[test]
fn source_defined_corpus_reaches_every_helper_natively() {
    let tables = tables();
    for c in cases().iter().step_by(101) {
        observe(c, &tables);
    }
}

#[cfg(feature = "cpp-oracle")]
#[test]
fn actual_original_distance_command_and_lut_records_match() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let capture =
        std::path::PathBuf::from(std::env::var_os("BROTLI_DISTANCE_COMMAND_CAPTURE").unwrap());
    std::fs::create_dir_all(&capture).unwrap();
    let oracle = std::env::var_os("BROTLI_DISTANCE_COMMAND_ORACLE").unwrap();
    let commands = Command::new(&oracle).arg("commands").output().unwrap();
    assert!(commands.status.success() && commands.stderr.is_empty());
    let mut expected_commands = Vec::new();
    for v in tables::CMD_LUT {
        for scalar in [
            u64::from(v.insert_len_extra_bits),
            u64::from(v.copy_len_extra_bits),
            v.distance_code as i64 as u64,
            u64::from(v.context),
            u64::from(v.insert_len_offset),
            u64::from(v.copy_len_offset),
        ] {
            expected_commands.extend_from_slice(&scalar.to_le_bytes());
        }
    }
    assert_eq!(commands.stdout, expected_commands);
    std::fs::write(capture.join("commands.bin"), &commands.stdout).unwrap();
    let tables = tables();
    let cases = cases();
    let path = capture.join("requests.txt");
    let mut file = std::io::BufWriter::new(std::fs::File::create(&path).unwrap());
    let mut expected = Vec::new();
    let mut widths = Vec::new();
    for c in &cases {
        writeln!(
            file,
            "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            c.op,
            c.family,
            c.code,
            c.safe,
            c.postfix,
            c.direct,
            c.limit,
            c.available,
            c.val,
            c.input,
            c.block,
            c.rb_index,
            c.context,
            c.history[0],
            c.history[1],
            c.history[2],
            c.history[3],
            c.seed,
            c.pattern
        )
        .unwrap();
        let bytes = observe(c, &tables);
        widths.push(bytes.len());
        expected.extend(bytes);
    }
    file.flush().unwrap();
    drop(file);
    let source = Command::new(&oracle)
        .arg("run")
        .stdin(Stdio::from(std::fs::File::open(&path).unwrap()))
        .output()
        .unwrap();
    assert!(
        source.status.success(),
        "{}",
        String::from_utf8_lossy(&source.stderr)
    );
    assert!(source.stderr.is_empty());
    std::fs::write(capture.join("source.bin"), &source.stdout).unwrap();
    std::fs::write(capture.join("rust.bin"), &expected).unwrap();
    assert_eq!(source.stdout.len(), expected.len());
    let mut offset = 0;
    for (i, width) in widths.into_iter().enumerate() {
        assert_eq!(
            &source.stdout[offset..offset + width],
            &expected[offset..offset + width],
            "case{i}: {:?}",
            cases[i]
        );
        offset += width;
    }
    let counts: (usize, usize) = (cases.len(), cases.iter().filter(|c| c.op == 3).count());
    std::fs::write(
        capture.join("counts.txt"),
        format!("{} {}\n", counts.0, counts.1),
    )
    .unwrap();
}
