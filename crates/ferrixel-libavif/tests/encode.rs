//! End to end: Ferrixel's pipeline with the libavif-rs backend, checked by
//! decoding the result with an independent decoder (`avif-decode`, rav1d)
//! and comparing it with the rows Ferrixel handed to the backend.

use std::sync::Mutex;

use ferrixel::pipeline::{
    ChromaSubsampling, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat,
    InputProfilePolicy, OutputFormat, ProcessingErrorKind, RowEncoder, TransformOptions,
    transcode_with,
};
use ferrixel_libavif::LibavifBackend;

const JPEG: &[u8] = include_bytes!("../../../tests/fixtures/sample.jpg");

/// Records the RGBA rows Ferrixel produces, as the reference for a decode.
#[derive(Default)]
struct Rows(Mutex<Vec<u8>>);

struct RowSink<'a>(&'a Rows);

impl EncoderBackend for Rows {
    fn format(&self) -> OutputFormat {
        OutputFormat::Avif
    }
    fn name(&self) -> &'static str {
        "rows"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        self.0.lock().unwrap().clear();
        Ok(Box::new(RowSink(self)))
    }
}

impl RowEncoder for RowSink<'_> {
    fn write_row(&mut self, _: u32, rgba: &[u8]) -> Result<(), EncodeError> {
        self.0.0.lock().unwrap().extend_from_slice(rgba);
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(OutputFormat::Avif, Vec::new(), 0, false))
    }
}

/// What the independent decoder reports about one file.
struct Decoded {
    width: usize,
    height: usize,
    /// RGBA8; alpha 255 when the file has no alpha item.
    rgba: Vec<u8>,
    alpha: bool,
    depth: u32,
    /// Colour primaries, transfer, matrix and full range.
    cicp: (u16, u16, u16, bool),
}

/// Decode with libavif-rs's own decoder (its dav1d port, validated byte for
/// byte against upstream libavif). `avif-decode` cannot be linked here: its
/// rav1d exports the same `dav1d_*` C symbols as libavif-rs's dav1d port.
fn decode(file: &[u8]) -> Decoded {
    use avif::{avif as image_api, read, reformat};
    // SAFETY (whole block): every object comes from its libavif constructor,
    // is checked for null, and is freed exactly once at the end; the modules'
    // `avifImage` and `avifRGBImage` types translate the same C structs, so
    // the pointer casts keep their layout. `file` outlives the decode.
    unsafe {
        let decoder = read::avifDecoderCreate();
        let image = image_api::avifImageCreateEmpty();
        assert!(!decoder.is_null() && !image.is_null());
        let result = read::avifDecoderReadMemory(decoder, image.cast(), file.as_ptr(), file.len());
        assert_eq!(result, 0, "libavif-rs could not decode the output");
        let mut rgb: reformat::avifRGBImage = std::mem::zeroed();
        image_api::avifRGBImageSetDefaults((&raw mut rgb).cast(), image.cast());
        rgb.format = 1; // RGBA
        rgb.depth = 8;
        assert_eq!(
            image_api::avifRGBImageAllocatePixels((&raw mut rgb).cast()),
            0
        );
        assert_eq!(reformat::avifImageYUVToRGB(image.cast(), &raw mut rgb), 0);
        let (width, height) = ((*image).width as usize, (*image).height as usize);
        let rgba = std::slice::from_raw_parts(rgb.pixels, rgb.rowBytes as usize * height)
            .chunks(rgb.rowBytes as usize)
            .flat_map(|row| row[..width * 4].to_vec())
            .collect();
        let out = Decoded {
            width,
            height,
            rgba,
            alpha: !(*image).alphaPlane.is_null(),
            depth: (*image).depth,
            cicp: (
                (*image).colorPrimaries,
                (*image).transferCharacteristics,
                (*image).matrixCoefficients,
                (*image).yuvRange == 1,
            ),
        };
        image_api::avifRGBImageFreePixels((&raw mut rgb).cast());
        image_api::avifImageDestroy(image);
        read::avifDecoderDestroy(decoder);
        out
    }
}

/// Mean and largest absolute difference per sample.
fn difference(a: &[u8], b: &[u8]) -> (f64, u8) {
    assert_eq!(a.len(), b.len());
    let mut sum = 0_u64;
    let mut max = 0_u8;
    for (&x, &y) in a.iter().zip(b) {
        let d = x.abs_diff(y);
        sum += u64::from(d);
        max = max.max(d);
    }
    #[allow(clippy::cast_precision_loss)]
    let mean = sum as f64 / a.len() as f64;
    (mean, max)
}

fn options(width: u32, height: u32) -> TransformOptions {
    TransformOptions {
        output_width: width,
        output_height: height,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        quality: 95.0,
        speed: 9,
        ..TransformOptions::default()
    }
}

#[test]
fn encodes_what_ferrixel_produced_with_the_same_color_signals() {
    let reference = Rows::default();
    for (depth, chroma, threads) in [
        (10, ChromaSubsampling::Yuv444, 1),
        (10, ChromaSubsampling::Yuv444, 0),
        (8, ChromaSubsampling::Yuv444, 1),
        (10, ChromaSubsampling::Yuv420, 1),
    ] {
        let options = TransformOptions {
            chroma_subsampling: chroma,
            encoder_threads: threads,
            ..options(96, 64)
        };
        transcode_with(JPEG, ImageFormat::Jpeg, options, &reference).unwrap();
        let rows = reference.0.lock().unwrap().clone();
        let output =
            transcode_with(JPEG, ImageFormat::Jpeg, options, &LibavifBackend { depth }).unwrap();
        assert_eq!(&output.bytes[4..8], b"ftyp");
        assert_eq!(output.report.encoder_backend, "libavif-rs/aom");
        assert!(output.report.opaque_encoder_memory);
        assert!(output.report.encoder_output_frame_bytes >= 96 * 64 * 4);
        let decoded = decode(&output.bytes);
        assert_eq!((decoded.width, decoded.height), (96, 64));
        assert!(!decoded.alpha, "opaque output has no alpha item");
        assert_eq!(decoded.depth, u32::from(depth));
        // The same signals as Ferrixel's rav1e sink: BT.709 primaries, sRGB
        // transfer, BT.601 matrix, full range.
        assert_eq!(decoded.cicp, (1, 13, 6, true));
        // A wrong conversion shifts every pixel; the remaining error is the
        // lossy encode at quality 95.
        let (mean, max) = difference(&decoded.rgba, &rows);
        let limit = if chroma == ChromaSubsampling::Yuv420 {
            4.0
        } else {
            2.0
        };
        assert!(
            mean < limit,
            "{depth}-bit {chroma:?}: mean {mean}, max {max}"
        );
    }
}

#[test]
fn straight_alpha_survives_the_encode() {
    // Ferrixel's AVIF input needs rav1d, which cannot be linked next to
    // libavif-rs (see `decode`), so the backend is driven directly with the
    // rows the pipeline would hand it.
    let (width, height) = (24_u32, 16_u32);
    let row = |y: u32| -> Vec<u8> {
        (0..width)
            .flat_map(|x| {
                let alpha = u8::try_from(x * 255 / (width - 1)).unwrap();
                let rgb = [
                    u8::try_from(x * 10).unwrap(),
                    u8::try_from(y * 15).unwrap(),
                    120,
                ];
                if alpha == 0 {
                    [0, 0, 0, 0]
                } else {
                    [rgb[0], rgb[1], rgb[2], alpha]
                }
            })
            .collect()
    };
    let backend = LibavifBackend::default();
    let mut request = backend_request(width, height);
    request.has_alpha = true;
    let mut encoder = backend.begin(&request).unwrap();
    let mut expected = Vec::new();
    for y in 0..height {
        let rgba = row(y);
        encoder.write_row(y, &rgba).unwrap();
        expected.extend_from_slice(&rgba);
    }
    let image = encoder.finish().unwrap();
    // The RGBA frame and 10-bit Y, Cb, Cr and alpha planes.
    assert_eq!(
        image.frame_materialization_bytes,
        (width * height * 4 + 2 * width * height * 4) as usize
    );
    let decoded = decode(&image.bytes);
    assert!(decoded.alpha, "the output carries an alpha item");
    let alpha_of = |data: &[u8]| data.chunks(4).map(|p| p[3]).collect::<Vec<_>>();
    let (mean, _) = difference(&alpha_of(&decoded.rgba), &alpha_of(&expected));
    assert!(mean < 2.0, "alpha mean difference {mean}");
}

/// A request as `transcode_with` would build it for an opaque image.
fn backend_request(width: u32, height: u32) -> EncodeRequest {
    struct Capture<'a>(&'a Mutex<Option<EncodeRequest>>, &'a Rows);
    impl EncoderBackend for Capture<'_> {
        fn format(&self) -> OutputFormat {
            OutputFormat::Avif
        }
        fn name(&self) -> &'static str {
            "capture"
        }
        fn begin(&self, request: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
            *self.0.lock().unwrap() = Some(*request);
            self.1.begin(request)
        }
    }
    let reference = Rows::default();
    let seen = Mutex::new(None);
    transcode_with(
        JPEG,
        ImageFormat::Jpeg,
        options(width, height),
        &Capture(&seen, &reference),
    )
    .unwrap();
    seen.into_inner().unwrap().unwrap()
}

#[test]
fn rows_out_of_order_or_missing_fail() {
    let backend = LibavifBackend::default();
    let request = backend_request(4, 3);
    let mut encoder = backend.begin(&request).unwrap();
    assert!(encoder.write_row(1, &[0; 16]).is_err());
    let mut encoder = backend.begin(&request).unwrap();
    assert!(encoder.write_row(0, &[0; 12]).is_err());
    let mut encoder = backend.begin(&request).unwrap();
    encoder.write_row(0, &[0; 16]).unwrap();
    assert!(encoder.finish().is_err());
}

#[test]
fn invalid_settings_are_rejected_before_decoding() {
    let cases = [
        (LibavifBackend::default(), options(8, 8).quality, 11),
        (LibavifBackend { depth: 12 }, 50.0, 6),
        (LibavifBackend::default(), 101.0, 6),
        (LibavifBackend::default(), f32::NAN, 6),
    ];
    for (backend, quality, speed) in cases {
        let error = transcode_with(
            JPEG,
            ImageFormat::Jpeg,
            TransformOptions {
                quality,
                speed,
                ..options(8, 8)
            },
            &backend,
        )
        .unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::InvalidOptions, "{error}");
    }
    // libavif's own scale: quality 0 is valid.
    let output = transcode_with(
        JPEG,
        ImageFormat::Jpeg,
        TransformOptions {
            quality: 0.0,
            speed: 10,
            ..options(8, 8)
        },
        &LibavifBackend::default(),
    )
    .unwrap();
    assert_eq!(decode(&output.bytes).width, 8);
}
