// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::quant_weights::{
    DctQuantWeightParams, DequantMatrices, MAX_DISTANCE_BANDS, QuantEncoding, QuantEncodingData,
};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("qst-")
}
fn read_params(args: &mut std::str::SplitWhitespace<'_>) -> DctQuantWeightParams {
    let kind = number::<u32>(args);
    let n = number(args);
    let bands = std::array::from_fn(|_| std::array::from_fn(|_| f32::from_bits(number(args))));
    match kind {
        0 => DctQuantWeightParams::empty(),
        1 => DctQuantWeightParams::new(bands, n),
        2 => DctQuantWeightParams::from_bands(&std::array::from_fn(|c| [bands[c][0]])),
        3 => DctQuantWeightParams::from_bands(&std::array::from_fn(|c| {
            [bands[c][0], bands[c][1], bands[c][2]]
        })),
        _ => DctQuantWeightParams::from_bands(&bands),
    }
}
fn weights<const N: usize>(values: &[f32; 27]) -> [[f32; N]; 3] {
    std::array::from_fn(|c| std::array::from_fn(|i| values[c * N + i]))
}
fn read(args: &mut std::str::SplitWhitespace<'_>) -> (QuantEncoding, Vec<i32>) {
    let mode = number::<u32>(args);
    let p = read_params(args);
    let p2 = read_params(args);
    let w = std::array::from_fn(|_| f32::from_bits(number(args)));
    let shift = number(args);
    let n = number(args);
    let input: Vec<_> = (0..n).map(|_| number(args)).collect();
    let q = match mode {
        0 => QuantEncoding::library::<0>(),
        1 => QuantEncoding::identity(&weights::<3>(&w)),
        2 => QuantEncoding::dct2(&weights::<6>(&w)),
        3 => QuantEncoding::dct4(&p, &weights::<2>(&w)),
        4 => QuantEncoding::dct4x8(&p, &[w[0], w[1], w[2]]),
        5 => QuantEncoding::afv(&p, &p2, &weights::<9>(&w)),
        6 => QuantEncoding::dct(&p),
        7 => QuantEncoding::raw(&input, shift),
        _ => panic!("quant fixture mode"),
    };
    (q, input)
}
fn params(out: &mut String, p: &DctQuantWeightParams) {
    write!(out, " {}", p.num_distance_bands).unwrap();
    for row in p.distance_bands {
        for v in row {
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
}
fn floats<const N: usize>(out: &mut String, w: &[[f32; N]; 3]) {
    for row in w {
        for v in row {
            write!(out, " {}", v.to_bits()).unwrap();
        }
    }
}
fn values<T: std::fmt::Display>(out: &mut String, vs: &[T]) {
    for v in vs {
        write!(out, " {v}").unwrap();
    }
}
fn encoding(out: &mut String, q: &QuantEncoding) {
    write!(out, " Q {}", q.mode() as u32).unwrap();
    params(out, &q.dct_params);
    match q.data() {
        QuantEncodingData::Identity(w) => floats(out, w),
        QuantEncodingData::Dct2(w) => floats(out, w),
        QuantEncodingData::Dct4(w) => floats(out, w),
        QuantEncodingData::Afv(w) => floats(out, w),
        QuantEncodingData::Dct4X8(w) => {
            for v in w {
                write!(out, " {}", v.to_bits()).unwrap();
            }
        }
        QuantEncodingData::Raw { qtable, qtable_den } => {
            write!(
                out,
                " {} {}",
                u32::from(qtable.is_some()),
                qtable_den.to_bits()
            )
            .unwrap();
            if let Some(table) = qtable {
                write!(out, " {}", table.len()).unwrap();
                values(out, table);
            }
        }
    }
    params(out, &q.dct_params_afv_4x4);
    write!(out, " {}", q.predefined).unwrap();
}
fn mutate(q: &mut QuantEncoding, v: i32) {
    if let Some(table) = q.raw_table_mut() {
        if let Some(x) = table.first_mut() {
            *x ^= v;
        }
    }
}
fn matrices(out: &mut String, m: &DequantMatrices) {
    write!(
        out,
        " M {} {}",
        m.computed_mask,
        u32::from(m.have_table_storage())
    )
    .unwrap();
    values(out, m.table_offsets());
    for c in 0..3 {
        write!(
            out,
            " {},{}",
            m.dc_quant(c).to_bits(),
            m.inv_dc_quant(c).to_bits()
        )
        .unwrap();
    }
    write!(out, " {}", m.encodings().len()).unwrap();
    for q in m.encodings() {
        encoding(out, q);
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut out = String::new();
    if kind == "qst-owner" {
        let (mut q, input) = read(args);
        write!(out, "I {}", input.len()).unwrap();
        values(&mut out, &input);
        encoding(&mut out, &q);
        let mut copy = q.clone();
        mutate(&mut q, 0x13579);
        encoding(&mut out, &q);
        encoding(&mut out, &copy);
        let mut assigned = QuantEncoding::raw(&[101, 202], 0);
        assigned.clone_from(&copy);
        mutate(&mut copy, 0x2468);
        encoding(&mut out, &copy);
        encoding(&mut out, &assigned);
        let moved = QuantEncoding::move_from(&mut copy);
        encoding(&mut out, &copy);
        encoding(&mut out, &moved);
        encoding(&mut out, &copy.clone());
    } else {
        let count = number(args);
        let mut input: Vec<_> = (0..count).map(|_| read(args).0).collect();
        let mut m = DequantMatrices::default();
        matrices(&mut out, &m);
        m.set_dc_quant(&[3.0, 7.0, 11.0]);
        for phase in 0..2 {
            m.computed_mask = 0x5a5a + phase;
            m.set_encodings(&input);
            for q in &mut input {
                mutate(q, 0x4321 + phase as i32);
            }
            matrices(&mut out, &m);
        }
    }
    out
}
fn recipe(mode: usize, kind: usize, trial: usize, seed: &mut u64) -> String {
    let mut out = mode.to_string();
    for k in [kind, (kind + trial) % 5] {
        let n = [0, 1, 17, 18, usize::MAX][trial % 5];
        write!(out, " {k} {n}").unwrap();
        for c in 0..3 {
            for i in 0..MAX_DISTANCE_BANDS {
                let bits = if trial == 0 {
                    0
                } else if trial == 1 {
                    [0x80000000, 0x7fa23456, 0x7fc34567, 0x7f800000, 0xff800000][(c + i) % 5]
                } else {
                    next(seed) as u32
                };
                write!(out, " {bits}").unwrap();
            }
        }
    }
    for i in 0..27 {
        let bits = if trial % 2 == 0 {
            (i as f32 - 13.0).to_bits()
        } else {
            next(seed) as u32
        };
        write!(out, " {bits}").unwrap();
    }
    let n = [0, 1, 3, 17, 65][trial % 5];
    write!(out, " {} {n}", trial % 32).unwrap();
    for _ in 0..n {
        write!(out, " {}", next(seed) as i32).unwrap();
    }
    out
}
#[test]
fn quantization_constructors_copy_assignment_and_moves_match_active_source_members() {
    let mut cases = Vec::new();
    let mut seed = 0x7192_ef03_45ab_6dc8;
    for mode in 0..8 {
        for kind in 0..5 {
            for trial in 0..12 {
                cases.push(format!(
                    "qst-owner {}",
                    recipe(mode, kind, trial, &mut seed)
                ));
            }
        }
    }
    for shift in [12, 16, 24, 30, 31] {
        cases.push(format!("qst-owner {}", recipe(7, 1, shift, &mut seed)));
    }
    compare(cases);
}
#[test]
fn matrix_constructor_offsets_default_encodings_and_reused_vector_owners_match_source() {
    let mut cases = Vec::new();
    let mut seed = 0xb56d_24f0_9ae1_738c;
    for n in [0, 1, 3, 17, 18, 35] {
        for trial in 0..4 {
            let mut request = format!("qst-matrices {n}");
            for i in 0..n {
                write!(
                    request,
                    " {}",
                    recipe(i % 8, (i + trial) % 5, i + trial, &mut seed)
                )
                .unwrap();
            }
            cases.push(request);
        }
    }
    compare(cases);
}
