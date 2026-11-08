//! WP8 differential tests of the safe API (`src/decode.rs`, ADR 0006).
//!
//! The safe API calls the translated libjpeg API in a fixed order. Each case
//! runs a decode through it and the same calls through the C oracle: the
//! WP6 probe `tools/oracle/probes/wp6/api.c` for decoding, with the output
//! parameters as options and the crops, skips and reads as its plan, and the
//! WP2 probe `tools/oracle/probes/wp2/header.c` for the saved markers and the
//! ICC profile. Both sides report the output dimensions, the crop, an FNV-1a
//! hash of every row read, the warnings with their codes and texts, and the
//! error. The memory limit is compared with the oracle's `JPEGMEM`, which
//! sets the same `max_memory_to_use`.
//!
//! The probe prints the warnings of a failed `jpeg_start_decompress` or
//! `jpeg_finish_decompress` too; the safe API drops the decoder with the
//! error, so those cases compare the error and the rows only.

#[path = "support/inputs.rs"]
mod inputs;
#[path = "support/oracle.rs"]
mod oracle;

use inputs::{eight_bit_variants, encode, entropy_positions, four_component, image, join, parts};
use jpeg::jerror::J_MESSAGE_CODE;
use jpeg::{
    DctMethod, Decoder, DecoderOptions, Error, MAX_KEPT_WARNINGS, Message, OutputOptions,
    PixelFormat, Scale,
};
use oracle::{Fields, Oracle};

/// The probes' FNV-1a constants, as `api.c` defines them.
const FNV_OFFSET: u64 = 1_469_598_103_934_665_603;
const FNV_PRIME: u64 = 1_099_511_628_211;

/// A step of a decode after `jpeg_start_decompress`.
#[derive(Clone, Copy, Debug)]
enum Step {
    /// `Scanlines::crop`, the probe's `cX:W`.
    Crop(u32, u32),
    /// `Scanlines::skip_rows`, the probe's `sK`.
    Skip(u32),
    /// `Scanlines::read_rows` with a buffer of K rows, the probe's `rK`.
    Read(u32),
    /// `Scanlines::finish` now, the probe's `e`.
    Finish,
}

/// What a decode showed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Observed {
    /// `output_width`, `output_height` and `output_components` after the
    /// start.
    dims: Option<(u32, u32, u32)>,
    /// The crop result and the new `output_width`.
    crop: Option<(u32, u32, u32)>,
    rows: Vec<String>,
    /// The warnings and their count, when the side could report them.
    warnings: Option<(Vec<(u32, String)>, u64)>,
    /// The error code and text.
    error: Option<(u32, String)>,
}

fn row_hash(row: &[u8]) -> String {
    let mut hash = FNV_OFFSET;
    for &sample in row {
        hash = (hash ^ u64::from(sample)).wrapping_mul(FNV_PRIME);
    }
    format!("{hash:016x}")
}

fn unhex(hex: &str) -> String {
    let bytes: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).unwrap())
        .collect();
    String::from_utf8(bytes).unwrap()
}

fn message(message: &Message) -> (u32, String) {
    (message.code.0, message.text.clone())
}

/// The `J_COLOR_SPACE` number of each format, as `jpeglib.h` numbers them.
fn jcs(format: PixelFormat) -> u32 {
    match format {
        PixelFormat::Grayscale => 1,
        PixelFormat::Rgb => 2,
        PixelFormat::YCbCr => 3,
        PixelFormat::Cmyk => 4,
        PixelFormat::Ycck => 5,
        PixelFormat::Rgbx => 7,
        PixelFormat::Bgr => 8,
        PixelFormat::Bgrx => 9,
        PixelFormat::Xbgr => 10,
        PixelFormat::Xrgb => 11,
        PixelFormat::Rgba => 12,
        PixelFormat::Bgra => 13,
        PixelFormat::Abgr => 14,
        PixelFormat::Argb => 15,
    }
}

const FORMATS: [PixelFormat; 14] = [
    PixelFormat::Grayscale,
    PixelFormat::Rgb,
    PixelFormat::Rgbx,
    PixelFormat::Bgr,
    PixelFormat::Bgrx,
    PixelFormat::Xbgr,
    PixelFormat::Xrgb,
    PixelFormat::Rgba,
    PixelFormat::Bgra,
    PixelFormat::Abgr,
    PixelFormat::Argb,
    PixelFormat::YCbCr,
    PixelFormat::Cmyk,
    PixelFormat::Ycck,
];

/// The probe options for `output` and `plan`.
fn probe_args(output: &OutputOptions, plan: &[Step]) -> Vec<String> {
    let mut args = Vec::new();
    if let Some(format) = output.format {
        args.extend(["-out".to_owned(), jcs(format).to_string()]);
    }
    if output.scale != Scale::ONE {
        args.extend([
            "-scale".to_owned(),
            output.scale.num.to_string(),
            output.scale.denom.to_string(),
        ]);
    }
    match output.dct_method {
        DctMethod::IntSlow => {}
        DctMethod::IntFast => args.extend(["-dct".to_owned(), "ifast".to_owned()]),
        DctMethod::Float => args.extend(["-dct".to_owned(), "float".to_owned()]),
    }
    if !output.fancy_upsampling {
        args.push("-nofancy".to_owned());
    }
    if !output.block_smoothing {
        args.push("-nosmooth".to_owned());
    }
    if !plan.is_empty() {
        let steps: Vec<String> = plan
            .iter()
            .map(|step| match step {
                Step::Crop(x, width) => format!("c{x}:{width}"),
                Step::Skip(rows) => format!("s{rows}"),
                Step::Read(rows) => format!("r{rows}"),
                Step::Finish => "e".to_owned(),
            })
            .collect();
        args.extend(["-plan".to_owned(), steps.join(",")]);
    }
    args
}

/// Runs the probe; `jpegmem` sets `JPEGMEM`, otherwise it is removed.
fn run_oracle(
    oracle: &Oracle,
    output: &OutputOptions,
    plan: &[Step],
    jpegmem: Option<&str>,
    data: &[u8],
) -> Observed {
    let fields = oracle
        .probe("wp6", "api")
        .run_env(&probe_args(output, plan), data, &[("JPEGMEM", jpegmem)])
        .success()
        .fields();
    observe_oracle(&fields)
}

fn numbers(value: &str) -> Vec<u32> {
    value.split(',').map(|n| n.parse().unwrap()).collect()
}

fn warnings_of(fields: &Fields) -> Vec<(u32, String)> {
    fields
        .0
        .iter()
        .filter(|(key, _)| key == "msg")
        .filter_map(|(_, value)| {
            let rest = value.strip_prefix("-1:")?;
            let (code, hex) = rest.split_once(":=").unwrap();
            Some((code.parse().unwrap(), unhex(hex)))
        })
        .collect()
}

fn observe_oracle(fields: &Fields) -> Observed {
    let dims = fields.get("dims").map(|value| {
        let n = numbers(value);
        (n[0], n[1], n[3])
    });
    let crop = fields.get("crop").map(|value| {
        let n = numbers(value);
        (n[0], n[1], n[2])
    });
    let rows = match fields.get("rows") {
        None | Some("") => Vec::new(),
        Some(rows) => rows.split(',').map(str::to_owned).collect(),
    };
    let code = u32::try_from(fields.int("error")).unwrap();
    let error = (code != 0).then(|| (code, fields.get("error_message").unwrap().to_owned()));
    let mut warnings = warnings_of(fields);
    warnings.truncate(MAX_KEPT_WARNINGS);
    let count = u64::try_from(fields.int("num_warnings")).unwrap();
    Observed {
        dims,
        crop,
        rows,
        warnings: Some((warnings, count)),
        error,
    }
}

/// The port's side: the safe API with `options`, `output` and `plan`, then
/// one row per read until the end and `finish`.
fn run_port(
    options: &DecoderOptions,
    output: &OutputOptions,
    plan: &[Step],
    data: &[u8],
) -> Observed {
    let mut observed = Observed::default();
    let fail = |observed: &mut Observed, error: Error| {
        let (Error::Library(m) | Error::Warning(m)) = error else {
            panic!("unexpected {error:?}");
        };
        observed.error = Some(message(&m));
    };
    let decoder = match Decoder::with_options(data, options) {
        Ok(decoder) => decoder,
        Err(error) => {
            fail(&mut observed, error);
            return observed;
        }
    };
    let warnings = |report: jpeg::Report| {
        Some((
            report.warnings.iter().map(message).collect(),
            report.warning_count,
        ))
    };
    // The header must agree with what the start reports.
    let header = *decoder.header();
    let mut scanlines = match decoder.start(output) {
        Ok(scanlines) => scanlines,
        Err(error) => {
            fail(&mut observed, error);
            return observed;
        }
    };
    assert!(header.width > 0 && header.height > 0);
    let info = *scanlines.info();
    observed.dims = Some((info.width, info.height, info.components));
    // Padded and alpha formats give the same bytes, so the reported
    // format shows which one libjpeg was given.
    if let Some(format) = output.format {
        assert_eq!(info.format, format);
    }
    let read = |scanlines: &mut jpeg::Scanlines<'_>, observed: &mut Observed, rows: u32| {
        let row_bytes = scanlines.info().row_bytes();
        let mut buffer = vec![0; row_bytes * rows as usize];
        let count = scanlines.read_rows(&mut buffer)?;
        for row in buffer.chunks_exact(row_bytes).take(count as usize) {
            observed.rows.push(row_hash(row));
        }
        Ok::<u32, Error>(count)
    };
    for step in plan {
        let result = match *step {
            Step::Crop(x, width) => scanlines.crop(x, width).map(|(x, width)| {
                observed.crop = Some((x, width, scanlines.info().width));
            }),
            Step::Skip(rows) => scanlines.skip_rows(rows).map(drop),
            Step::Read(rows) => read(&mut scanlines, &mut observed, rows).map(drop),
            Step::Finish => {
                match scanlines.finish() {
                    Ok(report) => observed.warnings = warnings(report),
                    Err(error) => fail(&mut observed, error),
                }
                return observed;
            }
        };
        if let Err(error) = result {
            observed.warnings = warnings(scanlines.report());
            fail(&mut observed, error);
            return observed;
        }
    }
    while scanlines.next_row() < scanlines.info().height {
        if let Err(error) = read(&mut scanlines, &mut observed, 1) {
            observed.warnings = warnings(scanlines.report());
            fail(&mut observed, error);
            return observed;
        }
    }
    match scanlines.finish() {
        Ok(report) => observed.warnings = warnings(report),
        Err(error) => fail(&mut observed, error),
    }
    observed
}

/// Compares the two sides; warnings only where the port reports them.
fn assert_same(mut port: Observed, oracle: Observed, label: &str) {
    if port.warnings.is_none() {
        port.warnings.clone_from(&oracle.warnings);
    }
    if port != oracle {
        let row = port.rows.iter().zip(&oracle.rows).position(|(a, e)| a != e);
        panic!(
            "{label}: first differing row {row:?}\n  port:   {:?} {:?} {} rows {:?} {:?}\n  oracle: {:?} {:?} {} rows {:?} {:?}",
            port.dims,
            port.crop,
            port.rows.len(),
            port.warnings,
            port.error,
            oracle.dims,
            oracle.crop,
            oracle.rows.len(),
            oracle.warnings,
            oracle.error,
        );
    }
}

fn compare(
    oracle: &Oracle,
    output: &OutputOptions,
    plan: &[Step],
    data: &[u8],
    label: &str,
) -> Observed {
    let expected = run_oracle(oracle, output, plan, None, data);
    let actual = run_port(&DecoderOptions::default(), output, plan, data);
    let label = format!("{label} {output:?} {plan:?}");
    assert_same(actual, expected.clone(), &label);
    expected
}

fn with(adjust: impl FnOnce(&mut OutputOptions)) -> OutputOptions {
    let mut output = OutputOptions::default();
    adjust(&mut output);
    output
}

/// The output parameters compared for every input.
fn modes() -> Vec<OutputOptions> {
    vec![
        OutputOptions::default(),
        with(|o| o.fancy_upsampling = false),
        with(|o| o.dct_method = DctMethod::IntFast),
        with(|o| o.dct_method = DctMethod::Float),
        with(|o| o.block_smoothing = false),
        with(|o| o.format = Some(PixelFormat::Grayscale)),
    ]
}

/// Upstream's test images and the 8-bit encoder variants, read whole, and
/// with a crop, skips and multi-row reads.
#[test]
fn decodes_match_the_oracle() {
    let Some(oracle) = Oracle::locate("decodes_match_the_oracle") else {
        return;
    };
    let mut inputs: Vec<(String, Vec<u8>)> = ["testorig.jpg", "testimgari.jpg", "testimgint.jpg"]
        .iter()
        .map(|name| ((*name).to_owned(), image(name)))
        .collect();
    inputs.extend(eight_bit_variants(&oracle));
    for (label, data) in &inputs {
        for output in modes() {
            let fields = compare(&oracle, &output, &[], data, label);
            assert_eq!(fields.error, None, "{label} {output:?}");
            compare(
                &oracle,
                &output,
                &[
                    Step::Crop(37, 61),
                    Step::Skip(11),
                    Step::Read(7),
                    Step::Skip(40),
                    Step::Read(16),
                ],
                data,
                label,
            );
        }
    }
}

/// Every output format for every color space of the input, with a crop and
/// a skip; conversions libjpeg lacks fail with its error.
#[test]
fn formats_match_the_oracle() {
    let Some(oracle) = Oracle::locate("formats_match_the_oracle") else {
        return;
    };
    let inputs = [
        ("testorig.jpg".to_owned(), image("testorig.jpg")),
        ("cmyk".to_owned(), four_component(&oracle, &["cmyk"])),
        (
            "ycck 2x2".to_owned(),
            four_component(&oracle, &["ycck", "2x2,1x1,1x1,2x2"]),
        ),
        ("-rgb".to_owned(), encode(&oracle, &["-rgb"])),
        ("-grayscale".to_owned(), encode(&oracle, &["-grayscale"])),
    ];
    let mut failures = 0;
    for (label, data) in &inputs {
        for format in FORMATS.map(Some).into_iter().chain([None]) {
            let output = with(|o| o.format = format);
            for plan in [
                &[][..],
                &[Step::Crop(17, 50), Step::Skip(9), Step::Read(30)],
            ] {
                let fields = compare(&oracle, &output, plan, data, label);
                if fields.error.is_some() {
                    failures += 1;
                }
            }
        }
    }
    // CMYK and YCCK files convert only to CMYK and YCCK.
    assert!(failures > 0, "no conversion failed");
}

/// Every scale from 1/8 to 16/8 and factors between them, with crops.
#[test]
fn scales_match_the_oracle() {
    let Some(oracle) = Oracle::locate("scales_match_the_oracle") else {
        return;
    };
    let inputs = [
        ("testorig.jpg".to_owned(), image("testorig.jpg")),
        (
            "-progressive".to_owned(),
            encode(&oracle, &["-progressive"]),
        ),
        (
            "-sample 1x1".to_owned(),
            encode(&oracle, &["-sample", "1x1"]),
        ),
        ("-grayscale".to_owned(), encode(&oracle, &["-grayscale"])),
    ];
    let mut scales: Vec<Scale> = (1..=16).map(|num| Scale { num, denom: 8 }).collect();
    scales.extend([
        Scale { num: 1, denom: 3 },
        Scale { num: 3, denom: 5 },
        Scale { num: 2, denom: 1 },
        Scale { num: 5, denom: 1 },
        Scale { num: 1, denom: 100 },
    ]);
    for (label, data) in &inputs {
        for scale in &scales {
            for fancy in [true, false] {
                let output = with(|o| {
                    o.scale = *scale;
                    o.fancy_upsampling = fancy;
                });
                compare(&oracle, &output, &[], data, label);
                compare(
                    &oracle,
                    &output,
                    &[Step::Crop(3, 9), Step::Skip(2), Step::Read(3)],
                    data,
                    label,
                );
            }
        }
    }
}

/// Crops, skips and reads in many combinations, including requests libjpeg
/// rejects and an early finish.
#[test]
fn plans_match_the_oracle() {
    let Some(oracle) = Oracle::locate("plans_match_the_oracle") else {
        return;
    };
    let data = image("testorig.jpg");
    let progressive = encode(&oracle, &["-progressive"]);
    let plans: &[&[Step]] = &[
        &[Step::Crop(0, 227)],
        &[Step::Crop(0, 1)],
        &[Step::Crop(226, 1)],
        &[Step::Crop(100, 20), Step::Read(149)],
        &[Step::Crop(100, 200)],
        &[Step::Crop(0, 0)],
        &[Step::Read(1), Step::Crop(0, 10)],
        &[Step::Skip(0), Step::Skip(149)],
        &[Step::Skip(148), Step::Read(5)],
        &[Step::Skip(500)],
        &[Step::Read(149), Step::Read(1)],
        &[Step::Read(1000)],
        &[
            Step::Skip(16),
            Step::Read(16),
            Step::Skip(16),
            Step::Read(1),
        ],
        &[Step::Read(10), Step::Finish],
        &[Step::Finish],
    ];
    for (label, input) in [("testorig.jpg", &data), ("-progressive", &progressive)] {
        for plan in plans {
            compare(&oracle, &OutputOptions::default(), plan, input, label);
        }
    }
}

/// Inputs that are no JPEG image or end in their header.
#[test]
fn header_errors_match_the_oracle() {
    let Some(oracle) = Oracle::locate("header_errors_match_the_oracle") else {
        return;
    };
    let data = image("testorig.jpg");
    let mut inputs = vec![
        b"not a JPEG image".to_vec(),
        vec![0xFF, 0xD8],
        vec![0xFF, 0xD8, 0xFF, 0xD9],
        data[..20].to_vec(),
        data[..200].to_vec(),
        data[..600].to_vec(),
    ];
    // An SOF with zero width.
    let mut zero = data.clone();
    let sof = zero.windows(2).position(|w| w == [0xFF, 0xC0]).unwrap();
    zero[sof + 7] = 0;
    zero[sof + 8] = 0;
    inputs.push(zero);
    for (index, input) in inputs.iter().enumerate() {
        let fields = compare(
            &oracle,
            &OutputOptions::default(),
            &[],
            input,
            &format!("input {index}"),
        );
        assert!(
            fields.error.is_some() || fields.warnings.unwrap().1 > 0,
            "input {index}"
        );
    }
}

/// Truncated and damaged streams continue with warnings as libjpeg does.
#[test]
fn damaged_streams_match_the_oracle() {
    let Some(oracle) = Oracle::locate("damaged_streams_match_the_oracle") else {
        return;
    };
    let plan = [Step::Skip(30), Step::Read(1), Step::Skip(60)];
    for (label, data) in eight_bit_variants(&oracle) {
        let mut damaged: Vec<Vec<u8>> = entropy_positions(&data, 4)
            .into_iter()
            .map(|cut| data[..cut].to_vec())
            .collect();
        for position in entropy_positions(&data, 3) {
            let mut mutated = data.clone();
            mutated[position] ^= 0x55;
            damaged.push(mutated);
        }
        // Block smoothing changes only coefficients that a cut progressive
        // file lacks.
        let nosmooth = with(|o| o.block_smoothing = false);
        for input in &damaged {
            compare(&oracle, &OutputOptions::default(), &[], input, &label);
            compare(&oracle, &OutputOptions::default(), &plan, input, &label);
            compare(&oracle, &nosmooth, &[], input, &label);
        }
    }
}

/// With `fail_on_warning`, a decode stops at the first warning the oracle
/// emits, after the rows the oracle reads before it.
#[test]
fn fail_on_warning_stops_at_the_first_warning() {
    let Some(oracle) = Oracle::locate("fail_on_warning_stops_at_the_first_warning") else {
        return;
    };
    let options = DecoderOptions {
        fail_on_warning: true,
        ..DecoderOptions::default()
    };
    let output = OutputOptions::default();
    let mut stopped = 0;
    for (label, data) in eight_bit_variants(&oracle) {
        let mut inputs = vec![data.clone()];
        inputs.extend(
            entropy_positions(&data, 3)
                .into_iter()
                .map(|cut| data[..cut].to_vec()),
        );
        for position in entropy_positions(&data, 3) {
            let mut mutated = data.clone();
            mutated[position] ^= 0x55;
            inputs.push(mutated);
        }
        for input in &inputs {
            let expected = run_oracle(&oracle, &output, &[], None, input);
            let actual = run_port(&options, &output, &[], input);
            let (warnings, _) = expected.warnings.clone().unwrap();
            match warnings.first() {
                None => assert_same(actual, expected, &label),
                Some(first) => {
                    stopped += 1;
                    assert_eq!(actual.error.as_ref(), Some(first), "{label}");
                    assert!(
                        expected.rows.starts_with(&actual.rows),
                        "{label}: the port's rows are no prefix of the oracle's"
                    );
                    let warnings = actual.warnings.map(|(w, count)| (w.len(), count));
                    assert!(
                        matches!(warnings, None | Some((0, 1))),
                        "{label}: {warnings:?}"
                    );
                }
            }
        }
    }
    assert!(stopped > 0);
}

/// `DecoderOptions::max_memory` limits the whole-image buffer of a
/// progressive decode as `JPEGMEM` does in the oracle.
#[test]
fn memory_limit_matches_jpegmem() {
    let Some(oracle) = Oracle::locate("memory_limit_matches_jpegmem") else {
        return;
    };
    let progressive = encode(&oracle, &["-progressive"]);
    let baseline = image("testorig.jpg");
    let output = OutputOptions::default();
    let mut failed = 0;
    for (label, data) in [("-progressive", &progressive), ("testorig.jpg", &baseline)] {
        for thousands in [1_usize, 50, 150, 200, 1000] {
            let jpegmem = thousands.to_string();
            let expected = run_oracle(&oracle, &output, &[], Some(&jpegmem), data);
            let options = DecoderOptions {
                max_memory: Some(thousands * 1000),
                ..DecoderOptions::default()
            };
            let actual = run_port(&options, &output, &[], data);
            if expected.error.is_some() {
                failed += 1;
            }
            assert_same(actual, expected, &format!("{label} JPEGMEM={jpegmem}"));
        }
    }
    assert!(failed > 0, "no limit was low enough to fail");
}

/// `data` with `segments` (marker, payload) inserted after SOI.
fn with_segments(data: &[u8], segments: &[(u8, &[u8])]) -> Vec<u8> {
    let mut all = parts(data);
    for (index, (marker, payload)) in segments.iter().enumerate() {
        all.insert(1 + index, inputs::Part::Segment(*marker, payload.to_vec()));
    }
    join(&all)
}

/// The EXIF data and the ICC profile equal the oracle's saved APP1 markers
/// and `jpeg_read_icc_profile`, with the same warnings.
#[test]
fn metadata_matches_the_oracle() {
    let Some(oracle) = Oracle::locate("metadata_matches_the_oracle") else {
        return;
    };
    let base = image("testorig.jpg");
    let icc1 = oracle::test_image("test1.icc");
    let icc3 = oracle::test_image("test3.icc");
    let tiff = b"MM\0*\0\0\0\x08\0\x01\x01\x12\0\x03\0\0\0\x01\0\x06\0\0\0\0\0\0";
    let exif = [&b"Exif\0\0"[..], tiff].concat();
    let xmp = b"http://ns.adobe.com/xap/1.0/\0<x:xmpmeta/>".to_vec();
    let with_icc = encode(&oracle, &["-icc", icc1.to_str().unwrap()]);
    let mut bogus_icc = encode(&oracle, &["-icc", icc3.to_str().unwrap()]);
    let at = bogus_icc
        .windows(12)
        .position(|w| w == b"ICC_PROFILE\0")
        .unwrap();
    bogus_icc[at + 12] = 0; /* sequence number 0 */
    let inputs = vec![
        ("plain", base.clone()),
        ("icc1", with_icc.clone()),
        ("icc3", encode(&oracle, &["-icc", icc3.to_str().unwrap()])),
        ("bogus icc", bogus_icc),
        ("exif", with_segments(&base, &[(0xE1, &exif)])),
        (
            "xmp then exif",
            with_segments(&with_icc, &[(0xE1, &xmp), (0xE1, &exif)]),
        ),
        ("xmp", with_segments(&base, &[(0xE1, &xmp)])),
        ("short exif", with_segments(&base, &[(0xE1, b"Exif\0")])),
        ("empty exif", with_segments(&base, &[(0xE1, b"Exif\0\0")])),
    ];
    for (label, data) in &inputs {
        let fields = oracle
            .probe("wp2", "header")
            .run(
                &["-save", "225", "65535", "-save", "226", "65535", "-icc"],
                data,
            )
            .success()
            .fields();
        assert_eq!(fields.get("error"), Some("0"), "{label}");
        let saved_exif = fields
            .0
            .iter()
            .filter(|(key, _)| key.starts_with("saved"))
            .find_map(|(_, value)| {
                let (head, hex) = value.split_once(":=").unwrap();
                let bytes: Vec<u8> = (0..hex.len())
                    .step_by(2)
                    .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).unwrap())
                    .collect();
                (head.starts_with("225,") && bytes.starts_with(b"Exif\0\0"))
                    .then(|| bytes[6..].to_vec())
            });
        let icc = fields.get("icc_data").map(|hex| {
            (0..hex.len())
                .step_by(2)
                .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).unwrap())
                .collect::<Vec<u8>>()
        });

        let mut decoder = Decoder::new(data).unwrap();
        assert_eq!(decoder.exif().map(<[u8]>::to_vec), saved_exif, "{label}");
        assert_eq!(decoder.icc_profile().unwrap(), icc, "{label}");
        let report = decoder.report();
        let warnings: Vec<_> = report.warnings.iter().map(message).collect();
        assert_eq!(warnings, warnings_of(&fields), "{label}");
        assert_eq!(
            report.warning_count,
            u64::try_from(fields.int("num_warnings")).unwrap()
        );
    }
    assert!(Decoder::new(&inputs[4].1).unwrap().exif() == Some(&tiff[..]));
    assert!(
        Decoder::new(&inputs[1].1)
            .unwrap()
            .icc_profile()
            .unwrap()
            .is_some()
    );

    let strict = DecoderOptions {
        fail_on_warning: true,
        ..DecoderOptions::default()
    };
    let mut decoder = Decoder::with_options(&inputs[3].1, &strict).unwrap();
    let Err(Error::Warning(warning)) = decoder.icc_profile() else {
        panic!("a bogus ICC profile must fail with fail_on_warning");
    };
    assert_eq!(warning.code, J_MESSAGE_CODE::JWRN_BOGUS_ICC);
}

/// The header and the output dimensions before the start equal those the
/// start reports.
#[test]
fn output_info_matches_the_start() {
    // The test images come from the upstream submodule, which only the
    // oracle jobs check out.
    if Oracle::locate("output_info_matches_the_start").is_none() {
        return;
    }
    let data = image("testorig.jpg");
    let mut decoder = Decoder::new(&data).unwrap();
    let header = *decoder.header();
    assert_eq!(
        (
            header.width,
            header.height,
            header.components,
            header.precision
        ),
        (227, 149, 3, 8)
    );
    assert_eq!(header.color_space, jpeg::ColorSpace::YCbCr);
    assert!(!header.progressive && !header.arithmetic);
    let output = with(|o| {
        o.scale = Scale { num: 3, denom: 8 };
        o.format = Some(PixelFormat::Rgba);
    });
    let info = decoder.output_info(&output).unwrap();
    let scanlines = decoder.start(&output).unwrap();
    assert_eq!(*scanlines.info(), info);
    assert_eq!((info.width, info.height, info.components), (86, 56, 4));
    assert_eq!(info.row_bytes(), 86 * 4);
}

/// Arguments the safe API rejects before calling libjpeg, and requests out
/// of the port's scope, which the oracle decodes (ADR 0005).
#[test]
fn rejected_requests_fail() {
    // The test images come from the upstream submodule, which only the
    // oracle jobs check out.
    if Oracle::locate("rejected_requests_fail").is_none() {
        return;
    }
    let data = image("testorig.jpg");
    let mut scanlines = Decoder::new(&data)
        .unwrap()
        .start(&OutputOptions::default())
        .unwrap();
    let row = scanlines.info().row_bytes();
    for length in [0, 1, row - 1, row + 1] {
        assert!(matches!(
            scanlines.read_rows(&mut vec![0; length]),
            Err(Error::InvalidArgument(_))
        ));
    }
    assert_eq!(scanlines.next_row(), 0);
    assert_eq!(scanlines.skip_rows(149).unwrap(), 149);
    assert_eq!(scanlines.read_rows(&mut vec![0; row]).unwrap(), 0);
    assert!(scanlines.report().warnings.is_empty());
    scanlines.finish().unwrap();

    let empty = Decoder::new(&[]).err().unwrap();
    let Error::Library(empty) = empty else {
        panic!("{empty:?}");
    };
    assert_eq!(empty.code, J_MESSAGE_CODE::JERR_INPUT_EMPTY);
    assert_eq!(empty.to_string(), "Empty input file");

    let Some(oracle) = Oracle::locate("rejected_requests_fail") else {
        return;
    };
    let twelve = encode(&oracle, &["-precision", "12"]);
    let decoder = Decoder::new(&twelve).unwrap();
    assert_eq!(decoder.header().precision, 12);
    let Err(Error::Library(message)) = decoder.start(&OutputOptions::default()) else {
        panic!("a 12-bit image must fail");
    };
    assert_eq!(message.code, J_MESSAGE_CODE::JERR_BAD_PRECISION);
}
