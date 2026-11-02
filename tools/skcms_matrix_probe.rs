// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Protocol-only adapter linked to the actual Rust matrix implementation.
use jxl::skcms::types::Matrix3x3;
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut args = line.split_whitespace();
        assert_eq!(args.next(), Some("skcms-concat"));
        let alias: u8 = args.next().unwrap().parse().unwrap();
        let mut a = Matrix3x3::default();
        let mut b = Matrix3x3::default();
        for matrix in [&mut a, &mut b] {
            for row in &mut matrix.vals {
                for value in row {
                    *value = f32::from_bits(args.next().unwrap().parse().unwrap());
                }
            }
        }
        assert!(args.next().is_none());
        let output = a.concat(if alias == 0 { &b } else { &a });
        print!("V");
        for row in output.vals {
            for value in row {
                print!(" {}", value.to_bits());
            }
        }
        println!();
    }
}
