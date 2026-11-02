// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Signature and container-header helpers from `lib/jxl/decode.cc`.
//! Public constants follow `lib/include/jxl/decode.h`; the incremental decoder
//! state machine and C ABI entry points are separate, unfinished boundaries.

use super::common::JXL_SIGNATURE_BOX;
use super::headers::CODESTREAM_MARKER;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlSignature(pub u32);
impl JxlSignature {
    pub const NOT_ENOUGH_BYTES: Self = Self(0);
    pub const INVALID: Self = Self(1);
    pub const CODESTREAM: Self = Self(2);
    pub const CONTAINER: Self = Self(3);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlDecoderStatus(pub u32);
impl JxlDecoderStatus {
    pub const SUCCESS: Self = Self(0);
    pub const ERROR: Self = Self(1);
    pub const NEED_MORE_INPUT: Self = Self(2);
    pub const NEED_PREVIEW_OUT_BUFFER: Self = Self(3);
    pub const NEED_IMAGE_OUT_BUFFER: Self = Self(5);
    pub const JPEG_NEED_MORE_OUTPUT: Self = Self(6);
    pub const BOX_NEED_MORE_OUTPUT: Self = Self(7);
    pub const BASIC_INFO: Self = Self(0x40);
    pub const COLOR_ENCODING: Self = Self(0x100);
    pub const PREVIEW_IMAGE: Self = Self(0x200);
    pub const FRAME: Self = Self(0x400);
    pub const FULL_IMAGE: Self = Self(0x1000);
    pub const JPEG_RECONSTRUCTION: Self = Self(0x2000);
    pub const BOX: Self = Self(0x4000);
    pub const FRAME_PROGRESSION: Self = Self(0x8000);
    pub const BOX_COMPLETE: Self = Self(0x10000);
}

pub fn out_of_bounds(a: usize, b: usize, size: usize) -> bool {
    let pos = a.wrapping_add(b);
    if pos > size {
        return true;
    }
    if pos < a {
        return true;
    }
    false
}

pub fn initial_basic_info_size_hint() -> usize {
    let container_header_size = 48;
    let max_codestream_basic_info_size = 50;
    container_header_size + max_codestream_basic_info_size
}

pub fn read_signature(buf: &[u8], pos: &mut usize) -> JxlSignature {
    if *pos >= buf.len() {
        return JxlSignature::NOT_ENOUGH_BYTES;
    }
    let buf = &buf[*pos..];
    if buf[0] == 0xff {
        if buf.len() < 2 {
            return JxlSignature::NOT_ENOUGH_BYTES;
        } else if buf[1] == CODESTREAM_MARKER {
            *pos += 2;
            return JxlSignature::CODESTREAM;
        } else {
            return JxlSignature::INVALID;
        }
    }
    if buf[0] == 0 {
        if buf.len() < JXL_SIGNATURE_BOX.len() {
            return JxlSignature::NOT_ENOUGH_BYTES;
        } else if buf[..JXL_SIGNATURE_BOX.len()] == JXL_SIGNATURE_BOX {
            *pos += JXL_SIGNATURE_BOX.len();
            return JxlSignature::CONTAINER;
        } else {
            return JxlSignature::INVALID;
        }
    }
    JxlSignature::INVALID
}

pub fn jxl_signature_check(buf: &[u8]) -> JxlSignature {
    let mut pos = 0;
    read_signature(buf, &mut pos)
}

/// Source output parameters remain unchanged until their original assignments.
/// Incomplete extended headers retain type, size=1, and header_size=16.
pub fn parse_box_header(
    input: &[u8],
    mut pos: usize,
    file_pos: usize,
    box_type: &mut [u8; 4],
    box_size: &mut usize,
    header_size: &mut usize,
) -> JxlDecoderStatus {
    if out_of_bounds(pos, 8, input.len()) {
        *header_size = 8;
        return JxlDecoderStatus::NEED_MORE_INPUT;
    }
    let box_start = pos;
    *box_size = u32::from_be_bytes(input[pos..pos + 4].try_into().unwrap()) as usize;
    pos += 4;
    box_type.copy_from_slice(&input[pos..pos + 4]);
    pos += 4;
    if *box_size == 1 {
        *header_size = 16;
        if out_of_bounds(pos, 8, input.len()) {
            return JxlDecoderStatus::NEED_MORE_INPUT;
        }
        let box_size_64 = u64::from_be_bytes(input[pos..pos + 8].try_into().unwrap());
        pos += 8;
        *box_size = box_size_64 as usize;
        if box_size_64 != *box_size as u64 {
            return JxlDecoderStatus::ERROR;
        }
    }
    *header_size = pos - box_start;
    if *box_size > 0 && *box_size < *header_size {
        return JxlDecoderStatus::ERROR;
    }
    if file_pos.wrapping_add(*box_size) < file_pos {
        return JxlDecoderStatus::ERROR;
    }
    JxlDecoderStatus::SUCCESS
}
