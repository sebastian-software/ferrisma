//! Decision D6 (Ferrisma ADR-0020 amendment of 2026-11-13): v1.0 executes
//! no SIMD above AVX2 on x86-64 and above baseline Neon on AArch64. The
//! translated RTCD tables are the only place the port selects a kernel at
//! run time, and `rtcd/x86_64.pl` and `rtcd/arm64.pl` (generated with
//! `--disable-avx512` and `--disable-neon_dotprod/neon_i8mm/sve/sve2`) keep
//! every specialization above that scope out of them. This scan fails if a
//! regeneration ever dispatches on a higher capability or names a higher
//! kernel, on either target of the one translation (codec ADR 0063).

use std::fs;
use std::path::{Path, PathBuf};

const ABOVE_SCOPE_FLAGS: [&str; 5] = [
    "HAS_AVX512",
    "HAS_NEON_DOTPROD",
    "HAS_NEON_I8MM",
    "HAS_SVE",
    "HAS_SVE2",
];
const ABOVE_SCOPE_SUFFIXES: [&str; 5] = ["_avx512", "_neon_dotprod", "_neon_i8mm", "_sve", "_sve2"];

fn rtcd_sources(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rtcd_sources(&path, found);
        } else if path.to_string_lossy().ends_with("_rtcd.rs") {
            found.push(path);
        }
    }
}

/// Whether `text` contains `word` followed by a character that cannot
/// continue an identifier (so `HAS_SVE` does not match `HAS_SVE2`).
fn contains_word(text: &str, word: &str) -> bool {
    text.match_indices(word).any(|(at, _)| {
        !text[at + word.len()..]
            .chars()
            .next()
            .is_some_and(|next| next.is_ascii_alphanumeric() || next == '_')
    })
}

#[test]
fn rtcd_tables_dispatch_nothing_above_avx2_or_neon() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut sources = Vec::new();
    rtcd_sources(&root.join("libaom"), &mut sources);
    sources.sort();
    // aom_dsp, aom_scale and av1; since codec ADR 0063 each unit holds the
    // tables of both targets (and every build profile) under cfg(target_arch).
    assert_eq!(sources.len(), 3, "{sources:?}");
    for path in &sources {
        let text = fs::read_to_string(path).unwrap();
        for arch in ["aarch64", "x86_64"] {
            let attribute = format!("#[cfg(target_arch = \"{arch}\")]");
            assert!(text.contains(&attribute), "{}: no {arch} table", path.display());
        }
    }
    for path in &sources {
        let text = fs::read_to_string(path).unwrap();
        for (number, line) in text.lines().enumerate() {
            let location = format!("{}:{}", path.display(), number + 1);
            if line.contains("flags &") {
                for flag in ABOVE_SCOPE_FLAGS {
                    assert!(!contains_word(line, flag), "{location}: dispatch on {flag}");
                }
            }
            for suffix in ABOVE_SCOPE_SUFFIXES {
                assert!(!contains_word(line, suffix), "{location}: kernel {suffix}");
            }
        }
    }
}
