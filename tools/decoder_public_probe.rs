// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Replays dpub-run requests on the actual Rust public decoder.
extern crate jxl;
#[path = "../tests/support/decoder_public.rs"]
mod driver;
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut args = line.split_whitespace();
        assert_eq!(args.next(), Some("dpub-run"));
        println!("{}", driver::observe(&mut args));
    }
}
