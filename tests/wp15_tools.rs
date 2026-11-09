//! WP15 consumer gates: upstream tools, unmodified, linked against the C
//! library and against the Rust static library, must give identical results
//! (`tools/oracle/probes/wp15/oracle.cmake`):
//!
//! - `webpmux` (`examples/webpmux.c`), which reads, edits and writes WebP
//!   containers through the whole mux API: `-info`, `-get`, `-set`, `-strip`,
//!   `-duration`, `-frame` with `-loop` and `-bgcolor`, argument files and
//!   errors, on test vectors, files with metadata and animations;
//! - `img2webp` (`examples/img2webp.c`), which encodes frame sequences with the
//!   animation encoder and re-muxes the loop count: the oracle's build and
//!   `probes/wp15/img2webp_rs`, over WP14's option sets and more;
//! - `anim_diff` (`examples/anim_diff.c` with `anim_util.c`), which decodes two
//!   animations with the animation decoder and compares them, with its
//!   options and dumped frames.
//!
//! Each run of a tool happens in an empty directory; its exit status, standard
//! output and error and every file it writes must agree.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp14_support/mod.rs"]
mod wp14_support;
#[path = "wp8_support/mod.rs"]
mod wp8_support;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use oracle::Oracle;
use wp14_support::{img2webp_animations, synthetic_animations, work_dir, write_all};

/// A probe executable of the oracle build, which must exist.
fn executable(oracle: &Oracle, path: &str) -> PathBuf {
    let path = oracle.directory().join(path);
    assert!(
        path.is_file(),
        "missing {}; rebuild the oracle with python3 tools/build_oracle.py --rust-lib",
        path.display()
    );
    path
}

/// What one run of a tool leaves behind: the exit code, standard output and
/// error, and the files it wrote.
#[derive(Debug, PartialEq, Eq)]
struct Run {
    code: Option<i32>,
    stdout: String,
    stderr: String,
    files: BTreeMap<String, Vec<u8>>,
}

/// Runs `tool` with `args` in the empty directory `cwd` and collects the run;
/// the files it wrote (also in subdirectories) are removed afterwards.
fn run(tool: &Path, cwd: &Path, args: &[String]) -> Run {
    std::fs::create_dir_all(cwd).unwrap();
    let output = Command::new(tool)
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap_or_else(|error| panic!("cannot run {}: {error}", tool.display()));
    let mut files = BTreeMap::new();
    collect(cwd, cwd, &mut files);
    Run {
        code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        files,
    }
}

fn collect(root: &Path, directory: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
    for entry in std::fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect(root, &path, files);
            std::fs::remove_dir(&path).unwrap();
        } else {
            let name = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            files.insert(name, std::fs::read(&path).unwrap());
            std::fs::remove_file(&path).unwrap();
        }
    }
}

/// Runs both builds of a tool with each argument list and compares the runs.
/// Returns the number of runs and of failed runs.
fn compare_tools(
    c_tool: &Path,
    rust_tool: &Path,
    directory: &Path,
    commands: &[Vec<String>],
) -> (usize, usize) {
    let mut failures = 0;
    for args in commands {
        let c = run(c_tool, &directory.join("c"), args);
        let rust = run(rust_tool, &directory.join("rust"), args);
        assert_eq!(c, rust, "{} {}", rust_tool.display(), args.join(" "));
        if c.code != Some(0) {
            failures += 1;
        }
    }
    (commands.len(), failures)
}

fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|arg| (*arg).to_owned()).collect()
}

fn path_arg(path: &Path) -> String {
    path.display().to_string()
}

#[test]
fn webpmux_matches_between_the_c_and_the_rust_library() {
    let Some(oracle) = Oracle::locate("webpmux_matches_between_the_c_and_the_rust_library") else {
        return;
    };
    let c_tool = executable(&oracle, "probes/wp15/webpmux");
    let rust_tool = executable(&oracle, "probes/wp15/webpmux_rs");
    let directory = work_dir("wp15_tools_webpmux");
    let inputs = directory.join("inputs");
    std::fs::create_dir_all(&inputs).unwrap();

    // Metadata payloads and argument files.
    let icc = inputs.join("profile.icc");
    std::fs::write(
        &icc,
        (0..300u32).map(|i| (i * 7) as u8).collect::<Vec<u8>>(),
    )
    .unwrap();
    let exif = inputs.join("meta.exif");
    std::fs::write(&exif, b"Exif\0\0MM\0*\0\0\0\x08").unwrap();
    let xmp = inputs.join("meta.xmp");
    std::fs::write(&xmp, b"<x:xmpmeta xmlns:x='adobe:ns:meta/'/>").unwrap();
    let empty = inputs.join("empty.bin");
    std::fs::write(&empty, b"").unwrap();

    // Still images: test vectors, and files with metadata that the C tool
    // writes first (the inputs are the same for both builds).
    let mut stills: Vec<PathBuf> = oracle::test_vectors()
        .into_iter()
        .filter(|path| std::fs::metadata(path).unwrap().len() < 40_000)
        .step_by(4)
        .collect();
    for (index, source) in stills.clone().iter().take(4).enumerate() {
        let target = inputs.join(format!("meta{index}.webp"));
        let mut command = Command::new(&c_tool);
        command
            .args(["-set", "icc"])
            .arg(&icc)
            .arg(source)
            .arg("-o")
            .arg(&target);
        assert!(command.status().unwrap().success());
        for (option, file) in [("exif", &exif), ("xmp", &xmp)] {
            let status = Command::new(&c_tool)
                .args(["-set", option])
                .arg(file)
                .arg(&target)
                .arg("-o")
                .arg(&target)
                .status()
                .unwrap();
            assert!(status.success());
        }
        stills.push(target);
    }
    let animations: Vec<PathBuf> = write_all(&inputs, &synthetic_animations())
        .into_iter()
        .step_by(9)
        .map(|(_, path)| path)
        .chain(
            img2webp_animations(&oracle, &inputs)
                .into_iter()
                .step_by(3)
                .map(|(_, path)| path),
        )
        .collect();

    let mut commands = Vec::new();
    for input in stills.iter().chain(&animations) {
        let input = path_arg(input);
        commands.push(args(&["-info", &input]));
        for what in ["icc", "exif", "xmp"] {
            commands.push(args(&["-get", what, &input, "-o", "out.bin"]));
            commands.push(args(&["-strip", what, &input, "-o", "out.webp"]));
        }
        for nth in ["0", "1", "2", "99"] {
            commands.push(args(&["-get", "frame", nth, &input, "-o", "frame.webp"]));
        }
        commands.push(args(&[
            "-set",
            "icc",
            &path_arg(&icc),
            &input,
            "-o",
            "out.webp",
        ]));
        commands.push(args(&[
            "-set",
            "exif",
            &path_arg(&exif),
            &input,
            "-o",
            "out.webp",
        ]));
        commands.push(args(&[
            "-set",
            "xmp",
            &path_arg(&empty),
            &input,
            "-o",
            "out.webp",
        ]));
        commands.push(args(&["-set", "loop", "3", &input, "-o", "out.webp"]));
        commands.push(args(&["-set", "loop", "70000", &input, "-o", "out.webp"]));
        commands.push(args(&[
            "-set",
            "bgcolor",
            "255,0,128,64",
            &input,
            "-o",
            "out.webp",
        ]));
        commands.push(args(&["-duration", "50", &input, "-o", "out.webp"]));
        commands.push(args(&[
            "-duration",
            "70,2",
            "-duration",
            "30,1,0",
            &input,
            "-o",
            "out.webp",
        ]));
    }
    // Animations from still frames.
    let frames: Vec<String> = stills.iter().take(6).map(|path| path_arg(path)).collect();
    commands.push(args(&[
        "-frame",
        &frames[0],
        "+100",
        "-frame",
        &frames[1],
        "+50+10+10+1-b",
        "-loop",
        "2",
        "-bgcolor",
        "255,255,255,255",
        "-o",
        "anim.webp",
    ]));
    commands.push(args(&[
        "-frame",
        &frames[2],
        "+0+3+5",
        "-frame",
        &frames[3],
        "+16777216",
        "-o",
        "anim.webp",
    ]));
    commands.push(args(&[
        "-frame",
        &frames[4],
        "+20+0+0+0+b",
        "-frame",
        &frames[5],
        "+20+1+1+1",
        "-frame",
        &frames[4],
        "+20",
        "-o",
        "anim.webp",
    ]));
    commands.push(args(&[
        "-frame",
        &path_arg(&animations[0]),
        "+10",
        "-o",
        "anim.webp",
    ]));
    // Argument files, help, version and errors.
    let argument_file = inputs.join("args.txt");
    std::fs::write(&argument_file, format!("-info {}\n", frames[0])).unwrap();
    commands.push(vec![path_arg(&argument_file)]);
    for list in [
        &["-version"][..],
        &["-h"],
        &[],
        &["-info"],
        &["-get", "icc", &frames[0]],
        &["-set", "loop", "x", &frames[0], "-o", "out.webp"],
        &["-info", &path_arg(&empty)],
        &["-strip", "abc", &frames[0], "-o", "out.webp"],
        &["-info", "missing.webp"],
    ] {
        commands.push(args(list));
    }
    let (runs, failures) = compare_tools(&c_tool, &rust_tool, &directory, &commands);
    eprintln!(
        "webpmux_matches_between_the_c_and_the_rust_library: {runs} runs, {failures} failing runs"
    );
}

#[test]
fn img2webp_matches_between_the_c_and_the_rust_library() {
    let Some(oracle) = Oracle::locate("img2webp_matches_between_the_c_and_the_rust_library") else {
        return;
    };
    let c_tool = oracle.tool("img2webp");
    let rust_tool = executable(&oracle, "probes/wp15/img2webp_rs");
    let directory = work_dir("wp15_tools_img2webp");
    let inputs = directory.join("inputs");
    std::fs::create_dir_all(&inputs).unwrap();
    // Writes the PAM sequences (opaque, clear and half backgrounds).
    let _ = img2webp_animations(&oracle, &inputs);
    let frames = |background: &str| -> Vec<String> {
        (0..8)
            .map(|index| path_arg(&inputs.join(format!("{background}_{index}.pam"))))
            .collect()
    };
    let option_sets: [&[&str]; 22] = [
        &[],
        &["-lossy", "-q", "60"],
        &["-mixed", "-q", "70"],
        &["-loop", "3", "-d", "70", "-lossy", "-q", "40"],
        &["-kmin", "2", "-kmax", "3", "-lossless"],
        &["-min_size", "-lossy", "-q", "80"],
        &["-kmax", "0", "-exact", "-lossy", "-m", "2"],
        &["-loop", "0", "-mixed"],
        &["-loop", "65535", "-lossy"],
        &["-sharp_yuv", "-lossy", "-q", "50"],
        &["-near_lossless", "40", "-lossless"],
        &["-m", "0", "-q", "100", "-lossless"],
        &["-noexact", "-lossless", "-d", "0"],
        &["-mixed", "-min_size"],
        &["-kmax", "1", "-lossy"],
        &["-kmin", "3", "-kmax", "10", "-mixed"],
        &["-v", "-mixed"],
        &["-d", "-5"],
        &["-loop", "-1"],
        &["-unknown"],
        &["-q", "abc"],
        &["-lossy", "-q", "75", "-exact"],
    ];
    let mut commands = Vec::new();
    for background in ["opaque", "clear", "half"] {
        let frames = frames(background);
        for options in option_sets {
            let mut list = args(options);
            for (index, frame) in frames.iter().enumerate() {
                if index == 3 {
                    list.extend(args(&["-d", "1"]));
                }
                if index == 5 {
                    list.extend(args(&["-lossy", "-q", "30"]));
                }
                list.push(frame.clone());
            }
            list.extend(args(&["-o", "out.webp"]));
            commands.push(list);
        }
    }
    // One frame, the same frame repeated, a frame of another size, no input,
    // no output file, and a trailing option.
    let opaque = frames("opaque");
    let grid = path_arg(&oracle::test_data().join("grid.pam"));
    commands.push(args(&[&opaque[0], "-o", "one.webp"]));
    commands.push(args(&[
        &opaque[2],
        &opaque[2],
        &opaque[2],
        &opaque[3],
        "-o",
        "same.webp",
    ]));
    commands.push(args(&[&opaque[0], &grid, "-o", "mismatch.webp"]));
    commands.push(args(&["-lossy", "-o", "none.webp"]));
    commands.push(args(&[&opaque[0], &opaque[1]]));
    commands.push(args(&[
        &opaque[0],
        &opaque[1],
        "-o",
        "trailing.webp",
        "-lossy",
    ]));
    commands.push(args(&["-h"]));
    let (runs, failures) = compare_tools(&c_tool, &rust_tool, &directory, &commands);
    eprintln!(
        "img2webp_matches_between_the_c_and_the_rust_library: {runs} runs, {failures} failing runs"
    );
}

#[test]
fn anim_diff_matches_between_the_c_and_the_rust_library() {
    let Some(oracle) = Oracle::locate("anim_diff_matches_between_the_c_and_the_rust_library")
    else {
        return;
    };
    let c_tool = executable(&oracle, "probes/wp15/anim_diff");
    let rust_tool = executable(&oracle, "probes/wp15/anim_diff_rs");
    let directory = work_dir("wp15_tools_anim_diff");
    let inputs = directory.join("inputs");
    std::fs::create_dir_all(&inputs).unwrap();
    let mut files: Vec<PathBuf> = img2webp_animations(&oracle, &inputs)
        .into_iter()
        .map(|(_, path)| path)
        .collect();
    files.extend(
        write_all(&inputs, &synthetic_animations())
            .into_iter()
            .step_by(11)
            .map(|(_, path)| path),
    );
    let still = path_arg(&oracle::test_data().join("lossless1.webp"));
    let mut commands = Vec::new();
    for (index, pair) in files.windows(2).enumerate() {
        let (first, second) = (path_arg(&pair[0]), path_arg(&pair[1]));
        commands.push(args(&[&first, &first]));
        commands.push(args(&[&first, &second]));
        match index % 4 {
            0 => commands.push(args(&[&first, &second, "-min_psnr", "30"])),
            1 => commands.push(args(&[&first, &second, "-raw_comparison"])),
            2 => commands.push(args(&[&first, &second, "-max_diff", "3"])),
            _ => commands.push(args(&[&first, &first, "-dump_frames", "."])),
        }
    }
    let first = path_arg(&files[0]);
    for list in [
        &[first.as_str(), still.as_str()][..],
        &[still.as_str(), still.as_str()],
        &[first.as_str()],
        &[first.as_str(), "missing.webp"],
        &[first.as_str(), first.as_str(), "-min_psnr"],
        &["-h"],
        &["-version"],
    ] {
        commands.push(args(list));
    }
    let (runs, failures) = compare_tools(&c_tool, &rust_tool, &directory, &commands);
    eprintln!(
        "anim_diff_matches_between_the_c_and_the_rust_library: {runs} runs, {failures} failing runs"
    );
}
