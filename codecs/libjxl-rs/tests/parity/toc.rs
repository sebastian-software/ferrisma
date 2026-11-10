// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Original-source permutation/section-table comparisons and generated corpora.

use super::*;
use jxl::jxl::coeff_order::{coeff_order_context, decode_permutation};
use jxl::jxl::coeff_order_fwd::*;
use jxl::jxl::enc_coeff_order::encode_permutation;
use jxl::jxl::enc_toc::{write_toc_permutation, write_toc_sizes};
use jxl::jxl::lehmer_code::{PermutationWord, compute_lehmer_code, decode_lehmer_code};
use jxl::jxl::toc::{ac_group_index, max_bits, num_toc_entries, read_group_offsets, read_toc};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("toc-")
}
fn values<T: std::fmt::Display>(out: &mut String, values: &[T]) {
    write!(out, " {}", values.len()).unwrap();
    for v in values {
        write!(out, " {v}").unwrap();
    }
}
fn aux(out: &mut String, aux: &AuxOut) {
    for layer in [LayerType::Header, LayerType::Toc] {
        let a = aux.layer(layer);
        write!(
            out,
            " {},{},{}",
            a.total_bits, a.histogram_bits, a.extra_bits
        )
        .unwrap();
    }
}
fn lehmer<T: PermutationWord + std::fmt::Display>(
    args: &mut std::str::SplitWhitespace<'_>,
    decode: bool,
) -> String {
    let n: usize = number(args);
    let mut permutation = vec![T::from_index(55); n];
    let mut code = vec![33; n];
    let padded = n.max(1).next_power_of_two();
    let mut temp = vec![77; if decode { padded } else { n + 1 }];
    let status = if decode {
        for v in &mut code {
            *v = number(args);
        }
        decode_lehmer_code(&code, &mut temp, n, &mut permutation)
    } else {
        for v in &mut permutation {
            *v = T::from_index(number(args));
        }
        compute_lehmer_code(&permutation, &mut temp, n, &mut code)
    };
    let mut out = status_code(&status).to_string();
    values(&mut out, &permutation);
    values(&mut out, &code);
    values(&mut out, &temp);
    out
}
fn read_observation(n: usize, offset: usize, bytes: &[u8]) -> String {
    let mut sizes = vec![7, 8];
    let mut permutation = vec![2, 0, 1];
    let mut br = BitReader::new(bytes);
    br.skip_bits(offset);
    let read = read_toc(
        jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
        n,
        &mut br,
        &mut sizes,
        &mut permutation,
    );
    let mut out = format!(
        "{} {} {}",
        status_code(&read),
        br.total_bits_consumed(),
        status_code(&br.close())
    );
    values(&mut out, &sizes);
    values(&mut out, &permutation);
    sizes = vec![7, 8];
    let mut offsets = vec![99, 88];
    let mut total = 77;
    let mut br = BitReader::new(bytes);
    br.skip_bits(offset);
    let read = read_group_offsets(
        jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
        n,
        &mut br,
        &mut offsets,
        &mut sizes,
        Some(&mut total),
    );
    write!(
        out,
        " {} {} {} {total}",
        status_code(&read),
        br.total_bits_consumed(),
        status_code(&br.close())
    )
    .unwrap();
    values(&mut out, &sizes);
    values(&mut out, &offsets);
    out
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "toc-lehmer16" => lehmer::<u16>(args, false),
        "toc-lehmer32" => lehmer::<u32>(args, false),
        "toc-lehmer-read16" => lehmer::<u16>(args, true),
        "toc-lehmer-read32" => lehmer::<u32>(args, true),
        "toc-layout" => {
            let pass = number(args);
            let group = number(args);
            let ng = number(args);
            let nd = number(args);
            let np = number(args);
            let n = number(args);
            let rows = number(args);
            let cols = number(args);
            let value = number(args);
            let (mut r, mut c) = (rows, cols);
            coefficient_layout(&mut r, &mut c);
            format!(
                "{} {} {} {} {} {r} {c} {}",
                ac_group_index(pass, group, ng, nd),
                num_toc_entries(ng, nd, np),
                max_bits(n),
                coefficient_rows(rows, cols),
                coefficient_columns(rows, cols),
                coeff_order_context(value)
            )
        }
        "toc-permutation" => {
            let skip = number(args);
            let n = number(args);
            let offset = number(args);
            let order: Vec<_> = (0..n).map(|_| number(args)).collect();
            let mut writer = BitWriter::new();
            prefix(&mut writer, offset);
            let mut a = AuxOut::default();
            let status = encode_permutation(
                &order,
                skip,
                n,
                &mut writer,
                LayerType::Header,
                Some(&mut a),
            );
            let mut out = format!("{} {}", status_code(&status), writer.bits_written());
            pad(&mut writer);
            write!(out, " {}", hex(writer.get_span())).unwrap();
            aux(&mut out, &a);
            if status.is_ok() {
                let mut decoded = vec![55; n];
                let mut br = BitReader::new(writer.get_span());
                br.skip_bits(offset);
                let read = decode_permutation(
                    jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
                    skip,
                    n,
                    Some(&mut decoded),
                    &mut br,
                );
                write!(
                    out,
                    " {} {} {}",
                    status_code(&read),
                    br.total_bits_consumed(),
                    status_code(&br.close())
                )
                .unwrap();
                values(&mut out, &decoded);
            }
            out
        }
        "toc-permutation-read" => {
            let skip = number(args);
            let n = number(args);
            let offset = number(args);
            let null: u32 = number(args);
            let bytes = unhex(args.next().unwrap());
            let mut decoded = vec![55; n];
            let mut br = BitReader::new(&bytes);
            br.skip_bits(offset);
            let read = decode_permutation(
                jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
                skip,
                n,
                if null != 0 { None } else { Some(&mut decoded) },
                &mut br,
            );
            let mut out = format!(
                "{} {} {}",
                status_code(&read),
                br.total_bits_consumed(),
                status_code(&br.close())
            );
            values(&mut out, &decoded);
            out
        }
        "toc-write" => {
            let n = number(args);
            let perm: u32 = number(args);
            let offset = number(args);
            let sizes: Vec<_> = (0..n).map(|_| number(args)).collect();
            let permutation: Vec<_> = (0..if perm != 0 { n } else { 0 })
                .map(|_| number(args))
                .collect();
            let mut writer = BitWriter::new();
            prefix(&mut writer, offset);
            let mut a = AuxOut::default();
            let first = write_toc_permutation(&permutation, &mut writer, Some(&mut a));
            let second = if first.is_ok() {
                write_toc_sizes(&sizes, &mut writer, Some(&mut a))
            } else {
                Err(StatusCode::GenericError)
            };
            let mut out = format!(
                "{} {} {}",
                status_code(&first),
                status_code(&second),
                writer.bits_written()
            );
            pad(&mut writer);
            write!(out, " {}", hex(writer.get_span())).unwrap();
            aux(&mut out, &a);
            write!(out, " {}", read_observation(n, offset, writer.get_span())).unwrap();
            out
        }
        "toc-read" => {
            let n = number(args);
            let offset = number(args);
            let bytes = unhex(args.next().unwrap());
            read_observation(n, offset, &bytes)
        }
        _ => panic!("unknown section-table request"),
    }
}
fn request(kind: &str, head: &str, values: &[u32]) -> String {
    let mut out = format!("{kind} {head}");
    for v in values {
        write!(out, " {v}").unwrap();
    }
    out
}
fn shuffled(n: usize, skip: usize, seed: &mut u64) -> Vec<u32> {
    let mut order: Vec<_> = (0..n as u32).collect();
    for i in (skip + 1..n).rev() {
        let j = skip + next(seed) as usize % (i + 1 - skip);
        order.swap(i, j);
    }
    order
}

#[test]
fn lehmer_coding_matches_workspaces_partial_outputs_and_both_word_widths() {
    let mut cases = Vec::new();
    let mut seed = 274177;
    for n in (0..130).chain([255, 256, 257, 1025, 65536, 65537]) {
        for bits in [16, 32] {
            if bits == 16 && n > 65536 {
                continue;
            }
            let mut order = shuffled(n, 0, &mut seed);
            cases.push(request(
                &format!("toc-lehmer{bits}"),
                &n.to_string(),
                &order,
            ));
            if n > 1 {
                order[1] = order[0];
                cases.push(request(
                    &format!("toc-lehmer{bits}"),
                    &n.to_string(),
                    &order,
                ));
            }
            let mut code: Vec<_> = (0..n)
                .map(|i| (next(&mut seed) as usize % (n - i)) as u32)
                .collect();
            cases.push(request(
                &format!("toc-lehmer-read{bits}"),
                &n.to_string(),
                &code,
            ));
            if n > 0 {
                code[n / 2] = u32::MAX;
                cases.push(request(
                    &format!("toc-lehmer-read{bits}"),
                    &n.to_string(),
                    &code,
                ));
            }
        }
    }
    compare(cases);
}
#[test]
fn permutation_streams_match_original_entropy_bytes_and_cross_decode() {
    let mut cases = Vec::new();
    let mut seed = 617983;
    for n in [1, 2, 3, 7, 8, 9, 31, 32, 33, 64, 129, 256, 1025] {
        for offset in [0, 1, 7] {
            for skip in [0, n / 2, n] {
                let order = shuffled(n, skip, &mut seed);
                cases.push(request(
                    "toc-permutation",
                    &format!("{skip} {n} {offset}"),
                    &order,
                ));
                cases.push(request(
                    "toc-permutation",
                    &format!("{skip} {n} {offset}"),
                    &(0..n as u32).collect::<Vec<_>>(),
                ));
                let mut reverse: Vec<_> = (0..n as u32).collect();
                reverse[skip..].reverse();
                cases.push(request(
                    "toc-permutation",
                    &format!("{skip} {n} {offset}"),
                    &reverse,
                ));
                let mut rotate: Vec<_> = (0..n as u32).collect();
                if skip < n {
                    rotate[skip..].rotate_left(1);
                }
                cases.push(request(
                    "toc-permutation",
                    &format!("{skip} {n} {offset}"),
                    &rotate,
                ));
            }
        }
    }
    // The source uses unsigned subtraction and an empty loop for skip > size.
    for skip in [3, usize::MAX] {
        cases.push(format!("toc-permutation {skip} 1 7 0"));
    }
    cases.push("toc-permutation 0 2 1 0 0".into());
    compare(cases);
}
#[test]
fn section_tables_match_offsets_reordering_size_boundaries_and_accounting() {
    let mut cases = Vec::new();
    let mut seed = 715823;
    let boundaries = [
        0usize,
        1,
        1023,
        1024,
        17407,
        17408,
        4211711,
        4211712,
        (1 << 30) + 4211711,
        (1 << 30) + 4211712,
        u32::MAX as usize,
        usize::MAX,
    ];
    for n in [0, 1, 2, 3, 8, 17, 65, 256, 1025, 65536] {
        for perm in 0..2 {
            for offset in [0, 1, 7] {
                let mut case = format!("toc-write {n} {perm} {offset}");
                for i in 0..n {
                    write!(
                        case,
                        " {}",
                        if n == 65536 {
                            i % 1024
                        } else {
                            boundaries[i % boundaries.len()]
                        }
                    )
                    .unwrap();
                }
                if perm != 0 {
                    for v in shuffled(n, 0, &mut seed) {
                        write!(case, " {v}").unwrap();
                    }
                }
                cases.push(case);
            }
        }
    }
    for value in boundaries {
        for offset in 0..8 {
            cases.push(format!("toc-write 1 0 {offset} {value}"));
        }
    }
    for i in 0..1024 {
        let ng = if i % 4 == 0 {
            1
        } else {
            next(&mut seed) as usize % 65536
        };
        let nd = next(&mut seed) as usize % 65536;
        let np = i % 12;
        cases.push(format!(
            "toc-layout {np} {i} {ng} {nd} {np} {} {i} {} {}",
            if i % 2 == 0 { i } else { usize::MAX },
            i * 7,
            next(&mut seed) as u32
        ));
    }
    compare(cases);
}
#[test]
fn malformed_and_truncated_sections_preserve_status_budget_checks_and_reused_state() {
    let mut cases = Vec::new();
    let mut seed = 392879;
    for len in 0..48 {
        for trial in 0..16 {
            let bytes: Vec<_> = (0..len).map(|_| next(&mut seed) as u8).collect();
            cases.push(format!(
                "toc-read {} {} {}",
                [0, 1, 2, 8, 65536, 65537, usize::MAX][trial % 7],
                trial % 8,
                hex(&bytes)
            ));
            let n = trial * 2 + 1;
            cases.push(format!(
                "toc-permutation-read {} {n} {} {} {}",
                trial % n,
                trial % 8,
                trial % 2,
                hex(&bytes)
            ));
        }
    }
    // Truncate real translated TOCs/permutations; the counterpart parser is
    // compiled from the original source, with its own independent entropy code.
    let order = shuffled(33, 0, &mut seed);
    let mut writer = BitWriter::new();
    write_toc_permutation(&order, &mut writer, None).unwrap();
    write_toc_sizes(&[17; 33], &mut writer, None).unwrap();
    for len in 0..=writer.get_span().len() {
        cases.push(format!("toc-read 33 0 {}", hex(&writer.get_span()[..len])));
    }
    let mut writer = BitWriter::new();
    encode_permutation(&order, 0, order.len(), &mut writer, LayerType::Header, None).unwrap();
    pad(&mut writer);
    for len in 0..=writer.get_span().len() {
        for null in 0..2 {
            cases.push(format!(
                "toc-permutation-read 0 33 0 {null} {}",
                hex(&writer.get_span()[..len])
            ));
        }
    }
    compare(cases);
}
