//! Reproducible multithreaded JPEG XL decode benchmark (issue #100).
//!
//! Two subcommands keep encoder memory out of the decoder's `/usr/bin/time -l`
//! RSS measurement:
//!
//! Build the release binary first so `/usr/bin/time -l` measures only the
//! decoder benchmark process (not `cargo`):
//! `cargo build --release --features jpeg-xl-read,jpeg-xl-write,webp-read \
//!    --example jxl_decode_threads_bench`
//!
//! 1. Generate a deterministic multi-group `VarDCT` or lossless JXL fixture:
//!    `target/release/examples/jxl_decode_threads_bench generate vardct 4000 3000 /tmp/vardct.jxl`
//! 2. Measure decode wall time (best / median of REPS) and verify bit-identity
//!    against `threads=1` (run each thread count in its own `/usr/bin/time -l`
//!    process to record peak RSS; also works on single-group small inputs such
//!    as `tests/fixtures/sample.jxl` or a generated `128x128` file to verify
//!    zero runner startup overhead when `groups <= 1` or pixels < 65,536):
//!    `/usr/bin/time -l target/release/examples/jxl_decode_threads_bench \
//!       bench /tmp/vardct.jxl full 4 5`
//! 3. Run the self-contained small multi-group `320x64` `VarDCT` check (1T/8T/20T):
//!    `target/release/examples/jxl_decode_threads_bench small-multigroup [reps]`

use std::env;
use std::fs;
use std::hint::black_box;
use std::num::NonZeroUsize;
use std::time::Instant;

use ferrisma::pipeline::{
    Crop, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat,
    InputProfilePolicy, JpegXlLossless, JpegXlLossy, JpegXlOptions, OutputFormat, OutputGeometry,
    OutputSettings, RowEncoder, Threads, TransformOptions, transcode, transcode_with,
};
use ferrisma_jxl_decoder::{
    Region, decode_into_with_threads, decode_region_into_with_threads, inspect,
};
use webp::api::encode::{PixelLayout, PixelRows, encode_lossless};

struct DigestBackend;
struct DigestRowEncoder(u64);

impl EncoderBackend for DigestBackend {
    fn format(&self) -> OutputFormat {
        OutputFormat::WebP
    }
    fn name(&self) -> &'static str {
        "digest"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        Ok(Box::new(DigestRowEncoder(0xcbf2_9ce4_8422_2325)))
    }
}

impl RowEncoder for DigestRowEncoder {
    fn write_row(&mut self, _: u32, rgba: &[u8]) -> Result<(), EncodeError> {
        let mut hash = self.0;
        for &byte in rgba {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0100_0000_01b3);
        }
        self.0 = hash;
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(
            OutputFormat::WebP,
            self.0.to_le_bytes().to_vec(),
            0,
            false,
        ))
    }
}

fn synthetic_rgba(width: u32, height: u32) -> Vec<u8> {
    let w = usize::try_from(width).expect("width");
    let h = usize::try_from(height).expect("height");
    let mut data = Vec::with_capacity(w * h * 4);
    let w_div = u64::from(width.max(2));
    let h_div = u64::from(height.max(2));
    let sum_div = (u64::from(width) + u64::from(height)).max(2);
    for y in 0..height {
        for x in 0..width {
            let hash = x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503);
            let noise = u8::try_from((hash >> 13) & 0x1f).expect("5-bit noise");
            let red = u8::try_from((u64::from(x) * 255) / w_div).expect("u8") ^ noise;
            let green = u8::try_from((u64::from(y) * 255) / h_div).expect("u8") ^ (noise >> 1);
            let blue = u8::try_from(((u64::from(x) + u64::from(y)) * 255) / sum_div).expect("u8")
                ^ (noise << 1);
            data.extend_from_slice(&[red, green, blue, 255]);
        }
    }
    data
}

fn encode_fixture_bytes(kind: &str, width: u32, height: u32) -> Vec<u8> {
    let rgba = synthetic_rgba(width, height);
    let webp_bytes = encode_lossless(PixelRows {
        layout: PixelLayout::Rgba,
        data: &rgba,
        width,
        height,
        stride: usize::try_from(width).expect("width") * 4,
    })
    .expect("encode intermediate lossless WebP");
    let jxl_opts = match kind {
        "vardct" => JpegXlOptions::Lossy(JpegXlLossy::default().with_distance(1.0)),
        "lossless" => JpegXlOptions::Lossless(JpegXlLossless::default()),
        other => panic!("unknown kind {other}; expected vardct or lossless"),
    };
    let options = TransformOptions::new(OutputGeometry::Exact { width, height })
        .with_output(OutputSettings::JpegXl(jxl_opts))
        .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
        .with_max_managed_bytes(1024 * 1024 * 1024)
        .with_max_output_bytes(256 * 1024 * 1024);
    transcode(&webp_bytes, ImageFormat::WebP, &options)
        .expect("encode JXL fixture")
        .bytes
}

fn generate(kind: &str, width: u32, height: u32, out_path: &str) {
    let encoded = encode_fixture_bytes(kind, width, height);
    fs::write(out_path, &encoded).expect("write JXL fixture");
    println!(
        "generated kind={kind} dims={width}x{height} bytes={} path={out_path}",
        encoded.len()
    );
}

fn hash_f32_bits(samples: &[f32]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for &sample in samples {
        for byte in sample.to_bits().to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0100_0000_01b3);
        }
    }
    hash
}

fn bench_decoder(path: &str, mode: &str, threads: usize, reps: usize) {
    let bytes = fs::read(path).expect("read JXL input");
    let info = inspect(&bytes, 100_000_000).expect("inspect JXL");
    let region = match mode {
        "full" | "pipeline-full" => Region::full(&info),
        "crop" | "pipeline-crop" => Region {
            x: info.width / 4,
            y: info.height / 4,
            width: (info.width / 2).max(1),
            height: (info.height / 2).max(1),
        },
        other => panic!("unknown mode {other}; expected full, crop, pipeline-full, pipeline-crop"),
    };
    let mut times_ms = Vec::with_capacity(reps);
    let mut digest = 0_u64;
    let mut peak_managed = 0_usize;

    if mode.starts_with("pipeline-") {
        let mut options = TransformOptions::new(OutputGeometry::Exact {
            width: region.width,
            height: region.height,
        })
        .with_output(OutputSettings::default_for(OutputFormat::WebP))
        .with_profile_policy(InputProfilePolicy::AssumeSrgbIfMissing)
        .with_transform_threads(Threads::Count(
            NonZeroUsize::new(threads.max(1)).expect("nonzero threads"),
        ))
        .with_encoder_threads(Threads::ONE)
        .with_max_managed_bytes(1024 * 1024 * 1024);
        if mode == "pipeline-crop" {
            options = options.with_crop(Some(Crop::new(
                region.x,
                region.y,
                region.width,
                region.height,
            )));
        }
        for _ in 0..reps {
            let start = Instant::now();
            let output = transcode_with(&bytes, ImageFormat::JpegXl, &options, &DigestBackend)
                .expect("pipeline JXL decode");
            times_ms.push(start.elapsed().as_secs_f64() * 1e3);
            digest = u64::from_le_bytes(output.bytes.try_into().expect("8-byte digest"));
            peak_managed = output.report.peak_managed_bytes;
        }
    } else {
        let count = if mode == "full" {
            info.sample_count().expect("sample count")
        } else {
            info.region_sample_count(region).expect("region count")
        };
        let mut output = vec![0.0_f32; count];
        for _ in 0..reps {
            let start = Instant::now();
            if mode == "full" {
                decode_into_with_threads(&bytes, &info, &mut output, 100_000_000, threads)
                    .expect("decode_into_with_threads");
            } else {
                decode_region_into_with_threads(
                    &bytes,
                    &info,
                    region,
                    &mut output,
                    100_000_000,
                    threads,
                )
                .expect("decode_region_into_with_threads");
            }
            times_ms.push(start.elapsed().as_secs_f64() * 1e3);
        }
        digest = hash_f32_bits(&output);
        peak_managed = output.len() * 4;
        black_box(&output);
    }

    times_ms.sort_by(f64::total_cmp);
    println!(
        "mode={mode} dims={}x{} region={}x{} threads={threads} reps={reps} best_ms={:.2} median_ms={:.2} managed_bytes={peak_managed} digest={digest:016x}",
        info.width,
        info.height,
        region.width,
        region.height,
        times_ms[0],
        times_ms[reps / 2],
    );
}

fn bench_small_multigroup(reps: usize) {
    let bytes = encode_fixture_bytes("vardct", 320, 64);
    let info = inspect(&bytes, 100_000_000).expect("inspect 320x64 JXL");
    let count = info.sample_count().expect("sample count");
    let mut output = vec![0.0_f32; count];
    let mut expected_digest = None;
    for threads in [1_usize, 8_usize, 20_usize] {
        let mut times_ms = Vec::with_capacity(reps);
        for _ in 0..reps {
            let start = Instant::now();
            decode_into_with_threads(&bytes, &info, &mut output, 100_000_000, threads)
                .expect("decode_into_with_threads");
            times_ms.push(start.elapsed().as_secs_f64() * 1e3);
        }
        let digest = hash_f32_bits(&output);
        black_box(&output);
        if let Some(prev) = expected_digest {
            assert_eq!(digest, prev, "digest changed at threads={threads}");
        } else {
            expected_digest = Some(digest);
        }
        times_ms.sort_by(f64::total_cmp);
        println!(
            "case=small-multigroup mode=full dims={}x{} threads={threads} reps={reps} best_ms={:.2} median_ms={:.2} digest={digest:016x}",
            info.width,
            info.height,
            times_ms[0],
            times_ms[reps / 2],
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("generate") => {
            assert_eq!(
                args.len(),
                5,
                "generate <vardct|lossless> <w> <h> <out.jxl>"
            );
            generate(
                &args[1],
                args[2].parse().expect("width"),
                args[3].parse().expect("height"),
                &args[4],
            );
        }
        Some("bench") => {
            assert_eq!(
                args.len(),
                5,
                "bench <in.jxl> <full|crop|pipeline-full|pipeline-crop> <threads> <reps>"
            );
            bench_decoder(
                &args[1],
                &args[2],
                args[3].parse().expect("threads"),
                args[4].parse().expect("reps"),
            );
        }
        Some("small-multigroup") => {
            let reps: usize = args.get(1).map_or(100, |s| s.parse().expect("reps"));
            assert!(reps > 0, "reps must be >= 1");
            bench_small_multigroup(reps);
        }
        _ => panic!(
            "usage:\n  generate <vardct|lossless> <w> <h> <out.jxl>\n  bench <in.jxl> <full|crop|pipeline-full|pipeline-crop> <threads> <reps>\n  small-multigroup [reps]"
        ),
    }
}
