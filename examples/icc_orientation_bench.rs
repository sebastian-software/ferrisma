//! Reproducible benchmark for ICC `ChannelTable`/`GrayTable` caching, fusion,
//! small-image cutoff (issue #219) and `OrientedSource` streaming/gather/thread
//! partitioning (issue #68).
//!
//! Run in release mode:
//! `cargo run --release -p ferrisma --features cli --example icc_orientation_bench`
//!
//! Optionally pass a directory containing pre-generated JPEGs (as produced by
//! the ADR-0019 / ADR-0023 measurement commands) as the first argument to
//! benchmark 24 MP / 48 MP camera-size JPEGs directly.

use std::error::Error;
use std::num::NonZeroUsize;
use std::path::Path;
use std::time::{Duration, Instant};

use ferrisma::pipeline::{
    BatchItem, Engine, ImageFormat, InputProfilePolicy, OutputGeometry, OutputSettings, Threads,
    TransformOptions, WebpLossy, WebpOptions, transcode,
};

fn webp_fast() -> OutputSettings {
    OutputSettings::WebP(WebpOptions::Lossy(WebpLossy::default().with_method(0)))
}

fn median(mut values: Vec<Duration>) -> Duration {
    values.sort_unstable();
    values[values.len() / 2]
}

#[allow(clippy::cast_precision_loss)]
fn bench_single(
    label: &str,
    bytes: &[u8],
    options: &TransformOptions,
    reps: usize,
) -> Result<(), Box<dyn Error>> {
    let mut opens = Vec::with_capacity(reps);
    let mut decodes = Vec::with_capacity(reps);
    let mut transforms = Vec::with_capacity(reps);
    let mut totals = Vec::with_capacity(reps);
    let mut peak = 0_usize;
    let mut orient = 0_usize;
    let mut copied = 0_u64;
    for _ in 0..reps {
        let out = transcode(bytes, ImageFormat::Jpeg, options)?;
        opens.push(out.report.timings.open);
        decodes.push(out.report.timings.decode);
        transforms.push(out.report.timings.transform);
        totals.push(out.report.timings.total);
        peak = out.report.peak_managed_bytes;
        orient = out.report.orientation_buffer_bytes;
        copied = out.report.decoder_work.wrapper_copied_bytes;
    }
    println!(
        "{label:24} open={:6.2}ms decode={:7.2}ms xform={:7.2}ms total={:7.2}ms peak={:7.2}MB orient={:6.2}MB copied={:6.2}MB",
        median(opens).as_secs_f64() * 1e3,
        median(decodes).as_secs_f64() * 1e3,
        median(transforms).as_secs_f64() * 1e3,
        median(totals).as_secs_f64() * 1e3,
        peak as f64 / 1e6,
        orient as f64 / 1e6,
        copied as f64 / 1e6,
    );
    Ok(())
}

fn bench_icc_batch(
    label: &str,
    jpeg: &[u8],
    count: usize,
    width: u32,
    height: u32,
    threads: usize,
) -> Result<(), Box<dyn Error>> {
    let options = TransformOptions::new(OutputGeometry::Exact { width, height })
        .with_output(webp_fast())
        .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
        .with_transform_threads(Threads::ONE)
        .with_encoder_threads(Threads::ONE);
    let items: Vec<BatchItem<'_>> = (0..count)
        .map(|_| BatchItem::new(jpeg, ImageFormat::Jpeg, options.clone()))
        .collect();
    let engine = Engine::new(Threads::Count(
        NonZeroUsize::new(threads.max(1)).expect("nonzero"),
    ))?;

    // Batch-scoped ICC cache: first item validates and caches the profile
    // transform/table; remaining items in the batch reuse it, and the cache is
    // dropped before `transcode_batch` returns.
    let t0 = Instant::now();
    let batch = engine.transcode_batch(&items);
    let batch_ms = t0.elapsed().as_secs_f64() * 1e3;
    let batch_open_sum_ms: f64 = batch
        .iter()
        .map(|r| r.as_ref().expect("ok").report.timings.open.as_secs_f64() * 1e3)
        .sum();

    // Uncached sequential `transcode` baseline for comparison.
    let t1 = Instant::now();
    let mut uncached_open_sum_ms = 0.0_f64;
    for (idx, item) in items.iter().enumerate() {
        let single = transcode(item.encoded(), item.format(), item.options())?;
        uncached_open_sum_ms += single.report.timings.open.as_secs_f64() * 1e3;
        // Byte-identity check: batch output must match single transcode.
        assert_eq!(
            single.bytes,
            batch[idx].as_ref().expect("ok").bytes,
            "batch item {idx} must be byte-identical to single transcode"
        );
    }
    let uncached_ms = t1.elapsed().as_secs_f64() * 1e3;

    println!(
        "{label:24} n={count} t={threads} batch={batch_ms:6.2}ms (open_sum={batch_open_sum_ms:5.2}ms) uncached_1t={uncached_ms:6.2}ms (open_sum={uncached_open_sum_ms:5.2}ms)"
    );
    Ok(())
}

fn load_repo_fixture(rel: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel);
    if !path.exists() {
        return Err(format!(
            "fixture {} not found (packaged source excludes tests/fixtures/); pass a directory of JPEGs as argv[1]",
            path.display()
        )
        .into());
    }
    Ok(std::fs::read(path)?)
}

fn run_directory(dir: &Path) -> Result<(), Box<dyn Error>> {
    if !dir.is_dir() {
        return Err(format!(
            "benchmark input directory does not exist or is not a directory: {}",
            dir.display()
        )
        .into());
    }
    let mut ran = 0_usize;
    for (label, file, w, h, icc, threads) in [
        ("24mp-o1-t1", "24mp-o1.jpg", 1600, 1067, false, 1),
        ("24mp-o2-t1", "24mp-o2.jpg", 1600, 1067, false, 1),
        ("24mp-o3-t1", "24mp-o3.jpg", 1600, 1067, false, 1),
        ("24mp-o4-t1", "24mp-o4.jpg", 1600, 1067, false, 1),
        ("24mp-o6-t1", "24mp-o6.jpg", 1067, 1600, false, 1),
        ("24mp-o1-t8", "24mp-o1.jpg", 1600, 1067, false, 8),
        ("24mp-o2-t8", "24mp-o2.jpg", 1600, 1067, false, 8),
        ("24mp-o3-t8", "24mp-o3.jpg", 1600, 1067, false, 8),
        ("24mp-o4-t8", "24mp-o4.jpg", 1600, 1067, false, 8),
        ("24mp-o6-t8", "24mp-o6.jpg", 1067, 1600, false, 8),
        ("48mp-o1-t1", "48mp-o1.jpg", 1600, 1200, false, 1),
        ("48mp-o2-t1", "48mp-o2.jpg", 1600, 1200, false, 1),
        ("48mp-o3-t1", "48mp-o3.jpg", 1600, 1200, false, 1),
        ("48mp-o4-t1", "48mp-o4.jpg", 1600, 1200, false, 1),
        ("48mp-o6-t1", "48mp-o6.jpg", 1200, 1600, false, 1),
        ("48mp-o1-t8", "48mp-o1.jpg", 1600, 1200, false, 8),
        ("48mp-o2-t8", "48mp-o2.jpg", 1600, 1200, false, 8),
        ("48mp-o3-t8", "48mp-o3.jpg", 1600, 1200, false, 8),
        ("48mp-o4-t8", "48mp-o4.jpg", 1600, 1200, false, 8),
        ("48mp-o6-t8", "48mp-o6.jpg", 1200, 1600, false, 8),
        ("24mp-srgb-icc-t1", "24mp-srgb-icc.jpg", 1600, 1067, true, 1),
        ("24mp-srgb-icc-t8", "24mp-srgb-icc.jpg", 1600, 1067, true, 8),
        ("24mp-p3-icc-t1", "24mp-p3-icc.jpg", 1600, 1067, true, 1),
        ("24mp-p3-icc-t8", "24mp-p3-icc.jpg", 1600, 1067, true, 8),
        ("24mp-gray-icc-t1", "24mp-gray-icc.jpg", 1600, 1067, true, 1),
        ("24mp-gray-icc-t8", "24mp-gray-icc.jpg", 1600, 1067, true, 8),
        ("64x64-p3-icc-t1", "64x64-p3-icc.jpg", 32, 32, true, 1),
        ("64x64-gray-icc-t1", "64x64-gray-icc.jpg", 32, 32, true, 1),
    ] {
        let path = dir.join(file);
        if !path.exists() {
            continue;
        }
        let bytes = std::fs::read(&path)?;
        let opts = TransformOptions::new(OutputGeometry::Exact {
            width: w,
            height: h,
        })
        .with_output(webp_fast())
        .with_profile_policy(if icc {
            InputProfilePolicy::UseRenderedIcc
        } else {
            InputProfilePolicy::AssumeSrgbIfMissing
        })
        .with_transform_threads(Threads::Count(NonZeroUsize::new(threads).expect("nz")))
        .with_encoder_threads(Threads::ONE);
        bench_single(label, &bytes, &opts, 3)?;
        ran += 1;
    }
    for (label, file, w, h) in [
        ("batch10-1600-p3-t1", "1600x1200-p3-icc.jpg", 400, 300),
        ("batch10-1600-p3-t8", "1600x1200-p3-icc.jpg", 400, 300),
        ("batch10-64-p3-t1", "64x64-p3-icc.jpg", 32, 32),
    ] {
        let path = dir.join(file);
        if path.exists() {
            let bytes = std::fs::read(&path)?;
            let t = if label.ends_with("t8") { 8 } else { 1 };
            bench_icc_batch(label, &bytes, 10, w, h, t)?;
            ran += 1;
        }
    }
    if ran == 0 {
        return Err(format!(
            "no recognized benchmark fixtures found in {}",
            dir.display()
        )
        .into());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if let Some(dir) = args.get(1) {
        return run_directory(Path::new(dir));
    }

    // Default smoke benchmark using repository fixtures loaded at runtime so
    // the example still compiles from packaged source (where `tests/fixtures/`
    // is excluded).
    let paris_base = load_repo_fixture("tests/fixtures/decoder-work/paris-420.jpg")?;
    let p3_icc = load_repo_fixture("tests/fixtures/icc/DisplayP3-v4.icc")?;
    let gray_icc = load_repo_fixture("tests/fixtures/parity/gray-gray-icc.jpg")?;
    let mut paris_p3 = Vec::with_capacity(paris_base.len() + p3_icc.len() + 18);
    paris_p3.extend_from_slice(&paris_base[..2]);
    let payload_len = u16::try_from(14 + p3_icc.len() + 2)?;
    paris_p3.extend_from_slice(&[0xff, 0xe2]);
    paris_p3.extend_from_slice(&payload_len.to_be_bytes());
    paris_p3.extend_from_slice(b"ICC_PROFILE\0\x01\x01");
    paris_p3.extend_from_slice(&p3_icc);
    paris_p3.extend_from_slice(&paris_base[2..]);

    let opts = TransformOptions::new(OutputGeometry::Exact {
        width: 200,
        height: 150,
    })
    .with_output(webp_fast())
    .with_profile_policy(InputProfilePolicy::UseRenderedIcc)
    .with_transform_threads(Threads::ONE)
    .with_encoder_threads(Threads::ONE);
    bench_single("paris-p3-icc-t1", &paris_p3, &opts, 5)?;
    bench_icc_batch("batch10-paris-p3-t1", &paris_p3, 10, 200, 150, 1)?;
    bench_icc_batch("batch10-paris-p3-t4", &paris_p3, 10, 200, 150, 4)?;
    bench_icc_batch("batch10-gray-icc-t1", &gray_icc, 10, 16, 16, 1)?;
    Ok(())
}
