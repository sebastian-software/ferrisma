//! WP3 differential test: Huffman tree building for the encoder,
//! `VP8LCreateHuffmanTree` and `VP8LCreateCompressedHuffmanTree`
//! (`src/utils/huffman_encode_utils.c`). One script drives the C probe
//! `tools/oracle/probes/wp3/huffman_encode.c` and the Rust translation, which
//! builds the expected output. The histograms are seeded: sparse and dense
//! counts, runs of equal and of zero counts, Fibonacci and geometric counts
//! that exceed the depth limit, a single and two used symbols, and empty
//! histograms, for the alphabet sizes and depth limits of the encoder. The
//! modified histogram, the RLE marks, the code lengths, the codes and the
//! tokens of the compressed tree are compared.

#[path = "support/oracle.rs"]
mod oracle;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::utils::huffman_encode_utils::{
    HuffmanTree, HuffmanTreeCode, HuffmanTreeToken, vp8l_create_compressed_huffman_tree,
    vp8l_create_huffman_tree,
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
    fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }
}

fn hex(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "-".to_owned()
    } else {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

fn hex32(values: &[u32]) -> String {
    if values.is_empty() {
        "-".to_owned()
    } else {
        values.iter().map(|value| format!("{value:08x}")).collect()
    }
}

/// A seeded histogram of `size` symbols with counts below 2^26, so that the
/// `int` arithmetic of the RLE heuristic cannot overflow.
fn histogram(rng: &mut Rng, size: usize, kind: u64) -> Vec<u32> {
    let mut counts = vec![0_u32; size];
    match kind {
        0 => {
            // Sparse.
            for _ in 0..=rng.below(size as u64 / 4 + 1) {
                counts[rng.below(size as u64) as usize] = 1 + rng.below(1000) as u32;
            }
        }
        1 => {
            // Dense.
            for count in &mut counts {
                *count = rng.below(1 << 20) as u32;
            }
        }
        2 => {
            // Runs of equal counts, of near-equal counts and of zeros.
            let mut at = 0;
            while at < size {
                let run = (1 + rng.below(20) as usize).min(size - at);
                let base = match rng.below(3) {
                    0 => 0,
                    1 => 1 + rng.below(50) as u32,
                    _ => rng.below(5000) as u32,
                };
                for count in &mut counts[at..at + run] {
                    *count = if base == 0 {
                        0
                    } else {
                        base + rng.below(4) as u32
                    };
                }
                at += run;
            }
        }
        3 => {
            // Fibonacci counts give the deepest trees.
            let (mut a, mut b) = (1_u32, 1_u32);
            let mut slots: Vec<usize> = (0..size).collect();
            for _ in 0..size.min(36) {
                let slot = slots.swap_remove(rng.below(slots.len() as u64) as usize);
                counts[slot] = a;
                (a, b) = (b, (a + b).min(1 << 25));
            }
        }
        4 => {
            // Geometric counts.
            for (i, count) in counts.iter_mut().enumerate().take(26) {
                *count = 1 << i;
            }
        }
        5 => {
            // One used symbol.
            counts[rng.below(size as u64) as usize] = 1 + rng.below(1 << 20) as u32;
        }
        6 => {
            // Two used symbols.
            let first = rng.below(size as u64) as usize;
            let second = rng.below(size as u64) as usize;
            counts[first] = 1 + rng.below(1 << 10) as u32;
            counts[second] = 1 + rng.below(1 << 10) as u32;
        }
        7 => {}
        8 => {
            // Runs of exactly equal counts, of 3 to 9 symbols, next to close
            // values, and zero runs around the thresholds of 5 and 7.
            let mut at = 0;
            let mut base = 20 + rng.below(1000) as u32;
            while at < size {
                let run = (3 + rng.below(7) as usize).min(size - at);
                let value = if rng.below(5) == 0 { 0 } else { base };
                counts[at..at + run].fill(value);
                at += run;
                base = (base + rng.below(7) as u32).saturating_sub(3).max(1);
            }
        }
        _ => {
            for count in &mut counts {
                if rng.below(3) != 0 {
                    *count = rng.below(64) as u32;
                }
            }
        }
    }
    counts
}

/// Code lengths with runs, for the compressed tree.
fn lengths_with_runs(rng: &mut Rng, size: usize) -> Vec<u8> {
    let mut lengths = Vec::with_capacity(size);
    while lengths.len() < size {
        let bound = if rng.below(4) == 0 { 300 } else { 12 };
        let run = (1 + rng.below(bound) as usize).min(size - lengths.len());
        let value = if rng.below(3) == 0 {
            0
        } else {
            rng.below(16) as u8
        };
        lengths.extend(std::iter::repeat_n(value, run));
    }
    lengths
}

fn tree_case(
    rng: &mut Rng,
    commands: &mut String,
    expected: &mut String,
    limit: i32,
    counts: Vec<u32>,
) -> Vec<u8> {
    let size = counts.len();
    // The encoder starts from zeroed lengths; sometimes keep stale lengths up to
    // the limit, which the tree leaves alone for unused symbols.
    let initial: Vec<u8> = if rng.below(5) == 0 {
        (0..size)
            .map(|_| rng.below(limit as u64 + 1) as u8)
            .collect()
    } else {
        vec![0; size]
    };
    writeln!(
        commands,
        "tree {limit} {} {}",
        hex32(&counts),
        hex(&initial)
    )
    .unwrap();
    let mut histogram = counts;
    let mut buf_rle = vec![0xaa_u8; size];
    let mut huff_tree = vec![HuffmanTree::default(); 3 * size];
    let mut code = HuffmanTreeCode {
        num_symbols: i32::try_from(size).unwrap(),
        code_lengths: initial,
        codes: vec![0; size],
    };
    vp8l_create_huffman_tree(
        &mut histogram,
        limit,
        &mut buf_rle,
        &mut huff_tree,
        &mut code,
    );
    writeln!(expected, "histogram={}", hex32(&histogram).replace('-', "")).unwrap();
    writeln!(expected, "rle={}", hex(&buf_rle).replace('-', "")).unwrap();
    writeln!(
        expected,
        "lengths={}",
        hex(&code.code_lengths).replace('-', "")
    )
    .unwrap();
    let codes: String = code.codes.iter().map(|c| format!("{c:04x}")).collect();
    writeln!(expected, "codes={codes}").unwrap();
    code.code_lengths
}

fn compress_case(commands: &mut String, expected: &mut String, lengths: &[u8]) {
    writeln!(commands, "compress {}", hex(lengths)).unwrap();
    let code = HuffmanTreeCode {
        num_symbols: i32::try_from(lengths.len()).unwrap(),
        code_lengths: lengths.to_vec(),
        codes: Vec::new(),
    };
    let mut tokens = vec![HuffmanTreeToken::default(); lengths.len()];
    let count = vp8l_create_compressed_huffman_tree(&code, &mut tokens);
    writeln!(expected, "tokens={count}").unwrap();
    let text: String = tokens[..count as usize]
        .iter()
        .map(|token| format!("{:02x}{:02x}", token.code, token.extra_bits))
        .collect();
    writeln!(expected, "token={text}").unwrap();
}

fn script() -> (String, String) {
    let mut rng = Rng(0x5eed_0013);
    let mut commands = String::new();
    let mut expected = String::new();
    // An empty histogram.
    tree_case(&mut rng, &mut commands, &mut expected, 15, Vec::new());
    for round in 0..400_u64 {
        let (size, limit) = match round % 6 {
            0 => (19, 7),
            1 => (40, 15),
            2 => (256, 15),
            3 => (280, 15),
            4 => (280 + (1 << (1 + rng.below(11))), 15),
            _ => (1 + rng.below(300) as usize, 15),
        };
        let kind = rng.below(10);
        let counts = histogram(&mut rng, size, kind);
        let lengths = tree_case(&mut rng, &mut commands, &mut expected, limit, counts);
        // The encoder compresses the code it built.
        compress_case(&mut commands, &mut expected, &lengths);
        if round % 4 == 0 {
            let size = 1 + rng.below(2400) as usize;
            let lengths = lengths_with_runs(&mut rng, size);
            compress_case(&mut commands, &mut expected, &lengths);
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
fn encoder_huffman_trees_match_the_c_functions() {
    let Some(oracle) = Oracle::locate("encoder_huffman_trees_match_the_c_functions") else {
        return;
    };
    let (commands, expected) = script();
    let trees = expected
        .lines()
        .filter(|line| line.starts_with("codes="))
        .count();
    let compressed = expected
        .lines()
        .filter(|line| line.starts_with("tokens="))
        .count();
    eprintln!(
        "{trees} trees, {compressed} compressed trees, {} lines",
        expected.lines().count()
    );
    assert!(
        trees > 400 && compressed > 450,
        "{trees} trees, {compressed} compressed trees"
    );
    let output = oracle
        .probe("wp3", "huffman_encode")
        .run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    assert!(
        actual == expected,
        "the encoder Huffman trees differ from the C functions: {}",
        first_difference(&expected, &actual)
    );
}
