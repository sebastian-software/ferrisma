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
//! The crops come from two images: `testorig.ppm`, a photograph, and a
//! pseudo-random image whose YCbCr values reach the saturation of the color
//! conversions and the extremes of the upsamplers' arithmetic.
//!
//! With `WITH_SIMD`, `kernels_are_dispatched` checks that the port selects
//! the kernels of the instruction set that `LIBJPEG_EXPECT_SIMD` names
//! (`neon`, `avx2`, `sse2` or `none`; `tools/check.py` sets it for each
//! mode, ADR 0008), so that a decode that matches the oracle cannot have
//! passed through the C routines alone. Without the variable it expects the
//! best instruction set that the CPU reports, or none with
//! `JSIMD_FORCENONE=1`.

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

/// The width of the source images.
const SOURCE_WIDTH: usize = 227;

/// The source images: name and RGB pixels, `SOURCE_WIDTH` columns wide and
/// at least `HEIGHT` rows high.
fn sources() -> [(&'static str, Vec<u8>); 2] {
    let data = inputs::image("testorig.ppm");
    let header = b"P6\n227 149\n255\n";
    assert!(data.starts_with(header), "testorig.ppm has a new header");
    [
        ("testorig", data[header.len()..].to_vec()),
        ("noise", noise(SOURCE_WIDTH * HEIGHT as usize * 3)),
    ]
}

/// `len` pseudo-random bytes (xorshift64): random RGB pixels, whose YCbCr
/// values and differences between neighbors cover the whole range.
fn noise(len: usize) -> Vec<u8> {
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    (0..len)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state.to_be_bytes()[0]
        })
        .collect()
}

/// The top-left `width` x `height` pixels of `pixels` as a PPM.
fn crop(pixels: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut out = format!("P6\n{width} {height}\n255\n").into_bytes();
    for y in 0..height as usize {
        let start = y * SOURCE_WIDTH * 3;
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
    for (source, pixels) in sources() {
        for (sampling, switches) in SAMPLINGS {
            for width in WIDTHS {
                let data = encode(&oracle, &crop(&pixels, width, HEIGHT), switches);
                for output in modes(sampling) {
                    let expected = oracle_rows(&oracle, &output, &data);
                    let actual = port_rows(&output, &data);
                    assert_eq!(
                        actual, expected,
                        "{source} {sampling} width {width} {output:?}"
                    );
                    cases += 1;
                }
            }
        }
    }
    assert_eq!(cases, 2 * (4 * WIDTHS.len() * 14 + WIDTHS.len() * 5));
}

/// The instruction set whose kernels the port must select (ADR 0008,
/// decision 6).
#[cfg(with_simd)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Expected {
    Neon,
    Avx2,
    Sse2,
    None,
}

#[cfg(with_simd)]
impl Expected {
    /// `LIBJPEG_EXPECT_SIMD`, or the best instruction set that the CPU and
    /// the `JSIMD_FORCE*` variables leave.
    fn from_environment() -> Expected {
        let forced = |name| std::env::var_os(name).is_some_and(|value| value == "1");
        match std::env::var("LIBJPEG_EXPECT_SIMD").as_deref() {
            Ok("neon") => Expected::Neon,
            Ok("avx2") => Expected::Avx2,
            Ok("sse2") => Expected::Sse2,
            Ok("none") => Expected::None,
            Ok(other) => panic!("LIBJPEG_EXPECT_SIMD={other}: expected neon, avx2, sse2 or none"),
            Err(_) if forced("JSIMD_FORCENONE") => Expected::None,
            Err(_) if cfg!(target_arch = "aarch64") => Expected::Neon,
            Err(_) if forced("JSIMD_FORCESSE2") => Expected::Sse2,
            Err(_) if jpeg::jsimdcpu::jpeg_simd_cpu_support() & jpeg::jsimd::JSIMD_AVX2 != 0 => {
                Expected::Avx2
            }
            Err(_) => Expected::Sse2,
        }
    }

    /// The `simd_support` mask that `init_simd` stores.
    fn support(self) -> u32 {
        use jpeg::jsimd::{JSIMD_AVX2, JSIMD_NEON, JSIMD_SSE, JSIMD_SSE2};
        match self {
            Expected::Neon => JSIMD_NEON,
            Expected::Avx2 => JSIMD_SSE | JSIMD_SSE2 | JSIMD_AVX2,
            Expected::Sse2 => JSIMD_SSE | JSIMD_SSE2,
            Expected::None => 0,
        }
    }

    /// The instruction set that `jsimd_set_<kernel>` returns: upstream has
    /// AVX2 kernels only for some entry points, no float IDCT for Neon and
    /// no h1v2 fancy upsampling for x86-64.
    fn kernel(self, kernel: &str) -> u32 {
        use jpeg::jsimd::{JSIMD_AVX2, JSIMD_NEON, JSIMD_NONE, JSIMD_SSE2};
        const AVX2: &[&str] = &[
            "ycc_rgb",
            "h2v1_upsample",
            "h2v2_upsample",
            "h2v1_fancy_upsample",
            "h2v2_fancy_upsample",
            "h2v1_merged_upsample",
            "h2v2_merged_upsample",
            "idct_islow",
        ];
        match self {
            Expected::Neon if kernel == "idct_float" => JSIMD_NONE,
            Expected::Neon => JSIMD_NEON,
            Expected::Avx2 | Expected::Sse2 if kernel == "h1v2_fancy_upsample" => JSIMD_NONE,
            Expected::Avx2 if AVX2.contains(&kernel) => JSIMD_AVX2,
            Expected::Avx2 | Expected::Sse2 => JSIMD_SSE2,
            Expected::None => JSIMD_NONE,
        }
    }
}

#[cfg(with_simd)]
#[test]
fn kernels_are_dispatched() {
    use jpeg::jdapimin::{jpeg_create_decompress, jpeg_destroy_decompress, jpeg_read_header};
    use jpeg::jdapistd::jpeg_start_decompress;
    use jpeg::jdatasrc::jpeg_mem_src;
    use jpeg::jerror::jpeg_std_error;
    use jpeg::jpeglib::{jpeg_decompress_struct, jpeg_error_mgr};
    use jpeg::jsimd::{
        jsimd_set_h1v2_fancy_upsample, jsimd_set_h2v1_fancy_upsample,
        jsimd_set_h2v1_merged_upsample, jsimd_set_h2v1_upsample, jsimd_set_h2v2_fancy_upsample,
        jsimd_set_h2v2_merged_upsample, jsimd_set_h2v2_upsample, jsimd_set_idct_2x2,
        jsimd_set_idct_4x4, jsimd_set_idct_float, jsimd_set_idct_ifast, jsimd_set_idct_islow,
        jsimd_set_ycc_rgb,
    };

    // The test image comes from the upstream checkout, which the oracle
    // builds need, and `JSIMD_FORCE*` selects the oracle's mode too.
    if Oracle::locate("kernels_are_dispatched").is_none() {
        return;
    }
    let expected = Expected::from_environment();
    // testorig.jpg is 4:2:0 YCbCr: fancy h2v2 upsampling, YCbCr-to-RGB
    // conversion and the islow IDCT; at 1/2 the 4x4 IDCT too, and without
    // fancy upsampling the merged h2v2 upsampler.
    let data = inputs::image("testorig.jpg");
    for (scale_denom, fancy) in [(1, true), (2, true), (1, false)] {
        let what = format!("scale 1/{scale_denom}, fancy {fancy}, {expected:?}");
        let mut err = Box::new(jpeg_error_mgr::default());
        jpeg_std_error(&mut err);
        let mut cinfo = jpeg_decompress_struct::new(err);
        jpeg_create_decompress(&mut cinfo).unwrap();
        jpeg_mem_src(&mut cinfo, &data).unwrap();
        jpeg_read_header(&mut cinfo, true).unwrap();
        cinfo.scale_denom = scale_denom;
        cinfo.do_fancy_upsampling = fancy;
        assert!(jpeg_start_decompress(&mut cinfo).unwrap());

        assert_eq!(
            cinfo.master().pub_.simd_support,
            expected.support(),
            "{what}"
        );
        // The slots that the decode selected. At 1/2 the chroma components
        // get the 8x8 IDCT, which leaves nothing to upsample (jdmaster.c's
        // jpeg_core_output_dimensions), and the luma component the 4x4 IDCT.
        let (slot, kernel) = match (scale_denom, fancy) {
            (1, true) => (
                cinfo.upsample().h2v2_upsample_simd.is_some(),
                "h2v2_fancy_upsample",
            ),
            (2, _) => (cinfo.idct().idct_4x4_simd.is_some(), "idct_4x4"),
            _ => (
                cinfo.upsample().merged_upsample_simd.is_some(),
                "h2v2_merged_upsample",
            ),
        };
        let some = |kernel| expected.kernel(kernel) != jpeg::jsimd::JSIMD_NONE;
        assert_eq!(slot, some(kernel), "{kernel} slot, {what}");
        if fancy {
            assert_eq!(
                cinfo.cconvert().color_convert_simd.is_some(),
                some("ycc_rgb"),
                "ycc_rgb slot, {what}"
            );
        }
        assert_eq!(
            cinfo.idct().idct_simd.is_some(),
            some("idct_islow"),
            "idct_islow slot, {what}"
        );

        // The instruction set of each entry point, which a decode that
        // matches the oracle cannot show: the AVX2 and SSE2 kernels give the
        // same output.
        if scale_denom == 1 && fancy {
            let mut upsampler = *cinfo.upsample();
            let selected = [
                ("ycc_rgb", jsimd_set_ycc_rgb(&mut cinfo)),
                (
                    "h2v1_upsample",
                    jsimd_set_h2v1_upsample(&mut cinfo, &mut upsampler),
                ),
                (
                    "h2v2_upsample",
                    jsimd_set_h2v2_upsample(&mut cinfo, &mut upsampler),
                ),
                (
                    "h2v1_fancy_upsample",
                    jsimd_set_h2v1_fancy_upsample(&mut cinfo, &mut upsampler),
                ),
                (
                    "h2v2_fancy_upsample",
                    jsimd_set_h2v2_fancy_upsample(&mut cinfo, &mut upsampler),
                ),
                (
                    "h1v2_fancy_upsample",
                    jsimd_set_h1v2_fancy_upsample(&mut cinfo, &mut upsampler),
                ),
                (
                    "h2v1_merged_upsample",
                    jsimd_set_h2v1_merged_upsample(&mut cinfo),
                ),
                (
                    "h2v2_merged_upsample",
                    jsimd_set_h2v2_merged_upsample(&mut cinfo),
                ),
                ("idct_islow", jsimd_set_idct_islow(&mut cinfo)),
                ("idct_ifast", jsimd_set_idct_ifast(&mut cinfo)),
                ("idct_float", jsimd_set_idct_float(&mut cinfo)),
                ("idct_2x2", jsimd_set_idct_2x2(&mut cinfo)),
                ("idct_4x4", jsimd_set_idct_4x4(&mut cinfo)),
            ];
            for (kernel, set) in selected {
                assert_eq!(set, expected.kernel(kernel), "jsimd_set_{kernel}, {what}");
            }
        }
        jpeg_destroy_decompress(&mut cinfo).unwrap();
    }
}
