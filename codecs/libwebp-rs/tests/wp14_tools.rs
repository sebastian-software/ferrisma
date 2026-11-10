//! WP14 consumer gates: upstream tools, unmodified, linked against the C
//! library and against the Rust static library, must give identical results.
//!
//! - `anim_dump` (`examples/anim_dump.c` with `anim_util.c`), which decodes an
//!   animation with `WebPAnimDecoder` and writes every canvas: the oracle builds
//!   it against the C library as `probes/wp14/anim_dump` and against the Rust
//!   library as `probes/wp14/anim_dump_rs` (`tools/oracle/probes/wp14/oracle.cmake`).
//!   Both run on every synthetic and `img2webp` animation and on test vectors,
//!   in PAM, and for some inputs in TIFF and PNG; the exit status, the standard
//!   output and error, and every written file must agree.
//! - `cwebp -metadata` on WebP input, which reads the ICC, EXIF and XMP chunks
//!   through `WebPDemux()` and `WebPDemuxGetChunk()` (imageio's `ReadWebP`): the
//!   oracle's `cwebp` and WP13's Rust-linked `probes/wp12/cwebp`, which the
//!   demux link stand-ins kept from this path before WP14.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **GIF input to `anim_dump`** needs giflib and is not built (see
//!    `tools/oracle/probes/wp14/oracle.cmake`); PNG output needs libpng in
//!    imageio, whose flags both builds share, so the two agree whether or not
//!    the host has it.

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
use wp8_support::{chunk, riff, top_level_chunks, vp8x_payload};
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

/// Runs `tool` with `args` in the empty directory `cwd` and collects the run.
fn run(tool: &Path, cwd: &Path, args: &[&str]) -> Run {
    std::fs::create_dir_all(cwd).unwrap();
    let output = Command::new(tool)
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap_or_else(|error| panic!("cannot run {}: {error}", tool.display()));
    let mut files = BTreeMap::new();
    for entry in std::fs::read_dir(cwd).unwrap() {
        let path = entry.unwrap().path();
        files.insert(
            path.file_name().unwrap().to_string_lossy().into_owned(),
            std::fs::read(&path).unwrap(),
        );
        std::fs::remove_file(&path).unwrap();
    }
    Run {
        code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        files,
    }
}

#[test]
fn anim_dump_matches_between_the_c_and_the_rust_library() {
    let Some(oracle) = Oracle::locate("anim_dump_matches_between_the_c_and_the_rust_library")
    else {
        return;
    };
    let c_tool = executable(&oracle, "probes/wp14/anim_dump");
    let rs_tool = executable(&oracle, "probes/wp14/anim_dump_rs");
    let directory = work_dir("wp14_tools_anim_dump");
    let inputs_dir = directory.join("inputs");
    std::fs::create_dir_all(&inputs_dir).unwrap();
    let mut files = write_all(&inputs_dir, &synthetic_animations());
    files.extend(img2webp_animations(&oracle, &inputs_dir));
    for name in [
        "lossless_vec_1_0.webp",
        "lossy_alpha1.webp",
        "test.webp",
        "very_short.webp",
    ] {
        let path = oracle::test_data().join(name);
        files.push((name.to_owned(), path));
    }

    let (mut runs, mut written, mut failed) = (0, 0, 0);
    for (index, (_, path)) in files.iter().enumerate() {
        let path = path.to_str().unwrap();
        let mut formats = vec![vec!["-pam"]];
        if index % 5 == 0 {
            formats.push(vec!["-tiff"]);
        }
        if index % 23 == 0 {
            formats.push(Vec::new()); // PNG, which needs libpng in imageio
        }
        for format in formats {
            let mut args = vec!["-folder", ".", "-prefix", "frame_"];
            args.extend(format);
            args.push(path);
            let c = run(&c_tool, &directory.join("c"), &args);
            let rs = run(&rs_tool, &directory.join("rs"), &args);
            assert_eq!(rs, c, "anim_dump {args:?}");
            runs += 1;
            written += c.files.len();
            failed += usize::from(c.code != Some(0));
        }
    }
    // Several files in one call, with one that fails in the middle.
    let several: Vec<&str> = ["random_00", "frame_outside_canvas", "edges"]
        .iter()
        .map(|name| {
            files
                .iter()
                .find(|(found, _)| found == name)
                .unwrap()
                .1
                .to_str()
                .unwrap()
        })
        .collect();
    let mut args = vec!["-pam"];
    args.extend(&several);
    assert_eq!(
        run(&rs_tool, &directory.join("rs"), &args),
        run(&c_tool, &directory.join("c"), &args)
    );
    for args in [&["-version"][..], &["-h"], &[], &["-folder"]] {
        let c = run(&c_tool, &directory.join("c"), args);
        let rs = run(&rs_tool, &directory.join("rs"), args);
        assert_eq!(rs, c, "anim_dump {args:?}");
    }
    eprintln!("compared {runs} anim_dump runs, {written} written frames, {failed} failed runs");
}

/// A still image in an extended container with ICC, EXIF and XMP chunks around
/// the image of `test.webp`.
fn still_with_metadata() -> Vec<u8> {
    let file = std::fs::read(oracle::test_data().join("test.webp")).unwrap();
    let (_, vp8) = top_level_chunks(&file)
        .into_iter()
        .find(|(tag, _)| tag == b"VP8 ")
        .unwrap();
    let mut body = chunk(b"VP8X", &vp8x_payload(0x20 | 0x08 | 0x04, 128, 128));
    body.extend_from_slice(&chunk(b"ICCP", b"an ICC profile that nobody parses"));
    body.extend_from_slice(&chunk(b"VP8 ", &vp8));
    body.extend_from_slice(&chunk(b"EXIF", b"Exif\0\0MM\0*"));
    body.extend_from_slice(&chunk(b"XMP ", b"<x:xmpmeta xmlns:x='adobe:ns:meta/'/>"));
    riff(&body)
}

#[test]
fn cwebp_metadata_matches_between_the_c_and_the_rust_library() {
    let Some(oracle) = Oracle::locate("cwebp_metadata_matches_between_the_c_and_the_rust_library")
    else {
        return;
    };
    let c_tool = oracle.tool("cwebp");
    let rs_tool = executable(&oracle, "probes/wp12/cwebp");
    let directory = work_dir("wp14_tools_cwebp");
    let inputs_dir = directory.join("inputs");
    std::fs::create_dir_all(&inputs_dir).unwrap();
    let mut inputs = write_all(
        &inputs_dir,
        &[("still_with_metadata".to_owned(), still_with_metadata())],
    );
    let animations = synthetic_animations();
    let animated: Vec<(String, Vec<u8>)> = animations
        .into_iter()
        .filter(|(name, _)| name == "metadata" || name == "still_alpha")
        .collect();
    inputs.extend(write_all(&inputs_dir, &animated));
    for name in ["lossy_alpha1.webp", "lossless_vec_1_0.webp"] {
        inputs.push((name.to_owned(), oracle::test_data().join(name)));
    }

    let mut runs = 0;
    for (_, path) in &inputs {
        let path = path.to_str().unwrap();
        for metadata in ["all", "icc", "exif,xmp", "none"] {
            for extra in [&["-q", "60"][..], &["-lossless", "-m", "1"]] {
                let mut args = vec!["-quiet", "-metadata", metadata];
                args.extend(extra);
                args.extend([path, "-o", "out.webp"]);
                let c = run(&c_tool, &directory.join("c"), &args);
                let rs = run(&rs_tool, &directory.join("rs"), &args);
                assert_eq!(rs, c, "cwebp {args:?}");
                if path.ends_with("still_with_metadata.webp") && metadata == "all" {
                    // The metadata path was taken: the chunks reach the output.
                    assert_eq!(c.code, Some(0), "{c:?}");
                    let output = &c.files["out.webp"];
                    for tag in [b"ICCP", b"EXIF", b"XMP "] {
                        assert!(output.windows(4).any(|window| window == tag), "{tag:?}");
                    }
                }
                runs += 1;
            }
        }
    }
    eprintln!("compared {runs} cwebp runs");
}
