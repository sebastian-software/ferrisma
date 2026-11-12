use std::process::Command;

#[test]
fn invalid_git_revision_fails_without_outputs() {
    let output = Command::new(env!("CARGO_BIN_EXE_ferrisma-xtask"))
        .args([
            "affected",
            "--base",
            "not-a-valid-ferrisma-revision",
            "--head",
            "HEAD",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("git diff failed"));
}
