//! WP7 differential test: `ReadHuffmanCodesHelper` of `src/dec/vp8l_dec.c`,
//! which reads the Huffman codes of the tree groups, as
//! `tests/fuzzer/huffman_fuzzer.cc` calls it, against the C probe
//! `tools/oracle/probes/wp7/huffman_codes.c`.
//!
//! The streams hold simple codes (one or two symbols, inside and outside the
//! alphabet), complete codes of 8-bit lengths sent through a code-length code,
//! corrupted bits and truncations, for every color cache size, with and without
//! a mapping of unused groups. The test compares the result, the decoder status
//! and bit reader, every tree group (trees, trivial literals and codes, packed
//! tables) and the table segments.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp7_support/mod.rs"]
mod wp7_support;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::dec::vp8l_dec::{read_huffman_codes_helper, vp8l_new};
use webp::utils::bit_reader_utils::vp8l_init_bit_reader;
use webp::utils::huffman_utils::HuffmanTables;
use wp7_support::{Rng, compare, hex, print_decoder, print_groups, print_tables};

/// Writes bits least significant first, as the VP8L reader reads them.
#[derive(Default)]
struct Bits {
    bytes: Vec<u8>,
    count: usize,
}

impl Bits {
    fn put(&mut self, value: u32, n_bits: u32) {
        for i in 0..n_bits {
            if self.count.is_multiple_of(8) {
                self.bytes.push(0);
            }
            let bit = ((value >> i) & 1) as u8;
            *self.bytes.last_mut().unwrap() |= bit << (self.count % 8);
            self.count += 1;
        }
    }
}

/// The order of the code length code lengths (C: `kCodeLengthCodeOrder`).
const CODE_LENGTH_CODE_ORDER: [usize; 19] = [
    17, 18, 0, 1, 2, 3, 4, 5, 16, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
];

/// How a case writes its codes.
#[derive(Clone, Copy)]
enum Style {
    /// Simple codes of one symbol: trivial literals and codes.
    Single,
    /// Simple codes of one or two symbols: packed tables.
    Simple,
    /// Simple codes and complete 8-bit codes.
    Mixed,
    /// Any code, also with symbols outside the alphabet and random bits.
    Any,
}

/// Writes a simple code of one or two symbols below `limit`.
fn put_simple(bits: &mut Bits, rng: &mut Rng, limit: u32, two: bool) {
    bits.put(1, 1);
    bits.put(u32::from(two), 1);
    let wide = rng.below(4) != 0;
    bits.put(u32::from(wide), 1);
    let first = rng.below(u64::from(if wide { limit } else { 2 })) as u32;
    bits.put(first, if wide { 8 } else { 1 });
    if two {
        bits.put(rng.below(u64::from(limit)) as u32, 8);
    }
}

/// Writes a complete code of 256 symbols of length 8 through a code-length
/// code of the two symbols 0 and 8, each of length 1.
fn put_complete(bits: &mut Bits, rng: &mut Rng, alphabet_size: u32) {
    bits.put(0, 1);
    bits.put(12 - 4, 4);
    for &symbol in &CODE_LENGTH_CODE_ORDER[..12] {
        bits.put(u32::from(symbol == 0 || symbol == 8), 3);
    }
    let use_length = rng.below(4) == 0;
    bits.put(u32::from(use_length), 1);
    let mut chosen = vec![false; alphabet_size as usize];
    let mut count = 0;
    while count < 256 {
        let symbol = rng.below(u64::from(alphabet_size)) as usize;
        if !chosen[symbol] {
            chosen[symbol] = true;
            count += 1;
        }
    }
    let last = chosen.iter().rposition(|&c| c).unwrap() + 1;
    if use_length {
        // max_symbol = 2 + bits, with length_nbits = 2 + 2 * 6 = 14.
        bits.put(6, 3);
        bits.put((last - 2) as u32, 14);
    }
    let end = if use_length {
        last
    } else {
        alphabet_size as usize
    };
    for &c in &chosen[..end] {
        bits.put(u32::from(c), 1);
    }
}

/// Writes one Huffman code for an alphabet of `alphabet_size` symbols.
fn put_code(bits: &mut Bits, rng: &mut Rng, alphabet_size: u32, style: Style) {
    let limit = alphabet_size.min(256);
    match style {
        Style::Single => put_simple(bits, rng, limit, false),
        Style::Simple => {
            let two = rng.below(2) == 0;
            put_simple(bits, rng, limit, two);
        }
        Style::Mixed => {
            if alphabet_size >= 256 && rng.below(2) == 0 {
                put_complete(bits, rng, alphabet_size);
            } else {
                let two = rng.below(2) == 0;
                put_simple(bits, rng, limit, two);
            }
        }
        Style::Any => match rng.below(10) {
            0..=3 => {
                let two = rng.below(2) == 0;
                put_simple(bits, rng, 256, two);
            }
            4..=6 if alphabet_size >= 256 => put_complete(bits, rng, alphabet_size),
            _ => {
                for _ in 0..rng.range(1, 64) {
                    bits.put(rng.below(2) as u32, 1);
                }
            }
        },
    }
}

struct Helper {
    color_cache_bits: i32,
    num: i32,
    num_max: i32,
    mapping: Option<Vec<i32>>,
    data: Vec<u8>,
}

fn run(case: &Helper, commands: &mut String, expected: &mut String) {
    let mapping = case.mapping.as_ref().map_or("-".to_owned(), |mapping| {
        mapping
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(",")
    });
    let data = if case.data.is_empty() {
        "-".to_owned()
    } else {
        hex(&case.data)
    };
    writeln!(
        commands,
        "helper {} {} {} {mapping} {data}",
        case.color_cache_bits, case.num, case.num_max
    )
    .unwrap();

    let mut dec = vp8l_new();
    let mut tables = HuffmanTables::default();
    let mut groups = None;
    vp8l_init_bit_reader(&mut dec.br, &case.data);
    let ok = read_huffman_codes_helper(
        case.color_cache_bits,
        case.num,
        case.num_max,
        case.mapping.as_deref(),
        &mut dec,
        &case.data,
        &mut tables,
        &mut groups,
    );
    writeln!(expected, "ok={}", i32::from(ok)).unwrap();
    print_decoder(expected, &dec);
    print_groups(expected, groups.as_deref(), case.num);
    print_tables(expected, &tables);
}

#[test]
fn huffman_codes_match_the_c_reader() {
    let test = "huffman_codes_match_the_c_reader";
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let mut rng = Rng(0x5eed_0708);
    let mut commands = String::new();
    let mut expected = String::new();
    let mut count = 0;
    for index in 0..1500 {
        let color_cache_bits = rng.range(0, 11) as i32;
        let num = rng.range(1, 6) as i32;
        let (num_max, mapping) = match rng.below(6) {
            0..=3 => (num, None),
            4 => {
                // A mapping where every group is used once and the others are -1.
                let num_max = num + rng.range(0, 4) as i32;
                let mut mapping = vec![-1; num_max as usize];
                let mut positions: Vec<usize> = (0..num_max as usize).collect();
                for i in (1..positions.len()).rev() {
                    positions.swap(i, rng.below(i as u64 + 1) as usize);
                }
                for (group, &position) in positions[..num as usize].iter().enumerate() {
                    mapping[position] = group as i32;
                }
                (num_max, Some(mapping))
            }
            _ => {
                // The arguments that fail before reading anything.
                if rng.below(2) == 0 {
                    (num + 1, None)
                } else {
                    (num - 1, Some(vec![0; (num - 1).max(0) as usize]))
                }
            }
        };
        let style = [Style::Single, Style::Simple, Style::Mixed, Style::Any][index % 4];
        let mut bits = Bits::default();
        let green = 280
            + if color_cache_bits > 0 {
                1 << color_cache_bits
            } else {
                0
            };
        for _ in 0..num_max {
            for alphabet_size in [green, 256, 256, 256, 40] {
                put_code(&mut bits, &mut rng, alphabet_size, style);
            }
        }
        let mut data = bits.bytes;
        match rng.below(10) {
            0 => {
                let position = rng.below(data.len() as u64 + 1) as usize;
                data.truncate(position);
            }
            1 if !data.is_empty() => {
                let position = rng.below(data.len() as u64) as usize;
                data[position] ^= 1 << rng.below(8);
            }
            _ => {}
        }
        let case = Helper {
            color_cache_bits,
            num,
            num_max,
            mapping,
            data,
        };
        if case.num_max < 1 {
            continue;
        }
        run(&case, &mut commands, &mut expected);
        count += 1;
    }
    let output = oracle
        .probe("wp7", "huffman_codes")
        .run::<&str>(&[], commands.as_bytes());
    output.success();
    compare(test, &output.stdout, &expected);
    let successes = expected.lines().filter(|line| *line == "ok=1").count();
    let packed = expected.lines().filter(|line| *line == "packed=1").count();
    let trivial = expected
        .lines()
        .filter(|line| *line == "trivial_code=1")
        .count();
    eprintln!(
        "{test}: {count} calls compared, {successes} succeeded; {packed} packed tables, {trivial} trivial codes"
    );
}
