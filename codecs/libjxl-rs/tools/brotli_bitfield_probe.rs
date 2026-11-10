// Copyright 2015 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
// Standalone native probe for the Rust representation, using actual owned blocks.
#[path = "../src/brotli/bitfield.rs"]
mod bitfield;
#[path = "../src/brotli/memory.rs"]
mod memory;

use std::io::BufRead;
use std::mem::MaybeUninit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let offset: usize = args[2].parse().unwrap();
    let initialized = args.get(3).is_none_or(|arg| arg != "--uninitialized");
    // SAFETY: Both callbacks absent select the independent default allocator;
    // no context pointer or custom callback lifetime is retained.
    let allocator = unsafe { memory::Allocator::from_callbacks(None, None, std::ptr::null_mut()) }.unwrap();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut values = line.split_whitespace().map(|n| n.parse::<u32>().unwrap());
        let seed = values.next().unwrap();
        let count = values.next().unwrap();
        let mut allocation = allocator.allocate(size).unwrap();
        assert_eq!(allocation.len(), size);
        assert!(!allocation.is_empty());
        assert_eq!(allocation.as_ptr() as usize % 4, 0);
        if initialized { allocation.fill(seed as u8); }
        print!("{size} {offset}");
        for _ in 0..count {
            let field = values.next().unwrap();
            let value = values.next().unwrap();
            let (shift, width) = match field {
                0..=5 => (field, 1),
                6 => (6, 6),
                7 => (12, 8),
                _ => panic!("unknown source field"),
            };
            let storage: &mut [MaybeUninit<u8>; 4] = (&mut allocation.storage_mut()
                [offset..offset + 4])
                .try_into()
                .unwrap();
            let mut word = bitfield::BitfieldWord::new(storage);
            word.write(shift, width, value);
            // SAFETY: The immediately preceding write initialized every bit
            // selected for this read, even in the uninitialized-block lane.
            let read = unsafe { word.read(shift, width) };
            print!(" {read}:");
            // SAFETY: fill initialized every byte and the helper retains that
            // initialization without reading unknown bits into Rust scalars.
            if initialized {
                let bytes = unsafe { allocation.initialized_bytes(0..size) };
                for (i, byte) in bytes.iter().enumerate() {
                    if (offset..offset + 4).contains(&i) {
                        print!("{byte:02x}");
                    } else {
                        assert_eq!(*byte, seed as u8, "changed non-bitfield byte");
                    }
                }
            }
        }
        assert!(values.next().is_none());
        println!();
        allocator.release(Some(allocation));
    }
}
