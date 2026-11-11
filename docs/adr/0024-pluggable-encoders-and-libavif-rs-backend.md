# ADR-0024: Pluggable output encoders and a libavif-rs backend

- **Status:** Experimental
- **Date:** 2026-11-06
- **Last reviewed:** 2026-11-11
- **Supersedes:** None
- **Partially superseded by:** [0026](0026-codecs-in-one-repository-and-workspace.md), repository/workspace placement and dependency pinning; [0030](0030-still-webp-output-and-format-neutral-encoder-result.md), output result and feature coupling; [0031](0031-workspace-libavif-input-experiment.md), AVIF input and the former duplicate-symbol exclusion
- **Related:** [0017](0017-minimal-public-facade-and-cli.md), [0020](0020-kernel-crate-and-isa-dispatch.md), [0018](0018-optional-codec-and-cms-dependencies.md), [0022](0022-parallel-row-pipeline-and-engine.md), [M2 milestone](../milestones/M2-performance-architecture.md) work package 2, [Sharp comparison](../benchmarks/M2-sharp-comparison.md), [libavif-rs](../../codecs/libavif-rs), [libwebp-rs plan](../research/2026-11-05-libwebp-port-plan.md)

## Context

After work package 5 the encoder is the largest stage on real photographs (about 260 of 356 ms for the 12 Mpx phone photo at 1600×2133), and the [Sharp comparison](../benchmarks/M2-sharp-comparison.md) showed that Sharp's libaom writes 4–12 % smaller files than Ferrixel's `rav1e` at equal SSIMULACRA2. ADR-0018 names libavif-rs as the target for AVIF output, with adoption gates (a) a safe API or an isolated adapter crate, (b) a stable toolchain, (c) a pinnable dependency, (d) CI evidence and (e) documented semantics.

Verified facts on 2026-11-06:

- **libavif-rs** (`f3b2981`) exposes the original libavif C ABI as `pub unsafe extern "C"` Rust functions, requires `nightly-2026-09-07` (`#![feature(extern_types)]`), builds only for aarch64 macOS and x86-64 GNU Linux (`compile_error!` elsewhere), links libvmaf through `pkg-config` (its AOM port's tune-VMAF path), and has no SIMD yet. It is public on GitHub and pinnable by revision.
- **Speed and size of its AOM** (encoder-only probe on the 1600×2133 SSIMULACRA2 reference of the dog photo, 20 threads, M1 Ultra): speed 6 encodes in about 270–300 ms, speed 7 in about 160 ms, speed 5 in 1.7 s. At the score of Ferrixel's `rav1e` default (73.61) it needs 235 kB at speed 6 and 250 kB at speed 7, against 262 kB for `rav1e` speed 6 and 243 kB for Sharp's libaom (effort 4, with SIMD). The scalar port is therefore not slower than `rav1e` with all cores.
- **Symbol clash.** libavif-rs's dav1d port and `rav1d` (used by `avif-decode`, Ferrixel's `avif-read`) both export the `dav1d_*` C symbols. Linking both into one binary produced duplicate-symbol warnings from the macOS linker; calling either then risks undefined behavior.
- **The maintainer's decision** (2026-11-06): integrate libavif-rs now; its `unsafe` C ABI and the deviation from gates (a) and (b) are accepted where nothing else works, since the same people own Ferrixel, libavif-rs and libjxl-rs. A libwebp-rs port with a planned `webp-write` adapter is underway, so the extension point should not be AVIF-specific.

## Decision

1. **Encoders are pluggable at the row boundary.** The public `EncoderBackend` trait (`format()`, `begin(&EncodeRequest) -> Box<dyn RowEncoder>`, `name()`) and `RowEncoder` trait (`write_row(y, rgba)`, `finish() -> EncodedImage`) receive the finished straight-alpha sRGB8 RGBA rows top to bottom, after Ferrixel's decode, color, crop, resample and quantization. `transcode_with` and `Engine::transcode_batch_with` take a backend; `transcode` selects the built-in `rav1e` or still WebP sink. `EncodeRequest` carries output format, typed WebP settings, size, alpha, AVIF quality and speed, resolved encoder threads and chroma layout; each backend validates quality and speed on its own scale. Backend errors keep their kind (`InvalidOptions`, `ResourceExhausted`, `Failed`) and message through `ProcessingError`. The report names the encoder (`encoder_backend`), and the CLI (`ferrixel::cli::run`) takes an optional backend, so an adapter ships the same command and report line.
2. **libavif-rs lives behind an isolated adapter crate.** Since ADR-0026, `crates/ferrixel-libavif` belongs to the common workspace, uses its pinned nightly, and depends on `codecs/libavif-rs` and Ferrixel by local paths. Its original separate workspace and Git pin are superseded. The adapter contains all `unsafe` calls, each with its safety argument and every libavif object owned by a guard. The core crate keeps `#![forbid(unsafe_code)]`. Its codec-free and feature sets without `avif-read`/`cli` support stable Rust 1.98; AVIF input and the libavif encoder adapter need the pinned nightly on supported native targets. Do not use a workspace-wide `--all-features` run as a stable-toolchain check. The backend collects one RGBA frame, converts it with `avifImageRGBToYUV` and encodes with `avifEncoderWrite`; its signals match the `rav1e` sink (CICP 1/13/6, full range, 10 bits by default). `maxThreads` follows the request, and libavif's automatic tiling is on unless one thread was requested, as tiles are for `rav1e`.
3. **No AVIF input next to libavif-rs (2026-11-06 restriction).** The original adapter enabled only JPEG and JPEG XL input while `avif-read` used `avif-decode`/`rav1d`. [ADR-0031](0031-workspace-libavif-input-experiment.md) supersedes this restriction: AVIF input now uses libavif/AOM through a separate safe wrapper, so it can share the encoder adapter's binary. The combined CLI feature set still needs its own scoped link and behavior check.
4. **`rav1e` stays the default for AVIF output** in `transcode`, and the core does not depend on the adapter. Switching the default needs the adapter to meet the remaining ADR-0018 gates (stable toolchain, safe API) or an explicit maintainer decision.

## Consequences

+ At matched SSIMULACRA2, libavif-rs's AOM at speed 6 writes 5–13 % fewer bytes than `rav1e` on the four test photos and is at or below Sharp on all four, at 8–32 % more time with all cores; speed 7 is 25–43 % faster than `rav1e` and still smaller on three of four (Validation).
+ Encoders are replaceable without touching the pipeline; ADR-0030 adds built-in still WebP output and format-tagged `Output::bytes`.
+ The core's stable compatibility and `unsafe` policy do not change; ADR-0026 makes the shared development default nightly.
- The adapter needs nightly and one of two platforms. CI runs it on macOS arm64 and x86-64 Linux. Since libavif-rs `8be367f` it needs no native libraries (see the amendment log).
- The backend materializes a full RGBA frame plus libavif's planes; their
  known sample bytes are reserved before `begin`. Allocation overhead, codec
  scratch and compressed output remain opaque to the engine budget.
- libavif's bytes depend on the thread count (as Sharp's do), unlike `rav1e`'s size-derived tiles.
- The original duplicate-symbol exclusion was removed by ADR-0031's
  `avif-read` replacement. Native/nightly and full decoder-selection gates
  remain distinct from link compatibility.
- As initially implemented in 2026, `pipeline` required `avif-write`; ADR-0030 removed that coupling, so external backends can compile without `rav1e`.

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

The remaining triggers are a stable libavif toolchain or safe API, material changes to its SIMD or AOM profile, a change in the combined AVIF input/output behavior, or evidence that a streaming YUV path to AOM is worth the materialized RGBA frame. The VMAF profile, SIMD, WebP output, and AVIF input triggers have already been reviewed in the amendments below.

## Amendment log

- 2026-11-06 — Initial decision: public row-level encoder backends; libavif-rs adapter crate outside the workspace; measured matched-quality bytes and time.
- 2026-11-06 — CI runs the adapter on x86-64 Linux too: the job builds libvmaf 3.2.1 with the options of libavif-rs's `tools/build_aom_vmaf.py` (Meson 1.7.2, static, no assembly) and caches it; lint and the four adapter tests pass there. libvmaf is linked only because libavif-rs's AOM was translated with `CONFIG_TUNE_VMAF=1`; Ferrixel never uses the VMAF tuning modes.
- 2026-11-07 — Review trigger "SIMD" fired: libavif-rs `c9b583c` adds run-time-dispatched Neon kernels to its AOM, with byte-identical output. Re-measured ([After Neon](../benchmarks/M2-libavif-backend.md#after-neon-libavif-rs-c9b583c)): speed 6 is now smaller and 21–40 % faster than `rav1e` with all cores on all four photographs, 26–37 % faster with one thread, on macOS ARM64 only. The decision stands: `rav1e` stays the default, because the adoption gates of ADR-0018 (stable toolchain, safe API, no libvmaf) are still open and x86-64 has no SIMD yet. The adapter pin moves to `c9b583c`.
- 2026-11-07 — Review trigger "AOM profile without VMAF tuning" fired: libavif-rs `8be367f` builds libaom with `CONFIG_TUNE_VMAF=0` ([libavif-rs ADR 0057](../../codecs/libavif-rs/docs/adr/0057-translate-libaom-without-vmaf-tuning.md), [retained libavif-rs#10 commit](https://github.com/sebastian-software/ferrixel/commit/8be367f0dde9e7e04299d7de400a3e0abe843e42)). Output for every non-VMAF tuning is byte-identical, and AOM now rejects VMAF tuning modes, which Ferrixel never selects. The adapter pin moves to `8be367f`, and both CI jobs drop libvmaf, pkg-config on macOS and the Linux libvmaf source build. The decision stands: `rav1e` stays the default while ADR-0018 gates (a) safe API and (b) stable toolchain are open.
- 2026-11-09 — Review trigger "SIMD" fired for x86-64: libavif-rs `3df161d` dispatches SSE2, SSE4.1 and AVX2 kernels in its AOM on GNU Linux ([libavif-rs ADR 0058](../../codecs/libavif-rs/docs/adr/0058-dispatch-hand-translated-x86-simd-on-gnu-linux.md), [retained libavif-rs#11 commit](https://github.com/sebastian-software/ferrixel/commit/3df161d9715ac743a1c7305f9a37ffaa3ae51c05)), with byte-identical output. Measured on a four-vCPU AMD EPYC 7763 CI runner ([x86-64 Linux with AVX2](../benchmarks/M2-libavif-backend.md#x86-64-linux-with-avx2-libavif-rs-3df161d)): speed 6 is 5–13 % smaller and 41–47 % faster than `rav1e` with all cores, 43–50 % faster with one thread. libavif-rs is now ahead on both CI architectures. The decision stands: `rav1e` stays the default while ADR-0018 gates (a) safe API and (b) stable toolchain are open. The adapter pin moves to `3df161d`.

- 2026-11-10 — ADR-0026 supersedes the separate workspace and Git pin. The adapter and imported codec use the root lockfile and toolchain; the unsafe boundary, default encoder, feature exclusions, and original measured evidence remain unchanged.
- 2026-11-11 — ADR-0030 removes the `avif-write` prerequisite for the backend API, tags encoded results with their format, and adds built-in still WebP output. The direct YUV-to-AOM candidate remains unmeasured and open under issue #60.
- 2026-11-11 — ADR-0031 connects AVIF input to the same workspace port through a separate safe read wrapper with explicit AOM selection. This supersedes decision 3's original linker exclusion and allows a combined input/encoder binary; it does not change the default `rav1e` output choice or complete the Argon and final selection gates.

- 2026-11-11 — The adapter remains the explicit regular opt-in through
  `transcode_with` and its `cli` binary; selecting it from the core as a
  default would create a dependency cycle because the adapter already depends
  on Ferrixel. The additive `required_frame_bytes` backend preflight reports a
  checked bound for the concurrent RGBA and libavif YUV(A) sample planes, and
  the engine reserves it from before `begin` through `finish`. The port's
  `avifImageAllocatePlanes` uses tightly packed rows, including rounded-up 420
  chroma dimensions. Existing backends default to zero and can remain opaque.
  Allocator overhead, conversion scratch, AOM allocations and encoded output
  are not covered; `max_output_bytes` is checked after the output exists.
  This is a pixel-frame reservation, not a process-memory cap. Tests cover
  invalid or overflowing dimensions, row errors, output limits, alpha and
  metadata across 8/10-bit, 420/444 and thread settings, plus a same-process
  AVIF input-to-output path. On macOS ARM64, FFmpeg 9.0.2/libdav1d independently
  decoded a 96×64 10-bit 444 adapter CLI output; `ffprobe` identified
  full-range BT.709/sRGB/BT.601 signals. This one file is not a codec
  conformance corpus. The nightly and native-target restrictions, ADR-0025
  two-architecture quality, memory, binary-size and corpus selection criteria,
  and release/license gates remain open. The encoder follows the reference-origin
  route; decoder conformance is a separate criterion where applicable.
  Reconsider the default and its dependency routing when those gates and a safe
  package boundary are established.

- 2026-11-11 — [ADR-0037](0037-selectable-workspace-avif-encoder.md) extracts
  the C ABI boundary into an engine-independent safe writer bridge. The
  compatibility adapter delegates to the same backend used by an explicit
  `avif-own-write` facade and CLI selection. This supersedes the dependency
  cycle that prevented regular selection, not the Rav1e default or the
  ADR-0025 comparison and selection gates. Known frame accounting and opaque
  codec/output limitations remain unchanged.
