// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#![allow(dead_code)]
#[path = "../src/brotli/dictionary.rs"]
mod dictionary;
#[path = "../src/brotli/memory.rs"]
mod memory;
#[path = "../src/brotli/raw_bytes.rs"]
mod raw_bytes;
#[path = "../src/brotli/raw_transform.rs"]
mod raw_transform;
#[path = "../src/brotli/tables.rs"]
mod tables;
#[path = "../src/brotli/transform.rs"]
mod transform;

use memory::Allocator;
use raw_transform::RawByteDestination;
use std::io::{self, Write};

fn main() {
    let mode = std::env::args().nth(1).expect("seeded or unknown");
    assert!(mode == "seeded" || mode == "unknown");
    let dictionary = dictionary::brotli_get_dictionary();
    let transforms = transform::brotli_get_transforms();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    for length in 4..=24usize {
        for index in 0..1usize << dictionary.size_bits_by_length[length] {
            let offset = dictionary.offsets_by_length[length] as usize + index * length;
            let word = &dictionary.data[offset..];
            for transform in 0..transforms.num_transforms {
                // The actual source destination allocation is the only payload.
                let mut owner = Allocator::default().allocate(64).expect("allocation");
                let mut dst = if mode == "seeded" {
                    let seed = length as u32 * 11 + index as u32 * 3 + transform * 17;
                    for (i, byte) in owner.storage_mut().iter_mut().enumerate() {
                        byte.write(seed.wrapping_add(i as u32 * 37) as u8);
                    }
                    // SAFETY: The exact source seed loop initialized all64
                    // bytes in this same owner; its exclusive borrow is live.
                    unsafe { RawByteDestination::with_initialized_prefix(owner.storage_mut(), 64) }
                } else {
                    RawByteDestination::new(owner.storage_mut())
                };
                let result = transform::brotli_transform_dictionary_word_to(
                    &mut dst,
                    word,
                    length as i32,
                    transforms,
                    transform as i32,
                );
                assert!(result >= 0);
                let result_len = result as usize;
                assert!(result_len <= 64);
                out.write_all(&result.to_le_bytes()).unwrap();
                if mode == "seeded" {
                    assert_eq!(dst.initialized_prefix_len(), 64);
                    out.write_all(dst.initialized_prefix()).unwrap();
                } else {
                    assert_eq!(dst.initialized_prefix_len(), result_len);
                    out.write_all(dst.initialized_prefix()).unwrap();
                }
            }
        }
    }
    out.flush().unwrap();
}
