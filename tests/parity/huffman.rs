// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Boundary adapters and deterministic corpora for original Huffman routines.

use super::*;
use jxl::jxl::dec_huffman::HuffmanDecodingData;
use jxl::jxl::enc_huffman::build_and_store_huffman_tree;
use jxl::jxl::enc_huffman_tree::{
    convert_bit_depths_to_symbols, create_huffman_tree, write_huffman_tree,
};
use jxl::jxl::huffman_table::{HuffmanCode, build_huffman_table};

fn table_observation(result: &mut String, table: &[HuffmanCode]) {
    write!(result, " {}", table.len()).unwrap();
    for entry in table {
        write!(result, " {},{}", entry.bits, entry.value).unwrap();
    }
}

fn read_observation(
    result: &mut String,
    data: &mut HuffmanDecodingData,
    alphabet: usize,
    reader: &mut BitReader<'_>,
    num_symbols: usize,
) {
    let ok = data.read_from_bit_stream(alphabet, reader);
    write!(
        result,
        " {} {}",
        u32::from(ok),
        reader.total_bits_consumed()
    )
    .unwrap();
    table_observation(result, &data.table);
    if ok {
        for _ in 0..num_symbols {
            reader.refill();
            write!(result, " {}", data.read_symbol(reader)).unwrap();
        }
    }
    write!(
        result,
        " {} {}",
        reader.total_bits_consumed(),
        status_code(&reader.close())
    )
    .unwrap();
}

fn encode(
    histogram: &[u32],
    offset: usize,
    symbols: &[u16],
) -> (Status, usize, usize, BitWriter, Vec<u8>, Vec<u16>) {
    let mut depth = vec![0u8; histogram.len()];
    let mut bits = vec![0u16; histogram.len()];
    let mut writer = BitWriter::new();
    prefix(&mut writer, offset);
    let mut tree_bits = 0;
    let status = writer.with_max_bits(
        32 * histogram.len() + 256 + 15 * symbols.len(),
        LayerType::Header,
        None,
        |w, _| {
            build_and_store_huffman_tree(histogram, &mut depth, &mut bits, w)?;
            tree_bits = w.bits_written();
            for &symbol in symbols {
                let i = usize::from(symbol);
                w.write(usize::from(depth[i]), u64::from(bits[i]));
            }
            Ok(())
        },
        false,
    );
    let total_bits = writer.bits_written();
    pad(&mut writer);
    (status, tree_bits, total_bits, writer, depth, bits)
}

pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "huffman" => {
            let (length, offset): (usize, usize) = (number(args), number(args));
            let histogram: Vec<u32> = (0..length).map(|_| number(args)).collect();
            let num_symbols: usize = number(args);
            let symbols: Vec<u16> = (0..num_symbols).map(|_| number(args)).collect();
            let (status, tree_bits, total_bits, writer, depth, bits) =
                encode(&histogram, offset, &symbols);
            let mut result = format!(
                "{} {tree_bits} {total_bits} {} {length}",
                status_code(&status),
                hex(writer.get_span())
            );
            for i in 0..length {
                write!(result, " {},{}", depth[i], bits[i]).unwrap();
            }
            let mut data = HuffmanDecodingData::default();
            let mut reader = BitReader::new(writer.get_span());
            reader.skip_bits(offset);
            read_observation(&mut result, &mut data, length, &mut reader, num_symbols);
            result
        }
        "huffman-read" => {
            let alphabet = number(args);
            let bytes = unhex(args.next().unwrap());
            let (offset, seed_size, num_symbols) = (number(args), number(args), number(args));
            let mut data = HuffmanDecodingData {
                table: (0..seed_size)
                    .map(|i: usize| HuffmanCode {
                        bits: 3,
                        value: (i * 17) as u16,
                    })
                    .collect(),
            };
            let mut reader = BitReader::new(&bytes);
            reader.skip_bits(offset);
            let mut result = String::new();
            read_observation(&mut result, &mut data, alphabet, &mut reader, num_symbols);
            result
        }
        "huffman-rle" => {
            let (length, initial): (usize, usize) = (number(args), number(args));
            let depth: Vec<u8> = (0..length).map(|_| number(args)).collect();
            let mut tree = vec![0xA5; length + initial];
            let mut extra = vec![0x5A; length + initial];
            let mut size = initial;
            write_huffman_tree(&depth, &mut size, &mut tree, &mut extra);
            let mut result = size.to_string();
            for i in 0..size {
                write!(result, " {},{}", tree[i], extra[i]).unwrap();
            }
            result
        }
        "huffman-tree" => {
            let (length, limit): (usize, i32) = (number(args), number(args));
            let histogram: Vec<u32> = (0..length).map(|_| number(args)).collect();
            let mut depth = vec![0; length];
            let mut bits = vec![0; length];
            create_huffman_tree(&histogram, limit, &mut depth);
            convert_bit_depths_to_symbols(&depth, &mut bits);
            let mut result = length.to_string();
            for i in 0..length {
                write!(result, " {},{}", depth[i], bits[i]).unwrap();
            }
            result
        }
        "huffman-table" => {
            let (root_bits, length): (usize, usize) = (number(args), number(args));
            let depth: Vec<u8> = (0..length).map(|_| number(args)).collect();
            let mut counts = [0u16; 16];
            for &d in &depth {
                counts[usize::from(d)] += 1;
            }
            let mut table = vec![HuffmanCode::default(); 1 << 16];
            let size = build_huffman_table(&mut table, root_bits, &depth, &mut counts);
            let mut result = size.to_string();
            for count in counts {
                write!(result, " {count}").unwrap();
            }
            table.truncate(size as usize);
            table_observation(&mut result, &table);
            result
        }
        _ => panic!("unknown Huffman command: {kind}"),
    }
}

fn histogram(length: usize, trial: usize, seed: &mut u64) -> Vec<u32> {
    let mut counts = vec![0; length];
    match trial {
        0 => {}
        1 => counts[length - 1] = 7,
        2 => counts.fill(1),
        3 => {
            for value in &mut counts {
                *value = (next(seed) % 10000 + 1) as u32;
            }
        }
        4 => {
            for value in counts.iter_mut().step_by(7) {
                *value = 3;
            }
        }
        5..=10 => {
            let n = (2 + (trial - 5) / 2).min(length);
            for i in 0..n {
                counts[i * (length - 1) / (n - 1).max(1)] =
                    if trial % 2 == 0 && i == 0 { 10000 } else { 1 };
            }
        }
        11 => {
            let (mut a, mut b) = (1, 1);
            for value in counts.iter_mut().take(32) {
                *value = a;
                (a, b) = (b, a + b);
            }
        }
        12 => {
            counts.fill(1);
            counts[length / 2] = 1 << 28;
        }
        _ => {
            for value in &mut counts {
                *value = if next(seed) % 4 == 0 {
                    0
                } else {
                    (next(seed) % 4 + 1) as u32
                };
            }
        }
    }
    counts
}

fn request<T: std::fmt::Display>(prefix: String, values: &[T]) -> String {
    let mut line = prefix;
    for value in values {
        write!(line, " {value}").unwrap();
    }
    line
}

#[test]
fn huffman_histograms_bytes_tables_and_symbols_match_cpp() {
    let mut cases = Vec::new();
    let mut seed = 192837;
    for length in [
        1, 2, 3, 4, 5, 7, 18, 32, 49, 50, 51, 64, 256, 257, 512, 1024, 4096,
    ] {
        for trial in 0..14 {
            let counts = histogram(length, trial, &mut seed);
            let symbols: Vec<u16> = counts
                .iter()
                .enumerate()
                .filter(|&(_, &count)| count != 0)
                .map(|(i, _)| i as u16)
                .collect();
            for offset in 0..8 {
                let line = request(format!("huffman {length} {offset}"), &counts);
                cases.push(request(format!("{line} {}", symbols.len()), &symbols));
            }
        }
    }
    // Unsigned population sums deliberately wrap in the original tree pool.
    for counts in [vec![u32::MAX; 2], vec![u32::MAX, 1, 1], vec![1 << 31; 4]] {
        let symbols: Vec<u16> = (0..counts.len() as u16).collect();
        for offset in 0..8 {
            let line = request(format!("huffman {} {offset}", counts.len()), &counts);
            cases.push(request(format!("{line} {}", symbols.len()), &symbols));
        }
    }
    compare(cases);
}

#[test]
fn huffman_depth_limits_ties_and_canonical_symbols_match_cpp() {
    let mut cases = Vec::new();
    let mut seed = 65432;
    for length in [1, 2, 5, 18, 32, 51, 256, 4096] {
        for limit in [1, 2, 3, 5, 8, 15] {
            for trial in 1..14 {
                let counts = histogram(length, trial, &mut seed);
                let n = counts.iter().filter(|&&c| c != 0).count();
                if n == 0 || n > 1 << limit {
                    continue;
                }
                cases.push(request(format!("huffman-tree {length} {limit}"), &counts));
            }
        }
    }
    cases.extend(["huffman-tree 1 0 4294967295", "huffman-tree 1 -1 1"].map(str::to_owned));
    for counts in [vec![u32::MAX; 2], vec![u32::MAX, 1, 1], vec![1 << 31; 4]] {
        cases.push(request(
            format!("huffman-tree {} 15", counts.len()),
            &counts,
        ));
    }
    compare(cases);
}

#[test]
fn huffman_rle_boundaries_and_nonempty_output_prefix_match_cpp() {
    let mut cases = Vec::new();
    let mut seed = 8292;
    for length in [0, 1, 2, 3, 7, 8, 11, 12, 49, 50, 51, 64, 128, 256, 4096] {
        for run in [1, 2, 3, 4, 7, 8, 10, 11, 12, 31, 255, 1024, 4096] {
            for trial in 0..4 {
                let depth: Vec<u8> = (0..length)
                    .map(|i| match trial {
                        0 => 8,
                        1 => {
                            if i / run % 2 == 0 {
                                0
                            } else {
                                15
                            }
                        }
                        2 => {
                            if i / run % 2 == 0 {
                                1
                            } else {
                                2
                            }
                        }
                        _ => (next(&mut seed) % 16) as u8,
                    })
                    .collect();
                cases.push(request(
                    format!("huffman-rle {length} {}", trial % 3),
                    &depth,
                ));
            }
        }
    }
    compare(cases);
}

#[test]
fn huffman_lookup_tables_and_consumed_counts_match_cpp() {
    let mut cases = Vec::new();
    let mut seed = 19221;
    for length in [1, 2, 3, 4, 5, 17, 18, 31, 32, 257, 512, 4096] {
        let mut depth = if length == 1 { vec![1] } else { vec![1, 1] };
        while depth.len() < length {
            let i = next(&mut seed) as usize % depth.len();
            if depth[i] == 15 {
                continue;
            }
            depth[i] += 1;
            depth.push(depth[i]);
        }
        let mut sparse = Vec::new();
        for &d in &depth {
            sparse.push(d);
            if next(&mut seed) % 4 == 0 {
                sparse.push(0);
            }
        }
        for values in [&depth, &sparse] {
            for root in [1, 5, 8, 15] {
                cases.push(request(
                    format!("huffman-table {root} {}", values.len()),
                    values,
                ));
            }
        }
    }
    cases.push(request(
        "huffman-table 8 32768".to_owned(),
        &vec![15; 32768],
    ));
    cases.push(request("huffman-table 8 32769".to_owned(), &vec![0; 32769]));
    compare(cases);
}

#[test]
fn huffman_truncations_corruption_and_reused_table_state_match_cpp() {
    let mut cases = Vec::new();
    let mut seed = 126543;
    for alphabet in [0, 1, 2, 3, 4, 5, 18, 51, 256, 4096, 32768, 32769] {
        for offset in 0..8 {
            for length in 0..24 {
                let bytes: Vec<u8> = (0..length).map(|_| next(&mut seed) as u8).collect();
                cases.push(format!(
                    "huffman-read {alphabet} {} {offset} {} 9",
                    hex(&bytes),
                    if length % 2 == 0 { 512 } else { 0 }
                ));
            }
        }
    }
    for alphabet in [1, 2, 3, 4, 5, 18, 51, 256, 4096] {
        for trial in [1, 2, 3, 4, 6, 8, 10, 11, 12] {
            let counts = histogram(alphabet, trial, &mut seed);
            for offset in 0..8 {
                let (_, _, _, writer, _, _) = encode(&counts, offset, &[]);
                let bytes = writer.get_span();
                for length in 0..=bytes.len() {
                    cases.push(format!(
                        "huffman-read {alphabet} {} {offset} 512 9",
                        hex(&bytes[..length])
                    ));
                }
                for _ in 0..8 {
                    let mut damaged = bytes.to_vec();
                    let i = next(&mut seed) as usize % (damaged.len() * 8);
                    damaged[i / 8] ^= 1 << (i % 8);
                    cases.push(format!(
                        "huffman-read {alphabet} {} {offset} 512 9",
                        hex(&damaged)
                    ));
                }
            }
        }
    }
    compare(cases);
}
