//! WP6 differential tests of master control and the decompression API:
//! `jdmaster.c`, `jdapistd.c` and `jpeg_finish_decompress` of `jdapimin.c`.
//!
//! The probe `tools/oracle/probes/wp6/api.c` decompresses through the
//! public functions as an application does: `jpeg_read_header`,
//! `jpeg_start_decompress`, an output pass of `jpeg_read_scanlines`,
//! `jpeg_skip_scanlines` and `jpeg_crop_scanline` calls that a plan gives,
//! or of `jpeg_read_raw_data`, or the output passes of buffered-image mode,
//! and `jpeg_finish_decompress`, repeating a call when the source suspends.
//! It prints every message, the output dimensions, a hash of every output
//! row, every count the calls returned, the progress monitor's calls, the
//! final state, the memory pools and the error. Its rows equal `djpeg`'s
//! output for the upstream test images, cropped, skipped and scaled.
//! `support/wp6.rs` runs the same sequence on the port; each case compares
//! both outputs exactly.
//!
//! The inputs are upstream's test images, the 8-bit encoder variants of the
//! earlier packages, four-component images, and streams cut and altered
//! from them. 12-bit images are out of scope (ADR 0005): the oracle's
//! `master_selection` starts its 12-bit modules, while the port fails with
//! `JERR_BAD_PRECISION`, so `out_of_scope_requests_fail` checks the port
//! alone, as it does for color quantization.

#[path = "support/inputs.rs"]
mod inputs;
#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp2.rs"]
mod wp2;
#[path = "support/wp4.rs"]
mod wp4;
#[path = "support/wp6.rs"]
mod wp6;

use inputs::{eight_bit_variants, encode, entropy_positions, four_component, image, variants};
use oracle::{Fields, Oracle};

/// Runs `args` on `data` with the probe and the port and compares the output.
fn compare(oracle: &Oracle, args: &[&str], data: &[u8], label: &str) -> Fields {
    let expected = oracle
        .probe("wp6", "api")
        .run(args, data)
        .success()
        .fields();
    let actual = wp6::run_rust(args, data);
    if actual != expected {
        let first = actual
            .0
            .iter()
            .zip(&expected.0)
            .position(|(a, e)| a != e)
            .unwrap_or(actual.0.len().min(expected.0.len()));
        let (port, c) = (actual.0.get(first), expected.0.get(first));
        if let (Some((key, port)), Some((_, c))) = (port, c)
            && key == "rows"
        {
            let row = port.split(',').zip(c.split(',')).position(|(a, e)| a != e);
            panic!("{label} {args:?}: output row {row:?} differs");
        }
        panic!(
            "{label} {args:?}: outputs differ at field {first}\n  port:   {port:?}\n  oracle: {c:?}"
        );
    }
    expected
}

/// Compares `args` and requires a clean decode.
fn compare_ok(oracle: &Oracle, args: &[&str], data: &[u8], label: &str) -> Fields {
    let fields = compare(oracle, args, data, label);
    assert_eq!(fields.get("error"), Some("0"), "{label} {args:?}");
    fields
}

const TEST_IMAGES: [&str; 3] = ["testorig.jpg", "testimgari.jpg", "testimgint.jpg"];

/// The output modes compared for every input.
const MODES: &[&[&str]] = &[
    &[],
    &["-nofancy"],
    &["-dct", "ifast", "-rows", "3"],
    &["-dct", "float", "-nofancy", "-rows", "2"],
    &["-scale", "1", "8"],
    &["-scale", "3", "8", "-calc"],
    &["-scale", "13", "8", "-nofancy", "-rows", "16"],
    &["-out", "1", "-progress"],
    &["-out", "7", "-nosmooth", "-extra"],
    &["-progress", "-twice"],
];

#[test]
fn test_images_match_the_oracle() {
    let Some(oracle) = Oracle::locate("test_images_match_the_oracle") else {
        return;
    };
    for name in TEST_IMAGES {
        for args in MODES {
            compare_ok(&oracle, args, &image(name), name);
        }
    }
}

#[test]
fn encoder_variants_match_the_oracle() {
    let Some(oracle) = Oracle::locate("encoder_variants_match_the_oracle") else {
        return;
    };
    for (label, data) in eight_bit_variants(&oracle) {
        for args in MODES {
            compare_ok(&oracle, args, &data, &label);
        }
    }
}

/// Crops of every alignment and width, with fancy and plain (merged)
/// upsampling and at several scales, including widths whose chroma becomes
/// narrower than two samples, which reinitializes the upsampler.
#[test]
fn crops_match_the_oracle() {
    let Some(oracle) = Oracle::locate("crops_match_the_oracle") else {
        return;
    };
    let mut inputs: Vec<(String, Vec<u8>)> = TEST_IMAGES
        .iter()
        .map(|name| ((*name).to_owned(), image(name)))
        .collect();
    for switches in [
        &["-sample", "1x1"][..],
        &["-sample", "2x1"],
        &["-sample", "4x1,1x1,1x1"],
        &["-sample", "1x2", "-progressive"],
        &["-grayscale"],
        &["-progressive"],
    ] {
        inputs.push((switches.join(" "), encode(&oracle, switches)));
    }
    for (label, data) in &inputs {
        for scale in [
            &[][..],
            &["-scale", "1", "2"],
            &["-scale", "3", "8"],
            &["-scale", "2", "1"],
        ] {
            for fancy in [&[][..], &["-nofancy"]] {
                for (x, w) in [
                    (0, 1),
                    (0, 16),
                    (1, 1),
                    (7, 9),
                    (16, 40),
                    (40, 73),
                    (100, 2),
                    (213, 14),
                    (0, 227),
                    (226, 1),
                ] {
                    let crop = format!("c{x}:{w}");
                    let mut args = vec!["-plan", &crop];
                    args.extend_from_slice(scale);
                    args.extend_from_slice(fancy);
                    let fields = compare(&oracle, &args, data, label);
                    assert!(
                        fields.get("error") == Some("0") || fields.get("error") == Some("70"),
                        "{label} {args:?}: {:?}",
                        fields.get("error_message")
                    );
                }
            }
        }
    }
}

/// Skips at every position relative to the iMCU rows: with context rows
/// (fancy upsampling of vertically subsampled chroma), with the simple main
/// controller, with the merged upsampler and its spare row, for single- and
/// multi-scan images, combined with crops and scaling.
#[test]
fn skips_match_the_oracle() {
    let Some(oracle) = Oracle::locate("skips_match_the_oracle") else {
        return;
    };
    let mut inputs: Vec<(String, Vec<u8>)> = TEST_IMAGES
        .iter()
        .map(|name| ((*name).to_owned(), image(name)))
        .collect();
    for switches in [
        &["-sample", "1x1"][..],
        &["-sample", "2x1"],
        &["-sample", "1x2"],
        &["-sample", "4x1,1x1,1x1", "-restart", "1"],
        &["-grayscale"],
        &["-progressive"],
        &["-sample", "1x2", "-progressive"],
    ] {
        inputs.push((switches.join(" "), encode(&oracle, switches)));
    }
    let plans = [
        "s1",
        "s2",
        "s7",
        "s8",
        "s9",
        "s15",
        "s16",
        "s17",
        "s31",
        "s32",
        "s33",
        "s100",
        "s148",
        "s149",
        "s500",
        "s0",
        "r1,s1",
        "r3,s14",
        "r7,s1,r1,s8",
        "r15,s2",
        "r16,s16,r1,s31",
        "r17,s40,r2,s3",
        "r30,s0,s5,r9,s60",
        "r140,s20",
        "c40:73,s21,r10,s50",
        "c100:2,s9,r3,s17",
    ];
    for (label, data) in &inputs {
        for plan in plans {
            for extra in [
                &[][..],
                &["-nofancy"],
                &["-scale", "1", "2"],
                &["-nofancy", "-rows", "2"],
            ] {
                let mut args = vec!["-plan", plan];
                args.extend_from_slice(extra);
                compare_ok(&oracle, &args, data, label);
            }
        }
    }
}

/// Raw data output, including a buffer that is too short.
#[test]
fn raw_data_matches_the_oracle() {
    let Some(oracle) = Oracle::locate("raw_data_matches_the_oracle") else {
        return;
    };
    for (label, data) in eight_bit_variants(&oracle) {
        for args in [
            &["-raw"][..],
            &["-raw", "-scale", "1", "2"],
            &["-raw", "-scale", "1", "8", "-progress"],
            &["-raw", "-rawlines", "100"],
        ] {
            compare_ok(&oracle, args, &data, &label);
        }
        let fields = compare(&oracle, &["-raw", "-rawlines", "1"], &data, &label);
        assert_eq!(fields.get("error"), Some("23"), "{label}: JERR_BUFFER_SIZE");
    }
}

/// Buffered-image mode: every output pass, a limited number of passes,
/// scan numbers out of range, skips and crops within a pass, and the
/// progress monitor.
#[test]
fn buffered_image_matches_the_oracle() {
    let Some(oracle) = Oracle::locate("buffered_image_matches_the_oracle") else {
        return;
    };
    for (label, data) in eight_bit_variants(&oracle) {
        for args in [
            &["-buffered"][..],
            &["-buffered", "-progress"],
            &["-buffered", "-passes", "1"],
            &["-buffered", "-passes", "2", "-scan", "0"],
            &["-buffered", "-scan", "99", "-nofancy"],
            &["-buffered", "-scan", "-3", "-plan", "s10,r5,s30"],
            &["-buffered", "-plan", "c0:40,r20", "-scale", "1", "2"],
            &["-buffered", "-nosmooth", "-rows", "4", "-twice"],
        ] {
            compare_ok(&oracle, args, &data, &label);
        }
    }
}

/// Calls at the wrong time and invalid requests, with the error each
/// raises.
#[test]
fn errors_match_the_oracle() {
    let Some(oracle) = Oracle::locate("errors_match_the_oracle") else {
        return;
    };
    let data = image("testorig.jpg");
    let progressive = encode(&oracle, &["-progressive"]);
    for (args, input, code) in [
        (&["-plan", "d"][..], &data, 20),         /* JERR_BAD_STATE */
        (&["-plan", "n"], &data, 20),             /* JERR_BAD_STATE */
        (&["-plan", "r1,c0:10"], &data, 20),      /* JERR_BAD_STATE */
        (&["-plan", "C"], &data, 124),            /* JERR_BAD_CROP_SPEC */
        (&["-plan", "c0:0"], &data, 70),          /* JERR_WIDTH_OVERFLOW */
        (&["-plan", "c200:28"], &data, 70),       /* JERR_WIDTH_OVERFLOW */
        (&["-plan", "c4294967295:2"], &data, 70), /* JERR_WIDTH_OVERFLOW */
        (&["-plan", "r10,e"], &data, 67),         /* JERR_TOO_LITTLE_DATA */
        (&["-plan", "e"], &data, 67),             /* JERR_TOO_LITTLE_DATA */
        (&["-raw", "-plan", "c0:10"], &data, 47), /* JERR_NOTIMPL */
        (&["-raw", "-plan", "r1"], &data, 20),    /* JERR_BAD_STATE */
        (&["-raw", "-plan", "s1"], &data, 20),    /* JERR_BAD_STATE */
        (&["-raw", "-plan", "P"], &data, 15),     /* JERR_BAD_PRECISION */
        (&["-raw", "-extra"], &data, 0),          /* JWRN_TOO_MUCH_DATA */
        (&["-quantize", "-raw"], &data, 47),      /* JERR_NOTIMPL */
        (&["-plan", "S"], &data, 20),             /* JERR_BAD_STATE */
        (&["-plan", "O"], &data, 20),             /* JERR_BAD_STATE */
        (&["-plan", "F"], &data, 20),             /* JERR_BAD_STATE */
        (&["-plan", "P,r1"], &data, 15),          /* JERR_BAD_PRECISION */
        (&["-plan", "P,s3"], &data, 15),          /* JERR_BAD_PRECISION */
        (&["-plan", "P,c0:8"], &data, 15),        /* JERR_BAD_PRECISION */
        (&["-precision", "9"], &data, 15),        /* JERR_BAD_PRECISION */
        (&["-scale", "0", "0"], &data, 0),
        (&["-scale", "4294967295", "1"], &data, 0),
        (&["-scale", "1", "4294967295"], &data, 0),
        (&["-ccir601"], &data, 25),             /* JERR_CCIR601_NOTIMPL */
        (&["-ccir601", "-nofancy"], &data, 25), /* JERR_CCIR601_NOTIMPL */
        (&["-out", "2", "-plan", "c8:8"], &data, 0),
        (&["-buffered", "-newcolormap"], &progressive, 46), /* JERR_MODE_CHANGE */
        (&["-buffered", "-plan", "e"], &progressive, 20),   /* JERR_BAD_STATE */
        (&["-buffered", "-plan", "n"], &progressive, 20),   /* JERR_BAD_STATE */
        (&["-buffered", "-plan", "r10,F"], &progressive, 20), /* JERR_BAD_STATE */
        (&["-buffered", "-plan", "O"], &progressive, 20),   /* JERR_BAD_STATE */
        (&["-buffered", "-absorb", "-scan", "99"], &progressive, 0),
        (
            &["-buffered", "-absorb", "-plan", "s40,r3", "-progress"],
            &progressive,
            0,
        ),
        (&["-buffered", "-raw"], &progressive, 20), /* JERR_BAD_STATE: raw reads outside an output pass */
        (&["-raw", "-buffered", "-passes", "1"], &progressive, 20), /* JERR_BAD_STATE */
    ] {
        let fields = compare(&oracle, args, input, "error case");
        assert_eq!(
            fields.int("error"),
            code,
            "{args:?}: {:?}",
            fields.get("error_message")
        );
    }
}

/// Every scale from 1/8 to 16/8, with crops and skips.
#[test]
fn scales_match_the_oracle() {
    let Some(oracle) = Oracle::locate("scales_match_the_oracle") else {
        return;
    };
    let mut inputs: Vec<(String, Vec<u8>)> =
        vec![("testorig.jpg".to_owned(), image("testorig.jpg"))];
    for switches in [
        &["-sample", "2x1"][..],
        &["-sample", "1x2", "-progressive"],
        &["-sample", "4x1,1x1,1x1"],
        &["-sample", "3x2", "-arithmetic"],
        &["-grayscale"],
    ] {
        inputs.push((switches.join(" "), encode(&oracle, switches)));
    }
    for (label, data) in &inputs {
        for num in 1..=16 {
            let num = num.to_string();
            for args in [
                &["-scale", &num, "8"][..],
                &["-scale", &num, "8", "-nofancy", "-calc"],
                &["-scale", &num, "8", "-plan", "c3:20,s5,r9,s7"],
                &["-scale", &num, "8", "-nofancy", "-raw"],
            ] {
                compare_ok(&oracle, args, data, label);
            }
        }
    }
}

/// A sequential file with one scan per component: the multi-scan paths of
/// skipping and the progress estimate for a nonprogressive multi-scan file.
#[test]
fn sequential_multiscan_matches_the_oracle() {
    let Some(oracle) = Oracle::locate("sequential_multiscan_matches_the_oracle") else {
        return;
    };
    let script = std::env::temp_dir().join(format!("libjpeg-rs-wp6-{}.scan", std::process::id()));
    std::fs::write(&script, "0;\n1;\n2;\n").unwrap();
    let data = encode(&oracle, &["-scans", script.to_str().unwrap()]);
    let _ = std::fs::remove_file(&script);
    for args in [
        &["-progress"][..],
        &["-progress", "-plan", "s20,r3,s50"],
        &["-nofancy", "-plan", "r9,s100"],
        &["-buffered", "-progress"],
        &["-buffered", "-absorb", "-plan", "s33,r2"],
        &["-raw", "-progress"],
        &["-suspend", "100", "-progress", "-plan", "s20,r3,s50"],
    ] {
        compare_ok(&oracle, args, &data, "sequential multiscan");
    }
}

/// Skips that end shortly before the last row, after reads of every phase
/// of an iMCU row, so that the input side reaches the last iMCU row while
/// skipping.
#[test]
fn skips_near_the_end_match_the_oracle() {
    let Some(oracle) = Oracle::locate("skips_near_the_end_match_the_oracle") else {
        return;
    };
    let mut inputs: Vec<(String, Vec<u8>)> =
        vec![("testorig.jpg".to_owned(), image("testorig.jpg"))];
    for switches in [
        &["-sample", "1x1"][..],
        &["-sample", "1x2"],
        &["-progressive"],
    ] {
        inputs.push((switches.join(" "), encode(&oracle, switches)));
    }
    for (label, data) in &inputs {
        let height: u32 = compare_ok(&oracle, &[], data, label)
            .get("dims")
            .unwrap()
            .split(',')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        for read in [0, 1, 2, 7, 8, 15, 16, 17, 31, 32, 33] {
            for left in 1..=24 {
                let plan = format!("r{read},s{}", height - read - left);
                for fancy in [&[][..], &["-nofancy"]] {
                    let mut args = vec!["-plan", &plan];
                    args.extend_from_slice(fancy);
                    compare_ok(&oracle, &args, data, label);
                }
            }
        }
    }
}

/// Suspending input through every API call that reads data.
#[test]
fn suspension_matches_the_oracle() {
    let Some(oracle) = Oracle::locate("suspension_matches_the_oracle") else {
        return;
    };
    let mut inputs: Vec<(String, Vec<u8>)> = TEST_IMAGES
        .iter()
        .map(|name| ((*name).to_owned(), image(name)))
        .collect();
    for switches in [
        &["-progressive"][..],
        &["-progressive", "-arithmetic"],
        &["-sample", "1x2"],
    ] {
        inputs.push((switches.join(" "), encode(&oracle, switches)));
    }
    for (label, data) in inputs {
        for step in ["1", "7", "64", "1000"] {
            for args in [
                &["-suspend", step][..],
                &["-suspend", step, "-nofancy", "-rows", "2", "-progress"],
                &["-suspend", step, "-plan", "r5,s20,r3,s40"],
                &["-suspend", step, "-raw"],
                &["-suspend", step, "-buffered", "-progress"],
            ] {
                let mut args = args.to_vec();
                args.push("-twice");
                compare(&oracle, &args, &data, &label);
            }
        }
    }
}

/// Four-component images and the output color spaces with crops and skips.
#[test]
fn color_spaces_match_the_oracle() {
    let Some(oracle) = Oracle::locate("color_spaces_match_the_oracle") else {
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
    for (label, data) in inputs {
        for space in 0..=15 {
            let space = space.to_string();
            for args in [
                &["-out", &space][..],
                &["-out", &space, "-nofancy", "-plan", "c17:50,s9,r30"],
            ] {
                compare(&oracle, args, &data, &label);
            }
        }
    }
}

/// Truncated and damaged streams through the API, with skips.
#[test]
fn damaged_streams_match_the_oracle() {
    let Some(oracle) = Oracle::locate("damaged_streams_match_the_oracle") else {
        return;
    };
    for (label, data) in eight_bit_variants(&oracle) {
        for cut in entropy_positions(&data, 5) {
            compare(&oracle, &[], &data[..cut], &label);
            compare(
                &oracle,
                &["-plan", "s30,r5,s60", "-progress"],
                &data[..cut],
                &label,
            );
        }
        for position in entropy_positions(&data, 3) {
            let mut mutated = data.clone();
            mutated[position] ^= 0x55;
            compare(&oracle, &[], &mutated, &label);
            compare(
                &oracle,
                &["-nofancy", "-plan", "s17,r1,s50"],
                &mutated,
                &label,
            );
            compare(&oracle, &["-buffered"], &mutated, &label);
        }
    }
}

/// Requests outside the port's scope fail where the oracle decodes:
/// 12-bit images and a 12-bit output precision (`JERR_BAD_PRECISION`),
/// color quantization and lossless images (`JERR_NOT_COMPILED`), as ADR
/// 0005 says.
#[test]
fn out_of_scope_requests_fail() {
    let Some(oracle) = Oracle::locate("out_of_scope_requests_fail") else {
        return;
    };
    let data = image("testorig.jpg");
    for (args, code) in [
        (&["-precision", "12"][..], 15),
        (&["-quantize"], 48),
        (&["-quantize", "-twopass"], 48),
        (&["-quantize", "-out", "1"], 48),
        (&["-quantize", "-buffered"], 48),
    ] {
        let fields = wp6::run_rust(args, &data);
        assert_eq!(fields.int("error"), code, "{args:?}");
    }
    let lossless = encode(&oracle, &["-lossless", "1"]);
    let fields = wp6::run_rust(&[], &lossless);
    assert_eq!(
        fields.int("error"),
        48,
        "lossless: {:?}",
        fields.get("error_message")
    );
    for (label, data) in variants(&oracle)
        .into_iter()
        .filter(|(label, _)| label.contains("-precision 12"))
    {
        let fields = wp6::run_rust(&[], &data);
        assert_eq!(fields.int("error"), 15, "{label}");
    }
}
