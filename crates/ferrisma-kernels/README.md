# ferrisma-kernels

Runtime-dispatched SIMD kernels for Ferrisma behind a safe API, as decided in [ADR-0020](../../docs/adr/0020-kernel-crate-and-isa-dispatch.md). Not published; consumed by the `ferrisma` engine crate, which keeps `#![forbid(unsafe_code)]`.

## Levels

| Level | Status | Notes |
| --- | --- | --- |
| `scalar` | implemented | Reference body; the oracle of the differential tests |
| `neon` | implemented | One RGBA pixel per 128-bit vector; `horizontal_rgba` pipelines 4 independent output pixels with `vfmaq_laneq_f32` in exact sequential tap order; bit-identical to `scalar` for `horizontal_rgba`, `axpy`, `vertical_rgba`, and `vertical_rgba_add` |
| `avx2` | implemented | Two pixels (taps) per 256-bit vector with FMA; `horizontal_rgba` pipelines 2 independent output pixels while keeping each pixel's exact 2-slot accumulation order and `reduce_slots` tail (bit-identical to single-pixel `avx2`); `axpy`, `vertical_rgba`, and `vertical_rgba_add` are bit-identical to `scalar` |
| `avx512` | implemented, opt-in | Four pixels (taps) per 512-bit vector with FMA; `horizontal_rgba` pipelines 2 independent output pixels while keeping each pixel's exact 4-slot accumulation order and `reduce_slots` tail (bit-identical to single-pixel `avx512`); `axpy`, `vertical_rgba`, and `vertical_rgba_add` are bit-identical to `scalar`. Historically measured 1.35× over `avx2` on a 2026-11-06 hosted x86-64 runner for the previous single-pixel `horizontal_rgba` + per-tap `axpy` kernels; the updated pipelined `horizontal_rgba` + fused `vertical_rgba` kernels have not yet been measured on an AVX-512 host. Not selected by default in v1.0 (decision D6): request it with an explicit ceiling |
| `sve2` | detected, not implemented | The scalar body compiled for SVE2 was 3× slower than `neon` in the vertical pass on Neoverse; SVE2 machines run `neon` (`Fallback`) |

`Kernels::best()` selects the highest supported level up to `DEFAULT_CEILING`: `avx2` on x86-64 and `neon` on aarch64 ([ADR-0020](../../docs/adr/0020-kernel-crate-and-isa-dispatch.md) amendment of 2026-11-13). On an AVX-512 machine it reports `avx2` with reason `DefaultCeiling`, so default output does not depend on AVX-512 (issue #239). `Kernels::with_ceiling(IsaLevel::Avx512)` (engine: `kernel_level_ceiling = Some(KernelLevel::Avx512)`, CLI: `--kernel-level=avx512`) is the opt-in. `Kernels::available()` and the differential tests still cover `avx512` where the host has it.

`cargo run -p ferrisma-kernels --example detect` prints the machine level, the chosen level, the default ceiling and the levels the tests exercise on this host; `cargo run --release -p ferrisma-kernels --example bench` times each level on the 24 Mpx to 1600 resample shape. CI runs both on every runner.

## Kernels

- `horizontal_rgba`: one row of a separable resample over four `f32` channels with precomputed contiguous taps. Pipelines `L::HORIZONTAL_PIXELS` independent output pixels (`neon`: 4 pixels via `vfmaq_laneq_f32` in exact sequential tap order when `max_run >= 4` and an unrolled `0..=3`-tap single-pixel path when `max_run < 4`, bit-identical to `scalar` for all finite inputs; `avx2` and `avx512`: 2 pixels when `max_run >= SLOTS` preserving each pixel's exact `SLOTS`-strided accumulation order and `reduce_slots` tail, bit-identical to single-pixel evaluation at the same ISA level for all finite inputs).
- `axpy`: `acc += row * weight`, a single vertical resample step; bit-identical at every level.
- `vertical_rgba`: fused multi-row vertical resample (`out[i] = sum_t rows[t][i] * weights[t]`) fusing up to 4 vertical taps per register-blocked pass with hoisted weights and pre-sliced rows (`8 KiB` / `2048`-`f32` column strips when `rows.len() > 4`, bounding concurrent streams to 5 per pass — 4 input strips + 1 output strip, `40 KiB` working set across the 5 strips — writing `out` once for `<= 4` taps and making `ceil(taps / 4)` strip passes for deeper filters, reducing intermediate output read-modify-write traffic by up to 4× relative to per-tap `axpy` and improving locality on the measured ARM64 and x86-64 hosts); bit-identical to zeroing `out` + per-tap `axpy` at every level.
- `vertical_rgba_add`: accumulating variant (`acc[i] += sum_t rows[t][i] * weights[t]`) of `vertical_rgba`, allowing callers to chunk arbitrarily deep tap lists across fixed-size stack buffers with zero heap allocation; bit-identical to continuing per-tap `axpy` at every level.
- `srgb_to_linear`: sign-preserving extended sRGB EOTF on `f32` samples returning `f64`, evaluated in `f64` lanes via a degree-8 Remez minimax polynomial on `[-1, 1]` (`< 1.4e-10` relative error; bit-identical in full `f64` across SIMD levels including tails and in-range lanes of mixed-range vectors, and within `< 1.4e-10` relative error / `<= 1` ULP as `f32` of `scalar`, which uses `a * b + c` to avoid software `libm::fma` on baseline x86-64); outside `[-1, 1]`, every level delegates to the target's `f64::powf` (bit-identical across ISA levels on the same target).
- `linear_to_srgb_u16`: sRGB OETF and half-away-from-zero quantization to `0..=max` (`u16`), evaluated in `f64` lanes via a degree-8 Remez minimax polynomial with a per-lane exact `f64` OETF fallback whenever a curved-segment lane is within `1e-4` of a half-code tie `k + 0.5`; bit-identical across every level and exact against the `f64` OETF reference for every finite `f32` and `max`.

## `unsafe` budget

`unsafe` is allowed only in `src/lanes.rs` (intrinsics, pointer loads and stores behind length-checked slices) and `src/dispatch.rs` (calling a `#[target_feature]` entry after runtime detection). Every block carries a `SAFETY` comment; `clippy::undocumented_unsafe_blocks` and `clippy::multiple_unsafe_ops_per_block` are denied.

| Date | `unsafe {` blocks | Change |
| --- | ---: | --- |
| 2026-11-06 | 21 | Initial `Scalar`, `Neon` and `Avx2` levels for `horizontal_rgba` and `axpy` |
| 2026-11-06 | 40 | `Avx512` level (lanes and dispatch) |
| 2026-11-26 | 129 | `LanesF64` (`Neon`, `Avx2`, `Avx512`, `f64` EOTF output + per-lane half-code tie mask) and dispatch for `srgb_to_linear` and `linear_to_srgb_u16` (issue #281) |
| 2026-11-26 | 139 | `Neon::fma_lane` (`vfmaq_laneq_f32::<0..3>` for 4-pixel pipelined `horizontal_rgba`, +4 in `lanes.rs`) and dispatch for fused `vertical_rgba` + `vertical_rgba_add` (+6 in `dispatch.rs`) (issue #283) |

Growth without an amendment to ADR-0020 is a review finding (`grep -rn 'unsafe {' crates/ferrisma-kernels/src | wc -l`).

## Tests

`tests/differential.rs` runs every level the host supports against the scalar body: `axpy`, `vertical_rgba`, `vertical_rgba_add`, and `linear_to_srgb_u16` must be bit-identical (`0 ULP`) across every ISA level for all finite inputs (including unaligned widths, multi-batch vertical chunking, and every half-code threshold for 8/10/12/16 bits), `srgb_to_linear` within 1 ULP of `scalar` and exact `f64::powf`, and `horizontal_rgba` bit-identical (`0 ULP`) between `scalar` and `neon` and between single-pixel and multi-pixel pipelined evaluation at every ISA level (including `avx2` and `avx512`) for all finite inputs, crop-invariant to unrelated pixels outside a tap run, bounded by `4 · f32::EPSILON · taps` against both `scalar` and the `f64` oracle across the tested unit / near-unit input ranges (`[-0.2, 1.3]` samples, `[-0.3, 1.3]` weights, every integer tap count `1..=128`, the 8-tap skewed positive sum, and random positive sums on `avx2`/`avx512` — not a universal guarantee for arbitrary-magnitude opposite-sign cancellation, e.g. `±67,108,864` `cancel_at_tap_4` returns `0.0` on `scalar` and `0.125` on `avx2`), and preserving large opposite-sign and unequal-weight cancellation. Unit tests in `src/body.rs` additionally verify the `SLOTS = 2` (`Avx2`) and `SLOTS = 4` (`Avx512`) pipelined horizontal (including mixed 1-/2-tap Area-style short-tap tables) and chunked vertical bodies portably on every target. Levels the host cannot run are printed as untested. The engine's `fast_execute_tests` additionally hold every available level to ADR-0021's bound against the `f64` oracle at 8, 10, 12 and 16 bits.
