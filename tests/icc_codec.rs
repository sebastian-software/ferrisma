// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::{
    base::status::StatusCode,
    icc_codec::{decode_var_int, shuffle, unpredict_icc},
    padded_bytes::PaddedBytes,
};
// Captured from the pristine original command decoder, independently of Rust.
#[test]
fn original_tenth_varint_overflow_preserves_the_output_and_consumed_position() {
    let input = [255, 255, 255, 255, 255, 255, 255, 255, 255, 2];
    let mut pos = 0;
    let mut out = 42;
    assert_eq!(
        decode_var_int(&input, &mut pos, &mut out),
        Err(StatusCode::GenericError)
    );
    assert_eq!((pos, out), (10, 42));
}
#[test]
fn original_nonrectangular_shuffle_keeps_the_missing_column_positions() {
    let mut data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    shuffle(&mut data, 4).unwrap();
    assert_eq!(data, [0, 3, 6, 9, 1, 4, 7, 2, 5, 8]);
}
#[test]
fn original_unpredict_failure_retains_the_initialized_prefix_and_nonempty_guard() {
    let mut output = PaddedBytes::default();
    assert_eq!(
        unpredict_icc(&[3, 0, 1, 2], &mut output),
        Err(StatusCode::GenericError)
    );
    assert_eq!(output.as_slice(), [1, 2]);
    assert_eq!(
        unpredict_icc(&[0, 0], &mut output),
        Err(StatusCode::GenericError)
    );
    assert_eq!(output.as_slice(), [1, 2]);
    output.clear();
    unpredict_icc(&[0, 0], &mut output).unwrap();
    assert!(output.is_empty());
}
