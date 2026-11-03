// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
#[path = "../src/brotli/block_length.rs"]
pub mod block_length;
#[path = "../src/brotli/constants.rs"]
pub mod constants;
#[path = "../src/brotli/huffman.rs"]
pub mod huffman;
#[path = "../src/brotli/layout.rs"]
pub mod layout;
#[path = "../src/brotli/memory.rs"]
pub mod memory;
#[path = "../src/brotli/running.rs"]
pub mod running;
#[path = "../src/brotli/symbols.rs"]
pub mod symbols;

use bit_reader::{BitReader, Reg};
use block_length::*;
use huffman::HuffmanCode;
use layout::DecoderStateStorage;
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

struct BitsOnly;
impl HuffmanTable for BitsOnly {
    fn bits(&self, index: usize) -> u8 {
        assert_eq!(index, 0);
        1
    }
    fn value(&self, _: usize) -> u16 {
        panic!("unreached value must remain unknown")
    }
}
struct NoTable;
impl HuffmanTable for NoTable {
    fn bits(&self, _: usize) -> u8 {
        panic!("suffix must not read table bits")
    }
    fn value(&self, _: usize) -> u16 {
        panic!("suffix must not read table value")
    }
}

#[test]
fn symbol_shortage_leaves_actual_unknown_index_and_result_untouched() {
    let mut allocation = owner(None);
    let state = state(&mut allocation);
    state.substate_read_block_length.write(0);
    let mut result = MaybeUninit::uninit();
    let mut br = BitReader::new();
    // SAFETY: actual StateInit checkpoint NONE; index/result are deliberately
    // unknown. Only the reached bits(0) field is supplied; no value is read.
    assert!(!unsafe {
        safe_read_block_length(
            &mut state.substate_read_block_length,
            &mut state.block_length_index,
            &mut result,
            &BitsOnly,
            &mut br,
        )
    });
    // SAFETY: StateInit initialized this field; failed symbol read preserves it.
    assert_eq!(unsafe { state.substate_read_block_length.assume_init() }, 0);
    // Neither unknown output is read merely to assert that it was untouched.
}

#[test]
fn suffix_checkpoint_initializes_only_index_then_resumes_without_table() {
    let mut allocation = owner(None);
    let state = state(&mut allocation);
    state.substate_read_block_length.write(0);
    let table = [HuffmanCode::new(0, 25); 256];
    let mut br = BitReader::new();
    let mut result = MaybeUninit::uninit();
    // SAFETY: source NONE and complete singleton table; unknown index/result
    // are not read. Suffix shortage must initialize only index and substate.
    assert!(!unsafe {
        safe_read_block_length(
            &mut state.substate_read_block_length,
            &mut state.block_length_index,
            &mut result,
            &table[..],
            &mut br,
        )
    });
    // SAFETY: the just-reached suffix checkpoint wrote these two fields.
    assert_eq!(unsafe { state.block_length_index.assume_init() }, 25);
    assert_eq!(unsafe { state.substate_read_block_length.assume_init() }, 1);
    br.set_input(&[0x19, 0x73, 0xe1]);
    // SAFETY: source suffix checkpoint supplied index 25, backing contains all
    // 24 suffix bits; no table field exists on this reached source path.
    assert!(unsafe {
        safe_read_block_length(
            &mut state.substate_read_block_length,
            &mut state.block_length_index,
            &mut result,
            &NoTable,
            &mut br,
        )
    });
    // SAFETY: successful suffix read just initialized result and substate.
    assert_eq!(
        unsafe { result.assume_init() },
        Reg::from(constants::BROTLI_PREFIX_CODE_RANGES[25].offset) + 0xe17319
    );
    assert_eq!(unsafe { state.substate_read_block_length.assume_init() }, 0);
}

#[test]
fn every_non_none_raw_state_uses_the_retained_index() {
    for raw in [1, 2, 0x7fffffff, 0x80000000, u32::MAX] {
        let mut allocation = owner(None);
        let state = state(&mut allocation);
        state.substate_read_block_length.write(raw);
        state.block_length_index.write(0);
        let mut result = MaybeUninit::uninit();
        let mut br = BitReader::new();
        br.val = 3;
        br.bit_pos = 2;
        // SAFETY: arbitrary raw u32 source state has initialized retained
        // index 0; the C source's else branch accepts all non-NONE values.
        assert!(unsafe {
            safe_read_block_length(
                &mut state.substate_read_block_length,
                &mut state.block_length_index,
                &mut result,
                &NoTable,
                &mut br,
            )
        });
        // SAFETY: successful call initialized result.
        assert_eq!(unsafe { result.assume_init() }, 4);
    }
}

fn tables() -> Vec<Vec<HuffmanCode>> {
    let mut tables = Vec::new();
    for family in 0..4 {
        for code in 0..26 {
            let mut out = vec![MaybeUninit::new(HuffmanCode::new(165, 48879)); 512];
            if family < 2 {
                huffman::brotli_build_simple_huffman_table(
                    &mut out,
                    8,
                    &mut [code, (code + 1) % 26, 0, 0],
                    family,
                );
            } else {
                let mut lengths = if family == 2 {
                    (0..26)
                        .map(|i| if i < 6 { 4 } else { 5 })
                        .collect::<Vec<_>>()
                } else {
                    (1..=14).chain([15, 15]).collect::<Vec<_>>()
                };
                while lengths.len() < 26 {
                    let index = (0..lengths.len()).min_by_key(|&i| lengths[i]).unwrap();
                    lengths[index] += 1;
                    lengths.push(lengths[index]);
                }
                let mut links = [65535; 42];
                let mut next: [usize; 16] = std::array::from_fn(|i| i);
                let mut count = [0; 16];
                for (i, &len) in lengths.iter().enumerate() {
                    count[len] += 1;
                    links[next[len]] = i as u16;
                    next[len] = i + 16;
                }
                huffman::brotli_build_huffman_table(&mut out, 8, &links, 16, &mut count);
            }
            tables.push(
                out.into_iter()
                    .map(|slot| {
                        // SAFETY: fixture initializes every entry before source builder;
                        // writes preserve valid bits/value; no struct padding is observed.
                        unsafe { slot.assume_init() }
                    })
                    .collect(),
            );
        }
    }
    tables
}

#[derive(Clone, Debug)]
struct Case {
    family: usize,
    code: usize,
    op: u32,
    state: u32,
    index_known: bool,
    index: Reg,
    available: Reg,
    val: Reg,
    input: Vec<u8>,
    chunks: Vec<usize>,
}
fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    let base = Case {
        family: 0,
        code: 0,
        op: 1,
        state: 0,
        index_known: false,
        index: 0,
        available: 0,
        val: 0,
        input: Vec::new(),
        chunks: vec![0],
    };
    for family in 0..4 {
        let codes = if family < 2 { 26 } else { 1 };
        for code in 0..codes {
            let c = Case {
                family,
                code,
                ..base.clone()
            };
            for available in 0..=39 {
                for val in [0, Reg::MAX, 0xaaaa5555ffff0000] {
                    for size in 0..=4 {
                        cases.push(Case {
                            available,
                            val,
                            input: (0..size).map(|i| (i * 73 + 19) as u8).collect(),
                            chunks: vec![size],
                            ..c.clone()
                        });
                    }
                    cases.push(Case {
                        available,
                        val,
                        input: vec![19, 92, 165, 238],
                        chunks: vec![0, 0, 1, 0, 1, 1, 1],
                        ..c.clone()
                    });
                }
            }
            for available in [0, 1, 8, 15, 16, 17, 31, 32, 33, 55, 56, 63] {
                for val in [0, Reg::MAX, 0xaaaa5555ffff0000] {
                    cases.push(Case {
                        op: 0,
                        available,
                        val,
                        input: (0..128).map(|i| (i * 73 + 19) as u8).collect(),
                        chunks: vec![128],
                        ..c.clone()
                    });
                }
            }
        }
        if family >= 2 {
            for val in 0..32768 {
                cases.push(Case {
                    family,
                    val,
                    available: 39,
                    ..base.clone()
                });
            }
        }
    }
    for index in 0..26 {
        for state in [1, 2, 0x7fffffff, 0x80000000, u32::MAX] {
            for available in 0..=24 {
                for val in [0, Reg::MAX, 0x123456789abcdef0] {
                    cases.push(Case {
                        state,
                        index,
                        index_known: true,
                        available,
                        val,
                        input: vec![19, 92, 165],
                        chunks: vec![0, 1, 0, 1, 1],
                        ..base.clone()
                    });
                }
            }
        }
    }
    cases
}

fn observe(c: &Case, table: &[HuffmanCode], seed: u8) -> Vec<u8> {
    let mut allocation = owner(Some(seed));
    let state = state(&mut allocation);
    state.substate_read_block_length.write(c.state);
    if c.index_known {
        state.block_length_index.write(c.index);
    }
    let mut result = MaybeUninit::new(3735928559);
    let mut br = BitReader::new();
    br.val = c.val;
    br.bit_pos = c.available;
    let mut out = Vec::new();
    let mut offset = 0;
    for &chunk in &c.chunks {
        br.set_input(&c.input[offset..offset + chunk]);
        let ok = if c.op == 0 {
            result.write(read_block_length(table, &mut br));
            true
        } else {
            // SAFETY: fixture seeds every owner byte and supplies the
            // source's raw-state/index domain with disjoint actual fields.
            unsafe {
                safe_read_block_length(
                    &mut state.substate_read_block_length,
                    &mut state.block_length_index,
                    &mut result,
                    table,
                    &mut br,
                )
            }
        };
        // SAFETY: result is fixture-initialized; state is written initially and
        // preserved/replaced by source. Index observation is bytewise below.
        let fields = unsafe {
            [
                u64::from(ok),
                result.assume_init(),
                br.val,
                br.bit_pos,
                (offset + br.position()) as u64,
                (offset + br.guard_position()) as u64,
                br.available_in() as u64,
                u64::from(state.substate_read_block_length.assume_init()),
            ]
        };
        for field in fields {
            out.extend_from_slice(&field.to_le_bytes());
        }
        // SAFETY: this fixture physically seeds every byte in the actual
        // block; observing this field bytewise does not assert logical init.
        out.extend_from_slice(unsafe {
            std::slice::from_raw_parts(
                state.block_length_index.as_ptr().cast::<u8>(),
                size_of::<Reg>(),
            )
        });
        if ok {
            break;
        }
        assert_eq!(br.available_in(), 0);
        offset += chunk;
    }
    out
}

#[test]
fn all_source_table_families_and_checkpoints_run_natively() {
    let tables = tables();
    for c in cases().iter().step_by(101) {
        observe(c, &tables[c.family * 26 + c.code], 165);
    }
}

#[cfg(feature = "cpp-oracle")]
#[test]
fn actual_original_block_observations_match_every_byte() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let cases = cases();
    let tables = tables();
    let capture = std::path::PathBuf::from(std::env::var_os("BROTLI_BLOCK_MTF_CAPTURE").unwrap())
        .join("block");
    std::fs::create_dir_all(&capture).unwrap();
    let table_source = Command::new(std::env::var_os("BROTLI_BLOCK_MTF_ORACLE").unwrap())
        .arg("tables")
        .output()
        .unwrap();
    assert!(table_source.status.success());
    assert!(table_source.stderr.is_empty());
    let mut table_expected = Vec::new();
    for table in &tables {
        for entry in table {
            table_expected.extend_from_slice(&u64::from(entry.bits).to_le_bytes());
            table_expected.extend_from_slice(&u64::from(entry.value).to_le_bytes());
        }
    }
    assert_eq!(table_source.stdout, table_expected);
    std::fs::write(capture.join("tables.bin"), &table_source.stdout).unwrap();
    let requests = capture.join("requests.txt");
    let mut file = std::io::BufWriter::new(std::fs::File::create(&requests).unwrap());
    let mut expected = Vec::new();
    for c in &cases {
        write!(
            file,
            "{} {} {} {} {} {} {} {} {} {}",
            c.family,
            c.code,
            c.op,
            c.state,
            u8::from(c.index_known),
            c.index,
            c.available,
            c.val,
            c.input.len(),
            c.chunks.len()
        )
        .unwrap();
        for chunk in &c.chunks {
            write!(file, " {chunk}").unwrap();
        }
        write!(file, " ").unwrap();
        for byte in &c.input {
            write!(file, "{byte:02x}").unwrap();
        }
        writeln!(file).unwrap();
        for seed in [165, 90] {
            expected.extend(observe(c, &tables[c.family * 26 + c.code], seed));
        }
    }
    file.flush().unwrap();
    drop(file);
    let source = Command::new(std::env::var_os("BROTLI_BLOCK_MTF_ORACLE").unwrap())
        .arg("block")
        .stdin(Stdio::from(std::fs::File::open(&requests).unwrap()))
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
    for (i, (source, rust)) in source
        .stdout
        .chunks_exact(72)
        .zip(expected.chunks_exact(72))
        .enumerate()
    {
        assert_eq!(source, rust, "record {i}");
    }
    std::fs::write(
        capture.join("counts.txt"),
        format!("{} {}\n", cases.len(), expected.len() / 72),
    )
    .unwrap();
}
