// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

use super::*;
use constants::*;
use std::fmt::Write as _;
use std::io::Write as _;
use std::process::{Command, Stdio};

fn number<T: std::str::FromStr>(args: &mut std::str::SplitWhitespace<'_>) -> T {
    args.next().unwrap().parse().ok().unwrap()
}

fn observe(request: &str) -> String {
    let mut args = request.split_whitespace();
    let kind = args.next().unwrap();
    let mut out = String::new();
    if kind == "alphabet" {
        return brotli_distance_alphabet_size(
            number(&mut args),
            number(&mut args),
            number(&mut args),
        )
        .to_string();
    }
    if kind == "distance" {
        let result = brotli_calculate_distance_code_limit(
            number(&mut args),
            number(&mut args),
            number(&mut args),
        );
        return format!("{} {}", result.max_alphabet_size, result.max_distance);
    }
    if kind == "constants" {
        write!(
            out,
            "{} {} {}",
            std::mem::size_of::<HuffmanCode>(),
            std::mem::align_of::<HuffmanCode>(),
            u8::from(cfg!(target_arch = "aarch64"))
        )
        .unwrap();
        for range in BROTLI_PREFIX_CODE_RANGES {
            write!(out, " {} {}", range.offset, range.nbits).unwrap();
        }
        for p in 0..=3 {
            for d in (0..=(15 << p)).step_by(1 << p) {
                write!(
                    out,
                    " {} {}",
                    brotli_distance_alphabet_size(p, d, 24),
                    brotli_distance_alphabet_size(p, d, 62)
                )
                .unwrap();
            }
        }
        for w in 10..=30 {
            write!(out, " {}", brotli_max_backward_limit(w)).unwrap();
        }
        return out;
    }
    let table = if kind == "simple" {
        let root: usize = number(&mut args);
        let num = number(&mut args);
        let mut val = std::array::from_fn::<_, 4, _>(|_| number(&mut args));
        let mut table = initialized_table((1usize << root) + 8);
        let size = brotli_build_simple_huffman_table(&mut table, root, &mut val, num);
        write!(out, "{size}").unwrap();
        for v in val {
            write!(out, " {v}").unwrap();
        }
        table
    } else {
        let is_general = kind == "general" || kind == "general-accessor";
        let root = if is_general { number(&mut args) } else { 5 };
        let n = if is_general { number(&mut args) } else { 18 };
        let lengths: Vec<u8> = (0..n).map(|_| number(&mut args)).collect();
        let (links, mut count) = symbol_lists(&lengths);
        let mut table = initialized_table(if is_general { n + 384 } else { 40 });
        let size = if kind == "general-accessor" {
            let (words, initialized) = partial_symbol_lists(&lengths);
            brotli_build_huffman_table_with_symbols(
                &mut table,
                root,
                |index| {
                    let slot = 16usize.checked_add_signed(index).unwrap();
                    assert!(initialized[slot], "uninitialized symbol-list read: {index}");
                    // SAFETY: the independent per-slot metadata is set only after
                    // the source head/link write. The check above proves this slot
                    // initialized, and the borrowed backing is live and immutable.
                    unsafe { words[slot].assume_init() }
                },
                &mut count,
            )
        } else if is_general {
            brotli_build_huffman_table(&mut table, root, &links, 16, &mut count)
        } else {
            brotli_build_code_lengths_huffman_table(&mut table, &lengths, &mut count);
            32
        };
        write!(out, "{size}").unwrap();
        for v in count {
            write!(out, " {v}").unwrap();
        }
        for v in links {
            write!(out, " {v}").unwrap();
        }
        table
    };
    for code in initialized_codes(&table) {
        write!(out, " {} {}", code.bits, code.value).unwrap();
    }
    out
}

fn compare(cases: Vec<String>) {
    let oracle = std::env::var_os("BROTLI_HUFFMAN_ORACLE")
        .expect("build the original Brotli Huffman oracle and set BROTLI_HUFFMAN_ORACLE");
    let mut child = Command::new(oracle)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let input = cases.join("\n") + "\n";
    let writer = std::thread::spawn(move || stdin.write_all(input.as_bytes()).unwrap());
    let output = child.wait_with_output().unwrap();
    writer.join().unwrap();
    assert!(output.status.success());
    let output = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<_> = output.lines().collect();
    assert_eq!(lines.len(), cases.len());
    for (request, source) in cases.iter().zip(lines) {
        assert_eq!(observe(request), source, "request: {request}");
    }
    eprintln!("Brotli Huffman differential requests: {}", cases.len());
}

fn request(prefix: String, lengths: &[u8]) -> String {
    let mut result = prefix;
    for length in lengths {
        write!(result, " {length}").unwrap();
    }
    result
}

fn next(seed: &mut u32) -> u32 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    *seed
}

#[test]
fn simple_cases_roots_permutations_duplicates_and_defined_unknown_cases_match_source() {
    let mut cases = Vec::new();
    let mut seed = 8312;
    for num in 0..=5 {
        let minimum = [0, 1, 2, 2, 3, 0][num];
        for root in minimum..=8 {
            for trial in 0..24 {
                let val = if trial < 4 {
                    [trial; 4]
                } else {
                    std::array::from_fn(|_| next(&mut seed) as u16)
                };
                cases.push(format!(
                    "simple {root} {num} {} {} {} {}",
                    val[0], val[1], val[2], val[3]
                ));
            }
        }
    }
    compare(cases);
}

#[test]
fn code_length_singletons_complete_and_defined_incomplete_tables_match_source() {
    let mut cases = Vec::new();
    for symbol in 0..18 {
        for length in 1..=5 {
            let mut lengths = [0u8; 18];
            lengths[symbol] = length;
            cases.push(request("lengths".to_owned(), &lengths));
        }
    }
    let mut seed = 7631;
    for trial in 0..200 {
        let mut leaves = vec![0u8];
        for _ in 0..(trial % 17 + 1) {
            let possible: Vec<_> = leaves
                .iter()
                .enumerate()
                .filter(|(_, depth)| **depth < 5)
                .map(|(i, _)| i)
                .collect();
            if possible.is_empty() {
                break;
            }
            let i = possible[next(&mut seed) as usize % possible.len()];
            leaves[i] += 1;
            leaves.push(leaves[i]);
        }
        let mut lengths = [0u8; 18];
        for (i, &depth) in leaves.iter().enumerate() {
            lengths[i] = depth;
        }
        for i in (1..18).rev() {
            lengths.swap(i, next(&mut seed) as usize % (i + 1));
        }
        cases.push(request("lengths".to_owned(), &lengths));
        if trial % 3 == 0 {
            if let Some(i) = lengths.iter().position(|&depth| depth != 0) {
                lengths[i] = 0;
            }
            cases.push(request("lengths".to_owned(), &lengths));
        }
    }
    compare(cases);
}

fn general_cases() -> Vec<String> {
    let mut cases = Vec::new();
    let mut seed = 1461;
    for root in [7, 8] {
        for n in [2, 3, 4, 10, 18, 26, 64, 256, 258, 272, 544, 704] {
            for trial in 0..20 {
                let mut leaves = vec![0u8];
                while leaves.len() < n {
                    let possible: Vec<_> = leaves
                        .iter()
                        .enumerate()
                        .filter(|(_, depth)| **depth < 15)
                        .map(|(i, _)| i)
                        .collect();
                    let i = if trial % 4 == 0 {
                        possible[0]
                    } else {
                        possible[next(&mut seed) as usize % possible.len()]
                    };
                    leaves[i] += 1;
                    leaves.push(leaves[i]);
                }
                for i in (1..n).rev() {
                    leaves.swap(i, next(&mut seed) as usize % (i + 1));
                }
                cases.push(request(format!("general {root} {n}"), &leaves));
            }
        }
        for lengths in [
            vec![1],
            vec![2, 2],
            vec![3, 0, 3],
            vec![15],
            vec![15, 0, 0, 15],
        ] {
            cases.push(request(
                format!("general {root} {}", lengths.len()),
                &lengths,
            ));
        }
        for depth in 1..=15 {
            let n = 1 << depth;
            cases.push(request(format!("general {root} {n}"), &vec![depth; n]));
        }
    }
    cases
}

#[test]
fn general_root_copies_second_level_boundaries_and_maximum_depth_match_source() {
    compare(general_cases());
}

#[test]
fn partially_initialized_symbol_arena_getter_matches_every_original_general_case() {
    compare(
        general_cases()
            .into_iter()
            .map(|request| request.replacen("general ", "general-accessor ", 1))
            .collect(),
    );
}

#[test]
fn constants_distance_direct_first_group_and_unsigned_boundaries_match_source() {
    let mut cases = vec!["constants".to_owned()];
    for p in 0..=3 {
        for d in (0..=(15 << p)).step_by(1 << p) {
            for distance in [
                0,
                1,
                2,
                3,
                4,
                5,
                7,
                8,
                15,
                16,
                31,
                32,
                63,
                64,
                127,
                128,
                255,
                256,
                1023,
                1024,
                65535,
                0x3fffffc,
                0x7ffffffc,
                u32::MAX - 4,
            ] {
                cases.push(format!("distance {distance} {p} {d}"));
            }
        }
    }
    compare(cases);
}

#[test]
fn alphabet_unsigned_increment_and_result_wrapping_match_original_macro() {
    let mut cases = Vec::new();
    for postfix in (0u32..=30).chain(std::iter::once(u32::MAX)) {
        for direct in [0u32, 1, 15, 120, u32::MAX - 16, u32::MAX - 15, u32::MAX] {
            for bits in [0u32, 1, 24, 62, u32::MAX] {
                cases.push(format!("alphabet {postfix} {direct} {bits}"));
            }
        }
    }
    compare(cases);
}
