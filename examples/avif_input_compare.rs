//! Matched interim-versus-workspace AVIF input measurement harness.
//! Use the frozen `scripts/bench/avif-input-f0ff.rs` on the interim revision.
//! The digest-only output backend keeps encoder work out of the comparison.
//! `AVIF_BENCH_DUMP=<path>` writes one RGBA result for pixel comparison.

use ferrixel::pipeline::{
    Crop, EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat,
    InputProfilePolicy, OutputFormat, ResizeFilter, RowEncoder, TransformOptions, transcode_with,
};
use std::{env, fs, time::Instant};
struct NullBackend;
struct NullEncoder {
    digest: u64,
    pixels: Vec<u8>,
}
impl EncoderBackend for NullBackend {
    fn format(&self) -> OutputFormat {
        OutputFormat::Avif
    }
    fn name(&self) -> &'static str {
        "digest"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        Ok(Box::new(NullEncoder {
            digest: 0,
            pixels: Vec::new(),
        }))
    }
}
impl RowEncoder for NullEncoder {
    fn write_row(&mut self, _: u32, rgba: &[u8]) -> Result<(), EncodeError> {
        if env::var_os("AVIF_BENCH_DUMP").is_some() {
            self.pixels.extend_from_slice(rgba);
        }
        for b in rgba {
            self.digest = self
                .digest
                .wrapping_mul(1_099_511_628_211)
                .wrapping_add(u64::from(*b));
        }
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(
            OutputFormat::Avif,
            if env::var_os("AVIF_BENCH_DUMP").is_some() {
                self.pixels
            } else {
                self.digest.to_le_bytes().to_vec()
            },
            0,
            false,
        ))
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let bytes = fs::read(&args[1]).unwrap();
    let mut opts = TransformOptions {
        output_width: args[2].parse().unwrap(),
        output_height: args[3].parse().unwrap(),
        filter: ResizeFilter::Lanczos3,
        transform_threads: 1,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        ..TransformOptions::default()
    };
    if args.len() == 8 {
        opts.crop = Some(Crop {
            x: args[4].parse().unwrap(),
            y: args[5].parse().unwrap(),
            width: args[6].parse().unwrap(),
            height: args[7].parse().unwrap(),
        });
    }
    for i in 0..if env::var_os("AVIF_BENCH_DUMP").is_some() {
        1
    } else {
        5
    } {
        let start = Instant::now();
        let out = transcode_with(&bytes, ImageFormat::Avif, opts, &NullBackend).unwrap();
        let digest = if let Some(path) = env::var_os("AVIF_BENCH_DUMP") {
            fs::write(path, &out.bytes).unwrap();
            0
        } else {
            u64::from_le_bytes(out.bytes.try_into().unwrap())
        };
        println!(
            "run={i} elapsed_ms={:.3} total_ms={:.3} open_ms={:.3} decode_ms={:.3} transform_ms={:.3} peak_managed={} full_frame_buffers={} decoder_requests={} digest={digest:016x}",
            start.elapsed().as_secs_f64() * 1000.0,
            out.report.timings.total.as_secs_f64() * 1000.0,
            out.report.timings.open.as_secs_f64() * 1000.0,
            out.report.timings.decode.as_secs_f64() * 1000.0,
            out.report.timings.transform.as_secs_f64() * 1000.0,
            out.report.peak_managed_bytes,
            out.report.decoder_full_frame_buffers,
            out.report.decoder_requests
        );
    }
}
