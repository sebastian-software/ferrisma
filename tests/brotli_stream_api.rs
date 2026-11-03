// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Actual original requests/results and source provenance are checked in under
// tools/captures. The durable runner re-observes each under both original graphs.
#[path = "../tools/brotli_stream_api_probe.rs"]
mod probe;
fn captures(group: &str) {
    let mut count = 0;
    for row in include_str!("../tools/captures/brotli_stream_api.tsv").lines() {
        let fields: Vec<_> = row.splitn(3, '\t').collect();
        if fields[0] == group {
            assert_eq!(probe::observe(fields[1]), fields[2], "{}", fields[1]);
            count += 1;
        }
    }
    assert!(count > 0, "missing actual source group {group}");
}
#[test]
fn original_take_output_null_and_nonnull_empty_ranges() {
    captures("take");
}
#[test]
fn original_take_output_size_aliases_actual_size_t_members() {
    captures("alias");
}
#[test]
fn original_raw_dictionary_null_size_limits_and_c_enum_domain() {
    captures("attach");
}
#[test]
fn original_callback_getters_observe_same_live_native_storage() {
    captures("callback");
}
