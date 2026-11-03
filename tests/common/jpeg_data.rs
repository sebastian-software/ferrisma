// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Data-only observation protocol, shared by differential and native capture tests.
use super::{number, unhex};
use jxl::jxl::jpeg::jpeg_data::*;
fn byte_vectors(args: &mut std::str::SplitWhitespace<'_>) -> Vec<Vec<u8>> {
    let n: usize = number(args);
    (0..n).map(|_| unhex(args.next().unwrap())).collect()
}
pub fn read_data(args: &mut std::str::SplitWhitespace<'_>) -> JpegData {
    let mut j = JpegData {
        width: number(args),
        height: number(args),
        restart_interval: number(args),
        app_data: byte_vectors(args),
        com_data: byte_vectors(args),
        ..Default::default()
    };
    let n: usize = number(args);
    for _ in 0..n {
        let mut q = JpegQuantTable {
            precision: number(args),
            index: number(args),
            is_last: number::<u8>(args) != 0,
            ..Default::default()
        };
        for v in &mut q.values {
            *v = number(args);
        }
        j.quant.push(q);
    }
    let n: usize = number(args);
    for _ in 0..n {
        let mut h = JpegHuffmanCode {
            slot_id: number(args),
            is_last: number::<u8>(args) != 0,
            ..Default::default()
        };
        for v in &mut h.counts {
            *v = number(args);
        }
        for v in &mut h.values {
            *v = number(args);
        }
        j.huffman_code.push(h);
    }
    let n: usize = number(args);
    for _ in 0..n {
        let mut c = JpegComponent {
            id: number(args),
            h_samp_factor: number(args),
            v_samp_factor: number(args),
            quant_idx: number(args),
            width_in_blocks: number(args),
            height_in_blocks: number(args),
            ..Default::default()
        };
        let n: usize = number(args);
        c.coeffs = (0..n).map(|_| number(args)).collect();
        j.components.push(c);
    }
    let n: usize = number(args);
    for _ in 0..n {
        let mut s = JpegScanInfo {
            ss: number(args),
            se: number(args),
            ah: number(args),
            al: number(args),
            num_components: number(args),
            ..Default::default()
        };
        for c in &mut s.components[..s.num_components as usize] {
            c.comp_idx = number(args);
            c.dc_tbl_idx = number(args);
            c.ac_tbl_idx = number(args);
        }
        let n: usize = number(args);
        s.reset_points = (0..n).map(|_| number(args)).collect();
        let n: usize = number(args);
        s.extra_zero_runs = (0..n)
            .map(|_| ExtraZeroRunInfo {
                block_idx: number(args),
                num_extra_zero_runs: number(args),
            })
            .collect();
        j.scan_info.push(s);
    }
    j.marker_order = unhex(args.next().unwrap());
    j.inter_marker_data = byte_vectors(args);
    j.tail_data = unhex(args.next().unwrap());
    j.has_zero_padding_bit = number::<u8>(args) != 0;
    j.padding_bits = unhex(args.next().unwrap());
    let allocated = number::<u8>(args) != 0;
    if allocated && j.padding_bits.is_empty() {
        j.padding_bits.reserve(1);
    }
    j
}
