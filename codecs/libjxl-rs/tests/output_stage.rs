// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "common/output_stage.rs"]
mod rust_observer;

#[test]
fn output_buffers_callbacks_and_partial_owners_match_independent_original_native_captures() {
    let mut count = 0;
    for row in include_str!("data/output-stage-native.tsv").lines() {
        let mut fields = row.splitn(3, '\t');
        let name = fields.next().unwrap();
        let wire = fields.next().unwrap();
        let expected = fields.next().unwrap();
        let mut args = wire.split_whitespace();
        let kind = args.next().unwrap();
        assert!(rust_observer::handles(kind));
        assert_eq!(rust_observer::observe(kind, &mut args), expected, "{name}");
        assert!(args.next().is_none());
        count += 1;
    }
    assert_eq!(count, 9);
}
