# ferrisma-kernels

Runtime-dispatched SIMD kernels for Ferrisma behind a safe API, as decided in [ADR-0020](../../docs/adr/0020-kernel-crate-and-isa-dispatch.md). Not published; consumed by the `ferrisma` engine crate, which keeps `#![forbid(unsafe_code)]`.

## Levels

| Level | Status | Notes |
| --- | --- | --- |
| `scalar` | implemented | Reference body; the oracle of the differential tests |
| `neon` | implemented | One RGBA pixel per 128-bit vector; bit-identical to `scalar` |
| `avx2` | implemented | Two pixels (taps) per 256-bit vector with FMA; reassociated horizontal sums, within the tested bound |
| `avx512` | implemented, opt-in | Four pixels (taps) per 512-bit vector with FMA; 1.35× over `avx2` on the hosted x86-64 runner. Not selected by default in v1.0 (decision D6): request it with an explicit ceiling |
| `sve2` | detected, not implemented | The scalar body compiled for SVE2 was 3× slower than `neon` in the vertical pass on Neoverse; SVE2 machines run `neon` (`Fallback`) |

`Kernels::best()` selects the highest supported level up to `DEFAULT_CEILING`: `avx2` on x86-64 and `neon` on aarch64 ([ADR-0020](../../docs/adr/0020-kernel-crate-and-isa-dispatch.md) amendment of 2026-11-13). On an AVX-512 machine it reports `avx2` with reason `DefaultCeiling`, so default output does not depend on AVX-512 (issue #239). `Kernels::with_ceiling(IsaLevel::Avx512)` (engine: `kernel_level_ceiling = Some(KernelLevel::Avx512)`, CLI: `--kernel-level=avx512`) is the opt-in. `Kernels::available()` and the differential tests still cover `avx512` where the host has it.

`cargo run -p ferrisma-kernels --example detect` prints the machine level, the chosen level, the default ceiling and the levels the tests exercise on this host; `cargo run --release -p ferrisma-kernels --example bench` times each level on the 24 Mpx to 1600 resample shape. CI runs both on every runner.

## Kernels

- `horizontal_rgba`: one row of a separable resample over four `f32` channels with precomputed contiguous taps.
- `axpy`: `acc += row * weight`, the vertical resample step; bit-identical at every level.
- `srgb_to_linear`: sign-preserving extended sRGB EOTF on `f32` samples returning `f64`, evaluated in `f64` lanes via a degree-8 Remez minimax polynomial on `[-1, 1]` (`< 1.4e-10` relative error; bit-identical in full `f64` across SIMD levels including tails and in-range lanes of mixed-range vectors, and within `< 1.4e-10` relative error / `<= 1` ULP as `f32` of `scalar`, which uses `a * b + c` to avoid software `libm::fma` on baseline x86-64); outside `[-1, 1]`, every level delegates to the target's `f64::powf` (bit-identical across ISA levels on the same target).
- `linear_to_srgb_u16`: sRGB OETF and half-away-from-zero quantization to `0..=max` (`u16`), evaluated in `f64` lanes via a degree-8 Remez minimax polynomial with a per-lane exact `f64` OETF fallback whenever a curved-segment lane is within `1e-4` of a half-code tie `k + 0.5`; bit-identical across every level and exact against the `f64` OETF reference for every finite `f32` and `max`.

## `unsafe` budget

`unsafe` is allowed only in `src/lanes.rs` (intrinsics, pointer loads and stores behind length-checked slices) and `src/dispatch.rs` (calling a `#[target_feature]` entry after runtime detection). Every block carries a `SAFETY` comment; `clippy::undocumented_unsafe_blocks` and `clippy::multiple_unsafe_ops_per_block` are denied.

| Date | `unsafe {` blocks | Change |
| --- | ---: | --- |
| 2026-11-06 | 21 | Initial `Scalar`, `Neon` and `Avx2` levels for `horizontal_rgba` and `axpy` |
| 2026-11-06 | 40 | `Avx512` level (lanes and dispatch) |
| 2026-11-26 | 129 | `LanesF64` (`Neon`, `Avx2`, `Avx512`, `f64` EOTF output + per-lane half-code tie mask) and dispatch for `srgb_to_linear` and `linear_to_srgb_u16` (issue #281) |

Growth without an amendment to ADR-0020 is a review finding (`grep -rn 'unsafe {' crates/ferrisma-kernels/src | wc -l`).

## Tests

`tests/differential.rs` runs every level the host supports against the scalar body: `axpy` and `linear_to_srgb_u16` must be bit-identical across every level (with `linear_to_srgb_u16` verified exact at every half-code threshold for 8/10/12/16 bits), `srgb_to_linear` within 1 ULP of `scalar` and exact `f64::powf`, and `horizontal_rgba` bit-identical for `neon` and within 4 ULP (or 4 · `f32::EPSILON` · taps for cancelling sums) for `avx2`. Levels the host cannot run are printed as untested. The engine's `fast_execute_tests` additionally hold every available level to ADR-0021's bound against the `f64` oracle at 8, 10, 12 and 16 bits.
