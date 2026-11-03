// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Independent original WriteJpeg captures: minimal D9 marker order, with
// short/failing callbacks and null versus retained empty tail storage.
use jxl::jxl::base::status::{StatusCode, status_code};
use jxl::jxl::jpeg::dec_jpeg_data_writer::write_jpeg;
use jxl::jxl::jpeg::jpeg_data::JpegData;

fn minimal(tail: Vec<u8>) -> JpegData {
    JpegData {
        marker_order: vec![0xd9],
        tail_data: tail,
        ..Default::default()
    }
}
#[test]
fn short_writes_keep_the_original_unadvanced_pointer_and_callback_bytes() {
    let jpg = minimal(vec![1, 2, 3]);
    let mut offered = Vec::new();
    let mut written = Vec::new();
    let status = write_jpeg(&jpg, |data| {
        let bytes = data.unwrap();
        offered.push(bytes.to_vec());
        let n = bytes.len().min(1);
        written.extend_from_slice(&bytes[..n]);
        n
    });
    assert_eq!(status_code(&status), 0);
    assert_eq!(
        offered,
        vec![
            vec![0xff, 0xd8],
            vec![0xff],
            vec![0xff, 0xd9],
            vec![0xff],
            vec![1, 2, 3],
            vec![1, 2],
            vec![1]
        ]
    );
    assert_eq!(written, [0xff, 0xff, 0xff, 0xff, 1, 1, 1]);
}
#[test]
fn a_zero_write_retains_the_original_not_enough_bytes_status_and_prefix() {
    let jpg = minimal(vec![1, 2, 3]);
    let mut offers = Vec::new();
    let mut written = Vec::new();
    let status = write_jpeg(&jpg, |data| {
        let bytes = data.unwrap();
        let n = if offers.is_empty() { bytes.len() } else { 0 };
        offers.push(bytes.to_vec());
        written.extend_from_slice(&bytes[..n]);
        n
    });
    assert_eq!(status, Err(StatusCode::NotEnoughBytes));
    assert_eq!(offers, vec![vec![0xff, 0xd8], vec![0xff, 0xd9]]);
    assert_eq!(written, [0xff, 0xd8]);
}
#[test]
fn null_and_retained_empty_tail_callbacks_remain_distinct() {
    for retained in [false, true] {
        let mut tail = Vec::new();
        if retained {
            tail.reserve(1);
        }
        let jpg = minimal(tail);
        let mut offers = Vec::new();
        let status = write_jpeg(&jpg, |data| {
            offers.push(data.map(<[u8]>::to_vec));
            data.map_or(0, <[u8]>::len)
        });
        assert_eq!(status_code(&status), 0);
        assert_eq!(
            offers,
            vec![
                Some(vec![0xff, 0xd8]),
                Some(vec![0xff, 0xd9]),
                if retained { Some(vec![]) } else { None }
            ]
        );
    }
}
