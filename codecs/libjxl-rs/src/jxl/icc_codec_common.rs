// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Common compressed-profile helpers from lib/jxl/icc_codec_common.{h,cc}.
use super::base::status::{Status, StatusCode};
use super::padded_bytes::PaddedBytes;
pub const ICC_HEADER_SIZE: usize = 128;
pub type Tag = [u8; 4];
pub const ACSP_TAG: Tag = *b"acsp";
pub const BKPT_TAG: Tag = *b"bkpt";
pub const BTRC_TAG: Tag = *b"bTRC";
pub const BXYZ_TAG: Tag = *b"bXYZ";
pub const CHAD_TAG: Tag = *b"chad";
pub const CHRM_TAG: Tag = *b"chrm";
pub const CPRT_TAG: Tag = *b"cprt";
pub const CURV_TAG: Tag = *b"curv";
pub const DESC_TAG: Tag = *b"desc";
pub const DMDD_TAG: Tag = *b"dmdd";
pub const DMND_TAG: Tag = *b"dmnd";
pub const GBD_TAG: Tag = *b"gbd ";
pub const GTRC_TAG: Tag = *b"gTRC";
pub const GXYZ_TAG: Tag = *b"gXYZ";
pub const KTRC_TAG: Tag = *b"kTRC";
pub const KXYZ_TAG: Tag = *b"kXYZ";
pub const LUMI_TAG: Tag = *b"lumi";
pub const MAB_TAG: Tag = *b"mAB ";
pub const MBA_TAG: Tag = *b"mBA ";
pub const MLUC_TAG: Tag = *b"mluc";
pub const MNTR_TAG: Tag = *b"mntr";
pub const PARA_TAG: Tag = *b"para";
pub const RGB_TAG: Tag = *b"RGB ";
pub const RTRC_TAG: Tag = *b"rTRC";
pub const RXYZ_TAG: Tag = *b"rXYZ";
pub const SF32_TAG: Tag = *b"sf32";
pub const TEXT_TAG: Tag = *b"text";
pub const VCGT_TAG: Tag = *b"vcgt";
pub const WTPT_TAG: Tag = *b"wtpt";
pub const XYZ_TAG: Tag = *b"XYZ ";
pub const TAG_STRINGS: [Tag; 17] = [
    CPRT_TAG, WTPT_TAG, BKPT_TAG, RXYZ_TAG, GXYZ_TAG, BXYZ_TAG, KXYZ_TAG, RTRC_TAG, GTRC_TAG,
    BTRC_TAG, KTRC_TAG, CHAD_TAG, DESC_TAG, CHRM_TAG, DMND_TAG, DMDD_TAG, LUMI_TAG,
];
pub const TYPE_STRINGS: [Tag; 8] = [
    XYZ_TAG, DESC_TAG, TEXT_TAG, MLUC_TAG, PARA_TAG, CURV_TAG, SF32_TAG, GBD_TAG,
];
pub const COMMAND_TAG_UNKNOWN: u8 = 1;
pub const COMMAND_TAG_TRC: u8 = 2;
pub const COMMAND_TAG_XYZ: u8 = 3;
pub const COMMAND_TAG_STRING_FIRST: u8 = 4;
pub const COMMAND_INSERT: u8 = 1;
pub const COMMAND_SHUFFLE2: u8 = 2;
pub const COMMAND_SHUFFLE4: u8 = 3;
pub const COMMAND_PREDICT: u8 = 4;
pub const COMMAND_XYZ: u8 = 10;
pub const COMMAND_TYPE_START_FIRST: u8 = 16;
pub const FLAG_BIT_OFFSET: u8 = 64;
pub const FLAG_BIT_SIZE: u8 = 128;
pub const NUM_ICC_CONTEXTS: usize = 41;
pub fn decode_uint32(data: &[u8], pos: usize) -> u32 {
    decode_uint32_sized(data, data.len(), pos)
}
fn decode_uint32_sized(data: &[u8], size: usize, pos: usize) -> u32 {
    if pos.wrapping_add(4) > size {
        0
    } else {
        u32::from_be_bytes(data[pos..pos + 4].try_into().unwrap())
    }
}
pub fn append_uint32(value: u32, data: &mut PaddedBytes) -> Status {
    let pos = data.len();
    data.resize(pos.wrapping_add(4))?;
    data.bytes_mut()[pos..pos + 4].copy_from_slice(&value.to_be_bytes());
    Ok(())
}
pub fn decode_keyword(data: &[u8], pos: usize) -> Tag {
    if pos.wrapping_add(4) > data.len() {
        *b"    "
    } else {
        data[pos..pos + 4].try_into().unwrap()
    }
}
pub fn encode_keyword(keyword: &Tag, data: &mut [u8], pos: usize) {
    if pos.wrapping_add(3) >= data.len() {
        return;
    }
    data[pos..pos + 4].copy_from_slice(keyword);
}
pub fn append_keyword(keyword: &Tag, data: &mut PaddedBytes) -> Status {
    data.append(keyword)
}
pub fn check_out_of_bounds(a: u64, b: u64, size: u64) -> Status {
    let pos = a.wrapping_add(b);
    if pos > size || pos < a {
        Err(StatusCode::GenericError)
    } else {
        Ok(())
    }
}
pub fn check_is_32_bit(value: u64) -> Status {
    if value & !0xffff_ffff != 0 {
        Err(StatusCode::GenericError)
    } else {
        Ok(())
    }
}
pub fn icc_initial_header_prediction(size: u32) -> [u8; ICC_HEADER_SIZE] {
    let mut header = [
        0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, b'm', b'n', b't', b'r', b'R', b'G', b'B', b' ', b'X',
        b'Y', b'Z', b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b'a', b'c', b's', b'p', 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 246, 214, 0,
        1, 0, 0, 0, 0, 211, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    header[..4].copy_from_slice(&size.to_be_bytes());
    header
}
pub fn icc_predict_header(icc: &[u8], header: &mut [u8; ICC_HEADER_SIZE], pos: usize) {
    if pos == 8 && icc.len() >= 8 {
        header[80..84].copy_from_slice(&icc[4..8]);
    }
    if pos == 41 && icc.len() >= 41 {
        if icc[40] == b'A' {
            header[41..44].copy_from_slice(b"PPL");
        }
        if icc[40] == b'M' {
            header[41..44].copy_from_slice(b"SFT");
        }
    }
    if pos == 42 && icc.len() >= 42 {
        if icc[40] == b'S' && icc[41] == b'G' {
            header[42..44].copy_from_slice(b"I ");
        }
        if icc[40] == b'S' && icc[41] == b'U' {
            header[42..44].copy_from_slice(b"NW");
        }
    }
}
fn predict_value(p1: u32, p2: u32, p3: u32, order: i32) -> u32 {
    match order {
        0 => p1,
        1 => p1.wrapping_mul(2).wrapping_sub(p2),
        2 => p1
            .wrapping_mul(3)
            .wrapping_sub(p2.wrapping_mul(3))
            .wrapping_add(p3),
        _ => 0,
    }
}
pub fn linear_predict_icc_value(
    data: &[u8],
    start: usize,
    i: usize,
    stride: usize,
    width: usize,
    order: i32,
) -> u8 {
    let pos = start.wrapping_add(i);
    if width == 1 {
        predict_value(
            u32::from(data[pos - stride]),
            u32::from(data[pos - stride * 2]),
            u32::from(data[pos - stride * 3]),
            order,
        ) as u8
    } else if width == 2 {
        let p = start.wrapping_add(i & !1);
        let load = |n| {
            let off = p - stride * n;
            u32::from(u16::from_be_bytes([data[off], data[off + 1]]))
        };
        let pred = predict_value(load(1), load(2), load(3), order) as u16;
        if i & 1 != 0 {
            pred as u8
        } else {
            (pred >> 8) as u8
        }
    } else {
        let p = start.wrapping_add(i & !3);
        // The original logical size is the current prediction position. Only
        // the preceding words need physical backing; pos itself may lie beyond
        // that backing in a defined direct helper call.
        let load = |n| decode_uint32_sized(data, pos, p - stride * n);
        let pred = predict_value(load(1), load(2), load(3), order);
        (pred >> ((3 - (i & 3)) * 8)) as u8
    }
}
fn byte_kind1(b: u8) -> usize {
    if b.is_ascii_alphabetic() {
        0
    } else if b.is_ascii_digit() || b == b'.' || b == b',' {
        1
    } else if b == 0 {
        2
    } else if b == 1 {
        3
    } else if b < 16 {
        4
    } else if b == 255 {
        6
    } else if b > 240 {
        5
    } else {
        7
    }
}
fn byte_kind2(b: u8) -> usize {
    if b.is_ascii_alphabetic() {
        0
    } else if b.is_ascii_digit() || b == b'.' || b == b',' {
        1
    } else if b < 16 {
        2
    } else if b > 240 {
        3
    } else {
        4
    }
}
pub fn icc_ans_context(i: usize, b1: usize, b2: usize) -> usize {
    if i <= 128 {
        0
    } else {
        1 + byte_kind1(b1 as u8) + byte_kind2(b2 as u8) * 8
    }
}
