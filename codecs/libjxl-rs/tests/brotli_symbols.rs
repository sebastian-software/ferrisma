// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Isolated module registration for this unintegrated support packet.
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
#[path = "../src/brotli/constants.rs"]
pub mod constants;
#[path = "../src/brotli/huffman.rs"]
pub mod huffman;
#[path = "../src/brotli/symbols.rs"]
pub mod symbols;

use bit_reader::{BitReader, Reg};
use huffman::HuffmanCode;
use std::cell::RefCell;
use std::mem::MaybeUninit;
use symbols::*;

fn tables() -> Vec<Vec<HuffmanCode>> {
    (0..35)
        .map(|id| {
            let mut out = vec![MaybeUninit::new(HuffmanCode::new(165, 48879)); 33200];
            let size = if id < 5 {
                huffman::brotli_build_simple_huffman_table(
                    &mut out,
                    8,
                    &mut [65535, 0, 257, 17],
                    id,
                )
            } else {
                let depth = if id < 20 {
                    id - 4
                } else if id == 34 {
                    1
                } else {
                    id - 18
                } as usize;
                let n = if id < 20 {
                    1 << depth
                } else if id == 34 {
                    4
                } else {
                    depth + 1
                };
                let mut links = vec![65535; n + 16];
                let mut count = [0; 16];
                let mut next: [usize; 16] = std::array::from_fn(|i| i);
                for i in 0..n {
                    let len = if id == 34 {
                        i & 1
                    } else if id < 20 {
                        depth
                    } else if i < depth - 1 {
                        i + 1
                    } else {
                        depth
                    };
                    if len == 0 {
                        continue;
                    }
                    count[len] += 1;
                    links[next[len]] = i as u16;
                    next[len] = i + 16;
                }
                huffman::brotli_build_huffman_table(&mut out, 8, &links, 16, &mut count)
            } as usize;
            out[..size]
                .iter()
                .map(|slot| {
                    // SAFETY: every slot was initialized before the source
                    // builder; its writes replace both fields with valid data.
                    unsafe { slot.assume_init() }
                })
                .collect()
        })
        .collect()
}

struct PartialTable {
    bits: Vec<MaybeUninit<u8>>,
    values: Vec<MaybeUninit<u16>>,
    initialized_bits: Vec<bool>,
    initialized_values: Vec<bool>,
    trace: RefCell<Vec<(char, usize)>>,
}

impl PartialTable {
    fn new(n: usize) -> Self {
        Self {
            bits: vec![MaybeUninit::uninit(); n],
            values: vec![MaybeUninit::uninit(); n],
            initialized_bits: vec![false; n],
            initialized_values: vec![false; n],
            trace: RefCell::new(Vec::new()),
        }
    }
    fn bit(&mut self, index: usize, bits: u8) {
        self.bits[index].write(bits);
        self.initialized_bits[index] = true;
    }
    fn val(&mut self, index: usize, value: u16) {
        self.values[index].write(value);
        self.initialized_values[index] = true;
    }
}

impl HuffmanTable for PartialTable {
    fn bits(&self, index: usize) -> u8 {
        self.trace.borrow_mut().push(('b', index));
        assert!(self.initialized_bits[index]);
        // SAFETY: independent per-field metadata is assigned only after the
        // corresponding write above. Other fields/slots remain uninitialized.
        unsafe { self.bits[index].assume_init() }
    }
    fn value(&self, index: usize) -> u16 {
        self.trace.borrow_mut().push(('v', index));
        assert!(self.initialized_values[index]);
        // SAFETY: checked per-field metadata proves this exact value was
        // written; no other field or table padding is accessed.
        unsafe { self.values[index].assume_init() }
    }
}

#[test]
fn failing_safe_reads_touch_only_reached_bits_fields() {
    let mut table = PartialTable::new(512);
    let mut br = BitReader::new();
    let mut result = 4660;
    table.bit(0, 1);
    assert!(!safe_decode_symbol(&table, &mut br, &mut result));
    assert_eq!(result, 4660);
    assert_eq!(*table.trace.borrow(), [('b', 0)]);
    table.trace.borrow_mut().clear();
    table.bit(7, 5);
    br.val = 7;
    br.bit_pos = 3;
    assert!(!safe_decode_symbol(&table, &mut br, &mut result));
    assert_eq!(*table.trace.borrow(), [('b', 7), ('b', 7)]);
    assert_eq!((br.val, br.bit_pos, result), (7, 3, 4660));
    table.trace.borrow_mut().clear();
    table.bit(7, 10);
    assert!(!safe_decode_symbol(&table, &mut br, &mut result));
    assert_eq!(*table.trace.borrow(), [('b', 7)]);
    table.trace.borrow_mut().clear();
    table.val(7, 249);
    table.bit(259, 3);
    br.val = 775; // root 7, second-level index 7 + 249 + 3.
    br.bit_pos = 10;
    assert!(!safe_decode_symbol(&table, &mut br, &mut result));
    assert_eq!(
        *table.trace.borrow(),
        [('b', 7), ('b', 7), ('v', 7), ('b', 259)]
    );
    assert_eq!((br.val, br.bit_pos, result), (775, 10, 4660));
}

#[test]
fn successful_two_level_reads_preserve_separate_source_load_order() {
    let mut table = PartialTable::new(260);
    table.bit(7, 10);
    table.val(7, 249);
    table.bit(259, 2);
    table.val(259, 65535);
    let mut br = BitReader::new();
    br.val = 775;
    br.bit_pos = 15;
    assert_eq!(decode_symbol(775, &table, &mut br), 65535);
    assert_eq!((br.val, br.bit_pos), (0, 5));
    assert_eq!(
        *table.trace.borrow(),
        [('b', 7), ('b', 7), ('v', 7), ('b', 259), ('v', 259)]
    );
    table.trace.borrow_mut().clear();
    br.val = 775;
    br.bit_pos = 10;
    let mut result = 123;
    assert!(safe_decode_symbol(&table, &mut br, &mut result));
    assert_eq!(result, 65535);
    assert_eq!(
        *table.trace.borrow(),
        [
            ('b', 7),
            ('b', 7),
            ('v', 7),
            ('b', 259),
            ('b', 259),
            ('v', 259)
        ]
    );
}

#[test]
fn safe_preload_has_no_reader_or_table_access() {
    let table = PartialTable::new(0);
    let mut br = BitReader::new();
    br.val = 123;
    br.bit_pos = 3;
    let (mut bits, mut value) = (11, 65535);
    for safe in [-2147483648, -1, 1, 2147483647] {
        preload_symbol(safe, &table, &mut br, &mut bits, &mut value);
    }
    assert_eq!((bits, value, br.val, br.bit_pos), (11, 65535, 123, 3));
    assert!(table.trace.borrow().is_empty());
}

#[test]
fn preloaded_two_level_path_does_not_reload_cached_root_fields() {
    let mut table = PartialTable::new(260);
    table.bit(259, 2);
    table.val(259, 65535);
    table.bit(0, 0);
    table.val(0, 17);
    let mut br = BitReader::new();
    br.val = 775;
    br.bit_pos = 30;
    let (mut bits, mut value) = (10, 249);
    assert_eq!(
        read_preloaded_symbol(&table, &mut br, &mut bits, &mut value),
        65535
    );
    assert_eq!((bits, value, br.bit_pos), (0, 17, 20));
    assert_eq!(
        *table.trace.borrow(),
        [('b', 259), ('v', 259), ('b', 0), ('v', 0)]
    );
    // Root 7's bits and value were not initialized in this actual table owner.
}

#[test]
fn writer_extends_actual_initialized_destination_only_at_reached_indices() {
    let codes = tables();
    let input = [19; 128];
    let mut br = BitReader::new();
    br.set_input(&input);
    let (mut bits, mut value) = (0, 0);
    preload_symbol(0, &codes[4][..], &mut br, &mut bits, &mut value);
    let mut dst = [MaybeUninit::<u8>::uninit(); 64];
    for slot in &mut dst[..3] {
        slot.write(7);
    }
    let mut initialized = 3;
    let count = copy_preloaded_symbols_to_u8_with_writer(
        &codes[4][..],
        &mut br,
        &mut bits,
        &mut value,
        |index, byte| {
            assert_eq!(index, initialized);
            dst[index as usize].write(byte);
            initialized += 1;
        },
        3,
        41,
    );
    assert_eq!(count, 41);
    assert_eq!(initialized, 44);
    // This proof deliberately never observes the unwritten prefix or tail.
}

#[derive(Clone, Copy, Debug)]
struct Case {
    op: u32,
    table: usize,
    available: Reg,
    val: Reg,
    input: usize,
    pos: i32,
    limit: i32,
    safe: i32,
}

fn cases() -> Vec<Case> {
    let mut out = Vec::new();
    for table in 0..35 {
        let base = Case {
            op: 0,
            table,
            available: 15,
            val: 0,
            input: 0,
            pos: 3,
            limit: 31,
            safe: 0,
        };
        // Every 15-bit prefix exercises every reached entry in all uniform
        // depths 1..15 and all long-chain secondary tables, not just a digest.
        for val in 0..32768 {
            out.push(Case { val, ..base });
        }
        for available in 0..=15 {
            let mask = (1 << available) - 1;
            let mut vals = vec![0, mask, Reg::MAX, 0xaaaa_5555_ffff_0000u64 as Reg];
            if available <= 8 {
                vals.extend(0..=mask);
            } else {
                vals.extend((0..256).map(|i| ((i * 257) as Reg) & mask));
            }
            for val in vals {
                if available < 15 {
                    out.push(Case {
                        op: 1,
                        available,
                        val,
                        ..base
                    });
                }
                for input in [0, 1, 2, 3] {
                    out.push(Case {
                        op: 2,
                        available,
                        val,
                        input,
                        ..base
                    });
                    out.push(Case {
                        op: 7,
                        available,
                        val,
                        input,
                        ..base
                    });
                }
            }
        }
        for available in [0, 1, 8, 9, 15, 16, 17, 31, 32, 33, 55, 56, 63] {
            for val in [0, 1, 255, 256, 32767, Reg::MAX] {
                for op in [3, 4, 5] {
                    out.push(Case {
                        op,
                        available,
                        val,
                        input: 128,
                        ..base
                    });
                }
                for input in [8, 27, 28, 29, 35, 64, 128] {
                    for limit in [-16, -1, 0, 1, 4, 15, 31, 61] {
                        out.push(Case {
                            op: 6,
                            available,
                            val,
                            input,
                            limit,
                            ..base
                        });
                    }
                }
            }
        }
        for safe in [-2147483648, -1, 1, 2147483647] {
            out.push(Case {
                op: 4,
                available: 0,
                safe,
                ..base
            });
        }
    }
    out
}

#[cfg(target_pointer_width = "64")]
fn reg_to_u64(value: Reg) -> u64 {
    value
}
#[cfg(not(target_pointer_width = "64"))]
fn reg_to_u64(value: Reg) -> u64 {
    u64::from(value)
}

fn observe(case: Case, table: &[HuffmanCode]) -> [u8; 136] {
    let input: [u8; 128] = std::array::from_fn(|i| (i * 73 + 19) as u8);
    let mut destination: [u8; 64] = std::array::from_fn(|i| (i * 37 + 165) as u8);
    let mut br = BitReader::new();
    br.set_input(&input[..case.input]);
    br.val = case.val;
    br.bit_pos = case.available;
    let (mut bits, mut value, mut result) = (5, 4660, 3735928559);
    let mut status = 1i32;
    match case.op {
        0 => result = decode_symbol(case.val, table, &mut br),
        1 => status = i32::from(safe_decode_symbol(table, &mut br, &mut result)),
        2 | 7 => status = i32::from(safe_read_symbol(table, &mut br, &mut result)),
        3 => result = read_symbol(table, &mut br),
        4 => preload_symbol(case.safe, table, &mut br, &mut bits, &mut value),
        5 => {
            preload_symbol(0, table, &mut br, &mut bits, &mut value);
            result = read_preloaded_symbol(table, &mut br, &mut bits, &mut value);
        }
        6 => {
            preload_symbol(0, table, &mut br, &mut bits, &mut value);
            status = copy_preloaded_symbols_to_u8(
                table,
                &mut br,
                &mut bits,
                &mut value,
                &mut destination,
                case.pos,
                case.limit,
            );
        }
        _ => unreachable!(),
    }
    let fields = [
        status as i64 as u64,
        reg_to_u64(result),
        reg_to_u64(br.val),
        reg_to_u64(br.bit_pos),
        br.position() as u64,
        br.guard_position() as u64,
        br.available_in() as u64,
        reg_to_u64(bits),
        reg_to_u64(value),
    ];
    let mut out = [0; 136];
    for (i, word) in fields.into_iter().enumerate() {
        out[i * 8..i * 8 + 8].copy_from_slice(&word.to_le_bytes());
    }
    out[72..].copy_from_slice(&destination);
    out
}

#[test]
fn signed_guard_distance_does_not_become_unsigned_remaining_capacity() {
    let code = [HuffmanCode::new(0, 257); 256];
    let case = Case {
        op: 6,
        table: 0,
        available: 0,
        val: 0,
        input: 8,
        pos: 3,
        limit: 31,
        safe: 0,
    };
    let out = observe(case, &code);
    assert_eq!(u64::from_le_bytes(out[..8].try_into().unwrap()), 0);
    // Initial peek moved next to 7 but guard remains 0: negative difference.
    assert_eq!(u64::from_le_bytes(out[32..40].try_into().unwrap()), 7);
    assert_eq!(u64::from_le_bytes(out[40..48].try_into().unwrap()), 0);
}

#[test]
fn native_corpus_exercises_all_helpers_without_an_oracle_dependency() {
    let tables = tables();
    for case in cases().into_iter().step_by(257) {
        assert!(
            std::panic::catch_unwind(|| observe(case, &tables[case.table])).is_ok(),
            "{case:?}"
        );
    }
}

#[cfg(feature = "cpp-oracle")]
#[test]
fn actual_original_tables_and_every_symbol_observation_match() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let oracle = std::env::var_os("BROTLI_SYMBOLS_ORACLE").expect("BROTLI_SYMBOLS_ORACLE");
    let tables = tables();
    let source = Command::new(&oracle).arg("tables").output().unwrap();
    assert!(
        source.status.success(),
        "{}",
        String::from_utf8_lossy(&source.stderr)
    );
    assert!(source.stderr.is_empty());
    let words: Vec<u64> = source
        .stdout
        .as_chunks::<8>()
        .0
        .iter()
        .map(|word| u64::from_le_bytes(*word))
        .collect();
    let mut cursor = 0;
    for table in &tables {
        assert_eq!(words[cursor], table.len() as u64);
        cursor += 1;
        for code in table {
            assert_eq!(
                &words[cursor..cursor + 2],
                &[u64::from(code.bits), u64::from(code.value)]
            );
            cursor += 2;
        }
    }
    assert_eq!(cursor, words.len());
    let cases = cases();
    let request_path =
        std::env::temp_dir().join(format!("brotli-symbols-{}.requests", std::process::id()));
    let mut requests = std::io::BufWriter::new(std::fs::File::create(&request_path).unwrap());
    for c in &cases {
        writeln!(
            requests,
            "{} {} {} {} {} {} {} {}",
            c.op, c.table, c.available, c.val, c.input, c.pos, c.limit, c.safe
        )
        .unwrap();
    }
    requests.flush().unwrap();
    drop(requests);
    let source = Command::new(&oracle)
        .arg("run")
        .stdin(Stdio::from(std::fs::File::open(&request_path).unwrap()))
        .output()
        .unwrap();
    assert!(
        source.status.success(),
        "{}",
        String::from_utf8_lossy(&source.stderr)
    );
    assert!(source.stderr.is_empty());
    assert_eq!(source.stdout.len(), cases.len() * 136);
    for (case, actual) in cases.iter().zip(source.stdout.as_chunks::<136>().0) {
        assert_eq!(
            observe(*case, &tables[case.table]),
            actual.as_slice(),
            "{case:?}"
        );
    }
    if let Some(directory) = std::env::var_os("BROTLI_SYMBOLS_CAPTURE") {
        let directory = std::path::PathBuf::from(directory);
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::copy(&request_path, directory.join("requests.txt")).unwrap();
        std::fs::write(directory.join("source.bin"), &source.stdout).unwrap();
        std::fs::write(
            directory.join("tables.bin"),
            words
                .iter()
                .flat_map(|v| v.to_le_bytes())
                .collect::<Vec<_>>(),
        )
        .unwrap();
        std::fs::write(directory.join("records.txt"), format!("{}\n", cases.len())).unwrap();
    }
    std::fs::remove_file(request_path).unwrap();
}
