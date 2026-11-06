//! WP3 differential test: the color cache of `src/utils/color_cache_utils.c`
//! and `.h`. One script drives the C probe
//! `tools/oracle/probes/wp3/color_cache.c` and the Rust translation, which
//! builds the expected output: initialization for every hash size the format
//! allows, inserts, sets, lookups, index and containment queries, copies
//! between caches of the same size, clears, and the hash for every shift.

#[path = "support/oracle.rs"]
mod oracle;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::utils::color_cache_utils::{
    VP8LColorCache, vp8l_color_cache_clear, vp8l_color_cache_contains, vp8l_color_cache_copy,
    vp8l_color_cache_get_index, vp8l_color_cache_init, vp8l_color_cache_insert,
    vp8l_color_cache_lookup, vp8l_color_cache_set, vp8l_hash_pix,
};
use webp::webp::format_constants::MAX_CACHE_BITS;

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

    fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }

    fn argb(&mut self) -> u32 {
        self.next_u64() as u32
    }
}

fn colors(cache: &VP8LColorCache) -> String {
    let count = 1_usize << cache.hash_bits;
    cache.colors[..count]
        .iter()
        .map(|color| format!("{color:08x}"))
        .collect()
}

fn script() -> (String, String) {
    let mut rng = Rng(0x5eed_0023);
    let mut commands = String::new();
    let mut expected = String::new();
    let mut caches = [VP8LColorCache::default(), VP8LColorCache::default()];

    // The hash for every shift that keeps the result an int, and corner values.
    for shift in 1..32 {
        for argb in [0, 1, 0xffff_ffff, 0x8000_0000, rng.argb(), rng.argb()] {
            writeln!(commands, "hash {argb} {shift}").unwrap();
            writeln!(expected, "hash={}", vp8l_hash_pix(argb, shift)).unwrap();
        }
    }

    for round in 0..60 {
        let bits = 1 + (round % MAX_CACHE_BITS);
        for (c, cache) in caches.iter_mut().enumerate() {
            writeln!(commands, "init {c} {bits}").unwrap();
            vp8l_color_cache_clear(cache);
            let ok = vp8l_color_cache_init(cache, bits);
            writeln!(
                expected,
                "init={}\nshift={}\nbits={}",
                i32::from(ok),
                cache.hash_shift,
                cache.hash_bits
            )
            .unwrap();
        }
        let size = 1_u64 << bits;
        let mut seen: Vec<u32> = Vec::new();
        for _ in 0..200 {
            let c = rng.below(2) as usize;
            match rng.below(6) {
                0 | 1 => {
                    let argb = rng.argb();
                    seen.push(argb);
                    writeln!(commands, "insert {c} {argb}").unwrap();
                    vp8l_color_cache_insert(&mut caches[c], argb);
                }
                2 => {
                    let key = rng.below(size) as u32;
                    let argb = rng.argb();
                    writeln!(commands, "set {c} {key} {argb}").unwrap();
                    vp8l_color_cache_set(&mut caches[c], key, argb);
                }
                3 => {
                    let key = rng.below(size) as u32;
                    writeln!(commands, "lookup {c} {key}").unwrap();
                    writeln!(
                        expected,
                        "lookup={}",
                        vp8l_color_cache_lookup(&caches[c], key)
                    )
                    .unwrap();
                }
                4 => {
                    let argb = if seen.is_empty() || rng.below(2) == 0 {
                        rng.argb()
                    } else {
                        seen[rng.below(seen.len() as u64) as usize]
                    };
                    writeln!(commands, "contains {c} {argb}").unwrap();
                    writeln!(
                        expected,
                        "contains={}",
                        vp8l_color_cache_contains(&caches[c], argb)
                    )
                    .unwrap();
                    writeln!(commands, "index {c} {argb}").unwrap();
                    writeln!(
                        expected,
                        "index={}",
                        vp8l_color_cache_get_index(&caches[c], argb)
                    )
                    .unwrap();
                }
                _ => {
                    let (from, to) = (c, 1 - c);
                    writeln!(commands, "copy {from} {to}").unwrap();
                    let source = caches[from].clone();
                    vp8l_color_cache_copy(&source, &mut caches[to]);
                }
            }
        }
        for (c, cache) in caches.iter().enumerate() {
            writeln!(commands, "dump {c}").unwrap();
            writeln!(expected, "colors={}", colors(cache)).unwrap();
        }
        if round % 5 == 0 {
            writeln!(commands, "clear 1").unwrap();
            vp8l_color_cache_clear(&mut caches[1]);
            writeln!(expected, "null={}", i32::from(caches[1].colors.is_empty())).unwrap();
        }
    }
    (commands, expected)
}

#[test]
fn color_cache_matches_the_c_cache() {
    let Some(oracle) = Oracle::locate("color_cache_matches_the_c_cache") else {
        return;
    };
    let (commands, expected) = script();
    let observations = expected.lines().count();
    assert!(
        observations > 3000,
        "the script has {observations} observations"
    );
    let output = oracle
        .probe("wp3", "color_cache")
        .run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    let differs = expected
        .lines()
        .zip(actual.lines())
        .position(|(e, a)| e != a)
        .map_or_else(String::new, |line| {
            format!("first difference at line {line}")
        });
    assert!(
        actual == expected,
        "the color cache differs from the C cache: {differs}"
    );
}
