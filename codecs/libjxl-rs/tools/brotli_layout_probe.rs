// Copyright 2013, 2015, 2017 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
#[path = "../src/brotli/layout.rs"]
pub mod layout;

fn main() {
    for (name, size, align) in layout::source_sizes() {
        print!(" {name}:{size}:{align}");
    }
    for (name, offset) in layout::source_offsets() {
        print!(" {name}:{offset}");
    }
    println!(" FLAGS:{}", layout::flags_offset());
}
