//! Decode one still AVIF through the libavif port with a decoder chosen by
//! name: the Rust counterpart of the C `decode_still.c` that
//! `scripts/bench/ci-perf/codec_avif.sh` writes, call for call.
//!
//! ```sh
//! cargo run --release -p ferrisma-libavif --example decode_codec -- aom|dav1d THREADS input.avif output.ppm
//! ```
//!
//! The benchmark uses it for the `port-dav1d` column: the port's dav1d,
//! which the engine does not select (ADR-0031), with the kernels rebuilt so
//! far (ADR 0062). Settings: the primary item, one image, nearest chroma
//! upsampling to 8-bit RGB. `elapsed_ms` covers decoder creation, parse,
//! decode and YUV to RGB; reading the input and writing the PPM are outside.
use std::time::Instant;

use avif::{avif as image_api, read, reformat};

const AVIF_RESULT_OK: u32 = 0;
const AVIF_CODEC_CHOICE_AOM: u32 = 1;
const AVIF_CODEC_CHOICE_DAV1D: u32 = 2;
const AVIF_DECODER_SOURCE_PRIMARY_ITEM: u32 = 1;
const AVIF_RGB_FORMAT_RGB: u32 = 0;
const AVIF_CHROMA_UPSAMPLING_NEAREST: u32 = 3;

fn fail(message: &str) -> ! {
    eprintln!("decode_codec: {message}");
    std::process::exit(1);
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [codec, threads, input, output] = arguments.as_slice() else {
        fail("usage: decode_codec aom|dav1d THREADS input.avif output.ppm");
    };
    let choice = match codec.as_str() {
        "aom" => AVIF_CODEC_CHOICE_AOM,
        "dav1d" => AVIF_CODEC_CHOICE_DAV1D,
        _ => fail("codec must be aom or dav1d"),
    };
    let threads: i32 = threads.parse().unwrap_or_else(|_| fail("threads"));
    let data = std::fs::read(input).unwrap_or_else(|_| fail("read input"));

    // SAFETY: the libavif C ABI is used as documented in `avif.h`; the
    // decoder and the RGB image are created, used and destroyed once, and
    // `data` outlives the decoder.
    unsafe {
        let start = Instant::now();
        let decoder = read::avifDecoderCreate();
        if decoder.is_null() {
            fail("avifDecoderCreate");
        }
        (*decoder).codecChoice = choice;
        (*decoder).maxThreads = threads;
        (*decoder).requestedSource = AVIF_DECODER_SOURCE_PRIMARY_ITEM;
        (*decoder).imageSizeLimit = u32::MAX;
        (*decoder).imageCountLimit = 1;
        if read::avifDecoderSetIOMemory(decoder, data.as_ptr(), data.len()) != AVIF_RESULT_OK
            || read::avifDecoderParse(decoder) != AVIF_RESULT_OK
        {
            fail("parse");
        }
        if read::avifDecoderNthImage(decoder, 0) != AVIF_RESULT_OK {
            fail("decode");
        }
        let mut rgb: reformat::avifRGBImage = std::mem::zeroed();
        image_api::avifRGBImageSetDefaults((&raw mut rgb).cast(), (*decoder).image.cast());
        rgb.depth = 8;
        rgb.format = AVIF_RGB_FORMAT_RGB;
        rgb.alphaPremultiplied = 0;
        rgb.chromaUpsampling = AVIF_CHROMA_UPSAMPLING_NEAREST;
        if image_api::avifRGBImageAllocatePixels((&raw mut rgb).cast()) != AVIF_RESULT_OK
            || reformat::avifImageYUVToRGB((*decoder).image.cast(), &raw mut rgb) != AVIF_RESULT_OK
        {
            fail("YUV to RGB");
        }
        let elapsed = start.elapsed();

        let mut ppm = format!("P6\n{} {}\n255\n", rgb.width, rgb.height).into_bytes();
        for y in 0..rgb.height as usize {
            let row =
                std::slice::from_raw_parts(rgb.pixels.add(y * rgb.rowBytes as usize), rgb.width as usize * 3);
            ppm.extend_from_slice(row);
        }
        std::fs::write(output, ppm).unwrap_or_else(|_| fail("write output"));
        println!(
            "elapsed_ms={:.3} width={} height={} codec={codec} threads={threads}",
            elapsed.as_secs_f64() * 1000.0,
            rgb.width,
            rgb.height
        );
        image_api::avifRGBImageFreePixels((&raw mut rgb).cast());
        read::avifDecoderDestroy(decoder);
    }
}
