use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    time::Instant,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Default, Debug, PartialEq, Eq)]
struct Selection {
    engine: bool,
    kernels: bool,
    adapter: bool,
    jpeg: bool,
    webp: bool,
    jxl: bool,
    avif: bool,
    workspace: bool,
}
impl Selection {
    fn all(&mut self) {
        *self = Self {
            engine: true,
            kernels: true,
            adapter: true,
            jpeg: true,
            webp: true,
            jxl: true,
            avif: true,
            workspace: true,
        };
    }
    fn classify(&mut self, path: &str) {
        if path.starts_with("codecs/libjpeg-turbo-rs/") || path == "codecs/libjpeg-turbo-rs" {
            self.jpeg = true;
            self.workspace = true;
        } else if path.starts_with("codecs/libwebp-rs/") || path == "codecs/libwebp-rs" {
            self.webp = true;
            self.workspace = true;
        } else if path.starts_with("codecs/libjxl-rs/") || path == "codecs/libjxl-rs" {
            self.jxl = true;
            self.workspace = true;
        } else if path.starts_with("codecs/libavif-rs/") || path == "codecs/libavif-rs" {
            self.avif = true;
            self.adapter = true;
            self.workspace = true;
        } else if path.starts_with("crates/ferrixel-kernels/") {
            self.kernels = true;
            self.engine = true;
            self.adapter = true;
        } else if path.starts_with("crates/ferrixel-libavif/") {
            self.adapter = true;
        } else if ["src/", "tests/", "examples/"]
            .iter()
            .any(|p| path.starts_with(p))
            || [
                "scripts/audit-licenses.py",
                "scripts/test_audit_licenses.py",
                "scripts/bench-m1.sh",
                "scripts/m1-performance-smoke.sh",
                "scripts/m1-sharp-smoke.cjs",
            ]
            .contains(&path)
            || path.starts_with("scripts/bench/")
        {
            self.engine = true;
            self.adapter = true;
        } else if (path.starts_with("docs/")
            && [".md", ".png", ".jpg", ".jpeg", ".svg", ".webp"]
                .iter()
                .any(|suffix| path.ends_with(suffix)))
            || (!path.contains('/') && path.ends_with(".md"))
        {
        } else {
            self.all();
        }
    }
    fn output(&self) -> String {
        let mut rows = Vec::new();
        for (name, selected) in [
            ("jpeg", self.jpeg),
            ("webp", self.webp),
            ("jxl", self.jxl),
            ("avif", self.avif),
        ] {
            if selected {
                rows.push(format!(
                    "{{\"codec\":\"{name}\",\"runner\":\"ubuntu-24.04\"}}"
                ));
                if name != "avif" {
                    rows.push(format!(
                        "{{\"codec\":\"{name}\",\"runner\":\"ubuntu-24.04-arm\"}}"
                    ));
                }
            }
        }
        let has = !rows.is_empty();
        if !has {
            rows.push("{\"codec\":\"none\",\"runner\":\"ubuntu-24.04\"}".into());
        }
        format!(
            "engine={}\nkernels={}\nadapter={}\njpeg={}\nwebp={}\njxl={}\navif={}\nworkspace={}\nhas-codecs={}\nmatrix={{\"include\":[{}]}}\n",
            self.engine,
            self.kernels,
            self.adapter,
            self.jpeg,
            self.webp,
            self.jxl,
            self.avif,
            self.workspace,
            has,
            rows.join(",")
        )
    }
}
fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}
fn option(args: &[String], flag: &str) -> Result<Option<String>> {
    let mut values = args.iter().enumerate().filter(|(_, a)| a.as_str() == flag);
    let Some((i, _)) = values.next() else {
        return Ok(None);
    };
    if values.next().is_some() {
        return Err(format!("duplicate {flag}").into());
    }
    Ok(Some(
        args.get(i + 1)
            .filter(|v| !v.starts_with("--"))
            .ok_or(format!("missing value for {flag}"))?
            .clone(),
    ))
}
fn validate_args(args: &[String], valued: &[&str], switches: &[&str]) -> Result<()> {
    let mut i = 0;
    while i < args.len() {
        let flag = args[i].as_str();
        if valued.contains(&flag) {
            i += 1;
            if i == args.len() || args[i].starts_with("--") {
                return Err(format!("missing value for {flag}").into());
            }
        } else if !switches.contains(&flag) {
            return Err(format!("unknown argument: {flag}").into());
        }
        i += 1;
    }
    Ok(())
}
fn selection_from_git(dir: &Path, base: &str, head: &str) -> Result<Selection> {
    let result = Command::new("git")
        .current_dir(dir)
        .args([
            "diff",
            "--no-renames",
            "--name-only",
            "-z",
            base,
            head,
            "--",
        ])
        .output()?;
    if !result.status.success() {
        return Err(format!(
            "git diff failed: {}",
            String::from_utf8_lossy(&result.stderr)
        )
        .into());
    }
    let mut selection = Selection::default();
    for path in result.stdout.split(|b| *b == 0).filter(|p| !p.is_empty()) {
        match std::str::from_utf8(path) {
            Ok(p) => selection.classify(p),
            Err(_) => selection.all(),
        }
    }
    Ok(selection)
}
fn affected(args: &[String]) -> Result<()> {
    validate_args(args, &["--base", "--head", "--output"], &["--all"])?;
    let mut selection = Selection::default();
    let all = args.iter().any(|a| a == "--all");
    let base = option(args, "--base")?;
    let head = option(args, "--head")?;
    let output = option(args, "--output")?;
    if all {
        if base.is_some() || head.is_some() {
            return Err("--all cannot be combined with --base or --head".into());
        }
        selection.all();
    } else {
        let base = base.ok_or("missing --base")?;
        let head = head.ok_or("missing --head")?;
        selection = selection_from_git(&root(), &base, &head)?;
    }
    let result = selection.output();
    print!("{result}");
    if let Some(path) = output {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        file.write_all(result.as_bytes())?;
    }
    Ok(())
}
fn run(dir: &Path, command: &[&str]) -> Result<()> {
    let start = Instant::now();
    println!("[{}] {}", dir.display(), command.join(" "));
    let status = Command::new(command[0])
        .args(&command[1..])
        .current_dir(dir)
        .status()?;
    println!(
        "[{}] completed in {:.1}s ({status})",
        command[0],
        start.elapsed().as_secs_f64()
    );
    if !status.success() {
        return Err(format!("command failed: {}", command.join(" ")).into());
    }
    Ok(())
}
fn codecs(args: &[String]) -> Result<()> {
    validate_args(args, &["--codec", "--stage"], &["--tests-only"])?;
    let codec = option(args, "--codec")?.ok_or("missing --codec")?;
    let stage = option(args, "--stage")?.ok_or("missing --stage")?;
    let tests_only = args.iter().any(|x| x == "--tests-only");
    let dir = root().join("codecs").join(match codec.as_str() {
        "jpeg" => "libjpeg-turbo-rs",
        "webp" => "libwebp-rs",
        "jxl" => "libjxl-rs",
        "avif" => "libavif-rs",
        _ => return Err("invalid codec".into()),
    });
    if !["pr", "static", "test", "oracle"].contains(&stage.as_str()) {
        return Err("invalid stage".into());
    }
    if tests_only && stage != "pr" {
        return Err("--tests-only requires --stage pr".into());
    }
    if stage != "pr" && codec != "avif" {
        let wrapper_stage = if stage == "oracle" { "test" } else { &stage };
        let mut cmd = vec!["python3", "tools/check.py", "--stage", wrapper_stage];
        if stage == "oracle" && codec != "jxl" {
            cmd.push("--oracle");
        }
        run(&dir, &cmd)?;
        if stage == "oracle" && codec == "webp" {
            cmd.push("--no-simd");
            run(&dir, &cmd)?;
        }
        return Ok(());
    }
    if codec == "avif" {
        if stage == "static" || (stage == "pr" && !tests_only) {
            for cmd in [
                &["python3", "tools/verify_upstream.py", "--tracked"][..],
                &["python3", "tools/check_translation.py"],
                &["python3", "tools/generate_licenses.py", "--check"],
                &[
                    "python3",
                    "-m",
                    "unittest",
                    "discover",
                    "-s",
                    "tests/ports",
                    "-p",
                    "*_test.py",
                ],
            ] {
                run(&dir, cmd)?;
            }
        }
        if stage == "test" || stage == "pr" {
            run(
                &dir,
                &[
                    "cargo",
                    "test",
                    "--locked",
                    "--offline",
                    "--release",
                    "-p",
                    "libavif-rs",
                    "-p",
                    "libavif-aom",
                    "-p",
                    "libavif-dav1d",
                    "-p",
                    "libavif-sharpyuv",
                ],
            )?;
        }
        if stage == "pr" {
            run(
                &dir,
                &[
                    "cargo",
                    "test",
                    "--locked",
                    "--offline",
                    "--profile",
                    "checked",
                    "-p",
                    "libavif-sharpyuv",
                    "--test",
                    "cross_port",
                ],
            )?;
            run(
                &dir,
                &[
                    "python3",
                    "tools/check_aom_golden.py",
                    "--implementation",
                    "rust",
                ],
            )?;
        }
        if stage == "oracle" {
            for cmd in [
                &["python3", "tools/build_sharpyuv.py"][..],
                &[
                    "cargo",
                    "build",
                    "--release",
                    "--locked",
                    "--offline",
                    "-p",
                    "libavif-sharpyuv",
                ],
                &["python3", "tools/check_sharpyuv.py"],
                &[
                    "cargo",
                    "test",
                    "--release",
                    "--locked",
                    "--offline",
                    "-p",
                    "libavif-sharpyuv",
                    "--test",
                    "cross_port",
                ],
                &["python3", "tools/check_aom_golden.py"],
            ] {
                run(&dir, cmd)?;
            }
        }
        return Ok(());
    }
    let package = match codec.as_str() {
        "jpeg" => "libjpeg-turbo-rs",
        "webp" => "libwebp-rs",
        _ => "libjxl-rs",
    };
    if !tests_only {
        run(
            &dir,
            &["python3", "tools/check.py", "--stage", "static", "--msrv"],
        )?;
    }
    run(
        &dir,
        &[
            "cargo",
            "test",
            "--locked",
            "--offline",
            "-p",
            package,
            "--profile",
            "checked",
        ],
    )?;
    if codec != "jxl" {
        run(
            &dir,
            &[
                "cargo",
                "test",
                "--locked",
                "--offline",
                "-p",
                package,
                "--profile",
                "checked",
                "--no-default-features",
            ],
        )?;
    } else if tests_only {
        let mut names = Vec::new();
        for entry in fs::read_dir(dir.join("tests"))? {
            let path = entry?.path();
            if path.extension().is_some_and(|e| e == "rs")
                && fs::read_to_string(&path)?.contains("cfg(not(debug_assertions))")
            {
                names.push(path.file_stem().unwrap().to_string_lossy().into_owned());
            }
        }
        names.sort();
        for name in names {
            run(
                &dir,
                &[
                    "cargo",
                    "test",
                    "--locked",
                    "--offline",
                    "-p",
                    package,
                    "--release",
                    "--test",
                    &name,
                ],
            )?;
        }
    }
    Ok(())
}
fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) { Some("affected") => affected(&args[1..]), Some("codecs") => codecs(&args[1..]), _ => Err("usage: cargo xtask affected --base SHA --head SHA [--output PATH] | affected --all | codecs --codec NAME --stage pr|static|test|oracle [--tests-only]".into()) };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("xtask: {e}");
            ExitCode::FAILURE
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dependencies_and_docs() {
        let mut s = Selection::default();
        s.classify("docs/architecture.md");
        assert_eq!(s, Selection::default());
        s.classify("crates/ferrixel-kernels/src/lib.rs");
        assert!(s.kernels && s.engine && s.adapter && !s.jpeg);
        s.classify("codecs/libavif-rs/src/lib.rs");
        assert!(s.avif && s.adapter);
    }
    #[test]
    fn conservative_and_webp() {
        let mut s = Selection::default();
        s.classify("codecs/libwebp-rs/src/lib.rs");
        assert!(s.webp && s.workspace && !s.adapter && !s.avif);
        s.classify("new-config.toml");
        assert_eq!(s, {
            let mut all = Selection::default();
            all.all();
            all
        });
    }
    #[test]
    fn git_diff_selects_both_sides_of_rename_and_deletion() {
        let dir = std::env::temp_dir().join(format!("ferrixel-xtask-test-{}", std::process::id()));
        fs::create_dir_all(dir.join("codecs/libjpeg-turbo-rs")).unwrap();
        fs::create_dir_all(dir.join("codecs/libjxl-rs")).unwrap();
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
        git(&["init", "-q"]);
        git(&["config", "commit.gpgsign", "false"]);
        git(&["config", "user.name", "Test"]);
        git(&["config", "user.email", "test@example.invalid"]);
        fs::write(dir.join("codecs/libjpeg-turbo-rs/old.rs"), "same content").unwrap();
        fs::write(dir.join("codecs/libjpeg-turbo-rs/deleted.rs"), "deleted").unwrap();
        git(&["add", "."]);
        git(&["commit", "-qm", "initial"]);
        fs::rename(
            dir.join("codecs/libjpeg-turbo-rs/old.rs"),
            dir.join("codecs/libjxl-rs/new.rs"),
        )
        .unwrap();
        fs::remove_file(dir.join("codecs/libjpeg-turbo-rs/deleted.rs")).unwrap();
        git(&["add", "-A"]);
        git(&["commit", "-qm", "move"]);
        let selection = selection_from_git(&dir, "HEAD~1", "HEAD").unwrap();
        assert!(selection.jpeg && selection.jxl && !selection.webp && selection.workspace);
        fs::remove_dir_all(dir).unwrap();
    }
    #[test]
    fn docs_code_is_conservative() {
        let mut selection = Selection::default();
        selection.classify("docs/example.rs");
        assert!(selection.jpeg && selection.engine && selection.workspace);
    }
    #[test]
    fn known_engine_scripts_are_scoped_and_unknown_scripts_fail_closed() {
        let mut selection = Selection::default();
        selection.classify("scripts/audit-licenses.py");
        assert!(selection.engine && selection.adapter && !selection.jpeg);
        selection.classify("scripts/new-shared-tool.py");
        assert!(
            selection.jpeg
                && selection.webp
                && selection.jxl
                && selection.avif
                && selection.workspace
        );
    }
    #[test]
    fn failed_subcommand_propagates() {
        assert!(run(&root(), &["git", "--not-a-valid-git-option"]).is_err());
    }
    #[test]
    fn sentinel_matrix() {
        let out = Selection::default().output();
        assert!(out.contains("has-codecs=false"));
        assert!(out.contains("\"codec\":\"none\""));
    }
}
