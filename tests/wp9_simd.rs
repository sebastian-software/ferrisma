//! WP9 tests of the SIMD configuration (ADR 0007).
//!
//! The Neon kernels branch on the width of a row: whole vectors of 16 or 8
//! columns, then a tail of single lanes, and the fancy upsamplers treat the
//! first and last columns apart. Upstream's test images are 227 columns
//! wide, so these tests encode crops of `testorig.ppm` in many widths and
//! sampling factors with the oracle's `cjpeg` and compare every decode, row
//! by row, with the WP6 probe `tools/oracle/probes/wp6/api.c`, for the
//! output formats, upsampling modes, IDCT methods and scales that select
//! different kernels. They run in every configuration: against the simd
//! oracle they check the Neon kernels (and, with `JSIMD_FORCENONE=1`, the C
//! routines of the SIMD build), against the reference oracle the portable
//! decoder at the same sizes.
//!
//! With `WITH_SIMD`, `kernels_are_dispatched` checks that the port selects
//! the kernels, or none of them with `JSIMD_FORCENONE=1`, so that a decode
//! that matches the oracle cannot have passed through the C routines alone.

#[path = "support/inputs.rs"]
mod inputs;
#[path = "support/oracle.rs"]
mod oracle;

use std::process::Command;

use jpeg::{DctMethod, Decoder, OutputOptions, PixelFormat, Scale};
use oracle::Oracle;

/// The probes' FNV-1a constants, as `api.c` defines them.
const FNV_OFFSET: u64 = 1_469_598_103_934_665_603;
const FNV_PRIME: u64 = 1_099_511_628_211;

/// The widths: around every multiple of 8 and 16 up to 66, and wider ones
/// with odd tails.
const WIDTHS: [u32; 22] = [
    1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 23, 24, 25, 31, 32, 33, 47, 48, 49, 65, 101,
];

/// The height: two and a half MCU rows of 2x2 sampling.
const HEIGHT: u32 = 37;

/// The `cjpeg` switches of each input kind.
const SAMPLINGS: [(&str, &[&str]); 5] = [
    ("4:4:4", &["-sample", "1x1"]),
    ("4:2:2", &["-sample", "2x1"]),
    ("4:2:0", &["-sample", "2x2"]),
    ("4:4:0", &["-sample", "1x2"]),
    ("gray", &["-grayscale"]),
];

fn row_hash(row: &[u8]) -> String {
    let mut hash = FNV_OFFSET;
    for &sample in row {
        hash = (hash ^ u64::from(sample)).wrapping_mul(FNV_PRIME);
    }
    format!("{hash:016x}")
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

/// The top-left `width` x `height` pixels of `testorig.ppm` as a PPM.
fn crop(width: u32, height: u32) -> Vec<u8> {
    let data = inputs::image("testorig.ppm");
    let header = b"P6\n227 149\n255\n";
    assert!(data.starts_with(header), "testorig.ppm has a new header");
    let pixels = &data[header.len()..];
    let mut out = format!("P6\n{width} {height}\n255\n").into_bytes();
    for y in 0..height as usize {
        let start = y * 227 * 3;
        out.extend_from_slice(&pixels[start..start + width as usize * 3]);
    }
    out
}

/// Encodes `ppm` with the oracle's `cjpeg` and `switches`.
fn encode(oracle: &Oracle, ppm: &[u8], switches: &[&str]) -> Vec<u8> {
    // One pair of files per call: tests run in parallel.
    static CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let call = CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let base = std::env::temp_dir().join(format!("libjpeg-rs-wp9-{}-{call}", std::process::id()));
    let input = base.with_extension("ppm");
    let output = base.with_extension("jpg");
    std::fs::write(&input, ppm).unwrap();
    let status = Command::new(oracle.tool("cjpeg"))
        .args(switches)
        .arg("-outfile")
        .arg(&output)
        .arg(&input)
        .status()
        .unwrap();
    assert!(status.success(), "cjpeg {switches:?}");
    let data = std::fs::read(&output).unwrap();
    let _ = std::fs::remove_file(&input);
    let _ = std::fs::remove_file(&output);
    data
}

/// The probe options for `output`.
fn probe_args(output: &OutputOptions) -> Vec<String> {
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
    args
}

/// The output dimensions and row hashes of the oracle's decode.
fn oracle_rows(oracle: &Oracle, output: &OutputOptions, data: &[u8]) -> (String, Vec<String>) {
    let fields = oracle
        .probe("wp6", "api")
        .run(&probe_args(output), data)
        .success()
        .fields();
    assert_eq!(fields.int("error"), 0, "{fields:?}");
    let dims: Vec<&str> = fields.get("dims").unwrap().split(',').collect();
    let rows = match fields.get("rows") {
        None | Some("") => Vec::new(),
        Some(rows) => rows.split(',').map(str::to_owned).collect(),
    };
    (format!("{},{},{}", dims[0], dims[1], dims[3]), rows)
}

/// The output dimensions and row hashes of the port's decode, one row per
/// read.
fn port_rows(output: &OutputOptions, data: &[u8]) -> (String, Vec<String>) {
    let mut scanlines = Decoder::new(data).unwrap().start(output).unwrap();
    let info = *scanlines.info();
    let mut row = vec![0; info.row_bytes()];
    let mut rows = Vec::new();
    while scanlines.read_rows(&mut row).unwrap() == 1 {
        rows.push(row_hash(&row));
    }
    scanlines.finish().unwrap();
    (
        format!("{},{},{}", info.width, info.height, info.components),
        rows,
    )
}

/// The output options that select different kernels for `sampling`.
fn modes(sampling: &str) -> Vec<OutputOptions> {
    let base = OutputOptions::default();
    let with = |change: &dyn Fn(&mut OutputOptions)| {
        let mut output = base;
        change(&mut output);
        output
    };
    let mut modes = vec![
        base,
        with(&|o| o.dct_method = DctMethod::IntFast),
        with(&|o| o.scale = Scale { num: 1, denom: 2 }),
        with(&|o| o.scale = Scale { num: 1, denom: 4 }),
        with(&|o| o.scale = Scale { num: 1, denom: 8 }),
    ];
    if sampling != "gray" {
        for format in [
            PixelFormat::Rgbx,
            PixelFormat::Bgr,
            PixelFormat::Bgra,
            PixelFormat::Xbgr,
            PixelFormat::Argb,
            PixelFormat::YCbCr,
        ] {
            modes.push(with(&|o| o.format = Some(format)));
        }
        // Merged upsampling for RGB output; plain upsampling elsewhere.
        modes.push(with(&|o| o.fancy_upsampling = false));
        modes.push(with(&|o| {
            o.fancy_upsampling = false;
            o.format = Some(PixelFormat::Xrgb);
        }));
        modes.push(with(&|o| {
            o.fancy_upsampling = false;
            o.format = Some(PixelFormat::YCbCr);
        }));
    }
    modes
}

#[test]
fn widths_and_samplings_match_the_oracle() {
    let Some(oracle) = Oracle::locate("widths_and_samplings_match_the_oracle") else {
        return;
    };
    let mut cases = 0;
    for (sampling, switches) in SAMPLINGS {
        for width in WIDTHS {
            let data = encode(&oracle, &crop(width, HEIGHT), switches);
            for output in modes(sampling) {
                let expected = oracle_rows(&oracle, &output, &data);
                let actual = port_rows(&output, &data);
                assert_eq!(actual, expected, "{sampling} width {width} {output:?}");
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 4 * WIDTHS.len() * 14 + WIDTHS.len() * 5);
}

#[cfg(with_simd)]
#[test]
fn kernels_are_dispatched() {
    use jpeg::jdapimin::{jpeg_create_decompress, jpeg_destroy_decompress, jpeg_read_header};
    use jpeg::jdapistd::jpeg_start_decompress;
    use jpeg::jdatasrc::jpeg_mem_src;
    use jpeg::jerror::jpeg_std_error;
    use jpeg::jpeglib::{jpeg_decompress_struct, jpeg_error_mgr};
    use jpeg::jsimd::JSIMD_NEON;

    let forced_none = std::env::var_os("JSIMD_FORCENONE").is_some_and(|value| value == "1");
    // testorig.jpg is 4:2:0 YCbCr: fancy h2v2 upsampling, YCbCr-to-RGB
    // conversion and the islow IDCT; at 1/2 the 4x4 IDCT too.
    let data = inputs::image("testorig.jpg");
    for scale_denom in [1, 2] {
        let mut err = Box::new(jpeg_error_mgr::default());
        jpeg_std_error(&mut err);
        let mut cinfo = jpeg_decompress_struct::new(err);
        jpeg_create_decompress(&mut cinfo).unwrap();
        jpeg_mem_src(&mut cinfo, &data).unwrap();
        jpeg_read_header(&mut cinfo, true).unwrap();
        cinfo.scale_denom = scale_denom;
        assert!(jpeg_start_decompress(&mut cinfo).unwrap());

        let support = cinfo.master().pub_.simd_support;
        // At 1/2 the chroma components get the 8x8 IDCT, which leaves
        // nothing to upsample (jdmaster.c's jpeg_core_output_dimensions),
        // and the luma component the 4x4 IDCT.
        let slots = if scale_denom == 1 {
            [
                cinfo.upsample().h2v2_upsample_simd.is_some(),
                cinfo.cconvert().color_convert_simd.is_some(),
                cinfo.idct().idct_simd.is_some(),
            ]
        } else {
            [
                cinfo.idct().idct_4x4_simd.is_some(),
                cinfo.cconvert().color_convert_simd.is_some(),
                cinfo.idct().idct_simd.is_some(),
            ]
        };
        if forced_none {
            assert_eq!(support, 0);
            assert_eq!(slots, [false; 3], "scale 1/{scale_denom}");
        } else {
            assert_eq!(support, JSIMD_NEON);
            assert_eq!(slots, [true; 3], "scale 1/{scale_denom}");
        }
        jpeg_destroy_decompress(&mut cinfo).unwrap();
    }
}
