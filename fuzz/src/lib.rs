//! Shared decoder/encoder fuzz driver. The first byte is a control byte;
//! every remaining byte is passed unchanged to the selected image decoder.

use ferrisma::pipeline::{
    ImageFormat, InputProfilePolicy, JpegXlOptions, Output, OutputFormat, ProcessingError,
    ResizeFit, ResizeOptions, TransformOptions, WorkingPrecision, transcode, transcode_resized,
};

const MAX_INPUT_BYTES: usize = 512 * 1024;
const MAX_INPUT_PIXELS: u64 = 1_000_000;
const MAX_OUTPUT_BYTES: usize = 512 * 1024;
const MAX_MANAGED_BYTES: usize = 2 * 1024 * 1024;

/// Returns `None` for a missing selector or an oversized input. Other errors
/// are expected fuzz outcomes; a successful result reached the output encoder.
///
/// Every lane pins its AVIF depth explicitly, so the engine default never
/// changes what a lane covers. Since 2026-11-15 the engine has one reader
/// per input format and one AVIF writer (the workspace libavif/AOM port):
/// the former `transcode_own_jxl` lane (another JPEG XL reader) and
/// `transcode_own_avif` lane (another AVIF encoder) are merged into this
/// one. This runs the 10-bit writer; [`avif_depths`] lists the depths the
/// `transcode` target runs for one input.
pub fn run_case(bytes: &[u8]) -> Option<Result<Output, ProcessingError>> {
    run_case_at_depth(bytes, 10)
}

/// The AVIF depths the `transcode` target runs for `bytes`: 10 and 8 for an
/// AVIF-output selector (bit 2 clear), only the default otherwise, as the
/// former `transcode` and `transcode_own_avif` lanes did together.
pub fn avif_depths(bytes: &[u8]) -> &'static [u8] {
    match bytes.first() {
        Some(control) if control & 4 == 0 => &[10, 8],
        _ => &[10],
    }
}

/// [`run_case`] with AVIF output at `depth` (8 or 10); WebP output keeps the
/// default depth, which it requires.
pub fn run_case_at_depth(bytes: &[u8], depth: u8) -> Option<Result<Output, ProcessingError>> {
    run_case_with_depth(bytes, depth)
}

/// Engine-managed memory of the JPEG XL output lane. The lossless sink
/// reserves its measured one-shot peak, about 76 bytes per opaque pixel and
/// 101 with alpha (issue #223), on top of the 4-byte input frame. 32 MiB
/// admits 320x320 with alpha (about 10 MiB) and the resample plan of the
/// 8192x1 and 1x8192 classes (about 18 MB for a small input), so those
/// reach the encoder. A request that does not fit is rejected with
/// `ResourceExhausted`, which the lane accepts.
const JXL_OUT_MAX_MANAGED_BYTES: usize = 32 * 1024 * 1024;

/// What the JPEG XL output lane asked for, so the target can check the result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JxlOutRequest {
    /// Lossless effort 2 rather than lossy VarDCT.
    pub lossless: bool,
    /// The requested output size.
    pub geometry: JxlOutGeometry,
}

/// Output geometry of the JPEG XL output lane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JxlOutGeometry {
    /// Exactly this width and height (`transcode`).
    Exact(u32, u32),
    /// Fit inside a square of this edge, never enlarged (`transcode_resized`):
    /// small inputs keep their own size, so their alpha edges stay intact.
    Inside(u32),
}

/// The JPEG XL output lane. Control byte, low bit first:
///
/// - bits 0–1: input JPEG (0), JPEG XL (1), AVIF (2), WebP (3);
/// - bit 2: lossy `VarDCT` (0) or lossless effort 2 (1);
/// - bits 3–4: the profile policy, as in [`run_case`];
/// - bits 5–7: geometry class: 0 small (1–16 per axis, from the input
///   length), 1 8192x1, 2 1x8192, 3 64x64, 4 inside 64x64 without
///   enlargement (keeps a small input's alpha), 5 inside 320x320 without
///   enlargement (crosses the 256-pixel group boundary), 6 and 7 64x64 at
///   the smallest admitted lossy distance (0.05) and the largest below 3.0;
///   lossless ignores the distance. Every other lossy class uses d1.0.
///
/// The encoder is serial (it requires `encoder_threads == 1`), so no bit
/// selects a thread count. JPEG XL input uses the build's default reader,
/// the workspace libjxl-rs decoder under `cli`.
pub fn run_jxl_out_case(bytes: &[u8]) -> Option<(JxlOutRequest, Result<Output, ProcessingError>)> {
    let (&control, encoded) = bytes.split_first()?;
    if encoded.is_empty() || encoded.len() > MAX_INPUT_BYTES {
        return None;
    }
    let kind = match control & 3 {
        0 => ImageFormat::Jpeg,
        1 => ImageFormat::JpegXl,
        2 => ImageFormat::Avif,
        _ => ImageFormat::WebP,
    };
    let lossless = control & 4 != 0;
    let profile_policy = match (control >> 3) & 3 {
        0 if kind == ImageFormat::JpegXl => InputProfilePolicy::UseRenderedIcc,
        0 | 2 => InputProfilePolicy::AssumeSrgbIfMissing,
        1 => InputProfilePolicy::Strict,
        _ => InputProfilePolicy::UseRenderedIcc,
    };
    let class = control >> 5;
    // Mutations change the length, so the small class covers all sizes.
    let small = u32::try_from(encoded.len() % 256).unwrap_or(0);
    let geometry = match class {
        0 => JxlOutGeometry::Exact(1 + small % 16, 1 + small / 16),
        1 => JxlOutGeometry::Exact(8192, 1),
        2 => JxlOutGeometry::Exact(1, 8192),
        4 => JxlOutGeometry::Inside(64),
        5 => JxlOutGeometry::Inside(320),
        _ => JxlOutGeometry::Exact(64, 64),
    };
    let distance = match class {
        6 => JpegXlOptions::MIN_LOSSY_DISTANCE,
        7 => 2.999,
        _ => 1.0,
    };
    let (output_width, output_height) = match geometry {
        JxlOutGeometry::Exact(width, height) => (width, height),
        JxlOutGeometry::Inside(_) => (1, 1),
    };
    let options = TransformOptions {
        output_format: OutputFormat::JpegXl,
        jpeg_xl: if lossless {
            JpegXlOptions::Lossless { effort: 2 }
        } else {
            JpegXlOptions::Lossy { distance }
        },
        output_width,
        output_height,
        profile_policy,
        transform_threads: 1,
        encoder_threads: 1,
        max_input_bytes: MAX_INPUT_BYTES,
        max_input_pixels: MAX_INPUT_PIXELS,
        max_output_bytes: MAX_OUTPUT_BYTES,
        max_managed_bytes: JXL_OUT_MAX_MANAGED_BYTES,
        ..TransformOptions::default()
    };
    let result = match geometry {
        JxlOutGeometry::Exact(..) => transcode(encoded, kind, options),
        JxlOutGeometry::Inside(edge) => transcode_resized(
            encoded,
            kind,
            options,
            ResizeOptions {
                width: Some(edge),
                height: Some(edge),
                fit: ResizeFit::Inside,
                without_enlargement: true,
            },
        ),
    };
    Some((JxlOutRequest { lossless, geometry }, result))
}

/// The JPEG XL codestream signature and the container's signature box.
const JXL_CODESTREAM: [u8; 2] = [0xff, 0x0a];
const JXL_CONTAINER: [u8; 12] = [
    0, 0, 0, 0x0c, b'J', b'X', b'L', b' ', 0x0d, 0x0a, 0x87, 0x0a,
];

/// Cheap invariants of a successful JPEG XL output; panics on a violation,
/// which libFuzzer reports as a crash. The output is decoded back with the
/// workspace decoder and its dimensions (not its pixels) are compared.
///
/// # Panics
/// When the output is not JPEG XL, exceeds the output limit, has other
/// dimensions than requested, or does not decode.
pub fn assert_jxl_output(request: &JxlOutRequest, output: &Output) {
    assert_eq!(output.format, OutputFormat::JpegXl);
    assert!(
        output.bytes.starts_with(&JXL_CODESTREAM) || output.bytes.starts_with(&JXL_CONTAINER),
        "output lacks a JPEG XL signature"
    );
    assert!(output.bytes.len() <= MAX_OUTPUT_BYTES);
    match request.geometry {
        JxlOutGeometry::Exact(width, height) => {
            assert_eq!((output.width, output.height), (width, height));
        }
        JxlOutGeometry::Inside(edge) => {
            assert!((1..=edge).contains(&output.width) && (1..=edge).contains(&output.height));
        }
    }
    let pixels = u64::from(output.width) * u64::from(output.height);
    let info = ferrisma_jxl_decoder::inspect(&output.bytes, pixels)
        .unwrap_or_else(|error| panic!("the engine's JPEG XL output does not parse: {error:?}"));
    assert_eq!((info.width, info.height), (output.width, output.height));
    assert!(matches!(info.channels, 3 | 4), "{} channels", info.channels);
    let mut samples = vec![0.0; info.sample_count().expect("checked size")];
    ferrisma_jxl_decoder::decode_into(&output.bytes, &info, &mut samples, pixels)
        .unwrap_or_else(|error| panic!("the engine's JPEG XL output does not decode: {error:?}"));
}

fn run_case_with_depth(bytes: &[u8], depth: u8) -> Option<Result<Output, ProcessingError>> {
    let (&control, encoded) = bytes.split_first()?;
    if encoded.is_empty() || encoded.len() > MAX_INPUT_BYTES {
        return None;
    }
    let kind = match control & 3 {
        0 => ImageFormat::Jpeg,
        1 => ImageFormat::JpegXl,
        2 => ImageFormat::Avif,
        _ => ImageFormat::WebP,
    };
    let output_format = if control & 4 == 0 {
        OutputFormat::Avif
    } else {
        OutputFormat::WebP
    };
    // Select an explicit policy; no profile is guessed when one is present.
    // Policy 0 keeps the rendered-ICC policy for JPEG XL, which before
    // 2026-11-15 described every file by an ICC profile; it now reads enum
    // encodings as typed signals under every policy (owner decision, #222).
    let profile_policy = match (control >> 3) & 3 {
        0 if kind == ImageFormat::JpegXl => InputProfilePolicy::UseRenderedIcc,
        0 | 2 => InputProfilePolicy::AssumeSrgbIfMissing,
        1 => InputProfilePolicy::Strict,
        _ => InputProfilePolicy::UseRenderedIcc,
    };
    let precision = if control & 0x20 == 0 {
        WorkingPrecision::F32
    } else {
        WorkingPrecision::F64Reference
    };
    let (output_width, output_height) = match control >> 6 {
        0 => (1 + u32::from(control & 7), 1 + u32::from(encoded[0] % 8)),
        1 => (8192, 1),
        2 => (1, 8192),
        _ => (64, 64),
    };
    let options = TransformOptions {
        output_format,
        // A nondefault AVIF depth conflicts with WebP output; WebP keeps the
        // default, like the other shared AVIF options.
        avif_depth: if output_format == OutputFormat::Avif {
            depth
        } else {
            10
        },
        output_width,
        output_height,
        profile_policy,
        precision,
        // WebP requires these shared AVIF options to retain their defaults.
        quality: if output_format == OutputFormat::Avif {
            60.0
        } else {
            75.0
        },
        speed: if output_format == OutputFormat::Avif {
            10
        } else {
            6
        },
        transform_threads: 1,
        encoder_threads: 1,
        max_input_bytes: MAX_INPUT_BYTES,
        max_input_pixels: MAX_INPUT_PIXELS,
        max_output_bytes: MAX_OUTPUT_BYTES,
        max_managed_bytes: MAX_MANAGED_BYTES,
        ..TransformOptions::default()
    };
    Some(transcode(encoded, kind, options))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrisma::pipeline::ProcessingErrorKind;

    const JPEG: &[u8] = include_bytes!("../../tests/fixtures/sample.jpg");
    const JXL: &[u8] = include_bytes!("../../tests/fixtures/sample.jxl");
    const AVIF: &[u8] = include_bytes!("../../tests/fixtures/orientation/avif-1.avif");
    const WEBP: &[u8] = include_bytes!("../../tests/fixtures/webp/reference-lossy.webp");

    fn framed(selector: u8, image: &[u8]) -> Vec<u8> {
        let mut bytes = vec![selector];
        bytes.extend_from_slice(image);
        bytes
    }

    #[test]
    fn framed_seeds_reach_the_transform_and_both_encoders() {
        for (selector, image) in [(0, JPEG), (1, JXL), (2, AVIF), (3, WEBP)] {
            for (flag, format) in [(0, OutputFormat::Avif), (4, OutputFormat::WebP)] {
                let output = run_case(&framed(selector | flag, image))
                    .expect("input admitted by the driver")
                    .unwrap_or_else(|error| panic!("selector {}: {error}", selector | flag));
                assert_eq!(output.format, format);
                assert_eq!(output.report.decoder_requests, 1);
                assert!(output.width > 0 && output.height > 0);
                assert!(!output.bytes.is_empty());
                match format {
                    OutputFormat::Avif => assert_eq!(&output.bytes[4..8], b"ftyp"),
                    OutputFormat::WebP => assert_eq!(&output.bytes[..4], b"RIFF"),
                    // `OutputFormat` is non-exhaustive (ADR-0040).
                    other => panic!("unexpected output format {other:?}"),
                }
            }
        }
    }

    const RGBA_JXL: &[u8] = include_bytes!("../../tests/fixtures/jxl-lossless/rgba-37x19.jxl");

    /// Runs one framed seed through the JPEG XL output lane and checks the
    /// target's invariants on success.
    fn jxl_out(selector: u8, image: &[u8]) -> Result<Output, ProcessingError> {
        let (request, result) =
            run_jxl_out_case(&framed(selector, image)).expect("input admitted by the driver");
        if let Ok(output) = &result {
            assert_jxl_output(&request, output);
        }
        result
    }

    #[test]
    fn jxl_output_lane_writes_both_profiles_from_every_input_format() {
        for (selector, image) in [(0, JPEG), (1, JXL), (2, AVIF), (3, WEBP)] {
            // Geometry class 3 (64x64), lossy then lossless, default policy.
            for profile in [0, 4] {
                let control = selector | profile | 3 << 5;
                let output = jxl_out(control, image)
                    .unwrap_or_else(|error| panic!("selector {control:#04x}: {error}"));
                assert_eq!((output.width, output.height), (64, 64));
                assert_eq!(output.report.decoder_requests, 1);
            }
        }
        // The small class 0, the extreme aspect classes (1, 2) and the
        // distance bounds (6, 7) reach the encoder within the lane's budget.
        for class in [0_u8, 1, 2, 6, 7] {
            for profile in [0, 4] {
                let control = profile | class << 5;
                jxl_out(control, JPEG)
                    .unwrap_or_else(|error| panic!("selector {control:#04x}: {error}"));
            }
        }
    }

    #[test]
    fn jxl_output_lane_keeps_alpha_in_both_profiles() {
        for profile in [0, 4] {
            let output = jxl_out(1 | profile | 4 << 5, RGBA_JXL).expect("alpha seed must encode");
            assert_eq!((output.width, output.height), (37, 19));
            let info = ferrisma_jxl_decoder::inspect(&output.bytes, 37 * 19).unwrap();
            assert_eq!(info.channels, 4, "alpha survives the round trip");
        }
        assert!(run_jxl_out_case(&[4]).is_none());
        // Strict policy still rejects the untagged JPEG before encoding.
        let error = jxl_out(4 | 1 << 3, JPEG).unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::UnsupportedColor);
    }

    #[test]
    fn workspace_avif_writer_reaches_both_depths_from_every_input_format() {
        for depth in [8, 10] {
            for (selector, image) in [(0, JPEG), (1, JXL), (2, AVIF), (3, WEBP)] {
                let output = run_case_at_depth(&framed(selector, image), depth)
                    .expect("input admitted by the driver")
                    .expect("fixture must reach the workspace AVIF writer");
                assert_eq!(output.format, OutputFormat::Avif);
                assert_eq!(output.report.encoder_backend, "libavif-rs/aom");
                assert!(!output.bytes.is_empty());
            }
        }
        assert_eq!(avif_depths(&framed(4, JPEG)), &[10]);
        assert_eq!(avif_depths(&framed(0, JPEG)), &[10, 8]);
    }

    // The workflow supplies its generated corpus here, so these results cover
    // the exact seed bytes that are passed to libFuzzer, including rejections.
    #[test]
    fn campaign_seeds_report_reachability() {
        type CaseRunner = fn(&[u8]) -> Option<Result<Output, ProcessingError>>;
        let Some(root) = std::env::var_os("FERRISMA_FUZZ_SEEDS") else {
            return;
        };
        let lanes = [
            ("transcode", "depth=10", run_case as CaseRunner),
            (
                "transcode",
                "depth=8",
                (|bytes| run_case_at_depth(bytes, 8)) as CaseRunner,
            ),
            (
                "transcode_jxl_out",
                "jxl-out",
                (|bytes| {
                    let (request, result) = run_jxl_out_case(bytes)?;
                    if let Ok(output) = &result {
                        assert_jxl_output(&request, output);
                    }
                    Some(result)
                }) as CaseRunner,
            ),
        ];
        for (lane, variant, run) in lanes {
            let mut paths: Vec<_> = std::fs::read_dir(std::path::Path::new(&root).join(lane))
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .collect();
            paths.sort();
            assert!(!paths.is_empty());
            for path in paths {
                let data = std::fs::read(&path).unwrap();
                let result = run(&data).expect("campaign seed must be admitted by the driver");
                let name = path.file_name().unwrap().to_string_lossy();
                if name == "icc-valid" || name == "avif-alpha" {
                    assert!(
                        result.is_ok(),
                        "the supported ICC/alpha seed must reach encoding"
                    );
                } else if name.starts_with("jxl-gray-") {
                    // Gray and gray+alpha JPEG XL are expanded to RGB(A)
                    // since 2026-11-15 (owner decision, ADR-0043); until
                    // then these eight seeds were `UnsupportedInput`.
                    assert!(
                        result.is_ok(),
                        "{lane}/{name}: gray JPEG XL must reach encoding: {:?}",
                        result.as_ref().err()
                    );
                } else if name.starts_with("jxl-oversized-toc-section")
                    || name.starts_with("jxl-unknown-colour-space")
                {
                    // The JPEG XL reader, like libjxl, finds the declared
                    // 1 GiB section missing and rejects the unknown colour
                    // space. (Until 2026-11-15 the `transcode` lane read them
                    // with jxl-oxide: `ResourceExhausted` and
                    // `UnsupportedColor`.)
                    assert!(
                        matches!(&result, Err(error) if error.kind() == ProcessingErrorKind::MalformedInput),
                        "{lane}/{name}"
                    );
                } else if name.starts_with("icc-tag-") {
                    assert!(
                        matches!(&result, Err(error) if error.kind() == ProcessingErrorKind::UnsupportedColor)
                    );
                }
                match result {
                    Ok(output) => {
                        assert!(output.report.decoder_requests > 0);
                        assert!(!output.bytes.is_empty());
                        if output.format == OutputFormat::Avif {
                            assert_eq!(output.report.encoder_backend, "libavif-rs/aom");
                        }
                        eprintln!(
                            "{lane}/{name} ({variant}): encoded {:?} via {}",
                            output.format, output.report.encoder_backend
                        );
                    }
                    Err(error) => {
                        eprintln!("{lane}/{name} ({variant}): rejected {:?}", error.kind())
                    }
                }
            }
        }
    }

    /// Smoke run 43690971661 (`docs/fuzzing.md`): one TOC section declaring
    /// 4,211,712 + 2^30 - 1 bytes in an 11-byte codestream, found on the
    /// former jxl-oxide reader. The only JPEG XL reader reserves nothing for
    /// it and, like libjxl, reports the file truncated, in every lane.
    #[test]
    fn oversized_jxl_toc_section_is_a_controlled_error() {
        const HOSTILE: &[u8] =
            include_bytes!("../../tests/fixtures/hostile/jxl-oversized-toc-section.jxl");
        for selector in [1, 1 | 4] {
            let error = run_case(&framed(selector, HOSTILE)).unwrap().unwrap_err();
            assert_eq!(error.kind(), ProcessingErrorKind::MalformedInput);
        }
        let error = run_case_at_depth(&framed(1, HOSTILE), 8)
            .unwrap()
            .unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::MalformedInput);
        let (_, result) = run_jxl_out_case(&framed(1 | 3 << 5, HOSTILE)).unwrap();
        assert_eq!(
            result.unwrap_err().kind(),
            ProcessingErrorKind::MalformedInput
        );
    }

    #[test]
    fn strict_policy_and_input_caps_remain_enforced() {
        let strict_jpeg = run_case(&framed(8, JPEG)).unwrap().unwrap_err();
        assert_eq!(strict_jpeg.kind(), ProcessingErrorKind::UnsupportedColor);
        // An embedded ICC is not replaced by the assume-if-missing policy.
        // (Until 2026-11-15 `sample.jxl`, whose enum sRGB is now a typed
        // signal; owner decision, #222.)
        const ICC_JXL: &[u8] = include_bytes!("../../tests/fixtures/parity/rgb-p3-icc.jxl");
        let tagged_jxl = run_case(&framed(1 | 0x10, ICC_JXL)).unwrap().unwrap_err();
        assert_eq!(tagged_jxl.kind(), ProcessingErrorKind::UnsupportedColor);
        assert!(run_case(&[]).is_none());
        assert!(run_case(&[0]).is_none());
        assert!(run_case(&vec![0; MAX_INPUT_BYTES + 2]).is_none());

        // Only the JPEG SOF dimensions change; admission must reject the
        // declared geometry before attempting to render the truncated data.
        let mut oversized = JPEG.to_vec();
        let sof = oversized
            .windows(2)
            .position(|pair| pair == [0xff, 0xc0])
            .unwrap();
        oversized[sof + 5..sof + 7].copy_from_slice(&1001_u16.to_be_bytes());
        oversized[sof + 7..sof + 9].copy_from_slice(&1001_u16.to_be_bytes());
        let error = run_case(&framed(0, &oversized)).unwrap().unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::InputLimitExceeded);

        // The 8192x1 output lane is admitted by the driver but should not
        // bypass the 2 MiB engine budget while building its filter plan.
        let error = run_case(&framed(0x40, JPEG)).unwrap().unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::ResourceExhausted);
    }
}
