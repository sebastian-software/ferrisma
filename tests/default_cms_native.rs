// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// These exact words were captured from both actual pristine source contexts;
// this architecture-scoped test executes Rust without invoking a C++ process.
#![cfg(all(feature = "cpp-oracle", target_arch = "aarch64", target_os = "macos"))]
#[path = "common/default_cms.rs"]
mod observation;

#[test]
fn default_cms_independent_original_arm64_observations() {
    let fixture = include_str!("data/default-cms-native-arm64.tsv");
    let mut count = 0;
    for (index, line) in fixture.lines().enumerate() {
        let (request, expected) = line.split_once('\t').unwrap();
        assert_eq!(
            observation::observe(request),
            expected,
            "native source case {index}"
        );
        count += 1;
    }
    assert_eq!(count, 16);
}
