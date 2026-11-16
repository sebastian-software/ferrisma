//! The mechanical guard of ADR-0044 (no assembly in the project).
//!
//! `cargo xtask assembly` fails when
//!
//! - a tracked Rust file outside `codecs/*/upstream/` invokes one of the
//!   inline or global assembly macros of `core::arch` (`asm`, `global_asm`,
//!   `naked_asm`) or carries a naked-function attribute, and the file and its
//!   number of sites are not exactly an entry of `assembly-allowlist.txt`
//!   next to this crate's manifest;
//! - an entry of that list is stale: its file has fewer, more or no sites;
//! - Git tracks an assembly source (`*.S`, `*.s`, `*.asm`, `*.nasm`) outside
//!   `codecs/*/upstream/`;
//! - a tracked `build.rs` outside `codecs/*/upstream/` mentions `cc::`,
//!   NASM or an `.S` source.
//!
//! The upstream snapshots keep their assembly: they build only the reference
//! builds the gate compares against (ADR-0044, D1).
//!
//! The macro names are spelled without the bang throughout this file, so the
//! guard does not find itself.
use std::{collections::BTreeMap, fs, path::Path, process::Command};

use crate::Result;

pub const ALLOWLIST: &str = "tools/xtask/assembly-allowlist.txt";
const MACROS: [&str; 3] = ["asm", "global_asm", "naked_asm"];
const ASSEMBLY_SOURCES: [&str; 4] = ["*.S", "*.s", "*.asm", "*.nasm"];

/// The snapshots of the originals, `codecs/<port>/upstream/...`.
pub fn in_upstream(path: &str) -> bool {
    let mut parts = path.split('/');
    parts.next() == Some("codecs") && parts.next().is_some() && parts.next() == Some("upstream")
}

fn is_ident(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// The number of assembly sites in Rust source text: invocations of the
/// assembly macros (the macro name as a whole identifier, the bang, then an
/// opening delimiter) and naked-function attributes (`naked` or
/// `unsafe(naked)` inside `#[...]`, whitespace ignored).
pub fn assembly_sites(text: &str) -> usize {
    let bytes = text.as_bytes();
    let mut sites = 0;
    for (bang, _) in text.match_indices('!') {
        let mut start = bang;
        while start > 0 && is_ident(bytes[start - 1]) {
            start -= 1;
        }
        let rest = text[bang + 1..].trim_start();
        if MACROS.contains(&&text[start..bang]) && rest.starts_with(['(', '{', '[']) {
            sites += 1;
        }
    }
    for (open, _) in text.match_indices("#[") {
        let Some(close) = text[open..].find(']') else {
            continue;
        };
        let inner: String = text[open + 2..open + close]
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        if inner == "naked" || inner == "unsafe(naked)" {
            sites += 1;
        }
    }
    sites
}

/// Whether a build script compiles or assembles foreign sources: `cc::`,
/// NASM, or an `.S` file name (the extension as a whole word).
pub fn build_script_assembles(text: &str) -> bool {
    if text.contains("cc::") || text.to_ascii_lowercase().contains("nasm") {
        return true;
    }
    text.match_indices(".S").any(|(at, _)| {
        text.as_bytes()
            .get(at + 2)
            .is_none_or(|&next| !is_ident(next))
    })
}

/// `path count description` per line; `#` starts a comment line.
pub fn parse_allowlist(text: &str) -> Result<BTreeMap<String, usize>> {
    let mut entries = BTreeMap::new();
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.splitn(3, char::is_whitespace);
        let (Some(path), Some(count), Some(description)) =
            (fields.next(), fields.next(), fields.next())
        else {
            return Err(format!(
                "{ALLOWLIST}:{}: expected `path count description`",
                number + 1
            )
            .into());
        };
        let count: usize = count
            .parse()
            .map_err(|_| format!("{ALLOWLIST}:{}: `{count}` is not a count", number + 1))?;
        if count == 0 || description.trim().is_empty() {
            return Err(format!(
                "{ALLOWLIST}:{}: needs a positive count and a description",
                number + 1
            )
            .into());
        }
        if entries.insert(path.to_string(), count).is_some() {
            return Err(format!("{ALLOWLIST}:{}: {path} listed twice", number + 1).into());
        }
    }
    Ok(entries)
}

fn tracked(root: &Path, patterns: &[&str]) -> Result<Vec<String>> {
    let output = Command::new("git")
        .current_dir(root)
        .args(["ls-files", "-z", "--"])
        .args(patterns)
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "git ls-files failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(output
        .stdout
        .split(|b| *b == 0)
        .filter(|p| !p.is_empty())
        .map(|p| String::from_utf8_lossy(p).into_owned())
        .filter(|p| !in_upstream(p))
        .collect())
}

/// Every violation of ADR-0044 in the Git checkout at `root`, compared with
/// the allowlist text.
pub fn problems(root: &Path, allowlist: &str) -> Result<Vec<String>> {
    let allowed = parse_allowlist(allowlist)?;
    let mut found = BTreeMap::new();
    for path in tracked(root, &["*.rs"])? {
        let Ok(text) = fs::read_to_string(root.join(&path)) else {
            continue; // deleted in the working tree, or not UTF-8
        };
        let sites = assembly_sites(&text);
        if sites > 0 {
            found.insert(path, sites);
        }
    }
    let mut problems = Vec::new();
    for (path, &sites) in &found {
        match allowed.get(path) {
            None => problems.push(format!(
                "{path}: {sites} assembly site(s), not on {ALLOWLIST} (ADR-0044)"
            )),
            Some(&listed) if listed != sites => problems.push(format!(
                "{path}: {sites} assembly site(s), {ALLOWLIST} lists {listed}"
            )),
            Some(_) => {}
        }
    }
    for (path, listed) in &allowed {
        if !found.contains_key(path) {
            problems.push(format!(
                "{ALLOWLIST}: stale entry {path} ({listed} listed, none found)"
            ));
        }
    }
    for path in tracked(root, &ASSEMBLY_SOURCES)? {
        problems.push(format!(
            "{path}: assembly source outside codecs/*/upstream/ (ADR-0044)"
        ));
    }
    for path in tracked(root, &["build.rs", "*/build.rs"])? {
        let text = fs::read_to_string(root.join(&path)).unwrap_or_default();
        if build_script_assembles(&text) {
            problems.push(format!(
                "{path}: build script mentions cc::, NASM or an .S source (ADR-0044)"
            ));
        }
    }
    Ok(problems)
}

pub fn check(root: &Path) -> Result<()> {
    let allowlist = fs::read_to_string(root.join(ALLOWLIST))?;
    let problems = problems(root, &allowlist)?;
    if !problems.is_empty() {
        for problem in &problems {
            eprintln!("{problem}");
        }
        return Err(format!("{} ADR-0044 violation(s)", problems.len()).into());
    }
    let sites: usize = parse_allowlist(&allowlist)?.values().sum();
    println!("ADR-0044: no assembly outside the {sites} listed site(s) of {ALLOWLIST}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // The macro names are put together at run time (see the module comment).
    fn invocation(name: &str, rest: &str) -> String {
        format!("{name}!{rest}")
    }

    #[test]
    fn every_form_of_a_site_is_found() {
        let samples = [
            format!(
                "unsafe {{ core::arch::{} }}",
                invocation("asm", "(\"nop\")")
            ),
            format!(
                "unsafe {{ std::arch::{} }}",
                invocation("asm", " {\"nop\"}")
            ),
            format!(
                "use core::arch::asm;\nunsafe {{ {} }}",
                invocation("asm", "(\n\"nop\")")
            ),
            invocation("global_asm", "(\".globl f\")"),
            format!("fn f() {{ {} }}", invocation("naked_asm", "(\"ret\")")),
            format!("#[{}]\nfn f() {{}}", "naked"),
            format!("#[ unsafe( {} ) ]\nfn f() {{}}", "naked"),
        ];
        for sample in &samples {
            assert_eq!(assembly_sites(sample), 1, "{sample}");
        }
        assert_eq!(assembly_sites(&samples.concat()), samples.len());
    }

    #[test]
    fn mentions_and_other_macros_are_not_sites() {
        for sample in [
            "use core::arch::asm;".to_string(),
            format!("// no `{}` here", invocation("asm", "")),
            invocation("my_asm", "(\"x\")"),
            invocation("asmx", "(\"x\")"),
            "core::arch::x86_64::_xgetbv(0)".to_string(),
            "#[naked_functions]".to_string(),
        ] {
            assert_eq!(assembly_sites(&sample), 0, "{sample}");
        }
    }

    #[test]
    fn build_scripts_that_assemble_are_found() {
        assert!(build_script_assembles("cc::Build::new().file(\"x.c\");"));
        assert!(build_script_assembles("Command::new(\"NASM\")"));
        assert!(build_script_assembles("let source = \"kernel.S\";"));
        assert!(!build_script_assembles(
            "println!(\"cargo:rustc-cfg=x\"); let s = x.Status;"
        ));
    }

    #[test]
    fn allowlist_needs_counts_and_descriptions() {
        assert_eq!(
            parse_allowlist("# c\n\na/b.rs 2 what it is\n").unwrap(),
            BTreeMap::from([("a/b.rs".to_string(), 2)])
        );
        assert!(parse_allowlist("a/b.rs 2\n").is_err());
        assert!(parse_allowlist("a/b.rs two what\n").is_err());
        assert!(parse_allowlist("a/b.rs 0 what\n").is_err());
        assert!(parse_allowlist("a.rs 1 x\na.rs 1 y\n").is_err());
    }

    #[test]
    fn upstream_snapshots_are_exempt() {
        assert!(in_upstream("codecs/libavif-rs/upstream/aom/x.asm"));
        assert!(!in_upstream("codecs/libavif-rs/src/upstream/x.rs"));
        assert!(!in_upstream("upstream/x.S"));
    }

    #[test]
    fn a_git_fixture_is_checked_against_its_allowlist() {
        let dir =
            std::env::temp_dir().join(format!("ferrisma-xtask-assembly-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let git = |args: &[&str]| {
            let out = Command::new("git")
                .current_dir(&dir)
                .args(args)
                .output()
                .unwrap();
            assert!(
                out.status.success(),
                "{}",
                String::from_utf8_lossy(&out.stderr)
            );
        };
        let write = |path: &str, text: &str| {
            let path = dir.join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, text).unwrap();
        };
        fs::create_dir_all(&dir).unwrap();
        git(&["init", "-q"]);
        let site = format!(
            "unsafe {{ core::arch::{} }}\n",
            invocation("asm", "(\"nop\")")
        );
        write("src/listed.rs", &site.repeat(2));
        write("codecs/port/upstream/vendor.rs", &site);
        write("codecs/port/upstream/kernel.asm", "ret\n");
        write("build.rs", "fn main() {}\n");
        git(&["add", "."]);
        let listed = "src/listed.rs 2 two test sites; open owner question, ADR-0044\n";
        assert_eq!(problems(&dir, listed).unwrap(), Vec::<String>::new());

        // A site that is not listed, a count that changed, a stale entry.
        write("src/new.rs", &site);
        git(&["add", "."]);
        let found = problems(&dir, listed).unwrap();
        assert!(
            found.len() == 1 && found[0].starts_with("src/new.rs"),
            "{found:?}"
        );
        let found = problems(&dir, "src/listed.rs 1 x\nsrc/new.rs 1 x\n").unwrap();
        assert!(
            found.len() == 1 && found[0].contains("lists 1"),
            "{found:?}"
        );
        let found = problems(&dir, "src/listed.rs 2 x\nsrc/new.rs 1 x\nsrc/gone.rs 1 x\n").unwrap();
        assert!(
            found.len() == 1 && found[0].contains("stale entry src/gone.rs"),
            "{found:?}"
        );

        // Assembly sources and build scripts outside the upstream snapshots.
        let allowed = "src/listed.rs 2 x\nsrc/new.rs 1 x\n";
        write("src/kernel.S", "ret\n");
        write("crates/c/build.rs", "fn main() { cc::Build::new(); }\n");
        git(&["add", "."]);
        let found = problems(&dir, allowed).unwrap();
        assert_eq!(found.len(), 2, "{found:?}");
        assert!(found[0].starts_with("src/kernel.S") && found[1].starts_with("crates/c/build.rs"));
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn the_repository_matches_its_allowlist() {
        check(&crate::root()).unwrap();
    }
}
