// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Real encoded entropy streams feed the original pinned decoder template and
// its native translation. Fixture context selection is protocol construction.
use super::*;
use jxl::jxl::ac_context::{BlockCtxMap, zero_density_context};
use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyType};
use jxl::jxl::coeff_order::{COEFF_ORDER_OFFSET, STRATEGY_ORDER};
use jxl::jxl::dct_util::AcPtrMut;
use jxl::jxl::dec_ans::{ANSCode, ANSSymbolReader, Checkpoint, decode_histograms};
use jxl::jxl::dec_group::decode_ac_var_block;
use jxl::jxl::entropy_coder::predict_from_top_and_left;

pub(super) fn handles(kind: &str) -> bool {
    kind == "acg-decode"
}
fn context_map(custom: bool) -> BlockCtxMap {
    if custom {
        BlockCtxMap {
            num_ctxs: 3,
            num_dc_ctxs: 2,
            qf_thresholds: vec![3, 8],
            ctx_map: (0..3 * 13 * 3 * 2)
                .map(|i| ((i * 7 + 1) % 3) as u8)
                .collect(),
            ..BlockCtxMap::default()
        }
    } else {
        BlockCtxMap::default()
    }
}
fn checkpoint(out: &mut String, reader: &ANSSymbolReader<'_>, br: &BitReader<'_>) {
    let mut cp = Checkpoint::default();
    reader.save(&mut cp);
    write!(
        out,
        " {},{},{},{},{}",
        br.total_bits_consumed(),
        cp.state,
        cp.num_decoded,
        cp.num_to_copy,
        cp.copy_pos
    )
    .unwrap();
}
fn array<T: std::fmt::Display>(out: &mut String, values: &[T]) {
    write!(out, " {}", values.len()).unwrap();
    for v in values {
        write!(out, " {v}").unwrap();
    }
}
pub(super) fn observe(_: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let width: usize = number(args);
    let strategy = number(args);
    let c = number(args);
    let bx = number(args);
    let lbx = number(args);
    let by = number(args);
    let stride = number(args);
    let top: u8 = number(args);
    let seed: usize = number(args);
    let initial: i32 = number(args);
    let order_mode: u8 = number(args);
    let custom: u8 = number(args);
    let ctx_offset = number(args);
    let qf = number(args);
    let qdc = number(args);
    let bytes = unhex(args.next().unwrap());
    let offset = number(args);
    let calls = number(args);
    let warmup_count = number(args);
    let acs = AcStrategy::from_raw_strategy(AcStrategyType(strategy));
    let covered = 1 << acs.log2_covered_blocks();
    let size = covered * 64;
    let order_offset =
        COEFF_ORDER_OFFSET[3 * usize::from(STRATEGY_ORDER[strategy as usize]) + c] * 64;
    let mut order = vec![0; order_offset + size];
    acs.compute_natural_coeff_order(&mut order[order_offset..]);
    if order_mode != 0 {
        order[order_offset + covered..].reverse();
    }
    let mut row: Vec<i32> = (0..stride * acs.covered_blocks_y() + 16)
        .map(|i| ((i * 5 + seed) % 64) as i32)
        .collect();
    let row_top: Vec<i32> = (0..stride).map(|i| ((i * 3 + seed) % 64) as i32).collect();
    let qdc_row = vec![qdc; lbx + 1];
    let qf_row = vec![qf; row.len()];
    let map = context_map(custom != 0);
    let mut block16: Vec<i16> = (0..size)
        .map(|i| (initial + (i % 13) as i32 - 6) as i16)
        .collect();
    let mut block32: Vec<i32> = (0..size).map(|i| initial + (i % 13) as i32 - 6).collect();
    let mut br = BitReader::new(&bytes);
    br.skip_bits(offset);
    let mut code = ANSCode::default();
    let mut contexts = Vec::new();
    let status = decode_histograms(
        &mut br,
        ctx_offset + map.num_ac_contexts() as usize + 16,
        &mut code,
        &mut contexts,
        false,
    );
    let mut out = format!(
        "{} {}",
        status_code(&status) as i32,
        br.total_bits_consumed()
    );
    let shifts: Vec<usize> = (0..calls).map(|_| number(args)).collect();
    if status.is_ok() {
        let mut decoder = ANSSymbolReader::create(&code, &mut br, 0).unwrap();
        write!(
            out,
            " {} {}",
            u8::from(decoder.uses_lz77()),
            u8::from(code.use_prefix_code)
        )
        .unwrap();
        checkpoint(&mut out, &decoder, &br);
        let warmup: Vec<_> = (0..warmup_count)
            .map(|_| decoder.read_hybrid_uint(0, &mut br, &contexts))
            .collect();
        array(&mut out, &warmup);
        checkpoint(&mut out, &decoder, &br);
        for shift in shifts {
            let block = if width == 16 {
                AcPtrMut::I16(&mut block16)
            } else {
                AcPtrMut::I32(&mut block32)
            };
            let top = (top != 0).then_some(row_top.as_slice());
            let status = if decoder.uses_lz77() {
                decode_ac_var_block::<true>(
                    ctx_offset,
                    acs.log2_covered_blocks(),
                    &mut row,
                    top,
                    stride,
                    c,
                    bx,
                    by,
                    lbx,
                    acs,
                    &order,
                    &mut br,
                    &mut decoder,
                    &contexts,
                    &qdc_row,
                    &qf_row,
                    &map,
                    block,
                    shift,
                )
            } else {
                decode_ac_var_block::<false>(
                    ctx_offset,
                    acs.log2_covered_blocks(),
                    &mut row,
                    top,
                    stride,
                    c,
                    bx,
                    by,
                    lbx,
                    acs,
                    &order,
                    &mut br,
                    &mut decoder,
                    &contexts,
                    &qdc_row,
                    &qf_row,
                    &map,
                    block,
                    shift,
                )
            };
            write!(out, " {}", status_code(&status) as i32).unwrap();
            checkpoint(&mut out, &decoder, &br);
            array(&mut out, &row);
            if width == 16 {
                array(&mut out, &block16);
            } else {
                array(&mut out, &block32);
            }
        }
        write!(out, " {}", u8::from(decoder.check_ans_final_state())).unwrap();
    }
    let bits = br.total_bits_consumed();
    let closed = br.close();
    write!(out, " {bits} {}", status_code(&closed) as i32).unwrap();
    out
}
#[derive(Clone)]
struct Case {
    width: usize,
    strategy: u32,
    c: usize,
    bx: usize,
    top: bool,
    seed: usize,
    initial: i32,
    reverse: bool,
    custom: bool,
    ctx_offset: usize,
    qf: i32,
    qdc: u8,
    prefix: bool,
    lz: usize,
    offset: usize,
    counts: Vec<usize>,
    values: Vec<Vec<u32>>,
    shifts: Vec<usize>,
    expect_last_padding_context: bool,
}
impl Case {
    fn new(strategy: u32, width: usize, prefix: bool, lz: usize) -> Self {
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(strategy));
        let available = (64 << acs.log2_covered_blocks()) - (1 << acs.log2_covered_blocks());
        let mut values = vec![0; available.min(160)];
        let n = values.len();
        values[n - 3..].copy_from_slice(&[2, 3, 65535]);
        Self {
            width,
            strategy,
            c: strategy as usize % 3,
            bx: strategy as usize % 2,
            top: strategy % 3 != 0,
            seed: strategy as usize + 7,
            initial: if width == 16 { 32760 } else { -12345 },
            reverse: strategy % 2 != 0,
            custom: strategy % 2 != 0,
            ctx_offset: strategy as usize % 4,
            qf: if strategy % 4 == 0 { -1 } else { 5 },
            qdc: (strategy % 2) as u8,
            prefix,
            lz,
            offset: strategy as usize % 8,
            counts: vec![3, 3],
            values: vec![values.clone(), values],
            shifts: vec![0, 3],
            expect_last_padding_context: false,
        }
    }
    fn warmup_count(&self) -> usize {
        if self.lz == 0 { 0 } else { 512 }
    }
    fn map(&self) -> BlockCtxMap {
        context_map(self.custom)
    }
    fn stride(&self) -> usize {
        AcStrategy::from_raw_strategy(AcStrategyType(self.strategy)).covered_blocks_x()
            + self.bx
            + 3
    }
    fn fixture(&self) -> String {
        let map = self.map();
        let acs = AcStrategy::from_raw_strategy(AcStrategyType(self.strategy));
        let covered = 1 << acs.log2_covered_blocks();
        let size = covered * 64;
        let stride = self.stride();
        let mut row: Vec<i32> = (0..stride * acs.covered_blocks_y() + 16)
            .map(|i| ((i * 5 + self.seed) % 64) as i32)
            .collect();
        let row_top: Vec<i32> = (0..stride)
            .map(|i| ((i * 3 + self.seed) % 64) as i32)
            .collect();
        let ord = usize::from(STRATEGY_ORDER[self.strategy as usize]);
        let block_ctx = map.context(
            i32::from(if self.custom { self.qdc } else { 0 }),
            self.qf as u32,
            ord,
            self.c,
        );
        // Repeated, varied symbols in a real encoder context make LZ77's
        // original savings decision select its enabled mode. Decode this
        // prefix through the same actual reader before the AC boundary.
        let mut tokens: Vec<(usize, u32)> = (0..self.warmup_count())
            .map(|i| (0, (i % 31) as u32))
            .collect();
        let mut final_coefficient_context = None;
        for (&count, values) in self.counts.iter().zip(&self.values) {
            let predicted = predict_from_top_and_left(
                self.top.then_some(row_top.as_slice()),
                &row,
                self.bx,
                32,
            );
            tokens.push((
                self.ctx_offset + map.non_zero_context(predicted as u32, block_ctx as u32) as usize,
                count as u32,
            ));
            if count > size - covered {
                continue;
            }
            for y in 0..acs.covered_blocks_y() {
                for x in 0..acs.covered_blocks_x() {
                    row[self.bx + x + y * stride] =
                        ((count + covered - 1) >> acs.log2_covered_blocks()) as i32;
                }
            }
            let mut remaining = count;
            let mut prev = usize::from(count <= size / 16);
            for (k, &value) in (covered..size).zip(values) {
                if remaining == 0 {
                    break;
                }
                let density =
                    zero_density_context(remaining, k, covered, acs.log2_covered_blocks(), prev);
                let ctx = self.ctx_offset
                    + map.zero_density_contexts_offset(block_ctx as u32) as usize
                    + density;
                final_coefficient_context = Some((k, density, ctx));
                tokens.push((ctx, value));
                prev = usize::from(value != 0);
                remaining -= prev;
            }
        }
        if self.expect_last_padding_context {
            assert!(!self.custom);
            assert_eq!(self.ctx_offset, 0);
            assert_eq!(block_ctx, 14);
            assert_eq!(block_ctx, map.num_ctxs - 1);
            assert_eq!(
                final_coefficient_context,
                Some((
                    size - 1,
                    473,
                    self.ctx_offset + map.num_ac_contexts() as usize + 15
                )),
                "the final coefficient must select the last ProcessACGlobal padding entry",
            );
            assert_eq!(
                tokens[tokens.len() - 2].0,
                map.num_ac_contexts() as usize + 14
            );
            assert_eq!(
                tokens.last().unwrap().0,
                map.num_ac_contexts() as usize + 15
            );
        }
        let mut request = format!(
            "acg-fixture {} {} {} {} {}",
            u8::from(self.prefix),
            self.lz,
            self.offset,
            self.ctx_offset + map.num_ac_contexts() as usize + 16,
            tokens.len()
        );
        for (ctx, value) in tokens {
            write!(request, " {ctx} {value}").unwrap();
        }
        request
    }
    fn decode(&self, bytes: &[u8]) -> String {
        let mut request = format!(
            "acg-decode {} {} {} {} 2 9 {} {} {} {} {} {} {} {} {} {} {} {} {}",
            self.width,
            self.strategy,
            self.c,
            self.bx,
            self.stride(),
            u8::from(self.top),
            self.seed,
            self.initial,
            u8::from(self.reverse),
            u8::from(self.custom),
            self.ctx_offset,
            self.qf,
            if self.custom { self.qdc } else { 0 },
            hex(bytes),
            self.offset,
            self.shifts.len(),
            self.warmup_count()
        );
        for shift in &self.shifts {
            write!(request, " {shift}").unwrap();
        }
        request
    }
}
fn streams(cases: &[Case]) -> Vec<Vec<u8>> {
    original_observations(&cases.iter().map(Case::fixture).collect::<Vec<_>>())
        .iter()
        .zip(cases)
        .map(|(out, case)| {
            let fields: Vec<_> = out.split_whitespace().collect();
            assert_eq!(
                fields[2],
                if case.lz == 0 { "0" } else { "1" },
                "actual original LZ77 mode"
            );
            assert_eq!(fields[3], if case.prefix { "1" } else { "0" });
            unhex(fields[1])
        })
        .collect()
}
#[test]
fn ac_group_symbols_all_strategies_widths_entropy_modes_orders_and_pass_accumulation() {
    let mut cases = Vec::new();
    for strategy in 0..27 {
        for width in [16, 32] {
            for prefix in [false, true] {
                for lz in [0, 2] {
                    cases.push(Case::new(strategy, width, prefix, lz));
                }
            }
        }
    }
    let bytes = streams(&cases);
    compare(
        cases
            .iter()
            .zip(bytes)
            .map(|(case, bytes)| case.decode(&bytes))
            .collect(),
    );
}
#[test]
fn ac_group_symbols_predictors_channels_zero_full_and_rejected_counts() {
    let mut cases = Vec::new();
    for width in [16, 32] {
        for c in 0..3 {
            for bx in [0, 2] {
                for top in [false, true] {
                    for kind in 0..5 {
                        let mut case =
                            Case::new(if kind == 4 { 4 } else { 0 }, width, kind % 2 != 0, 0);
                        case.c = c;
                        case.bx = bx;
                        case.top = top;
                        case.custom = true;
                        case.counts = vec![match kind {
                            0 => 0,
                            1 => 1,
                            2 => 63,
                            3 => 64,
                            _ => 3,
                        }];
                        case.values = vec![match kind {
                            0 | 3 => vec![],
                            1 => vec![u32::MAX],
                            2 => vec![2; 63],
                            _ => vec![2; 3],
                        }];
                        case.shifts = vec![if width == 16 { 16 } else { 20 }];
                        cases.push(case);
                    }
                }
            }
        }
    }
    // A valid initial count that cannot be exhausted keeps row scatter and
    // every decoded addition before the final failure. The largest default
    // block context also reaches the source's invalid-stream density limit.
    for strategy in [0, 18, 19] {
        for prefix in [false, true] {
            let mut case = Case::new(strategy, 32, prefix, 0);
            let log = AcStrategy::from_raw_strategy(AcStrategyType(strategy)).log2_covered_blocks();
            let count = (64 << log) - (1 << log);
            case.c = 2;
            if strategy == 19 {
                case.custom = false;
                case.ctx_offset = 0;
                case.expect_last_padding_context = true;
            }
            case.counts = vec![count];
            case.values = vec![{
                let mut values = vec![0; count];
                values[0] = 2;
                if strategy == 19 {
                    // Keep prev=1 for the final symbol while remaining nzeros
                    // still rounds to 63, selecting density 473 in context 14.
                    values[count - 2] = 2;
                }
                values
            }];
            case.shifts = vec![0];
            cases.push(case);
        }
    }
    // Positive initial values keep INT64_MIN coefficients inside the source
    // defined ptrdiff_t addition domain while testing signed narrowing.
    for width in [16, 32] {
        for prefix in [false, true] {
            for shift in [0, 1, 15, 16, 20, 31, 32, 48, 63] {
                let mut case = Case::new(0, width, prefix, 0);
                case.initial = 100;
                case.counts = vec![4];
                case.values = vec![vec![2, 3, u32::MAX - 1, u32::MAX]];
                case.shifts = vec![shift];
                cases.push(case);
            }
        }
    }
    let bytes = streams(&cases);
    compare(
        cases
            .iter()
            .zip(bytes)
            .map(|(case, bytes)| case.decode(&bytes))
            .collect(),
    );
}
#[test]
fn ac_group_symbols_truncated_headers_payloads_and_final_reader_state() {
    let cases: Vec<_> = [false, true]
        .into_iter()
        .flat_map(|prefix| {
            [0, 2]
                .into_iter()
                .map(move |lz| Case::new(0, 32, prefix, lz))
        })
        .collect();
    let streams = streams(&cases);
    let mut requests = Vec::new();
    for (case, bytes) in cases.iter().zip(streams) {
        for len in 0..=bytes.len() {
            requests.push(case.decode(&bytes[..len]));
        }
    }
    compare(requests);
}
