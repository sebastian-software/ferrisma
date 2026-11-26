use webp::api::decode::{ColorMode, Config as DecodeConfig, Container, decode};
use webp::api::encode::{PixelLayout, PixelRows, encode_lossless, encode_lossy};

use crate::geometry::Size;
use crate::memory::MemoryBudget;
use crate::pipeline::{
    BatchItem, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, Engine, ImageFormat,
    InputProfilePolicy, OutputFormat, ProcessingErrorKind, RowEncoder, Threads, TransformOptions,
    WebpOptions, transcode, transcode_with,
};
use crate::pipeline::{OutputGeometry, OutputSettings, WebpLossless};
use crate::webp_sink::{WebpChunkWriter, WebpSink, encode_webp_chunks_for_test};

struct RgbaBackend;
struct RgbaRows(Vec<u8>);
impl EncoderBackend for RgbaBackend {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "rgba-reference"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        Ok(Box::new(RgbaRows(Vec::new())))
    }
}
impl RowEncoder for RgbaRows {
    fn write_row(&mut self, _: u32, rgba: &[u8]) -> Result<(), EncodeError> {
        self.0.extend_from_slice(rgba);
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(OutputFormat::WebP, self.0, 0, false))
    }
}

fn input() -> Vec<u8> {
    let mut pixels = Vec::new();
    for y in 0..8_u8 {
        for x in 0..8_u8 {
            pixels.extend_from_slice(&[
                x * 29,
                y * 31,
                x * 7 + y * 11,
                if x == 0 && y == 0 { 0 } else { 32 + x * 20 },
            ]);
        }
    }
    encode_lossless(PixelRows {
        layout: PixelLayout::Rgba,
        data: &pixels,
        width: 8,
        height: 8,
        stride: 32,
    })
    .unwrap()
}

fn opaque_input() -> Vec<u8> {
    let pixels = [30_u8, 80, 140].repeat(64);
    encode_lossless(PixelRows {
        layout: PixelLayout::Rgb,
        data: &pixels,
        width: 8,
        height: 8,
        stride: 24,
    })
    .unwrap()
}

fn options(lossless: bool) -> TransformOptions {
    TransformOptions::new(OutputGeometry::Exact {
        width: 8,
        height: 8,
    })
    .with_output(OutputSettings::WebP(if lossless {
        WebpOptions::Lossless(WebpLossless::new(4))
    } else {
        WebpOptions::default()
    }))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
}

#[test]
fn still_webp_outputs_decode_with_straight_alpha_and_srgb_metadata() {
    for lossless in [false, true] {
        let output = transcode(&input(), ImageFormat::WebP, &options(lossless)).unwrap();
        assert_eq!(output.format, OutputFormat::WebP);
        assert_eq!(&output.bytes[..4], b"RIFF");
        assert_eq!(&output.bytes[8..12], b"WEBP");
        let container = Container::new(&output.bytes).unwrap();
        let icc = container.icc_profile().unwrap();
        assert_eq!(icc, include_bytes!("../assets/icc/sRGB2014.icc"));
        assert!(container.exif().is_none());
        let decoded = decode(&output.bytes, &DecodeConfig::new(ColorMode::Rgba)).unwrap();
        assert_eq!(decoded.width(), 8);
        assert_eq!(decoded.height(), 8);
        let rows = decoded.rows().unwrap();
        let first = rows.row(0).unwrap();
        assert!(first[3] < 255);
        assert!(first[3] < first[7]);
        if lossless {
            let expected =
                transcode_with(&input(), ImageFormat::WebP, &options(true), &RgbaBackend).unwrap();
            let actual: Vec<u8> = (0..8).flat_map(|y| rows.row(y).unwrap().to_vec()).collect();
            assert_eq!(actual, expected.bytes);
        }
        assert!(output.report.encoder_output_frame_bytes >= 8 * 8 * 4);
        assert!(output.report.opaque_encoder_memory);
    }
}

#[test]
fn webp_output_frame_and_file_limits_fail_before_retaining_result() {
    let tiny = options(false).with_max_managed_bytes(8 * 8 * 4 - 1);
    assert_eq!(
        transcode(&input(), ImageFormat::WebP, &tiny)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::ResourceExhausted
    );
    // The output limit has its own kind since 2026-11-16
    // (`ResourceExhausted` before).
    let tiny = options(false).with_max_output_bytes(1024);
    assert_eq!(
        transcode(&input(), ImageFormat::WebP, &tiny)
            .unwrap_err()
            .kind(),
        ProcessingErrorKind::OutputLimitExceeded
    );
}

#[test]
fn webp_encoder_threads_other_than_one_name_the_serial_encoder() {
    for threads in [0, 2] {
        let threaded = options(false).with_encoder_threads(
            std::num::NonZeroUsize::new(threads).map_or(Threads::Automatic, Threads::Count),
        );
        let error = transcode(&input(), ImageFormat::WebP, &threaded).unwrap_err();
        assert_eq!(error.kind(), ProcessingErrorKind::InvalidOptions);
        assert_eq!(
            error.to_string(),
            "the WebP encoder is serial; set encoder_threads to 1"
        );
        // An external backend validates its own thread count (ADR-0024).
        let threaded = options(true).with_encoder_threads(
            std::num::NonZeroUsize::new(threads).map_or(Threads::Automatic, Threads::Count),
        );
        assert!(transcode_with(&input(), ImageFormat::WebP, &threaded, &RgbaBackend).is_ok());
    }
    let output = transcode(&input(), ImageFormat::WebP, &options(false)).unwrap();
    assert_eq!(output.report.encoder_threads, 1);
}

#[test]
fn exact_final_byte_limit_accepts_opaque_lossy_alpha_and_lossless() {
    for (input, lossless) in [(opaque_input(), false), (input(), false), (input(), true)] {
        let options = options(lossless);
        let full = transcode(&input, ImageFormat::WebP, &options).unwrap();
        let at_limit = transcode(
            &input,
            ImageFormat::WebP,
            &options.clone().with_max_output_bytes(full.bytes.len()),
        )
        .unwrap();
        assert_eq!(at_limit.bytes, full.bytes);
        let too_small = transcode(
            &input,
            ImageFormat::WebP,
            &options.clone().with_max_output_bytes(full.bytes.len() - 1),
        );
        assert_eq!(
            too_small.unwrap_err().kind(),
            ProcessingErrorKind::OutputLimitExceeded
        );
    }
}

#[test]
fn lossy_converted_planes_are_reserved_before_encoding() {
    // 8×8 ARGB = 256 bytes; YUV420 plus alpha = 64 + 16 + 16 + 64.
    let budget = MemoryBudget::new(256 + 160 - 1);
    let mut sink = WebpSink::new(Size::new(8, 8).unwrap(), &options(false), &budget).unwrap();
    let row = [10_u8, 20, 30, 64].repeat(8);
    for y in 0..8 {
        sink.write_row(y, &row).unwrap();
    }
    assert!(sink.finish().is_err());
    assert_eq!(budget.snapshot().live, 0);
    assert_eq!(budget.snapshot().peak, 256);
}

#[test]
fn multithreaded_webp_transcode_matches_single_threaded_for_lossy_and_lossless() {
    let (width, height) = (545_u32, 81_u32);
    let make_input = |alpha: bool, lossy: bool| -> Vec<u8> {
        let channels = if alpha { 4 } else { 3 };
        let mut pixels = Vec::with_capacity(width as usize * height as usize * channels);
        for y in 0..height {
            for x in 0..width {
                pixels.extend_from_slice(&[
                    u8::try_from((x * 7 + y * 13) % 256).unwrap(),
                    u8::try_from(((x * 19) ^ (y * 29)) % 256).unwrap(),
                    u8::try_from((x * 3 + y * 5) % 256).unwrap(),
                ]);
                if alpha {
                    pixels.push(u8::try_from((x * 11 + y * 17) % 256).unwrap());
                }
            }
        }
        let rows = PixelRows {
            layout: if alpha {
                PixelLayout::Rgba
            } else {
                PixelLayout::Rgb
            },
            data: &pixels,
            width,
            height,
            stride: width as usize * channels,
        };
        if lossy {
            encode_lossy(rows, 78.0).unwrap()
        } else {
            encode_lossless(rows).unwrap()
        }
    };
    let four = Threads::Count(std::num::NonZeroUsize::new(4).unwrap());
    let two = Threads::Count(std::num::NonZeroUsize::new(2).unwrap());
    let base = TransformOptions::new(OutputGeometry::Exact {
        width: 256,
        height: 48,
    })
    .with_output(OutputSettings::WebP(WebpOptions::default()))
    .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing);

    for (alpha, lossy) in [(false, true), (true, true), (false, false), (true, false)] {
        let encoded = make_input(alpha, lossy);
        let single = transcode(
            &encoded,
            ImageFormat::WebP,
            &base.clone().with_transform_threads(Threads::ONE),
        )
        .unwrap();
        let multi = transcode(
            &encoded,
            ImageFormat::WebP,
            &base.clone().with_transform_threads(four),
        )
        .unwrap();
        assert_eq!(
            multi.bytes, single.bytes,
            "alpha={alpha} lossy={lossy}: multithreaded WebP transcode changed bytes"
        );
        let batch = Engine::new(two).unwrap().transcode_batch(&[BatchItem::new(
            &encoded,
            ImageFormat::WebP,
            base.clone().with_transform_threads(four),
        )]);
        let batched = batch.into_iter().next().unwrap().unwrap();
        assert_eq!(batched.bytes, single.bytes);
        assert_eq!(batched.report.transform_threads, 1);
    }
}

#[test]
fn multi_chunk_webp_sink_output_is_exact_and_deterministic() {
    let (width, height) = (128_u32, 96_u32);
    let size = Size::new(width, height).unwrap();
    let row_bytes = width as usize * 4;
    let make_rgba = |alpha: bool| -> Vec<u8> {
        let mut pixels = Vec::with_capacity(width as usize * height as usize * 4);
        for y in 0..height {
            for x in 0..width {
                let hash = x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503);
                let noise = u8::try_from((hash >> 13) & 0x3f).unwrap();
                let r = u8::try_from((x * 2) % 256).unwrap() ^ noise;
                let g = u8::try_from((y * 2) % 256).unwrap();
                let b = u8::try_from((x + y) % 256).unwrap() ^ (noise << 1);
                let a = if !alpha {
                    255
                } else if x % 17 == 0 {
                    0
                } else {
                    u8::try_from((x * 5 + y * 3) % 255 + 1).unwrap()
                };
                pixels.extend_from_slice(&[r, g, b, a]);
            }
        }
        pixels
    };

    for (alpha, lossless) in [(false, false), (true, false), (true, true)] {
        let rgba = make_rgba(alpha);
        let opts = TransformOptions::new(OutputGeometry::Exact { width, height }).with_output(
            OutputSettings::WebP(if lossless {
                WebpOptions::Lossless(WebpLossless::new(4))
            } else {
                WebpOptions::default()
            }),
        );

        let run_sink = |max_output_bytes: usize| {
            let budget = MemoryBudget::new(16 << 20);
            let mut sink = WebpSink::new(
                size,
                &opts.clone().with_max_output_bytes(max_output_bytes),
                &budget,
            )
            .unwrap();
            for (y, row) in rgba.chunks_exact(row_bytes).enumerate() {
                sink.write_row(u32::try_from(y).unwrap(), row).unwrap();
            }
            let result = sink.finish();
            assert_eq!(budget.snapshot().live, 0);
            result
        };

        let first = run_sink(opts.max_output_bytes()).unwrap();
        let second = run_sink(opts.max_output_bytes()).unwrap();
        assert_eq!(first.bytes, second.bytes);
        assert_eq!(&first.bytes[..4], b"RIFF");
        assert_eq!(&first.bytes[8..12], b"WEBP");

        // Exact output byte limit still holds with geometric Vec::try_reserve.
        let at_limit = run_sink(first.bytes.len()).unwrap();
        assert_eq!(at_limit.bytes, first.bytes);
        assert!(matches!(
            run_sink(first.bytes.len() - 1),
            Err(crate::error::Error::OutputLimitExceeded { .. })
        ));

        let container = Container::new(&first.bytes).unwrap();
        assert_eq!(
            container.icc_profile().unwrap(),
            include_bytes!("../assets/icc/sRGB2014.icc")
        );
        let decoded = decode(&first.bytes, &DecodeConfig::new(ColorMode::Rgba)).unwrap();
        assert_eq!((decoded.width(), decoded.height()), (width, height));
        if lossless {
            let rows = decoded.rows().unwrap();
            let actual: Vec<u8> = (0..height)
                .flat_map(|y| rows.row(y).unwrap().to_vec())
                .collect();
            assert_eq!(actual, rgba);
        }
    }
}

#[test]
fn webp_chunk_writer_reserves_padded_riff_size_without_pad_reallocation() {
    use webp::api::encode::{Config, Picture, Preset};

    // 130x96 with this generator produces an odd VP8L chunk length and a
    // trailing 1-byte RIFF pad chunk; without reserving the checked padded RIFF
    // size from the first header, that final byte would double Vec capacity.
    let (width, height) = (130_u32, 96_u32);
    let make_argb = |alpha: bool| -> Vec<u32> {
        let mut argb = Vec::with_capacity(width as usize * height as usize);
        for y in 0..height {
            for x in 0..width {
                let hash = x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503);
                let noise = u8::try_from((hash >> 13) & 0x3f).unwrap();
                let r = u8::try_from((x * 2) % 256).unwrap() ^ noise;
                let g = u8::try_from((y * 2) % 256).unwrap();
                let b = u8::try_from((x + y) % 256).unwrap() ^ (noise << 1);
                let a = if !alpha {
                    255
                } else if x % 17 == 0 {
                    0
                } else {
                    u8::try_from((x * 5 + y * 3) % 255 + 1).unwrap()
                };
                argb.push(u32::from_be_bytes([a, r, g, b]));
            }
        }
        argb
    };

    let lossy_cfg = || {
        Config::preset(Preset::Default, 75.0)
            .unwrap()
            .method(4)
            .exact(true)
    };
    let mut saw_padded_vp8l = false;
    for (label, alpha, config) in [
        (
            "lossless-alpha",
            true,
            Config::new().lossless_level(4).unwrap().exact(true),
        ),
        ("lossy-opaque", false, lossy_cfg()),
        ("lossy-alpha", true, lossy_cfg()),
    ] {
        let mut argb = make_argb(alpha);
        let mut picture = Picture::from_argb(&mut argb, width, height, width as usize).unwrap();
        let mut observations = Vec::new();
        let encoded =
            encode_webp_chunks_for_test(&config, &mut picture, 16 << 20, |chunk, bytes| {
                observations.push((
                    chunk.len(),
                    bytes.len(),
                    bytes.capacity(),
                    bytes.as_ptr() as usize,
                ));
            })
            .unwrap();
        assert_eq!(&encoded[..4], b"RIFF");
        assert_eq!(&encoded[8..12], b"WEBP");
        let riff_payload = u32::from_le_bytes(encoded[4..8].try_into().unwrap()) as usize;
        let advertised_total = 8 + riff_payload;
        assert_eq!(encoded.len(), advertised_total);
        assert!(
            observations.len() >= 3,
            "{label}: expected multi-chunk encode, got {} chunks",
            observations.len()
        );
        // After the first (RIFF header) callback, capacity must already cover
        // the full advertised padded RIFF file size and remain completely
        // unchanged (no reallocation, same capacity and buffer pointer) across
        // every subsequent chunk through the final pad byte.
        let (_, _, initial_cap, initial_ptr) = observations[0];
        assert!(
            initial_cap >= advertised_total,
            "{label}: capacity after RIFF header ({initial_cap}) < advertised RIFF size ({advertised_total})"
        );
        for (idx, &(chunk_len, len, cap, ptr)) in observations.iter().enumerate().skip(1) {
            assert_eq!(
                cap, initial_cap,
                "{label}: capacity grew on chunk {idx} (chunk_len={chunk_len}, len={len})"
            );
            assert_eq!(
                ptr, initial_ptr,
                "{label}: buffer reallocated on chunk {idx} (chunk_len={chunk_len}, len={len})"
            );
        }
        if &encoded[12..16] == b"VP8L" {
            let vp8l_len = u32::from_le_bytes(encoded[16..20].try_into().unwrap()) as usize;
            if vp8l_len % 2 == 1 {
                assert_eq!(
                    observations.last().unwrap().0,
                    1,
                    "{label}: odd VP8L length must end with a 1-byte RIFF pad chunk"
                );
                saw_padded_vp8l = true;
            }
        }
    }
    assert!(
        saw_padded_vp8l,
        "130x96 lossless test case must exercise an odd VP8L length with a 1-byte RIFF pad"
    );
}

/// Reproducible multi-chunk WebP writer benchmark comparing the pre-#282
/// per-chunk `try_reserve_exact(chunk.len())` writer against the current
/// RIFF-header pre-reserving `encode_webp_chunks_for_test` (issue #282,
/// ADR-0030).
///
/// Run in release mode:
/// `cargo test --release -p ferrisma --lib --no-default-features --features webp-read,webp-write -- --ignored --nocapture --exact webp_output_tests::bench_webp_chunk_writer_growth_and_peak`
#[test]
#[ignore = "manual release benchmark for ADR-0030 (#282)"]
#[allow(clippy::too_many_lines)] // Self-contained before/after benchmark harness.
fn bench_webp_chunk_writer_growth_and_peak() {
    use std::time::Instant;
    use webp::api::encode::{Config, Encoder, Picture, Preset};

    fn make_argb(width: u32, height: u32, alpha: bool) -> Vec<u32> {
        let mut argb = Vec::with_capacity(width as usize * height as usize);
        for y in 0..height {
            for x in 0..width {
                let hash = x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503);
                let noise = u8::try_from((hash >> 13) & 0x3f).unwrap();
                let r = u8::try_from((x * 3) % 256).unwrap() ^ noise;
                let g = u8::try_from((y * 5) % 256).unwrap() ^ (noise >> 1);
                let b = u8::try_from((x + y * 2) % 256).unwrap() ^ (noise << 1);
                let a = if !alpha {
                    255
                } else if x % 17 == 0 {
                    0
                } else {
                    u8::try_from((x * 5 + y * 3) % 255 + 1).unwrap()
                };
                argb.push(u32::from_be_bytes([a, r, g, b]));
            }
        }
        argb
    }

    fn median_us(mut xs: Vec<f64>) -> f64 {
        xs.sort_by(f64::total_cmp);
        xs[xs.len() / 2]
    }

    let cases = [
        (
            "lossless-alpha-1024x768-padded",
            1024_u32,
            768_u32,
            true,
            Config::new().lossless_level(0).unwrap().exact(true),
        ),
        (
            "lossy-alpha-2048x1536",
            2048_u32,
            1536_u32,
            true,
            Config::preset(Preset::Default, 80.0)
                .unwrap()
                .method(0)
                .exact(true),
        ),
        (
            "lossy-opaque-2048x1536",
            2048_u32,
            1536_u32,
            false,
            Config::preset(Preset::Default, 80.0)
                .unwrap()
                .method(0)
                .exact(true),
        ),
    ];

    for (label, width, height, alpha, config) in cases {
        let mut argb = make_argb(width, height, alpha);
        let mut picture = Picture::from_argb(&mut argb, width, height, width as usize).unwrap();
        let mut chunks: Vec<Vec<u8>> = Vec::new();
        Encoder::new(&config)
            .encode_to(&mut picture, |chunk| {
                chunks.push(chunk.to_vec());
                true
            })
            .unwrap();
        let chunk_lens: Vec<usize> = chunks.iter().map(Vec::len).collect();

        // Measure live encode_webp_chunks_for_test (RIFF pre-reservation).
        let mut live_caps = Vec::new();
        let live_out = encode_webp_chunks_for_test(&config, &mut picture, 64 << 20, |_, bytes| {
            live_caps.push((bytes.capacity(), bytes.as_ptr() as usize));
        })
        .unwrap();
        let live_reallocs = live_caps
            .windows(2)
            .filter(|w| w[0].0 != w[1].0 || w[0].1 != w[1].1)
            .count();

        // 1. Collect allocation counters outside the timing loops: distinguish
        // capacity-growth requests from actual pointer moves.
        let mut base_bytes = Vec::new();
        let mut base_growths = 0_usize;
        let mut base_moves = 0_usize;
        let mut base_moved_bytes = 0_usize;
        let mut base_max_growth_copy_bound = 0_usize;
        let mut prev: Option<(usize, usize)> = None;
        for c in &chunks {
            let before_len = base_bytes.len();
            base_bytes.try_reserve_exact(c.len()).unwrap();
            base_bytes.extend_from_slice(c);
            let cur = (base_bytes.capacity(), base_bytes.as_ptr() as usize);
            if let Some((prev_cap, prev_ptr)) = prev {
                if cur.0 != prev_cap {
                    base_growths += 1;
                    base_max_growth_copy_bound += before_len;
                }
                if cur.1 != prev_ptr {
                    base_moves += 1;
                    base_moved_bytes += before_len;
                }
            }
            prev = Some(cur);
        }
        assert_eq!(base_bytes, live_out);
        let base_peak_cap = base_bytes.capacity();

        let mut cand_writer = WebpChunkWriter::new(64 << 20);
        let mut cand_growths = 0_usize;
        let mut cand_moves = 0_usize;
        let mut cand_moved_bytes = 0_usize;
        let mut prev: Option<(usize, usize)> = None;
        for c in &chunks {
            let before_len = cand_writer.bytes().len();
            assert!(cand_writer.push_chunk(c));
            let cur = (
                cand_writer.bytes().capacity(),
                cand_writer.bytes().as_ptr() as usize,
            );
            if let Some((prev_cap, prev_ptr)) = prev {
                if cur.0 != prev_cap {
                    cand_growths += 1;
                }
                if cur.1 != prev_ptr {
                    cand_moves += 1;
                    cand_moved_bytes += before_len;
                }
            }
            prev = Some(cur);
        }
        let cand_peak_cap = cand_writer.bytes().capacity();
        assert_eq!(cand_writer.into_bytes(), live_out);

        // 2. Time both writers with zero counter overhead (1 warmup + 7 interleaved reps),
        // replaying the captured chunks through the production `WebpChunkWriter`.
        let mut old_us = Vec::new();
        let mut new_us = Vec::new();
        for rep in 0..8 {
            let t0 = Instant::now();
            let mut bytes = Vec::new();
            for c in &chunks {
                bytes.try_reserve_exact(c.len()).unwrap();
                bytes.extend_from_slice(std::hint::black_box(c));
            }
            let elapsed = t0.elapsed().as_secs_f64() * 1e6;
            let bytes = std::hint::black_box(bytes);
            assert_eq!(bytes, live_out);
            if rep > 0 {
                old_us.push(elapsed);
            }

            let t0 = Instant::now();
            let mut writer = WebpChunkWriter::new(64 << 20);
            for c in &chunks {
                assert!(writer.push_chunk(std::hint::black_box(c)));
            }
            let elapsed = t0.elapsed().as_secs_f64() * 1e6;
            let bytes = std::hint::black_box(writer.into_bytes());
            assert_eq!(bytes, live_out);
            if rep > 0 {
                new_us.push(elapsed);
            }
        }
        println!(
            "{label}: len={}B chunks={chunk_lens:?} | baseline cap_growths={base_growths} ptr_moves={base_moves} moved_bytes={base_moved_bytes}B (upper_bound={base_max_growth_copy_bound}B) peak_cap={base_peak_cap}B writer_med={:.1}us | candidate cap_growths={cand_growths} ptr_moves={cand_moves} moved_bytes={cand_moved_bytes}B peak_cap={cand_peak_cap}B writer_med={:.1}us (live_reallocs={live_reallocs})",
            live_out.len(),
            median_us(old_us),
            median_us(new_us),
        );
    }
}
