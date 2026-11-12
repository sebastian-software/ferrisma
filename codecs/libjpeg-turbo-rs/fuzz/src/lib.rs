//! The driver of the `decode` fuzz target (ADR 0009): the safe decoding API
//! (ADR 0006) on arbitrary bytes.
//!
//! The input is the JPEG data itself, so upstream's test images and its
//! `fuzz/jpeg.dict` apply unchanged. Each input runs through [`CASES`],
//! modeled on upstream's `fuzz/decompress.cc`, which drives the `TurboJPEG`
//! API, mapped onto `jpeg::Decoder` and `jpeg::Scanlines`: every pixel format
//! family, the fast and float IDCTs, fast upsampling, reduced and enlarged
//! scaling, a column crop with skipped rows, and failing on the first
//! warning. Every case reads the header, the EXIF data and the ICC profile,
//! asks for the output dimensions, decodes all remaining rows and finishes.
//!
//! Like upstream's targets, it ignores images of more than one megapixel to
//! bound memory. A `jpeg::Error` ends a case: the API documents that the
//! decoder cannot continue after one. A panic, a crash, a timeout or a
//! sanitizer report is a finding.

use jpeg::{DctMethod, Decoder, DecoderOptions, OutputOptions, PixelFormat, Scale};

/// Upstream's limit on `width * height`, as in Google's OSS-Fuzz target.
pub const MAX_PIXELS: u64 = 1_048_576;

/// The output size limit for enlarged scaling: four times the input limit.
pub const MAX_OUTPUT_PIXELS: u64 = 4 * MAX_PIXELS;

/// Rows per `read_rows` call; more than one, to cover multi-row requests.
const STRIP_ROWS: usize = 8;

/// One decode of the input.
#[derive(Clone, Copy, Debug)]
pub struct Case {
    /// A short name for reports.
    pub name: &'static str,
    /// The output layout; `None` keeps libjpeg's default.
    pub format: Option<PixelFormat>,
    /// The scaling factor.
    pub scale: Scale,
    /// The inverse DCT.
    pub dct_method: DctMethod,
    /// `do_fancy_upsampling`.
    pub fancy_upsampling: bool,
    /// `do_block_smoothing`.
    pub block_smoothing: bool,
    /// Upstream's cropping region `{ 32, 16, 65, 59 }` for images of at
    /// least 97 x 75 output pixels: crop the columns, skip the rows above.
    pub crop: bool,
    /// [`DecoderOptions::fail_on_warning`].
    pub fail_on_warning: bool,
}

/// The cases every input runs through, in order.
pub const CASES: [Case; 7] = [
    // Upstream's first iteration: fast upsampling and the fast IDCT.
    Case {
        name: "rgb-ifast-fastupsample",
        format: Some(PixelFormat::Rgb),
        scale: Scale::ONE,
        dct_method: DctMethod::IntFast,
        fancy_upsampling: false,
        block_smoothing: false,
        crop: false,
        fail_on_warning: false,
    },
    Case {
        name: "bgrx-half-crop",
        format: Some(PixelFormat::Bgrx),
        scale: Scale { num: 1, denom: 2 },
        dct_method: DctMethod::IntSlow,
        fancy_upsampling: true,
        block_smoothing: true,
        crop: true,
        fail_on_warning: false,
    },
    Case {
        name: "abgr-eighth",
        format: Some(PixelFormat::Abgr),
        scale: Scale { num: 1, denom: 8 },
        dct_method: DctMethod::IntSlow,
        fancy_upsampling: true,
        block_smoothing: true,
        crop: false,
        fail_on_warning: false,
    },
    // The float IDCT, from upstream's `decompress_libjpeg.cc`.
    Case {
        name: "gray-float-crop",
        format: Some(PixelFormat::Grayscale),
        scale: Scale::ONE,
        dct_method: DctMethod::Float,
        fancy_upsampling: true,
        block_smoothing: true,
        crop: true,
        fail_on_warning: false,
    },
    // Only CMYK and YCCK files convert to CMYK.
    Case {
        name: "cmyk-fail-on-warning",
        format: Some(PixelFormat::Cmyk),
        scale: Scale::ONE,
        dct_method: DctMethod::IntSlow,
        fancy_upsampling: true,
        block_smoothing: true,
        crop: false,
        fail_on_warning: true,
    },
    // The stored color space at 6/8, an IDCT size between upstream's cases.
    Case {
        name: "ycbcr-three-quarters",
        format: Some(PixelFormat::YCbCr),
        scale: Scale { num: 3, denom: 4 },
        dct_method: DctMethod::IntSlow,
        fancy_upsampling: true,
        block_smoothing: true,
        crop: false,
        fail_on_warning: false,
    },
    // libjpeg's default output at 16/8, the largest scale.
    Case {
        name: "default-double",
        format: None,
        scale: Scale { num: 2, denom: 1 },
        dct_method: DctMethod::IntSlow,
        fancy_upsampling: true,
        block_smoothing: true,
        crop: false,
        fail_on_warning: false,
    },
];

/// Why a case stopped before finishing.
#[derive(Debug)]
pub enum Stop {
    /// The image or its output exceeds the size limits.
    TooLarge,
    /// The API returned an error.
    Error(jpeg::Error),
}

impl From<jpeg::Error> for Stop {
    fn from(error: jpeg::Error) -> Self {
        Stop::Error(error)
    }
}

/// Decodes `data` as `case` describes; on success, a sum over the metadata,
/// every output sample and the warning count.
///
/// # Errors
///
/// [`Stop`] when the image is too large or a call fails.
pub fn decode(data: &[u8], case: &Case) -> Result<u64, Stop> {
    let options = DecoderOptions {
        max_memory: None,
        fail_on_warning: case.fail_on_warning,
    };
    let mut decoder = Decoder::with_options(data, &options)?;
    let header = *decoder.header();
    let pixels = u64::from(header.width) * u64::from(header.height);
    if pixels == 0 || pixels > MAX_PIXELS {
        return Err(Stop::TooLarge);
    }
    let mut sum = decoder.exif().map_or(0, |exif| exif.len() as u64);
    if let Some(icc) = decoder.icc_profile()? {
        sum += icc.iter().map(|&byte| u64::from(byte)).sum::<u64>();
    }
    let output = OutputOptions {
        format: case.format,
        scale: case.scale,
        dct_method: case.dct_method,
        fancy_upsampling: case.fancy_upsampling,
        block_smoothing: case.block_smoothing,
    };
    let info = decoder.output_info(&output)?;
    if u64::from(info.width) * u64::from(info.height) > MAX_OUTPUT_PIXELS {
        return Err(Stop::TooLarge);
    }
    let mut scanlines = decoder.start(&output)?;
    if case.crop && info.width >= 97 && info.height >= 75 {
        scanlines.crop(32, 65)?;
        scanlines.skip_rows(16)?;
    }
    let row_bytes = scanlines.info().row_bytes();
    let mut buffer = vec![0; row_bytes * STRIP_ROWS];
    loop {
        let rows = scanlines.read_rows(&mut buffer)? as usize;
        if rows == 0 {
            break;
        }
        // Touch every output sample, as upstream's targets do.
        sum += buffer[..rows * row_bytes]
            .iter()
            .map(|&sample| u64::from(sample))
            .sum::<u64>();
    }
    let report = scanlines.finish()?;
    Ok(sum + report.warning_count)
}

/// Runs every case on `data`; the fuzz target's body.
pub fn fuzz(data: &[u8]) {
    for case in &CASES {
        let _ = std::hint::black_box(decode(data, case));
    }
}

#[cfg(test)]
mod tests {
    use super::{CASES, Stop, decode};
    use std::path::Path;

    fn read(path: &str) -> Vec<u8> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
        std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
    }

    /// The names of the cases that finish on `data`.
    fn finishing(data: &[u8]) -> Vec<&'static str> {
        CASES
            .iter()
            .filter(|case| decode(data, case).is_ok())
            .map(|case| case.name)
            .collect()
    }

    #[test]
    fn a_ycbcr_image_finishes_every_case_but_cmyk() {
        let data = read("../upstream/libjpeg-turbo/testimages/testorig.jpg");
        let expected: Vec<_> = CASES
            .iter()
            .map(|case| case.name)
            .filter(|&name| name != "cmyk-fail-on-warning")
            .collect();
        assert_eq!(finishing(&data), expected);
        let cmyk = CASES
            .iter()
            .find(|case| case.name == "cmyk-fail-on-warning");
        assert!(matches!(
            decode(&data, cmyk.unwrap()),
            Err(Stop::Error(jpeg::Error::Library(_)))
        ));
    }

    #[test]
    fn a_cmyk_image_reaches_the_cmyk_case() {
        let data = read("../../../tests/fixtures/cmyk.jpg");
        assert!(finishing(&data).contains(&"cmyk-fail-on-warning"));
    }

    #[test]
    fn an_arithmetic_image_finishes() {
        let data = read("../upstream/libjpeg-turbo/testimages/testimgari.jpg");
        assert!(finishing(&data).contains(&"rgb-ifast-fastupsample"));
    }

    #[test]
    fn a_twelve_bit_image_is_an_error() {
        let data = read("../upstream/libjpeg-turbo/testimages/monkey12.jpg");
        assert_eq!(finishing(&data), Vec::<&str>::new());
    }

    #[test]
    fn non_jpeg_data_is_an_error() {
        assert_eq!(finishing(b""), Vec::<&str>::new());
        assert_eq!(finishing(b"\xff\xd8\xff"), Vec::<&str>::new());
    }

    /// Classifies every starting seed in `LIBJPEG_FUZZ_SEEDS`, written by
    /// `fuzz/seed_corpus.py`: each 8-bit seed must finish at least one case.
    #[test]
    fn seeds_are_classified() {
        let Some(root) = std::env::var_os("LIBJPEG_FUZZ_SEEDS") else {
            eprintln!("LIBJPEG_FUZZ_SEEDS is unset; no seeds classified");
            return;
        };
        let mut paths: Vec<_> = std::fs::read_dir(&root)
            .expect("the seed directory")
            .map(|entry| entry.expect("a seed").path())
            .collect();
        paths.sort();
        assert!(
            !paths.is_empty(),
            "no seeds in {}",
            Path::new(&root).display()
        );
        for path in paths {
            let data = std::fs::read(&path).expect("a seed");
            let finished = finishing(&data);
            let name = path.file_name().unwrap().to_string_lossy();
            println!(
                "seed {name}: {} of {} cases finish: {finished:?}",
                finished.len(),
                CASES.len()
            );
            if !name.contains("monkey12") {
                assert!(!finished.is_empty(), "seed {name} finishes no case");
            }
        }
    }
}
