// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
use bit_reader::{BitReader, Reg};
use std::io::BufRead;

fn main() {
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut args = line.split_whitespace();
        let op: u32 = args.next().unwrap().parse().unwrap();
        let val: Reg = args.next().unwrap().parse().unwrap();
        let bits: Reg = args.next().unwrap().parse().unwrap();
        let n: Reg = args.next().unwrap().parse().unwrap();
        let hex = args.next().unwrap();
        assert!(args.next().is_none());
        let data: Vec<u8> = if hex == "-" {
            vec![]
        } else {
            (0..hex.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
                .collect()
        };
        let mut br = BitReader::new();
        br.set_input(&data);
        br.val = val;
        br.bit_pos = bits;
        let mut value = 0xdeadbeef;
        let mut ok = true;
        let mut output = vec![0xcc; n as usize + 8];
        match op {
            0 => ok = br.warmup(),
            1 => ok = br.pull_byte(),
            2 => ok = br.safe_get_bits(n, &mut value),
            3 => ok = br.safe_read_bits(n, &mut value),
            4 => ok = br.safe_read_bits32(n, &mut value),
            5 => ok = br.safe_read_bits32_slow(n, &mut value),
            6 => br.fill_bit_window(n),
            7 => match n {
                1 => br.fill_bit_window_const::<1>(),
                8 => br.fill_bit_window_const::<8>(),
                16 => br.fill_bit_window_const::<16>(),
                24 => br.fill_bit_window_const::<24>(),
                32 => br.fill_bit_window_const::<32>(),
                _ => panic!("unknown source literal"),
            },
            8 => value = br.get16_bits_unmasked(),
            9 => value = br.get_bits(n),
            10 => {
                value = match n {
                    1 => br.get_bits_const::<1>(),
                    8 => br.get_bits_const::<8>(),
                    16 => br.get_bits_const::<16>(),
                    24 => br.get_bits_const::<24>(),
                    32 => br.get_bits_const::<32>(),
                    _ => panic!("unknown source literal"),
                }
            }
            11 => br.normalize(),
            12 => {
                ok = br.safe_get_bits(n, &mut value);
                br.unload();
            }
            13 => ok = br.jump_to_byte_boundary(),
            14 => br.copy_bytes(&mut output, n as usize),
            15 => {
                let saved = br.save();
                ok = br.safe_read_bits32(n, &mut value);
                br.restore(saved);
            }
            16 => value = br.read_bits24(n),
            17 => value = br.read_bits32(n),
            18 => value = br.take_bits(n),
            19 => br.drop_bits(n),
            20 => br.fill_bit_window16(),
            21 => {
                br.safe_read_bits(3, &mut value);
                let saved = br.save();
                ok = br.safe_read_bits32(n, &mut value);
                br.restore(saved);
            }
            23 => {
                value = match n {
                    1 => br.read_bits24_const::<1>(),
                    8 => br.read_bits24_const::<8>(),
                    16 => br.read_bits24_const::<16>(),
                    24 => br.read_bits24_const::<24>(),
                    _ => panic!("unknown source literal"),
                }
            }
            24 => {
                value = match n {
                    1 => br.read_bits32_const::<1>(),
                    8 => br.read_bits32_const::<8>(),
                    16 => br.read_bits32_const::<16>(),
                    24 => br.read_bits32_const::<24>(),
                    32 => br.read_bits32_const::<32>(),
                    _ => panic!("unknown source literal"),
                }
            }
            _ => panic!("unknown source operation"),
        }
        print!(
            "{} {value} {} {} {} {} {} {} {}",
            u8::from(ok),
            br.bits_unmasked(),
            br.available_bits(),
            br.position(),
            br.guard_position(),
            br.available_in(),
            br.remaining_bytes(),
            u8::from(br.check_input_amount())
        );
        if op == 14 {
            print!(" ");
            for b in output {
                print!("{b:02x}");
            }
        }
        println!();
    }
}
