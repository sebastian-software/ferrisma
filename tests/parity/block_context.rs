// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::ac_context::{self, BlockCtxMap};
use jxl::jxl::enc_context_map::encode_block_ctx_map;
use jxl::jxl::entropy_coder::{self, decode_block_ctx_map};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("bc-")
}
fn values<T: std::fmt::Display>(out: &mut String, values: &[T]) {
    write!(out, " {}", values.len()).unwrap();
    for v in values {
        write!(out, " {v}").unwrap();
    }
}
fn read_values<T: FromStr>(args: &mut std::str::SplitWhitespace<'_>) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    (0..number(args)).map(|_| number(args)).collect()
}
fn read_map(args: &mut std::str::SplitWhitespace<'_>) -> BlockCtxMap {
    BlockCtxMap {
        dc_thresholds: std::array::from_fn(|_| read_values(args)),
        qf_thresholds: read_values(args),
        ctx_map: read_values(args),
        num_ctxs: number(args),
        num_dc_ctxs: number(args),
    }
}
fn map(out: &mut String, m: &BlockCtxMap) {
    write!(out, " M {} {}", m.num_ctxs, m.num_dc_ctxs).unwrap();
    for v in &m.dc_thresholds {
        values(out, v);
    }
    values(out, &m.qf_thresholds);
    values(out, &m.ctx_map);
}
fn spec(m: &BlockCtxMap) -> String {
    let mut out = String::new();
    for v in &m.dc_thresholds {
        values(&mut out, v);
    }
    values(&mut out, &m.qf_thresholds);
    values(&mut out, &m.ctx_map);
    write!(out, " {} {}", m.num_ctxs, m.num_dc_ctxs).unwrap();
    out
}
fn seed() -> BlockCtxMap {
    BlockCtxMap {
        dc_thresholds: [vec![-17, 31], vec![101], vec![-5, 9, 27]],
        qf_thresholds: vec![3, 29, 197],
        ctx_map: vec![99, 5, 11, 7, 2],
        num_ctxs: 101,
        num_dc_ctxs: 257,
    }
}
fn custom(dc: [Vec<i32>; 3], qf: Vec<u32>, contexts: usize, pattern: usize) -> BlockCtxMap {
    let num_dc_ctxs = dc.iter().map(|v| v.len() + 1).product();
    let count = 39 * num_dc_ctxs * (qf.len() + 1);
    BlockCtxMap {
        dc_thresholds: dc,
        qf_thresholds: qf,
        ctx_map: (0..count)
            .map(|i| {
                (match pattern {
                    0 => i % contexts,
                    1 => (i / 7) % contexts,
                    _ => usize::from(i % 19 == 0),
                } as u8)
            })
            .collect(),
        num_ctxs: contexts,
        num_dc_ctxs,
    }
}
fn decode(out: &mut String, m: &mut BlockCtxMap, bytes: &[u8], offset: usize) {
    let mut br = BitReader::new(bytes);
    br.skip_bits(offset);
    let status = decode_block_ctx_map(&mut br, m);
    write!(
        out,
        " R {} {} {}",
        status_code(&status),
        br.total_bits_consumed(),
        status_code(&br.close())
    )
    .unwrap();
    map(out, m);
}
fn aux(out: &mut String, a: &AuxOut) {
    out.push_str(" A");
    for l in &a.layers {
        write!(
            out,
            " {} {} {} {} {}",
            l.num_clustered_histograms,
            l.extra_bits,
            l.histogram_bits,
            l.total_bits,
            l.clustered_entropy.to_bits()
        )
        .unwrap();
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "bc-tables" => {
            let mut out = format!(
                "{} {} {} {}",
                ac_context::DCT_ORDER_CONTEXT_START,
                ac_context::NON_ZERO_BUCKETS,
                ac_context::ZERO_DENSITY_CONTEXT_COUNT,
                ac_context::ZERO_DENSITY_CONTEXT_LIMIT
            );
            for v in ac_context::COEFF_FREQ_CONTEXT {
                write!(out, " {v}").unwrap();
            }
            for v in ac_context::COEFF_NUM_NONZERO_CONTEXT {
                write!(out, " {v}").unwrap();
            }
            for d in [
                entropy_coder::DC_THRESHOLD_DIST,
                entropy_coder::QF_THRESHOLD_DIST,
            ] {
                for i in 0..4 {
                    write!(out, " {}", d.get_distr(i).d).unwrap();
                }
            }
            out
        }
        "bc-zero" => ac_context::zero_density_context(
            number(args),
            number(args),
            number(args),
            number(args),
            number(args),
        )
        .to_string(),
        "bc-buckets" => {
            let m = BlockCtxMap {
                num_ctxs: number(args),
                ..BlockCtxMap::default()
            };
            let nz = number(args);
            let block = number(args);
            format!(
                "{} {} {}",
                m.non_zero_context(nz, block),
                m.zero_density_contexts_offset(block),
                m.num_ac_contexts()
            )
        }
        "bc-lookup" => {
            let m = read_map(args);
            m.context(number(args), number(args), number(args), number(args))
                .to_string()
        }
        "bc-predict" => {
            let x = number(args);
            let top = number::<u32>(args) != 0;
            let def = number(args);
            let t = number(args);
            let l = number(args);
            let row = vec![l; x + 1];
            let above = vec![t; x + 1];
            entropy_coder::predict_from_top_and_left(top.then_some(above.as_slice()), &row, x, def)
                .to_string()
        }
        "bc-read" => {
            let mut m = read_map(args);
            let n = number(args);
            let mut out = String::new();
            for _ in 0..n {
                let bytes = unhex(args.next().unwrap());
                let offset = number(args);
                decode(&mut out, &mut m, &bytes, offset);
            }
            out
        }
        "bc-encode" => {
            let m = read_map(args);
            let offset = number(args);
            let limit: usize = number(args);
            let mut w = BitWriter::new();
            prefix(&mut w, offset);
            let mut a = AuxOut::default();
            for (i, l) in a.layers.iter_mut().enumerate() {
                l.num_clustered_histograms = 7 + i;
                l.extra_bits = 11 + i;
                l.histogram_bits = 13 + i;
                l.total_bits = 17 + i;
                l.clustered_entropy = 0.75;
            }
            let status = encode_block_ctx_map(&m, &mut w, Some(&mut a));
            let mut out = format!("W {} {}", status_code(&status), w.bits_written());
            pad(&mut w);
            write!(out, " {}", hex(w.get_span())).unwrap();
            aux(&mut out, &a);
            let mut decoded = read_map(args);
            let bytes = w.get_span();
            decode(
                &mut out,
                &mut decoded,
                &bytes[..limit.min(bytes.len())],
                offset,
            );
            out
        }
        _ => panic!("block context command {kind}"),
    }
}
fn encoded(m: &BlockCtxMap, offset: usize) -> Vec<u8> {
    let mut w = BitWriter::new();
    prefix(&mut w, offset);
    encode_block_ctx_map(m, &mut w, None).unwrap();
    pad(&mut w);
    w.get_span().to_vec()
}
#[test]
fn block_context_tables_and_full_zero_density_domains_match() {
    let mut cases = vec!["bc-tables".into()];
    for log in 0..=10 {
        let covered = 1usize << log;
        for nz in 1..64 {
            for k in 1..64 {
                for prev in 0..2 {
                    cases.push(format!(
                        "bc-zero {} {} {covered} {log} {prev}",
                        nz * covered - (covered - 1),
                        k * covered + covered - 1
                    ));
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn block_context_nonzero_buckets_preserve_clamping_and_unsigned_narrowing() {
    let mut cases = Vec::new();
    for contexts in [0, 1, 15, 16, 128, u32::MAX as usize, usize::MAX] {
        for nz in (0..=70).chain([255, 1008, u32::MAX]) {
            for block in [0, 1, 15, 17, u32::MAX] {
                cases.push(format!("bc-buckets {contexts} {nz} {block}"));
            }
        }
    }
    compare(cases);
}
#[test]
fn block_context_lookup_preserves_channel_order_dc_and_strict_thresholds() {
    let m = custom([vec![-5, 7], vec![], vec![]], vec![7, 2, 7], 16, 0);
    let s = spec(&m);
    let mut cases = Vec::new();
    for dc in 0..3 {
        for qf in [0, 1, 2, 3, 6, 7, 8, 99, u32::MAX] {
            for ord in 0..13 {
                for c in 0..5 {
                    cases.push(format!("bc-lookup{s} {dc} {qf} {ord} {c}"));
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn block_context_prediction_preserves_null_edges_and_signed_division() {
    let mut cases = Vec::new();
    let vals = [-1_000_000_000, -5, -2, -1, 0, 1, 2, 5, 1_000_000_000];
    for x in [0, 1, 7, 32] {
        for top in 0..2 {
            for t in vals {
                for l in vals {
                    cases.push(format!("bc-predict {x} {top} -12345 {t} {l}"));
                }
            }
        }
    }
    for x in [0, 1] {
        for top in 0..2 {
            for t in [i32::MIN, i32::MAX] {
                for l in [i32::MIN, i32::MAX] {
                    if x == 0 || top == 0 {
                        cases.push(format!("bc-predict {x} {top} {} {t} {l}", i32::MIN));
                    }
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn block_context_real_encoder_covers_threshold_boundaries_maps_and_accounting() {
    let mut maps = vec![
        BlockCtxMap::default(),
        BlockCtxMap {
            num_ctxs: 991,
            num_dc_ctxs: 997,
            ..BlockCtxMap::default()
        },
        custom([vec![1; 15], vec![], vec![]], vec![1, 5, 300], 16, 0), // exactly 64 partitions
        custom([vec![1; 15], vec![2; 3], vec![]], vec![], 16, 0),      // exactly 64 DC partitions
        custom([vec![1; 12], vec![2; 4], vec![]], vec![], 16, 0),      // first rejected 65
        custom([vec![], vec![], vec![]], (1..=15).collect(), 16, 0),
    ];
    for dc in [
        0,
        -1,
        1,
        -7,
        7,
        -8,
        8,
        -135,
        135,
        -136,
        136,
        -32903,
        32903,
        -32904,
        32904,
        i32::MIN,
        i32::MAX,
    ] {
        maps.push(custom([vec![dc], vec![], vec![]], vec![], 2, 0));
    }
    for qf in [0, 1, 4, 5, 12, 13, 44, 45, 300, 301, u32::MAX] {
        maps.push(custom([vec![], vec![], vec![]], vec![qf], 3, 0));
    }
    for contexts in [1, 2, 3, 7, 8, 16, 17] {
        for pattern in 0..3 {
            maps.push(custom(
                [vec![-11, 35], vec![9], vec![]],
                vec![5, 35],
                contexts,
                pattern,
            ));
        }
    }
    let mut oversized = custom([vec![1; 15], vec![2; 15], vec![3; 15]], vec![4; 15], 2, 0);
    oversized.ctx_map.truncate(39);
    maps.push(oversized);
    let mut cases = Vec::new();
    for m in maps {
        for offset in 0..8 {
            for limit in [0, 1, 3, usize::MAX] {
                cases.push(format!(
                    "bc-encode{} {offset} {limit}{}",
                    spec(&m),
                    spec(&seed())
                ));
            }
        }
    }
    compare(cases);
}
#[test]
fn block_context_decoding_preserves_reused_owners_errors_and_truncated_streams() {
    let ordinary = custom([vec![-9, 12], vec![], vec![17]], vec![1, 44], 16, 0);
    let oversized = custom([vec![1; 4], vec![2; 4], vec![3]], vec![4], 2, 0);
    let too_many = custom([vec![], vec![], vec![]], vec![], 17, 0);
    let mut missing = custom([vec![], vec![], vec![]], vec![], 3, 0);
    missing.ctx_map.iter_mut().for_each(|v| {
        if *v == 1 {
            *v = 2
        }
    });
    let mut cases = Vec::new();
    for offset in 0..8 {
        let bytes = encoded(&ordinary, offset);
        for len in 0..=bytes.len() {
            cases.push(format!(
                "bc-read{} 1 {} {offset}",
                spec(&seed()),
                hex(&bytes[..len])
            ));
        }
        let all = [
            encoded(&oversized, offset),
            encoded(&ordinary, offset),
            encoded(&too_many, offset),
            encoded(&missing, offset),
            encoded(&BlockCtxMap::default(), offset),
        ];
        let mut request = format!("bc-read{} {}", spec(&seed()), all.len() + 1);
        for bytes in all {
            write!(request, " {} {offset}", hex(&bytes)).unwrap();
        }
        write!(request, " - {offset}").unwrap();
        cases.push(request);
    }
    for offset in 0..8 {
        let m = custom([vec![i32::MIN], vec![], vec![]], vec![], 2, 0);
        let original = encoded(&m, offset);
        for residual in [0u32, 1, 65807, 65808, u32::MAX - 65808, u32::MAX] {
            let mut bytes = original.clone();
            // Corrupt only the first real encoder payload: default flag (1),
            // threshold count (4), selector (2), then this 32-bit residual.
            // This reaches U32Coder's original offset/narrowing behavior.
            for bit in 0..32 {
                let at = offset + 7 + bit;
                bytes[at / 8] = (bytes[at / 8] & !(1 << (at % 8)))
                    | ((((residual >> bit) & 1) as u8) << (at % 8));
            }
            cases.push(format!(
                "bc-read{} 1 {} {offset}",
                spec(&seed()),
                hex(&bytes)
            ));
        }
    }
    let mut rng = 0x8a4ef3;
    for len in 0..=24 {
        for _ in 0..12 {
            let bytes: Vec<_> = (0..len).map(|_| next(&mut rng) as u8).collect();
            for offset in 0..8 {
                cases.push(format!(
                    "bc-read{} 1 {} {offset}",
                    spec(&seed()),
                    hex(&bytes)
                ));
            }
        }
    }
    compare(cases);
}
