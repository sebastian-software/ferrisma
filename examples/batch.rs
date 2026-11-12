//! Batch throughput: transcode every JPEG in a directory with the `Engine`
//! (all cores, one image per worker) and sequentially in the same process,
//! and print images per second for both (ADR-0022, M2 work package 4).
//!
//! `cargo run --release --features cli --example batch -- DIR WIDTH [THREADS] [HEIGHT]`
//! (THREADS 0 = all cores; HEIGHT defaults to two thirds of WIDTH)

use std::error::Error;
use std::time::Instant;

use ferrisma::pipeline::{
    AvifEncoder, BatchItem, Engine, ImageFormat, InputProfilePolicy, TransformOptions, transcode,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let (Some(dir), Some(width)) = (args.get(1), args.get(2)) else {
        eprintln!("usage: batch DIR WIDTH [THREADS] [HEIGHT]");
        std::process::exit(2);
    };
    let width: u32 = width.parse()?;
    let threads: usize = args.get(3).map_or(Ok(0), |value| value.parse())?;
    let mut files: Vec<_> = std::fs::read_dir(dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("jpg"))
        })
        .collect();
    files.sort();
    let data: Vec<Vec<u8>> = files.iter().map(std::fs::read).collect::<Result<_, _>>()?;
    let base = TransformOptions {
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        // The recorded batch comparisons use rav1e's quality scale; keep that
        // encoder explicit now that libavif is the `cli` default.
        avif_encoder: AvifEncoder::Rav1e,
        quality: 80.0,
        speed: 6,
        ..TransformOptions::default()
    };
    let height = args
        .get(4)
        .map_or(Ok(width * 2 / 3), |value| value.parse::<u32>())?;
    let options = TransformOptions {
        output_width: width,
        output_height: height,
        ..base
    };
    let items: Vec<BatchItem<'_>> = data
        .iter()
        .map(|encoded| BatchItem {
            encoded,
            format: ImageFormat::Jpeg,
            options,
        })
        .collect();

    let engine = Engine::new(threads);
    let started = Instant::now();
    let results = engine.transcode_batch(&items);
    let batch = started.elapsed().as_secs_f64();
    let failed = results.iter().filter(|result| result.is_err()).count();

    let started = Instant::now();
    for item in &items {
        let single = TransformOptions {
            transform_threads: 1,
            encoder_threads: 1,
            ..item.options
        };
        transcode(item.encoded, item.format, single)?;
    }
    let sequential = started.elapsed().as_secs_f64();

    #[allow(clippy::cast_precision_loss)]
    let count = items.len() as f64;
    println!(
        "images: {}, failed: {failed}, output {width}x{height}, engine threads: {}",
        items.len(),
        engine.threads()
    );
    println!("engine batch: {batch:.2} s, {:.1} images/s", count / batch);
    println!(
        "sequential in-process: {sequential:.2} s, {:.1} images/s",
        count / sequential
    );
    Ok(())
}
