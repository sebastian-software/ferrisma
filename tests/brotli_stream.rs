// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Every expected record is an actual pinned-original API observation. The
// checked-in recipe/binary/map/command hashes live beside the TSV captures.
#[path = "../tools/brotli_stream_probe.rs"]
mod probe;
fn captures(group: &str) {
    let mut count = 0;
    for line in include_str!("../tools/captures/brotli_stream.tsv").lines() {
        let mut fields = line.splitn(3, '\t');
        let name = fields.next().unwrap();
        let request = fields.next().unwrap();
        let expected = fields.next().unwrap();
        if name == group {
            assert_eq!(probe::observe(request), expected, "{request}");
            count += 1;
        }
    }
    assert!(count > 0, "missing actual source capture group {group}");
}
#[test]
fn original_small_streams_and_partial_headers() {
    captures("small");
}
#[test]
fn original_dictionary_transform_commands() {
    captures("dictionary");
}
#[test]
fn original_history_copy_and_ring_wrap() {
    captures("history");
}
#[test]
fn original_raw_cursor_aliases_and_fatal_unused_arguments() {
    captures("aliases");
}
#[test]
fn original_custom_allocator_failure_checkpoints() {
    captures("allocator");
}
#[test]
fn original_metadata_callback_replacement_and_reload() {
    captures("callbacks");
}
#[test]
fn original_take_output_zero_and_bounded_sizes() {
    captures("take");
}
#[test]
fn original_client_input_storage_replacement() {
    captures("swapped");
}
#[test]
fn original_terminal_and_partial_state_reuse() {
    captures("replay");
}
#[test]
fn original_prepared_raw_compound_dictionary() {
    captures("compound");
}
