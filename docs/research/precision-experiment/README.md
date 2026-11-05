# ADR-0021 precision experiment

Evidence for [ADR-0021](../../adr/0021-working-precision-policy.md): the error of the proposed lookup tables, of `f32` as the working type and of a 16-bit fixed-point path, measured against the `f64` scalar reference of the M1 pipeline.

The program is standalone. It has its own `Cargo.toml`, no dependencies and no link to the `ferrixel` crate; the reference functions in `src/main.rs` are hand copies of `src/working_color.rs` and `src/quality.rs` at commit `17f3f46` (2026-11-05) and must be kept in sync by hand when the reference changes.

## Run

```sh
cd docs/research/precision-experiment
cargo run --release > results/$(date +%F)-$(hostname -s).md
```

Options: `--width N --height N` (chain image size, default 1024x768), `--p-cap N` (hard cap on coefficient precision, default 16), `--skip-exhaustive-f32` (skip the 1.07 billion value encoder check). A full run takes a few seconds on a 20-core machine.

## What it measures

| Section | Measurement | Inputs |
| --- | --- | --- |
| E1 | 256-entry decode tables (f32 and Q1.14) against the f64 transfer function; Q1.14 decode followed by exact encode round-trips every code | exhaustive 8-bit |
| E2 | Display P3 decode through folded product tables (Q.30 and f32), range of out-of-gamut linear values | exhaustive 2^24 triples |
| E3 | Encoder shapes: nearest-index tables with 12, 14 and 16 fractional bits versus the exact bucket-plus-threshold encoder | 2^24 points in [0, 1] and 2^20 points in [0, 0.01] |
| E4 | Exact f32 encoder against `encode_linear_srgb8(x as f64)` | every f32 in [0, 1] |
| E5 | f32 `powf` decode for F32 input | 2^24 points in [-0.2, 1.5] |
| E6 | Fixed-point premultiply by 8-bit alpha, divide versus shift form | exhaustive 256 x 256 |
| E7 | Filter gain G, negative-lobe sum N and max weight of Area and Lanczos3; adaptive coefficient precision; interval-propagated headroom of the Q1.14 format | all sizes 1..=96 squared, ratios up to 8192 |
| E8 | Full `decode -> premultiply -> horizontal -> vertical -> sRGB8` chains: f32 and fixed point against f64 | 17 synthetic images x 12 geometries x 2 filters |
| E9 | Worst case per geometry, pooled result per input, largest accumulator and row magnitudes observed versus the planner bound | summary of E8 |

The fixed-point chain models Q1.14 samples and intermediate rows, i16 weights with per-axis adaptive precision and an exact row sum, i32 accumulation (computed in i64 and asserted to fit i32), round-half-up shifts, and the exact 16385-entry encoder. The f32 chain uses the 256-entry f32 tables, f32 weights and sums, and the exact bucket-plus-threshold encoder.

## Results

- [2026-11-05-apple-m1-ultra.md](results/2026-11-05-apple-m1-ultra.md): the run recorded in ADR-0021 (default options). Apple M1 Ultra, 20 threads, macOS 27.0, rustc 1.100.0-nightly (5a2be9f5f 2026-09-06).
- [2026-11-05-apple-m1-ultra-p-cap-15.md](results/2026-11-05-apple-m1-ultra-p-cap-15.md): the same run with `--p-cap 15 --skip-exhaustive-f32`, to show the cost of a universal 15-bit coefficient cap instead of the per-axis bound rule.

Synthetic inputs are deterministic (xorshift seed in `cases()`), so a rerun on the same code reproduces the tables exactly; timings vary.
