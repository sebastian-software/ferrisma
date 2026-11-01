// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
pub(super) fn handles(kind: &str) -> bool {
    kind == "aux-stats"
}
fn counters(a: &AuxOut) -> [usize; 12] {
    [
        a.num_blocks,
        a.num_small_blocks,
        a.num_dct4x8_blocks,
        a.num_afv_blocks,
        a.num_dct8_blocks,
        a.num_dct8x16_blocks,
        a.num_dct8x32_blocks,
        a.num_dct16_blocks,
        a.num_dct16x32_blocks,
        a.num_dct32_blocks,
        a.num_dct32x64_blocks,
        a.num_dct64_blocks,
    ]
}
fn set_counters(a: &mut AuxOut, c: [usize; 12]) {
    [
        a.num_blocks,
        a.num_small_blocks,
        a.num_dct4x8_blocks,
        a.num_afv_blocks,
        a.num_dct8_blocks,
        a.num_dct8x16_blocks,
        a.num_dct8x32_blocks,
        a.num_dct16_blocks,
        a.num_dct16x32_blocks,
        a.num_dct32_blocks,
        a.num_dct32x64_blocks,
        a.num_dct64_blocks,
    ] = c;
}
pub(super) fn seed(seed: u64) -> AuxOut {
    let mut a = AuxOut::default();
    for (i, l) in a.layers.iter_mut().enumerate() {
        l.num_clustered_histograms = (seed as usize).wrapping_add(i);
        l.histogram_bits = (seed as usize).wrapping_add(i * 3);
        l.extra_bits = (seed as usize).wrapping_add(i * 5);
        l.total_bits = (seed as usize).wrapping_add(i * 7);
        l.clustered_entropy = seed.wrapping_add(i as u64) as f64 / 8.0;
    }
    set_counters(
        &mut a,
        std::array::from_fn(|i| (seed as usize).wrapping_add(i + 1)),
    );
    a.num_butteraugli_iters = seed as i32;
    a
}
fn read(args: &mut std::str::SplitWhitespace<'_>) -> AuxOut {
    let mut a = AuxOut::default();
    for l in &mut a.layers {
        l.num_clustered_histograms = number(args);
        l.histogram_bits = number(args);
        l.extra_bits = number(args);
        l.total_bits = number(args);
        l.clustered_entropy = f64::from_bits(number(args));
    }
    set_counters(&mut a, std::array::from_fn(|_| number(args)));
    a.num_butteraugli_iters = number(args);
    a
}
pub(super) fn output(out: &mut String, a: &AuxOut) {
    write!(out, " X {}", a.total_bits()).unwrap();
    for l in &a.layers {
        write!(
            out,
            " {} {} {} {} {}",
            l.num_clustered_histograms,
            l.histogram_bits,
            l.extra_bits,
            l.total_bits,
            l.clustered_entropy.to_bits()
        )
        .unwrap();
    }
    for c in counters(a) {
        write!(out, " {c}").unwrap();
    }
    write!(out, " {}", a.num_butteraugli_iters).unwrap();
}
pub(super) fn observe(_kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let n = number(args);
    let mut a = read(args);
    let mut out = String::new();
    output(&mut out, &a);
    for _ in 0..n {
        let layer = number::<i32>(args);
        let use_self = number::<u32>(args) != 0;
        let b = read(args);
        let b = if use_self { a.clone() } else { b };
        if layer < 0 {
            a.assimilate(&b);
        } else {
            a.layers[layer as usize].assimilate(&b.layers[layer as usize]);
        }
        output(&mut out, &a);
    }
    out
}
fn input(a: &AuxOut) -> String {
    let mut out = String::new();
    for l in &a.layers {
        write!(
            out,
            " {} {} {} {} {}",
            l.num_clustered_histograms,
            l.histogram_bits,
            l.extra_bits,
            l.total_bits,
            l.clustered_entropy.to_bits()
        )
        .unwrap();
    }
    for c in counters(a) {
        write!(out, " {c}").unwrap();
    }
    write!(out, " {}", a.num_butteraugli_iters).unwrap();
    out
}
#[test]
fn auxiliary_assimilation_preserves_all_fields_unsigned_wrapping_float_bits_and_self_inputs() {
    let mut cases = Vec::new();
    let mut rng = 0x4397_8bd6_a215_ef03;
    let floats = [
        0u64,
        0x8000000000000000,
        0x3ff0000000000000,
        0xbff0000000000000,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff8123456789abc,
        0x7ff0123456789abc,
    ];
    for trial in 0..240 {
        let mut a = seed(trial as u64);
        let mut b = seed((trial + 1) as u64);
        for (i, l) in a.layers.iter_mut().enumerate() {
            l.num_clustered_histograms = next(&mut rng) as usize;
            l.total_bits = next(&mut rng) as usize;
            l.clustered_entropy = f64::from_bits(floats[(trial + i) % floats.len()]);
        }
        for (i, l) in b.layers.iter_mut().enumerate() {
            l.extra_bits = next(&mut rng) as usize;
            l.histogram_bits = next(&mut rng) as usize;
            l.clustered_entropy = f64::from_bits(floats[(trial + i + 3) % floats.len()]);
        }
        set_counters(&mut a, std::array::from_fn(|_| next(&mut rng) as usize));
        set_counters(&mut b, std::array::from_fn(|_| next(&mut rng) as usize));
        a.num_butteraugli_iters = trial as i32 - 100;
        b.num_butteraugli_iters = trial as i32 - 200;
        cases.push(format!(
            "aux-stats 3{} -1 0{} {} 0{} -1 1{}",
            input(&a),
            input(&b),
            trial % 15,
            input(&b),
            input(&b)
        ));
    }
    compare(cases);
}
