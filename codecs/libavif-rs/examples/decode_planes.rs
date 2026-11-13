//! Decode one still AVIF through libavif and the Rust AOM decoder and write
//! the decoded planes and RGB, the counterpart of `tools/decode_still.c` for
//! the SIMD-against-SIMD gate (`tools/check_aom_simd_original.py`).
//!
//! ```sh
//! cargo run --release --example decode_planes -- input.avif planes.raw rgb.raw
//! ```
//!
//! Settings follow `crates/ferrisma-avif-read`: the AOM decoder, one thread,
//! the primary item. `planes.raw` holds the Y, U, V and alpha planes row by row
//! without padding (two bytes per sample, native order, above 8 bits);
//! `rgb.raw` holds `avifImageYUVToRGB` with nearest chroma upsampling, RGB or
//! RGBA, 8-bit for 8-bit images and 16-bit otherwise.
use std::io::Write;

use avif::{avif as image_api, read, reformat};

const AVIF_RESULT_OK: u32 = 0;
const AVIF_CODEC_CHOICE_AOM: u32 = 1;
const AVIF_DECODER_SOURCE_PRIMARY_ITEM: u32 = 1;
const AVIF_RGB_FORMAT_RGB: u32 = 0;
const AVIF_RGB_FORMAT_RGBA: u32 = 1;
const AVIF_CHROMA_UPSAMPLING_NEAREST: u32 = 3;

/// Writes `height` rows of `width * bytes` bytes from a plane with `row_bytes`.
///
/// # Safety
/// `plane` must point to `height` rows of `row_bytes` readable bytes.
unsafe fn write_plane(out: &mut Vec<u8>, plane: *const u8, row_bytes: u32, width: u32, height: u32, bytes: u32) {
    for y in 0..height as usize {
        // SAFETY: the caller guarantees `height` readable rows of `row_bytes`.
        let row = unsafe {
            std::slice::from_raw_parts(plane.add(y * row_bytes as usize), (width * bytes) as usize)
        };
        out.extend_from_slice(row);
    }
}

fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let [input, planes_path, rgb_path] = arguments.as_slice() else {
        panic!("usage: decode_planes input.avif planes.raw rgb.raw");
    };
    let bytes = std::fs::read(input).expect("read input");
    // SAFETY: the libavif C ABI is used as documented in `avif.h`; the decoder
    // is created, used and destroyed once, and `bytes` outlives it.
    unsafe {
        let decoder = read::avifDecoderCreate();
        assert!(!decoder.is_null());
        (*decoder).codecChoice = AVIF_CODEC_CHOICE_AOM;
        (*decoder).maxThreads = 1;
        (*decoder).requestedSource = AVIF_DECODER_SOURCE_PRIMARY_ITEM;
        (*decoder).imageSizeLimit = u32::MAX;
        (*decoder).imageCountLimit = 1;
        assert_eq!(read::avifDecoderSetIOMemory(decoder, bytes.as_ptr(), bytes.len()), AVIF_RESULT_OK);
        assert_eq!(read::avifDecoderParse(decoder), AVIF_RESULT_OK, "parse");
        assert_eq!(read::avifDecoderNthImage(decoder, 0), AVIF_RESULT_OK, "decode");
        let image = &*(*decoder).image;
        let sample = if image.depth > 8 { 2 } else { 1 };
        let mut info: image_api::avifPixelFormatInfo = std::mem::zeroed();
        image_api::avifGetPixelFormatInfo(image.yuvFormat, &raw mut info);
        let mut planes = Vec::new();
        write_plane(&mut planes, image.yuvPlanes[0], image.yuvRowBytes[0], image.width, image.height, sample);
        if info.monochrome == 0 {
            let width = (image.width + info.chromaShiftX as u32) >> info.chromaShiftX;
            let height = (image.height + info.chromaShiftY as u32) >> info.chromaShiftY;
            for channel in 1..3 {
                write_plane(&mut planes, image.yuvPlanes[channel], image.yuvRowBytes[channel], width, height,
                            sample);
            }
        }
        let alpha = !image.alphaPlane.is_null();
        if alpha {
            write_plane(&mut planes, image.alphaPlane, image.alphaRowBytes, image.width, image.height, sample);
        }
        std::fs::write(planes_path, planes).expect("write planes");

        let mut rgb: reformat::avifRGBImage = std::mem::zeroed();
        image_api::avifRGBImageSetDefaults((&raw mut rgb).cast(), (*decoder).image.cast());
        rgb.depth = if image.depth > 8 { 16 } else { 8 };
        rgb.format = if alpha { AVIF_RGB_FORMAT_RGBA } else { AVIF_RGB_FORMAT_RGB };
        rgb.alphaPremultiplied = 0;
        rgb.chromaUpsampling = AVIF_CHROMA_UPSAMPLING_NEAREST;
        rgb.maxThreads = 1;
        assert_eq!(image_api::avifRGBImageAllocatePixels((&raw mut rgb).cast()), AVIF_RESULT_OK);
        assert_eq!(reformat::avifImageYUVToRGB((*decoder).image.cast(), &raw mut rgb), AVIF_RESULT_OK,
                   "YUV to RGB");
        let mut out = Vec::new();
        let channels = if alpha { 4 } else { 3 };
        write_plane(&mut out, rgb.pixels, rgb.rowBytes, rgb.width, rgb.height, channels * (rgb.depth / 8));
        std::fs::write(rgb_path, out).expect("write rgb");
        println!("width={} height={} depth={} format={} alpha={}", image.width, image.height, image.depth,
                 image.yuvFormat, i32::from(alpha));
        std::io::stdout().flush().expect("flush");
        image_api::avifRGBImageFreePixels((&raw mut rgb).cast());
        read::avifDecoderDestroy(decoder);
    }
}
