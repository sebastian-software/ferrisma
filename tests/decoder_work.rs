//! Decoder work through the public facade (#19, ADR-0012, ADR-0023): what
//! the executor requested, what each codec actually wrote, the overfetch, and
//! the lower bounds of reconstruction and entropy work. Counts only; no
//! timing. A codec returning a crop must never be reported as having skipped
//! work, so ROI codecs report `WorkCount::AtLeast`.
#![cfg(any(
    feature = "jpeg-read",
    feature = "webp-read",
    feature = "jpeg-xl-read",
    feature = "jpeg-xl-own-read",
    feature = "avif-read"
))]
// The JPEG XL tests also need `icc`; with a JPEG XL feature alone the shared
// helpers are compiled but unused.
#![allow(dead_code, unused_imports)]

use ferrisma::pipeline::{
    Crop, DecoderWork, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat,
    InputProfilePolicy, Output, OutputFormat, ResizeFilter, RowEncoder, SourceAccess,
    TransformOptions, WorkCount, transcode_with,
};

/// Captures rows instead of encoding, so the facade runs without a writer.
struct RowCapture;
struct CapturedRows(Vec<u8>);

impl EncoderBackend for RowCapture {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "test-row-capture"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        Ok(Box::new(CapturedRows(Vec::new())))
    }
}

impl RowEncoder for CapturedRows {
    fn write_row(&mut self, _: u32, row: &[u8]) -> Result<(), EncodeError> {
        self.0.extend_from_slice(row);
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(OutputFormat::WebP, self.0, 0, false))
    }
}

fn rect(x: u32, y: u32, width: u32, height: u32) -> Crop {
    Crop {
        x,
        y,
        width,
        height,
    }
}

fn srgb() -> TransformOptions {
    TransformOptions {
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        ..TransformOptions::default()
    }
}

/// Crop, then a Lanczos3 downscale to half size, on one transform thread.
fn run(bytes: &[u8], format: ImageFormat, crop: Crop, options: TransformOptions) -> Output {
    let output = transcode_with(
        bytes,
        format,
        TransformOptions {
            output_format: OutputFormat::WebP,
            crop: Some(crop),
            output_width: crop.width.div_ceil(2),
            output_height: crop.height.div_ceil(2),
            filter: ResizeFilter::Lanczos3,
            transform_threads: 1,
            ..options
        },
        &RowCapture,
    )
    .unwrap();
    let work = output.report.decoder_work;
    // The resize clamps at the crop's edges: no filter halo is requested,
    // and each requested row is delivered exactly once in one request.
    assert_eq!(work.crop, crop);
    assert_eq!(work.requested, crop);
    assert_eq!(work.delivered_rows, u64::from(crop.height));
    assert_eq!(output.report.decoder_requests, 1);
    let area = |r: Crop| u64::from(r.width) * u64::from(r.height);
    assert_eq!(area(work.codec_request), area(crop));
    assert_eq!(
        work.overfetch_pixels,
        area(work.codec_output) - area(work.codec_request)
    );
    assert!(work.reconstructed_pixels.at_least() >= area(work.codec_output));
    output
}

fn assert_work(
    actual: DecoderWork,
    codec_request: Crop,
    codec_output: Crop,
    reconstructed: WorkCount,
    entropy: WorkCount,
    copied: u64,
) {
    assert_eq!(actual.codec_request, codec_request, "{actual:?}");
    assert_eq!(actual.codec_output, codec_output, "{actual:?}");
    assert_eq!(actual.reconstructed_pixels, reconstructed, "{actual:?}");
    assert_eq!(actual.entropy_rows, entropy, "{actual:?}");
    assert_eq!(actual.wrapper_copied_bytes, copied, "{actual:?}");
}

/// 403×302 4:2:0 baseline JPEG (iMCU 16×16). libjpeg aligns the left edge
/// of the column crop (including one context column) to the iMCU, every row
/// is entropy-decoded, and the adapter allocates no full frame.
#[cfg(feature = "jpeg-read")]
#[test]
fn jpeg_crop_reports_imcu_column_overfetch_and_whole_entropy_pass() {
    let jpeg = include_bytes!("fixtures/decoder-work/paris-420.jpg");
    let crop = rect(101, 50, 40, 20);
    let output = run(jpeg, ImageFormat::Jpeg, crop, srgb());
    assert_eq!(output.report.input_access, SourceAccess::CodecRegionRequest);
    assert_eq!(output.report.decoder_full_frame_buffers, 0);
    // x = floor((101 - 1) / 16) * 16 = 96; width = 141 + 1 - 96 = 46.
    assert_work(
        output.report.decoder_work,
        crop,
        rect(96, 50, 46, 20),
        WorkCount::AtLeast(46 * 21),
        WorkCount::Exact(302),
        0,
    );
    assert_eq!(output.report.decoder_work.overfetch_pixels, 6 * 20);
}

/// EXIF orientation 6 (24×16 stored, 16×24 upright): the display crop maps
/// to its stored rectangle, read once and copied twice (fill and gather).
#[cfg(feature = "jpeg-read")]
#[test]
fn oriented_jpeg_crop_maps_to_stored_request_and_reports_copies() {
    let jpeg = include_bytes!("fixtures/orientation/jpeg-6.jpg");
    let crop = rect(2, 5, 12, 15);
    let output = run(jpeg, ImageFormat::Jpeg, crop, srgb());
    // Transpose to (5, 2, 15, 12); flip y: 16 - 2 - 12 = 2.
    let stored = rect(5, 2, 15, 12);
    assert_eq!(output.report.orientation_buffer_bytes, 15 * 12 * 3);
    assert_work(
        output.report.decoder_work,
        stored,
        // iMCU 8: x = floor(4 / 8) * 8 = 0; width = 20 + 1 - 0 = 21.
        rect(0, 2, 21, 12),
        WorkCount::AtLeast(21 * 13),
        WorkCount::Exact(16),
        2 * 15 * 12 * 3,
    );
}

/// Lossy WebP (VP8) with EXIF orientation 5, 96×72 stored: the adapter asks
/// for full-width rows from the top plus eight context rows, and VP8
/// reconstructs every row above the crop at full width.
#[cfg(feature = "webp-read")]
#[test]
fn lossy_webp_crop_decodes_full_width_rows_from_the_top() {
    let webp = include_bytes!("fixtures/decoder-work/paris-o5-lossy.webp");
    let crop = rect(11, 21, 30, 40);
    let output = run(webp, ImageFormat::WebP, crop, srgb());
    assert_eq!(output.report.input_access, SourceAccess::CodecRegionRequest);
    // Orientation 5 transposes: stored (21, 11, 40, 30). Output rows
    // 0..min(11 + 30 + 8, 72) = 49 at the full width 96.
    let stored = rect(21, 11, 40, 30);
    assert_work(
        output.report.decoder_work,
        stored,
        rect(0, 0, 96, 49),
        WorkCount::AtLeast(96 * 49),
        WorkCount::AtLeast(49),
        2 * 40 * 30 * 3,
    );
    assert_eq!(output.report.decoder_work.overfetch_pixels, 96 * 49 - 1200);
}

/// Lossless WebP (VP8L): the returned crop is even-aligned and small, but
/// VP8L decodes every row above it at full width, and the adapter reports a
/// full-frame fallback.
#[cfg(feature = "webp-read")]
#[test]
fn lossless_webp_crop_is_small_but_reconstruction_is_not() {
    let webp = include_bytes!("fixtures/decoder-work/paris-o5-lossless.webp");
    let crop = rect(11, 21, 30, 40);
    let output = run(webp, ImageFormat::WebP, crop, srgb());
    assert_eq!(output.report.input_access, SourceAccess::FullFrameFallback);
    assert_work(
        output.report.decoder_work,
        rect(21, 11, 40, 30),
        rect(20, 10, 41, 31),
        WorkCount::AtLeast(96 * 41),
        WorkCount::AtLeast(41),
        2 * 40 * 30 * 3,
    );
    assert_eq!(output.report.decoder_work.overfetch_pixels, 41 * 31 - 1200);
}

/// The explicitly selected jxl-oxide backend returns exactly the crop; its
/// internal group work is unobserved, so only the output is a lower bound.
#[cfg(all(feature = "jpeg-xl-read", feature = "icc"))]
#[test]
fn jxl_oxide_crop_reports_only_a_lower_bound() {
    let jxl = include_bytes!("fixtures/orientation/jxl-1.jxl");
    let crop = rect(1, 1, 4, 3);
    let options = TransformOptions {
        jxl_decoder: ferrisma::pipeline::JpegXlDecoder::JxlOxide,
        profile_policy: InputProfilePolicy::UseRenderedIcc,
        ..TransformOptions::default()
    };
    let output = run(jxl, ImageFormat::JpegXl, crop, options);
    assert_eq!(output.report.input_access, SourceAccess::CodecRegionRequest);
    assert_work(
        output.report.decoder_work,
        crop,
        crop,
        WorkCount::AtLeast(12),
        WorkCount::AtLeast(3),
        0,
    );
}

/// The libjxl-rs backend (the default when compiled in) decodes the full
/// frame for any crop. Since 2026-11-12 (ADR-0032) its wrapper retains only
/// the requested region through the decoder's image-out callback: the
/// reconstruction work stays full-frame, but no full-frame buffer is held,
/// and the full-request test below compares the managed peaks.
#[cfg(all(feature = "jpeg-xl-own-read", feature = "icc"))]
#[test]
fn libjxl_rs_crop_reports_the_full_frame_fallback() {
    let jxl = include_bytes!("fixtures/orientation/jxl-1.jxl");
    let crop = rect(1, 1, 4, 3);
    let output = run(
        jxl,
        ImageFormat::JpegXl,
        crop,
        TransformOptions {
            jxl_decoder: ferrisma::pipeline::JpegXlDecoder::LibjxlRs,
            // This backend reports its emitted color as the DATA ICC.
            profile_policy: InputProfilePolicy::UseRenderedIcc,
            ..TransformOptions::default()
        },
    );
    assert_eq!(output.report.input_access, SourceAccess::FullFrameFallback);
    assert_eq!(output.report.decoder_full_frame_buffers, 0);
    assert_work(
        output.report.decoder_work,
        crop,
        rect(0, 0, 24, 16),
        WorkCount::AtLeast(24 * 16),
        WorkCount::AtLeast(16),
        0,
    );
    assert_eq!(output.report.decoder_work.overfetch_pixels, 24 * 16 - 12);
}

/// A full-frame request is the one libjxl-rs read that still retains a
/// full-frame buffer. A 4×3 crop of the same file needs at least the
/// difference between the 24×16 and 4×3 RGB F32 buffers less managed memory
/// (every other buffer of the crop run is also no larger).
#[cfg(all(feature = "jpeg-xl-own-read", feature = "icc"))]
#[test]
fn libjxl_rs_full_request_retains_one_full_frame() {
    let jxl = include_bytes!("fixtures/orientation/jxl-1.jxl");
    let options = || TransformOptions {
        jxl_decoder: ferrisma::pipeline::JpegXlDecoder::LibjxlRs,
        profile_policy: InputProfilePolicy::UseRenderedIcc,
        ..TransformOptions::default()
    };
    let full = run(jxl, ImageFormat::JpegXl, rect(0, 0, 24, 16), options());
    assert_eq!(full.report.input_access, SourceAccess::FullFrameFallback);
    assert_eq!(full.report.decoder_full_frame_buffers, 1);
    assert!(full.report.peak_managed_bytes >= 24 * 16 * 3 * 4);
    assert_eq!(full.report.decoder_work.overfetch_pixels, 0);
    let crop = run(jxl, ImageFormat::JpegXl, rect(1, 1, 4, 3), options());
    assert_eq!(crop.report.decoder_full_frame_buffers, 0);
    assert!(
        full.report.peak_managed_bytes - crop.report.peak_managed_bytes
            >= (24 * 16 - 4 * 3) * 3 * 4,
        "full {} crop {}",
        full.report.peak_managed_bytes,
        crop.report.peak_managed_bytes
    );
}

/// AVIF decodes and converts the whole frame for every request, then the
/// orientation wrapper copies the stored crop (irot 3: orientation 6).
#[cfg(feature = "avif-read")]
#[test]
fn avif_crop_reports_the_full_frame_fallback_and_orientation_copies() {
    let avif = include_bytes!("fixtures/orientation/avif-6.avif");
    let crop = rect(2, 5, 12, 15);
    let output = run(avif, ImageFormat::Avif, crop, srgb());
    assert_eq!(output.report.input_access, SourceAccess::FullFrameFallback);
    assert_eq!(output.report.decoder_full_frame_buffers, 1);
    assert_work(
        output.report.decoder_work,
        rect(5, 2, 15, 12),
        rect(0, 0, 24, 16),
        WorkCount::AtLeast(24 * 16),
        WorkCount::AtLeast(16),
        2 * 15 * 12 * 3,
    );
    assert_eq!(output.report.decoder_work.overfetch_pixels, 24 * 16 - 180);
}
