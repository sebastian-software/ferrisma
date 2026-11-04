// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Retained observations of the original public C API, replayed on the Rust
// public decoder without the C++ oracle. tools/decoder_public_captures.py
// writes them from the original; the public decoder audit regenerates them.
#[path = "support/decoder_public.rs"]
mod driver;

fn capture(index: usize) {
    let row = include_str!("../tools/captures/decoder_public.tsv")
        .lines()
        .nth(index)
        .unwrap();
    let (request, expected) = row.split_once('\t').unwrap();
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("dpub-run"));
    #[cfg(feature = "cpp-oracle")]
    jxl::jxl::box_content_decoder::take_null_input_copies();
    assert_eq!(driver::observe(&mut args), expected, "{request}");
    // Retained captures stay within the source-defined domain: none reaches
    // the original's undefined null raw-box copy.
    #[cfg(feature = "cpp-oracle")]
    assert_eq!(
        jxl::jxl::box_content_decoder::take_null_input_copies(),
        0,
        "{request}"
    );
}

#[test]
fn raw_codestream_header_events_with_byte_chunks() {
    capture(0);
}
#[test]
fn kept_orientation_gray_basic_info() {
    capture(1);
}
#[test]
fn embedded_icc_profile_getters_and_undersized_output() {
    capture(2);
}
#[test]
fn container_boxes_with_decompressed_brob_and_small_buffers() {
    capture(3);
}
#[test]
fn extra_channel_names_orientation_and_partial_codestream() {
    capture(4);
}
#[test]
fn animation_partial_codestream_and_frame_index_boxes() {
    capture(5);
}
#[test]
fn box_api_misuse_and_rewind_of_level_box_file() {
    capture(6);
}
#[test]
fn subscription_and_input_misuse() {
    capture(7);
}
#[test]
fn input_copy_allocation_failure() {
    capture(8);
}
#[test]
fn uncoalesced_layers_with_extra_channel_buffers() {
    capture(9);
}
#[test]
fn animation_frames_through_multithreaded_callbacks() {
    capture(10);
}
#[test]
fn filtered_vardct_frame_with_cms_output_profile() {
    capture(11);
}
#[test]
fn vardct_groups_through_the_thread_runner() {
    capture(12);
}
#[test]
fn multithreaded_callbacks_through_the_resizable_runner() {
    capture(13);
}
