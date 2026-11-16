//! End to end: Ferrisma's pipeline with the libavif-rs backend, checked by
//! decoding the result with libavif-rs and comparing it with the rows
//! Ferrisma handed to the backend. Independent decoder checks are documented
//! separately in ADR-0024.

use std::sync::Mutex;

use ferrisma::pipeline::{
    AvifOptions, BatchItem, ChromaSubsampling, EncodeError, EncodeErrorKind, EncodeRequest,
    EncodedImage, EncoderBackend, Engine, ImageFormat, InputProfilePolicy, OutputFormat,
    ProcessingErrorKind, RowEncoder, TransformOptions, WorkingPrecision, transcode, transcode_with,
};
use ferrisma_libavif::LibavifBackend;

const JPEG: &[u8] = include_bytes!("../../../tests/fixtures/sample.jpg");

/// Records the RGBA rows Ferrisma produces, as the reference for a decode:
/// 8-bit rows as they are, deeper rows (since 2026-11-15) rounded to 8 bits
/// for the comparison with the 8-bit decode.
#[derive(Default)]
struct Rows(Mutex<Vec<u8>>);

struct RowSink<'a>(&'a Rows, u32);

impl EncoderBackend for Rows {
    fn format(&self) -> OutputFormat {
        OutputFormat::Avif
    }
    fn name(&self) -> &'static str {
        "rows"
    }
    fn row_depths(&self) -> &'static [u8] {
        &[8, 10, 12]
    }
    fn begin(&self, request: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        self.0.lock().unwrap().clear();
        Ok(Box::new(RowSink(self, (1 << request.depth) - 1)))
    }
}

impl RowEncoder for RowSink<'_> {
    fn write_row(&mut self, _: u32, rgba: &[u8]) -> Result<(), EncodeError> {
        self.0.0.lock().unwrap().extend_from_slice(rgba);
        Ok(())
    }
    fn write_row_u16(&mut self, _: u32, rgba: &[u16]) -> Result<(), EncodeError> {
        let max = self.1;
        self.0.0.lock().unwrap().extend(
            rgba.iter()
                .map(|&v| u8::try_from((u32::from(v) * 255 + max / 2) / max).unwrap()),
        );
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
    alpha_premultiplied: bool,
    depth: u32,
    /// Color primaries, transfer, matrix and full range.
    cicp: (u16, u16, u16, bool),
    chroma: u32,
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
            alpha_premultiplied: (*image).alphaPremultiplied != 0,
            depth: (*image).depth,
            chroma: (*image).yuvFormat,
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
        // Alpha at the color quality, as libavif did before the alpha
        // quality became its own setting on 2026-11-15.
        avif: AvifOptions {
            quality: 95,
            alpha_quality: 95,
            speed: 9,
            ..AvifOptions::default()
        },
        ..TransformOptions::default()
    }
}

#[test]
fn encodes_what_ferrisma_produced_with_the_same_color_signals() {
    let reference = Rows::default();
    for (depth, chroma, threads) in [
        (10, ChromaSubsampling::Yuv444, 1),
        (10, ChromaSubsampling::Yuv444, 0),
        (8, ChromaSubsampling::Yuv444, 1),
        (10, ChromaSubsampling::Yuv420, 1),
        (8, ChromaSubsampling::Yuv420, 2),
    ] {
        let base = options(96, 64);
        let options = TransformOptions {
            avif: AvifOptions {
                chroma,
                depth: Some(depth),
                ..base.avif
            },
            encoder_threads: threads,
            ..base
        };
        transcode_with(JPEG, ImageFormat::Jpeg, options, &reference).unwrap();
        let rows = reference.0.lock().unwrap().clone();
        // The depth reaches the backend in the request (since 2026-11-15).
        let output = transcode_with(JPEG, ImageFormat::Jpeg, options, &LibavifBackend).unwrap();
        assert_eq!(&output.bytes[4..8], b"ftyp");
        let built_in = transcode(JPEG, ImageFormat::Jpeg, options).unwrap();
        assert_eq!(
            built_in.bytes, output.bytes,
            "built-in and compatibility adapter must agree"
        );
        assert_eq!(built_in.report.encoder_backend, "libavif-rs/aom");
        assert_eq!(output.report.encoder_backend, "libavif-rs/aom");
        assert!(output.report.opaque_encoder_memory);
        assert!(output.report.encoder_output_frame_bytes >= 96 * 64 * 4);
        assert!(
            output.report.peak_managed_bytes >= output.report.encoder_output_frame_bytes,
            "known adapter frames must count toward the managed peak"
        );
        let decoded = decode(&output.bytes);
        assert_eq!((decoded.width, decoded.height), (96, 64));
        assert!(!decoded.alpha, "opaque output has no alpha item");
        assert_eq!(decoded.depth, u32::from(depth));
        assert_eq!(
            decoded.chroma,
            if chroma == ChromaSubsampling::Yuv444 {
                1
            } else {
                3
            }
        );
        // BT.709 primaries, sRGB transfer, BT.601 matrix, full range (the
        // signals the removed rav1e sink also wrote).
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
    // Drive the row contract directly to exercise nonopaque samples.
    let (width, height) = (25_u32, 17_u32);
    let row = |y: u32| -> Vec<u8> {
        (0..width)
            .flat_map(|x| {
                let alpha = u8::try_from(x * 255 / (width - 1)).unwrap();
                let rgb = [
                    u8::try_from(x * 10).unwrap(),
                    u8::try_from(y * 14).unwrap(),
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
    let mut request = backend_request(width, height);
    request.has_alpha = true;
    for (depth, chroma, threads) in [
        (8, ChromaSubsampling::Yuv444, 1),
        (10, ChromaSubsampling::Yuv444, 2),
        (8, ChromaSubsampling::Yuv420, 2),
        (10, ChromaSubsampling::Yuv420, 1),
    ] {
        let backend = LibavifBackend;
        // The rows arrive at the request's depth (since 2026-11-15): 10-bit
        // rows are the 8-bit pattern scaled to 10 bits.
        request.depth = depth;
        request.avif.depth = Some(depth);
        request.avif.chroma = chroma;
        request.threads = threads;
        let expected_bytes = backend.required_frame_bytes(&request).unwrap();
        let mut encoder = backend.begin(&request).unwrap();
        let mut expected = Vec::new();
        for y in 0..height {
            let rgba = row(y);
            if depth > 8 {
                let deep: Vec<u16> = rgba
                    .iter()
                    .map(|&v| u16::try_from((u32::from(v) * 1023 + 127) / 255).unwrap())
                    .collect();
                encoder.write_row_u16(y, &deep).unwrap();
            } else {
                encoder.write_row(y, &rgba).unwrap();
            }
            expected.extend_from_slice(&rgba);
        }
        let image = encoder.finish().unwrap();
        assert_eq!(image.frame_materialization_bytes, expected_bytes);
        let decoded = decode(&image.bytes);
        assert!(decoded.alpha, "the output carries an alpha item");
        assert!(!decoded.alpha_premultiplied, "output alpha is straight");
        assert_eq!(decoded.depth, u32::from(depth));
        assert_eq!(decoded.cicp, (1, 13, 6, true));
        let alpha_of = |data: &[u8]| data.chunks(4).map(|p| p[3]).collect::<Vec<_>>();
        let (mean, _) = difference(&alpha_of(&decoded.rgba), &alpha_of(&expected));
        let colors = |data: &[u8]| {
            data.chunks(4)
                .zip(expected.chunks(4))
                .filter(|(_, reference)| reference[3] >= 128)
                .flat_map(|(pixel, _)| pixel[..3].iter().copied())
                .collect::<Vec<_>>()
        };
        let (color_mean, _) = difference(&colors(&decoded.rgba), &colors(&expected));
        assert!(
            color_mean < 6.0,
            "{depth}-bit {chroma:?}: color mean difference {color_mean}"
        );
        assert!(
            mean < 2.0,
            "{depth}-bit {chroma:?}: alpha mean difference {mean}"
        );
    }
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
    let backend = LibavifBackend;
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
    let base = options(8, 8).avif;
    let cases = [
        AvifOptions { speed: 11, ..base },
        // 12 bits is admitted since 2026-11-15; 9 is not an AV1 depth.
        AvifOptions {
            depth: Some(9),
            ..base
        },
        AvifOptions {
            quality: 101,
            ..base
        },
        AvifOptions {
            alpha_quality: 101,
            ..base
        },
    ];
    for avif in cases {
        let error = transcode_with(
            JPEG,
            ImageFormat::Jpeg,
            TransformOptions {
                avif,
                ..options(8, 8)
            },
            &LibavifBackend,
        )
        .unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::InvalidOptions, "{error}");
    }
    // libavif's own scale: quality 0 is valid.
    let output = transcode_with(
        JPEG,
        ImageFormat::Jpeg,
        TransformOptions {
            avif: AvifOptions {
                quality: 0,
                alpha_quality: 0,
                speed: 10,
                ..base
            },
            ..options(8, 8)
        },
        &LibavifBackend,
    )
    .unwrap();
    assert_eq!(decode(&output.bytes).width, 8);
}

#[test]
fn frame_preflight_counts_odd_chroma_and_alpha_planes() {
    let mut request = backend_request(5, 3);
    // The RGBA frame is 4 B/px at 8 bits and 8 B/px above (the rows arrive
    // at the depth since 2026-11-15), plus the planes at 1 or 2 B/sample.
    for (depth, chroma, alpha, expected) in [
        (8, ChromaSubsampling::Yuv444, false, 60 + 15 * 3),
        (10, ChromaSubsampling::Yuv444, true, 120 + 2 * 15 * 4),
        (8, ChromaSubsampling::Yuv420, true, 60 + 15 + 2 * 3 * 2 + 15),
        (
            10,
            ChromaSubsampling::Yuv420,
            false,
            120 + 2 * (15 + 2 * 3 * 2),
        ),
        (12, ChromaSubsampling::Yuv444, false, 120 + 2 * 15 * 3),
    ] {
        request.avif.chroma = chroma;
        request.depth = depth;
        request.avif.depth = Some(depth);
        request.has_alpha = alpha;
        assert_eq!(
            LibavifBackend.required_frame_bytes(&request).unwrap(),
            expected
        );
    }
}

#[test]
fn invalid_geometry_is_rejected_before_allocation() {
    let backend = LibavifBackend;
    let mut request = backend_request(4, 3);
    for (width, height) in [(0, 3), (4, 0), (u32::MAX, 1), (u32::MAX / 4, u32::MAX)] {
        request.width = width;
        request.height = height;
        let Err(error) = backend.begin(&request) else {
            panic!("accepted {width}x{height}")
        };
        assert_eq!(
            error.kind(),
            EncodeErrorKind::InvalidOptions,
            "{width}x{height}: {error}"
        );
    }
    request.width = 4;
    request.height = 3;
    request.format = OutputFormat::WebP;
    let Err(error) = backend.begin(&request) else {
        panic!("accepted WebP output")
    };
    assert_eq!(error.kind(), EncodeErrorKind::InvalidOptions);
}

#[test]
fn output_limit_is_enforced_after_backend_finish() {
    let backend = LibavifBackend;
    let opts = options(16, 12);
    let output = transcode_with(JPEG, ImageFormat::Jpeg, opts, &backend).unwrap();
    let mut limited = opts;
    limited.max_output_bytes = output.bytes.len() - 1;
    let error = transcode_with(JPEG, ImageFormat::Jpeg, limited, &backend).unwrap_err();
    assert_eq!(error.kind(), ProcessingErrorKind::ResourceExhausted);
}

#[cfg(feature = "cli")]
#[test]
fn generated_avif_can_be_read_and_written_in_the_same_pipeline() {
    let backend = LibavifBackend;
    let first = transcode_with(JPEG, ImageFormat::Jpeg, options(32, 24), &backend).unwrap();
    let second =
        transcode_with(&first.bytes, ImageFormat::Avif, options(16, 12), &backend).unwrap();
    let decoded = decode(&second.bytes);
    assert_eq!((decoded.width, decoded.height), (16, 12));
    assert_eq!(decoded.cicp, (1, 13, 6, true));
    assert_eq!(second.report.encoder_backend, "libavif-rs/aom");
    let mut request = backend_request(8, 8);
    request.has_alpha = true;
    let mut frame = backend.begin(&request).unwrap();
    for y in 0..8 {
        let row: Vec<u8> = (0..8).flat_map(|x| [160, 40, 80, (x * 32) as u8]).collect();
        frame.write_row(y, &row).unwrap();
    }
    let transparent = frame.finish().unwrap();
    let output = transcode(
        &transparent.bytes,
        ImageFormat::Avif,
        TransformOptions {
            avif: AvifOptions {
                depth: Some(8),
                ..options(8, 8).avif
            },
            ..options(8, 8)
        },
    )
    .unwrap();
    let decoded = decode(&output.bytes);
    assert!(decoded.alpha && !decoded.alpha_premultiplied);
    assert_eq!(decoded.depth, 8);
    assert_eq!(decoded.cicp, (1, 13, 6, true));
}

#[test]
fn own_encoder_facade_handles_reference_batch_and_invalid_settings() {
    let base = options(16, 12);
    let opts = TransformOptions {
        avif: AvifOptions {
            depth: Some(10),
            ..base.avif
        },
        output_width: 16,
        output_height: 12,
        transform_threads: 1,
        encoder_threads: 1,
        precision: WorkingPrecision::F64Reference,
        ..base
    };
    let single = transcode(JPEG, ImageFormat::Jpeg, opts).unwrap();
    assert_eq!(single.report.encoder_backend, "libavif-rs/aom");
    let results = Engine::new(2).transcode_batch(&[
        BatchItem {
            encoded: JPEG,
            format: ImageFormat::Jpeg,
            options: opts,
        },
        BatchItem {
            encoded: JPEG,
            format: ImageFormat::Jpeg,
            options: opts,
        },
    ]);
    for output in results {
        assert_eq!(output.unwrap().bytes, single.bytes);
    }
    for avif in [
        AvifOptions {
            depth: Some(9),
            ..opts.avif
        },
        AvifOptions {
            quality: 101,
            ..opts.avif
        },
        AvifOptions {
            speed: 11,
            ..opts.avif
        },
    ] {
        let bad = TransformOptions { avif, ..opts };
        assert_eq!(
            transcode(JPEG, ImageFormat::Jpeg, bad).unwrap_err().kind(),
            ProcessingErrorKind::InvalidOptions
        );
    }
    let quality_zero = transcode(
        JPEG,
        ImageFormat::Jpeg,
        TransformOptions {
            avif: AvifOptions {
                quality: 0,
                speed: 0,
                ..opts.avif
            },
            ..opts
        },
    )
    .unwrap();
    assert_eq!(quality_zero.report.encoder_backend, "libavif-rs/aom");
    // AVIF settings other than the defaults conflict with non-AVIF output
    // (an AVIF encoder selection did until 2026-11-15, then `avif_depth`).
    let webp_conflict = transcode(
        JPEG,
        ImageFormat::Jpeg,
        TransformOptions {
            output_format: OutputFormat::WebP,
            avif: AvifOptions {
                depth: Some(10),
                ..AvifOptions::default()
            },
            ..TransformOptions::default()
        },
    );
    assert_eq!(
        webp_conflict.unwrap_err().kind(),
        ProcessingErrorKind::InvalidOptions
    );
    let limited = TransformOptions {
        max_managed_bytes: 100,
        ..opts
    };
    assert_eq!(
        transcode(JPEG, ImageFormat::Jpeg, limited)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::ResourceExhausted
    );
    let limited = TransformOptions {
        max_output_bytes: single.bytes.len() - 1,
        ..opts
    };
    assert_eq!(
        transcode(JPEG, ImageFormat::Jpeg, limited)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::ResourceExhausted
    );
    // The depth is part of the request since 2026-11-15, so the built-in
    // writer and the same backend through `transcode_with` agree (until
    // then a nondefault depth with an external backend was refused).
    let external = transcode_with(JPEG, ImageFormat::Jpeg, opts, &LibavifBackend)
        .expect("every AVIF setting reaches the backend");
    assert_eq!(external.report.encoder_backend, "libavif-rs/aom");
    assert_eq!(external.bytes, single.bytes);
}

#[cfg(feature = "cli")]
#[test]
fn compatibility_cli_keeps_default_backend_and_rejects_conflicting_selection() {
    let input = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/sample.jpg"
    );
    let output = std::env::temp_dir().join(format!(
        "ferrisma-compat-avif-{}-cli.avif",
        std::process::id()
    ));
    let run = |flags: &[&str]| {
        std::process::Command::new(env!("CARGO_BIN_EXE_ferrisma-libavif"))
            .args([
                input,
                output.to_str().unwrap(),
                "8",
                "8",
                "--assume-srgb",
                "--encoder-threads=1",
            ])
            .args(flags)
            .output()
            .unwrap()
    };
    let result = run(&[]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stdout).contains("encoder=libavif-rs/aom"));
    // `--avif-depth=` reaches this backend since 2026-11-15 (it was refused
    // with an external backend before).
    let result = run(&["--avif-depth=10"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    // The removed `--avif-encoder=` and `--quality=` are unknown options
    // (2026-11-15).
    for (flags, message) in [
        (["--avif-encoder=libavif"], "unknown option"),
        (["--avif-encoder=rav1e"], "unknown option"),
        (["--quality=56"], "unknown option"),
    ] {
        let conflict = run(&flags);
        assert!(!conflict.status.success());
        assert!(
            String::from_utf8_lossy(&conflict.stderr).contains(message),
            "{flags:?}"
        );
    }
    std::fs::remove_file(output).unwrap();
}

#[cfg(feature = "cli")]
#[test]
fn independent_oriented_avif_reference_reaches_own_encoder() {
    let input = include_bytes!("../../../tests/fixtures/orientation/avif-6.avif");
    let reference = include_bytes!("../../../tests/fixtures/orientation/avif-6.rgb");
    let output = transcode(
        input,
        ImageFormat::Avif,
        TransformOptions {
            avif: AvifOptions {
                quality: 100,
                alpha_quality: 100,
                speed: 10,
                depth: Some(10),
                ..AvifOptions::default()
            },
            encoder_threads: 1,
            ..options(16, 24)
        },
    )
    .unwrap();
    let image = decode(&output.bytes);
    assert_eq!((image.width, image.height), (16, 24));
    let actual: Vec<u8> = image
        .rgba
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|pixel| pixel[..3].iter().copied())
        .collect();
    let (mean, max) = difference(&actual, reference);
    assert!(mean < 5.0, "oriented reference: mean {mean}, max {max}");
}
