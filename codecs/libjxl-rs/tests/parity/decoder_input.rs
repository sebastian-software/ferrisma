// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "../support/decoder_input.rs"]
mod driver;
pub(super) fn handles(kind: &str) -> bool {
    kind == "di-run" || kind == "di-short"
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "di-short" {
        driver::observe_short(args)
    } else {
        driver::observe(args)
    }
}
#[test]
fn decoder_input_persistent_histories_match_original() {
    super::compare(
        include_str!("../../tools/captures/decoder_input_requests.txt")
            .lines()
            .map(str::to_owned)
            .collect(),
    );
}
