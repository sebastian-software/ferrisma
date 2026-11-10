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

use bit_reader::{BitReader, Reg};
use huffman::HuffmanCode;
use huffman_reader::*;
use layout::{DecoderStateStorage, HeaderArenaStorage, HuffmanCodeStorage};
use memory::{Allocation, Allocator};
use running::*;
use status::*;
use std::ffi::c_void;
use std::mem::{MaybeUninit, align_of, offset_of, size_of};

struct Context {
    seed: Option<u8>,
    size: usize,
    freed: usize,
}

unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: test context remains live through matching free; callback is not
    // called while the block is borrowed. No block exists before this call.
    let context = unsafe { &mut *opaque.cast::<Context>() };
    let layout = std::alloc::Layout::from_size_align(size, 16).unwrap();
    // SAFETY: nonzero source storage request and valid source-compatible alignment.
    let ptr = unsafe { std::alloc::alloc(layout) };
    if !ptr.is_null() {
        if let Some(seed) = context.seed {
            // SAFETY: actual callback-returned writable block spans exactly size.
            unsafe { ptr.write_bytes(seed, size) };
        }
        context.size = size;
    }
    ptr.cast()
}
unsafe extern "C" fn free(opaque: *mut c_void, block: *mut c_void) {
    if block.is_null() {
        return;
    }
    // SAFETY: original context and exactly-once callback owner remain live;
    // all header/table views have ended before this matching free checkpoint.
    let context = unsafe { &mut *opaque.cast::<Context>() };
    let layout = std::alloc::Layout::from_size_align(context.size, 16).unwrap();
    // SAFETY: block was produced by allocate with this exact layout.
    unsafe { std::alloc::dealloc(block.cast(), layout) };
    context.freed += 1;
}
fn owner(context: &mut Context) -> Allocation {
    // SAFETY: caller retains the context until the returned owner is dropped;
    // test callbacks provide actual exclusive aligned storage and do not reenter.
    let allocator = unsafe {
        Allocator::from_callbacks(
            Some(allocate),
            Some(free),
            std::ptr::from_mut(context).cast(),
        )
    }
    .unwrap();
    allocator
        .allocate(size_of::<DecoderStateStorage>())
        .unwrap()
}
fn header(owner: &mut Allocation) -> &mut HeaderArenaStorage {
    let storage = owner.storage_mut();
    let offset = offset_of!(DecoderStateStorage, arena);
    assert!(offset + size_of::<HeaderArenaStorage>() <= storage.len());
    // SAFETY: source-aligned callback block contains actual arena storage at
    // this verified repr(C) offset. HeaderArenaStorage consists exclusively
    // of MaybeUninit fields, so creating this view initializes/reads no payload.
    unsafe {
        &mut *storage
            .as_mut_ptr()
            .cast::<u8>()
            .add(offset)
            .cast::<HeaderArenaStorage>()
    }
}
fn checkpoint(header: &mut HeaderArenaStorage) {
    // Exact header portion of actual decode.c2614 source checkpoint; source
    // observer executes that checkpoint through the original stream decoder.
    header.sub_loop_counter.write(0);
    let origin = header
        .symbols_lists_array
        .as_ptr()
        .cast::<u16>()
        .wrapping_add(16);
    header.symbol_lists.write(origin as usize);
    header.substate_huffman.write(BROTLI_STATE_HUFFMAN_NONE);
    header
        .substate_tree_group
        .write(BROTLI_STATE_TREE_GROUP_NONE);
    header
        .substate_context_map
        .write(BROTLI_STATE_CONTEXT_MAP_NONE);
}

unsafe fn read_external(
    max: Reg,
    limit: Reg,
    table: &mut [MaybeUninit<HuffmanCode>],
    out: Option<&mut MaybeUninit<Reg>>,
    br: &mut BitReader<'_>,
    header: &mut HeaderArenaStorage,
) -> BrotliDecoderErrorCode {
    let (mut fields, _) = split_header(header);
    // SAFETY: caller retains the same source initialization/stable-owner and
    // disjoint external-output preconditions as the production resume boundary.
    unsafe { huffman_reader::read_huffman_code(max, limit, table, out, br, &mut fields) }
}

#[test]
fn source_selector_resume_does_not_initialize_unreached_fields_or_output() {
    let mut context = Context {
        seed: None,
        size: 0,
        freed: 0,
    };
    let mut allocation = owner(&mut context);
    let h = header(&mut allocation);
    checkpoint(h);
    let mut br = BitReader::new();
    let mut table = [MaybeUninit::uninit(); 264];
    let mut table_size = MaybeUninit::uninit();
    // Genuine encoder fixture0 is byte0x11, alpha16 singleton symbol1. Split
    // its selector low2 bits from the remainder; no encoded output is invented.
    br.val = 1;
    br.bit_pos = 2;
    // SAFETY: source NONE checkpoint above, real owner stable, disjoint
    // reader/output; only source-written fields are subsequently observed.
    assert_eq!(
        unsafe { read_external(16, 16, &mut table, Some(&mut table_size), &mut br, h) },
        BROTLI_DECODER_NEEDS_MORE_INPUT
    );
    // SAFETY: failed size read wrote SIMPLE_SIZE and prior selector wrote1.
    assert_eq!(
        unsafe { h.substate_huffman.assume_init_read() },
        BROTLI_STATE_HUFFMAN_SIMPLE_SIZE
    );
    assert_eq!(unsafe { h.sub_loop_counter.assume_init_read() }, 1);
    br.set_input(&[0x04]);
    // SAFETY: same actual owner and exact partial checkpoint from previous call.
    assert_eq!(
        unsafe { read_external(16, 16, &mut table, Some(&mut table_size), &mut br, h) },
        BROTLI_DECODER_SUCCESS
    );
    // SAFETY: success initialized table_size and the singleton builder wrote
    // bits/value in every slot0..256. Padding and unwritten tail remain unread.
    assert_eq!(unsafe { table_size.assume_init_read() }, 256);
    for code in &table[..256] {
        assert_eq!(unsafe { code.assume_init_read() }, HuffmanCode::new(0, 1));
    }
    drop(allocation);
    assert_eq!(context.freed, 1);
}

#[test]
fn unknown_substate_reads_only_the_initialized_discriminant() {
    let mut context = Context {
        seed: None,
        size: 0,
        freed: 0,
    };
    let mut allocation = owner(&mut context);
    let h = header(&mut allocation);
    h.substate_huffman.write(u32::MAX);
    let mut br = BitReader::new();
    let mut table = [MaybeUninit::uninit(); 1];
    // SAFETY: unknown source state reaches only initialized discriminant and
    // default error; pointer token/counters/arrays/output all remain unknown.
    assert_eq!(
        unsafe { read_external(16, 16, &mut table, None, &mut br, h) },
        BROTLI_DECODER_ERROR_UNREACHABLE
    );
}

#[test]
fn genuine_complex_headers_use_only_source_initialized_arena_slots() {
    // Exact native encoder fixture5 (static code-length alphabet) and fixture6
    // (dynamic alphabet), alpha8 with uniform symbol populations. Feature
    // parity verifies both byte sequences directly against actual encoder output.
    for input in [
        &[0x54, 0x55, 0x55, 0x55, 0xff, 0xbc, 0x2c][..],
        &[0x1e, 0, 0, 0, 0][..],
    ] {
        let mut context = Context {
            seed: None,
            size: 0,
            freed: 0,
        };
        let mut allocation = owner(&mut context);
        let h = header(&mut allocation);
        checkpoint(h);
        let mut br = BitReader::new();
        let mut table = [MaybeUninit::uninit(); 264];
        let mut size = MaybeUninit::uninit();
        let mut result = BROTLI_DECODER_NEEDS_MORE_INPUT;
        for byte in input {
            br.set_input(std::slice::from_ref(byte));
            // SAFETY: exact source NONE/partial-resume checkpoint on actual
            // unknown backing. The source's writes initialize histo prefix6,
            // length bytes, table fields, then histo16/next16/negative heads.
            result = unsafe { read_external(8, 8, &mut table, Some(&mut size), &mut br, h) };
            if result != BROTLI_DECODER_NEEDS_MORE_INPUT {
                break;
            }
        }
        assert_eq!(result, BROTLI_DECODER_SUCCESS);
        // SAFETY: completed original builders initialize size and all fields
        // in root table[0..256]. No unwritten array tail or padding is observed.
        assert_eq!(unsafe { size.assume_init_read() }, 256);
        let symbols = [0, 4, 2, 6, 1, 5, 3, 7];
        for (i, code) in table[..256].iter().enumerate() {
            assert_eq!(
                unsafe { code.assume_init_read() },
                HuffmanCode::new(3, symbols[i % 8])
            );
        }
    }
}

#[test]
fn simple_alphabet_error_does_not_read_unwritten_symbol_storage() {
    let mut context = Context {
        seed: None,
        size: 0,
        freed: 0,
    };
    let mut allocation = owner(&mut context);
    let h = header(&mut allocation);
    checkpoint(h);
    let mut br = BitReader::new();
    br.set_input(&[0x11]);
    let mut table = [MaybeUninit::uninit(); 264];
    let mut out = MaybeUninit::uninit();
    // SAFETY: actual source NONE state and genuine singleton fixture; reduced
    // alphabet limit makes symbol1 invalid before the first symbol-list write.
    assert_eq!(
        unsafe { read_external(16, 1, &mut table, Some(&mut out), &mut br, h) },
        BROTLI_DECODER_ERROR_FORMAT_SIMPLE_HUFFMAN_ALPHABET
    );
    // Unwritten words, output table and optional size are deliberately not read.
}

#[test]
fn repeated_zero_lengths_and_overflow_keep_source_scalar_mutations() {
    let mut symbol = 7;
    let mut repeat = 9;
    let mut space = 32768;
    let mut prev = 8;
    let mut repeat_len = 3;
    let mut lists = [MaybeUninit::uninit(); 720];
    let mut histo = [0u16; 16];
    let mut next: [i32; 16] = std::array::from_fn(|i| i as i32 - 16);
    process_repeated_code_length(
        17,
        5,
        704,
        &mut symbol,
        &mut repeat,
        &mut space,
        &mut prev,
        &mut repeat_len,
        &mut lists,
        &mut histo,
        &mut next,
    );
    assert_eq!(
        (symbol, repeat, space, prev, repeat_len),
        (15, 8, 32768, 8, 0)
    );
    process_repeated_code_length(
        17,
        7,
        20,
        &mut symbol,
        &mut repeat,
        &mut space,
        &mut prev,
        &mut repeat_len,
        &mut lists,
        &mut histo,
        &mut next,
    );
    assert_eq!(
        (symbol, repeat, space, prev, repeat_len),
        (20, 58, 0xfffff, 8, 0)
    );
    assert_eq!(histo, [0; 16]);
    // No list slot was written in either zero-code path; none is read.
}

#[test]
fn same_owner_context_map_output_and_working_fields_are_disjoint_borrows() {
    let mut context = Context {
        seed: None,
        size: 0,
        freed: 0,
    };
    let mut allocation = owner(&mut context);
    let h = header(&mut allocation);
    checkpoint(h);
    let (mut fields, context_table) = split_header(h);
    let output = context_map_output(context_table);
    let mut br = BitReader::new();
    br.set_input(&[0x1e, 0, 0, 0, 0]);
    // SAFETY: genuine encoder dynamic-header fixture6, source NONE checkpoint,
    // stable actual owner. Safe splitter supplies disjoint source working
    // fields and context-map output, mirroring actual decode.c1101's caller.
    assert_eq!(
        unsafe { huffman_reader::read_huffman_code(8, 8, output, None, &mut br, &mut fields) },
        BROTLI_DECODER_SUCCESS
    );
    let symbols = [0, 4, 2, 6, 1, 5, 3, 7];
    for (i, code) in output[..256].iter().enumerate() {
        // SAFETY: successful complete builder initialized these exact fields;
        // padding and all untouched tail entries remain unobserved.
        assert_eq!(
            unsafe { code.assume_init_read() },
            HuffmanCode::new(3, symbols[i % 8])
        );
    }
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
    use std::path::Path;
    use std::process::{Command, Stdio};

    #[derive(Clone)]
    struct Case {
        max: Reg,
        limit: Reg,
        opt: bool,
        initial_bits: Reg,
        initial_val: Reg,
        data: Vec<u8>,
        chunks: Vec<usize>,
        state: u32,
        embedded: bool,
    }
    fn word(bytes: &[u8], cursor: &mut usize) -> u64 {
        let value = u64::from_le_bytes(bytes[*cursor..*cursor + 8].try_into().unwrap());
        *cursor += 8;
        value
    }
    #[cfg(target_pointer_width = "64")]
    fn reg64(value: Reg) -> u64 {
        value
    }
    #[cfg(not(target_pointer_width = "64"))]
    fn reg64(value: Reg) -> u64 {
        u64::from(value)
    }
    type Fixture = (Reg, usize, Vec<u8>);
    fn fixtures(oracle: &std::ffi::OsStr) -> (Vec<Fixture>, Vec<u8>) {
        let process = Command::new(oracle).arg("fixtures").output().unwrap();
        assert!(
            process.status.success(),
            "{}",
            String::from_utf8_lossy(&process.stderr)
        );
        assert!(process.stderr.is_empty());
        let mut cursor = 0;
        let n = word(&process.stdout, &mut cursor);
        assert_eq!(n, 50);
        let mut out = Vec::new();
        for _ in 0..n {
            let alpha = word(&process.stdout, &mut cursor) as Reg;
            let bits = word(&process.stdout, &mut cursor) as usize;
            let n = word(&process.stdout, &mut cursor) as usize;
            out.push((alpha, bits, process.stdout[cursor..cursor + n].to_vec()));
            cursor += n;
        }
        assert_eq!(cursor, process.stdout.len());
        (out, process.stdout)
    }
    fn cases(fixtures: &[Fixture]) -> Vec<Case> {
        let mut out = Vec::new();
        for (fixture, (max, _, original)) in fixtures.iter().enumerate() {
            for initial_bits in 0..8 {
                let initial_val = Reg::from(original[0]) & ((1 << initial_bits) - 1);
                let mut data = Vec::new();
                for i in 0..original.len() {
                    let pair = u16::from(original[i])
                        | (u16::from(*original.get(i + 1).unwrap_or(&0)) << 8);
                    data.push((pair >> initial_bits) as u8);
                }
                let base = Case {
                    max: *max,
                    limit: (*max).min(704),
                    opt: true,
                    initial_bits,
                    initial_val,
                    data: data.clone(),
                    chunks: vec![],
                    state: 0,
                    embedded: false,
                };
                for n in 0..=data.len() {
                    out.push(Case {
                        chunks: vec![n],
                        ..base.clone()
                    });
                }
                for chunk in [1, 2, 3, 7] {
                    let mut chunks = vec![0];
                    let mut left = data.len();
                    while left > 0 {
                        let n = left.min(chunk);
                        chunks.push(n);
                        left -= n;
                    }
                    chunks.push(0);
                    out.push(Case {
                        chunks,
                        ..base.clone()
                    });
                }
                let mut padded = data;
                padded.extend([0; 64]);
                out.push(Case {
                    chunks: vec![padded.len()],
                    data: padded,
                    opt: initial_bits % 2 == 0,
                    ..base
                });
            }
            let base = Case {
                max: *max,
                limit: (*max).min(704),
                opt: true,
                initial_bits: 0,
                initial_val: 0,
                data: original.clone(),
                chunks: vec![original.len()],
                state: 0,
                embedded: false,
            };
            for limit in [0, 1, base.limit - 1] {
                out.push(Case {
                    limit,
                    ..base.clone()
                });
            }
            if fixture < 5 {
                for index in 0..original.len() {
                    for value in 0..=255 {
                        let mut data = original.clone();
                        data[index] = value;
                        out.push(Case {
                            data,
                            ..base.clone()
                        });
                    }
                }
            } else {
                for bit in 0..(original.len() * 8).min(128) {
                    let mut data = original.clone();
                    data[bit / 8] ^= 1 << (bit % 8);
                    out.push(Case {
                        data,
                        ..base.clone()
                    });
                }
                for index in 0..original.len().min(4) {
                    for value in [0, 1, 255] {
                        let mut data = original.clone();
                        data[index] = value;
                        out.push(Case {
                            data,
                            ..base.clone()
                        });
                    }
                }
            }
            out.push(Case {
                state: u32::MAX,
                ..base
            });
        }
        for (max, _, data) in fixtures.iter().filter(|(max, _, _)| *max <= 256) {
            let base = Case {
                max: *max,
                limit: *max,
                opt: false,
                initial_bits: 0,
                initial_val: 0,
                data: data.clone(),
                chunks: vec![data.len()],
                state: 0,
                embedded: true,
            };
            for n in 0..=data.len() {
                out.push(Case {
                    chunks: vec![n],
                    ..base.clone()
                });
            }
            let mut chunks = vec![0];
            chunks.extend(vec![1; data.len()]);
            chunks.push(0);
            out.push(Case {
                chunks,
                ..base.clone()
            });
            let mut padded = data.clone();
            padded.extend([0; 64]);
            out.push(Case {
                chunks: vec![padded.len()],
                data: padded,
                ..base.clone()
            });
            out.push(Case { limit: 0, ..base });
        }
        out
    }
    unsafe fn seeded_header_bytes(h: &HeaderArenaStorage, fields: &[(usize, usize)]) -> Vec<u8> {
        let mut out = Vec::with_capacity(3756);
        let base = std::ptr::from_ref(h).cast::<u8>();
        for &(offset, size) in fields {
            if offset == offset_of!(HeaderArenaStorage, symbol_lists) {
                // SAFETY: genuine source checkpoint initialized this pointer;
                // only after that checkpoint is owner-relative normalization legal.
                assert_eq!(
                    unsafe { h.symbol_lists.assume_init_read() },
                    h.symbols_lists_array
                        .as_ptr()
                        .cast::<u16>()
                        .wrapping_add(16) as usize
                );
                out.extend(16u64.to_le_bytes());
            } else {
                // SAFETY: test callback seeded every actual field byte before
                // the reader; all reader scalar/array writes store initialized
                // bytes. Schema excludes every padding byte, including padding
                // replaced by a HuffmanCode assignment. No logical unknown byte
                // claim is inferred from this physical fixture initialization.
                out.extend(unsafe { std::slice::from_raw_parts(base.add(offset), size) });
            }
        }
        out
    }
    fn observe(case: &Case, seed: u8, fields: &[(usize, usize)]) -> Vec<Vec<u8>> {
        let mut context = Context {
            seed: Some(seed),
            size: 0,
            freed: 0,
        };
        let mut allocation = owner(&mut context);
        let h = header(&mut allocation);
        checkpoint(h);
        h.substate_huffman.write(case.state);
        let mut table = vec![
            MaybeUninit::new(HuffmanCode::new(
                seed,
                if seed == 0xa5 { 48879 } else { 4660 }
            ));
            1152
        ];
        let mut table_size = MaybeUninit::new(3735928559);
        let mut br = BitReader::new();
        br.val = case.initial_val;
        br.bit_pos = case.initial_bits;
        let mut offset = 0;
        let mut records = Vec::new();
        for &chunk in &case.chunks {
            br.set_input(&case.data[offset..offset + chunk]);
            // SAFETY: real source checkpoint and subsequent exact resume state
            // in the stable actual owner. Table/input/optional output disjoint;
            // generated/malformed data stay in defined source alphabet domains.
            let result = unsafe {
                if case.embedded {
                    let (mut fields, context_table) = split_header(h);
                    huffman_reader::read_huffman_code(
                        case.max,
                        case.limit,
                        context_map_output(context_table),
                        if case.opt {
                            Some(&mut table_size)
                        } else {
                            None
                        },
                        &mut br,
                        &mut fields,
                    )
                } else {
                    read_external(
                        case.max,
                        case.limit,
                        &mut table,
                        if case.opt {
                            Some(&mut table_size)
                        } else {
                            None
                        },
                        &mut br,
                        h,
                    )
                }
            };
            // SAFETY: seeded scalar output began initialized; source writes a
            // valid scalar on success and otherwise retains that initial value.
            let table_size = unsafe { table_size.assume_init_read() };
            let mut record = Vec::new();
            for field in [
                result as i64 as u64,
                reg64(br.val),
                reg64(br.bit_pos),
                (offset + br.position()) as u64,
                (offset + br.guard_position()) as u64,
                br.available_in() as u64,
                reg64(table_size),
            ] {
                record.extend(field.to_le_bytes());
            }
            // SAFETY: this callback-backed fixture physically seeded all field
            // bytes; exact structural padding exclusions are independently tested.
            record.extend(unsafe { seeded_header_bytes(h, fields) });
            for code in &table {
                // SAFETY: all output bits/value fields began initialized and
                // builder assignments replace them with valid fields; no padding.
                let code = unsafe { code.assume_init_read() };
                record.push(code.bits);
                record.extend(code.value.to_le_bytes());
            }
            records.push(record);
            if result != BROTLI_DECODER_NEEDS_MORE_INPUT {
                break;
            }
            assert_eq!(br.available_in(), 0);
            offset += chunk;
        }
        records
    }

    #[test]
    fn actual_encoder_headers_all_partial_reader_fields_and_tables_match() {
        let oracle =
            std::env::var_os("BROTLI_HUFFMAN_READER_ORACLE").expect("BROTLI_HUFFMAN_READER_ORACLE");
        let process = Command::new(&oracle).arg("schema").output().unwrap();
        assert!(process.status.success());
        assert!(process.stderr.is_empty());
        let mut cursor = 0;
        assert_eq!(
            word(&process.stdout, &mut cursor),
            size_of::<HeaderArenaStorage>() as u64
        );
        assert_eq!(
            word(&process.stdout, &mut cursor),
            align_of::<HeaderArenaStorage>() as u64
        );
        assert_eq!(
            word(&process.stdout, &mut cursor),
            size_of::<DecoderStateStorage>() as u64
        );
        assert_eq!(
            word(&process.stdout, &mut cursor),
            offset_of!(DecoderStateStorage, arena) as u64
        );
        let field_bytes = word(&process.stdout, &mut cursor) as usize;
        assert_eq!(field_bytes, 3756);
        let fields = fields();
        assert_eq!(word(&process.stdout, &mut cursor), fields.len() as u64);
        for &(offset, size) in &fields {
            assert_eq!(word(&process.stdout, &mut cursor), offset as u64);
            assert_eq!(word(&process.stdout, &mut cursor), size as u64);
        }
        assert_eq!(cursor, process.stdout.len());
        let (fixtures, fixture_bytes) = fixtures(&oracle);
        assert_eq!(fixtures[0].2, [0x11]);
        assert_eq!(fixtures[5].2, [0x54, 0x55, 0x55, 0x55, 0xff, 0xbc, 0x2c]);
        assert_eq!(fixtures[6].2, [0x1e, 0, 0, 0, 0]);
        let cases = cases(&fixtures);
        let directory = std::env::var_os("BROTLI_HUFFMAN_READER_CAPTURE")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                std::env::temp_dir().join(format!("brotli-huffman-reader-{}", std::process::id()))
            });
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(directory.join("fixtures.bin"), fixture_bytes).unwrap();
        std::fs::write(directory.join("schema.bin"), &process.stdout).unwrap();
        let mut requests =
            BufWriter::new(std::fs::File::create(directory.join("requests.txt")).unwrap());
        let mut expected =
            BufWriter::new(std::fs::File::create(directory.join("rust.bin")).unwrap());
        let mut spans = Vec::new();
        let mut statuses = std::collections::BTreeMap::new();
        let mut record_count = 0usize;
        for case in &cases {
            write!(
                requests,
                "{} {} {} {} {} {} {} {}",
                case.max,
                case.limit,
                u8::from(case.opt) | (u8::from(case.embedded) << 1),
                case.initial_bits,
                case.initial_val,
                case.chunks.len(),
                case.data.len(),
                case.state
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
            let a = observe(case, 0xa5, &fields);
            let b = observe(case, 0x5a, &fields);
            assert_eq!(a.len(), b.len());
            for (a, b) in a.iter().zip(&b) {
                assert_eq!(&a[..56], &b[..56], "seed-independent status/bitreader/size");
                let status = i64::from_le_bytes(a[..8].try_into().unwrap());
                *statuses.entry(status).or_insert(0usize) += 1;
            }
            spans.push((record_count, a.len()));
            record_count += a.len() + b.len();
            for record in a.into_iter().chain(b) {
                expected.write_all(&record).unwrap();
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
        let mut a = BufReader::new(std::fs::File::open(directory.join("rust.bin")).unwrap());
        let mut b = BufReader::new(std::fs::File::open(directory.join("source.bin")).unwrap());
        let bytes = a.get_ref().metadata().unwrap().len() as usize;
        assert_eq!(bytes as u64, b.get_ref().metadata().unwrap().len());
        let mut offset = 0;
        while offset < bytes {
            let mut ab = [0; 16384];
            let mut bb = [0; 16384];
            let n = (bytes - offset).min(ab.len());
            a.read_exact(&mut ab[..n]).unwrap();
            b.read_exact(&mut bb[..n]).unwrap();
            assert_eq!(&ab[..n], &bb[..n], "byte offset {offset}");
            offset += n;
        }
        let record_bytes = 56 + field_bytes + 1152 * 3;
        assert_eq!(offset, record_count * record_bytes);
        observed_seed_masks(&directory, &spans, record_bytes);
        for required in [1, 2, -4, -5, -6, -7, -31] {
            assert!(
                statuses.contains_key(&required),
                "missing status {required}: {statuses:?}"
            );
        }
        std::fs::write(
            directory.join("counts.txt"),
            format!(
                "{} {} {}\n{statuses:?}\n",
                cases.len(),
                record_count,
                record_bytes
            ),
        )
        .unwrap();
    }
    fn observed_seed_masks(directory: &Path, spans: &[(usize, usize)], record_bytes: usize) {
        use std::io::{Seek, SeekFrom};
        let mut source = std::fs::File::open(directory.join("source.bin")).unwrap();
        let mut masks = BufWriter::new(
            std::fs::File::create(directory.join("observed-seed-independent-byte-masks.bin"))
                .unwrap(),
        );
        for &(start, steps) in spans {
            for step in 0..steps {
                let mut a = vec![0; record_bytes];
                let mut b = vec![0; record_bytes];
                source
                    .seek(SeekFrom::Start(((start + step) * record_bytes) as u64))
                    .unwrap();
                source.read_exact(&mut a).unwrap();
                source
                    .seek(SeekFrom::Start(
                        ((start + steps + step) * record_bytes) as u64,
                    ))
                    .unwrap();
                source.read_exact(&mut b).unwrap();
                assert_eq!(&a[..56], &b[..56]);
                for (a, b) in a[56..].iter().zip(&b[56..]) {
                    masks.write_all(&[u8::from(a == b)]).unwrap();
                }
            }
        }
        masks.flush().unwrap();
        // These masks describe observed seed-independent field bytes. They do
        // not establish initialization, and never justify production typed reads.
    }
}
