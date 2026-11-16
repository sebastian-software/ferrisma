//! The command's error output and exit status (since 2026-11-16, ADR-0040
//! amendment of that date): `--help` prints the usage on standard output and
//! exits 0; every usage error exits 2 and every processing error 1, each
//! with one `error: <message>` line on standard error, and an error the
//! engine returned adds a `failure: kind=… reason=…` line.
#![cfg(feature = "cli")]

use std::path::PathBuf;
use std::process::{Command, Output};

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn temp(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("cli-exit-{}-{name}", std::process::id()))
}

fn ferrisma(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ferrisma"))
        .args(args)
        .output()
        .unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn help_prints_the_usage_on_standard_output_and_succeeds() {
    for flag in ["--help", "-h"] {
        for args in [&[flag][..], &["missing.jpg", "out.webp", "8", "8", flag]] {
            let result = ferrisma(args);
            assert_eq!(result.status.code(), Some(0), "{args:?}");
            let stdout = text(&result.stdout);
            assert!(stdout.starts_with("usage: "), "{stdout}");
            assert!(stdout.contains("--kernel-level=scalar|neon|avx2|avx512"));
            assert!(stdout.contains("--help|-h"));
            assert!(result.stderr.is_empty(), "{}", text(&result.stderr));
        }
    }
    // `--version` keeps its precedence and output.
    let version = ferrisma(&["--help", "--version"]);
    assert_eq!(version.status.code(), Some(0));
    assert!(text(&version.stdout).starts_with("ferrisma "));
}

#[test]
fn usage_errors_exit_with_status_2() {
    let jpeg = format!("{ROOT}/tests/fixtures/sample.jpg");
    let webp = temp("usage.webp");
    let webp = webp.to_str().unwrap();
    let avif = temp("usage.avif");
    let avif = avif.to_str().unwrap();
    // Too few positional arguments print the usage after the error line.
    for args in [&[][..], &[jpeg.as_str(), webp, "8"]] {
        let result = ferrisma(args);
        assert_eq!(result.status.code(), Some(2), "{args:?}");
        let stderr = text(&result.stderr);
        assert!(
            stderr.starts_with(
                "error: missing arguments; INPUT OUTPUT WIDTH HEIGHT are required\nusage: "
            ),
            "{stderr}"
        );
        assert!(result.stdout.is_empty(), "{args:?}");
    }
    for (output, flags, message) in [
        (webp, &["--fast"][..], "error: unknown option: --fast\n"),
        (
            webp,
            &["--area", "--area"],
            "error: --area may only be given once\n",
        ),
        (
            webp,
            &["--assume-srgb", "--use-rendered-icc"],
            "error: --assume-srgb and --use-rendered-icc are mutually exclusive\n",
        ),
        (
            webp,
            &["--transform-threads=x"],
            "error: invalid value in --transform-threads=x: invalid digit found in string\n",
        ),
        (
            webp,
            &["--crop=0,0,4"],
            "error: crop must be x,y,width,height\n",
        ),
        (
            webp,
            &["--avif-quality=80"],
            "error: --avif-quality requires .avif output\n",
        ),
        // A value the engine refuses as out of range is a usage error too;
        // its `failure:` line names the kind.
        (
            avif,
            &["--assume-srgb", "--avif-depth=9"],
            "error: the requested output depth is not one this encoder writes\n\
             failure: kind=invalid-options reason=none\n",
        ),
        (
            webp,
            &["--assume-srgb", "--max-input-mpx=0"],
            "error: input byte/pixel limits must be nonzero\n\
             failure: kind=invalid-options reason=none\n",
        ),
    ] {
        let mut args = vec![jpeg.as_str(), output, "8", "8"];
        args.extend_from_slice(flags);
        let result = ferrisma(&args);
        assert_eq!(result.status.code(), Some(2), "{flags:?}");
        assert_eq!(text(&result.stderr), message, "{flags:?}");
        assert!(result.stdout.is_empty(), "{flags:?}");
    }
    for (args, message) in [
        (
            [jpeg.as_str(), webp, "x", "8"],
            "error: invalid WIDTH \"x\": invalid digit found in string\n",
        ),
        (
            [jpeg.as_str(), "out.png", "8", "8"],
            "error: output must have a .avif, .webp or .jxl extension\n",
        ),
        (
            ["in.png", webp, "8", "8"],
            "error: unsupported input file extension; expected .jpg, .jpeg, .webp, .jxl or .avif\n",
        ),
        (
            [jpeg.as_str(), webp, "0", "8"],
            "error: image dimensions must be nonzero\nfailure: kind=invalid-options reason=none\n",
        ),
    ] {
        let result = ferrisma(&args);
        assert_eq!(result.status.code(), Some(2), "{args:?}");
        assert_eq!(text(&result.stderr), message, "{args:?}");
    }
}

#[test]
fn processing_errors_exit_with_status_1() {
    let jpeg = format!("{ROOT}/tests/fixtures/sample.jpg");
    let webp = temp("processing.webp");
    let webp = webp.to_str().unwrap();
    let run = |input: &str, size: &str, flags: &[&str]| {
        let mut args = vec![input, webp, size, size];
        args.extend_from_slice(flags);
        ferrisma(&args)
    };
    // A refused input: an untagged JPEG under the strict policy.
    let refused = run(&jpeg, "8", &[]);
    assert_eq!(refused.status.code(), Some(1));
    let stderr = text(&refused.stderr);
    let lines: Vec<&str> = stderr.lines().collect();
    assert_eq!(lines.len(), 2, "{stderr}");
    assert!(lines[0].starts_with("error: "), "{stderr}");
    assert_eq!(
        lines[1],
        "failure: kind=unsupported-color reason=color-signal-missing"
    );
    // Malformed input.
    let broken = temp("broken.jpg");
    std::fs::write(&broken, b"not a jpeg").unwrap();
    let malformed = run(broken.to_str().unwrap(), "8", &["--assume-srgb"]);
    assert_eq!(malformed.status.code(), Some(1));
    assert!(
        text(&malformed.stderr).ends_with("\nfailure: kind=malformed-input reason=none\n"),
        "{}",
        text(&malformed.stderr)
    );
    // A resource limit: the output frame does not fit a 1 MiB budget.
    let exhausted = run(&jpeg, "4000", &["--assume-srgb", "--budget-mb=1"]);
    assert_eq!(exhausted.status.code(), Some(1));
    assert!(
        text(&exhausted.stderr).ends_with("\nfailure: kind=resource-exhausted reason=none\n"),
        "{}",
        text(&exhausted.stderr)
    );
    // An unreadable input is a processing error without a `failure:` line.
    let missing = temp("missing.jpg");
    let unreadable = run(missing.to_str().unwrap(), "8", &["--assume-srgb"]);
    assert_eq!(unreadable.status.code(), Some(1));
    let stderr = text(&unreadable.stderr);
    assert!(
        stderr.starts_with(&format!("error: cannot read {}: ", missing.display())),
        "{stderr}"
    );
    assert_eq!(stderr.lines().count(), 1, "{stderr}");
    std::fs::remove_file(broken).unwrap();
    // Success still exits 0, with nothing but the memory note on stderr.
    let success = run(&jpeg, "8", &["--assume-srgb"]);
    assert_eq!(success.status.code(), Some(0), "{}", text(&success.stderr));
    assert!(!text(&success.stderr).contains("error"));
    std::fs::remove_file(webp).unwrap();
}
