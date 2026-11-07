//! WP12 differential test of the C ABI: the probe
//! `tools/oracle/probes/wp12/decode.c` built against the C library
//! (`probes/wp12/decode`) and against the Rust static library through
//! `src/abi/decode.rs` (`probes/wp12/decode_rs`, the consumer build of
//! `tools/oracle/probes/wp12/oracle.cmake`) runs the same commands, and the
//! outputs must be identical: every `NULL` and invalid argument, the features,
//! `WebPDecode` with library and caller memory, the simple API, the `...Into`
//! functions and the incremental decoder through every constructor, with
//! `WebPIAppend`, `WebPIUpdate`, `WebPIDecGetRGB`, `WebPIDecGetYUVA` and
//! `WebPIDecodedArea` after every step.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp12_support/mod.rs"]
mod wp12_support;

use std::path::Path;

use oracle::Oracle;
use wp12_support::{Geometry, Options, Rng, compare, geometry_for, output_dimensions, vectors};

fn dimensions(path: &Path) -> (i32, i32, bool) {
    let data = std::fs::read(path).unwrap();
    let mut f = webp::webp::decode::WebPBitstreamFeatures::default();
    let _ = webp::webp::decode::webp_get_features(&data, &mut f);
    (f.width, f.height, f.has_alpha != 0)
}

/// Runs the commands through the C library and through the Rust library's C
/// ABI and compares the outputs.
fn run(test: &str, commands: &[String]) {
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let input: String = commands.concat();
    let c = oracle
        .probe("wp12", "decode")
        .run::<&str>(&[], input.as_bytes());
    c.success();
    let rust = oracle
        .probe("wp12", "decode_rs")
        .run::<&str>(&[], input.as_bytes());
    rust.success();
    let rust_text = String::from_utf8(rust.stdout.clone()).expect("UTF-8");
    compare(test, &c.stdout, &rust_text);
}

#[test]
fn abi_answers_null_and_invalid_arguments_as_c() {
    let commands: Vec<String> = [
        "test.webp",
        "lossless_vec_1_0.webp",
        "lossy_alpha1.webp",
        "small_1x1.webp",
    ]
    .iter()
    .map(|name| format!("abi {}\n", oracle::test_data().join(name).display()))
    .collect();
    run("abi_answers_null_and_invalid_arguments_as_c", &commands);
}

#[test]
fn abi_decodes_as_c_through_the_advanced_and_simple_api() {
    let mut commands = Vec::new();
    let mut rng = Rng(5150);
    for path in oracle::test_vectors() {
        let p = path.display();
        commands.push(format!("features {p} -1\n"));
        commands.push(format!("features {p} 25\n"));
        if !vectors().contains(&path) {
            commands.push(format!("simple {p} -1 RGBA\n"));
            continue;
        }
        let (width, height, alpha) = dimensions(&path);
        for func in ["RGB", "RGBA", "ARGB", "BGR", "BGRA", "YUV"] {
            commands.push(format!("simple {p} -1 {func}\n"));
        }
        for mode in 0..=13u32 {
            commands.push(format!(
                "decode {p} -1 {mode} {} -\n",
                Options::default().token()
            ));
        }
        if width <= 0 {
            continue;
        }
        for (func, mode) in [
            ("RGB", 0u32),
            ("RGBA", 1),
            ("ARGB", 4),
            ("BGR", 2),
            ("BGRA", 3),
            ("YUV", 11),
        ] {
            let geometry = geometry_for(mode, width, height, rng.range(0, 3) as i32, false);
            commands.push(format!("into {p} -1 {func} {}\n", geometry.token()));
        }
        let mut options = Options::default();
        options.0[Options::FLIP] = 1;
        options.0[Options::THREADS] = 1;
        options.0[Options::CROP] = 1;
        options.0[Options::CROP_LEFT] = width / 5;
        options.0[Options::CROP_TOP] = height / 7;
        options.0[Options::CROP_WIDTH] = (width / 2).max(1);
        options.0[Options::CROP_HEIGHT] = (height / 2).max(1);
        for scale in [false, true] {
            let mut options = options;
            if scale {
                options.0[Options::SCALE] = 1;
                options.0[Options::SCALED_WIDTH] = (width / 3).max(1);
                options.0[Options::SCALED_HEIGHT] = (height * 2 / 3).max(1);
            }
            let mode = rng.below(13) as u32;
            commands.push(format!("decode {p} -1 {mode} {} -\n", options.token()));
            if let Some((w, h)) = output_dimensions(width, height, &options) {
                let mode = rng.below(13) as u32;
                let geometry = geometry_for(mode, w, h, 3, mode == 12 || (mode == 11 && alpha));
                commands.push(format!(
                    "decode {p} -1 {mode} {} {}\n",
                    options.token(),
                    geometry.token()
                ));
            }
        }
    }
    run(
        "abi_decodes_as_c_through_the_advanced_and_simple_api",
        &commands,
    );
}

#[test]
fn abi_decodes_incrementally_as_c() {
    let mut commands = Vec::new();
    let mut rng = Rng(808);
    for path in vectors() {
        let p = path.display();
        let (width, height, alpha) = dimensions(&path);
        let size = std::fs::metadata(&path).unwrap().len();
        let chunk = if size <= 32768 { 64 } else { 4096 };
        let mut options = Options::default();
        options.0[Options::THREADS] = 1;
        options.0[Options::FLIP] = rng.below(2) as i32;
        let none = Options::default().token();
        for append in [0, 1] {
            let mode = rng.below(13);
            commands.push(format!(
                "incr {p} -1 {chunk} {append} config {mode} {} -\n",
                options.token()
            ));
        }
        commands.push(format!("incr {p} -1 {chunk} 1 new 0 {none} -\n"));
        if width <= 0 {
            continue;
        }
        let mode = rng.below(13) as u32;
        let geometry = geometry_for(mode, width, height, 2, mode == 12 || (mode == 11 && alpha));
        commands.push(format!(
            "incr {p} -1 {chunk} 0 config {mode} {none} {}\n",
            geometry.token()
        ));
        let rgb = rng.below(11) as u32;
        let geometry: Geometry = geometry_for(rgb, width, height, 1, false);
        commands.push(format!(
            "incr {p} -1 {chunk} 1 rgb {rgb} {none} {}\n",
            geometry.token()
        ));
        commands.push(format!("incr {p} -1 {chunk} 0 rgb {rgb} {none} -\n"));
        let geometry = geometry_for(12, width, height, 0, true);
        commands.push(format!(
            "incr {p} -1 {chunk} 1 yuva 12 {none} {}\n",
            geometry.token()
        ));
        commands.push(format!("incr {p} -1 {chunk} 0 yuva 12 {none} -\n"));
        let geometry = geometry_for(11, width, height, 4, false);
        commands.push(format!(
            "incr {p} -1 {chunk} 1 yuv 11 {none} {}\n",
            geometry.token()
        ));
        commands.push(format!(
            "incr {p} {} 64 1 config 1 {none} -\n",
            size * 2 / 3
        ));
    }
    run("abi_decodes_incrementally_as_c", &commands);
}
