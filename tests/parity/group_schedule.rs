// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Real original encoder streams exercise the original scheduler and native
// translation. Sparse observations retain every changed initialized sample.
use super::*;
use jxl::jxl::ac_strategy::{AcStrategy, AcStrategyType};
use jxl::jxl::base::rect::Rect;
use jxl::jxl::coeff_order::{COEFF_ORDER_MAX_SIZE, COEFF_ORDER_OFFSET, STRATEGY_ORDER};
use jxl::jxl::dct_util::AcPtrMut;
use jxl::jxl::dec_ans::{ANSCode, decode_histograms};
use jxl::jxl::dec_cache::{GroupDecCache, PassesDecoderState};
use jxl::jxl::dec_group::{GetBlock, GetBlockFromBitstream};
use jxl::jxl::frame_header::FrameHeader;
use jxl::jxl::image::Plane;

pub(super) fn handles(kind: &str) -> bool {
    kind == "gsc-cache" || kind == "gsc-decode"
}
fn coeffs<T: Copy + PartialEq + std::fmt::Display>(
    out: &mut String,
    block: &[T],
    initial: impl Fn(usize) -> T,
) {
    let changed = block
        .iter()
        .enumerate()
        .filter(|&(i, v)| *v != initial(i))
        .count();
    write!(out, " {} {changed}", block.len()).unwrap();
    for (i, &v) in block.iter().enumerate() {
        if v != initial(i) {
            write!(out, " {i} {v}").unwrap();
        }
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut out = String::new();
    if kind == "gsc-cache" {
        let count = number(args);
        let mut cache = GroupDecCache::default();
        for _ in 0..count {
            let passes = number(args);
            let used = number(args);
            let status = cache.init_once(passes, used);
            let sizes = cache.storage_sizes();
            write!(
                out,
                "{} {} {} {} {} {} {}",
                status_code(&status),
                sizes[3],
                u8::from(cache.dec_group_block.is_some()),
                u8::from(cache.dec_group_qblock.is_some()),
                u8::from(cache.dec_group_qblock16.is_some()),
                u8::from(cache.scratch_space.is_some()),
                cache.scratch_space.unwrap_or(0)
            )
            .unwrap();
            for image in &cache.num_nzeroes {
                write!(
                    out,
                    " {} {} {}",
                    image.xsize(),
                    image.ysize(),
                    image.pixels_per_row()
                )
                .unwrap();
            }
            out.push_str(" ; ");
        }
        return out;
    }
    let width: usize = number(args);
    let raw: u32 = number(args);
    let first: usize = number(args);
    let passes: usize = number(args);
    let sampling: usize = number(args);
    let order_mode: u8 = number(args);
    let x0 = number(args);
    let y0 = number(args);
    let seed: usize = number(args);
    let calls = number(args);
    let histograms = number(args);
    let mut header = FrameHeader::new(None);
    header
        .chroma_subsampling
        .set(
            &[if sampling != 0 { 2 } else { 1 }, 1, 1],
            &[if sampling == 2 { 2 } else { 1 }, 1, 1],
        )
        .unwrap();
    header.passes.num_passes = (first + passes) as u32;
    for p in 0..first + passes {
        header.passes.shift[p] = number(args);
    }
    let mut state = PassesDecoderState::new();
    let shared = &mut state.shared_storage;
    shared.num_histograms = histograms;
    shared.coeff_order_size = COEFF_ORDER_MAX_SIZE;
    shared
        .coeff_orders
        .resize((first + passes) * COEFF_ORDER_MAX_SIZE, 0);
    let acs = AcStrategy::from_raw_strategy(AcStrategyType(raw));
    let size = 64 << acs.log2_covered_blocks();
    let covered = size / 64;
    for p in 0..first + passes {
        for c in 0..3 {
            let offset = p * COEFF_ORDER_MAX_SIZE
                + COEFF_ORDER_OFFSET[3 * usize::from(STRATEGY_ORDER[raw as usize]) + c] * 64;
            let order = &mut shared.coeff_orders[offset..offset + size];
            acs.compute_natural_coeff_order(order);
            if order_mode != 0 && p % 2 != 0 {
                order[covered..].reverse();
            }
        }
    }
    shared.raw_quant_field = Plane::create(x0 + 32, y0 + 32).unwrap();
    shared.quant_dc = Plane::create(x0 + 32, y0 + 32).unwrap();
    for y in 0..y0 + 32 {
        shared.raw_quant_field.row_mut(y)[..x0 + 32].fill(1);
        shared.quant_dc.row_mut(y)[..x0 + 32].fill(0);
    }
    state.code.resize_with(first + passes, ANSCode::default);
    state.context_map.resize_with(first + passes, Vec::new);
    let mut bytes = Vec::new();
    let mut offsets = Vec::new();
    for _ in 0..passes {
        bytes.push(unhex(args.next().unwrap()));
        offsets.push(number(args));
    }
    let mut readers: Vec<_> = bytes.iter().map(|bytes| BitReader::new(bytes)).collect();
    let mut hist_ok = true;
    for p in 0..passes {
        readers[p].skip_bits(offsets[p]);
        let status = decode_histograms(
            &mut readers[p],
            histograms * shared.block_ctx_map.num_ac_contexts() as usize + 16,
            &mut state.code[p + first],
            &mut state.context_map[p + first],
            false,
        );
        write!(
            out,
            "{} {} ; ",
            status_code(&status),
            readers[p].total_bits_consumed()
        )
        .unwrap();
        hist_ok &= status.is_ok();
    }
    let mut cache = GroupDecCache::default();
    cache.init_once(passes, 1usize << raw).unwrap();
    for p in 0..passes {
        for c in 0..3 {
            for y in 0..32 {
                for x in 0..32 {
                    cache.num_nzeroes[p].plane_row_mut(c, y)[x] =
                        ((p * 7 + c * 13 + y * 5 + x + seed) % 64) as i32;
                }
            }
        }
    }
    let mut b16: [Vec<i16>; 3] = std::array::from_fn(|_| {
        (0..size)
            .map(|i| (seed as i32 + (i % 7) as i32) as i16)
            .collect()
    });
    let mut b32: [Vec<i32>; 3] =
        std::array::from_fn(|_| (0..size).map(|i| seed as i32 + (i % 7) as i32).collect());
    let init = if hist_ok {
        GetBlockFromBitstream::init(
            &header,
            &mut readers,
            passes,
            if histograms == 1 {
                0
            } else {
                usize::BITS as usize - (histograms - 1).leading_zeros() as usize
            },
            Rect::new(x0, y0, 32, 32),
            &mut cache.num_nzeroes,
            shared,
            &state.code,
            &state.context_map,
            first,
        )
    } else {
        Err(StatusCode::GenericError)
    };
    write!(
        out,
        "{} I",
        match &init {
            Ok(_) => 0,
            Err(e) => *e as i32,
        }
    )
    .unwrap();
    let mut status = Ok(());
    let mut final_state = false;
    match init {
        Ok(mut get) => {
            for _ in 0..calls {
                let start = number(args);
                let bx = number(args);
                let by = number(args);
                if status.is_ok() {
                    get.start_row(start);
                    if width == 16 {
                        let [b0, b1, b2] = &mut b16;
                        status = get.load_block(
                            bx,
                            by,
                            acs,
                            size,
                            acs.log2_covered_blocks(),
                            &mut [AcPtrMut::I16(b0), AcPtrMut::I16(b1), AcPtrMut::I16(b2)],
                        );
                    } else {
                        let [b0, b1, b2] = &mut b32;
                        status = get.load_block(
                            bx,
                            by,
                            acs,
                            size,
                            acs.log2_covered_blocks(),
                            &mut [AcPtrMut::I32(b0), AcPtrMut::I32(b1), AcPtrMut::I32(b2)],
                        );
                    }
                }
            }
            if status.is_ok() {
                final_state = get.check_ans_final_state().is_ok();
            }
        }
        Err(error) => {
            status = Err(error);
            for _ in 0..calls {
                for _ in 0..3 {
                    let _: usize = number(args);
                }
            }
        }
    }
    write!(out, " {} {}", status_code(&status), u8::from(final_state)).unwrap();
    for c in 0..3 {
        if width == 16 {
            coeffs(&mut out, &b16[c], |i| (seed as i32 + (i % 7) as i32) as i16);
        } else {
            coeffs(&mut out, &b32[c], |i| seed as i32 + (i % 7) as i32);
        }
    }
    out.push_str(" N");
    for p in 0..passes {
        for c in 0..3 {
            let mut values = Vec::new();
            for y in 0..32 {
                values.extend_from_slice(&cache.num_nzeroes[p].plane_row(c, y)[..32]);
            }
            let changed = values
                .iter()
                .enumerate()
                .filter(|&(i, &v)| {
                    v != ((p * 7 + c * 13 + (i / 32) * 5 + i % 32 + seed) % 64) as i32
                })
                .count();
            write!(out, " {changed}").unwrap();
            for (i, &v) in values.iter().enumerate() {
                if v != ((p * 7 + c * 13 + (i / 32) * 5 + i % 32 + seed) % 64) as i32 {
                    write!(out, " {i} {v}").unwrap();
                }
            }
        }
    }
    out.push_str(" R");
    for reader in &mut readers {
        write!(
            out,
            " {} {}",
            reader.total_bits_consumed(),
            status_code(&reader.close())
        )
        .unwrap();
    }
    out
}

fn fixture(
    prefix: bool,
    lz: usize,
    offset: usize,
    histograms: usize,
    selected: usize,
    values: &[u32],
) -> String {
    let mut request = format!(
        "gsc-fixture {} {lz} {offset} {histograms} {selected} {}",
        u8::from(prefix),
        values.len()
    );
    for value in values {
        write!(request, " {value}").unwrap();
    }
    let original = original_observations(&[request]);
    let parts: Vec<_> = original[0].split_whitespace().collect();
    if lz != 0 {
        assert_eq!(parts[2], "1", "the original fixture must select real LZ77");
    }
    assert_eq!(parts[3], if prefix { "1" } else { "0" });
    parts[1].to_owned()
}
#[allow(clippy::too_many_arguments)]
fn request(
    width: usize,
    raw: u32,
    first: usize,
    passes: usize,
    sampling: usize,
    order: bool,
    histograms: usize,
    selected: usize,
    prefix: bool,
    lz: usize,
    coords: &[(usize, usize, usize)],
    zero: bool,
) -> String {
    let mut out = format!(
        "gsc-decode {width} {raw} {first} {passes} {sampling} {} 32 32 29 {} {histograms}",
        u8::from(order),
        coords.len()
    );
    for p in 0..first + passes {
        write!(out, " {}", p % 4).unwrap();
    }
    let mut streams = Vec::new();
    for pass in 0..passes {
        let mut values = Vec::new();
        for (call, &(_, bx, by)) in coords.iter().enumerate() {
            for c in [1, 0, 2] {
                let h = usize::from(sampling != 0 && c != 1);
                let v = usize::from(sampling == 2 && c != 1);
                if (bx >> h) << h != bx || (by >> v) << v != by {
                    continue;
                }
                values.push(u32::from(!zero));
                if !zero {
                    let position = if lz == 0 { call } else { call % 32 };
                    values.push((2 * (pass * 7 + c * 3 + position + 1)) as u32);
                }
            }
        }
        let offset = (pass * 3 + 1) % 8;
        streams.push((
            fixture(prefix, lz, offset, histograms, selected, &values),
            offset,
        ));
    }
    for (bytes, offset) in streams {
        write!(out, " {bytes} {offset}").unwrap();
    }
    for &(start, bx, by) in coords {
        write!(out, " {start} {bx} {by}").unwrap();
    }
    out
}
#[test]
fn group_cache_capacity_and_pass_image_lifecycle_matches_original() {
    let mut cases = Vec::new();
    for raw in 0..27 {
        cases.push(format!(
            "gsc-cache 4 1 {} 8 {} 3 1 2 {}",
            1usize << raw,
            1usize << raw,
            1usize << 26
        ));
    }
    cases.push(format!(
        "gsc-cache 3 8 {} 1 1 4 {}",
        (1usize << 27) - 1,
        1usize << 63
    ));
    compare(cases);
}
#[test]
fn all_strategy_widths_channel_pass_orders_and_first_pass_match_original() {
    let mut cases = Vec::new();
    for raw in 0..27 {
        for width in [16, 32] {
            for prefix in [false, true] {
                cases.push(request(
                    width,
                    raw,
                    1,
                    3,
                    0,
                    true,
                    3,
                    2,
                    prefix,
                    0,
                    &[(0, 0, 0)],
                    false,
                ));
            }
        }
    }
    compare(cases);
}
#[test]
fn row_prediction_subsampling_histogram_selection_and_all_pass_counts_match_original() {
    let mut cases = Vec::new();
    let coords = [(0, 0, 0), (0, 1, 0), (1, 0, 1), (1, 1, 1), (2, 2, 2)];
    for passes in 1..=8 {
        for sampling in 0..3 {
            cases.push(request(
                32,
                0,
                0,
                passes,
                sampling,
                true,
                3,
                1,
                passes % 2 == 0,
                0,
                &coords,
                false,
            ));
        }
    }
    for width in [16, 32] {
        for prefix in [false, true] {
            cases.push(request(
                width, 0, 2, 2, 0, false, 3, 3, prefix, 0, &coords, false,
            ));
            let full = request(width, 0, 2, 2, 0, true, 1, 0, prefix, 0, &coords, false);
            let mut parts: Vec<String> = full.split_whitespace().map(str::to_owned).collect();
            // Header then first+passes shift values, followed by each stream/offset.
            let stream = 12 + 4;
            let bytes = unhex(&parts[stream]);
            for keep in [0, bytes.len() / 2, bytes.len() - 1] {
                parts[stream] = hex(&bytes[..keep]);
                cases.push(parts.join(" "));
            }
        }
    }
    for width in [16, 32] {
        for prefix in [false, true] {
            let base = request(
                width,
                0,
                1,
                3,
                0,
                true,
                1,
                0,
                prefix,
                0,
                &[(0, 0, 0)],
                false,
            );
            let mut parts: Vec<String> = base.split_whitespace().map(str::to_owned).collect();
            // Source c=Y/pass0 changes before invalid Y/pass1 stops every later channel/pass.
            parts[18] = fixture(prefix, 0, 4, 1, 0, &[64, 1, 2, 1, 2]);
            let malformed = parts.join(" ");
            assert!(original_observations(std::slice::from_ref(&malformed))[0].contains(" I 1 0 "));
            cases.push(malformed);
            let mut unfinished = vec![1];
            unfinished.resize(64, 0);
            parts[18] = fixture(prefix, 0, 4, 1, 0, &unfinished);
            cases.push(parts.join(" "));
            if !prefix {
                let mut parts: Vec<String> = base.split_whitespace().map(str::to_owned).collect();
                // Correct blocks with a real unread ANS token must fail the final checksum.
                parts[16] = fixture(false, 0, 1, 1, 0, &[1, 2, 1, 4, 1, 6, 13]);
                let unfinished = parts.join(" ");
                assert!(
                    original_observations(std::slice::from_ref(&unfinished))[0].contains(" I 0 0 ")
                );
                cases.push(unfinished);
            }
        }
    }
    for prefix in [false, true] {
        cases.push(request(
            32, 0, 0, 3, 0, false, 1, 0, prefix, 0, &coords, true,
        ));
    }
    let coords: Vec<_> = (0..32)
        .flat_map(|y| (0..32).map(move |x| (y, x, y)))
        .collect();
    for prefix in [false, true] {
        cases.push(request(
            32, 0, 0, 2, 0, false, 1, 0, prefix, 2, &coords, false,
        ));
    }
    compare(cases);
}
