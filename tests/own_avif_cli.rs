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
    for flags in [
        ["--avif-encoder=libavif", "--avif-depth=8"],
        ["--avif-depth=8", "--avif-encoder=libavif"],
    ] {
        let result = run(&flags);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert!(String::from_utf8_lossy(&result.stdout).contains("encoder=libavif-rs/aom"));
        assert_eq!(&std::fs::read(&output).unwrap()[4..8], b"ftyp");
    }
    let invalid = run(&["--avif-encoder=rav1e", "--avif-depth=8"]);
    assert!(!invalid.status.success());
    assert!(String::from_utf8_lossy(&invalid.stderr).contains("requires --avif-encoder=libavif"));
    std::fs::remove_file(output).unwrap();
}
