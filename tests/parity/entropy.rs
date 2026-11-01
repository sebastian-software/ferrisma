// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Boundary adapters and deterministic corpora for original upstream helpers.

use super::*;
use jxl::jxl::ans_common::{self, AliasTable, Entry};
use jxl::jxl::base::{bits as word, byte_order as order};
use jxl::jxl::field_encodings::{FieldEnum, enum_valid, values};
use jxl::jxl::fields::{Visitor, visit_enum};
use jxl::jxl::types::JxlEndianness;

#[derive(Clone, Copy)]
struct OracleEnum(u32);

impl FieldEnum for OracleEnum {
    const BITS: u64 = (1 << 0) | (1 << 1) | (1 << 7) | (1 << 63);
    const NAME: &'static str = "OracleEnum";
    fn from_raw(value: u32) -> Self {
        Self(value)
    }
    fn raw(self) -> u32 {
        self.0
    }
}

struct EnumBundle {
    value: OracleEnum,
}
impl Fields for EnumBundle {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visit_enum(visitor, OracleEnum(0), &mut self.value)
    }
}

fn word_observation<T: word::UnsignedWord>(value: T) -> String {
    let mut result = format!(
        "{} {}",
        word::num0_bits_above_ms1_bit(value),
        word::num0_bits_below_ls1_bit(value)
    );
    if value.as_u64() != 0 {
        write!(
            result,
            " {} {} {} {}",
            word::floor_log2_nonzero(value),
            word::ceil_log2_nonzero(value),
            word::num0_bits_above_ms1_bit_nonzero(value),
            word::num0_bits_below_ls1_bit_nonzero(value)
        )
        .unwrap();
    }
    result
}

fn byte_order_observation(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let value: u64 = number(args);
    let input = unhex(args.next().unwrap());
    let p = &input[1..];
    let f = order::load_le_float(p);
    let mut result = format!(
        "{} {} {} {} {} {} {} {} {} {}",
        order::load16(order::OrderBE, p),
        order::load16(order::OrderLE, p),
        order::load32(order::OrderBE, p),
        order::load32(order::OrderLE, p),
        order::load_be64(p),
        order::load_le64(p),
        order::load_be_float(p).to_bits(),
        f.to_bits(),
        order::bswap_float(f).to_bits(),
        u32::from(order::is_little_endian())
    );
    for endianness in [
        JxlEndianness::NATIVE,
        JxlEndianness::LITTLE,
        JxlEndianness::BIG,
    ] {
        write!(result, " {}", u32::from(order::swap_endianness(endianness))).unwrap();
    }
    for op in 0..6 {
        let mut bytes = [0xA5; 10];
        match op {
            0 => order::store16(order::OrderBE, value as u32, &mut bytes[1..]),
            1 => order::store16(order::OrderLE, value as u32, &mut bytes[1..]),
            2 => order::store32(order::OrderBE, value as u32, &mut bytes[1..]),
            3 => order::store32(order::OrderLE, value as u32, &mut bytes[1..]),
            4 => order::store_be64(value, &mut bytes[1..]),
            _ => order::store_le64(value, &mut bytes[1..]),
        }
        write!(result, " {}", hex(&bytes)).unwrap();
    }
    result
}

fn alias_observation(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let log_alpha: usize = number(args);
    let length: usize = number(args);
    let distribution = (0..length).map(|_| number(args)).collect();
    let mut table = vec![Entry::default(); 1 << log_alpha];
    let status = ans_common::init_alias_table(distribution, 12, log_alpha, &mut table);
    let mut result = format!("{} {}", status_code(&status), table.len());
    for &entry in &table {
        let Entry {
            cutoff,
            right_value,
            freq0,
            offsets1,
            freq1_xor_freq0,
        } = entry;
        write!(
            result,
            " {cutoff},{right_value},{freq0},{offsets1},{freq1_xor_freq0}"
        )
        .unwrap();
    }
    if status.is_ok() {
        for i in 0..4096 {
            let symbol = AliasTable::lookup(&table, i, 12 - log_alpha, (4096 >> log_alpha) - 1);
            write!(
                result,
                " {},{},{}",
                symbol.value, symbol.offset, symbol.freq
            )
            .unwrap();
        }
    }
    result
}

pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    match kind {
        "word" => {
            let width: u32 = number(args);
            let value: u64 = number(args);
            if width == 32 {
                word_observation(value as u32)
            } else {
                word_observation(value)
            }
        }
        "byteorder" => byte_order_observation(args),
        "enum-info" => {
            let raw = number(args);
            let mut result = status_code(&enum_valid(OracleEnum(raw))).to_string();
            for value in values::<OracleEnum>() {
                write!(result, " {}", value.raw()).unwrap();
            }
            result
        }
        "enum-read" => {
            let bytes = unhex(args.next().unwrap());
            let offset = number(args);
            let mut fields = EnumBundle {
                value: OracleEnum(number(args)),
            };
            let mut reader = BitReader::new(&bytes);
            reader.skip_bits(offset);
            let status = bundle::read(&mut reader, &mut fields);
            format!(
                "{} {} {} {}",
                status_code(&status),
                fields.value.raw(),
                reader.total_bits_consumed(),
                status_code(&reader.close())
            )
        }
        "alias" => alias_observation(args),
        "flat" => {
            let (length, total) = (number(args), number(args));
            let mut result = String::new();
            for count in ans_common::create_flat_histogram(length, total) {
                write!(result, "{count},").unwrap();
            }
            result
        }
        "population" => {
            ans_common::get_population_count_precision(number(args), number(args)).to_string()
        }
        _ => panic!("unknown entropy command: {kind}"),
    }
}

#[test]
fn bit_words_and_unaligned_byte_order_match_original_helpers() {
    let mut cases = Vec::new();
    let mut seed = 9203;
    let mut values = vec![0, 1, u32::MAX as u64, u64::MAX];
    for shift in 0..64 {
        let value = 1u64 << shift;
        values.extend([value - 1, value, value.saturating_add(1)]);
    }
    for _ in 0..2048 {
        values.push(next(&mut seed));
    }
    for value in values {
        for width in [32, 64] {
            cases.push(format!("word {width} {value}"));
        }
        let bytes: Vec<_> = (0..16).map(|_| next(&mut seed) as u8).collect();
        cases.push(format!("byteorder {value} {}", hex(&bytes)));
    }
    compare(cases);
}

#[test]
fn raw_enum_values_and_failed_reads_preserve_source_state() {
    let mut cases = Vec::new();
    for raw in (0..=128).chain([u32::MAX]) {
        cases.push(format!("enum-info {raw}"));
    }
    for byte in 0..=255 {
        for offset in 0..8 {
            for len in 0..=2 {
                let bytes = [byte, 0xA5];
                cases.push(format!("enum-read {} {offset} 69", hex(&bytes[..len])));
            }
        }
    }
    compare(cases);
}

#[test]
fn ans_alias_entries_and_every_lookup_match_the_reference() {
    let mut cases = Vec::new();
    let mut seed = 71237;
    for log_alpha in 0..=8 {
        let max_symbols = 1usize << log_alpha;
        for length in [0, 1, max_symbols / 2, max_symbols] {
            for trial in 0..8 {
                let mut distribution = vec![0i32; length];
                if length > 0 {
                    for _ in 0..4096 {
                        let idx = if trial == 0 {
                            0
                        } else {
                            next(&mut seed) as usize % length
                        };
                        distribution[idx] += 1;
                    }
                }
                let mut line = format!("alias {log_alpha} {length}");
                for count in distribution {
                    write!(line, " {count}").unwrap();
                }
                cases.push(line);
            }
        }
    }
    cases.extend(
        [
            "alias 8 2 4095 0",
            "alias 8 2 4096 4096",
            "alias 0 2 2048 2048",
            "alias 13 0",
        ]
        .map(str::to_owned),
    );
    for length in 1..=256 {
        cases.push(format!("flat {length} 4096"));
    }
    for logcount in 0..=12 {
        for shift in 0..=12 {
            cases.push(format!("population {logcount} {shift}"));
        }
    }
    compare(cases);
}
