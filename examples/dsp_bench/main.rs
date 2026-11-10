//! Reproducible micro-benchmarks of the DSP dispatch functions (ADR 0008).
//!
//! `cargo run --release --example dsp_bench [-- <module>...]` prints, for each
//! registered case, the median nanoseconds per call of the C translation and
//! of each SIMD variant the host selects, over seeded inputs. Without
//! arguments every module runs; with arguments only the named modules do.
//!
//! The harness has no dependency: inputs come from a seeded xorshift
//! generator, a measurement is a fixed number of calls, and the result is the
//! median of repeated measurements, which makes two runs on one host agree to
//! a few percent. Each SIMD package adds a file `examples/dsp_bench/<name>.rs`
//! with a `pub fn cases() -> Vec<Case>` and registers it in [`MODULES`]; the
//! tables in its pull request come from this program. The scalar build
//! (`--no-default-features`) runs the C translation only.

use std::time::Instant;

mod alpha_processing;
mod dec;
mod filters;
mod lossless;
mod rescaler;
mod upsampling;
mod x86;
mod yuv;

/// A benchmark case: a dispatch function of one module, measured as the
/// closure runs it on the inputs it prepared.
pub struct Case {
    /// The module the case belongs to (`dec`, `enc`, ...).
    pub module: &'static str,
    /// The dispatch function's name.
    pub function: &'static str,
    /// The variant under measurement: `C`, `SSE2`, `SSE41`, `AVX2`, `NEON`.
    pub variant: &'static str,
    /// Runs the function once on prepared inputs.
    pub run: Box<dyn FnMut()>,
}

/// Builds the cases of one module.
pub type Cases = fn() -> Vec<Case>;

#[cfg(target_arch = "aarch64")]
mod alpha_processing_neon;
#[cfg(target_arch = "aarch64")]
mod cost_neon;
#[cfg(target_arch = "aarch64")]
mod dec_neon;
#[cfg(target_arch = "aarch64")]
mod enc_neon;
#[cfg(target_arch = "aarch64")]
mod filters_neon;
#[cfg(target_arch = "aarch64")]
mod lossless_enc_neon;
#[cfg(target_arch = "aarch64")]
mod lossless_neon;
#[cfg(target_arch = "aarch64")]
mod sharpyuv_neon;
#[cfg(target_arch = "aarch64")]
mod upsampling_neon;
#[cfg(target_arch = "aarch64")]
mod yuv_neon;

/// The registered modules: a name to select on the command line and the
/// function that builds its cases. The SIMD packages add their entries here;
/// WP20's NEON modules (named after their `src/dsp` module) run on 64-bit ARM,
/// with the C rows only in the scalar build.
const MODULES: &[(&str, Cases)] = &[
    #[cfg(target_arch = "aarch64")]
    ("alpha_processing_neon", alpha_processing_neon::cases),
    #[cfg(target_arch = "aarch64")]
    ("cost_neon", cost_neon::cases),
    #[cfg(target_arch = "aarch64")]
    ("dec_neon", dec_neon::cases),
    #[cfg(target_arch = "aarch64")]
    ("enc_neon", enc_neon::cases),
    #[cfg(target_arch = "aarch64")]
    ("filters_neon", filters_neon::cases),
    #[cfg(target_arch = "aarch64")]
    ("lossless_enc_neon", lossless_enc_neon::cases),
    #[cfg(target_arch = "aarch64")]
    ("lossless_neon", lossless_neon::cases),
    #[cfg(target_arch = "aarch64")]
    ("sharpyuv_neon", sharpyuv_neon::cases),
    #[cfg(target_arch = "aarch64")]
    ("upsampling_neon", upsampling_neon::cases),
    #[cfg(target_arch = "aarch64")]
    ("yuv_neon", yuv_neon::cases),
    ("dec", dec::cases),
    ("upsampling", upsampling::cases),
    ("yuv", yuv::cases),
    ("alpha", alpha_processing::cases),
    ("filters", filters::cases),
    ("rescaler", rescaler::cases),
    ("lossless", lossless::cases),
];

/// Calls per measurement.
const CALLS: u32 = 2_000;
/// Measurements per case; the median is reported.
const MEASUREMENTS: usize = 9;

/// A seeded xorshift64* generator for reproducible inputs.
pub struct Rng(u64);

impl Rng {
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self(seed.max(1))
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    /// `len` seeded bytes.
    pub fn bytes(&mut self, len: usize) -> Vec<u8> {
        (0..len).map(|_| (self.next_u64() >> 24) as u8).collect()
    }
}

/// Median nanoseconds per call of `run`.
fn measure(run: &mut dyn FnMut()) -> f64 {
    let mut samples = Vec::with_capacity(MEASUREMENTS);
    for _ in 0..MEASUREMENTS {
        let start = Instant::now();
        for _ in 0..CALLS {
            run();
        }
        samples.push(start.elapsed().as_nanos() as f64 / f64::from(CALLS));
    }
    samples.sort_by(f64::total_cmp);
    samples[MEASUREMENTS / 2]
}

fn main() {
    let selected: Vec<String> = std::env::args().skip(1).collect();
    let mut printed = 0;
    println!(
        "{:<12} {:<34} {:<6} {:>12}",
        "module", "function", "variant", "ns/call"
    );
    for (name, cases) in MODULES {
        if !selected.is_empty() && !selected.iter().any(|s| s == name) {
            continue;
        }
        for mut case in cases() {
            let nanoseconds = measure(&mut case.run);
            println!(
                "{:<12} {:<34} {:<6} {:>12.1}",
                case.module, case.function, case.variant, nanoseconds
            );
            printed += 1;
        }
    }
    if printed == 0 {
        println!(
            "no cases registered{}",
            if selected.is_empty() {
                ""
            } else {
                " for the named modules"
            }
        );
    }
}
