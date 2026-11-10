//! WP13 consumer gates: upstream's `cwebp` and `imageio`, unmodified, linked
//! against the Rust static library through the C ABI (the consumer build of
//! `tools/oracle/probes/wp13/oracle.cmake`, as `probes/wp12/cwebp`).
//!
//! - `test_cwebp.sh`, unmodified, encodes every test-data file that the
//!   oracle's `cwebp` reads with `cwebp` and with `cwebp -noasm` and compares the
//!   two, in the three option sets of `tools/oracle_baseline.py`
//!   (`CWEBP_OPTION_SETS`). The test runs it with the oracle's `cwebp` and with
//!   the Rust-linked `cwebp`: both pass every file, and a checksum program that
//!   records what it reads (`--md5exec`) shows that the two write the same bytes
//!   for every file and run.
//! - `cwebp` writes the same file, the same report on standard error (the
//!   statistics of `WebPAuxStats`) and exits with the same status as the
//!   oracle's `cwebp` over an option matrix (quality, method, presets,
//!   lossless levels, near-lossless, alpha options, rate control, filters,
//!   resizing, cropping, blending, sharp YUV, threads, distortion reports), on
//!   the inputs up to 1 MB. With `-alpha_method 0` the reports are compared up
//!   to the lossless-alpha statistics, which C reads uninitialized
//!   (`tests/wp13_encode.rs`, `# Excluded undefined behavior`, item 1).
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Input formats.** The oracle on the development host reads PNG (libpng)
//!    but not TIFF; the tools test selects the inputs that the oracle's `cwebp`
//!    reads, so a host without libpng runs the script on the other files. The
//!    encoder matrix reads `grid.pam` and `peak.pam`, the pictures of
//!    `grid.png` and `peak.png`.

#[path = "support/oracle.rs"]
mod oracle;

use std::path::{Path, PathBuf};
use std::process::Command;

use oracle::Oracle;

/// The Rust-linked `cwebp` of the consumer build.
fn rust_cwebp(oracle: &Oracle) -> PathBuf {
    let path = oracle.directory().join("probes/wp12/cwebp");
    assert!(
        path.is_file(),
        "missing {}; rebuild the oracle with python3 tools/build_oracle.py --rust-lib",
        path.display()
    );
    path
}

/// A fresh scratch directory inside the oracle's variant directory, per process,
/// so that test runs at the same time do not share one.
fn scratch(oracle: &Oracle, name: &str) -> PathBuf {
    let directory = oracle
        .directory()
        .join("wp13-tools")
        .join(format!("{name}-{}", std::process::id()));
    if directory.exists() {
        std::fs::remove_dir_all(&directory).unwrap();
    }
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

/// An `md5sum` that accepts standard input, as `tools/oracle_baseline.py` finds it.
fn md5_tool() -> PathBuf {
    for name in ["md5sum", "gmd5sum"] {
        for dir in std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()) {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return candidate;
            }
        }
    }
    panic!("test_cwebp.sh needs md5sum (GNU coreutils, or macOS 14 or later)");
}

/// The test-data files that `cwebp` reads, sorted, as `cwebp_inputs` of
/// `tools/oracle_baseline.py` selects them.
fn cwebp_inputs(cwebp: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(oracle::test_data())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .filter(|name| !name.ends_with(".sh") && !name.ends_with(".md5") && !name.ends_with(".txt"))
        .collect();
    names.sort();
    names
        .into_iter()
        .filter(|name| {
            Command::new(cwebp)
                .args(["-quiet", "-o", "/dev/null"])
                .arg(oracle::test_data().join(name))
                .output()
                .unwrap()
                .status
                .success()
        })
        .collect()
}

/// The option sets of `CWEBP_OPTION_SETS` in `tools/oracle_baseline.py`.
const OPTION_SETS: [(&str, &[&str]); 3] = [
    ("default", &[]),
    ("lossless", &["--lossless"]),
    (
        "mt_noalpha_q30_m6",
        &[
            "--mt",
            "--noalpha",
            "--extra_args=-q 30 -m 6 -sharp_yuv -af",
        ],
    ),
];

/// Runs `test_cwebp.sh` with `cwebp` in `directory` on `inputs`; returns
/// whether the script succeeded, its log, and the checksums that the
/// recording checksum program saw, in order.
fn run_test_cwebp(
    cwebp: &Path,
    directory: &Path,
    options: &[&str],
    inputs: &[String],
) -> (bool, String, Vec<String>) {
    let record = directory.join("md5-record.sh");
    let log = directory.join("md5.log");
    std::fs::write(
        &record,
        format!(
            "#!/bin/sh\nsum=$({} | cut -d' ' -f1)\necho \"$sum\" >> '{}'\necho \"$sum  -\"\n",
            md5_tool().display(),
            log.display()
        ),
    )
    .unwrap();
    let mut args = vec![
        oracle::test_data()
            .join("test_cwebp.sh")
            .to_str()
            .unwrap()
            .to_owned(),
        format!("--exec={}", cwebp.display()),
        format!("--md5exec=sh {}", record.display()),
    ];
    args.extend(options.iter().map(|option| (*option).to_owned()));
    args.extend(inputs.iter().cloned());
    let output = Command::new("sh")
        .args(&args)
        .current_dir(oracle::test_data())
        .output()
        .expect("sh runs");
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    let sums = std::fs::read_to_string(&log)
        .unwrap_or_default()
        .lines()
        .map(str::to_owned)
        .collect();
    (output.status.success(), text, sums)
}

/// `test_cwebp.sh` with option set `index`, through both `cwebp` builds.
fn test_cwebp_set(test: &str, index: usize) {
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let (name, options) = OPTION_SETS[index];
    let c_cwebp = oracle.tool("cwebp");
    let rust = rust_cwebp(&oracle);
    let inputs = cwebp_inputs(&c_cwebp);
    assert_eq!(
        inputs,
        cwebp_inputs(&rust),
        "{test}: the readable inputs differ"
    );
    let mut results = Vec::new();
    for (label, cwebp) in [("oracle", &c_cwebp), ("rust", &rust)] {
        let directory = scratch(&oracle, &format!("test_cwebp-{name}-{label}"));
        let (ok, log, sums) = run_test_cwebp(cwebp, &directory, options, &inputs);
        std::fs::write(directory.join("test_cwebp.log"), &log).unwrap();
        let files_ok = log.lines().filter(|line| line.ends_with(":\tOK")).count();
        assert!(
            ok,
            "{test}: test_cwebp.sh with the {label} cwebp failed:\n{log}"
        );
        assert!(log.trim_end().ends_with("DONE (1 of 1)"), "{test}: {log}");
        assert_eq!(log.matches("FAILED").count(), 0, "{test}: {log}");
        assert_eq!(
            files_ok,
            inputs.len(),
            "{test}: files checked by the {label} cwebp"
        );
        assert_eq!(sums.len(), 2 * inputs.len(), "{test}: recorded checksums");
        results.push(sums);
    }
    for (index, (c, rust)) in results[0].iter().zip(&results[1]).enumerate() {
        assert_eq!(
            c,
            rust,
            "{test}: {} ({}) differs from the oracle",
            inputs[index / 2],
            if index % 2 == 0 {
                "cwebp"
            } else {
                "cwebp -noasm"
            }
        );
    }
    eprintln!(
        "{test}: {} inputs, {} identical outputs",
        inputs.len(),
        results[0].len()
    );
}

#[test]
fn test_cwebp_script_passes_with_the_rust_library_default() {
    test_cwebp_set("test_cwebp_script_passes_with_the_rust_library_default", 0);
}

#[test]
fn test_cwebp_script_passes_with_the_rust_library_lossless() {
    test_cwebp_set("test_cwebp_script_passes_with_the_rust_library_lossless", 1);
}

#[test]
fn test_cwebp_script_passes_with_the_rust_library_mt_noalpha() {
    test_cwebp_set(
        "test_cwebp_script_passes_with_the_rust_library_mt_noalpha",
        2,
    );
}

/// The output file, standard error and exit status of one `cwebp` run.
fn run_cwebp(cwebp: &Path, args: &[&str], out: &Path) -> (Vec<u8>, String, Option<i32>) {
    let _ = std::fs::remove_file(out);
    let output = Command::new(cwebp)
        .args(args)
        .arg("-o")
        .arg(out)
        .output()
        .expect("cwebp runs");
    let file = std::fs::read(out).unwrap_or_default();
    let stderr = String::from_utf8_lossy(&output.stderr).replace(&out.display().to_string(), "OUT");
    (file, stderr, output.status.code())
}

#[test]
fn cwebp_writes_the_files_and_reports_of_the_oracle() {
    let test = "cwebp_writes_the_files_and_reports_of_the_oracle";
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let c_cwebp = oracle.tool("cwebp");
    let rust = rust_cwebp(&oracle);
    let directory = scratch(&oracle, "cwebp");
    let inputs = cwebp_inputs(&c_cwebp);
    let option_sets: &[&[&str]] = &[
        &[],
        &["-q", "0", "-m", "0"],
        &["-q", "100", "-m", "6"],
        &["-preset", "photo", "-q", "50"],
        &["-preset", "drawing"],
        &["-preset", "icon", "-m", "3"],
        &["-preset", "text", "-q", "90"],
        &["-preset", "picture", "-af"],
        &["-lossless", "-q", "100", "-m", "6"],
        &["-z", "0"],
        &["-z", "9"],
        &["-near_lossless", "40"],
        &["-lossless", "-exact"],
        &["-alpha_q", "50", "-alpha_filter", "best"],
        &["-alpha_q", "0", "-alpha_method", "0"],
        &["-alpha_filter", "none", "-exact"],
        &["-noalpha"],
        &["-size", "2000", "-pass", "6"],
        &["-psnr", "38", "-pass", "3"],
        &["-qrange", "20", "60", "-size", "1500"],
        &["-segments", "1", "-partition_limit", "50", "-sns", "100"],
        &["-f", "100", "-sharpness", "7", "-strong"],
        &["-nostrong", "-f", "0"],
        &["-sharp_yuv", "-pre", "4"],
        &["-pre", "2", "-q", "20"],
        &["-jpeg_like", "-q", "60"],
        &["-low_memory", "-mt"],
        &["-mt", "-lossless"],
        &["-resize", "50", "0"],
        &["-crop", "3", "5", "40", "30"],
        &["-blend_alpha", "0xff00ff"],
        &["-hint", "graph", "-lossless"],
        &["-print_psnr"],
        &["-print_ssim", "-lossless"],
        &["-print_lsim", "-q", "40"],
        &["-short"],
        &["-noasm", "-m", "5"],
        &["-quiet", "-d", "/dev/null"],
    ];
    let mut runs = 0;
    for (index, input) in inputs.iter().enumerate() {
        let path = oracle::test_data().join(input);
        let size = std::fs::metadata(&path).unwrap().len();
        let path = path.to_str().unwrap();
        // The default for the inputs up to 1 MB, and two further option sets in
        // rotation for those up to 200 KB; test_cwebp.sh covers the large ones.
        let others = option_sets.len() - 1;
        let picks: &[usize] = if size > 1_000_000 {
            &[]
        } else if size > 200_000 {
            &[0]
        } else {
            &[0, 1 + (2 * index) % others, 1 + (2 * index + 1) % others]
        };
        for &pick in picks {
            let mut args: Vec<&str> = option_sets[pick].to_vec();
            args.push(path);
            let c = run_cwebp(&c_cwebp, &args, &directory.join("c.webp"));
            let r = run_cwebp(&rust, &args, &directory.join("rust.webp"));
            assert_eq!(c.2, r.2, "{test}: exit status of cwebp {args:?}");
            let (mut c_report, mut r_report) = (c.1, r.1);
            if args.windows(2).any(|pair| pair == ["-alpha_method", "0"]) {
                // Uncompressed alpha: C reports the lossless statistics of an
                // uninitialized FilterTrial (undefined, see `src/enc/alpha_enc.rs`,
                // `# Excluded undefined behavior`), the translation zeros, which
                // cwebp does not print.
                for report in [&mut c_report, &mut r_report] {
                    if let Some(at) = report.find("Lossless-alpha compressed size") {
                        report.truncate(at);
                    }
                }
            }
            assert_eq!(c_report, r_report, "{test}: report of cwebp {args:?}");
            assert!(c.0 == r.0, "{test}: output of cwebp {args:?} differs");
            runs += 1;
        }
    }
    eprintln!("{test}: {runs} runs on {} inputs", inputs.len());
}
