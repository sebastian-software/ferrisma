//! WP15 differential tests of the mux (`src/mux/muxedit.rs`, `muxinternal.rs`,
//! `muxread.rs`) against the C oracle.
//!
//! The probe `tools/oracle/probes/wp15/mux.c` runs a script of mux commands
//! against the C library; `wp15_support::mux_report` runs the same script
//! through the translation and must print the same lines: every return code,
//! every output (with the probe's marked output values where C leaves them),
//! and the size, `NULL` flag and digest of every byte string, with the full
//! bytes for chosen cases. The inputs are the pinned test vectors, WP14's
//! hand-assembled animations and `img2webp` animations, their truncations and
//! bit flips, and seeded edit sequences from scratch over generated
//! bitstreams, WebP files and chunk payloads.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp14_support/mod.rs"]
mod wp14_support;
#[path = "wp15_support/mod.rs"]
mod wp15_support;
#[path = "wp8_support/mod.rs"]
mod wp8_support;

use std::fmt::Write as _;
use std::path::Path;

use oracle::Oracle;
use webp::enc::picture_enc::{webp_encode_lossless_rgba, webp_encode_rgba};
use wp8_support::{Rng, chunk, digest, riff, top_level_chunks, truncation_points, vp8x_payload};
use wp14_support::{
    Alpha, AnimSpec, Codec, FrameSpec, build, img2webp_animations, pattern, synthetic_animations,
    work_dir,
};
use wp15_support::{compare, file_token, mux_report};

/// Runs `script` through the C probe and the translation and compares them.
fn check(oracle: &Oracle, test: &str, script: &str) -> usize {
    let output = oracle
        .probe("wp15", "mux")
        .run::<&str>(&[], script.as_bytes());
    output.success();
    let c = String::from_utf8(output.stdout).unwrap();
    let rust = mux_report(script);
    let context: String = script.lines().take(3).collect::<Vec<_>>().join(" | ");
    compare(test, &context, &c, &rust);
    script.lines().count()
}

/// Runs the scripts in batches of commands, so that one probe process runs
/// several of them; each batch starts from a fresh mux.
fn check_all(oracle: &Oracle, test: &str, scripts: &[String]) -> usize {
    let mut commands = 0;
    for batch in scripts.chunks(16) {
        commands += check(oracle, test, &batch.concat());
    }
    commands
}

#[test]
fn version_matches_the_c_probe() {
    let Some(oracle) = Oracle::locate("version_matches_the_c_probe") else {
        return;
    };
    check(&oracle, "version", "version\n");
}

/// The script for one WebP file: created with and without copying, queried,
/// assembled (a round trip) and edited.
fn file_script(token: &str, with_hex: bool) -> String {
    let hex = i32::from(with_hex);
    format!(
        "create 0 {token}\nstate\nassemble {hex}\nstate\ncreate 1 {token}\ngetframe 1 {hex}\n\
         getframe 0 0\nsetcanvas 0 0\nassemble 0\nstate\ndelframe 1\nstate\nassemble 0\n"
    )
}

#[test]
fn test_vectors_match_the_c_mux() {
    let Some(oracle) = Oracle::locate("test_vectors_match_the_c_mux") else {
        return;
    };
    let mut scripts = Vec::new();
    let mut round_trips = 0;
    for path in oracle::test_vectors() {
        let bytes = std::fs::read(&path).unwrap();
        let token = format!("file:{}", path.display());
        scripts.push(file_script(&token, bytes.len() <= 2048));
        // The round trip of an unedited mux.
        let report = mux_report(&format!("create 0 {token}\nassemble 0\n"));
        if report.contains(&format!(
            "assemble.data={},0,{:016x}",
            bytes.len(),
            digest(&bytes)
        )) {
            round_trips += 1;
        }
    }
    let commands = check_all(&oracle, "test_vectors", &scripts);
    eprintln!(
        "test_vectors_match_the_c_mux: {} vectors, {commands} commands, {round_trips} exact round trips",
        scripts.len()
    );
    assert!(round_trips > 0);
}

#[test]
fn animations_match_the_c_mux() {
    let Some(oracle) = Oracle::locate("animations_match_the_c_mux") else {
        return;
    };
    let directory = work_dir("wp15_mux_animations");
    let mut files: Vec<(String, std::path::PathBuf)> =
        wp14_support::write_all(&directory, &synthetic_animations());
    files.extend(img2webp_animations(&oracle, &directory));
    let mut scripts = Vec::new();
    for (index, (_, path)) in files.iter().enumerate() {
        let token = format!("file:{}", path.display());
        let mut script = file_script(&token, index % 16 == 0);
        // Frame and chunk edits on the parsed animation.
        write!(
            script,
            "create 0 {token}\ngetframe 2 0\ndelframe 0\nsetanim 4278190080 7\n\
             setchunk ICCP 1 hex:0102\nsetchunk abcd 0 hex:\nsetcanvas 5000 5000\nassemble 0\n\
             state\ndelchunk abcd\ndelchunk EXIF\nsetcanvas 0 0\nassemble {}\nstate\n",
            i32::from(index % 32 == 0)
        )
        .unwrap();
        scripts.push(script);
    }
    let commands = check_all(&oracle, "animations", &scripts);
    eprintln!(
        "animations_match_the_c_mux: {} files, {commands} commands",
        files.len()
    );
}

#[test]
fn truncated_and_corrupt_files_match_the_c_mux() {
    let Some(oracle) = Oracle::locate("truncated_and_corrupt_files_match_the_c_mux") else {
        return;
    };
    let directory = work_dir("wp15_mux_truncated");
    let mut sources: Vec<Vec<u8>> = Vec::new();
    for path in oracle::test_vectors() {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        if [
            "lossy_alpha1.webp",
            "lossless1.webp",
            "test.webp",
            "lossy_extreme_probabilities.webp",
        ]
        .contains(&name.as_str())
        {
            sources.push(std::fs::read(&path).unwrap());
        }
    }
    for (_, bytes) in synthetic_animations().into_iter().step_by(23) {
        sources.push(bytes);
    }
    let mut scripts = Vec::new();
    let mut count = 0;
    for (index, bytes) in sources.iter().enumerate() {
        for point in truncation_points(bytes) {
            let token = file_token(&directory, &format!("cut{count}.webp"), &bytes[..point]);
            count += 1;
            scripts.push(format!(
                "create 0 {token}\nstate\nassemble 0\ncreate 1 {token}\nassemble 0\n"
            ));
        }
        let mut rng = Rng(0x15 + index as u64);
        for _ in 0..24 {
            let mut flipped = bytes.clone();
            let position = rng.below(flipped.len() as u64) as usize;
            flipped[position] ^= 1 << rng.below(8);
            let token = file_token(&directory, &format!("flip{count}.webp"), &flipped);
            count += 1;
            scripts.push(format!("create 1 {token}\nstate\nassemble 0\n"));
        }
    }
    // Extended size fields: a RIFF size past the end, chunk sizes over the
    // remaining bytes, over the RIFF size and over MAX_CHUNK_PAYLOAD.
    let small = std::fs::read(oracle::test_data().join("lossless1.webp")).unwrap();
    for riff_size in [0u32, 4, 11, 12, 13, 0xffff_fff6, 0xffff_fff7, u32::MAX] {
        let mut file = small.clone();
        file[4..8].copy_from_slice(&riff_size.to_le_bytes());
        let token = file_token(&directory, &format!("riff{riff_size}.webp"), &file);
        scripts.push(format!("create 0 {token}\nstate\nassemble 0\n"));
    }
    for chunk_size in [0u32, 1, 7, 8, 0xffff_fff6, 0xffff_fff7, u32::MAX] {
        let mut file = small.clone();
        file[16..20].copy_from_slice(&chunk_size.to_le_bytes());
        let token = file_token(&directory, &format!("chunk{chunk_size}.webp"), &file);
        scripts.push(format!("create 0 {token}\nstate\nassemble 0\n"));
    }
    let commands = check_all(&oracle, "truncated", &scripts);
    eprintln!(
        "truncated_and_corrupt_files_match_the_c_mux: {} files, {commands} commands",
        scripts.len()
    );
}

//------------------------------------------------------------------------------
// Edit sequences from scratch

/// The data that the edit sequences use, as DATA tokens.
struct Pool {
    /// Raw VP8 and VP8L bitstreams and single-image WebP files.
    images: Vec<String>,
    /// Animated WebP files (an image of `WebPMuxSetImage` takes the first frame).
    animations: Vec<String>,
    /// Data that is no image.
    garbage: Vec<String>,
    /// Chunk payloads.
    payloads: Vec<String>,
}

/// The payload of the first chunk with `tag` of a WebP file.
fn payload(file: &[u8], tag: &[u8; 4]) -> Vec<u8> {
    top_level_chunks(file)
        .into_iter()
        .find(|(name, _)| name == tag)
        .map(|(_, bytes)| bytes)
        .unwrap_or_else(|| panic!("no {} chunk", String::from_utf8_lossy(tag)))
}

fn hex_token(bytes: &[u8]) -> String {
    format!("hex:{}", wp8_support::hex(bytes))
}

fn pool(directory: &Path) -> Pool {
    let mut images = Vec::new();
    let mut animations = Vec::new();
    let mut n = 0;
    let mut save = |bytes: &[u8]| {
        n += 1;
        file_token(directory, &format!("data{n}.bin"), bytes)
    };
    for (index, (width, height)) in [(1, 1), (5, 3), (16, 16), (33, 17), (64, 40)]
        .into_iter()
        .enumerate()
    {
        let seed = 7 + index as u64;
        let opaque = pattern(width, height, seed, Alpha::Opaque);
        let alpha = pattern(width, height, seed, Alpha::Gradient);
        let (w, h) = (width as i32, height as i32);
        let lossy = webp_encode_rgba(&opaque, w, h, w * 4, 70.0).unwrap();
        let lossy_alpha = webp_encode_rgba(&alpha, w, h, w * 4, 50.0).unwrap();
        let lossless = webp_encode_lossless_rgba(&alpha, w, h, w * 4).unwrap();
        let lossless_opaque = webp_encode_lossless_rgba(&opaque, w, h, w * 4).unwrap();
        images.push(save(&lossy));
        images.push(save(&lossy_alpha));
        images.push(save(&lossless));
        images.push(save(&lossless_opaque));
        images.push(save(&payload(&lossy, b"VP8 ")));
        images.push(save(&payload(&lossless, b"VP8L")));
        // ALPH next to VP8L (dropped by MuxImageFinalize) and a lone ALPH.
        let alph = payload(&lossy_alpha, b"ALPH");
        let mut with_alph = chunk(b"VP8X", &vp8x_payload(0x10, width, height));
        with_alph.extend_from_slice(&chunk(b"ALPH", &alph));
        with_alph.extend_from_slice(&chunk(b"VP8L", &payload(&lossless, b"VP8L")));
        images.push(save(&riff(&with_alph)));
    }
    for (index, frames) in [1usize, 2, 4].into_iter().enumerate() {
        let specs: Vec<FrameSpec> = (0..frames)
            .map(|i| {
                let mut frame = FrameSpec::new(2 * i as u32, 2 * index as u32, 9 + i as u32, 7);
                frame.codec = if i % 2 == 0 {
                    Codec::Lossless
                } else {
                    Codec::Lossy(60.0)
                };
                frame
            })
            .collect();
        let mut spec = AnimSpec::new(20, 16, specs);
        spec.metadata = index == 1;
        animations.push(save(&build(&spec)));
    }
    let mut garbage = vec![
        "hex:".to_owned(),
        hex_token(b"RIFF"),
        hex_token(b"RIFF\x04\0\0\0WEBP"),
        hex_token(b"\x2f\0\0\0\0"),
        hex_token(b"\x9d\x01\x2a\x01\x00"),
    ];
    let lossy = std::fs::read(oracle::test_data().join("test.webp")).unwrap();
    garbage.push(save(&lossy[..lossy.len() / 2]));
    garbage.push(save(&lossy[..40]));
    let mut rng = Rng(99);
    garbage.push(save(&(0..300).map(|_| rng.byte()).collect::<Vec<u8>>()));
    let payloads = [0usize, 1, 2, 3, 7, 8, 33, 100]
        .iter()
        .map(|&size| {
            hex_token(
                &(0..size)
                    .map(|i| (i * 37 + size) as u8)
                    .collect::<Vec<u8>>(),
            )
        })
        .collect();
    Pool {
        images,
        animations,
        garbage,
        payloads,
    }
}

const FOURCCS: [&str; 14] = [
    "ICCP", "EXIF", "XMP_", "VP8X", "ANIM", "ANMF", "ALPH", "VP8_", "VP8L", "abcd", "zzzz", "UNKN",
    "XMP ", "iccp",
];

fn pick<'p>(rng: &mut Rng, items: &'p [String]) -> &'p str {
    &items[rng.below(items.len() as u64) as usize]
}

fn pick_value(rng: &mut Rng, values: &[i64]) -> i64 {
    values[rng.below(values.len() as u64) as usize]
}

/// One seeded edit sequence.
fn edit_sequence(rng: &mut Rng, pool: &Pool) -> String {
    let mut script = String::from("new\n");
    let steps = 6 + rng.below(30);
    for _ in 0..steps {
        let copy = rng.below(2);
        let line = match rng.below(17) {
            0 | 1 => {
                let fourcc = FOURCCS[rng.below(FOURCCS.len() as u64) as usize].replace(' ', "_");
                format!("setchunk {fourcc} {copy} {}", pick(rng, &pool.payloads))
            }
            2 => {
                let fourcc = FOURCCS[rng.below(FOURCCS.len() as u64) as usize].replace(' ', "_");
                format!("getchunk {fourcc}")
            }
            3 => {
                let fourcc = FOURCCS[rng.below(FOURCCS.len() as u64) as usize].replace(' ', "_");
                format!("delchunk {fourcc}")
            }
            4 => {
                let data = match rng.below(6) {
                    0 => pick(rng, &pool.animations),
                    1 => pick(rng, &pool.garbage),
                    _ => pick(rng, &pool.images),
                };
                format!("setimage {copy} {data}")
            }
            5..=8 => {
                let data = match rng.below(8) {
                    0 => pick(rng, &pool.animations),
                    1 => pick(rng, &pool.garbage),
                    _ => pick(rng, &pool.images),
                };
                let x = pick_value(rng, &[0, 1, 2, 3, 10, 31, -1, -2, 16_777_214, 16_777_216]);
                let y = pick_value(rng, &[0, 1, 4, 7, 12, -3, 16_777_215]);
                let duration = pick_value(rng, &[0, 1, 40, 1000, 16_777_215, 16_777_216, -1]);
                let id = if rng.below(10) == 0 {
                    pick_value(rng, &[0, 6, 9, 99])
                } else {
                    3
                };
                let dispose = pick_value(rng, &[0, 0, 1, 1, 2]);
                let blend = pick_value(rng, &[0, 0, 1, 1, 2]);
                format!("pushframe {copy} {x} {y} {duration} {id} {dispose} {blend} {data}")
            }
            9 => format!("getframe {} {}", rng.below(5), rng.below(2)),
            10 => format!("delframe {}", rng.below(5)),
            11 => {
                let bgcolor = pick_value(rng, &[0, 0xffff_ffff, 0x1234_5678]);
                let loops = pick_value(rng, &[0, 1, 65535, 65536, -1]);
                format!("setanim {bgcolor} {loops}")
            }
            12 => {
                // Areas from 2^31 to 2^32 overflow C's int product (excluded,
                // docs/status/wp15.md).
                let (width, height) = [
                    (0, 0),
                    (0, 5),
                    (5, 0),
                    (20, 16),
                    (64, 64),
                    (1, 1),
                    (-1, 4),
                    (16_777_216, 1),
                    (16_777_217, 1),
                    (65_536, 65_536),
                    (65_536, 65_537),
                    (100_000, 100_000),
                ][rng.below(12) as usize];
                format!("setcanvas {width} {height}")
            }
            13 => "getcanvas\nfeatures\ngetanim".to_owned(),
            14 => format!(
                "numchunks {}",
                pick_value(rng, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 99])
            ),
            15 => format!("assemble {}", rng.below(2)),
            _ => "state".to_owned(),
        };
        script.push_str(&line);
        script.push('\n');
    }
    script.push_str("state\nassemble 1\nstate\n");
    script
}

#[test]
fn edit_sequences_match_the_c_mux() {
    let Some(oracle) = Oracle::locate("edit_sequences_match_the_c_mux") else {
        return;
    };
    let directory = work_dir("wp15_mux_edits");
    let pool = pool(&directory);
    let mut scripts = Vec::new();
    for seed in 0..400 {
        let mut rng = Rng(0x1500 + seed);
        scripts.push(edit_sequence(&mut rng, &pool));
    }
    // Directed cases: chunk order and copying, empty chunks with and without
    // copying (C's NULL bytes of an empty copy clear the VP8X flags), a second
    // unknown chunk (C's ChunkSetHead refuses it), canvas sizes that the frames
    // exceed, the single-frame cleanup, and images replaced by frames.
    let image = &pool.images[2];
    let raw = &pool.images[5];
    let animation = &pool.animations[2];
    for directed in [
        format!("new\nsetimage 0 {image}\nsetchunk ICCP 1 hex:\nassemble 0\nstate\n"),
        format!("new\nsetimage 0 {image}\nsetchunk ICCP 0 hex:\nassemble 1\nstate\n"),
        format!(
            "new\nsetimage 1 {image}\nsetchunk EXIF 1 hex:\nsetchunk XMP_ 0 hex:00\nassemble 1\n"
        ),
        format!(
            "new\nsetimage 1 {raw}\nsetchunk abcd 1 hex:01\nsetchunk efgh 1 hex:02\nsetchunk abcd 0 hex:03\nassemble 1\nstate\n"
        ),
        format!(
            "new\nsetchunk XMP_ 1 hex:3c3e\nsetchunk ICCP 1 hex:0102\nsetchunk EXIF 0 hex:4500\nsetimage 0 {image}\nassemble 1\n"
        ),
        format!("new\npushframe 1 0 0 100 3 0 0 {raw}\nassemble 1\nstate\n"),
        format!(
            "new\npushframe 1 4 4 100 3 1 1 {raw}\nassemble 1\nstate\nsetcanvas 4 4\nassemble 0\n"
        ),
        format!(
            "new\npushframe 1 0 0 100 3 0 0 {raw}\npushframe 1 6 2 50 3 1 0 {image}\nsetanim 0 3\nassemble 1\nstate\nsetcanvas 70 50\nassemble 1\nsetcanvas 8 8\nassemble 0\n"
        ),
        format!("new\nsetimage 1 {image}\npushframe 1 0 0 100 3 0 0 {raw}\nstate\n"),
        format!("new\npushframe 1 0 0 100 3 0 0 {raw}\nsetimage 1 {image}\nstate\nassemble 1\n"),
        format!("new\nsetimage 0 {animation}\nstate\nassemble 1\n"),
        format!(
            "new\nsetchunk VP8X 0 hex:00000000000000000000\nsetimage 1 {image}\nassemble 1\nstate\n"
        ),
        format!("new\nsetchunk ANIM 0 hex:00000000\nsetimage 1 {image}\nassemble 1\nstate\n"),
        "new\nassemble 0\nstate\nsetcanvas 3 3\nassemble 0\n".to_owned(),
        format!(
            "newversion 512\nnewversion 265\nsetimage 1 {image}\nassemble 0\ncreateversion 512 0 {image}\ncreateversion 264 1 {image}\nstate\n"
        ),
    ] {
        scripts.push(directed);
    }
    let commands = check_all(&oracle, "edits", &scripts);
    eprintln!(
        "edit_sequences_match_the_c_mux: {} sequences, {commands} commands",
        scripts.len()
    );
}
