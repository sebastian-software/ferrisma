# ADR-0018: Feature-gated codec and CMS dependencies, MSRV and lockfile

- **Status:** Experimental
- **Date:** 2026-11-05
- **Last reviewed:** 2026-11-12
- **Supersedes:** None
- **Partially superseded by:** [0031](0031-workspace-libavif-input-experiment.md), experimental AVIF input and its native/nightly toolchain boundary; final adoption gates remain open.
- **Related:** [libavif-rs](../../codecs/libavif-rs), [libjxl-rs](../../codecs/libjxl-rs), [0005](0005-codec-capabilities-and-pixel-contracts.md), [0010](0010-dual-licensing.md), [0012](0012-codec-source-contract.md), [0015](0015-typed-output-color-signals.md), [0017](0017-minimal-public-facade-and-cli.md), [encoder and CMS research](../research/2026-10-31-encoder-and-cms-candidates.md), [codec adapter audit](../research/codec-adapter-audit.md), [JPEG and WebP evaluation](../research/2026-11-05-jpeg-and-webp-codec-evaluation.md), [libwebp port plan](../research/2026-11-05-libwebp-port-plan.md), [license audit](../license-audit.md)

## Context

The research notes of 2026-10-31 named candidates but chose no dependency. PR #11 then added four optional crates (`ravif`, `avif-decode`, `avif-parse`, `moxcms`), raised the package MSRV to 1.98 and left `Cargo.lock` uncommitted, without a decision record. AGENTS.md requires an ADR before new dependencies. This ADR records that history, current choices, evidence, and open gates. It does not declare any of them release-ready. The dated 2026-11-05 facts below describe the original dependency graph; the amendment log records later replacements.

**Strategic direction.** Ferrisma's AVIF and JPEG XL codecs are meant to come from the sister projects [libavif-rs](../../codecs/libavif-rs) and [libjxl-rs](../../codecs/libjxl-rs): mechanical, reference-equivalent Rust ports of libavif (with libaom, dav1d and SharpYUV) and libjxl. Their state on 2026-11-05:

- **libavif-rs:** complete port with Rust AOM, dav1d and SharpYUV; validated as a whole pipeline on macOS ARM64 and, since 2026-11-06 (`f3b2981`), on GNU Linux x86-64 for six Cargo feature sets. It exposes the original unsafe C ABI, requires nightly Rust and sets `publish = false`.
- **libjxl-rs:** complete public decoder API and decoder C ABI with exact parity against libjxl; thread runners in review; no encoder yet. Its Rust API is provisional (`jxl::jxl` module tree) and the crate is not publishable.
- **libwebp-rs:** does not exist yet. WebP input and output are planned as a third port of libwebp 1.6.0 following the libjxl-rs method ([plan](../research/2026-11-05-libwebp-port-plan.md)); the libavif-rs repository already pins the same libwebp revision for SharpYUV.

Verified facts (2026-11-05, `main` at 9291c33):

- `default = []`. Without features the crate has no dependencies and forbids `unsafe`.
- `avif-decode` 3.0.0 declares `rust-version = 1.98`. Because `rust-version` is package-wide, the codec-free core also requires 1.98.
- `avif-decode` uses `rav1d` (unsafe Rust with optional assembly; NASM is needed on x86 hosts) and `avif-parse` 2.1.0 (MPL-2.0). Ferrisma also depends on `avif-parse` **directly** for the dimension preflight.
- Neither `avif-decode` nor `avif-parse` exposes the primary item's `colr`, `irot`, `imir` or `clap` properties or the AV1 sequence-header color configuration. `avif-decode` converts YUV with the bitstream matrix and range.
- `ravif` 0.13 defaults to 10-bit AV1 output (`BitDepth::Auto`) and omits the `colr` box when the values equal its sRGB/BT.709 primaries, BT.601 matrix, full-range default.

## Decision

Keep every codec and CMS behind its own Cargo feature, off by default:

| Feature | Crate | Role | Status of the choice |
| --- | --- | --- | --- |
| `jpeg-read` | workspace `libjpeg-turbo-rs` 0.1 | JPEG input (safe scanlines and codec crop request) | Adopted for engine input; scaled IDCT remains unused, ADR-0012/0025 |
| `jpeg-xl-read` | `jxl-oxide` 0.12 | Default JPEG XL input with a codec ROI request | Interim pending ADR-0025 selection gates |
| `jpeg-xl-own-read` | workspace `ferrisma-jxl-decoder` → `libjxl-rs` | Explicitly selected JPEG XL input with a full-frame fallback | **Experimental opt-in** through an isolated safe wrapper ([ADR-0032](0032-jpeg-xl-workspace-decoder-input.md)); external conformance and selection gates remain open |
| `avif-write` | `rav1e` 0.8 / `avif-serialize` 0.8 | Default AVIF output encoder, driven directly since 2026-11-06 | **Interim choice** (permissive, Rust plus assembly); target: libavif-rs |
| none (adapter crate) | `ferrisma-libavif` → libavif-rs (Rust AOM) | Optional output encoder through `transcode_with`, in an isolated workspace crate (ADR-0024/0026) | **Adopted with gates (a) and (b) waived** by the maintainer: nightly, `unsafe` C ABI isolated in the adapter, no native libraries since libavif-rs `8be367f`; macOS arm64 and x86-64 Linux CI |
| `avif-read` | workspace `ferrisma-avif-read` → `libavif-rs` / AOM | AVIF input, budgeted full-frame RGB fallback | **Experimental integration** through isolated safe wrapper; pinned nightly and native macOS ARM64/GNU Linux x86-64 ([ADR-0031](0031-workspace-libavif-input-experiment.md)); full Argon/dav1d and final selection gates remain open |
| `icc` | `moxcms` 0.9 | Rendered-ICC → sRGB source adapter | Selected independent LittleCMS fixtures pass (ADR-0023); broader real-image, LUT/intent, and HDR validation remains open |
| `webp-read` | workspace `libwebp-rs` 0.1 | Still WebP input, lossy and lossless | **Experimental adoption**, [ADR-0029](0029-webp-still-input.md) |
| `webp-write` | workspace `libwebp-rs` 0.1 | Still WebP output, lossy and lossless | **Experimental adoption**, [ADR-0030](0030-still-webp-output-and-format-neutral-encoder-result.md); ICC fixture has a separate license notice |
| `jpeg-xl-write` | workspace `libjxl-rs` 0.1 (safe fixed-profile entry point) | Lossless effort-2 JPEG XL output | **Experimental opt-in**, not in `cli`, [ADR-0039](0039-jpeg-xl-output-through-the-workspace-encoder.md); license/NOTICE review (#66) open; `jxl-encoder` stays excluded |
| JPEG output | none | — | **Not planned**; JPEG stays input-only through `jpeg-read` |

Rules that apply to all of them:

1. **Ferrisma owns color and geometry semantics.** Where a crate does not expose what the emitted samples mean, Ferrisma reads the evidence itself. For AVIF, a dependency-free, bounds-checked parser (`codecs/avif_meta.rs`) reads primary-item `colr`, `ispe`, `irot`, `imir`, and `clap`; a same-module helper in the pinned libavif port reads the selected AV1 color samples' sequence headers before pixel decode. Contradictory signals, unsupported transforms and unknown essential properties fail closed ([ADR-0031](0031-workspace-libavif-input-experiment.md)).
2. **Third-party codecs are interim.** The default JPEG XL reader `jxl-oxide` and default AVIF encoder `rav1e` remain interim runtime choices; `avif-decode` and `avif-parse` have been removed from the AVIF input graph. Still WebP input and output use workspace libwebp-rs. Adapters must stay replaceable without public API impact. The local libjxl-rs decoder is an explicit opt-in while ADR-0025's remaining gates are open; its encoder is not complete. `rav1d-safe` remains a historical fallback candidate, not the current AVIF reader.
3. **Adoption gates for the sister ports.** Ferrisma switches a codec when the port provides (a) a safe Rust API, or a small isolated adapter crate, because this package forbids `unsafe`; (b) a stable Rust toolchain; (c) a dependency Ferrisma can pin (published crate, pinned git revision, or committed workspace path/version); (d) evidence on Ferrisma's CI targets; and (e) exposed rendered color, orientation and alpha semantics, so that Ferrisma-side parsing such as `avif_meta.rs` becomes unnecessary or is cross-checked.

   The 2026-11-11 AVIF input integration is explicitly experimental while
   gate (b) and full native/conformance evidence remain open; it is not a
   declaration that final selection under ADR-0025 has finished.
4. **License gate.** Strong copyleft stays blocked by `scripts/audit-licenses.py`. MPL-2.0 `avif-parse` was a historical review item for the former AVIF input graph, not a current runtime dependency or an allowlist entry ([license audit](../license-audit.md)). The AGPL-3.0/commercial `jxl-encoder` remains unapproved; JPEG XL output needs its own ADR. The AGPL-3.0/commercial `zenjpeg` and `zenwebp` are unapproved for the same reason.
5. **MSRV.** Declare `rust-version = 1.98` for the single crate for now, and keep the MSRV CI job. Do not split crates only to lower the core MSRV before a consumer needs it (ADR-0008).
6. **Lockfile.** Commit `Cargo.lock`. The package ships a binary, CI and benchmarks need reproducible dependency versions, and the license inventory must refer to an exact version set.

## Consequences

+ The default build keeps only the first-party kernel dependency and a safe engine API; each codec can be audited, replaced or dropped independently.
+ Color correctness does not depend on a crate's undocumented conversion choices.
+ A committed lockfile makes CI, license inventory and benchmark results reproducible.
- Library users of the codec-free core still inherit Rust 1.98.
- AVIF input reaches unsafe generated libavif/AOM code through an isolated safe wrapper and needs the pinned nightly on the port's supported native targets; the engine crate itself still forbids `unsafe`.
- The former `avif-parse` and `avif-decode` three-pass path is replaced by the independent property check plus libavif's retained selected color samples ([ADR-0031](0031-workspace-libavif-input-experiment.md)).
- The lockfile needs regular updates (Renovate or manual) to pick up fixes.

## Alternatives considered

- **`libavif` or `dav1d` through FFI:** mature and complete, but adds a native toolchain and an unsafe boundary to the core build; reconsider only if pure-Rust decoders cannot meet correctness or coverage.
- **`rav1d-safe` now:** safer than `avif-decode`, but needs its own container assembly and dependency review; kept as a fallback behind the sister ports.
- **Wait for the sister ports before any AVIF/JXL support:** would block M1 on port work (safe API, stable toolchain, encoder) that has its own schedule.
- **Accept `avif-decode`'s color handling as is:** rejected; it silently ignores container color and orientation.
- **Split into `ferrisma-core` and codec crates now:** would lower the core MSRV, but ADR-0008 asks for a demonstrated need first.
- **Keep `Cargo.lock` uncommitted:** conventional for libraries years ago, but current Cargo guidance and the binary target favor committing it.
- **`image-webp` as an interim WebP decoder:** permissive and safe, but without lossy encoding or decode-side cropping, and the libwebp-rs decoder is the first port milestone; deferred unless WebP input is needed earlier.
- **libwebp through FFI (`webpx`):** rejected for the same reasons as libavif through FFI.
- **A JPEG encoder now (`mozjpeg-rs`, BSD-3-Clause, pure Rust):** deferred; M1 output is AVIF and no consumer needs JPEG output. It is the recorded candidate if that changes.

## Validation

- Feature-gated tests per adapter, including real ravif output with and without `colr`, contradictory `nclx`, PQ/HLG classification and the strict AVIF → AVIF facade path.
- CI: scoped feature tests and Clippy on their supported toolchains, an MSRV job for the codec-free and non-AVIF engine, fuzz smoke, and the license metadata screen. An engine all-feature build now includes native/nightly `avif-read`; workspace-wide all-features is not the component validation contract.

Before **Accepted**: complete transitive license and NOTICE review for the current graph; review native and unsafe code in the selected codec paths; exercise malformed AVIF/ICC and full AV1 conformance where required; and complete final decoder selection under ADR-0025. Historical MPL obligations apply only to distributions containing the former `avif-parse` graph.

## Review when

Review the active dependency graph when libavif-rs or libjxl-rs meets the remaining adoption gates (safe API, stable toolchain, pinnable release), the libjxl-rs encoder lands, another decoder exposes useful planar output and container metadata, a consumer needs an older engine MSRV, a dependency changes license, the adopted libwebp-rs path changes its parity, crop, color or memory contract, or a consumer needs JPEG output or expanded WebP output. The former `avif-decode` MSRV and `avif-parse` MPL triggers apply only if that historical graph is restored.

## Amendment log


- 2026-11-05 — Recorded the codec/CMS dependencies, MSRV and lockfile policy introduced with PR #11 and the AVIF color fix.
- 2026-11-05 — Made libavif-rs and libjxl-rs the replacement target for all AVIF/JPEG XL codecs, defined adoption gates and demoted `rav1d-safe` to a fallback.
- 2026-11-05 — Recorded JPEG as input-only, the planned libwebp-rs port as the WebP target with no interim WebP dependency, and `zenjpeg`/`zenwebp` as blocked by the license gate.
- 2026-11-06 — `avif-write` now enables `ravif`'s `asm` and `threading` features (M2 work package 2). `rav1e`'s assembly is bit-identical to its Rust fallbacks on all five images of the [encoder experiment](../research/encoder-experiment/results/2026-11-06-apple-m1-ultra.md) and encodes 2.1–2.4× faster on Apple Silicon. Consequences: the output encoder now links hand-written assembly (compiled with `cc` on aarch64 and NASM on x86-64, so `avif-write` builds need NASM on x86-64 like `avif-read` already did), and `rayon` becomes a runtime dependency of the encoder. No new crate enters the lockfile; `rayon`, `cc` and `nasm-rs` were already present.
- 2026-11-06 — libavif-rs admitted GNU Linux x86-64 (`f3b2981`), which covers Ferrisma's CI target. Gate (d) is now satisfiable for AVIF; gates (a) safe API and (b) stable toolchain remain open, and the libvmaf build requirement is an additional obstacle.
- 2026-11-06 — `avif-write` drives `rav1e` directly through Ferrisma's own sink and writes the container with `avif-serialize`; `ravif` remains only as a dev-dependency for one test fixture. Reason: `ravif` derives AV1 tiles from the thread count and supports neither 4:2:0 nor tile control. The sink keeps `ravif`'s quantizer curve and speed overrides (`src/av1_tuning.rs`, BSD-3-Clause notice retained) because the encoder experiment found `rav1e`'s plain presets 1.6–2× slower at equal SSIMULACRA2; with them the direct path produces byte-identical output to `ravif`.
- 2026-11-06 — `jpeg-read` enables `zune-jpeg`'s `neon` and `x86` features (ADR-0023): runtime-detected SIMD IDCT, upsampling and color conversion inside the decoder, with the decoder's own `unsafe`. Measured on the Apple M1 Ultra: the decode stage of the synthetic 24 and 48 Mpx JPEGs fell by about 20 %. The `icc` adapter now asks moxcms for linear-light sRGB output.
- 2026-11-06 — The maintainer accepted libavif-rs as an encoder now, waiving gates (a) and (b) for it: ADR-0024 adds pluggable encoders and the `crates/ferrisma-libavif` adapter outside the workspace, so the core stays stable and `unsafe`-free. libavif-rs's dav1d port and `rav1d` export the same `dav1d_*` symbols, so the adapter cannot be combined with `avif-read`.
- 2026-11-06 — ADR-0025 sets the final selection rule for the interim codecs: reference implementation or, for decoders, recorded conformance; replacement and removal only after a measurement on Arm and x86-64. JPEG input gains a sister-port target (libjpeg-turbo or jpegli, chosen by measurement) in place of `zune-jpeg`.
- 2026-11-07 — libavif-rs `8be367f` builds its AOM without VMAF tuning, which removes the libvmaf build requirement for AVIF. Gates (a) safe API and (b) stable toolchain remain open.
- 2026-11-10 — ADR-0026 moves the four ports and the existing adapter into the common workspace. Source links now point to their local locations; codec selection, the scoped adapter waiver, and the remaining adoption gates are unchanged.
- 2026-11-11 — `jpeg-read` now selects the workspace `libjpeg-turbo-rs` safe decoder, replacing the original zune table entry and full-frame adapter. The engine no longer depends on `zune-jpeg` at runtime. The existing standalone zune benchmark harness remains for comparison. No JPEG encoder or scaled-IDCT policy is added. The JPEG port already meets the safe API, stable toolchain, local pin, and reference validation gates; native platform and release evidence remain scoped to the codec validation tiers. Review the switch if its output semantics, platform validation, or measured performance diverge from the recorded evidence.
- 2026-11-11 — ADR-0029 adopts still WebP input through the workspace port's safe API. The monorepo path/version and shared lockfile replace the obsolete external revision gate #86 for this use. Decoder color, alpha, orientation and bounded caller-owned output are recorded there; output encoding and animation remain separate.
- 2026-11-11 — ADR-0030 adopts still `webp-write` through the workspace safe encoder API, with a separate ICC profile license notice and a frame/output limit contract.
- 2026-11-11 — ADR-0032 connects the local libjxl-rs decoder experimentally through an independent safe wrapper and explicit `jpeg-xl-own-read` selection. The measured photo-sized full/crop regression keeps jxl-oxide as the default `jpeg-xl-read` backend. The full external conformance and final selection gates remain open.
- 2026-11-11 — [ADR-0031](0031-workspace-libavif-input-experiment.md) replaces the interim `avif-decode`/`avif-parse`/`rav1d` input path with a separate safe wrapper over the workspace libavif/AOM reference decoder. It removes the duplicate `dav1d_*` link path with the encoder adapter, but brings the pinned nightly and native-target boundary to `avif-read`. Eight-fixture pinned C byte parity and the measured photo trade-off are recorded in [M2 AVIF input evidence](../benchmarks/M2-avif-input.md); full Argon/dav1d and final ADR-0025 selection gates remain open. This supersedes the 2026-11-06 collision statement for current builds without rewriting its historical rationale.

- 2026-11-11 — [ADR-0037](0037-selectable-workspace-avif-encoder.md) exposes
  the workspace libavif/AOM writer as an explicit `avif-own-write` facade and
  CLI choice through an independent safe bridge. The codec-free and
  stable-compatible feature sets do not acquire the nightly native writer.
  Rav1e remains the default and ADR-0025's final selection gates remain open.
- 2026-11-12 — [ADR-0039](0039-jpeg-xl-output-through-the-workspace-encoder.md)
  settles the separate JPEG XL output gate: opt-in `jpeg-xl-write` uses the
  workspace libjxl-rs encoder's verified lossless effort-2 profile through a
  safe entry point. The AGPL-3.0/commercial `jxl-encoder` stays unapproved;
  the NOTICE review (#66) remains open, so the feature stays out of `cli`.
