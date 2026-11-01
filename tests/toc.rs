// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Lehmer size ranges adapt lehmer_code_test.cc; section payload integration is
// local infrastructure and is not an image codec test.

use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_toc::{write_toc_permutation, write_toc_sizes};
use jxl::jxl::lehmer_code::{PermutationWord, compute_lehmer_code, decode_lehmer_code};
use jxl::jxl::toc::read_group_offsets;

fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}
fn round_trip<T: PermutationWord + Eq + std::fmt::Debug>(n: usize, seed: &mut u64) {
    let mut permutation: Vec<_> = (0..n).map(T::from_index).collect();
    let mut decoded = vec![T::from_index(0); n];
    let mut code = vec![0; n];
    let mut temp = vec![0; 2 * n];
    for _ in 0..3 {
        for i in (1..n).rev() {
            let j = next(seed) as usize % (i + 1);
            permutation.swap(i, j);
        }
        compute_lehmer_code(&permutation, &mut temp, n, &mut code).unwrap();
        temp.fill(0);
        decode_lehmer_code(&code, &mut temp, n, &mut decoded).unwrap();
        assert_eq!(decoded, permutation);
    }
}
#[test]
fn upstream_lehmer_size_ranges_round_trip_in_rust() {
    let mut seed = 128243;
    for n in 1..1026 {
        round_trip::<u16>(n, &mut seed);
    }
    for n in 65536..65540 {
        round_trip::<u32>(n, &mut seed);
    }
}
#[test]
fn permuted_section_offsets_locate_distinct_payloads() {
    let sections: [&[u8]; 3] = [b"abc", b"d", b"efgh"];
    let mut writer = BitWriter::new();
    write_toc_permutation(&[2, 0, 1], &mut writer, None).unwrap();
    write_toc_sizes(&[3, 1, 4], &mut writer, None).unwrap();
    let start = writer.get_span().len();
    let mut br = BitReader::new(writer.get_span());
    let mut offsets = Vec::new();
    let mut sizes = Vec::new();
    let mut total = 0;
    read_group_offsets(3, &mut br, &mut offsets, &mut sizes, Some(&mut total)).unwrap();
    assert_eq!(br.total_bits_consumed(), start * 8);
    br.close().unwrap();
    let payload: Vec<_> = sections.into_iter().flatten().copied().collect();
    let observed: Vec<_> = offsets
        .iter()
        .zip(&sizes)
        .map(|(&offset, &size)| &payload[offset as usize..offset as usize + size as usize])
        .collect();
    assert_eq!(observed, [&b"efgh"[..], &b"abc"[..], &b"d"[..]]);
    assert_eq!(total, 8);
}
