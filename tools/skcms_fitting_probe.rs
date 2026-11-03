// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Standalone protocol adapter linked to the actual Rust crate.
#[path = "../tests/common/skcms_fitting.rs"]
mod adapter;
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        println!("{}", adapter::observe(&line.unwrap()));
    }
}
