// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::decode::{self as api, JxlDecoderStatus as Status, JxlSignature as Signature};

#[test]
fn incomplete_box_header_preserves_unassigned_outputs() {
    let mut tag = *b"seed";
    let mut size = 99;
    let mut header = 88;
    assert_eq!(
        api::parse_box_header(&[0; 7], 0, 0, &mut tag, &mut size, &mut header),
        Status::NEED_MORE_INPUT
    );
    assert_eq!((tag, size, header), (*b"seed", 99, 8));
    assert_eq!(
        api::parse_box_header(
            &[0, 0, 0, 1, b'j', b'x', b'l', b'c'],
            0,
            0,
            &mut tag,
            &mut size,
            &mut header
        ),
        Status::NEED_MORE_INPUT
    );
    assert_eq!((tag, size, header), (*b"jxlc", 1, 16));
}

#[test]
fn signature_prefix_is_not_rejected_before_original_minimum_length() {
    let mut pos = 1;
    assert_eq!(
        api::read_signature(&[0x91, 0, 0xff, 0xff], &mut pos),
        Signature::NOT_ENOUGH_BYTES
    );
    assert_eq!(pos, 1);
    assert_eq!(
        api::read_signature(&[0x91, 0xff, 0x0a], &mut pos),
        Signature::CODESTREAM
    );
    assert_eq!(pos, 3);
}

#[test]
fn box_file_position_wrap_is_an_error_after_header_outputs() {
    let mut tag = *b"seed";
    let mut size = 99;
    let mut header = 88;
    assert_eq!(
        api::parse_box_header(
            &[0, 0, 0, 16, b'j', b'x', b'l', b'c'],
            0,
            usize::MAX - 15,
            &mut tag,
            &mut size,
            &mut header
        ),
        Status::ERROR
    );
    assert_eq!((tag, size, header), (*b"jxlc", 16, 8));
}
