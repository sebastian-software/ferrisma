//! WP17 differential test: the safe encoder API (`webp::api::encode`,
//! ADR 0006) against the C encoder. The cases are WP13's records
//! (`support/wp13_cases.rs`, `support/wp13_matrix.rs`); the C probe
//! `tools/oracle/probes/wp13/encode.c` runs them in a separate process, and
//! this test replays each one through the API: `Picture::import` for the
//! imported pictures, `Picture::from_argb` or `Picture::from_yuva` for the
//! caller's planes (layout `Caller`), `Encoder` with the case's progress hook,
//! statistics and writer (`encode_to` with the custom writer, `encode` for
//! the memory writer), and `Config::from` the case's `WebPConfig`.
//!
//! Compared per case: `valid=` (`Config::validate`), `ret=`, `error=`, every
//! progress report, every writer call and the bytes accepted, all
//! `WebPAuxStats` fields (floats as bit patterns, poisoned before the call as
//! the probe does) and the caller's buffers with their padding. Not compared:
//! `pic=`, because the API does not expose a picture's planes (`caller=`
//! still shows what the encoder wrote into caller memory); the memory
//! writer's capacity (`mem=` keeps the size); and the bytes of a failed
//! memory-writer encoding, which `Encoder::encode` does not return.
//!
//! Left out: views and `extra_info`, which the API does not offer (views are
//! skipped, `extra_info_type` is set to 0). The API refuses two kinds of
//! input before it encodes, which C refuses inside `WebPEncode`: dimensions
//! above 16383 (`InvalidDimensions`, C: `VP8_ENC_ERROR_BAD_DIMENSION`) and a
//! quality outside 0 to 100 (`InvalidParameter`, C:
//! `VP8_ENC_ERROR_INVALID_CONFIGURATION`); the test prints C's code for them
//! and so checks that the observable state (no callback, untouched
//! statistics) is C's.
//!
//! Floating point results (PSNR) are compared bit for bit on the host that
//! runs the test; equality on other hosts is not assumed.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp13_cases.rs"]
mod wp13_cases;
#[path = "support/wp13_matrix.rs"]
mod wp13_matrix;

use std::cell::RefCell;

use oracle::{Fields, Oracle};
use webp::api::encode::{
    Config, EncodeError, Encoder, Picture, PixelLayout, PixelRows, SampleForm, WebPAuxStats,
    YuvaPlanes,
};
use webp::enc::picture_csp_enc::{webp_picture_import_rgb, webp_picture_import_rgba};
use webp::webp::encode::{WebPEncodingError, WebPPicture, webp_picture_init};
use webp::webp::types::{PlaneOffset, PlaneStorage};
use wp13_cases::{
    Case, F, FNV_START, Layout, fnv, join, poisoned_stats, records, split_records, stats_line,
};
use wp13_matrix as matrix;

/// What the callbacks of one encoding saw.
#[derive(Default)]
struct Sinks {
    out: RefCell<Vec<u8>>,
    sizes: RefCell<Vec<usize>>,
    progress: RefCell<Vec<i32>>,
    memory: RefCell<Option<Vec<u8>>>,
}

/// The rows of a case: RGB for sources 0 and 3, RGBA for 1 and 2.
fn rows(case: &Case) -> (PixelLayout, Vec<u8>, usize) {
    let width = case.width as usize;
    if case.source == 0 || case.source == 3 {
        let rgb = case
            .rgba
            .iter()
            .flat_map(|word| word.to_le_bytes()[..3].to_vec())
            .collect();
        (PixelLayout::Rgb, rgb, 3 * width)
    } else {
        let rgba = case
            .rgba
            .iter()
            .flat_map(|word| word.to_le_bytes())
            .collect();
        (PixelLayout::Rgba, rgba, 4 * width)
    }
}

/// The C code that the probe prints for an error of the API. The API's own
/// refusals map to the code C records for the same input (see the module
/// documentation); any other refusal of a WP13 case is a failure.
fn printed_code(case: &Case, error: EncodeError) -> u32 {
    if let Some(code) = error.code() {
        return code.0;
    }
    match error {
        EncodeError::InvalidDimensions { width, height } => {
            assert!(
                width > 16383 || height > 16383,
                "unexpected refusal: {}",
                case.describe()
            );
            WebPEncodingError::VP8_ENC_ERROR_BAD_DIMENSION.0
        }
        EncodeError::InvalidParameter { name: "quality" } => {
            assert!(
                !(0.0..=100.0).contains(&case.quality),
                "unexpected refusal: {}",
                case.describe()
            );
            WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION.0
        }
        other => panic!("unexpected refusal {other:?}: {}", case.describe()),
    }
}

/// Encodes `picture` with the case's hook, statistics and writer.
fn encode(
    case: &Case,
    config: &Config,
    picture: &mut Picture<'_>,
    sinks: &Sinks,
    stats: &mut WebPAuxStats,
) -> Result<(), EncodeError> {
    let mut encoder = Encoder::new(config);
    if case.abort_at >= -1 {
        let abort_at = case.abort_at;
        encoder = encoder.progress(move |percent| {
            let call = sinks.progress.borrow().len();
            sinks.progress.borrow_mut().push(percent);
            call as i32 != abort_at
        });
    }
    if case.with_stats {
        encoder = encoder.stats(stats);
    }
    if case.memory_writer {
        let bytes = encoder.encode(picture)?;
        *sinks.memory.borrow_mut() = Some(bytes);
        return Ok(());
    }
    let fail_write_at = case.fail_write_at;
    encoder.encode_to(picture, |data| {
        let call = sinks.sizes.borrow().len();
        sinks.sizes.borrow_mut().push(data.len());
        if call as i32 == fail_write_at {
            return false;
        }
        sinks.out.borrow_mut().extend_from_slice(data);
        true
    })
}

/// The caller memory of `Layout::Caller`, as the probe's `ToCaller` builds it:
/// the imported planes copied into rows `pad` samples longer, filled with
/// 0x5a, with one guard sample.
struct CallerPlanes {
    yuva: Vec<Vec<u8>>,
    argb: Option<Vec<u32>>,
    y_stride: usize,
    uv_stride: usize,
    a_stride: usize,
    argb_stride: usize,
}

fn copy_rows(
    storage: &PlaneStorage<'_, u8>,
    plane: PlaneOffset,
    src_stride: i32,
    width: usize,
    height: usize,
    stride: usize,
) -> Vec<u8> {
    let region = storage.region(plane.region).unwrap();
    let mut out = vec![0x5a_u8; stride * height + 1];
    for y in 0..height {
        let from = plane.offset + y * src_stride as usize;
        out[y * stride..y * stride + width].copy_from_slice(&region[from..from + width]);
    }
    out
}

fn caller_planes(case: &Case, pad: usize) -> CallerPlanes {
    // The probe imports with the library, then copies the planes.
    let (layout, data, stride) = rows(case);
    let mut source = WebPPicture::default();
    assert!(webp_picture_init(&mut source));
    source.width = case.width;
    source.height = case.height;
    source.use_argb = i32::from(case.source >= 2);
    let imported = if layout == PixelLayout::Rgb {
        webp_picture_import_rgb(&mut source, &data, stride as i32)
    } else {
        webp_picture_import_rgba(&mut source, &data, stride as i32)
    };
    assert!(imported, "import: {}", case.describe());
    let (w, h) = (case.width as usize, case.height as usize);
    let (uv_w, uv_h) = (w.div_ceil(2), h.div_ceil(2));
    if source.use_argb != 0 {
        let plane = source.argb.unwrap();
        let region = source.argb_storage.region(plane.region).unwrap();
        let argb_stride = w + pad;
        let mut argb = vec![0x5a5a_5a5a_u32; argb_stride * h + 1];
        for y in 0..h {
            let from = plane.offset + y * source.argb_stride as usize;
            argb[y * argb_stride..y * argb_stride + w].copy_from_slice(&region[from..from + w]);
        }
        return CallerPlanes {
            yuva: Vec::new(),
            argb: Some(argb),
            y_stride: 0,
            uv_stride: 0,
            a_stride: 0,
            argb_stride,
        };
    }
    let mut yuva = vec![
        copy_rows(
            &source.yuva,
            source.y.unwrap(),
            source.y_stride,
            w,
            h,
            w + pad,
        ),
        copy_rows(
            &source.yuva,
            source.u.unwrap(),
            source.uv_stride,
            uv_w,
            uv_h,
            uv_w + pad,
        ),
        copy_rows(
            &source.yuva,
            source.v.unwrap(),
            source.uv_stride,
            uv_w,
            uv_h,
            uv_w + pad,
        ),
    ];
    if let Some(a) = source.a {
        yuva.push(copy_rows(&source.yuva, a, source.a_stride, w, h, w + pad));
    }
    CallerPlanes {
        yuva,
        argb: None,
        y_stride: w + pad,
        uv_stride: uv_w + pad,
        a_stride: w + pad,
        argb_stride: 0,
    }
}

impl CallerPlanes {
    fn picture(&mut self, case: &Case) -> Result<Picture<'_>, EncodeError> {
        let (width, height) = (case.width as u32, case.height as u32);
        if let Some(argb) = self.argb.as_mut() {
            return Picture::from_argb(argb, width, height, self.argb_stride);
        }
        let [y, u, v, rest @ ..] = self.yuva.as_mut_slice() else {
            panic!("three planes at least");
        };
        let planes = YuvaPlanes {
            y,
            y_stride: self.y_stride,
            u,
            v,
            uv_stride: self.uv_stride,
            a: rest.first_mut().map(Vec::as_mut_slice),
            a_stride: self.a_stride,
        };
        Picture::from_yuva(width, height, planes)
    }

    /// What the probe's `PrintCaller` prints.
    fn line(&self) -> String {
        let mut line = String::new();
        for i in 0..4 {
            match self.yuva.get(i) {
                Some(plane) => line.push_str(&format!("{:016x},", fnv(FNV_START, plane))),
                None => line.push_str("-,"),
            }
        }
        match &self.argb {
            None => line.push('-'),
            Some(argb) => {
                let mut hash = FNV_START;
                for v in argb {
                    hash = fnv(hash, &v.to_le_bytes());
                }
                line.push_str(&format!("{hash:016x}"));
            }
        }
        line
    }
}

/// Replays `case` through the API and prints what the probe prints, without
/// `pic=`.
fn replay(case: &Case) -> Fields {
    let config = Config::from(case.config());
    let valid = config.validate();
    if let Err(error) = valid {
        // The API's own refusal of a quality outside 0 to 100, which C's
        // validation also rejects.
        printed_code(case, error);
    }
    let mut fields = Fields::new().with("valid", i32::from(valid.is_ok()));
    let sinks = Sinks::default();
    let mut stats = poisoned_stats();
    let (result, caller) = match case.layout {
        Layout::Library => {
            let (layout, data, stride) = rows(case);
            let form = if case.source >= 2 {
                SampleForm::Argb
            } else {
                SampleForm::Yuva
            };
            let rows = PixelRows {
                layout,
                data: &data,
                width: case.width as u32,
                height: case.height as u32,
                stride,
            };
            let result = Picture::import(rows, form)
                .and_then(|mut picture| encode(case, &config, &mut picture, &sinks, &mut stats));
            (result, None)
        }
        Layout::Caller { pad } => {
            let mut planes = caller_planes(case, pad as usize);
            let result = planes
                .picture(case)
                .and_then(|mut picture| encode(case, &config, &mut picture, &sinks, &mut stats));
            (result, Some(planes.line()))
        }
        Layout::View { .. } => unreachable!("views are not in the API"),
    };
    fields = fields
        .with("ret", i32::from(result.is_ok()))
        .with(
            "error",
            result.map_or_else(|e| printed_code(case, e), |()| 0),
        )
        .with("progress", join(&sinks.progress.borrow()));
    if case.memory_writer {
        if let Some(bytes) = sinks.memory.borrow().as_ref() {
            fields = fields.with("mem", bytes.len()).hex("out", bytes);
        }
    } else {
        fields = fields
            .with("writes", join(&sinks.sizes.borrow()))
            .hex("out", &sinks.out.borrow());
    }
    if case.with_stats {
        fields = fields.with("stats", stats_line(&stats));
    }
    if let Some(caller) = caller {
        fields = fields.with("caller", caller);
    }
    fields
}

/// The probe's output for one record, without what the API cannot show. A
/// successful encoding has no error in the API; C can record
/// `VP8_ENC_ERROR_USER_ABORT` and still succeed, when the hook's abort falls on
/// a report whose result libwebp ignores (`ResetAllMBInfo`, and `StatLoop` in
/// `VP8EncLoop` for lossy methods 0 to 2). The second value counts that case.
fn comparable(case: &Case, mut c: Fields) -> (Fields, bool) {
    let failed = c.get("ret") == Some("0");
    let mut ignored_abort = false;
    if !failed {
        for (key, value) in &mut c.0 {
            if key == "error" && value != "0" {
                assert_eq!(value, "10", "success with an error: {}", case.describe());
                assert!(case.abort_at >= 0, "{}", case.describe());
                ignored_abort = true;
                *value = "0".to_owned();
            }
        }
    }
    c.0.retain(|(key, _)| key != "pic");
    if case.memory_writer {
        if failed {
            c.0.retain(|(key, _)| key != "mem" && key != "out");
        } else {
            for (key, value) in &mut c.0 {
                if key == "mem" {
                    *value = value.split(',').next().unwrap().to_owned();
                }
            }
        }
    }
    (c, ignored_abort)
}

/// The cases of a WP13 group that the API can express.
fn api_cases(cases: Vec<Case>) -> Vec<Case> {
    cases
        .into_iter()
        .filter(|case| !matches!(case.layout, Layout::View { .. }))
        .map(|mut case| {
            case.extra_info_type = 0;
            case
        })
        .collect()
}

/// Runs `cases` through the probe and the API and compares them.
fn compare(test: &str, cases: impl FnOnce() -> Vec<Case>) -> Option<Vec<(Case, Fields)>> {
    let oracle = Oracle::locate(test)?;
    let cases = api_cases(cases());
    let output = oracle
        .probe("wp13", "encode")
        .run::<&str>(&[], &records(&cases));
    output.success();
    let expected = split_records(output.fields());
    assert_eq!(expected.len(), cases.len(), "{test}: one output per record");
    let (mut encoded, mut ignored_aborts) = (0, 0);
    let mut outputs = Vec::new();
    for (case, c) in cases.into_iter().zip(expected) {
        let (c, ignored_abort) = comparable(&case, c);
        ignored_aborts += usize::from(ignored_abort);
        let rust = replay(&case);
        if c != rust {
            let keys: Vec<_> =
                c.0.iter()
                    .zip(&rust.0)
                    .filter(|(a, b)| a != b)
                    .map(|(a, _)| a.0.clone())
                    .collect();
            panic!(
                "{test}: case {} differs in {keys:?} (key lists {:?} and {:?})\nC:   {:?}\nAPI: {:?}",
                case.describe(),
                c.keys(),
                rust.keys(),
                c.0.iter()
                    .filter(|(k, _)| keys.contains(k))
                    .collect::<Vec<_>>(),
                rust.0
                    .iter()
                    .filter(|(k, _)| keys.contains(k))
                    .collect::<Vec<_>>()
            );
        }
        if rust.get("ret") == Some("1") {
            encoded += 1;
        }
        outputs.push((case, c));
    }
    eprintln!(
        "{test}: {} cases, {encoded} encoded, {ignored_aborts} with an abort that libwebp ignored",
        outputs.len()
    );
    Some(outputs)
}

#[test]
fn lossy_quality_method_and_preset_match_the_c_encoder() {
    compare(
        "lossy_quality_method_and_preset_match_the_c_encoder",
        matrix::lossy_quality_method_and_preset_cases,
    );
}

#[test]
fn lossy_tools_and_threads_match_the_c_encoder() {
    // The extra_info_type dimension of the group is not in the API; its
    // cases without a map remain.
    compare("lossy_tools_and_threads_match_the_c_encoder", || {
        matrix::lossy_tools_cases()
            .into_iter()
            .filter(|case| case.extra_info_type == 0)
            .collect()
    });
}

#[test]
fn lossy_alpha_matches_the_c_encoder() {
    compare(
        "lossy_alpha_matches_the_c_encoder",
        matrix::lossy_alpha_cases,
    );
}

#[test]
fn lossless_exact_and_near_lossless_match_the_c_encoder() {
    compare(
        "lossless_exact_and_near_lossless_match_the_c_encoder",
        matrix::lossless_cases,
    );
}

#[test]
fn targets_match_the_c_encoder() {
    compare("targets_match_the_c_encoder", matrix::rate_control_cases);
}

#[test]
fn progress_aborts_and_writer_failures_match_the_c_encoder() {
    let test = "progress_aborts_and_writer_failures_match_the_c_encoder";
    let Some(outputs) = compare(test, matrix::progress_aborts_and_writers_cases) else {
        return;
    };
    // The aborts reach every progress call, and the writer failures happen.
    let aborted = outputs
        .iter()
        .filter(|(_, c)| c.get("error") == Some("10"))
        .count();
    let refused = outputs
        .iter()
        .filter(|(_, c)| c.get("error") == Some("8"))
        .count();
    assert!(aborted > 100, "{test}: {aborted} aborts");
    assert!(refused > 30, "{test}: {refused} writer failures");
    eprintln!("{test}: {aborted} aborts, {refused} writer failures");
}

#[test]
fn caller_planes_match_the_c_encoder() {
    let test = "caller_planes_match_the_c_encoder";
    let Some(outputs) = compare(test, matrix::views_and_caller_memory_cases) else {
        return;
    };
    let caller = outputs
        .iter()
        .filter(|(case, _)| matches!(case.layout, Layout::Caller { .. }))
        .count();
    let argb = outputs
        .iter()
        .filter(|(case, _)| matches!(case.layout, Layout::Caller { .. }) && case.source >= 2)
        .count();
    assert!(
        caller >= 30 && argb > 0 && argb < caller,
        "{test}: {caller} cases, {argb} ARGB"
    );
}

#[test]
fn dimensions_and_invalid_arguments_match_the_c_encoder() {
    let test = "dimensions_and_invalid_arguments_match_the_c_encoder";
    let Some(outputs) = compare(test, matrix::dimensions_and_invalid_arguments_cases) else {
        return;
    };
    // Both of the API's early refusals occur.
    assert!(
        outputs
            .iter()
            .any(|(case, _)| case.width > 16383 || case.height > 16383)
    );
    assert!(outputs.iter().any(|(case, _)| case.quality > 100.0));
}

#[test]
fn random_configurations_match_the_c_encoder() {
    compare(
        "random_configurations_match_the_c_encoder",
        matrix::random_configurations_cases,
    );
}

/// A case with a raw configuration field set through `Config::raw_mut` gives
/// the same `WebPConfig` as `Config::from` (Rust only: the replay uses
/// `Config::from`, so this pins the other route).
#[test]
fn raw_mut_and_from_build_the_same_configuration() {
    let cases = matrix::random_configurations_cases();
    for case in cases.iter().take(40) {
        let raw = case.config();
        let mut config = Config::new();
        *config.raw_mut() = raw;
        assert_eq!(config, Config::from(raw));
        assert_eq!(*config.as_raw(), raw);
    }
    // The typed setters write the fields the matrix sets.
    let case = &cases[0];
    let raw = case.config();
    let typed = Config::from(raw)
        .method(raw.method as u8)
        .exact(raw.exact != 0)
        .near_lossless(raw.near_lossless as u8)
        .multithreaded(raw.thread_level != 0);
    assert_eq!(*typed.as_raw(), raw, "{}", case.describe());
    assert_eq!(case.get(F::Method), raw.method);
}

/// Straight RGBA8 rows packed as `0xAARRGGBB` words into caller memory
/// (`Picture::from_argb`) encode exactly as the same rows imported into ARGB
/// (`Picture::import`), which the groups above compare with C, and, for a
/// lossy encoding without sharp YUV, as the rows imported into YUVA, which is
/// `cwebp`'s route. This is the packing for a sink that fills a budgeted frame
/// row by row (Rust only: every route ends in `webp_encode`).
#[test]
fn rgba_rows_packed_as_argb_words_encode_as_the_importer() {
    let mut rng = wp13_cases::Rng(0x1717_a4b0);
    let configs = [
        Config::new(),
        Config::new().quality(30.0).method(6).sharp_yuv(true),
        Config::new().lossless(true),
        Config::new().lossless_level(9).unwrap().exact(true),
    ];
    let mut compared = 0;
    for round in 0..32 {
        let alpha = wp13_cases::ALPHAS[round % 4];
        let content = wp13_cases::CONTENTS[(round / 4) % 4];
        let (width, height) = (1 + rng.below(40) as u32, 1 + rng.below(30) as u32);
        let words = wp13_cases::picture(&mut rng, content, alpha, width as i32, height as i32);
        let rgba: Vec<u8> = words.iter().flat_map(|word| word.to_le_bytes()).collect();
        for config in &configs {
            let rows = PixelRows {
                layout: PixelLayout::Rgba,
                data: &rgba,
                width,
                height,
                stride: 4 * width as usize,
            };
            let mut imported = Picture::import(rows, SampleForm::Argb).unwrap();
            let expected = Encoder::new(config).encode(&mut imported).unwrap();
            // One row at a time, as a sink receives them, into padded rows.
            let stride = width as usize + 3;
            let mut frame = vec![0_u32; stride * height as usize];
            for (y, row) in rgba.chunks(4 * width as usize).enumerate() {
                for (x, p) in row.chunks(4).enumerate() {
                    frame[y * stride + x] = u32::from_be_bytes([p[3], p[0], p[1], p[2]]);
                }
            }
            let mut picture = Picture::from_argb(&mut frame, width, height, stride).unwrap();
            let bytes = Encoder::new(config).encode(&mut picture).unwrap();
            assert_eq!(bytes, expected, "round {round}, {config:?}");
            compared += 1;
            // cwebp imports straight into YUVA for a lossy encoding without
            // sharp YUV or preprocessing; the bytes are the same.
            let raw = config.as_raw();
            if raw.lossless == 0 && raw.use_sharp_yuv == 0 && raw.preprocessing == 0 {
                let mut yuva = Picture::import(rows, SampleForm::Yuva).unwrap();
                let direct = Encoder::new(config).encode(&mut yuva).unwrap();
                assert_eq!(direct, expected, "round {round}, YUVA import, {config:?}");
                compared += 1;
            }
        }
    }
    assert_eq!(compared, 160);
}
