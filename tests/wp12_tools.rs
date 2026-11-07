//! WP12 consumer gates: upstream's `dwebp`, `webpinfo` and `imageio`, unmodified,
//! linked against the Rust static library through the C ABI (the consumer
//! build of `tools/oracle/probes/wp12/oracle.cmake`, as `probes/wp12/dwebp` and
//! `probes/wp12/webpinfo`).
//!
//! - `test_dwebp.sh` with the Rust-linked `dwebp` verifies all 480 checksums of
//!   `libwebp_tests.md5`, each with and without `-noasm` (960 verifications), and
//!   `test_lossless.sh` its 33 vectors in five formats with and without `-noasm`
//!   (330 comparisons), both unmodified, in a copy of the test data, as
//!   `tools/oracle_baseline.py` runs them against the oracle's own `dwebp`;
//! - `webpinfo` prints the same output and exits with the same status as the
//!   oracle's `webpinfo` on every vector, with each option, and on truncated
//!   files;
//! - `dwebp` writes the same files as the oracle's `dwebp` over an option
//!   matrix (pixel formats, cropping, resizing, flipping, threads, dithering,
//!   incremental decoding, caller memory).

#[path = "support/oracle.rs"]
mod oracle;

use std::path::{Path, PathBuf};
use std::process::Command;

use oracle::Oracle;

/// A tool of the consumer build.
fn consumer_tool(oracle: &Oracle, name: &str) -> PathBuf {
    let path = oracle.directory().join("probes/wp12").join(name);
    assert!(
        path.is_file(),
        "missing {}; rebuild the oracle with python3 tools/build_oracle.py --rust-lib",
        path.display()
    );
    path
}

/// A fresh scratch directory inside the oracle's variant directory.
fn scratch(oracle: &Oracle, name: &str) -> PathBuf {
    let directory = oracle.directory().join("wp12-tools").join(name);
    if directory.exists() {
        std::fs::remove_dir_all(&directory).unwrap();
    }
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

/// A copy of the test data, which the scripts write into.
fn test_data_copy(oracle: &Oracle, name: &str) -> PathBuf {
    let directory = scratch(oracle, name);
    for entry in std::fs::read_dir(oracle::test_data()).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            std::fs::copy(entry.path(), directory.join(entry.file_name())).unwrap();
        }
    }
    directory
}

/// An `md5sum` that accepts `-c -`, as `tools/oracle_baseline.py` finds it.
fn md5_tool() -> PathBuf {
    for name in ["md5sum", "gmd5sum"] {
        for dir in std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()) {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return candidate;
            }
        }
    }
    panic!("test_dwebp.sh needs md5sum (GNU coreutils, or macOS 14 or later)");
}

fn run_script(directory: &Path, args: &[String]) -> (bool, String) {
    let output = Command::new("sh")
        .args(args)
        .current_dir(directory)
        .output()
        .expect("sh runs");
    let mut log = String::from_utf8_lossy(&output.stdout).into_owned();
    log.push_str(&String::from_utf8_lossy(&output.stderr));
    (output.status.success(), log)
}

#[test]
fn test_dwebp_script_passes_with_the_rust_library() {
    let Some(oracle) = Oracle::locate("test_dwebp_script_passes_with_the_rust_library") else {
        return;
    };
    let dwebp = consumer_tool(&oracle, "dwebp");
    let directory = test_data_copy(&oracle, "test_dwebp");
    let (ok, log) = run_script(
        &directory,
        &[
            "test_dwebp.sh".to_owned(),
            format!("--exec={}", dwebp.display()),
            format!("--md5exec={}", md5_tool().display()),
            "libwebp_tests.md5".to_owned(),
        ],
    );
    std::fs::write(directory.join("test_dwebp.log"), &log).unwrap();
    let verified: Vec<&str> = log
        .lines()
        .filter_map(|line| line.strip_suffix(": OK"))
        .collect();
    let distinct: std::collections::BTreeSet<&str> = verified.iter().copied().collect();
    assert!(ok, "test_dwebp.sh failed:\n{log}");
    assert!(log.trim_end().ends_with("DONE"), "{log}");
    assert_eq!(log.matches("FAILED").count(), 0, "{log}");
    assert_eq!(verified.len(), 960, "verifications");
    assert_eq!(distinct.len(), 480, "distinct checksums");
}

#[test]
fn test_lossless_script_passes_with_the_rust_library() {
    let Some(oracle) = Oracle::locate("test_lossless_script_passes_with_the_rust_library") else {
        return;
    };
    let dwebp = consumer_tool(&oracle, "dwebp");
    let directory = test_data_copy(&oracle, "test_lossless");
    let (ok, log) = run_script(
        &directory,
        &[
            "test_lossless.sh".to_owned(),
            format!("--exec={}", dwebp.display()),
        ],
    );
    std::fs::write(directory.join("test_lossless.log"), &log).unwrap();
    let compared: Vec<&str> = log
        .lines()
        .filter_map(|line| line.strip_suffix(": OK"))
        .collect();
    let vectors: std::collections::BTreeSet<&str> = compared
        .iter()
        .map(|name| name.rsplit_once('.').map_or(*name, |(stem, _)| stem))
        .collect();
    assert!(ok, "test_lossless.sh failed:\n{log}");
    assert!(log.trim_end().ends_with("ALL TESTS OK"), "{log}");
    assert_eq!(compared.len(), 330, "comparisons");
    assert_eq!(vectors.len(), 33, "vectors");
}

/// The standard output, standard error and exit status of a tool run.
fn tool_output(tool: &Path, args: &[&str]) -> (Vec<u8>, Vec<u8>, Option<i32>) {
    let output = Command::new(tool)
        .args(args)
        .output()
        .expect("the tool runs");
    (output.stdout, output.stderr, output.status.code())
}

#[test]
fn webpinfo_output_matches_the_oracle() {
    let Some(oracle) = Oracle::locate("webpinfo_output_matches_the_oracle") else {
        return;
    };
    let c_tool = oracle.tool("webpinfo");
    let rust_tool = consumer_tool(&oracle, "webpinfo");
    let directory = scratch(&oracle, "webpinfo");
    let mut files: Vec<PathBuf> = oracle::test_vectors();
    // Truncated copies: inside the VP8X, inside a chunk header and halfway.
    for path in oracle::test_vectors().iter().step_by(3) {
        let data = std::fs::read(path).unwrap();
        for cut in [12, 21, 34, data.len() / 2] {
            if cut < data.len() {
                let name = format!("{}.{cut}", path.file_name().unwrap().to_str().unwrap());
                let truncated = directory.join(name);
                std::fs::write(&truncated, &data[..cut]).unwrap();
                files.push(truncated);
            }
        }
    }
    let mut runs = 0;
    for file in &files {
        let file = file.to_str().unwrap();
        for options in [
            &[][..],
            &["-diag"],
            &["-summary"],
            &["-bitstream_info"],
            &["-quiet"],
        ] {
            let mut args: Vec<&str> = options.to_vec();
            args.push(file);
            let c = tool_output(&c_tool, &args);
            let rust = tool_output(&rust_tool, &args);
            assert_eq!(
                (
                    String::from_utf8_lossy(&c.0),
                    String::from_utf8_lossy(&c.1),
                    c.2
                ),
                (
                    String::from_utf8_lossy(&rust.0),
                    String::from_utf8_lossy(&rust.1),
                    rust.2
                ),
                "webpinfo {args:?}"
            );
            runs += 1;
        }
    }
    let version = tool_output(&rust_tool, &["-version"]);
    assert_eq!(version, tool_output(&c_tool, &["-version"]));
    assert!(runs > 600);
}

#[test]
fn dwebp_writes_the_files_of_the_oracle() {
    let Some(oracle) = Oracle::locate("dwebp_writes_the_files_of_the_oracle") else {
        return;
    };
    let c_tool = oracle.tool("dwebp");
    let rust_tool = consumer_tool(&oracle, "dwebp");
    let directory = scratch(&oracle, "dwebp");
    let option_sets: &[&[&str]] = &[
        &["-pam"],
        &["-ppm", "-nofancy"],
        &["-bmp", "-nofilter"],
        &["-tiff", "-mt"],
        &["-pam", "-flip", "-mt"],
        &["-pam", "-crop", "3", "5", "40", "31"],
        &["-yuv", "-crop", "4", "6", "33", "17", "-flip"],
        &["-pam", "-resize", "37", "0"],
        &[
            "-ppm", "-crop", "1", "1", "60", "60", "-resize", "20", "23", "-mt",
        ],
        &["-pam", "-dither", "50", "-alpha_dither"],
        &["-pam", "-incremental"],
        &["-yuv", "-incremental", "-mt", "-flip"],
        &["-pgm", "-incremental", "-crop", "8", "8", "48", "48"],
        &["-alpha"],
        &["-pixel_format", "rgbA", "-external_memory", "1"],
        &[
            "-pixel_format",
            "Argb",
            "-external_memory",
            "2",
            "-incremental",
        ],
        &[
            "-pixel_format",
            "RGBA_4444",
            "-external_memory",
            "1",
            "-flip",
        ],
        &["-pixel_format", "rgbA_4444", "-mt"],
        &[
            "-pixel_format",
            "RGB_565",
            "-incremental",
            "-external_memory",
            "1",
        ],
        &["-pixel_format", "YUVA", "-external_memory", "1"],
        &[
            "-pixel_format",
            "YUV",
            "-external_memory",
            "2",
            "-incremental",
        ],
        &["-pixel_format", "BGRA", "-noasm", "-nofancy"],
    ];
    let mut files = 0;
    for (index, path) in oracle::test_vectors().iter().enumerate() {
        let name = path.file_name().unwrap().to_str().unwrap();
        if name == "bryce.webp" || name == "lossless_big_random_alpha.webp" {
            continue;
        }
        // Every vector with three option sets, in rotation.
        for set in 0..3 {
            let options = option_sets[(index * 3 + set) % option_sets.len()];
            let c_out = directory.join("c.out");
            let rust_out = directory.join("rust.out");
            let _ = std::fs::remove_file(&c_out);
            let _ = std::fs::remove_file(&rust_out);
            let mut args: Vec<&str> = vec![path.to_str().unwrap()];
            args.extend_from_slice(options);
            let mut c_args = args.clone();
            c_args.extend(["-o", c_out.to_str().unwrap()]);
            let mut rust_args = args.clone();
            rust_args.extend(["-o", rust_out.to_str().unwrap()]);
            let c = tool_output(&c_tool, &c_args);
            let rust = tool_output(&rust_tool, &rust_args);
            assert_eq!(c.2, rust.2, "dwebp {args:?}: exit status");
            assert_eq!(
                std::fs::read(&c_out).ok(),
                std::fs::read(&rust_out).ok(),
                "dwebp {args:?}: output file"
            );
            files += 1;
        }
    }
    assert!(files > 300);
}
