// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Keep this independent native test in its own crate so the source modules
// are loaded once; every frozen capture/assertion remains unchanged.
#[path = "../tools/brotli_stream_probe.rs"]
mod stream_probe;
#[test]
fn all_fifty_one_frozen_whole_stream_source_captures_are_retained() {
    let mut count = 0;
    for row in include_str!("../tools/captures/brotli_stream.tsv").lines() {
        let fields: Vec<_> = row.splitn(3, '\t').collect();
        assert_eq!(stream_probe::observe(fields[1]), fields[2], "{}", fields[1]);
        count += 1;
    }
    assert_eq!(count, 51);
}
