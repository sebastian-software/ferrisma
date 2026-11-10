# ADR-0024: Pluggable output encoders and a libavif-rs backend

- **Status:** Experimental
- **Date:** 2026-11-06
- **Last reviewed:** 2026-11-10
- **Supersedes:** None
- **Partially superseded by:** [0026](0026-codecs-in-one-repository-and-workspace.md), repository/workspace placement and dependency pinning only
- **Related:** [0017](0017-minimal-public-facade-and-cli.md), [0020](0020-kernel-crate-and-isa-dispatch.md), [0018](0018-optional-codec-and-cms-dependencies.md), [0022](0022-parallel-row-pipeline-and-engine.md), [M2 milestone](../milestones/M2-performance-architecture.md) work package 2, [Sharp comparison](../benchmarks/M2-sharp-comparison.md), [libavif-rs](../../codecs/libavif-rs), [libwebp-rs plan](../research/2026-11-05-libwebp-port-plan.md)

## Context

After work package 5 the encoder is the largest stage on real photographs (about 260 of 356 ms for the 12 Mpx phone photo at 1600×2133), and the [Sharp comparison](../benchmarks/M2-sharp-comparison.md) showed that Sharp's libaom writes 4–12 % smaller files than Ferrixel's `rav1e` at equal SSIMULACRA2. ADR-0018 names libavif-rs as the target for AVIF output, with adoption gates (a) a safe API or an isolated adapter crate, (b) a stable toolchain, (c) a pinnable dependency, (d) CI evidence and (e) documented semantics.

Verified facts on 2026-11-06:

- **libavif-rs** (`f3b2981`) exposes the original libavif C ABI as `pub unsafe extern "C"` Rust functions, requires `nightly-2026-09-07` (`#![feature(extern_types)]`), builds only for aarch64 macOS and x86-64 GNU Linux (`compile_error!` elsewhere), links libvmaf through `pkg-config` (its AOM port's tune-VMAF path), and has no SIMD yet. It is public on GitHub and pinnable by revision.
- **Speed and size of its AOM** (encoder-only probe on the 1600×2133 SSIMULACRA2 reference of the dog photo, 20 threads, M1 Ultra): speed 6 encodes in about 270–300 ms, speed 7 in about 160 ms, speed 5 in 1.7 s. At the score of Ferrixel's `rav1e` default (73.61) it needs 235 kB at speed 6 and 250 kB at speed 7, against 262 kB for `rav1e` speed 6 and 243 kB for Sharp's libaom (effort 4, with SIMD). The scalar port is therefore not slower than `rav1e` with all cores.
- **Symbol clash.** libavif-rs's dav1d port and `rav1d` (used by `avif-decode`, Ferrixel's `avif-read`) both export the `dav1d_*` C symbols. Linking both into one binary produced duplicate-symbol warnings from the macOS linker; calling either then risks undefined behavior.
- **The maintainer's decision** (2026-11-06): integrate libavif-rs now; its `unsafe` C ABI and the deviation from gates (a) and (b) are accepted where nothing else works, since the same people own Ferrixel, libavif-rs and libjxl-rs. A libwebp-rs port with a planned `webp-write` adapter is underway, so the extension point should not be AVIF-specific.

## Decision

1. **Encoders are pluggable at the row boundary.** The public `EncoderBackend` trait (`begin(&EncodeRequest) -> Box<dyn RowEncoder>`, `name()`) and `RowEncoder` trait (`write_row(y, rgba)`, `finish() -> EncodedImage`) receive the finished straight-alpha sRGB8 RGBA rows top to bottom, after Ferrixel's decode, color, crop, resample and quantization. `transcode_with` and `Engine::transcode_batch_with` take a backend; `transcode` keeps the built-in `rav1e` sink. `EncodeRequest` carries size, alpha, quality, speed, resolved encoder threads and chroma layout; each backend validates quality and speed on its own scale. Backend errors keep their kind (`InvalidOptions`, `ResourceExhausted`, `Failed`) and message through `ProcessingError`. The report names the encoder (`encoder_backend`), and the CLI (`ferrixel::cli::run`) takes an optional backend, so an adapter ships the same command and report line.
2. **libavif-rs lives behind an isolated adapter crate.** Since ADR-0026, `crates/ferrixel-libavif` belongs to the common workspace, uses its pinned nightly, and depends on `codecs/libavif-rs` and Ferrixel by local paths. Its original separate workspace and Git pin are superseded. The adapter contains all `unsafe` calls, each with its safety argument and every libavif object owned by a guard. The core crate keeps `#![forbid(unsafe_code)]`, stable Rust, MSRV 1.98 and `--all-features` builds. The backend collects one RGBA frame, converts it with `avifImageRGBToYUV` and encodes with `avifEncoderWrite`; its signals match the `rav1e` sink (CICP 1/13/6, full range, 10 bits by default). `maxThreads` follows the request, and libavif's automatic tiling is on unless one thread was requested, as tiles are for `rav1e`.
3. **No AVIF input next to libavif-rs.** Until AVIF input also comes from libavif-rs, the adapter enables only JPEG and JPEG XL input, and its tests decode with libavif-rs's own decoder rather than `avif-decode`.
4. **`rav1e` stays the default** for `transcode`, and the core does not depend on the adapter. Switching the default needs the adapter to meet the remaining ADR-0018 gates (stable toolchain, safe API) or an explicit maintainer decision.

## Consequences

+ At matched SSIMULACRA2, libavif-rs's AOM at speed 6 writes 5–13 % fewer bytes than `rav1e` on the four test photos and is at or below Sharp on all four, at 8–32 % more time with all cores; speed 7 is 25–43 % faster than `rav1e` and still smaller on three of four (Validation).
+ Encoders become replaceable without touching the pipeline; a libwebp-rs `webp-write` adapter can use the same interface (and will rename `Output::avif_bytes` when output stops being AVIF only).
+ The core's stable compatibility and `unsafe` policy do not change; ADR-0026 makes the shared development default nightly.
- The adapter needs nightly and one of two platforms. CI runs it on macOS arm64 and x86-64 Linux. Since libavif-rs `8be367f` it needs no native libraries (see the amendment log).
- The backend materializes a full RGBA frame plus libavif's planes outside Ferrixel's budget (reported, not bounded), and AOM's memory is opaque.
- libavif's bytes depend on the thread count (as Sharp's do), unlike `rav1e`'s size-derived tiles.
- A binary cannot have both libavif-rs and Ferrixel's `avif-read`.
- `pipeline` (and therefore the backend API) still requires the `avif-write` feature, so `rav1e` is compiled even when only a backend is used.

## Alternatives considered

- **A Cargo feature in the core** (`avif-write-libavif`). Rejected for now: it makes `--all-features` nightly-only and puts `unsafe` FFI into the core crate, against ADR-0018 and ADR-0020's isolation rule.
- **Declaring libavif's C ABI by hand** (own `extern "C"` and `#[repr(C)]` structs) instead of calling the generated Rust functions. Rejected: it duplicates the struct layouts libavif-rs already validates.
- **Calling libaom's encoder API directly** (`libavif-aom`'s `aom::raw`) and writing the container with `avif-serialize`. Possible later for streaming YUV rows without the RGBA frame; libavif's own RGB to YUV conversion and container are the reference-equivalent path today.
- **Waiting for libavif-rs's safe API and stable toolchain.** Rejected by the maintainer's decision; the measured byte savings are available now.

## Validation

- Core tests (`pipeline_tests`): a recording backend receives one request with the resolved settings, every row in order with `4 * width` bytes and alpha 255, identical rows for 1 and 4 transform threads; backend errors in `begin`, `write_row` and `finish` keep their kind and message on both executors; input errors never reach the backend; the engine runs one encoder per image with one thread.
- Adapter tests (`crates/ferrixel-libavif/tests`): output decoded by libavif-rs's decoder has the requested size, CICP 1/13/6, full range and depth, and differs from the rows Ferrixel produced by a mean below 2 codes (4:4:4) or 4 codes (4:2:0) at quality 95, for 10 and 8 bits and 1 or all threads; straight alpha survives; out-of-order or missing rows and invalid settings fail.
- Matched comparison (target A of the Sharp comparison, all cores, 4:4:4; `rav1e` bytes at its quality-80 anchor):

| image | `rav1e` speed 6 | Sharp (libaom, effort 4) | libavif-rs speed 6 | libavif-rs speed 7 |
| --- | ---: | ---: | ---: | ---: |
| dog 1600×2133 | 261,847 | 242,950 | 236,050 (−9.9 %) | 248,455 (−5.1 %) |
| flower 1600×1067 | 126,582 | 115,932 | 110,156 (−13.0 %) | 117,493 (−7.2 %) |
| kodim03 512×341 | 14,467 | 13,593 | 12,907 (−10.8 %) | 13,519 (−6.6 %) |
| kodim23 512×341 | 14,335 | 13,569 | 13,569 (−5.3 %) | 14,693 (+2.5 %) |

- End-to-end time at those settings, best of five, all cores, load 3.9–5.9 ([details](../benchmarks/M2-libavif-backend.md), including one thread):

| image | `rav1e` speed 6 | libavif-rs speed 6 | libavif-rs speed 7 |
| --- | ---: | ---: | ---: |
| dog | 357 ms | 401 ms | 267 ms |
| flower | 206 ms | 271 ms | 147 ms |
| kodim03 | 97 ms | 110 ms | 55 ms |
| kodim23 | 97 ms | 105 ms | 56 ms |

With one thread, libavif-rs speed 6 takes 21–50 % longer than `rav1e`: its AOM has no SIMD yet.

- The decision is falsified if libavif-rs at its matched setting is not smaller than `rav1e` at equal score on real photographs, or if a backend can change the pixels Ferrixel produced.

## Review when

libavif-rs gains a stable toolchain, a safe API, SIMD, or an AOM profile without VMAF tuning (`CONFIG_TUNE_VMAF=0`), which would drop the libvmaf requirement; Ferrixel reads AVIF through libavif-rs (which removes the symbol clash); the libwebp-rs adapter needs the interface; a streaming YUV path to AOM becomes worth the materialized RGBA frame.

## Amendment log

- 2026-11-06 — Initial decision: public row-level encoder backends; libavif-rs adapter crate outside the workspace; measured matched-quality bytes and time.
- 2026-11-06 — CI runs the adapter on x86-64 Linux too: the job builds libvmaf 3.2.1 with the options of libavif-rs's `tools/build_aom_vmaf.py` (Meson 1.7.2, static, no assembly) and caches it; lint and the four adapter tests pass there. libvmaf is linked only because libavif-rs's AOM was translated with `CONFIG_TUNE_VMAF=1`; Ferrixel never uses the VMAF tuning modes.
- 2026-11-07 — Review trigger "SIMD" fired: libavif-rs `c9b583c` adds run-time-dispatched Neon kernels to its AOM, with byte-identical output. Re-measured ([After Neon](../benchmarks/M2-libavif-backend.md#after-neon-libavif-rs-c9b583c)): speed 6 is now smaller and 21–40 % faster than `rav1e` with all cores on all four photographs, 26–37 % faster with one thread, on macOS ARM64 only. The decision stands: `rav1e` stays the default, because the adoption gates of ADR-0018 (stable toolchain, safe API, no libvmaf) are still open and x86-64 has no SIMD yet. The adapter pin moves to `c9b583c`.
- 2026-11-07 — Review trigger "AOM profile without VMAF tuning" fired: libavif-rs `8be367f` builds libaom with `CONFIG_TUNE_VMAF=0` ([libavif-rs ADR 0057](../../codecs/libavif-rs/docs/adr/0057-translate-libaom-without-vmaf-tuning.md), [libavif-rs#10](https://github.com/sebastian-software/libavif-rs/pull/10)). Output for every non-VMAF tuning is byte-identical, and AOM now rejects VMAF tuning modes, which Ferrixel never selects. The adapter pin moves to `8be367f`, and both CI jobs drop libvmaf, pkg-config on macOS and the Linux libvmaf source build. The decision stands: `rav1e` stays the default while ADR-0018 gates (a) safe API and (b) stable toolchain are open.
- 2026-11-09 — Review trigger "SIMD" fired for x86-64: libavif-rs `3df161d` dispatches SSE2, SSE4.1 and AVX2 kernels in its AOM on GNU Linux ([libavif-rs ADR 0058](../../codecs/libavif-rs/docs/adr/0058-dispatch-hand-translated-x86-simd-on-gnu-linux.md), [libavif-rs#11](https://github.com/sebastian-software/libavif-rs/pull/11)), with byte-identical output. Measured on a four-vCPU AMD EPYC 7763 CI runner ([x86-64 Linux with AVX2](../benchmarks/M2-libavif-backend.md#x86-64-linux-with-avx2-libavif-rs-3df161d)): speed 6 is 5–13 % smaller and 41–47 % faster than `rav1e` with all cores, 43–50 % faster with one thread. libavif-rs is now ahead on both CI architectures. The decision stands: `rav1e` stays the default while ADR-0018 gates (a) safe API and (b) stable toolchain are open. The adapter pin moves to `3df161d`.

- 2026-11-10 — ADR-0026 supersedes the separate workspace and Git pin. The adapter and imported codec use the root lockfile and toolchain; the unsafe boundary, default encoder, feature exclusions, and original measured evidence remain unchanged.
