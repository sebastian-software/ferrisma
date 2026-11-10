// libwebp v1.6.0, src/dec/quant_dec.c:
//
// Copyright 2010 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Quantizer initialization
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/dec/quant_dec.c` from libwebp v1.6.0.
//!
//! Work package: WP6 (VP8 decoder core).

use crate::dec::common_dec::NUM_MB_SEGMENTS;
use crate::dec::vp8i_dec::VP8Decoder;
use crate::utils::bit_reader_utils::{vp8_get, vp8_get_signed_value, vp8_get_value};

// upstream: src/dec/quant_dec.c clip
fn clip(v: i32, m: i32) -> i32 {
    if v < 0 {
        0
    } else if v > m {
        m
    } else {
        v
    }
}

/// The table entry at the clipped index, which C uses as an array index.
fn at<T: Copy>(table: &[T; 128], index: i32) -> T {
    table[usize::try_from(index).expect("a clipped index")]
}

// upstream: src/dec/quant_dec.c kDcTable
/// DC dequantization factors by quantizer index (Paragraph 14.1).
#[rustfmt::skip]
const DC_TABLE: [u8; 128] = [
    4, 5, 6, 7, 8, 9, 10, 10,
    11, 12, 13, 14, 15, 16, 17, 17,
    18, 19, 20, 20, 21, 21, 22, 22,
    23, 23, 24, 25, 25, 26, 27, 28,
    29, 30, 31, 32, 33, 34, 35, 36,
    37, 37, 38, 39, 40, 41, 42, 43,
    44, 45, 46, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58,
    59, 60, 61, 62, 63, 64, 65, 66,
    67, 68, 69, 70, 71, 72, 73, 74,
    75, 76, 76, 77, 78, 79, 80, 81,
    82, 83, 84, 85, 86, 87, 88, 89,
    91, 93, 95, 96, 98, 100, 101, 102,
    104, 106, 108, 110, 112, 114, 116, 118,
    122, 124, 126, 128, 130, 132, 134, 136,
    138, 140, 143, 145, 148, 151, 154, 157,
];

// upstream: src/dec/quant_dec.c kAcTable
/// AC dequantization factors by quantizer index (Paragraph 14.1).
#[rustfmt::skip]
const AC_TABLE: [u16; 128] = [
    4, 5, 6, 7, 8, 9, 10, 11,
    12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32, 33, 34, 35,
    36, 37, 38, 39, 40, 41, 42, 43,
    44, 45, 46, 47, 48, 49, 50, 51,
    52, 53, 54, 55, 56, 57, 58, 60,
    62, 64, 66, 68, 70, 72, 74, 76,
    78, 80, 82, 84, 86, 88, 90, 92,
    94, 96, 98, 100, 102, 104, 106, 108,
    110, 112, 114, 116, 119, 122, 125, 128,
    131, 134, 137, 140, 143, 146, 149, 152,
    155, 158, 161, 164, 167, 170, 173, 177,
    181, 185, 189, 193, 197, 201, 205, 209,
    213, 217, 221, 225, 229, 234, 239, 245,
    249, 254, 259, 264, 269, 274, 279, 284,
];

//------------------------------------------------------------------------------
// Paragraph 9.6

// upstream: src/dec/quant_dec.c VP8ParseQuant
/// Parses the quantizer indexes from the first partition and fills `dec.dqm`.
/// C reads `dec->br`; `data` is the input that it reads (ADR 0003).
#[expect(
    clippy::cast_possible_wrap,
    reason = "C assigns the unsigned value of at most 7 bits to an int"
)]
#[expect(
    clippy::similar_names,
    reason = "the C names dqy1_dc, dqy2_ac and dquv_ac"
)]
pub fn vp8_parse_quant(dec: &mut VP8Decoder, data: &[u8]) {
    let br = &mut dec.br;
    let base_q0 = vp8_get_value(br, data, 7) as i32;
    let dqy1_dc = if vp8_get(br, data) != 0 {
        vp8_get_signed_value(br, data, 4)
    } else {
        0
    };
    let dqy2_dc = if vp8_get(br, data) != 0 {
        vp8_get_signed_value(br, data, 4)
    } else {
        0
    };
    let dqy2_ac = if vp8_get(br, data) != 0 {
        vp8_get_signed_value(br, data, 4)
    } else {
        0
    };
    let dquv_dc = if vp8_get(br, data) != 0 {
        vp8_get_signed_value(br, data, 4)
    } else {
        0
    };
    let dquv_ac = if vp8_get(br, data) != 0 {
        vp8_get_signed_value(br, data, 4)
    } else {
        0
    };

    let hdr = dec.segment_hdr;
    for i in 0..NUM_MB_SEGMENTS as usize {
        let q = if hdr.use_segment {
            let mut q = i32::from(hdr.quantizer[i]);
            if !hdr.absolute_delta {
                q += base_q0;
            }
            q
        } else if i > 0 {
            dec.dqm[i] = dec.dqm[0];
            continue;
        } else {
            base_q0
        };
        {
            let m = &mut dec.dqm[i];
            m.y1_mat[0] = i32::from(at(&DC_TABLE, clip(q + dqy1_dc, 127)));
            m.y1_mat[1] = i32::from(at(&AC_TABLE, clip(q, 127)));

            m.y2_mat[0] = i32::from(at(&DC_TABLE, clip(q + dqy2_dc, 127))) * 2;
            // For all x in [0..284], x*155/100 is bitwise equal to (x*101581) >> 16.
            // The smallest precision for that is '(x*6349) >> 12' but 16 is a good
            // word size.
            m.y2_mat[1] = (i32::from(at(&AC_TABLE, clip(q + dqy2_ac, 127))) * 101_581) >> 16;
            if m.y2_mat[1] < 8 {
                m.y2_mat[1] = 8;
            }

            m.uv_mat[0] = i32::from(at(&DC_TABLE, clip(q + dquv_dc, 117)));
            m.uv_mat[1] = i32::from(at(&AC_TABLE, clip(q + dquv_ac, 127)));

            m.uv_quant = q + dquv_ac; // for dithering strength evaluation
        }
    }
}
