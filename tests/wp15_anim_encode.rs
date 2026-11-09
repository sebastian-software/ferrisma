//! WP15 differential tests of the animation encoder (`src/mux/anim_encode.rs`)
//! against the C oracle.
//!
//! The probe `tools/oracle/probes/wp15/anim_enc.c` runs a script of encoder
//! commands against the C library; `wp15_support::anim_report` runs it through
//! the translation and must print the same lines: every result, error code,
//! error string, the number of progress-hook calls and the size and digest of
//! every assembled animation (byte-identical output). The one recorded
//! deviation is the hook that C calls again in `WebPAnimEncoderAssemble`
//! (`wp15_support::ANIM_DEVIATION_KEYS`). The assembled animations are then
//! decoded with WP14's animation decoder and compared with the C decoder
//! (`tests/wp14_support::anim_report` against `probes/wp14/anim`).
//!
//! Frames are generated RGBA files: moving shapes over an opaque background,
//! over a transparent one whose pixels keep their colors (so that the lossless
//! cleanup of transparent pixels, which C performs through its view of the
//! canvas, matters to later candidates), over a translucent one and over
//! noise, at odd and even sizes, with repeated frames.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp14_support/mod.rs"]
mod wp14_support;
#[path = "wp15_support/mod.rs"]
mod wp15_support;
#[path = "wp8_support/mod.rs"]
mod wp8_support;

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use oracle::Oracle;
use wp8_support::Rng;
use wp14_support::{AnimCommand, work_dir};
use wp15_support::{anim_report, compare_anim};

/// The background of a generated frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Background {
    /// An opaque gradient.
    Opaque,
    /// Transparent pixels that keep a color gradient.
    Clear,
    /// Translucent (alpha 128) pixels.
    Half,
    /// Opaque noise: many colors.
    Noise,
    /// Two flat colors: few colors.
    Flat,
    /// Opaque noise on the left half, transparent pixels that keep a color
    /// gradient on the right: lossy candidates win, and the lossless
    /// candidate's cleanup of the transparent pixels changes what they encode.
    Patchy,
}

/// The RGBA samples of frame `index`: a disc and a square that move with the
/// index over the background.
fn frame(width: u32, height: u32, index: u32, background: Background) -> Vec<u8> {
    let mut rng = Rng(u64::from(width * 1000 + height));
    let mut out = Vec::with_capacity((width * height * 4) as usize);
    let cx = i64::from(index * 5 % width.max(1));
    let cy = i64::from(index * 3 % height.max(1));
    let sx = i64::from(width.max(1) - 1 - index * 4 % width.max(1));
    let radius = i64::from(width.min(height) / 4 + 1);
    for y in 0..i64::from(height) {
        for x in 0..i64::from(width) {
            let noise = [rng.byte(), rng.byte(), rng.byte()];
            let disc = (x - cx).pow(2) + (y - cy).pow(2) <= radius * radius;
            let square = (x - sx).abs() <= 3 && (y - i64::from(height) / 2).abs() <= 3;
            let pixel = if disc {
                [240, 50, 20, 255]
            } else if square {
                [20, 190, 70, 170]
            } else {
                match background {
                    Background::Opaque => [(x * 5) as u8, (y * 7) as u8, 100, 255],
                    Background::Clear => [(x * 9) as u8, (y * 4) as u8, 200, 0],
                    Background::Half => [(x * 3) as u8, 90, (y * 6) as u8, 128],
                    Background::Noise => [noise[0], noise[1], noise[2], 255],
                    Background::Patchy => {
                        if x < i64::from(width) / 2 {
                            [noise[0], noise[1], noise[2], 255]
                        } else {
                            [(x * 9) as u8, (y * 4) as u8, 200, 0]
                        }
                    }
                    Background::Flat => {
                        if (x / 8 + y / 8) % 2 == 0 {
                            [10, 20, 30, 255]
                        } else {
                            [200, 210, 220, 255]
                        }
                    }
                }
            };
            out.extend_from_slice(&pixel);
        }
    }
    out
}

/// Writes the frame files that a test uses, once per (size, background,
/// index).
struct Frames {
    directory: PathBuf,
}

impl Frames {
    fn path(&self, width: u32, height: u32, index: u32, background: Background) -> PathBuf {
        let path = self
            .directory
            .join(format!("{background:?}_{width}x{height}_{index}.rgba"));
        if !path.exists() {
            std::fs::write(&path, frame(width, height, index, background)).unwrap();
        }
        path
    }
}

/// One encoder script.
#[derive(Clone, Debug)]
struct Case {
    name: String,
    options: String,
    width: u32,
    height: u32,
    background: Background,
    /// Frame index and timestamp of each `add`.
    frames: Vec<(u32, i32)>,
    /// The `config` line before each frame (the last one repeats).
    configs: Vec<String>,
    /// `use_argb` of each frame (the last one repeats).
    argb: Vec<i32>,
    hook: i32,
    /// The timestamp of a final `addnull`.
    null_frame: Option<i32>,
    /// Lines inserted before `assemble`.
    extra: String,
}

impl Case {
    fn new(name: &str, width: u32, height: u32, background: Background, count: u32) -> Self {
        Case {
            name: name.to_owned(),
            options: "options null".to_owned(),
            width,
            height,
            background,
            frames: (0..count).map(|i| (i, 100 * i as i32)).collect(),
            configs: vec!["config null".to_owned()],
            argb: vec![1],
            hook: -1,
            null_frame: Some(100 * count as i32),
            extra: String::new(),
        }
    }

    fn script(&self, frames: &Frames, output: &Path) -> String {
        let mut script = String::new();
        writeln!(script, "{}", self.options).unwrap();
        writeln!(script, "new {} {}", self.width, self.height).unwrap();
        writeln!(script, "hook {}", self.hook).unwrap();
        for (position, &(index, timestamp)) in self.frames.iter().enumerate() {
            let config = &self.configs[position.min(self.configs.len() - 1)];
            let argb = self.argb[position.min(self.argb.len() - 1)];
            let path = frames.path(self.width, self.height, index, self.background);
            writeln!(script, "{config}").unwrap();
            writeln!(
                script,
                "add {timestamp} {} {} {argb} {}",
                self.width,
                self.height,
                path.display()
            )
            .unwrap();
        }
        if let Some(timestamp) = self.null_frame {
            writeln!(script, "addnull {timestamp}").unwrap();
        }
        script.push_str(&self.extra);
        writeln!(script, "assemble {}", output.display()).unwrap();
        script
    }
}

/// Runs the cases through the C probe and the translation, compares the
/// outputs and decodes every assembled animation with both animation
/// decoders. Returns the number of encoder commands.
fn run_cases(oracle: &Oracle, test: &str, cases: &[Case]) -> usize {
    let directory = work_dir(test);
    let frames = Frames {
        directory: directory.to_path_buf(),
    };
    let mut commands = 0;
    let mut deviations = 0;
    let mut outputs = Vec::new();
    for (number, case) in cases.iter().enumerate() {
        let output = directory.join(format!("out{number}.webp"));
        let script = case.script(&frames, &output);
        commands += script.lines().count();
        let result = oracle
            .probe("wp15", "anim_enc")
            .run::<&str>(&[], script.as_bytes());
        result.success();
        let c = String::from_utf8(result.stdout).unwrap();
        let rust = anim_report(&script);
        deviations += compare_anim(test, &case.name, &c, &rust);
        if output.exists() {
            outputs.push(output);
        }
    }
    decode_outputs(oracle, test, &outputs);
    eprintln!(
        "{test}: {} cases, {commands} commands, {} animations decoded, {deviations} assemble.hook deviations",
        cases.len(),
        outputs.len()
    );
    commands
}

/// Decodes the C and the Rust output of each case: the C file with the C
/// animation decoder (WP14's probe), the Rust file with the translation's
/// (`wp14_support::anim_report`); the files are equal, as the digests showed.
fn decode_outputs(oracle: &Oracle, test: &str, outputs: &[PathBuf]) {
    for path in outputs {
        let rust_path = PathBuf::from(format!("{}.rs", path.display()));
        assert_eq!(
            std::fs::read(path).unwrap(),
            std::fs::read(&rust_path).unwrap(),
            "{test}: {} differs",
            path.display()
        );
        for mode in [1, 8] {
            let command = AnimCommand {
                mode,
                threads: 0,
                reset: 0,
                dump: 0,
                path: path.clone(),
            };
            let c = oracle
                .probe("wp14", "anim")
                .run::<&str>(&[], command.line(false).as_bytes());
            c.success();
            let rust_command = AnimCommand {
                path: rust_path.clone(),
                ..command
            };
            let rust = wp14_support::anim_report(&rust_command);
            // The probe numbers its commands; the report has no such line.
            let c: String = String::from_utf8(c.stdout)
                .unwrap()
                .lines()
                .filter(|line| !line.starts_with("command="))
                .flat_map(|line| [line, "\n"])
                .collect();
            assert_eq!(c, rust, "{test}: decoding {} differs", path.display());
        }
    }
}

const CONFIGS: [&str; 9] = [
    "config null",
    "config lossless=1 quality=75 method=4",
    "config lossless=0 quality=60 method=4",
    "config lossless=0 quality=90 method=2 alpha_quality=60",
    "config lossless=1 quality=20 method=0 exact=1",
    "config lossless=0 quality=50 method=6 exact=1 thread_level=1",
    "config lossless=1 quality=90 method=5 near_lossless=60 thread_level=1",
    "config lossless=0 quality=75 method=3 use_sharp_yuv=1 sns_strength=0",
    "config lossless=0 quality=30 method=4 autofilter=1 filter_strength=40 segments=2",
];

#[test]
fn option_and_config_matrix_matches_the_c_encoder() {
    let Some(oracle) = Oracle::locate("option_and_config_matrix_matches_the_c_encoder") else {
        return;
    };
    let options = [
        "options null",
        "options 0 0 0 0 0 4294967295 0",
        "options 0 2 4 0 0 0 3",
        "options 1 0 0 0 0 16711935 1",
        "options 0 0 0 1 0 4294967295 0",
        "options 1 0 0 1 0 0 0",
        "options 0 1 3 1 0 12345 65535",
    ];
    let backgrounds = [
        Background::Opaque,
        Background::Clear,
        Background::Half,
        Background::Noise,
        Background::Flat,
    ];
    let mut cases = Vec::new();
    for (o, option) in options.iter().enumerate() {
        for (c, config) in CONFIGS.iter().enumerate() {
            let background = backgrounds[(o + c) % backgrounds.len()];
            let (width, height) = [(48, 32), (37, 23), (24, 24)][(o * 3 + c) % 3];
            let mut case = Case::new(
                &format!("matrix {option} {config}"),
                width,
                height,
                background,
                6,
            );
            case.options = (*option).to_owned();
            case.configs = vec![(*config).to_owned()];
            // A repeated frame, merged into the previous one.
            case.frames[3].0 = case.frames[2].0;
            // YUV input for some frames.
            if c % 3 == 2 {
                case.argb = vec![1, 0];
            }
            cases.push(case);
        }
    }
    // Both candidates of a frame, where the lossy one wins after the lossless
    // one cleaned the transparent pixels of the canvas (C writes through its
    // view of the canvas; docs/status/wp15.md).
    for option in [
        "options 1 0 0 1 0 0 0",
        "options 0 0 0 1 0 0 0",
        "options 1 2 5 1 0 0 0",
    ] {
        for config in [
            CONFIGS[1],
            CONFIGS[2],
            CONFIGS[6],
            "config lossless=0 quality=70 exact=0",
        ] {
            for (width, height) in [(48, 32), (37, 29)] {
                let mut case = Case::new(
                    &format!("patchy {option} {config} {width}x{height}"),
                    width,
                    height,
                    Background::Patchy,
                    5,
                );
                case.options = option.to_owned();
                case.configs = vec![config.to_owned()];
                cases.push(case);
            }
        }
    }
    // Mixed configurations per frame.
    let mut mixed = Case::new("mixed configs", 40, 30, Background::Clear, 8);
    mixed.options = "options 0 0 0 1 0 0 0".to_owned();
    mixed.configs = CONFIGS[1..].iter().map(|c| (*c).to_owned()).collect();
    cases.push(mixed);
    run_cases(&oracle, "wp15_anim_matrix", &cases);
}

#[test]
fn keyframe_options_match_the_c_encoder() {
    let Some(oracle) = Oracle::locate("keyframe_options_match_the_c_encoder") else {
        return;
    };
    let mut cases = Vec::new();
    let max = i32::MAX;
    for kmax in [-5, -1, 0, 1, 2, 3, 5, 10, 31, 40, 100, max] {
        let mut kmins = vec![
            -3,
            0,
            1,
            2,
            kmax.saturating_sub(1),
            kmax,
            kmax.saturating_add(1),
            kmax / 2,
            kmax / 2 + 1,
            kmax.saturating_sub(31),
            kmax.saturating_sub(30),
        ];
        kmins.sort_unstable();
        kmins.dedup();
        for kmin in kmins {
            for minimize in [0, 1] {
                let mut case = Case::new(
                    &format!("kmin {kmin} kmax {kmax} minimize {minimize}"),
                    32,
                    24,
                    if kmin % 2 == 0 {
                        Background::Opaque
                    } else {
                        Background::Clear
                    },
                    14,
                );
                case.options = format!("options {minimize} {kmin} {kmax} 0 0 0 0");
                case.configs = vec![CONFIGS[(kmax.unsigned_abs() as usize + 1) % 3].to_owned()];
                cases.push(case);
            }
        }
    }
    // Long sequences past kmax.
    for (kmin, kmax, mixed) in [(3, 5, 0), (6, 10, 1), (2, 3, 0), (0, 1, 1)] {
        let mut case = Case::new(
            &format!("long kmin {kmin} kmax {kmax}"),
            28,
            20,
            Background::Half,
            40,
        );
        case.options = format!("options 0 {kmin} {kmax} {mixed} 0 4294967295 0");
        case.configs = vec![CONFIGS[2].to_owned(), CONFIGS[1].to_owned()];
        cases.push(case);
    }
    run_cases(&oracle, "wp15_anim_keyframes", &cases);
}

#[test]
fn timestamps_errors_and_hooks_match_the_c_encoder() {
    let Some(oracle) = Oracle::locate("timestamps_errors_and_hooks_match_the_c_encoder") else {
        return;
    };
    let mut cases = Vec::new();
    let base = Case::new("base", 30, 20, Background::Clear, 5);
    let mut with = |name: &str, change: &dyn Fn(&mut Case)| {
        let mut case = base.clone();
        case.name = name.to_owned();
        change(&mut case);
        cases.push(case);
    };
    with("decreasing timestamp", &|c| c.frames[3].1 = 150);
    with("repeated timestamps", &|c| {
        c.frames[2].1 = 100;
        c.frames[3].1 = 100;
    });
    with("gap of MAX_DURATION", &|c| c.frames[2].1 = 100 + (1 << 24));
    with("gap just below MAX_DURATION", &|c| {
        c.frames = vec![(0, 0), (1, (1 << 24) - 1), (2, (1 << 24) + 5)];
    });
    with("merged durations past MAX_DURATION", &|c| {
        c.frames = vec![
            (0, 0),
            (1, 10),
            (1, 10_000_010),
            (1, 20_000_010),
            (2, 20_000_020),
        ];
    });
    with("merged durations past MAX_DURATION, lossy", &|c| {
        c.frames = vec![
            (0, 0),
            (1, 10),
            (1, 10_000_010),
            (1, 20_000_010),
            (1, 30_000_010),
        ];
        c.configs = vec![CONFIGS[2].to_owned()];
    });
    with("merged durations past MAX_DURATION at the end", &|c| {
        c.frames = vec![(0, 0), (0, 16_000_000)];
        c.null_frame = Some(33_000_000);
    });
    with("int wrap-around", &|c| {
        c.frames = vec![(0, i32::MAX - 150), (1, i32::MAX - 50), (2, i32::MIN + 50)];
        c.null_frame = Some(i32::MIN + 150);
    });
    with("negative timestamps", &|c| {
        c.frames = vec![(0, -300), (1, -200), (2, -100)]
    });
    with("no null frame", &|c| c.null_frame = None);
    with("no null frame, one frame", &|c| {
        c.frames.truncate(1);
        c.null_frame = None;
    });
    with("no null frame, repeated", &|c| {
        c.frames = vec![(0, 0), (1, 70), (1, 140), (2, 230)];
        c.null_frame = None;
    });
    with("null frame first", &|c| {
        c.extra = "addnull 600\naddnull 700\n".to_owned()
    });
    with("null frame before frames", &|c| {
        c.frames.insert(0, (0, 0));
        c.null_frame = Some(1000);
        c.extra = "addnull 1001\n".to_owned();
    });
    with("no frames", &|c| {
        c.frames.clear();
        c.null_frame = None;
    });
    with("only a null frame", &|c| c.frames.clear());
    with("invalid config", &|c| {
        c.configs = vec![
            CONFIGS[1].to_owned(),
            "config quality=101".to_owned(),
            CONFIGS[2].to_owned(),
        ];
    });
    with("invalid config first", &|c| {
        c.configs = vec!["config method=7".to_owned()]
    });
    for (fail_at, label) in [
        (0, "counting hook"),
        (1, "hook fails at 1"),
        (5, "hook fails at 5"),
    ] {
        with(label, &|c| c.hook = fail_at);
        with(&format!("{label}, lossy"), &|c| {
            c.hook = fail_at;
            c.configs = vec![CONFIGS[2].to_owned()];
        });
        with(&format!("{label}, mixed"), &|c| {
            c.hook = fail_at;
            c.options = "options 0 0 0 1 0 0 0".to_owned();
        });
    }
    with("hook, single sub-rectangle frame", &|c| {
        c.hook = 0;
        c.frames.truncate(1);
    });
    with("frame of the wrong size", &|c| {
        c.extra = format!("add 900 31 20 1 {}\n", "MISSING");
    });
    with("canvas 0 x 0", &|c| {
        c.width = 0;
        c.height = 0;
        c.frames.clear();
    });
    with("canvas 0 x 4", &|c| {
        c.width = 0;
        c.height = 4;
        c.frames.clear();
    });
    with("canvas 65536 x 65536", &|c| {
        c.width = 65_536;
        c.height = 65_536;
        c.frames.clear();
    });
    let extra_cases = cases.clone();
    // The wrong-size frame needs a file of its size: point the line at one.
    let directory = work_dir("wp15_anim_errors_files");
    let wrong = directory.join("wrong.rgba");
    std::fs::write(&wrong, frame(31, 20, 0, Background::Opaque)).unwrap();
    let mut cases = extra_cases;
    for case in &mut cases {
        case.extra = case.extra.replace("MISSING", &wrong.display().to_string());
    }
    run_cases(&oracle, "wp15_anim_errors", &cases);

    // Version checks, the options defaults and an encoder that was never
    // created.
    let script = "optionsinit 265\noptionsinit 512\noptionsinit 264\nnewversion 512 4 4\n\
                  newversion 265 4 4\nnew -4 4\nnew 4 -4\nnewversion 512 4 4\naddnull 0\nassemble -\n\
                  getchunk ICCP\nsetchunk ICCP 1 hex:01\ndelchunk ICCP\n";
    let c = oracle
        .probe("wp15", "anim_enc")
        .run::<&str>(&[], script.as_bytes());
    c.success();
    compare_anim(
        "wp15_anim_versions",
        script,
        &String::from_utf8(c.stdout).unwrap(),
        &anim_report(script),
    );
}

#[test]
fn single_frames_and_chunks_match_the_c_encoder() {
    let Some(oracle) = Oracle::locate("single_frames_and_chunks_match_the_c_encoder") else {
        return;
    };
    let mut cases = Vec::new();
    // A single frame that covers the canvas, and one that is a sub-rectangle
    // (C's OptimizeSingleFrame decodes it onto the canvas and re-encodes it),
    // for every configuration, with and without mixed compression.
    for (c, config) in CONFIGS.iter().enumerate() {
        for mixed in [0, 1] {
            for background in [Background::Opaque, Background::Clear, Background::Noise] {
                let mut case = Case::new(
                    &format!("single {config} mixed {mixed} {background:?}"),
                    [33, 40, 17][c % 3],
                    [21, 24, 9][c % 3],
                    background,
                    1,
                );
                case.options = format!("options 0 0 0 {mixed} 0 0 0");
                case.configs = vec![(*config).to_owned()];
                cases.push(case);
            }
        }
    }
    // A 1 x 1 canvas and frames whose difference is a single pixel.
    let mut tiny = Case::new("1x1", 1, 1, Background::Opaque, 3);
    tiny.configs = vec![CONFIGS[1].to_owned(), CONFIGS[2].to_owned()];
    cases.push(tiny);
    // Metadata chunks set, replaced, read and deleted around the frames.
    for (copy, extra) in [
        (
            0,
            "setchunk ICCP 0 hex:0102030405\nsetchunk EXIF 1 hex:45786966\nsetchunk XMP_ 0 hex:3c78\n\
             getchunk ICCP\ngetchunk EXIF\ngetchunk XMP_\ngetchunk abcd\n",
        ),
        (
            1,
            "setchunk abcd 1 hex:\nsetchunk ICCP 1 hex:\ngetchunk ICCP\ngetchunk abcd\n\
             setchunk efgh 1 hex:01\ndelchunk abcd\ndelchunk ICCP\ndelchunk VP8_\nsetchunk ANMF 1 hex:00\n",
        ),
        (
            1,
            "setchunk EXIF 0 hex:\nsetchunk XMP_ 1 hex:3c3e\nsetchunk ICCP 1 hex:ff\nsetchunk ICCP 0 hex:fe\n",
        ),
    ] {
        let mut case = Case::new(&format!("chunks {copy}"), 26, 18, Background::Half, 3);
        case.extra = extra.to_owned();
        cases.push(case.clone());
        case.frames.truncate(1);
        case.name.push_str(" single");
        cases.push(case);
    }
    run_cases(&oracle, "wp15_anim_single", &cases);
}
