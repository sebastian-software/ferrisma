# M2 — Performance architecture for 2026 CPUs and GPUs

- **Status:** Proposed; work package 0 (baseline harness) in progress, work package 1 (scalar `f32` executor) landed, work package 2 (encoder) landed except the libavif-rs comparison, work package 3 (SIMD kernels) in progress
- **Created:** 2026-11-05
- **Audience:** Ferrixel maintainers and contributors
- **Horizon:** The milestone after [M1](M1-complete-image-pipeline.md); M1-RC gates stay open and are not replaced by this plan
- **Scope:** Make the M1 pipeline (`decode → crop → linear-light resize → quantize → encode`) fast on current hardware without giving up its correctness, memory and observability contracts
- **Ambition:** Measurably better than libvips and Sharp at *matched output quality* on modern Arm and x86-64 machines, with a GPU path that is used only where it is measured to win
- **Architecture direction:** [ADR-0006](../adr/0006-cpu-first-gpu-capable-execution.md) (Proposed), [ADR-0007](../adr/0007-correctness-and-performance-evidence.md) (Accepted); new ADRs 0020–0024 listed below

## Milestone statement

> Ferrixel converts real JPEG, JPEG XL and AVIF photographs to the same AVIF output as the M1 reference path, but through fused, SIMD-dispatched, tile-parallel kernels. On Apple Silicon, Arm Neoverse and 2026 x86-64 it beats libvips and Sharp on single-image latency *and* multi-image throughput at matched perceptual quality, with lower peak memory, and the execution plan explains which ISA, precision, tile shape and device were chosen. A GPU backend exists for at least one measured workload where it wins end to end, transfers included.

Nothing in this milestone may change the *meaning* of an output: the M1 scalar path remains the oracle, and every fast path is tested against it.

## Why "like you would build it today" is a real advantage

libvips (and Sharp on top of it) is excellent, but its shape is from 2005: one operation per stage, each stage producing a region buffer for the next, a global thread pool that splits one image into horizontal strips, SIMD only in the operations that received hand-written Orc or Highway paths, linear-light processing as a float detour, and codecs as C libraries behind libheif/libjpeg-turbo. Its biggest practical win is *shrink-on-load*: letting libjpeg decode at 1/2, 1/4 or 1/8 scale. These statements describe libvips 8.1x as we understand it and must be verified against the actual versions used in the comparison corpus (work package 0); they are hypotheses about where to win, not benchmark results.

Ferrixel already has the two ingredients libvips cannot retrofit: a planner that knows the whole chain before any pixel moves, and a region contract that never needs a full-frame intermediate. The performance architecture builds on exactly that:

1. **Whole-chain fusion per tile.** One pass per tile does transfer decode, horizontal resample, vertical resample and output quantization while the data is in L1/L2, instead of four stage buffers.
2. **Precision chosen by the planner, not by the API.** 8-bit SDR input can run in 16-bit fixed point (twice the SIMD lanes of f32) with a proven error bound; f32 where the bound does not hold; f64 only as the test oracle.
3. **Kernels written once, dispatched per ISA at run time.** NEON on every Arm core, SVE2 where present, AVX2 as the x86-64 floor, AVX-512/AVX10 where present. The plan reports the chosen level.
4. **Tile-level work stealing across *and* within images.** No global static pool; an `Engine` owns its workers, buffers and budget, so many small images saturate 20 cores as well as one 48-megapixel image does.
5. **Decode-side scaling and native layouts for every codec that permits it**, reported honestly as codec capability rather than assumed.
6. **GPU as a planned, costed backend** with unified memory on Apple Silicon and integrated Intel/AMD graphics as the first targets, because there the transfer term is near zero.

## Target hardware (2026)

The matrix below sets what must be built and measured. Feature availability is read at run time; the build never assumes a micro-architecture. Claims about unreleased parts are expectations and must be re-checked when the CI runners exist.

| Class | Representative parts | SIMD floor | Dispatched extras | Notes |
| --- | --- | --- | --- | --- |
| Apple Silicon | M1 Ultra (dev machine), M4, M5 | NEON 128-bit, four SIMD pipes per performance core | SME/SME2 on M4+ (matrix unit; not a first target) | Unified memory, Metal GPU, hardware JPEG decode, AV1 hardware *decode* on M3+ |
| Arm Neoverse servers | AWS Graviton4 (Neoverse V2), Graviton successors, NVIDIA Grace | NEON | SVE2 (128-bit vectors on V2) | Many cores, modest per-core cache; throughput mode matters most |
| x86-64 servers | AMD Zen 5 EPYC, Intel Xeon 6 (Granite Rapids), Diamond Rapids | AVX2 (x86-64-v3) | AVX-512 (native 512-bit on Zen 5 and Xeon), AVX10.2 and APX on Diamond Rapids | AMX exists but has no obvious resampling use; VNNI-style u8 dot products are an experiment |
| x86-64 clients | Intel Core Ultra (Arrow Lake, Lunar Lake, Panther Lake), AMD Ryzen 9000 | AVX2 | AVX-512 on AMD; Intel client AVX10 expected with Nova Lake, not assumed | Intel Xe/Arc iGPUs have AV1 hardware encode and shared memory |
| GPUs | Apple GPU, Intel Xe/Arc, AMD RDNA, NVIDIA | wgpu compute (Metal, Vulkan, DX12) | — | Discrete parts pay PCIe transfer; integrated parts do not |

Policy: **AVX2 and NEON are the floors**; everything above is a runtime-dispatched bonus that must prove itself in the benchmark matrix before it is kept.

## Acceptance contract

M2 is done only when all of these hold:

1. **Same pictures.** For the whole corpus, every fast path produces output that matches the M1 scalar `f64` reference within the documented bound of [ADR-0021](../adr/0021-working-precision-policy.md) (at most one code per channel after sRGB8 quantization, with stated shares of exact matches); integer paths are additionally bit-identical to their scalar implementation across ISAs, tile shapes and thread counts ([ADR-0020](../adr/0020-kernel-crate-and-isa-dispatch.md)). Bit-exactness against the oracle itself is not required, because any path with different rounding points differs from it at ties. The AVIF output uses the same encoder configuration. No fast path may change color policy, alpha association, orientation or crop semantics.
2. **Matched-quality comparison.** Sharp and libvips comparisons use equal geometry, the same linear-light intent, and AVIF settings tuned to equal SSIMULACRA2 (and Butteraugli as a second opinion). Elapsed time at different quality is not a result.
3. **Three hardware classes, two workload shapes.** Apple Silicon, Arm Neoverse and x86-64 (AVX2 floor, AVX-512 where available), each with single-image latency (p50/p95) on 1, 12, 24 and 48 megapixel inputs and multi-image throughput (images per second, all cores) on a many-small-images set.
4. **Lower peak memory** than the comparators on the same workloads, with engine-managed, known codec and opaque codec memory still reported separately.
5. **Explainable plans.** The report names the ISA level, working precision, tile shape, decode-side scale factor, device and every materialization.
6. **GPU evidence either way.** At least one workload where the GPU path wins end to end including transfers, or a documented measurement showing it does not on the tested devices. GPU is never the default without that evidence.
7. **Safety boundary intact.** The core crate keeps `#![forbid(unsafe_code)]`. Intrinsics live in a separate, audited kernel crate with a safe API, differential tests and a scalar fallback.

## Work packages

Ordered by evidence first, then algorithmic wins, then hardware-specific wins. A package may span several pull requests. Each package ends with numbers in `docs/benchmarks/`, not with a claim.

| # | Work package | Exit evidence | Depends on |
| --- | --- | --- | --- |
| 0 | **Baseline and profiling harness** | Per-stage timers (decode, transform, encode) in the processing report; `hyperfine` and `samply` scripts; a corpus of real photographs at 1/12/24/48 Mpx plus a many-small-images set; quality-matched Sharp and libvips comparators with SSIMULACRA2; numbers for the current M1 path on the dev machine and on CI. Nothing else starts before this exists, per ADR-0007. | — |
| 1 | **Algorithmic cleanup of the M1 executor** | Working precision f32 instead of f64; transfer decode once per source sample through a 256-entry exact LUT, not once per filter tap; linear-to-sRGB8 through a verified LUT; O(1) row-cache addressing instead of a linear scan per column and tap; three-channel paths for opaque RGB; precomputed tap tables. Expected to be a multiple, measured against package 0. | 0 |
| 2 | **Encoder throughput** | The AVIF sink currently pins `rav1e` to one thread and hands it a packed RGBA frame. Measure thread count and speed presets, YUV 4:2:0 input instead of RGBA, and encoder alternatives through libavif-rs (the project's own port, see ADR-0018). Record quality-matched bytes and time for each option. | 0 |
| 3 | **Kernel crate and ISA dispatch** (ADR-0020, ADR-0021) | `ferrixel-kernels`: horizontal and vertical resample, transfer LUT apply, pack/unpack, premultiply/unpremultiply, as width-agnostic kernels with scalar reference, NEON, SVE2, AVX2 and AVX-512 variants chosen at run time; differential tests against the scalar reference on random and adversarial inputs; 16-bit fixed-point path with a measured error bound against the f64 oracle. | 1 |
| 4 | **Tiled, fused, work-stealing executor** (ADR-0022) | The planner emits output bands with filter halos; each band runs the fused chain on one worker; horizontal-pass rows are shared between adjacent bands without recomputation; a decoder stage feeds a bounded row ring ahead of the workers; an `Engine` owns the pool, buffer pool and budget; cancellation and backpressure across workers; batch API for many images. Measured scaling from 1 to all cores on all three hardware classes. | 3 |
| 5 | **Decode-side scaling and native layouts** (ADR-0023) | JPEG DCT-domain 1/2, 1/4, 1/8 decode where the decoder supports it (verify `zune-jpeg`; otherwise contribute it or switch adapter); JPEG XL reduced-resolution decode where `jxl-oxide` exposes it; AVIF stays full-resolution but gains a native 10-bit YUV 4:2:0 path into the encoder without an RGB round trip, under an explicit *gamma-domain native* quality policy that is distinct from the linear-light reference (ADR-0011 stays in force until that policy is written). The report states the achieved decode scale factor. | 4 |
| 6 | **Cross-platform benchmark CI** | macOS arm64, Linux arm64 (Graviton) and Linux x86-64 (AVX2 and AVX-512 runners) run the package-0 harness on every main merge and keep a history; regressions beyond a stated threshold fail the job. | 0, 4 |
| 7 | **GPU experiment** (ADR-0024) | One `wgpu` compute pair (transfer decode plus separable resample) using the same tile contract; measured upload, kernel and readback time on Apple unified memory, Intel/AMD integrated and one discrete GPU; a cost model in the planner that picks the device per workload; CPU/GPU outputs compared within the stated bound. Keep or drop by evidence. | 4, 6 |
| 8 | **Hardware codec adapters** (optional) | VideoToolbox (Apple) and VA-API (Linux) decode/encode as FFI-isolated, feature-gated adapters with the same honest capability reporting; only if package 2 shows the software encoder is the ceiling. | 2, 7 |

### Checkpoints

- **M2-A (Fast scalar):** packages 0–2. Same architecture, honest numbers, obvious waste removed, encoder no longer artificially single-threaded.
- **M2-B (Hardware-aware CPU):** packages 3–6. SIMD kernels, tile parallelism, decode-side scaling, benchmark CI on three hardware classes.
- **M2-C (Measured leadership):** packages 7–8. GPU evidence, optional hardware codecs, the comparison tables that back any public claim.

## Decisions to record as ADRs

| ADR | Topic | Core question |
| --- | --- | --- |
| [0020](../adr/0020-kernel-crate-and-isa-dispatch.md) (Proposed) | Kernel crate, ISA dispatch and the `unsafe` boundary | Decided: a separate `ferrixel-kernels` crate with `std::arch` intrinsics behind a safe API, hand-written runtime dispatch, the chosen level in plan and report, differential tests per level |
| [0021](../adr/0021-working-precision-policy.md) (Proposed) | Working precision policy | Decided: `f32` by default, Q1.14 fixed point for opaque 8-bit SDR input under a per-plan interval bound, exact LUTs, a testable bound against the `f64` oracle, backed by `docs/research/precision-experiment` |
| 0022 | Tiled fused executor and engine-owned parallelism | Band shapes and halos, decoder-ahead pipelining for sequential codecs, shared horizontal rows, budget across workers, batch scheduling, no global pool |
| 0023 | Decode-side scaling and native YUV policy | Which codecs really skip work, how the scale factor enters the plan, the gamma-domain native path as a separate declared quality policy; facts in [the codec capability audit](../research/2026-11-05-decode-scaling-and-encoder-capabilities.md) |
| 0024 | GPU execution policy | Cost model inputs, unified-memory fast path, when the planner may choose the device, dependency weight of `wgpu`, output equivalence bound |

Each ADR starts as Proposed, gets an experiment, and moves to Experimental only with numbers from package 0's harness.

## Architectural guardrails

- **The reference path stays.** The M1 scalar executor remains compiled and tested as the oracle; no fast path is merged without a differential test against it.
- **No unmeasured claims.** Every package lands with before/after numbers from the same harness on the same machine, and the benchmark docs record CPU, cores, OS, rustc, codec versions and command lines.
- **Honest capability reporting.** A codec that returns a smaller image has not necessarily done less work; the report says what was requested, what was skipped and what was materialized.
- **Safe core, audited edges.** `unsafe` only in the kernel crate and FFI adapters, each with a documented audit and fuzz target.
- **Dispatch by runtime detection, never by build flags alone.** A binary built on one machine must be correct and fast on another.
- **GPU is opt-in by evidence**, per workload class, never a global default.

## Risks and decision gates

| Risk | Gate / fallback |
| --- | --- |
| Rust decoders lack DCT-domain scaling (`zune-jpeg`) or reduced-resolution decode (`jxl-oxide`) | Verify in package 5; contribute upstream, implement a reduced IDCT in the adapter, or document the full-decode ceiling and the throughput cost |
| `rav1e` remains much slower than libaom/SVT-AV1 even multi-threaded | Package 2 measures; libavif-rs encoder backends are the planned route; hardware encode (package 8) is the escape hatch |
| Stable Rust has no portable SIMD without `unsafe` or a wrapper crate | ADR-0020 picks between intrinsics behind a safe API and a wrapper crate; MSRV 1.98 and the `forbid(unsafe_code)` core are constraints, not casualties |
| 16-bit fixed point violates the error bound on extreme downscales or alpha edges | Planner falls back to f32 for that tile; the bound is tested on adversarial inputs |
| Tile parallelism increases peak memory through per-worker buffers | Budget is shared across workers; the scheduler throttles in-flight bands; peak RSS stays a tracked benchmark metric |
| `wgpu` adds a large dependency tree and platform surface | Feature-gated; package 7 can end with "not worth it" and that is a valid outcome |
| Unfair comparisons (different filters, transfer intent, encoder presets) | Package 0 fixes geometry, intent and quality matching before any number is published |
| Benchmark CI on Arm servers and AVX-512 runners is unavailable or noisy | Self-hosted or cloud runners are a prerequisite for package 6; without them results are labeled dev-machine only |

## Explicit non-goals

- Hand-written assembly or a bespoke compute DSL.
- Matching the complete libvips/Sharp operation surface; new operations are a separate milestone.
- Dropping the scalar reference or the strict color policy for speed.
- GPU by default, or a GPU abstraction beyond what `wgpu` provides.
- Claims about hardware that was not measured.

## Progress note — package 0 (2026-11-05)

`ProcessingReport::timings` now reports open, decode, transform, encode and total wall-clock per run; `scripts/bench/` holds the corpus generator, the hyperfine/peak-RSS harness and the profiler wrapper; [M2-baseline.md](../benchmarks/M2-baseline.md) records the first dev-machine numbers on a synthetic corpus. Two findings set the order of packages 1 and 2: about 80 % of the transform stage is the sRGB transfer decode executed once per filter tap (`pow` is the top symbol), and the encoder is a fixed 2.7–3.9 s per 1600-wide output because rav1e runs single-threaded with scalar kernels. CPU/wall is 1.0 on 20 cores. Still open in package 0: Sharp and libvips comparators on this machine, a real-photo corpus and SSIMULACRA2 quality matching.

## Progress note — package 1 (2026-11-05)

`src/fast_execute.rs` is the default executor: `f32`, transfer decode once per sample through exact tables, flat tap tables, an O(1) row ring, three-channel opaque RGB and an exact `f32` to sRGB8 encoder. It passes the ADR-0021 differential test against the `f64` oracle, which stays selectable as `WorkingPrecision::F64Reference`. On the dev machine the transform stage fell by 16–40× (5.9 s to 0.17 s at 24 Mpx) and end-to-end time by up to 3.5×; the encoder is now 86–100 % of wall time, so package 2 comes before SIMD. Not done in package 1: the Q1.14 path (package 3, ADR-0021 rule 2) and multi-threading (package 4). See [M2-baseline.md](../benchmarks/M2-baseline.md).

## Progress note — package 2 (2026-11-06)

A standalone, quality-matched [encoder experiment](../research/encoder-experiment/README.md) measured `rav1e` assembly, threads, speed presets, 4:2:0 and bit depth on real photographs at equal SSIMULACRA2. Landed: `avif-write` enables `ravif`'s `asm` (bit-identical, 2.1–2.4× faster encode) and `threading`, and `TransformOptions::encoder_threads` exposes the thread count with 1 as the deterministic default. Result on the dev machine: 24 Mpx JPEG to 1600-wide AVIF in 1.55 s on one encoder thread and 0.44 s on all cores, down from 8.83 s at the start of M2; sequential throughput 13.3 and 27.9 images/s. Remaining in package 2: a direct `rav1e` sink that decouples AV1 tiles from threads and offers 4:2:0 10-bit (−0.5 % to −11.9 % bytes at equal quality), and the libavif-rs encoder comparison. Since 2026-11-06 libavif-rs supports GNU Linux x86-64 as well as macOS ARM64; it still needs nightly, an `unsafe` C ABI and libvmaf, so it can be measured in a standalone experiment but not yet adopted under ADR-0018. See [M2-baseline.md](../benchmarks/M2-baseline.md).

## Progress note — package 2, direct `rav1e` sink (2026-11-06)

Ferrixel now drives `rav1e` itself: rows become 10-bit YCbCr directly in the encoder's frame (no RGBA copy, frame size budgeted), AV1 tiles follow the output size so parallel output is machine-independent, and 4:2:0 is selectable. One-thread 4:4:4 output is byte-identical to the former `ravif` path; with all cores, 24 Mpx to 1600 wide takes 0.41 s at 4:4:4 and 0.39 s at 4:2:0. `ravif`'s tuning tables are kept in `src/av1_tuning.rs` with their BSD-3-Clause notice. Package 2 is complete except the libavif-rs encoder comparison. Decided on 2026-11-06: 4:4:4 stays the default, as in Sharp; 4:2:0 remains an explicit option although it saved bytes at equal SSIMULACRA2 on all five experiment images.

## Progress note — package 3, first step (2026-11-06)

`crates/ferrixel-kernels` exists (ADR-0020, now Experimental): a safe API over `std::arch` kernels with runtime dispatch, `Scalar`, `Neon` and `Avx2` levels for the horizontal resample and the vertical `axpy`, 21 audited `unsafe` blocks, differential tests per level, the chosen level in `ProcessingReport` and a CLI/option ceiling. The f32 executor uses it with four-channel pixels; the ADR-0021 oracle test now runs for every level the host supports, and CI adds an `ubuntu-24.04-arm` job. On Apple M1 the transform stage fell by 1.4–1.6× (24 Mpx 169 ms to 106 ms), from the layout rather than the intrinsics; x86-64 AVX2 is tested but not yet timed. Remaining in package 3: the Q1.14 path of ADR-0021 with its own kernels and measurement, table-driven linearize and quantize kernels, AVX-512 and SVE2 levels once hardware or runners can test them, and an x86-64 timing run.
