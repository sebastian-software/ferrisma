// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};

pub(super) fn handles(kind: &str) -> bool {
    kind == "finalize-frame"
}
pub(super) fn observe(_: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let w = number(args);
    let h = number(args);
    let gray = number::<u32>(args) != 0;
    let depth = if number::<u32>(args) == 8 {
        LosslessSampleDepth::Uint8
    } else {
        LosslessSampleDepth::Uint16
    };
    let has_aux = number::<u32>(args) != 0;
    let mut aux = aux_stats::seed(number(args));
    let config = LosslessThunderConfig::new(w, h, gray, depth).unwrap();
    let mut frame = config.create_frame().unwrap();
    let n = number(args);
    let mut groups = Vec::new();
    for _ in 0..n {
        let mut code = BitWriter::new();
        let bytes = unhex(args.next().unwrap());
        code.append_byte_aligned(&bytes).unwrap();
        prefix(&mut code, number(args));
        groups.push(code);
    }
    let result = frame.finalize_groups(&groups, if has_aux { Some(&mut aux) } else { None });
    let mut out = result.as_ref().err().map_or(0, |s| *s as i32).to_string();
    if let Ok(writer) = result {
        write!(out, " {} {}", writer.bits_written(), hex(writer.get_span())).unwrap();
    }
    write!(out, " {}", u32::from(frame.header().all_default)).unwrap();
    aux_stats::output(&mut out, &aux);
    out
}

#[test]
fn frame_finalization_matches_original_header_toc_group_bytes_and_pre_toc_alignment_errors() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    for (w, h) in [(1, 1), (17, 9), (257, 3), (2049, 5)] {
        for gray in [0, 1] {
            for depth in [8, 16] {
                for has_aux in [0, 1] {
                    for n in [0, 1, 3, 11] {
                        let mut request =
                            format!("finalize-frame {w} {h} {gray} {depth} {has_aux} 7 {n}");
                        for i in 0..n {
                            let bytes: Vec<_> =
                                (0..i * 29).map(|j| (j * 137 + i * 17) as u8).collect();
                            write!(request, " {} 0", hex(&bytes)).unwrap();
                        }
                        cases.push(request.clone());
                        if n != 0 {
                            request.pop();
                            request.push('5');
                            cases.push(request);
                        }
                    }
                }
            }
        }
    }
    // Exercise the original TOC distribution transitions using actual storage.
    for size in [1023, 1024, 17407, 17408, 4211711, 4211712] {
        cases.push(format!(
            "finalize-frame 17 9 0 8 1 0 1 {} 0",
            hex(&vec![0x69; size])
        ));
    }
    compare(cases);
}
