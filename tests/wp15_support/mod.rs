//! Shared helpers of the WP15 differential tests: the scripts of the probes
//! `tools/oracle/probes/wp15/mux.c` and `anim_enc.c`, and reports that
//! reproduce their output with the Rust translation.
//!
//! Include it after `wp8_support` and `oracle`:
//! `#[path = "wp8_support/mod.rs"] mod wp8_support;`,
//! `#[path = "support/oracle.rs"] mod oracle;` and
//! `#[path = "wp15_support/mod.rs"] mod wp15_support;`.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use std::borrow::Cow;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use webp::enc::picture_enc::{webp_encode_lossless_rgba, webp_encode_rgba};
use webp::mux::anim_encode::{
    WebPAnimEncoder, webp_anim_encoder_add, webp_anim_encoder_assemble,
    webp_anim_encoder_delete_chunk, webp_anim_encoder_get_chunk, webp_anim_encoder_get_error,
    webp_anim_encoder_new_internal, webp_anim_encoder_set_chunk,
};
use webp::mux::muxedit::{
    webp_mux_assemble, webp_mux_delete_chunk, webp_mux_delete_frame, webp_mux_push_frame,
    webp_mux_set_animation_params, webp_mux_set_canvas_size, webp_mux_set_chunk,
    webp_mux_set_image, webp_new_internal,
};
use webp::mux::muxi::{CHUNK_INDEX, WebPMux, data_is_null};
use webp::mux::muxinternal::{chunk_get_index_from_four_cc, webp_get_mux_version};
use webp::mux::muxread::{
    webp_mux_create_internal, webp_mux_get_animation_params, webp_mux_get_canvas_size,
    webp_mux_get_chunk, webp_mux_get_features, webp_mux_get_frame, webp_mux_num_chunks,
};
use webp::webp::encode::{WebPConfig, WebPPicture, webp_config_init, webp_picture_init};
use webp::webp::mux::{
    WEBP_MUX_ABI_VERSION, WebPAnimEncoderOptions, WebPChunkId, WebPMuxAnimParams, WebPMuxError,
    WebPMuxFrameInfo, webp_anim_encoder_options_init,
};
use webp::webp::mux_types::{WebPData, WebPMuxAnimBlend, WebPMuxAnimDispose};

use crate::oracle::Oracle;
use crate::wp8_support::{
    Rng, chunk, digest, hex, riff, top_level_chunks, truncation_points, vp8x_payload,
};
use crate::wp14_support::{Alpha, AnimSpec, Codec, FrameSpec, build, pattern};

/// The value of the probe's marked fields as C prints them: `memset` with
/// 0x5a, read as `int` or `uint32_t`.
const MARK: i32 = 0x5a5a_5a5a;

/// The C code of a result.
pub fn code<T>(result: &Result<T, WebPMuxError>) -> i32 {
    WebPMuxError::of(result).0
}

/// A FOURCC token of the probes: four characters with `_` for a space.
pub fn fourcc(token: &str) -> [u8; 4] {
    let bytes: [u8; 4] = token.as_bytes().try_into().expect("a four-character tag");
    bytes.map(|byte| if byte == b'_' { b' ' } else { byte })
}

/// The bytes of a DATA token, which `null` and `huge` (ABI-only) do not have.
fn load(token: &str) -> Vec<u8> {
    if let Some(path) = token.strip_prefix("file:") {
        std::fs::read(path).unwrap_or_else(|error| panic!("cannot read {path}: {error}"))
    } else if let Some(digits) = token.strip_prefix("hex:") {
        (0..digits.len() / 2)
            .map(|i| u8::from_str_radix(&digits[2 * i..2 * i + 2], 16).unwrap())
            .collect()
    } else {
        panic!("the Rust report has no data for {token}")
    }
}

/// The DATA tokens of a script's lines, at their positions in the lines.
fn data_tokens(line: &[&str]) -> Vec<usize> {
    match line.first().copied() {
        Some(
            "create" | "setimage" | "setchunk" | "pushframe" | "createversion" | "setchunkenc",
        ) => {
            vec![line.len() - 1]
        }
        _ => Vec::new(),
    }
}

/// `PrintData`: size, `NULL` and digest, and the hex when asked.
fn print_data(out: &mut String, key: &str, bytes: &[u8], null: bool, with_hex: bool) {
    writeln!(
        out,
        "{key}.data={},{},{:016x}",
        bytes.len(),
        i32::from(null),
        digest(bytes)
    )
    .unwrap();
    if with_hex {
        writeln!(out, "{key}.hex={}", hex(bytes)).unwrap();
    }
}

/// `GetChunk` of the mux probe for a chunk getter.
fn print_get_chunk(out: &mut String, token: &str, result: Result<&WebPData<'_>, WebPMuxError>) {
    let tag = fourcc(token);
    writeln!(out, "getchunk.{token}={}", code(&result)).unwrap();
    match result {
        Ok(data) => print_data(
            out,
            "getchunk",
            &data.bytes,
            data_is_null(data),
            data.bytes.len() <= 64,
        ),
        // C's MuxGet resets the output for a known, non-image chunk type.
        Err(WebPMuxError::WEBP_MUX_NOT_FOUND)
            if chunk_get_index_from_four_cc(&tag) != CHUNK_INDEX::IDX_UNKNOWN =>
        {
            writeln!(out, "getchunk.{token}.reset=1,0").unwrap();
        }
        Err(_) => writeln!(out, "getchunk.{token}.untouched=1").unwrap(),
    }
}

fn print_get_frame(out: &mut String, mux: &WebPMux<'_>, nth: u32, with_hex: bool) {
    let result = webp_mux_get_frame(mux, nth);
    writeln!(out, "getframe.{nth}={}", code(&result)).unwrap();
    match result {
        Ok(frame) => {
            writeln!(
                out,
                "getframe.{nth}.info={},{},{},{},{},{},{}",
                frame.x_offset,
                frame.y_offset,
                frame.duration,
                frame.id.0,
                frame.dispose_method.0,
                frame.blend_method.0,
                MARK.cast_unsigned()
            )
            .unwrap();
            print_data(out, "getframe", &frame.bitstream.bytes, false, with_hex);
        }
        Err(_) => {
            writeln!(
                out,
                "getframe.{nth}.info={MARK},{MARK},{MARK},{MARK},{MARK},{MARK},{}",
                MARK.cast_unsigned()
            )
            .unwrap();
            writeln!(out, "getframe.{nth}.untouched=1").unwrap();
        }
    }
}

fn print_num_chunks(out: &mut String, mux: &WebPMux<'_>, id: i32) {
    let result = webp_mux_num_chunks(mux, WebPChunkId(id.cast_unsigned()));
    writeln!(
        out,
        "numchunks.{id}={},{}",
        code(&result),
        result.unwrap_or(-77)
    )
    .unwrap();
}

fn print_get_anim(out: &mut String, mux: &WebPMux<'_>) {
    let result = webp_mux_get_animation_params(mux);
    let (bgcolor, loop_count) = match result {
        Ok(params) => (params.bgcolor, params.loop_count),
        Err(_) => (MARK.cast_unsigned(), -77),
    };
    writeln!(out, "getanim={},{bgcolor},{loop_count}", code(&result)).unwrap();
}

fn print_get_canvas(out: &mut String, mux: &WebPMux<'_>) {
    let result = webp_mux_get_canvas_size(mux);
    let (width, height) = result.unwrap_or((-77, -77));
    writeln!(out, "getcanvas={},{width},{height}", code(&result)).unwrap();
}

fn print_features(out: &mut String, mux: &WebPMux<'_>) {
    let result = webp_mux_get_features(mux);
    let flags = result.unwrap_or(MARK.cast_unsigned());
    writeln!(out, "features={},{flags}", code(&result)).unwrap();
}

fn print_assemble(out: &mut String, mux: &mut WebPMux<'_>, with_hex: bool) {
    let result = webp_mux_assemble(mux);
    writeln!(out, "assemble={}", code(&result)).unwrap();
    match result {
        Ok(data) => print_data(out, "assemble", &data.bytes, false, with_hex),
        Err(_) => print_data(out, "assemble", &[], true, false),
    }
}

/// The tags that `state` queries.
const STATE_TAGS: [&str; 11] = [
    "VP8X", "ICCP", "ANIM", "EXIF", "XMP_", "ANMF", "ALPH", "VP8_", "VP8L", "abcd", "zzzz",
];

fn print_state(out: &mut String, mux: &WebPMux<'_>) {
    print_features(out, mux);
    print_get_canvas(out, mux);
    for id in 0..=11 {
        print_num_chunks(out, mux, id);
    }
    print_get_anim(out, mux);
    for token in STATE_TAGS {
        print_get_chunk(out, token, webp_mux_get_chunk(mux, &fourcc(token)));
    }
    let images = webp_mux_num_chunks(mux, WebPChunkId::WEBP_CHUNK_IMAGE).unwrap_or(0);
    for nth in 0..=images.cast_unsigned() + 1 {
        print_get_frame(out, mux, nth, false);
    }
}

fn int(token: &str) -> i32 {
    token
        .parse()
        .unwrap_or_else(|_| panic!("not an integer: {token}"))
}

fn uint(token: &str) -> u32 {
    token
        .parse()
        .unwrap_or_else(|_| panic!("not an unsigned integer: {token}"))
}

/// What the probe prints for a command on a missing mux (after a failed
/// `create`): C's `NULL` checks, which have no counterpart in the
/// translation's API, return `WEBP_MUX_INVALID_ARGUMENT` and write no output,
/// except `WebPMuxAssemble`, which zeroes its output first.
fn null_mux_report(out: &mut String, line: &[&str]) {
    let invalid = WebPMuxError::WEBP_MUX_INVALID_ARGUMENT.0;
    let unmarked = MARK.cast_unsigned();
    let frame = |out: &mut String, nth: u32| {
        writeln!(out, "getframe.{nth}={invalid}").unwrap();
        writeln!(
            out,
            "getframe.{nth}.info={MARK},{MARK},{MARK},{MARK},{MARK},{MARK},{unmarked}"
        )
        .unwrap();
        writeln!(out, "getframe.{nth}.untouched=1").unwrap();
    };
    let chunk = |out: &mut String, token: &str| {
        writeln!(out, "getchunk.{token}={invalid}").unwrap();
        writeln!(out, "getchunk.{token}.untouched=1").unwrap();
    };
    match line[0] {
        "setchunk" | "delchunk" | "setimage" | "pushframe" | "delframe" | "setanim"
        | "setcanvas" => writeln!(out, "{}={invalid}", line[0]).unwrap(),
        "getchunk" => chunk(out, line[1]),
        "getframe" => frame(out, uint(line[1])),
        "getanim" => writeln!(out, "getanim={invalid},{unmarked},-77").unwrap(),
        "getcanvas" => writeln!(out, "getcanvas={invalid},-77,-77").unwrap(),
        "features" => writeln!(out, "features={invalid},{unmarked}").unwrap(),
        "numchunks" => writeln!(out, "numchunks.{}={invalid},-77", line[1]).unwrap(),
        "assemble" => {
            writeln!(out, "assemble={invalid}").unwrap();
            print_data(out, "assemble", &[], true, false);
        }
        "state" => {
            writeln!(out, "features={invalid},{unmarked}").unwrap();
            writeln!(out, "getcanvas={invalid},-77,-77").unwrap();
            for id in 0..=11 {
                writeln!(out, "numchunks.{id}={invalid},-77").unwrap();
            }
            writeln!(out, "getanim={invalid},{unmarked},-77").unwrap();
            for token in STATE_TAGS {
                chunk(out, token);
            }
            frame(out, 0);
            frame(out, 1);
        }
        other => panic!("the Rust report has no command {other}"),
    }
}

/// The output of the mux probe for `script`, from the Rust translation. The
/// script must not use `nulls` or the `null` and `huge` data, which exist only
/// at the C ABI; commands on a missing mux print C's `NULL` results.
pub fn mux_report(script: &str) -> String {
    let lines: Vec<Vec<&str>> = script
        .lines()
        .map(|line| line.split_whitespace().collect())
        .filter(|tokens: &Vec<&str>| !tokens.is_empty())
        .collect();
    // The data of every line, loaded first, so that a mux may borrow it until
    // the end, as the probe keeps its buffers.
    let arena: Vec<Vec<Vec<u8>>> = lines
        .iter()
        .map(|line| {
            data_tokens(line)
                .into_iter()
                .map(|i| load(line[i]))
                .collect()
        })
        .collect();
    let mut out = String::new();
    let mut mux: Option<WebPMux<'_>> = None;
    for (line, data) in lines.iter().zip(&arena) {
        let data = data.first().map(|bytes| WebPData {
            bytes: Cow::Borrowed(bytes.as_slice()),
        });
        let uses_mux = !matches!(
            line[0],
            "version" | "new" | "newversion" | "create" | "createversion" | "delete"
        );
        if uses_mux && mux.is_none() {
            null_mux_report(&mut out, line);
            continue;
        }
        match line[0] {
            "version" => writeln!(out, "version={}", webp_get_mux_version()).unwrap(),
            "new" => {
                mux = webp_new_internal(WEBP_MUX_ABI_VERSION);
                writeln!(out, "new={}", i32::from(mux.is_some())).unwrap();
            }
            "newversion" => {
                mux = webp_new_internal(int(line[1]));
                writeln!(out, "new={}", i32::from(mux.is_some())).unwrap();
            }
            "create" | "createversion" => {
                let (version, copy) = if line[0] == "create" {
                    (WEBP_MUX_ABI_VERSION, int(line[1]))
                } else {
                    (int(line[1]), int(line[2]))
                };
                mux = webp_mux_create_internal(data.as_ref().unwrap(), copy, version);
                writeln!(out, "create={}", i32::from(mux.is_some())).unwrap();
            }
            "delete" => {
                mux = None;
                writeln!(out, "delete=1").unwrap();
            }
            "setchunk" => {
                let mux = mux.as_mut().unwrap();
                let result =
                    webp_mux_set_chunk(mux, &fourcc(line[1]), data.as_ref().unwrap(), int(line[2]));
                writeln!(out, "setchunk={}", code(&result)).unwrap();
            }
            "getchunk" => {
                let mux = mux.as_ref().unwrap();
                print_get_chunk(&mut out, line[1], webp_mux_get_chunk(mux, &fourcc(line[1])));
            }
            "delchunk" => {
                let result = webp_mux_delete_chunk(mux.as_mut().unwrap(), &fourcc(line[1]));
                writeln!(out, "delchunk={}", code(&result)).unwrap();
            }
            "setimage" => {
                let result =
                    webp_mux_set_image(mux.as_mut().unwrap(), data.as_ref().unwrap(), int(line[1]));
                writeln!(out, "setimage={}", code(&result)).unwrap();
            }
            "pushframe" => {
                let frame = WebPMuxFrameInfo {
                    bitstream: data.clone().unwrap(),
                    x_offset: int(line[2]),
                    y_offset: int(line[3]),
                    duration: int(line[4]),
                    id: WebPChunkId(int(line[5]).cast_unsigned()),
                    dispose_method: WebPMuxAnimDispose(int(line[6]).cast_unsigned()),
                    blend_method: WebPMuxAnimBlend(int(line[7]).cast_unsigned()),
                    pad: [0],
                };
                let result = webp_mux_push_frame(mux.as_mut().unwrap(), &frame, int(line[1]));
                writeln!(out, "pushframe={}", code(&result)).unwrap();
            }
            "getframe" => {
                print_get_frame(
                    &mut out,
                    mux.as_ref().unwrap(),
                    uint(line[1]),
                    int(line[2]) != 0,
                );
            }
            "delframe" => {
                let result = webp_mux_delete_frame(mux.as_mut().unwrap(), uint(line[1]));
                writeln!(out, "delframe={}", code(&result)).unwrap();
            }
            "setanim" => {
                let params = WebPMuxAnimParams {
                    bgcolor: uint(line[1]),
                    loop_count: int(line[2]),
                };
                let result = webp_mux_set_animation_params(mux.as_mut().unwrap(), &params);
                writeln!(out, "setanim={}", code(&result)).unwrap();
            }
            "getanim" => print_get_anim(&mut out, mux.as_ref().unwrap()),
            "setcanvas" => {
                let result =
                    webp_mux_set_canvas_size(mux.as_mut().unwrap(), int(line[1]), int(line[2]));
                writeln!(out, "setcanvas={}", code(&result)).unwrap();
            }
            "getcanvas" => print_get_canvas(&mut out, mux.as_ref().unwrap()),
            "features" => print_features(&mut out, mux.as_ref().unwrap()),
            "numchunks" => print_num_chunks(&mut out, mux.as_ref().unwrap(), int(line[1])),
            "assemble" => print_assemble(&mut out, mux.as_mut().unwrap(), int(line[1]) != 0),
            "state" => print_state(&mut out, mux.as_ref().unwrap()),
            other => panic!("the Rust report has no command {other}"),
        }
    }
    out
}

/// Compares a probe's output with the expected text line by line and panics
/// at the first difference, with the script's line for context.
pub fn compare(test: &str, context: &str, actual: &str, expected: &str) {
    let mut actual_lines = actual.lines();
    let mut expected_lines = expected.lines();
    let mut number = 0;
    loop {
        number += 1;
        match (actual_lines.next(), expected_lines.next()) {
            (None, None) => return,
            (a, e) => {
                if a != e {
                    panic!(
                        "{test}: output line {number} differs\n  C:    {a:?}\n  Rust: {e:?}\n  in: {context}"
                    );
                }
            }
        }
    }
}

/// Writes `bytes` to `directory/name` and returns the `file:` token.
pub fn file_token(directory: &Path, name: &str, bytes: &[u8]) -> String {
    let path = directory.join(name);
    std::fs::write(&path, bytes).unwrap();
    format!("file:{}", path.display())
}

//------------------------------------------------------------------------------
// The animation encoder probe

/// The keys whose values differ from C by a recorded deviation: C's encoder
/// calls the last frame's progress hook again in `WebPAnimEncoderAssemble`
/// (`docs/status/wp15.md`), the translation does not.
pub const ANIM_DEVIATION_KEYS: [&str; 1] = ["assemble.hook"];

/// Compares the animation encoder probe's output with the translation's, line
/// by line; on a key of [`ANIM_DEVIATION_KEYS`] the translation must print 0
/// and C may print any value. Returns the number of lines where C's differs.
pub fn compare_anim(test: &str, context: &str, c: &str, rust: &str) -> usize {
    let mut deviations = 0;
    let (c_lines, rust_lines): (Vec<&str>, Vec<&str>) =
        (c.lines().collect(), rust.lines().collect());
    for (number, (c_line, rust_line)) in c_lines.iter().zip(&rust_lines).enumerate() {
        if c_line == rust_line {
            continue;
        }
        let deviation = ANIM_DEVIATION_KEYS
            .iter()
            .any(|key| c_line.starts_with(&format!("{key}=")) && *rust_line == format!("{key}=0"));
        assert!(
            deviation,
            "{test}: output line {} differs\n  C:    {c_line:?}\n  Rust: {rust_line:?}\n  in: {context}",
            number + 1
        );
        deviations += 1;
    }
    assert_eq!(
        c_lines.len(),
        rust_lines.len(),
        "{test}: the outputs have different lengths\n  in: {context}"
    );
    deviations
}

/// The configuration of a `config` command, as the probe sets it.
fn parse_config(tokens: &[&str]) -> Option<WebPConfig> {
    if tokens == ["null"] {
        return None;
    }
    let mut config = WebPConfig::default();
    assert!(webp_config_init(&mut config));
    for token in tokens {
        let (key, value) = token.split_once('=').expect("KEY=VALUE");
        let value: f64 = value.parse().unwrap();
        #[expect(
            clippy::cast_possible_truncation,
            reason = "the probe's (int) and (float)"
        )]
        let (int, float) = (value as i32, value as f32);
        match key {
            "quality" => config.quality = float,
            "lossless" => config.lossless = int,
            "method" => config.method = int,
            "exact" => config.exact = int,
            "thread_level" => config.thread_level = int,
            "alpha_quality" => config.alpha_quality = int,
            "alpha_compression" => config.alpha_compression = int,
            "alpha_filtering" => config.alpha_filtering = int,
            "near_lossless" => config.near_lossless = int,
            "use_sharp_yuv" => config.use_sharp_yuv = int,
            "sns_strength" => config.sns_strength = int,
            "filter_strength" => config.filter_strength = int,
            "filter_sharpness" => config.filter_sharpness = int,
            "autofilter" => config.autofilter = int,
            "segments" => config.segments = int,
            "preprocessing" => config.preprocessing = int,
            "partitions" => config.partitions = int,
            "pass" => config.pass = int,
            "target_size" => config.target_size = int,
            "low_memory" => config.low_memory = int,
            other => panic!("unknown config key {other}"),
        }
    }
    Some(config)
}

fn print_enc_error(out: &mut String, key: &str, enc: Option<&WebPAnimEncoder<'_>>) {
    match enc {
        Some(enc) => writeln!(out, "{key}.error={}", webp_anim_encoder_get_error(enc)).unwrap(),
        None => writeln!(out, "{key}.error=(null)").unwrap(),
    }
}

/// The output of the animation encoder probe for `script`, from the
/// translation, with the assembled bytes written where the script asks. The
/// script must not use `nulls`. Commands on a missing encoder print C's `NULL`
/// results.
#[expect(
    clippy::too_many_lines,
    reason = "one interpreter of the probe's commands"
)]
pub fn anim_report(script: &str) -> String {
    use std::cell::Cell;
    use std::rc::Rc;

    let lines: Vec<Vec<&str>> = script
        .lines()
        .map(|line| line.split_whitespace().collect())
        .filter(|tokens: &Vec<&str>| !tokens.is_empty())
        .collect();
    let arena: Vec<Option<Vec<u8>>> = lines
        .iter()
        .map(|line| (line[0] == "setchunk").then(|| load(line[3])))
        .collect();
    let mut out = String::new();
    let mut enc: Option<WebPAnimEncoder<'_>> = None;
    let mut options: Option<WebPAnimEncoderOptions> = None;
    let mut config: Option<WebPConfig> = None;
    let mut hook_mode = -1;
    let invalid = WebPMuxError::WEBP_MUX_INVALID_ARGUMENT.0;
    for (line, data) in lines.iter().zip(&arena) {
        match line[0] {
            "options" => {
                options = if line[1..] == ["null"] {
                    None
                } else {
                    let mut value = WebPAnimEncoderOptions::default();
                    assert!(webp_anim_encoder_options_init(&mut value));
                    value.minimize_size = int(line[1]);
                    value.kmin = int(line[2]);
                    value.kmax = int(line[3]);
                    value.allow_mixed = int(line[4]);
                    value.verbose = int(line[5]);
                    value.anim_params.bgcolor = uint(line[6]);
                    value.anim_params.loop_count = int(line[7]);
                    Some(value)
                };
            }
            "optionsinit" => {
                let mark = MARK.cast_unsigned();
                let mut value = WebPAnimEncoderOptions {
                    anim_params: WebPMuxAnimParams {
                        bgcolor: mark,
                        loop_count: MARK,
                    },
                    minimize_size: MARK,
                    kmin: MARK,
                    kmax: MARK,
                    allow_mixed: MARK,
                    verbose: MARK,
                    padding: [mark; 4],
                };
                let ok = webp::mux::anim_encode::webp_anim_encoder_options_init_internal(
                    &mut value,
                    int(line[1]),
                );
                writeln!(out, "optionsinit={}", i32::from(ok)).unwrap();
                writeln!(
                    out,
                    "optionsinit.options={},{},{},{},{},{},{},{}",
                    value.anim_params.bgcolor,
                    value.anim_params.loop_count,
                    value.minimize_size,
                    value.kmin,
                    value.kmax,
                    value.allow_mixed,
                    value.verbose,
                    value.padding[0]
                )
                .unwrap();
            }
            "new" | "newversion" => {
                let (version, width, height) = if line[0] == "new" {
                    (WEBP_MUX_ABI_VERSION, int(line[1]), int(line[2]))
                } else {
                    (int(line[1]), int(line[2]), int(line[3]))
                };
                enc = webp_anim_encoder_new_internal(width, height, options.as_ref(), version);
                writeln!(out, "new={}", i32::from(enc.is_some())).unwrap();
                if let Some(enc) = &enc {
                    print_enc_error(&mut out, "new", Some(enc));
                }
            }
            "delete" => {
                enc = None;
                writeln!(out, "delete=1").unwrap();
            }
            "config" => config = parse_config(&line[1..]),
            "hook" => hook_mode = int(line[1]),
            "add" => {
                let (timestamp, width, height, argb) =
                    (int(line[1]), int(line[2]), int(line[3]), int(line[4]));
                let rgba = std::fs::read(line[5]).unwrap();
                let mut pic = WebPPicture::default();
                assert!(webp_picture_init(&mut pic));
                pic.width = width;
                pic.height = height;
                pic.use_argb = argb;
                assert!(webp::enc::picture_csp_enc::webp_picture_import_rgba(
                    &mut pic,
                    &rgba,
                    width * 4
                ));
                let calls = Rc::new(Cell::new(0));
                if hook_mode >= 0 {
                    let (calls, fail_at) = (Rc::clone(&calls), hook_mode);
                    pic.progress_hook = Some(Box::new(move |_percent: i32| {
                        calls.set(calls.get() + 1);
                        !(fail_at > 0 && calls.get() == fail_at)
                    }));
                }
                let ok = match enc.as_mut() {
                    Some(enc) => {
                        webp_anim_encoder_add(enc, Some(&mut pic), timestamp, config.as_ref())
                    }
                    None => false,
                };
                writeln!(out, "add={}", i32::from(ok)).unwrap();
                writeln!(out, "add.error_code={}", pic.error_code.0).unwrap();
                writeln!(out, "add.use_argb={}", pic.use_argb).unwrap();
                print_enc_error(&mut out, "add", enc.as_ref());
                writeln!(out, "add.hook={}", calls.get()).unwrap();
            }
            "addnull" => {
                let ok = match enc.as_mut() {
                    Some(enc) => webp_anim_encoder_add(enc, None, int(line[1]), config.as_ref()),
                    None => false,
                };
                writeln!(out, "add={}", i32::from(ok)).unwrap();
                print_enc_error(&mut out, "add", enc.as_ref());
            }
            "assemble" => {
                let mut data = WebPData::default();
                let ok = match enc.as_mut() {
                    Some(enc) => webp_anim_encoder_assemble(enc, &mut data),
                    None => false,
                };
                writeln!(out, "assemble={}", i32::from(ok)).unwrap();
                print_enc_error(&mut out, "assemble", enc.as_ref());
                writeln!(
                    out,
                    "assemble.data={},{:016x}",
                    data.bytes.len(),
                    digest(&data.bytes)
                )
                .unwrap();
                writeln!(out, "assemble.hook=0").unwrap();
                if line[1] != "-" && !data.bytes.is_empty() {
                    std::fs::write(format!("{}.rs", line[1]), &data.bytes).unwrap();
                }
            }
            "setchunk" => {
                let data = WebPData {
                    bytes: Cow::Borrowed(data.as_deref().unwrap()),
                };
                let result = match enc.as_mut() {
                    Some(enc) => {
                        webp_anim_encoder_set_chunk(enc, &fourcc(line[1]), &data, int(line[2]))
                    }
                    None => Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT),
                };
                writeln!(out, "setchunk={}", code(&result)).unwrap();
            }
            "getchunk" => {
                let token = line[1];
                let Some(enc) = enc.as_ref() else {
                    writeln!(out, "getchunk.{token}={invalid}").unwrap();
                    writeln!(out, "getchunk.{token}.untouched=1").unwrap();
                    continue;
                };
                let result = webp_anim_encoder_get_chunk(enc, &fourcc(token));
                writeln!(out, "getchunk.{token}={}", code(&result)).unwrap();
                match result {
                    Ok(data) => writeln!(
                        out,
                        "getchunk.{token}.data={},{},{:016x}",
                        data.bytes.len(),
                        i32::from(data_is_null(data)),
                        digest(&data.bytes)
                    )
                    .unwrap(),
                    Err(WebPMuxError::WEBP_MUX_NOT_FOUND)
                        if chunk_get_index_from_four_cc(&fourcc(token))
                            != CHUNK_INDEX::IDX_UNKNOWN =>
                    {
                        writeln!(out, "getchunk.{token}.data=0,1,{:016x}", digest(&[])).unwrap();
                    }
                    Err(_) => writeln!(out, "getchunk.{token}.untouched=1").unwrap(),
                }
            }
            "delchunk" => {
                let result = match enc.as_mut() {
                    Some(enc) => webp_anim_encoder_delete_chunk(enc, &fourcc(line[1])),
                    None => Err(WebPMuxError::WEBP_MUX_INVALID_ARGUMENT),
                };
                writeln!(out, "delchunk={}", code(&result)).unwrap();
            }
            other => panic!("the Rust report has no command {other}"),
        }
    }
    out
}

//------------------------------------------------------------------------------
// Mux scripts

/// The script for one WebP file: created with and without copying, queried,
/// assembled (a round trip) and edited.
pub fn file_script(token: &str, with_hex: bool) -> String {
    let hex = i32::from(with_hex);
    format!(
        "create 0 {token}\nstate\nassemble {hex}\nstate\ncreate 1 {token}\ngetframe 1 {hex}\n\
         getframe 0 0\nsetcanvas 0 0\nassemble 0\nstate\ndelframe 1\nstate\nassemble 0\n"
    )
}

/// The scripts of the pinned test vectors, one per vector.
pub fn vector_scripts() -> Vec<String> {
    crate::oracle::test_vectors()
        .iter()
        .map(|path| {
            let size = std::fs::metadata(path).unwrap().len();
            file_script(&format!("file:{}", path.display()), size <= 2048)
        })
        .collect()
}

/// The scripts of the animated files: WP14's synthetic animations and the
/// oracle's `img2webp` animations, parsed and edited.
pub fn animation_scripts(oracle: &Oracle, directory: &Path) -> Vec<String> {
    let mut files: Vec<(String, std::path::PathBuf)> =
        crate::wp14_support::write_all(directory, &crate::wp14_support::synthetic_animations());
    files.extend(crate::wp14_support::img2webp_animations(oracle, directory));
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
    scripts
}

/// The scripts of truncated and bit-flipped files and of extreme size fields.
pub fn truncated_scripts(directory: &Path) -> Vec<String> {
    let mut sources: Vec<Vec<u8>> = Vec::new();
    for path in crate::oracle::test_vectors() {
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
    for (_, bytes) in crate::wp14_support::synthetic_animations()
        .into_iter()
        .step_by(23)
    {
        sources.push(bytes);
    }
    let mut scripts = Vec::new();
    let mut count = 0;
    for (index, bytes) in sources.iter().enumerate() {
        for point in truncation_points(bytes) {
            let token = file_token(directory, &format!("cut{count}.webp"), &bytes[..point]);
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
            let token = file_token(directory, &format!("flip{count}.webp"), &flipped);
            count += 1;
            scripts.push(format!("create 1 {token}\nstate\nassemble 0\n"));
        }
    }
    // Extended size fields: a RIFF size past the end, chunk sizes over the
    // remaining bytes, over the RIFF size and over MAX_CHUNK_PAYLOAD.
    let small = std::fs::read(crate::oracle::test_data().join("lossless1.webp")).unwrap();
    for riff_size in [0u32, 4, 11, 12, 13, 0xffff_fff6, 0xffff_fff7, u32::MAX] {
        let mut file = small.clone();
        file[4..8].copy_from_slice(&riff_size.to_le_bytes());
        let token = file_token(directory, &format!("riff{riff_size}.webp"), &file);
        scripts.push(format!("create 0 {token}\nstate\nassemble 0\n"));
    }
    for chunk_size in [0u32, 1, 7, 8, 0xffff_fff6, 0xffff_fff7, u32::MAX] {
        let mut file = small.clone();
        file[16..20].copy_from_slice(&chunk_size.to_le_bytes());
        let token = file_token(directory, &format!("chunk{chunk_size}.webp"), &file);
        scripts.push(format!("create 0 {token}\nstate\nassemble 0\n"));
    }
    scripts
}

/// The data that the edit sequences use, as DATA tokens.
pub struct Pool {
    /// Raw VP8 and VP8L bitstreams and single-image WebP files.
    pub images: Vec<String>,
    /// Animated WebP files (an image of `WebPMuxSetImage` takes the first frame).
    pub animations: Vec<String>,
    /// Data that is no image.
    pub garbage: Vec<String>,
    /// Chunk payloads.
    pub payloads: Vec<String>,
}

/// The payload of the first chunk with `tag` of a WebP file.
pub fn payload(file: &[u8], tag: &[u8; 4]) -> Vec<u8> {
    top_level_chunks(file)
        .into_iter()
        .find(|(name, _)| name == tag)
        .map(|(_, bytes)| bytes)
        .unwrap_or_else(|| panic!("no {} chunk", String::from_utf8_lossy(tag)))
}

pub fn hex_token(bytes: &[u8]) -> String {
    format!("hex:{}", hex(bytes))
}

pub fn pool(directory: &Path) -> Pool {
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
    let lossy = std::fs::read(crate::oracle::test_data().join("test.webp")).unwrap();
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

pub const FOURCCS: [&str; 14] = [
    "ICCP", "EXIF", "XMP_", "VP8X", "ANIM", "ANMF", "ALPH", "VP8_", "VP8L", "abcd", "zzzz", "UNKN",
    "XMP ", "iccp",
];

pub fn pick<'p>(rng: &mut Rng, items: &'p [String]) -> &'p str {
    &items[rng.below(items.len() as u64) as usize]
}

pub fn pick_value(rng: &mut Rng, values: &[i64]) -> i64 {
    values[rng.below(values.len() as u64) as usize]
}

/// One seeded edit sequence.
pub fn edit_sequence(rng: &mut Rng, pool: &Pool) -> String {
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

/// The seeded edit sequences and the directed cases.
pub fn edit_scripts(directory: &Path) -> Vec<String> {
    let pool = pool(directory);
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
    scripts
}

//------------------------------------------------------------------------------
// Animation encoder cases

/// The background of a generated frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Background {
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
pub fn frame(width: u32, height: u32, index: u32, background: Background) -> Vec<u8> {
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
pub struct Frames {
    pub directory: PathBuf,
}

impl Frames {
    pub fn path(&self, width: u32, height: u32, index: u32, background: Background) -> PathBuf {
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
pub struct Case {
    pub name: String,
    pub options: String,
    pub width: u32,
    pub height: u32,
    pub background: Background,
    /// Frame index and timestamp of each `add`.
    pub frames: Vec<(u32, i32)>,
    /// The `config` line before each frame (the last one repeats).
    pub configs: Vec<String>,
    /// `use_argb` of each frame (the last one repeats).
    pub argb: Vec<i32>,
    pub hook: i32,
    /// The timestamp of a final `addnull`.
    pub null_frame: Option<i32>,
    /// Lines inserted before `assemble`.
    pub extra: String,
}

impl Case {
    pub fn new(name: &str, width: u32, height: u32, background: Background, count: u32) -> Self {
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

    pub fn script(&self, frames: &Frames, output: &Path) -> String {
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

pub const CONFIGS: [&str; 9] = [
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

/// The option and configuration matrix.
pub fn matrix_cases() -> Vec<Case> {
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
    cases
}

/// Every validity case of `SanitizeEncoderOptions` and long sequences.
pub fn keyframe_cases() -> Vec<Case> {
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
    cases
}

/// Timestamps, errors and progress hooks; the wrong-size frame is written to
/// `directory`.
pub fn error_cases(directory: &Path) -> Vec<Case> {
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
    let wrong = directory.join("wrong.rgba");
    std::fs::write(&wrong, frame(31, 20, 0, Background::Opaque)).unwrap();
    let mut cases = extra_cases;
    for case in &mut cases {
        case.extra = case.extra.replace("MISSING", &wrong.display().to_string());
    }
    cases
}

/// Version checks, the options defaults and an encoder that was never created.
pub fn version_script() -> String {
    "optionsinit 265\noptionsinit 512\noptionsinit 264\nnewversion 512 4 4\n\
                  newversion 265 4 4\nnew -4 4\nnew 4 -4\nnewversion 512 4 4\naddnull 0\nassemble -\n\
                  getchunk ICCP\nsetchunk ICCP 1 hex:01\ndelchunk ICCP\n".to_owned()
}

/// Single frames, a 1 x 1 canvas and metadata chunks.
pub fn single_cases() -> Vec<Case> {
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
    cases
}
