#![cfg(feature = "cli")]

use std::process::Command;

#[test]
fn cli_writes_avif_through_libavif_at_either_depth() {
    let input = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/sample.jpg");
    let output =
        std::env::temp_dir().join(format!("ferrisma-own-avif-{}-cli.avif", std::process::id()));
    let run = |flags: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_ferrisma"))
            .args([
                input,
                output.to_str().unwrap(),
                "8",
                "8",
                "--assume-srgb",
                "--encoder-threads=1",
            ])
            .args(flags)
            .output()
            .unwrap()
    };
    let encoder = |flags: &[&str]| {
        let result = run(flags);
        assert!(
            result.status.success(),
            "{flags:?}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(&std::fs::read(&output).unwrap()[4..8], b"ftyp");
        let stdout = String::from_utf8_lossy(&result.stdout).into_owned();
        stdout
            .split_whitespace()
            .find_map(|field| field.strip_prefix("encoder="))
            .expect("the report names the encoder")
            .to_owned()
    };
    // One AVIF writer since 2026-11-15: libavif, with its own settings.
    for flags in [
        &[][..],
        &["--avif-depth=8"],
        &["--avif-depth=10"],
        &["--avif-depth=12"],
        &["--avif-quality=55"],
        &[
            "--avif-alpha-quality=90",
            "--avif-speed=8",
            "--avif-chroma=420",
        ],
    ] {
        assert_eq!(encoder(flags), "libavif/aom", "{flags:?}");
    }
    // The former selector is an unknown option, never silently ignored.
    for flags in [&["--avif-encoder=libavif"][..], &["--avif-encoder=rav1e"]] {
        let removed = run(flags);
        assert!(!removed.status.success(), "{flags:?}");
        assert!(
            String::from_utf8_lossy(&removed.stderr).contains("unknown option"),
            "{}",
            String::from_utf8_lossy(&removed.stderr)
        );
    }
    // 12 is AVIF's largest depth and admitted since 2026-11-15 (owner
    // decision on the output depth); before, 12 was the refused example.
    for depth in ["--avif-depth=9", "--avif-depth=16"] {
        let invalid = run(&[depth]);
        assert!(!invalid.status.success(), "{depth}");
    }
    // libavif's scale is 0-100; quality 0 is accepted, not remapped.
    assert_eq!(encoder(&["--avif-quality=0"]), "libavif/aom");
    // The format-independent flags were removed on 2026-11-15.
    for flags in [&["--quality=55"][..], &["--speed=6"], &["--chroma=420"]] {
        let removed = run(flags);
        assert_eq!(removed.status.code(), Some(2), "{flags:?}");
        assert!(
            String::from_utf8_lossy(&removed.stderr).starts_with("error: unknown option: --"),
            "{}",
            String::from_utf8_lossy(&removed.stderr)
        );
    }
    std::fs::remove_file(output).unwrap();
}

/// `--avif-decoder=` (since 2026-11-22): the dav1d port decodes AVIF input
/// by default, `libaom` selects the reference decoder, both write the same
/// bytes and the report names the one that ran. With other input the
/// nondefault choice is refused as a usage error (ADR-0042), not ignored.
#[test]
fn cli_reads_avif_with_dav1d_by_default_and_libaom_on_request() {
    let avif = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/orientation/avif-6.avif"
    );
    let jpeg = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/sample.jpg");
    let output = |tag: &str| {
        std::env::temp_dir().join(format!(
            "ferrisma-avif-decoder-{}-{tag}.webp",
            std::process::id()
        ))
    };
    let run = |input: &str, tag: &str, flags: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_ferrisma"))
            .args([
                input,
                output(tag).to_str().unwrap(),
                "8",
                "8",
                "--assume-srgb",
            ])
            .args(flags)
            .output()
            .unwrap()
    };
    let decoder = |result: &std::process::Output| {
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        String::from_utf8_lossy(&result.stdout)
            .split_whitespace()
            .find_map(|field| field.strip_prefix("decoder="))
            .expect("the report names the decoder")
            .to_owned()
    };
    assert_eq!(decoder(&run(avif, "default", &[])), "libavif/dav1d");
    assert_eq!(
        decoder(&run(avif, "dav1d", &["--avif-decoder=dav1d"])),
        "libavif/dav1d"
    );
    assert_eq!(
        decoder(&run(avif, "libaom", &["--avif-decoder=libaom"])),
        "libavif/aom"
    );
    let default = std::fs::read(output("default")).unwrap();
    assert_eq!(default, std::fs::read(output("dav1d")).unwrap());
    assert_eq!(default, std::fs::read(output("libaom")).unwrap());
    let refused = run(jpeg, "jpeg", &["--avif-decoder=libaom"]);
    assert_eq!(refused.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&refused.stderr)
            .contains("avif_decoder applies to AVIF input only"),
        "{}",
        String::from_utf8_lossy(&refused.stderr)
    );
    for tag in ["default", "dav1d", "libaom", "jpeg"] {
        let _ = std::fs::remove_file(output(tag));
    }
}
