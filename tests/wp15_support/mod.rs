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
use std::path::Path;

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

use crate::wp8_support::{digest, hex};

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
