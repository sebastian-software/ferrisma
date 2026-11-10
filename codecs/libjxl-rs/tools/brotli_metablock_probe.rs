// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#![allow(dead_code)]
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
#[path = "../src/brotli/bitfield.rs"]
pub mod bitfield;
#[path = "../src/brotli/metablock.rs"]
pub mod metablock;
#[path = "../src/brotli/running.rs"]
pub mod running;
#[path = "../src/brotli/status.rs"]
pub mod status;

use bit_reader::{BitReader, Reg};
use std::fmt::Write as _;
use std::io::{self, BufRead};
use std::mem::MaybeUninit;

pub fn observe(line: &str) -> String {
    let args: Vec<_> = line.split_whitespace().collect();
    assert_eq!(args.len(), 8);
    let op: u32 = args[0].parse().unwrap();
    let seed: u8 = args[1].parse().unwrap();
    let initial_state: u32 = args[2].parse().unwrap();
    let accumulator: Reg = args[3].parse().unwrap();
    let bits: Reg = args[4].parse().unwrap();
    let large: u32 = args[5].parse().unwrap();
    let chunk: usize = args[6].parse().unwrap();
    let hex = args[7];
    let data: Vec<u8> = if hex == "-" {
        Vec::new()
    } else {
        (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect()
    };
    let mut br = BitReader::new();
    br.val = accumulator;
    br.bit_pos = bits;
    let mut word = MaybeUninit::new(u32::from_ne_bytes([seed; 4]));
    // SAFETY: Four aligned exclusively borrowed bytes; initialized here only
    // to preserve the source observer's seeded byte trace, not core state.
    let bytes =
        unsafe { std::slice::from_raw_parts_mut(word.as_mut_ptr().cast::<MaybeUninit<u8>>(), 4) };
    bitfield::BitfieldWord::new(bytes.try_into().unwrap()).write(5, 1, large);
    let mut var_state = MaybeUninit::new(initial_state);
    let mut meta_state = MaybeUninit::new(initial_state);
    let mut counter = MaybeUninit::new(i32::from_ne_bytes([seed; 4]));
    let mut length = MaybeUninit::new(i32::from_ne_bytes([seed; 4]));
    let mut value = MaybeUninit::new(0xdeadbeef as Reg);
    let mut offset = 0;
    let mut output = String::new();
    loop {
        let mut extent = data.len() - offset;
        if chunk != 0 && extent > chunk {
            extent = chunk;
        }
        br.set_input(&data[offset..offset + extent]);
        // SAFETY: This corpus begins with initialized seeded fields. Window
        // rows retain its eight-bit accumulator precondition; varlen/meta
        // rows start at NONE or the unreachable discriminant, then resume
        // only from checkpoints returned by these actual source helpers.
        let result = unsafe {
            match op {
                0 => metablock::decode_window_bits(&mut word, &mut br),
                1 => metablock::decode_var_len_uint8(&mut var_state, &mut br, &mut value),
                2 => metablock::decode_meta_block_length(
                    metablock::MetaBlockFields {
                        substate: &mut meta_state,
                        flags: &mut word,
                        remaining_len: &mut length,
                        loop_counter: &mut counter,
                    },
                    &mut br,
                ),
                _ => panic!("unknown helper"),
            }
        };
        // SAFETY: These observer-only values all began initialized, and source
        // helpers never uninitialize them. No core unknown field is exposed.
        let (v, vs, ms, count, len, flags) = unsafe {
            (
                value.assume_init(),
                var_state.assume_init(),
                meta_state.assume_init(),
                counter.assume_init(),
                length.assume_init(),
                word.assume_init(),
            )
        };
        write!(
            output,
            "{result}:{v}:{}:{}:{}:{vs}:{ms}:{count}:{len}:",
            br.val,
            br.bit_pos,
            offset + br.position()
        )
        .unwrap();
        for byte in flags.to_ne_bytes() {
            write!(output, "{byte:02x}").unwrap();
        }
        output.push(' ');
        offset += extent;
        if result != status::BROTLI_DECODER_NEEDS_MORE_INPUT {
            break;
        }
        assert_eq!(br.available_in(), 0);
        if offset == data.len() {
            break;
        }
    }
    output.push('\n');
    output
}

fn main() {
    for line in io::stdin().lock().lines() {
        print!("{}", observe(&line.unwrap()));
    }
}
