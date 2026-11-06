//! Encode one still image through libavif and the Rust AOM encoder, the way
//! an all-intra caller drives it: `avifImageRGBToYUV`, then
//! `avifEncoderWrite`. Used to profile and time the encoder.
//!
//! ```sh
//! cargo run --release --example encode_still -- input.bmp output.avif \
//!     --quality=56 --speed=6 --threads=1 --depth=10 --repeat=3
//! ```
//!
//! The input is a 24- or 32-bit BMP as macOS `sips -s format bmp` writes it
//! (uncompressed, or 32-bit with byte-aligned channel masks); alpha is
//! ignored. Each repetition prints `encode_us=<microseconds>` for
//! `avifEncoderWrite` alone and the output size. `--threads=0` uses all
//! available cores and enables automatic tiling. `--report-rtcd` prints
//! whether libaom's RTCD tables selected the SIMD kernels (ADR 0056).
use std::time::Instant;

use avif::{avif as image_api, rawdata, reformat, write};

const AVIF_RESULT_OK: u32 = 0;
const AVIF_PIXEL_FORMAT_YUV444: u32 = 1;
const AVIF_PIXEL_FORMAT_YUV420: u32 = 3;
const AVIF_RGB_FORMAT_RGB: u32 = 0;
const AVIF_RANGE_FULL: u32 = 1;

struct Options {
    input: String,
    output: String,
    quality: i32,
    speed: i32,
    threads: i32,
    depth: u32,
    yuv444: bool,
    repeat: usize,
    report_rtcd: bool,
}

fn parse() -> Options {
    let mut positional = Vec::new();
    let mut options = Options {
        input: String::new(),
        output: String::new(),
        quality: 56,
        speed: 6,
        threads: 1,
        depth: 10,
        yuv444: false,
        repeat: 1,
        report_rtcd: false,
    };
    for argument in std::env::args().skip(1) {
        let (key, value) = argument.split_once('=').unwrap_or((&argument, ""));
        match key {
            "--quality" => options.quality = value.parse().expect("quality"),
            "--speed" => options.speed = value.parse().expect("speed"),
            "--threads" => options.threads = value.parse().expect("threads"),
            "--depth" => options.depth = value.parse().expect("depth"),
            "--yuv444" => options.yuv444 = true,
            "--repeat" => options.repeat = value.parse().expect("repeat"),
            "--report-rtcd" => options.report_rtcd = true,
            _ if key.starts_with("--") => panic!("unknown option {key}"),
            _ => positional.push(argument),
        }
    }
    assert_eq!(positional.len(), 2, "usage: encode_still input.bmp output.avif [options]");
    options.output = positional.pop().unwrap();
    options.input = positional.pop().unwrap();
    if options.threads == 0 {
        options.threads = std::thread::available_parallelism().map_or(1, |n| n.get() as i32);
    }
    options
}

/// Returns width, height and tightly packed RGB rows, top row first.
fn read_bmp(path: &str) -> (u32, u32, Vec<u8>) {
    let data = std::fs::read(path).expect("read input");
    assert_eq!(&data[..2], b"BM", "not a BMP file");
    let word = |offset: usize| u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
    let offset = word(10) as usize;
    let width = word(18) as i32;
    let height = word(22) as i32;
    let bits = u16::from_le_bytes([data[28], data[29]]);
    assert!(bits == 24 || bits == 32, "BMP input must have 24 or 32 bits per pixel");
    // Uncompressed (BI_RGB, B G R [X]) or 32-bit BI_BITFIELDS with byte masks.
    let channel = |mask: u32| mask.trailing_zeros() as usize / 8;
    let order = match word(30) {
        0 => [2, 1, 0],
        3 if bits == 32 => [channel(word(54)), channel(word(58)), channel(word(62))],
        _ => panic!("compressed BMP input is not supported"),
    };
    let (width, top_down) = (width as u32, height < 0);
    let height = height.unsigned_abs();
    let bytes = (bits / 8) as usize;
    let stride = (width as usize * bytes).div_ceil(4) * 4;
    let mut rgb = Vec::with_capacity(width as usize * height as usize * 3);
    for y in 0..height as usize {
        let row = if top_down { y } else { height as usize - 1 - y };
        let start = offset + row * stride;
        for x in 0..width as usize {
            let pixel = &data[start + x * bytes..];
            rgb.extend_from_slice(&[pixel[order[0]], pixel[order[1]], pixel[order[2]]]);
        }
    }
    (width, height, rgb)
}

fn main() {
    let options = parse();
    let (width, height, mut rgb) = read_bmp(&options.input);
    let format = if options.yuv444 { AVIF_PIXEL_FORMAT_YUV444 } else { AVIF_PIXEL_FORMAT_YUV420 };
    let mut last = Vec::new();
    for _ in 0..options.repeat {
        // SAFETY: the libavif C ABI is used as documented in `avif.h`; every
        // object is created, used, and destroyed exactly once in this block,
        // and `rgb` outlives the conversion that reads it.
        unsafe {
            let image = image_api::avifImageCreate(width, height, options.depth, format);
            assert!(!image.is_null());
            (*image).colorPrimaries = 1;
            (*image).transferCharacteristics = 13;
            (*image).matrixCoefficients = 6;
            (*image).yuvRange = AVIF_RANGE_FULL;
            let mut rgb_image: reformat::avifRGBImage = std::mem::zeroed();
            image_api::avifRGBImageSetDefaults((&raw mut rgb_image).cast(), image.cast());
            rgb_image.format = AVIF_RGB_FORMAT_RGB;
            rgb_image.depth = 8;
            rgb_image.pixels = rgb.as_mut_ptr();
            rgb_image.rowBytes = width * 3;
            let result = reformat::avifImageRGBToYUV(image.cast(), &raw const rgb_image);
            assert_eq!(result, AVIF_RESULT_OK, "avifImageRGBToYUV");

            let encoder = write::avifEncoderCreate();
            assert!(!encoder.is_null());
            (*encoder).quality = options.quality;
            (*encoder).qualityAlpha = options.quality;
            (*encoder).speed = options.speed;
            (*encoder).maxThreads = options.threads;
            (*encoder).autoTiling = i32::from(options.threads != 1);
            let mut output: write::avifRWData = std::mem::zeroed();
            let start = Instant::now();
            let result = write::avifEncoderWrite(encoder, image.cast(), &raw mut output);
            let elapsed = start.elapsed();
            assert_eq!(result, AVIF_RESULT_OK, "avifEncoderWrite");
            println!("encode_us={} bytes={}", elapsed.as_micros(), output.size);
            last = std::slice::from_raw_parts(output.data, output.size).to_vec();
            rawdata::avifRWDataFree((&raw mut output).cast());
            write::avifEncoderDestroy(encoder);
            image_api::avifImageDestroy(image);
        }
    }
    std::fs::write(&options.output, last).expect("write output");
    if options.report_rtcd {
        println!("rtcd={}", rtcd_selection());
    }
}

/// Reports whether libaom's RTCD tables, which the first encoder set up,
/// select the hand-translated SIMD kernels (ADR 0056) or the `_c` functions.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn rtcd_selection() -> &'static str {
    use aom::simd::av1__encoder__arm__av1_highbd_quantize_neon::av1_highbd_quantize_fp_neon;
    extern "C" {
        // RTCD pointer defined by the translated av1_rtcd.c (aom_ports/arm.h
        // dispatch); only its address value is read.
        static av1_highbd_quantize_fp: usize;
    }
    // SAFETY: setup_rtcd_internal() assigned the pointer during encoder
    // initialization, which happened-before this read on this thread.
    let selected = unsafe { av1_highbd_quantize_fp };
    if selected == av1_highbd_quantize_fp_neon as usize { "simd(neon)" } else { "c" }
}

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
fn rtcd_selection() -> &'static str {
    "c(no dispatch on this target)"
}
