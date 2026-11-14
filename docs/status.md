# Current project status

**Reviewed: 2026-11-14** (`main` at `5004f34e`). Ferrisma is an experimental image engine
and codec workspace. M1 is implemented in part and under validation, not a
release candidate. This page owns the current cross-component overview;
the linked guides and evidence records own the details.

The `ferrisma` npm package and crate at version `0.0.0` have been published as
placeholders. They contain no image processing implementation; the functional
engine is available from this repository. The name and domain are recorded in
[ADR-0038](adr/0038-ferrisma-project-name.md). The domain `ferrisma.dev` does
not serve this project's site yet: on 2026-11-13 it answered with a GitHub
Pages page that is not ours
([#251](https://github.com/sebastian-software/ferrisma/issues/251)).

The repository contains both the pipeline a caller can exercise today and the
codec ports intended for its future backends. A port's coverage, its Rust API,
its integration into the engine, and release acceptance are separate questions.

## What works through the engine

The optional Rust facade and CLI run this supported SDR path:

```text
JPEG / WebP / JPEG XL / AVIF
        ↓
decode and interpret rendered color
        ↓
apply orientation → crop → resize in linear light
        ↓
quantize → encode AVIF, still WebP, or JPEG XL (lossy or lossless)
```

| Format | Current engine path | Workspace port relationship |
| --- | --- | --- |
| JPEG | Input through workspace `libjpeg-turbo-rs` (`jpeg-read`); codec-level crop request and budgeted RGB row | Scaled IDCT remains unused by the adapter; codec memory and reconstruction are opaque |
| JPEG XL | Input through workspace `libjxl-rs` via `ferrisma-jxl-decoder` (`jpeg-xl-own-read`, in `cli`), the default whenever compiled in since 2026-11-12; `jxl-oxide` (`jpeg-xl-read`) stays an explicit backend (`--jxl-decoder=jxl-oxide`) and is the default only without `jpeg-xl-own-read`. Experimental output (`jpeg-xl-write`, in `cli` since 2026-11-12): lossy d1.0 by default through `ferrisma-jxl-encoder`, or lossless | The workspace reader has no ROI request: every crop reconstructs and budgets a full F32 frame and reports `FullFrameFallback`. The external conformance corpus passes on macOS ARM64 ([record](../codecs/libjxl-rs/docs/CONFORMANCE.md)). The owner confirmed it as the default on 2026-11-13 ([ADR-0041](adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md) D1). Since 2026-11-14 the decoder runs its NEON path by default on AArch64 and its scalar path on x86-64 (libjxl-rs [ADR 0004](../codecs/libjxl-rs/docs/adr/0004-simd-paths.md)). Its speed is reported against libjxl, not as a gate: on AArch64, one machine, 1.3–2.9× libjxl's NEON build with one thread, and 1.28× and 1.57× on the two inputs measured again after the render-pipeline changes ([record of 2026-11-14](benchmarks/v1.0-jxl-decode-three-column-2026-11-14.md)); no x86-64 measurement of the current decoder exists. The jxl-oxide gap of 2026-11-11 (about 3.6× slower full, 9.5× slower crop, [one photo](benchmarks/M2-jxl-input-switch.md)) predates the SIMD path and is superseded; it has not been measured again. x86-64 conformance stays open ([ADR-0032](adr/0032-jpeg-xl-workspace-decoder-input.md)). Lossy output (VarDCT, effort 7, distance 0.05 to below 3.0, XYB with enum sRGB, patches off, straight 8-bit alpha coded losslessly) uses the port's public encoder, byte-identical with libjxl on macOS AArch64, and encodes every still image since the 2026-11-12 alpha/patches amendment; the patches-off policy and its unmeasured size cost on text-like content are pending owner confirmation. Lossless uses the verified effort-2 profile (enum sRGB, 8-bit, straight alpha) and reserves its measured one-shot peak since 2026-11-13, so under the default 256 MiB budget the largest lossless output is about 3.5 MP instead of about 9.5 MP (#223). The engine reads its own straight-alpha lossy output back under `--use-rendered-icc` since 2026-11-13 (#222). The lossy default and the lossy codec-memory estimate are pending owner confirmation ([ADR-0039](adr/0039-jpeg-xl-output-through-the-workspace-encoder.md)) |
| AVIF | Workspace libavif/AOM input (`avif-read`); workspace libavif/AOM output (`avif-own-write`, in `cli`), the default encoder whenever compiled in since 2026-11-12; Rav1e (`avif-write`) stays an explicit backend (`--avif-encoder=rav1e`) and the default only without `avif-own-write` | Independent read/write wrappers isolate the unsafe ABI. `quality`/`speed` are passed unchanged on the selected encoder's own scale; the scales are not comparable (rav1e 70/80/90 ≈ AOM 50/55/78). Native/nightly requirements stay; the workspace port is the default by owner decision (ADR-0041 D1), the default quality per encoder (#216) is still pending ([ADR-0037](adr/0037-selectable-workspace-avif-encoder.md)) |
| WebP | Still input and output through workspace `libwebp-rs` (`webp-read`, `webp-write`); lossy crop requests, lossless full-frame internal work | The output sink budgets an ARGB frame and predictable lossy planes, tags sRGB ICC, and reports opaque codec memory; animation is unsupported |

The `cli` feature enables the current engine adapters and ICC path together.
The authoritative feature selection is in [Cargo.toml](../Cargo.toml); the
[public facade](../src/pipeline.rs) and [CLI](../src/cli.rs) expose the supported
operations. JPEG output is not an engine feature; JPEG XL output (lossy by default, or
lossless) requires `jpeg-xl-write`, which `cli` includes.

Current behavior and boundaries:

- **Color and geometry:** SDR sRGB/Display P3 reference math, Area and Lanczos3
  resizing, and opt-in RGB ICC conversion through `moxcms`. Input color must be
  explicit. The rendered-ICC path accepts opaque RGB and RGBA with straight
  alpha, which bypasses the transform (#222); HDR, contradictory metadata,
  premultiplied or unspecified alpha under ICC, and other unsupported alpha
  cases fail closed. The JPEG input adapter emits RGB8 and rejects CMYK/YCCK files;
  supported JPEG, WebP, JPEG XL, and AVIF orientation is applied before crop;
  AVIF `clap` and unsupported transform orderings are rejected.
- **Execution:** the default `f32` executor is checked against an `f64`
  reference. Runtime CPU dispatch, a parallel row pipeline, and batch execution
  exist. Per-image thread requests above 256 fail before decoding; batch worker
  counts are capped at 256. OS worker-start failure returns a resource error
  after joining partially started workers ([ADR-0022](adr/0022-parallel-row-pipeline-and-engine.md)).
  GPU execution and a general multi-output DAG remain design work.
- **Thumbnail facade:** `transcode_resized` resolves one or two dimensions,
  inside/centered-cover/fill geometry, and explicit no-enlargement behavior
  after orientation and an optional upright crop. Existing exact-size calls
  retain their pixel behavior. Results report actual dimensions; the public
  `Output` type gains fields in this unpublished experimental API. JPEG/WebP
  header inspection has byte/pixel limits and does not validate complete pixel
  data or assume a color profile ([ADR-0034](adr/0034-thumbnail-geometry.md)).
- **Memory:** engine-managed frame, row, and scratch allocations are budgeted
  and reported. Working-linear resize also reserves a conservative tap-table
  and scheduling allowance before footprint construction; reported peaks now
  include it ([ADR-0004](adr/0004-buffer-ownership-and-budget.md)). Codec
  allocations, graph nodes, allocator/runtime internals, and worker stacks
  remain outside that budget. JPEG and lossless WebP codec internals, AVIF decoding, and AVIF
  encoding may require full frames; there is no process-wide memory-cap guarantee.
  External encoders can declare known simultaneous pixel-frame bytes before
  allocation; the engine holds that reservation through encoder completion.
  The libavif backend declares its RGBA and YUV(A) planes, while codec scratch,
  allocator overhead and encoded bytes remain outside that reservation.
  The AVIF reader charges its full RGB output to the engine budget but retains
  opaque libavif/AOM scratch. On one measured photo it took 8–14% longer and
  about 2× peak RSS versus the former interim decoder; this does not rank
  decoders on other workloads ([evidence](benchmarks/M2-avif-input.md)).
- **Planar experiments:** borrowed YUV420P10 supports even-origin crop-only
  views. It is not a general high-bit-depth pipeline, and has no planar resize
  or HDR tone-mapping policy.

See [architecture](architecture.md), [M1 evidence and gates](milestones/M1-evidence-and-gates.md),
and [the color contract](design/color-management-contract.md) for the tested
contracts and their limits.

## Components in the workspace

There are **15 Cargo packages**. The four packages in the AVIF row below are
separate workspace members; they are grouped here by their shared port.

| Component | Implemented role | Remaining boundary or adoption gate |
| --- | --- | --- |
| [`ferrisma`](../Cargo.toml) | Feature-gated facade/CLI and the SDR pipeline above | M1 quality, resource, security, license, and release acceptance |
| [`ferrisma-kernels`](../crates/ferrisma-kernels) | Safe API over scalar, NEON, AVX2, and AVX-512 resampling; runtime selection | Host-specific evidence still applies; no SVE2 kernel or GPU backend |
| [`libsharpyuv-core`](../crates/libsharpyuv-core) | One safe SharpYUV numeric conversion shared by WebP and AVIF | Separate adapters preserve WebP slices/SIMD and AVIF C ABI/allocation behavior; measured AVIF kernel cost and native oracle gates are recorded in [ADR-0033](adr/0033-shared-sharpyuv-conversion-core.md) |
| [`ferrisma-libavif`](../crates/ferrisma-libavif/README.md) | Compatibility adapter and CLI re-exporting the engine's libavif backend | Delegates to the independent writer; native/nightly and codec acceptance gates remain |
| [`ferrisma-avif-read`](../crates/ferrisma-avif-read) | Safe, independent read wrapper over libavif's AOM decoder; selected by `avif-read` | Pinned nightly and supported macOS ARM64/GNU Linux x86-64; full Argon/dav1d adoption evidence remains open |
| [`ferrisma-avif-write`](../crates/ferrisma-avif-write) | Independent safe row writer over the workspace AOM encoder; selected by `avif-own-write` | Isolates unsafe ABI calls; known RGBA/YUV(A) bytes are reserved, codec scratch and encoded output remain opaque |
| [`ferrisma-jxl-decoder`](../crates/ferrisma-jxl-decoder) | Safe, independent wrapper for the default local JPEG XL decoder (`jpeg-xl-own-read`) | Full-frame fallback for every crop; default confirmed (ADR-0041 D1); speed is reported against libjxl, `jxl-oxide` is context; x86-64 conformance open |
| [`ferrisma-jxl-encoder`](../crates/ferrisma-jxl-encoder) | Safe, independent wrapper for the port's public JPEG XL encoder; lossy output of `jpeg-xl-write` | Admits only the port's lossy profile (straight 8-bit alpha; patches off by default, on only by request, then a typed `PatchDictionary` admission for text-like content); codec memory is a source-derived estimate; x86-64 parity and timing open |
| [`libjpeg-turbo-rs`](../codecs/libjpeg-turbo-rs/README.md) | 8-bit decoder, scaled decode, crop/skip, metadata, safe scanlines, and SIMD; selected for engine JPEG input | Scaled-IDCT quality policy and deeper cross-platform adoption evidence remain open; encoding, lossless JPEG, and 12/16-bit samples are outside the port's scope |
| [`libwebp-rs`](../codecs/libwebp-rs/README.md) | Safe decode/encode APIs, incremental decode, containers/animation, and SharpYUV | Still-image input and output adopted through `webp-read` and `webp-write`; SIMD paths are gated against libwebp's SIMD build per level ([status](../codecs/libwebp-rs/docs/SIMD-STATUS.md)) |
| [`libjxl-rs`](../codecs/libjxl-rs/docs/PORT_STATUS.md) | Incremental decoder with a SIMD translation of every decode-path Highway export (NEON selected by default on AArch64, scalar on x86-64; AC groups decode concurrently under a runner), C-ABI surface, and reference comparison suites | Default engine JPEG XL input since 2026-11-12, confirmed by the owner on 2026-11-13 ([ADR-0041](adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md) D1), and the lossy and lossless `jpeg-xl-write` profiles; public APIs are provisional, encoder incomplete (patch encoding, alpha deeper than 12 bits, other efforts) and scalar only (its Highway exports are unported, #231, #243), and deeper acceptance remains open |
| [`libavif-rs`](../codecs/libavif-rs/README.md), `libavif-aom`, `libavif-dav1d`, `libavif-sharpyuv` | AVIF container and codec ports, using generated and owned Rust with C-ABI boundaries | No stable safe Rust API claim; validated native hosts are macOS ARM64 and GNU Linux x86-64; package-specific deep gates remain |
| [`ferrisma-xtask`](../tools/xtask) | Standard-library Rust control plane, invoked as `cargo xtask`, for affected components and validation stages | Existing Python oracle harnesses migrate incrementally without losing evidence |

The engine forbids unsafe code. The kernel, adapter, and codec boundaries have
their own policies; that engine rule does not describe the entire dependency
graph. Package versions, licenses, and eventual publication remain independent.
All four original codec repositories are retired; work and issue tracking now
live here. [Codec development](codecs.md) owns setup, package selection,
provenance, toolchains, and release boundaries.

## How validation is organized

| Tier | Purpose | What it does not establish |
| --- | --- | --- |
| Scoped PR checks | Affected-component Rust tests, retained reference fixtures, provenance, linting, and MSRV checks | A fresh comparison with every C/C++ oracle on every change |
| Nightly / selected deep branch runs | Live oracle comparisons, sanitizers, profiles, and native architecture checks where supported | Completion of manual exhaustive suites or the M1 release gates |
| Manual exhaustive checks | AOM original-consumer, recovery, translation, and other specifically selected acceptance work | A blanket guarantee for untested platforms or feature combinations |

Required live oracles fail their jobs if unavailable. Changes to oracle pins,
goldens, unsafe ABI, SIMD, or floating-point behavior require the relevant deep
evidence before acceptance. No expected output is refreshed merely to turn a
failure green. [ADR-0027](adr/0027-tiered-validation-and-rust-test-orchestration.md)
owns this policy; [GitHub Actions](https://github.com/sebastian-software/ferrisma/actions)
shows current run results. A green PR check is not full codec certification.

## Next integration and release gates

**v1.0 definition** ([ADR-0041](adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md),
owner decision of 2026-11-13). Version 1.0 is reserved for the state in
which every codec port's results are 1:1 with its ported original: the
port's scalar path bit-identical to the original's scalar build, and its
SIMD paths (NEON on AArch64; AVX2 and the lower x86 levels the original
selects) bit-identical to the original's SIMD build at the same level, on
AArch64 and on x86-64, with NaN payloads the only exemption. A port without
the original's SIMD paths in that scope does not meet it. There is **no
performance gate**: speed is reported against the original in three columns
(scalar one thread, SIMD one thread, SIMD all threads), and third-party
libraries are context. Validation is local first (AArch64 natively, x86-64
under Rosetta 2) with one hosted Linux x86-64 run per release candidate.
Until then, releases are 0.x pre-releases from `main`. No port meets the
full gate on both targets yet; [the quality gate guide](quality-gate.md)
lists what each port can check today and the recorded results.

**Hosted cross-check (state of 2026-11-14, `main` at `5004f34e`).** The
first hosted Linux x86-64 run of the SIMD-against-SIMD gates (nightly run
[43401560676](https://github.com/sebastian-software/ferrisma/actions/runs/43401560676)
on `main` at `dbf7b095`) failed for libwebp-rs and libjxl-rs in the C and C++
oracle processes ([#252](https://github.com/sebastian-software/ferrisma/issues/252)).
Both causes were defects of the test scaffolding (buffers not aligned or
sized like the originals' own) and are fixed on `main`.

- **libwebp-rs** passed on hosted Linux x86-64 at every level: run
  [43471093624](https://github.com/sebastian-software/ferrisma/actions/runs/43471093624)
  (`WEBP: Checks`, branch `v1/webp` at `690ad251`, whose `codecs/libwebp-rs`
  tree is identical to `main`'s), `avx2`, `sse41` and `sse2`, 1,131 tests
  each, 0 failed.
- **libjxl-rs** is still open on x86-64 on `main`. In run
  [43533597186](https://github.com/sebastian-software/ferrisma/actions/runs/43533597186)
  (branch `v1/jxl-merge7` at `718b32d6`) four row tests (CMS, tone mapping,
  splines, blending) differed from the static AVX2 oracle in single
  words (one to three units in the last place, once the sign of a zero); the analysis in port ADR 0004 traces each to a
  function outside Highway's per-target code that the static build
  contracts with FMA and upstream's release build does not.
  The owner decided on 2026-11-14 that the AVX2 reference is upstream's
  dispatch build capped at AVX2 (ADR-0041 section 1). That oracle is on
  `main` since `81d2adab` (port ADR 0004, "AVX2 reference: upstream's
  dispatch build"), with two follow-up fixes. With it, run
  [43558043274](https://github.com/sebastian-software/ferrisma/actions/runs/43558043274)
  (branch `v1/jxl-avx2-fixes`, based on `718b32d6`, before the NEON default
  and the threading and row changes) passed every job, and in run
  [43599382021](https://github.com/sebastian-software/ferrisma/actions/runs/43599382021)
  (the merged tree, with those changes) all four ordinary-oracle test shards
  passed on AVX2; its sanitized shards had not finished when this was
  written. The decoder keeps the scalar path
  on x86-64 until a hosted x86-64 run of the combined decoder passes (port
  ADR 0004, "Decoder path" condition (d); #96).
- **AOM:** no workflow runs the comparison with the original's SIMD build
  (`tools/check_aom_simd_original.py`), so no hosted run of it exists.
- **Nightly AVIF `root / generate` (#204):** the cause, an owned source file
  the root profile checks did not know, is fixed on `main` (`63d77626`,
  libavif-rs ADR 0054 amendment); no nightly has completed since (run
  [43578105956](https://github.com/sebastian-software/ferrisma/actions/runs/43578105956)
  was queued).

**Performance records against the original** (reported, not gated): the
[three-column record of 2026-11-13](benchmarks/v1.0-three-column-2026-11-13.md)
covers every codec on five CI platforms at `e4038a4a`, before the JPEG XL
SIMD work; the [JPEG XL decode record of 2026-11-14](benchmarks/v1.0-jxl-decode-three-column-2026-11-14.md)
covers the current decoder on one AArch64 machine, with addenda after the
concurrent AC group decode and the render-pipeline changes. Neither is a
universal speed claim.

- Complete the independent image-quality, hardening, resource, and dependency
  evidence in the [M1 gate register](milestones/M1-evidence-and-gates.md).
- Adopt workspace codecs through real end-to-end cases under
  [ADR-0025](adr/0025-codec-selection-reference-conformance-and-image-scope.md),
  preserving rendered color, orientation, buffer ownership, and conformance.
  Workspace membership alone does not satisfy those gates.
- Meet the quality gate above for each port on both targets. The oracles of
  the originals exist (scalar and SIMD; x86-64 builds under Rosetta for
  libjpeg-turbo-rs, libwebp-rs, libjxl-rs and libaom). Still open: the
  libjxl-rs encoder's Highway exports are unported (#231, #243), so the
  encoder meets at most the scalar half; its decoder's SIMD path, which has
  every decode-path export and is the default on AArch64 since 2026-11-14
  (port ADR 0004), stays unselected on x86-64 until hosted AVX2 evidence of
  the combined decoder exists (#96); the AOM port dispatches 513 RTCD
  functions to Neon on macOS ARM64 since 2026-11-13 (libavif-rs ADR 0056
  amendment), but its x86 kernels build only for GNU Linux x86-64, and the
  `avx2` level against the original has never run (#242, #125, #124, #122);
  the hosted Linux x86-64 cross-check for libjxl-rs and AOM (#241, #252);
  the owner's open questions O1–O9
  ([#250](https://github.com/sebastian-software/ferrisma/issues/250)). The
  workspace JPEG XL reader and AVIF writer are the confirmed defaults
  (ADR-0041 D1); x86-64 JPEG XL conformance stays open, and lossy d1.0 JPEG
  XL output (ADR-0039) and the default AVIF quality (#216) are still pending
  owner confirmation (ADR-0041 O5). The
  [supported matrix](supported-matrix.md) lists the current defaults, their
  limits and the 2026-11-12 macOS ARM64 verification.
- Validate the extracted shared SharpYUV core with native Linux and macOS
  API, ABI, allocation, callback, and full original-consumer oracles. Preserve
  the independent C reference and review the measured AVIF kernel cost in
  [ADR-0033](adr/0033-shared-sharpyuv-conversion-core.md) and
  [issue #75](https://github.com/sebastian-software/ferrisma/issues/75).
- Continue verified Rust/TypeScript tooling consolidation. The shared runner
  is Rust already; retained Python harnesses are compatibility assets, not a
  new orchestration layer.

The [roadmap](roadmap.md) connects these gates to the milestone plans.
Historical benchmark results have their own commits, corpora, hardware, and
quality settings; they are not current universal speed claims. Update this
overview when integration or acceptance changes, with a link to the owning
evidence rather than a percentage-complete estimate.
