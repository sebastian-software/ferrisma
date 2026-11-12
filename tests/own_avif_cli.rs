#![cfg(feature = "cli")]

use std::process::Command;

#[test]
fn normal_cli_selects_own_encoder_with_order_independent_depth() {
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
    for flags in [
        &["--avif-encoder=libavif", "--avif-depth=8"][..],
        &["--avif-depth=8", "--avif-encoder=libavif"],
        // `cli` includes `avif-own-write`, so libavif is the default encoder
        // and a depth alone applies to it.
        &[],
        &["--avif-depth=8"],
        &["--quality=55"],
    ] {
        assert_eq!(encoder(flags), "libavif-rs/aom", "{flags:?}");
    }
    // Rav1e stays an explicit backend with its own quality scale.
    assert_eq!(encoder(&["--avif-encoder=rav1e"]), "rav1e");
    assert_eq!(encoder(&["--avif-encoder=rav1e", "--quality=80"]), "rav1e");
    let invalid = run(&["--avif-encoder=rav1e", "--avif-depth=8"]);
    assert!(!invalid.status.success());
    assert!(
        String::from_utf8_lossy(&invalid.stderr).contains("requires the libavif encoder"),
        "{}",
        String::from_utf8_lossy(&invalid.stderr)
    );
    // Quality reaches only the selected encoder: Rav1e rejects 0 (its scale
    // is 1-100), libavif accepts it on its own 0-100 scale.
    assert!(
        !run(&["--avif-encoder=rav1e", "--quality=0"])
            .status
            .success()
    );
    assert_eq!(encoder(&["--quality=0"]), "libavif-rs/aom");
    std::fs::remove_file(output).unwrap();
}
