// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::box_content_decoder::JxlBoxContentDecoder;
use jxl::jxl::decode::JxlDecoderStatus;

#[cfg(feature = "cpp-oracle")]
#[path = "../tools/box_content_decoder_probe.rs"]
mod probe;

#[test]
fn constructor_and_drop_need_no_uninitialized_scalar_observation() {
    drop(JxlBoxContentDecoder::new());
    drop(JxlBoxContentDecoder::default());
}

#[test]
fn original_plain_prefix_and_remaining_bytes_are_retained() {
    // Actual source capture `plain` in tools/captures/box_content_decoder.tsv.
    let input = *b"abc";
    let mut output = [0xcc; 4];
    let mut decoder = JxlBoxContentDecoder::new();
    decoder.start_box(false, false, 5);
    let mut next_out = output.as_mut_ptr();
    let mut available_out = 4;
    // SAFETY: Initialized caller storage and cursor/count cells are live and
    // disjoint; StartBox initialized every source field and no callback exists.
    let status = unsafe {
        decoder.process_raw(
            input.as_ptr(),
            input.len(),
            0,
            &mut next_out,
            &mut available_out,
        )
    };
    assert_eq!(status, JxlDecoderStatus::NEED_MORE_INPUT);
    assert_eq!(available_out, 1);
    assert_eq!(output, [b'a', b'b', b'c', 0xcc]);
}

#[cfg(feature = "cpp-oracle")]
fn captures(group: &str) {
    let mut count = 0;
    for row in include_str!("../tools/captures/box_content_decoder.tsv").lines() {
        let mut fields = row.splitn(3, '\t');
        let name = fields.next().unwrap();
        let request = fields.next().unwrap();
        let expected = fields.next().unwrap();
        if name == group {
            assert_eq!(probe::observe(request), expected, "{request}");
            count += 1;
        }
    }
    assert!(count > 0, "missing actual original capture group {group}");
}

#[cfg(feature = "cpp-oracle")]
#[test]
fn original_constructor_plain_and_position_guards() {
    for group in ["constructor", "plain", "guards"] {
        captures(group);
    }
}
#[cfg(feature = "cpp-oracle")]
#[test]
fn original_brob_header_order_and_output_backpressure() {
    for group in ["header", "brob"] {
        captures(group);
    }
}
#[cfg(feature = "cpp-oracle")]
#[test]
fn original_truncation_malformed_trailing_and_reset_partial_state() {
    // decode-test: decode_test.cc's BrobBoxInputIsRestrictedToBoxSize and
    // BrobBoxTruncatedStreamWithExactBoxSizeReturnsError.
    for group in ["truncated", "malformed", "trailing", "reset", "decode-test"] {
        captures(group);
    }
}
