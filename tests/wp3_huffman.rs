//! WP3 differential test: Huffman table construction and the table segments
//! of `src/utils/huffman_utils.c`. One script of commands drives the C probe
//! `tools/oracle/probes/wp3/huffman.c` and the Rust translation, which builds
//! the expected output. The code-length sets are seeded and fuzzed: complete
//! codes from random trees, single-symbol codes, incomplete and
//! over-subscribed codes, lengths above 15, all-zero sets and uniformly random
//! lengths, for the alphabet sizes of the decoder and random ones, with and
//! without a root table. Every table entry, the return value and the segment
//! chain are compared after every command.

#[path = "support/oracle.rs"]
mod oracle;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::utils::huffman_utils::{
    HUFFMAN_TABLE_BITS, HuffmanTables, HuffmanTablesSegment, LENGTHS_TABLE_BITS,
    vp8l_build_huffman_table, vp8l_htree_groups_new, vp8l_huffman_tables_allocate,
    vp8l_huffman_tables_deallocate,
};

/// SplitMix64, a small deterministic generator for the script.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// A value in `0..bound`.
    fn below(&mut self, bound: usize) -> usize {
        (self.next_u64() % bound as u64) as usize
    }
}

fn hex(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "-".to_owned()
    } else {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

/// Lengths of a complete prefix code with `leaves` codes of at most 15 bits,
/// from a random tree, spread over `size` symbols.
fn complete_code(rng: &mut Rng, size: usize, leaves: usize) -> Vec<u8> {
    let mut depths: Vec<u8> = vec![1, 1];
    while depths.len() < leaves {
        let splittable: Vec<usize> = (0..depths.len()).filter(|&i| depths[i] < 15).collect();
        if splittable.is_empty() {
            break;
        }
        // Prefer shallow or deep leaves at random, to reach both kinds of trees.
        let pick = if rng.below(2) == 0 {
            splittable[rng.below(splittable.len())]
        } else {
            *splittable
                .iter()
                .max_by_key(|&&i| depths[i])
                .expect("not empty")
        };
        let depth = depths[pick] + 1;
        depths[pick] = depth;
        depths.push(depth);
    }
    let mut lengths = vec![0_u8; size];
    let mut slots: Vec<usize> = (0..size).collect();
    for depth in depths {
        let slot = slots.swap_remove(rng.below(slots.len()));
        lengths[slot] = depth;
    }
    lengths
}

/// A seeded code-length set of `size` symbols, valid or not.
fn code_lengths(rng: &mut Rng, size: usize) -> Vec<u8> {
    let kind = rng.below(10);
    if kind == 0 || size < 2 {
        // One symbol, of any length.
        let mut lengths = vec![0_u8; size];
        lengths[rng.below(size)] = 1 + rng.below(15) as u8;
        return lengths;
    }
    // At least two codes, at most one per symbol and 2^15 in all.
    let leaves = 2 + rng.below(size.min(1 << 15) - 1);
    match kind {
        1 => vec![0; size],
        2 => (0..size).map(|_| rng.below(16) as u8).collect(),
        3 => {
            // A length above MAX_ALLOWED_CODE_LENGTH.
            let mut lengths = complete_code(rng, size, leaves);
            let at = rng.below(size);
            lengths[at] = 16 + rng.below(240) as u8;
            lengths
        }
        4 | 5 => {
            // An incomplete or over-subscribed variant of a complete code.
            let mut lengths = complete_code(rng, size, leaves);
            let used: Vec<usize> = (0..size).filter(|&i| lengths[i] != 0).collect();
            let at = used[rng.below(used.len())];
            match rng.below(3) {
                0 => lengths[at] = 0,
                1 if lengths[at] < 15 => lengths[at] += 1,
                _ if lengths[at] > 1 => lengths[at] -= 1,
                _ => lengths[at] = 0,
            }
            lengths
        }
        _ => complete_code(rng, size, leaves),
    }
}

/// The alphabet sizes of the decoder: code-length codes, distances, literals
/// with every color cache size, and random sizes.
fn alphabet_size(rng: &mut Rng) -> usize {
    match rng.below(6) {
        0 => 19,
        1 => 40,
        2 => 256,
        3 => {
            280 + if rng.below(2) == 0 {
                0
            } else {
                1 << (1 + rng.below(11))
            }
        }
        4 => 2 + rng.below(30),
        _ => 1 + rng.below(600),
    }
}

fn root_bits(rng: &mut Rng, size: usize) -> i32 {
    match rng.below(8) {
        0 => 1 + rng.below(12) as i32,
        1 if size <= 19 => LENGTHS_TABLE_BITS,
        _ => HUFFMAN_TABLE_BITS,
    }
}

/// The C `has_start` test and `curr_table - start` of one segment.
fn segment_line(segment: &HuffmanTablesSegment) -> String {
    let has_start = !segment.start.is_empty();
    let curr = if has_start { segment.curr_table } else { 0 };
    format!("segment={},{curr},{}\n", segment.size, i32::from(has_start))
}

fn state(tables: &HuffmanTables) -> String {
    let count = 1 + tables.next_segments.len();
    let mut out = format!("segments={count}\ncurrent={}\n", tables.curr_segment);
    for index in 0..count {
        out += &segment_line(tables.segment(index));
    }
    out
}

fn lengths_of(bytes: &[u8]) -> Vec<i32> {
    bytes.iter().map(|&length| i32::from(length)).collect()
}

fn script() -> (String, String) {
    let mut rng = Rng(0x5eed_0003);
    let mut commands = String::new();
    let mut expected = String::new();
    let mut tables = HuffmanTables::default();

    // Allocation decisions of the tree groups; 0 is excluded (WP1, item 12).
    for count in [1, 5, 1000, 0x1_0000, -1, -0x7fff_ffff, i32::MAX] {
        writeln!(commands, "groups {count}").unwrap();
        let ok = vp8l_htree_groups_new(count).is_some();
        writeln!(expected, "groups={}", i32::from(ok)).unwrap();
    }

    // Root segments that fit a table exactly, which needs a new segment
    // (`curr_table + total_size >= size`), or with one entry to spare.
    for round in 0..40 {
        let alphabet = alphabet_size(&mut rng);
        let bits = root_bits(&mut rng, alphabet);
        let alphabet = alphabet.max(2);
        let leaves = 2 + rng.below(alphabet.min(1 << 15) - 1);
        let lengths = complete_code(&mut rng, alphabet, leaves);
        let code_lengths = lengths_of(&lengths);
        let total = vp8l_build_huffman_table(None, bits, &code_lengths);
        assert!(total > 0, "a complete code is accepted");
        let size = total + round % 2;
        writeln!(commands, "alloc {size}").unwrap();
        let ok = vp8l_huffman_tables_allocate(size, &mut tables);
        writeln!(expected, "alloc={}", i32::from(ok)).unwrap();
        expected += &state(&tables);
        writeln!(commands, "build {bits} {}", hex(&lengths)).unwrap();
        let ret = vp8l_build_huffman_table(Some(&mut tables), bits, &code_lengths);
        writeln!(expected, "size={ret}").unwrap();
        let segment = tables.segment(tables.curr_segment);
        let at = segment.curr_table;
        expected += "codes=";
        for code in &segment.start[at..at + ret as usize] {
            write!(expected, "{:02x}{:04x}", code.bits, code.value).unwrap();
        }
        expected += "\n";
        expected += &state(&tables);
    }

    for round in 0..120 {
        // A root segment that is sometimes too small, so that builds chain new
        // segments; a negative size fails the allocation.
        let size: i32 = match rng.below(12) {
            0 => -1 - rng.below(1000) as i32,
            1..=4 => 1 + rng.below(300) as i32,
            _ => 1 + rng.below(6000) as i32,
        };
        writeln!(commands, "alloc {size}").unwrap();
        let ok = vp8l_huffman_tables_allocate(size, &mut tables);
        writeln!(expected, "alloc={}", i32::from(ok)).unwrap();
        expected += &state(&tables);
        if !ok {
            continue;
        }
        let builds = 1 + rng.below(12);
        for _ in 0..builds {
            let alphabet = alphabet_size(&mut rng);
            let lengths = code_lengths(&mut rng, alphabet);
            let bits = root_bits(&mut rng, alphabet);
            let code_lengths = lengths_of(&lengths);
            if rng.below(4) == 0 {
                writeln!(commands, "validate {bits} {}", hex(&lengths)).unwrap();
                let ret = vp8l_build_huffman_table(None, bits, &code_lengths);
                writeln!(expected, "size={ret}").unwrap();
                continue;
            }
            writeln!(commands, "build {bits} {}", hex(&lengths)).unwrap();
            let ret = vp8l_build_huffman_table(Some(&mut tables), bits, &code_lengths);
            writeln!(expected, "size={ret}").unwrap();
            if ret > 0 {
                let segment = tables.segment(tables.curr_segment);
                let at = segment.curr_table;
                expected += "codes=";
                for code in &segment.start[at..at + ret as usize] {
                    write!(expected, "{:02x}{:04x}", code.bits, code.value).unwrap();
                }
                expected += "\n";
            }
            expected += &state(&tables);
            if ret > 0 {
                writeln!(commands, "advance {ret}").unwrap();
                tables.segment_mut(tables.curr_segment).curr_table += ret as usize;
                expected += &state(&tables);
            }
        }
        if round % 3 == 0 {
            commands += "dealloc\n";
            vp8l_huffman_tables_deallocate(&mut tables);
            expected += &segment_line(&tables.root);
        }
    }
    (commands, expected)
}

/// The first line where the two outputs differ, for the failure message.
fn first_difference(expected: &str, actual: &str) -> String {
    let mut expected_lines = expected.lines();
    let mut actual_lines = actual.lines();
    for index in 0.. {
        match (expected_lines.next(), actual_lines.next()) {
            (None, None) => return "the outputs are equal".to_owned(),
            (Some(e), Some(a)) if e == a => {}
            (e, a) => {
                let shorten =
                    |line: Option<&str>| line.map(|l| l.chars().take(200).collect::<String>());
                return format!(
                    "line {index}: expected {:?}, the C probe printed {:?}",
                    shorten(e),
                    shorten(a)
                );
            }
        }
    }
    unreachable!()
}

#[test]
fn huffman_tables_match_the_c_construction() {
    let Some(oracle) = Oracle::locate("huffman_tables_match_the_c_construction") else {
        return;
    };
    let (commands, expected) = script();
    let builds = expected
        .lines()
        .filter(|line| line.starts_with("size="))
        .count();
    let rejected = expected.lines().filter(|line| *line == "size=0").count();
    let chained = expected
        .lines()
        .filter(|line| line.starts_with("segments=") && *line != "segments=1")
        .count();
    let long_sets = commands
        .lines()
        .filter(|line| {
            line.split(' ')
                .nth(2)
                .is_some_and(|lengths| lengths.len() > 2 * 512)
        })
        .count();
    eprintln!(
        "{builds} builds, {rejected} rejected, {chained} states with chained segments, {long_sets} sets over 512 symbols, {} lines",
        expected.lines().count()
    );
    assert!(builds > 500, "the script builds {builds} tables");
    assert!(
        chained > 20 && long_sets > 20,
        "{chained} chained states, {long_sets} long sets"
    );
    assert!(
        rejected > 100 && builds - rejected > 300,
        "{rejected} of {builds} are rejected"
    );
    let output = oracle
        .probe("wp3", "huffman")
        .run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    assert!(
        actual == expected,
        "the Huffman tables differ from the C construction: {}",
        first_difference(&expected, &actual)
    );
}
