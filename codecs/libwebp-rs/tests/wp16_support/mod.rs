//! Shared helpers of the WP16 differential tests: the commands of WP12's probe
//! `tools/oracle/probes/wp12/decode.c`, replayed through the safe decoder API
//! (`webp::api::decode`), and the comparison of the probe's lines with the
//! API's.
//!
//! The API exposes less than the probe prints (no plane offsets, no buffer
//! descriptions, no `WebPValidateDecoderConfig`), so the API's lines may hold
//! `*` for a whole value or for one comma-separated field, which matches any
//! value there; every other line and field must be equal, in order.
//!
//! Include it with `#[path = "wp16_support/mod.rs"] mod wp16_support;`, after
//! `support/oracle.rs` as `oracle` and `wp12_support/mod.rs` as
//! `wp12_support`.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use std::fmt::Write as _;

use webp::api::decode::{
    ColorMode, Config, DecodeError, IncrementalDecoder, Output, Rows, Status, YuvaPlanes, decode,
    decode_into,
};
use webp::utils::rescaler_utils::webp_rescaler_get_scaled_dimensions;

use crate::wp12_support::{GUARD, Geometry, Options, digest, plane_slices};

/// The API's color mode for C's `WEBP_CSP_MODE` value, or `None` for
/// `MODE_LAST` and beyond, which the API cannot express.
pub fn color_mode(mode: u32) -> Option<ColorMode> {
    Some(match mode {
        0 => ColorMode::Rgb,
        1 => ColorMode::Rgba,
        2 => ColorMode::Bgr,
        3 => ColorMode::Bgra,
        4 => ColorMode::Argb,
        5 => ColorMode::Rgba4444,
        6 => ColorMode::Rgb565,
        7 => ColorMode::PremultipliedRgba,
        8 => ColorMode::PremultipliedBgra,
        9 => ColorMode::PremultipliedArgb,
        10 => ColorMode::PremultipliedRgba4444,
        11 => ColorMode::Yuv,
        12 => ColorMode::Yuva,
        _ => return None,
    })
}

/// Why the API refuses a configuration that libwebp decodes: the deviations 1
/// to 3 in `src/api/decode.rs` (`# Deviations from upstream`, ADR 0007).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Deviation {
    /// An odd `crop_left` or `crop_top` (libwebp snaps lossy crops).
    OddCrop,
    /// A dithering strength above 100 (`WebPDecode` clamps it).
    Dithering,
    /// A scaled side above 16383.
    LargeScale,
}

/// The API's form of a probe command's options.
pub enum Mapped {
    /// The configuration, and the deviation whose refusal it may meet.
    Config(Config, Option<Deviation>),
    /// Options that the API cannot express (negative values, strengths above
    /// 255).
    Unrepresentable,
}

/// The configuration for `mode` and `options` on an image of `width` by
/// `height`. A derived scaled side (0) is computed with the translation's
/// `WebPRescalerGetScaledDimensions` from the cropped size, as libwebp does,
/// because the API needs both sides.
pub fn map_options(mode: ColorMode, options: &Options, width: i32, height: i32) -> Mapped {
    let v = options.0;
    let mut config = Config::new(mode);
    let mut deviation = None;
    let unsigned = |value: i32| u32::try_from(value).ok();
    let (mut w, mut h) = (width, height);
    if v[Options::CROP] != 0 {
        let (Some(left), Some(top), Some(cw), Some(ch)) = (
            unsigned(v[Options::CROP_LEFT]),
            unsigned(v[Options::CROP_TOP]),
            unsigned(v[Options::CROP_WIDTH]),
            unsigned(v[Options::CROP_HEIGHT]),
        ) else {
            return Mapped::Unrepresentable;
        };
        config = config.crop(left, top, cw, ch);
        if left % 2 != 0 || top % 2 != 0 {
            deviation = Some(Deviation::OddCrop);
        }
        (w, h) = (v[Options::CROP_WIDTH], v[Options::CROP_HEIGHT]);
    }
    if v[Options::SCALE] != 0 {
        let (mut sw, mut sh) = (v[Options::SCALED_WIDTH], v[Options::SCALED_HEIGHT]);
        if sw < 0 || sh < 0 {
            return Mapped::Unrepresentable;
        }
        if (sw == 0) != (sh == 0) {
            let (mut dw, mut dh) = (sw, sh);
            if webp_rescaler_get_scaled_dimensions(w, h, &mut dw, &mut dh) {
                (sw, sh) = (dw, dh);
            }
        }
        if sw > 16383 || sh > 16383 {
            deviation = Some(Deviation::LargeScale);
        }
        config = config.scale(sw.cast_unsigned(), sh.cast_unsigned());
    }
    let strength = |value: i32| u8::try_from(value).ok();
    let (Some(dither), Some(alpha_dither)) = (
        strength(v[Options::DITHER]),
        strength(v[Options::ALPHA_DITHER]),
    ) else {
        return Mapped::Unrepresentable;
    };
    if dither > 100 || alpha_dither > 100 {
        deviation = deviation.or(Some(Deviation::Dithering));
    }
    config = config
        .bypass_filtering(v[Options::BYPASS] != 0)
        .fancy_upsampling(v[Options::NOFANCY] == 0)
        .multithreaded(v[Options::THREADS] != 0)
        .dithering(dither)
        .alpha_dithering(alpha_dither)
        .flip(v[Options::FLIP] != 0);
    Mapped::Config(config, deviation)
}

/// The caller memory of `geometry` as an API output for `mode`, over the
/// guarded blocks' planes, or `None` when the API cannot express it (a
/// negative stride, different U and V strides, fewer than three planes).
pub fn output<'a>(
    mode: ColorMode,
    geometry: &Geometry,
    blocks: &'a mut [Vec<u8>],
) -> Option<Output<'a>> {
    let strides: Option<Vec<usize>> = geometry
        .0
        .iter()
        .map(|&(stride, _)| usize::try_from(stride).ok())
        .collect();
    let strides = strides?;
    let mut slices = plane_slices(blocks).into_iter();
    if mode.is_yuv() {
        if strides.len() < 3 || strides[1] != strides[2] {
            return None;
        }
        Some(Output::Planes(YuvaPlanes {
            y: slices.next()?,
            y_stride: strides[0],
            u: slices.next()?,
            v: slices.next()?,
            uv_stride: strides[1],
            a: slices.next(),
            a_stride: strides.get(3).copied().unwrap_or(0),
        }))
    } else {
        Some(Output::Rows {
            data: slices.next()?,
            stride: *strides.first()?,
        })
    }
}

/// The C status that an API result stands for: the code of libwebp's errors,
/// and `VP8_STATUS_INVALID_PARAM` for the API's validation, which refuses
/// what libwebp refuses with that code.
pub fn status_code<T>(result: &Result<T, DecodeError>) -> u32 {
    match result {
        Ok(_) => 0,
        Err(error) => error.code().map_or(2, |code| code.0),
    }
}

/// The digest of the rows, row after row, as the probe's `PrintRowsDigest`.
pub fn rows_digest(rows: &Rows<'_>) -> u64 {
    let bytes: Vec<u8> = rows.iter().flatten().copied().collect();
    digest(&bytes)
}

/// The `ext` lines of the probe for the guarded blocks.
pub fn ext_lines(out: &mut String, blocks: &[Vec<u8>]) {
    for (index, block) in blocks.iter().enumerate() {
        writeln!(out, "ext{index}={:016x}", digest(block)).unwrap();
    }
}

/// Whether the blocks still hold the probe's pattern.
pub fn untouched(blocks: &[Vec<u8>], geometry: &Geometry) -> bool {
    blocks == geometry.blocks().as_slice()
}

/// What a replayed command gives.
pub enum Replay {
    /// The probe's lines with wildcards.
    Lines(String),
    /// The API refused a configuration that libwebp decodes (the deviation);
    /// the probe's status for it must be 0 (or `idec=1`).
    Refused(Deviation, DecodeError),
    /// The command cannot be expressed through the API.
    Skipped,
}

/// The `decode` command of the probe, through `decode` or `decode_into`.
pub fn replay_decode(
    data: &[u8],
    mode: u32,
    options: &Options,
    geometry: &Geometry,
    width: i32,
    height: i32,
) -> Replay {
    let Some(color) = color_mode(mode) else {
        return Replay::Skipped;
    };
    let Mapped::Config(config, deviation) = map_options(color, options, width, height) else {
        return Replay::Skipped;
    };
    let mut out = String::new();
    let buffer_key = if color.is_yuv() { "yuva" } else { "rgba" };
    if geometry.0.is_empty() {
        let result = decode(data, &config);
        if let (Some(deviation), Err(error)) = (deviation, &result)
            && error.code().is_none()
        {
            return Replay::Refused(deviation, *error);
        }
        writeln!(out, "valid=*\nstatus={}\ninput=*", status_code(&result)).unwrap();
        match result {
            Ok(image) => {
                writeln!(out, "out={mode},{},{},0", image.width(), image.height()).unwrap();
                writeln!(out, "{buffer_key}=*").unwrap();
                writeln!(out, "mem={:016x}", digest(image.samples())).unwrap();
            }
            Err(_) => writeln!(out, "out=*\n{buffer_key}=*\nmem=null").unwrap(),
        }
        return Replay::Lines(out);
    }
    let mut blocks = geometry.blocks();
    let result = {
        let Some(output) = output(color, geometry, &mut blocks) else {
            return Replay::Skipped;
        };
        decode_into(data, &config, output)
    };
    if let (Some(deviation), Err(error)) = (deviation, &result)
        && error.code().is_none()
    {
        assert!(
            untouched(&blocks, geometry),
            "a refused output stays untouched"
        );
        return Replay::Refused(deviation, *error);
    }

    writeln!(out, "valid=*\nstatus={}\ninput=*", status_code(&result)).unwrap();
    match result {
        Ok((w, h)) => writeln!(out, "out={mode},{w},{h},1").unwrap(),
        Err(_) => out.push_str("out=*\n"),
    }
    writeln!(out, "{buffer_key}=*").unwrap();
    ext_lines(&mut out, &blocks);
    Replay::Lines(out)
}

/// One `incr` command of the probe.
pub struct Incremental<'c> {
    pub data: &'c [u8],
    pub chunk: usize,
    pub append: u32,
    pub kind: &'c str,
    pub mode: u32,
    pub options: Options,
    pub geometry: Geometry,
    pub width: i32,
    pub height: i32,
}

/// The `incr` command of the probe, through `IncrementalDecoder`.
///
/// `config` is `WebPIDecode` without data; `rgb`, `yuva`, `yuv` and `new` are
/// `WebPINewRGB`, `WebPINewYUVA`, `WebPINewYUV` and `WebPINewDecoder`, which
/// set no options: the API's default options decode identically.
/// `fconfig` (features first, which matters only for slow memory) and the
/// constructor arguments that the API cannot express are skipped.
pub fn replay_incremental(case: &Incremental<'_>) -> Replay {
    let library = case.geometry.0.is_empty();
    // The colorspace that the probe's decoder has.
    let mode = match case.kind {
        "config" | "rgb" => case.mode,
        "new" => 0,
        "yuva" | "yuv" => {
            if !library && (case.geometry.0.len() == 3 || case.kind == "yuv") {
                11
            } else {
                12
            }
        }
        _ => return Replay::Skipped,
    };
    let Some(color) = color_mode(mode) else {
        return Replay::Skipped;
    };
    if case.kind == "rgb" && color.is_yuv() {
        return Replay::Skipped; // WebPINewRGB with a YUV mode
    }
    if matches!(case.kind, "yuva" | "yuv") && !library && case.geometry.0.len() < 3 {
        return Replay::Skipped; // a missing chroma plane
    }
    let (config, deviation) = if case.kind == "config" {
        match map_options(color, &case.options, case.width, case.height) {
            Mapped::Config(config, deviation) => (config, deviation),
            Mapped::Unrepresentable => return Replay::Skipped,
        }
    } else {
        (Config::new(color), None)
    };
    let mut blocks = case.geometry.blocks();
    let mut out = String::new();
    {
        let decoder = if library {
            IncrementalDecoder::new(&config)
        } else {
            let Some(output) = output(color, &case.geometry, &mut blocks) else {
                return Replay::Skipped;
            };
            IncrementalDecoder::with_output(&config, output)
        };
        let mut decoder = match decoder {
            Ok(decoder) => decoder,
            Err(error) => {
                if let Some(deviation) = deviation {
                    return Replay::Refused(deviation, error);
                }
                out.push_str("idec=0\n");
                return Replay::Lines(out);
            }
        };
        out.push_str("idec=1\n");
        let mut pos = 0;
        let mut prev_last_y = -1_i64;
        let mut result;
        loop {
            let next = if case.data.len() - pos < case.chunk {
                case.data.len()
            } else {
                pos + case.chunk
            };
            result = if case.append == 1 || (case.append == 2 && pos == 0) {
                decoder.append(&case.data[pos..next])
            } else {
                // A fresh copy each time, so that the data moves between
                // calls, as WP12's replay of `WebPIUpdate` does.
                let fresh = case.data[..next].to_vec();
                decoder.update(&fresh)
            };
            pos = next;
            let code = match result {
                Ok(Status::Complete) => 0,
                Ok(Status::NeedsMoreData) => 5,
                Err(error) => error.code().map_or(2, |code| code.0),
            };
            writeln!(out, "step={pos},{code}").unwrap();
            step_lines(&mut out, &decoder, color, !library, &mut prev_last_y);
            if result != Ok(Status::NeedsMoreData) || pos == case.data.len() {
                break;
            }
        }
        end_lines(
            &mut out,
            decoder,
            case,
            mode,
            result == Ok(Status::Complete),
        );
    }
    if !library {
        ext_lines(&mut out, &blocks);
    }
    Replay::Lines(out)
}

/// `PrintIncremental` of the probe, from the API.
fn step_lines(
    out: &mut String,
    decoder: &IncrementalDecoder<'_>,
    color: ColorMode,
    external: bool,
    prev_last_y: &mut i64,
) {
    let size = decoder.output_size();
    let rows = decoder.decoded_rows();
    // Without a size, libwebp's area may exist with a width of 0 (a lossless
    // header before the output is set up), which the API reports as none.
    match size {
        Some((w, _)) => writeln!(out, "area=1,0,0,{w},{rows}").unwrap(),
        None => writeln!(out, "area=*,0,0,0,{rows}").unwrap(),
    }
    // The size that the getters report; caller memory is lent before the
    // header sets the size, with 0 rows.
    let wh = size.map_or_else(|| "*,*".to_owned(), |(w, h)| format!("{w},{h}"));
    if color.is_yuv() {
        match decoder.planes() {
            None => out.push_str("yuva=null,*,*,*\n"),
            Some(planes) => {
                let a = if planes.a.is_some() { "*" } else { "-1" };
                let a_stride = planes.a.map_or(0, |a| a.stride());
                writeln!(
                    out,
                    "yuva=*,*,*,{a},{},{wh},{},{},{a_stride}",
                    planes.y.len(),
                    planes.y.stride(),
                    planes.u.stride()
                )
                .unwrap();
                let last_y = i64::from(planes.y.len());
                if last_y != *prev_last_y {
                    writeln!(out, "y={:016x}", rows_digest(&planes.y)).unwrap();
                    if external {
                        writeln!(out, "u={:016x}", rows_digest(&planes.u)).unwrap();
                        writeln!(out, "v={:016x}", rows_digest(&planes.v)).unwrap();
                        if let Some(a) = planes.a {
                            writeln!(out, "a={:016x}", rows_digest(&a)).unwrap();
                        }
                    }
                    *prev_last_y = last_y;
                }
            }
        }
    } else {
        match decoder.rows() {
            None => out.push_str("rgb=null,*,*,*,*\n"),
            Some(rows) => {
                writeln!(out, "rgb=*,{},{wh},{}", rows.len(), rows.stride()).unwrap();
                let last_y = i64::from(rows.len());
                if last_y != *prev_last_y {
                    writeln!(out, "rows={:016x}", rows_digest(&rows)).unwrap();
                    *prev_last_y = last_y;
                }
            }
        }
    }
}

/// The probe's lines after the steps, before the caller's blocks: the output
/// buffer and the library memory.
fn end_lines(
    out: &mut String,
    decoder: IncrementalDecoder<'_>,
    case: &Incremental<'_>,
    mode: u32,
    complete: bool,
) {
    let library = case.geometry.0.is_empty();
    let size = decoder.output_size();
    if case.kind != "config" && size.is_none() {
        return; // the probe prints only the caller's blocks
    }
    let key = if mode >= 11 { "yuva" } else { "rgba" };
    match (complete, size) {
        (true, Some((w, h))) => {
            writeln!(out, "out={mode},{w},{h},{}", i32::from(!library)).unwrap();
        }
        _ => out.push_str("out=*\n"),
    }
    writeln!(out, "{key}=*").unwrap();
    if library {
        if complete {
            let image = decoder.into_image().expect("a completed decoding");
            writeln!(out, "mem={:016x}", digest(image.samples())).unwrap();
        } else {
            out.push_str("mem=*\n");
        }
    }
}

/// Splits the probe's output into its commands: the lines after each `> `
/// echo.
pub fn split(output: &str) -> Vec<(String, Vec<String>)> {
    let mut blocks: Vec<(String, Vec<String>)> = Vec::new();
    for line in output.lines() {
        if let Some(command) = line.strip_prefix("> ") {
            blocks.push((command.to_owned(), Vec::new()));
        } else if let Some((_, lines)) = blocks.last_mut() {
            lines.push(line.to_owned());
        }
    }
    blocks
}

/// Whether the probe's line matches the API's line with its wildcards.
pub fn matches(probe: &str, api: &str) -> bool {
    let (Some((probe_key, probe_value)), Some((api_key, api_value))) =
        (probe.split_once('='), api.split_once('='))
    else {
        return probe == api;
    };
    if probe_key != api_key {
        return false;
    }
    if api_value == "*" {
        return true;
    }
    let probe_fields: Vec<&str> = probe_value.split(',').collect();
    let api_fields: Vec<&str> = api_value.split(',').collect();
    probe_fields.len() == api_fields.len()
        && probe_fields
            .iter()
            .zip(&api_fields)
            .all(|(p, a)| *a == "*" || p == a)
}

/// The tally of a replay.
#[derive(Debug, Default)]
pub struct Tally {
    pub compared: usize,
    pub skipped: usize,
    pub refused: std::collections::BTreeMap<Deviation, usize>,
}

/// Compares the probe's lines of one command with the API's replay, and
/// panics at the first difference.
pub fn check(test: &str, command: &str, probe: &[String], replay: Replay, tally: &mut Tally) {
    match replay {
        Replay::Skipped => tally.skipped += 1,
        Replay::Refused(deviation, error) => {
            // C's status: `status=` of a decoding, or the codes of the steps.
            let status = probe.iter().find_map(|line| line.strip_prefix("status="));
            let steps: Vec<&str> = probe
                .iter()
                .filter_map(|line| line.strip_prefix("step="))
                .filter_map(|step| step.rsplit(',').next())
                .collect();
            let c_refused = status == Some("2") || steps.contains(&"2");
            let c_decoded = status == Some("0")
                || (status.is_none()
                    && !steps.is_empty()
                    && steps.iter().all(|code| *code == "0" || *code == "5"));
            if c_refused {
                // Both refuse the configuration (libwebp with INVALID_PARAM).
                tally.compared += 1;
            } else {
                assert!(
                    c_decoded,
                    "{test}: `{command}`: the API refused it ({error:?}, {deviation:?}), C failed otherwise:\n{}",
                    probe.join("\n")
                );
                *tally.refused.entry(deviation).or_default() += 1;
            }
        }
        Replay::Lines(api) => {
            let api: Vec<&str> = api.lines().collect();
            for (index, (p, a)) in probe.iter().zip(&api).enumerate() {
                assert!(
                    matches(p, a),
                    "{test}: `{command}`: line {index} differs\n--- C ---\n{p}\n--- API ---\n{a}\n--- C block ---\n{}",
                    probe.join("\n")
                );
            }
            assert_eq!(
                probe.len(),
                api.len(),
                "{test}: `{command}`: C printed {} lines, the API {}\n--- C ---\n{}\n--- API ---\n{}",
                probe.len(),
                api.len(),
                probe.join("\n"),
                api.join("\n")
            );
            tally.compared += 1;
        }
    }
}

/// The guard on both sides of the probe's blocks, for tests that build their
/// own memory.
pub const BLOCK_GUARD: usize = GUARD;
