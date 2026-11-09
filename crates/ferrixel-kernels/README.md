# ferrixel-kernels

Runtime-dispatched SIMD kernels for Ferrixel behind a safe API, as decided in [ADR-0020](../../docs/adr/0020-kernel-crate-and-isa-dispatch.md). Not published; consumed by the `ferrixel` engine crate, which keeps `#![forbid(unsafe_code)]`.

## Levels

| Level | Status | Notes |
| --- | --- | --- |
| `scalar` | implemented | Reference body; the oracle of the differential tests |
| `neon` | implemented | One RGBA pixel per 128-bit vector; bit-identical to `scalar` |
| `avx2` | implemented | Two pixels (taps) per 256-bit vector with FMA; reassociated horizontal sums, within the tested bound |
| `avx512` | implemented | Four pixels (taps) per 512-bit vector with FMA; 1.35× over `avx2` on the hosted x86-64 runner |
| `sve2` | detected, not implemented | The scalar body compiled for SVE2 was 3× slower than `neon` in the vertical pass on Neoverse; SVE2 machines run `neon` (`Fallback`) |

`cargo run -p ferrixel-kernels --example detect` prints the machine level, the chosen level and the levels the tests exercise on this host; `cargo run --release -p ferrixel-kernels --example bench` times each level on the 24 Mpx to 1600 resample shape. CI runs both on every runner.

## Kernels

- `horizontal_rgba`: one row of a separable resample over four `f32` channels with precomputed contiguous taps.
- `axpy`: `acc += row * weight`, the vertical resample step; bit-identical at every level.

## `unsafe` budget

`unsafe` is allowed only in `src/lanes.rs` (intrinsics, pointer loads and stores behind length-checked slices) and `src/dispatch.rs` (calling a `#[target_feature]` entry after runtime detection). Every block carries a `SAFETY` comment; `clippy::undocumented_unsafe_blocks` and `clippy::multiple_unsafe_ops_per_block` are denied.

| Date | `unsafe {` blocks | Change |
| --- | ---: | --- |
| 2026-11-06 | 21 | Initial `Scalar`, `Neon` and `Avx2` levels for `horizontal_rgba` and `axpy` |
| 2026-11-06 | 40 | `Avx512` level (lanes and dispatch) |

Growth without an amendment to ADR-0020 is a review finding (`grep -rn 'unsafe {' crates/ferrixel-kernels/src | wc -l`).

## Tests

`tests/differential.rs` runs every level the host supports against the scalar body: `axpy` must be bit-identical, `horizontal_rgba` bit-identical for `neon` and within 4 ULP (or 4 · `f32::EPSILON` · taps for cancelling sums) for `avx2`. Levels the host cannot run are printed as untested. The engine's `fast_execute_tests` additionally hold every available level to ADR-0021's bound against the `f64` oracle after sRGB8 quantization.
