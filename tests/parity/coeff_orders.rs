// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

use super::*;
use jxl::jxl::coeff_order::{COEFF_ORDER_MAX_SIZE, STRATEGY_ORDER, decode_coeff_orders};

const SENTINEL: u32 = 0x3141_5926;

pub(super) fn handles(kind: &str) -> bool {
    kind == "co-orders-read"
}

pub(super) fn observe(_: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let actions = number(args);
    let mut order = vec![SENTINEL; COEFF_ORDER_MAX_SIZE + 4];
    let mut out = String::new();
    for a in 0..actions {
        let used = number(args);
        let acs = number(args);
        let prefix = number(args);
        let bytes = unhex(args.next().unwrap());
        let mut br = BitReader::new(&bytes);
        br.skip_bits(prefix);
        let status = decode_coeff_orders(used, acs, &mut order, &mut br);
        if a != 0 {
            out.push(' ');
        }
        write!(
            out,
            "{} {} {} {} {}",
            status_code(&status),
            br.total_bits_consumed(),
            status_code(&br.close()),
            order.len(),
            order.iter().filter(|&&v| v != SENTINEL).count()
        )
        .unwrap();
        for (index, &value) in order.iter().enumerate() {
            if value != SENTINEL {
                write!(out, " {index}:{value}").unwrap();
            }
        }
    }
    out
}

fn fixture(used: u16, variant: usize, prefix: usize) -> Vec<u8> {
    let response =
        original_observations(&[format!("co-orders-generate {used} {variant} {prefix}")]);
    let words: Vec<_> = response[0].split_whitespace().collect();
    assert_eq!(words[0], "0");
    unhex(words[2])
}

fn compare_chunks(requests: Vec<String>) {
    // Sparse observations reconstruct the full packed owner, rather than
    // hashing it; bound peak memory for the largest 256x256 order buckets.
    for chunk in requests.chunks(8) {
        compare(chunk.to_vec());
    }
}

#[test]
fn coefficient_orders_default_aliases_and_untouched_storage() {
    let mut cases = Vec::new();
    for acs in (0..27).map(|raw| 1u32 << raw).chain([
        0,
        0x07ff_ffff,
        0xffff_ffff,
        0xf800_0000,
        0x0555_5555,
        0x02aa_aaaa,
    ]) {
        for prefix in [0, 3, 7] {
            cases.push(format!("co-orders-read 1 0 {acs} {prefix} ff"));
        }
    }
    compare_chunks(cases);
}

#[test]
fn coefficient_orders_real_permutations_and_unused_entropy_consumption() {
    let mut cases = Vec::new();
    for ord in 0..13 {
        let mask = 1 << ord;
        let raw = STRATEGY_ORDER.iter().position(|&x| x == ord).unwrap();
        for variant in 0..4 {
            let prefix = (ord as usize + variant) % 8;
            let data = hex(&fixture(mask, variant, prefix));
            for acs in [0, 1 << raw] {
                cases.push(format!("co-orders-read 1 {mask} {acs} {prefix} {data}"));
            }
        }
    }
    for mask in [3, 0x15, 0x1fff, 0x2000, 0x8000] {
        let data = hex(&fixture(mask, 2, 5));
        for acs in [0, 1, 0x07ff_ffff] {
            cases.push(format!("co-orders-read 1 {mask} {acs} 5 {data}"));
        }
    }
    compare_chunks(cases);
}

#[test]
fn coefficient_orders_truncation_corruption_and_reused_output() {
    let mut cases = Vec::new();
    for mask in [1, 3, 0x15] {
        let data = fixture(mask, 3, 3);
        for len in 0..=data.len() {
            let bytes = hex(&data[..len]);
            for acs in [0, 1, 0x1ff] {
                cases.push(format!("co-orders-read 1 {mask} {acs} 3 {bytes}"));
            }
        }
        for i in 0..data.len() {
            for bit in [0, 3, 7] {
                let mut changed = data.clone();
                changed[i] ^= 1 << bit;
                cases.push(format!("co-orders-read 1 {mask} 1 3 {}", hex(&changed)));
            }
        }
        cases.push(format!(
            "co-orders-read 3 0 0 0 - {mask} 1 3 {} 0 2 0 -",
            hex(&data)
        ));
        for len in [0, data.len() / 2, data.len().saturating_sub(1)] {
            cases.push(format!(
                "co-orders-read 3 0 1 0 - {mask} 1 3 {} 0 2 0 -",
                hex(&data[..len])
            ));
        }
    }
    // Nonzero reserved order bits still initialize/check an entropy reader.
    for used in [0x2000, 0x8000, 0xe000, 0xffff] {
        for bytes in ["-", "00", "ff", "0000000000000000", "ffffffffffffffff"] {
            cases.push(format!("co-orders-read 1 {used} 0 0 {bytes}"));
        }
    }
    compare_chunks(cases);
}
