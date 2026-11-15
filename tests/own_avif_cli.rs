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
    // One AVIF writer since 2026-11-15: libavif, with a depth.
    for flags in [
        &[][..],
        &["--avif-depth=8"],
        &["--avif-depth=10"],
        &["--quality=55"],
    ] {
        assert_eq!(encoder(flags), "libavif-rs/aom", "{flags:?}");
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
    let invalid = run(&["--avif-depth=12"]);
    assert!(!invalid.status.success());
    // libavif's scale is 0-100; quality 0 is accepted, not remapped.
    assert_eq!(encoder(&["--quality=0"]), "libavif-rs/aom");
    std::fs::remove_file(output).unwrap();
}
